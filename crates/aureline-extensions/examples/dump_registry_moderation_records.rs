//! Dump serialized catalog descriptor and support-export records for the
//! checked registry moderation fixture suite.
//!
//! Used by the schema-validation lane:
//!
//! ```text
//! cargo run --example dump_registry_moderation_records -p aureline-extensions
//! ```

use aureline_extensions::{
    evaluate_catalog_descriptor, project_catalog_descriptor_support_export, CatalogDescriptorInput,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    input: CatalogDescriptorInput,
}

fn main() {
    for (name, raw) in fixtures() {
        let fixture: Fixture = serde_json::from_str(raw)
            .unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"));
        let record = evaluate_catalog_descriptor(fixture.input);
        let export = project_catalog_descriptor_support_export(
            &record,
            &format!("catalog_descriptor_support_export:{}", record.descriptor_id),
        );
        println!("=== {name} / catalog_descriptor ===");
        println!("{}", serde_json::to_string_pretty(&record).unwrap());
        println!("=== {name} / support_export ===");
        println!("{}", serde_json::to_string_pretty(&export).unwrap());
    }
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "mirrorable_catalog_approved",
            include_str!(
                "../../../fixtures/extensions/m3/registry_moderation/mirrorable_catalog_approved.json"
            ),
        ),
        (
            "staged_pending_moderation",
            include_str!(
                "../../../fixtures/extensions/m3/registry_moderation/staged_pending_moderation.json"
            ),
        ),
        (
            "limited_compatibility_catalog",
            include_str!(
                "../../../fixtures/extensions/m3/registry_moderation/limited_compatibility_catalog.json"
            ),
        ),
        (
            "revoked_catalog_refused",
            include_str!(
                "../../../fixtures/extensions/m3/registry_moderation/revoked_catalog_refused.json"
            ),
        ),
        (
            "quarantined_publisher_refused",
            include_str!(
                "../../../fixtures/extensions/m3/registry_moderation/quarantined_publisher_refused.json"
            ),
        ),
    ]
}
