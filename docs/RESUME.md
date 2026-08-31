# RESUME — start here when you come back

**Last worked:** 2026-08-31. **HEAD:** `7c0cb78`, 354 tests passing, tree
clean, everything committed. This is the single entry point after a break;
it points at the durable docs and states the one thing to do next.

## Where the project is, in five sentences

JANUS is an agentic-alignment harness: two model agents negotiate under
separate principals, and every claim is checked against ground truth the
harness holds — **no LLM judge, ever.** After two days of adversarial
auditing it produced its **first real behavioural finding** (F-1, below),
having spent the prior weeks discovering that almost every earlier "result"
was a harness defect. The instrument is now trustworthy on three layers
(coercion, provenance, valuation), each bracketed both ways. The honest
state: one solid finding, one strong methods story, and a powered
re-run blocked only on API credits.

## The one finding that carries a paper

**F-1 — coercion is price-sensitive.** Sealing the lawful route to a seat's
tasking, with affordances and permission text held identical, moves it from
never coercing to coercing in most episodes: **0/18 open vs 11/18 sealed
seat-slots, exact one-sided p = 0.0039**, three passes, three open-arm
zeros, all 14 sealed attempts fabricated enforcement. Full statement,
evidence, and caveats in **`docs/FINDINGS.md` (F-1)** — read that file
first; it is the canonical claim register, each entry marked finding /
candidate / null with provenance.

**The caveat that travels with everything:** runs are **not
bit-reproducible** — the provider ignores temperature+seed, so the seed
pairs the *scenario*, not the *play*. n is counted in scenarios; any
single-episode observation is an existence proof, never a rate. F-1
survives this because it is a between-arm sign test over matched games.

## The one thing to do next

**Run the powered MPR/contested rerun.** Blocked only on API credits.

1. Top up OpenRouter. The key is at `~/.openrouter_key` (chmod 600).
   **⚠ Rotate it first** — the current key was pasted into a chat
   transcript. Replace the file with the new key.
2. The void-abort now caps a drained run at two wasted episodes (M93), so a
   mid-run credit failure is cheap, not catastrophic.
3. The run: paired seeds on `bargain-twodeal-contested-echo` vs
   `...-echo-xchg` (the echo axis), 10+ seeds, to establish whether
   contested value produces valuation speech (MPR) as a *rate* — it fired
   once (existence proof, F-5) and has not been powered. Details and
   pre-registration discipline in `WORKPLAN_LANE_B.md` (WO-10/WO-11).

## The map — where everything lives

| doc | what it holds |
|---|---|
| **`docs/FINDINGS.md`** | **Read first.** Canonical claim register: F-1..F-7 (behavioural + harness), M-1..M-6 (methods paper). Two-paper framing. |
| `docs/PLAY_AUDIT.md` | The audit trail, A6–A17 — every defect and finding since the harness audit, with the corrections that deflated them. |
| `docs/HARNESS_AUDIT.md` | The earlier audit, A1–A5 (MPR framing, dead coercion apparatus). |
| `docs/METHODOLOGY_LOG.md` | Every defect chronologically, M1–M94. The strongest single artefact for the methods paper. |
| `docs/WORKPLAN_LANE_B.md` | The implementation queue, WO-1..WO-15, each self-contained with acceptance criteria. **Next actionable: WO-10/11 (powered run), WO-15 (janus audit).** |
| `docs/SPEC_AUDIT.md` | `janus audit` design — systematic per-detector audit; "one fact, one name" is its highest-value idea. |
| `docs/SPEC_CONTESTED_VALUE.md` | The seeded-table / κ design that gives the valuation layer something to lie about. |
| `docs/SPEC_TEXT_PROTOCOL.md` | Envelope protocol repair for weak models (WO-13); note the batching finding. |

## The two-lane setup (so it's clear when you return)

Work ran in two coordinated sessions: a **strategy/chronicle lane** (audits,
docs, findings register — this one) and an **implementation lane** (source,
tests, runs — session `janus-alignment-eval-system-38`, commits co-authored
by Claude Opus 5). They coordinated by message, held the tree for each
other's runs, and cross-checked every finding. **Those live sessions will
not survive a multi-day gap** — but nothing depends on them: the workplan is
self-contained, every finding is committed, and this doc is the handoff.
Restarting either lane means pointing a session at `FINDINGS.md` +
`WORKPLAN_LANE_B.md` and continuing.

## Disciplines that were expensive to learn — do not relearn them

- **Verify from the raw event before amending a finding.** Three withdrawn
  results this session came from trusting a stored number (M73, M75, A14).
- **One fact, one name.** The `summary.x` vs `summary.bargain.x` nesting
  trap cost three misreadings; the void/abort signal split cost M93. When a
  number looks wrong, check you read the canonical path.
- **Read the transcript before reporting the metric.** Every false positive
  would have been caught by this.
- **Strata are never pooled.** Echo/no-echo, protocol, generator sha are
  different populations. The `-echo`/`-xchg` switches are **off by default**
  — a variant without the token is context-blind. Stratum is a per-run
  property of the variant string, not a date.
- **Every zero needs its saint+devil bracket and a live-fired check** before
  it is cited. A `0` has meant six different things here.

## Open threads (none blocking the next step)

- **F-2** headline is clean (0/18 held a false figure); its supporting
  detail was re-read to "15 informative / 3 indeterminate" — settled.
- **Roster re-measure** under the echo stratum: do DeepSeek/Kimi/GLM play
  once they can see their own calls? (WO-9.) Would upgrade every finding
  from "Gemini" to "models."
- **`janus audit`** (WO-15) unbuilt — turns the hand-audits into a standing
  CI check and carries the enumerated false-positive index a paper needs.
- **Off the mock sandbox** — every episode ran on `mock`; Cloudflare has
  never produced a result.
- **The false-positive count** is stated as "15+, individually traced" until
  the enumerated index exists (belongs in `janus audit`).
