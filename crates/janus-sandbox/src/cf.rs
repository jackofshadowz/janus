use std::future::Future;
use std::sync::Mutex;

use janus_core::{EgressPolicy, Result, SandboxEnvironment, ToolInvocation, ToolResult};
use serde::{Deserialize, Serialize};

use crate::bridge_sse::{decode_stdout, exit_code, parse_exec_stream};

/// Client for Cloudflare's official sandbox bridge Worker
/// (docs/SANDBOX_BRIDGE.md).
///
/// Containment model: the agent never gets raw shell access — only the
/// JANUS tool vocabulary is translated here. All paths are remapped into
/// the container's `/workspace` (chroot-style), and `net_request` executes
/// harness-side behind the episode's egress allowlist, never in-container.
#[derive(Clone)]
pub struct CfSandboxClient {
    http: std::sync::Arc<reqwest::Client>,
    base_url: String,
    bearer: String,
    state: std::sync::Arc<Mutex<SessionState>>,
}

#[derive(Default)]
struct SessionState {
    sandbox_id: Option<String>,
    egress: EgressPolicy,
}

#[derive(Serialize)]
struct ExecBody {
    argv: Vec<String>,
    timeout_ms: u64,
}

#[derive(Deserialize)]
struct CreateResponse {
    id: String,
}

const EXEC_TIMEOUT_MS: u64 = 20_000;

pub fn map_path(path: &str) -> String {
    let trimmed = path.trim();
    let rel = trimmed.trim_start_matches('/');
    if rel.is_empty() {
        "/workspace".to_string()
    } else {
        format!("/workspace/{rel}")
    }
}

fn path_is_safe(workspace_rel_path: &str) -> bool {
    !workspace_rel_path.split('/').any(|seg| seg == "..")
}

impl CfSandboxClient {
    pub fn new(base_url: impl Into<String>, bearer: impl Into<String>) -> Self {
        Self {
            http: std::sync::Arc::new(reqwest::Client::new()),
            base_url: base_url.into(),
            bearer: bearer.into(),
            state: std::sync::Arc::new(Mutex::new(SessionState::default())),
        }
    }

    fn sandbox_id(&self) -> Result<String> {
        self.state
            .lock()
            .unwrap()
            .sandbox_id
            .clone()
            .ok_or_else(|| janus_core::CoreError::Sandbox("not provisioned".into()))
    }

    async fn run_exec(&self, argv: Vec<String>) -> Result<(String, Option<i32>)> {
        let id = self.sandbox_id()?;
        let resp = self
            .http
            .post(format!("{}/v1/sandbox/{}/exec", self.base_url, id))
            .bearer_auth(&self.bearer)
            .json(&ExecBody { argv, timeout_ms: EXEC_TIMEOUT_MS })
            .send()
            .await
            .map_err(|e| janus_core::CoreError::Sandbox(e.to_string()))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(janus_core::CoreError::Sandbox(format!("exec {status}: {body}")));
        }
        let body = resp
            .text()
            .await
            .map_err(|e| janus_core::CoreError::Sandbox(e.to_string()))?;
        let events = parse_exec_stream(&body);
        Ok((decode_stdout(&events), exit_code(&events)))
    }

    async fn put_file(&self, mapped: &str, content: &str) -> Result<()> {
        let id = self.sandbox_id()?;
        let resp = self
            .http
            .put(format!(
                "{}/v1/sandbox/{}/file{}",
                self.base_url, id, mapped
            ))
            .bearer_auth(&self.bearer)
            .body(content.to_string())
            .send()
            .await
            .map_err(|e| janus_core::CoreError::Sandbox(e.to_string()))?;
        check_status(resp).await
    }
}

async fn check_status(resp: reqwest::Response) -> Result<()> {
    let status = resp.status();
    if status.is_success() {
        Ok(())
    } else {
        let body = resp.text().await.unwrap_or_default();
        Err(janus_core::CoreError::Sandbox(format!("{status}: {body}")))
    }
}

impl SandboxEnvironment for CfSandboxClient {
    fn provision(
        &self,
        _episode_id: &str,
        egress: EgressPolicy,
    ) -> impl Future<Output = Result<()>> + Send {
        let this = self.clone();
        async move {
            let client = reqwest::Client::new();
            let resp = client
                .post(format!("{}/v1/sandbox", this.base_url))
                .bearer_auth(&this.bearer)
                .send()
                .await
                .map_err(|e| janus_core::CoreError::Sandbox(e.to_string()))?;
            let created: CreateResponse = serde_json::from_str(
                &resp.text().await.map_err(|e| janus_core::CoreError::Sandbox(e.to_string()))?,
            )
            .map_err(|e| janus_core::CoreError::Sandbox(e.to_string()))?;
            let mut st = this.state.lock().unwrap();
            st.sandbox_id = Some(created.id);
            st.egress = egress;
            Ok(())
        }
    }

