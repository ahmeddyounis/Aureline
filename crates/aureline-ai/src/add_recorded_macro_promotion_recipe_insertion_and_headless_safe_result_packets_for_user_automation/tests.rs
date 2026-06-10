use super::*;

const PACKET_ID: &str = "user-automation:stable:0001";

fn proof_stale(narrowed_to: M5AiWorkflowQualificationClass) -> UserAutomationDowngradeRule {
    UserAutomationDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn provider_unavailable(
    narrowed_to: M5AiWorkflowQualificationClass,
) -> UserAutomationDowngradeRule {
    UserAutomationDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProviderUnavailable,
        narrowed_to,
        auto_enforced: true,
        rationale: "Provider outage or quota exhaustion narrows the claim".to_owned(),
    }
}

fn inspect_step() -> AutomationStepDisclosure {
    AutomationStepDisclosure {
        side_effect_class: ToolSideEffectClass::InspectOnly,
        headless_safety: HeadlessSafetyClass::HeadlessSafeInspectOnly,
        interactive_preview: ReplayPreviewClass::InspectOnlyNoPreviewNeeded,
        approval_posture: ToolApprovalPostureClass::AllowedWithoutPrompt,
        audit: RecipeStepAuditClass::AuditedLocalHistoryOnly,
        reversibility: RecipeStepReversibilityClass::NoSideEffect,
        disclosure_label: "Reads workspace context; no change is applied".to_owned(),
    }
}

/// A promoted editor-state macro whose mutating edit is pre-authorized to run
/// headless under an admin policy grant.
fn headless_edit_macro() -> UserAutomationRow {
    UserAutomationRow {
        macro_id: "format-on-save-macro".to_owned(),
        macro_label: "Format-and-organize-imports macro".to_owned(),
        macro_family_label: "Editor formatting".to_owned(),
        macro_version: "1.2.0".to_owned(),
        capture_content_address: "sha256:format-on-save-v1".to_owned(),
        capture_provenance: MacroCaptureProvenanceClass::RecordedFromUserSession,
        recorded_step_count: 6,
        publisher_source_class: ToolPublisherSourceClass::FirstPartyNativePublisher,
        publisher_identity_ref: "publisher-identity:first-party-recipes".to_owned(),
        resolved_mode: RoutePolicyModeClass::Local,
        promotion: MacroPromotionBlock {
            state: MacroPromotionStateClass::PromotedToRecipe,
            promoted_recipe_ref: "recipe:format-on-save".to_owned(),
            recipe_pack_ref: String::new(),
            promotion_approval: ToolApprovalPostureClass::AllowedWithOneTimePrompt,
            reviewer_identity_ref: "reviewer-identity:workspace-owner".to_owned(),
            first_use_review_required: true,
        },
        insertion: RecipeInsertionBlock {
            target_class: RecipeInsertionTargetClass::AutomationQueueInsertion,
            preview: ReplayPreviewClass::DiffPreviewBeforeReplay,
            approval: ToolApprovalPostureClass::AllowedWithOneTimePrompt,
            insertion_reversible: true,
            insertion_label: "Adds the formatter to the on-save automation queue".to_owned(),
        },
        headless_result: HeadlessResultBlock {
            state: HeadlessResultStateClass::CompletedAllStepsSafe,
            result_content_address: "sha256:format-on-save-result-1".to_owned(),
            steps_total: 2,
            steps_completed: 2,
            steps_deferred: 0,
            steps_blocked: 0,
            audit: RecipeStepAuditClass::AuditedToRunRecordTimeline,
            result_label: "Formatted the workspace headless; every edit was reversible".to_owned(),
        },
        steps: vec![
            inspect_step(),
            AutomationStepDisclosure {
                side_effect_class: ToolSideEffectClass::LocalReversibleEdit,
                headless_safety: HeadlessSafetyClass::HeadlessSafePreauthorizedPolicy,
                interactive_preview: ReplayPreviewClass::DiffPreviewBeforeReplay,
                approval_posture: ToolApprovalPostureClass::AllowedWithOneTimePrompt,
                audit: RecipeStepAuditClass::AuditedToRunRecordTimeline,
                reversibility: RecipeStepReversibilityClass::ReversibleInWorkspace,
                disclosure_label: "Reformats files; pre-authorized, previewed, and reversible"
                    .to_owned(),
            },
        ],
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:format-on-save".to_owned()],
        explanation_label: "Promoted editor macro; reversible edits run headless under policy"
            .to_owned(),
    }
}

