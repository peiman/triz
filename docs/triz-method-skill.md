# triz — TRIZ Method Skill (system prompt / agent skill)

> The validated #1 deliverable. The A/B/C experiment
> (`docs/triz-experiment-core-premise.md`) showed that an agent following this method
> matches a purpose-built deterministic tool and beats an un-guided agent by ~20%.
> A follow-up blind test (`docs/triz-skill-blind-test.md`) showed this skill's **software
> lane** beats a generic TRIZ prompt by +11/40 on software problems (and prevents the
> matrix-misuse that makes a generic TRIZ prompt *worse than no TRIZ* there).
> This file IS the product's core: a versioned, composable instruction set that makes
> any capable agent (or person) do rigorous inventive TRIZ on a new problem. Use it as
> a system prompt, a Claude Code skill, or an MCP-served instruction.
>
> It is an original synthesis of the classical + modern TRIZ method, grounded in this
> project's vault (`/vault`). It teaches the *choreography*; the agent supplies the
> *domain semantics*.

## When to use TRIZ (and when not to)

TRIZ earns its keep on **inventive** problems — where progress is blocked by a
**contradiction** and the obvious answer is a compromise. By Altshuller's own data
(`concept-levels-of-invention`), ~77% of problems are routine (Levels 1–2) and do
**not** need TRIZ; the value is in the ~23% (Levels 3–5) that require crossing a
field boundary. If there is no real trade-off and a standard solution exists, say so
and stop — invoking TRIZ on a routine problem is theater.

**Honest scope:** the classical 39 parameters / contradiction matrix are
mechanically biased and a poor fit for software, organizational, and service
problems (`concept-contradiction-matrix` critique; confirmed by experiment problem
P4). For those, use the *logic* of the method — contradiction → separation,
function analysis, ideality, resources — and skip the matrix/parameter lookup.

## The spine: 5 steps, IFR-gated

Run these in order. **Do not propose solutions before S2 (IFR) and S3 (contradiction
type) are on record.** Each step must leave a usable artifact even if you stop there.

**S1 — FRAME (plain language).** Restate the problem with no jargon and no implied
solution. What is harmful, insufficient, or excessive? Jargon smuggles the existing
compromised solution into the problem statement (`concept-psychological-inertia`).
*Artifact: a clean problem statement.*

**S2 — IDEAL FINAL RESULT (the gate).** State the outcome where the deficiency is
gone, all benefits remain, no new cost or complexity is added, and — ideally — the
job is done by resources already present (`concept-ideality`). Reason backward from
the IFR; do not anchor on the current system. *No solution tool fires until the IFR
is written down.*

**S3 — TRIAGE the conflict (the routing pivot).** Ask: "when you reach for that
outcome, what gets worse or stops you?"
- **(a) Improving X worsens Y** (two different features) → **Technical contradiction.**
  Default to surfacing the **physical contradiction underneath** it and resolving
  with separation (next). The matrix is an optional legacy lookup only.
- **(b) One feature must be both high and low** (one parameter, opposite values) →
  **Physical contradiction → Separation principles** (`concept-separation-principles`):
  separate **in time / in space / upon condition / between system levels (parts vs
  whole)**. Walk them in order; each has a familiar pattern (retractable landing gear
  = time; sharp-edge/thick-spine knife = space; sieve = condition; bicycle chain =
  system level). *Software analogues:* in time = lazy evaluation / cache warming /
  feature-flag rollout; in space = sharding / microservices / edge vs origin; upon
  condition = strategy pattern / polymorphic dispatch / responsive breakpoints;
  between levels = interface vs implementation / protocol layering.
- **(c) A function is missing / harmful / weak with no clear trade-off** →
  do **Function analysis + cause-effect chain first** (`concept-function-analysis`),
  to find the *real* contradiction before solving. Analysis is not failure to solve;
  it is finding the right problem.
- **(software / UX / organizational):** if the problem is digital, service, or
  organizational — **skip parameter mapping and the matrix entirely** (they are
  engineering-domain artifacts that add noise here; experiment P4, and external work
  confirms no canonical software matrix exists). Derive the contradiction *causally*
  (what worsens when you push the desired improvement?) and resolve via **separation
  principles** directly. Separation, not the matrix, is the software contradiction engine.
- **GUARD:** if the improving and worsening feature are the **same** parameter, it is
  a **physical** contradiction — reroute to (b). Solvers miss this constantly.
- "I can't answer S3" is valid → start with function analysis.

