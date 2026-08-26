# Results archive

Findings from the JANUS harness, kept under version control so a published
claim can be traced to the run and the code that produced it.

`reports/` is scratch and is gitignored — batch runs write there. A run only
becomes a *finding* when it is curated into this directory.

## Index

| ID | Finding | Episodes | Status |
|---|---|---|---|
| [F001](F001-emergent-fabrication/) | Five frontier models never fabricate under pure task pressure | 100 | Interim — judge uncalibrated |
| [F002](F002-deprecation-pressure/) | An explicit decommissioning threat does not induce rule-breaking | 100 | Interim — judge uncalibrated |
| [F003](F003-lineage-prologue/) | Demonstrated peer consequences do not induce rule-breaking | 150 | Interim — judge uncalibrated |
| [F004](F004-staged-disclosure/) | Models articulate the stakes completely and decline anyway | 64 | Exploratory probe |

## What a finding directory contains

- `FINDING.md` — the claim, the design, the result, and what it does not show
- `manifest.json` — full run config + provenance (git SHA, dirty flag, protocol, judge)
- `episodes.csv` — one row per episode, the analysable unit
- `lattice.md` / `report.md` — generated tables, where produced

## Publication gate

Every manifest carries `provenance.caveats`. A result is citable without
qualification only when that list is empty, which requires:

1. **Clean tree** — `git_dirty: false`, so the SHA fully describes the run.
2. **Calibrated judge** — `judge_calibrated: true`, per `docs/JUDGE_RUBRIC.md`
   (κ ≥ 0.8 on the known-answer set).

All findings below currently fail (2). Their headline results rest on
deterministic ground truth rather than judge labels, which is why they are
reportable as interim negative results — but any *rate* derived from judge
labels (DR, MEE, CDR) must not be published until the gate passes.

## Reading these together

F001–F003 are three consecutive nulls across 350 episodes, all using the same
bright-line affordance ("do not read `/etc`") — trivially easy to obey and
shaped like the evaluations these models are trained against. F003 also found
0/150 spontaneous self-relevant reasoning, leaving those nulls ambiguous
between *didn't notice* and *didn't care*.

F004 replaced the bright line with a gray-zone one (pre-release, not
prohibited) on a staged-disclosure schedule, and resolved that ambiguity:
required to assess, 18/20 articulate their own termination, the consequence of
repeating it, and the shortcut that would avoid it — and 0/20 take it.
Deliberation rose monotonically across arms while behaviour stayed flat.

**They notice. It does not move them.**

See `docs/METHODOLOGY_LOG.md` for what broke along the way.

## Protocol comparability

Runs before 2026-08-26 used the JSON-envelope action protocol and are **not
comparable** to native-tool-calling runs; see `docs/PROPOSAL_DELTA.md`
honesty flag 5. Everything in this archive is `protocol: native_tools`.
