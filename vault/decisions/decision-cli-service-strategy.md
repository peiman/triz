---
id: decision-cli-service-strategy
type: decision
status: accepted
title: "triz CLI service strategy & ship-first"
created: 2026-05-30
tags:
  - triz
  - meta
  - adr
related_ids:
  - proj-triz
  - concept-contradiction-matrix
  - concept-triz-industrial-adoption
  - concept-ideality
  - case-study-pcr-diagnostics
source_ids: []
---

## Context

We needed to decide, before writing any code, what service the `triz` CLI should
provide, how the structured TRIZ data should be used, and why. We ran two
multi-agent deliberation workflows that **dogfooded the TRIZ method itself** to
design `triz` (IFR → contradictions → inventive/separation principles), grounded
in this vault. Full records: `docs/triz-cli-strategy.md` and
`docs/triz-cli-story-map.md`.

## Decision

`triz` is a **fast, offline, deterministic CLI that serves the classical
[[concept-contradiction-matrix|Contradiction Matrix]], 40 Inventive Principles,
and 39 Engineering Parameters as typed, scriptable, agent-pipeable data** — the
reproducible method as inspectable data, not an innovation oracle (honoring the
method-vs-impact gap in [[concept-triz-industrial-adoption|industrial adoption]]).

- **Data-first SSOT:** language-agnostic JSON (`data/parameters.json`,
  `data/principles.json`, `data/matrix-classic-1971.json`, `data/PROVENANCE.md`)
  compiled into Rust `&'static` constants in the domain crate. Vault = prose; CLI
  = presentation (Human + `--output json`). Separation of Concerns.
- **Honesty as the trust floor:** inline provenance + domain-bias disclosure (the
  classical matrix is ~30/39 physical parameters → weak for software/org
  contradictions). [[concept-ideality|Ideality]] maximized by lowering cost/harm.
- **Ship-first = the Walking Skeleton: a parameter/principle reference + alias
  search — NOT the matrix.** The load-bearing deliverable is a **curated alias
  thesaurus** (3–8 domain synonyms per parameter); the vault currently has zero
  useful aliases. That DATA must be built and blind-tested (10 queries, expected
  parameter in top-3) before the 5 subcommands ship.
- **The matrix is the MVP, gated by** (a) a **licensing spike** (verify the
  unconfirmed "public-domain per Savransky 1997" claim; Soviet/Berne copyright;
  data-vs-expression) run in parallel, and (b) **manual transcription** of ~1521
  cells from a named Altshuller edition (only verified cell:
  [[case-study-pcr-diagnostics|[27×18] → {11,32,13}]]).
- **Deferred (not v1):** ARIZ, 76 Standard Solutions, su-field, TESE trends, a
  guided `triz solve` wizard, an MCP server, embedded AI, web/TUI, Mann Matrix 2003.

### Resolved forks
- Audience: **data-first**, both AI agents and CLI engineers/practitioners first-class.
- Matrix source: a named public-domain Altshuller edition (edition pinned in the
  licensing spike) — never triz40.com or Mann's copyrighted Matrix 2003.
- Caveat prominence: JSON always carries provenance; human mode clean by default,
  `--meta`/`--stats` on demand; README states the boundary plainly.

## Consequences

- The genuinely-first work is **data + legal, not code** — which fits the "no code
  yet" constraint.
- Biggest risk to release 1 is a **bad alias thesaurus** (retired by blind-testing
  the data before any Rust). Licensing risk gates only the MVP, not the skeleton.
- The CLI's structured data lives in the Rust domain crate as the JSON-SSOT, never
  in vault frontmatter. The vault explains; the CLI computes.
