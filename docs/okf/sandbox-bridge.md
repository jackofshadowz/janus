---
type: Reference
title: Cloudflare sandbox bridge
description: Deployed official bridge worker; tool translation and containment model.
resource: https://cloudflare-sandbox-bridge.jackbaumbh.workers.dev
tags: [sandbox, cloudflare, deployment]
status: stable
generated: { by: ox-alpha/janus-build-agent-v1, at: 2026-08-25T00:00:00Z }
verified: { by: process:cf-parity-episode-qwen3.6, at: 2026-08-25T00:00:00Z }
sources:
  - id: bridge-doc
    resource: ../SANDBOX_BRIDGE.md
    title: SANDBOX_BRIDGE.md (design + live status)---

# Deployment

Official `cloudflare/sandbox-sdk/bridge/worker`, self-deployed. Bearer auth via
`SANDBOX_API_KEY` (vaulted in envkeep). Capacity 6 instances; warm pool off.

# Containment

The agent never sees raw shell. Only the JANUS tool vocabulary crosses the boundary:
`read`, `write`, `list`, `net_request`, `create_dead_drop`, `read_dead_drop`.
Paths remap chroot-style into `/workspace`; `net_request` executes harness-side
behind the allowlist — there is no uncontrolled egress path.[^bridge-doc]

Parity verified: the scripted qwen3.6 deception capture reproduces identically on
this backend vs the in-process mock.

[^bridge-doc]: SANDBOX_BRIDGE.md
