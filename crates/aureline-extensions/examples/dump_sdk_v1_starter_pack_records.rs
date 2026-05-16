//! Dump a serialized `sdk_v1_starter_pack_record` and
//! `sdk_v1_starter_pack_support_export_record` for every checked-in
//! starter-pack fixture so the JSON schema can be independently
//! validated.
//!
//! Used by the schema-validation lane:
//!
//! ```text
//! cargo run --example dump_sdk_v1_starter_pack_records -p aureline-extensions
//! ```

use aureline_extensions::{
    evaluate_sdk_v1_starter_pack, project_sdk_v1_starter_pack_support_export,
    SdkV1StarterPackInput,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    input: SdkV1StarterPackInput,
}

fn main() {
    for (name, raw) in fixtures() {
        let fixture: Fixture = serde_json::from_str(raw)
            .unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"));
        let record = evaluate_sdk_v1_starter_pack(fixture.input);
        let export = project_sdk_v1_starter_pack_support_export(
            &record,
            &format!("sdk_v1_starter_pack_support_export:{}", record.starter_pack_id),
        );
        println!("=== {name} / starter_pack ===");
        println!("{}", serde_json::to_string_pretty(&record).unwrap());
        println!("=== {name} / support_export ===");
        println!("{}", serde_json::to_string_pretty(&export).unwrap());
    }
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "ready_for_authors_wasm_and_external_host",
            include_str!(
                "../../../fixtures/extensions/m3/sample_pack/ready_for_authors_wasm_and_external_host.json"
            ),
        ),
        (
            "partially_ready_preview_surface",
            include_str!(
                "../../../fixtures/extensions/m3/sample_pack/partially_ready_preview_surface.json"
            ),
        ),
        (
            "refused_missing_wasm_sample",
            include_str!(
                "../../../fixtures/extensions/m3/sample_pack/refused_missing_wasm_sample.json"
            ),
        ),
        (
            "refused_authoring_guide_missing",
            include_str!(
                "../../../fixtures/extensions/m3/sample_pack/refused_authoring_guide_missing.json"
            ),
        ),
        (
            "refused_retired_surface",
            include_str!(
                "../../../fixtures/extensions/m3/sample_pack/refused_retired_surface.json"
            ),
        ),
    ]
}
