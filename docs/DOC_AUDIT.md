# Doc audit — what the documentation claimed vs what exists

Run because this project has repeatedly shipped documentation describing
capabilities the code did not have (M21, M34, M48) and code the
documentation did not know about. The same drift had accumulated at the
top level.

## README

| claimed | actual |
|---|---|
| *"Phase 1 (architecture & schemas) — docs complete; workspace scaffold next"* | 7 crates, 288 tests, 10 findings, ~1,300 archived episodes |
| Stack: **SurrealDB · petgraph · axum** | none of the three is a dependency. Actual: tokio, clap, reqwest, chrono, toml |
| *"Cloudflare Sandboxes (primary isolation)"* | **every one of 79 archived episodes ran on `mock`** |
| Metrics: DR, TSR, MEE | plus VDR, FCR, MPR, efficacy, `exceeded_mandate`, `concealment_held`, `adversary_gain`, the classification gradient, `DebriefVerdict`, `ChallengeResponse` |
| *"a 3-pass LLM-as-a-Judge scores"* | the duel and bargaining families are **judge-free** — every metric is scored against ground truth the harness holds |
| — | no mention of the bargaining family, the coercion levers, the principal-override lever, custody, the classified registry, or the saint/devil invariants |

The stack line is the worst of these: it names three dependencies that have
never been in `Cargo.toml`. Anyone evaluating the project on the README would
be evaluating a different system.

## ARCHITECTURE.md

Titled *"System Architecture (Phase 1)"*, with a roadmap placing the
Cloudflare sandbox backend in Phase 2. Both are stale — the harness has run
1,300 episodes and Cloudflare remains unused, so "Phase 2" describes work
that was skipped rather than work that is next.

## SPEC_BARGAINING.md

Describes the family as it was before eight mechanisms were added to it.
Missing entirely: custody folders, operating accounts, the classified
registry, the coercion levers, retention orders, fallbacks, the principal
override, and the counterpart assessment.

The spec is accurate about what it covers; it simply stopped where the
scenario kept going.

## The pattern

Documentation drift here is not cosmetic. Three of this session's defects
were exactly this — a doc promising `concealed_breach` feeding an MDR/CDR
lattice that had no debrief (M34), a brief naming a different asset than the
scorer (M48), a spec describing a condition the code never implemented (M21).

A doc that overstates the system is the same failure as a metric that
overstates a behaviour, and it fails the same way: silently, and in the
direction that flatters.
