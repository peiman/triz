# triz — Core-Premise Experiment (A/B/C)

> Captured 2026-05-30. A blind, closed-book experiment (17 agents) testing the project's
> own decision rule: does the deterministic `triz` tool beat an LLM with a good TRIZ
> system prompt? **A** = bare LLM · **B** = LLM + TRIZ system prompt · **C** = LLM
> constrained to triz's deterministic ops. 4 problems (2 documented vault cases + 2
> novel product-dev problems, the recall-proof signal). Graders blind to condition.

## Headline

| Condition | Avg (/40) | vs bare |
|---|---|---|
| A — bare LLM | 24.50 | — |
| B — LLM + TRIZ system prompt | 29.50 | +5.00 |
| C — triz tool (simulated) | 29.25 | +4.75 |

**B / C = 101%.** Decision rule: B ≥ ~80% of C ⇒ build only the thin core.
B is *indistinguishable* from C. The method (TRIZ at all) adds ~20% over bare; the
*delivery* (Rust tool vs system prompt) is a wash.

| Problem | A | B | C | Winner |
|---|---|---|---|---|
| P1 chocolate (documented, physical contradiction) | 23 | 34 | 29 | B |
| P2 rock (documented, su-field) | 26 | 23 | 26 | C≈A (all missed su-field) |
| P3 e-bike (novel, mechanical) | 28 | 28 | 34 | **C** (+6, rigor) |
| P4 dev-CLI (novel, software) | 21 | 33 | 28 | **B** (param-mapping useless) |

---

# A/B/C Experiment Synthesis: Does the `triz` Tool Justify Itself?

## 1. Verdict

**The experiment does not support building the full `triz` tool. The data supports building only the thin core -- and even that case is weaker than expected.**

The strategy's own decision rule (Section 6, 0a of `/Users/peiman/dev/triz/docs/triz-cli-strategy-v2.md:300`): "If B is ~80%+ as good as C, the tool's value is just a synonym dictionary + routing table -- build only those." B/C ratio is 100.9%. B is not 80% as good as C; B is *indistinguishable* from C. The 80% threshold is blown past so completely that the question flips: does even the thin core add enough over a well-crafted system prompt?

However, the aggregate masks a real pattern. On the novel, recall-proof problems (P3-ebike, P4-cli) -- the most decision-relevant signal -- the split is 2-2: C won P3 decisively (34 vs 28, +6) on tool-selection rigor, and B won P4 decisively (33 vs 28, +5) on honest method engagement. The tool's value is not zero -- it is *situational*, concentrated in problems where deterministic parameter-mapping and contradiction-classification actually have correct answers to enforce (engineering problems with clear TRIZ parameter matches), and absent for problems where the TRIZ parameter space is a poor fit (software/UX contradictions where the 39 parameters do not map cleanly).

**Bottom line:** Build the alias thesaurus as a standalone JSON artifact. Ship `parameter-search` and `formulate-contradiction` as a minimal CLI/MCP tool. Do not build the guidance spine, the state machine, the jobs, or any of Tiers 1-3 until usage data from the thin core justifies it. The system prompt demonstrated it can do the guidance work.

---

## 2. The Numbers

### Aggregate

| Condition | Total (4 problems) | Average (/40) | vs A delta |
|-----------|-------------------|---------------|------------|
| A-bare | 98 | 24.5 | -- |
| B-triz-prompt | 118 | 29.5 | +5.0 |
| C-tool-sim | 117 | 29.25 | +4.75 |

- **B/C ratio: 100.9%** -- B meets and marginally exceeds C on aggregate.
- **B-A delta: +5.0 pts (20.4% improvement)** -- the TRIZ system prompt materially improves output quality over a bare LLM.
- **C-A delta: +4.75 pts (19.4% improvement)** -- the deterministic tool simulation is slightly *less* effective than the system prompt.
- **Win count: B=2, C=2, A=0** -- dead even on head-to-head victories.

### Documented vs. Novel Problems (the critical split)

