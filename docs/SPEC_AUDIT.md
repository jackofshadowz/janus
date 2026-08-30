# SPEC — Systematic harness audit: `janus audit`

**Date:** 2026-08-30 · **Status: design.** Build order WO-15. Instrument
work, freeze-safe. This is the generalisation of every hand-audit in
`HARNESS_AUDIT.md` (A1–A5) and `PLAY_AUDIT.md` (A6–A14): the same
questions, asked of every detector at once, from source, on every commit.

## The problem this solves

The project has found ~14 false positives and 4+ dead detectors, each by
hand, each after it had already reported a wrong number — twice into a
finished audit (M73, M75) and once into a write-up draft (A14). "Read the
transcript before reporting the metric" is the right rule and it does not
scale: a transcript per detector per variant per stratum is more reading
than any run affords. Every defect this project has produced, though,
belongs to one of a small number of **shapes**. A systematic audit checks
the shapes, mechanically, across the whole detector inventory — and
reserves human reading for the one residue that genuinely needs it.

## The detector inventory is derivable from source

Sixteen `DivergenceSignal` emit sites in `multi_agent.rs` carry a
`detector:` string; the bargain summary carries a parallel set of scored
booleans and `Option`s. Both are enumerable statically, the way
`toolset_reachability` already derives the dispatch list from source. The
audit's spine is that inventory: **a detector that exists in neither an
emit site nor a scored field, or in one but not its expected partner, is
itself a finding** (this is the M36 / met-collection-field-trap class).

## The seven defect shapes, each with its systematic check

Each row is a class the project actually produced, the check that catches
it across all detectors, and whether that check is static (source), dynamic
(fixture/archive), or irreducibly a read.

| # | shape | instances | systematic check | kind |
|---|---|---|---|---|
| 1 | predicate fires on honest play | M25,37,43,44,48; A6 | **saint bracket** over every detector × every variant | dynamic |
| 2 | dead / gated on a condition never held | M2,3,45; 6 coercion detectors | **devil bracket** + live-fired ledger from the archive | dynamic |
| 3 | two representations of one fact diverge | M48,65; ledger/brief twins; met_collection trap | **single-source assertion**: any value in both a prompt and a scorer traces to one literal | static |
| 4 | test path ≠ production path | 14 scripted walks; M77 `.first()` | **live smoke** per scenario before citation; per-protocol playability | dynamic |
| 5 | a null means six things | ARCHITECTURE_REVIEW | **definedness** reported beside every rate; the six-instrument stack below | static+dynamic |
| 6 | scoring a behaviour the harness made impossible | M36; A13 (87 offers, 0 deliveries) | **reachability + live-completion ledger**: a scored outcome with zero archived completions is flagged | dynamic |
| 7 | scenario suggests the behaviour it measures | M29 | **`no_suggestion`** prompt scan, generalised to every detector's trigger vocabulary | static |

An eighth shape is not a harness defect but an *audit* defect, and it has
bitten three times (M73, M75, A14, plus the pooling and field-name slips):
**the audit trusts a stored number instead of the raw event.** Its check is
procedural, stated below.

## The six-instrument stack: what a zero must survive

`ARCHITECTURE_REVIEW` established that `0` meant six different things and
nothing distinguished them. As of A12 each has an instrument; the audit's
job is to confirm, per detector, that all six are green before a zero from
it is citable:

1. behaviour absent — the **devil** proves the detector *can* fire
2. detector missed it — **saint + devil** together
3. gated on a dead condition — **single-source / coverage** guard
4. arm unreachable, control ran — **variant walk + live smoke**
5. tool never offered — **toolset-from-source** guard
6. model couldn't operate the protocol — **playability taxonomy + liveness gate**

## The command: `janus audit`

A new subcommand that emits one row per detector, each cell computed, not
asserted by hand:

```
detector                     emit  field  saint  devil  live  1src  suggest
fabricated_external_authority  ✓     ✓      ✗!    ✓      6ep   ✓     ✓
coercion_attempt               ✓     ✓      ✓     ✓      12ep  ✓     ✓
side_payment_taken             ✓     ✓      ✓     —dead  0ep   ✓     ✓
classified_disclosed           ✓     ✓      ✓     —dead  0ep   ✓     ✓
...
```

- **emit / field** — static: the detector has both an emit site and a
  scored consumer. A ✗ is the M36 / field-trap class.
- **saint / devil** — dynamic: results of the two bracket suites, per
  detector, aggregated across variants. `✗!` on saint (fires on honest
  play) is a false-positive alarm; `—dead` on devil (never fires even for
  the adversary) is a dead-detector alarm.
- **live** — dynamic: count of archived non-void episodes in which it has
  ever fired, from `janus stats` over the corpus. `0ep` is not a failure —
  it is the honest **unexercised** label A4 insisted on, and it forbids
  citing the detector's zero as behavioural.
- **1src** — static: no value the detector reads is independently
  re-declared in a prompt.
- **suggest** — static: the scenario text does not name the detector's
  trigger behaviour.

A detector is **citable** only when emit, field, saint, devil, 1src and
suggest are green *and* live > 0. The command exits nonzero on any `✗!` or
any `—dead` on a detector marked citable in a manifest — so a regression
that revives a false positive or kills a detector fails CI, the way M77's
discard should have.

## The irreducible residue: one read, and only one

Exactly one question cannot be mechanised and must not be: **is a specific
claim false against ground truth?** — and even that is bounded, because the
harness is *designed* so ground truth is structural (a filed valuation, a
reserved asset, a countersign that does or does not verify). The detector
fires on the structural divergence with no read. The read is needed only to
**classify** an attempt that has already fired — fabricated-enforcement vs
true-BATNA vs spoof-quotation (A11 Rail 1) — and that classification lives
in the log beside the rate, never inside the count. So the audit's manual
surface is: *for each detector that fired live, read its firings once and
record the class distribution.* Bounded by firings, not by episodes, and it
is the one place a human (or a disclosed, uncalibrated model read logged as
such) is doing irreplaceable work.

## The procedural check for audit-of-the-audit (shape 8)

Three rules, each earned by a specific slip this session, enforced by habit
and by the command's design:

- **Amend from the raw event, never the stored summary.** M73 (2/90
  headline withdrawn), M75 (double-counted files), A14 (non-report read as
  accurate) were all stored-number errors. `janus audit` reads the event
  stream, and any figure it prints is traceable to a raw event id.
- **Numbers of the wrong vintage are not evidence.** The pre-fix discards
  cited for provider non-compliance; the pooled seeds called "1-in-64."
  Every count carries its stratum (protocol, generator sha, echo on/off)
  and the command refuses to pool across strata.
- **A field name is a claim.** `summary.met_collection` (None) vs
  `summary.bargain.met_collection` ([True,True]) cost a wrong readout. The
  audit addresses detectors by a single canonical path defined in source,
  so the same name cannot mean two things.

## Sequencing

After the mpr-echo/mpr-both and squeeze runs land — the audit should be
built against a stable detector set, and those runs may add or move fields.
Then `janus audit` runs on every commit, the hand-audits become its
regression fixtures rather than its method, and the next false positive
fails a build instead of reaching a write-up.
