//! MCP server (stdio, newline-delimited JSON-RPC 2.0).
//! Exposes the company graph to any MCP-capable agent: validate, query, resolve.
//! The workspace root is fixed at server start; every tool call re-reads from disk,
//! so agents always see the current state of the description.

use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

pub fn serve(root: PathBuf) -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let Ok(msg) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let id = msg.get("id").cloned();
        let method = msg.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let params = msg.get("params").cloned().unwrap_or(json!({}));

        let outcome: Option<Result<Value, (i64, String)>> = match method {
            "initialize" => Some(Ok(json!({
                "protocolVersion": params
                    .get("protocolVersion")
                    .cloned()
                    .unwrap_or(json!("2024-11-05")),
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "charta", "version": env!("CARGO_PKG_VERSION") }
            }))),
            "ping" => Some(Ok(json!({}))),
            "tools/list" => Some(Ok(tools_list())),
            "tools/call" => Some(Ok(tools_call(&root, &params))),
            _ if id.is_some() => Some(Err((-32601, format!("method not found: {method}")))),
            _ => None, // notification — no response
        };

        if let (Some(outcome), Some(id)) = (outcome, id) {
            let response = match outcome {
                Ok(result) => json!({ "jsonrpc": "2.0", "id": id, "result": result }),
                Err((code, message)) => json!({
                    "jsonrpc": "2.0", "id": id,
                    "error": { "code": code, "message": message }
                }),
            };
            let mut out = stdout.lock();
            writeln!(out, "{response}")?;
            out.flush()?;
        }
    }
    Ok(())
}

fn tools_list() -> Value {
    json!({ "tools": [
        {
            "name": "charta_validate",
            "description": "Validate the company description (L0 well-formedness, L1 referential integrity, L2 required fields). Returns resource count, errors, warnings, findings.",
            "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false }
        },
        {
            "name": "charta_query",
            "description": "Query the company graph. verb 'orphans' lists resources nothing references; verb 'backlinks' lists incoming edges for a target (e.g. 'goal/funding-readiness').",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "verb": { "type": "string", "enum": ["orphans", "backlinks"] },
                    "target": { "type": "string", "description": "type/id — required for 'backlinks'" }
                },
                "required": ["verb"],
                "additionalProperties": false
            }
        },
        {
            "name": "charta_resolve",
            "description": "Resolve one resource by address (type/id, e.g. 'process/invoicing'): envelope, full prose body, and incoming references. Use this to read how the company defines a role, process, goal, policy, or product.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "address": { "type": "string", "description": "type/id" }
                },
                "required": ["address"],
                "additionalProperties": false
            }
        }
    ]})
}

fn text_result(text: String, is_error: bool) -> Value {
    json!({ "content": [ { "type": "text", "text": text } ], "isError": is_error })
}

fn tools_call(root: &Path, params: &Value) -> Value {
    let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
    let args = params.get("arguments").cloned().unwrap_or(json!({}));

    let mut ws = crate::load(root);
    crate::validate(&mut ws);

    match name {
        "charta_validate" => {
            let errors = ws.findings.iter().filter(|f| f.severity == "error").count();
            let warnings = ws
                .findings
                .iter()
                .filter(|f| f.severity == "warning")
                .count();
            let out = json!({
                "resources": ws.resources.len(),
                "errors": errors,
                "warnings": warnings,
                "findings": ws.findings,
            });
            text_result(serde_json::to_string_pretty(&out).unwrap(), errors > 0)
        }
        "charta_query" => {
            let graph = crate::graph(&ws);
            match args.get("verb").and_then(|v| v.as_str()) {
                Some("orphans") => text_result(
                    serde_json::to_string_pretty(&crate::orphans(&graph)).unwrap(),
                    false,
                ),
                Some("backlinks") => match args.get("target").and_then(|t| t.as_str()) {
                    Some(target) => {
                        let edges: Vec<Value> = crate::backlinks(&graph, target)
                            .iter()
                            .map(|e| json!({ "from": e.from, "field": e.field }))
                            .collect();
                        text_result(serde_json::to_string_pretty(&edges).unwrap(), false)
                    }
                    None => text_result("'backlinks' requires 'target' (type/id)".into(), true),
                },
                _ => text_result("unknown verb — use 'orphans' or 'backlinks'".into(), true),
            }
        }
        "charta_resolve" => {
            let Some(address) = args.get("address").and_then(|a| a.as_str()) else {
                return text_result("'address' (type/id) is required".into(), true);
            };
            match ws.resources.iter().find(|r| r.addr() == address) {
                None => {
                    let known: Vec<String> = ws.resources.iter().map(|r| r.addr()).collect();
                    text_result(
                        format!(
                            "'{address}' not found. Known resources: {}",
                            known.join(", ")
                        ),
                        true,
                    )
                }
                Some(r) => {
                    let graph = crate::graph(&ws);
                    let backlinks: Vec<Value> = crate::backlinks(&graph, address)
                        .iter()
                        .map(|e| json!({ "from": e.from, "field": e.field }))
                        .collect();
                    let envelope = serde_json::to_value(&r.fields)
                        .unwrap_or_else(|_| json!("<unserializable frontmatter>"));
                    let out = json!({
                        "address": address,
                        "file": r.file,
                        "envelope": envelope,
                        "body": r.body,
                        "backlinks": backlinks,
                    });
                    text_result(serde_json::to_string_pretty(&out).unwrap(), false)
                }
            }
        }
        other => text_result(format!("unknown tool: {other}"), true),
    }
}
