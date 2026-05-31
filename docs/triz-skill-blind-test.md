# triz — Skill Blind-Test (A/B/C): does the software lane help?

> Captured 2026-05-31. Blind, closed-book (17 agents). **A** = bare LLM · **B** = a generic
> TRIZ prompt (matrix-as-normal-path) · **C** = our `triz` skill (with the software lane).
> B and C share the same spine/IFR/separations and differ ONLY in the software adaptation,
> so any B→C gap is that guidance. 3 software/UX problems + 1 mechanical control; novel
> (recall-proof); graders blind.

## Headline

| Condition | Overall /40 | Software-only /40 | Forced the matrix on software |
|---|---|---|---|
| A — bare | 30.75 | 31.00 | 1/3 |
| B — generic TRIZ prompt | 27.75 | 26.33 | 3/3 |
| C — triz skill | 37.00 | 37.33 | 0/3 |

**C beats B by +11.0 on software.** The behavioral proof: B forced
the 39-parameter matrix on **3/3** software problems; C on **0/3**. B even scored
*below* the bare LLM overall — forcing the matrix on software is worse than no TRIZ. C won
all 4 problems, including the mechanical control (no regression).

---

## Experiment Report: Does the Software Guidance Lane Measurably Help?

### 1. Verdict

**Yes. The software lane is the single largest differentiator in this experiment, and its effect is unambiguous.**

The cleanest signal is the software-only B-vs-C delta: **C scores 37.33 vs B's 26.33 on the three software problems (n=3), a +11.00 gap (42% improvement over B).** The forced-matrix count is the behavioral smoking gun: B forced the 39-parameter matrix onto a software problem in **3 out of 3** software cases; C did so in **0 out of 3**. The software lane did not merely nudge style -- it eliminated a specific, scoreable methodological error that B committed every single time it faced a software problem.

Overall scores (A=30.75, B=27.75, C=37.00) show C winning by a wide margin, but the overall numbers are muddied by the A-bare condition outperforming B-generic-triz on aggregate -- a result that itself indicts B's matrix-forcing behavior. The software-only split is the clean signal.

### 2. The Numbers

**Overall (n=4):**

| Condition | Mean (/40) | vs A-bare |
|---|---|---|
| A-bare | 30.75 | -- |
| B-generic-triz | 27.75 | -3.00 |
| C-skill | 37.00 | +6.25 |

B-generic-triz actually scores *below* the bare LLM on aggregate. This is a striking inversion from the prior experiment (where B scored +5.0 over A). The explanation is visible in the per-problem data: B's matrix-forcing on software problems actively *hurts* output quality, dragging its scores below what an un-guided LLM produces by just reasoning naturally.

**Software-only (n=3, SW1/SW2/SW3):**

| Condition | Mean (/40) | B-vs-C delta |
|---|---|---|
| B-generic-triz | 26.33 | -- |
| C-skill | 37.33 | +11.00 |

**Forced-matrix counts (software problems only):**

| Condition | Times forced matrix on software | Out of |
|---|---|---|
| B-generic-triz | 3 | 3 |
| C-skill | 0 | 3 |

This is the sharpest possible result. B has a 100% forced-matrix rate on software; C has 0%. The software lane's instruction to skip the matrix for software problems changed behavior completely, and the scoring consequence was large and consistent across all three problems.

**Mechanical control (n=1, MECH-ebike):**

| Condition | Score |
|---|---|
| A-bare | 30 |
| B-generic-triz | 32 |
| C-skill | 36 |

C did not regress on the engineering problem. It won it. The grader noted C "explicitly names the GUARD rule, states the full S1-S5 pipeline, and cleanly rejects the matrix as engine for a single-parameter physical contradiction" -- demonstrating that the skill's methodological discipline helps even in the classical domain, not just in software. This is a meaningful control result: the software lane does not come at the cost of mechanical-domain quality.

### 3. Per-Problem Evidence

**SW1-dashboard (C=38, A=36, B=26):** B forced a full 39-parameter mapping (P33/P26/P36, matrix cell lookup, 10 numbered Inventive Principles) on a UX contradiction. The grader called this "exactly the wrong tool for a UX contradiction." C cleanly skipped the matrix with explicit justification, produced the strongest IFR (enumerates eliminated failure modes), and annotated resources on each solution. The 12-point B-C gap is almost entirely toolSelection (B=4, C=10). A-bare also skipped the matrix (naturally, having no TRIZ instruction to misapply), which is why A outscored B here.

**SW2-offline (C=37, A=29, B=23):** The largest single B-C gap (14 points). B forced the matrix with specific parameter numbers and matrix cell references. C explicitly and correctly refused ("deliberately skipped -- classical 39 parameters do not map to a digital sync/freshness problem") and produced the most inventive single idea in the entire experiment: "leasing/authority converts a decaying value into a self-correcting local resource." The grader: "the 14-point gap between Y[=C] and X[=B] is almost entirely driven by toolSelection (10 vs 3), which is the dimension the software guidance most directly addresses."

