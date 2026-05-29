#!/bin/bash
# PreToolUse hook on Read — when the agent reads a triz vault note, fire
# RecordNoteAccess via `vaultmind note get` (so activation/`self` stays
# accurate) and inject a one-line header naming the canonical retrieval
# command. Read still proceeds; this hook never blocks.
#
# Adapted from vaultmind's internal/hookscripts/vault-track-read.sh, with the
# default path pattern repointed at this project's `vault/` directory and the
# PATH-installed binary.

set -uo pipefail

HOOK_INPUT=$(cat)

TOOL_NAME=$(echo "$HOOK_INPUT" | python3 -c "import json,sys; print(json.load(sys.stdin).get('tool_name',''))" 2>/dev/null || echo "")
FILE_PATH=$(echo "$HOOK_INPUT" | python3 -c "import json,sys; print(json.load(sys.stdin).get('tool_input',{}).get('file_path',''))" 2>/dev/null || echo "")

if [ "$TOOL_NAME" != "Read" ] || [ -z "$FILE_PATH" ]; then
  exit 0
fi

# Fast-skip Reads on files that obviously aren't triz vault notes.
shopt -s extglob
PATTERN="${VAULT_PATH_PATTERN:-*/vault/*.md}"
case "$FILE_PATH" in
  $PATTERN) ;;
  *) exit 0 ;;
esac

# Walk up to the vault root (the dir containing .vaultmind/).
VAULT_ROOT=$(dirname "$FILE_PATH")
while [ "$VAULT_ROOT" != "/" ] && [ "$VAULT_ROOT" != "." ]; do
  if [ -d "$VAULT_ROOT/.vaultmind" ]; then
    break
  fi
  VAULT_ROOT=$(dirname "$VAULT_ROOT")
done
if [ ! -d "$VAULT_ROOT/.vaultmind" ]; then
  exit 0
fi

if ! command -v vaultmind >/dev/null 2>&1; then
  exit 0
fi
VAULTMIND=$(command -v vaultmind)

REL_PATH="${FILE_PATH#"$VAULT_ROOT"/}"

LOG_DIR="${HOME}/.vaultmind/triz-track-read"
mkdir -p "$LOG_DIR" 2>/dev/null
TIMESTAMP=$(date +%Y%m%dT%H%M%S)

# Synchronous note get with a 3s timeout (PreToolUse blocks until exit;
# fail fast on a hung index). macOS lacks `timeout` — try gtimeout, else none.
TIMEOUT_CMD=""
if command -v timeout >/dev/null 2>&1; then
  TIMEOUT_CMD="timeout 3"
elif command -v gtimeout >/dev/null 2>&1; then
  TIMEOUT_CMD="gtimeout 3"
fi
NOTE_OUTPUT=$(VAULTMIND_CALLER=triz-track-read $TIMEOUT_CMD "$VAULTMIND" note get "$REL_PATH" --vault "$VAULT_ROOT" 2>/dev/null)
NOTE_STATUS=$?

NOTE_RESOLVED=1
if [ "$NOTE_STATUS" != "0" ] || [ -z "$NOTE_OUTPUT" ]; then
  NOTE_RESOLVED=0
elif echo "$NOTE_OUTPUT" | head -1 | grep -q "^No note found"; then
  NOTE_RESOLVED=0
fi

if [ "$NOTE_RESOLVED" = "0" ]; then
  printf '{"timestamp":"%s","file_path":"%s","note_status":%d,"injected":false}\n' \
    "$TIMESTAMP" "$FILE_PATH" "$NOTE_STATUS" \
    > "$LOG_DIR/${TIMESTAMP}-skip.json" 2>/dev/null
  exit 0
fi

HEADER="[vault-track-read] Read on triz vault note \"$REL_PATH\" — access recorded. Canonical retrieval next time: vaultmind note get $REL_PATH --vault vault (or by id: vaultmind note get <id> --vault vault)."

python3 -c "
import json, sys
print(json.dumps({
    'hookSpecificOutput': {
        'hookEventName': 'PreToolUse',
        'additionalContext': sys.argv[1],
    }
}))
" "$HEADER"

printf '{"timestamp":"%s","file_path":"%s","injected":true}\n' \
  "$TIMESTAMP" "$FILE_PATH" \
  > "$LOG_DIR/${TIMESTAMP}-inject.json" 2>/dev/null

exit 0
