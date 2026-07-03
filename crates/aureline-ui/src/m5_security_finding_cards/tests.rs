use super::*;

fn seeded() -> SecurityFindingCard {
    current_m5_security_finding_card().expect("canonical security finding card loads and validates")
}

fn cloned() -> SecurityFindingCard {
    seeded()
}

#[test]
fn checked_in_security_finding_card_validates_clean() {
    let card = seeded();
    assert_eq!(card.record_kind, M5_SECURITY_FINDING_CARD_RECORD_KIND);
    assert_eq!(card.schema_version, M5_SECURITY_FINDING_CARD_SCHEMA_VERSION);
    assert!(card.validate().is_empty(), "{:?}", card.validate());
}

#[test]
fn card_separates_finding_class_scope_severity_confidence_and_freshness() {
    let card = seeded();
    assert_eq!(card.finding_class, "package");
    assert_eq!(card.affected_scope.scope_class, "package");
    assert!(card
        .affected_scope
        .package_refs
        .contains(&"pkg:cargo:openssl-sys".to_owned()));
    assert_eq!(card.severity, "critical");
    assert_eq!(card.confidence, "high");
    assert_eq!(card.freshness_state, "no_fix_yet");
}

#[test]
fn fix_availability_is_not_collapsed_into_remediation() {
    let card = seeded();
    assert_eq!(card.fix_availability.state, "no_fix_yet");
    assert_eq!(card.fix_availability.fixed_version_ref, "version:none");
    assert!(!card.fix_availability.can_auto_apply);
    assert!(card.fix_availability.review_required);
    assert!(card.remediation.no_fix_yet);
    assert_eq!(card.remediation.safest_next_step, "apply_mitigation");
    assert!(card.remediation.local_validation.available);
    assert_eq!(
        card.remediation.docs_help_path.docs_ref,
        "docs:security:dependency-remediation:no-fix-yet"
    );
}

#[test]
fn suppression_label_and_audit_actions_remain_exportable() {
    let card = seeded();
    assert_eq!(card.suppression_state.state, "exception_expired");
    assert_eq!(card.suppression_state.display_label, "Exception expired");
    assert!(card.suppression_state.visible_in_export);
    assert!(card
        .audit_actions
        .iter()
        .all(|action| action.included_in_support_export));
    assert!(card
        .audit_actions
        .iter()
        .any(|action| action.label == "export_audit_record"));
}

#[test]
fn projections_reuse_one_contract_for_review_package_health_companion_and_support() {
    let card = seeded();
    let review = card
        .projection_for("review_pane")
        .expect("review projection exists");
    let package = card
        .projection_for("package_manager")
        .expect("package projection exists");
    let health = card
        .projection_for("project_health_center")
        .expect("health projection exists");
    let companion = card
        .projection_for("companion_client")
        .expect("companion projection exists");
    let support = card
        .projection_for("support_export")
        .expect("support projection exists");

    for projection in [&review, &package, &health, &companion, &support] {
        assert_eq!(projection.card_id, card.card_id);
        assert_eq!(projection.finding_id, card.finding_id);
        assert_eq!(projection.finding_class, card.finding_class);
        assert_eq!(projection.scope_class, card.affected_scope.scope_class);
        assert_eq!(projection.severity, card.severity);
        assert_eq!(projection.confidence, card.confidence);
        assert_eq!(projection.freshness_state, card.freshness_state);
        assert_eq!(
            projection.fix_availability_state,
            card.fix_availability.state
        );
        assert_eq!(projection.suppression_state, card.suppression_state.state);
        assert_eq!(
            projection.suppression_display_label,
            card.suppression_state.display_label
        );
        assert_eq!(
            projection.safest_next_step,
            card.remediation.safest_next_step
        );
        assert!(projection.local_validation_available);
        assert_eq!(projection.primary_audit_action, "export_audit_record");
    }
}

#[test]
fn export_copy_preserves_security_finding_truth() {
    let card = seeded();
    let exported = card.export_safe_json();
    for required in [
        "finding:m5:openssl-sys:no-fixed-release",
        "package",
        "scope:package-health:openssl-sys:workspace-member:aureline-deps",
        "critical",
        "high",
        "no_fix_yet",
        "Exception expired",
        "apply_mitigation",
        "validation:cargo-test-with-openssl-feature-disabled",
        "docs:security:dependency-remediation:no-fix-yet",
        "export_audit_record",
    ] {
        assert!(exported.contains(required), "export dropped {required}");
    }
    for forbidden in [
        "api_key",
        "password",
        "bearer ",
        "raw advisory",
        "raw exploit",
    ] {
        assert!(!exported.to_lowercase().contains(forbidden));
    }
}

#[test]
fn hidden_suppression_fails_validation() {
    let mut card = cloned();
    card.suppression_state.visible_in_export = false;
    assert!(
        card.validate()
            .contains(&SecurityFindingCardViolation::SuppressionLabelOrExportMissing),
        "{:?}",
        card.validate()
    );
}

#[test]
fn no_fix_yet_auto_apply_contradiction_fails_validation() {
    let mut card = cloned();
    card.fix_availability.can_auto_apply = true;
    assert!(
        card.validate()
            .contains(&SecurityFindingCardViolation::NoFixYetContradiction),
        "{:?}",
        card.validate()
    );
}

#[test]
fn copy_export_that_drops_audit_actions_fails_validation() {
    let mut card = cloned();
    card.copy_export
        .export_fields
        .retain(|field| field != "audit_actions");
    assert!(
        card.validate()
            .contains(&SecurityFindingCardViolation::CopyExportDropsFindingTruth),
        "{:?}",
        card.validate()
    );
}
