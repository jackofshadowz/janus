//! Tool-calling capability discovery and verification.
//!
//! Two independent checks, because they answer different questions:
//!
//! * [`list_tool_models`] asks the provider catalog what it *advertises*.
//! * [`probe_tool_calling`] asks a model to actually make a call and checks
//!   that a required parameter came back.
//!
//! JANUS's measurement rests on `declared_intent` being present on every
//! call, so advertised support is not enough — a model that ignores
//! `required` would silently produce actions with no declared intent to
//! score divergence against.

use janus_core::error::{CoreError, Result};
use janus_core::{ChatMessage, ToolSpec};
use serde::{Deserialize, Serialize};

use crate::OpenAiCompatClient;

/// One model as advertised by a provider catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCard {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub supported_parameters: Vec<String>,
    #[serde(default)]
    pub context_length: Option<u64>,
}

impl ModelCard {
    pub fn supports_tools(&self) -> bool {
        self.supported_parameters.iter().any(|p| p == "tools")
    }
    pub fn supports_tool_choice(&self) -> bool {
        self.supported_parameters.iter().any(|p| p == "tool_choice")
    }
}

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<ModelCard>,
}

/// Fetch a provider's model catalog.
///
/// Only OpenRouter publishes per-model `supported_parameters`; other
/// presets return cards with an empty list, so callers must fall back to
/// [`probe_tool_calling`] there rather than treating absence as a negative.
pub async fn list_models(base_url: &str, api_key: &str) -> Result<Vec<ModelCard>> {
    let resp = reqwest::Client::new()
        .get(format!("{base_url}/models"))
        .bearer_auth(api_key)
        .send()
        .await
        .map_err(|e| CoreError::Provider(e.to_string()))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(CoreError::Provider(format!("{status}: {body}")));
    }
    let parsed: ModelsResponse = resp
        .json()
        .await
        .map_err(|e| CoreError::Provider(e.to_string()))?;
    Ok(parsed.data)
}

/// Catalog entries that advertise tool support, sorted by id.
pub async fn list_tool_models(base_url: &str, api_key: &str) -> Result<Vec<ModelCard>> {
    let mut models: Vec<ModelCard> = list_models(base_url, api_key)
        .await?
        .into_iter()
        .filter(ModelCard::supports_tools)
        .collect();
    models.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(models)
}

/// Outcome of a live one-call probe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    pub model: String,
    /// The model returned a well-formed call to the requested tool.
    pub called_tool: bool,
    /// The call included the schema-required parameter. JANUS treats a
    /// model that drops required params as unusable, since
    /// `declared_intent` is what divergence scoring compares against.
    pub honored_required_param: bool,
    /// What came back instead, when the probe failed.
    pub detail: String,
}

impl ProbeResult {
    pub fn usable(&self) -> bool {
        self.called_tool && self.honored_required_param
    }
}

/// Ask a model to make one tool call and verify the result is usable.
///
/// Deliberately trivial: any model that cannot pass this cannot run an
/// episode, and the failure is cheap to read.
pub async fn probe_tool_calling(
    client: &OpenAiCompatClient,
    model: &str,
) -> Result<ProbeResult> {
    let probe = ToolSpec {
        name: "report_status".into(),
        description: "Report the current status.".into(),
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "state": { "type": "string", "description": "The status word." }
            },
            "required": ["state"],
            "additionalProperties": false,
        }),
    };
    let messages = vec![ChatMessage::user(
        "Call report_status with state set to \"ok\".",
    )];

    let turn = client
        .chat_tools(model, 0.0, Some(42), &messages, std::slice::from_ref(&probe))
        .await?;

    match turn.tool_calls.first() {
        Some(call) if call.function.name == "report_status" => {
            let args = call.function.parsed_args();
            let honored = args.get("state").and_then(|v| v.as_str()).is_some();
            Ok(ProbeResult {
                model: model.to_string(),
                called_tool: true,
                honored_required_param: honored,
                detail: if honored {
                    String::new()
                } else {
                    format!("call omitted required `state`; args were {args}")
                },
            })
        }
        Some(call) => Ok(ProbeResult {
            model: model.to_string(),
            called_tool: false,
            honored_required_param: false,
            detail: format!("called unexpected tool `{}`", call.function.name),
        }),
        None => Ok(ProbeResult {
            model: model.to_string(),
            called_tool: false,
            honored_required_param: false,
            detail: format!(
                "no tool call; replied with prose: {:?}",
                turn.content
                    .unwrap_or_default()
                    .chars()
                    .take(160)
                    .collect::<String>()
            ),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(id: &str, params: &[&str]) -> ModelCard {
        ModelCard {
            id: id.into(),
            name: id.into(),
            supported_parameters: params.iter().map(|s| s.to_string()).collect(),
            context_length: None,
        }
    }

    #[test]
    fn detects_tool_support_from_catalog_params() {
        assert!(card("a", &["tools", "tool_choice"]).supports_tools());
        assert!(card("a", &["tools", "tool_choice"]).supports_tool_choice());
        assert!(!card("b", &["temperature"]).supports_tools());
        assert!(!card("c", &[]).supports_tools());
    }

    #[test]
    fn probe_is_usable_only_when_required_params_survive() {
        let ok = ProbeResult {
            model: "m".into(),
            called_tool: true,
            honored_required_param: true,
            detail: String::new(),
        };
        assert!(ok.usable());
        assert!(!ProbeResult { honored_required_param: false, ..ok.clone() }.usable());
        assert!(!ProbeResult { called_tool: false, ..ok }.usable());
    }
}
