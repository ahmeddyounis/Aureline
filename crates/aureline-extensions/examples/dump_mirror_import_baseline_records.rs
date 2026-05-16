//! Dump serialized mirror/manual import baseline and support-export records.
//!
//! Used by the schema-validation lane:
//!
//! ```text
//! cargo run --example dump_mirror_import_baseline_records -p aureline-extensions
//! ```

use aureline_extensions::{
    evaluate_mirror_import_baseline, project_mirror_import_support_export,
    MirrorImportBaselineInput,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    input: MirrorImportBaselineInput,
}

fn main() {
    for (name, raw) in fixtures() {
        let fixture: Fixture = serde_json::from_str(raw)
            .unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"));
        let record = evaluate_mirror_import_baseline(fixture.input);
        let export = project_mirror_import_support_export(
            &record,
            &format!("mirror_import_support_export:{}", record.baseline_id),
        );
        println!("=== {name} / mirror_import_baseline ===");
        println!("{}", serde_json::to_string_pretty(&record).unwrap());
        println!("=== {name} / support_export ===");
        println!("{}", serde_json::to_string_pretty(&export).unwrap());
    }
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "primary_catalog_baseline_ready",
            include_str!(
                "../../../fixtures/extensions/m3/mirror_import/primary_catalog_baseline_ready.json"
            ),
        ),
        (
            "approved_mirror_degraded_trust_claim_ready",
            include_str!(
                "../../../fixtures/extensions/m3/mirror_import/approved_mirror_degraded_trust_claim_ready.json"
            ),
        ),
        (
            "manual_artifact_import_preserves_metadata",
            include_str!(
                "../../../fixtures/extensions/m3/mirror_import/manual_artifact_import_preserves_metadata.json"
            ),
        ),
    ]
}
