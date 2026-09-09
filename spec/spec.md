# Company as Code — specification

> **Working draft, v0 — freezes at the first public release.** The spec is extracted from
> dogfooding (M0–M3), and the conformance suite in [`../conformance/`](../conformance/) is its
> authoritative form. Prose here describes intent; where prose and fixtures disagree, fixtures
> win. Spec text: CC-BY-4.0 ([LICENSE](LICENSE)); reference implementation: Apache-2.0.

## 1. Artifact

A company description is a directory tree under version control:

- one **manifest** at the root: `company.yaml`
- one **resource per file**: markdown with a YAML frontmatter envelope
- resources addressed as `<type>/<id>` (e.g. `role/finance`), independent of file location;
  by convention files live in per-type directories (`processes/invoicing.md`)

The on-disk format **is** the interchange format. There is no export step; portability means
`git clone` plus a green `validate`.

## 2. Manifest

```yaml
# company.yaml
api: company-as-code.org/v0
name: Example GmbH
extensions: []          # namespaces of extension types in use, e.g. ["x-coos"]
```

## 3. Envelope

Every resource file begins with YAML frontmatter:

```markdown
---
api: company-as-code.org/v0
type: process
id: invoicing
title: Monthly invoicing
owner: role/finance
serves: [goal/cashflow]
policies: [policy/four-eyes-payments]
---

Trigger: 1st of each month.
Steps: … (plain prose, one page max; agents read this)
Done when: all invoices sent and logged; exceptions filed to role/finance.
```

Required for every resource: `api`, `type`, `id`. `id` is unique per type, kebab-case. `type`,
`title`, `description`, and `tags` carry OKF semantics (see §8) — there is deliberately no
separate `kind` field. Everything after the frontmatter is **prose**: natural language for agents
and humans. The prose is never schema-validated.

## 4. Core types (v0 candidates)

Extracted, not designed: this list holds only what dogfooding has justified. Per-type required
fields are defined by the fixtures in `conformance/`.

| type | one line | typed refs (candidates) |
|---|---|---|
| `company` | identity, purpose | — |
| `goal` | desired outcome with a metric | `owner` → role |
| `role` | responsibility bundle; held by a person or an agent | — |
| `process` | recurring way of working | `owner` → role, `serves` → goal[], `policies` → policy[] |
| `policy` | constraint; machine-checkable where possible | — |
| `product` | thing the company offers | `owner` → role |
| `decision` | dated, immutable record of a choice | free refs |
| `module` | parameterized template instantiating the above | — |

Agent autonomy boundaries (what an agent may do alone vs. what needs approval) are expressed in
`role` resources for agent roles — they are description, and runtimes enforce them.

## 5. References

- **Typed refs** live in frontmatter fields (`owner: role/finance`). They MUST resolve; an
  unresolved typed ref is an L1 error.
- **Prose links** use `[[type/id]]` wiki syntax inside the body. They are indexed and warned on,
  never errors.

## 6. Conformance levels

| Level | Checks |
|---|---|
| **L0** | parseable: valid YAML frontmatter, required envelope fields, unique ids |
| **L1** | referential integrity: every typed ref resolves; orphan report |
| **L2** | per-type schemas: required fields per type (as fixed by fixtures) |
| **L3** | policy rules (opt-in, pluggable): e.g. "every process has an owner" |
| **L4** | optional LLM-judge plausibility (only level touching a model) |

A validator conforms at level N if it passes all fixtures for levels ≤ N.

## 7. Extensions & evolution

- Extension types and fields are namespaced (`x-coos/approval-card`, `x-firm/opportunity`).
- **Ignore and preserve:** an implementation MUST NOT fail on unknown types or fields and MUST
  preserve them verbatim on write.
- **Round-trip idempotence:** read → write → read must be byte-stable for conforming input. This
  is a conformance requirement.
- `api` versions the envelope, not your company: bumps are rare and migration is a documented,
  mechanical step.

## 8. OKF compatibility

Company as Code is a **strict profile of Google's
[Open Knowledge Format (OKF)](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)**:

- Every resource is a conformant OKF concept — `type` is OKF's one required key; `title`,
  `description`, and `tags` carry OKF semantics; unknown keys are preserved in both directions.
- What charta adds on top is the strict layer OKF deliberately leaves out: identity (`id`),
  addressing (`type/id`), typed references with mandatory resolution, per-type required fields,
  and the validator.
- OKF concepts (frontmatter without our `api` key) coexist untouched in the same tree; charta
  ignores them.
- OKF's trust and lifecycle families (`status`, `generated`, `verified`, `stale_after`, `sources`,
  with the `human:`/`process:` actor convention) are planned as optional envelope families —
  adopted with OKF semantics, not reinvented.

## 9. Non-goals (v0)

No DSL. No query language. No workflow engine. No UI. No hosted service. No org-chart editor.
