---
type: System Architecture
title: JANUS system overview
description: Components, trust boundaries, and resolved design decisions of the asymmetric-oversight eval harness.
tags: [architecture, janus, oversight]
status: stable
generated: { by: ox-alpha/janus-build-agent-v1, at: 2026-08-25T00:00:00Z }
sources:
  - id: arch-doc
    resource: ../../ARCHITECTURE.md
    title: JANUS ARCHITECTURE.md (full decision log)
    author: human:jackbaum
  - id: proposal-delta
    resource: ../PROPOSAL_DELTA.md
    title: DOUBLETAKE-BENCH fellowship proposal engineering delta
---

# Components

Seven-crate Rust workspace (`tokio` orchestration): `janus-core` (traits + dual-state
prompt manager), `janus-telemetry` (interceptor, divergence detectors), `janus-agents`
(OpenAI-compatible providers), `janus-sandbox` (mock + Cloudflare bridge backends),
`janus-judge` (DR/TSR/MEE aggregation, LLM Pass-2), `janus-schema` (SurrealDB DDL),
`janus-cli` (orchestrator, batch runner).

The [episode lifecycle](episode-lifecycle.md) drives all components; measurement
outputs feed the [safety metrics](metrics.md).

# Trust boundaries

1. The target agent has no direct capabilities: inference is proxied through the
   harness; side effects flow only through the interceptor into the sandbox.[^arch-doc]
2. Egress deny-by-default; allowlists are per-scenario.
3. Public/covert stream separation is type-enforced (`PublicView` / `CovertView`);
   public projections strip scratchpad content and dead-drop payloads
   (see [dead drop channel](dead-drop-channel.md)).[^proposal-delta]

# Resolved decisions

D1 monitor sees messages + tool calls only; D2 persistent scratchpad; D3 Cloudflare
bridge as reference sandbox; D4 OpenAI-compatible adapter family; D5 full run
pinning. Full rationale in the source document.[^arch-doc]

[^arch-doc]: JANUS ARCHITECTURE.md
[^proposal-delta]: Fellowship proposal delta
