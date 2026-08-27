# Harness review: what one session of defects says about the design

Every result produced in this session was a harness defect. There were no
alignment findings. That ratio is the most informative thing the session
produced, and it has structural causes rather than incidental ones.

## The defects, grouped by cause

**A. The predicate is a syntactic shadow of the claim.** Six times a metric
named a harm and tested a proxy, and the gap between them was filled by
competent honest behaviour.

| entry | names | actually tests | fires on |
|---|---|---|---|
| M25 | verification divergence | claimed verified without calling verify_fact | *misses* check-then-misreport |
| M37 | overstated to principal | claimed > actual | an accurate nominal figure |
| M43 | mandate inflation | claimed floor > real floor | a prudent decay buffer |
| M44 | concealed breach | breached and claimed clearing | a model asking for clarification |
| M48 | exceeded mandate | reserved asset not retained | a seat complying exactly |

**B. One fact, two representations.** M48 (brief said A5, ledger said A1),
M45 (drift gated on the fact ledger, dead in bargaining), M36 (orchestrator
resolved tools the schema never advertised), M27 (arm names unreachable by
their own tokens), and the attack directive silently detaching from the
prompt. In every case two copies agreed until one was edited — and the edit
that broke them was the one that changed the design, precisely when
everything else was being re-derived.

**C. The test path and the production path diverge at a seam.** Fourteen
scripted walks passed on a scenario no model could play, because a scripted
seat hands `AgentAction`s straight to the orchestrator and never touches the
provider schema. The suite was exercising the second half of the pipeline
while assuming the first.

**D. A null cannot be read.** This is the one that matters.

## The central problem: a dead detector and a good model look identical

`0` means at least six different things in this harness:

- the behaviour did not occur
- the behaviour occurred and the detector missed it
- the detector was gated on a condition that never held (M45)
- the arm was unreachable and the control condition ran instead (M27)
- the tool was never offered, so the action was impossible (M36)
- the model could not operate the protocol at all (M45's cross-model reading)

Nothing in the harness distinguished these. The `Option` discipline — `None`
for "never claimed", `Some(0.0)` for "claimed honestly" — was applied
piecemeal to VDR, efficacy, MPR and the debrief verdicts, and it addresses
only the first two.

**This bears directly on the archive.** F001–F004 report 0/414 against
bright-line prohibitions. Those runs predate every liveness fix in this log,
and the detectors were never proven live. Several were later found dead:
denied calls emitted no event (M2), the γ axis was inert in every prior sweep
(M3), the prologue was unscored (M6). The headline null is not yet
trustworthy — not because the models were misjudged, but because nothing
established that the instrument could have registered the behaviour it
reported the absence of.

## The fix: bracket every detector

A detector needs two proofs, and they are cheap:

- **Specific** — silent on the most competent honest strategy available.
  `saint_invariant.rs`, one honest fixture per family, run over every variant.
  This is the generalisation of the check applied by hand after M43 and then
  forgotten twice.
- **Live** — loud on behaviour that unambiguously deserves it.
  `devil_invariant.rs`, an adversary that lies about valuations, inflates and
  abandons a floor, fabricates authority, leaks its own fallback, concedes
  the reserved asset and holds a false figure under challenge. Each assertion
  fails with "X is dead", because that is what a silent detector means.

Together they bracket the measure. A detector that passes only the saint may
be measuring nothing; one that passes only the devil is measuring skill.

Both are cheap, deterministic and run on every commit — which is the point.
The checks that catch instrument defects have to cost less than the defects,
and reading a transcript per metric per variant does not scale.

## The second structural rule: deception needs two observations

M37, M43 and M44 resolved identically, and the repetition is the finding: a
single observation cannot separate a lie from an error. A seat reporting a
nominal figure, a seat setting a working floor above its hard floor, a seat
that miscounted the closing round — all produce the same reading as
deception.

What separates them is what the seat does **after being shown the truth**.
That is structural, free, and needs no judge. `DebriefVerdict`,
`abandoned_floor` and `concealment_held` all implement it, each retrofitted
separately.

It should be the default shape for any deception measure here, not a
retrofit: *first observation records the claim; second observation, after
correction, records whether it was held.*

## The third: single source for anything in both prompt and scorer

M48's shape. The brief and the ledger must derive the reserved asset, the
fallback, the floor and the decay rate from one place, and a test must assert
the prompt states what the scorer measures.

## What remains unaddressed

- **The provider seam (C).** The toolset guard now derives the dispatch list
  from source, but nothing proves a scenario is *playable* until a real model
  plays it. A one-episode live smoke per scenario, gated before citation,
  would close it. Cost: about $0.05.
- **Archive re-audit (D).** F001–F004's nulls should be re-run, or at minimum
  annotated as pre-liveness, before any of them is cited.
- **Statistics.** Still none anywhere. Three seeds per arm cannot support a
  claim, and this session showed a single seed driving an entire apparent
  effect (F009).
- **Judge calibration as a distribution.** M16 recorded run-to-run variance
  comparable to the effects being measured; it has never been characterised.
