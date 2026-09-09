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

## Regenerating the video

`demo.gif` / `demo.mp4` (embedded in the root README) are rendered from a recording of this
script. To regenerate after a change:

```
DEMO_PAUSE=3 asciinema rec --overwrite --window-size 112x26 -c "bash demo.sh" demo.cast
agg --theme 000000,e6e6e6,000000,ff6b60,2fd158,e8a344,57aaf7,c792ea,56b6c2,e6e6e6,5c6370,ff6b60,2fd158,e8a344,57aaf7,c792ea,56b6c2,ffffff --font-size 20 demo.cast demo.gif
ffmpeg -y -i demo.gif -movflags faststart -pix_fmt yuv420p -vf "scale=trunc(iw/2)*2:trunc(ih/2)*2" demo.mp4
```

(`brew install asciinema agg ffmpeg`. The theme is pure black with the site's amber as accent.)