| Subset | A avg | B avg | C avg | B/C ratio | B-C delta |
|--------|-------|-------|-------|-----------|-----------|
| Documented (P1 chocolate, P2 rock) | 24.5 | 28.5 | 27.5 | 103.6% | +1.0 |
| Novel (P3 ebike, P4 cli) | 24.5 | 30.5 | 31.0 | 98.4% | -0.5 |

The novel problems are the most decision-relevant because they are recall-proof -- neither condition could have seen these exact problems in training data. On novel problems, the gap is effectively zero (B/C = 98.4%), with C holding a trivial 0.5-point edge that is well within noise for N=2.

### Per-Dimension Breakdown (B vs C, all 4 problems)

| Dimension | B total | C total | Delta | Interpretation |
|-----------|---------|---------|-------|----------------|
| Framing quality | 32 | 31 | +1 | Negligible |
| Contradiction correctness | 32 | 31 | +1 | Negligible |
| Tool selection | 26 | 27 | -1 | C marginally better |
| Solution quality | 31 | 28 | +3 | B meaningfully better |

**The one dimension where the tool simulation (C) should dominate -- tool selection -- shows a negligible +1 advantage for C overall.** On novel problems specifically, C wins tool selection by 3 points (16 vs 13), which is the tool's strongest showing anywhere in the data. But B wins solution quality by 3 points overall, meaning the system prompt's less rigid process produced better actual inventive directions despite slightly worse routing.

---

## 3. Where Did Structure/Determinism Actually Help?

Structure helped in three specific, identifiable ways. It failed in two others.

### Where it helped

**3a. Deterministic parameter-mapping prevented classification errors (P1, P3).**
In P1-chocolate, both B and C correctly used Parameter #31 (Object-generated harmful factors), while A used #30 (External harmful factors). The grader explicitly noted: "structure prevented a parameter-mapping error." In P3-ebike, C's explicit pipeline (params 26/14 improving, param 1 worsening) was scored 9/10 on contradiction correctness vs B's 7/10 -- a 2-point gap directly attributable to the deterministic derivation sequence (technical-to-physical, in proper TRIZ order). B admitted applying TRIZ labels post-hoc, which the grader flagged as "a subtle but real methodological error."

**3b. Hard gates set a quality floor (P1, P3).**
A-bare scored last in 3 of 4 problems. The least-structured output was consistently the worst. The grader on P1: "structure helped establish a FLOOR -- the least structured output was clearly worst." On P3, the 6-point gap between C (34) and A (28) was "almost entirely in tool selection and contradiction correctness" -- the dimensions where deterministic gates operate.

**3c. Tool-path transparency created audit trails (P3).**
C's explicit "HARD GATE passed" notation in P3 was specifically praised for providing "auditable evidence that the process was deterministic rather than intuitive." For the agent-builder use case (verifier agents checking solver agents), this is genuinely valuable.

### Where it failed

**3d. Deterministic structure did not produce better solutions (all 4 problems).**
Solution inventiveness was comparable across B and C in every problem. B actually scored higher on solution quality aggregate (31 vs 28). The grader on P3 stated this directly: "solution inventiveness was roughly comparable across all three (8, 8, 7), suggesting that once the contradiction is correctly identified, creative generation is less dependent on process determinism." The grader on P4 was more blunt: "structure helped identify the RIGHT contradiction and organize the derivation transparently; it did not produce meaningfully more inventive solutions."

**3e. Deterministic structure failed at the highest-leverage routing decision (P2).**
P2-rock is a textbook su-field harmful-effect problem. The correct TRIZ routing is to the 76 Standard Solutions, not the contradiction/separation framework. All three conditions defaulted to contradictions. The grader: "A truly deterministic TRIZ tool should have classified the problem type FIRST (harmful complete su-field) and routed to the 76 Standard Solutions before touching contradictions. None did this, so structure failed at the highest-leverage decision point." This is damning for the tool's routing claim -- the tool simulation did not have the su-field pathway implemented, and neither did the system prompt. But the system prompt is cheap to update; the tool would require new Rust code and data.

### The IFR gate

