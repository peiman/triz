#!/bin/bash
# SessionStart hook — inject the triz vault's activation state (recent / hot /
# stale notes) so a new session is anchored in what the knowledgebase has been
# working on. Uses `vaultmind self` against this project's `vault/`.
#
# Fail-safe: skips silently if the binary or vault is missing. Never blocks
# session start.

if ! command -v vaultmind >/dev/null 2>&1; then
  exit 0
fi
VAULTMIND=$(command -v vaultmind)
VAULT_PATH="${CLAUDE_PROJECT_DIR:-$(pwd)}/vault"

if [ ! -d "$VAULT_PATH/.vaultmind" ]; then
  exit 0
fi

SELF=$(VAULTMIND_CALLER=triz-session-start "$VAULTMIND" self --vault "$VAULT_PATH" --limit 5 2>/dev/null)
if [ -z "$SELF" ]; then
  exit 0
fi

echo "TRIZ VAULT STATE (recent / hot / stale — your associative memory for this project; 'vaultmind ask \"<query>\" --vault vault' to recall):"
echo ""
echo "$SELF"
