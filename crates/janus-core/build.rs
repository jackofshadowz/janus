//! Captures build-time provenance so every result can be traced back to the
//! exact code that produced it.

use std::process::Command;

fn main() {
    let sha = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into());

    // A dirty tree means the recorded SHA does not fully describe the run.
    // Results carry the marker so it cannot be quietly cited as reproducible.
    //
    // Scoped to source. The archive under `results/` is tracked, so clearing
    // a run directory before a rerun — `rm -rf results/runs/x` — leaves
    // tracked deletions and marked the rebuilt binary dirty. A three-arm run
    // came back stamped "not reproducible" when the only difference from
    // HEAD was deleted result files, which cannot affect the code that
    // produced it. A caveat that fires where it cannot apply teaches a
    // reader to skip caveats, and this project already made that mistake
    // once with judge calibration (METHODOLOGY_LOG M38).
    //
    // Paths that can change behaviour: the crates, the manifests, the lock
    // file. Anything else — results, docs, notes — leaves the binary
    // identical and is not reported as a reproducibility risk.
    let dirty = Command::new("git")
        .args([
            "status",
            "--porcelain",
            "--",
            "crates",
            "Cargo.toml",
            "Cargo.lock",
            "build.rs",
        ])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);

    println!("cargo:rustc-env=JANUS_GIT_SHA={sha}");
    println!("cargo:rustc-env=JANUS_GIT_DIRTY={dirty}");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/index");
}
