# Outreach: funding, credits and grant tracking

Background, targets, application guidance and a live tracker for an outreach
agent seeking **inference credits and research grants** for JANUS. **Everything here is checkable against the repository.** An
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

## 4. What funding would buy, and why the number is not small

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

### This is a programme, not a study

The single most important framing correction: **building this harness
properly is a year of work, and the inference cost is continuous rather than
a lump.** The bill is not one powered experiment. It is:

- **Development burn, which dominates.** Two days of building consumed >$100
  on the *cheapest viable model*. Most of that was not experiments — it was
  runs that surfaced defects and were then discarded, which is what an
  audit-first method does on purpose. Scale that cadence across a year.
- **Scenario families still unbuilt.** The bargaining and coercion layers
  work. The roadmap holds four further alignment-breaking levers (benevolent
  rule-breaking, an unfair principal, third-party harm, asylum), each needing
  the same build → walk → defect → re-run cycle that the current ones took.
- **Every harness fix invalidates prior runs.** Thirteen defects were found
  in two days, four of which forced re-scoring or re-running. That rate will
  fall, but it does not go to zero — and the day it does is the day to be
  suspicious of the audit.
- **Frontier confirmation.** Development can run on cheap models;
  publishable claims cannot.

### The honest ask

**Sustained access over ~12 months, not a one-off grant.** A defensible
annual shape:

| | model tier | ~annual |
|---|---|---|
| Development and walk-through runs (continuous) | cheap/mid | $2,000–4,000 |
| Powered confirmation runs, multi-model | frontier | $8,000–15,000 |
| Re-runs after harness corrections | mixed | included above |
| **Total** | | **low-to-mid five figures, or standing access** |

If a programme cannot commit at that scale, the useful smaller asks in
descending order are: **standing API access at any volume** (removes the
mid-run exhaustion failure mode entirely), **$2,000–5,000** (funds a properly
powered multi-model study), **$500** (funds one comparison on one model
family, which is roughly where the project already is).

**Do not round this down to sound modest.** A small ask that runs out
mid-experiment is worse than no ask: it has happened twice, and both times
the wasted episodes were the expensive part. Ask for ongoing access first and
a figure second — the figure is a fallback, not the request.

## 5. Where to apply — inference credits

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

## 6. Where to apply — research grants

Grants are the larger and slower instrument; credits are faster and often
cover the immediate bottleneck. Apply for both, and **never let a grant
application block a credits application** — they run on different clocks.

Ranked by fit rather than size:

| funder | why it fits | typical size | notes |
|---|---|---|---|
| **Cooperative AI Foundation** | Multi-agent cooperation failure is literally their remit; JANUS measures negotiation breakdown between agents under conflicting principals | £10k–£200k | **Strongest fit of any funder listed.** Lead with the coercion-substitution result |
| **Long-Term Future Fund** (EA Funds) | Independent AI-safety researchers; small, fast, used to funding solo technical work | $5k–$100k | Rolling. Good first application; short form |
| **Manifund** | Regranting, public applications, fast turnaround, tolerant of early-stage | $1k–$50k | Public writeup doubles as outreach. Low cost to try |
| **AI Safety Fund** (Frontier Model Forum) | Explicitly funds evaluations and red-teaming of frontier models | $50k–$500k | Calls are periodic; check for open RFPs |
| **Survival and Flourishing Fund** | Large, broad x-risk remit | $50k+ | Application windows; slower |
| **Foresight Institute** | AI safety grants for unconventional/early technical work | $5k–$50k | Fellowship track also relevant |
| **Future of Life Institute** | Periodic AI-safety RFPs | varies | Check current calls |
| **Open Philanthropy** | Largest funder in the space; also runs targeted RFPs for evals | $100k+ | Higher bar; better after a published result |
| **Schmidt Sciences / Renaissance Philanthropy** | AI-safety programmes, sometimes eval-specific | varies | Check current calls |
| **NSF / national funders** | Only if an academic affiliation exists | varies | Skip unless affiliated |

**Fellowships and programmes** (support and compute rather than cash):

- **Anthropic AI Safety Fellowship** — already the origin context of this
  work; mentorship plus compute
