# charta

Reference toolchain for Company as Code. **Not started — begins at M1** (after the format has
survived dogfooding on a real company repo).

Planned: one Rust core (single static binary; WASM build for embedding; thin npm wrapper) exposing

| verb | what |
|---|---|
| `charta validate` | conformance levels L0–L3 (opt-in L4 LLM-judge); JSON output, CI exit codes |
| `charta graph` / `charta query` | resolve subgraphs, backlinks, orphans, impact paths — fixed verbs, JSON out, no query language |
| `charta plan` | graph diff between two git refs, with impact set |
| `charta fmt` | canonical formatting, clean diffs |
| MCP server | resolve/validate/query (plan later) — every MCP-capable agent becomes a read runtime |

Anti-goals mirror the spec: no DSL, no workflow engine, no UI, no hosted anything.
