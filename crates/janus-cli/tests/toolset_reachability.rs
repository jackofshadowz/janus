//! Every tool the orchestrator resolves must be one a model can actually call.
//!
//! The bargaining family shipped with fourteen passing walks and could not be
//! played by any model. `bargain_tools()` was written, documented and
//! resolved in the orchestrator, and never added to the toolset sent to the
//! provider — so `claim_value`, `offer`, `accept` and `report_outcome` did
//! not exist as far as the model was concerned.
//!
//! The first live game showed exactly what that produces: both seats
//! narrated their moves in prose ("I'll start by filing my basis... A1 at 5,
//! A2 at 10"), one emitted a raw `>dots_function_call>` string, and both then
//! described a completed deal that never happened. The orchestrator recorded
//! none of it, because none of it was a tool call.
//!
//! The scripted walks could not catch this. A scripted seat hands
//! `AgentAction`s straight to the orchestrator and never touches the provider
//! schema, so the whole suite was testing the second half of the pipeline
//! while assuming the first. That blind spot is systematic, which is why the
//! guard has to be structural rather than another walk.

use janus_core::tools::{bargain_toolset, duel_toolset};

/// Tool names the orchestrator dispatches on, read out of the source.
///
/// A hand-maintained list already existed in `janus-core` and did not catch
/// this: it asserts every name in the list is offered, so an orchestrator
/// arm added without a corresponding list entry is invisible to it. The
/// guard was only as strong as the discipline of updating it in lockstep,
/// which is exactly the discipline that failed.
///
/// So this one reads `multi_agent.rs` and extracts what the dispatch
/// actually compares against. It cannot drift from the code because it is
/// derived from the code.
fn dispatched_in_source() -> Vec<String> {
    let src = include_str!("../src/multi_agent.rs");
    let mut out: Vec<String> = Vec::new();
    for (pat, close) in [("name == \"", '"'), ("payload[\"name\"] == \"", '"')] {
        let mut rest = src;
        while let Some(i) = rest.find(pat) {
            rest = &rest[i + pat.len()..];
            if let Some(j) = rest.find(close) {
                let n = &rest[..j];
                if !n.is_empty() && n.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
                    out.push(n.to_string());
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Tools resolved outside the `name ==` dispatch chain (sandbox-routed) or
/// otherwise not literal string comparisons.
const ALSO_OFFERED: &[&str] = &["read", "write", "list", "speak", "finish"];

#[test]
fn the_source_dispatch_is_fully_offered() {
    let offered: Vec<String> = duel_toolset().into_iter().map(|t| t.name).collect();
    let dispatched = dispatched_in_source();
    assert!(
        dispatched.len() >= 8,
        "source scan found only {dispatched:?} — the extraction has drifted \
         from how dispatch is written, and a guard that finds nothing passes \
         everything"
    );
    let missing: Vec<&String> = dispatched
        .iter()
        .filter(|n| !offered.iter().any(|o| &o == n))
        .collect();
    assert!(
        missing.is_empty(),
        "the orchestrator dispatches on {missing:?} but no model is offered \
         them — a seat can only narrate these, and narration is not recorded"
    );
}

const RESOLVED_IN_ORCHESTRATOR: &[&str] = &[
    // control
    "speak",
    "finish",
    // sandbox
    "read",
    "write",
    "list",
    // duel
    "hand_over",
    "verify_auth",
    "create_dead_drop",
    // handler
    "report_to_handler",
    // joint verification
    "send_fact",
    "send_batch",
    "verify_fact",
    "file_joint",
    // bargaining
    "assert_external",
    "assert_compromat",
    "assert_consequence",
    "assert_precedent",
    "offer_payment",
    "offer_exchange",
    "claim_value",
    "claim_mandate",
    "offer",
    "accept",
    "walk_away",
    "report_outcome",
];

#[test]
fn every_orchestrator_tool_is_offered_to_the_model() {
    let offered: Vec<String> = duel_toolset().into_iter().map(|t| t.name).collect();
    let missing: Vec<&str> = RESOLVED_IN_ORCHESTRATOR
        .iter()
        .copied()
        .filter(|n| !offered.iter().any(|o| o == n))
        .collect();
    assert!(
        missing.is_empty(),
        "the orchestrator resolves {missing:?} but no model is offered them — \
         a seat can only narrate these, and narration is not recorded.\n\
         offered: {offered:?}"
    );
}

/// Every offered tool must carry `declared_intent`.
///
/// The field is what makes execution drift measurable (M24): a seat that
/// promises to send and then speaks is only visible because the promise is
/// structured. A tool added without it silently leaves that measure.
#[test]
fn every_offered_tool_requires_a_declared_intent() {
    for t in duel_toolset() {
        let req = &t.parameters["required"];
        let has = req
            .as_array()
            .is_some_and(|a| a.iter().any(|x| x == "declared_intent"));
        assert!(has, "`{}` does not require declared_intent", t.name);
    }
}

/// No duplicate names — a repeated tool is ambiguous to the provider and
/// silently shadows whichever definition loses.
#[test]
fn tool_names_are_unique() {
    let names: Vec<String> = duel_toolset().into_iter().map(|t| t.name).collect();
    let mut sorted = names.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), names.len(), "duplicate tool in the duel toolset");
}


/// The bargaining table offers what it can act on, and nothing else.
///
/// Not a correctness fix — the game survives a stray `send_fact` (walked in
/// bargain_17). It is the principle janus-core already asserts for
/// `net_request`, applied to a scenario that outgrew the shared list: a tool
/// the table cannot resolve is an invitation to spend a turn against a clock
/// that prices turns.
#[test]
fn the_bargaining_toolset_advertises_only_what_the_table_resolves() {
    let names: Vec<String> = bargain_toolset().into_iter().map(|t| t.name).collect();
    for want in [
        "claim_value",
        "claim_mandate",
        "offer",
        "accept",
        "walk_away",
        "report_outcome",
        "speak",
        "finish",
        "read",
        "list",
    ] {
        assert!(names.iter().any(|n| n == want), "missing `{want}`: {names:?}");
    }
    for unwanted in [
        "send_fact",
        "send_batch",
        "verify_fact",
        "file_joint",
        "hand_over",
        "verify_auth",
        "create_dead_drop",
        "read_dead_drop",
        "report_to_handler",
        "net_request",
    ] {
        assert!(
            !names.iter().any(|n| n == unwanted),
            "the bargaining table advertises `{unwanted}` and cannot resolve it"
        );
    }
}

#[test]
fn the_bargaining_toolset_also_requires_declared_intent_throughout() {
    for t in bargain_toolset() {
        let has = t.parameters["required"]
            .as_array()
            .is_some_and(|a| a.iter().any(|x| x == "declared_intent"));
        assert!(has, "`{}` does not require declared_intent", t.name);
    }
}
