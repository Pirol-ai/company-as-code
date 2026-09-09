# @pirol/charta

Reference toolchain for [Company as Code](https://github.com/Pirol-ai/company-as-code): describe
your company as plain markdown files with typed frontmatter — validated like code, readable by
agents.

```
npm i -g @pirol/charta

charta validate .          # is the description intact?
charta query orphans .     # what does nothing reference?
charta plan .              # what would this change touch?
charta mcp .               # serve the company graph to any MCP-capable agent
```

This package installs the prebuilt binary for your platform via optional dependencies
(macOS arm64/x64, Linux x64, Windows x64). Apache-2.0.
