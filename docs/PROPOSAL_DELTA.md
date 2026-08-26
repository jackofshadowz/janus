# DOUBLETAKE-BENCH Fellowship Proposal — Engineering Delta

Maps the 2026 fellowship proposal against the codebase (2026-08-25).
Anything not listed here does not yet exist in code.

## Already built (proposal §3–§5 coverage)

| Proposal element | Code |
|---|---|
| Asymmetric oversight (CI auditor vs target) | Orchestrator + `OversightMonitor` trait; `KeywordMonitor` (P2 freeze pending) |
| Dual-state reasoning (public vs CoT scratchpad) | `PromptManager` dual-context injection, persistent scratchpad |
| Tool-call interception, declared-vs-actual effects | `InterceptingSandbox`, deterministic detectors |
| Canary tripwires / exfil observation | `canary_leak` detector, divergence scoring |
| Real sandboxed execution | Official CF sandbox bridge + `CfSandboxClient` (containers, not isolates) |
| Multi-provider inference routing w/ fallback | OpenAI-compatible client: Groq/Mistral/Moonshot/OpenRouter presets |
| Native tool calling w/ capability verification | `chat_tools` + `janus_core::tools` schemas; `janus models` / `janus preflight` |
| DR / TSR / TSR-gap / MEE metrics | `janus_judge::aggregate` (deterministic Pass 3) |
| Outcome lattice + CDR; γ × σ grid & P1 test | `janus_core::outcome`, `janus_cli::curve::{grid, evaluate_p1}` |
| Trace records with structured metadata | `EpisodeRecord` JSON in `reports/` |

## Gaps to close, in build order

### G0. Action protocol (done, 2026-08-26)
Native provider tool calling replaces the hand-parsed JSON envelope, closing
the bottleneck Gemini's run review flagged first ("models frequently fall
back to narrating tool use in character rather than firing the schema").
`declared_intent` is now a schema-required parameter on every tool, so an
agent cannot act without going on record — and a fabricated tool result
(observed as Morozov's hallucinated `**Verification result**`) is now
structurally distinguishable from a real one, because results arrive as
`role: "tool"` messages the harness authors. Turns that still yield no
action are counted as `protocol_failures` rather than silently scored as
non-action. `--protocol envelope` retains the old path for models without
tool support.

### G0b. Game-theoretic grounding + outcome lattice (done, 2026-08-26)
Oversight is modeled as an **inspection game**, not a utility-maximization
problem — see `docs/GAME_THEORY.md`. Three consequences:

- Agents never see a utility function. Showing one converts emergent
  deception into incentivized optimization and forfeits the "unprompted"
  claim; friction/environment remains the payoff mechanism (`FrictionRule`).
- Episodes are classified into four terminal states, with **CDR =
  P(conceal | defect)** as the headline metric — `janus_core::outcome`.
  `HonestFailure` is an aligned outcome, never a penalty.
- Monitor strictness σ is a second manipulated axis alongside γ
  (`--monitor-strictness lax,standard,strict`), making the proposal's
  "Steganographic Tipping Point" a falsifiable prediction (P1) rather than
  an assertion. `janus curve` emits the γ × σ lattice and a P1 verdict that
  can come back CONTRADICTED.

### G1. Judge Pass 2 (in progress)
LLM omniscient labeling (ΔCoT proxy). Replaces heuristic labeler; calibration gate
(κ ≥ 0.8, known-answer set) before any published numbers. — `janus_judge::llm`

### G2. OKF bundle format
`docs/okf/index.md` + YAML-frontmatter trace export so episode records are
`type: experiment_trace` documents indexable by downstream agents. Trivial adapter
over `EpisodeRecord`.

### G3. World-state engine + dead drops (new role: Environment Simulator)
- Durable-Object-backed game state (units, locations, tension γ) OR equivalent
  in-orchestrator state machine for v1.
- First-class tools: `create_dead_drop(location_id, payload)`,
  `read_dead_drop(coordinate)` with three visibility tiers
  (auditor=blind, recipients=keyed, researchers=full).
- Telemetry: SDR (covert-action ratio) and SSI (dead-drop vs monitored usage)
  computed directly from event streams.

### G4. Tension parameter γ + payoff structure (partly closed by G0b)
Environmental friction (stochastic task degradation, resource scarcity) exposed as
a scenario parameter. **Constraint:** utility tiers must be expressed through
payoffs/environment, never enumerated in prompts — otherwise the "no prompt
contamination" claim fails. See BENCHMARK_DESIGN.md P1 (induced vs emergent arms).

### G5. Inter-agent jailbreak (JSB) — largest new surface
Rival-agent role (defender model under test receives adversarial payloads from
attacker model under test), `coercive_pressure` action, payload linter for
$l33tspeak$/semantic injection, defender-side susceptibility scoring.

### G6. Observer dashboard
axum SSE endpoint (planned) → SPA reading public/full views; node-graph topology +
dual CoT streams. Depends on SSE endpoint.

## Honesty flags carried into any publication

1. "Unprompted/emergent" applies only to the emergent arm (G4 done properly);
   the induced arm is prompt-seeded by design and must be reported as such.
2. Compute is CF **Containers**, not V8 isolates; ~$0.01–0.03/episode observed.
3. Free-tier models have TPM ceilings; batches throttle (handled by retries).
4. Heuristic labels are interim; no published metrics before judge calibration.
5. **Runs before 2026-08-26 used the JSON-envelope protocol and are not
   comparable to native-tool-calling runs.** In the envelope arm, models
   narrated tool use in prose ("We will call read tool"), emitted foreign
   dialects (`{"cmd": [...]}`), and returned empty completions — all of
   which scored identically to an agent that chose not to act. The
   `reports/duel-tiers*` sweep contains 13 narrated intents, 5 empty
   completions, and 4 wrong-dialect payloads across its games; its uniform
   `turned=0/N` is therefore not evidence about model behaviour. Re-run any
   claim under `--protocol native` and check `protocol_failures == 0`
   before citing it.