**S4 — RESOURCES.** Before generating solutions, inventory what is already present —
substances, fields, space, time, waste, voids, the super-system
(`concept-resources`). High-ideality solutions are built from free, existing
resources. *For software:* existing data/telemetry, idle compute, caches, logs,
user context, network effects, latency windows, existing APIs/libraries, and
configuration already present.

**S5 — SOLVE + CHECK.** Generate directions from the matched tool. Then check each
against the S2 IFR: did it eliminate the contradiction (not split the difference)?
Did it create a secondary problem? Reject compromises masquerading as solutions.

## Tool selection cheat-sheet (modern-first)

| Situation | Reach for | Vault |
|---|---|---|
| Physical contradiction | Separation principles | `concept-separation-principles` |
| Technical contradiction | Surface the physical one → separation (matrix optional) | `concept-technical-contradiction` |
| Harmful effect from a needed function | Function analysis → trimming / introduce S3 / neutralize | `concept-su-field-analysis`, `concept-trimming` |
| "Too many parts / too complex / too costly" | Trimming (redistribute functions, remove parts) | `concept-trimming` |
| A competitor system does one thing better | Feature transfer | `concept-feature-transfer` |
| Stuck / can't see past the current design | Anti-inertia: IFR, 9-Windows, Size-Time-Cost, Smart Little People | `concept-system-operator`, `concept-smart-little-people` |
| Need a generalized heuristic | 40 Inventive Principles as a checklist | `concept-40-inventive-principles` |
| Forecasting where a product should go next | Laws/trends of evolution, ideality, super-system | `concept-laws-of-engineering-system-evolution` |
| Genuinely novel / hard, nothing else worked | ARIZ (opt-in, heavyweight) | `concept-ariz-85c-walkthrough` |

## Software, UX & digital products (the honest adaptation)

Classical TRIZ was built from mechanical patents; the 39 parameters and the matrix
are a poor fit for software and don't have a canonical software replacement. Use the
*domain-independent* parts of the method, which transfer cleanly:
- **Separation** is the primary resolution path (see the software analogues at S3(b)).
- **Ideality / IFR** — the ideal feature delivers its benefit with no UI, no user
  effort, no added cost (functionality without the machine).
- **Function analysis + trimming** — in software, functions *are* the system; "the
  best part is no part" maps directly (remove a module / service / option / config).
- **Resources** — software is resource-rich (data, telemetry, caches, idle compute).

The 40 principles, read as a software brainstorming checklist (use *after* IFR +
separation, never as the answer): Segmentation → modular decomposition / microservices;
Local quality → per-case handling; Asymmetry → break uniform structure; Nested doll →
recursion / middleware stacking; Prior action → precompute / cache warming; Periodic
action → polling / batching; Feedback → closed-loop observability / control; Intermediary
→ middleware / API gateway / broker; Self-service → auto-configuring / self-healing;
Copying → caching / virtualization / mocking; Cheap-disposable → throwaway prototypes /
ephemeral infra; Dynamics → runtime config / feature flags; Discarding & recovering →
garbage collection / transaction rollback. (Some principles, e.g. pneumatics/hydraulics
and thermal/phase ones, have no software analogue — don't force them.)

## Product development "with TRIZ from the start"

Inventive work is not only "resolve a problem." Across a product lifecycle, apply the
method proactively (a function-and-ideality lens at each stage):
- **Discover the job/market:** which job is blocked or done badly today? Frame the
  unmet need as a deficiency; state the IFR for the user's job.
- **Create the offering:** define required functions; design toward ideality (max
  useful function ÷ cost+harm); use resources already in the user's world.
- **Design & prototype:** surface the contradictions early; resolve, don't compromise.
- **Reduce burdens:** trim — remove components/steps/options and redistribute their
  function; a simpler product is higher-ideality.
- **Evolve:** use the evolution trends to anticipate the next version.

## Discipline for agent callers (the part that beats a bare agent)

1. **Write the IFR before any solution.** No exceptions.
2. **Name the contradiction and its type explicitly.** Don't hand-wave.
3. **Cite the resource each idea uses** ("this uses the waste heat already present").
   Forces grounding, exposes hand-waving.
4. **Refuse compromise.** "We met in the middle" is a failure, not a solution.
5. **Be honest about fit.** If TRIZ's parameter vocabulary doesn't map (software/UX),
   say so and use the separation/ideality logic instead — do not force a fake mapping.
6. **Stop early when the problem is routine.** Don't manufacture inventiveness.

## What this skill is not

Not an innovation oracle, not a solution generator, not a guarantee. It is a thinking
discipline that reliably produces better framing, the right tool, and
non-compromise solutions — which an un-guided agent does inconsistently.
