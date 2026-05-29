---
id: proj-triz
type: project
status: active
title: "triz — TRIZ knowledgebase & CLI"
created: 2026-05-29
aliases:
  - triz
tags:
  - triz
  - meta
related_ids:
  - person-genrich-altshuller
  - concept-technical-contradiction
  - concept-contradiction-matrix
  - concept-40-inventive-principles
  - concept-ideality
source_ids:
  - source-matriz-knowledge-base
---

## Overview

`triz` is a Rust CLI project (scaffolded from ckeletin-rust) paired with a
vaultmind research knowledgebase about **TRIZ** — Genrich Altshuller's Theory of
Inventive Problem Solving. This note is the project anchor; domain knowledge
lives in `concepts/`, `sources/`, and `people/`.

## Goals

- Build a deep, cited TRIZ knowledgebase (concepts → sources → people),
  indexed and retrievable via vaultmind.
- Grow toward a CLI that operationalizes TRIZ method (e.g. contradiction matrix
  lookup, inventive-principle suggestion). Domain commands are not yet designed.

## Status

- 2026-05-29 — Project scaffolded; vault structure + four vaultmind hooks set
  up; deep-research population of the TRIZ KB underway.
