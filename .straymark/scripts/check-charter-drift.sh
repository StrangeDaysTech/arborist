#!/usr/bin/env bash
# check-charter-drift.sh — flag declared-but-not-modified files at Charter close.
#
# Ported from Sentinel scripts/check-plan-drift.sh (validated empirically with
# zero false positives across PLAN-05 retrospective + PLAN-06 prospective).
# StrayMark surface: Charters live at .straymark/charters/NN-slug.md (this is
# the rebrand of Sentinel's docs/plans/NN-*.md layout).
#
# This script catches drifts of OMISSION (file declared in Charter, never touched)
# and SCOPE EXPANSION (file modified but not declared). Both kinds were caught by
# external auditors in PLAN-05 (F4 evaluator_test gap, F5 architectural hallucination,
# modified extras like wire_gen.go).
#
# Usage:
#   .straymark/scripts/check-charter-drift.sh <charter-file> <git-range>
#
# Examples:
#   .straymark/scripts/check-charter-drift.sh .straymark/charters/01-format-v4.md fd65e87..473f6e0
#   .straymark/scripts/check-charter-drift.sh .straymark/charters/01-format-v4.md HEAD~1..HEAD
#
# Exit codes:
#   0 — no drift detected (or only documented out-of-scope extras)
#   1 — drift found that needs attention (declared-but-not-modified or
#       undocumented scope expansion)
#   2 — usage error
#
# Output is human-readable; the agent reading this should take action by either
# completing the missing work or documenting the drift in the AILOG under
# "## Risk" as `R<N+1> (new, not in Charter)`.
#
# Note: `straymark charter drift` (cli-3.7.0+) wraps this script and adds
# AILOG-awareness — it suppresses alerts on paths already documented as R<N>
# in any AILOG referenced by the Charter's `originating_ailogs` frontmatter.
# Invoking this script directly is fine but you'll see those documented-drift
# alerts as well; use the CLI for the de-noised view.

set -euo pipefail

