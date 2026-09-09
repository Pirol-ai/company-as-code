# Demo

[`company/`](company/) is **Aurora Roasters** — a small fictional coffee roastery described as
Company as Code: 11 resources (roles, goals, processes, policies) you can read like any text.
Browse it to see what a described company looks like.

[`demo.sh`](demo.sh) runs the 30-second story on a throwaway copy of it:

```
bash demo.sh
```

1. `charta validate .` — green: every reference resolves.
2. The Ops role gets deleted.
3. `validate` fails loudly: processes and goals point at a role that no longer exists.
4. `charta plan .` shows the blast radius *before* the change lands.

Nothing outside a temp directory is touched. Looking for a starting point for **your** company
instead? That's [`../template/`](../template/).
