# Company as Code

**Describe your company once — every AI agent works by your rules.**

How your company works lives in your head, in chat threads, and in wiki pages nobody updates.
That was survivable when a new coworker arrived once a quarter. Now every AI agent session is a
new coworker — one that shows up a hundred times a day, knowing nothing. Company as Code is an
open standard for writing it down once: goals, roles, processes, and policies as plain markdown
files in git — readable by people, followed by agents, and verified like code.

<!-- demo video lands here after the measurement runs -->

## What it looks like

One file per resource. A small typed header, then normal language:

```markdown
---
api: company-as-code.org/v0
type: process
id: invoicing
title: Monthly invoicing
owner: role/ops
serves: [goal/wholesale-growth]
policies: [policy/four-eyes-payments]
---

Trigger: 1st of each month. Steps: agent drafts invoices from delivery notes;
ops reviews; send; log. Done when: all invoices sent and logged.
```

The header is the graph: this process is *owned by* a role, *serves* a goal, is *bounded by* a
policy — and none of those references can break silently, because the validator refuses them.
The body is for the readers, human and machine. Files without the `api:` key are none of our
business: your notes and docs live in the same repo, untouched.

## Install

```
brew install pirol-ai/tap/charta
```

```
npm i -g @pirol/charta
```

(macOS, Linux, Windows; binaries on the [releases page](https://github.com/Pirol-ai/company-as-code/releases).
Building from source: `cargo build --release --manifest-path charta/Cargo.toml`.)

## Try it in 30 seconds

Watch a company break — loudly instead of silently:

```
git clone https://github.com/Pirol-ai/company-as-code && cd company-as-code
bash demo/demo.sh
```

A role vanishes from a small roastery ([`demo/company/`](demo/company/) — browse it, it's just
files); `charta validate` catches every dangling reference, and `charta plan` shows the blast
radius *before* the change lands.

Then start your own from the starter in [`template/`](template/):

```
cp -r template my-company && cd my-company
charta validate .          # green — every reference resolves
charta query orphans .     # what does nothing serve?
charta plan .              # what would your uncommitted change touch?
charta mcp .               # serve the company graph to any MCP-capable agent
```

Describe what a new coworker would need on day one — every agent session is that coworker. Grow
it from incidents, not ambition.

## The toolchain

`charta` is one small binary (Apache-2.0): `validate` (three conformance levels: well-formed →
references resolve → required fields), `graph` and `query` (orphans, backlinks — JSON out, no
query language), `plan` (diff against a git baseline, with impact set), and `mcp` (an MCP server
exposing the graph as tools, so agents read the company the way you do). Details in
[`charta/`](charta/).

## Layout

| Path | What |
|---|---|
| [`spec/`](spec/) | The format: purpose, envelope, types, references, conformance levels. Working draft; freezes at 1.0. |
| [`conformance/`](conformance/) | Executable fixtures — the suite, not the prose, is the standard. |
| [`charta/`](charta/) | The reference toolchain (Rust). |
| [`template/`](template/) | Minimal starter company. |
| [`demo/`](demo/) | A browsable example company and the 30-second demo. |

## OKF compatible

Company as Code is **100% compatible with Google's
[Open Knowledge Format](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)** —
a strict *profile* of it: every resource is a conformant OKF concept (`type`, `title`, `description`, `tags` carry OKF
semantics; unknown keys are preserved both ways), and OKF concepts coexist untouched in the same
tree. On top of OKF's tolerant substrate it adds identity (`id`), `type/id` addressing, typed
references with mandatory resolution, and per-type required fields.

## Design tenets

1. **Prose primary, schema minimal.** Semantics are one page of natural language; structure lives
   in the header. Every added field is a tax on adoption.
2. **Validate the skeleton, never the prose.** Formal notations died of the precision they
   demanded; agents tolerate ambiguity. Only the references and required fields are enforced.
3. **The description belongs to the company, not to any tool.** The on-disk format is the
   interchange format: export is `git clone`.
4. **Ignore and preserve.** Unknown types and fields are kept verbatim; round-trip idempotence is
   a conformance requirement.
5. **No DSL.** Markdown + YAML header; no invented query language. Fixed verbs, JSON out; agents
   compose.

## License

Toolchain and fixtures: Apache-2.0 ([LICENSE](LICENSE)). Specification text: CC-BY-4.0
([spec/LICENSE](spec/LICENSE)). An open standard, created and stewarded by
[Pirol Labs](https://pirol.ai).