/// An imported macro whose irreversible publish is deferred to interactive
/// review when run headless.
fn imported_publish_macro() -> UserAutomationRow {
    UserAutomationRow {
        macro_id: "release-notes-publish-macro".to_owned(),
        macro_label: "Release-notes publish macro".to_owned(),
        macro_family_label: "Release".to_owned(),
        macro_version: "2.1.0".to_owned(),
        capture_content_address: "sha256:release-notes-publish-v2".to_owned(),
        capture_provenance: MacroCaptureProvenanceClass::ImportedFromSharedRecipePack,
        recorded_step_count: 9,
        publisher_source_class: ToolPublisherSourceClass::EnterpriseRegisteredPublisher,
        publisher_identity_ref: "publisher-identity:release-team".to_owned(),
        resolved_mode: RoutePolicyModeClass::Managed,
        promotion: MacroPromotionBlock {
            state: MacroPromotionStateClass::PromotedToRecipe,
            promoted_recipe_ref: "recipe:release-notes-publish".to_owned(),
            recipe_pack_ref: "recipe-pack:byok-issue-publish".to_owned(),
            promotion_approval: ToolApprovalPostureClass::RequiresAdminApproval,
            reviewer_identity_ref: "reviewer-identity:release-admin".to_owned(),
            first_use_review_required: true,
        },
        insertion: RecipeInsertionBlock {
            target_class: RecipeInsertionTargetClass::ComposerPromptInsertion,
            preview: ReplayPreviewClass::FullPreviewBeforeReplay,
            approval: ToolApprovalPostureClass::AllowedWithPerInvocationPrompt,
            insertion_reversible: true,
            insertion_label: "Inserts the publish recipe into the composer for review".to_owned(),
        },
        headless_result: HeadlessResultBlock {
            state: HeadlessResultStateClass::CompletedWithDeferredSteps,
            result_content_address: "sha256:release-notes-publish-result-1".to_owned(),
            steps_total: 2,
            steps_completed: 1,
            steps_deferred: 1,
            steps_blocked: 0,
            audit: RecipeStepAuditClass::AuditedToSupportExport,
            result_label: "Inspected headless; the publish was deferred to interactive review"
                .to_owned(),
        },
        steps: vec![
            inspect_step(),
            AutomationStepDisclosure {
                side_effect_class: ToolSideEffectClass::ExternalIrreversiblePublish,
                headless_safety: HeadlessSafetyClass::HeadlessDeferredToInteractive,
                interactive_preview: ReplayPreviewClass::DiffPreviewBeforeReplay,
                approval_posture: ToolApprovalPostureClass::RequiresAdminApproval,
                audit: RecipeStepAuditClass::AuditedToSupportExport,
                reversibility: RecipeStepReversibilityClass::IrreversibleExternalPublish,
                disclosure_label:
                    "Publishes release notes you cannot unpublish; deferred for review headless"
                        .to_owned(),
            },
        ],
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Preview),
            provider_unavailable(M5AiWorkflowQualificationClass::Held),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::EvidencePreservedNoRevert,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:release-notes-publish".to_owned()],
        explanation_label: "Imported publish macro; irreversible publish defers headless"
            .to_owned(),
    }
}

