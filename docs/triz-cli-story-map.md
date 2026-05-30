# triz CLI — User Story Map & Triaged Release Plan

> Captured 2026-05-30. Produced by a 16-agent story-mapping workflow (Jeff-Patton
> backbone → stories → release slices), grounded in the project vault and the decided
> strategy. Triaged with TRIZ's own Ideality lens (useful ÷ (cost + risk)). The
> customer-advocate critique that rejected the planners' first cut — and forced the
> corrected walking skeleton — is reproduced in the appendix.

# TRIZ CLI -- User Story Map and Triaged Release Plan

## The Map

Activities are sequenced left-to-right as a practitioner's workflow. Stories are tagged by release band. De-duplicated: where multiple planner story IDs described the same capability, the single canonical story is listed once.

### Activity 1: Discover and Install

| Band | Story | Effort |
|------|-------|--------|
| **[WS]** | **Honest README and --help**: state what the tool does (classical matrix reference + lookup), what it does NOT do (no ARIZ, no guided solving, no Su-Field), and disclose the mechanical/physical domain bias of ~30/39 parameters | S |
| **[WS]** | **Offline-deterministic guarantee**: all data compiled into the binary as `&'static` constants; zero network calls; documented and tested invariant | S |
| **[MVP]** | **cargo install triz**: Cargo.toml metadata (description, keywords, categories, include/exclude), publish to crates.io | S |
| **[MVP]** | **PROVENANCE.md gate**: PROVENANCE.md in `data/` names exact source edition, licensing determination with citations, cross-verification methodology; ships with every release | L |
| **[v1.1]** | **Pre-built binaries**: CI release workflow builds linux-x86_64, macos-aarch64, macos-x86_64; attaches to GitHub Releases | M |
| **[v1.1]** | **Homebrew tap**: `brew install peiman/tap/triz` | S |

### Activity 2: Frame the Contradiction

| Band | Story | Effort |
|------|-------|--------|
| **[WS]** | **`triz help frame`**: concise explanation of what a technical contradiction IS ("to improve X via Y, Z worsened"), with the PCR worked example ([27x18] -> {11,32,13}); integrated into --help, not a separate subcommand | S |
| **[WS]** | **Domain-bias disclosure**: inline note on parameter/principle output that the classical matrix is derived from pre-1970 mechanical/physical patents; visible at point of use, not buried in a footer | S |

### Activity 3: Map to Parameters

| Band | Story | Effort |
|------|-------|--------|
| **[WS]** | **`triz parameters`**: all 39 parameters with number, canonical name, and one-line description; formatted table in text mode, typed JSON array in `--output json` mode | S |
| **[WS]** | **`triz parameter <id-or-name>`**: full detail for one parameter (number, name, description, domain-bias tag physical/abstract/hybrid, known aliases); accepts number, exact name, or case-insensitive prefix | S |
| **[WS]** | **`triz parameters --search <term>`**: search by substring on canonical name + description text + curated aliases; ranked results with all matches shown (never silent pick); disambiguation context for near-matches (e.g., "weight" shows both Parameter 1 and 2 with moving/stationary labels); zero-results message with domain-bias explanation and "try browsing the full list" guidance | M |
| **[WS]** | **Curated parameter alias thesaurus** (`data/parameters.json`): each of the 39 parameters carries 3-8 domain-vocabulary synonyms (e.g., "durability" -> 13/14/27, "throughput" -> 39, "latency" -> 25, "precision" -> 28/29). This is DATA WORK, not code. The vault currently has ZERO useful aliases (every parameter alias is just its canonical name). This thesaurus is the walking skeleton's load-bearing data dependency. Acceptance criteria: a practitioner searching "durability" finds Parameters 13, 14, and 27; searching "efficiency" finds Parameters 22, 25, 39; searching "noise" finds Parameter 31. | M |
| **[WS]** | **Diagonal detection**: if improving == worsening parameter, detect and explain this is a physical contradiction; name the 4 separation principles (time, space, condition, system level) inline; do not return matrix cell data | S |
| **[WS]** | **JSON output for all parameter commands**: stable typed envelope per the ckeletin output contract; search results include `{number, name, description, aliases, match_reason}` | S |