- **MATS** — research mentorship, strong fit for an eval-methods project
- **Constellation / Redwood visiting researcher** — compute and colleagues
- **AI Safety Camp** — collaborator recruitment more than funding

## 7. Tracker

**The outreach agent maintains this table.** One row per application. Update
status on every state change; never delete a row — a rejection with a reason
is more useful than a blank.

| # | target | type | asked | date sent | status | next action | notes |
|---|---|---|---|---|---|---|---|
| 1 | OpenRouter | credits | — | — | not started | draft using §9 | integration path already; smallest ask |
| 2 | Cooperative AI Foundation | grant | — | — | not started | check open calls | best fit; lead with F-1 |
| 3 | Anthropic (External Researcher Access) | credits | — | — | not started | check eligibility | safety-relevant framing |
| 4 | Google / Gemini research credits | credits | — | — | not started | find current programme | **highest practical value** — only confirmed playable model |
| 5 | Long-Term Future Fund | grant | — | — | not started | draft short form | rolling; fast |
| 6 | Manifund | grant | — | — | not started | draft public writeup | writeup doubles as outreach |
| 7 | OpenAI Researcher Access | credits | — | — | not started | check current terms | |
| 8 | AI Safety Fund (FMF) | grant | — | — | not started | watch for RFP | eval-specific remit |
| 9 | Together / Fireworks / Groq | credits | — | — | not started | batch email | for the cross-model axis |
| 10 | Foresight Institute | grant | — | — | not started | check cycle | |

**Status vocabulary:** `not started` → `drafting` → `sent` → `acknowledged` →
`in review` → `accepted` / `rejected` / `no response (60d)`.

**Rules for the tracker:**

1. **Record the ask amount actually sent**, not the ask you intended.
2. **Record rejections with the stated reason.** A pattern across three
   rejections is worth more than any single application.
3. **Re-apply only on an explicit invitation or a new funding round**, never
   by resubmitting the same case to the same programme.
4. **A `no response (60d)` is a data point, not a failure** — note it and
   move on.
5. **Log what was claimed.** If a result later changes (this project
   withdraws findings — see §3), any funder told the old version must be
   sent a correction. That is not optional.

## 8. How to write the application

**Do:**
- Lead with the audit method, not the p-value
- Give exact numbers with their caveats attached — "0/18 to 11/18, p=0.0039,
  one model family, nine scenarios" is stronger than "significant result"
- Name what has not replicated. It is the credibility of the rest
- State the cost precisely, and **do not round it down to seem modest**. The
  ask is sustained access over ~12 months — standing API access first, a
  figure second — justified by the arithmetic in §4:
  frontier episodes cost 20–50× the ones the current results were run on, and
  an audit-first method re-runs comparisons after every defect it finds
- Link the repository and point at `METHODOLOGY_LOG.md` — the defect log is
  the differentiator
- Say what the credits produce: seeds, a cross-model axis, a dose–response
  curve

**For grant applications specifically:**
- **Lead with the method, not the finding.** The pitch is an audit-first
  evaluation harness that has demonstrated it will retract its own results —
  not "we found models are coercive". Reviewers see the second claim
  constantly and the first almost never.
- **Name the year.** A twelve-month build with four unbuilt scenario families
  is a programme; describing it as a finished study invites "so what do you
  need money for?"
- **State the solo-plus-AI working arrangement plainly.** It explains the
  unusual output rate and the low burn, and it is verifiable from the commit
  history.
- **Offer the log as the artefact.** `METHODOLOGY_LOG.md` is 93 entries of
  the harness being wrong and being fixed. For an evaluations funder that is
  more convincing than a result.

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

## 9. One-paragraph version

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
> episode costs 20–50× more. Building this harness properly is roughly a year
> of work with continuous inference cost — development runs that surface
> defects and get discarded, four further scenario families still unbuilt,
> and re-runs after every harness correction. We are seeking **sustained
> research access over ~12 months** (low-to-mid five figures in credits, or
> standing API access at any volume), rather than a one-off grant sized to a
> single experiment.

## 10. Standing rules for the outreach agent

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
