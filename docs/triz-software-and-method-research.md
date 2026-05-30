# triz — Software-TRIZ & Clean-room Method Research

> Captured 2026-05-30. 5-agent research workflow (independent authoritative sources; opensourcetriz.com explicitly NOT used — clean-room).

# Software-TRIZ Research Synthesis and Vault Authoring Plan

## 1. Software + TRIZ -- What Is Real

### 1a. The classical matrix fails for software (confirmed by internal + external evidence)

The project's own A/B/C experiment (`/Users/peiman/dev/triz/docs/triz-experiment-core-premise.md`) proved this empirically: on problem P4 (novel dev-CLI/software), the deterministic parameter-mapping path was "useless" and the honest "(none)" parameter-skip scored highest (B=33 vs C=28). This is not a local artifact. Three independent external findings converge:

- **Beckmann (2015)** found post-2000 analogy efforts (mapping the 40 principles 1:1 to software) "had limited success." His method decomposes IT into objects/data/algorithms and produces IT-native principles "deliberately very different from the original 40." (*Procedia Engineering* 131, pp. 993-1001; DOI via sciencedirect.com/science/article/pii/S1877705815042976.) Confidence: HIGH on existence and claim; MODERATE on internal detail (paywalled).

- **MDPI *Systems* 2019** (vol. 7(3):39, mdpi.com/2079-8954/7/3/39) argues directly that TRIZ's original distinguishing features came from physical/mechanical systems and "few of them apply to digital systems," defining 22 additional system characteristics absent from the original matrix. Confidence: HIGH on existence/claims; LOW on author names (not independently confirmed -- cite by title/DOI).

- **Mann Matrix 2003 lineage** (Mann & DeWulf, TRIZCON 2003; osaka-gu.ac.jp/php/nakagawa/TRIZ/eTRIZ/epapers/e2003Papers/eMannDeWulf0303/eMannMatrix030316.html) explicitly produced matrices for "technical, business AND software applications," growing 39 to 48+ parameters. But no canonical software matrix won adoption. Mann himself argues software problems sit in the business/process periphery and routes them through a 31-parameter business matrix. Confidence: HIGH the software matrix exists; UNVERIFIED exact parameters.

**Bottom line:** There is no canonical software contradiction matrix to adopt. Three independent research programs tried and none produced a standard. The vault's existing `concept-contradiction-matrix.md:86-91` critique ("biased toward mechanical-era inventions") is correct and externally corroborated.

### 1b. What transfers cleanly to software

Four TRIZ tools work for software without modification, and the vault already has notes for all of them:

**Separation principles** (`concept-separation-principles.md`) -- parameter-free, map directly to software patterns. Separation in time = lazy/eager evaluation, precompute/cache warming, feature flags with time-based rollout. Separation in space = sharding, microservices, CDN edge vs origin. Separation upon condition = strategy pattern, polymorphic dispatch, responsive breakpoints. Separation between system levels = interface vs implementation, protocol layers, composite pattern. This is the load-bearing recommendation from the research: **separation, not the matrix, is the software contradiction engine.** Source: Souchkov's RCA+/VCM+ methodology (xtriz.com/publications/RCA_Plus_July2011.pdf) sidesteps the matrix entirely and applies separation to causally-derived contradictions. Confidence: HIGH.

**Ideality / IFR** (`concept-ideality.md`) -- Bhushan's TRIZCON 2008 case (the-trizjournal.com/case-study-use-of-triz-in-software-design/) used ideality directly on software: "ideal = fully independent single-function modules." For digital products, the ideal feature delivers its benefit with zero UI, zero user effort, zero added cost -- functionality without the machine. The project's own experiment confirmed IFR enforcement works via system prompt (experiment doc lines 100-106). Confidence: HIGH.

**Function analysis + trimming** (`concept-function-analysis.md`, `concept-trimming.md`) -- functions ARE the system in software (not physical components), so function modeling is arguably more native to software than to mechanics. Trim a module, service, API call, config option. "The best part is no part" maps directly. Kluender (2011, Procedia Engineering 9; sciencedirect.com/science/article/pii/S1877705811001767) applied function analysis to re-architect a flight-simulation system with inventive principles including Self-service and Segmentation. Confidence: HIGH.

