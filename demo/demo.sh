#!/usr/bin/env bash
# The release demo, per the comms rule: never show the format — show the behavior.
# Runs on a throwaway copy of ./company (Aurora Roasters); touches nothing else.
# Recorded with asciinema for the README video (see README.md here).
# Beats: green validate → the Ops role gets deleted → validate catches it → plan shows the impact.
set -euo pipefail

SRC="$(cd "$(dirname "$0")/company" && pwd)"
DIR="$(mktemp -d)/aurora-roasters"
mkdir -p "$DIR"
cp -r "$SRC/." "$DIR"
cd "$DIR"
git init -q && git add -A && git -c user.name=demo -c user.email=demo@example.com commit -qm "day one"

PAUSE="${DEMO_PAUSE:-1}"
CHAR="${DEMO_CHAR:-0}"   # per-character typing delay; 0 = instant (interactive use)

type_out() { # prints a line character by character, so the cursor visibly types
  local s="$1" i
  for ((i = 0; i < ${#s}; i++)); do
    printf '%s' "${s:$i:1}"
    [ "$CHAR" != "0" ] && sleep "$CHAR"
  done
  printf '\n'
}

say() { type_out "$1"; sleep 0.6; }

step() {
  echo
  type_out "\$ $*"
  sleep 0.4
  "$@" || true
  sleep "$PAUSE"
}

say "# A small coffee roastery, described in plain files — verified like code."
step charta validate .

echo
say "# Sam quits. Someone deletes the Ops role…"
step rm roles/ops.md
step charta validate .

echo
say "# …and before anything lands, plan shows what that would actually touch:"
step charta plan .

echo
say "# A goal unowned, three processes ownerless — visible before the change, not weeks after."
say "# Nothing in this company can break silently anymore."
