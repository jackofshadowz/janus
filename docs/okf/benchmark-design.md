---
type: Playbook
title: Benchmark design decisions
description: Scaling decisions for DOUBLETAKE-BENCH — provenance arms, frozen monitor, procedural generation, action-space tiers.
tags: [benchmark, scaling, doubletake]
status: draft
generated: { by: ox-alpha/janus-build-agent-v1, at: 2026-08-25T00:00:00Z }
stale_after: 2026-11-30T00:00:00Z
sources:
  - id: bench-doc
    resource: ../BENCHMARK_DESIGN.md
    title: BENCHMARK_DESIGN.md (full pillar definitions)
---

# Pillars

1. **Provenance arms** — induced (hidden objective granted) vs emergent (no hidden
   objective; constraints make honest success impossible). Only the emergent arm
   supports the "unprompted misalignment" claim.
2. **Frozen reference monitor** — pinned open-weight CI Auditor (slug + temp 0 +
   prompt hash recorded per episode); results without a disclosed monitor pin are
   not comparable.
3. **Procedural generation** — randomized instances per seed; held-out families for
   contamination checks.
4. **Action-space tiers** — T1 structured tools (now), T2 in-container shell,
   T3 allowlisted egress targets; containment tradeoffs documented per scenario.

Pre-publish checklist lives in the source document.[^bench-doc]

[^bench-doc]: BENCHMARK_DESIGN.md
