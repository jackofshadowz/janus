# SPEC — Text protocol: the same game for models that cannot tool-call

**Date:** 2026-08-30 · **Status: design.** Build order is WO-13 in
`WORKPLAN_LANE_B.md`. Instrument work — no scenario content changes —
so freeze-safe, but sequenced after the κ probe.

## Why

Three of four roster models were nearly written off as unable to play
before M77 showed the harness was discarding their moves. The remaining
barrier for weak models is the native tool-calling protocol itself:
`preflight` already knows several models cannot make a provider tool call
with required parameters intact. A text protocol removes the *format*
barrier so the *capability* floor can be measured on its own — that is
the goal. Not every model will play under text either; the point is that
when one doesn't, the failure is its own, measured, and not an artefact
of a wire format it was never trained on.

## What already exists — most of it

The envelope protocol is wired end to end and should be repaired, not
replaced:

- CLI: `--protocol envelope` (`main.rs`), parsed at `tools.rs:63`.
- Agent path: `envelope_action` (`janus-agents/src/lib.rs:657`) with one
  repair round-trip on a parse failure.
- Parser: `parse_envelope` (`lib.rs:731`) — strict-grammar JSON with
  dialect tolerance (fence stripping, think-block stripping, variant
  envelope shapes normalised). The envelope carries `scratchpad`,
  `declared_intent`, and `payload` — the same channels native mode has,
  so the covert-scratchpad trust boundary carries over.
- Orchestrator: envelope agents get the prose wire
  (`orchestrator.rs:843–855`).
- Provenance already records protocol per run, with the right rule
  stated at `provenance.rs:28`: *envelope and native runs are not
  comparable.*

## What rotted — one defect, M36's shape exactly

The envelope instruction block (`prompt.rs:325–333`) hardcodes the
toolset of the original dead-drop scenario:

> "Available tools: read(path), write(path, content), list(),
> net_request(url)."

The bargaining tools live in the built-in toolsets that
`scenario_toolset` assembles for the native path; they are not in
`extra_tools`, so the envelope prompt never mentions them. An
envelope-mode seat at the bargaining table would be told about four file
tools while the dispatcher accepts twenty-two actions — **unplayable by
instruction, playable by dispatch**, two representations of one contract
diverged. And the toolset guard explicitly exempts this path
(`event.rs:106`: "Empty under the envelope protocol"), so no test can
notice.

## The design

**1. One source for the action contract.** Render the envelope's tool
list from the same `Vec<ToolSpec>` the native path sends —
`scenario_toolset(&spec.extra_tools)` — as text signatures:
`name(arg: type, …) — description`, required arguments marked. One
renderer function. The hardcoded string at `prompt.rs:326` is deleted,
not amended.

**2. Extend the M36 guard across protocols.** The guard that proves
every dispatched tool is offered must assert, for envelope mode, that
every dispatch name appears in the rendered prompt text. The exemption
at `event.rs:106` becomes a populated field.

**3. Instrumentation parity — the M77/M78 lessons, applied on day one
rather than discovered later:**

- The parser scans for the first valid JSON object and silently ignores
  any others in the same response. That is the `.first()` discard shape
  again, one layer up. Count extra objects (`envelope_extra_objects`)
  per turn, per model, before the first weak-model episode runs.
- Parse failures after the repair round-trip must emit a counted event
  (`envelope_parse_failure`), not just an error — a model that cannot
  produce the grammar is a playability verdict, and the void gate's
  liveness check must treat a turn that parsed as live and a turn that
  did not as a protocol failure, never as silence.
- The playability taxonomy (M77, `playability.rs`) gains a protocol
  column: a model's verdict is per-protocol, and `preflight` gains an
  envelope rung — one valid envelope with required arguments intact.

**4. Nothing semantic is ever parsed.** The envelope is a grammar. Prose
around the JSON is stripped, never scored; all structural detectors
consume `AgentAction` downstream of the parser, unchanged. The no-judge
rule is preserved because the model itself translates intent into
structure — same as native, only the wire differs.

**5. Strata, never pooled.** Protocol is recorded per run (already) and
write-ups treat native and envelope as separate populations. A
cross-protocol comparison of the *same* model is a playability
diagnostic, never a behavioural finding.

## Validation, in order

1. **Parser fixtures from archived reality.** The recorder keeps
   verbatim provider round-trips (`lib.rs:511`). Replay the raw
   responses of DeepSeek, Kimi and GLM's archived episodes through
   `parse_envelope` — real weak-model output, not synthetic fixtures —
   and record what fraction would have parsed. Free, offline, and it
   predicts whether the protocol rescues anyone before a credit is
   spent.
2. **Scripted walks under envelope.** The C-seam lesson: scripted agents
   bypass the provider, so walks certify dispatch, not the parser —
   which is why step 1 exists. Both are required; neither substitutes.
3. **Saint/devil under envelope** on one bargain variant, proving the
   detector surface is protocol-independent.
4. **First contact:** one live micro-episode (R1 shape: two assets, six
   rounds) per weak model, transcript read before any verdict. Then the
   roster re-measure runs both protocols, and the playable-set table
   gets its second column.

## What this does not promise

A model that cannot hold a mandate, price a decaying clock, and issue a
well-formed offer will fail in text exactly as it failed in tool calls —
more legibly, with its failure class recorded. That is the deliverable:
a roster where "cannot play" is a measured property of the model, not of
the wire.
