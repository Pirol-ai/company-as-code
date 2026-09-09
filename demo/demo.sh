#!/usr/bin/env bash
# The release demo, per the comms rule: never show the format — show the behavior.
# Runs on a throwaway copy of ./company (Aurora Roasters, 12 resources); touches nothing else.
# Record with asciinema/vhs for the README video.
# Beats: green validate → a role vanishes → validate catches it → plan shows the blast radius.
set -euo pipefail

SRC="$(cd "$(dirname "$0")/company" && pwd)"
DIR="$(mktemp -d)/aurora-roasters"
mkdir -p "$DIR"
cp -r "$SRC/." "$DIR"
cd "$DIR"
git init -q && git add -A && git -c user.name=demo -c user.email=demo@example.com commit -qm "day one"

step() { echo; echo "\$ $*"; "$@" || true; sleep "${DEMO_PAUSE:-1}"; }

echo "# A small coffee roastery, described in plain files — verified like code."
step charta validate .

echo
echo "# Sam quits. Someone deletes the Ops role…"
rm roles/ops.md

step charta validate .

echo
echo "# …and before anything lands, plan shows what that would actually touch:"
step charta plan .

echo
echo "# A goal unowned, three processes ownerless — visible before the change, not weeks after."
echo "# Nothing in this company can break silently anymore."