**Resources** (`concept-resources.md`) -- software is resource-rich: existing data, idle compute, caches, logs, telemetry, network effects, latency windows, user context. The highest-ideality digital solutions exploit existing resources rather than adding subsystems. Confidence: MEDIUM (logical mapping, not a single citing source).

### 1c. What to add to `docs/triz-method-skill.md`

The method skill at `/Users/peiman/dev/triz/docs/triz-method-skill.md` already handles the software case partially (lines 24-27: "For those [software/UX], use the logic of the method -- contradiction to separation, function analysis, ideality, resources -- and skip the matrix/parameter lookup"). The concrete additions needed:

1. **Add a software-lane routing rule at S3 (TRIAGE).** Currently S3 (lines 45-62) routes to TC/PC/function-analysis but does not explicitly detect "this is a software/digital problem" and skip parameter mapping. Add: "If the problem is software, UX, or organizational: skip parameter mapping entirely. Derive the contradiction causally (what worsens when the desired improvement is attempted?) and resolve via separation principles directly. The 39 parameters and contradiction matrix are engineering-domain artifacts and produce noise here (experiment P4; Beckmann 2015)."

2. **Add software glosses to the separation principles in S3(b).** Currently the examples are all physical (retractable landing gear, knife, sieve, bicycle chain). Add parallel software examples: "In time = lazy evaluation / cache warming / feature-flag rollout. In space = sharding / microservices / edge vs origin. Upon condition = strategy pattern / polymorphic dispatch / responsive breakpoints. Between system levels = interface vs implementation / protocol layering / composite pattern."

3. **Add a software-resources prompt at S4.** Currently S4 (lines 64-67) lists "substances, fields, space, time, waste, voids, the super-system." For software, add: "For software: existing data/telemetry, idle compute, caches, logs, user context, network effects, latency windows, existing APIs/libraries, configuration already present."

4. **Add software-glossed 40-principle examples as an optional brainstorming aid after S5.** Per Rea (2001, novalis.org/triz-talk/softwarearticle.html; 2005, the-trizjournal.com/triz-software-using-inventive-principles/), the verified software meanings: Segmentation(1) = modular decomposition / microservices; Taking Out(2) = extract/parse into independent units; Local Quality(3) = non-uniform handling; Prior Action(10) = precompute / cache warming / JVM bytecode compilation; Feedback(23) = closed-loop observability / rate-based control; Mediator(24) = middleware / API gateway / message broker; Self-service(25) = auto-update / self-configuring components; Copying(26) = shallow copy / references / caching; Cheap Disposable(27) = throwaway prototypes / ephemeral infra; Discarding & Recovering(34) = garbage collection / transaction rollback. Surface these AFTER IFR + separation, as inspiration -- never as deterministic outputs. (Rea explicitly notes Pneumatics/Hydraulics(29) and Phase Transitions(36) have no software analogy -- a documented gap.)

### 1d. The LLM-TRIZ landscape (what agent-native means)

Four published systems attempt LLM+TRIZ automation (2024-2025). All share the same gap this project fills:

- **AutoTRIZ** (Jiang et al., arXiv:2403.13002, 2024-2025): 3 LLM modules + 1 fixed module over a TRIZ KB. Validated on mechanical cases only.
- **TRIZ-GPT** (Chen et al., arXiv:2408.05897, ASME IDETC-CIE 2024): 4-step pipeline mapping to the 39 parameters with human-in-the-loop. GPT-4 contradiction recall 0.691. Mechanical validation only.
- **TRIZ Agents** (Szczepanik & Chudziak, arXiv:2506.18783, ICAART 2025): multi-agent collaboration. Directly supports the project's agent-native framing. Still classical-matrix-bound.
- **Patent drafting** (Springer 2025, DOI 10.1007/978-3-032-08847-5_10): multi-agent TRIZ for patent generation. Metadata only (paywalled).

**The convergent gap:** every system (1) leans on the classical 39-parameter matrix + mechanical case corpus, (2) depends on a fixed/proprietary KB with scarce training cases, (3) confines validation to mechanical engineering. None addresses software/UX/service problems. The project's thesis -- TRIZ for digital/product problems, agent-native, beyond the mechanical matrix -- is exactly the unsolved frontier. Confidence: HIGH on the convergent gap.

