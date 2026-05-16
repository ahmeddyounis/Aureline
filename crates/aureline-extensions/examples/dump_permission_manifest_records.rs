//! Dump a serialized `permission_manifest_delta_record` and
//! `permission_manifest_support_export_record` for every checked-in
//! delta fixture so the JSON schema can be independently validated.
//!
//! Used by the schema-validation lane:
//!
//! ```text
//! cargo run --example dump_permission_manifest_records -p aureline-extensions
//! ```

use aureline_extensions::{
    evaluate_permission_manifest_delta, project_permission_manifest_support_export,
    PermissionManifestDeltaInput,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    input: PermissionManifestDeltaInput,
}

fn main() {
    for (name, raw) in fixtures() {
        let fixture: Fixture = serde_json::from_str(raw)
            .unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"));
        let next_manifest = fixture.input.next_manifest.clone();
        let record = evaluate_permission_manifest_delta(fixture.input);
        let export = project_permission_manifest_support_export(
            &next_manifest,
            Some(&record),
            &format!("permission_manifest_support_export:{}", record.delta_id),
        );
        println!("=== {name} / delta ===");
        println!("{}", serde_json::to_string_pretty(&record).unwrap());
        println!("=== {name} / support_export ===");
        println!("{}", serde_json::to_string_pretty(&export).unwrap());
    }
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "unchanged_no_reconsent",
            include_str!(
                "../../../fixtures/extensions/m3/permission_deltas/unchanged_no_reconsent.json"
            ),
        ),
        (
            "narrowing_only",
            include_str!("../../../fixtures/extensions/m3/permission_deltas/narrowing_only.json"),
        ),
        (
            "widening_added_scope",
            include_str!(
                "../../../fixtures/extensions/m3/permission_deltas/widening_added_scope.json"
            ),
        ),
        (
            "widening_added_capability_class",
            include_str!(
                "../../../fixtures/extensions/m3/permission_deltas/widening_added_capability_class.json"
            ),
        ),
        (
            "rationale_only_change",
            include_str!(
                "../../../fixtures/extensions/m3/permission_deltas/rationale_only_change.json"
            ),
        ),
        (
            "mirror_origin_preserved",
            include_str!(
                "../../../fixtures/extensions/m3/permission_deltas/mirror_origin_preserved.json"
            ),
        ),
        (
            "quarantined_publisher_refused",
            include_str!(
                "../../../fixtures/extensions/m3/permission_deltas/quarantined_publisher_refused.json"
            ),
        ),
    ]
}
