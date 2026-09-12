# Changelog

## v0.3.0

- `charta init` — creates `company.yaml` in the current directory (name defaults to the directory
  name). No repository needed to start: install, `init`, describe.

## v0.2.0

- `charta graph --format mermaid` — the company graph as a Mermaid flowchart; GitHub renders it
  natively in markdown (typed references solid, prose links dotted)
- `charta graph --format dot` — Graphviz DOT output for every other visualization tool
- README: the example company embedded as a rendered graph

## v0.1.0

First public release of the Company as Code spec (working draft, frozen as v0 at this release)
and the charta toolchain:

- `charta validate` — conformance levels L0 (well-formed) · L1 (referential integrity) · L2
  (per-type required fields), human and `--json` output, CI exit codes
- `charta graph` / `charta query` — nodes + edges, orphans, backlinks
- `charta plan` — committed baseline vs. working tree: added/changed/removed + impact set
- `charta mcp` — stdio MCP server (`charta_validate`, `charta_query`, `charta_resolve`)
- Conformance suite (fixtures are the authoritative spec)
- OKF compatibility: every resource is a conformant Open Knowledge Format concept
- `template/` starter company; Homebrew tap + npm distribution via the release workflow
