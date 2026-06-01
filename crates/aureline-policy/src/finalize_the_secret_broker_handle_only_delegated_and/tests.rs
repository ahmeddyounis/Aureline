use super::*;

fn page() -> FinalizeSecretBrokerPage {
    seeded_finalize_secret_broker_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(
        page.defects.len(),
        0,
        "seeded page must have zero defects; got: {:?}",
        page.defects
            .iter()
            .map(|d| d.narrow_reason_token.as_str())
            .collect::<Vec<_>>()
    );
    assert!(validate_finalize_secret_broker_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        FinalizeSecretBrokerQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_embeds_clean_beta_page() {
    let page = page();
    assert_eq!(page.beta_page.defects.len(), 0);
    assert!(
        aureline_auth::secret_broker::validate_secret_broker_beta_page(&page.beta_page).is_ok()
    );
}

#[test]
fn seeded_page_covers_all_five_required_flow_classes() {
    let page = page();
    assert!(
        page.all_required_flow_classes_covered(),
        "All five required flow classes must be covered; missing: {:?}",
        SecretBrokerFlowClass::ALL
            .iter()
            .filter(|fc| !page.rows.iter().any(|r| r.flow_class_token == fc.as_str()))
            .map(|fc| fc.as_str())
            .collect::<Vec<_>>()
    );
}

#[test]
fn seeded_page_has_delegated_and_session_only_rows_with_explicit_rotation_notes() {
    let page = page();
    assert!(page.delegated_and_session_only_rows_have_explicit_rotation_notes());
}

#[test]
fn seeded_page_remembered_approvals_are_narrow() {
    let page = page();
    assert!(page.remembered_approvals_are_narrow());
}

#[test]
fn seeded_page_no_raw_secret_material() {
    let page = page();
    assert!(page.no_raw_secret_material());
}

#[test]
fn seeded_page_rows_are_all_stable() {
    let page = page();
    assert!(!page.rows.is_empty());
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            FinalizeSecretBrokerQualificationClass::Stable.as_str(),
            "row '{}' must qualify stable; got '{}'",
            row.row_id,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            FinalizeSecretBrokerNarrowReasonClass::NotNarrowed.as_str(),
            "row '{}' must have not_narrowed reason",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_summary_counts_match_rows() {
    let page = page();
    assert_eq!(page.summary.row_count, page.rows.len());
    assert_eq!(page.summary.stable_row_count, page.rows.len());
    assert_eq!(page.summary.beta_row_count, 0);
    assert_eq!(page.summary.withdrawn_row_count, 0);
    assert_eq!(page.summary.beta_page_defect_count, 0);
}

#[test]
fn seeded_page_has_delegated_and_session_only_rows_in_summary() {
    let page = page();
    assert!(
        page.summary.delegated_identity_row_count > 0,
        "must have at least one delegated-identity row"
    );
    assert!(
        page.summary.session_only_row_count > 0,
        "must have at least one session-only row"
    );
}

#[test]
fn seeded_page_remembered_approvals_are_counted_in_summary() {
    let page = page();
    let expected: usize = page.rows.iter().map(|r| r.remembered_approvals.len()).sum();
    assert_eq!(page.summary.remembered_approval_count, expected);
    assert!(page.summary.remembered_approval_count > 0);
}

#[test]
fn support_export_wraps_clean_page() {
    let page = page();
    let export = FinalizeSecretBrokerSupportExport::from_page(
        "policy:finalize-secret-broker:export:stable:0001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_secret_values_excluded);
    assert!(export.raw_handle_ids_excluded);
    assert!(export.remembered_approval_lineage_preserved);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(export.page.summary.withdrawn_row_count, 0);
}

#[test]
fn raw_secret_material_present_withdraws_row() {
    let mut page = seeded_finalize_secret_broker_page();
    // Inject a raw-secret row.
    page.rows[0].raw_secret_material_excluded = false;
    let defects = audit_finalize_secret_broker_rows(&page.rows, &page.beta_page);
    assert!(
        defects
            .iter()
            .any(|d| d.narrow_reason == FinalizeSecretBrokerNarrowReasonClass::RawSecretMaterialPresent),
        "RawSecretMaterialPresent defect expected"
    );
}

#[test]
fn literal_flattening_detected_withdraws_row() {
    let mut page = seeded_finalize_secret_broker_page();
    page.rows[0].no_literal_flattening = false;
    let defects = audit_finalize_secret_broker_rows(&page.rows, &page.beta_page);
    assert!(
        defects
            .iter()
            .any(|d| d.narrow_reason == FinalizeSecretBrokerNarrowReasonClass::LiteralFlatteningDetected),
        "LiteralFlatteningDetected defect expected"
    );
}

#[test]
fn missing_handle_class_on_stable_claim_withdraws() {
    let mut page = seeded_finalize_secret_broker_page();
    // Replace a row's handle class with Missing while keeping stable qualification.
    let row = &mut page.rows[0];
    row.handle_class = SecretBrokerHandleClass::Missing;
    row.handle_class_token = SecretBrokerHandleClass::Missing.as_str().to_owned();
    row.qualification_token = FinalizeSecretBrokerQualificationClass::Stable.as_str().to_owned();
    let defects = audit_finalize_secret_broker_rows(&page.rows, &page.beta_page);
    assert!(
        defects
            .iter()
            .any(|d| d.narrow_reason == FinalizeSecretBrokerNarrowReasonClass::MissingHandleOnStableClaim),
        "MissingHandleOnStableClaim defect expected"
    );
}

#[test]
fn generic_rotation_note_triggers_defect_on_delegated_row() {
    let mut page = seeded_finalize_secret_broker_page();
    let delegated_row = page
        .rows
        .iter_mut()
        .find(|r| r.handle_class == SecretBrokerHandleClass::DelegatedIdentity)
        .expect("delegated row must exist");
    delegated_row.rotation_state.rotation_note = "reconnect".to_owned();
    let defects = audit_finalize_secret_broker_rows(&page.rows, &page.beta_page);
    assert!(
        defects
            .iter()
            .any(|d| d.narrow_reason == FinalizeSecretBrokerNarrowReasonClass::RotationNoteIsGeneric),
        "RotationNoteIsGeneric defect expected for delegated row with generic note"
    );
}

#[test]
fn narrow_approval_missing_actor_triggers_defect() {
    let mut page = seeded_finalize_secret_broker_page();
    let row_with_approvals = page
        .rows
        .iter_mut()
        .find(|r| !r.remembered_approvals.is_empty())
        .expect("row with remembered approvals must exist");
    row_with_approvals.remembered_approvals[0].actor_ref = String::new();
    let defects = audit_finalize_secret_broker_rows(&page.rows, &page.beta_page);
    assert!(
        defects
            .iter()
            .any(|d| d.narrow_reason == FinalizeSecretBrokerNarrowReasonClass::RememberedApprovalNotNarrow),
        "RememberedApprovalNotNarrow defect expected"
    );
}

#[test]
fn revocation_trigger_missing_when_rotation_invalidates_decisions() {
    let mut page = seeded_finalize_secret_broker_page();
    let session_row = page
        .rows
        .iter_mut()
        .find(|r| r.handle_class == SecretBrokerHandleClass::SessionOnly)
        .expect("session-only row must exist");
    // Ensure the rotation event invalidates decisions and the approval lacks a trigger.
    session_row.rotation_state.rotation_event = CredentialRotationEventClass::HandleExpired;
    session_row.rotation_state.rotation_event_token =
        CredentialRotationEventClass::HandleExpired.as_str().to_owned();
    if !session_row.remembered_approvals.is_empty() {
        session_row.remembered_approvals[0].revocation_trigger_token = String::new();
    }
    let defects = audit_finalize_secret_broker_rows(&page.rows, &page.beta_page);
    assert!(
        defects
            .iter()
            .any(|d| d.narrow_reason == FinalizeSecretBrokerNarrowReasonClass::RevocationTriggerMissing),
        "RevocationTriggerMissing defect expected when rotation invalidates remembered decisions"
    );
}

#[test]
fn missing_required_flow_class_triggers_coverage_gap_defect() {
    let page = seeded_finalize_secret_broker_page();
    // Remove the managed_runtime row to create a coverage gap.
    let rows_without_managed_runtime: Vec<_> = page
        .rows
        .clone()
        .into_iter()
        .filter(|r| r.flow_class != SecretBrokerFlowClass::ManagedRuntime)
        .collect();
    let defects = audit_finalize_secret_broker_rows(&rows_without_managed_runtime, &page.beta_page);
    assert!(
        defects
            .iter()
            .any(|d| d.narrow_reason == FinalizeSecretBrokerNarrowReasonClass::FlowClassCoverageGap),
        "FlowClassCoverageGap defect expected when managed_runtime rows are removed"
    );
}

#[test]
fn qualification_class_stable_checks() {
    assert!(FinalizeSecretBrokerQualificationClass::Stable.is_stable());
    assert!(!FinalizeSecretBrokerQualificationClass::Beta.is_stable());
    assert!(!FinalizeSecretBrokerQualificationClass::Withdrawn.is_stable());
    assert!(FinalizeSecretBrokerQualificationClass::Stable.is_claimable());
    assert!(FinalizeSecretBrokerQualificationClass::Beta.is_claimable());
    assert!(!FinalizeSecretBrokerQualificationClass::Withdrawn.is_claimable());
    assert!(!FinalizeSecretBrokerQualificationClass::Preview.is_claimable());
}

#[test]
fn narrow_reason_withdrawal_sentinel_checks() {
    assert!(FinalizeSecretBrokerNarrowReasonClass::LiteralFlatteningDetected.is_withdrawal_reason());
    assert!(FinalizeSecretBrokerNarrowReasonClass::MissingHandleOnStableClaim.is_withdrawal_reason());
    assert!(FinalizeSecretBrokerNarrowReasonClass::RawSecretMaterialPresent.is_withdrawal_reason());
    assert!(!FinalizeSecretBrokerNarrowReasonClass::BetaPageHasDefects.is_withdrawal_reason());
    assert!(!FinalizeSecretBrokerNarrowReasonClass::FlowClassCoverageGap.is_withdrawal_reason());
    assert!(!FinalizeSecretBrokerNarrowReasonClass::NotNarrowed.is_withdrawal_reason());
}

#[test]
fn handle_class_degraded_checks() {
    assert!(SecretBrokerHandleClass::SessionOnly.is_degraded());
    assert!(SecretBrokerHandleClass::Missing.is_degraded());
    assert!(!SecretBrokerHandleClass::OsKeychain.is_degraded());
    assert!(!SecretBrokerHandleClass::EnterpriseVault.is_degraded());
    assert!(!SecretBrokerHandleClass::DelegatedIdentity.is_degraded());
    assert!(!SecretBrokerHandleClass::WorkspaceVariable.is_degraded());
}

#[test]
fn rotation_event_invalidates_decisions_checks() {
    assert!(CredentialRotationEventClass::RotatedByIssuer.invalidates_remembered_decisions());
    assert!(CredentialRotationEventClass::VaultKeychainLoss.invalidates_remembered_decisions());
    assert!(CredentialRotationEventClass::HandleExpired.invalidates_remembered_decisions());
    assert!(CredentialRotationEventClass::ApprovalRevoked.invalidates_remembered_decisions());
    assert!(!CredentialRotationEventClass::BrowserDeviceCodeRenewal.invalidates_remembered_decisions());
    assert!(!CredentialRotationEventClass::NoRotationRequired.invalidates_remembered_decisions());
}

#[test]
fn rotation_state_explicit_note_check() {
    let explicit = CredentialRotationState {
        rotation_event: CredentialRotationEventClass::NoRotationRequired,
        rotation_event_token: "no_rotation_required".to_owned(),
        rotation_note: "Handle is live within freshness window.".to_owned(),
        expires_at: String::new(),
        invalidates_remembered_decisions: false,
        reapproval_path_available: true,
        replay_posture_token: "replay_safe".to_owned(),
    };
    assert!(explicit.rotation_note_is_explicit());

    let generic = CredentialRotationState {
        rotation_note: "reconnect".to_owned(),
        ..explicit.clone()
    };
    assert!(!generic.rotation_note_is_explicit());

    let empty = CredentialRotationState {
        rotation_note: String::new(),
        ..explicit
    };
    assert!(!empty.rotation_note_is_explicit());
}
