---
# A runtime (kyte) owns everything under its own namespace here.
# charta must not fail on it, must not validate it, and must give it back unchanged.
api: company-as-code.org/v0
type: process
id: onboarding
title:  Customer onboarding
owner: role/ops
policies:   []
x-coos/steps:
  - id: welcome-call
    assignee: role/ops
    decision:
      question: "Enterprise plan?"
      branches: { yes: provisioning, no: self-serve }
  - id: provisioning
    assignee: role/ops
x-coos/run-mode: 'supervised'
---

Trigger: a contract is signed.
Steps: welcome call, then provisioning or self-serve. Done when the customer has logged in.

The executable form of the same steps, owned by the runtime, ignored by charta:

```x-coos/steps
welcome-call -> decision(Enterprise plan?)
  yes -> provisioning
  no  -> self-serve
```

Owned by [[role/ops]].
