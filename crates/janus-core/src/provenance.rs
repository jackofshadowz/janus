//! Result provenance.
//!
//! A number is only publishable if you can say what produced it. Every
//! episode record and every run manifest carries this block so a result can
//! be traced to an exact commit, scenario text, protocol, and judge.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Short content hash, used to pin scenario text independently of its file
/// name — a bundle edited in place would otherwise silently change what a
/// scenario id refers to.
pub fn content_hash(s: &str) -> String {
    let digest = Sha256::digest(s.as_bytes());
    format!("{:x}", digest)[..16].to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Provenance {
    /// Harness crate version.
    pub harness_version: String,
    /// Commit that built the harness, or "unknown" outside a git tree.
    pub git_sha: String,
    /// Uncommitted changes were present at build time. **A result with
    /// `git_dirty: true` is not reproducible** and must not be published
    /// without qualification.
    pub git_dirty: bool,
    /// Action protocol; envelope and native runs are not comparable.
    pub protocol: String,
    /// Sandbox backend.
    pub sandbox: String,
    /// Judge identifier: `heuristic-v0` or `llm:<provider>:<model>`.
    pub judge: String,
    /// Whether the judge cleared the calibration gate for this run. Metrics
    /// from an uncalibrated judge are interim by definition.
    pub judge_calibrated: bool,
    /// Unix seconds.
    pub created_unix: u64,
}

impl Provenance {
    pub fn new(
        protocol: impl Into<String>,
        sandbox: impl Into<String>,
        judge: impl Into<String>,
        created_unix: u64,
    ) -> Self {
        Self {
            harness_version: env!("CARGO_PKG_VERSION").to_string(),
            git_sha: option_env!("JANUS_GIT_SHA").unwrap_or("unknown").to_string(),
            git_dirty: option_env!("JANUS_GIT_DIRTY") == Some("true"),
            protocol: protocol.into(),
            sandbox: sandbox.into(),
            judge: judge.into(),
            judge_calibrated: false,
            created_unix,
        }
    }

    /// Whether this run may be cited without a caveat.
    pub fn is_publishable(&self) -> bool {
        !self.git_dirty && self.git_sha != "unknown" && self.judge_calibrated
    }

    /// Human-readable reasons this run is not citable as-is.
    pub fn caveats(&self) -> Vec<String> {
        let mut v = Vec::new();
        if self.git_sha == "unknown" {
            v.push("built outside a git tree — code version unknown".into());
        }
        if self.git_dirty {
            v.push(format!(
                "built from a dirty tree at {} — not reproducible",
                self.git_sha
            ));
        }
        if !self.judge_calibrated {
            v.push(format!(
                "judge `{}` has not cleared the calibration gate — metrics are interim",
                self.judge
            ));
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prov() -> Provenance {
        Provenance::new("native_tools", "mock", "heuristic-v0", 1_700_000_000)
    }

    #[test]
    fn uncalibrated_judge_blocks_publication() {
        let p = prov();
        assert!(!p.is_publishable());
        assert!(p.caveats().iter().any(|c| c.contains("calibration")));
    }

    #[test]
    fn dirty_tree_blocks_publication_even_when_calibrated() {
        let mut p = prov();
        p.judge_calibrated = true;
        p.git_sha = "abc123def456".into();
        p.git_dirty = true;
        assert!(!p.is_publishable());
        assert!(p.caveats().iter().any(|c| c.contains("not reproducible")));
    }

    #[test]
    fn clean_calibrated_run_is_publishable() {
        let mut p = prov();
        p.judge_calibrated = true;
        p.git_sha = "abc123def456".into();
        p.git_dirty = false;
        assert!(p.is_publishable());
        assert!(p.caveats().is_empty());
    }

    #[test]
    fn content_hash_is_stable_and_discriminating() {
        assert_eq!(content_hash("abc"), content_hash("abc"));
        assert_ne!(content_hash("abc"), content_hash("abd"));
        assert_eq!(content_hash("abc").len(), 16);
    }
}
