# Conformance suite

**The suite, not the spec prose, is the standard.** A validator conforms at level N when it
produces the expected result for every fixture at levels ≤ N (see spec §6 for levels).

## Layout

```
fixtures/
  valid/<name>/       a complete company description that must validate green
  invalid/<name>/     a description with exactly one class of defect
                      + expected.json: the minimal expected findings
```

`expected.json` (draft shape — will harden with charta M1):

```json
{ "level": "L1", "errors": [ { "code": "unresolved-ref", "resource": "process/invoicing", "field": "owner" } ] }
```

## Rules for fixtures

- One defect class per invalid fixture; name the directory after the defect.
- Fixtures are the place where per-kind required fields get fixed — add a fixture, and the spec
  table follows, never the reverse.
- Round-trip fixtures (read → write → byte-identical) live under `valid/` and are mandatory from M1.
