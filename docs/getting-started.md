# Getting Started with triz

**triz is a TRIZ method you run *with* an AI agent (or by hand) to break through a hard problem that's stuck on a trade-off — instead of settling for a compromise.**

TRIZ (Theory of Inventive Problem Solving) is a body of methods, distilled by Genrich Altshuller from patent analysis, for resolving the *contradictions* that block inventive problems. triz packages that method as a versioned skill you can drop into Claude Code, paste into any LLM, or back with an optional CLI and knowledgebase.

---

## Is triz for you?

triz is honest about where it helps. A quick gut-check:

- **Yes, if you're stuck on a real trade-off** — "make it lighter *but* keep it strong," "lower latency *but* keep cost down," "more features *but* simpler to use" — and the obvious answer is to split the difference. That's the inventive ~23% of problems where TRIZ earns its keep.
- **Probably not, if your problem is routine.** By Altshuller's data, most problems have a known standard solution and don't need TRIZ. The skill will tell you this and stop, rather than manufacture inventiveness.
- **Mechanical / physical trade-offs are the sweet spot.** Weight vs. strength, speed vs. accuracy, power vs. heat — these map cleanly onto the classical 39 parameters and the contradiction lane.
- **Software / UX / organizational problems work too — just differently.** The classical 39-parameter contradiction matrix is mechanically biased and a poor fit there (in our tests, forcing it measured *worse* than no TRIZ at all). For those, triz uses the domain-independent logic — separation principles, ideality, function analysis, resources — and *skips* the matrix.

You don't have to know any of this theory to start. The method runs the choreography for you.

---

## Quick start (60 seconds)

Pick **one** path. You do not need all three.

| Path | Use this if… | What you get |
|---|---|---|
| **A — Claude Code** *(recommended)* | You use Claude Code | The skill auto-triggers on trade-off language and runs the full method. No setup beyond copying one folder. |
| **B — Any LLM** | You use ChatGPT / Gemini / another agent | Paste one Markdown file as the system prompt; state your problem. |
| **C — The CLI** | You want a scriptable vocabulary lookup + contradiction classifier | A small `triz` binary for two deterministic engineering operations. It *frames*, it doesn't *solve*. |

**You do not need Rust, the CLI, or any special tooling to use the method** — Paths A and B are pure prompting. The CLI (Path C) is an optional convenience.

---

## Path A — Use it with Claude Code (recommended)

This is the primary path. The repo ships a ready-to-use Claude Code skill at `.claude/skills/triz/SKILL.md`. It's self-contained — the core method lives entirely in that one file. You do **not** need the Rust CLI, the data files, or vaultmind for it to work.

### 1. Get the skill

```bash
git clone https://github.com/peiman/triz.git
cd triz
```

### 2. Install it — global or per-project

**Global** (available in *every* Claude Code session on your machine):

```bash
cp -R .claude/skills/triz ~/.claude/skills/
```

**Per-project** (available only inside one of your projects):

```bash
cp -R .claude/skills/triz /path/to/your-project/.claude/skills/triz/
```

| Where you installed | Where it's active |
|---|---|
| `~/.claude/skills/triz/` | Every Claude Code session on this machine |
| `<your-project>/.claude/skills/triz/` | That project only |
| The cloned `triz` repo itself | Already active here — **no copy needed** |

### 3. Confirm it's there

The skill is installed when this file exists:

```bash
ls ~/.claude/skills/triz/SKILL.md
```

If you had a Claude Code session open, start a fresh one so it picks up the new skill.

### 4. How it triggers

The skill **auto-triggers** when you describe a problem in trade-off / contradiction language — for example:

- "X fights Y" / "we keep trading off A vs B"
- "resolve this contradiction"
- "I need an inventive, non-compromise solution to…"

If it doesn't fire on its own, force it explicitly:

```
apply TRIZ to: <your problem>
```

It intentionally **stays quiet on routine problems** that have a standard solution — that's by design, not a bug.

### A short worked example

You type, in any project:

> We need this drone arm ~30% lighter, but it cannot lose stiffness or it'll vibrate. Every option so far is a compromise.

The skill runs the IFR-gated spine:

- **S1 — Frame.** "An arm must carry the same load with the same stiffness, at ~30% less mass. Currently mass and stiffness rise and fall together."
- **S2 — IFR (the gate).** "The arm provides full stiffness where the load path needs it, and contributes *zero* mass everywhere else — ideally using structure already present." *(No solution is proposed until this is written down.)*
- **S3 — Triage.** "Improving *weight* worsens *strength/stiffness* → **technical contradiction**. The underlying physical contradiction: the cross-section must be *thick* (for stiffness) and *thin* (for light weight) at the same time."
- **S4 — Resources.** "Already present: the load is non-uniform along the arm; most of the arm is lightly stressed; the inside is solid material doing nothing structural."
- **S5 — Solve + Check.** Separation **in space** — thick/solid along the load path, thin or hollow elsewhere: topology optimization, internal ribbing/lattice, a tapered hollow section, or a stiffer material that lets you remove mass. Each is checked against the IFR: *does it keep full stiffness with less mass, or just split the difference?* Compromises are rejected.

