//! Protected tests for beta route/exposure support-export parity.

use std::collections::BTreeSet;

use aureline_support::{
    audit_route_exposure_matrix, current_route_exposure_matrix, validate_route_exposure_matrix,
    RouteExposureSupportExport, ROUTE_EXPOSURE_MATRIX_RECORD_KIND,
    ROUTE_EXPOSURE_MATRIX_SCHEMA_VERSION,
};

#[test]
fn route_exposure_matrix_validates_and_covers_required_origins() {
    let matrix = current_route_exposure_matrix().expect("matrix parses");
    validate_route_exposure_matrix(&matrix).expect("matrix validates");
    assert_eq!(matrix.record_kind, ROUTE_EXPOSURE_MATRIX_RECORD_KIND);
    assert_eq!(matrix.schema_version, ROUTE_EXPOSURE_MATRIX_SCHEMA_VERSION);

    let origins = matrix
        .rows
        .iter()
        .map(|row| row.origin.origin_class.as_str())
        .collect::<BTreeSet<_>>();
    for required in [
        "local_desktop",
        "remote_helper",
        "managed_workspace",
        "browser_companion",
        "provider_linked_context",
        "embedded_docs_help_webview",
        "headless_cli",
    ] {
        assert!(origins.contains(required), "missing origin {required}");
    }
}

#[test]
fn route_exposure_support_export_is_metadata_only() {
    let matrix = current_route_exposure_matrix().expect("matrix parses");
    let export = RouteExposureSupportExport::from_matrix(
        "support-export:route-exposure:test",
        "2026-05-18T00:00:00Z",
        matrix,
    );
    assert_eq!(export.row_count, 12);
    assert_eq!(export.high_risk_row_count, 12);
    assert!(export.findings.is_empty());
    assert!(export.raw_urls_excluded);
    assert!(export.raw_tokens_excluded);
    assert!(export.raw_provider_payloads_excluded);

    let json = serde_json::to_string(&export).expect("serialize support export");
    assert!(!json.contains("https://"));
    assert!(!json.contains("Bearer "));
    assert!(!json.contains("ssh://"));
}

#[test]
fn high_risk_unknown_exposure_is_a_defect() {
    let mut matrix = current_route_exposure_matrix().expect("matrix parses");
    matrix.rows[0].exposure.action_exposure_class = "exposure_unknown_requires_review".into();
    let findings = audit_route_exposure_matrix(&matrix);
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id == "row.high_risk_unknown_exposure"),
        "expected unknown high-risk exposure defect: {findings:#?}",
    );
}

#[test]
fn browser_handoff_without_packet_is_a_defect() {
    let mut matrix = current_route_exposure_matrix().expect("matrix parses");
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| row.row_id == "route-exposure:provider-comment-browser-handoff")
        .expect("provider comment row");
    row.handoff.browser_handoff_packet_ref = None;
    let findings = audit_route_exposure_matrix(&matrix);
    assert!(
        findings
            .iter()
            .any(|finding| finding.check_id == "row.handoff_packet_missing"),
        "expected missing handoff packet defect: {findings:#?}",
    );
}