**The project's B~=C result** (experiment doc lines 16-18) confirms a capable LLM already "knows" TRIZ, so LLM-TRIZ value is orchestration, auditability, and deterministic classification -- not teaching the LLM TRIZ. The method-as-skill (system prompt) IS the agent-native delivery. Confidence: HIGH.

---

## 2. The Clean-Room Authoring Plan (Prioritized)

All content is original authoring from independent references (MATRIZ wiki/glossary, Souchkov/xTRIZ primary PDFs, peer-reviewed papers, Christensen, Savransky, Terninko-Zusman-Zlotin, Value Engineering/Miles). No content from opensourcetriz.com (blocked, never fetched). Each note follows the vault convention at `/Users/peiman/dev/triz/vault/templates/concept.md`: frontmatter (id/type/title/created/tags/related_ids/source_ids/confidence), Overview, Key Properties, Connections.

**Authoring sequence is dependency-ordered** (note 1 is a dependency for notes 3, 4, and 5):

### Note 1: `concept-cause-effect-chain-analysis` (FIRST -- dependency for 3 others)

| Field | Value |
|-------|-------|
| **id** | `concept-cause-effect-chain-analysis` |
| **type** | `concept` |
| **title** | Cause-Effect Chain Analysis (CECA) |
| **confidence** | `high` |
| **tags** | `triz`, `tools`, `modern-triz` |
| **related_ids** | `concept-function-analysis`, `concept-technical-contradiction`, `concept-physical-contradiction`, `concept-trimming`, `concept-ariz` |
| **source_ids** | `source-matriz-knowledge-base`, `source-terninko-2000-su-field` (or a new Terninko/Zusman/Zlotin 1998 source note) |

