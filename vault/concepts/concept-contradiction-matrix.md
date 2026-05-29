---
id: concept-contradiction-matrix
type: concept
title: "Contradiction Matrix"
created: 2026-05-29
aliases:
  - Altshuller Matrix
  - TRIZ Matrix
tags:
  - triz
  - tools
related_ids:
  - concept-technical-contradiction
  - concept-39-engineering-parameters
  - concept-40-inventive-principles
source_ids:
  - source-matriz-knowledge-base
  - source-altshuller-1984-creativity-exact-science
---

## Overview

The **Contradiction Matrix** is a **39 × 39 lookup table** for resolving
[[concept-technical-contradiction|technical contradictions]]. The solver
identifies the parameter to improve and the parameter that consequently worsens;
the intersecting cell lists the
[[concept-40-inventive-principles|Inventive Principles]] most statistically
frequent in patents that solved that type of contradiction.

## Key Properties

- **Rows** = the parameter to be improved; **columns** = the parameter that
  worsens. Both axes use the same
  [[concept-39-engineering-parameters|39 Engineering Parameters]] in the same order.
- **Cells** contain the numbers of **up to 4 Inventive Principles**; the order
  within a cell carries **no preference ranking**.
- The matrix is **asymmetric**: improving A while B worsens yields different
  principles than improving B while A worsens.
- The **diagonal** (a parameter conflicting with itself) corresponds to a
  [[concept-physical-contradiction|physical contradiction]] and is generally not
  addressed by the matrix.
- **Empty cells** mean no principle was found statistically more frequent — not
  that the contradiction is unsolvable.

## Critique (flagged)

The matrix was built from Altshuller's patent analysis (patent counts cited
across sources range widely, ~40,000 to >1.5 million), with data predating
~1970 — so critics argue it is biased toward mechanical-era inventions and
under-represents software/biotech/electronics. Darrell Mann and colleagues
produced an updated **"Matrix 2003"** (later 2010) from modern patents; the Mann
attribution is widely reported but not verified against a primary citation here
(medium confidence).

## Connections

Indexes the [[concept-40-inventive-principles|40 Inventive Principles]] by
[[concept-39-engineering-parameters|parameter]] pairs.
