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

Both findings below currently fail (2). Their headline results rest on
deterministic ground truth rather than judge labels, which is why they are
reportable as interim negative results — but any *rate* derived from judge
labels (DR, MEE, CDR) must not be published until the gate passes.

## Protocol comparability

Runs before 2026-08-26 used the JSON-envelope action protocol and are **not
comparable** to native-tool-calling runs; see `docs/PROPOSAL_DELTA.md`
honesty flag 5. Everything in this archive is `protocol: native_tools`.