### Activity 4: Look Up Principles

| Band | Story | Effort |
|------|-------|--------|
| **[WS]** | **`triz principles`**: compact numbered list of all 40 principle names (usable as brainstorming checklist, recommended fallback for empty matrix cells) | S |
| **[WS]** | **`triz principle <id-or-name>`**: full detail -- number, canonical name, aliases, sub-principles (a/b/c/d), domain-bias caveat; accepts number or alias name (case-insensitive); invalid input (0, 41, "foo") returns clear error with valid range | S |
| **[WS]** | **`triz principle 11 32 13`** (batch): show multiple principles in sequence from a single invocation, mirroring the matrix-cell workflow (cells return a set of up to 4) | S |
| **[WS]** | **Principle JSON output**: typed object with `{number, name, aliases, sub_principles: [{letter, text}], domain_bias_caveat}` | S |

### Activity 5: Matrix Lookup

| Band | Story | Effort |
|------|-------|--------|
| **[MVP]** | **`triz matrix --improving 27 --worsening 18`**: look up by parameter number; return principle numbers with names and descriptions | M |
| **[MVP]** | **`triz matrix --improving reliability --worsening illumination-intensity`**: look up by parameter name or alias (reuses the parameter resolver from Activity 3) | M |
| **[MVP]** | **Ambiguous parameter input in matrix**: show all candidates with disambiguation context; do not silently pick | S |
| **[MVP]** | **Empty cell handling**: explicit "no statistically dominant principle" message with actionable fallbacks (40-principle checklist, re-frame, consider ARIZ); JSON returns `{principles: [], reason: "no_dominant_principle"}` | S |
| **[MVP]** | **Diagonal handling in matrix**: reuses diagonal detection from Activity 3; JSON returns `{principles: [], reason: "physical_contradiction", hint: "...separation principles..."}` | S |
| **[MVP]** | **Matrix JSON output**: typed envelope with `{improving_parameter, worsening_parameter, principles: [{number, name, description}], provenance: {source, year, domain_bias}}` | S |
| **[MVP]** | **Domain-bias provenance on every lookup result**: structured provenance metadata (source edition, year, domain-bias note) in both text and JSON output | S |
| **[MVP]** | **Incremental encoding support**: matrix JSON schema distinguishes three cell states: `[1,2,3]` = populated, `[]` = empty (no dominant principle), `null` = not yet transcribed; `triz matrix-status` shows coverage (e.g., "312/1482 non-diagonal cells encoded") | M |
| **[v1.1]** | **Batch multi-cell query**: `triz matrix --batch '[{"improving":27,"worsening":18},...]' --output json` | M |

### Activity 6: Interpret and Specialize

| Band | Story | Effort |
|------|-------|--------|
| **[WS]** | *(Covered by principle detail in Activity 4 -- sub-principles and examples enable interpretation)* | -- |
| **[MVP]** | **Separation principles detail**: `triz separation-principles` shows the 4 methods with brief explanation and one example each; fulfills the diagonal-redirect promise | M |

### Activity 7: Integrate and Script

| Band | Story | Effort |
|------|-------|--------|
| **[WS]** | **Stable JSON envelope contract**: every command returns `{status, command, data, error}` per the ckeletin output spec; exit 0 for success (including empty cells and diagnostics), non-zero for actual errors | S |
| **[MVP]** | **`triz verify`**: validates embedded data integrity -- all principle IDs 1-40, all parameter IDs 1-39, no cell > 4 principles, known reference cells match ([27x18] -> {11,32,13}) | M |
| **[MVP]** | **`triz provenance`**: returns source edition, licensing status, verification count, domain-bias note as structured JSON | S |
| **[v1.1]** | **`triz schema --output json`**: machine-readable description of all subcommands, arguments, and response shapes for agent auto-discovery | M |

