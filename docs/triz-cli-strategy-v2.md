# triz — Reframed Strategy (v2): Agent-Native, Guided TRIZ Substrate

> Captured 2026-05-30. Produced by a 13-agent TRIZ-dogfooding deliberation for the
> reframed vision (agent-native + guided + product-development skills→jobs; matrix
> demoted to legacy). Winner of a 3-judge panel: "triz — Guided Job Runner" (tally
> {"triz \u2014 Composable Operation Library": 132, "triz \u2014 Guided Job Runner": 140, "triz \u2014 TRIZ Co-pilot / Methodology Guardrail": 127, "triz \u2014 Agent-Native Guided TRIZ Substrate for Inventive Product Development": 131}). Adversarial critique and completeness review kept in full.

# triz -- Reframed Strategy

## An Agent-Native, Guided TRIZ Substrate for Inventive Product Development

---

## 1. The Service and Why

### What triz is

triz is a deterministic method substrate that exposes composable TRIZ skills -- chained into product-development jobs -- with a triage-spine guidance model that routes humans and agents to the right tool at the right moment and refuses premature solving. It is used WITH AI agents to attack new problems and build products with TRIZ in mind from the start, AND it guides anyone through the process and helps them do the right thing.

### Primary users

1. **AI agents used by product developers** -- the agent calls triz as a tool (CLI piping JSON now, MCP later) during inventive problem-solving. triz provides the deterministic structure the agent's own reasoning cannot reliably self-enforce.
2. **Human practitioners at a terminal** -- engineers, inventors, and product thinkers who want the modern TRIZ toolkit in a fast, offline, Socratic CLI rather than a proprietary desktop app or a training course.

### The existential question: what does triz add over an agent that already knows TRIZ?

The customer-advocate critique raises the hardest question honestly: a capable LLM with a good system prompt can state the IFR, walk through separations, name principles, and refuse compromises. Much of what the four designs propose is replicable by 50 lines of prompt engineering. That critique is correct, and the strategy must address it head-on rather than hand-wave.

**What triz genuinely adds -- thin but real, and sufficient:**

1. **Deterministic parameter search via a curated alias thesaurus.** An LLM mapping "download speed" to Parameter 9 is unreliable -- it might hallucinate a wrong parameter, and the caller would never know. A curated, blind-tested synonym table with deterministic scoring is verifiably more reliable than LLM vibes. This is a JSON file and a string matcher -- small, but it is the anti-friction front door for every downstream operation, and it is the one thing an LLM cannot do for itself: look up a verified, curated, tested mapping. (`concept-39-engineering-parameters.md` + the must-create thesaurus.)

2. **Deterministic routing that is auditable and reproducible.** Given the same inputs, the same routing decision every time. An LLM might classify the same contradiction differently on Tuesday than Monday. For audit trails, for chaining tools in pipelines, for verifier agents checking the solver agent's work, deterministic routing matters. The contradiction classifier (same param = physical, different params = technical, per `concept-contradiction-matrix.md:63-65`) is one if-statement -- but one that must fire correctly every time, not "usually."

3. **The method as composable, pipeable data -- not prose in a prompt.** A system prompt is monolithic and invisible: the agent either follows it or does not, and you cannot inspect, version, test, or compose the intermediate steps. triz exposes each method step as a typed, structured, independently-callable operation with deterministic input/output contracts. An agent can call `triz skill parameter-search "battery drain" --output json`, get a ranked list with confidence scores, pipe it into `triz skill contradict`, and get a classification with a `why` field and a `next` recommendation. Each step is inspectable, testable, and auditable. A system prompt cannot be piped.

4. **The guidance spine as an enforceable protocol, not a suggestion.** A system prompt says "always state the IFR first." The agent can ignore it. triz's guided-solve mode hard-gates: the state machine literally will not emit solution directions until the IFR and contradiction-type slots are filled. The enforcement is structural, not advisory. For agent builders who need their agents to be reliably rigorous (not just usually rigorous), this is the difference.

5. **A shared SSOT that multiple agents and humans can reference.** When a solver agent, a verifier agent, and a human reviewer all need to agree on "what TRIZ parameter does this map to?" or "which separation principles apply here?", a deterministic substrate with versioned data and inline provenance is the shared ground truth. A system prompt is per-agent and invisible to others.

**What triz does NOT add (honesty):**

- Teaching the 40 principles to an LLM (it already knows them with richer examples than a hand-transcribed JSON file).
- The Socratic conversation itself (LLMs do this natively; encoding it as a Rust FSM adds implementation cost but not capability).
- Fighting psychological inertia in the Altshullerian sense (PI is a human cognitive bias; LLMs have a different failure mode -- confident hallucination and step-skipping -- which the hard-gate protocol addresses but the PI-breaking tools like STC/9-Windows/SLP do not directly fix for agents).
- Being an "innovation oracle" or an "AI wrapper." triz is a deterministic substrate. It computes, classifies, routes, and gates. It does not reason, generate, or create.

**The design implication:** triz's value is concentrated in a narrow but real band: **verified reference data, deterministic classification and routing, composable typed operations, and enforceable protocol gates.** The strategy should build exactly that band and nothing more in iteration 1. Everything that is replicable by a system prompt (principle descriptions, Socratic pacing, pedagogical explanations) is deferred to the vault prose or left to the calling agent. This is itself a TRIZ move: apply trimming (`concept-trimming.md:36`) -- remove components and redistribute their functions to the caller.

### The "co-pilot riding alongside" framing

From Design 3: triz is not a pipeline owner that runs the whole show. It is a co-pilot that rides alongside any reasoning agent, injecting the right check, reference, or routing decision at the right moment. The agent retains full ownership of domain semantics and creative embodiment. triz provides the method's guardrails, not the method's content. This framing should inform the MCP design in iteration 2+: reactive, event-driven consultation, not a pre-composed pipeline.

---

## 2. The Skills-to-Jobs Model

### Composable skills (operations)

Each skill is an independently callable, deterministic operation following the `ping.rs` SSOT pattern at `crates/domain/src/ping.rs:6` -- a domain struct with `Serialize` + `Display`, a pure `execute()` function, data compiled to `&'static` from language-agnostic `data/*.json` with inline `PROVENANCE`. No LLM, no network, no framework imports (enforced by `crates/domain/Cargo.toml:8-14` which permits only serde).

Every skill response includes a `next` field (recommended next operation) and a `why` field (one-line rationale), even in direct-op mode. This is the lightest possible guidance layer for expert callers -- triz tells you what to consider next without forcing you to follow. (Grafted from Design 1.)

| Skill | What it does | Vault grounding | Data status |
|---|---|---|---|
| **principle-lookup** | Return principle N's name, sub-principles (a/b/c), examples | `principle-01-segmentation.md` through `principle-40-*` (40 notes with clean sub-principle structure) | HAVE -- transcribe to `data/principles.json` |
| **parameter-lookup** | Return parameter N's name, gloss, aliases | `parameter-01-*` through `parameter-39-*` (39 notes) | HAVE names/glosses; MUST CREATE alias thesaurus |
| **parameter-search** | Free-text to ranked candidate parameters via deterministic alias scoring | Depends on alias thesaurus | MUST CREATE (the load-bearing dataset) |
| **formulate-contradiction** | Classify a framed conflict as TC (two params) or PC (one param, two values); route | `concept-technical-contradiction.md:38-39`, `concept-contradiction-matrix.md:63-65` (diagonal insight) | HAVE logic rules; BUILD classifier |
| **suggest-separations** | For a PC, return applicable separations (time/space/condition/system-levels) with discriminating questions and principle subsets | `concept-separation-principles.md:31-38` (4 separations with examples); subset map flagged UNVERIFIED at line 44-46 | HAVE separations; MUST SOURCE subset map |
| **idealize** | State the IFR scaffold; compute ideality ratio (benefits / (costs + harm)) | `concept-ideality.md:34` (equation), `:38-40` (IFR-first mandate) | HAVE formula + criteria; BUILD calculator + template |
| **find-resources** | Walk the 6-category resource taxonomy as a structured checklist | `concept-resources.md:33-38` (6 categories x 3 readiness levels, voids, waste-as-resource) | HAVE taxonomy; BUILD checklist structure |
| **function-analysis** | Build/validate a function model (components + SAO functions, classified Useful/Harmful, Normal/Insufficient/Excessive) | `concept-function-analysis.md:34-38` (exact categories) | HAVE schema; BUILD Rust types + validator |
| **causal-chain** | Build a directed cause-effect graph, detect roots, find best intervention points | Referenced within ARIZ but no dedicated vault note (small content gap) | BUILD (graph logic is generic) |
| **trim** | Propose trimming candidates, apply MATRIZ rule ladder (A/X/B/C/D/E) | `concept-trimming.md:36-39` (rule order; lettering beyond A/B/C partially unverified) | HAVE rule order; BUILD engine |
| **fight-inertia** | Fire STC operator, 9-Windows grid, Smart Little People prompts on demand | `concept-psychological-inertia.md:34-36`, `concept-system-operator.md:37`, `concept-smart-little-people.md:31` | HAVE all three; BUILD prompt scaffolds |
| **validate-framing** | Check well-formedness before any solve: contradiction has named params, IFR stated, function model complete | Derived from constraint rules across concept notes | BUILD validators (pure logic) |
| **matrix-lookup** [LEGACY] | Classical 39x39 cell lookup | `concept-contradiction-matrix.md:72` (1 of ~1521 cells transcribed) | NOT POPULATED; optional data pack |

