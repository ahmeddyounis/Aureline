//! Support-side parity drill for portable bundle handoff.

use aureline_support::portable_bundle_handoff::{
    compile_portable_bundle_support_export_envelope, PortableBundleSupportExportEnvelope,
    PORTABLE_BUNDLE_SUPPORT_ENVELOPE_RECORD_KIND, PORTABLE_BUNDLE_SUPPORT_ROW_RECORD_KIND,
};

#[test]
fn support_envelope_preserves_portable_bundle_reopen_truth() {
    let envelope = compile_portable_bundle_support_export_envelope(
        "support_export:portable_bundle:test",
        "2026-05-18T17:00:00Z",
    )
    .expect("envelope compiles");
    assert_eq!(
        envelope.record_kind,
        PORTABLE_BUNDLE_SUPPORT_ENVELOPE_RECORD_KIND
    );
    assert!(envelope.is_export_safe());
    assert_eq!(envelope.rows.len(), 4);

    for row in &envelope.rows {
        assert_eq!(row.record_kind, PORTABLE_BUNDLE_SUPPORT_ROW_RECORD_KIND);
        assert!(row.no_live_provider_authority);
        assert!(row.diff_ref_count > 0);
        assert!(row.evidence_ref_count > 0);
    }
    assert!(
        envelope
            .rows
            .iter()
            .any(|row| !row.staleness_labels.is_empty()),
        "support rows must preserve stale-validation labels"
    );
}

#[test]
fn support_envelope_round_trips_through_json() {
    let envelope = compile_portable_bundle_support_export_envelope(
        "support_export:portable_bundle:test",
        "2026-05-18T17:00:00Z",
    )
    .expect("envelope compiles");
    let json = serde_json::to_string(&envelope).expect("envelope serializes");
    let parsed: PortableBundleSupportExportEnvelope =
        serde_json::from_str(&json).expect("envelope round-trips");
    assert_eq!(parsed, envelope);
}