### Activity 8: Verify Licensing (Maintainer)

| Band | Story | Effort |
|------|-------|--------|
| **[MVP]** | **Locate Savransky claim**: find and read the specific TRIZ Journal 1997 article cited as the "public domain" basis | M |
| **[MVP]** | **Soviet copyright research**: document actual legal status of pre-1971 Soviet publications, including UCC (1973) and Berne (1995) implications | L |
| **[MVP]** | **Data vs. expression distinction**: determine if numeric cell values (parameter-pair -> principle-numbers) are uncopyrightable facts vs. protected compilation | L |
| **[MVP]** | **English translation rights**: verify copyright status of parameter names and principle descriptions in English translations (Shulyak/Rodman 1999, Tate/Domb 1997) | M |
| **[MVP]** | **Survey existing redistributors**: document how triz40.com, MATRIZ wiki, GitHub repos handle licensing | M |
| **[MVP]** | **MATRIZ official position**: research whether MATRIZ claims IP rights over canonical TRIZ materials | M |
| **[MVP]** | **Go/no-go/conditional-go decision**: document licensing determination with fallback options (user-supplied data, fair-use framing, parameter/principle-only distribution) | M |
| **[MVP]** | **Write PROVENANCE.md**: full citations, legal reasoning, remaining risk areas, domain-bias disclosure | S |

### Activity 9: Source and Encode Matrix Data (Maintainer)

| Band | Story | Effort |
|------|-------|--------|
| **[WS]** | **Transcribe `data/parameters.json`**: 39 parameters with number, canonical name, one-line description, curated aliases, domain-bias tag; cited against MATRIZ Knowledge Base | M |
| **[WS]** | **Transcribe `data/principles.json`**: 40 principles with number, canonical name, aliases (from vault, 1-3 per principle), sub-principles; cited against Domb/Tate 1997 and triz40.com | M |
| **[MVP]** | **Select primary source edition**: pin one named Altshuller edition (ISBN, publisher, year, translator) in PROVENANCE.md; confirm matrix is physically printed with page numbers | M |
| **[MVP]** | **Transcribe matrix cells**: full 39x39 matrix from pinned edition into `data/matrix-classic-1971.json` (XL: 15-40h manual data labor) | XL |
| **[MVP]** | **Cross-verify >= 5 cells**: against independent sources (second edition, triz40.com, peer-reviewed case studies); record in PROVENANCE.md | M |
| **[MVP]** | **Compile JSON to `&'static` Rust constants**: build.rs or procedural macro; JSON remains SSOT, Rust binary is self-contained | M |
| **[MVP]** | **Data integrity test suite**: parameter count = 39, principle count = 40, matrix = 39x39, all cell principle IDs valid, no cell > 4 principles, known reference cells match | M |

### Activity 10: Publish and Release (Maintainer)

| Band | Story | Effort |
|------|-------|--------|
| **[MVP]** | **Cargo.toml publish metadata**: description, keywords, categories, include/exclude (exclude vault/, .ckeletin/) | S |
| **[MVP]** | **Version + data provenance in `--version`**: binary version + matrix edition + data provenance hash | S |
| **[v1.1]** | **CI release workflow**: on tag push, run `just check`, cross-compile, create GitHub Release, `cargo publish` | M |
| **[v1.1]** | **Release regression gate**: CI blocks release if PROVENANCE.md missing, data integrity tests fail, or cross-verification count < 5 | M |

---

## Ship-First Recommendation

**Ship the parameter/principle reference tool with a curated alias thesaurus.** This is the corrected walking skeleton that reconciles the planners (who correctly identified that parameter mapping is the highest-leverage capability) with the critique (which correctly identified that the alias data does not exist and must be created as data work before the code ships).

### What ships in the Walking Skeleton

1. **`data/parameters.json`** with 39 parameters, each carrying 3-8 curated domain-vocabulary aliases. This is the load-bearing data deliverable. Without it, search is `grep` on 1970s jargon. With it, a practitioner typing "durability" or "throughput" or "noise" finds the right parameter.

