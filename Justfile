# Project task runner
# Framework tasks imported from .ckeletin/Justfile

import '.ckeletin/Justfile'

binary_name := "triz"

# Single gateway — all checks (CKSPEC-ENF-001)
check: ckeletin-check test ckeletin-health
    @echo "All checks passed."

# Run tests
test:
    cargo nextest run --workspace 2>/dev/null || cargo test --workspace

# Run tests with coverage (CKSPEC-TEST-002: 85% minimum)
coverage:
    cargo llvm-cov --workspace --fail-under-lines 85

# Build release binary
build:
    cargo build --release

# Vault path for the TRIZ knowledgebase (single source of truth)
vault_dir := "vault"

# (Re)index the TRIZ knowledgebase — incremental; embeds with minilm
vault-index:
    vaultmind index --embed --model minilm --vault {{vault_dir}}

# Full rebuild of the vault index (use after large content changes / ranking issues)
vault-reindex:
    vaultmind index --embed --model minilm --full --vault {{vault_dir}}

# Vault health: unresolved links, embedding coverage, Obsidian-incompatible links
vault-doctor:
    vaultmind doctor --vault {{vault_dir}}

# Initialize scaffold for a new project (run once after clone)
init name:
    .ckeletin/scripts/init.sh {{name}}
