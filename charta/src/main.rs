use std::path::PathBuf;
use std::process::ExitCode;

fn usage() -> ExitCode {
    eprintln!(
        "charta — reference toolchain for Company as Code\n\n\
         Usage:\n  charta init [path]\n  charta validate [path] [--json]\n  \
         charta graph [path] [--format json|mermaid|dot]\n  \
         charta query orphans [path]\n  charta query backlinks <type/id> [path]\n  \
         charta plan [path] [--from <ref>] [--json]\n  charta mcp [path]\n"
    );
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut json = false;
    let mut from = String::from("HEAD");
    let mut format = String::from("json");
    let mut positional: Vec<String> = Vec::new();
    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--json" => json = true,
            "--from" => {
                i += 1;
                match raw.get(i) {
                    Some(v) => from = v.clone(),
                    None => return usage(),
                }
            }
            "--format" => {
                i += 1;
                match raw.get(i) {
                    Some(v) => format = v.clone(),
                    None => return usage(),
                }
            }
            s if s.starts_with("--") => return usage(),
            s => positional.push(s.to_string()),
        }
        i += 1;
    }
    let Some(cmd) = positional.first().cloned() else {
        return usage();
    };

    let path_arg = |idx: usize| -> PathBuf {
        positional
            .get(idx)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
    };

    match cmd.as_str() {
        "init" => {
            let root = path_arg(1);
            let manifest = root.join("company.yaml");
            if manifest.exists() {
                eprintln!("init: {} already exists", manifest.display());
                return ExitCode::FAILURE;
            }
            if let Err(e) = std::fs::create_dir_all(&root) {
                eprintln!("init: cannot create {}: {e}", root.display());
                return ExitCode::FAILURE;
            }
            // Default the name to the directory it lives in; the founder edits one line.
            let name = std::fs::canonicalize(&root)
                .ok()
                .and_then(|p| p.file_name().map(|s| s.to_string_lossy().into_owned()))
                .unwrap_or_else(|| "My Company".to_string());
            let contents = format!("api: {}\nname: {name}\nextensions: []\n", charta::API_V0);
            if let Err(e) = std::fs::write(&manifest, contents) {
                eprintln!("init: cannot write {}: {e}", manifest.display());
                return ExitCode::FAILURE;
            }
            println!("Created {} (name: {name})", manifest.display());
            println!(
                "This marks the root of your company description. Now describe your company —\n\
                 tell your agent how it works, or write the first resource file yourself.\n\
                 Check it any time with: charta validate ."
            );
            ExitCode::SUCCESS
        }
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
            match format.as_str() {
                "json" => println!("{}", serde_json::to_string_pretty(&g).unwrap()),
                "mermaid" => print!("{}", charta::to_mermaid(&g)),
                "dot" => print!("{}", charta::to_dot(&g)),
                _ => return usage(),
            }
            ExitCode::SUCCESS
        }
        "plan" => {
            let root = path_arg(1);
            match charta::plan::plan(&root, &from) {
                Ok(p) => {
                    if json {
                        println!("{}", serde_json::to_string_pretty(&p).unwrap());
                    } else {
                        print!("{}", charta::plan::render(&p));
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("plan: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "mcp" => {
            let root = path_arg(1);
            match charta::mcp::serve(root) {
                Ok(()) => ExitCode::SUCCESS,
                Err(e) => {
                    eprintln!("mcp server error: {e}");
                    ExitCode::FAILURE
                }
            }
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
