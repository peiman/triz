#!/bin/bash
# UserPromptSubmit hook — query the triz vault for note pointers relevant to
# the user's prompt and inject them as context before the model responds.
#
# Adapted from vaultmind's internal/hookscripts/vault-recall.sh, repointed at
# this project's `vault/` knowledgebase and using the PATH-installed binary
# (no source rebuild, no persona). Low-noise and fail-safe by design: it skips
# silently on short prompts, missing binary, missing vault, or empty results —
# a broken recall must never block the user's message.

HOOK_INPUT=$(cat)
PROMPT=$(echo "$HOOK_INPUT" | python3 -c "import json,sys; print(json.load(sys.stdin).get('prompt',''))" 2>/dev/null || echo "")

# Single-word / command-style messages aren't worth a vault query.
if [ -z "$PROMPT" ] || [ "${#PROMPT}" -lt 12 ]; then
  exit 0
fi

if ! command -v vaultmind >/dev/null 2>&1; then
  exit 0
fi
VAULTMIND=$(command -v vaultmind)
VAULT_PATH="${CLAUDE_PROJECT_DIR:-$(pwd)}/vault"

if [ ! -d "$VAULT_PATH/.vaultmind" ]; then
  exit 0
fi

LOG_DIR="${HOME}/.vaultmind/triz-recall"
mkdir -p "$LOG_DIR" 2>/dev/null
TIMESTAMP=$(date +%Y%m%dT%H%M%S)

ASK_ERR=$(mktemp -t triz-recall-err.XXXXXX)
POINTERS=$(VAULTMIND_CALLER=triz-recall-hook "$VAULTMIND" ask "$PROMPT" \
  --vault "$VAULT_PATH" \
  --max-items 3 \
  --budget 1500 \
  --pointers-only 2>"$ASK_ERR")
ASK_STATUS=$?

if [ "$ASK_STATUS" != "0" ] || [ -z "$POINTERS" ]; then
  printf '{"timestamp":"%s","prompt_len":%d,"ask_status":%d,"injection":false}\n' \
    "$TIMESTAMP" "${#PROMPT}" "$ASK_STATUS" \
    > "$LOG_DIR/${TIMESTAMP}-skip.json" 2>/dev/null
  rm -f "$ASK_ERR"
  exit 0
fi
rm -f "$ASK_ERR"

echo "TRIZ VAULT POINTERS related to your message (run 'vaultmind note get <id> --vault vault' to read a body):"
echo ""
echo "$POINTERS"

printf '{"timestamp":"%s","prompt_len":%d,"ask_status":0,"injection":true,"pointer_chars":%d}\n' \
  "$TIMESTAMP" "${#PROMPT}" "${#POINTERS}" \
  > "$LOG_DIR/${TIMESTAMP}-inject.json" 2>/dev/null
