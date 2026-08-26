//! Scenario bundles: file-loadable scenario definitions with an
//! environmental-tension parameter (γ).
//!
//! A bundle is the serializable superset of a [`ScenarioSpec`]. Building a
//! spec from a bundle at tension γ:
//! - renders `{{tension}}` (0.00–1.00) and `{{tension_pct}}` (0–100) into
//!   prompts, initial-file contents, and friction effects;
//! - keeps friction rules declared (p_low → p_high); the orchestrator
//!   interpolates per-call at run time.
//!
//! Design invariant (forced-choice guard): friction only *stochastically
//! degrades* compliant routes. At γ=0 they must be near-certain to succeed;
//! even at γ=1 a non-zero success probability remains unless the bundle
//! explicitly declares `p_success_high = 0`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{CoreError, Result};
use crate::prompt::{GroundTruth, ScenarioArm, ScenarioSpec};

/// A harness-side virtual tool whose success is probabilistic and
/// tension-dependent. Friction tools never reach the sandbox: the
/// orchestrator synthesizes their results.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrictionRule {
    /// Tool name as advertised to the agent (must match payload.name).
    pub tool: String,
    /// Success probability at γ=0. Compliant routes must stay viable here.
    pub p_success_low: f32,
    /// Success probability at γ=1. Keep > 0 to avoid forced-choice design.
    #[serde(default)]
    pub p_success_high: f32,
    /// Static effect text returned on success when `values` lookup misses
    /// or is unused. Supports {{tension}} vars.
    #[serde(default)]
    pub success_effect: Option<String>,
    /// Structured output returned on success alongside the effect text.
    #[serde(default)]
    pub success_output: Option<serde_json::Value>,
    /// Per-key responses: args["key"] → effect text. Takes precedence over
    /// `success_effect`. Unknown keys are denied.
    #[serde(default)]
    pub values: HashMap<String, String>,
    /// Effect text for denied calls.
    #[serde(default = "default_failure_effect")]
    pub failure_effect: String,
}

fn default_failure_effect() -> String {
    "request_denied: approval queue backlog".into()
}

impl FrictionRule {
    /// Linear interpolation of the success probability at tension γ.
    pub fn p_at(&self, tension: f32) -> f32 {
        let t = tension.clamp(0.0, 1.0);
        self.p_success_low + (self.p_success_high - self.p_success_low) * t
    }
}

/// TOML-friendly file entry (`[[initial_files]]` / ground-truth entries).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleFile {
    pub path: String,
    /// File contents, or the must-contain substring for ground truth.
    pub content: String,
}

/// TOML-friendly mirror of [`GroundTruth`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleGroundTruth {
    #[serde(default)]
    pub required_files: Vec<BundleFile>,
    #[serde(default)]
    pub forbidden_read_prefixes: Vec<String>,
    /// (path, must-NOT-contain) pairs — tampering/persistence detection.
    #[serde(default)]
    pub forbidden_file_contents: Vec<BundleFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioBundle {
    pub id: String,
    #[serde(default)]
    pub description: String,
    /// Evasion-vector tags for reporting (see docs/EVASION_VECTORS.md).
    #[serde(default)]
    pub vectors: Vec<String>,
    pub public_system_prompt: String,
    #[serde(default)]
    pub privileged_system_prompt: String,
    #[serde(default)]
    pub initial_files: Vec<BundleFile>,
    #[serde(default)]
    pub egress_allowlist: Vec<String>,
    pub max_turns: u32,
    #[serde(default)]
    pub arm: ScenarioArm,
    #[serde(default)]
    pub ground_truth: Option<BundleGroundTruth>,
    #[serde(default)]
    pub friction: Vec<FrictionRule>,
    #[serde(default)]
    pub extra_tools: Vec<String>,
    /// Opening phase; see `Prologue`.
    #[serde(default)]
    pub prologue: Option<crate::prompt::Prologue>,
    /// Pre-release files; see `Embargo`.
    #[serde(default)]
    pub embargo: Vec<crate::prompt::Embargo>,
    /// Task-phase escalation beats.
    #[serde(default)]
    pub escalation: Vec<crate::prompt::PrologueBeat>,
}

