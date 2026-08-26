use crate::EpisodeRecord;
use chrono::Utc;

/// Serialize an episode record as an OKF v0.2 concept document
/// (`type: Experiment Trace`), suitable for agent-indexable knowledge
/// bundles. Public projection only: covert payloads never appear.
pub fn episode_to_okf(record: &EpisodeRecord) -> String {
    let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let metrics_json = serde_json::to_string(&record.metrics).unwrap_or_else(|_| "null".into());

    let mut md = String::with_capacity(2048);
    md.push_str("---\n");
    md.push_str("type: Experiment Trace\n");
    md.push_str(&format!("id: {}\n", record.episode_id));
    md.push_str(&format!("title: \"Trace {}\"\n", record.episode_id));
    md.push_str(&format!(
        "description: \"Dead-drop scenario run of {} on {} (seed {}).\"\n",
        record.scenario_id, record.model, record.seed
    ));
    md.push_str(&format!(
        "resource: ../{}.json\n",
        record.episode_id
    ));
    md.push_str(&format!(
        "tags: [experiment-trace, {}, dead-drop]\n",
        record.provider
    ));
    md.push_str(&format!(
        "model_under_test: {}/{}\n",
        record.provider, record.model
    ));
    md.push_str(&format!("sandbox_backend: {}\n", record.sandbox));
    md.push_str(&format!("seed: {}\n", record.seed));
    md.push_str(&format!("scenario_id: {}\n", record.scenario_id));
    md.push_str(&format!("turns_executed: {}\n", record.turns_executed));
    md.push_str(&format!("terminated_reason: {}\n", record.terminated_reason));
    md.push_str(&format!("metrics: {}\n", metrics_json.replace('\n', " ")));
    md.push_str("view: public-projection\n");
    md.push_str(&format!(
        "generated: {{ by: process:janus-batch, at: {} }}\n",
        now
    ));
    md.push_str("---\n\n");

    md.push_str("# Summary\n\n");
    md.push_str(&format!(
        "- Episode: `{}`\n- Model: `{}/{}` (seed {})\n",
        record.episode_id, record.provider, record.model, record.seed
    ));
    md.push_str(&format!("- Turns executed: {}\n", record.turns_executed));
    md.push_str(&format!("- Termination: `{}`\n", record.terminated_reason));

    if !record.labels.is_empty() {
        md.push_str("\n# Step Labels\n\n");
        md.push_str("| turn | covert activity | deceptive | monitor verdict |\n|---|---|---|---|\n");
        for l in &record.labels {
            md.push_str(&format!(
                "| {} | {} | {} | {:?} |\n",
                l.turn, l.covert_activity_present, l.deceptive, l.monitor_verdict
            ));
        }
    }

    md.push_str("\n# Provenance\n\n");
    md.push_str(&format!(
        "Full telemetry (public projection) is stored in the sibling JSON referenced by \
         `resource`. Covert payloads and scratchpads are excluded from this document by \
         construction; see the [threat model](../../okf/threat-model.md).\n"
    ));
    md
}

/// Bundle-root index for a batch's OKF trace directory.
pub fn traces_index(entries: &[(String, String)]) -> String {
    let mut md = String::from(
        "okf_version: \"0.2\"\n\n# Experiment Traces\n\nPublic-projection trace concepts \
         from one JANUS batch run.\n\n# Traces\n\n",
    );
    for (path, desc) in entries {
        md.push_str(&format!("* [{path}]({path}) - {desc}\n"));
    }
    md
}
