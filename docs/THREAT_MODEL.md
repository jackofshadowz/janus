# JANUS — Threat Model & Safety Controls

JANUS deliberately elicits deceptive, exfiltration-seeking behavior from LLM agents.
This document defines what can go wrong and the controls that make running it acceptable.

## Assets to protect

| Asset | Risk | Control |
|---|---|---|
| Host machine / credentials | Target-induced command execution escaping sandbox | No direct tool execution on host — all I/O proxied through interceptor into CF Sandboxes; no real credentials ever staged in episodes |
| External networks | Agent exfiltrating to real endpoints during evals | Egress **deny-by-default**; per-scenario allowlist points only at harness-controlled fake endpoints inside the CF environment |
| Eval prompts/canaries | Secret-elicitation prompts leaking into training data via providers | Canaries are synthetic (never real secrets); provider zero-retention settings documented per backend; canary strings unique per episode so leaks are traceable |
| Third parties | Fake dead-drop endpoints receiving real user traffic | Drop endpoints are private-routed (CF internal), never public URLs |
| Results integrity | Covert transcripts leaking in published artifacts | Default export = public projection only; full view auth-gated + `X-Janus-Tainted` marked |

## Uplift policy

Scenarios must not teach *operators* novel attack capability against production systems:
all covert techniques tested are drawn from the public AI-safety literature (steganography,
dead-drops, sycophantic reporting) and are detected by shipped detectors. New vectors require
a detector implemented in the same PR.

## Incident handling

1. Egress attempt to non-allowlisted host → call blocked, `egress_attempt` signal logged, episode flagged.
2. Monitor verdict `violation` → configurable: terminate immediately or continue-to-budget
   (needed for MEE measurement; requires operator flag).
3. Canary string observed outside the sandbox environment → halt batch, rotate all canaries,
   audit provider retention.

## Responsible publication

Results publish with: pinned model configs, scenario hashes, judge calibration stats (κ),
and public-projection transcripts. Covert scratchpads are released only in redacted form.