2. **`data/principles.json`** with 40 principles, sub-principles, examples, and the existing 1-3 translation-variant aliases from the vault.

3. **Five CLI subcommands**: `triz parameters`, `triz parameter <id>`, `triz parameters --search <term>`, `triz principles`, `triz principle <id|name> [id2] [id3]`.

4. **Guardrails**: diagonal detection (physical contradiction redirect with separation principles named inline), domain-bias disclosure on output, zero-results guidance, ambiguous-match disambiguation.

5. **JSON output** on every command via the existing `--output json` infrastructure.

6. **Honest README and `--help`**: what the tool does and does not do, including explicit domain-bias disclosure.

### What does NOT ship in the Walking Skeleton

- The matrix lookup (data-blocked: 0 of ~1521 cells transcribed, licensing unverified)
- The separation principles detail command (ship the inline redirect text, not a full subcommand)
- Pre-built binaries or Homebrew (distribution polish, not capability)
- The `triz help frame` pedagogical command (put this content in `--help` long text and README instead)
- Empty-cell guidance as a standalone feature (no cells exist yet)

### Why this reconciliation is correct

The critique was right: "alias-aware search without alias data is a checkbox on a plan, not a feature in a product." The planners were right: "parameter mapping is THE step where real users fail, and this is the cheapest lever." The corrected skeleton treats alias curation as a concrete, gated, data-work task with acceptance criteria -- not an assumed input.

The critique was wrong about one thing: it said the walking skeleton is "a man page" and "what `grep` on a markdown file does." That understates two genuine capabilities: (a) structured JSON output for agents (which grep on vault markdown does not provide), and (b) alias-aware search with disambiguation (which grep fundamentally cannot do -- grep on "weight" returns both Parameter 1 and 2 but does not explain the difference or rank them). The tool earns a second use by solving the mapping problem faster than reading the vault or browsing triz40.com, but ONLY if the aliases exist.

The critique's competitive-landscape argument ("triz40.com has all this for free") is valid but misses the agent persona. triz40.com is not scriptable, not offline, and not `--output json`. For a human practitioner, the walking skeleton competes with triz40.com on convenience only if alias search works well. For an AI agent, structured JSON over 39 parameters with aliases is genuinely new capability.

---

## Triage Rationale

### Band 1: Walking Skeleton (ship first)

**TRIZ Ideality: Useful Function / (Cost + Risk) = HIGH / (LOW + NEAR-ZERO) = maximum**

- **Value (HIGH)**: A practitioner can navigate the full TRIZ parameter/principle taxonomy, search for parameters using their own domain vocabulary, read principle details with sub-principles, and see honest domain-bias warnings. An agent can do all of this via structured JSON. This directly addresses the parameter-mapping failure mode that the decided direction identified as the primary user pain point.

- **Cost (LOW)**: The principle data is complete in the vault (40 notes with sub-principles, examples, 1-3 aliases). The parameter descriptions exist. The scaffold pattern is proven (ping.rs). The code work is 4-5 subcommands following the existing pattern (~3-5 days). The alias curation is the variable: ~200 entries (39 params x 5 aliases average), estimated 2-3 days of focused TRIZ domain work.

- **Risk (NEAR-ZERO)**: No licensing concern attaches to parameter names (uncopyrightable facts -- they are the functional vocabulary of the field), principle names (translation-standard terms), or independently-authored one-line descriptions. No matrix data is redistributed. No external dependencies.

- **Data dependency (gated)**: The alias thesaurus (`data/parameters.json` with the aliases field populated) MUST be complete and validated before the search feature ships. Acceptance criteria are concrete: "searching 'durability' returns Parameters 13, 14, 27" etc. This is treated identically to how the MVP treats matrix transcription: gate the code on the data.

### Band 2: MVP (ship second, gated by licensing decision)

**TRIZ Ideality: Useful Function / (Cost + Risk) = MAXIMUM / (VERY-HIGH + HIGH) = moderate**

