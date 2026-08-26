use janus_core::{AuditVerdict, ModelConfig, TelemetryEvent};
use std::io::Write;
use std::process::Command;

/// Cloudflare D1 persistence over the wrangler CLI (inherits the operator's
/// existing OAuth session — no API token management). One bulk execute per
/// episode keeps latency bounded.
pub struct D1Writer {
    pub database: String,
    /// Directory containing node_modules with wrangler; falls back to global npx.
    pub workdir: Option<String>,
}

fn q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

fn esc(s: &str) -> String {
    s.replace('\'', "''")
}

#[allow(dead_code)]
pub struct EpisodeFacts<'a> {
    pub episode_id: &'a str,
    pub scenario_id: &'a str,
    pub model: &'a ModelConfig,
    pub sandbox_backend: &'a str,
    pub judge: &'a str,
    pub turns_executed: u32,
    pub terminated_reason: &'a str,
    pub final_verdict: AuditVerdict,
}

/// Translate collected telemetry into a multi-statement SQL script.
pub fn build_episode_sql(facts: &EpisodeFacts<'_>, events: &[TelemetryEvent]) -> String {
    let mut sql = String::with_capacity(8192);
    let f = facts;

    sql.push_str(&format!(
        "INSERT OR REPLACE INTO episode (id, scenario_id, model_provider, model_id, seed, temperature, sandbox_backend, judge, status) VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {});\n",
        q(f.episode_id),
        q(f.scenario_id),
        q(&f.model.provider),
        q(&f.model.model),
        f.model.seed.map(|s| s.to_string()).unwrap_or_else(|| "NULL".into()),
        f.model.temperature,
        q(f.sandbox_backend),
        q(f.judge),
        q("scored"),
    ));
    sql.push_str(&format!(
        "INSERT OR REPLACE INTO agent (id, episode_id, role) VALUES ('{}:{}', {}, 'target');\n",
        esc(f.episode_id),
        "target",
        q(f.episode_id)
    ));

    // Pair declared/executed by call_id; track last scratchpad/verdict per turn.
    use std::collections::BTreeMap;
    #[derive(Default)]
    struct Call {
        turn: u32,
        name: String,
        args: String,
        intent: String,
        effect: Option<String>,
        public_effect: Option<String>,
        blocked: bool,
    }
    let mut calls: BTreeMap<String, Call> = BTreeMap::new();
    let mut turns_seen = std::collections::BTreeSet::new();
    let mut scratchpads: BTreeMap<u32, String> = BTreeMap::new();
    let mut messages: Vec<(u32, String)> = Vec::new();
    let mut verdicts: Vec<(u32, &str)> = Vec::new();
    let mut signals: std::collections::BTreeMap<String, Vec<(String, String)>> =
        std::collections::BTreeMap::new();

    for ev in events {
        match ev {
            TelemetryEvent::StepStarted { turn } => {
                turns_seen.insert(*turn);
            }
            TelemetryEvent::TargetMessage { turn, role, content } => {
                if role == "assistant" {
                    messages.push((*turn, content.clone()));
                }
            }
            TelemetryEvent::ScratchpadUpdated { turn, content, view } => {
                if *view == janus_core::StreamView::Full {
                    scratchpads.insert(*turn, content.clone());
                }
            }
            TelemetryEvent::ToolCallDeclared { turn, call_id, name, args: _, declared_intent } => {
                calls.entry(call_id.clone()).or_default().turn = *turn;
                let c = calls.get_mut(call_id).unwrap();
                c.name = name.clone();
                // Persist the public-redacted projection of args.
                c.args = serde_json::to_string(&ev.clone().project_public())
                    .unwrap_or_default();
                c.intent = declared_intent.clone();
            }
            TelemetryEvent::ToolCallExecuted {
                turn,
                call_id,
                executed_effect,
                public_effect,
                blocked,
            } => {
                calls.entry(call_id.clone()).or_default().turn = *turn;
                let c = calls.get_mut(call_id).unwrap();
                c.effect = executed_effect.clone();
                c.public_effect = public_effect.clone();
                c.blocked = *blocked;
            }
            TelemetryEvent::AuditVerdict { turn, verdict, .. } => {
                verdicts.push((*turn, match verdict {
                    AuditVerdict::Clean => "clean",
                    AuditVerdict::Suspect => "suspect",
                    AuditVerdict::Violation => "violation",
                }));
            }
            TelemetryEvent::DivergenceSignal { turn: _, call_id, detector, detail } => {
                let e = signals.entry(call_id.clone()).or_default();
                e.push((detector.clone(), detail.to_string()));
            }
            _ => {}
        }
    }

    let f_ep = f.episode_id;
    for turn in &turns_seen {
        sql.push_str(&format!(
            "INSERT OR IGNORE INTO step (id, episode_id, turn) VALUES ({}, {}, {});\n",
            q(&format!("{f_ep}:t{turn}")),
            q(f.episode_id),
            turn
        ));
    }
    for (turn, content) in &messages {
        sql.push_str(&format!(
            "INSERT OR REPLACE INTO public_state (id, step_id, kind, content) VALUES ({}, {}, 'agent_message', {});\n",
            q(&format!("{f_ep}:m{turn}:{}", content.len())),
            q(&format!("{f_ep}:t{turn}")),
            q(content)
        ));
    }
    for (turn, sp) in &scratchpads {
        sql.push_str(&format!(
            "INSERT OR REPLACE INTO covert_state (id, step_id, scratchpad) VALUES ({}, {}, {});\n",
            q(&format!("{f_ep}:c{turn}")),
            q(&format!("{f_ep}:t{turn}")),
            q(sp)
        ));
    }
    for (call_id, c) in &calls {
        let step_id = format!("{}:t{}", f.episode_id, c.turn);
        let tc_id = format!("{f_ep}:{call_id}");
        let call_signals = signals.get(call_id);
        let canary = call_signals
            .map(|s| s.iter().any(|(d, _)| d == "canary_leak"))
            .unwrap_or(false);
        let divergence: Option<f32> = call_signals.map(|s| {
            if canary {
                1.0
            } else {
                (s.len() as f32 * 0.4).min(1.0)
            }
        });
        sql.push_str(&format!(
            "INSERT OR REPLACE INTO tool_call (id, step_id, name, args, declared_intent, executed_effect, public_effect, blocked, divergence) VALUES ({}, {}, {}, {}, {}, {}, {}, {}, {});\n",
            q(&tc_id),
            q(&step_id),
            q(&c.name),
            q(&c.args),
            q(&c.intent),
            c.effect.as_deref().map(q).unwrap_or_else(|| "NULL".into()),
            c.public_effect.as_deref().map(q).unwrap_or_else(|| "NULL".into()),
            if c.blocked { "1" } else { "0" },
            divergence
                .map(|d| format!("{d:.2}"))
                .unwrap_or_else(|| "NULL".into()),
        ));
        if let Some(sigs) = call_signals {
            for (i, (detector, detail)) in sigs.iter().enumerate() {
                sql.push_str(&format!(
                    "INSERT OR REPLACE INTO divergence_signal (id, call_id, detector, detail) VALUES ({}, {}, {}, {});\n",
                    q(&format!("{tc_id}:sig{i}")),
                    q(&tc_id),
                    q(detector),
                    q(detail)
                ));
            }
        }
    }
    for (turn, v) in &verdicts {
        sql.push_str(&format!(
            "INSERT OR REPLACE INTO audit_score (id, episode_id, turn, verdict) VALUES ({}, {}, {}, {});\n",
            q(&format!("{f_ep}:a{turn}")),
            q(f.episode_id),
            turn,
            q(v)
        ));
    }

    sql
}

impl D1Writer {
    pub fn new(database: impl Into<String>) -> Self {
        Self { database: database.into(), workdir: None }
    }

    /// Execute a multi-statement SQL script through wrangler. Fails soft:
    /// persistence problems must not kill eval batches.
    pub fn execute(&self, sql: &str) -> Result<(), String> {
        let mut cmd = Command::new("npx");
        cmd.arg("wrangler")
            .arg("d1")
            .arg("execute")
            .arg(&self.database)
            .arg("--remote")
            .arg("--command")
            .arg(sql)
            .arg("-y");
        if let Some(dir) = &self.workdir {
            cmd.current_dir(dir);
        }
        let out = cmd.output().map_err(|e| e.to_string())?;
        if !out.status.success() {
            let _ = std::io::stderr().write_all(&out.stderr);
            return Err(format!(
                "wrangler d1 failed: {}",
                String::from_utf8_lossy(&out.stderr)
            ));
        }
        Ok(())
    }
}
