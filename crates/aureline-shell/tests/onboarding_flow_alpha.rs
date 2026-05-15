use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::{IdentityModeAlias, IdentityModeBaselinePacket};
use aureline_shell::onboarding::{
    OnboardingFlow, OnboardingFlowEntry, OnboardingFlowPersistedRecordKind, OnboardingFlowRequest,
    OnboardingFlowStageKind, OnboardingIdentityModeChoice, OnboardingImportFlowRequest,
    RollbackCheckpointConfirmationState,
};
use aureline_telemetry::onboarding::OnboardingEventName;
use aureline_workspace::{ImportApplyDecision, WideningVector};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn identity_packet() -> IdentityModeBaselinePacket {
    let path = repo_root().join("fixtures/auth/identity_mode_alpha/baseline_all_modes.json");
    let payload = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("identity fixture {} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("identity fixture {} must parse: {err}", path.display()))
}

fn import_source() -> String {
    repo_root()
        .join("fixtures/import/m1_classifier_cases/vscode_workspace")
        .display()
        .to_string()
}

fn stage_kinds(flow: &OnboardingFlow) -> Vec<OnboardingFlowStageKind> {
    flow.sequence.iter().map(|stage| stage.stage_kind).collect()
}

#[test]
fn no_account_fast_path_reaches_landing_without_identity_persistence() {
    let flow = OnboardingFlow::build(OnboardingFlowRequest::new(
        "flow:onboarding.no-account.local-open",
        identity_packet(),
        OnboardingIdentityModeChoice::NoAccountFastPath,
        OnboardingFlowEntry::local_folder("~/Code/example"),
    ))
    .expect("flow builds");

    assert!(flow.reaches_first_useful_work_landing());
    assert!(!flow.has_persisted_identity_choice());
    assert!(flow.persisted_records.iter().all(|record| {
        record.record_kind != OnboardingFlowPersistedRecordKind::IdentityModeChoice
    }));
    assert_eq!(
        flow.rollback_checkpoint_confirmation.confirmation_state,
        RollbackCheckpointConfirmationState::NotRequiredNoImport
    );
    assert_eq!(
        flow.first_useful_work_landing.landing_surface,
        "explorer_plus_readme_or_changed_files"
    );
    assert_eq!(
        stage_kinds(&flow),
        vec![
            OnboardingFlowStageKind::IdentityModeChoice,
            OnboardingFlowStageKind::ImportDiffReview,
            OnboardingFlowStageKind::RollbackCheckpointConfirmation,
            OnboardingFlowStageKind::FirstUsefulWorkLanding,
        ]
    );
    assert!(flow
        .learning_tour_step_refs
        .contains(&"step:aureline.entry.open-folder".to_string()));
    assert!(!flow
        .learning_tour_step_refs
        .contains(&"step:aureline.import.preview-before-apply".to_string()));
}

#[test]
fn explicit_managed_identity_choice_is_recorded_before_landing() {
    let flow = OnboardingFlow::build(OnboardingFlowRequest::new(
        "flow:onboarding.managed.local-open",
        identity_packet(),
        OnboardingIdentityModeChoice::ChosenMode {
            identity_mode: IdentityModeAlias::ManagedConvenience,
        },
        OnboardingFlowEntry::local_folder("~/Code/example"),
    ))
    .expect("flow builds");

    assert!(flow.reaches_first_useful_work_landing());
    assert!(flow.has_persisted_identity_choice());
    assert_eq!(
        flow.identity_mode_step
            .selected_identity_mode_token
            .as_deref(),
        Some("managed_convenience")
    );
    assert!(flow.persisted_records.iter().any(|record| {
        record.record_kind == OnboardingFlowPersistedRecordKind::IdentityModeChoice
            && record.record_ref.contains("managed_convenience")
    }));
}

#[test]
fn import_branch_reuses_non_widening_review_and_confirms_checkpoint() {
    let flow = OnboardingFlow::build(OnboardingFlowRequest::new(
        "flow:onboarding.import.vscode",
        identity_packet(),
        OnboardingIdentityModeChoice::NoAccountFastPath,
        OnboardingFlowEntry::ImportProfile(OnboardingImportFlowRequest::new(
            import_source(),
            "profile:default",
        )),
    ))
    .expect("flow builds");

    let packet = flow
        .import_diff_review
        .as_ref()
        .expect("import diff review is present");
    assert!(packet.every_row_has_before_after_diff());
    assert!(packet.every_row_uses_one_checkpoint());
    assert!(packet.rollback_checkpoint.clear_pre_apply_checkpoint());

    let review = flow
        .import_apply_review
        .as_ref()
        .expect("non-widening review is present");
    assert!(review.allowed);
    assert_eq!(review.decision, ImportApplyDecision::ApplyAllowed);

    assert_eq!(
        flow.rollback_checkpoint_confirmation.confirmation_state,
        RollbackCheckpointConfirmationState::Confirmed
    );
    assert_eq!(
        flow.rollback_checkpoint_confirmation
            .checkpoint_ref
            .as_deref(),
        Some(packet.rollback_checkpoint.checkpoint_ref.as_str())
    );
    assert!(flow.rollback_checkpoint_confirmation.confirmed_before_apply);
    assert_eq!(
        flow.first_useful_work_landing.landing_surface,
        "import_compare_or_restore_sheet"
    );
    assert!(flow.admission_checkpoint_route.is_contract_valid());
    assert!(flow.persisted_records.iter().any(|record| {
        record.record_kind == OnboardingFlowPersistedRecordKind::ImportedProfileHistory
    }));
    assert!(flow.persisted_records.iter().any(|record| {
        record.record_kind == OnboardingFlowPersistedRecordKind::RollbackCheckpointConfirmation
    }));
    assert!(flow
        .learning_tour_step_refs
        .contains(&"step:aureline.import.preview-before-apply".to_string()));
    assert!(flow
        .telemetry_event_names
        .contains(&OnboardingEventName::MigrationRollbackCheckpointWritten));
}

#[test]
fn widening_import_branch_blocks_apply_before_checkpoint_confirmation() {
    let flow = OnboardingFlow::build(OnboardingFlowRequest::new(
        "flow:onboarding.import.blocked",
        identity_packet(),
        OnboardingIdentityModeChoice::NoAccountFastPath,
        OnboardingFlowEntry::ImportProfile(
            OnboardingImportFlowRequest::new(import_source(), "profile:default")
                .with_widening_vectors(vec![WideningVector::WorkspaceTrust]),
        ),
    ))
    .expect("flow builds");

    let review = flow
        .import_apply_review
        .as_ref()
        .expect("non-widening review is present");
    assert!(!review.allowed);
    assert_eq!(review.decision, ImportApplyDecision::Blocked);
    assert!(review
        .reasons
        .iter()
        .any(|reason| reason.contains("workspace_trust")));

    assert_eq!(
        flow.rollback_checkpoint_confirmation.confirmation_state,
        RollbackCheckpointConfirmationState::BlockedByNonWideningReview
    );
    assert!(!flow.rollback_checkpoint_confirmation.confirmed_before_apply);
    assert_eq!(
        flow.admission_checkpoint_route
            .checkpoint
            .admission_class
            .as_str(),
        "policy_blocked"
    );
    assert_eq!(
        flow.admission_checkpoint_route
            .first_useful_route
            .route_reason_class
            .as_str(),
        "policy_or_trust_narrowing"
    );
    assert_eq!(
        flow.first_useful_work_landing.landing_surface,
        "import_compare_or_restore_sheet"
    );
    assert!(flow.admission_checkpoint_route.is_contract_valid());
    assert!(!flow
        .telemetry_event_names
        .contains(&OnboardingEventName::MigrationRollbackCheckpointWritten));
}