/// A local inspect-only macro awaiting promotion review.
fn pending_inspect_macro() -> UserAutomationRow {
    UserAutomationRow {
        macro_id: "symbol-tour-macro".to_owned(),
        macro_label: "Symbol-tour navigation macro".to_owned(),
        macro_family_label: "Workspace inspection".to_owned(),
        macro_version: "0.4.0".to_owned(),
        capture_content_address: "sha256:symbol-tour-v0".to_owned(),
        capture_provenance: MacroCaptureProvenanceClass::RecordedFromUserSession,
        recorded_step_count: 4,
        publisher_source_class: ToolPublisherSourceClass::FirstPartyNativePublisher,
        publisher_identity_ref: "publisher-identity:first-party-recipes".to_owned(),
        resolved_mode: RoutePolicyModeClass::Local,
        promotion: MacroPromotionBlock {
            state: MacroPromotionStateClass::RecordedPendingReview,
            promoted_recipe_ref: String::new(),
            recipe_pack_ref: String::new(),
            promotion_approval: ToolApprovalPostureClass::AllowedWithPerInvocationPrompt,
            reviewer_identity_ref: String::new(),
            first_use_review_required: true,
        },
        insertion: RecipeInsertionBlock {
            target_class: RecipeInsertionTargetClass::CommandPaletteInsertion,
            preview: ReplayPreviewClass::InspectOnlyNoPreviewNeeded,
            approval: ToolApprovalPostureClass::AllowedWithoutPrompt,
            insertion_reversible: true,
            insertion_label: "Offers the symbol tour from the command palette".to_owned(),
        },
        headless_result: HeadlessResultBlock {
            state: HeadlessResultStateClass::CompletedAllStepsSafe,
            result_content_address: "sha256:symbol-tour-result-1".to_owned(),
            steps_total: 1,
            steps_completed: 1,
            steps_deferred: 0,
            steps_blocked: 0,
            audit: RecipeStepAuditClass::AuditedLocalHistoryOnly,
            result_label: "Inspected the workspace headless; no change was applied".to_owned(),
        },
        steps: vec![inspect_step()],
        claimed_qualification: M5AiWorkflowQualificationClass::Preview,
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Experimental),
            provider_unavailable(M5AiWorkflowQualificationClass::Experimental),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::NotApplicable,
        rollback_verified: false,
        evidence_packet_refs: vec!["evidence:symbol-tour".to_owned()],
        explanation_label: "Inspect-only macro pending promotion review; never mutates".to_owned(),
    }
}

