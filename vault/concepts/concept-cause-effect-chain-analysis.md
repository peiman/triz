---
id: concept-cause-effect-chain-analysis
type: concept
title: "Cause-Effect Chain Analysis (CECA)"
created: 2026-05-31
aliases:
  - CECA
  - Cause-Effect Chain Analysis
  - Root-Conflict Analysis
  - RCA+
tags:
  - triz
  - tools
  - analysis
  - modern-triz
confidence: medium
related_ids:
  - concept-function-analysis
  - concept-technical-contradiction
  - concept-physical-contradiction
  - concept-trimming
  - concept-ariz
source_ids:
  - source-terninko-zusman-zlotin-1998
  - source-souchkov-xtriz
  - source-matriz-knowledge-base
---

## Overview

**Cause-Effect Chain Analysis (CECA)** is a modern-TRIZ analytical tool that builds
a directed chain (a graph) from an observed **disadvantage** backward through its
causes to the **root causes**, in order to pick the **key disadvantage** — the cause
whose elimination removes the most downstream harm. It answers a different question
than [[concept-function-analysis|function analysis]]: function analysis models *what
components do to each other*; CECA models *why a disadvantage exists*. It is the
missing link between "my function model shows harmful/insufficient functions" and
"I now know which contradiction to formulate."

## Key Properties

- Start from a target disadvantage and ask "why?" repeatedly, recording each cause
  as a node; causes branch (**OR**: several independent causes) and converge
  (**AND**: causes that must co-occur).
- Terminate a branch at a **root cause** — a cause that is a law of nature, a
  deliberate requirement, or outside the system boundary (you stop, not solve).
- Select the **key disadvantage / key contradiction**: the node whose removal
  eliminates the most harm at the least cost. Feed it into
  [[concept-technical-contradiction|contradiction]] formulation or
  [[concept-trimming|trimming]].
- **Root-Conflict Analysis (RCA+)** (Souchkov) is a contradiction-oriented variant:
  it derives the contradiction directly from the causal chain and resolves it with
  [[concept-separation-principles|separation principles]] — **skipping the matrix**.
  This makes CECA/RCA+ especially useful for software and business problems where
  the classical parameters do not fit.

## Connections

The analytical bridge from [[concept-function-analysis|function analysis]] to
[[concept-technical-contradiction|contradiction]] resolution and
[[concept-trimming|trimming]]; a building block of the
[[concept-triz-product-development-workflow|product-development workflow]].
