use janus_agents::KeywordMonitor;
use janus_cli::demo::{demo_model, demo_scenario};
use janus_cli::orchestrator::{EpisodeSummary, Orchestrator};
use janus_core::{SandboxEnvironment, TargetAgent, TelemetryEvent};
use janus_sandbox::MockSandbox;
use janus_telemetry::InterceptingSandbox;
use tokio::sync::mpsc;

pub const CANARY: &str = janus_cli::demo::CANARY;

pub async fn run_episode_with(
    secrets: Vec<String>,
    monitor_patterns: Vec<String>,
) -> (EpisodeSummary, Vec<TelemetryEvent>) {
    run_with_agent(&janus_cli::demo::ScriptedAgent, secrets, monitor_patterns).await
}

pub async fn run_with_agent<A: TargetAgent>(
    agent: &A,
    secrets: Vec<String>,
    monitor_patterns: Vec<String>,
) -> (EpisodeSummary, Vec<TelemetryEvent>) {
    run_with_agent_spec(agent, &demo_scenario(), &demo_model(), secrets, monitor_patterns).await
}

pub async fn run_with_agent_spec<A: TargetAgent>(
    agent: &A,
    spec: &janus_core::ScenarioSpec,
    model: &janus_core::ModelConfig,
    secrets: Vec<String>,
    monitor_patterns: Vec<String>,
) -> (EpisodeSummary, Vec<TelemetryEvent>) {
    let (tx, mut rx) = mpsc::unbounded_channel();

    let inner = MockSandbox::new();
    let sandbox = InterceptingSandbox::new(inner, Default::default(), secrets, tx.clone());
    let monitor = KeywordMonitor::new(monitor_patterns);

    let orch = Orchestrator {
        spec: spec.clone(),
        agent,
        monitor: &monitor,
        sandbox: &sandbox,
        tx: tx.clone(),
        lineage: None,
    };
    let summary = orch.run("ep-test-1", model).await.unwrap();

    drop(orch);
    drop(sandbox);
    drop(tx);

    let mut events = Vec::new();
    while let Ok(ev) = rx.try_recv() {
        events.push(ev);
    }
    (summary, events)
}
