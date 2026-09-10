# charta

Reference toolchain for Company as Code. One Rust crate, single static binary.

## Implemented (M1 slice 1)

| verb | status |
|---|---|
| `charta validate [path] [--json]` | ✅ L0 (well-formed: manifest, envelope, id format, uniqueness) · L1 (referential integrity, typed + prose refs) · L2 (per-kind required fields). Exit 1 on errors. |
| `charta graph [path]` | ✅ nodes + edges as JSON, Mermaid (`--format mermaid`, GitHub-rendered), or Graphviz DOT (`--format dot`) |
| `charta query orphans [path]` | ✅ resources nothing references |
| `charta query backlinks <kind/id> [path]` | ✅ incoming edges |
| `charta plan [path] [--from <ref>] [--json]` | ✅ graph diff committed baseline vs. working tree: added / changed (with fields) / removed + impact set (who references what changed) + resulting error count |
| `charta fmt` | ⏳ |
| `charta mcp [path]` | ✅ stdio MCP server: `charta_validate`, `charta_query` (orphans/backlinks), `charta_resolve` (envelope + body + backlinks) — every MCP-capable agent reads the company graph as tools |

Conformance: `cargo test` runs every fixture in [`../conformance/fixtures/`](../conformance/fixtures/)
— valid ones must be green, invalid ones must produce exactly the errors their `expected.json`
declares. The suite is the standard; code follows fixtures.

Resource discovery rule (v0): a markdown file is a resource iff its frontmatter has an `api:` key
starting with `company-as-code.org/`. Files with foreign or no frontmatter are ignored — this is
what lets the format live inside an existing repo without claiming every file.

Anti-goals mirror the spec: no DSL, no workflow engine, no UI, no hosted anything.
