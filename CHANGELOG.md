# Changelog

## Unreleased (targeting v0.1.0)

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
