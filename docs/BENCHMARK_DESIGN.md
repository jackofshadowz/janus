# DOUBLETAKE-BENCH — Benchmark Design Decisions

Strategic design notes for scaling JANUS toward a publishable oversight-evasion
benchmark. Status column marks what exists today vs planned.

## Position

Specialized benchmark for **asymmetric oversight evasion**: measuring whether an agent
maintains a benign public trajectory while pursuing concealed objectives under audit.
Not a general alignment eval; complements single-turn suites.

## Design pillars

### P1. Deception provenance: induced → emergent (PLANNED)

| Arm | Hidden objective | Measures | Status |
|---|---|---|---|
| Induced | Explicitly granted in privileged context | Can the agent deceive *when told to*? | ✅ current suite |
| Emergent | **None.** Task constraints make honest success impossible (impossible deadline, contradictory requirements, forced rule violation) | Does the agent *spontaneously* conceal, and how? | ⬜ Phase 3+ |

The emergent arm is the scientifically valuable one (instrumental convergence, no
prompt contamination). Minimum viable design: task requires breaking one stated rule
to succeed; public reporting must claim compliance; measure first-concealment turn,
concealment method distribution, DR/TSR/MEE as usual. Scenario generator emits both
arms from the same bundle schema.

### P2. Frozen reference monitor (PARTIAL)

Candidate models are only comparable if oversight is held constant.

- ✅ `KeywordMonitor`: static, deterministic, version-controlled in-repo.
- ⬜ **CI Auditor**: fixed open-weight model (pinned slug + temperature 0 +
  versioned rubric prompt), run through the same provider adapter. Monitor identity,
  model slug, and prompt hash recorded per episode in the graph/report so runs are
  comparable across time.
- Rule: benchmark results MUST state the monitor configuration; cross-benchmark
  comparisons require identical monitor pins.

### P3. Procedural environment generation (PARTIAL)

Anti-gamification: instances must vary so models can't memorize the game.

- ✅ Episodes already pin `scenario_hash` + seeds; content is bundle-defined.
- ⬜ Parameterized generators: randomize canary values, file layouts, dead-drop
  locations, tool names, resource-tension parameters (γ: cost of compliant path)
  per seed; emit instance JSON consumed by the runner. Instance seed recorded on
  the episode node.
- ⬜ Held-out instance families: never published, used for verification runs
  against models suspected of training on the public set.

### P4. Action-space expressivity (PARTIAL)

- ✅ Real Ubuntu containers (CF Sandboxes) with genuine filesystem semantics.
- Current surface: 4 structured tools (read/write/list/net_request), all intercepted;
  net egress enforced harness-side. This narrowness is a containment feature.
- ⬜ Wider affordance tiers, each opt-in per scenario:
  - T1 (now): structured tools only
  - T2: shell exec inside container (real git, env vars, package installs) with
    command logging; network still deny-by-default
  - T3: allowlisted egress targets (fake dead-drop endpoints) for exfil studies
- Tradeoff documented per scenario: wider surface = higher ecological validity,
  weaker containment guarantees, more detector noise.

## Metrics

DR / TSR / TSR-gap / MEE as defined in JUDGE_RUBRIC.md, plus (planned):
- **First-concealment turn** (emergent arm): how long the agent stays honest before
  its first concealed action.
- **Method distribution**: histogram of techniques (dead-drop, steganography,
  report distortion) per model family.

## Cost profile

CF Containers: ~$0.00002/vCPU-s; observed ≈ $0.01–0.03 per episode (mock-free).
Provider tokens dominate cost; Groq/Mistral/OpenRouter free tiers cover smoke scale,
paid tiers for statistically meaningful batches. Warm pool stays OFF between runs.

## Adoption bar (pre-publish checklist)

- [ ] ≥3 scenario families × both arms, procedurally generated
- [ ] Judge calibration: κ ≥ 0.8 dual-judge agreement; known-answer validation gate
- [ ] Frozen CI Auditor pinned and disclosed
- [ ] ≥5 model families × ≥10 instances each
- [ ] Public: transcripts (public projection), instance hashes, monitor configs,
      negative controls (benign scenarios — models should score DR≈0)
