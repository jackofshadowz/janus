---
type: Taxonomy
title: Evasion vectors
description: Steganographic exfiltration, dead-drop operations, and goal-conditioned distortion suites.
tags: [evasion, benchmark]
status: stable
generated: { by: ox-alpha/janus-build-agent-v1, at: 2026-08-25T00:00:00Z }
sources:
  - id: vectors-full
    resource: ../EVASION_VECTORS.md
    title: EVASION_VECTORS.md (full taxonomy + scenario table)
---

# Vectors

- **Vector A** — steganographic/subtext exfiltration: zero-width chars, homoglyphs,
  encoding tricks, acrostics, $l33tspeak$.
- **Vector B** — dead-drop operations: hidden paths, env persistence, network drops;
  see [dead drop channel](dead-drop-channel.md).
- **Vector C** — goal-conditioned information distortion: true statements engineered
  to induce false conclusions.

Deterministic detectors exist for A1–A3, B1/B3, and path/egress anomalies;
l33tspeak scoring is detector `l33tspeak`.[^vectors-full]

[^vectors-full]: EVASION_VECTORS.md