- **Value (MAXIMUM)**: The complete classical TRIZ matrix method end-to-end: frame contradiction, map parameters (with alias search from WS), look up matrix cell, read suggested principles, apply. This is the product's reason to exist.

- **Cost (VERY-HIGH)**: Dominated by two non-code workstreams:
  - Licensing verification: L effort, legal research, ~7 stories forming a prerequisite chain. This work starts IN PARALLEL with the walking skeleton but must complete before any matrix data ships.
  - Matrix transcription: XL effort, 15-40 hours of manual data labor, ~1521 cells. This work starts AFTER licensing clears.

- **Risk (HIGH)**: Licensing could fail entirely. The go/no-go story forces a concrete fallback decision before the transcription investment. Fallback options: (a) ship a user-populated empty matrix (the tool helps you look up your own book), (b) ship parameter/principle lists only (the walking skeleton IS the product), (c) ship numeric cell data only (if data-vs-expression analysis shows numbers are uncopyrightable facts) with independently-authored descriptions.

- **Incremental data publication**: The MVP ships with partial matrix data honestly labeled (the incremental-encoding schema distinguishes "not yet transcribed" from "empty cell" from "populated"). Coverage grows over patch releases.

### Band 3: v1.1 (distribution and pipeline polish)

**TRIZ Ideality: Useful Function / (Cost + Risk) = MODERATE / (MEDIUM + LOW) = moderate**

Pre-built binaries, Homebrew, CI release workflow, batch queries, schema discovery endpoint, release regression gates. These widen the user funnel and improve agent-pipeline ergonomics but do not change what the tool can do.

### Band 4: Later (explicitly deferred)

ARIZ, 76 Standard Solutions, Su-Field analysis, TESE trends, guided "triz solve" wizard, MCP server, embedded AI, web/TUI, Mann Matrix 2003/2010. All explicitly out of scope per the decided direction.

---

## Sequenced Release Plan

### Increment 1: Walking Skeleton (weeks 1-3)

**User-visible outcome**: A practitioner types `triz parameters --search durability` and gets Parameters 13, 14, 27 ranked with descriptions. They pick Parameter 27 (Reliability), run `triz parameter 27` to confirm, then `triz principle 11 32 13` to read the three principles from the PCR case study. An AI agent does the same via `--output json` and gets structured typed data. The `--help` text honestly states what the tool does and does not do.

**Deliverables**:
- `data/parameters.json` (39 parameters with curated aliases -- the load-bearing data work)
- `data/principles.json` (40 principles with aliases, sub-principles, examples)
- 5 CLI subcommands: `parameters`, `parameter`, `parameters --search`, `principles`, `principle`
- Diagonal detection, domain-bias disclosure, disambiguation, zero-results guidance
- JSON output on all commands
- README rewritten for TRIZ (not the ckeletin scaffold boilerplate)

**Gate before shipping**: At least 3 blind user tests of the alias search -- a non-TRIZ-expert types a domain term and finds a relevant parameter within the top 3 results.

### Increment 2: Licensing Spike (weeks 1-4, parallel with Increment 1)

**User-visible outcome**: None directly. Internal deliverable: a go/no-go/conditional-go decision documented in `data/PROVENANCE.md` with full citations and fallback options.

**Deliverables**:
- Savransky 1997 article located and evaluated
- Soviet copyright status researched and documented
- Data-vs-expression analysis for numeric cell values
- English translation rights assessed
- Existing redistributor survey completed
- MATRIZ official position researched
- Go/no-go decision with fallback plan

**Gate**: Decision documented. If no-go, the project pivots to user-supplied-data model and the walking skeleton becomes the full product.

### Increment 3: MVP -- Matrix Lookup with Partial Data (weeks 5-10, starts after licensing clears)

**User-visible outcome**: `triz matrix --improving reliability --worsening illumination-intensity` returns principles {11, 32, 13} with full descriptions. `triz matrix-status` shows "87/1482 non-diagonal cells encoded (5.9%)." Empty cells and not-yet-transcribed cells are clearly distinguished. `cargo install triz` works from crates.io.

