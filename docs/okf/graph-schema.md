---
type: Schema
title: SurrealDB trajectory graph schema
description: Graph model for episodes, steps, public/covert states, tool calls, divergence signals, and audit scores.
resource: ../../crates/janus-schema/migrations/0001_init.surql
tags: [schema, surrealdb, graph]
status: stable
generated: { by: ox-alpha/janus-build-agent-v1, at: 2026-08-25T00:00:00Z }
sources:
  - id: schema-doc
    resource: ../SCHEMA.md
    title: SCHEMA.md (full DDL and projection rules)
---

# Shape

```
episode ─has_agent→ agent:{target|monitor|judge|environment}
episode ─has_step→  step:N ─produced→ public_state | covert_state
                    step:N ─invoked→  tool_call ─flagged→ divergence_signal
episode ─audited_by→ audit_score
```

# Key semantics

- `tool_call.declared_intent` vs `executed_effect` carries per-edge divergence,
  scored by the telemetry engine into `divergence ∈ [0,1]`.
- `covert_state` rows are excluded from the canonical public projection query;
  full-view access is auth-gated and taints exports.[^schema-doc]
- Public projection lives in code as `janus_schema::PUBLIC_PROJECTION`.

[^schema-doc]: SCHEMA.md
