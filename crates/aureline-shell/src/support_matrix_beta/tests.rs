use super::*;

use aureline_runtime::{SupportMatrixBetaManifest, SupportMatrixBetaSupportExport};

#[test]
fn panel_renders_one_row_per_canonical_wedge_row() {
    let manifest =
        SupportMatrixBetaManifest::canonical("matrix:shell:test", "2026-05-16T00:00:00Z");
    let panel = SupportMatrixBetaPanel::from_manifest(&manifest);
    assert_eq!(panel.record_kind, SUPPORT_MATRIX_BETA_PANEL_RECORD_KIND);
    assert_eq!(panel.rows.len(), manifest.rows.len());
    // TS/JS is preview/limited — protected dispatch must be blocked on at
    // least one row, so the panel summary must reflect that.
    assert!(panel.any_row_blocks_protected_dispatch);
}

#[test]
fn plaintext_block_quotes_typed_tokens_verbatim() {
    let manifest =
        SupportMatrixBetaManifest::canonical("matrix:shell:plain", "2026-05-16T00:00:00Z");
    let panel = SupportMatrixBetaPanel::from_manifest(&manifest);
    let plain = panel.render_plaintext();

    assert!(plain.contains("Wedge: python"));
    assert!(plain.contains("Wedge: typescript_javascript"));

    // Closed support-class tokens must appear verbatim in the rendered
    // panel.
    assert!(plain.contains("Launch: supported"));
    assert!(plain.contains("Attach: preview"));
    assert!(plain.contains("Test: supported"));
    assert!(plain.contains("Test: limited"));

    // Lane tokens are quoted verbatim from the runtime, not paraphrased.
    assert!(plain.contains("local_host=supported"));
    assert!(plain.contains("container=supported"));
    assert!(plain.contains("remote_attach=preview"));
    assert!(plain.contains("request_workspace=preview"));

    // Downgrade tokens must be quoted verbatim so support consumers can
    // grep the matrix output against the closed vocabulary.
    assert!(plain.contains("- block_protected_dispatch_on_capsule_drift"));
    assert!(plain.contains("- block_on_unclaimed_test_framework"));

    // Missing-feature wording: TS/JS has no claimed framework yet.
    assert!(plain.contains("Claimed frameworks: (none)"));
}

#[test]
fn from_support_export_round_trips_through_panel() {
    let manifest =
        SupportMatrixBetaManifest::canonical("matrix:shell:export", "2026-05-16T00:00:00Z");
    let export = SupportMatrixBetaSupportExport::new(
        "support-export:shell:1",
        "2026-05-16T00:00:01Z",
        manifest.clone(),
        vec![],
    );
    let panel = SupportMatrixBetaPanel::from_support_export(&export);
    assert_eq!(panel.manifest_id, "matrix:shell:export");
    assert_eq!(panel.rows.len(), manifest.rows.len());
}
