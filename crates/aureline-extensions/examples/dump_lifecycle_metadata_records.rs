//! Dump serialized lifecycle metadata packet and support-export records
//! for the checked fixture suite and canonical packet.
//!
//! Used by the schema-validation lane:
//!
//! ```text
//! cargo run --example dump_lifecycle_metadata_records -p aureline-extensions
//! ```

use aureline_extensions::{
    current_extension_lifecycle_metadata_packet, evaluate_lifecycle_metadata_packet,
    project_lifecycle_metadata_support_export, LifecycleMetadataPacketInput,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    input: LifecycleMetadataPacketInput,
}

fn main() {
    let checked = current_extension_lifecycle_metadata_packet()
        .expect("checked lifecycle metadata packet must deserialize");
    println!("=== checked / lifecycle_metadata_packet ===");
    println!("{}", serde_json::to_string_pretty(&checked).unwrap());
    let checked_export = project_lifecycle_metadata_support_export(
        &checked,
        &format!(
            "extension_lifecycle_metadata_support_export:{}",
            checked.packet_id
        ),
    );
    println!("=== checked / support_export ===");
    println!("{}", serde_json::to_string_pretty(&checked_export).unwrap());

    for (name, raw) in fixtures() {
        let fixture: Fixture = serde_json::from_str(raw)
            .unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"));
        let record = evaluate_lifecycle_metadata_packet(fixture.input);
        let export = project_lifecycle_metadata_support_export(
            &record,
            &format!(
                "extension_lifecycle_metadata_support_export:{}",
                record.packet_id
            ),
        );
        println!("=== {name} / lifecycle_metadata_packet ===");
        println!("{}", serde_json::to_string_pretty(&record).unwrap());
        println!("=== {name} / support_export ===");
        println!("{}", serde_json::to_string_pretty(&export).unwrap());
    }
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "ready_beta_surfaces_with_deprecation",
            include_str!(
                "../../../fixtures/extensions/m3/lifecycle_metadata/ready_beta_surfaces_with_deprecation.json"
            ),
        ),
        (
            "refused_deprecated_missing_replacement",
            include_str!(
                "../../../fixtures/extensions/m3/lifecycle_metadata/refused_deprecated_missing_replacement.json"
            ),
        ),
    ]
}