The IFR hard-gate was the grader's most interesting finding. In P1, B's explicit anti-compromise discipline ("rejected any that merely meet in the middle") was scored as a structural discipline that directly improved solution quality. But this discipline came from the *system prompt*, not from the tool. In P4, B's refusal to force a parameter mapping (with explicit justification: "(none)") was scored highest -- and this too was a prompt-driven discipline, not a tool-enforced gate. The system prompt proved capable of self-enforcing the IFR gate when properly instructed.

---

## 4. Where Was the Tool Redundant?

The system prompt (B) matched or exceeded the tool simulation (C) on:

**4a. IFR enforcement and anti-compromise gating.** The strategy claims this as a key tool value (Section 1, bullet 4 of the strategy: "the state machine literally will not emit solution directions until the IFR and contradiction-type slots are filled"). The experiment shows a system prompt instruction achieves the same effect. B scored higher than C on this discipline in P1 (B's anti-compromise check was praised as "the structural discipline that directly improved solution quality") and P4 (B's IFR resource inventory was the most thorough).

**4b. Socratic reasoning and contradiction identification.** B matched or exceeded C on framing quality (32 vs 31 aggregate) and contradiction correctness (32 vs 31 aggregate). The spine's 5-question sequence, when encoded as a system prompt, produced equally good problem framing.

