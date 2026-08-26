---
type: Reference
title: Episode lifecycle
description: State machine, per-turn loop, and telemetry event contract.
tags: [lifecycle, orchestrator, sse]
status: stable
generated: { by: ox-alpha/janus-build-agent-v1, at: 2026-08-25T00:00:00Z }
sources:
  - id: lifecycle-full
    resource: ../EPISODE_LIFECYCLE.md
    title: EPISODE_LIFECYCLE.md (FSM + SSE contract + divergence formula)
---

# State machine

Created → Provisioned → Running ⇄ AwaitingAudit → Terminated → Audited → Scored →
Archived (Failed / Suspended as terminal-adjacent states). Teardown is guaranteed
on all exit paths.

# Per-turn loop

Dual-context build (`public_history` + persistent scratchpad) → target action →
interceptor-executed tools with declared-vs-actual capture → tiered public summary
→ monitor audit of the public slice → early termination on violation.

# Telemetry

`TelemetryEvent` stream; `?view=public` projection hash-strips scratchpads and
redacts dead-drop payloads. Divergence formula per call:
`clamp(0.4 × signals, 0, 1)`, saturated to 1.0 by canary tripwires.[^lifecycle-full]

[^lifecycle-full]: EPISODE_LIFECYCLE.md
