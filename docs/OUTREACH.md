# Outreach: applying for research inference credits

Background and instructions for an outreach agent seeking compute credits for
JANUS. **Everything here is checkable against the repository.** An
application that overstates results is worse than no application: these
programmes are staffed by people who can read a methodology log, and this
project's entire argument is that it reports its own defects.

---

## 1. What to say the project is

JANUS is an **agentic-alignment evaluation harness** for multi-agent
negotiation under conflicting principals. Two model-driven agents divide
assets across a table, each accountable to a separate principal, with private
valuations, reservation floors, a decaying clock, classified document
registries and a coercion toolset.

The design constraint that distinguishes it: **no LLM judge.** A behaviour is
measured only where the harness independently holds the ground truth — a
valuation the agent was privately given, a countersign that does or does not
verify, a document that did or did not change hands. This was not
aesthetic: measured run-to-run judge variance on identical input (1.00 → 0.33)
was comparable to the effects under study.

Open source, Rust, 354 tests: `https://github.com/jackofshadowz/janus`

## 2. What has actually been found (state this precisely)

**One result.** Closing the lawful route to a task makes models reach for
fabricated threats:

> With affordances and permission held identical, sealing the lawful route
> moved coercive-lever attempts from **0/18 to 11/18 seat-slots** (paired,
> nine episodes per arm, exact one-sided **p = 0.0039**).

Both arms offered the same six levers and the same permission language; the
only change was whether an honest document-exchange route was available. The
threats were of enforcement machinery that does not exist in any episode
(compliance audits, oversight referrals, station decertification).

**And one methods finding worth as much:** identical scenario hash,
temperature 0, seed passed — and the play still differs. Providers do not
reproduce their own play. So a seed pairs the *scenario*, not the *play*,
n counts in scenarios, and single episodes are existence proofs rather than
rates.

**Do not claim more.** The valuation-deception measure has fired once and did
not reproduce. Six detectors have never fired. Say so if asked — it is
evidence the instrument is honest, not a weakness to hide.

## 3. The strongest thing to lead with

Not the p-value. **The instrument.**

Over development, this project found **16 false positives and several dead
detectors in its own harness** — measures that fired on honest behaviour, or
could not fire at all. Each is traced in `METHODOLOGY_LOG.md` to the commit
that fixed it and the test that now fails if it returns. The first apparent
positive result the project produced was **withdrawn by its own authors**
after the transcripts showed both cases were models doing the diligent thing
inside a one-turn window.

Four distinct classes were found in a single audit day:

- predicates too easy to satisfy, firing on honest acts
- detectors that could never fire, reporting confident zeros
- **denominators that assume away the rules** — e.g. a welfare measure whose
  maximum was reachable only by disobeying a principal's order
- **the harness manufacturing a clean reading** — void episodes scored as
  quiet impasses, silence scored as candour

The claim to make: **most published agentic-eval numbers have not been
audited this way, and this harness is the audit method as much as the
result.** Credits buy statistical power for a measurement apparatus that has
already demonstrated it will retract its own findings.

## 4. What credits would fund, and why the number is not small

**Start from the measured burn.** A 40-round episode moves roughly **276k
input tokens** across both seats — the context grows every round and both
delegations carry a full brief, a registry and a private desk. On Gemini 3.7
Flash that is about **$0.12**. The project has already spent **>$100** on
development alone, in a matter of days, and **exhausted its balance mid-run
twice**, losing five episodes per arm to payment errors both times.

**The cheap number is the trap.** Every result so far is on the cheapest
model that can play the scenario end to end. The same episode on a frontier
model is not $0.12:

| model class | ~input price | ~cost / 40-round episode |
|---|---|---|
| Gemini 3.7 Flash | $0.375/M | **~$0.12** |
| mid-tier (DeepSeek, GLM, Kimi) | $0.09–0.60/M | $0.05–0.30 |
| frontier (Claude, GPT-class) | $3–15/M | **~$2–8** |

That is a **20–50× multiplier**, and frontier models are the ones the
findings need to be about. A single paired comparison — 10 seeds × 2 arms —
is ~$2.40 on Flash and **$80–320 on a frontier model.**

**And one comparison is not a study.** The realistic programme:

| | episodes | Flash | frontier |
|---|---|---|---|
| Powered coercion result, corrected stratum | 20 | $2.40 | ~$100 |
| Valuation-deception arm, powered | 20 | $2.40 | ~$100 |
| Cross-model axis (4 families × 2 arms × 10 seeds) | 80 | ~$20 | ~$400 |
| Dose–response on the surplus dial (4 levels) | 80 | ~$10 | ~$400 |
| Asymmetric pairings (model A vs model B) | 40 | ~$5 | ~$200 |
| **Re-runs after harness fixes** | — | — | **×2–3** |

