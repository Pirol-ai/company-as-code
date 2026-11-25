use std::path::{Path, PathBuf};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../conformance/fixtures")
}

fn run(root: &Path) -> charta::Workspace {
    let mut ws = charta::load(root);
    charta::validate(&mut ws);
    ws
}

#[test]
fn valid_fixtures_are_green() {
    let dir = fixtures().join("valid");
    let mut checked = 0;
    for entry in std::fs::read_dir(&dir).expect("valid fixtures dir") {
        let path = entry.unwrap().path();
        if !path.is_dir() {
            continue;
        }
        let ws = run(&path);
        let errors: Vec<_> = ws
            .findings
            .iter()
            .filter(|f| f.severity == "error")
            .collect();
        assert!(
            errors.is_empty(),
            "fixture {:?} must validate green, got: {:#?}",
            path.file_name().unwrap(),
            errors
        );
        assert!(
            !ws.resources.is_empty(),
            "fixture {:?} loaded no resources",
            path
        );
        checked += 1;
    }
    assert!(checked > 0, "no valid fixtures found");
}

#[test]
fn invalid_fixtures_produce_expected_errors() {
    let dir = fixtures().join("invalid");
    let mut checked = 0;
    for entry in std::fs::read_dir(&dir).expect("invalid fixtures dir") {
        let path = entry.unwrap().path();
        if !path.is_dir() {
            continue;
        }
        let expected_path = path.join("expected.json");
        let expected: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&expected_path).expect("expected.json"))
                .expect("expected.json parses");
        let ws = run(&path);
        for exp in expected["errors"].as_array().expect("errors array") {
            let matched = ws.findings.iter().any(|f| {
                f.severity == "error"
                    && f.code == exp["code"].as_str().unwrap_or_default()
                    && f.resource.as_deref() == exp["resource"].as_str()
                    && (exp["field"].is_null() || f.field.as_deref() == exp["field"].as_str())
                    && (exp["target"].is_null() || f.target.as_deref() == exp["target"].as_str())
            });
            assert!(
                matched,
                "fixture {:?}: expected error {exp} not found in {:#?}",
                path.file_name().unwrap(),
                ws.findings
            );
        }
        checked += 1;
    }
    assert!(checked > 0, "no invalid fixtures found");
}

/// §7 "ignore and preserve": reading a resource and writing it back must return the
/// file unchanged — byte for byte. Extension content (namespaced frontmatter keys,
/// fenced body blocks), comments, key order and quoting style all depend on this.
/// A runtime that stores an executable process in a body loses it the day this fails.
#[test]
fn round_trip_is_byte_identical() {
    let mut checked = 0;
    for family in ["valid", "invalid"] {
        let dir = fixtures().join(family);
        for entry in std::fs::read_dir(&dir).expect("fixtures dir") {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                continue;
            }
            let ws = charta::load(&path);
            for r in &ws.resources {
                let original = std::fs::read_to_string(path.join(&r.file)).expect("fixture file");
                let written = charta::write_resource(r);
                assert_eq!(
                    original,
                    written,
                    "round-trip changed {} in fixture {:?}",
                    r.file,
                    path.file_name().unwrap()
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "no resources round-tripped");
}

/// The extensions fixture must actually carry extension content in both places it can
/// appear — otherwise the round-trip test above proves nothing about preservation.
#[test]
fn extension_content_survives_and_is_not_validated() {
    let path = fixtures().join("valid/extensions");
    let ws = run(&path);
    assert!(
        ws.findings.iter().all(|f| f.severity != "error"),
        "extension content must not produce errors: {:#?}",
        ws.findings
    );
    let p = ws
        .resources
        .iter()
        .find(|r| r.addr() == "process/onboarding")
        .expect("process/onboarding");
    assert!(
        p.fields
            .contains_key(serde_yaml::Value::from("x-coos/steps")),
        "namespaced frontmatter key was dropped"
    );
    assert!(
        p.body.contains("```x-coos/steps"),
        "namespaced body block was dropped"
    );
    assert!(
        charta::write_resource(p).contains("x-coos/run-mode: 'supervised'"),
        "quoting style of an extension value was not preserved"
    );
}
