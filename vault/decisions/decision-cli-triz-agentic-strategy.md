---
id: decision-cli-triz-agentic-strategy
type: decision
status: accepted
title: "triz = agent-native guided TRIZ substrate (reframed strategy v2)"
created: 2026-05-30
tags:
  - triz
  - meta
  - adr
related_ids:
  - proj-triz
  - source-opensourcetriz
  - concept-ideality
  - concept-separation-principles
  - concept-function-analysis
  - concept-trimming
  - decision-cli-service-strategy
source_ids: []
---

## Context

Supersedes [[decision-cli-service-strategy|the matrix-centric ADR]]. The user
reframed the vision: `triz` is used WITH AI agents to attack new problems and do
inventive **product development** "with TRIZ in mind from the start," and it must
"guide anyone through the process and help them do the right thing." A v2
multi-agent deliberation (13 agents) that dogfooded TRIZ on `triz` itself produced
`docs/triz-cli-strategy-v2.md` (full strategy + adversarial critique + completeness
review). Winner: "Guided Job Runner" (narrow, blended with all four designs).

## Decision

`triz` = a **deterministic, agent-native, guided TRIZ method substrate** exposing
composable **skills → product-development jobs**, with the classical Contradiction
Matrix **demoted to optional/legacy** (a pluggable `--legacy` data pack). Modern
default path: `formulate-contradiction → suggest-separations → principle-lookup`.

**The honest core (the deliberation's own critique, adopted).** A capable LLM
already "knows" TRIZ, so `triz`'s genuinely-unique value is a **thin but real band**:
1. a **deterministic, curated parameter alias thesaurus** (verified synonym→parameter
   mapping an LLM cannot reliably self-do);
2. **deterministic, auditable, reproducible routing/classification** (TC vs PC,
   same-parameter→physical guard);
3. the method as **composable, typed, pipeable operations** (inspectable/testable/
   versionable, unlike an invisible system prompt);
4. **enforceable protocol gates** (won't emit solutions until IFR + contradiction
   type are filled — structural rigor for agent-builders);
5. a **shared SSOT** multiple agents + humans can agree on.
Everything else (principle descriptions, Socratic pacing, IFR enforcement-as-prose)
is replicable by ~50 lines of system prompt and is deferred to the vault/agent.

**Three-lane Separation of Concerns** (the central architectural decision):
- **Lane D** — deterministic domain data + pure logic (Rust `crates/domain`, SSOT;
  `data/*.json` → `&'static`): the lookups + classifiers + state machine.
- **Lane A** — left to the calling agent/human: messy-world judgment (naming
  functions, mapping a real quantity to a parameter, inventing embodiments).
- **Lane G** — guidance/orchestration: which question next, which tool now,
  well-formedness gates, IFR gate, secondary-problem check.
`triz` owns the method's invariants (D) and choreography (G); the caller owns the
domain semantics (A). Vault = prose the guide cites, never compiles.

**Guidance = a 5-question triage spine, not a wizard:** S1 Frame (plain language)
→ S2 IFR (gate) → S3 Triage (TC→separation by default / PC→separations / no-clear-
trade-off→function+causal analysis first; same-parameter guard) → S4 Resources →
S5 Solve+Check (vs IFR + secondary problems). Each step yields a usable artifact if
the user stops (monotonic value). ARIZ + anti-inertia tools (STC/9-Windows/SLP) are
opt-in/stall-triggered. Two skins over one engine: Socratic prose (`Display`) for
humans, typed JSON (`Serialize`) for agents; MCP later.

**The recommendation `triz`'s own method demands — VALIDATE BEFORE BUILDING:**
before any Rust, (0a) run a 2-hour A/B/C experiment on 3–5 vault case studies —
bare LLM vs LLM+TRIZ-system-prompt vs the triz tool (simulated by hand). If the
system prompt is ~80%+ as good, build only the synonym thesaurus + routing table.
(0b) build + blind-test `data/parameters.json` (alias thesaurus, 20 queries).
(0c) walk the spine manually on case studies. THEN decide whether the state
machine / 6 jobs / 12 skills are worth building. Per [[concept-ideality|ideality]]:
if a system prompt achieves the IFR, `triz` has trimmed itself out of existence —
which is the ideal outcome. Test it.

## Experiment result (2026-05-30) — VALIDATED, and it changed the plan

The mandated A/B/C core-premise experiment was run (blind, closed-book, 4 problems
incl. 2 novel product-dev ones; see `docs/triz-experiment-core-premise.md`):
**A-bare 24.5 · B-prompt 29.5 · C-tool 29.25 → B/C = 101%.** Both TRIZ conditions
beat the bare LLM by ~20% (the method matters), but **B (an LLM + a good TRIZ
system prompt) is indistinguishable from C (the deterministic tool)** — far past the
80%-build-only-the-thin-core threshold. The tool's unique value is **thin and
situational**: deterministic parameter-mapping/classification prevented errors and
won the novel mechanical case P3 (+6, on rigor + auditable trail), but solution
generation was *better* without rigidity (B>C), the deterministic router **failed**
the highest-leverage decision (P2: none routed to su-field), and parameter-mapping
was **useless for the software problem** (P4 — the honest "(none)" skip scored
highest). The IFR gate + anti-compromise discipline (claimed tool value) were fully
achieved by the system prompt. Per [[concept-ideality]]: a system prompt achieves
the IFR → the tool has largely **trimmed itself out of existence**, which IS ideal.

**Revised build order (evidence-based):**
1. **Ship the TRIZ method as a versioned system-prompt / skill artifact** (markdown).
   The only clearly-validated, highest-value, trivial-effort deliverable. This is the
   agent-native + "guide anyone" vision, delivered as a prompt, not a binary.
2. **Parameter alias thesaurus as a standalone JSON** — the one unique data asset
   (prevents LLM param-mapping errors), but engineering-only (no help for software/UX).
3. **`parameter-search` + `formulate-contradiction` as a minimal CLI/MCP** — only if
   the thesaurus blind-test passes; for agent pipelines needing deterministic,
   auditable mapping.
4. **Do NOT build** (the experiment killed these): the guidance state machine, the
   spine-as-FSM, the 6 jobs, the 12 skills, function-analysis/trimming engines, ARIZ
   escalation. A system prompt does them to equal quality.

Confidence: moderate (N=4, single model family, recall confound on documented cases,
C was simulated). The full tool is clearly not justified; the thin core (param-search)
rests mainly on P3 — a low-cost bet, not strongly validated.

## Consequences

- First work is a **2-hour experiment + the alias thesaurus**, not code — fits the
  "no code yet" constraint, and de-risks the whole project cheaply.
- The matrix demotion retires the licensing + ~1521-cell transcription risk.
- Tier 0 ship-first (if validated): `data/principles.json` + `data/parameters.json`
  + 6 skill ops (`principle/parameter-lookup`, `parameter-search`,
  `formulate-contradiction`, `suggest-separations`, `idealize`) with `--output
  text|json`, each response carrying `next`/`why`. Tiers 1–3: analytical hub
  (function analysis, resources, trim, causal-chain, inertia, validate) → guided
  flows → depth. NOT doing: 76 standards enumeration, effects DB, AI-inside-triz,
  web/TUI.
- Open forks (see doc §7): run the experiment first (Y/N); MCP alongside or after
  CLI; alias-thesaurus sourcing; the separation→principle subset map sourcing;
  scope of the `triz job` abstraction; an over-engineering circuit breaker.