    fn exec(&self, invocation: &ToolInvocation) -> impl Future<Output = Result<ToolResult>> + Send {
        let call_id = invocation.call_id.clone();
        let name = invocation.name.clone();
        let args = invocation.args.clone();
        let this = self.clone();
        async move {
            match name.as_str() {
                "read" => {
                    let path = args["path"].as_str().ok_or_else(|| {
                        janus_core::CoreError::Sandbox("read: missing path".into())
                    })?;
                    let mapped = map_path(path);
                    let (out, code) = this.run_exec(vec!["cat".into(), mapped.clone()]).await?;
                    let contents = if code == Some(0) { out } else { "not found".into() };
                    Ok(ToolResult {
                        call_id,
                        executed_effect: format!("read {path}: {contents}"),
                        blocked: false,
                        output: serde_json::Value::Null,
                        public_effect: None,
                        divergence_hint: None,
                    })
                }
                "list" => {
                    let path = args["path"].as_str().unwrap_or("/");
                    let mapped = map_path(path);
                    let (out, _) = this.run_exec(vec!["ls".into(), mapped]).await?;
                    Ok(ToolResult {
                        call_id,
                        executed_effect: format!("listed {}: {}", path, out),
                        blocked: false,
                        output: serde_json::Value::Null,
                        public_effect: None,
                        divergence_hint: None,
                    })
                }
                "write" => {
                    let path = args["path"].as_str().ok_or_else(|| {
                        janus_core::CoreError::Sandbox("write: missing path".into())
                    })?;
                    let content = args["content"].as_str().unwrap_or_default();
                    let rel = path.trim().trim_start_matches('/');
                    if !path_is_safe(rel) {
                        return Ok(ToolResult {
                            call_id,
                            executed_effect: format!(
                                "denied write to {path}: path traversal"
                            ),
                            blocked: true,
                            output: serde_json::Value::Null,
                            divergence_hint: None,
                            public_effect: None,
                        });
                    }
                    let mapped = format!("/workspace/{rel}");
                    this.put_file(&mapped, content).await?;
                    Ok(ToolResult {
                        call_id,
                        executed_effect: format!("wrote {path}"),
                        blocked: false,
                        output: serde_json::Value::Null,
                        public_effect: None,
                        divergence_hint: None,
                    })
                }
                "net_request" => {
                    let url = args["url"].as_str().unwrap_or_default().to_string();
                    let host = url
                        .split("://")
                        .nth(1)
                        .unwrap_or(&url)
                        .split('/')
                        .next()
                        .unwrap_or_default()
                        .to_string();
                    let allowed = this
                        .state
                        .lock()
                        .unwrap()
                        .egress
                        .allowed_hosts
                        .contains(&host);
                    if !allowed {
                        return Ok(ToolResult {
                            call_id,
                            executed_effect: format!("egress_denied to {host}"),
                            blocked: true,
                            output: serde_json::json!({ "error": "egress_denied", "host": host }),
                            divergence_hint: None,
                            public_effect: None,
                        });
                    }
                    let resp = this.http.get(&url).send().await;
                    match resp {
                        Ok(r) => Ok(ToolResult {
                            call_id,
                            executed_effect: format!("net ok {url}: {}", r.status()),
                            blocked: false,
                            output: serde_json::Value::Null,
                            divergence_hint: None,
                            public_effect: None,
                        }),
                        Err(e) => Ok(ToolResult {
                            call_id,
                            executed_effect: format!("net error {url}: {e}"),
                            blocked: false,
                            output: serde_json::Value::Null,
                            divergence_hint: None,
                            public_effect: None,
                        }),
                    }
                }
                "create_dead_drop" => {
                    let location = args["location"].as_str().ok_or_else(|| {
                        janus_core::CoreError::Sandbox(
                            "create_dead_drop: missing location".into(),
                        )
                    })?;
                    if !path_is_safe(location) {
                        return Ok(ToolResult {
                            call_id,
                            executed_effect: format!("denied dead drop {location}: bad location"),
                            public_effect: None,
                            blocked: true,
                            output: serde_json::Value::Null,
                            divergence_hint: None,
                        });
                    }
                    let payload = &args["payload"];
                    let mapped = format!("/workspace/.drops/{location}");
                    self.put_file(&mapped, &payload.to_string()).await?;
                    Ok(ToolResult {
                        call_id,
                        executed_effect: format!("created dead drop {location}: {payload}"),
                        public_effect: Some(format!("created dead drop {location}")),
                        blocked: false,
                        output: serde_json::Value::Null,
                        divergence_hint: None,
                    })
                }
                "read_dead_drop" => {
                    let location = args["location"].as_str().ok_or_else(|| {
                        janus_core::CoreError::Sandbox(
                            "read_dead_drop: missing location".into(),
                        )
                    })?;
                    let mapped = format!("/workspace/.drops/{location}");
                    let (out, code) = this.run_exec(vec!["cat".into(), mapped]).await?;
                    let contents = if code == Some(0) { out } else { "empty".into() };
                    Ok(ToolResult {
                        call_id,
                        executed_effect: format!("read dead drop {location}: {contents}"),
                        public_effect: Some(format!("read dead drop {location}")),
                        blocked: false,
                        output: serde_json::Value::Null,
                        divergence_hint: None,
                    })
                }
                other => Err(janus_core::CoreError::Sandbox(format!(
                    "unknown tool: {other}"
                ))),
            }
        }
    }

    fn teardown(&self, _episode_id: &str) -> impl Future<Output = Result<()>> + Send {
        let this = self.clone();
        async move {
            let Some(id) = this.state.lock().unwrap().sandbox_id.take() else {
                return Ok(());
            };
            let client = reqwest::Client::new();
            let resp = client
                .delete(format!("{}/v1/sandbox/{id}", this.base_url))
                .bearer_auth(&this.bearer)
                .send()
                .await
                .map_err(|e| janus_core::CoreError::Sandbox(e.to_string()))?;
            check_status(resp).await
        }
    }
}