if [ $# -lt 2 ]; then
  echo "Usage: $0 <charter-file> <git-range>" >&2
  echo "Example: $0 .straymark/charters/01-format-v4.md fd65e87..473f6e0" >&2
  exit 2
fi

CHARTER_FILE="$1"
GIT_RANGE="$2"

if [ ! -f "$CHARTER_FILE" ]; then
  echo "ERROR: Charter file not found: $CHARTER_FILE" >&2
  exit 2
fi

# Extract declared files from the "## Files to modify" / "## Archivos a modificar"
# / "## 要修改的文件" section. Reads the section delimited by the heading and the
# next `## ` heading. For markdown table rows (`| col1 | col2 | ...`) the script
# extracts only column 1 — the "File" column — to avoid false positives from
# backtick-quoted path references in the "Change" column (F3 of the
# CHARTER-02 telemetry, fw-4.6.0). Non-table content (bullets, prose) is
# preserved as-is for backward compatibility with adopters who use bullet lists
# instead of tables.
declared=$(awk '
    /^## (Files to modify|Archivos a modificar|要修改的文件)/ { in_table=1; next }
    in_table && /^## / { in_table=0 }
    in_table {
        if (/^\|/) {
            # Markdown table row. After splitting on |, cols[1] is empty (text
            # before the leading |), cols[2] is the first column, etc.
            n = split($0, cols, "|")
            if (n >= 2) {
                col1 = cols[2]
                # Trim whitespace.
                sub(/^[ \t]+/, "", col1)
                sub(/[ \t]+$/, "", col1)
                # Skip separator row (only dashes/spaces/colons) and header
                # row variants. The grep below filters non-paths anyway, but
                # silencing these here keeps awk output clean.
                if (col1 ~ /^[-: ]+$/) next
                if (col1 ~ /^[*]*[Ff]ile[*]*$/) next
                if (col1 ~ /^[*]*[Aa]rchivo[*]*$/) next
                if (col1 ~ /^[*]*文件[*]*$/) next
                print col1
            }
        } else {
            # Non-table content (bullets, prose) — preserve current behavior.
            print
        }
    }
' "$CHARTER_FILE" | grep -oP '`\K[^`]+(?=`)' | grep -E '\.(go|sql|yaml|yml|md|sh|ts|tsx|js|jsx|rs|py|java|kt|rb|cs|cpp|c|h|hpp|swift|toml|json|tf)$|\.straymark/' | sort -u)

if [ -z "$declared" ]; then
  echo "WARN: no files extracted from §Files to modify in $CHARTER_FILE" >&2
  echo "  Either the section is missing, the table format is unusual, or the" >&2
  echo "  declared paths don't have recognized extensions. Script can't help — exit clean." >&2
  exit 0
fi

# Get files actually modified in the range.
modified=$(git diff --name-only "$GIT_RANGE" 2>/dev/null | sort -u)

if [ -z "$modified" ]; then
  echo "WARN: no files modified in range $GIT_RANGE — Charter may not have executed." >&2
  exit 0
fi

# Set diff: declared but not modified.
#
# Two wildcard forms are supported in declared paths:
#   1. Ellipsis form `prefix...suffix` — any modified file with that prefix
#      satisfies the wildcard. Used historically in AILOG references like
#      `.straymark/07-ai-audit/agent-logs/AILOG-...md`.
#   2. Glob form `prefix*suffix` (added fw-4.6.2): any modified file whose
#      path matches the glob (`*` → `.*` regex) satisfies the wildcard.
#      Used for bulk Charter declarations like `AILOG-*.md`. Reported as the
#      new finding in CHARTER-04 of issue #81.
declared_omitted=""
while IFS= read -r decl; do
  # 1. Ellipsis wildcard.
  if [[ "$decl" == *...* ]]; then
    prefix="${decl%...*}"
    if echo "$modified" | grep -q "^${prefix}"; then
      continue  # any match satisfies the wildcard.
    fi
    declared_omitted+="$decl"$'\n'
    continue
  fi
  # 2. Glob wildcard. Convert `*` → `.*` and escape `.` for the regex match.
  if [[ "$decl" == *\** ]]; then
    glob_re=$(printf '%s' "$decl" | sed 's/\./\\./g; s/\*/.*/g')
    if echo "$modified" | grep -qE "^${glob_re}$"; then
      continue
    fi
    declared_omitted+="$decl"$'\n'
    continue
  fi
  # 3. Literal path.
  if ! echo "$modified" | grep -qx "$decl"; then
    declared_omitted+="$decl"$'\n'
  fi
done <<< "$declared"
declared_omitted=$(echo "$declared_omitted" | sed '/^$/d')

# Set diff: modified but not declared (scope expansion).
modified_extra=""
while IFS= read -r mod; do
  # Allow Charter-doc and AILOG paths through without alarm — those are always
  # in scope when the Charter itself or the AILOG of execution gets touched.
  if [[ "$mod" == .straymark/charters/* || "$mod" == .straymark/07-ai-audit/* ]]; then
    continue
  fi
  if ! echo "$declared" | grep -qx "$mod"; then
    # Also allow if the declared list has a wildcard prefix that matches.
    # Recognizes both forms: `prefix...suffix` and `prefix*suffix` (fw-4.6.2).
    matched_wildcard=0
    while IFS= read -r decl; do
      if [[ "$decl" == *...* ]]; then
        prefix="${decl%...*}"
        if [[ "$mod" == ${prefix}* ]]; then
          matched_wildcard=1
          break
        fi
      elif [[ "$decl" == *\** ]]; then
        glob_re=$(printf '%s' "$decl" | sed 's/\./\\./g; s/\*/.*/g')
        if echo "$mod" | grep -qE "^${glob_re}$"; then
          matched_wildcard=1
          break
        fi
      fi
    done <<< "$declared"
    if [ $matched_wildcard -eq 0 ]; then
      modified_extra+="$mod"$'\n'
    fi
  fi
done <<< "$modified"
modified_extra=$(echo "$modified_extra" | sed '/^$/d')

# --- Report ---

echo "=== Charter drift check ==="
echo "  Charter: $CHARTER_FILE"
echo "  Range:   $GIT_RANGE"
echo "  Declared: $(echo "$declared" | wc -l) files"
echo "  Modified: $(echo "$modified" | wc -l) files"
echo ""

exit_code=0

if [ -n "$declared_omitted" ]; then
  echo "WARNING: Declared in Charter but NOT modified ($(echo "$declared_omitted" | wc -l) files):"
  echo "$declared_omitted" | sed 's/^/  - /'
  echo ""
  echo "  Action: either complete the work, or document in AILOG under '## Risk'"
  echo "  as 'R<N+1> (new, not in Charter)' explaining why this file did not need"
  echo "  changes (Charter was wrong, scope simplified, etc.)."
  echo ""
  exit_code=1
fi

if [ -n "$modified_extra" ]; then
  echo "INFO: Modified but NOT declared ($(echo "$modified_extra" | wc -l) files, scope expansion):"
  echo "$modified_extra" | sed 's/^/  - /'
  echo ""
  echo "  Action: if intentional, document the scope expansion in AILOG."
  echo "  Common reasons: mock updates after interface change, generated"
  echo "  files (e.g. wire_gen.go), pre-existing drift fix needed to unblock work."
  echo ""
  # Scope expansion is informational, not a hard failure — exit_code stays.
fi

if [ -z "$declared_omitted" ] && [ -z "$modified_extra" ]; then
  echo "OK No drift detected. Charter and execution are in sync."
fi

exit $exit_code