You get **2–3 concrete, non-compromise directions** — not a lecture, and not "just meet in the middle."

---

## Path B — Use it with any LLM

No Claude Code required. Works with ChatGPT, Gemini, Claude.ai, a local model — anything you can give a system prompt to.

1. Get the method file. Either clone the repo (above) and open `docs/triz-method-skill.md`, or download just that one file:

   ```bash
   curl -O https://raw.githubusercontent.com/peiman/triz/main/docs/triz-method-skill.md
   ```

2. **Copy its entire contents** and paste it as the **system prompt** (or the first message) of your chat / agent.
3. *(Optional, for engineering problems)* Attach `data/parameters.json` (the 39 engineering parameters + an engineer-vocabulary alias thesaurus) and `data/principles.json` (the 40 inventive principles) so the model can do lookups.
4. State your problem in plain language.

A ready-to-paste example to try after loading the method:

> Apply TRIZ. Our espresso machine needs to heat water faster, but a more powerful heater overshoots the target temperature and scorches the shot. We keep trading off speed against temperature stability.

A good answer restates the problem cleanly, writes the **Ideal Final Result** *before* proposing anything, names it as a contradiction (here a physical one — the heater must be both high-power and low-power), inventories resources, then offers non-compromise directions (e.g. separation in time: a high-power burst plus a controlled cutoff; or separation in space: a small fast pre-heater feeding a stable reservoir) — each checked against the IFR.

---

## Path C — Use the CLI

The CLI is an **optional, deliberately thin** front-end. It does the two deterministic things an LLM does *unreliably* on its own:

1. `parameter-search` — map everyday engineer vocabulary to the right TRIZ engineering parameter(s).
2. `formulate-contradiction` — classify a conflict as **technical** or **physical** and route it.

> **Important:** the CLI **frames**, it does not **solve**. It hands you the contradiction type and the right resolution lane — solution *generation* lives in the skill/agent (Paths A and B). And it is **engineering-domain only**.

### Prerequisites