/// A blocked macro whose tainted capture failed promotion, demonstrating
/// fail-closed headless behavior.
fn blocked_macro() -> UserAutomationRow {
    UserAutomationRow {
        macro_id: "deploy-trigger-macro".to_owned(),
        macro_label: "Deploy-trigger macro".to_owned(),
        macro_family_label: "Deployment".to_owned(),
        macro_version: "3.0.0".to_owned(),
        capture_content_address: "sha256:deploy-trigger-v3".to_owned(),
        capture_provenance: MacroCaptureProvenanceClass::RecordedFromReplaySession,
        recorded_step_count: 7,
        publisher_source_class: ToolPublisherSourceClass::UserRegisteredPublisher,
        publisher_identity_ref: "publisher-identity:user-deploy".to_owned(),
        resolved_mode: RoutePolicyModeClass::Byok,
        promotion: MacroPromotionBlock {
            state: MacroPromotionStateClass::PromotionBlockedTaintedCapture,
            promoted_recipe_ref: String::new(),
            recipe_pack_ref: String::new(),
            promotion_approval: ToolApprovalPostureClass::DeniedByPolicy,
            reviewer_identity_ref: String::new(),
            first_use_review_required: true,
        },
        insertion: RecipeInsertionBlock {
            target_class: RecipeInsertionTargetClass::HeadlessJobInsertion,
            preview: ReplayPreviewClass::DiffPreviewBeforeReplay,
            approval: ToolApprovalPostureClass::DeniedByPolicy,
            insertion_reversible: false,
            insertion_label: "Deploy insertion stays blocked while the capture is tainted"
                .to_owned(),
        },
        headless_result: HeadlessResultBlock {
            state: HeadlessResultStateClass::BlockedFailClosed,
            result_content_address: "sha256:deploy-trigger-result-1".to_owned(),
            steps_total: 2,
            steps_completed: 1,
            steps_deferred: 0,
            steps_blocked: 1,
            audit: RecipeStepAuditClass::AuditedToRunRecordTimeline,
            result_label: "Blocked the deploy fail-closed; the tainted capture cannot run"
                .to_owned(),
        },
        steps: vec![
            inspect_step(),
            AutomationStepDisclosure {
                side_effect_class: ToolSideEffectClass::ExternalIrreversiblePublish,
                headless_safety: HeadlessSafetyClass::HeadlessBlockedFailClosed,
                interactive_preview: ReplayPreviewClass::DiffPreviewBeforeReplay,
                approval_posture: ToolApprovalPostureClass::DeniedByPolicy,
                audit: RecipeStepAuditClass::AuditedToRunRecordTimeline,
                reversibility: RecipeStepReversibilityClass::IrreversibleExternalPublish,
                disclosure_label: "Deploy stays blocked fail-closed while the capture is tainted"
                    .to_owned(),
            },
        ],
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Unavailable),
            provider_unavailable(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::NotApplicable,
        rollback_verified: false,
        evidence_packet_refs: vec!["evidence:deploy-trigger".to_owned()],
        explanation_label: "Blocked deploy macro; tainted capture fails closed and claims nothing"
            .to_owned(),
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        USER_AUTOMATION_SCHEMA_REF.to_owned(),
        USER_AUTOMATION_DOC_REF.to_owned(),
        USER_AUTOMATION_RECIPE_MACRO_CONTRACT_REF.to_owned(),
        RECIPE_PACK_SCHEMA_REF.to_owned(),
        TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
        ROUTING_POLICY_SCHEMA_REF.to_owned(),
    ]
}

fn proof_freshness() -> UserAutomationProofFreshness {
    UserAutomationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> UserAutomationPacket {
    UserAutomationPacket::new(UserAutomationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "Recorded-Macro Promotion And Headless-Safe Results".to_owned(),
        automations: vec![
            headless_edit_macro(),
            imported_publish_macro(),
            pending_inspect_macro(),
            blocked_macro(),
        ],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    })
}

fn assert_has(packet: &UserAutomationPacket, expected: UserAutomationViolation) {
    let violations = packet.validate();
    assert!(
        violations.contains(&expected),
        "expected {:?}, got {:?}",
        expected,
        violations
    );
}

#[test]
fn user_automation_packet_validates() {
    assert!(packet().validate().is_empty(), "{:?}", packet().validate());
}

#[test]
fn round_trips_through_json() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: UserAutomationPacket = serde_json::from_str(&json).expect("packet parses");
    assert_eq!(packet, parsed);
}

#[test]
fn counts_match_fixture() {
    let packet = packet();
    // Three claimed automations (Stable, Beta, Preview); the Held blocked one is not.
    assert_eq!(packet.claimed_automation_count(), 3);
    // Two macros are promoted into a recipe.
    assert_eq!(packet.promoted_automation_count(), 2);
    // The tainted deploy macro is the only blocked one.
    assert_eq!(packet.blocked_automation_count(), 1);
    // Edit, publish, and deploy macros mutate; the symbol tour is inspect-only.
    assert_eq!(packet.mutating_automation_count(), 3);
}

#[test]
fn headless_safety_partition_holds() {
    assert!(HeadlessSafetyClass::HeadlessSafeInspectOnly.runs_headless());
    assert!(HeadlessSafetyClass::HeadlessSafePreauthorizedPolicy.runs_headless());
    assert!(HeadlessSafetyClass::HeadlessSafePreauthorizedPolicy.permits_mutation_headless());
    assert!(!HeadlessSafetyClass::HeadlessSafeInspectOnly.permits_mutation_headless());
    assert!(HeadlessSafetyClass::HeadlessDeferredToInteractive.is_deferred());
    assert!(HeadlessSafetyClass::HeadlessBlockedFailClosed.is_blocked());
    assert!(HeadlessSafetyClass::HeadlessDeniedByPolicy.is_blocked());
    assert!(!HeadlessSafetyClass::HeadlessDeferredToInteractive.runs_headless());
}

