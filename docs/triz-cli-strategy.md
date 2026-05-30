# triz CLI — Service Strategy

> Captured 2026-05-30. Produced by a 13-agent multi-perspective deliberation workflow
> that dogfooded the TRIZ method itself (IFR → contradictions → inventive/separation
> principles), grounded in the project's own vaultmind knowledgebase. Winner of a
> 3-judge panel (the fast/offline matrix-as-data vision won 249 to 106 to 81 over the
> "guided coach" and "teaching companion" visions), adversarially critiqued by a
> customer advocate and a completeness reviewer (both reproduced below — the critique
> materially shaped the plan and is kept, not buried).

# `triz` Strategy: What to Build, How to Use the Data, and Why -- Decided by Dogfooding TRIZ

---

## 1. The Service & Why

### What `triz` is

`triz` is a fast, offline, deterministic CLI that serves the classical TRIZ Contradiction Matrix, 40 Inventive Principles, and 39 Engineering Parameters as a typed, scriptable, agent-pipeable Single Source of Truth. It ships the reproducible method as inspectable data -- not an innovation oracle, not an AI wrapper, not a consulting platform.

The one-sentence positioning: **the only tool where a shell command or AI agent can get a verified, provenance-tagged, deterministic matrix lookup in under 100ms, offline, with no account, no API key, and no hallucination risk.**

### Why this, and not something grander

The answer is anchored in two facts the vault documents with high confidence:

**Fact 1 -- The method-vs-impact gap** (`concept-triz-industrial-adoption`, Spreafico & Russo 2016): TRIZ's evidence base splits into unverifiable corporate ROI anecdotes (Samsung, Intel -- no reproducible method) and reproducible academic method papers (the PCR case -- full method, no ROI). The defensible, honest value is in the method. A tool that makes the method fast, correct, and inspectable is truthful. A tool that promises "innovation ROI" is selling the unverifiable half. `triz` ships the reproducible half.

**Fact 2 -- The 77/23 scope boundary** (`concept-levels-of-invention`): ~77% of inventions are Level 1-2, solvable within the solver's own specialty without TRIZ. Only the ~23% at Levels 3-5 require crossing field boundaries -- the one place TRIZ earns its keep. TRIZ is structurally a *cross-domain prompt engine against psychological inertia* (`concept-psychological-inertia`), not a general problem-solving oracle. The tool should be honest about this scope.

### Primary users and ranked jobs

**1. AI agents and LLM tool-use pipelines (the unoccupied niche, highest strategic value).** Every TRIZ+AI paper (arXiv 2408.05897 TRIZ-GPT, arXiv 2506.18783 TRIZ Agents) identifies the same gap: agents hallucinate matrix cells because no deterministic, typed, local TRIZ data source exists for them to call. triz40.com has no API. Goldfire/Accuris is enterprise-cloud. No OSS repo ships a clean-licensed, complete, machine-readable 39x39. `triz matrix 27 18 --output json` fills this gap with zero friction: install a binary, add it to the agent's tool manifest, get verified cells back as typed JSON.

The customer-advocate critique objects that an agent developer could "embed the 30KB lookup table as a JSON constant in their own codebase." This is fair as a theoretical point but wrong as a practical one: (a) no such verified, provenance-tagged, public-domain JSON artifact exists today -- someone has to create it, and `triz` is the vehicle; (b) embedding a raw lookup table gives you numbers, not enriched principle records with names, aliases, sub-principles, and provenance; (c) a maintained binary with a stable interface is more durable than a pasted constant that drifts. That said, the critique correctly identifies that **the data artifact itself is the core value, and the CLI is a delivery mechanism for it.** This reshapes the roadmap (see Section 4).

**2. CLI-native engineers who want sub-second lookup without a browser.** triz40.com is the status quo. It works. But it requires a browser, internet, click-through navigation, login-gated extras, and presents the matrix without provenance or confidence data. `triz` wins on: speed (sub-100ms vs. page-load + click + click), offline (air-gapped lab, plane, factory), scriptable (pipe to jq, embed in a Makefile), and honesty (provenance tags inline). The trade-off is real: triz40.com has richer examples and illustrations that `triz` v1 will not have. But the core job (contradiction -> principles) is served faster and more transparently.

The critique correctly notes that most R&D engineers (mechanical, electrical, chemical) are not CLI-native and do not have Rust toolchains. This is addressed by: (a) shipping pre-built binaries for major platforms (not just `cargo install`); (b) treating `--output json` as the integration surface so GUI/web/IDE wrappers can be built on top without reimplementing the data layer. `triz` is the engine, not the only interface.

**3. Patent analysts mapping invention contradiction structure.** The matrix and Levels of Invention are literally derived from patent corpora. Strong conceptual fit. Failure mode: the matrix's mechanical-era bias makes it weak for software/biotech patents. The tool must say so.

**4. TRIZ educators needing a reproducible, auditable reference.** Real but low commercial value. Well-served by existing content. Deprioritized.

### The adoption story, reconciled with the critique

The critique's hardest hit is: "The tool solves the easiest part of the TRIZ workflow (a table lookup) while deferring the hardest part (parameter mapping, which is where users fail)."

This is true. The honest response is not to pretend it is false but to sequence correctly:

