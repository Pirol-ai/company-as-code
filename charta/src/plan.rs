//! `charta plan` — graph diff between a committed baseline and the working tree.
//! Shows what a change to the company description would touch, before it lands:
//! added / changed / removed resources plus the impact set (who references them),
//! and how many validation errors the resulting state would carry.

use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Serialize)]
pub struct Changed {
    pub address: String,
    pub fields: Vec<String>,
    pub prose: bool,
}

#[derive(Debug, Serialize)]
pub struct Impact {
    pub from: String,
    pub field: String,
    pub to: String,
}

#[derive(Debug, Serialize)]
pub struct Plan {
    pub from: String,
    pub added: Vec<String>,
    pub changed: Vec<Changed>,
    pub removed: Vec<String>,
    pub impacts: Vec<Impact>,
    pub resulting_errors: usize,
}

/// Materialise `<ref>` of the git repo at `root` into a temp directory via `git archive | tar`.
fn materialise_baseline(root: &Path, git_ref: &str) -> Result<PathBuf, String> {
    let tmp = std::env::temp_dir().join(format!("charta-plan-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).map_err(|e| format!("cannot create temp dir: {e}"))?;

    let mut git = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["archive", git_ref])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot run git: {e}"))?;
    let git_out = git.stdout.take().expect("piped stdout");

    let tar = Command::new("tar")
        .args(["-x", "-C"])
        .arg(&tmp)
        .stdin(Stdio::from(git_out))
        .status()
        .map_err(|e| format!("cannot run tar: {e}"))?;

    let git_status = git.wait().map_err(|e| format!("git failed: {e}"))?;
    if !git_status.success() {
        let mut err = String::new();
        if let Some(mut stderr) = git.stderr.take() {
            use std::io::Read;
            let _ = stderr.read_to_string(&mut err);
        }
        return Err(format!(
            "git archive '{git_ref}' failed — is '{}' a git repo and the ref valid? {}",
            root.display(),
            err.trim()
        ));
    }
    if !tar.success() {
        return Err("tar extraction of the baseline failed".into());
    }
    Ok(tmp)
}

pub fn plan(root: &Path, git_ref: &str) -> Result<Plan, String> {
    let baseline_dir = materialise_baseline(root, git_ref)?;

    let mut baseline = crate::load(&baseline_dir);
    crate::validate(&mut baseline);
    let mut current = crate::load(root);
    crate::validate(&mut current);

    let base_map: BTreeMap<String, &crate::Resource> =
        baseline.resources.iter().map(|r| (r.addr(), r)).collect();
    let cur_map: BTreeMap<String, &crate::Resource> =
        current.resources.iter().map(|r| (r.addr(), r)).collect();

    let mut added = Vec::new();
    let mut changed = Vec::new();
    let mut removed = Vec::new();

    for (addr, cur) in &cur_map {
        match base_map.get(addr) {
            None => added.push(addr.clone()),
            Some(base) => {
                let mut fields = Vec::new();
                let keys: std::collections::BTreeSet<String> = base
                    .fields
                    .keys()
                    .chain(cur.fields.keys())
                    .filter_map(|k| k.as_str().map(str::to_string))
                    .collect();
                for key in keys {
                    let kv = serde_yaml::Value::from(key.as_str());
                    if base.fields.get(&kv) != cur.fields.get(&kv) {
                        fields.push(key);
                    }
                }
                let prose = base.body != cur.body;
                if !fields.is_empty() || prose {
                    changed.push(Changed {
                        address: addr.clone(),
                        fields,
                        prose,
                    });
                }
            }
        }
    }
    for addr in base_map.keys() {
        if !cur_map.contains_key(addr) {
            removed.push(addr.clone());
        }
    }

    // Impact set: who references the changed/removed resources.
    // Removed resources no longer appear in the current graph, so their incoming
    // edges come from the baseline graph; changed resources use the current graph.
    let base_graph = crate::graph(&baseline);
    let cur_graph = crate::graph(&current);
    let mut impacts = Vec::new();
    for addr in &removed {
        for e in crate::backlinks(&base_graph, addr) {
            impacts.push(Impact {
                from: e.from.clone(),
                field: e.field.clone(),
                to: addr.clone(),
            });
        }
    }
    for c in &changed {
        for e in crate::backlinks(&cur_graph, &c.address) {
            impacts.push(Impact {
                from: e.from.clone(),
                field: e.field.clone(),
                to: c.address.clone(),
            });
        }
    }

    let resulting_errors = current
        .findings
        .iter()
        .filter(|f| f.severity == "error")
        .count();

    let _ = std::fs::remove_dir_all(&baseline_dir);

    Ok(Plan {
        from: git_ref.to_string(),
        added,
        changed,
        removed,
        impacts,
        resulting_errors,
    })
}

pub fn render(p: &Plan) -> String {
    let mut out = String::new();
    for a in &p.added {
        out.push_str(&format!("+ {a}\n"));
    }
    for c in &p.changed {
        let mut what = c.fields.clone();
        if c.prose {
            what.push("prose".into());
        }
        out.push_str(&format!("~ {}    ({})\n", c.address, what.join(", ")));
    }
    for r in &p.removed {
        out.push_str(&format!("- {r}\n"));
    }
    for i in &p.impacts {
        out.push_str(&format!("! {}    {} → {}\n", i.from, i.field, i.to));
    }
    out.push_str(&format!(
        "{} added · {} changed · {} removed · {} impacted · resulting errors: {}\n",
        p.added.len(),
        p.changed.len(),
        p.removed.len(),
        p.impacts.len(),
        p.resulting_errors
    ));
    out
}