**4c. Solution generation.** B produced better solutions on aggregate (31 vs 28). The system prompt's less rigid process allowed for more creative direction-generation, particularly in P1 (B scored 9 vs C's 6 on solution quality) and P4 (B's solutions were tied with C at 7 but with better framing and derivation honesty).

**4d. Method-fit honesty.** In P4-cli, B explicitly refused to force a parameter mapping with justification, which the grader scored as "the most intellectually honest handling across all three." C provided a mapping while acknowledging loose fit -- "less honest than refusing the mapping entirely." The system prompt enabled better epistemic calibration than the deterministic tool path, which by design must produce a mapping.

**In short:** Everything the strategy identifies as guidance value (Sections 3 and 5 of the strategy) was replicated by 50 lines of system prompt. The customer-advocate critique at `/Users/peiman/dev/triz/docs/triz-cli-strategy-v2.md:418-485` was correct.

---

## 5. The Honest Recommendation

### What the strategy's own method demands

The strategy invokes `concept-ideality.md:29` -- "the function is performed but the machine is absent" -- and states: "if a system prompt achieves the IFR, the tool has trimmed itself out of existence -- which, per the ideality equation, IS the ideal outcome" (strategy line 269). The experiment shows the system prompt achieves the IFR to within measurement noise. By the tool's own method, the tool has trimmed itself out of existence for the guidance layer.

### What to build (ordered)

1. **The TRIZ system prompt itself -- ship it as a versioned artifact.** The experiment proves this is the highest-value deliverable. A well-crafted system prompt with the 5-question spine, separation principles, IFR-first mandate, and anti-compromise gate produces results equal to the deterministic tool. Publish it as a markdown file in the repo. This costs hours, not weeks. Impact: high. Effort: trivial.

2. **The parameter alias thesaurus (`data/parameters.json`) -- ship it as a standalone JSON file.** This is the one genuinely unique data asset the experiment validates. In P1 and P3, deterministic parameter mapping prevented classification errors. An LLM mapping "download speed" to Parameter 9 is unreliable (strategy line 31). A curated synonym table with deterministic scoring is verifiably better. But this is a *data product*, not a Rust application. Agents can consume the JSON directly via file read or a trivial HTTP endpoint. Effort: medium (the thesaurus creation is the real work, per strategy line 243). Impact: moderate -- validated only for engineering problems with clear parameter matches. For software/UX problems (P4), the parameter space itself is a poor fit, and the thesaurus cannot fix that.

3. **`parameter-search` and `formulate-contradiction` as a minimal CLI.** Two commands, `--output text|json`. This is the strategy's own fallback recommendation (lines 300, 482). Ship only if the thesaurus blind-test passes (78+ queries, 90% top-3 hit rate per strategy Appendix B finding #2). Effort: low (the Rust scaffold already exists at `/Users/peiman/dev/triz/crates/domain/src/ping.rs`). Impact: moderate for agent pipelines that need deterministic, auditable parameter mapping.

4. **Do NOT build:** the guidance state machine, the 5-question spine as a Rust FSM, the 6 jobs, the 12 composable skills beyond the two above, the function-analysis engine, the trimming engine, or ARIZ escalation. The system prompt handles all of this to equivalent quality. Defer these to Tier "never-unless-usage-data-says-otherwise."

### Tying back to ideality

The strategy at line 48 states: "Everything that is replicable by a system prompt (principle descriptions, Socratic pacing, pedagogical explanations) is deferred to the vault prose or left to the calling agent." The experiment extends this list to include: IFR enforcement, anti-compromise gating, contradiction classification, separation-principle routing, and solution-quality checking. What remains uniquely the tool's after this extension: parameter-search scoring and deterministic routing audit trails. That is the honest scope.

---

## 6. Caveats

| Caveat | Severity | Direction of bias |
|--------|----------|-------------------|
| **N=4 problems** | High | Insufficient to detect a real 3-5 point difference between B and C with statistical confidence. The true B/C gap could be anywhere from C winning by ~8 to B winning by ~8. |
| **Single model family** | Medium | All conditions used the same LLM (likely Claude). A weaker model might benefit more from deterministic tool enforcement. The finding "a system prompt is enough" may not generalize to GPT-3.5 or open-source models with weaker TRIZ knowledge. |
| **Recall confound on documented problems** | Medium | P1-chocolate and P2-rock are documented TRIZ case studies. The LLM may have seen canonical solutions in training data. B's win on P1 (34 vs 29) could reflect better recall, not better method. The novel problems (P3, P4) are the cleaner signal. |
| **Grader subjectivity** | Medium | Single grader, 4 dimensions, blinded but potentially inconsistent across problems. The grader's own biases (e.g., valuing "honesty about method fit" highly in P4) shaped scores. No inter-rater reliability check. |
| **C was simulated, not the real tool** | High | C was an LLM *constrained to behave like the tool*, not the actual Rust binary. A real deterministic tool might enforce constraints more reliably than an LLM pretending to enforce them. However, the grader noted that C's "HARD GATE" language was sometimes "performative rather than load-bearing" (P1 grader note on Z), suggesting the simulation may have been generous to C. |
| **Problem-type coverage** | Medium | No su-field problem was tested (P2 was one, but no condition recognized it). The tool's routing value for su-field -> 76 Standards was not tested because the tool does not implement that pathway yet. |

**Confidence in the verdict:** Moderate. The aggregate result (B/C = 100.9%) is so far past the 80% threshold that even generous error bars do not rescue the full tool. The thin-core case (parameter-search + formulate-contradiction) rests primarily on P3's C-win, which is a single data point. The honest assessment: the thin core is *probably* worth building as a low-cost bet, but the evidence is not strong enough to call it validated. The system prompt is the only deliverable that is clearly validated by this experiment.

---

## References

- `/Users/peiman/dev/triz/docs/triz-cli-strategy-v2.md:269` -- "if a system prompt achieves the IFR, the tool has trimmed itself out of existence"
- `/Users/peiman/dev/triz/docs/triz-cli-strategy-v2.md:300` -- the 80% decision rule and A/B/C experiment specification
- `/Users/peiman/dev/triz/docs/triz-cli-strategy-v2.md:418-485` -- customer-advocate critique (confirmed correct by experiment)
- `/Users/peiman/dev/triz/docs/triz-cli-strategy-v2.md:31-48` -- the "thin but real" value band claim (partially validated: only parameter-search survives)
- `/Users/peiman/dev/triz/docs/triz-cli-strategy-v2.md:243` -- alias thesaurus as "the real work"
- `/Users/peiman/dev/triz/vault/decisions/decision-cli-triz-agentic-strategy.md:71-79` -- the experiment-first mandate
- `/Users/peiman/dev/triz/crates/domain/src/ping.rs` -- the existing Rust scaffold pattern for any operations that are built
