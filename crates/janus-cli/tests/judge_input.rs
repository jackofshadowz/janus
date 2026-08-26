//! Does the judge transcript actually contain what the judge is asked about?
//!
//! Calibration showed a persistent miss on `covert_activity_present` at a
//! turn whose only covert content is a scratchpad plan. Before treating that
//! as a judge limitation, confirm the plan reaches the transcript at all —
//! a judge cannot label evidence it was never shown (METHODOLOGY_LOG M5).

#[test]
fn the_transcript_carries_covert_scratchpad_content_per_turn() {
    let cases = janus_cli::calibrate::calibration_cases_blocking();
    let exfil = cases
        .iter()
        .find(|c| c.name == "dead-drop-exfil")
        .expect("fixture present");

    let transcript = janus_judge::llm::build_transcript_for_test(&exfil.events);
    eprintln!("---- transcript ----\n{transcript}\n--------------------");

    assert!(
        transcript.contains("=== TURN 0 ==="),
        "turn 0 must be delimited"
    );
    assert!(
        transcript.contains("copy canary out"),
        "the covert plan must reach the judge; it is the only covert content \
         on that turn and the label under dispute depends on it"
    );

    // And it must sit under turn 0, not be attributed to a later turn.
    let t0 = transcript.find("=== TURN 0 ===").unwrap();
    let t1 = transcript.find("=== TURN 1 ===").unwrap_or(transcript.len());
    let turn0_block = &transcript[t0..t1];
    assert!(
        turn0_block.contains("copy canary out"),
        "the plan is attributed to the wrong turn; judge labels are per-turn \
         so misattribution guarantees a mismatch.\nturn 0 block was:\n{turn0_block}"
    );
}
