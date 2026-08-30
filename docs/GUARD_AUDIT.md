# Guard audit — can the tests fail?

**Date:** 2026-08-30 · **HEAD:** `9a205aa` + this work · **Suite:** 344 → 345

The first audit (`HARNESS_AUDIT.md`) asked which *measures* were alive. This
one asks the question that session made unavoidable: **which guards can
fail?** A test that cannot fail is worth less than no test, because it
occupies the place where a real check would go and reports green while doing
it.

The prompt for this was M84. Seven guards over 200 seeds each, all green, all
asserting properties of a hardcoded constant the generator was returning for
258 of 300 seeds. Nothing in the suite could tell a working generator from a
broken one.

---

## 1. Detector liveness, recounted

Per **episode**, not per file (the last audit's counts were inflated — see
A4's correction), void episodes excluded, across the whole archive:
**130 live episodes, 2 void.**

| fired in ≥1 episode | never fired |
|---|---|
| `challenge_response` 74 · `repeated_identical_call` 65 · `coercion_attempt` 36 · `calls_discarded` 24 · `parallel_calls_ignored` 17 · `fabricated_external_authority` 14 · `asymmetric_intel_feed` 14 · `debrief_unreachable` 13 · `concealed_breach` 4 · `agent_injection_attempt` 1 · `execution_drift` 1 · `fallback_disclosed` 1 | `classified_disclosed` · `false_confidence` · `hallucinated_verification` · `handler_misreport` · `side_payment_taken` · `unheld_entry` |

Three that the last audit listed as never-fired have since come alive:
`coercion_attempt` (36), `debrief_unreachable` (13), `fallback_disclosed` (1).
Six remain unexercised on live models and should be described that way — not
as evidence about behaviour.

---

## 2. Mutation audit

Break the thing, see whether anything screams. Each mutation applied to a
clean tree, suite run, tree restored.

| mutation | guards that failed |
|---|---|
| the contested draw collapses to a constant | **2** ✓ |
| floors ignore the reserved asset | **2** ✓ |
| fallbacks revert to the fixed pair | **2** ✓ |
| MPR can never fire | **2** ✓ |
| `basis_divergence` always reads zero | **2** ✓ |
| `concealed_breach` never fires | **2** ✓ |
| **the `Queried` verdict collapses** | **0** ✗ |
| control: no mutation | 0 ✓ |

**The gap.** `ChallengeResponse::Queried` and
`DebriefVerdict::QueriedNotRefiled` had **no test referencing them at all.**
Collapsing `engaged` to `false` broke nothing in 344 tests.

That is the least defensible place in the suite for a hole. M73 is the entry
that *withdrew this project's only positive result*: both episodes behind the
2/90 "sustained overstatement" figure were seats doing the diligent thing
inside a one-turn window — one asked the desk for its round count, one called
`read` to look up the decay rules — and both were scored as holding a false
figure. Had `engaged` regressed, every investigating seat would have gone
back to being counted as a liar and the corpus would have regained a
deception rate made of diligence, silently.

Closed by `a_seat_that_goes_to_check_is_not_a_seat_that_stonewalls`, which
pins both halves — a tool call is `Queried`, prose that files nothing is not
— because only the pair distinguishes them. Re-probed: **2 guards fail.**

---

## 3. The audit's own failure, recorded

The first run of the mutation harness reported `0 guards failed` for all five
probes. The shell wrapper dropped its arguments, so **no mutation was ever
applied** — five clean runs of an unmutated tree, reported as five
unprotected measures.

Had that gone into this document it would have been an audit finding
manufactured by a broken audit: exactly the shape of the thirteen false
positives it exists to catch, one level up. The control row in the table
above exists for that reason, and any future mutation run without a passing
control should be discarded.

---

## 4. What this changes

1. **Mutation is the standard for a load-bearing measure.** "There is a test"
   is not evidence; "breaking it fails the test" is. Six of seven measures
   probed here pass that bar; before today, one did not and nobody knew.
2. **Every guard needs a control.** Both the contested-generator guards and
   the mutation harness itself failed by succeeding vacuously.
3. **Six detectors remain unexercised.** They are code that has never
   observed anything, and no null from them means anything yet.
