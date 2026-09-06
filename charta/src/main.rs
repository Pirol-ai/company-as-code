use std::path::PathBuf;
use std::process::ExitCode;

fn usage() -> ExitCode {
    eprintln!(
        "charta — reference toolchain for Company as Code\n\n\
         Usage:\n  charta validate [path] [--json]\n  charta graph [path]\n  \
         charta query orphans [path]\n  charta query backlinks <kind/id> [path]\n"
    );
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let json = args.iter().any(|a| a == "--json");
    let positional: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    let Some(cmd) = positional.first() else {
        return usage();
    };

    let path_arg = |idx: usize| -> PathBuf {
        positional
            .get(idx)
            .map(|s| PathBuf::from(s.as_str()))
            .unwrap_or_else(|| PathBuf::from("."))
    };

    match cmd.as_str() {
        "validate" => {
            let root = path_arg(1);
            let mut ws = charta::load(&root);
            charta::validate(&mut ws);
            let errors = ws.findings.iter().filter(|f| f.severity == "error").count();
            let warnings = ws
                .findings
                .iter()
                .filter(|f| f.severity == "warning")
                .count();
            if json {
                let out = serde_json::json!({
                    "resources": ws.resources.len(),
                    "errors": errors,
                    "warnings": warnings,
                    "findings": ws.findings,
                });
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
            } else {
                for f in &ws.findings {
                    let loc = f.resource.as_deref().unwrap_or("-");
                    let field = f
                        .field
                        .as_deref()
                        .map(|s| format!(" {s}:"))
                        .unwrap_or_default();
                    println!(
                        "{} {} [{}]{} {}",
                        f.level, f.severity, loc, field, f.message
                    );
                }
                println!(
                    "{} resources · {} errors · {} warnings",
                    ws.resources.len(),
                    errors,
                    warnings
                );
            }
            if errors > 0 {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        "graph" => {
            let root = path_arg(1);
            let mut ws = charta::load(&root);
            charta::validate(&mut ws);
            let g = charta::graph(&ws);
            println!("{}", serde_json::to_string_pretty(&g).unwrap());
            ExitCode::SUCCESS
        }
        "query" => match positional.get(1).map(|s| s.as_str()) {
            Some("orphans") => {
                let root = path_arg(2);
                let mut ws = charta::load(&root);
                charta::validate(&mut ws);
                let g = charta::graph(&ws);
                for o in charta::orphans(&g) {
                    println!("{o}");
                }
                ExitCode::SUCCESS
            }
            Some("backlinks") => {
                let Some(target) = positional.get(2) else {
                    return usage();
                };
                let root = path_arg(3);
                let mut ws = charta::load(&root);
                charta::validate(&mut ws);
                let g = charta::graph(&ws);
                for e in charta::backlinks(&g, target) {
                    println!("{} --{}--> {}", e.from, e.field, e.to);
                }
                ExitCode::SUCCESS
            }
            _ => usage(),
        },
        _ => usage(),
    }
}
