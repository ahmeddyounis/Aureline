//! Dump serialized manifest editor session records for the checked fixture
//! suite.
//!
//! Used by the schema-validation lane:
//!
//! ```text
//! cargo run --example dump_manifest_editor_records -p aureline-extensions
//! ```

use aureline_extensions::{evaluate_manifest_editor_session, ManifestEditorSessionInput};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    input: ManifestEditorSessionInput,
}

fn main() {
    for (name, raw) in fixtures() {
        let fixture: Fixture = serde_json::from_str(raw)
            .unwrap_or_else(|err| panic!("fixture {name} must deserialize: {err}"));
        let session = evaluate_manifest_editor_session(fixture.input);
        println!("=== {name} / manifest_editor_session ===");
        println!("{}", serde_json::to_string_pretty(&session).unwrap());
    }
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "ready_to_publish_wasm",
            include_str!(
                "../../../fixtures/extensions/m3/manifest_editor/ready_to_publish_wasm.json"
            ),
        ),
        (
            "advisories_deprecated_state_eager_startup",
            include_str!(
                "../../../fixtures/extensions/m3/manifest_editor/advisories_deprecated_state_eager_startup.json"
            ),
        ),
        (
            "blocked_invalid_identity_and_vocabulary",
            include_str!(
                "../../../fixtures/extensions/m3/manifest_editor/blocked_invalid_identity_and_vocabulary.json"
            ),
        ),
    ]
}
