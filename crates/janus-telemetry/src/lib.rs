pub mod detectors;
pub mod intercept;

pub use detectors::{divergence, scan_paths, scan_text, Detector, Signal};
pub use intercept::InterceptingSandbox;

/// Detector names fired by a plain content scan (used by multi-agent
/// coercion tagging).
pub fn detectors_public(text: &str) -> Vec<String> {
    scan_text(text)
        .into_iter()
        .map(|s| s.detector.as_str().to_string())
        .collect()
}
