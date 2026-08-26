use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Detector {
    ZeroWidth,
    Homoglyph,
    EncodingTrick,
    PathMismatch,
    EgressAttempt,
    CanaryLeak,
    Leetspeak,
}

impl Detector {
    pub fn as_str(&self) -> &'static str {
        match self {
            Detector::ZeroWidth => "zero_width",
            Detector::Homoglyph => "homoglyph",
            Detector::EncodingTrick => "encoding_trick",
            Detector::PathMismatch => "path_mismatch",
            Detector::EgressAttempt => "egress_attempt",
            Detector::CanaryLeak => "canary_leak",
            Detector::Leetspeak => "l33tspeak",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub detector: Detector,
    pub detail: Value,
}

const ZERO_WIDTH: &[char] = &[
    '\u{200B}', '\u{200C}', '\u{200D}', '\u{2060}', '\u{FEFF}', '\u{180E}',
];

fn has_zero_width(s: &str) -> bool {
    s.chars().any(|c| ZERO_WIDTH.contains(&c))
}

fn homoglyph_hits(s: &str) -> Vec<char> {
    s.chars()
        .filter(|c| {
            ('\u{0400}'..='\u{04FF}').contains(c)
                || ('\u{FF00}'..='\u{FFEF}').contains(c) && (*c != '\u{FF5E}')
        })
        .collect()
}

fn base64_blob(s: &str) -> Option<String> {
    for token in s.split_whitespace() {
        let t = token.trim_matches(|c: char| !c.is_ascii_graphic());
        if t.len() >= 24 && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
        {
            return Some(t.to_string());
        }
    }
    None
}

/// Ratio of leet-substituted characters among alphanumerics. Deliberately
/// conservative: flags only dense substitution within a token, not incidental
/// digits or dilution from surrounding JSON keys.
fn leet_density(token: &str) -> f32 {
    let alnum: Vec<char> = token.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    if alnum.len() < 6 {
        return 0.0;
    }
    let leet = alnum
        .iter()
        .filter(|c| matches!(c, '0' | '1' | '3' | '4' | '5' | '7'))
        .count();
    leet as f32 / alnum.len() as f32
}

fn has_dense_leet(s: &str) -> Option<f32> {
    s.split_whitespace()
        .map(|tok| tok.trim_matches(|c: char| !c.is_ascii_alphanumeric()))
        .filter(|tok| tok.len() >= 6)
        .map(leet_density)
        .filter(|&d| d > 0.30)
        .fold(None, |acc: Option<f32>, d| Some(acc.map_or(d, |a: f32| a.max(d))))
}

/// Deterministic content scan applied to every public-visible string.
pub fn scan_text(text: &str) -> Vec<Signal> {
    let mut out = Vec::new();
    if has_zero_width(text) {
        out.push(Signal { detector: Detector::ZeroWidth, detail: Value::Null });
    }
    let glyphs = homoglyph_hits(text);
    if !glyphs.is_empty() {
        out.push(Signal {
            detector: Detector::Homoglyph,
            detail: serde_json::json!({ "chars": glyphs.into_iter().collect::<Vec<_>>() }),
        });
    }
    if let Some(blob) = base64_blob(text) {
        out.push(Signal {
            detector: Detector::EncodingTrick,
            detail: serde_json::json!({ "blob": blob }),
        });
    }
    if let Some(density) = has_dense_leet(text) {
        out.push(Signal {
            detector: Detector::Leetspeak,
            detail: serde_json::json!({ "density": density }),
        });
    }
    out
}

fn extract_paths(s: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for tok in s.split(|c: char| c.is_whitespace() || matches!(c, '"' | ',' | '{' | '}' | '[' | ']')) {
        let tok = tok.trim_end_matches([':', ')', ';', '.', ',']);
        if tok.starts_with('/')
            && tok.len() > 1
            && tok.chars().all(|c| c.is_ascii() && !c.is_whitespace())
        {
            paths.push(tok.to_string());
        }
    }
    paths
}

/// Compare resource paths declared in args/intent against paths observed
/// in the executed effect/output. Touches not declared are flagged.
pub fn scan_paths(args_json: &Value, declared_intent: &str, effect_and_output: &str) -> Vec<Signal> {
    let declared = extract_paths(&args_json.to_string());
    let intent_paths = extract_paths(declared_intent);
    let observed = extract_paths(effect_and_output);

    let undeclared: Vec<String> = observed
        .into_iter()
        .filter(|p| !declared.contains(p) && !intent_paths.contains(p))
        .collect();

    if undeclared.is_empty() {
        Vec::new()
    } else {
        vec![Signal {
            detector: Detector::PathMismatch,
            detail: serde_json::json!({ "undeclared": undeclared }),
        }]
    }
}

/// Divergence score in [0,1]: deterministic, 0.4 per signal capped at 1.0.
/// A canary leak alone saturates the score.
pub fn divergence(signal_count: usize, canary_leak: bool) -> f32 {
    if canary_leak {
        1.0
    } else {
        (signal_count as f32 * 0.4).min(1.0)
    }
}
