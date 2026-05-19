#!/usr/bin/env bash
# pre-pr.sh — Run `straymark charter drift` on any Charter currently
# `status: in-progress` before opening a PR. Designed to be installed via
# `straymark init --hooks` as a git pre-push hook, or invoked manually
# (e.g., from a Makefile target) when the operator prefers explicit invocation.
#
# Behavior:
#   - For each Charter in .straymark/charters/*.md whose frontmatter has
#     `status: in-progress`, run `straymark charter drift <id> --range
#     <upstream>..HEAD`. AILOG-suppression in the CLI silences alerts on
#     paths already documented as risks in the Charter's originating AILOGs.
#   - Exits 0 when there's nothing in-progress, or when all in-progress
#     Charters report clean (or AILOG-suppressed) drift.
#   - Exits 1 when at least one Charter reports unaccounted drift, with a
#     human-readable summary pointing to remediation paths.
#
# Configuration (environment):
#   STRAYMARK_UPSTREAM   git ref to compare HEAD against (default: origin/main)
#
# Why this is opt-in: per straymark-design-principles.md §6, friction is
# virtuous when the operator consents to it. We never install this hook
# automatically; adopters opt in via `straymark init --hooks` or by copying
# this file into .git/hooks/pre-push themselves.

set -euo pipefail

if ! command -v straymark >/dev/null 2>&1; then
  # straymark not installed (or not in PATH); the hook should be transparent
  # rather than blocking pushes for adopters who haven't yet opted in.
  exit 0
fi

UPSTREAM="${STRAYMARK_UPSTREAM:-origin/main}"

if [ ! -d .straymark/charters ]; then
  exit 0  # repo doesn't use Charters
fi

charters=$(grep -l '^status: in-progress' .straymark/charters/*.md 2>/dev/null || true)
if [ -z "$charters" ]; then
  exit 0  # nothing in-progress; nothing to check
fi

echo "[straymark pre-pr] Checking drift on in-progress Charters (range: $UPSTREAM..HEAD)..."
echo ""

exit_code=0
for charter_file in $charters; do
  charter_id=$(awk '/^charter_id:/ {print $2; exit}' "$charter_file" | tr -d '"')
  if [ -z "$charter_id" ]; then
    echo "  [skip] $charter_file has no charter_id in frontmatter"
    continue
  fi
  echo "  -- $charter_id ($charter_file) --"
  if ! straymark charter drift "$charter_id" --range "$UPSTREAM..HEAD"; then
    exit_code=1
  fi
  echo ""
done

if [ $exit_code -ne 0 ]; then
  echo ""
  echo "[straymark pre-pr] Drift detected on at least one Charter."
  echo "  - Address it before opening the PR (see hints above), or"
  echo "  - Document the drift in an AILOG under '## Risk' as 'R<N+1>"
  echo "    (new, not in Charter)' so the next run suppresses it, or"
  echo "  - Bypass with 'git push --no-verify' if the drift is intentional"
  echo "    and acknowledged."
fi

exit $exit_code
