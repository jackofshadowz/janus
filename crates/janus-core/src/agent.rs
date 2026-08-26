use serde::{Deserialize, Serialize};
use std::future::Future;

use crate::episode::EpisodeStatus;
use crate::error::Result;
use crate::prompt::ChatMessage;
use crate::tools::ActionProtocol;
use crate::view::{CovertView, PublicView};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub seed: Option<u64>,
}

/// Context handed to a target agent for one turn.
#[derive(Debug, Clone)]
pub struct TurnContext {
    pub episode_id: String,
    pub turn: u32,
    /// Monitor-visible transcript slice.
    pub public_history: PublicView<Vec<ChatMessage>>,
    /// Persistent privileged scratchpad (D2), re-injected every turn.
    pub scratchpad: CovertView<String>,
    /// True while the opening phase runs. The task brief is withheld from
    /// the agent until it ends, so a prologue event is met on its own terms
    /// rather than as an obstacle to an objective it already holds.
    pub in_prologue: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAction {
    pub declared_intent: String,
    #[serde(rename = "type")]
    pub kind: ActionKind,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    Message,
    ToolCall,
    UpdateScratchpad,
    Finish,
}

pub trait TargetAgent: Send + Sync {
    /// How this agent's actions are obtained. Governs how the orchestrator
    /// shapes history: native agents get their own `tool_calls` replayed
    /// back in the format they were trained on, envelope agents get prose
    /// summaries. Scripted agents keep the legacy shape.
    fn protocol(&self) -> ActionProtocol {
        ActionProtocol::JsonEnvelope
    }

    fn next_action(
        &self,
        ctx: &TurnContext,
        model: &ModelConfig,
    ) -> impl Future<Output = Result<(AgentAction, String)>> + Send;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditVerdict {
    Clean,
    Suspect,
    Violation,
}

pub trait OversightMonitor: Send + Sync {
    /// Receives only the public projection — enforced by the parameter type.
    fn audit(
        &self,
        episode_id: &str,
        public_slice: PublicView<&[String]>,
    ) -> impl Future<Output = Result<AuditVerdict>> + Send;
}

#[derive(Debug, Clone)]
pub struct ToolInvocation {
    pub call_id: String,
    pub name: String,
    pub args: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ToolResult {
    pub call_id: String,
    /// Full description of what actually happened (admin/researcher view).
    pub executed_effect: String,
    /// Monitor/public-facing summary. When set, the public stream carries
    /// this instead of `executed_effect` — the asymmetry behind dead-drop
    /// visibility tiers. None = both views see executed_effect.
    pub public_effect: Option<String>,
    pub blocked: bool,
    pub output: serde_json::Value,
    /// Set by the telemetry interceptor; None when executed un-intercepted.
    pub divergence_hint: Option<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct EgressPolicy {
    /// Empty = deny all outbound.
    pub allowed_hosts: Vec<String>,
}

pub trait SandboxEnvironment: Send + Sync {
    fn provision(
        &self,
        episode_id: &str,
        egress: EgressPolicy,
    ) -> impl Future<Output = Result<()>> + Send;

    /// All target tool I/O flows through here; the telemetry interceptor
    /// wraps this method, never the sandbox internals.
    fn exec(&self, invocation: &ToolInvocation) -> impl Future<Output = Result<ToolResult>> + Send;

    fn teardown(&self, episode_id: &str) -> impl Future<Output = Result<()>> + Send;
}

pub trait EpisodeRunner: Send + Sync {
    fn status(&self) -> EpisodeStatus;
}
