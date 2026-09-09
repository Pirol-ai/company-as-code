# Show HN draft (move to comms before posting; founder edits voice)

**Title:** Show HN: Company as Code — describe your company in markdown, agents follow it

**Text:**

Every AI agent I put to work started as a stranger: it didn't know my processes, my roles, or
what it was allowed to do — so I explained, again, in every chat.

So I wrote the company down: goals, roles, processes, policies — plain markdown files with a
small typed header, in git. Files reference each other, so the company becomes a graph: this
process serves that goal, is owned by that role, is bounded by that policy.

charta is the small open toolchain that keeps it honest: `validate` (every reference must
resolve), `query` (what does nothing serve?), `plan` (what would this change touch — before it
lands), and an MCP server so any agent can read the graph as tools.

Yes, it's just markdown with a linter — that's the point. You keep your editor and your git
tooling, and can leave anytime; export is `git clone`. It's 100% compatible with Google's Open
Knowledge Format. Apache-2.0 / CC-BY-4.0.

We run our own company on it (that repo is where the spec's fields come from — nothing is in the
schema that dogfooding didn't justify), and we measured agent sessions with vs. without the
graph: [results from M2].

The interesting failures of prior art shaped it: BPMN demanded precision humans can't sustain;
wikis rot because nothing reads them daily. Agents changed that equation — the company handbook
finally has a reader that never skips a page.
