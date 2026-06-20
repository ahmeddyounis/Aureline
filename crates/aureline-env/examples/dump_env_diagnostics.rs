//! Dumps the environment-artifact bundles, their diagnostics reports,
//! Project-Doctor probes, cross-channel comparison, and the fixture corpus.
//!
//! With no argument it prints `{ "bundles": ..., "reports": ...,
//! "probes": ..., "comparison": ..., "fixtures": [...] }` for human
//! inspection. Pass `bundles`, `reports`, `probes`, `comparison`, or
//! `fixtures` to print only that view. The `fixtures` form is the one
//! written, per fixture, under `fixtures/env/env-diagnostics/`.

use aureline_env::{
    compare_env_bundles, diagnose_bundle, doctor_env_probes, seeded_env_artifact_bundles,
    seeded_env_diagnostics_fixtures,
};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let bundles = seeded_env_artifact_bundles();
    let comparison = match (bundles.first(), bundles.get(1)) {
        (Some(base), Some(target)) => Some(compare_env_bundles(base, target)),
        _ => None,
    };
    let value = match mode.as_str() {
        "bundles" => serde_json::to_value(&bundles),
        "reports" => serde_json::to_value(bundles.iter().map(diagnose_bundle).collect::<Vec<_>>()),
        "probes" => serde_json::to_value(bundles.iter().map(doctor_env_probes).collect::<Vec<_>>()),
        "comparison" => serde_json::to_value(&comparison),
        "fixtures" => serde_json::to_value(seeded_env_diagnostics_fixtures()),
        _ => serde_json::to_value(serde_json::json!({
            "bundles": &bundles,
            "reports": bundles.iter().map(diagnose_bundle).collect::<Vec<_>>(),
            "probes": bundles.iter().map(doctor_env_probes).collect::<Vec<_>>(),
            "comparison": comparison,
            "fixtures": seeded_env_diagnostics_fixtures(),
        })),
    }
    .expect("environment-artifact bundles, reports, probes, and fixtures serialize");
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("pretty JSON renders")
    );
}
