//! Conformance dump for the recorded-macro / recipe-insertion / headless-result
//! user-automation packet.
//!
//! Prints the canonical support export so the checked-in artifact and fixtures
//! stay byte-aligned with the in-crate builder.

use aureline_ai::add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation::*;
use aureline_ai::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use aureline_ai::implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains::{
    RoutePolicyModeClass, ROUTING_POLICY_SCHEMA_REF,
};
use aureline_ai::implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay::{
    RecipeStepAuditClass, RecipeStepReversibilityClass, ReplayPreviewClass, RECIPE_PACK_SCHEMA_REF,
};
use aureline_ai::tool_gateway::{
    ToolApprovalPostureClass, ToolPublisherSourceClass, ToolSideEffectClass,
    TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
};

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

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = match which.as_str() {
        "fixture" => UserAutomationPacket::new(UserAutomationPacketInput {
            packet_id: "user-automation:fixture:blocked:0001".to_owned(),
            catalogue_label: "Headless Deploy Macro Blocks Fail-Closed".to_owned(),
            automations: vec![headless_edit_macro(), blocked_macro()],
            proof_freshness: UserAutomationProofFreshness {
                proof_freshness_slo_hours: 168,
                last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
                auto_narrow_on_stale: true,
            },
            source_contract_refs: source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: "2026-06-09T00:00:00Z".to_owned(),
        }),
        _ => UserAutomationPacket::new(UserAutomationPacketInput {
            packet_id: "user-automation:stable:0001".to_owned(),
            catalogue_label: "Recorded-Macro Promotion And Headless-Safe Results".to_owned(),
            automations: vec![
                headless_edit_macro(),
                imported_publish_macro(),
                pending_inspect_macro(),
                blocked_macro(),
            ],
            proof_freshness: UserAutomationProofFreshness {
                proof_freshness_slo_hours: 168,
                last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
                auto_narrow_on_stale: true,
            },
            source_contract_refs: source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: "2026-06-09T00:00:00Z".to_owned(),
        }),
    };

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );
    println!("{}", packet.export_safe_json());
}
