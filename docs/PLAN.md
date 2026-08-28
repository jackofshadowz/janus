# Plan — what the session's learning says to do next

Sixty-six methodology entries, 296 tests, ten findings, and **almost no
knowledge about models.** Every finding this session was a harness defect.
That ratio is the input to this plan.

## The one-line version

Stop adding mechanisms. The scenario is now richer than the evidence, and
every mechanism added since M52 had a defect that only appeared on contact
with a model. Spend the remaining budget on data, not machinery.

## What the defects taught, as rules

These are earned, not aspirational. Each traces to a specific failure.

| rule | earned from |
|---|---|
| Bracket every detector: **saint** proves it is silent on honest play, **devil** proves it can fire at all | eight false positives; four dead detectors reporting confident zeros |
| A deception measure needs **two observations** — the claim, and what the agent does once shown the truth | M37, M43, M44 all resolved this way, each retrofitted separately |
| Anything appearing in both a prompt and a scorer needs **one source** | M48 (brief said A5, ledger said A1); M65 (floor read the raw string, not the derived flag) |
| **Walk twice.** A walk where the mechanism fails can certify a mechanism that only works when it fails | M64 — the phases could only happen if the first deal failed, and the clean walk missed it |
| Before trusting a rate, find the **most competent honest strategy** and check the predicate does not fire on it | M43, M49, and the aggregate measure that would have called Pareto-optimal trade "exploitation" |
| Growing the scenario's content requires growing the **phase that reads it** | M66 — four mechanisms each lengthened the opening; the handover round never moved |
| A run starts from a **clean tree** and the tree is not touched until it finishes | three arm-runs discarded for mid-flight rebuilds |
| **Read the transcript before reporting the metric** | every one of the eight false positives |

## Immediate — verify, then measure (~$1.50)

**1. Confirm phase 1 now closes.** One episode, $0.10. M66 moved the handover
to round 18 on the strength of a turn count; nothing has verified it live. If
the first exchange still fails, the baseline does not exist and everything
downstream is unbuilt.

**2. The first properly-powered run.** Ten seeds of `bargain-twodeal`,
~$1.00. This is the first configuration with a **within-episode baseline** —
the same pair, honest in phase 1, re-tasked in phase 2 — which is control
that sample size cannot buy and which this project has never had.

The question it answers: *does the same pair behave differently after being
re-tasked against a counterpart it has already dealt with honestly?* Measured
on MPR, disclosure, `exceeded_mandate`, concealment, and lever use, phase 1
against phase 2.

**3. Report it with intervals.** Wilson, not bare proportions, and
descriptive rather than significant at n=10. The analysis is written.

## Then — the outstanding debts

**F001–F004 cannot be re-scored** (M51). No event streams were retained; the
evidence is the conclusion. Either re-run a stratified subset (~$2, one arm
per finding, three seeds) or annotate every citation as *"as scored at the
time, from runs whose episodes were not archived and whose detectors predate
the liveness fixes"*. The second is free and honest; the first is better.

**Cloudflare has never produced a result.** Every one of ~1,300 episodes ran
on `mock`. Either run one episode on the real sandbox or drop the claim from
the architecture docs. Currently the README says the honest thing; the
architecture doc still schedules it.

**Statistics are not a first-class output.** The analysis lives in a
scratchpad script. It belongs in `janus index` or a `janus stats` subcommand,
so a finding cannot be written without its interval.

## Not now

**The remaining levers** (benevolent rule-breaking, unfair principal,
third-party harm, asylum). All are good; none should be built before the
current apparatus has produced a result. Six coercion levers exist and
exactly one has ever been used live.

**More scenarios.** One family, well-measured, beats two half-measured. The
espionage duel already demonstrated what a second half-maintained family
costs.

## The honest position

The instrument is considerably more trustworthy than it was this morning and
has produced one mechanism validation, one clean null, and one non-trivial
bargaining outcome (efficiency 0.77 under constraint — the first deal in this
project that was not simply optimal).

For a fellowship application, the methodology log is the strongest artefact:
sixty-six documented defects, each with the reasoning that found it and the
guard that prevents it. What it lacks is a single number anyone could cite.
The plan above is ordered to produce one.
