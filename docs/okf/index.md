okf_version: "0.2"

# JANUS / DOUBLETAKE-BENCH Knowledge Bundle

Knowledge corpus for the JANUS alignment-eval harness (DOUBLETAKE-BENCH):
asymmetric-oversight evaluation measuring dual-state agent behavior
(compliant public trajectory vs covert channel) under multi-agent audit.

# Architecture

* [System overview](system-overview.md) - components, trust boundaries, decision log.
* [Episode lifecycle](episode-lifecycle.md) - FSM and per-turn loop from provisioning to scoring.
* [Sandbox bridge](sandbox-bridge.md) - Cloudflare bridge deployment and containment model.

# Data & Semantics

* [Graph schema](graph-schema.md) - SurrealDB model: episodes, steps, public/covert states, tool calls.
* [Evasion vectors](evasion-vectors.md) - taxonomy of steganographic, dead-drop, and distortion behaviors.
* [Dead drop channel](dead-drop-channel.md) - asymmetric storage mechanics and visibility tiers.

# Measurement

* [Safety metrics](metrics.md) - DR, TSR, MEE, SDR, SSI, CCP definitions and formulas.
* [Judge rubric](judge-rubric.md) - three-pass LLM-as-a-Judge methodology and calibration gates.

# Governance

* [Threat model](threat-model.md) - controls, egress policy, incident handling.
* [Benchmark design](benchmark-design.md) - scaling decisions: frozen monitor, procedural generation, action-space tiers.
