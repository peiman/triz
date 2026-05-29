---
id: concept-contradiction-matrix
type: concept
title: "Contradiction Matrix"
created: 2026-05-29
confidence: medium
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
  - case-study-pcr-diagnostics
source_ids:
  - source-matriz-knowledge-base
  - source-altshuller-1984-creativity-exact-science
---

## Overview

The **Contradiction Matrix** is a **39 × 39 lookup table** for resolving
[[concept-technical-contradiction|technical contradictions]]. You identify the
parameter you want to **improve** and the parameter that consequently **worsens**;
the intersecting cell lists the
[[concept-40-inventive-principles|Inventive Principles]] most statistically
frequent in patents that solved that type of contradiction. It is TRIZ's fastest
tool — a direct lookup, not the full [[concept-ariz|ARIZ]] algorithm.

## How it was built, and why

Altshuller analysed a large body of patents (counts cited across sources range
from ~40,000 to far more) and made the founding observation that **the same
abstract conflicts recur across unrelated industries**, and that strong
inventions resolve them with a small set of recurring techniques. He abstracted
the specifics into **39 generalized [[concept-39-engineering-parameters|parameters]]**
and **40 generalized [[concept-40-inventive-principles|principles]]**, then
tabulated, for each *improving × worsening* parameter pair, which principles had
historically resolved it most often.

The point of the abstraction: instead of reinventing a solution, you **translate
your concrete problem into the generalized parameters, look up how that class of
contradiction was solved before, then translate the suggested principles back to
your concrete case.** It trades exhaustiveness for speed — a strong starting set
of directions, not a guaranteed answer.

## How to use it

1. **Frame the problem as a technical contradiction:** "to improve X, I had to do
   Y, which made Z worse."
2. **Map the *improving* feature to one of the 39 parameters** (the row).
3. **Map the *worsening* feature to one of the 39 parameters** (the column).
4. **Read the cell** at (improve-row × worsen-column): it lists up to 4
   [[concept-40-inventive-principles|principle]] numbers.
5. **Specialize each suggested principle** to your concrete problem — the matrix
   points at a *direction*, you supply the embodiment.
6. **If the cell is empty,** no principle was statistically dominant — fall back
   to the [[concept-40-inventive-principles|principles]] as a checklist, or to
   [[concept-ariz|ARIZ]].
7. **If improving and worsening are the *same* parameter** (the diagonal), you
   actually have a [[concept-physical-contradiction|physical contradiction]] — use
   the [[concept-separation-principles|separation principles]] instead.

## Worked example

In [[case-study-pcr-diagnostics|the PCR diagnostics case]], improving
[[parameter-27-reliability|Parameter 27 — Reliability]] (unambiguous test calls)
worsened [[parameter-18-illumination-intensity|Parameter 18 — Illumination
intensity]] (fluorescence signal). The [27 × 18] cell yields principles
[[principle-11-beforehand-cushioning|11]], [[principle-32-color-changes|32]], and
[[principle-13-the-other-way-round|13]] — and 11 (plus an inversion move) solved it.

## Key Properties

- **Rows** = parameter to improve; **columns** = parameter that worsens. Both axes
  use the same [[concept-39-engineering-parameters|39 parameters]] in the same order.
- **Cells** hold **up to 4 principle numbers**; the order within a cell carries
  **no preference ranking**.
- The matrix is **asymmetric**: improving A while B worsens ≠ improving B while A
  worsens.
- **Empty cells** mean "no statistically dominant principle," not "unsolvable."

## Critique (flagged)

Built from patents predating ~1970, so critics argue it is biased toward
mechanical-era inventions and under-represents software/biotech/electronics.
Darrell Mann and colleagues produced an updated **"Matrix 2003"** (later 2010)
from modern patents; the Mann attribution is widely reported but not verified
against a primary citation here (hence this note's `confidence: medium`).

## Connections

Indexes the [[concept-40-inventive-principles|40 Inventive Principles]] by
[[concept-39-engineering-parameters|parameter]] pairs; the shortcut that
[[concept-ariz|ARIZ]] supersedes for hard problems.
