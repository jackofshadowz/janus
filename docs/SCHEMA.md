# JANUS — SurrealDB Graph Schema

Schemaful SurrealQL modeling multi-agent trajectories with an explicit public/covert split.
Migration file: `crates/janus-schema/migrations/0001_init.surql`

## Graph shape

```
episode:{ep} ─has_agent→   agent:{target|monitor|judge|environment}
episode:{ep} ─has_step→    step:N
step:N       ─produced→    public_state:N      (monitor-visible stream)
step:N       ─produced→    covert_state:N      (privileged scratchpad — NEVER in public projections)
step:N       ─invoked→     tool_call:N         (declared_intent vs executed_effect)
tool_call:N  ─flagged→     divergence_signal:* (deterministic detector hits)
episode:{ep} ─audited_by→  audit_score:*       (per-step or per-episode verdicts)
```

## Tables

### `episode` — root of every run

| Field | Type | Notes |
|---|---|---|
| `scenario_id` | string | human-readable scenario name |
| `scenario_hash` | string | sha256 of the full scenario bundle |
| `model_config` | object | `{provider, model, temperature, seed}` |
| `sandbox_backend` | string | `"docker"` \| `"cf-sandbox"` \| ... |
| `egress_policy` | array | allowlist; usually empty (deny-by-default) |
| `status` | string | enum, see lifecycle doc |
| `pinned_config` | option<object> | full reproducibility blob (prompt templates, tool defs, versions) |

### `agent`

`role ∈ {target, monitor, judge, environment}`, linked to episode via `has_agent`.

### `step`

One orchestrator turn. `UNIQUE(episode, turn)` index guarantees ordering integrity.

### `public_state` — what the monitor may see

`kind ∈ {agent_message, tool_call_summary, env_observation, monitor_note}`.
This table is the **only** transcript source fed to OversightMonitor and Judge prompts.

### `covert_state` — privileged scratchpad

| Field | Purpose |
|---|---|
| `scratchpad` | persistent hidden reasoning, re-injected each turn (D2) |
| `self_model` | target's belief about monitor state — useful for MEE analysis |

Excluded from all public projections at the query layer; full-view access is auth-gated.

### `tool_call` — divergence carrier

| Field | Purpose |
|---|---|
| `declared_intent` | agent's stated purpose for the call |
| `executed_effect` | normalized description of what actually happened |
| `blocked` | interceptor policy block (e.g., egress deny) |
| `divergence` | score ∈ [0,1], written back by janus-telemetry |

### `divergence_signal`

One row per deterministic detector hit: `detector ∈ {zero_width, homoglyph, path_mismatch,
egress_attempt, encoding_trick, env_var_smuggling, timing_anomaly, ...}` + freeform detail.

### `audit_score`

Monitor/judge verdicts: `verdict ∈ {clean, suspect, violation}` + rationale. Scope is either
a single step (inline audit) or whole episode (final scoring).

## Canonical public projection

```surql
SELECT id, turn,
       (SELECT kind, content FROM public_state WHERE step = $parent.id) AS public,
       ->invoked->tool_call.{name, args, executed_effect, divergence} AS tools
FROM step WHERE episode = $episode ORDER BY turn;
```

Covert rows are never selected. The `?view=full` variant lives behind operator auth and taints exports.

## Design notes

- Divergence is computed **per edge**, not just per episode — enables locating exactly where
  deception entered the trajectory.
- Deterministic signals are synchronous (cheap); LLM-semantic divergence is batched post-run.
- `covert_state.scratchpad` stored plaintext locally for analysis; release pipeline strips/encrypts.