**Coverage:** The modern-TRIZ root-cause tool that builds a directed graph from an observed disadvantage backward to its root causes, identifying the key disadvantage to attack. MATRIZ glossary definition (verbatim-grade): "an analytical tool that identifies the key disadvantages of the engineering system...by building cause-effect chains of disadvantages that link the initial disadvantage with its fundamental causes." Distinct from function analysis (which models components and their functions) -- CECA models WHY a disadvantage exists. Covers: building the chain from a target disadvantage; branching (AND/OR causes); selecting the key disadvantage (the cause whose elimination removes the most downstream harm); feeding the key disadvantage into contradiction formulation or trimming; the CECA-vs-plain-RCA distinction (CECA avoids RCA's single-cause shortcomings by chaining and surfacing contradictions directly). Souchkov's RCA+ variant combines classical RCA, Theory of Constraints, and TRIZ philosophy. This is the missing analytical link between "I built a function model and see harmful/insufficient functions" and "I know which contradiction to formulate."

**Independent sources:** MATRIZ glossary (wiki.matriz.org); Souchkov, "Accelerate Innovation with TRIZ" (xtriz.com, p.14, RCA+ description); Yeoh & Teo et al., "Introduction to cause-effect chain analysis plus (CECA+)," *Int. J. Advanced Manufacturing Technology*, Springer, 2018 (DOI 10.1007/s00170-018-2217-1); "On the Identification of Contradictions Using Cause-Effect Chain Analysis" (ResearchGate 294139303); "TRIZ-Based Cause and Effect Chains Analysis vs Root Cause Analysis" (ResearchGate 286447113).

### Note 2: `concept-harmful-function-neutralization` (completes the function-analysis tool family)

| Field | Value |
|-------|-------|
| **id** | `concept-harmful-function-neutralization` |
| **type** | `concept` |
| **title** | Harmful Function Neutralization |
| **confidence** | `high` |
| **tags** | `triz`, `tools`, `modern-triz` |
| **related_ids** | `concept-function-analysis`, `concept-trimming`, `concept-su-field-analysis`, `concept-76-standard-solutions`, `concept-ideality`, `concept-resources`, `concept-cause-effect-chain-analysis` |
| **source_ids** | `source-matriz-knowledge-base`, `source-altshuller-1984-creativity-exact-science` |

**Coverage:** Systematic methods for eliminating or neutralizing harmful functions identified by function analysis, as a distinct operation from trimming (which removes components) and contradiction resolution (which resolves trade-offs). The hierarchy of responses (grounded in 76 Standard Solutions Class 1.2, wiki.matriz.org): eliminate the cause (SIS 1.2.1: introduce separating substance); modify an existing substance as separator (SIS 1.2.2); sacrificial substance that draws off the harm (SIS 1.2.3); counteracting field that neutralizes or converts the harm to useful (SIS 1.2.4); the "convert harm to benefit" strategy (Principle 22, Blessing in Disguise) as a special case. Core distinction: eliminate vs block/insulate vs neutralize/counteract vs convert-to-benefit vs redirect. Connection to CECA (which finds the root cause of the harm). This fills a practical gap: `concept-function-analysis.md:38-39` identifies harmful functions as a key output but no note explains what to DO with them besides trimming.

**Independent sources:** MATRIZ Standard Inventive Solutions (wiki.matriz.org, Class 1.2); Altshuller's 76 Standard Solutions (original lineage via `source-altshuller-1984-creativity-exact-science`); ScienceDirect "Systematic analysis and usage of harmful resources" (S0360835220301935); Souchkov "Annotated List of Main TRIZ Tools and Techniques" (xtriz.com).

### Note 3: `concept-software-triz-adaptation` (highest-leverage for the project's goal)

| Field | Value |
|-------|-------|
| **id** | `concept-software-triz-adaptation` |
| **type** | `concept` |
| **title** | TRIZ for Software and Digital Systems |
| **confidence** | `medium` |
| **tags** | `triz`, `tools`, `software`, `adaptation` |
| **related_ids** | `concept-40-inventive-principles`, `concept-39-engineering-parameters`, `concept-contradiction-matrix`, `concept-separation-principles`, `concept-function-analysis`, `concept-cause-effect-chain-analysis`, `concept-ideality`, `concept-resources` |
| **source_ids** | `source-matriz-knowledge-base` (plus new source notes for Rea 2001/2005, Beckmann 2015, Souchkov RCA+) |

**Coverage:** Three layers. (1) Software reinterpretations of the 40 inventive principles -- verified Rea (2001) meanings: Segmentation=modular decomposition/microservices, Taking Out=parsing/extraction, Local Quality=non-uniform handling, Prior Action=precompute/cache-warming, Nesting=composition/embedding, Feedback=closed-loop observability, Mediator=middleware/API gateway, Self-service=self-configuring components, Copying=shallow copy/caching, Cheap Disposable=throwaway prototypes/ephemeral infra, Discarding & Recovering=GC/transaction rollback, Composite Materials=composite design pattern. Documented gaps: Pneumatics/Hydraulics(29) and Phase Transitions(36) have no software analogy (Rea). (2) Why the 39 parameters fail for software and the matrix should be skipped -- Beckmann 2015 found 1:1 analogies "had limited success"; MDPI Systems 2019 identified 22 missing digital characteristics; the project's own P4 experiment confirmed parameter-mapping was "useless." No canonical software parameter set to adopt (Mann, Bhushan, and Beckmann each built different ones). (3) What works instead: causal contradiction extraction (Souchkov RCA+/VCM+) feeding separation principles directly; function analysis applies cleanly because functions ARE the software system; resources are abundant in software. Horowitz/ASIT (PhD thesis, Tel Aviv 1999) shows TRIZ's value survives without the matrix: 2 conditions + 5 tools.

**Independent sources:** Rea, "TRIZ and Software -- 40 Principle Analogies," Parts I & II, *TRIZ Journal*, 2001 (novalis.org/triz-talk/softwarearticle.html; metodolog.ru/triz-journal/archives/2001/11/e/); Rea, "TRIZ for Software -- Using the Inventive Principles," *TRIZ Journal*, Jan 2005 (the-trizjournal.com/triz-software-using-inventive-principles/); Beckmann, "Method for Transferring the 40 Inventive Principles to Information Technology and Software," *Procedia Engineering* 131, 2015; "TRIZ for Digital Systems Engineering," *Systems* (MDPI), 2019, 7(3):39; Souchkov, RCA+ (xtriz.com/publications/RCA_Plus_July2011.pdf); Horowitz, *Creative Problem Solving in Engineering Design*, PhD thesis, Tel Aviv University, 1999 (ASIT); Bhushan, "Case Study: Use of TRIZ in Software Design," TRIZCON 2008 (the-trizjournal.com/case-study-use-of-triz-in-software-design/); Kluender, "TRIZ for software architecture," *Procedia Engineering* 9, 2011.

### Note 4: `concept-triz-product-development-workflow` (the compositional choreography layer)

| Field | Value |
|-------|-------|
| **id** | `concept-triz-product-development-workflow` |
| **type** | `concept` |
| **title** | TRIZ Product-Development Workflow |
| **confidence** | `medium` |
| **tags** | `triz`, `meta`, `workflow` |
| **related_ids** | `concept-function-analysis`, `concept-trimming`, `concept-feature-transfer`, `concept-cause-effect-chain-analysis`, `concept-su-field-analysis`, `concept-76-standard-solutions`, `concept-separation-principles`, `concept-ideality`, `concept-ariz` |
| **source_ids** | `source-matriz-knowledge-base` (plus new source notes as needed) |

**Coverage:** Maps which TRIZ tools (building blocks) compose into which product-development tasks and in what order. The vault has all individual tools but no note explaining WHEN and in WHAT ORDER to apply them. Covers: the distinction between analytical tools (function analysis, CECA, su-field) and solution tools (trimming, feature transfer, contradiction resolution, standards); the typical workflow sequences (function model -> CECA -> key disadvantage -> contradiction formulation -> separation/principles; or function model -> trimming -> secondary problems -> contradiction resolution); how these map to product-development phases (discovering needs, creating offerings, designing/prototyping, reducing burdens, resolving field problems, evolving). This is the "guided method substrate" the strategy decision (`decision-cli-triz-agentic-strategy.md:63-68`) calls for -- the choreography layer above the individual tools.

**Independent sources:** Souchkov, "Made with TRIZ" (Executive Overview, 2005, xtriz.com -- "Systematic Innovation Process, which structures the use of the techniques and tools according to the desired outcome"); MATRIZ certification syllabi (Levels 1-3, prescribe tool sequencing); Terninko/Zusman/Zlotin, *Systematic Innovation*, CRC Press, 1998 (chapter structure models the analytical-then-solution workflow); Miles/Value Engineering FAST (function-cost optimization, valuefoundation.org); Christensen, "Marketing Malpractice," HBR, 2005 (JTBD as demand-side front-end for the workflow).

### Note 5: `concept-triz-and-jtbd` (the demand-side bridge)

| Field | Value |
|-------|-------|
| **id** | `concept-triz-and-jtbd` |
| **type** | `concept` |
| **title** | TRIZ and Jobs-to-Be-Done (JTBD) Integration |
| **confidence** | `low` |
| **tags** | `triz`, `product-development`, `jtbd` |
| **related_ids** | `concept-function-analysis`, `concept-ideality`, `concept-technical-contradiction`, `concept-triz-product-development-workflow`, `concept-trimming` |
| **source_ids** | (new source notes for Christensen, Ulwick) |

**Coverage:** The structural parallel between JTBD and TRIZ: JTBD's "job" = TRIZ's main useful function; JTBD's "outcome expectations" = TRIZ parameters to improve; JTBD's over-served/under-served outcomes = TRIZ contradictions (improving an under-served outcome worsens an over-served one). Using ODI job maps to identify the system's function hierarchy. JTBD as demand-side discovery (what to build) feeding TRIZ as supply-side invention (how to build it inventively). Honest boundary: no rigorous published integration exists -- this is a documented gap/whitespace, not established literature. Confidence flagged as `low` for this reason.

**Independent sources:** Christensen et al., *Competing Against Luck*, Harper Business, 2016; Ulwick, *What Customers Want* / *Jobs to Be Done*, McGraw-Hill/Idea Bite Press, 2005/2016; Souchkov, "Defining Contradictions for Inventive Problem Solving in Business and Management," ETRIA TRIZ Future Conference, 2017 (uses value/function framing compatible with JTBD); Mann, *Hands-On Systematic Innovation for Business and Management*, 2007, Ch. 2 (function/job framing for non-engineering problems).

**IMPORTANT:** No rigorous TRIZ+JTBD integration paper was found. Searched specifically. Sources cover each framework separately; none integrates them. The synthesis proposed here is original and should be flagged as such in the note.

### Updates to existing notes (not new notes, but source-gap fixes)

- **`concept-resources.md`** -- add Savransky (2000) as the citation for "ready/derived/differential" readiness levels (currently unsourced in the note). Add Terninko/Zusman/Zlotin 1998 six categories as the primary citation. Source: University of Cambridge IfM TRIZ roadmapping Phase 1 report (ifm.eng.cam.ac.uk, quotes Savransky verbatim).

- **`concept-function-analysis.md`** -- add the Lawrence Miles / Value Engineering / FAST genealogy as the independent function-oriented root (Miles, GE, value analysis established Dec 1947; verb-noun word pairs). Add Souchkov "Accelerate Innovation with TRIZ" p.13 as a primary source for the description.

---

## 3. Software-TRIZ Note Plan (Specific)

The software-TRIZ material distributes across two deliverables:

### 3a. The vault note: `concept-software-triz-adaptation` (Note 3 above)

This is the permanent reference. It documents what is known, what works, what does not, and where the gaps are. It does NOT prescribe agent behavior -- that belongs in the method skill. Cross-references the vault's existing principle notes (which currently have physical-domain examples only) but does NOT modify those 40 notes. Instead, it serves as the single lookup for "what does Principle X mean in software?" with the Rea-verified mappings.

**What goes here:** The three-layer structure (software-glossed principles, why the matrix fails, what works instead). The documented case examples (Bhushan 2008 modularity/coupling, Kluender 2011 flight-sim architecture, Rea 2005 telecom protocol automation). The LLM-TRIZ landscape (AutoTRIZ, TRIZ-GPT, TRIZ Agents -- all mechanical-only, confirming the gap). The Horowitz/ASIT simplification as evidence TRIZ's value survives without the matrix.

### 3b. Updates to `docs/triz-method-skill.md`

This is the operational deliverable -- the system prompt that agents follow. The four additions listed in section 1c above: software-lane routing at S3, software separation examples at S3(b), software resources prompt at S4, optional software-glossed 40-principle brainstorming aid after S5.

### 3c. New source notes needed

The vault convention requires source notes for cited works. These are needed:

| source_id | Title/Author | Confidence |
|-----------|-------------|------------|
| `source-rea-2001-triz-software` | Rea, "TRIZ and Software -- 40 Principle Analogies," 2001 | HIGH |
| `source-beckmann-2015-it-principles` | Beckmann, "Method for Transferring the 40 Inventive Principles to IT," 2015 | HIGH existence / MODERATE detail |
| `source-bhushan-2008-software-case` | Bhushan, "Case Study: Use of TRIZ in Software Design," TRIZCON 2008 | HIGH |
| `source-souchkov-accelerate-innovation` | Souchkov, "Accelerate Innovation with TRIZ," 1997 rev. 2017 | HIGH |
| `source-horowitz-1999-asit` | Horowitz, "Creative Problem Solving in Engineering Design," PhD thesis, 1999 | HIGH |
| `source-savransky-2000-engineering-creativity` | Savransky, "Engineering of Creativity," CRC Press, 2000 | HIGH (for resource readiness levels) |
| `source-terninko-zusman-zlotin-1998` | Terninko/Zusman/Zlotin, "Systematic Innovation," CRC Press, 1998 | HIGH |

---

## 4. Honest Assessment: Genuine Value vs Scope Creep

### Genuinely valuable to add (in priority order)

1. **Updates to `triz-method-skill.md`** (software lane + examples). Effort: ~1 hour. Impact: HIGH. This is the validated #1 deliverable (`decision-cli-triz-agentic-strategy.md:99`). The software handling is the method skill's most obvious gap -- it says "skip the matrix" but does not say what to do instead. The four additions give concrete, source-backed guidance. Ships immediately as part of the product.

2. **`concept-cause-effect-chain-analysis`** (Note 1). Effort: ~1 hour. Impact: HIGH. This is the vault's most load-bearing gap. It is the missing analytical link the method skill already references at line 57 ("do Function analysis + cause-effect chain first") but which has no vault note to cite. It is also a dependency for three other proposed notes. The independent sources are strong (MATRIZ glossary, Springer 2018 peer-reviewed paper, Souchkov primary).

3. **`concept-harmful-function-neutralization`** (Note 2). Effort: ~45 min. Impact: MEDIUM. Completes the function-analysis tool family. The method skill's tool-selection table (line 79) says "Harmful effect from a needed function -> Function analysis -> trimming / introduce S3 / neutralize" but "neutralize" points nowhere. This note fills that gap. Grounded in the 76 Standard Solutions Class 1.2, which the vault already documents at the class level but not the strategy level.

4. **`concept-software-triz-adaptation`** (Note 3). Effort: ~2 hours. Impact: MEDIUM-HIGH for the project's stated goal (agent-native TRIZ for product development including software). This is the permanent reference backing the method skill's software lane. But the method skill update (item 1) is the operational deliverable -- this note is the scholarly backing, not the shipped product.

5. **Source note updates** (7 new source notes + 2 existing-note citation fixes). Effort: ~1.5 hours total. Impact: MEDIUM. Makes the vault's citation discipline machine-filterable and honest. Without these, the new concept notes lack proper `source_ids`.

### Valuable but lower priority (do after the above)

6. **`concept-triz-product-development-workflow`** (Note 4). Effort: ~1.5 hours. Impact: MEDIUM. The method skill at lines 88-98 already has the "product development with TRIZ from the start" section. This vault note provides the deeper reference. Useful, but the method skill already carries the operational content.

### Scope creep risk (defer or cut)

7. **`concept-triz-and-jtbd`** (Note 5). Effort: ~1 hour. Impact: LOW-MEDIUM. The confidence is honestly `low` -- no rigorous integration exists in the literature. The synthesis is original and speculative. The method skill's "Discover the job/market" bullet (line 92) already gestures at JTBD without requiring a formal vault note. This is interesting whitespace the project could eventually own, but authoring a `low`-confidence speculative note is not consistent with the vault's citation discipline. **Recommendation: defer until the project has its own worked examples of TRIZ+JTBD to ground it, or until a published integration appears.**

### Explicitly NOT doing (the experiment's lesson)

The experiment at `/Users/peiman/dev/triz/docs/triz-experiment-core-premise.md:130-141` killed the following, and nothing in the software-TRIZ research resuscitates them:

- **A software contradiction matrix or software parameter thesaurus.** Mann, Bhushan, and Beckmann each built different ones; none generalized. The experiment showed parameter-mapping was useless for P4. The alias thesaurus is engineering-only and should stay scoped to engineering.
- **Enumerating the 76 standards / effects DB / matrix transcription.** High cost, the LLM already covers it, no validated lift.
- **Modifying the 40 individual principle notes to add software examples.** The existing 40 notes (`principle-01` through `principle-40`) have physical-domain examples. Adding software glosses to each would be 40 edits of uncertain value. Better to keep the software glosses centralized in `concept-software-triz-adaptation` (Note 3) as the single lookup, consistent with SSOT.
- **Building a guidance state machine, ARIZ escalation, or function-analysis engine in Rust.** The system prompt does this to equal quality (B/C = 101%).

### The meta-lesson

The method-as-skill IS the deliverable (`decision-cli-triz-agentic-strategy.md:99-100`). Vault notes are research memory that backs the skill -- they are not the product. Every vault note proposed above earns its place by either (a) filling a gap the method skill already references but cannot cite, or (b) grounding the method skill's software lane with source-backed evidence. If a note does neither, it is scope creep. Note 5 (TRIZ+JTBD) is the marginal case and is flagged accordingly.

---

## References

### Project files
- `/Users/peiman/dev/triz/docs/triz-method-skill.md` -- the validated #1 deliverable; lines 24-27 (current software handling), 45-62 (S3 triage), 64-67 (S4 resources), 88-98 (product development section)
- `/Users/peiman/dev/triz/docs/triz-experiment-core-premise.md` -- A/B/C experiment results; lines 16-18 (B/C=101%), 92-93 (P4 parameter-mapping useless), 100-106 (IFR gate via prompt), 130-141 (what NOT to build)
- `/Users/peiman/dev/triz/vault/decisions/decision-cli-triz-agentic-strategy.md` -- strategy decision; lines 63-68 (5-question spine), 98-109 (revised build order)
- `/Users/peiman/dev/triz/vault/concepts/concept-separation-principles.md` -- current separation note (physical examples only, lines 33-38)
- `/Users/peiman/dev/triz/vault/concepts/concept-function-analysis.md` -- current FA note (lines 38-39 identify harmful functions as output but no note covers what to do)
- `/Users/peiman/dev/triz/vault/concepts/concept-resources.md` -- current resources note (lines 36-37: ready/derived/differential categories listed but unsourced)
- `/Users/peiman/dev/triz/vault/concepts/concept-contradiction-matrix.md` -- lines 86-91 (critique section already flags mechanical bias)

### External sources (verified, with confidence)
- Rea, K. "TRIZ and Software -- 40 Principle Analogies," Parts I & II, *TRIZ Journal*, 2001. HIGH. novalis.org/triz-talk/softwarearticle.html
- Rea, K. "TRIZ for Software -- Using the Inventive Principles," *TRIZ Journal*, Jan 2005. HIGH. the-trizjournal.com/triz-software-using-inventive-principles/
- Beckmann, H. "Method for Transferring the 40 Inventive Principles to IT and Software," *Procedia Engineering* 131, 2015. HIGH existence / MODERATE detail. sciencedirect.com/science/article/pii/S1877705815042976
- "TRIZ for Digital Systems Engineering," *Systems* (MDPI), 2019, 7(3):39. HIGH claims / LOW author names. mdpi.com/2079-8954/7/3/39
- Bhushan, N. "Case Study: Use of TRIZ in Software Design," TRIZCON 2008. HIGH. the-trizjournal.com/case-study-use-of-triz-in-software-design/
- Kluender, D. "TRIZ for software architecture," *Procedia Engineering* 9, 2011. HIGH. sciencedirect.com/science/article/pii/S1877705811001767
- Souchkov, V. RCA+ methodology, 2017. HIGH. xtriz.com/publications/RCA_Plus_July2011.pdf
- Souchkov, V. "Accelerate Innovation with TRIZ," 1997 rev. 2017. HIGH. xtriz.com/publications/AccelerateInnovationWithTRIZ.pdf
- Horowitz, R. *Creative Problem Solving in Engineering Design*, PhD thesis, Tel Aviv University, 1999 (ASIT). HIGH. asit.pro/biblio/horowitz-1999-creative-problem-solving-asit.php
- Mann, D. & DeWulf, S. "Updating the TRIZ Contradiction Matrix," TRIZCON 2003. HIGH existence / UNVERIFIED software parameters. osaka-gu.ac.jp/php/nakagawa/TRIZ/eTRIZ/epapers/e2003Papers/eMannDeWulf0303/eMannMatrix030316.html
- Savransky, S. *Engineering of Creativity*, CRC Press, 2000. HIGH (for resource readiness levels). Via Cambridge IfM TRIZ roadmapping report.
- Terninko/Zusman/Zlotin, *Systematic Innovation*, CRC Press, 1998. HIGH.
- MATRIZ glossary, wiki.matriz.org. HIGH.
- Jiang et al., "AutoTRIZ," arXiv:2403.13002, 2024-2025. HIGH.
- Chen et al., "TRIZ-GPT," arXiv:2408.05897, ASME IDETC-CIE 2024. HIGH.
- Szczepanik & Chudziak, "TRIZ Agents," arXiv:2506.18783, ICAART 2025. HIGH.

### UNVERIFIED (flagged, do not cite as fact)
- Mann's "PERFECT" acronym (only surfaced in an AI search summary, not the primary book review)
- Graham Rawlinson software-specific principle mapping (no verified publication found)
- Any IBM or US Navy software-TRIZ program (no evidence found; likely conflated with Rea's Lucent work and Mann's UAV case)
- MDPI *Systems* 2019 author names (cite by title/DOI only)