### Product-development jobs (composed skill chains)

Jobs are pre-composed sequences of skills for common product-development workflows, modeled after the opensourcetriz.com lifecycle proven over 25 years. Each job maps to the 5-question guidance spine (Section 3) and invokes skills at the right moments. Jobs are opinionated about sequence; skills are not.

| Rank | Job | Skills chained | What it prevents | Vault grounding |
|---|---|---|---|---|
| **1** | **Design and Prototype** | idealize -> formulate-contradiction -> suggest-separations (or causal-chain if Branch C) -> find-resources -> trim -> validate-framing (check vs IFR) | Agents accept trade-offs, never reach the physical contradiction | Densest TRIZ tool concentration; `concept-ariz-85c-walkthrough.md` Parts 1-5 |
| **2** | **Resolve Problems** | causal-chain -> formulate-contradiction -> suggest-separations -> fight-inertia (if stalled) -> validate-framing | Symptom-chasing, whack-a-mole fixes | `concept-ariz.md` (purpose-built for hard/non-standard problems) |
| **3** | **Create Offerings** | idealize (IFR FIRST) -> find-resources -> function-analysis (on incumbent) -> formulate-contradiction | Premature solving, skipping IFR, ignoring free resources | `concept-ideality.md:38-40` (IFR-first mandate) |
| **4** | **Reduce Burdens** | function-analysis -> idealize (baseline) -> trim -> find-resources -> idealize (delta) | Adding instead of removing | `concept-trimming.md`, `trend-increasing-trimming.md` |
| **5** | **Discover Markets** | function-analysis (on incumbent) -> idealize (where is ideality low?) -> fight-inertia (9-Windows) | Category inertia, "competitor list" instead of function analysis | `concept-function-analysis.md:38-39` |
| **6** | **Sell/License** | idealize (delta vs incumbents) -> principle-lookup (Levels of Invention gauge) | Overclaiming inventiveness, pitching features not ideality delta | `concept-levels-of-invention.md:32-38` |

---

## 3. The Guidance Model

### The triage spine: not a wizard

The guidance model resolves the rigid-wizard failure (ARIZ-85C has 9 parts, nobody finishes a 40-step form) by separating upon condition: branch depth on problem difficulty, not a fixed flow -- and between system levels: a fixed spine of 5 questions for everyone, with deep modules attached only when the spine detects they are needed.

