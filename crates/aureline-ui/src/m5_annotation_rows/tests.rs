use super::*;

fn seeded() -> AnnotationRow {
    current_m5_annotation_row().expect("canonical annotation row loads and validates")
}

fn cloned() -> AnnotationRow {
    seeded()
}

#[test]
fn checked_in_annotation_row_validates_clean() {
    let row = seeded();
    assert_eq!(row.record_kind, M5_ANNOTATION_ROW_RECORD_KIND);
    assert_eq!(row.schema_version, M5_ANNOTATION_ROW_SCHEMA_VERSION);
    assert!(row.validate().is_empty(), "{:?}", row.validate());
}

#[test]
fn row_discloses_provider_scanner_anchor_and_open_details() {
    let row = seeded();
    assert_eq!(row.source_provider.provider_label, "github_actions");
    assert_eq!(row.source_provider.provider_kind, "security_scanner");
    assert_eq!(row.source_provider.scanner_label, "cargo_deny");
    assert!(!row.source_provider.raw_provider_dump_included);
    assert_eq!(row.anchor.anchor_kind, "manifest");
    assert_eq!(row.anchor.anchor_ref, row.anchor_ref);
    assert_eq!(row.open_details_action.label, "open_details");
    assert!(row.open_details_action.enabled);
}

#[test]
fn stale_row_preserves_handoff_instead_of_retargeting() {
    let row = seeded();
    assert!(row.requires_stale_handoff());
    assert_eq!(row.freshness_state, "stale");
    assert_eq!(row.degraded_state, "stale");
    assert_eq!(row.stale_handoff.state, "stale");
    assert_eq!(row.stale_handoff.reason, "manifest_changed");
    assert_eq!(row.stale_handoff.previous_anchor_ref, row.anchor_ref);
    assert_ne!(row.stale_handoff.successor_anchor_ref, row.anchor_ref);
    assert!(row.stale_handoff.review_required);
    assert!(row.stale_handoff.silent_retarget_prohibited);
}

#[test]
fn projections_reuse_one_contract_for_code_review_health_and_support() {
    let row = seeded();
    let code = row
        .projection_for("code_surface")
        .expect("code-surface projection exists");
    let review = row
        .projection_for("review_pane")
        .expect("review pane projection exists");
    let health = row
        .projection_for("project_health_center")
        .expect("health projection exists");
    let support = row
        .projection_for("support_export")
        .expect("support projection exists");

    for projection in [&code, &review, &health, &support] {
        assert_eq!(projection.row_id, row.row_id);
        assert_eq!(
            projection.provider_label,
            row.source_provider.provider_label
        );
        assert_eq!(projection.scanner_label, row.source_provider.scanner_label);
        assert_eq!(projection.anchor_kind, row.anchor.anchor_kind);
        assert_eq!(projection.anchor_ref, row.anchor_ref);
        assert_eq!(projection.severity, row.severity);
        assert_eq!(projection.confidence, row.confidence);
        assert_eq!(projection.freshness_state, row.freshness_state);
        assert_eq!(projection.stale_handoff_state, row.stale_handoff.state);
        assert_eq!(projection.stale_handoff_reason, row.stale_handoff.reason);
        assert_eq!(
            projection.open_details_action_id,
            row.open_details_action.action_id
        );
    }
}

#[test]
fn export_copy_preserves_anchor_and_provenance_truth() {
    let row = seeded();
    let exported = row.export_safe_json();
    for required in [
        "source_provider",
        "github_actions",
        "cargo_deny",
        "anchor",
        "manifest",
        "stale_handoff",
        "manifest_changed",
        "open_details_action",
    ] {
        assert!(exported.contains(required), "export dropped {required}");
    }
    for forbidden in ["api_key", "password", "bearer ", "raw provider payload"] {
        assert!(!exported.to_lowercase().contains(forbidden));
    }
}

#[test]
fn stale_row_without_handoff_fails_validation() {
    let mut row = cloned();
    row.stale_handoff.reason = "not_applicable".to_owned();
    row.stale_handoff.review_required = false;
    row.stale_handoff.silent_retarget_prohibited = false;
    assert!(
        row.validate()
            .contains(&AnnotationRowViolation::StaleHandoffIncomplete),
        "{:?}",
        row.validate()
    );
}

#[test]
fn provider_raw_dump_fails_validation() {
    let mut row = cloned();
    row.source_provider.raw_provider_dump_included = true;
    assert!(
        row.validate()
            .contains(&AnnotationRowViolation::MissingProviderDisclosure),
        "{:?}",
        row.validate()
    );
}

#[test]
fn copy_export_that_drops_anchor_truth_fails_validation() {
    let mut row = cloned();
    row.copy_export
        .export_fields
        .retain(|f| f != "stale_handoff");
    assert!(
        row.validate()
            .contains(&AnnotationRowViolation::CopyExportDropsAnchorTruth),
        "{:?}",
        row.validate()
    );
}