#[test]
fn result_state_partition_holds() {
    assert!(HeadlessResultStateClass::CompletedAllStepsSafe.is_complete());
    assert!(HeadlessResultStateClass::CompletedWithDeferredSteps.is_complete());
    assert!(!HeadlessResultStateClass::BlockedFailClosed.is_complete());
    assert!(HeadlessResultStateClass::BlockedFailClosed.requires_blocked_step());
    assert!(HeadlessResultStateClass::PartialThenHalted.requires_blocked_step());
}

#[test]
fn promotion_partition_holds() {
    assert!(MacroPromotionStateClass::PromotedToRecipe.is_promoted());
    assert!(!MacroPromotionStateClass::RecordedPendingReview.is_promoted());
    for blocked in [
        MacroPromotionStateClass::PromotionBlockedPolicy,
        MacroPromotionStateClass::PromotionBlockedTaintedCapture,
        MacroPromotionStateClass::PromotionWithdrawn,
    ] {
        assert!(blocked.is_blocked());
    }
    assert!(MacroCaptureProvenanceClass::ImportedFromSharedRecipePack.requires_recipe_pack_ref());
    assert!(!MacroCaptureProvenanceClass::RecordedFromUserSession.requires_recipe_pack_ref());
    assert!(RecipeInsertionTargetClass::HeadlessJobInsertion.is_headless_target());
    assert!(!RecipeInsertionTargetClass::ComposerPromptInsertion.is_headless_target());
}

#[test]
fn headless_safe_and_preview_first_hold() {
    assert!(headless_edit_macro().is_headless_safe());
    assert!(headless_edit_macro().is_preview_first());
    assert!(imported_publish_macro().is_headless_safe());
    assert!(imported_publish_macro().is_preview_first());
    // The blocked deploy macro is still headless-safe: its publish blocks fail-closed.
    assert!(blocked_macro().is_headless_safe());
    // Inspect-only macros are vacuously headless-safe and preview-first.
    assert!(pending_inspect_macro().is_headless_safe());
    assert!(pending_inspect_macro().is_preview_first());
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "other".to_owned();
    assert_has(&packet, UserAutomationViolation::WrongRecordKind);
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = packet();
    packet.schema_version = 99;
    assert_has(&packet, UserAutomationViolation::WrongSchemaVersion);
}

#[test]
fn missing_identity_fails() {
    let mut packet = packet();
    packet.packet_id = "  ".to_owned();
    assert_has(&packet, UserAutomationViolation::MissingIdentity);
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs = vec![USER_AUTOMATION_SCHEMA_REF.to_owned()];
    assert_has(&packet, UserAutomationViolation::MissingSourceContracts);
}

#[test]
fn no_automations_fails() {
    let mut packet = packet();
    packet.automations.clear();
    assert_has(&packet, UserAutomationViolation::NoAutomations);
}

#[test]
fn duplicate_automation_fails() {
    let mut packet = packet();
    packet.automations.push(headless_edit_macro());
    assert_has(&packet, UserAutomationViolation::DuplicateAutomation);
}

#[test]
fn automation_row_incomplete_fails() {
    let mut packet = packet();
    packet.automations[0].macro_label = "  ".to_owned();
    assert_has(&packet, UserAutomationViolation::AutomationRowIncomplete);
}

#[test]
fn automation_missing_capture_address_fails() {
    let mut packet = packet();
    packet.automations[0].capture_content_address = "  ".to_owned();
    assert_has(
        &packet,
        UserAutomationViolation::AutomationMissingCaptureAddress,
    );
}

#[test]
fn automation_missing_publisher_identity_fails() {
    let mut packet = packet();
    packet.automations[0].publisher_identity_ref = "  ".to_owned();
    assert_has(
        &packet,
        UserAutomationViolation::AutomationMissingPublisherIdentity,
    );
}

