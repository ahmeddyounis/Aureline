//! Embedded docs/help boundary chrome seed.
//!
//! This module provides a minimal, runnable docs/help surface boundary card that
//! can be exercised inside the shell runtime. The card is intentionally host
//! rendered and includes a system-browser escape hatch.

use std::path::{Path, PathBuf};

use aureline_commands::invocation::now_rfc3339;
use serde_json::Value as JsonValue;

use super::boundary_card::EmbeddedBoundaryCardRecord;

const DOCS_HELP_BROWSER_HANDOFF_PACKET_REF: &str = "id:browser-handoff:docs-help:project-docs";

/// Returns a seeded embedded boundary card for one docs/help surface.
pub fn seeded_docs_help_boundary_card(
    build_identity_ref: impl Into<String>,
) -> EmbeddedBoundaryCardRecord {
    let mut record = baseline_docs_help_boundary_card_fixture();
    if let Some(source_truth) = record.source_truth.as_mut() {
        source_truth.running_build_identity_ref = build_identity_ref.into();
    }
    record.minted_at = now_rfc3339();
    record
}

/// Resolves a docs/help browser-handoff packet ref to a concrete URL.
pub fn resolve_docs_help_handoff_url(packet_ref: &str) -> Option<String> {
    if packet_ref != DOCS_HELP_BROWSER_HANDOFF_PACKET_REF {
        return None;
    }

    let repo_root = repo_root_from_manifest_dir()?;
    let doc_path = repo_root.join("docs/ux/embedded_boundary_contract.md");
    file_url_for_path(&doc_path)
}

fn repo_root_from_manifest_dir() -> Option<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.parent()?.parent().map(Path::to_path_buf)
}

fn file_url_for_path(path: &Path) -> Option<String> {
    let canonical = path.canonicalize().ok()?;
    let raw = canonical.to_string_lossy();
    if cfg!(windows) {
        Some(format!("file:///{}", raw.replace('\\', "/")))
    } else {
        Some(format!("file://{raw}"))
    }
}

fn baseline_docs_help_boundary_card_fixture() -> EmbeddedBoundaryCardRecord {
    const FIXTURE_JSON: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ux/embedded_surfaces/docs_help_boundary_contract_card.json"
    ));

    let value: JsonValue = serde_json::from_str(FIXTURE_JSON)
        .expect("docs/help boundary card fixture must parse as JSON");
    serde_json::from_value(value).expect("docs/help boundary card fixture must match record shape")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedded::boundary_card::SurfaceFamily;

    #[test]
    fn seeded_docs_help_boundary_card_exposes_browser_handoff_packet() {
        let card = seeded_docs_help_boundary_card("id:build:test:01");
        assert_eq!(card.record_kind, "embedded_boundary_card_record");
        assert_eq!(card.embedded_boundary_card_schema_version, 1);
        assert_eq!(card.surface_family, SurfaceFamily::EmbeddedDocsHelp);

        let action = card
            .open_in_browser_action()
            .expect("open in browser action must exist");
        let packet_ref = action
            .browser_handoff_packet_ref
            .as_deref()
            .expect("open in browser action must quote packet ref");
        assert_eq!(packet_ref, DOCS_HELP_BROWSER_HANDOFF_PACKET_REF);

        let url = resolve_docs_help_handoff_url(packet_ref)
            .expect("docs/help packet ref must resolve to URL");
        assert!(
            url.contains("embedded_boundary_contract.md"),
            "resolved url must target the embedded boundary contract doc: {url}"
        );
    }
}