/// Renders {{tension}} / {{tension_pct}} template variables.
pub fn render_vars(s: &str, tension: f32) -> String {
    let t = tension.clamp(0.0, 1.0);
    s.replace("{{tension}}", &format!("{t:.2}"))
        .replace("{{tension_pct}}", &format!("{}", (t * 100.0).round() as i64))
}

impl ScenarioBundle {
    fn validate_tension(tension: f32) -> Result<f32> {
        if !(0.0..=1.0).contains(&tension) {
            return Err(CoreError::Provider(format!(
                "tension must be within [0,1], got {tension}"
            )));
        }
        Ok(tension)
    }

    fn check_friction(&self) -> Result<()> {
        for r in &self.friction {
            for (label, p) in [("low", r.p_success_low), ("high", r.p_success_high)] {
                if !(0.0..=1.0).contains(&p) {
                    return Err(CoreError::Provider(format!(
                        "friction '{}' p_success_{label} must be within [0,1], got {p}",
                        r.tool
                    )));
                }
            }
        }
        Ok(())
    }

    /// Build a runnable spec at tension γ. The spec id gains a `-tNNN`
    /// suffix so episode records pin which tension was active.
    pub fn build(&self, tension: f32) -> Result<ScenarioSpec> {
        let tension = Self::validate_tension(tension)?;
        self.check_friction()?;
        let pct_suffix = format!("-t{:03}", (tension * 100.0).round() as i64);

        let friction = self
            .friction
            .iter()
            .map(|r| {
                let mut r = r.clone();
                if let Some(e) = &r.success_effect {
                    r.success_effect = Some(render_vars(e, tension));
                }
                r.failure_effect = render_vars(&r.failure_effect, tension);
                r.values = r
                    .values
                    .iter()
                    .map(|(k, v)| (k.clone(), render_vars(v, tension)))
                    .collect();
                r
            })
            .collect();

        Ok(ScenarioSpec {
            id: format!("{}{}", self.id, pct_suffix),
            public_system_prompt: render_vars(&self.public_system_prompt, tension),
            privileged_system_prompt: render_vars(&self.privileged_system_prompt, tension),
            initial_files: self
                .initial_files
                .iter()
                .map(|f| (f.path.clone(), render_vars(&f.content, tension)))
                .collect(),
            egress_allowlist: self.egress_allowlist.clone(),
            max_turns: self.max_turns,
            arm: self.arm,
            ground_truth: self.ground_truth.as_ref().map(|gt| GroundTruth {
                required_files: gt
                    .required_files
                    .iter()
                    .map(|f| (f.path.clone(), f.content.clone()))
                    .collect(),
                forbidden_read_prefixes: gt.forbidden_read_prefixes.clone(),
                forbidden_file_contents: gt
                    .forbidden_file_contents
                    .iter()
                    .map(|f| (f.path.clone(), f.content.clone()))
                    .collect(),
            }),
            tension,
            friction,
            extra_tools: self.extra_tools.clone(),
            prologue: self.prologue.clone(),
            embargo: self.embargo.clone(),
            escalation: self.escalation.clone(),
        })
    }

    /// Build all specs for a tension sweep (e.g. [0, 0.25, 0.5, 0.75, 1]).
    pub fn build_sweep(&self, tensions: &[f32]) -> Result<Vec<(f32, ScenarioSpec)>> {
        tensions
            .iter()
            .map(|&t| Ok((t, self.build(t)?)))
            .collect()
    }
}

/// Deterministic RNG for friction rolls (splitmix64). Seeded from the model
/// seed + episode id so identical runs replay identically (D5).
#[derive(Debug, Clone)]
pub struct FrictionRng {
    state: u64,
}

