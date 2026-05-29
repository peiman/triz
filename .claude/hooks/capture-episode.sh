#!/usr/bin/env bash
# SessionEnd hook — parse this session's JSONL transcript into a markdown
# "episode" under vault/episodes/. Durable per-session capture (no distillation,
# no indexing). Adapted from vaultmind's internal/hookscripts/capture-episode.sh
# to write into this project's vault and use the PATH-installed binary.
#
# Exits 0 on every path — a failed capture must never block session end.

set -eu

project_dir="${CLAUDE_PROJECT_DIR:-$(pwd)}"

# Claude Code encodes the absolute project path into the transcripts subdir name
# by replacing "/" with "-". Derive instead of hardcoding.
transcripts_subdir=$(printf '%s' "$project_dir" | sed 's|/|-|g')
transcripts_dir="$HOME/.claude/projects/$transcripts_subdir"
output_dir="$project_dir/vault/episodes"

if command -v vaultmind >/dev/null 2>&1; then
    binary=$(command -v vaultmind)
else
    exit 0
fi

payload=""
if [[ ! -t 0 ]]; then
    payload=$(cat || true)
fi

session_id=""
if [[ -n "$payload" ]] && command -v jq >/dev/null 2>&1; then
    session_id=$(printf '%s' "$payload" | jq -r '.session_id // empty' 2>/dev/null || true)
fi

transcript=""
if [[ -n "$session_id" && -f "$transcripts_dir/$session_id.jsonl" ]]; then
    transcript="$transcripts_dir/$session_id.jsonl"
elif [[ -d "$transcripts_dir" ]]; then
    transcript=$(ls -1t "$transcripts_dir"/*.jsonl 2>/dev/null | head -n1 || true)
fi

if [[ -z "$transcript" ]]; then
    exit 0
fi

mkdir -p "$output_dir"
VAULTMIND_CALLER=triz-capture-episode "$binary" episode capture "$transcript" --output-dir "$output_dir" >/dev/null 2>&1 || exit 0
exit 0