- **A recent stable Rust toolchain** (edition 2021) — install via [rustup](https://rustup.rs/). This is the *only* thing you need for the CLI. No minimum version is pinned; if you have an old toolchain lying around, run `rustup update` first.
- The first `cargo install` **downloads and compiles dependencies**, so it needs internet access and takes roughly half a minute the first time.
- `just` (the task runner) is **optional** — it's a convenience wrapper; plain `cargo` works everywhere below.
- vaultmind is **not** needed for the CLI (only for the vault path — see *Going deeper*).

### Install (one command)

From a clone of the repo, install a runnable `triz` binary onto your PATH:

```bash
cargo install --path crates/cli
```

This installs a binary named **`triz`** to `~/.cargo/bin/`. That directory is normally on your `PATH` (rustup sets it up). Confirm with:

```bash
which triz      # → /Users/you/.cargo/bin/triz
```

If `which triz` finds nothing, add `~/.cargo/bin` to your `PATH`.

**Build-only alternative** (no install):

```bash
cargo build --release
# → binary at ./target/release/triz
```

### Verify it worked

```bash
triz --version          # → triz 0.1.0 (or the current version)
triz parameter-search durability
```

Expected:

```
Parameters matching "durability":
14 — Strength
15 — Duration of action of moving object
16 — Duration of action of stationary object
27 — Reliability
```

### The two commands

**`parameter-search <query>`** — ranked engineering parameters for a free-text term:

```bash
$ triz parameter-search durability
Parameters matching "durability":
14 — Strength
15 — Duration of action of moving object
16 — Duration of action of stationary object
27 — Reliability
```

**`formulate-contradiction --improving <x> --worsening <y>`** — classify and route a conflict:

```bash
$ triz formulate-contradiction --improving weight --worsening strength
Improving: 1 — Weight of moving object
Worsening: 14 — Strength
Kind: Technical
Technical contradiction (two parameters conflict). Surface the physical contradiction underneath, then resolve with separation principles. The 39x39 contradiction matrix is optional/legacy and is not consulted here.
Note: These 39 parameters are engineering-domain only. For software/UX/organizational problems, skip parameters and go straight to separation principles + function analysis.
```

These are the **only** two domain commands (plus `ping`). For the full flag list:

```bash
triz --help
triz parameter-search --help
triz formulate-contradiction --help
```

### JSON output for scripts and agents

Add the global `--output json` flag. It's a global flag, so it works **before or after** the subcommand:

```bash
triz --output json parameter-search durability
triz parameter-search durability --output json     # equivalent
```

Every command returns the same envelope — `status`, `command`, `data`. (Keys are emitted in alphabetical order; key order isn't semantically meaningful.)

```json
{
  "status": "success",
  "command": "parameter-search",
  "data": {
    "matches": [
      {
        "gloss": "Resistance to changing under an applied force.",
        "matched_on": "durability",
        "name": "Strength",
        "number": 14,
        "score": 100
      }
    ],
    "query": "durability"
  }
}
```

`formulate-contradiction` JSON carries the classification in `data.kind` (`"technical"` or `"physical"`), both resolved sides, the `route`, and a `note`:

```json
{
  "status": "success",
  "command": "formulate-contradiction",
  "data": {
    "improving": { "name": "Weight of moving object", "number": 1, "query": "weight" },
    "kind": "technical",
    "note": "These 39 parameters are engineering-domain only. …",
    "route": "Technical contradiction (two parameters conflict). …",
    "worsening": { "name": "Strength", "number": 14, "query": "strength" }
  }
}
```

Tip: `triz --output json parameter-search durability | jq '[.data.matches[].number]'` gives you `[14,15,16,27]`.

### Engineering-domain-only — what that looks like

The parameters cover engineering/physical quantities only. There is **no `applies: false` boolean** — instead the scope is surfaced honestly in ways you can detect:

- A **true no-match** returns an empty array (`data.matches == []`) and prints, in human mode:
  > `No match for "asdfqwerzzz" — these parameters are engineering-domain only; … this may be a software/UX problem where parameters don't apply.`
- Every `formulate-contradiction` result carries the **`note`** field reminding you parameters are engineering-only. If a side doesn't resolve, its `number`/`name` are omitted in JSON and shown as `(no parameter matched "…")` in human mode.

For a software term you'll often get a weak, low-`score` fuzzy match rather than a clean one — a signal to switch to Path A/B and skip the matrix. Programmatically: branch on `matches` being empty and/or `score` being low; treat the `note` as the canonical scope reminder.

### Exit codes & errors

- **`0`** on success — *including* a no-match search (`matches: []`) and an unresolved contradiction side. A no-match is a valid answer, not an error.
- **`2`** for command-line usage errors (unknown subcommand, missing required flag). These are caught by the argument parser, which prints its own message to **stderr** — even in `--output json` mode there is no JSON envelope for these, because parsing fails before the app runs.
- **`1`** for application errors that occur *after* parsing (e.g. a bad `--config` path). In `--output json` mode these come back as a JSON envelope with `"status": "error"` on **stdout** (stderr stays clean); in human mode the error goes to **stderr**.

---

## A worked example, end to end

Let's take the engineering case all the way through, mixing the CLI (to *frame*) with the method (to *solve*).

**The problem:** *Make a steel mounting bracket ~30% lighter without losing its load capacity.* Every idea so far means accepting a weaker part.

**S1 — Frame (plain language).** A bracket must carry the same load while weighing ~30% less. Right now, removing material to save weight also removes strength.

**S2 — IFR (the gate).** The bracket carries its full rated load while its mass drops ~30% — ideally by removing only material that isn't carrying load. *Reason backward from this; don't anchor on the current solid shape. No solution fires until this is written.*

**S3 — Triage the contradiction.** Use the CLI to put precise names on it:

```bash
$ triz parameter-search "load capacity"
Parameters matching "load capacity":
14 — Strength
…

$ triz formulate-contradiction --improving weight --worsening strength
Improving: 1 — Weight of moving object
Worsening: 14 — Strength
Kind: Technical
Technical contradiction (two parameters conflict). Surface the physical contradiction underneath…
```

So it's a **technical contradiction** between *weight* (1) and *strength* (14). The physical contradiction underneath: the material must be *present* (for strength) and *absent* (for light weight) in the same part.

**S4 — Resources (already present, free).** The load path through the bracket is non-uniform — most of the part is lightly stressed. The geometry has unused volume. The same material can be re-shaped without buying anything new.

**S5 — Solve + Check (separation, not compromise).** Resolve the "present *and* absent" contradiction by **separating in space** — keep material exactly where the load flows, remove it where it doesn't:

1. **Topology optimization / organic ribbing** — solid along the computed load path, hollowed elsewhere.
2. **I-beam / box cross-section** — concentrate area where bending stress is highest (separation in space, classic form).
3. **Material substitution that keeps stiffness** — e.g. a higher specific-stiffness alloy or composite, so you remove mass without removing capacity.

Check each against the IFR: each aims to keep *full* load capacity at lower mass — not to "split the difference" by accepting a weaker bracket. That's the payoff: concrete directions that dissolve the trade-off.

---

## 1-minute TRIZ glossary

- **Technical contradiction** — when improving one feature makes a *different* feature worse (e.g. more strength → more weight). Two parameters in conflict.
- **Physical contradiction** — when *one* feature must take *opposite* values at the same time (e.g. a knife edge must be sharp *and* thick; coffee water must be hot *and* not-too-hot). One parameter, two demands.
- **IFR (Ideal Final Result)** — a description of the outcome where the deficiency is gone, all the benefits remain, and *no* new cost, harm, or complexity is added — ideally with the job done by resources already present. You write it *before* proposing solutions; it's the gate the whole method hangs on.
- **Separation principles** — the main way to resolve a physical contradiction: satisfy the opposite demands by separating them **in time**, **in space**, **upon a condition**, or **between system levels** (part vs. whole). Retractable landing gear separates in time; a sharp-edged, thick-spined knife separates in space.
- **Ideality** — the ratio of useful function to cost + harm. Higher is better; the ideal system delivers its benefit with no added machine, effort, or cost. It's the direction every TRIZ move should push toward.

---

## Going deeper

- **The method, in full** — [`triz-method-skill.md`](triz-method-skill.md). The canonical, versioned instruction set behind the skill. Read it to understand the spine, the tool-selection cheat-sheet, and the honest software adaptation.
- **The data** — [`../data/parameters.json`](../data/parameters.json) (39 engineering parameters + alias thesaurus, engineering-only) and [`../data/principles.json`](../data/principles.json) (40 inventive principles). Language-agnostic JSON any agent or tool can consume.
- **The knowledgebase (the vault)** — a 150+ note knowledgebase covering classical *and* modern TRIZ, every non-obvious claim cited. It's **optional** and needs [vaultmind](https://github.com/peiman/vaultmind) installed. Query it:

  ```bash
  vaultmind ask "how do I resolve a physical contradiction" --vault vault --pointers-only
  vaultmind note get concept-separation-principles --vault vault
  ```

- **The evidence** — the experiments in [`triz-experiment-core-premise.md`](triz-experiment-core-premise.md) and [`triz-skill-blind-test.md`](triz-skill-blind-test.md). The honest record of how the method was validated: the skill beats an un-guided agent by ~20%, and beats a *generic* TRIZ prompt by +11/40 on software problems. We don't claim more than that.

---

## FAQ / troubleshooting

**The skill didn't auto-trigger in Claude Code.**
Force it: start your message with `apply TRIZ to: …`. Also confirm it's installed (`ls ~/.claude/skills/triz/SKILL.md`) and start a fresh session if one was already open. Note: the skill deliberately stays quiet on routine problems that have a standard solution.

**Do I need Rust?**
No — only for the optional CLI (Path C). The method itself (Paths A and B) is pure prompting with no toolchain.

**Do I need Claude Code?**
No. Path B loads `docs/triz-method-skill.md` into any LLM. Claude Code (Path A) is just the smoothest experience because the skill auto-triggers.

**Does the skill need the CLI, the data files, or vaultmind to run?**
No. The core method is self-contained in `SKILL.md`. The `data/*.json` files are *optional* lookups, and the CLI and vault are entirely optional and not required for the skill.

**Can I use it for a software / UX problem?**
Yes — the method works. It just **skips the 39 parameters and the contradiction matrix** (which don't fit software) and resolves the contradiction *causally* via separation principles, ideality, and function analysis. The CLI's parameter commands, however, are engineering-only.

**Is the 39×39 contradiction matrix included?**
The 39 parameters ship in `data/parameters.json`, but the method treats the classical matrix as an **optional legacy lookup** and routes you instead to surfacing the physical contradiction and resolving it with separation. The CLI does not consult the matrix.

**vaultmind isn't installed.**
That's fine — it's only needed for the optional vault path. Everything in Paths A, B, and C works without it.

**Do I have to `git clone`?**
For the Claude Code skill (Path A) and the CLI (Path C), yes — clone the repo. For Path B you only need the single file `docs/triz-method-skill.md`, which you can download directly:
`curl -O https://raw.githubusercontent.com/peiman/triz/main/docs/triz-method-skill.md`.

---

*See the [README](../README.md) for the project rationale and evidence, the [method skill](triz-method-skill.md) for the full instruction set, and the dual [MIT](../LICENSE-MIT) / [Apache-2.0](../LICENSE-APACHE) license.*
