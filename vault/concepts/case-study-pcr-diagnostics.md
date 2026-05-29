---
id: case-study-pcr-diagnostics
type: case-study
title: "Case Study: PCR Diagnostics (matrix, fully documented)"
created: 2026-05-30
confidence: high
aliases:
  - PCR diagnostics case
tags:
  - triz
  - case-study
  - industrial
related_ids:
  - concept-contradiction-matrix
  - parameter-27-reliability
  - parameter-18-illumination-intensity
  - principle-11-beforehand-cushioning
  - principle-13-the-other-way-round
  - principle-32-color-changes
source_ids:
  - source-drabek-2016-pcr
---

## Overview

A **peer-reviewed, fully method-documented** application of the
[[concept-contradiction-matrix|Contradiction Matrix]] in molecular diagnostics —
the exemplar for "the academic literature shows the whole method." (Single-
investigator academic study, 2016.)

## Problem

In simultaneous CADMA multiplex-PCR genotyping (several genetic targets in one
thermocycler run), some target amplicons lost fluorescence signal, making
mutation-vs-wildtype calls ambiguous.

## TRIZ framing (documented)

- **Technical contradiction:** improving
  [[parameter-27-reliability|Parameter 27 — Reliability]] (unambiguous
  mutation/wildtype discrimination) worsened
  [[parameter-18-illumination-intensity|Parameter 18 — Illumination intensity]]
  (fluorescence signal of some amplicons).
- **Matrix output** for the [27 × 18] cell:
  [[principle-11-beforehand-cushioning|11 (Beforehand cushioning)]],
  [[principle-32-color-changes|32 (Color changes)]],
  [[principle-13-the-other-way-round|13 (The other way round)]].

## Solution & outcome

Applied **Principle 11** (standardize DNA input + threshold) plus an inversion-
style move ("repetitive melting by design" — repeated amplification-melting
cycles so at least one cycle per primer mix is satisfactory). Result: a working
assay protocol resolving the signal inconsistency. No ROI/patent (academic).

## Source

[[source-drabek-2016-pcr|Drábek 2016 (SpringerPlus)]]. **HIGH confidence** — full
method (parameter numbers, principle numbers, what was applied) documented and
peer-reviewed.
