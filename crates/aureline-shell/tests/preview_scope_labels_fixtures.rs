//! Fixture replay for preview-surface scope labels and handoff limits.
//!
//! The generated packet under `artifacts/compat/m3/` is consumed by shell
//! surfaces that need to quote lifecycle, support, client-scope, and handoff
//! labels without widening browser-companion or voice authority.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use aureline_shell::preview_scope_labels::{
    PreviewScopeLabelRegister, PREVIEW_SCOPE_LABEL_REGISTER_RECORD_KIND,
    PREVIEW_SCOPE_LABEL_REGISTER_SCHEMA_VERSION, REQUIRED_PREVIEW_SCOPE_PROJECTIONS,
    REQUIRED_PREVIEW_SCOPE_SURFACE_FAMILIES,
};

fn packet_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/compat/m3/qualified_preview_rows.json")
}

fn load_packet() -> PreviewScopeLabelRegister {
    PreviewScopeLabelRegister::load_from_path(packet_path())
        .expect("qualified preview-scope packet must load")
}

#[test]
fn generated_packet_validates_and_covers_required_families() {
    let packet = load_packet();
    assert_eq!(packet.record_kind, PREVIEW_SCOPE_LABEL_REGISTER_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        PREVIEW_SCOPE_LABEL_REGISTER_SCHEMA_VERSION
    );
    packet
        .validate()
        .expect("qualified preview-scope packet must validate");

    assert_eq!(packet.summary.row_count as usize, packet.rows.len());
    assert_eq!(
        packet.summary.support_export_row_count as usize,
        packet.support_export_rows.len()
    );

    let families: BTreeSet<_> = packet
        .rows
        .iter()
        .map(|row| row.surface_family.as_str())
        .collect();
    assert_eq!(
        families,
        REQUIRED_PREVIEW_SCOPE_SURFACE_FAMILIES
            .into_iter()
            .collect::<BTreeSet<_>>()
    );

    let lifecycles: BTreeSet<_> = packet
        .rows
        .iter()
        .map(|row| row.effective_lifecycle_label.as_str())
        .collect();
    assert!(lifecycles.contains("preview"));
    assert!(lifecycles.contains("beta"));
    assert!(!lifecycles.contains("stable"));
}

#[test]
fn browser_companion_and_voice_require_desktop_handoff_without_native_depth_claims() {
    let packet = load_packet();
    for family in ["browser_companion", "voice"] {
        let row = packet
            .rows
            .iter()
            .find(|row| row.surface_family == family)
            .expect("required preview surface family row exists");
        assert!(
            !row.native_depth_capability_claimed,
            "{family} must not claim native-depth capability"
        );
        assert!(row.handoff.required, "{family} must require handoff");
        assert_eq!(row.handoff.target, "desktop");
        assert!(
            !row.handoff.limitation_statement.trim().is_empty(),
            "{family} must carry a product-facing limitation"
        );
        assert!(
            row.handoff.preserves_context,
            "{family} handoff must preserve current context"
        );
    }
}

#[test]
fn support_exports_match_product_rows_and_strip_private_material() {
    let packet = load_packet();
    let support_by_ref: BTreeMap<_, _> = packet
        .support_export_rows
        .iter()
        .map(|row| (row.surface_ref.as_str(), row))
        .collect();

    for row in &packet.rows {
        let support = support_by_ref
            .get(row.surface_id.as_str())
            .expect("support export row exists for preview row");
        assert_eq!(&row.support_export, *support);
        assert_eq!(support.lifecycle_label, row.effective_lifecycle_label);
        assert_eq!(support.support_class, row.effective_support_class);
        assert_eq!(support.client_scope, row.client_scope);
        assert_eq!(support.freshness_state, row.evidence.freshness_state);
        assert_eq!(support.handoff_required, row.handoff.required);
        assert_eq!(support.handoff_target, row.handoff.target);
        assert!(support.raw_private_material_excluded);
        assert!(support.ambient_authority_excluded);
    }
}

#[test]
fn every_row_projects_to_required_shell_and_export_consumers() {
    let packet = load_packet();
    for consumer in REQUIRED_PREVIEW_SCOPE_PROJECTIONS {
        let rows = packet.rows_for_consumer(consumer);
        assert_eq!(
            rows.len(),
            packet.rows.len(),
            "{consumer} must receive every qualified preview-scope row"
        );
    }
}

#[test]
fn plaintext_summary_mentions_lifecycle_scope_and_handoff_labels() {
    let packet = load_packet();
    let plaintext = packet.render_plaintext();
    assert!(plaintext.contains("Notebook workflow parity"));
    assert!(plaintext.contains("Voice and dictation"));
    assert!(plaintext.contains("Browser companion"));
    assert!(plaintext.contains("Preview canvas"));
    assert!(plaintext.contains("Preview / Experimental / Desktop"));
    assert!(plaintext.contains("Beta / Limited / Browser companion"));
    assert!(plaintext.contains("handoff=Aureline desktop"));
    assert!(plaintext.contains("handoff=Desktop command review"));
}