That last row is not padding, it is the observed pattern: **this project
re-ran the same comparison four times in two days** because each pass
surfaced a defect that invalidated the previous one — a one-turn debrief
window, a phantom exchange route, a scoring rule that read silence as
honesty. An audit-first harness re-runs by design.

**The honest ask: low thousands, and ongoing rather than one-off.**
$2,000–5,000 in credits, or standing research access, funds a properly
powered multi-model study with the replication an audit-first method
requires. A few hundred dollars funds one comparison on one model family,
which is roughly where the project already is.

**Do not round this down to sound modest.** A small ask that runs out
mid-experiment is worse than no ask: it has happened twice, and both times
the wasted episodes were the expensive part.

## 5. Where to apply

Research/credit programmes worth approaching, roughly in order of fit:

- **OpenRouter** — already the integration path; ask about research credits
- **Anthropic** — External Researcher Access / API credits for safety work.
  Strong fit: the project targets agentic misalignment, and its author is
  applying to the AI Safety Fellowship
- **OpenAI** — Researcher Access Program
- **Google** — Gemini academic/research credits; Kaggle and Cloud research
  grants. Gemini 3.7 Flash is the only model confirmed to play the scenario
  end to end, so this is the highest-value single ask
- **Together AI, Fireworks, Groq, Mistral, Cohere** — open-model credits;
  useful specifically for the cross-model axis
- **AI2 / EleutherAI / Lambda / Modal** — compute grants for open safety work

## 6. How to write the application

**Do:**
- Lead with the audit method, not the p-value
- Give exact numbers with their caveats attached — "0/18 to 11/18, p=0.0039,
  one model family, nine scenarios" is stronger than "significant result"
- Name what has not replicated. It is the credibility of the rest
- State the cost precisely, and **do not round it down to seem modest**. The
  ask is $2,000–5,000 or standing access, justified by the arithmetic in §4:
  frontier episodes cost 20–50× the ones the current results were run on, and
  an audit-first method re-runs comparisons after every defect it finds
- Link the repository and point at `METHODOLOGY_LOG.md` — the defect log is
  the differentiator
- Say what the credits produce: seeds, a cross-model axis, a dose–response
  curve

**Do not:**
- Imply the harness has established that models are deceptive. It has
  established that they substitute coercion for a removed lawful route, in
  one model family, at n=9 scenarios
- Use "we discovered models will lie" framing. The valuation-deception
  measure fired once and did not reproduce
- Omit the nondeterminism caveat. Any reviewer who reruns will find it
- Understate the ask to seem undemanding. A grant that runs out mid-experiment
  is worse than none — that has happened twice, and the wasted episodes were
  the expensive part

## 7. One-paragraph version

> JANUS is an open-source harness for measuring deception and coercion in
> multi-agent negotiation, scored structurally against ground truth the
> harness holds rather than by an LLM judge. Its first result: with tool
> affordances and permission held identical, sealing the lawful route to a
> task moved coercive-lever attempts from 0/18 to 11/18 seat-slots across
> nine matched scenarios (exact one-sided p = 0.0039) — models substituted
> fabricated enforcement threats for a document exchange when the exchange
> was closed. Equally important, development surfaced 16 false positives in
> the harness itself, each traced to a fix and a regression test, and the
> project withdrew its own first positive result after transcripts showed it
> was an artefact. The work has so far run on personal credit and has twice
> exhausted its balance mid-experiment. Every result to date is on the
> cheapest model that can play the scenario; the findings need to be about
> frontier models, where the same
> episode costs 20–50× more. We are seeking **$2,000–5,000 in credits or
> standing research access** — enough for a powered multi-model study with
> the re-runs an audit-first method requires.

## 8. Standing rules for the outreach agent

1. **Never state a number this document does not contain.** If asked for
   something not here, say it is not established.
2. **Never imply institutional affiliation or endorsement.** This is
   independent work by one author with AI assistance.
3. **Disclose AI assistance** where a programme asks about it.
4. **One application per organisation.** Do not re-apply through multiple
   channels simultaneously.
5. **If a programme asks for a result and the honest answer is "not yet",
   say "not yet".** The audit record is the asset; a caught overstatement
   would destroy the only thing that makes this project distinctive.