**Critical build-order constraint (grafted from Design 3's risk):** Implement the spine as a simple linear sequence first. Validate with 3-5 worked examples from the vault case studies (`case-study-pcr-diagnostics.md`, `case-study-rock-breaking.md`, `case-study-chocolate-candy.md`, `case-study-furnace-conveyor.md`, `case-study-rabbit-enclosure.md`). THEN add branching and guards. The spine is a hypothesis about which questions matter -- treat it as one.

### The 5 questions

**S1 -- FRAME:** "Describe the situation in plain, jargon-free language. What is harmful, insufficient, or excessive?"
- ARIZ explicitly restates problems in everyday language to suppress psychological inertia (`concept-psychological-inertia.md:37-38`). Jargon imports the existing compromised solution into the problem statement.
- This is the single highest-value step. Even stopping here yields a clean problem statement.

**S2 -- IFR:** "State the Ideal Final Result: the deficiency is gone, all benefits stay, no new cost or complexity, ideally using resources already present. What does that outcome look like?"
- `concept-ideality.md:38-40`: IFR stated at the start, "countering psychological inertia and avoiding anchoring on existing, compromised solutions."
- No solution tool fires until IFR is on record. This gate is what separates TRIZ from brainstorming.

**S3 -- TRIAGE (the routing pivot):** "When you try to reach that outcome, what gets worse or stops you?"
- **(a)** Improving X makes Y worse [two different parameters] -> **Branch A: Technical Contradiction.** DEFAULT to surfacing the physical contradiction underneath (`concept-technical-contradiction.md:38-39`: "a TC often has a deeper physical contradiction as its root cause") and resolving with separation principles. The matrix is offered only as an optional legacy lookup with inline caveat.
- **(b)** X must be both high and low [one parameter, two opposite values] -> **Branch B: Physical Contradiction -> Separation Principles.** Walk the discriminating questions in order: (1) In time? (2) In space? (3) Upon condition? (4) Between system levels? Each question comes with a canonical example (`concept-separation-principles.md:31-38`: retractable landing gear / knife / sieve / bicycle chain).
- **(c)** A function is missing/harmful/weak with no clear trade-off -> **Branch C: Function Analysis + Causal Chain FIRST**, not contradictions. The modern-TRIZ front door. Output re-enters S3 once a real contradiction is exposed. "Analysis is not failure to solve -- it is how you find the right problem."
- **CRITICAL GUARD** (`concept-contradiction-matrix.md:63-65`): If improving and worsening parameter are THE SAME parameter, reroute from Branch A to Branch B. Unguided solvers constantly miss this.
- **"I cannot answer S3" is a valid input** and routes to function-analysis as the default starting tool, not rejection of the user. (Grafted from Design 2's risk about novel problem shapes.)

**S4 -- RESOURCES:** "What is already present -- system, components, environment, waste, voids -- that could do the job for free?"
- `concept-resources.md:29`: "the ideal solution is built almost entirely from existing resources."
- Asked BEFORE generating solutions to bias toward high-ideality answers.

**S5 -- SOLVE + CHECK:** Route to the matched tool, generate directions, then: "Does this satisfy the IFR from S2? Did it create a secondary problem?"
- `concept-ariz-85c-walkthrough.md:77`: ARIZ Part 7 quality-checks the solution concept against IFR-1 and any secondary problems introduced.
- Rejects compromises masquerading as solutions (`concept-technical-contradiction.md:37`: "resolve without compromise").

### Escalation and interrupts

**ARIZ escalation:** Opt-in, not default. Activates ONLY when (a) separation did not resolve it, (b) the problem is genuinely novel/non-standard, and (c) the user signals willingness to go deep. Then run the ARIZ-85C blocks from `concept-ariz-85c-walkthrough.md`. Most problems never need this -- saying so IS good guidance.

**Anti-fixation interrupts** (triggered by stall, not asked of everyone): STC operator (push Size/Time/Cost to 0 and infinity, `concept-psychological-inertia.md:35`), 9-Windows (3x3 grid, `concept-system-operator.md:37`), Smart Little People (`concept-smart-little-people.md:31`).

### How guidance differs for human vs agent callers

Same routing graph, different interaction contract. The decision graph (spine + branches + guards + state transitions) is a SINGLE deterministic engine in the Rust domain crate -- the SSOT. Two skins render it.

| Dimension | Human caller | Agent caller |
|---|---|---|
| **Pacing** | One question at a time, Socratic, conversational. Wait for answer before advancing. | Single structured call returns the whole decision graph + slots to fill. Agent fills slots and guide routes -- no turn-by-turn round trips. |
| **Output** | Prose with worked examples (knife, landing gear, sieve, bicycle chain) via `Display` impl | JSON with typed fields via `Serialize`: `{state, questions[], route, why, expected_inputs, tool_output}` |
| **Enforcement** | Lean toward less depth; offer "go deeper?" opt-ins | Hard-gate: refuse to emit solution directions until IFR + contradiction-type slots are filled. Require the agent to cite which resource/parameter each suggestion rests on (forces grounding). |
| **Why field** | Explanatory paragraph (humans need to trust the move) | Terse rationale tag (agents need audit trail, not persuasion) |
| **Trust posture** | Human knows the DOMAIN but not TRIZ. Guide teaches. | Agent knows TRIZ vocabulary but will skip steps and over-claim. Guide enforces. |
| **Key asymmetry** | Guidance fights FIXATION | Guidance fights STEP-SKIPPING and UNGROUNDED CONFIDENCE |

This reuses the existing ckeletin Output pattern at `crates/infrastructure/src/lib.rs:1-4` (re-exports `output::OutputMode::Human|Json`) and `crates/cli/src/main.rs:90-94` (dispatch on mode). The architecture is already built for this separation.

### Failure modes the guidance prevents

1. **Premature solving** -- S1-S2-S3 gated; no solution tool fires until IFR + contradiction type are stated.
2. **Bad framing / jargon-locked problem** -- S1 forces plain-language restatement.
3. **Wrong tool** -- S3 triage pivot + same-parameter reroute + Branch C "analyze first."
4. **Accepting compromise as solution** -- S5 checks concept against IFR and rejects "we met in the middle."
5. **Solution creates secondary problem** -- S5 explicit secondary-problem check.
6. **Anchoring on existing system** -- IFR backward-reasoning (S2) + resources-first (S4).
7. **Matrix-as-oracle** -- route demotes matrix to optional/legacy, always says "the matrix points a direction; you supply the embodiment."
8. **Wizard fatigue** -- 5-question spine + conditional depth; ARIZ is opt-in.

### The completion test

Every spine step MUST produce a usable artifact even if the user stops there. Value is monotonically increasing, never "finish or it is worthless":
- After S1 alone: a clean, jargon-free problem statement.
- After S1+S2: problem statement + IFR (already more than most brainstorming yields).
- After S1-S3: problem + IFR + classified contradiction with routing decision.
- After S1-S4: problem + IFR + contradiction + resource inventory.
- After S1-S5: problem + IFR + contradiction + resources + solution concept checked against IFR.

---

## 4. How We Use the Data and Method

### The three-lane separation (the central architectural decision)

This is the load-bearing distinction. Every operation splits into three lanes, cleanly mirroring the existing scaffold's domain/infrastructure/cli split (`crates/domain/Cargo.toml:8-14` enforces framework isolation; `crates/infrastructure/src/lib.rs:1-4` re-exports Output; `crates/cli/src/root.rs` owns presentation).

**LANE D -- Deterministic domain (Rust `crates/domain/`, the SSOT).** Closed reference data and pure-logic transforms. Same input, same output, offline, no LLM. This is what triz uniquely provides and what an agent cannot reliably reproduce from training data.

What is LOOKED UP (static reference data, compiled to `&'static` from `data/*.json` with inline PROVENANCE):
- 40 principles + sub-principles + examples (transcribed from vault `principle-01` through `principle-40`)
- 39 parameters + glosses + curated alias thesaurus (the must-create dataset)
- 4+ separation principles with per-separation principle-subset map (must source/verify)
- Resource taxonomy (6 categories x 3 readiness levels, from `concept-resources.md:33`)
- Ideality formula and IFR checklist criteria (from `concept-ideality.md:34`)
- ARIZ 9-part step graph (from `concept-ariz-85c-walkthrough.md`)
- STC/9-Windows/SLP prompt scaffolds
- Matrix cells [LEGACY, optional, pluggable data pack]

What is COMPUTED (pure logic, same input -> same output):
- Contradiction classification (TC vs PC, including diagonal detection)
- Parameter search scoring (token/alias match over the thesaurus)
- Function model validation (SAO completeness, dangling-reference detection)
- Ideality ratio arithmetic (benefits / (costs + harm))
- Causal chain graph operations (root detection, cycle check, intervention-point ranking)
- Trimming rule priority ladder application over a function model
- Guided-solve state machine transitions (state + input -> next state + questions + route)
- Validation gates (well-formedness checks before any solve operation)

**LANE A -- Left to the calling agent/human (open-world judgment no table can hold).** triz MUST NOT fake this with a lookup:
- Reading the messy situation and naming actual components/functions
- Deciding which parameter a real quantity maps to (triz offers ranked candidates via parameter-search; the caller disambiguates when confidence is low)
- Asserting cause-effect relationships ("X causes Y")
- Inventing the concrete embodiment of an abstract principle
- Identifying alternative systems for feature transfer
- The aspirational phrasing of the IFR for the specific problem

**LANE G -- Guidance/orchestration (deterministic process control over Lanes A + D):**
- Which question to ask next (the spine sequence)
- Which tool is appropriate now (the S3 routing pivot)
- Whether the agent's framing is well-formed (validation gates)
- Refusing premature solutioning (IFR gate)
- Checking solutions against IFR and catching secondary problems (S5)
- Firing inertia-breaking prompts on stall detection
- Requiring provenance citations from agent callers

**The rule:** triz owns the method's invariants (D) and the method's choreography (G); the agent/human owns the domain semantics (A). A guided agent does better TRIZ precisely because G forces it through D-validated gates instead of jumping straight to an answer.

### Where the vault sits

The vault (`/Users/peiman/dev/triz/vault/concepts/`) is the prose/teaching layer the guide CITES but never copies into compiled data. The vault explains (the "why" and worked examples drawn into the human skin's Display impl); the domain crate computes. This preserves two important properties: (a) the vault is a living document that gets corrected and expanded by human practitioners, independently of the compiled binary; (b) no copyrighted prose is compiled into the static data -- only unprotectable facts (principle names, parameter names, formula, separation categories).

### Where the legacy matrix sits

The classical 39x39 Altshuller Contradiction Matrix is DEMOTED to optional/legacy status. This is grounded in three converging lines of evidence:

1. The vault's own critique (`concept-contradiction-matrix.md:86-92`): "Built from patents predating ~1970, so critics argue it is biased toward mechanical-era inventions and under-represents software/biotech/electronics."
2. opensourcetriz.com (25-year practitioner resource): plainly states they do not use the contradiction matrix but separation principles instead.
3. The reframe's pragmatic analysis: the matrix requires ~1521 cells of manual transcription, carries unresolved licensing questions, and its value is strictly dominated by the modern path (formulate-contradiction -> suggest-separations -> principle-lookup) for most problems.

Concrete role:
- A PLUGGABLE, SEPARATELY-SOURCED DATA PACK behind a `--legacy` flag. Not built into the core, not on the critical path.
- The DEFAULT route for technical contradictions is: surface the physical contradiction underneath and resolve with separation principles.
- When offered, always carries provenance and bias disclosure.
- The principle and parameter LISTS (names, numbers, glosses) are unprotectable facts -- used freely. Only the CELL CONTENTS carry potential thin-compilation claims and require sourcing from a named public-domain Altshuller edition.
- This dissolution retires the prior ADR's biggest risk (`decision-cli-service-strategy.md`, already marked superseded at line 4-8).

### The data-sourcing story (now small)

| Dataset | Status | Effort | Blocking? |
|---|---|---|---|
| 40 principles + sub-principles/examples | HAVE in vault; transcribe to JSON | Low | Yes (Tier 0) |
| 39 parameters + glosses | HAVE in vault; transcribe to JSON | Low | Yes (Tier 0) |
| Parameter alias thesaurus (3-8 synonyms x 39) | MUST CREATE from scratch | Medium (the real work) | Yes (Tier 0, gates parameter-search) |
| 4 separations + examples | HAVE in vault | Low | Yes (Tier 0) |
| Separation-to-principle subset map | Flagged UNVERIFIED (`concept-separation-principles.md:44-46`); must source | Medium | Yes (Tier 0) |
| Resource taxonomy | HAVE in vault | Low | No (Tier 1) |
| ARIZ step graph | HAVE in vault | Low | No (Tier 2) |
| STC/9-Windows/SLP scaffolds | HAVE in vault | Low | No (Tier 1) |
| 76 individual standard solutions | Only 5-class skeleton in vault | High (data acquisition) | No (Tier 3, not doing now) |
| Effects database entries | Zero data in vault | High | No (Tier 3, not doing now) |
| Matrix cells (~1521) | 1 cell transcribed | High + licensing spike | No (optional, off critical path) |

### Dual-source truth and the sync problem

The customer-advocate critique correctly identifies a SSOT risk: the vault and the JSON are two sources for the same data, connected only by manual discipline. The resolution: the `data/*.json` files are the SSOT for compiled logic. The vault is the SSOT for prose and pedagogy. They contain DIFFERENT representations of the same facts -- the JSON carries typed, versioned, provenanced data; the vault carries explanatory prose. When a vault note is corrected, the corresponding JSON entry is re-transcribed. This is a manual sync cost that is low for 40+39 entries and acceptable for iteration 1. If the dataset grows, a build-time validator that cross-checks JSON entry counts against vault note counts is a cheap guard.

---

## 5. TRIZ, Dogfooded

### triz's Ideal Final Result

Per `concept-ideality.md:29` -- "the function is performed but the machine is absent."

The IFR of triz: the user (human or agent) arrives with a messy product-development problem and -- without needing to know TRIZ terminology, without choosing the wrong tool, without skipping the IFR, without accepting a compromise as a solution -- ends up doing rigorous inventive work. The guide itself disappears: the right TRIZ tool fires at the right moment with the right framing, using resources already present (the vault knowledge, the agent's domain reasoning, the deterministic method data), adding no complexity to the user's workflow.

Ideality = (all TRIZ benefits: contradiction resolution, IFR-backward reasoning, cross-domain effects, anti-inertia) / (zero: no TRIZ expertise required, no wizard fatigue, no wrong-tool misroutes, no licensing liability).

The honest check: if a system prompt achieves the IFR, the tool has trimmed itself out of existence -- which, per the ideality equation, IS the ideal outcome. The strategy must prove (by experiment, not by architecture) that the deterministic substrate adds enough over a system prompt to justify its existence. This experiment is the first item in the roadmap.

### The key contradictions, resolved

**Contradiction 1: Guided rigor vs agent speed.**
- *Statement:* The guidance must enforce methodological rigor (IFR-before-solving, contradiction-classification-before-resolution, framing-validation) to prevent the classic failure modes. But enforcement means more steps, more round-trips, more latency. One parameter -- interaction depth -- must be simultaneously deep (for rigor) and shallow (for speed). This is a physical contradiction.
- *Resolution: Separation upon condition* (`concept-separation-principles.md:35-36`) of problem difficulty via a triage gate. The 5-question spine is always available as hard gates, but each operation is ALSO callable independently without the spine. Simple problems: the agent calls `suggest-separations` or `principle-lookup` directly -- zero gate overhead, maximum speed. Hard problems or novice callers: the guided-solve state machine enforces the full spine, gating each step on well-formedness validation. The spine is segmented (Principle 1, `principle-01-segmentation.md`) into independently-valuable steps (value is monotonic). For agents specifically: a single JSON call returns the full decision graph with all slots, collapsing the interactive cost to one round trip. Dynamization (Principle 15): the system adapts its enforcement level to the caller's declared mode (expert/guided) and the problem's complexity signal.

**Contradiction 2: Opinionated method vs flexible composition.**
- *Statement:* triz must be opinionated enough to prevent wrong-tool selection, premature solving, and accepted compromise. But it must also be flexibly composable so expert agents can invoke any operation directly. Improving methodological safety worsens composability -- two different parameters.
- *Resolution: Separation between system levels* (`concept-separation-principles.md:37-38`). At the PARTS level (individual skills), each is unopinionated -- a pure function that takes structured input, returns structured output, and expresses no preference about what called it or what calls next. At the WHOLE level (the guided-solve orchestrator and the job pipelines), the system is highly opinionated -- it enforces the spine order, validates framing, refuses premature solutioning, and routes to the correct tool. The parts are segmented and independently callable; the whole merges them into an opinionated sequence. Like the bicycle chain from `concept-separation-principles.md:38`: flexible as a whole, rigid per link. Expert agents compose the rigid links in any order. Novices follow the flexible-but-opinionated chain. The opinion lives in the orchestration layer, never in the operations themselves.
- *Agent surface:* `triz skill <name>` (direct, ungated access to any skill) and `triz job <name>` (guided job runner with gates). Same domain functions power both -- no code duplication (Principle 5, Merging).

**Contradiction 3: Comprehensive toolkit vs lean shippable product.**
- *Statement:* The full modern TRIZ toolkit has 15+ operations and 6+ data tables. But shipping everything before anything means shipping nothing, and the ideality equation says maximize function while minimizing cost.
- *Resolution: Separation in time* (`concept-separation-principles.md:31-32`). Ship the highest-value operations FIRST (the ones that close the biggest agent-failure-mode gaps). The walking skeleton tests the full vertical slice (domain op -> CLI subcommand -> `--output text|json`) on simple operations before investing in complex ones. Each tier is usable without the next. Build the data infrastructure (the `data/*.json` + Rust static-compile pattern from `ping.rs`) correctly from iteration 1, so later operations slot in without rework (Principle 10, Prior Action). Apply trimming to scope: the effects database and 76-standard enumeration are NOT built first because they require large data-acquisition efforts -- their functions are redistributed to the vault prose and the calling agent's own knowledge (`concept-trimming.md:36`, rule A: the object performs the function itself). This is the single best TRIZ-dogfood move in the strategy. (Grafted from Design 4.)

**Contradiction 4: Human-friendly guidance vs agent-friendly protocol.**
- *Statement:* Humans need Socratic pacing, worked examples, emotional reframing. Agents need terse JSON, hard gates, batch slot-filling. One guidance engine must serve both. The interaction style must be simultaneously prose-rich and terse-structured.
- *Resolution: Separation between system levels* (`concept-separation-principles.md:37-38`). The decision graph is ONE deterministic state machine in the domain crate -- the SSOT. Two skins render it. The CLI's text mode formats each state as Socratic prose; the JSON mode returns the full state machine as structured JSON. This is already the pattern the scaffold uses (`crates/domain/src/ping.rs:6` -- `Serialize` + `Display` on the same struct): `Serialize` serves agents, `Display` serves humans, the domain logic is identical. Another Dimension (Principle 17): MCP adds a third surface without changing the engine.

**Contradiction 5: Deterministic substrate vs adaptive guidance.** (Grafted from Design 4 -- this names a tension the other designs handle implicitly but never surface.)
- *Statement:* The method's invariants (IFR-before-solving, reach-the-physical-contradiction, no-compromise) must be deterministic and reproducible. But real problem-solving is adaptive: the user's situation is messy, partial, and evolves.
- *Resolution: Separation in time WITHIN each step of the spine.* The SEQUENCE is rigid and deterministic (the state machine enforces: Frame before IFR, IFR before Triage). The CONTENT within each step is adaptive: the agent fills slots with messy, evolving input, and the substrate validates well-formedness but does not prescribe domain semantics. Dynamization (Principle 15): the spine is a rigid skeleton with flexible joints. The state machine supports backward transitions (re-enter S3 after Branch C analysis surfaces a real contradiction) -- deterministic in which transitions are legal, adaptive in which path the problem actually takes.

---

## 6. Scope and Lean Roadmap

### Pre-code validation (before any Rust)

**0a. Run the core-premise experiment.** Take 3-5 real inventive problems (the vault's case studies: `case-study-pcr-diagnostics.md`, `case-study-rock-breaking.md`, `case-study-chocolate-candy.md`). Run them through: (A) a bare LLM with no TRIZ prompting, (B) an LLM with a TRIZ system prompt containing the 5-question spine and separation questions, (C) the proposed triz tool (simulated manually by looking up the data and applying the routing rules by hand). Compare output quality. If B is 80%+ as good as C, the tool's value proposition is a synonym dictionary and a routing table -- build only those. If B is significantly worse, the tool matters and the full strategy is justified. This costs 2 hours and is worth more than 200 hours of Rust development on an unvalidated premise. **This is the single most important item in the entire roadmap.**

**0b. Build `data/parameters.json` with the alias thesaurus.** 39 entries, 3-8 aliases each. Created in a plain JSON file, not a Rust crate. Blind-test with 20 real-world queries (10 is too few). If the thesaurus is genuinely useful, it has value independent of the CLI -- agents can consume the JSON directly. If you cannot build a good thesaurus, the entire project is moot.

**0c. Walk 3-5 vault case studies through the spine manually.** Write the 5 spine questions as a markdown checklist. Walk the problems through. Note where the spine helps, where it is friction, and where it routes incorrectly. The spine is a hypothesis -- validate it before encoding.

### Tier 0 -- Walking skeleton (ship first)

- Transcribe `data/principles.json` (40 entries) and `data/parameters.json` (39 entries + aliases) from vault notes. Inline PROVENANCE per entry.
- Source the separation-to-principle-subset map against a primary MATRIZ reference. Mark provenance and confidence per row. Ship only verified subsets; flag gaps honestly.
- Domain operations following the `ping.rs` pattern: `principle-lookup`, `parameter-lookup`, `parameter-search`, `formulate-contradiction` (classifier), `suggest-separations`, `idealize`.
- CLI subcommands: `triz skill principle <N>`, `triz skill parameter <N>`, `triz skill search "<query>"`, `triz skill contradict ...`, `triz skill separate ...`, `triz skill idealize ...`. All with `--output text|json`.
- Every JSON response includes `next` and `why` fields.
- Record the new ADR superseding `decision-cli-service-strategy.md`.

### Tier 1 -- The analytical hub

- `function-analysis` (FunctionModel struct + SAO validator + disadvantage extractor)
- `find-resources` (resource taxonomy checklist)
- `trim` (trimming rule ladder over function model)
- `causal-chain` (directed graph with root detection)
- `fight-inertia` (STC/9-Windows/SLP prompt scaffolds)
- `validate-framing` (well-formedness gates)

### Tier 2 -- Guided flows

- The 5-question guidance spine as a domain FSM (implemented as a simple linear sequence first, branching/guards added after case-study validation).
- `triz job design-prototype` -- the first guided job, composing Tier 0+1 skills through the spine. Human mode (Socratic) and JSON mode (batch slot-fill).
- `triz job resolve-problem` -- the second guided job.

### Tier 3 -- Depth and breadth

- Full ARIZ-85C wizard (all 9 parts as opt-in guided flow)
- Su-field model + apply-standards (depends on 76 Standards data)
- Feature-transfer diff logic
- Evolve-via-trends (TESE trend positioning + next-state)
- MCP server crate (`crates/mcp/`, 4th workspace member sharing the domain crate)

### Not doing

- Matrix cell data transcription (~1521 cells) -- off critical path, behind licensing spike, optional/legacy.
- Effects database population -- substantial data acquisition, Tier 3+.
- 76 individual standard solutions -- only 5-class skeleton ships; individual entries are a data-creation gap.
- Web UI / TUI / embedded AI.
- Mann Matrix 2003 or any copyrighted third-party data.
- The 40-principle descriptions as compiled data beyond name/sub-principles/examples -- the LLM already knows these; triz adds the structured lookup, not the content.

---

## 7. Open Decisions for the User

### Decision 1: Run the core-premise experiment before building?

The customer-advocate critique argues the tool's value proposition reduces to a synonym dictionary and a routing table, and that a TRIZ system prompt replicates 80%+ of the guidance value. The strategy recommends running the A/B/C experiment described in Section 6 (0a) BEFORE writing any more Rust. This costs 2 hours. If the experiment shows the system prompt is nearly as good, the scope shrinks to parameter-search + formulate-contradiction as the entire v0.1 -- and the guidance spine, jobs, and state machine are not built. If the experiment shows the deterministic substrate adds significant value, the full strategy proceeds. This is the highest-leverage decision.

### Decision 2: Ship MCP alongside or after the CLI?

The design defers MCP to Tier 3. But the primary user persona (AI agents) consumes MCP, not CLI pipes. Deferring MCP means deferring the primary user's native interface. The counterargument: the CLI's `--output json` mode already serves agent-pipeable use cases, the domain crate's API surface needs to stabilize on a few operations before committing to an MCP contract, and MCP is architecturally trivial once the domain ops exist (just a new presentation skin). The decision: ship the CLI + JSON first and test whether agents actually get value from piped JSON before investing in MCP. But if the experiment (Decision 1) shows the deterministic substrate matters, consider promoting MCP to Tier 1 -- a minimal MCP server with 2 tools (parameter-search + formulate-contradiction) alongside the CLI, to get real agent usage data early.

### Decision 3: Alias thesaurus sourcing strategy?

The thesaurus is the make-or-break dataset. Two options:
- **(A) Curate manually from domain knowledge.** The builder (you) drafts 3-8 synonyms per parameter, blind-tests with 20 queries, iterates. Fast, closed-loop, but risks coverage gaps invisible until real users hit them.
- **(B) Seed from the vault + LLM-assisted expansion, then manually verify.** Use an LLM to propose candidate synonyms per parameter, then curate and verify each one. Broader coverage, but requires a verification pass to prevent garbage aliases.

Recommendation: B, with the verification pass as the quality gate. The blind-test (20 queries, expected parameter in top-3 at 90%+ hit rate) is the acceptance criterion regardless of sourcing method.

### Decision 4: How to handle the separation-to-principle subset map?

The vault explicitly flags this as UNVERIFIED outside MATRIZ (`concept-separation-principles.md:44-46`). Three options:
- **(A) Ship only verified subsets from the MATRIZ primary reference.** Safest, but may leave gaps. Honest about confidence.
- **(B) Ship the Domb/Mann per-separation lists with an UNVERIFIED confidence flag.** Broader coverage, but risks giving wrong principle recommendations.
- **(C) Ship without subsets initially; the separation questions alone are valuable.** The discriminating questions (time? space? condition? system levels?) guide the user to the right separation; the principle subset is a convenience, not a necessity.

Recommendation: A, falling back to C for any separation whose subset cannot be verified. Never ship unverified data without a prominent confidence flag.

### Decision 5: Scope of the "triz job" abstraction?

The jobs are modeled after opensourcetriz.com's proven lifecycle. But opensourcetriz.com's jobs are human facilitation workflows run by experienced TRIZ consultants over days/weeks. Encoding them as deterministic CLI pipelines assumes the sequential skill-chaining captures the value. This assumption is untested. Two options:
- **(A) Build jobs from the start (Tier 2).** The guidance spine naturally composes skills into job-shaped flows. Jobs are the product-development story that differentiates triz from a reference tool.
- **(B) Defer jobs; ship only skills + the spine.** Let users and agents compose their own workflows. Observe which compositions emerge, then codify the proven patterns as jobs.

Recommendation: A, but with the build-order constraint from Design 3's risk: validate the spine with 3-5 case studies first, then add jobs. Ship `design-prototype` as the first job (rank 1 for agent value) and observe whether it gets used before building the other five.

### Decision 6: The over-engineering circuit breaker

The codebase today has 1 domain struct, 1 CLI subcommand, 0 data files. The strategy proposes 13 skills, 6 jobs, a state machine, 7+ JSON data files. The ratio of design to working code is extreme. The circuit breaker: **after each tier ships, the user must observe at least one real use (personal or external) before greenlighting the next tier.** If Tier 0 skills are not used by anyone (including the builder's own agent workflows), the project is a personal learning exercise, not a product -- and that is fine, but the investment level should match.

---

**Key files referenced in this strategy:**
- `/Users/peiman/dev/triz/crates/domain/src/ping.rs` -- the SSOT pattern every operation follows
- `/Users/peiman/dev/triz/crates/domain/src/lib.rs` -- domain module registry (currently: `pub mod ping;`)
- `/Users/peiman/dev/triz/crates/domain/Cargo.toml:8-14` -- framework isolation enforcement
- `/Users/peiman/dev/triz/crates/infrastructure/src/lib.rs` -- Output re-exports
- `/Users/peiman/dev/triz/.ckeletin/crate/src/output.rs` -- Envelope + OutputMode::Human|Json
- `/Users/peiman/dev/triz/crates/cli/src/root.rs` -- CLI args + Commands enum
- `/Users/peiman/dev/triz/crates/cli/src/main.rs:90-94` -- output mode dispatch
- `/Users/peiman/dev/triz/vault/decisions/decision-cli-service-strategy.md` -- superseded ADR
- `/Users/peiman/dev/triz/vault/concepts/concept-ideality.md:34,38-40` -- ideality equation + IFR mandate
- `/Users/peiman/dev/triz/vault/concepts/concept-separation-principles.md:31-38,44-46` -- separations + UNVERIFIED flag
- `/Users/peiman/dev/triz/vault/concepts/concept-contradiction-matrix.md:63-65,72,86-92` -- diagonal insight, single cell, critique
- `/Users/peiman/dev/triz/vault/concepts/concept-psychological-inertia.md:34-38` -- PI + STC + SLP
- `/Users/peiman/dev/triz/vault/concepts/concept-ariz-85c-walkthrough.md` -- 9-part guided flow
- `/Users/peiman/dev/triz/vault/concepts/concept-function-analysis.md:34-38` -- function model categories
- `/Users/peiman/dev/triz/vault/concepts/concept-resources.md:29,33-39` -- resource taxonomy
- `/Users/peiman/dev/triz/vault/concepts/concept-trimming.md:36-39` -- trimming rule ladder
- `/Users/peiman/dev/triz/vault/concepts/concept-technical-contradiction.md:37-39` -- TC properties + PC root
- `/Users/peiman/dev/triz/vault/concepts/concept-physical-contradiction.md:29-34` -- PC definition
- `/Users/peiman/dev/triz/vault/concepts/concept-system-operator.md:37` -- 9-Windows grid
- `/Users/peiman/dev/triz/vault/concepts/concept-smart-little-people.md:31` -- SLP method
- `/Users/peiman/dev/triz/vault/concepts/concept-levels-of-invention.md:32-38` -- 5 levels
- `/Users/peiman/dev/triz/vault/concepts/concept-76-standard-solutions.md:36-42` -- 5-class skeleton
- `/Users/peiman/dev/triz/vault/concepts/concept-s-curve.md` -- S-curve stages
- `/Users/peiman/dev/triz/vault/concepts/concept-feature-transfer.md` -- feature transfer method

---

# Appendix A — Customer-Advocate Critique

**Verdict:** THE DESIGN IS INTELLECTUALLY IMPRESSIVE AND ALMOST CERTAINLY OVER-BUILT FOR ITS ACTUAL VALUE PROPOSITION.

The positive: The reframe away from the matrix toward modern TRIZ (separation principles, function analysis, IFR-first) is genuinely correct and well-grounded in the vault evidence. The three-lane separation (deterministic data / agent reasoning / guidance orchestration) is a clean architectural insight. The contradiction analysis applied to the tool's own design is rigorous and self-aware. The scope discipline (buildFirst vs notDoing) is honest. The demotion of the matrix dissolves the biggest prior risk cleanly.

The fatal problem: The design has not answered — and has avoided answering — the existential question: does a compiled Rust binary add enough value over an LLM with a good TRIZ system prompt to justify the engineering investment? The unique data contribution reduces to a parameter synonym dictionary and an unverified separation-to-principle mapping. Everything else the tool proposes to do (IFR enforcement, spine gating, Socratic questioning, principle lookup, inertia-breaking prompts) is replicable by 50 lines of system prompt. The design document itself is evidence of the problem: it is a 15,000-word architectural specification for a tool whose irreducible core is a curated synonym file and a routing table.

The recommendation: STOP designing and START testing. (1) Run the A/B/C experiment described above to validate the core premise. (2) Build data/parameters.json with the alias thesaurus. (3) Ship the two genuinely unique operations (parameter-search and formulate-contradiction) as a minimal CLI + MCP tool. (4) Walk 5 case studies through the spine manually. THEN decide whether the guidance state machine, the 6 jobs, the 12 composable skills, and the elaborate tiered build plan are worth building — based on evidence, not architecture.

The tool's own method demands this: state the IFR, refuse premature commitment, build from existing resources. The existing resource here is the LLM's own TRIZ knowledge. The IFR is 'the user does rigorous TRIZ without needing triz.' If a system prompt achieves that IFR, the tool has trimmed itself out of existence — which, per concept-ideality.md:29, IS the ideal outcome. Test whether it does before building the thing that might not need to exist.

**Guidance reality-check:** This is the hardest and most important question. Let me be blunt.

AN LLM AGENT ALREADY HAS TRIZ IN CONTEXT AND DOES NOT NEED MOST OF THIS.

Here is what a capable LLM (Claude Opus, GPT-4) can already do without any external tool, right now, for free:

1. State the IFR first and reason backward — if you tell it to in the system prompt.
2. Classify a contradiction as technical or physical — it knows the distinction.
3. Walk through separation principles in order (time, space, condition, system levels) — it knows all four with canonical examples.
4. Name the 40 principles and their sub-principles — it has them memorized from training data.
5. Name the 39 parameters — same.
6. Walk through a Socratic 5-question problem-framing sequence — this is what LLMs do natively, no state machine required.
7. Refuse to accept compromises and check solutions against the IFR — if instructed.
8. Fire inertia-breaking prompts (9 Windows, STC, Smart Little People) — it knows all of these.

So WHERE does the guidance actually add value over a smart agent winging it with a good TRIZ system prompt?

GENUINE VALUE (thin but real):
- The parameter alias thesaurus: an LLM mapping 'download speed' to Parameter 9 is unreliable — it might pick the wrong parameter, and you would never know. A curated, tested synonym table with deterministic scoring is genuinely more reliable than LLM vibes. This is real value. But it is a JSON file and a string matcher, not a guided state machine.
- Deterministic reproducibility: given the same inputs, the same routing decision every time. An LLM might route differently on Tuesday than Monday. For audit trails and regulatory contexts, deterministic routing matters. But this is a niche use case, not the mass adoption story the design tells.
- The separation-to-principle-subset map (IF verified and correct): an LLM might hallucinate which principles map to which separation. A verified lookup is more trustworthy. But the vault flags this map as UNVERIFIED, so this value does not exist yet.

CLAIMED VALUE THAT IS ACTUALLY HOLLOW:
- 'Hard-gating the agent through the spine' — an agent with a well-written system prompt does this without an external tool. The 'hard gate' is the prompt instruction 'Do not propose solutions until you have stated the IFR and classified the contradiction type.' That is free.
- 'Refusing premature solutioning' — same. A prompt instruction.
- 'The 40 principles as static data' — the LLM already has these with richer examples than a hand-transcribed JSON file will contain.
- 'The guidance state machine' — a Socratic conversation is what LLMs do natively. Encoding it as a Rust FSM adds implementation cost but not capability. The agent does not need a compiled binary to ask 5 questions in order.
- 'Fighting psychological inertia' — this is the deepest irony. The design claims to fight PI, but PI is a HUMAN cognitive bias. LLMs do not have psychological inertia in the Altshullerian sense — they do not have domain expertise that anchors them to familiar solutions. They have a DIFFERENT failure mode: confident hallucination and step-skipping. A system prompt handles step-skipping. Hallucination is not addressed by any of the proposed tools.

THE HONEST ANSWER: the guidance adds genuine value in exactly two narrow bands: (1) deterministic parameter search via a curated alias thesaurus, and (2) deterministic routing that is auditable and reproducible. Everything else — the spine, the gates, the Socratic pacing, the principle lookups, the separation questions, the IFR enforcement — is replicable by a well-crafted system prompt at near-zero cost. The design document is 15,000+ words of architecture for what is, at its core, a synonym dictionary and a routing table.

## Fatal flaws
- THE CORE VALUE PROPOSITION HAS NOT BEEN TESTED AND MAY BE HOLLOW. The design claims that 'an LLM has TRIZ in its training data but will confidently skip the IFR, jump to solutions, accept trade-offs, and over-claim inventiveness' — and that triz fixes this by hard-gating the agent. But nobody has tested whether this is actually true. A capable LLM given a system prompt saying 'Always state the IFR first, always identify the physical contradiction, never accept a compromise, always check the solution against the IFR' would do exactly what the 5-question spine does — for free, with zero Rust code, zero data transcription, and zero CLI overhead. The design assumes agents CANNOT self-discipline with TRIZ in their prompt/context. That assumption is the entire load-bearing premise and it has never been verified. Before building ANY of this, someone needs to run a controlled experiment: give Claude/GPT a TRIZ system prompt with the spine questions, run it on 5 real problems, and compare the output quality to an unguided agent. If the system-prompt version works 80% as well, the entire Rust substrate is over-engineering a solved problem. This is a FATAL verification gap because the design spends months of build effort on a premise that costs 30 minutes to test.
- THE DETERMINISTIC DATA VALUE IS THINNER THAN CLAIMED. The design repeatedly asserts that Lane D (deterministic data) is 'what triz UNIQUELY provides and what an agent cannot reliably reproduce from training data.' But examine what Lane D actually contains: (a) 40 principle names and sub-principles — an LLM already knows these verbatim. (b) 39 parameter names and glosses — an LLM already knows these. (c) 4 separation principles with examples — an LLM already knows these. (d) The ideality equation — trivially known. (e) A contradiction classifier ('same param = physical, different params = technical') — this is one if-statement that any LLM can execute. (f) A resource taxonomy (6 categories x 3 readiness levels) — known. The ONLY genuinely unique data assets the design identifies are: the parameter alias thesaurus (which does not exist yet) and the separation-to-principle-subset map (which the vault flags as UNVERIFIED). So the design proposes building a substantial Rust application whose unique data contribution is a synonym list and an unverified mapping table. Everything else is already in the LLM's training data with higher fidelity than a hand-transcribed JSON file.
- NO DATA DIRECTORY EXISTS AND NO DATA HAS BEEN CREATED. The design says 'build and test the alias thesaurus data BEFORE any Rust code' (Prior Action, Principle 10). But the repo has zero data files — no data/ directory, no parameters.json, no principles.json, no alias thesaurus. The entire Tier 0 scope depends on data that has not been started. The design has been through multiple deliberation cycles producing increasingly elaborate architectural documents while the foundational data work (which the design itself identifies as the single highest-priority task) remains at exactly zero. This is a process smell: the planning is substituting for the doing.

## Unaddressed contradictions
- THE DESIGN DOGFOODS TRIZ TO DESIGN TRIZ BUT MISAPPLIES ITS OWN METHOD. The design identifies 4 contradictions and resolves all of them. But it dodges the BIGGEST physical contradiction of the entire project: the tool must be USEFUL ENOUGH to justify its existence (high function) AND SIMPLE ENOUGH that people use it instead of just prompting an LLM (low friction). This is the existential contradiction — and it is a physical contradiction (one parameter, 'tool complexity/capability,' must be simultaneously high and low). The design never states this contradiction, never resolves it, and instead builds an elaborate system that pushes hard on 'high function' (12 composable skills, 6 jobs, a state machine, MCP, CLI) while hand-waving 'low friction' with 'the spine is only 5 questions.' But the friction is not the 5 questions — the friction is INSTALLING A RUST BINARY, LEARNING A NEW CLI, AND PIPING JSON instead of just adding 20 lines to your system prompt.
- THE HUMAN-VS-AGENT SEPARATION IS UNDER-RESOLVED. The design says 'same engine, two skins' and claims this resolves the human/agent contradiction via separation between system levels. But the GuidanceState struct must carry enough information for BOTH Socratic prose AND structured JSON. The design acknowledges this tension in risks ('if too terse, human mode loses pedagogical value; if too verbose, agent mode carries unnecessary payload') but does not actually resolve it — it just says 'the why field is the bridge.' That is not a resolution; it is a hope. In practice, the Display impl for a GuidanceState that serves both modes will be either a thin wrapper around the struct fields (bad for humans) or a rich prose generator (wasteful for agents). The 'two skins' architecture has never been tested against a real problem to see if the shared struct actually works for both.
- THE 'JOBS' ABSTRACTION HAS NO VALIDATION. The design defines 6 product-development jobs modeled after opensourcetriz.com's proven structure. But opensourcetriz.com's jobs are HUMAN FACILITATION workflows run by experienced TRIZ consultants over days/weeks. Encoding them as deterministic CLI pipelines (function-analysis PIPE trim PIPE idealize) assumes that the sequential skill-chaining captures the value of the facilitated workflow. This assumption is never tested. A 'triz job design-prototype' that mechanically chains 7 skills may produce output that looks like a TRIZ analysis but lacks the judgment a facilitator brings at each transition. The jobs may be cargo-culting the opensourcetriz structure without the substance.
- THE DESIGN NEVER ADDRESSES WHO THE ACTUAL USER IS AND WHETHER THEY EXIST. It says 'AI agents and human practitioners.' But: (a) AI agents today do not install CLI tools and pipe JSON — they use MCP or function-calling, and the MCP surface is explicitly deferred. (b) Human TRIZ practitioners already have commercial tools (IFR, Goldfire, CREAX) or use facilitator-led workshops — why would they switch to a CLI? (c) Engineers who do not know TRIZ will not search for 'triz' on crates.io — they do not know they need it. The design has no distribution or discovery strategy. It is building a tool for a user persona that may not exist in sufficient numbers to justify the engineering investment.

## Adoption risks
- COLD START PROBLEM FOR BOTH PERSONAS. Human practitioners who know TRIZ already have their tools and workflows. Engineers who do not know TRIZ will not discover a CLI tool called 'triz.' AI agents cannot use the tool until the MCP surface ships (deferred). The design has zero distribution strategy, zero discovery mechanism, and zero migration path from existing workflows. The only realistic early adopter is the builder themselves — which is fine for a personal tool but contradicts the 'guide ANYONE' aspiration.
- THE CLI IS THE WRONG SURFACE FOR THE PRIMARY USE CASE. The design's own analysis says the highest-value job is 'Design and Prototype' — a creative, iterative, messy process. A CLI that returns JSON envelopes is the wrong interaction modality for this. The Socratic human mode (one question at a time, wait for input) recreates a chatbot inside a terminal — but the user already HAS a chatbot (the LLM they are talking to). The natural interaction pattern is: user talks to their LLM, the LLM calls triz as a tool behind the scenes. But that requires MCP, which is deferred. So Tier 0 ships a surface that serves neither persona well: too mechanical for humans, not integrated enough for agents.
- THE ALIAS THESAURUS IS A COLD DATA PROBLEM. The design correctly identifies it as the make-or-break dataset. But building a good alias thesaurus requires knowing how REAL users describe their problems — and the project has zero users. The thesaurus will be built from the builder's intuition about what synonyms matter, blind-tested against the builder's own test queries. This is a closed loop. Without real usage data, the thesaurus will have coverage gaps that are invisible until real users hit them. The 'blind test with 10 queries' mitigation is better than nothing but is not a substitute for real-world usage telemetry.
- MAINTENANCE BURDEN OF HAND-CURATED DATA. The design proposes hand-transcribing 40 principles, 39 parameters, 11 trend ladders, separation data, resource taxonomies, ARIZ step graphs, and more into JSON files with inline PROVENANCE. This is substantial initial work AND ongoing maintenance. When the vault notes are corrected or expanded, the JSON must be re-transcribed. This is a SSOT violation — the vault and the JSON are TWO sources of truth for the same data, connected only by manual discipline. The design acknowledges 'the vault explains; the domain crate computes' but does not address how they stay in sync.
- OVER-ENGINEERING RISK IS EXTREME. The codebase today has: 1 Rust domain struct (PingResult), 1 CLI subcommand (ping), 0 data files, 0 tests beyond the ping smoke test. The design proposes: 12 composable skills, 6 jobs, a guidance state machine, 7+ JSON data files, 2 output modes, future MCP, and a tiered multi-month build plan. The ratio of design documentation to working code is approximately 100:1. This is a strong signal that the project is in 'architecture astronaut' territory — building the perfect system on paper while the actual software does nothing beyond health checks.

## What would make it real
- RUN THE EXPERIMENT THAT TESTS THE CORE PREMISE BEFORE WRITING ANY MORE RUST. Take 5 real inventive problems (use the vault's own case studies). Run them through: (A) a bare LLM with no TRIZ prompting, (B) an LLM with a TRIZ system prompt containing the 5-question spine, and (C) the proposed triz tool (simulated manually since it does not exist). Compare output quality. If B is 80%+ as good as C, the tool's value proposition is a synonym dictionary, not a guided substrate. If B is significantly worse than C, you have evidence the tool matters. This test costs 2 hours and is worth more than 200 hours of Rust development on an unvalidated premise.
- BUILD THE ALIAS THESAURUS FIRST — IN A PLAIN TEXT FILE, NOT A RUST CRATE. The design says 'Prior Action: build the data before the code.' Follow through. Create data/parameters.json with 39 entries and 3-8 aliases each. Test it with 20 real-world queries (not 10 — 10 is too few). Publish the thesaurus as a standalone artifact. If the thesaurus is genuinely useful, it has value independent of the Rust CLI — agents can consume the JSON directly. If you cannot build a good thesaurus, the entire project is moot.
- BUILD THE SIMPLEST POSSIBLE THING THAT DELIVERS THE UNIQUE VALUE. The unique value is: (1) deterministic parameter search, (2) deterministic contradiction classification and routing. Build those two operations and NOTHING ELSE. Skip the spine, skip the jobs, skip the state machine, skip the 40-principle transcription (the LLM already knows them). Ship 'triz search <query>' and 'triz contradict --improving N --worsening M' as the entire v0.1. If those two commands get used, expand. If not, you have learned cheaply.
- SHIP AN MCP TOOL BEFORE OR ALONGSIDE THE CLI. The design defers MCP to 'iteration 2+ when the domain crate API surface is stable.' But the primary user persona (AI agents) consumes MCP, not CLI pipes. Deferring MCP means deferring the primary user. The domain crate's API surface IS stable enough after 2 operations — parameter-search and formulate-contradiction do not need 12 skills to prove the interface. Ship a minimal MCP server with those 2 tools and test whether agents actually call them and get value. Real agent usage data is worth more than architectural speculation about what agents need.
- VALIDATE THE GUIDANCE SPINE AGAINST REAL PROBLEMS BEFORE ENCODING IT AS A STATE MACHINE. Write the 5 spine questions as a markdown checklist. Walk 3-5 real problems through it manually. Note where the spine helps, where it is friction, and where it routes incorrectly. THEN decide whether to encode it in Rust. The spine is a hypothesis about which questions matter — treat it as one. The ARIZ-85C walkthrough in the vault is a good source of test problems, but the case studies (PCR diagnostics, rock breaking, chocolate candy, furnace conveyor, rabbit enclosure) are the real validation material.

---

# Appendix B — Completeness Review

All file references verified. Now I have complete grounding. Let me synthesize the review.

---

**VERDICT: ACCEPT-WITH-RESERVATIONS**

**Overall Assessment**: This is an exceptionally well-reasoned strategy document -- the best kind of honest, self-critical technical planning. It correctly identifies the existential question ("what does this add over a system prompt?"), proposes a validation experiment before committing code, and dogfoods TRIZ on itself with genuine insight. The reservations concern specific gaps and under-examined areas the strategy misses, not structural problems.

**Pre-commitment Predictions**: Before deep reading, I predicted: (1) the "guidance" value claim would be over-asserted relative to what a system prompt achieves -- PARTIALLY CONFIRMED, the strategy addresses this honestly but Section 3's guidance model still over-engineers relative to the experiment gate; (2) the data-sourcing effort for the alias thesaurus would be under-examined -- CONFIRMED; (3) the SoC boundary (what is deterministic vs what requires LLM judgment) would have fuzzy edges -- PARTIALLY CONFIRMED in specific cases; (4) the vault-to-JSON sync would be hand-waved -- the strategy acknowledges it but the mitigation is weak; (5) the MCP deferral would create a chicken-and-egg problem with the stated primary user being agents -- CONFIRMED, the strategy sees it but under-weights it.

**Critical Findings** (blocks execution):

None. The strategy's own experiment gate (Section 6, 0a) is the correct critical-path item and is correctly identified.

**Major Findings** (causes significant rework):

1. **The guidance spine (Section 3) is over-specified relative to the experiment gate (Section 6, 0a).**
   - Evidence: Section 6 says `"If B is 80%+ as good as C, the tool's value proposition is a synonym dictionary and a routing table -- build only those."` But Section 3 specifies a detailed 5-question FSM with branching, state transitions, backward transitions, two rendering skins, anti-fixation interrupts, stall detection, and a "completion test." This is 60% of the document's intellectual weight, and most of it gets thrown away if the experiment says "build only the synonym dictionary and routing table."
   - Confidence: HIGH
   - Why this matters: An executor following this plan will internalize the spine design and over-invest in it before the experiment result is known. The plan's own logic says the spine might not be needed, but its presentation treats it as the centerpiece.
   - Fix: Restructure the document to make the experiment gate truly load-bearing. Move Section 3 into a conditional appendix: "IF experiment C significantly outperforms B, THEN the following guidance model applies." The Tier 0 scope should be stated as two possible outcomes: (a) synonym dictionary + routing table only, or (b) synonym dictionary + routing table + spine skeleton.

2. **The alias thesaurus -- the make-or-break dataset -- has no concrete creation methodology.**
   - Evidence: Section 6 (0b) says `"39 entries, 3-8 aliases each. Created in a plain JSON file"` and Decision 3 recommends `"B, with the verification pass as the quality gate."` But no word on: what constitutes a good alias (morphological variants? domain-specific jargon? cross-industry terms?), what the LLM-assisted expansion prompt looks like, how to handle polysemy (e.g., "speed" maps to Parameter 9 but also colloquially to "productivity" which is Parameter 39), or how to handle multi-word queries that span two parameters.
   - Confidence: HIGH
   - Why this matters: The strategy calls this `"the real work"` and `"the load-bearing dataset"` and `"if you cannot build a good thesaurus, the entire project is moot."` But it provides less operational detail on HOW to build it than on any other component. The blind-test acceptance criterion (20 queries, 90% top-3 hit rate) is good but insufficient: 20 queries across 39 parameters means most parameters are untested.
   - Fix: (a) Define the alias taxonomy: morphological variants, industry synonyms, colloquial terms, negation-based aliases (e.g., "fragile" -> Parameter 14 Strength, inverted). (b) Expand the blind test to at least 78 queries (2 per parameter) to ensure coverage. (c) Address polysemy explicitly: when a query matches multiple parameters with similar confidence, what happens? The `parameter-search` skill needs a disambiguation protocol, not just a ranked list.

3. **The `formulate-contradiction` classifier is described as trivially deterministic but is actually the hardest problem in the system.**
   - Evidence: Section 4, Lane D says `"Contradiction classification (TC vs PC, including diagonal detection)"` is a COMPUTED operation with `"same input -> same output."` The S3 triage pivot says `"(a) Improving X makes Y worse [two different parameters]"` vs `"(b) X must be both high and low [one parameter, two opposite values]."` But the input to this classifier is not "Parameter 9 vs Parameter 14" -- it is a natural-language description of a conflict. The HARD part is not the if-statement once you have parameter IDs; it is getting from "my battery drains too fast when the screen is bright" to "Parameter 22 (Loss of energy) vs Parameter 18 (Illumination intensity)." That mapping IS Lane A (agent judgment), which means the `formulate-contradiction` skill either (a) takes pre-mapped parameter IDs as input (trivial but not useful without the agent doing all the work) or (b) takes natural language and needs NLP/LLM assistance to map it (not deterministic, contradicts Lane D).
   - Confidence: HIGH
   - Why this matters: The strategy claims triz uniquely provides deterministic classification. But the hard step is the semantic mapping from natural language to parameters, which is explicitly assigned to Lane A (the caller). If the caller must do the hard work, the `formulate-contradiction` skill adds very little -- it is literally an if-statement comparing two integers for equality. The strategy needs to be honest about this: the real value chain is `parameter-search` (natural language -> parameter candidates) -> `formulate-contradiction` (two parameter IDs -> TC or PC), and the first step is the one that matters. The plan implies this chain but never names it as the critical path.
   - Fix: Make the canonical usage pipeline explicit: `parameter-search("battery drain") -> [P22, P25]` + `parameter-search("screen brightness") -> [P18]` -> `formulate-contradiction(improving=P18, worsening=P22) -> TC`. State that `formulate-contradiction` is intentionally trivial and its value is in the pipeline composition, not in the classification logic itself. Address: what happens when `parameter-search` returns ambiguous results for BOTH the improving and worsening sides? The combinatorial explosion of candidate pairs needs a strategy.

**Minor Findings** (suboptimal but functional):

1. **Vault note count is 141, not 139 as stated in the task description.** The strategy's body text does not repeat this number, so it is a context-setting error, not a plan error. The TESE trend notes are named `trend-*` not `tese-trend-*` as the task description states -- but there are indeed 11 of them, matching the count claim.

2. **The `data/` directory does not exist yet.** The strategy refers to `data/*.json` files throughout but the directory has not been created. Not blocking, but the executor should know this is greenfield.

3. **The `next` and `why` fields on every JSON response (Section 2) add coupling between skills.** Every skill must know what ALL other skills are to recommend a `next`. This is a soft dependency that could make adding/removing skills more complex than the "independently callable" framing suggests. Consider making `next` recommendations come from the guidance layer (Lane G) only, not from individual skills in direct-op mode.

4. **The human/agent table in Section 3 says agents get `"Hard-gate: refuse to emit solution directions until IFR + contradiction-type slots are filled"` but also says individual skills are callable directly without the spine.** These are contradictory unless the hard-gate only applies in `triz job` mode, which should be stated explicitly.

5. **The strategy references `crates/infrastructure/src/lib.rs:1-4` as `"Output re-exports"` but line 1-4 actually re-exports config, logging, output, and process -- four modules, not just Output.** The claim is not wrong (it does re-export output), but the line reference implies output is the only thing there.

6. **Decision 6 (the over-engineering circuit breaker) is the right instinct but has no enforcement mechanism.** It says `"the user must observe at least one real use before greenlighting the next tier"` but does not define what constitutes "real use" or who enforces this gate. This is an honor-system check on a solo project where the developer is also the user -- the very person most susceptible to "well, I use it in my head" rationalization.

**What's Missing** (gaps, unhandled edge cases, unstated assumptions):

- **No testing strategy for the domain operations.** The strategy says operations follow the `ping.rs` pattern, which includes unit tests. But it never specifies: what is the test strategy for `parameter-search` scoring? What is the test strategy for the contradiction classifier? What are the edge cases? What happens with empty input, Unicode input, input that matches zero parameters, input that matches ALL parameters? The `ping.rs` pattern is a 3-test example; real operations need significantly more test surface.

- **No error model.** The strategy never discusses what errors triz can produce and how they are reported. The ckeletin scaffold has error envelopes (`Envelope::error`), but: what error does `parameter-search` return when nothing matches? What error does `formulate-contradiction` return when the user provides the same parameter for both improving and worsening (this IS the diagonal detection, but is it an error or a reroute)? What error does `suggest-separations` return when the input is not a physical contradiction?

- **No versioning strategy for the data files.** The strategy says `data/*.json` files have `inline PROVENANCE` but does not specify: what happens when a principle description is corrected? Is the JSON file versioned independently? Is there a schema version field? Can an older triz binary read a newer data file? This matters because the strategy positions triz as a shared SSOT that multiple agents and humans reference.

- **The "co-pilot riding alongside" framing (end of Section 1) is asserted but never architecturally realized.** The strategy says triz is "not a pipeline owner" but a "reactive, event-driven consultation." Yet the entire architecture is request-response CLI calls. There is no event model, no subscription mechanism, no "riding alongside" protocol. The MCP deferral to Tier 3 means this framing is aspirational for the entire v0/v1/v2 timeline. This is fine as long as it is acknowledged as aspirational, not presented as a design property of what ships.

- **No consideration of the `concept-su-field-analysis` -> `concept-76-standard-solutions` pathway as a TRIZ routing branch.** The S3 triage has three branches (TC, PC, function analysis), but classical TRIZ has a fourth major pathway: su-field modeling -> 76 standard solutions. The strategy explicitly defers both su-field and standards to Tier 3, which is fine, but the triage spine should acknowledge this as a known gap in its routing coverage, not pretend the three branches are complete.

- **The "Sell/License" job (rank 6) uses `concept-levels-of-invention.md` as an "inventiveness gauge" but this is a descriptive classification, not a comparative metric.** The Levels of Invention describe TYPES of inventions (routine/minor/substantial/new-concept/discovery), not a score you can compute. `idealize (delta vs incumbents)` implies a quantitative comparison, but the ideality equation requires you to enumerate and quantify all benefits, costs, and harms -- which is Lane A (judgment), not Lane D (computation). The strategy's own ideality calculator can only do the arithmetic; the inputs are all judgment calls.

- **No consideration of what happens when the user/agent disagrees with triz's routing.** S3 routes to Branch A, B, or C. What if the user says "no, this is a physical contradiction, not a technical one"? Can they override? Is override logged? The "opinionated vs composable" contradiction resolution says skills are unopinionated and the orchestrator is opinionated -- but it does not address disagreement with the orchestrator's opinion.

**Ambiguity Risks**:

- `"data compiled to &'static from language-agnostic data/*.json with inline PROVENANCE"` -- Two interpretations: (A) the JSON files are read at compile time via `include_str!` or a build script and baked into the binary; (B) the JSON files are read at runtime from a known path. These have very different deployment, packaging, and update stories. The strategy should state which one. Given the `&'static` phrasing, (A) is likely intended, but this means updating data requires recompiling the binary.
  - Risk if wrong interpretation chosen: If (B), the tool needs a data-discovery mechanism and error handling for missing/corrupt files. If (A), the tool cannot be updated without a release cycle.

- `"validate-framing: Check well-formedness before any solve"` -- What counts as "well-formed"? Is "my thing is bad and I want it good" well-formed? Is "Parameter 9 conflicts with Parameter 14" well-formed? The ambiguity is between validating STRUCTURE (fields are filled) and validating QUALITY (the content is meaningful). The first is trivially automatable; the second requires judgment and is Lane A.
  - Risk if wrong interpretation chosen: If structure-only, the gate is trivially passable and adds no real protection against bad framing. If quality-assessed, it requires LLM assistance and breaks the deterministic contract.

**Multi-Perspective Notes**:

- **Executor**: The Tier 0 scope is executable but the ordering of tasks within it is underspecified. Does the executor transcribe principles first or parameters first? Does the alias thesaurus come before or after the `parameter-search` operation is coded? The strategy says "transcribe then build operations" but does not specify the dependency graph within Tier 0. An executor would benefit from a task list with explicit blocking relationships.

- **Stakeholder**: The strategy is honest that the tool might not justify its existence (the experiment gate). This is admirable but also means the stakeholder is being asked to invest review and planning effort in a project whose core premise is explicitly unvalidated. The experiment should have been run BEFORE this strategy was written, not scheduled as the first roadmap item. That said, the strategy costs thinking time, not engineering time, so the waste is bounded.

- **Skeptic**: The strongest argument against this approach: the strategy resolves Contradiction 1 (rigor vs speed) by offering both guided and direct modes, but this means maintaining two interfaces to the same engine from day one. For a solo developer building a tool whose premise is unvalidated, this doubles the surface area. Ship ONLY the direct/ungated skill calls in Tier 0. Add the guided spine only after the experiment confirms value AND at least one real user asks for guidance. The strategy's own trimming principle (`concept-trimming.md`) says: remove the component and redistribute its function. The guidance function can be redistributed to a system prompt or a README until demand is proven.

**Verdict Justification**: ACCEPT-WITH-RESERVATIONS because:

1. The strategy's overall architecture (three-lane separation, skills-to-jobs model, matrix demotion, experiment gate) is sound and well-grounded in the actual vault content. Every file reference I checked was accurate in both existence and content at the cited lines.

2. The honest self-critique (Section 1's "what triz does NOT add") is genuinely rare and genuinely correct. The strategy does not overclaim.

3. The three major findings are real but addressable without restructuring: (a) make the spine conditional on the experiment, (b) flesh out the thesaurus creation methodology, (c) make the parameter-search -> formulate-contradiction pipeline explicit.

4. The missing items (testing strategy, error model, versioning, override protocol, su-field routing gap) are gaps in operational detail, not strategic flaws. They need to be filled before Tier 0 is built but they do not invalidate the direction.

Operated in THOROUGH mode throughout. No escalation to ADVERSARIAL warranted -- the findings are real but do not suggest systemic issues; rather, they represent the natural gaps of a strategy document that invested heavily in the "why" and "what" while under-investing in the "how" of its own data-creation dependency.

Realist Check: Major Finding #1 (over-specified spine) was pressure-tested -- in practice the executor could simply skip Section 3 until post-experiment. But the document as written does not tell them to do that, and the psychological weight of a detailed design creates commitment bias. Severity maintained at MAJOR because the fix (restructuring the document) is cheap and prevents real waste. Major Finding #2 (thesaurus methodology) survives because the strategy itself says this is the make-or-break dataset with insufficient operational guidance. Major Finding #3 (classifier triviality) survives because the strategy presents the classification as a value-add when the real value is upstream in parameter-search; mislabeling where the value sits leads to wrong prioritization.

**Open Questions (unscored)**:

- The strategy does not mention localization/internationalization. TRIZ is practiced globally. Are the 39 parameter names English-only? Does `parameter-search` support non-English synonyms? This is probably out of scope for v0.1 but worth noting.

- The strategy says the vault has "zero useful aliases" (in the superseded ADR), but the parameter notes themselves contain glosses and alternative names. Has anyone checked whether the existing vault parameter notes already contain usable alias candidates? E.g., `concept-39-engineering-parameters.md` lists "Force (Intensity)" for Parameter 10 -- "Intensity" is already an alias.

- The `causal-chain` skill is listed in the skills table with the note "Referenced within ARIZ but no dedicated vault note (small content gap)." Is this really a content gap, or is causal analysis genuinely underdocumented in the vault? If the vault's ARIZ walkthrough is the only source, the causal-chain skill may be under-grounded relative to the other skills.

- The strategy assumes the ckeletin scaffold's `Output` pattern (Serialize + Display on the same struct) scales to complex state machine states with branching and slot-filling. The `PingResult` struct has one field. A guidance state with 5 questions, filled/unfilled slots, branch indicators, tool recommendations, and provenance citations will be a significantly more complex struct. Has anyone prototyped what this looks like as both JSON and human-readable Display output? The Display impl for a multi-field state struct could be unwieldy.
