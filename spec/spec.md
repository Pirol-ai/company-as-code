# Company as Code — specification

> **Working draft, v0.** Nothing in this document is normative yet. The spec is extracted from
> dogfooding (M0–M3), and the conformance suite in [`../conformance/`](../conformance/) is its
> authoritative form. Prose here describes intent; where prose and fixtures disagree, fixtures win.

## 1. Artifact

A company description is a directory tree under version control:

- one **manifest** at the root: `company.yaml`
- one **resource per file**: markdown with a YAML frontmatter envelope
- resources addressed as `<kind>/<id>` (e.g. `role/finance`), independent of file location;
  by convention files live in per-kind directories (`processes/invoicing.md`)

The on-disk format **is** the interchange format. There is no export step; portability means
`git clone` plus a green `validate`.

## 2. Manifest

```yaml
# company.yaml
api: companyascode.org/v0
name: Example GmbH
extensions: []          # namespaces of extension kinds in use, e.g. ["x-coos"]
```

## 3. Envelope

Every resource file begins with YAML frontmatter:

```markdown
---
api: companyascode.org/v0
kind: process
id: invoicing
name: Monthly invoicing
owner: role/finance
serves: [goal/cashflow]
policies: [policy/four-eyes-payments]
---

Trigger: 1st of each month.
Steps: … (plain prose, one page max; agents read this)
Done when: all invoices sent and logged; exceptions filed to role/finance.
```

Required for every kind: `api`, `kind`, `id`. `id` is unique per kind, kebab-case.
Everything after the frontmatter is **prose**: natural language for agents and humans. The prose
is never schema-validated.

## 4. Core kinds (v0 candidates)

Extracted, not designed: this list holds only what dogfooding has justified. Per-kind required
fields are defined by the fixtures in `conformance/`.

| kind | one line | typed refs (candidates) |
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
- **Prose links** use `[[kind/id]]` wiki syntax inside the body. They are indexed and warned on,
  never errors.

## 6. Conformance levels

| Level | Checks |
|---|---|
| **L0** | parseable: valid YAML frontmatter, required envelope fields, unique ids |
| **L1** | referential integrity: every typed ref resolves; orphan report |
| **L2** | per-kind schemas: required fields per kind (as fixed by fixtures) |
| **L3** | policy rules (opt-in, pluggable): e.g. "every process has an owner" |
| **L4** | optional LLM-judge plausibility (only level touching a model) |

A validator conforms at level N if it passes all fixtures for levels ≤ N.

## 7. Extensions & evolution

- Extension kinds and fields are namespaced (`x-coos/approval-card`, `x-firm/opportunity`).
- **Ignore and preserve:** an implementation MUST NOT fail on unknown kinds or fields and MUST
  preserve them verbatim on write.
- **Round-trip idempotence:** read → write → read must be byte-stable for conforming input. This
  is a conformance requirement.
- `api` versions the envelope, not your company: bumps are rare and migration is a documented,
  mechanical step.

## 8. Non-goals (v0)

No DSL. No query language. No workflow engine. No UI. No hosted service. No org-chart editor.