- **v1 ships the data and the lookup.** This is genuinely valuable for agents (who do not suffer human framing-friction and can iterate parameter mapping cheaply) and for users who already know their parameters (the repeat-use case). It is also the precondition for everything else -- you cannot build guided framing on top of data that does not exist.
- **v1 also ships alias-aware parameter input** (stolen from Vision 3's `triz contradict --improving NAME --worsening NAME`). This directly addresses the framing gap at the lowest cost: when the user types `triz matrix reliability illumination-intensity`, the tool resolves those names to Parameter 27 and Parameter 18 via the alias tables already present in the vault frontmatter. This is not a "fuzzy AI matcher" -- it is a structured thesaurus. It will not solve every framing problem, but it eliminates the requirement to memorize parameter numbers.
- **v2 adds guided framing as a composition layer** (stolen from Vision 2's `triz solve` concept). The key architectural insight from Vision 2: each step of the 7-step method (frame, classify, map, lookup, specialize) should be an independently invocable subcommand, so the guided flow is a thin orchestration over lookup primitives, not a separate system. The domain API is designed from v1 to support this even though the guided flow is not built yet.

The critique also objects: "honesty about limitations has never been a successful adoption driver for any developer tool." This is a reasonable empirical claim. The response: honesty is not the *adoption driver* -- it is the *trust floor*. The adoption driver is the agent use case (nothing else exists) and the speed/offline use case (faster than triz40.com for the job). Honesty prevents the tool from losing credibility with the serious users who discover the limitations on their own. It is defensive, not offensive. The tool earns adoption by being useful, not by being honest; but it retains trust by being honest, not by hiding.

### The domain-applicability caveat, stated plainly

The critique demands: "State clearly: this tool is designed for mechanical and physical engineering contradictions." This is correct, and the tool should say it. The classical matrix's 39 parameters are overwhelmingly physical/mechanical. 30 of 39 parameters (Weight, Length, Area, Volume, Speed, Force, Stress, Shape, Temperature, etc.) are physical quantities. For a software engineer trying to resolve "improving security worsens performance," the parameter vocabulary is a poor fit. The 40 *principles* are more domain-independent (Segmentation, Dynamics, Inversion apply anywhere), but the *matrix cell recommendations* -- which principles for which contradiction -- are grounded in mechanical-era patent statistics.

The tool surfaces this as a first-class data property, not a buried footnote: `source: classic-altshuller-pre-1971, domain-bias: mechanical/physical`. The principles themselves remain useful as a cross-domain prompt checklist even when the matrix's statistical weightings are domain-mismatched. The tool should route users to the full 40-principle checklist when the matrix fit is weak, rather than presenting irrelevant cell results with false authority.

---

## 2. How We Use the Data

### The three-layer architecture (Separation of Concerns)

The boundary is crisp:

| Layer | Owns | Does not own | Key constraint |
|-------|------|-------------|----------------|
| **Domain crate** (`crates/domain/`) | All structured TRIZ facts: principle records, parameter records, matrix cells, separation mappings, derived indices. Pure functions. `serde::Serialize` + `fmt::Display`. | Prose, formatting, I/O, framework deps. | No clap, figment, tracing, infrastructure imports. Enforced at compile time by `crates/domain/tests/architecture_violations.rs`. Only serde permitted (per `CLAUDE.md`). |
| **Vault** (`vault/`) | All narrative prose: teaching explanations, worked examples, case study stories, critique discussion, connections. | Numbers, cell contents, structural relationships. | The vault note `id` is the foreign key. The domain crate references vault note IDs but never duplicates prose. |
| **CLI crate** (`crates/cli/`) + **Infrastructure** (`crates/infrastructure/`) | Presentation: subcommand parsing, output format selection (human vs JSON via `OutputMode::Human`/`OutputMode::Json` -- already present at `crates/cli/src/main.rs:90-94`), error envelopes, logging. | Business logic, data ownership. | Follows the existing `ping.rs` pattern: CLI handler calls a domain pure function, passes result to `Output`. |

**Future interface shells** (stolen from Vision 3): an MCP crate (`crates/mcp/`) can be added as a fourth workspace member sharing the same domain crate. The domain types already derive `Serialize`, so MCP tool responses are a mechanical transformation. This is not built in v1 but is a deliberate architectural affordance of the workspace structure. The physical contradiction "the tool must be both a local binary and a network service" is resolved by separation between system levels (parts vs. whole): at the WHOLE level, `triz` is one knowledge engine; at the PARTS level, CLI and MCP are two thin shells over the same domain.

### Core data structures in the domain crate

All compile-time constants. No runtime file loading, no network, no database. Zero allocation for lookups.

**1. Parameters: `[Parameter; 39]`**

```
Parameter {
    number: u8,          // 1-39
    name: &'static str,  // "Reliability"
    aliases: &'static [&'static str],  // ["Reliability"]
    definition: &'static str,  // one-line from vault Overview
    vault_note_id: &'static str,  // "parameter-27-reliability"
}
```

Source: extractable TODAY from the 39 vault `parameter-NN-*` notes' frontmatter (`number`, `title`, `aliases`) plus the single-sentence `## Overview` body. All 39 confirmed present and consistently structured.

**2. Principles: `[Principle; 40]`**

```
Principle {
    number: u8,          // 1-40
    name: &'static str,  // "Segmentation"
    aliases: &'static [&'static str],  // ["Segmentation"]
    sub_principles: &'static [(char, &'static str)],  // [('a', "Divide..."), ('b', "Make...")]
    opposite: Option<u8>,  // Some(5) for Segmentation->Merging
    vault_note_id: &'static str,
}
```

Source: extractable TODAY from the 40 vault `principle-NN-*` notes' frontmatter (`number`, `title`, `aliases`, `related_ids` for opposite) plus `## Sub-principles` body sections (all 40 have lettered a/b/c entries). Confirmed at `principle-01-segmentation.md`: sub-principles a, b, c present as prose bullets; aliases in frontmatter; opposite linked via `related_ids`.

**3. Matrix: sparse `[(u8, u8, &'static [u8])]` or `[[Option<&'static [u8]>; 39]; 39]`**

Each cell maps `(improving_param_number, worsening_param_number)` to 0-4 principle numbers. Tagged with version metadata:

```
MatrixMeta {
    version: &'static str,       // "classic-1971"
    source: &'static str,        // "Altshuller, Creativity as an Exact Science (1984)"
    provenance: &'static str,    // "Public domain per Savransky/TRIZ Journal 1997"
    domain_bias: &'static str,   // "mechanical/physical engineering (pre-1970 patents)"
}
```

Source: **THIS DATA DOES NOT EXIST IN THE VAULT.** The matrix is described but 0% populated. Only one cell is physically present as prose: `[27 x 18] -> {11, 32, 13}` (from `concept-contradiction-matrix.md` and `case-study-pcr-diagnostics.md`). This is the critical path (see Section 2.5).

**4. Separation mappings (Tier 2, after matrix ships):**

```
SeparationMapping {
    separation_type: SeparationType,  // Time | Space | Condition | SystemLevel
    principles: &'static [u8],       // principle number subsets
    confidence: Confidence,           // High | Medium
    source: &'static str,            // "MATRIZ" | "Domb" | "Mann"
}
```

Source: `concept-separation-principles.md` names the 4 Western separations but contains **zero `principle-NN` references** and explicitly flags per-separation subsets as "unverified outside MATRIZ." Ship only MATRIZ-verified mappings as `High` confidence; defer or mark others `Medium`. This data requires external sourcing from MATRIZ materials.

### Query paths (all pure functions in domain, no side effects)

| Query | Signature | Complexity | Notes |
|-------|-----------|-----------|-------|
| Forward lookup | `matrix_lookup(improving: u8, worsening: u8) -> MatrixResult` | O(1) + O(k) where k<=4 | Returns `Principles(Vec<&Principle>)` or `Empty` or `PhysicalContradiction` (diagonal) |
| Reverse lookup | `principles_for(principle: u8) -> Vec<(u8, u8)>` | O(39*39) but matrix is tiny (1521 cells) | Derived by inverting the cell map; can be pre-computed at compile time |
| Principle frequency | `principle_frequency() -> Vec<(u8, usize)>` | O(39*39) | Count cell appearances per principle, sorted descending |
| Co-occurrence | `principle_cooccurrence(p: u8) -> Vec<(u8, usize)>` | O(39*39) | Which principles share cells with p |
| Diagonal detection | Built into `matrix_lookup` | O(1) | `improving == worsening` -> `PhysicalContradiction` variant with pointer to separation principles |
| Empty-cell handling | Built into `matrix_lookup` | O(1) | `None` cell -> `EmptyCell` variant with semantics: "no dominant principle, consider full 40 as checklist" |
| Search | `find_parameter(query: &str) -> Vec<&Parameter>` | O(39 * aliases) | Case-insensitive substring over names + aliases |
| Search | `find_principle(query: &str) -> Vec<&Principle>` | O(40 * aliases) | Same |
| Coverage stats | `matrix_stats() -> MatrixStats` | O(39*39) | Filled/empty counts, per-principle frequency, asymmetry checks |

### The data we do NOT have -- and how to source it

**The 39x39 matrix cells are the entire critical path.** Without them, `triz` cannot perform its flagship lookup. Everything else is downstream.

**Licensing status (confirmed by research):** The classical Altshuller matrix is **public domain** -- confirmed by Savransky (TRIZ Journal 1997): "in the public domain due to the goodwill of the author... anybody should remember that its idea belongs to G.S. Altshuller." The raw cell data is safe to redistribute with attribution.

**What to source from:** A primary public-domain Altshuller edition. The two candidates:
- *Creativity as an Exact Science* (Altshuller 1984, English: Gordon & Breach, ISBN 0677212305) -- the matrix is reproduced as a table.
- *The Innovation Algorithm* (Altshuller 1999, English: Technical Innovation Center, ISBN 0964074044) -- the later English edition, likely easier to obtain.

**What NOT to source from:**
- triz40.com -- proprietary digitization; the site demands attribution "even for AI citation" and the specific cell arrangement is their copyrighted work.
- Matrix 2003/2010/2022 (Mann/CREAX) -- fully copyrighted, commercial. Uses 48-50 parameters. Legally untouchable. Never ship these cells, never consult this source.
- Any unattributed spreadsheet or GitHub repo without clear provenance chain.

**Sourcing plan (the critique correctly demands this be concrete):**
1. Obtain a physical or verified-digital copy of one of the two named Altshuller editions. Record the exact ISBN, edition, and page numbers.
2. Transcribe the ~1521 cells manually. Each cell: 0-4 principle numbers (u8 values 1-40).
3. Cross-verify against at least 5 independently documented cell values. Currently confirmed: `[27x18] -> {11, 32, 13}` (PCR case, `case-study-pcr-diagnostics.md`). Need 4+ more from independent case studies or the academic literature.
4. Create a `PROVENANCE.md` in the repository documenting exact source, edition, page, transcription method, and verification results. Commit this before any cell data.
5. The transcription effort is estimated at 15-25 hours of careful manual work. This is the true MVP scope -- not the CLI subcommands.

**The critique's strongest point, acknowledged:** "Before writing a single line of Rust domain code, the project needs the matrix data." Partially true. The parameter and principle enums are independently useful (search, list, lookup by number/name/alias) and can ship before the matrix cells are sourced. But the flagship lookup -- the reason the tool exists -- requires the cells. The roadmap (Section 4) sequences accordingly: ship the enums first, source the matrix in parallel, ship the lookup when the data is ready.

### The data artifact as a standalone deliverable

Stolen from the critique's "what would make it real": **publish the verified matrix as a standalone, language-agnostic JSON artifact with full provenance metadata, under a clear license, in the repository.** This is the actual white space. The Rust binary is a delivery mechanism on top of it. If the JSON artifact gets adoption independently (embedded by agent developers, cited by researchers), that validates the data layer before the CLI wrapper matters.

Concretely: the repository ships a `data/` directory containing:
- `data/matrix-classic-1971.json` -- the 39x39 cells with provenance metadata
- `data/principles.json` -- the 40 principles with aliases and sub-principles
- `data/parameters.json` -- the 39 parameters with aliases and definitions
- `data/PROVENANCE.md` -- exact sourcing documentation

The Rust domain crate compiles these into `&'static` constants at build time (via `include!` or a build script). But the JSON files are the SSOT -- usable by any language, any tool, any agent, without Rust.

---

## 3. TRIZ, Dogfooded

### The Ideal Final Result of `triz`

Stated per `concept-ideality` ("the function is performed but the machine is absent"):

> The engineer or AI agent states a contradiction and instantly receives the statistically-grounded inventive principles -- without learning TRIZ notation, without network access, without a browser, without a subscription, and without the tool misrepresenting the data's provenance or limitations. The tool trends toward its own disappearance: repeated use teaches the user the thinking pattern until they no longer need the tool. The function (contradiction-to-principle lookup + cross-domain prompt against psychological inertia) is delivered; the cost (learning curve + tool complexity + access friction) approaches zero; the harm (false authority, hallucinated data, hidden limitations) is eliminated.

Applying the ideality equation from `concept-ideality`:

```
Ideality = Benefits / (Costs + Harm)

Benefits:  correct matrix lookup + cross-domain principle prompts + provenance transparency
Costs:     install binary + learn parameter vocabulary + source the data
Harm:      dated-data bias (reducible but not eliminable) + false confidence (eliminable by design)
```

The design maximizes ideality by: (a) raising benefits (sub-100ms lookup, JSON output, alias search, empty-cell routing); (b) lowering costs (pre-built binaries, alias-aware input, no account/API key); (c) eliminating harm (provenance tags, domain-bias disclosure, honest empty-cell semantics).

### The contradictions in building `triz`, and their resolutions

#### Contradiction 1 (Technical): Completeness vs. Simplicity

**Statement:** Improving information completeness (sub-principles, examples, aliases, separation mappings, co-occurrence stats, coverage warnings, provenance metadata -- everything a serious user or agent needs) worsens ease of operation (more output fields = more cognitive load for the engineer who just wants "which principles for my trade-off?").

**TRIZ mapping:** Improving Parameter 28 (Measurement accuracy / completeness of answer) worsens Parameter 33 (Ease of operation / simplicity of use).

**Resolution -- Principle 1 (Segmentation) + Principle 3 (Local Quality) + Principle 15 (Dynamics):**

Segment the output into independently composable tiers. The default response is the "3-second answer" -- just principle numbers and names. A `--verbose/-v` flag adds sub-principles, aliases, and examples. A `--meta` flag adds provenance, confidence, dated-data caveats, and coverage stats. Each tier is an independent slice. JSON mode (`--output json`) always returns the full structured envelope because agents parse, not read.

Concrete design decision:
- `triz matrix 27 18` -> "11 (Beforehand Cushioning), 32 (Color Changes), 13 (The Other Way Round)" -- three lines, done.
- `triz matrix 27 18 -v` -> adds sub-principles for each.
- `triz matrix 27 18 --meta` -> adds `source: classic-1971`, domain-bias note, co-occurrence stats.
- `triz matrix 27 18 --output json` -> full envelope always, agents never guess which flag to pass.

This is Principle 3 (Local Quality) in action: each "region" of the output is optimized for its consumer. Human mode is terse and scannable; JSON mode is complete and typed.

#### Contradiction 2 (Physical): Human-Readable AND Machine-Parseable

**Statement:** The output format must be BOTH human-readable (scannable, colored, formatted for a terminal) AND machine-parseable (structured, typed, deterministic JSON). A single attribute (output format) must simultaneously be two opposite things. This is a textbook physical contradiction per `concept-physical-contradiction`: "one and the same parameter is required to take two opposite, mutually exclusive values at once."

**Resolution -- Separation upon Condition (from `concept-separation-principles`):**

The output is human-readable WHEN the consumer is a human (default, stdout is a TTY) and machine-parseable WHEN the consumer is an agent (`--output json` flag, or auto-detected when stdout is piped). The conflicting requirements never coexist because they are separated by the condition of who is reading.

This is already the ckeletin architecture pattern. The infrastructure crate provides `Output::new(OutputMode::Human | OutputMode::Json)` (confirmed at `crates/cli/src/main.rs:90-94`). Domain types implement both `serde::Serialize` (for JSON) and `fmt::Display` (for human text), following the existing `ping.rs` pattern (`crates/domain/src/ping.rs:7-15`). The physical contradiction is fully dissolved by the existing architecture. No new mechanism needed -- Principle 25 (Self-service): the ckeletin scaffold IS the resolution.

Additionally, Principle 25 (Self-service) applies: the tool can detect whether stdout is a TTY (`isatty()`) and auto-select format. Pipe to jq? You get JSON. Run interactively? You get formatted text. The user does not need to remember a flag in the common case.

#### Contradiction 3 (Technical): Honesty vs. Perceived Authority

**Statement:** Improving reliability/trustworthiness of the data (surfacing the dated-data critique, flagging empty cells, exposing the pre-1970 mechanical-era bias, marking the matrix version) worsens the perceived authority of the tool (users expect a TRIZ matrix tool to be authoritative and confident, not hedged). The documented method-vs-impact adoption gap (`concept-triz-industrial-adoption`) shows the field already suffers from over-claiming.

**TRIZ mapping:** Improving Parameter 27 (Reliability) worsens Parameter 35 (Adaptability/versatility / perceived range of applicability).

**Resolution -- Principle 22 (Blessing in Disguise) + Principle 13 (The Other Way Round) + Principle 23 (Feedback):**

**Principle 22 -- Convert harm into benefit:** The matrix's known weakness (pre-1970 patent corpus, mechanical-era bias) is precisely what makes `triz` more trustworthy than competitors who hide it. Every matrix lookup carries a `source: classic-1971` provenance tag. This is not a disclaimer buried in docs -- it is inline, visible, machine-readable. The tool's honesty becomes its differentiator against triz40.com (no provenance) and commercial tools (limitations papered over).

**Principle 13 -- Inversion:** Invert the assumption that "authority requires confidence." In the documented method-vs-impact gap, the unverifiable half is the corporate ROI claims presented with high confidence; the verifiable half is the academic method presented with explicit limitations. `triz` aligns with the verifiable side. Transparency IS authority, not its opposite. The tool that shows you its provenance is more authoritative than the one that hides it, because you can verify its claims. This directly honors Manifesto Principle 1 (Truth-Seeking).

**Principle 23 -- Feedback:** The tool reports its own data health. `triz matrix --stats` shows filled vs. empty cell counts, principle frequency distributions, and the domain-bias caveat. The tool cross-checks itself. This is not a one-time disclaimer but structural self-reporting.

Concrete design decision: the `--meta` flag and JSON envelope carry provenance as a functional field, not a footer. An agent that receives `{confidence: "medium-mechanical-era-bias"}` can decide to also run a broader principle checklist. An agent that receives hallucinated principle numbers cannot recover. Informed trust is more durable than uninformed confidence.

#### Contradiction 4 (Physical): Static Data AND Dynamic Composition

**Statement (stolen from Vision 4):** The teaching content must be STATIC (deterministic, version-controlled, typed, reproducible -- the entire value proposition per `concept-triz-industrial-adoption`) AND DYNAMIC (responsive to the user's query context, surfacing the right principle at the right moment, linking to the right case study). A single attribute (content responsiveness) must be both frozen-correct and alive-contextual.

**Resolution -- Separation between System Levels (parts vs. whole):**

At the PARTS level, individual facts (matrix cells, principle definitions, parameter names, case-study metadata) are static and deterministic in the domain crate. At the WHOLE level, the assembled response to a query is dynamic, composed at runtime by selecting and arranging the right static parts based on the user's context.

Concretely: when the user queries cell `[27, 18]`, the CLI (a) looks up the static cell contents `{11, 32, 13}`, (b) enriches each with the static principle record (name, sub-principles), (c) checks if any case study references that cell (the PCR case does -- this is a small `cell -> case_study_id` foreign key in the domain crate, not embedded prose), and (d) composes a response that feels contextual but is fully deterministic. Run the same query twice, get the same answer.

This is the honest counterpoint to ChatGPT-based TRIZ tools that feel dynamic but hallucinate cells. The dynamism is in selection and composition, never in the facts.

#### Contradiction 5 (Technical, from the critique): Lean MVP vs. SSOT Authority

**Statement (surfaced by the customer advocate):** The tool must be BOTH a lean-first MVP (three tables, seven commands -- Manifesto Lean Iteration) AND a comprehensive SSOT worthy of the "typed Single Source of Truth" branding. But a lookup table with no examples, no sub-principle detail, no framing guidance is "not meaningfully different from a static JSON file on GitHub." The SSOT framing implies authority and completeness; the lean scope delivers a thin shell.

**TRIZ mapping:** Improving Parameter 33 (Ease of operation / development velocity) worsens Parameter 36 (Device complexity / completeness).

**Resolution -- Principle 1 (Segmentation) + Principle 10 (Preliminary Action):**

Segment the SSOT into independently shippable tiers (stolen from Vision 3's tiered phasing):

- **Tier 0 (ship immediately):** The 40 Principle enums and 39 Parameter enums as typed constants, queryable by number/name/alias. PLUS the standalone JSON data artifacts (`data/principles.json`, `data/parameters.json`). This is extractable from vault frontmatter TODAY and delivers list/search/lookup value independently.
- **Tier 1 (ship when matrix data is sourced):** The 39x39 matrix cells with provenance. PLUS `data/matrix-classic-1971.json`. This lights up the flagship lookup.
- **Tier 2 (ship after usage validates demand):** Separation-principle-to-principle-subset mappings, case-study-to-cell cross-references, enriched sub-principle records.

Each tier compiles and is useful independently. The "SSOT" label is honest at Tier 0 for principles and parameters; it becomes fully earned at Tier 1 when the matrix ships. The lean MVP IS the SSOT for the data it contains -- it just does not contain all data yet, and says so explicitly via `triz matrix --stats` (coverage reporting). Principle 23 (Feedback) applied to the tool's own completeness.

Principle 10 (Preliminary Action): pre-arrange the domain types so Tier 1 slots into the Tier 0 schema without breaking changes. The `MatrixResult` enum exists and returns `NotYetPopulated` in Tier 0 if someone tries a lookup before the matrix data is committed. The schema is ready before the data fills it.

---

## 4. Scope & Lean Roadmap

### Milestone 0: The Data Foundation (before any Rust code)

**Deliverable:** The verified classical Altshuller matrix as standalone JSON artifacts in the repository, with full provenance documentation.

- `data/parameters.json` -- 39 parameters with numbers, names, aliases, definitions. Source: vault frontmatter (available now).
- `data/principles.json` -- 40 principles with numbers, names, aliases, sub-principles, opposite-principle refs. Source: vault frontmatter + body sections (available now).
- `data/matrix-classic-1971.json` -- the 39x39 cells. Source: a named Altshuller primary edition (see Open Decision 1). **This is the critical path.**
- `data/PROVENANCE.md` -- exact ISBN, edition, page references, transcription method, verification results (minimum 5 cross-checked cells).
- License: data files under a permissive license with Altshuller attribution.

**Why this is first:** The critique is right that the data artifact is the core value. If the JSON files get adoption (embedded by agent developers, cited by researchers), they validate the project before the CLI matters. If transcribing the matrix proves infeasible or the provenance chain cannot be established, better to discover that before building typed Rust wrappers around air.

**Effort estimate:** Parameters + principles JSON: ~4 hours (parse vault frontmatter). Matrix transcription: 15-25 hours (manual, from a book). Verification: 3-5 hours. Provenance documentation: 2 hours. Total: ~25-35 hours, almost entirely data work.

### Milestone 1: Tier 0 -- Principles & Parameters in the Domain Crate

**Deliverable:** `triz principle`, `triz parameter` subcommands with human + JSON output.

- Domain crate: `Principle` and `Parameter` types as `&'static` arrays, compiled from the JSON data files.
- CLI commands: `triz principle <N|name>`, `triz principle list`, `triz principle find <query>`, and the parameter equivalents.
- `--output json` on every command (inherited from ckeletin infrastructure).
- Search: case-insensitive substring matching over names + aliases.
- Tests: following the existing `ping.rs` TDD pattern in both domain (`crates/domain/src/`) and CLI (`crates/cli/tests/cli.rs`).

**Value delivered:** Anyone can query the 40 principles and 39 parameters as typed data from the command line or via JSON. Not the flagship lookup, but independently useful for reference and for agent tool manifests.

### Milestone 2: Tier 1 -- The Matrix Lookup

**Deliverable:** `triz matrix` subcommand -- the flagship.

- Domain crate: matrix cells as a typed constant, loaded from `data/matrix-classic-1971.json` at compile time.
- CLI commands:
  - `triz matrix <improving> <worsening>` -- forward lookup. Accepts parameter numbers OR names/aliases (alias-aware input).
  - `triz matrix <improving> <worsening> -v` -- adds sub-principles for each returned principle.
  - `triz matrix <improving> <worsening> --meta` -- adds provenance tag, domain-bias caveat, co-occurrence stats.
  - `triz reverse <principle-number>` -- reverse lookup: all cells where this principle appears.
  - `triz matrix --stats` -- coverage report, fill rate, version tag, dated-data disclosure, principle frequency ranking.
- Diagonal detection: `improving == worsening` -> returns `PhysicalContradiction` variant with message routing to separation principles.
- Empty-cell handling: returns explicit "no dominant principle -- consider the full 40 as a checklist, or route to ARIZ for deeper analysis" with honest semantics.
- `--output json` always includes the full structured envelope with provenance metadata.
- PCR case study linked to cell `[27, 18]` as a cross-reference (domain stores the foreign key `case_study_id`, not the prose -- proving the case-study-to-cell architecture with one real example).

**Value delivered:** The flagship service. An agent or engineer runs one command and gets verified, provenance-tagged matrix results. The white space is filled.

### Milestone 3: Tier 2 -- Separation, Enrichment, Hardening

- Separation-principle-to-principle-subset mappings (only MATRIZ-verified as `confidence: High`).
- `triz separate [--type time|space|condition|system-level]` subcommand.
- Case-study cross-reference expansion (as more worked examples are documented).
- Shell completions (bash, zsh, fish) for discoverability.
- Pre-built binaries via CI (GitHub Releases) for macOS, Linux, Windows -- eliminating the `cargo install` barrier.
- Homebrew formula.

### Future (v2, only after real usage data exists)

- **Guided problem-framing** (`triz solve`): the 7-step method as a composition layer over lookup primitives. Each step is an independently invocable subcommand. The guided flow is a thin orchestration, not a separate system.
- **MCP server** (`crates/mcp/`): a new workspace member sharing the domain crate. Makes `triz` callable as an MCP tool by any AI agent that speaks the protocol.
- **Richer parameter thesaurus**: based on real user queries that fail to match, expand the alias tables with domain-specific synonyms.

### Explicitly NOT doing (with reasons)

| Not building | Why |
|-------------|-----|
| ARIZ (the 9-part algorithm) | Heavyweight, interactive, irreducible to a CLI lookup. `concept-ariz`: "for non-standard/hard problems the matrix cannot crack." Shipping it would over-promise. |
| 76 Standard Solutions | Not enumerated as data in the vault. Would require a separate research wave. |
| Su-Field analysis | Requires a modeling UI/language, not a lookup primitive. Prose-only in vault. |
| TESE Trends | No `number` field in vault notes; prose-only; low lookup value. |
| Matrix 2003/2010/2022 (Mann/CREAX) | Fully copyrighted commercial work. Legally untouchable. |
| AI/LLM features inside `triz` | `triz` is the deterministic SSOT that AI tools query. It does not embed an AI. Separation of concerns. |
| Web UI / TUI | CLI is the product; `--output json` is the integration surface. UIs are a separate project. |
| ROI claims or "innovation platform" framing | The method-vs-impact gap (`concept-triz-industrial-adoption`) shows this is the unverifiable half. `triz` ships method, not outcomes. |
| Guided framing wizard in v1 | Requires UX research on parameter selection. Build the lookup primitives first; compose guidance on top when real usage data exists. |
| Spaced-repetition / gamification | Adds complexity for unproven learning benefit. The tool teaches by surfacing concepts at the moment of need, not by drilling. |

---

## 5. Open Decisions for the User

### Decision 1: Which Altshuller edition to source the matrix from

The two candidates:

| Edition | Pros | Cons |
|---------|------|------|
| *Creativity as an Exact Science* (1984, Gordon & Breach, ISBN 0677212305) | Earlier, closer to the original Russian editions. Widely cited in TRIZ literature. | Harder to obtain (out of print). English translation quality varies. |
| *The Innovation Algorithm* (1999, Technical Innovation Center, ISBN 0964074044) | More accessible. Later English edition, likely cleaner typesetting. Most-cited in English-language TRIZ work. | Later edition may have transcription differences from the 1984 one. |

**The decision:** Which specific edition will be the primary source, and can you obtain a physical or verified-digital copy? The `PROVENANCE.md` must name the exact edition, ISBN, and page numbers. Ideally, cross-verify a sample of cells against the other edition to detect any transcription differences between them.

**If neither is obtainable:** The MATRIZ wiki (`wiki.matriz.org`) has an interactive matrix tool with a CC-attributed notice, but the specific CC variant is not pinned and the download path is not explicit. This could be a fallback source, but the provenance chain is weaker.

### Decision 2: Data artifact first, or Rust CLI first?

Two sequencing strategies:

| Strategy | Argument for | Argument against |
|----------|-------------|-----------------|
| **Data-first:** Publish `data/*.json` + `PROVENANCE.md` as a standalone commit/release before any Rust domain code. Let the JSON files be consumed independently. | Validates the core value (the data) before investing in the delivery mechanism (the CLI). If the data gets adoption, the CLI is justified. The critique's strongest recommendation. | Delays the CLI. The Tier 0 enums (principles/parameters) are trivial to type in Rust and could ship in parallel with matrix sourcing. |
| **Parallel:** Type the Tier 0 enums in the domain crate immediately (from vault frontmatter); source the matrix data in parallel; ship Milestone 1 (principle/parameter lookup) before the matrix data is ready. | Delivers working software sooner. The principle/parameter lookup has standalone value. Maintains development momentum. | Risk of building typed wrappers around data whose sourcing may fail. |

**My recommendation:** Parallel, with data-first for the matrix specifically. Ship Tier 0 (principles + parameters) as both JSON artifacts and Rust code immediately -- the vault frontmatter is available now and the typing is straightforward. Source the matrix cells as a dedicated data-entry effort, publish `data/matrix-classic-1971.json` when complete, then integrate into the domain crate for Milestone 2. This way Milestone 1 ships fast, the matrix sourcing is not blocked on Rust development, and the JSON artifact exists independently.

### Decision 3: How prominently to surface the domain-bias caveat

Three options:

| Level | What it looks like | Risk |
|-------|-------------------|------|
| **Inline on every lookup** | Every `triz matrix` result includes a one-line note: "Source: classic-1971, mechanical/physical engineering bias." JSON always carries the metadata. | May feel repetitive to experienced users. Some users may interpret honesty as the tool admitting it is bad. |
| **On first use + in --meta** | Show the caveat the first time `triz matrix` is run (or in `triz matrix --stats`), suppress on subsequent runs. JSON always carries the metadata. | Reduces annoyance but hides the caveat from users who need it most (those who do not run `--stats`). |
| **In JSON always, in human mode only via --meta** | Human-mode default is clean (just principle names). `--meta` adds provenance. JSON always includes it. | Cleanest default UX, but the honesty commitment is weaker for human users. |

**My recommendation:** Option 3, with one addition: `triz matrix --stats` is the dedicated "data health" command that surfaces everything, and the README prominently documents the provenance. Human-mode lookups are clean by default; JSON mode always includes provenance (agents need it); `--meta` is the opt-in for humans who want it. This balances the Segmentation principle (terse default) with the honesty commitment (always available, never hidden, prominent in docs and JSON).

### Decision 4: Scope of v1 -- does `triz reverse` and `triz matrix --stats` ship in Milestone 2, or are they Milestone 3?

The reverse lookup and coverage stats are derived computations over the matrix -- pure functions, no additional data needed. They are straightforward to implement. But they are not the core job (forward lookup). Including them in Milestone 2 makes the tool feel more complete but adds development scope.

**My recommendation:** Ship them in Milestone 2. They are small, derived, and high-value. The reverse lookup (`triz reverse 11` -> "which contradictions suggest Principle 11?") is uniquely useful for patent analysis and for understanding the matrix's statistical structure. The coverage stats (`triz matrix --stats`) are the self-reporting mechanism that earns the honesty commitment. Both are pure functions over data that already exists at Milestone 2. The marginal effort is low; the marginal value is high.

### Decision 5: Primary audience framing -- agent-first or engineer-first?

The visions frame the primary user differently. The judge-winning vision leads with AI agents; the critique argues this is "aspirational vaporware serving aspirational vaporware."

| Framing | Implications for design | Implications for adoption |
|---------|------------------------|--------------------------|
| **Agent-first** | JSON output is the primary interface; human mode is secondary. CLI ergonomics matter less than schema stability. MCP moves up in priority. | Adoption depends on the nascent TRIZ+AI ecosystem maturing. Small but uncontested niche. |
| **Engineer-first** | Human-mode UX is primary. Shell completions, man pages, memorable commands matter. JSON is the integration surface. | Adoption competes with triz40.com. Larger addressable audience but higher friction (CLI-non-native users). |
| **Data-first (the synthesis)** | The JSON data artifacts are the primary deliverable. Both CLI and agent consumption are delivery mechanisms on top. | Adoption starts with anyone who needs the data -- researchers, tool-builders, agent developers, CLI users. Broadest initial surface. |

**My recommendation:** Data-first framing, with both agent and engineer as first-class consumers. The JSON data artifacts (`data/*.json`) are the SSOT. The CLI is the reference implementation that proves the data works. `--output json` makes the CLI agent-friendly. Human-mode output makes it engineer-friendly. Neither is secondary. The data is primary.

---

## References

- `/Users/peiman/dev/triz/vault/concepts/concept-triz-industrial-adoption.md` -- the method-vs-impact gap (anchor for honesty)
- `/Users/peiman/dev/triz/vault/concepts/concept-psychological-inertia.md` -- the core user pain TRIZ addresses
- `/Users/peiman/dev/triz/vault/concepts/concept-levels-of-invention.md` -- the 77%/23% scope boundary
- `/Users/peiman/dev/triz/vault/concepts/concept-contradiction-matrix.md` -- dated-data critique + 7-step usage flow + diagonal->physical routing
- `/Users/peiman/dev/triz/vault/concepts/concept-ideality.md` -- IFR definition and ideality equation
- `/Users/peiman/dev/triz/vault/concepts/concept-physical-contradiction.md` -- physical contradiction definition
- `/Users/peiman/dev/triz/vault/concepts/concept-technical-contradiction.md` -- technical contradiction definition
- `/Users/peiman/dev/triz/vault/concepts/concept-separation-principles.md` -- 4 separations + unverified-subset caveat
- `/Users/peiman/dev/triz/vault/concepts/concept-ariz.md` -- ARIZ as heavyweight, for non-standard problems
- `/Users/peiman/dev/triz/vault/concepts/case-study-pcr-diagnostics.md` -- the ONE fully documented matrix application: [27x18]->{11,32,13}
- `/Users/peiman/dev/triz/vault/concepts/case-study-rabbit-enclosure.md` -- separation-in-space teaching example
- `/Users/peiman/dev/triz/vault/concepts/principle-01-segmentation.md` -- principle note structure (frontmatter + sub-principles + examples)
- `/Users/peiman/dev/triz/vault/concepts/parameter-27-reliability.md` -- parameter note structure (frontmatter + one-line overview)
- `/Users/peiman/dev/triz/crates/domain/src/ping.rs:1-15` -- the domain type pattern (Serialize + Display, pure function)
- `/Users/peiman/dev/triz/crates/cli/src/main.rs:90-94` -- existing OutputMode::Human/Json selection
- `/Users/peiman/dev/triz/crates/cli/src/root.rs:1-35` -- existing CLI structure (OutputFormat, Commands enum)
- `/Users/peiman/dev/triz/crates/domain/tests/architecture_violations.rs` -- compile-time architecture enforcement
- `/Users/peiman/dev/triz/Cargo.toml:1-38` -- workspace structure and dependency policy

---

# Appendix A — Customer-Advocate Critique (adversarial)

**Verdict:** No. Real TRIZ users would not adopt this in its current vision. The tool solves the easiest part of the TRIZ workflow (a table lookup any PDF or website already provides) while deferring the hardest part (parameter mapping, which is where users actually fail). The 'AI agent' primary user is a phantom -- no production agent pipeline is blocked on the absence of a Rust TRIZ binary, and any that wanted one could embed the 30KB lookup table directly. The critical-path data (the 1521 matrix cells) does not exist, has no concrete sourcing plan, and the entire vision is architecturally complete but data-empty. The honesty-as-differentiator thesis is intellectually appealing but commercially untested -- no developer tool has ever been adopted primarily because it was transparent about its own limitations. What exists today is a well-scaffolded Rust workspace with a ping command and 138 prose notes. The vision is a sophisticated rationalization for building a typed wrapper around a public-domain lookup table that fits in a spreadsheet. Ship the verified data as a standalone JSON artifact, solve the parameter-mapping problem, and target one concrete human workflow end-to-end -- then there might be something worth building a CLI around.

## Fatal flaws
- THE TOOL CANNOT HELP WHERE USERS ACTUALLY FAIL. The vision's own Phase 1 findings identify the hardest step as 'framing the contradiction correctly -- mapping a messy real problem onto two of the 39 generalized parameters is the hard, error-prone step, and the matrix gives garbage if the mapping is wrong.' Then the vision explicitly defers this: 'NOT guided problem-framing wizard in v1.' So the tool ships the easy part (the lookup, which is a table dereference any intern can do with a PDF) and skips the hard part (which parameters am I even looking at?). An engineer stuck on 'my battery is too heavy but needs more capacity' has to somehow know that maps to Parameter 1 (Weight of moving object) vs Parameter 26 (Quantity of substance) -- and if they pick wrong, they get wrong principles. The vision acknowledges this is where users stall, then builds a tool that does not address it. This is like building a calculator that requires you to already know the formula.
- THE 'KILLER USE CASE' (AI AGENTS) IS SPECULATIVE AND SELF-REFERENTIAL. The vision claims the primary user is AI agents needing a 'deterministic backstop against hallucinated matrix cells.' But this assumes: (a) AI agents are actually using TRIZ methodology in production pipelines today (they are not -- the cited papers are academic experiments), (b) these agents would install and shell-exec a Rust binary rather than just embedding the 1521-cell lookup table as a JSON constant in their own codebase (trivial -- the data is public domain and fits in ~30KB), and (c) the 'hallucination problem' for matrix cells is a real bottleneck rather than a minor footnote in agent-based ideation. The vision builds a typed Rust binary to serve a 30KB lookup table to users who could paste that table into a Python dict in 5 minutes. The overhead of 'cargo install triz' + subprocess invocation + JSON parsing is strictly worse than an embedded constant for any agent developer. The vision confuses 'no existing tool does X' with 'X needs to exist as a separate tool.'
- THE MATRIX DATA DOES NOT EXIST AND THE SOURCING PLAN IS HAND-WAVED. The vision's own data inventory confirms: 'The actual 39x39 contradiction matrix cell contents do NOT exist anywhere in the vault. The matrix is 0% populated.' The sourcing plan is 'transcribe from Altshuller's books.' But: (a) manually transcribing ~1521 cells (each with 0-4 principle numbers) from a book printed in Russian in 1969, available in English translations of varying quality, is a multi-day, error-prone effort with no verification method beyond spot-checking one known cell [27x18]; (b) the vision provides no concrete plan for WHO does this transcription, HOW it gets verified, or WHAT edition is used; (c) the vision simultaneously warns against contamination from triz40.com or Mann but provides no positive identification of which specific book, edition, and page the data will come from. This is not a 'risk to mitigate' -- it is the entire critical path, and the plan for it is 'source from a public-domain edition' without naming which one or confirming access to it.

## Unaddressed contradictions
- PHYSICAL CONTRADICTION THE VISION DODGED: The matrix data must be BOTH verifiably-from-Altshuller's-original (to satisfy the provenance/honesty commitment) AND practically-obtainable (to actually ship). These conflict because: the original Altshuller matrix was published in Russian-language editions; the English translations (Innovation Algorithm, 1999; Creativity as an Exact Science, 1984 English ed.) have known translation/typesetting variations; and the most accessible, already-digitized version IS the triz40.com one the vision forbids using. The vision treats data sourcing as a logistics task when it is actually a provenance-vs-obtainability physical contradiction it never frames as such -- ironic for a tool that dogfoods TRIZ.
- TECHNICAL CONTRADICTION: Improving the tool's honesty (surfacing dated-data caveats, empty-cell warnings, provenance tags on every output) worsens the VALUE PROPOSITION for the very users who might adopt it. The vision resolves this with 'Principle 22: Blessing in Disguise -- turn the weakness into a trust feature.' But this resolution is aspirational, not tested. The vault's own concept-triz-industrial-adoption documents that the field's trust problem is structural -- and 'transparent about its limitations' has never been a successful adoption driver for ANY developer tool in history. Developers adopt tools that solve problems, not tools that are honest about not solving them. The vision mistakes intellectual honesty for product-market fit.
- TECHNICAL CONTRADICTION THE VISION PAPERED OVER: The tool must be BOTH a lean-first MVP (three tables, seven commands -- the manifesto's Lean Iteration principle) AND a comprehensive SSOT worthy of the 'typed Single Source of Truth' branding. But a lookup table with 40 principles, 39 parameters, and a matrix -- with no examples, no sub-principle detail beyond lettered one-liners, no case study links, no framing guidance -- is not meaningfully different from a static JSON file on GitHub. The 'SSOT' framing implies authority and completeness; the 'lean MVP' reality is a thin shell over a lookup table. The vision never confronts this tension: it uses SSOT language to sell comprehensiveness while scoping to a minimal dataset that does not deliver on the promise.
- THE VISION NEVER ADDRESSES THE PARAMETER-MAPPING PHYSICAL CONTRADICTION: the user's problem description must be BOTH in their own domain language (how they think about it) AND in Altshuller's 39-parameter vocabulary (how the matrix indexes it). This is the core UX contradiction of every TRIZ tool and the reason framing is where users fail. The vision acknowledges this ('the matrix gives garbage if the mapping is wrong') then defers it entirely. The 'triz param find weight' fuzzy search is a cosmetic patch -- it helps you find a parameter by name, not figure out WHICH of your problem's aspects maps to WHICH parameter. The real contradiction (domain-specific vs. abstract vocabulary) is unresolved and unacknowledged as a contradiction.

## Adoption risks
- A BUSY ENGINEER WILL TRY IT ONCE AND ABANDON IT. Scenario: engineer installs triz, types 'triz matrix 14 1' (speed of an object vs weight), gets back 'Principles: 2, 28, 13, 38' with names. Now what? They read 'Principle 2: Taking out -- separate an interfering part or property from an object.' They stare at it. They do not know how to re-specialize this to their specific thermal management problem. The tool gave them the same four numbers they could have gotten from triz40.com in two clicks. The value-add over the status quo is approximately zero for the lookup itself -- the speed difference (50ms vs 5 seconds) does not matter for a task done once per project. They never open the tool again.
- THE DATED-MATRIX-CREDIBILITY PROBLEM IS WORSE THAN THE VISION ADMITS. The vision says the classical matrix is 'a starting set of directions.' But for a software engineer trying to resolve 'improving security worsens performance,' the 39 parameters (Weight of Moving Object, Length of Moving Object, Area of Moving Object...) are almost comically domain-mismatched. 30 of the 39 parameters are physical/mechanical. The matrix literally cannot express most software, organizational, or service-design contradictions. The vision's honesty about this ('may under-represent software/biotech/electronics') is an understatement that borders on dishonesty-by-euphemism. The correct statement is: 'this matrix was built for mechanical engineering and has near-zero applicability to software problems.' Surfacing a polite caveat does not fix the fact that the tool is irrelevant to the largest engineering population (software developers).
- THE METHOD-VS-IMPACT GAP MEANS THE TOOL CANNOT PROVE ITS OWN VALUE. The vision correctly identifies that TRIZ's impact evidence is unverifiable corporate anecdote. But this means: no testimonial, no case study, no metric can demonstrate that using 'triz' led to a better outcome than not using it. The tool ships 'the reproducible method as data' -- but the method's own evidence base shows method-completeness does not correlate with impact. The PCR case is the ONE fully documented application. ONE. Across all of TRIZ literature. The tool is a beautifully engineered delivery mechanism for a method whose real-world efficacy is supported by a single peer-reviewed case study.
- CARGO INSTALL IS NOT ZERO-FRICTION FOR THE TARGET USER. The vision claims 'the adoption barrier is zero: cargo install triz.' But R&D engineers (mechanical, electrical, chemical) do not have Rust toolchains installed. They are not CLI-native. They use SolidWorks, MATLAB, Excel. Asking them to install Rust to look up a matrix is like asking them to learn Latin to read a dictionary. The 'download a binary' alternative is mentioned in passing but not designed for -- no Homebrew formula, no apt package, no Windows installer, no standalone binary distribution plan. The actual adoption barrier for the stated primary human user segment is HIGH.

## What would make it real
- SHIP THE DATA AS A STANDALONE, LANGUAGE-AGNOSTIC ARTIFACT FIRST. Before writing a Rust CLI, publish the verified classical Altshuller matrix as a well-structured JSON or TOML file with full provenance metadata, under a clear license, in a standalone repository. This is the actual gap -- not the CLI wrapper. If the data artifact gets stars, downloads, and citations, THEN build the CLI on top of it. The Rust binary is premature optimization of a distribution problem that has not been validated.
- SOLVE THE PARAMETER-MAPPING PROBLEM OR DO NOT SHIP. The single highest-value feature would be interactive parameter mapping: 'describe your improving/worsening features in plain language, and the tool suggests candidate parameters with disambiguation.' This could be as simple as a curated synonym/keyword table per parameter (not an LLM feature -- a structured thesaurus). Without this, the tool is useful only to people who already know TRIZ notation, which is a tiny population that already has triz40.com.
- SOURCE AND VERIFY THE MATRIX DATA BEFORE ANYTHING ELSE. The entire vision is contingent on 1521 cells that do not exist. Before writing a single line of Rust domain code, the project needs: (a) a named, accessible primary source edition with ISBN and page reference, (b) a concrete transcription and dual-verification plan, (c) a cross-check against at least 5 known cell values from independent case studies (not just the one PCR case), and (d) a PROVENANCE.md committed to the repo before any code. If this step takes 40 hours of manual work, that should be acknowledged as the true MVP scope -- not the seven CLI subcommands.
- TARGET A REAL, REACHABLE USER SEGMENT WITH A CONCRETE WORKFLOW. Drop the 'AI agent' framing as the primary user -- it is aspirational vaporware serving aspirational vaporware. Instead, pick ONE concrete user workflow (e.g., 'a mechanical engineer using SolidWorks hits a weight-vs-strength trade-off during a design review') and design the tool end-to-end for that workflow, including: how they discover the tool, how they install it without Rust, how they map their problem to parameters, how they interpret the output, and how they take the next step. If the tool cannot serve one concrete person doing one concrete task better than triz40.com, it should not be built.
- HONESTLY SCOPE THE ADDRESSABLE PROBLEM SPACE. State clearly in the README and UX: 'This tool is designed for mechanical and physical engineering contradictions. The classical matrix has limited applicability to software, organizational, and service-design problems.' Do not hide behind 'may under-represent' -- state the boundary. This would actually BUILD the trust the vision claims to want, and it would prevent the most common failure mode (software engineer tries it, gets useless results, writes it off forever).

---

# Appendix B — Completeness Review

I have now completed my thorough investigation of the strategy document against the full vault (all 24 concept notes, 11 trend notes, 5 case studies, 16 source notes, all principle/parameter notes checked structurally) and the codebase. Let me synthesize my findings.

---

**VERDICT: ACCEPT-WITH-RESERVATIONS**

**Overall Assessment**: This is a genuinely strong strategy document -- well-grounded in the vault, honest about limitations, architecturally sound, and self-aware about its own contradictions. The dogfooding of TRIZ methodology is substantive, not performative. However, there are concrete gaps: several vault-documented TRIZ tools with real service potential are dismissed without adequate analysis, a critical data-provenance claim is unsourced, the principle data structure overpromises on extractability, and one user segment is absent entirely.

**Pre-commitment Predictions**: I expected to find (1) glossed-over licensing/provenance issues, (2) TRIZ tools dismissed without examination, (3) unsupported claims about vault data structure, (4) missing user segments, (5) ideality traps where the document's own honesty framework contradicts its lean scope. Findings: (1) confirmed -- the Savransky public-domain claim is not in the vault; (2) confirmed -- at least 4 vault-documented tools are dismissed in a single table row without analysis; (3) partially confirmed -- the `opposite` field claim is wrong; (4) confirmed -- TRIZ practitioners/consultants as integrators are absent; (5) partially confirmed -- the document acknowledges but does not fully resolve the "SSOT" branding vs. thin-data tension.

---

**Critical Findings** (blocks execution):

None. The strategy is sound enough to proceed to implementation planning.

---

**Major Findings** (causes significant rework):

**1. The Savransky public-domain claim is unverified -- the vault contains zero references to Savransky or "public domain".**
- Confidence: HIGH
- The strategy states: `"Public domain per Savransky/TRIZ Journal 1997"` and relies on this heavily for the entire data-sourcing legality. I searched the full vault for "Savransky," "public domain," and "license" -- zero results. This claim exists only in the strategy document itself, with no vault source note backing it. The strategy demands a `PROVENANCE.md` with exact sourcing, but the licensing foundation for the entire project is itself unsourced.
- Why this matters: If this claim is wrong or the Savransky article says something more nuanced (e.g., public domain for educational use only, or public domain in Russia but not under US/Berne copyright), the entire data-artifact strategy collapses. Soviet-era works have notoriously complex copyright status under the Berne Convention and bilateral treaties.
- Fix: Create a `source-savransky-1997-triz-journal.md` vault note. Obtain and read the actual Savransky 1997 TRIZ Journal article. Document the exact quote, its legal context, and whether it constitutes a copyright waiver or merely an expression of goodwill. Also investigate the copyright status of Soviet-era scientific works under current international law. This should be done before Milestone 0, not during it.

**2. The `Principle.opposite` field claim is wrong -- the vault frontmatter has no `opposite` field.**
- Confidence: HIGH
- The strategy states: `"opposite: Option<u8>, // Some(5) for Segmentation->Merging"` and claims this is extractable from vault frontmatter via `related_ids for opposite`. I verified: `principle-01-segmentation.md` has `related_ids: [concept-40-inventive-principles, concept-contradiction-matrix]` -- neither is principle-05. The opposite relationship exists only in the **prose body** (`"contrast with [[principle-05-merging|Merging]] (its opposite direction)"`), not in structured frontmatter. Only 4 of 40 principles mention opposites at all (1/5, 33/40, 38/39), and these are in prose `## Connections` sections, not frontmatter fields.
- Why this matters: The strategy claims principle data is "extractable TODAY from vault frontmatter." The `opposite` field requires either manual curation or prose-parsing for all 40 notes. The effort estimate for Milestone 0 (`~4 hours for parameters + principles JSON`) is too low if opposite-pair data must be manually researched and verified beyond the 4 documented pairs.
- Fix: Either (a) drop `opposite` from the Tier 0 `Principle` struct and add it in Tier 2 after manual research, or (b) add an `opposite_id` field to the vault principle note frontmatter for all 40 principles first, then extract. Acknowledge that only 4 pairs are currently documented.

**3. Four vault-documented TRIZ tools are dismissed in a single table row without any service-potential analysis.**
- Confidence: HIGH
- The "Explicitly NOT doing" table dismisses with one-line rationale:
  - **76 Standard Solutions**: `"Not enumerated as data in the vault."` -- FALSE. The vault note `concept-76-standard-solutions.md` has a structured 5-class breakdown with per-class counts (13, 23, 6, 17, 17). The case study `case-study-rock-breaking.md` documents a full worked example with subgroup reference (1.2). The dismissal is factually wrong about the vault's content.
  - **Su-Field Analysis**: `"Requires a modeling UI/language, not a lookup primitive."` -- Partially true, but the case study `case-study-furnace-conveyor.md` explicitly documents a **matrix + su-field verification workflow** where su-field analysis is used to filter matrix output. This is a composition pattern the strategy should at least acknowledge as a future query path.
  - **TESE Trends**: `"No number field in vault notes; prose-only; low lookup value."` -- The vault has 11 well-structured trend notes, each with a `parent_id`, `related_ids` linking to principles and concepts, and structured "Key Properties" sections with evolutionary progressions (e.g., `trend-increasing-dynamization.md` has a monolith-to-field progression). These are lookup-worthy data with clear structure.
  - **Trimming**: Not even mentioned in the "not doing" table. The vault has `concept-trimming.md` (a modern-TRIZ tool with structured rules) and `concept-function-analysis.md` (its input), plus an entire trend note (`trend-increasing-trimming.md`). Zero discussion of service potential.
- Why this matters: The strategy's own framing says `triz` should serve as a "typed, scriptable, agent-pipeable Single Source of Truth" for TRIZ. Dismissing tools that have structured data in the vault contradicts that SSOT ambition. Even if these are deferred, the strategy should at minimum (a) accurately describe what data exists, (b) articulate what service they could provide, and (c) defer them with honest rationale.
- Fix: Add an "Assessed and Deferred" section that accurately describes the vault data for each tool, states what service it could provide (e.g., "76 Standards: class-subgroup lookup by su-field model state, 5 classes with known counts"), and gives an honest deferral rationale (e.g., "requires the full 76-item enumeration, which is not in the vault and would need sourcing from Domb/Terninko/Miller 1999").

---

**Minor Findings** (suboptimal but functional):

1. **The "77/23 scope boundary" arithmetic is presented as vault-sourced but is derived, not primary.** The vault note `concept-levels-of-invention.md` gives percentages as 32/45/18/4/1. The strategy computes 32+45=77% (Levels 1-2) and 18+4+1=23% (Levels 3-5) and presents this as `"The 77/23 scope boundary (concept-levels-of-invention)"` -- as if it were a named concept in the vault. It is not. The vault calls the figures "32 / 45 / 18 / 4 / 1" and notes Levels 2 and 5 are "not primary-verified." The strategy should cite the raw figures and flag the derivation.

2. **The strategy says `"All 39 confirmed present and consistently structured"` for parameter notes.** I verified a sample: `parameter-27-reliability.md` has a one-sentence Overview. `parameter-01-weight-of-moving-object.md` has a one-sentence Overview plus a `## Connections` section. These are consistently structured, but the "definition" the strategy plans to extract (`definition: &'static str, // one-line from vault Overview`) is actually a full sentence with wiki-links like `"[[concept-39-engineering-parameters|39 Engineering Parameters]]"`. Parsing these into plain-text definitions requires stripping wiki-link syntax. Minor but the 4-hour estimate does not account for this.

3. **The `--output json` flag naming contradicts the existing codebase.** The strategy repeatedly refers to `--output json` but the actual CLI at `crates/cli/src/root.rs:8` uses `--output` with `OutputFormat::Text | OutputFormat::Json`, matching the existing pattern. The strategy is consistent with the codebase here, but then also proposes `--output json` alongside `--meta` and `-v` as new flags. The interaction between `--verbose` (already exists for debug logging at `root.rs:15`) and `-v` (proposed for sub-principle detail) is a collision. `-v` is already `--verbose`.

4. **The ARIZ-85C walkthrough note (`concept-ariz-85c-walkthrough.md`) is not referenced anywhere in the strategy.** This is a detailed 9-part procedure walkthrough that exists in the vault. Even if ARIZ is deferred from implementation, the strategy's References section should acknowledge the full vault content it considered.

5. **The concept notes `concept-system-operator.md` (9 Windows), `concept-smart-little-people.md`, `concept-super-system.md`, `concept-s-curve.md`, `concept-feature-transfer.md`, and `concept-function-analysis.md` are all absent from the strategy's analysis.** These are 6 vault concept notes with no mention. Some (System Operator, Smart Little People) are thinking tools rather than lookup tools and are fairly dismissed as out of scope. But `concept-function-analysis.md` and `concept-feature-transfer.md` are modern-TRIZ tools with structured properties that at least deserve acknowledgment in the "Assessed and Deferred" discussion.

---

**What's Missing** (gaps, unhandled edge cases, unstated assumptions):

- **User segment: TRIZ practitioners and consultants who would USE `triz` as part of their practice or embed it in their workflows.** The strategy lists AI agents, CLI engineers, patent analysts, and educators. It does not mention the working TRIZ practitioner/consultant -- someone who teaches workshops, runs facilitation sessions, and needs quick matrix lookups during live sessions. This person already knows the parameters by number and is the most natural power user of a fast CLI lookup. They are also the most likely to provide feedback on data correctness and to contribute alias expansions from real usage.

- **The furnace-conveyor case study documents a matrix + su-field verification workflow that is entirely unexamined as a service pattern.** Case `case-study-furnace-conveyor.md` shows that practitioners use the matrix output as *candidates* and then verify with su-field analysis. The strategy treats the matrix as the terminal step; it never discusses what happens downstream. Even a simple `triz matrix 27 18 --related-tools` that says "consider verifying via su-field analysis (see concept-su-field-analysis)" would add value. This is a composition pattern the document never considers.

- **No discussion of how the tool handles the well-documented asymmetry of the matrix.** The vault note `concept-contradiction-matrix.md` explicitly states: `"The matrix is asymmetric: improving A while B worsens != improving B while A worsens."` The strategy mentions this nowhere. When a user types `triz matrix 27 18` vs `triz matrix 18 27`, they get different results, and this is not intuitive. The tool should either (a) show both directions, (b) explain the asymmetry in `--meta` output, or (c) at minimum document this. The strategy's UX design does not address it.

- **No error handling design for the parameter-mapping step.** The strategy correctly identifies that "parameter mapping is where users fail" and defers guided framing to v2. But it does not discuss what happens when a user types `triz matrix reliability speed` and gets a result for [27, 9] -- is that what they meant? What if they meant [27, 25] (loss of time, not speed)? The alias-aware input is a good start, but when multiple parameters share related concepts (speed vs. loss of time vs. duration of action of moving object), the tool should at minimum warn about ambiguous matches. The strategy's `find_parameter` function signature returns `Vec<&Parameter>` but never discusses what happens when that vector has multiple results in the context of a matrix lookup.

- **The strategy never examines the vault's `concept-effects-database.md` for service potential.** This is a TRIZ tool that maps desired functions to physical/chemical/geometric effects -- and the vault note explicitly states it is `"organized by function, not by industry"` and directly attacks psychological inertia. While the actual database is massive and out of scope for v1, the concept of function-to-effect lookup is structurally identical to contradiction-to-principle lookup. The strategy should at least note this as a future data layer opportunity and assess whether the vault contains any enumerable subset.

- **The `data/` directory does not exist yet.** This is not a finding per se (the strategy proposes creating it), but the strategy discusses `data/*.json` as if the architectural decision is final without discussing the build-script mechanism. The strategy says `"via include! or a build script"` -- these are very different approaches with different implications for IDE support, compile times, and error messages. A `build.rs` that generates Rust source from JSON is a non-trivial piece of infrastructure that should be acknowledged as a development task.

- **No discussion of matrix cell ordering semantics.** The vault states: `"the order within a cell carries no preference ranking."` The strategy acknowledges this nowhere. When the tool returns `{11, 32, 13}` for cell [27,18], a user or agent might assume the first principle listed is the most relevant. The tool should either randomize order (to prevent false ranking inference) or explicitly state "no ranking" in output. This is a small but concrete UX/honesty gap.

- **The strategy does not address what happens when the tool's own data contradicts a user's existing TRIZ knowledge.** Different editions of the matrix have known transcription variants. If a user expects cell [X,Y] to contain principle P (because they learned from a different edition), and `triz` disagrees, the tool should have a way to surface which edition it sourced from -- which the `--meta` flag partially addresses, but the strategy does not discuss the specific scenario of user-data disagreement.

---

**Multi-Perspective Notes:**

- **Executor perspective**: The effort estimate of "25-35 hours" for Milestone 0 is plausible for the happy path but does not account for: (a) the Savransky copyright investigation, which could take days of legal research; (b) the possibility that the Altshuller editions are genuinely hard to obtain (both are out of print); (c) wiki-link stripping from vault definitions; (d) the `opposite` field requiring manual research. A more realistic estimate is 40-60 hours with a risk of the copyright investigation becoming a blocker.

- **Stakeholder perspective**: The strategy is honest about what it delivers and what it does not. The data-first framing (Decision 5) is the right call. The biggest stakeholder risk is that the 15-25 hour matrix transcription effort is tedious enough to stall the project indefinitely. The strategy should consider whether a community-sourcing approach (publish the empty schema + the 1 known cell, invite contributions with verification) could accelerate this.

- **Skeptic perspective**: The agent use case, while strategically compelling, is currently theoretical. The strategy cites two arXiv papers (TRIZ-GPT, TRIZ Agents) as evidence of demand, but these papers exist in the strategy document only -- they are not vault-sourced. If these papers don't actually identify a "deterministic local TRIZ data source" gap, the primary strategic positioning weakens. The CLI-engineer use case is real but niche. The data-artifact use case is the strongest defensible positioning because it doesn't depend on adoption predictions.

---

**Verdict Justification**: The strategy is well-crafted, intellectually honest, and architecturally sound. The dogfooding of TRIZ is substantive -- the five contradictions and their resolutions are not hand-waving. The vault grounding is mostly accurate. However, three issues prevent a clean ACCEPT: (1) the Savransky public-domain claim is the legal foundation of the entire project and is completely unverified in the vault -- this must be investigated before any data work begins; (2) the dismissal of 4+ vault-documented TRIZ tools is factually inaccurate about vault contents and strategically premature; (3) the `opposite` field claim is wrong, which undermines trust in the "extractable TODAY" assertions.

Review operated in THOROUGH mode throughout. Escalation to ADVERSARIAL was considered after finding the Savransky gap (a MAJOR finding), but the overall quality of the document is high enough that escalation would risk manufacturing outrage. The pattern is "a few blind spots in an otherwise rigorous document," not systemic carelessness.

Realist Check: Major finding #1 (Savransky) survives all pressure tests -- this involves legal/licensing risk with no mitigating factor. Major finding #2 (opposite field) is real but bounded in impact -- the fix is straightforward (drop the field from Tier 0). Major finding #3 (dismissed tools) is real but the deferral decision itself may be correct even if the rationale is sloppy -- the fix is to write honest rationale, not to change scope.

**Open Questions (unscored)**:

- The two arXiv papers cited (2408.05897, 2506.18783) are not in the vault and their claims are not verified. Do they actually identify the specific gap the strategy claims? LOW confidence this is a real problem -- the agent use case is defensible even without these specific citations -- but worth verifying.

- The MATRIZ wiki (`wiki.matriz.org`) is mentioned as a fallback matrix source. Is MATRIZ's interactive matrix tool actually available under CC? The strategy hedges (`"the specific CC variant is not pinned"`) but does not investigate. This could be important if the book-sourcing path fails.

- The strategy assumes the classical matrix has exactly 0-4 principles per cell. Is this actually confirmed across all 1521 cells, or could some cells have 5+? The vault says "up to 4" but this is not independently verified against a primary source.

- Could the trend notes' evolutionary progressions (e.g., dynamization: monolith -> field-based) serve as a lightweight "where is my system on this axis?" diagnostic tool? This might be higher-value than the strategy assumes, especially for the practitioner user segment.