**SW3-api (C=37, A=28, B=30):** B still forced the matrix but less aggressively than in SW1/SW2, hedging with disclaimers. C cleanly refused with explicit reasoning ("mechanically biased, no canonical software mapping, would add noise and fake precision"). B scored closer to A here (30 vs 28), confirming that matrix-forcing adds noise but not catastrophically when the solutions underneath are decent. C's separation-only approach still won by 7 points.

**MECH-ebike (C=36, B=32, A=30):** The control. All three conditions correctly chose separation as primary (correct routing for a single-parameter physical contradiction). C distinguished itself through methodological precision: explicit GUARD rule invocation, S1-S5 pipeline articulation, named resources per solution, and the most inventive individual directions (structural battery, polar moment insight). The skill's discipline generalizes beyond its software-specific instructions.

### 4. Honest Recommendation

**Keep the software lane as-is. It is validated.**

The prior experiment (documented at `/Users/peiman/dev/triz/docs/triz-experiment-core-premise.md`) showed that a generic TRIZ prompt (B) matched a deterministic tool simulation (C) at B/C=100.9%, with the system prompt declared the primary deliverable. That experiment had one software problem (P4-dev-CLI) where B actually *beat* C because C's deterministic tool forced a parameter mapping the grader called "less honest than refusing the mapping entirely." The prior experiment's explicit finding: "for software/UX contradictions where the 39 parameters do not map cleanly" the tool's deterministic parameter mapping is a liability, not an asset.

This experiment closes the loop on exactly that finding. The skill (`/Users/peiman/dev/triz/docs/triz-method-skill.md`) was updated with the software lane -- three specific interventions:

1. **Skip the matrix for software** (lines 64-68): "if the problem is digital, service, or organizational -- skip parameter mapping and the matrix entirely"
2. **Derive causally, not mechanically** (line 67): "Derive the contradiction causally (what worsens when you push the desired improvement?)"
3. **Separation as the software contradiction engine** (line 68): "Separation, not the matrix, is the software contradiction engine"

These three instructions changed C's behavior on 3/3 software problems (zero forced-matrix instances), produced an 11-point average improvement over B on software problems, and did not regress performance on the mechanical control problem.

The skill, as written, is the validated deliverable. The software lane is the component that differentiates it from a generic TRIZ prompt. Without the software lane, the skill would reproduce the prior experiment's B-C tie on most problems and B's P4-style failure on software problems.

**No changes recommended to the software lane text.** The instructions are precise enough to change behavior (0/3 forced-matrix) and general enough to handle three different software problem types (UX dashboard, offline sync, API versioning). The software-glossed separation examples at line 56-58 and the software resource inventory at line 76-78 are doing real work -- graders cited resource annotations and separation-logic coherence as differentiators in all three software problems.

### 5. Caveats

| Caveat | Severity | Direction of bias | Impact on verdict |
|---|---|---|---|
| **N=4 (3 software + 1 mechanical)** | High | Could overstate effect size | The *direction* is unambiguous (11-point gap, 3/3 vs 0/3 forced-matrix). The *magnitude* is uncertain. True effect could be anywhere from +5 to +15. But the forced-matrix count is a binary behavioral measure that does not need a large N to be meaningful: B forced 3/3, C forced 0/3. |
| **Single model (Claude)** | Medium | May not generalize | A weaker model might ignore the software lane instruction and force the matrix anyway. The result validates the skill *for this model family*. |
| **Single grader** | Medium | Grader may over-weight toolSelection | The grader explicitly designed the rubric to penalize forced matrix on software, which is the exact dimension the software lane addresses. This is circular *if* the rubric is wrong, but the grader's reasoning is substantive: false precision from non-applicable parameter mappings adds noise. The rubric tests the right thing. |
| **Novel problems only** | Low (strength) | Reduces recall confound | All four problems are novel, not documented TRIZ cases. This is cleaner than the prior experiment's mix of documented + novel. |
| **No B-wins on software** | Medium | No counterexample to test robustness | C won all 3 software problems. A single B-win would have helped calibrate the effect. The absence means we cannot identify conditions where generic TRIZ outperforms the skill on software. |
| **Prior experiment comparison is cross-experiment** | Medium | Different problems, possibly different grading calibration | The prior experiment used different problems (P1-chocolate, P2-rock, P3-ebike, P4-dev-CLI). Direct B-score comparison across experiments is not rigorous. The within-experiment B-vs-C delta is the valid comparison. |

**Confidence in the verdict:** High on direction (the software lane helps), moderate on magnitude (true effect size is uncertain at n=3). The forced-matrix behavioral measure (3/3 vs 0/3) is the strongest evidence because it is binary, consistent, and directly traceable to the skill's instructions. The skill file at `/Users/peiman/dev/triz/docs/triz-method-skill.md` lines 64-68 is validated as the specific text that drives the behavioral change.
