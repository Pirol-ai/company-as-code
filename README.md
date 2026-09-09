# Company as Code

An open standard for describing a company — its processes, roles, goals, products, and policies —
as versioned plain text: markdown files with typed frontmatter, connected by references into a
graph. Verified like code. Readable by people. Executed by agents.

It borrows Terraform's execution model, not its syntax: the description is a desired state, changes
are reviewable diffs (plan before apply), and agents — the actuator every earlier attempt at this
lacked — execute described processes and detect drift between the described and the lived company.
The semantics stay natural language; only the skeleton (types, references, policies) is validated.

**Status: private dogfooding.** The spec is being extracted from real usage on a real company
repo, not designed up front. It goes public together with a working validator and a demo — never
before.

## Quickstart

```
# after the first release:  brew install pirol-ai/tap/charta  ·  npm i -g @pirol/charta
# until then, build from source:
cargo build --release --manifest-path charta/Cargo.toml

cp -r template my-company && cd my-company
charta validate .          # green — every reference resolves
charta query orphans .     # what serves nothing?
charta plan .              # what would your uncommitted change touch?
charta mcp .               # give any MCP-capable agent the company graph
```

Then make [`template/`](template/) yours: describe what a new coworker would need on day one —
every agent session is that coworker.

**See it break first:** [`demo/`](demo/) holds a small described company and a 30-second story —
`bash demo/demo.sh` — where a role vanishes, `validate` catches it, and `plan` shows the blast
radius before anything lands.

## Layout

| Path | What |
|---|---|
| [`spec/`](spec/) | The format: envelope, kinds, references, conformance levels. Working draft. |
| [`conformance/`](conformance/) | Executable fixtures — the suite, not the prose, is the standard. |
| [`charta/`](charta/) | Reference toolchain (`validate` · `graph` · `query` · `plan` · MCP server). Not started; begins at M1. |

## OKF compatible

Company as Code is **100% compatible with Google's
[Open Knowledge Format (OKF)](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)**:
every resource is a conformant OKF concept (`type`, `title`, `description`, `tags` carry OKF
semantics; unknown keys are preserved in both directions), and OKF concepts live untouched in the
same tree — charta only validates files that opt in via the `api` key. Company as Code is a strict
*profile* of OKF: it adds identity (`id`), `type/id` addressing, typed references with mandatory
resolution, and per-type required fields on top of OKF's tolerant substrate.

## Design tenets

1. **Prose primary, schema minimal.** Process semantics are one page of natural language; structure
   lives in frontmatter. Every added field is a tax on adoption.
2. **Validate the skeleton, never the prose.** Well-formedness → referential integrity → per-kind
   schemas → opt-in policies. An optional LLM-judge level exists for plausibility; it is the only
   level that touches a model.
3. **Spec ≠ validator ≠ runtime.** The description belongs to the company, not to any tool (OCI
   logic). The on-disk format is the interchange format: export is `git clone`.
4. **Ignore and preserve.** Unknown kinds and fields are kept verbatim; round-trip idempotence is a
   conformance requirement, not a courtesy.
5. **No DSL.** Markdown + YAML frontmatter; no invented query language. Fixed verbs, JSON out;
   agents compose.

## Licensing

Toolchain and fixtures: Apache-2.0 (see [LICENSE](LICENSE)). Specification text (`spec/`):
CC-BY-4.0 (full text added before publication).
