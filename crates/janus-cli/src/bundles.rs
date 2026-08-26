//! Bundle file loader: TOML or JSON → [`ScenarioBundle`] → runnable
//! [`ScenarioSpec`] at a given tension.

use janus_core::{CoreError, Result, ScenarioBundle};

pub fn load_bundle(path: &str) -> Result<ScenarioBundle> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| CoreError::Sandbox(format!("read bundle {path}: {e}")))?;
    let ext = path.rsplit('.').next().unwrap_or("");
    match ext {
        "toml" => toml::from_str(&text)
            .map_err(|e| CoreError::Sandbox(format!("parse bundle {path}: {e}"))),
        "json" => serde_json::from_str(&text)
            .map_err(|e| CoreError::Sandbox(format!("parse bundle {path}: {e}"))),
        other => Err(CoreError::Sandbox(format!(
            "unsupported bundle format '.{other}' (expected .toml or .json): {path}"
        ))),
    }
}

/// Parse a comma-separated tension list ("0,0.25,1") into validated f32s.
pub fn parse_tensions(spec: &str) -> Result<Vec<f32>> {
    let mut out = Vec::new();
    for part in spec.split(',') {
        let t: f32 = part
            .trim()
            .parse()
            .map_err(|_| CoreError::Provider(format!("invalid tension '{part}'")))?;
        if !(0.0..=1.0).contains(&t) {
            return Err(CoreError::Provider(format!(
                "tension must be within [0,1], got {t}"
            )));
        }
        out.push(t);
    }
    if out.is_empty() {
        return Err(CoreError::Provider("need at least one tension".into()));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tension_list() {
        assert_eq!(parse_tensions("0,0.5,1").unwrap(), vec![0.0, 0.5, 1.0]);
        assert!(parse_tensions("1.5").is_err());
        assert!(parse_tensions("abc").is_err());
        assert!(parse_tensions("").is_err());
    }

    #[test]
    fn loads_toml_bundle_from_disk() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../bundles/oracle-backup.toml");
        let b = load_bundle(path).unwrap();
        assert!(!b.public_system_prompt.is_empty());
        assert!(!b.friction.is_empty());
        let spec = b.build(1.0).unwrap();
        assert_eq!(spec.id, "oracle-backup-t100");
    }

    /// Every shipped bundle must parse and build at both sweep endpoints.
    #[test]
    fn all_shipped_bundles_load_and_build() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../bundles");
        let entries = std::fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("read {dir}: {e}"));
        let mut count = 0;
        for entry in entries {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("toml") {
                continue;
            }
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let bundle =
                load_bundle(path.to_str().unwrap()).expect(&name);
            assert!(!bundle.id.is_empty(), "{name}: empty id");
            assert!(bundle.max_turns > 0, "{name}: max_turns must be positive");
            for t in [0.0f32, 1.0] {
                let spec = bundle.build(t).unwrap_or_else(|e| panic!("{name}@γ={t}: {e}"));
                assert_eq!(
                    spec.tension, t,
                    "{name}: tension not threaded into spec"
                );
                assert_eq!(
                    spec.initial_files.len(),
                    bundle.initial_files.len(),
                    "{name}: initial files dropped in build"
                );
                // Ground truth round-trips.
                if bundle.ground_truth.is_some() {
                    assert!(spec.ground_truth.is_some(), "{name}");
                }
                // Friction rules survive with interpolated effects.
                assert_eq!(spec.friction.len(), bundle.friction.len(), "{name}");
            }
            count += 1;
        }
        assert!(count >= 5, "expected ≥5 shipped bundles, found {count}");
    }
}
