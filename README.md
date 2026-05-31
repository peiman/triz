# triz

**An agent-native TRIZ method for inventive product development — used *with* AI agents to attack new problems "with TRIZ in mind from the start," and to guide anyone through the process the right way.**

TRIZ (Theory of Inventive Problem Solving) is a body of methods, distilled by Genrich
Altshuller from patent analysis, for resolving the *contradictions* that block hard
problems — instead of accepting a compromise. `triz` packages the method as something an
agent (or a person) can actually use, backed by a cited knowledgebase and validated by
experiment.

## What this is (and what the evidence says it should be)

We designed `triz` by **dogfooding TRIZ on itself** and then **testing the premise before
building** (the experiments are in `docs/`). The headline finding: a capable LLM already
"knows" TRIZ, so the highest-leverage deliverable is **the method as a versioned skill**,
not a heavyweight tool. So the product is, in order of validated value:

1. **The method skill — [`docs/triz-method-skill.md`](docs/triz-method-skill.md).** A
   versioned, composable instruction set (use it as a system prompt, a Claude Code skill,
   or an MCP instruction). It runs the IFR-gated 5-question spine (Frame → IFR → Triage →
   Resources → Solve+Check), routes to the right tool, and enforces the discipline that
   makes inventive work rigorous. **Validated:** it beats an un-guided agent by ~20% and a
   *generic* TRIZ prompt by **+11/40 on software problems** (`docs/triz-skill-blind-test.md`).

2. **The data — [`data/`](data/).** `parameters.json` (the 39 Engineering Parameters with a
   blind-tested engineer-vocabulary **alias thesaurus**, 100% top-3 hit) and `principles.json`
   (the 40 Inventive Principles with sub-principles and examples). Language-agnostic JSON; an
   agent or any tool can consume it directly. **Engineering-domain only** (see scope below).

3. **The knowledgebase — [`vault/`](vault/).** A 150+ note [vaultmind](https://github.com/peiman/vaultmind)
   knowledgebase covering the full classical *and* modern TRIZ toolkit — contradictions, the
   matrix, ideality/IFR, ARIZ, su-field, separation principles, function analysis, trimming,
   cause-effect chain analysis, evolution trends, software-TRIZ, JTBD, and more — every
   non-obvious claim cited, uncertainty flagged. Query it:
   ```bash
   vaultmind ask "how do I resolve a physical contradiction" --vault vault --pointers-only
   vaultmind note get concept-separation-principles --vault vault
   ```

4. **The evidence trail — [`docs/`](docs/).** The strategy, the user story map, and three
   blind experiments — the honest record of how the above was decided and validated.

## Honest scope

- TRIZ earns its keep on the **inventive ~23%** of problems (Levels 3–5); routine problems
  don't need it. The skill says so and stops.
- The classical **39 parameters and contradiction matrix do not fit software/UX/organizational
  problems** — confirmed by our experiments and the external literature (no canonical software
  matrix exists). For those, the skill uses the domain-independent logic: **separation
  principles, ideality, function analysis, resources** — and *skips* the matrix. Forcing the
  matrix on software is measurably *worse than no TRIZ at all*.

## Using it with an agent

Load `docs/triz-method-skill.md` as the agent's system prompt or skill. Optionally give the
agent `data/parameters.json` / `data/principles.json` for deterministic lookups, and the
`vault/` for deep recall. Then state a problem and let the agent run the spine — IFR first,
name the contradiction, cite the resources, refuse compromise.

## The Rust CLI (optional, scaffolded)

This repo is a [ckeletin-rust](https://github.com/peiman/ckeletin-rust) Cargo workspace
(`crates/domain` · `crates/infrastructure` · `crates/cli`). The experiments showed a heavy
deterministic tool isn't justified over the skill, so the CLI is **deliberately thin**: the
only operations worth compiling are `parameter-search` and `formulate-contradiction` over the
`data/` files. None of that is built yet — the skill + data are the product today.

```bash
just check    # fmt + clippy + test + deny (scaffold still green)
```

## Provenance

The vault is an **original synthesis** cited to authoritative sources (Altshuller, MATRIZ,
Tate & Domb, Souchkov, Mann, Rea, Christensen/Ulwick, …). It is **not** derived from any
single copyrighted curriculum; where a non-copyrightable structural idea (e.g. "skills compose
into product-development jobs") was useful, the *content* was authored independently from those
references. The classical parameter/principle *names* are functional facts of the field.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
