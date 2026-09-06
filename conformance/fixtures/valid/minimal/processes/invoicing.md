---
api: company-as-code.org/v0
kind: process
id: invoicing
name: Monthly invoicing
owner: role/finance
serves: [goal/cashflow]
---

Trigger: 1st of each month.
Steps: collect billable items; draft invoices; owner reviews; send; log.
Done when: all invoices sent and logged; exceptions filed to [[role/finance]].