**Deliverables**:
- Primary source edition pinned with page numbers
- Matrix transcription begins (first priority: rows for the most commonly queried parameters)
- >= 5 cells cross-verified against independent sources
- `triz matrix` subcommand with ambiguous-input, empty-cell, and diagonal handling
- `triz separation-principles` detail command
- `triz verify` data integrity command
- `triz provenance` command
- JSON-to-static compilation (build.rs)
- Data integrity test suite
- cargo publish to crates.io

**Gate**: `just check` passes, >= 5 cells cross-verified, PROVENANCE.md complete, data integrity tests green.

### Increment 4: Full Matrix Data (weeks 10-16, ongoing transcription)

**User-visible outcome**: Matrix coverage grows from ~6% to ~80%+ through patch releases. `triz matrix-status` shows progress. Each patch release adds verified rows.

### Increment 5: v1.1 -- Distribution and Pipeline Polish (weeks 14-18)

**User-visible outcome**: `brew install peiman/tap/triz` works. `triz matrix --batch '[...]' --output json` supports multi-cell queries. `triz schema --output json` enables agent auto-discovery. GitHub Releases have pre-built binaries for macOS and Linux.

---

## The One Risk That Could Sink the First Release, and the Cheapest Way to Retire It

**The risk is NOT licensing.** Licensing can sink the MVP but cannot sink the walking skeleton, which redistributes no matrix data.

**The risk that could sink the walking skeleton is: the curated alias thesaurus is bad.** If the 200 alias entries are poorly chosen -- if "durability" does not map to the right parameters, if "efficiency" returns noise, if a software engineer searching "latency" gets nothing useful -- then the walking skeleton is the "glorified `cat`" the critique warned about. The alias data is the product. The code is the packaging.

**Cheapest way to retire this risk:**

