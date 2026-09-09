#!/usr/bin/env bash
# The release demo, per the comms rule: never show the format — show the behavior.
# Record this with asciinema/vhs against a copy of the template company.
# Beats: green validate → break a reference → validate catches it → plan shows the blast radius.
set -euo pipefail

DIR="$(mktemp -d)/example-co"
mkdir -p "$DIR"
cp -r "$(dirname "$0")/../template/." "$DIR"
cd "$DIR"
git init -q && git add -A && git -c user.name=demo -c user.email=demo@example.com commit -qm "day one"

step() { echo; echo "\$ $*"; "$@" || true; sleep 1; }

echo "# Your company, described in plain files — verified like code."
step charta validate .

echo
echo "# Someone dissolves the agent-autonomy policy…"
rm policies/agent-autonomy.md

step charta validate .

echo
echo "# …and before anything lands, plan shows what that would actually touch:"
step charta plan .

echo
echo "# Nothing in this company can break silently anymore."
