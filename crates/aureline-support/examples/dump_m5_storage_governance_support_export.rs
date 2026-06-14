//! Dumps the metadata-safe M5 storage-governance support export as pretty
//! JSON. Used to regenerate the golden fixture at
//! `fixtures/storage/m5_artifact_family_storage_matrix/support_export.golden.json`.

use aureline_support::m5_storage_governance::current_m5_artifact_family_storage_matrix;

fn main() {
    let matrix = current_m5_artifact_family_storage_matrix().expect("matrix parses");
    let export = matrix.support_export(
        "support_export.m5_storage_governance.v1",
        "2026-06-14T00:00:00Z",
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&export).expect("serialize support export")
    );
}
