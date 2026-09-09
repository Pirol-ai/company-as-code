use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

pub mod mcp;

pub const API_PREFIX: &str = "company-as-code.org/";
pub const API_V0: &str = "company-as-code.org/v0";
pub const CORE_KINDS: &[&str] = &[
    "company", "goal", "role", "process", "policy", "product", "decision", "module",
];
/// Envelope fields treated as typed references. Values must resolve to an existing kind/id.
pub const REF_FIELDS_SINGLE: &[&str] = &["owner", "executed_by"];
pub const REF_FIELDS_LIST: &[&str] = &["serves", "policies"];
/// Per-kind required envelope fields (L2). Fixed by the conformance fixtures.
pub const REQUIRED_FIELDS: &[(&str, &str)] = &[("process", "owner"), ("goal", "owner")];

const SKIP_DIRS: &[&str] = &[".git", "node_modules", "target", "dist"];

#[derive(Debug, Clone, Serialize)]
pub struct Resource {
    pub r#type: String,
    pub id: String,
    pub title: Option<String>,
    pub file: String,
    #[serde(skip)]
    pub fields: serde_yaml::Mapping,
    #[serde(skip)]
    pub prose_refs: Vec<String>,
    #[serde(skip)]
    pub body: String,
}

impl Resource {
    pub fn addr(&self) -> String {
        format!("{}/{}", self.r#type, self.id)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Finding {
    pub level: String,
    pub severity: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct Node {
    pub r#type: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub file: String,
}

#[derive(Debug, Serialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub field: String,
}

#[derive(Debug, Serialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub struct Workspace {
    pub root: PathBuf,
    pub resources: Vec<Resource>,
    pub findings: Vec<Finding>,
}

fn finding(
    level: &str,
    severity: &str,
    code: &str,
    resource: Option<String>,
    field: Option<&str>,
    target: Option<&str>,
    message: String,
) -> Finding {
    Finding {
        level: level.into(),
        severity: severity.into(),
        code: code.into(),
        resource,
        field: field.map(Into::into),
        target: target.map(Into::into),
        message,
    }
}

fn valid_id(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !s.starts_with('-')
}

fn valid_addr(s: &str) -> bool {
    match s.split_once('/') {
        Some((kind, id)) => {
            let kind_ok = valid_id(kind) || (kind.starts_with("x-") && valid_id(&kind[2..]));
            kind_ok && valid_id(id)
        }
        None => false,
    }
}

/// Split a markdown file into (frontmatter-yaml, body). Returns None when no frontmatter block.
fn split_frontmatter(text: &str) -> Option<(&str, &str)> {
    let rest = text
        .strip_prefix("---\n")
        .or(text.strip_prefix("---\r\n"))?;
    for terminator in ["\n---\n", "\n---\r\n"] {
        if let Some(pos) = rest.find(terminator) {
            return Some((&rest[..pos], &rest[pos + terminator.len()..]));
        }
    }
    // frontmatter that ends the file: "---" on the last line
    for terminator in ["\n---", "\r\n---"] {
        if let Some(stripped) = rest.strip_suffix(terminator) {
            return Some((stripped, ""));
        }
    }
    None
}

fn scan_prose_refs(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = body.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'[' && bytes[i + 1] == b'[' {
            if let Some(end) = body[i + 2..].find("]]") {
                let inner = &body[i + 2..i + 2 + end];
                if inner.contains('/') && !inner.contains('\n') && !inner.contains(' ') {
                    out.push(inner.to_string());
                }
                i += 2 + end + 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

pub fn load(root: &Path) -> Workspace {
    let mut ws = Workspace {
        root: root.to_path_buf(),
        resources: Vec::new(),
        findings: Vec::new(),
    };

    // Manifest (L0)
    let manifest = root.join("company.yaml");
    match std::fs::read_to_string(&manifest) {
        Err(_) => ws.findings.push(finding(
            "L0",
            "error",
            "missing-manifest",
            None,
            None,
            None,
            "company.yaml not found at workspace root".into(),
        )),
        Ok(text) => match serde_yaml::from_str::<serde_yaml::Mapping>(&text) {
            Err(e) => ws.findings.push(finding(
                "L0",
                "error",
                "invalid-manifest",
                None,
                None,
                None,
                format!("company.yaml does not parse as YAML: {e}"),
            )),
            Ok(m) => {
                let api = m
                    .get(serde_yaml::Value::from("api"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !api.starts_with(API_PREFIX) {
                    ws.findings.push(finding(
                        "L0",
                        "error",
                        "manifest-api",
                        None,
                        Some("api"),
                        None,
                        format!("company.yaml api must start with {API_PREFIX} (got '{api}')"),
                    ));
                }
            }
        },
    }

    // Resource discovery
    let mut seen: BTreeMap<String, String> = BTreeMap::new();
    let walker = walkdir::WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !(e.file_type().is_dir()
                && (SKIP_DIRS.contains(&name.as_ref()) || name.starts_with('.')))
        });
    for entry in walker.flatten() {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let rel = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");
        let Some((front, body)) = split_frontmatter(&text) else {
            continue;
        };
        let mapping = match serde_yaml::from_str::<serde_yaml::Mapping>(front) {
            Ok(m) => m,
            Err(e) => {
                if front.contains(API_PREFIX) {
                    ws.findings.push(finding(
                        "L0",
                        "error",
                        "invalid-frontmatter",
                        Some(rel.clone()),
                        None,
                        None,
                        format!("frontmatter does not parse as YAML: {e}"),
                    ));
                }
                continue;
            }
        };
        let get = |key: &str| -> Option<String> {
            mapping
                .get(serde_yaml::Value::from(key))
                .and_then(|v| v.as_str())
                .map(str::to_string)
        };
        let Some(api) = get("api") else { continue }; // no api key → not a resource (foreign frontmatter)
        if !api.starts_with(API_PREFIX) {
            continue;
        }
        if api != API_V0 {
            ws.findings.push(finding(
                "L0",
                "warning",
                "unknown-api-version",
                Some(rel.clone()),
                Some("api"),
                None,
                format!("unknown api version '{api}' (this build knows {API_V0})"),
            ));
        }
        let ty = get("type");
        let id = get("id");
        match (&ty, &id) {
            (None, _) => {
                ws.findings.push(finding(
                    "L0",
                    "error",
                    "missing-type",
                    Some(rel.clone()),
                    Some("type"),
                    None,
                    "resource envelope has no 'type'".into(),
                ));
                continue;
            }
            (_, None) => {
                ws.findings.push(finding(
                    "L0",
                    "error",
                    "missing-id",
                    Some(rel.clone()),
                    Some("id"),
                    None,
                    "resource envelope has no 'id'".into(),
                ));
                continue;
            }
            _ => {}
        }
        let (ty, id) = (ty.unwrap(), id.unwrap());
        if !valid_id(&id) {
            ws.findings.push(finding(
                "L0",
                "error",
                "invalid-id",
                Some(format!("{ty}/{id}")),
                Some("id"),
                None,
                format!("id '{id}' must be kebab-case [a-z0-9-]"),
            ));
        }
        if !CORE_KINDS.contains(&ty.as_str()) && !ty.starts_with("x-") {
            ws.findings.push(finding(
                "L0",
                "warning",
                "unknown-type",
                Some(format!("{ty}/{id}")),
                Some("type"),
                None,
                format!("type '{ty}' is neither a core type nor x- namespaced (preserved, not validated)"),
            ));
        }
        let addr = format!("{ty}/{id}");
        if let Some(other) = seen.get(&addr) {
            ws.findings.push(finding(
                "L0",
                "error",
                "duplicate-id",
                Some(addr.clone()),
                None,
                None,
                format!("'{addr}' defined in both {other} and {rel}"),
            ));
        } else {
            seen.insert(addr, rel.clone());
        }
        ws.resources.push(Resource {
            r#type: ty,
            id,
            title: get("title"),
            file: rel,
            fields: mapping,
            prose_refs: scan_prose_refs(body),
            body: body.to_string(),
        });
    }
    ws
}

pub fn validate(ws: &mut Workspace) {
    let addrs: BTreeSet<String> = ws.resources.iter().map(|r| r.addr()).collect();
    let mut new: Vec<Finding> = Vec::new();

    for r in &ws.resources {
        // L1 — typed refs must resolve
        let check_ref =
            |new: &mut Vec<Finding>, field: &str, value: &serde_yaml::Value| match value.as_str() {
                Some(s) => {
                    if !valid_addr(s) {
                        new.push(finding(
                            "L1",
                            "error",
                            "invalid-ref",
                            Some(r.addr()),
                            Some(field),
                            Some(s),
                            format!("'{s}' is not a type/id reference"),
                        ));
                    } else if !addrs.contains(s) {
                        new.push(finding(
                            "L1",
                            "error",
                            "unresolved-ref",
                            Some(r.addr()),
                            Some(field),
                            Some(s),
                            format!("{} → {field}: '{s}' does not exist", r.addr()),
                        ));
                    }
                }
                None => new.push(finding(
                    "L1",
                    "error",
                    "invalid-ref",
                    Some(r.addr()),
                    Some(field),
                    None,
                    format!("{field} must be a string reference"),
                )),
            };
        for f in REF_FIELDS_SINGLE {
            if let Some(v) = r.fields.get(serde_yaml::Value::from(*f)) {
                check_ref(&mut new, f, v);
            }
        }
        for f in REF_FIELDS_LIST {
            if let Some(v) = r.fields.get(serde_yaml::Value::from(*f)) {
                match v.as_sequence() {
                    Some(seq) => {
                        for item in seq {
                            check_ref(&mut new, f, item);
                        }
                    }
                    None => new.push(finding(
                        "L1",
                        "error",
                        "invalid-ref",
                        Some(r.addr()),
                        Some(f),
                        None,
                        format!("{f} must be a list of references"),
                    )),
                }
            }
        }
        for p in &r.prose_refs {
            if !addrs.contains(p) {
                new.push(finding(
                    "L1",
                    "warning",
                    "unresolved-prose-ref",
                    Some(r.addr()),
                    None,
                    Some(p.as_str()),
                    format!("prose link [[{p}]] does not resolve"),
                ));
            }
        }
        // L2 — per-type required fields
        for (ty, req) in REQUIRED_FIELDS {
            if r.r#type == *ty && !r.fields.contains_key(serde_yaml::Value::from(*req)) {
                new.push(finding(
                    "L2",
                    "error",
                    "missing-required-field",
                    Some(r.addr()),
                    Some(req),
                    None,
                    format!("type '{ty}' requires field '{req}'"),
                ));
            }
        }
    }
    ws.findings.append(&mut new);
}

pub fn graph(ws: &Workspace) -> Graph {
    let addrs: BTreeSet<String> = ws.resources.iter().map(|r| r.addr()).collect();
    let mut edges = Vec::new();
    for r in &ws.resources {
        let mut push = |field: &str, to: &str| {
            if addrs.contains(to) {
                edges.push(Edge {
                    from: r.addr(),
                    to: to.into(),
                    field: field.into(),
                });
            }
        };
        for f in REF_FIELDS_SINGLE {
            if let Some(s) = r
                .fields
                .get(serde_yaml::Value::from(*f))
                .and_then(|v| v.as_str())
            {
                push(f, s);
            }
        }
        for f in REF_FIELDS_LIST {
            if let Some(seq) = r
                .fields
                .get(serde_yaml::Value::from(*f))
                .and_then(|v| v.as_sequence())
            {
                for item in seq {
                    if let Some(s) = item.as_str() {
                        push(f, s);
                    }
                }
            }
        }
        for p in &r.prose_refs {
            push("prose", p);
        }
    }
    Graph {
        nodes: ws
            .resources
            .iter()
            .map(|r| Node {
                r#type: r.r#type.clone(),
                id: r.id.clone(),
                title: r.title.clone(),
                file: r.file.clone(),
            })
            .collect(),
        edges,
    }
}

pub fn orphans(g: &Graph) -> Vec<String> {
    let referenced: BTreeSet<&String> = g.edges.iter().map(|e| &e.to).collect();
    g.nodes
        .iter()
        .map(|n| format!("{}/{}", n.r#type, n.id))
        .filter(|a| !referenced.contains(a))
        .collect()
}

pub fn backlinks<'g>(g: &'g Graph, target: &str) -> Vec<&'g Edge> {
    g.edges.iter().filter(|e| e.to == target).collect()
}
