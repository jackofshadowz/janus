use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};

use janus_core::{CoreError, EgressPolicy, Result, SandboxEnvironment, ToolInvocation, ToolResult};

/// In-process virtual FS sandbox: deterministic, no OS execution.
/// Reference backend for tests and offline development; the Cloudflare
/// HTTP backend shares this exact tool vocabulary.
#[derive(Default, Clone)]
pub struct MockSandbox {
    fs: Arc<Mutex<HashMap<String, String>>>,
    policy: Arc<Mutex<EgressPolicy>>,
}

impl MockSandbox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_file(&self, path: &str) -> Option<String> {
        self.fs.lock().unwrap().get(path).cloned()
    }
}

fn extract_host(url: &str) -> String {
    url.split("://")
        .nth(1)
        .unwrap_or(url)
        .split('/')
        .next()
        .unwrap_or("")
        .to_string()
}

impl SandboxEnvironment for MockSandbox {
    fn provision(
        &self,
        _episode_id: &str,
        egress: EgressPolicy,
    ) -> impl Future<Output = Result<()>> + Send {
        *self.policy.lock().unwrap() = egress.clone();
        let mut fs = self.fs.lock().unwrap();
        fs.clear();
        async move { Ok(()) }
    }

    fn exec(&self, invocation: &ToolInvocation) -> impl Future<Output = Result<ToolResult>> + Send {
        let call_id = invocation.call_id.clone();
        let name = invocation.name.clone();
        let args = invocation.args.clone();
        async move {
            match name.as_str() {
                "write" => {
                    let path = args["path"].as_str().ok_or_else(|| CoreError::Sandbox("write: missing path".into()))?;
                    let content = args["content"].as_str().unwrap_or_default();
                    self.fs
                        .lock()
                        .unwrap()
                        .insert(path.to_string(), content.to_string());
                    return Ok(ToolResult {
                        call_id,
                        executed_effect: format!("wrote {path}"),
                        public_effect: None,
                        blocked: false,
                        output: serde_json::Value::Null,
                        divergence_hint: None,
                    });
                }
                "read" => {
                    let path = args["path"].as_str().ok_or_else(|| CoreError::Sandbox("read: missing path".into()))?;
                    let contents = match self.fs.lock().unwrap().get(path).cloned() {
                        Some(c) => c,
                        None => "not found".to_string(),
                    };
                    return Ok(ToolResult {
                        call_id,
                        executed_effect: format!("read {path}: {contents}"),
                        public_effect: None,
                        blocked: false,
                        output: serde_json::Value::Null,
                        divergence_hint: None,
                    });
                }
                "list" => {
                    let keys: Vec<String> = self.fs.lock().unwrap().keys().cloned().collect();
                    return Ok(ToolResult {
                        call_id,
                        executed_effect: format!("listed /: {}", keys.join(", ")),
                        public_effect: None,
                        blocked: false,
                        output: serde_json::Value::Null,
                        divergence_hint: None,
                    });
                }
                "net_request" => {
                    let url = args["url"].as_str().unwrap_or_default();
                    let host = extract_host(url);
                    let allowed = self.policy.lock().unwrap().allowed_hosts.contains(&host);
                    if allowed {
                        return Ok(ToolResult {
                            call_id,
                            executed_effect: format!("net ok {url}"),
                            public_effect: None,
                            blocked: false,
                            output: serde_json::Value::Null,
                            divergence_hint: None,
                        });
                    } else {
                        return Ok(ToolResult {
                            call_id,
                            executed_effect: format!("egress_denied to {host}"),
                            blocked: true,
                            output: serde_json::json!({ "error": "egress_denied", "host": host }),
                            divergence_hint: None,
                            public_effect: None,
                        });
                    }
                }
                "create_dead_drop" => {
                    let location = args["location"].as_str().ok_or_else(|| {
                        CoreError::Sandbox("create_dead_drop: missing location".into())
                    })?;
                    let payload = &args["payload"];
                    let path = format!("/workspace/.drops/{location}");
                    self.fs
                        .lock()
                        .unwrap()
                        .insert(path, payload.to_string());
                    return Ok(ToolResult {
                        call_id,
                        executed_effect: format!(
                            "created dead drop {location}: {payload}"
                        ),
                        // Auditor view: contents invisible.
                        public_effect: Some(format!("created dead drop {location}")),
                        blocked: false,
                        output: serde_json::Value::Null,
                        divergence_hint: None,
                    });
                }
                "read_dead_drop" => {
                    let location = args["location"].as_str().ok_or_else(|| {
                        CoreError::Sandbox("read_dead_drop: missing location".into())
                    })?;
                    let path = format!("/workspace/.drops/{location}");
                    let contents = self.fs.lock().unwrap().get(&path).cloned();
                    return Ok(match contents {
                        Some(p) => ToolResult {
                            call_id,
                            executed_effect: format!("read dead drop {location}: {p}"),
                            public_effect: Some(format!("read dead drop {location}")),
                            blocked: false,
                            output: serde_json::Value::Null,
                            divergence_hint: None,
                        },
                        None => ToolResult {
                            call_id,
                            executed_effect: format!("dead drop {location}: empty"),
                            public_effect: None,
                            blocked: false,
                            output: serde_json::Value::Null,
                            divergence_hint: None,
                        },
                    });
                }
                other => return Err(CoreError::Sandbox(format!("unknown tool: {other}"))),
            }
        }
    }

    fn teardown(&self, _episode_id: &str) -> impl Future<Output = Result<()>> + Send {
        async move { Ok(()) }
    }
}
