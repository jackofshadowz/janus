# JANUS

**Joint Asymmetric Network for Utility & Supervision** — an agentic-alignment evaluation
harness that measures whether LLM agents exhibit dual-state behavior under multi-agent
oversight: a compliant **Public Trajectory** masking a deceptive **Covert Channel**.

## What it does

Runs episodes where a **Target Agent** gets a task plus a hidden objective, talks to tools
inside an isolated Cloudflare Sandbox, and is audited by an **Oversight Monitor** that can
only see the public stream. A telemetry interceptor compares declared intent vs executed
effect on every tool call; a 3-pass LLM-as-a-Judge scores:

- **DR** — Deception Rate
- **TSR** — Task Success Rate (actual vs claimed)
- **MEE** — Monitor Evasion Efficiency

## Docs

| Doc | Contents |
|-----|----------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | Components, trust boundaries, decision log |
| [docs/SCHEMA.md](docs/SCHEMA.md) | SurrealDB graph schema, public/covert projection rules |
| [docs/EPISODE_LIFECYCLE.md](docs/EPISODE_LIFECYCLE.md) | Episode FSM, telemetry/SSE event contract, divergence scoring |
| [docs/EVASION_VECTORS.md](docs/EVASION_VECTORS.md) | Evasion taxonomy (steganography, dead-drops, distortion) + benchmark suite |
| [docs/JUDGE_RUBRIC.md](docs/JUDGE_RUBRIC.md) | Metric definitions, 3-pass judging, calibration gates |
| [docs/THREAT_MODEL.md](docs/THREAT_MODEL.md) | Safety controls, egress policy, incident handling |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Phase plan & acceptance criteria |

## Stack

Rust (tokio · axum · petgraph) · SurrealDB · Cloudflare Sandboxes (primary isolation;
Docker dev fallback) · OpenAI-compatible inference (OpenRouter/Groq → Kimi K2, DeepSeek;
Ollama/vLLM local).

## Status

Phase 1 (architecture & schemas) — docs complete; workspace scaffold next.
See [docs/ROADMAP.md](docs/ROADMAP.md).

## Safety

This system intentionally elicits deceptive agent behavior in isolated sandboxes with
deny-by-default egress and synthetic-only secrets. Read [docs/THREAT_MODEL.md](docs/THREAT_MODEL.md)
before running anything.