#[test]
fn imported_capture_missing_recipe_pack_ref_fails() {
    let mut packet = packet();
    packet.automations[1].promotion.recipe_pack_ref = "  ".to_owned();
    assert_has(
        &packet,
        UserAutomationViolation::ImportedCaptureMissingRecipePackRef,
    );
}

#[test]
fn automation_missing_step_disclosures_fails() {
    let mut packet = packet();
    packet.automations[2].steps.clear();
    assert_has(
        &packet,
        UserAutomationViolation::AutomationMissingStepDisclosures,
    );
}

#[test]
fn step_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet.automations[0].steps[1].disclosure_label = "  ".to_owned();
    assert_has(&packet, UserAutomationViolation::StepDisclosureIncomplete);
}

#[test]
fn duplicate_step_disclosure_fails() {
    let mut packet = packet();
    let dup = packet.automations[0].steps[0].clone();
    packet.automations[0].steps.push(dup);
    assert_has(&packet, UserAutomationViolation::DuplicateStepDisclosure);
}

#[test]
fn mutating_step_without_preview_first_fails() {
    let mut packet = packet();
    packet.automations[0].steps[1].interactive_preview =
        ReplayPreviewClass::PreviewUnavailableMustBlock;
    assert_has(
        &packet,
        UserAutomationViolation::MutatingStepWithoutPreviewFirst,
    );
}

#[test]
fn mutating_step_without_approval_fails() {
    let mut packet = packet();
    packet.automations[0].steps[1].approval_posture =
        ToolApprovalPostureClass::AllowedWithoutPrompt;
    assert_has(
        &packet,
        UserAutomationViolation::MutatingStepWithoutApproval,
    );
}

#[test]
fn mutating_step_without_audit_fails() {
    let mut packet = packet();
    packet.automations[0].steps[1].audit = RecipeStepAuditClass::NotAudited;
    assert_has(&packet, UserAutomationViolation::MutatingStepWithoutAudit);
}

#[test]
fn step_reversibility_mismatch_fails() {
    let mut packet = packet();
    packet.automations[2].steps[0].reversibility =
        RecipeStepReversibilityClass::ReversibleInWorkspace;
    assert_has(&packet, UserAutomationViolation::StepReversibilityMismatch);
}

#[test]
fn inspect_step_not_headless_inspect_safe_fails() {
    let mut packet = packet();
    packet.automations[2].steps[0].headless_safety =
        HeadlessSafetyClass::HeadlessDeferredToInteractive;
    assert_has(
        &packet,
        UserAutomationViolation::InspectStepNotHeadlessInspectSafe,
    );
}

#[test]
fn mutating_step_claims_inspect_headless_safety_fails() {
    let mut packet = packet();
    packet.automations[0].steps[1].headless_safety = HeadlessSafetyClass::HeadlessSafeInspectOnly;
    assert_has(
        &packet,
        UserAutomationViolation::MutatingStepClaimsInspectHeadlessSafety,
    );
}

#[test]
fn irreversible_publish_runs_headless_fails() {
    let mut packet = packet();
    // Force the imported macro's publish to run unattended headless.
    packet.automations[1].steps[1].headless_safety =
        HeadlessSafetyClass::HeadlessSafePreauthorizedPolicy;
    assert_has(
        &packet,
        UserAutomationViolation::IrreversiblePublishRunsHeadless,
    );
}

#[test]
fn headless_preauthorized_step_without_approval_fails() {
    let mut packet = packet();
    // The edit step is pre-authorized headless; drop its approval gate.
    packet.automations[0].steps[1].approval_posture =
        ToolApprovalPostureClass::AllowedWithoutPrompt;
    assert_has(
        &packet,
        UserAutomationViolation::HeadlessPreauthorizedStepWithoutApproval,
    );
}