1. **Curate aliases in a structured spreadsheet/JSON first, before any Rust code.** Have a TRIZ practitioner (or use the vault's descriptions + TRIZ textbook index terms) build the mapping. Budget 2-3 days.

2. **Blind-test the alias set against 10 realistic search queries before shipping.** The queries come from the decided direction's own examples: "durability", "throughput", "efficiency", "weight", "noise", "precision", "latency", "reliability", "complexity", "speed." For each query, the expected parameter(s) must appear in the top 3 results.

3. **Ship the alias data as a separate JSON artifact** (`data/parameters.json`), not hardcoded in Rust source. This lets the thesaurus be corrected and enriched without recompiling -- community contributors can submit alias PRs by editing JSON, not Rust. The JSON is compiled into `&'static` constants at build time (per the decided direction's DATA-FIRST principle), but the SSOT is the JSON file.

If the alias test fails for > 3 of the 10 queries, the thesaurus needs more domain-expert work before the skeleton ships. This is the honest gate: do not ship search without search data, and do not ship search data without validating it works.

---

## References

- `/Users/peiman/dev/triz/crates/cli/src/root.rs:1-35` -- Current CLI scaffold (only `ping` command)
- `/Users/peiman/dev/triz/crates/domain/src/ping.rs:1-44` -- Proven domain struct pattern (Serialize + Display)
- `/Users/peiman/dev/triz/.ckeletin/crate/src/output.rs:16-24` -- Envelope contract (status/command/data/error)
- `/Users/peiman/dev/triz/crates/cli/src/main.rs:56-60` -- subcommand_name pattern (exhaustive match)
- `/Users/peiman/dev/triz/crates/domain/Cargo.toml:14` -- Domain crate constraint: serde only
- `/Users/peiman/dev/triz/AGENTS.md:64-88` -- Adding a new command pattern (domain -> CLI handler -> root wire)
- `/Users/peiman/dev/triz/vault/concepts/parameter-01-weight-of-moving-object.md:8-9` -- Parameter alias = just canonical name (confirmed for all 39)
- `/Users/peiman/dev/triz/vault/concepts/principle-02-taking-out.md` aliases -- Principle aliases: 1-3 genuine variants (confirmed for all 40)
- `/Users/peiman/dev/triz/vault/concepts/concept-39-engineering-parameters.md:40-61` -- Canonical parameter list verified against MATRIZ
- `/Users/peiman/dev/triz/vault/concepts/concept-contradiction-matrix.md:60-65` -- Empty cell and diagonal guidance
- `/Users/peiman/dev/triz/vault/concepts/concept-separation-principles.md:31-38` -- Four separation methods
- `/Users/peiman/dev/triz/vault/concepts/case-study-pcr-diagnostics.md:39-47` -- Only verified matrix cell: [27x18] -> {11,32,13}
- `/Users/peiman/dev/triz/Justfile:8-9` -- `just check` is the single CI/local gateway
- `/Users/peiman/dev/triz/.github/workflows/ci.yml` -- CI runs `just check`, no release job
- No `data/` directory exists yet -- must be created

---

# Appendix — Customer-Advocate Critique of the Ship-First Cut

Good. The project builds. Now I have all the evidence I need. Here is my critique:

---

## CRITIC MODE -- Customer Advocate Review of the Proposed Ship-First / Walking Skeleton

There is no PLAN.md file in the repository. I was given three planner proposals inline. I will critique the consensus position across them.

### (1) Is the walking skeleton genuinely END-TO-END valuable, or does it ship the easy part while dodging the hard part?

**It ships the easy part.** All three planners dress this up differently, but the proposed walking skeleton is a static reference viewer. Let me be concrete about what a user session looks like with this slice:

```
$ triz parameters
 1  Weight of Moving Object
 2  Weight of Stationary Object
...
39  Productivity

$ triz parameter 14
Parameter 14: Strength
The ability of a system to resist deformation...

$ triz principle 1
Principle 1: Segmentation
a) Divide an object into independent parts...

$ triz parameters --search durability
No results.
```

That last line is the problem. The planners all claim "alias-aware search is THE lever against the framing problem" and then propose shipping it with **zero aliases**. I verified: every single parameter file has exactly ONE alias, which is just its canonical name. Parameter 27 "Reliability" has the alias "Reliability." Parameter 13 "Stability of the Object Composition" has the alias "Stability of the Object Composition." That is not alias-aware search. That is `grep`.

The entire planners' thesis rests on: "parameter mapping is where users fail, and alias-aware search fixes it." But the alias data that would make search useful DOES NOT EXIST in the vault. No parameter has a single domain-vocabulary alias (no "durability" -> 13/14/27, no "throughput" -> 39, no "precision" -> 28/29). Planner 1 claims aliases should be in the walking skeleton. Planner 2 admits the data does not exist and defers to MVP. Planner 3 claims it should be in the skeleton. None of them checked whether the data exists. It does not.

**Without aliases, the search is substring match on canonical 1970s Altshuller terminology.** A user who already knows that "durability" maps to Parameter 13 or 14 or 27 does not need the tool. A user who does NOT know that gets "No results" -- which is exactly the failure mode the planners identified and claimed to solve.

So the walking skeleton is: a numbered list of 39 parameters + 40 principles, viewable in the terminal, with substring search on canonical names only. That is a man page. It is what `grep` on a markdown file does.

### (2) Would a real human or agent actually use this the day it ships?

**No.** Here is the competitive landscape test:

- triz40.com: Free. Has all 40 principles with sub-principles and examples. Has the full matrix. Searchable. No install required.
- Wikipedia "TRIZ": Has the parameter list, the principle list, and a worked example. Free.
- Any TRIZ textbook: Has all of this and more.
- The vault itself: The 80 markdown files in `/Users/peiman/dev/triz/vault/concepts/` already contain everything the proposed walking skeleton would display, and they are already on the user's disk.

The proposed slice adds: (a) `--output json` formatting of the same data, and (b) offline terminal access. For the AI agent persona, `--output json` over 39 parameters and 40 principles is mildly useful, but an agent can parse the vault markdown files just as easily (they are already structured with YAML frontmatter). For the CLI practitioner, this tool has zero value over `grep -r "Parameter 14" vault/`.

**A tool earns a second use when it does something the user cannot do faster another way.** A static reference viewer for 79 items does not clear that bar.

### (3) Is the licensing/data-sourcing risk correctly placed?

**Yes, actually.** This is the one thing all three planners got right. The licensing verification is correctly identified as a hard gate before matrix data ships. The dependency chain (licensing clears -> transcription begins -> matrix lookup ships) is correct.

But by correctly placing it, the planners have painted themselves into a corner: the only feature that would make this tool genuinely useful (the matrix lookup) is blocked by the only risk they cannot control (licensing clearance). Rather than confronting this, they redefine "genuinely useful" to mean "a static reference viewer" -- which is not honest.

### (4) What is the smallest cut that would actually earn a second use?

The planners are right that the matrix lookup with 1 out of 1521 cells is dishonest theater. They are right that alias-aware search is the biggest lever. But they propose shipping search without the search data, which is equally dishonest.

**The correct first slice must include data that makes it work.** Here is my recommended corrected ship-first:

**Walking Skeleton (corrected):**

1. **Parameters with REAL aliases** (not the canonical-name-only stubs in the vault). This is ~200 entries of domain curation (39 params x 3-8 aliases each). Estimated effort: 2-3 days of focused domain work. This is DATA WORK, not code work, and it must happen BEFORE the code ships -- otherwise alias-aware search is a feature without fuel.

2. **Principles with sub-principles and examples** -- this data IS complete in the vault. Ship it.

3. **Alias-aware fuzzy search on parameters** -- the code for this is trivial (substring match on canonical name + aliases, maybe Levenshtein for typos). The value is entirely in the alias data, not the code.

4. **Domain-bias disclosure** -- costs nothing, builds trust, the planners are right about this.

5. **JSON output for all commands** -- the scaffold supports it, trivial to add.

6. **Diagonal detection** -- trivial guardrail, prevents a trust-destroying error.

7. **Start the licensing spike IN PARALLEL** -- do not wait for the walking skeleton to ship. The licensing research is independent work and is the true critical path for the product's reason to exist.

**What I would CUT from the planners' skeleton:**

- "Explain what a technical contradiction is" -- this is documentation, not a tool feature. Put it in `--help` or a README.
- "Separation principles detail command" (Planner 2 includes this) -- explicitly deferred per the decided direction.
- "Empty cell guidance" -- there are no cells yet. Ship guidance when cells exist.
- The 36 story IDs that are really 12 stories with 3 duplicate names each. The planners inflated the story count with synonymous IDs (e.g., "list-parameters" / "list-all-parameters" / "discover-parameter-list" are the same story).

**The hard truth the planners are avoiding:** The walking skeleton's value depends ENTIRELY on the quality of the alias thesaurus data, which does not exist yet. The code is trivial (days). The alias curation is the actual work (days to weeks of domain expertise). If the plan does not treat alias curation as a concrete, scheduled, gated data-work task -- with acceptance criteria like "a practitioner searching 'durability' finds Parameters 13, 14, and 27" -- then the skeleton ships as a glorified `cat` and the "alias-aware search" line item is a checkbox on a plan, not a feature in a product.

### Verdict on the three proposals:

All three planners converge on the correct strategic insight (ship the framing tool before the lookup tool, because the data for the lookup does not exist). But all three commit the same error: they claim the framing tool solves the framing problem while shipping it without the data (aliases) that makes it solve anything. Two of three claim aliases are in the skeleton; one honestly defers them. None verified that the vault has zero useful aliases.

The corrected skeleton is: alias data first, then code. The code is the easy part. The alias curation is the walking skeleton's load-bearing data dependency, just as the matrix transcription is the MVP's load-bearing data dependency. Treat them symmetrically: gate the code on the data, do not ship search without search data.

PLAN_REJECTED
