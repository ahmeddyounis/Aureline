//! Shell parity coverage for portable bundle inspection.

use aureline_shell::portable_bundle_inspector::{
    build_portable_bundle_rows, render_portable_bundle_plaintext,
};

#[test]
fn shell_inspector_preserves_reopen_and_redaction_posture() {
    let rows = build_portable_bundle_rows().expect("portable bundle rows project");
    assert_eq!(rows.len(), 4);
    for row in &rows {
        assert!(!row.bundle_id.trim().is_empty());
        assert!(row.diff_ref_count > 0);
        assert!(row.evidence_ref_count > 0);
        assert!(matches!(
            row.authority_class.as_str(),
            "no_live_provider_authority"
                | "imported_stale_provider_snapshot"
                | "desktop_reauth_required"
                | "local_resume_only"
                | "authority_unknown_requires_review"
        ));
    }
    assert!(
        rows.iter()
            .any(|row| row.validation_freshness_class != "validation_current"
                && !row.staleness_labels.is_empty()),
        "stale validation must stay visible in shell rows"
    );
}

#[test]
fn shell_plaintext_includes_offline_and_resume_posture() {
    let rendered = render_portable_bundle_plaintext().expect("plaintext renders");
    assert!(rendered.contains("Portable bundle inspector beta"));
    assert!(rendered.contains("offline_review_handoff"));
    assert!(rendered.contains("browser_companion_handoff"));
    assert!(rendered.contains("desktop_resume_pending_revalidation"));
}