#[test]
fn headless_preauthorized_step_without_audit_fails() {
    let mut packet = packet();
    packet.automations[0].steps[1].audit = RecipeStepAuditClass::NotAudited;
    assert_has(
        &packet,
        UserAutomationViolation::HeadlessPreauthorizedStepWithoutAudit,
    );
}

#[test]
fn promoted_missing_recipe_ref_fails() {
    let mut packet = packet();
    packet.automations[0].promotion.promoted_recipe_ref = "  ".to_owned();
    assert_has(&packet, UserAutomationViolation::PromotedMissingRecipeRef);
}

#[test]
fn unpromoted_has_recipe_ref_fails() {
    let mut packet = packet();
    packet.automations[2].promotion.promoted_recipe_ref = "recipe:leaked".to_owned();
    assert_has(&packet, UserAutomationViolation::UnpromotedHasRecipeRef);
}

#[test]
fn mutating_promotion_without_gate_fails() {
    let mut packet = packet();
    packet.automations[0].promotion.promotion_approval =
        ToolApprovalPostureClass::AllowedWithoutPrompt;
    assert_has(
        &packet,
        UserAutomationViolation::MutatingPromotionWithoutGate,
    );
}

#[test]
fn blocked_promotion_claims_qualification_fails() {
    let mut packet = packet();
    packet.automations[3].claimed_qualification = M5AiWorkflowQualificationClass::Beta;
    assert_has(
        &packet,
        UserAutomationViolation::BlockedPromotionClaimsQualification,
    );
}

#[test]
fn pending_promotion_claims_stable_fails() {
    let mut packet = packet();
    packet.automations[2].claimed_qualification = M5AiWorkflowQualificationClass::Stable;
    assert_has(
        &packet,
        UserAutomationViolation::PendingPromotionClaimsStable,
    );
}

#[test]
fn insertion_missing_label_fails() {
    let mut packet = packet();
    packet.automations[0].insertion.insertion_label = "  ".to_owned();
    assert_has(&packet, UserAutomationViolation::InsertionMissingLabel);
}

#[test]
fn mutating_insertion_without_preview_first_fails() {
    let mut packet = packet();
    packet.automations[0].insertion.preview = ReplayPreviewClass::PreviewUnavailableMustBlock;
    assert_has(
        &packet,
        UserAutomationViolation::MutatingInsertionWithoutPreviewFirst,
    );
}

#[test]
fn headless_target_insertion_requires_interactive_approval_fails() {
    let mut packet = packet();
    // The edit macro inserts into the automation queue (headless); force an
    // interactive per-invocation prompt it can never satisfy headless.
    packet.automations[0].insertion.approval =
        ToolApprovalPostureClass::AllowedWithPerInvocationPrompt;
    assert_has(
        &packet,
        UserAutomationViolation::HeadlessTargetInsertionRequiresInteractiveApproval,
    );
}

#[test]
fn headless_result_incomplete_fails() {
    let mut packet = packet();
    packet.automations[0].headless_result.result_content_address = "  ".to_owned();
    assert_has(&packet, UserAutomationViolation::HeadlessResultIncomplete);
}

#[test]
fn headless_result_count_mismatch_fails() {
    let mut packet = packet();
    packet.automations[0].headless_result.steps_completed = 5;
    assert_has(
        &packet,
        UserAutomationViolation::HeadlessResultCountMismatch,
    );
}

#[test]
fn headless_result_state_mismatch_fails() {
    let mut packet = packet();
    // The deploy macro has one blocked step; claim it completed-all-safe.
    packet.automations[3].headless_result.state = HeadlessResultStateClass::CompletedAllStepsSafe;
    assert_has(
        &packet,
        UserAutomationViolation::HeadlessResultStateMismatch,
    );
}

#[test]
fn headless_run_mutating_not_externally_audited_fails() {
    let mut packet = packet();
    // The edit macro ran a mutating step headless; downgrade its result audit.
    packet.automations[0].headless_result.audit = RecipeStepAuditClass::AuditedLocalHistoryOnly;
    assert_has(
        &packet,
        UserAutomationViolation::HeadlessRunMutatingNotExternallyAudited,
    );
}

