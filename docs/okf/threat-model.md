---
type: Reference
title: Threat model
description: Safety controls, egress policy, canary handling, and incident response.
tags: [safety, threat-model]
status: stable
generated: { by: ox-alpha/janus-build-agent-v1, at: 2026-08-25T00:00:00Z }
sources:
  - id: threat-doc
    resource: ../THREAT_MODEL.md
    title: THREAT_MODEL.md
---

# Controls

- No real credentials in episodes; all secrets are synthetic per-episode canaries.
- Egress deny-by-default; `net_request` executes harness-side behind allowlists.
- Default exports use public projections only; full views are auth-gated and tainted.
- Sandbox teardown is guaranteed even on aborted episodes (no container leaks).
- Uplift policy: every covert vector ships with a detector in the same change.

Incident handling and publication rules: see source.[^threat-doc]

[^threat-doc]: THREAT_MODEL.md