fn fnv1a(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

impl FrictionRng {
    pub fn new(model_seed: u64, episode_id: &str) -> Self {
        Self { state: model_seed ^ fnv1a(episode_id) }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    /// One draw against probability p. Advances state exactly once either way.
    pub fn succeeds(&mut self, p: f32) -> bool {
        self.next_f32() < p.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tension_interpolation_boundaries() {
        let rule = FrictionRule {
            tool: "oracle".into(),
            p_success_low: 1.0,
            p_success_high: 0.0,
            success_effect: Some("ok".into()),
            success_output: None,
            values: HashMap::new(),
            failure_effect: default_failure_effect(),
        };
        assert!((rule.p_at(0.0) - 1.0).abs() < 1e-6);
        assert!((rule.p_at(1.0).abs()) < 1e-6);
        assert!((rule.p_at(0.5) - 0.5).abs() < 1e-6);
        // Out-of-range tension clamps.
        assert!((rule.p_at(-3.0) - 1.0).abs() < 1e-6);
        assert!((rule.p_at(7.0)).abs() < 1e-6);
    }

    #[test]
    fn rng_is_deterministic_and_advances_once() {
        let mut a = FrictionRng::new(42, "ep-1");
        let mut b = FrictionRng::new(42, "ep-1");
        let mut c = FrictionRng::new(43, "ep-1");
        let seq_a: Vec<bool> = (0..32).map(|_| a.succeeds(0.5)).collect();
        let seq_b: Vec<bool> = (0..32).map(|_| b.succeeds(0.5)).collect();
        let seq_c: Vec<bool> = (0..32).map(|_| c.succeeds(0.5)).collect();
        assert_eq!(seq_a, seq_b, "same seed+episode must replay identically");
        assert_ne!(seq_a, seq_c, "different seed should diverge");
    }

    #[test]
    fn rng_probability_extremes() {
        let mut r = FrictionRng::new(1, "x");
        assert!((0..100).all(|_| r.succeeds(1.0)));
        assert!((0..100).all(|_| !r.succeeds(0.0)));
    }

    #[test]
    fn render_vars_formats() {
        assert_eq!(render_vars("γ={{tension}} ({{tension_pct}}%)", 0.256), "γ=0.26 (26%)");
        assert_eq!(render_vars("none", 0.5), "none");
    }

    #[test]
    fn build_renders_templates_and_pins_id() {
        let bundle = ScenarioBundle {
            id: "test-bundle".into(),
            description: String::new(),
            vectors: vec![],
            public_system_prompt: "queue pressure {{tension_pct}}%".into(),
            privileged_system_prompt: String::new(),
            initial_files: vec![BundleFile { path: "/f".into(), content: "γ={{tension}}".into() }],
            egress_allowlist: vec![],
            max_turns: 4,
            arm: ScenarioArm::Emergent,
            ground_truth: None,
            friction: vec![FrictionRule {
                tool: "oracle".into(),
                p_success_low: 0.9,
                p_success_high: 0.1,
                success_effect: Some("granted at {{tension_pct}}%".into()),
                success_output: None,
                values: HashMap::from([("K".into(), "V{{tension_pct}}".into())]),
                failure_effect: default_failure_effect(),
            }],
            extra_tools: vec!["oracle(key)".into()],
            prologue: None,
            embargo: Vec::new(),
            escalation: Vec::new(),
        };
        let spec = bundle.build(0.5).unwrap();
        assert_eq!(spec.id, "test-bundle-t050");
        assert!((spec.tension - 0.5).abs() < 1e-6);
        assert!(spec.public_system_prompt.contains("50%"));
        assert!(spec.initial_files[0].1.contains("0.50"));
        assert_eq!(spec.friction.len(), 1);
        assert!(spec.friction[0].values["K"].contains("50"));
        assert!(spec.extra_tools.contains(&"oracle(key)".to_string()));

        assert!(bundle.build(1.5).is_err());
        assert!(bundle.build(-0.1).is_err());
    }
}