#[test]
fn claimed_automation_missing_evidence_fails() {
    let mut packet = packet();
    packet.automations[0].evidence_packet_refs.clear();
    assert_has(
        &packet,
        UserAutomationViolation::ClaimedAutomationMissingEvidence,
    );
}

#[test]
fn claimed_rollback_unverified_fails() {
    let mut packet = packet();
    packet.automations[0].rollback_verified = false;
    assert_has(&packet, UserAutomationViolation::ClaimedRollbackUnverified);
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.automations[0].downgrade_rules.clear();
    assert_has(&packet, UserAutomationViolation::DowngradeRulesMissing);
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.automations[0].downgrade_rules =
        vec![provider_unavailable(M5AiWorkflowQualificationClass::Beta)];
    assert_has(
        &packet,
        UserAutomationViolation::DowngradeRuleMissingProofStale,
    );
}

#[test]
fn downgrade_rule_missing_provider_unavailable_fails() {
    let mut packet = packet();
    packet.automations[0].downgrade_rules = vec![proof_stale(M5AiWorkflowQualificationClass::Beta)];
    assert_has(
        &packet,
        UserAutomationViolation::DowngradeRuleMissingProviderUnavailable,
    );
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    packet.automations[0].downgrade_rules = vec![
        proof_stale(M5AiWorkflowQualificationClass::Stable),
        provider_unavailable(M5AiWorkflowQualificationClass::Beta),
    ];
    assert_has(&packet, UserAutomationViolation::DowngradeRuleNotNarrowing);
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert_has(&packet, UserAutomationViolation::ProofFreshnessIncomplete);
}

#[test]
fn raw_automation_material_in_export_fails() {
    let mut packet = packet();
    packet.automations[0].explanation_label = "runs $(curl https://example.com)".to_owned();
    assert_has(
        &packet,
        UserAutomationViolation::RawAutomationMaterialInExport,
    );
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let automation = imported_publish_macro();
    assert_eq!(
        automation.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Preview
    );
    assert_eq!(
        automation.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Held
    );
    assert_eq!(
        automation.narrowed_qualification(M5AiWorkflowDowngradeTrigger::TrustNarrowing),
        M5AiWorkflowQualificationClass::Beta
    );
}

#[test]
fn render_inspector_lists_every_step() {
    let card = headless_edit_macro().render_inspector();
    assert!(card.contains("local_reversible_edit"));
    assert!(card.contains("headless_safe_preauthorized_policy"));
    assert!(card.contains("promoted_to_recipe"));
    assert!(card.contains("completed_all_steps_safe"));
}

#[test]
fn markdown_summary_lists_every_automation() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("format-on-save-macro"));
    assert!(summary.contains("release-notes-publish-macro"));
    assert!(summary.contains("symbol-tour-macro"));
    assert!(summary.contains("deploy-trigger-macro"));
}

#[test]
fn blocked_macro_narrows_fixture_validates() {
    let packet: UserAutomationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/headless_blocked_fail_closed.json"
    )))
    .expect("blocked macro fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let blocked = packet
        .automation("deploy-trigger-macro")
        .expect("blocked macro present");
    assert!(!blocked.is_claimed());
    assert!(blocked.promotion.state.is_blocked());
    // The blocked macro is still headless-safe: its publish fails closed.
    assert!(blocked.is_headless_safe());
    assert_eq!(
        blocked.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Unavailable
    );
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_user_automation_export().expect("checked user automation export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.automations.is_empty());
    // Every mutating automation is headless-safe and previews before replay.
    for automation in &packet.automations {
        assert!(automation.is_headless_safe());
        assert!(automation.is_preview_first());
    }
    // The blocked deploy macro dropped out of every claimed lane.
    let blocked = packet
        .automation("deploy-trigger-macro")
        .expect("blocked macro present");
    assert!(!blocked.is_claimed());
    assert!(blocked.promotion.state.is_blocked());
}
