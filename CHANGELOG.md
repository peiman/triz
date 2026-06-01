# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-06-01

First public release of **triz** — an agent-native TRIZ method for inventive product
development. The validated deliverable is the method skill; the CLI is a deliberately thin
companion exposing only the two deterministic operations the experiments justified.

### Added

- **TRIZ method skill** (`docs/triz-method-skill.md`, installed at `.claude/skills/triz/`):
  an IFR-gated 5-question spine (Frame → IFR → Triage → Resources → Solve+Check) with a
  software/UX track that skips the contradiction matrix in favour of separation principles
  and function analysis. Validated by three blind A/B/C experiments (`docs/`).
- **Data** — `data/parameters.json` (39 engineering parameters with a blind-tested,
  100%-top-3 alias thesaurus) and `data/principles.json` (40 inventive principles).
- **Knowledgebase** — a 150+ note vaultmind vault (`vault/`), original synthesis cited to
  authoritative sources (clean-room, not derived from any single copyrighted curriculum).
- **CLI** (`triz`, ckeletin-rust workspace), two engineering-domain commands:
  - `parameter-search <query>` — ranks the 39 parameters against everyday engineer
    vocabulary; top-5 ranked matches; `--output json`.
  - `formulate-contradiction --improving <x> --worsening <y>` — resolves both sides to
    parameters and classifies the conflict as **technical** (two parameters) or **physical**
    (same parameter, opposite values), routing each to its resolution path; `--output json`.
  Both carry an explicit caveat that the parameters are engineering-domain only.

### Notes

- Honest scope: TRIZ helps on the inventive minority of problems; for software/UX/
  organizational work the method uses separation + function analysis, not the matrix.
