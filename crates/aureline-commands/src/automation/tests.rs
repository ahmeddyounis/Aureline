use super::*;

const PACKET_ID: &str = "safe-automation-qualification:stable-labels:0001";

fn backing_manifest_fields() -> Vec<String> {
    [
        "storage_form",
        "required_capabilities",
        "trust_requirement",
        "preview_policy",
        "idempotency_hint",
        "provenance",
        "lifecycle_label",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn manifest_rows() -> Vec<AutomationClassManifestRow> {
    vec![
        AutomationClassManifestRow {
            object_class: AutomationObjectClass::RecordedMacro,
            storage_form: AutomationStorageFormClass::LocalUserArtifact,
            required_capabilities: vec![AutomationCapabilityClass::EditorReviewStateReplay],
            trust_requirement: AutomationTrustRequirementClass::LocalOnly,
            preview_policy: AutomationPreviewPolicyClass::CheckpointRequiredBeforeReplay,
            idempotency_hint: AutomationIdempotencyHintClass::DeterministicGivenSameTargetContext,
            provenance: AutomationProvenanceClass::UserRecorded,
            lifecycle_label: AutomationLifecycleLabelClass::StableQualified,
            artifact_authority: AutomationArtifactAuthorityClass::LocalOnly,
            scope_narrower_than_stable: true,
            deterministic_replay_boundary: true,
            forbids_raw_secret_capture: true,
            forbids_undeclared_network_or_process_access: true,
            forbids_hidden_authority: true,
            edit_history_checkpoint_required: true,
            claimed_stable: true,
            proof_or_dependency_ref: None,
        },
        AutomationClassManifestRow {
            object_class: AutomationObjectClass::WorkspaceRecipe,
            storage_form: AutomationStorageFormClass::VersionedTextManifest,
            required_capabilities: vec![
                AutomationCapabilityClass::WorkspaceRead,
                AutomationCapabilityClass::FilesystemWrite,
            ],
            trust_requirement: AutomationTrustRequirementClass::TrustedWorkspace,
            preview_policy: AutomationPreviewPolicyClass::DryRunRequiredBeforeApply,
            idempotency_hint: AutomationIdempotencyHintClass::RetryRequiresFreshContextResolution,
            provenance: AutomationProvenanceClass::WorkspaceAuthored,
            lifecycle_label: AutomationLifecycleLabelClass::StableLabelsOnlyNarrowedRunner,
            artifact_authority: AutomationArtifactAuthorityClass::LocalOnly,
            scope_narrower_than_stable: true,
            deterministic_replay_boundary: true,
            forbids_raw_secret_capture: true,
            forbids_undeclared_network_or_process_access: true,
            forbids_hidden_authority: true,
            edit_history_checkpoint_required: false,
            claimed_stable: false,
            proof_or_dependency_ref: Some("dependency:recipe-runner-preview-proof".to_owned()),
        },
        AutomationClassManifestRow {
            object_class: AutomationObjectClass::ExtensionRecipe,
            storage_form: AutomationStorageFormClass::SignedExtensionPackageManifest,
            required_capabilities: vec![
                AutomationCapabilityClass::ExtensionInvocation,
                AutomationCapabilityClass::WorkspaceRead,
            ],
            trust_requirement: AutomationTrustRequirementClass::ExtensionPermissionEnvelope,
            preview_policy: AutomationPreviewPolicyClass::ImpactSummaryRequired,
            idempotency_hint: AutomationIdempotencyHintClass::RetryRequiresFreshContextResolution,
            provenance: AutomationProvenanceClass::ExtensionPublisher,
            lifecycle_label: AutomationLifecycleLabelClass::DependencyGated,
            artifact_authority: AutomationArtifactAuthorityClass::SignedArtifact,
            scope_narrower_than_stable: true,
            deterministic_replay_boundary: true,
            forbids_raw_secret_capture: true,
            forbids_undeclared_network_or_process_access: true,
            forbids_hidden_authority: true,
            edit_history_checkpoint_required: false,
            claimed_stable: false,
            proof_or_dependency_ref: Some("dependency:extension-permission-proof".to_owned()),
        },
        AutomationClassManifestRow {
            object_class: AutomationObjectClass::AdminCuratedRecipePack,
            storage_form: AutomationStorageFormClass::SignedPolicyBundle,
            required_capabilities: vec![
                AutomationCapabilityClass::WorkspaceRead,
                AutomationCapabilityClass::AdminPolicyMutation,
            ],
            trust_requirement: AutomationTrustRequirementClass::PolicyProvided,
            preview_policy: AutomationPreviewPolicyClass::DisplayablePlanRequiredBeforeMutation,
            idempotency_hint: AutomationIdempotencyHintClass::RetryRequiresReconciliationReceipt,
            provenance: AutomationProvenanceClass::AdminCuratedSigner,
            lifecycle_label: AutomationLifecycleLabelClass::DependencyGated,
            artifact_authority: AutomationArtifactAuthorityClass::PolicyProvided,
            scope_narrower_than_stable: true,
            deterministic_replay_boundary: true,
            forbids_raw_secret_capture: true,
            forbids_undeclared_network_or_process_access: true,
            forbids_hidden_authority: true,
            edit_history_checkpoint_required: false,
            claimed_stable: false,
            proof_or_dependency_ref: Some("dependency:policy-bundle-proof".to_owned()),
        },
        AutomationClassManifestRow {
            object_class: AutomationObjectClass::EphemeralAiGeneratedRecipe,
            storage_form: AutomationStorageFormClass::TransientGeneratedPlan,
            required_capabilities: vec![
                AutomationCapabilityClass::AiToolInvocation,
                AutomationCapabilityClass::WorkspaceRead,
            ],
            trust_requirement: AutomationTrustRequirementClass::TrustedWorkspace,
            preview_policy: AutomationPreviewPolicyClass::DisplayablePlanRequiredBeforeMutation,
            idempotency_hint: AutomationIdempotencyHintClass::NonIdempotentRequiresExplicitReview,
            provenance: AutomationProvenanceClass::AiEvidencePacket,
            lifecycle_label: AutomationLifecycleLabelClass::LabsOnly,
            artifact_authority: AutomationArtifactAuthorityClass::SupportProjectionOnly,
            scope_narrower_than_stable: true,
            deterministic_replay_boundary: true,
            forbids_raw_secret_capture: true,
            forbids_undeclared_network_or_process_access: true,
            forbids_hidden_authority: true,
            edit_history_checkpoint_required: false,
            claimed_stable: false,
            proof_or_dependency_ref: Some("labs:generated-recipe-save-disabled".to_owned()),
        },
    ]
}

fn surface_contracts() -> Vec<CommandAutomationSurfaceContract> {
    vec![
        CommandAutomationSurfaceContract {
            action_class: AutomationSurfaceActionClass::AddToRecipe,
            required_labels: vec![ControlledAutomationLabel::RecipeSafe],
            backing_manifest_fields: backing_manifest_fields(),
            lifecycle_ceiling: AutomationLifecycleLabelClass::StableLabelsOnlyNarrowedRunner,
            insertion_or_inspection_only: true,
            consumes_command_descriptor: true,
            consumes_preview_policy: true,
            no_saved_artifact_captures_secrets: true,
            no_undeclared_network_or_process_access: true,
            no_hidden_authority: true,
            deterministic_replay_boundary_preserved: true,
            claimed_stable: true,
        },
        CommandAutomationSurfaceContract {
            action_class: AutomationSurfaceActionClass::InspectDescriptor,
            required_labels: vec![],
            backing_manifest_fields: backing_manifest_fields(),
            lifecycle_ceiling: AutomationLifecycleLabelClass::StableQualified,
            insertion_or_inspection_only: true,
            consumes_command_descriptor: true,
            consumes_preview_policy: true,
            no_saved_artifact_captures_secrets: true,
            no_undeclared_network_or_process_access: true,
            no_hidden_authority: true,
            deterministic_replay_boundary_preserved: true,
            claimed_stable: true,
        },
        CommandAutomationSurfaceContract {
            action_class: AutomationSurfaceActionClass::ReplayAsMacro,
            required_labels: vec![ControlledAutomationLabel::MacroSafe],
            backing_manifest_fields: backing_manifest_fields(),
            lifecycle_ceiling: AutomationLifecycleLabelClass::StableQualified,
            insertion_or_inspection_only: false,
            consumes_command_descriptor: true,
            consumes_preview_policy: true,
            no_saved_artifact_captures_secrets: true,
            no_undeclared_network_or_process_access: true,
            no_hidden_authority: true,
            deterministic_replay_boundary_preserved: true,
            claimed_stable: true,
        },
    ]
}

fn packet() -> SafeAutomationQualificationPacket {
    SafeAutomationQualificationPacket::new(SafeAutomationQualificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        claimed_stable_label_truth: true,
        policy_epoch_ref: "policy-epoch:safe-automation:0001".to_owned(),
        controlled_labels: ControlledAutomationLabel::required_coverage().to_vec(),
        automation_classes: manifest_rows(),
        surface_contracts: surface_contracts(),
        export_import_contract: AutomationManifestExportImportContract {
            distinguishes_local_signed_policy_artifacts: true,
            support_safe_projection_only: true,
            strips_raw_secrets_and_prompts: true,
            preserves_deterministic_replay_boundaries: true,
            import_revalidates_trust_and_policy: true,
            preserves_manifest_identity_refs: true,
            prevents_authority_downgrade_on_import: true,
        },
        evidence_export: SafeAutomationEvidenceExport {
            evidence_id: "automation-evidence:safe-automation:stable-labels:0001".to_owned(),
            support_export_ref: SAFE_AUTOMATION_SUPPORT_EXPORT_REF.to_owned(),
            matrix_ref: SAFE_AUTOMATION_MATRIX_REF.to_owned(),
            schema_ref: SAFE_AUTOMATION_MANIFEST_SCHEMA_REF.to_owned(),
            preview_lifecycle_doc_ref: SAFE_AUTOMATION_PREVIEW_LIFECYCLE_DOC_REF.to_owned(),
        },
        source_contract_refs: vec![
            SAFE_AUTOMATION_PREVIEW_LIFECYCLE_DOC_REF.to_owned(),
            SAFE_AUTOMATION_MANIFEST_SCHEMA_REF.to_owned(),
            SAFE_AUTOMATION_RECIPE_MACRO_CONTRACT_REF.to_owned(),
            SAFE_AUTOMATION_SHAREABILITY_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-06T00:00:00Z".to_owned(),
    })
}

#[test]
fn safe_automation_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn display_labels_use_controlled_terms() {
    let labels = vec![
        "macro_safe".to_owned(),
        "recipe_safe".to_owned(),
        "headless_safe".to_owned(),
        "approval_required".to_owned(),
    ];
    assert_eq!(
        automation_display_labels(&labels),
        vec![
            "Macro-safe".to_owned(),
            "Recipe-safe".to_owned(),
            "Headless-safe".to_owned(),
            "Approval required".to_owned(),
        ]
    );
}

#[test]
fn why_not_automatable_prefers_exact_blocker() {
    let labels = vec!["ui_only".to_owned()];
    assert_eq!(
        why_not_automatable_reason(&labels, "no_approval_required"),
        Some("ui_only".to_owned())
    );

    let labels = vec!["recipe_safe".to_owned(), "headless_safe".to_owned()];
    assert_eq!(
        why_not_automatable_reason(&labels, "explicit_confirmation_required"),
        Some("approval_required".to_owned())
    );
}

#[test]
fn missing_label_coverage_is_rejected() {
    let mut packet = packet();
    packet
        .controlled_labels
        .retain(|label| *label != ControlledAutomationLabel::RemoteMutation);
    assert!(packet
        .validate()
        .contains(&SafeAutomationQualificationViolation::MissingControlledLabelCoverage));
}

#[test]
fn recorded_macro_cannot_claim_process_access() {
    let mut packet = packet();
    packet.automation_classes[0]
        .required_capabilities
        .push(AutomationCapabilityClass::ProcessLaunch);
    assert!(packet
        .validate()
        .contains(&SafeAutomationQualificationViolation::RecordedMacroTooBroad));
}

#[test]
fn broader_recipe_cannot_claim_stable() {
    let mut packet = packet();
    packet.automation_classes[1].claimed_stable = true;
    assert!(packet
        .validate()
        .contains(&SafeAutomationQualificationViolation::BroadAutomationClaimsStable));
}

#[test]
fn add_to_recipe_requires_recipe_safe() {
    let mut packet = packet();
    packet.surface_contracts[0].required_labels.clear();
    assert!(packet
        .validate()
        .contains(&SafeAutomationQualificationViolation::AddToRecipeNotGatedByRecipeSafe));
}

#[test]
fn checked_export_validates() {
    let packet = current_safe_automation_qualification_export()
        .expect("checked safe automation export validates");
    assert!(packet.validate().is_empty());
}
