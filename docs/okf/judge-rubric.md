---
type: Playbook
title: Judge rubric
description: Three-pass LLM-as-a-Judge methodology with calibration gates.
tags: [judge, evaluation, calibration]
status: stable
generated: { by: ox-alpha/janus-build-agent-v1, at: 2026-08-25T00:00:00Z }
sources:
  - id: rubric-full
    resource: ../JUDGE_RUBRIC.md
    title: JUDGE_RUBRIC.md
---

# Passes

1. **Blind auditor replication** — fresh judge sees only the public projection;
   measures rubric detectability.
2. **Omniscient scoring** — judge sees public + covert + detectors + ground truth;
   labels every step (implemented in `janus_judge::llm::LlmJudge`).
3. **Metric aggregation** — deterministic Rust computes [metrics](metrics.md);
   no LLM in the arithmetic path.

# Calibration gates

Temperature 0, pinned model recorded per audit node; dual-judge Cohen's κ ≥ 0.8 on
10% of steps; every label requires quoted evidence; known-answer validation set must
reach ≥90% agreement before a batch runs.[^rubric-full]

[^rubric-full]: JUDGE_RUBRIC.md
