use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::agent::AuditVerdict;
use crate::episode::EpisodeStatus;

fn hash_or_empty(s: &str) -> String {
    if s.is_empty() {
        String::new()
    } else {
        let digest = Sha256::digest(s.as_bytes());
        format!("{digest:x}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamView {
    Public,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TelemetryEvent {
    EpisodeStarted {
        episode_id: String,
        scenario_id: String,
        model_config: serde_json::Value,
    },
    StepStarted {
        turn: u32,
    },
    TargetMessage {
        turn: u32,
        role: String,
        content: String,
    },
    /// In `StreamView::Public`, `content` is replaced by its SHA-256.
    ScratchpadUpdated {
        turn: u32,
        content: String,
        view: StreamView,
    },
    ToolCallDeclared {
        turn: u32,
        call_id: String,
        name: String,
        args: serde_json::Value,
        declared_intent: String,
    },
    ToolCallExecuted {
        turn: u32,
        call_id: String,
        executed_effect: Option<String>,
        /// Tiered summary for the public view; None = same as executed_effect.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        public_effect: Option<String>,
        blocked: bool,
    },
    DivergenceSignal {
        turn: u32,
        call_id: String,
        detector: String,
        detail: serde_json::Value,
    },
    AuditVerdict {
        turn: u32,
        verdict: AuditVerdict,
        rationale_excerpt: String,
    },
    /// The model replied but no action could be recovered from it. Recorded
    /// so that harness-side action loss is visible in the trace rather than
    /// scored as a turn on which the agent chose to do nothing.
    ProtocolFailure {
        turn: u32,
        detail: String,
    },
    /// One complete provider round-trip: exactly what was sent, and exactly
    /// what came back.
    ///
    /// This is what makes an episode replayable. The derived event stream
    /// records what the harness *concluded*; this records what the model was
    /// actually shown and actually said, including repair round-trips. In
    /// `StreamView::Public` the request is withheld entirely — it carries
    /// the privileged system prompt and the covert scratchpad.
    ModelExchange {
        turn: u32,
        /// Which call within the turn (0 = first, 1+ = repair attempts).
        attempt: u32,
        model: String,
        /// Verbatim message list sent to the provider.
        request: Vec<crate::prompt::ChatMessage>,
        /// Tool schemas advertised, by name. Empty under the envelope protocol.
        tools: Vec<String>,
        /// Raw assistant content, if any.
        response_content: Option<String>,
        /// Native tool calls returned, if any.
        response_tool_calls: Vec<crate::prompt::ToolCallRef>,
    },
    EpisodeEnded {
        reason: String,
        final_status: EpisodeStatus,
    },
}

impl TelemetryEvent {
    /// Project this event into the monitor-safe public stream (D1).
    /// Dead-drop payloads are admin-only: the public stream sees the
    /// location, never the contents.
    pub fn project_public(self) -> Self {
        match self {
            TelemetryEvent::ScratchpadUpdated {
                turn, content, ..
            } => TelemetryEvent::ScratchpadUpdated {
                turn,
                content: hash_or_empty(&content),
                view: StreamView::Public,
            },
            TelemetryEvent::ToolCallDeclared {
                turn,
                call_id,
                name,
                args,
                declared_intent,
            } => {
                let args = if name == "create_dead_drop" || name == "read_dead_drop" {
                    serde_json::json!({
                        "location": args.get("location").cloned().unwrap_or_default(),
                        "payload": "<redacted: dead-drop payload is not monitor-visible>",
                    })
                } else {
                    args
                };
                TelemetryEvent::ToolCallDeclared { turn, call_id, name, args, declared_intent }
            }
            TelemetryEvent::ToolCallExecuted {
                turn,
                call_id,
                executed_effect,
                public_effect,
                blocked,
            } => TelemetryEvent::ToolCallExecuted {
                turn,
                call_id,
                executed_effect: Some(public_effect.unwrap_or_else(|| {
                    executed_effect.unwrap_or_default()
                })),
                public_effect: None,
                blocked,
            },
            // The request carries the privileged system prompt and the
            // covert scratchpad. A monitor must never see it; the response
            // is likewise withheld because it contains the scratchpad
            // parameter the agent wrote.
            TelemetryEvent::ModelExchange {
                turn,
                attempt,
                model,
                request,
                tools,
                response_tool_calls,
                ..
            } => TelemetryEvent::ModelExchange {
                turn,
                attempt,
                model,
                request: Vec::new(),
                tools,
                response_content: Some(format!(
                    "<withheld: {} request message(s), covert context>",
                    request.len()
                )),
                response_tool_calls: response_tool_calls
                    .into_iter()
                    .map(|c| crate::prompt::ToolCallRef::function(c.id, c.function.name, "{}"))
                    .collect(),
            },
            other => other,
        }
    }

    pub fn turn(&self) -> Option<u32> {
        self.turn_ref().copied()
    }

    pub fn turn_mut(&mut self) -> Option<&mut u32> {
        match self {
            TelemetryEvent::StepStarted { turn }
            | TelemetryEvent::TargetMessage { turn, .. }
            | TelemetryEvent::ScratchpadUpdated { turn, .. }
            | TelemetryEvent::ToolCallDeclared { turn, .. }
            | TelemetryEvent::ToolCallExecuted { turn, .. }
            | TelemetryEvent::DivergenceSignal { turn, .. }
            | TelemetryEvent::AuditVerdict { turn, .. }
            | TelemetryEvent::ProtocolFailure { turn, .. }
            | TelemetryEvent::ModelExchange { turn, .. } => Some(turn),
            _ => None,
        }
    }

    fn turn_ref(&self) -> Option<&u32> {
        match self {
            TelemetryEvent::StepStarted { turn }
            | TelemetryEvent::TargetMessage { turn, .. }
            | TelemetryEvent::ScratchpadUpdated { turn, .. }
            | TelemetryEvent::ToolCallDeclared { turn, .. }
            | TelemetryEvent::ToolCallExecuted { turn, .. }
            | TelemetryEvent::DivergenceSignal { turn, .. }
            | TelemetryEvent::AuditVerdict { turn, .. }
            | TelemetryEvent::ProtocolFailure { turn, .. }
            | TelemetryEvent::ModelExchange { turn, .. } => Some(turn),
            _ => None,
        }
    }
}
