//! Conformance dump for the signed/shared recipe-pack graduation packet.
//!
//! Prints the canonical support export so the checked-in artifact and fixtures
//! stay byte-aligned with the in-crate builder.

use aureline_ai::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use aureline_ai::implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains::{
    RoutePolicyModeClass, ROUTING_POLICY_SCHEMA_REF,
};
use aureline_ai::implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay::*;
use aureline_ai::tool_gateway::{
    ToolApprovalPostureClass, ToolPublisherSourceClass, ToolSideEffectClass,
    TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
};

fn proof_stale(narrowed_to: M5AiWorkflowQualificationClass) -> RecipePackDowngradeRule {
    RecipePackDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn provider_unavailable(narrowed_to: M5AiWorkflowQualificationClass) -> RecipePackDowngradeRule {
    RecipePackDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProviderUnavailable,
        narrowed_to,
        auto_enforced: true,
        rationale: "Provider outage or quota exhaustion narrows the claim".to_owned(),
    }
}

fn inspect_disclosure() -> RecipeStepDisclosure {
    RecipeStepDisclosure {
        side_effect_class: ToolSideEffectClass::InspectOnly,
        replay_preview: ReplayPreviewClass::InspectOnlyNoPreviewNeeded,
        approval_posture: ToolApprovalPostureClass::AllowedWithoutPrompt,
        audit: RecipeStepAuditClass::AuditedLocalHistoryOnly,
        reversibility: RecipeStepReversibilityClass::NoSideEffect,
        disclosure_label: "Reads workspace context; no change is applied".to_owned(),
    }
}

fn org_template_pack() -> RecipePackRow {
    RecipePackRow {
        pack_id: "org-review-template".to_owned(),
        pack_label: "Organization review-comment template".to_owned(),
        pack_family_label: "Code review".to_owned(),
        pack_version: "1.4.0".to_owned(),
        manifest_content_address: "sha256:org-review-template-v1".to_owned(),
        descriptor_pack_ref: "descriptor-pack:org-review-template".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::EnterpriseRegisteredPublisher,
        publisher_identity_ref: "publisher-identity:enterprise-review".to_owned(),
        signature_class: RecipePackSignatureClass::AuthorAndOrganizationSignature,
        share_scope_class: RecipePackShareScopeClass::OrganizationManagedChannel,
        resolved_mode: RoutePolicyModeClass::Managed,
        automation_authority_class: AutomationAuthorityClass::ManagedOnlyTemplateAuthority,
        state: RecipePackStateClass::Admitted,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        step_disclosures: vec![
            inspect_disclosure(),
            RecipeStepDisclosure {
                side_effect_class: ToolSideEffectClass::ExternalReversibleComment,
                replay_preview: ReplayPreviewClass::FullPreviewBeforeReplay,
                approval_posture: ToolApprovalPostureClass::AllowedWithPerInvocationPrompt,
                audit: RecipeStepAuditClass::AuditedToRunRecordTimeline,
                reversibility: RecipeStepReversibilityClass::ReversibleInWorkspace,
                disclosure_label: "Posts a review comment you confirm first; can be deleted"
                    .to_owned(),
            },
        ],
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:org-review-template".to_owned()],
        explanation_label: "Managed review template; comments preview and audit before posting"
            .to_owned(),
    }
}

fn byok_publish_pack() -> RecipePackRow {
    RecipePackRow {
        pack_id: "byok-issue-publish".to_owned(),
        pack_label: "BYOK issue-tracker publish pack".to_owned(),
        pack_family_label: "Issue tracking".to_owned(),
        pack_version: "2.0.1".to_owned(),
        manifest_content_address: "sha256:byok-issue-publish-v2".to_owned(),
        descriptor_pack_ref: "descriptor-pack:byok-issue-publish".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::McpServerPublisher,
        publisher_identity_ref: "publisher-identity:byok-issue".to_owned(),
        signature_class: RecipePackSignatureClass::AuthorAndOrganizationSignature,
        share_scope_class: RecipePackShareScopeClass::PortableProfileExport,
        resolved_mode: RoutePolicyModeClass::Byok,
        automation_authority_class: AutomationAuthorityClass::ExternalIrreversibleAdminGated,
        state: RecipePackStateClass::Admitted,
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        step_disclosures: vec![
            inspect_disclosure(),
            RecipeStepDisclosure {
                side_effect_class: ToolSideEffectClass::ExternalIrreversiblePublish,
                replay_preview: ReplayPreviewClass::DiffPreviewBeforeReplay,
                approval_posture: ToolApprovalPostureClass::RequiresAdminApproval,
                audit: RecipeStepAuditClass::AuditedToSupportExport,
                reversibility: RecipeStepReversibilityClass::IrreversibleExternalPublish,
                disclosure_label:
                    "Files an issue you cannot unfile; you review the diff and approve first"
                        .to_owned(),
            },
        ],
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Preview),
            provider_unavailable(M5AiWorkflowQualificationClass::Held),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::EvidencePreservedNoRevert,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:byok-issue-publish".to_owned()],
        explanation_label: "BYOK publish pack; irreversible filing is admin-gated and audited"
            .to_owned(),
    }
}

fn local_inspect_pack() -> RecipePackRow {
    RecipePackRow {
        pack_id: "local-symbol-inspector".to_owned(),
        pack_label: "Local symbol-inspector pack".to_owned(),
        pack_family_label: "Workspace inspection".to_owned(),
        pack_version: "0.9.0".to_owned(),
        manifest_content_address: "sha256:local-symbol-inspector-v0".to_owned(),
        descriptor_pack_ref: "descriptor-pack:local-symbol-inspector".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::FirstPartyNativePublisher,
        publisher_identity_ref: "publisher-identity:first-party-recipes".to_owned(),
        signature_class: RecipePackSignatureClass::UnsignedLocalOnly,
        share_scope_class: RecipePackShareScopeClass::UserScopeLocalOnly,
        resolved_mode: RoutePolicyModeClass::Local,
        automation_authority_class: AutomationAuthorityClass::InspectOnlyNoAuthority,
        state: RecipePackStateClass::Admitted,
        claimed_qualification: M5AiWorkflowQualificationClass::Preview,
        step_disclosures: vec![inspect_disclosure()],
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Experimental),
            provider_unavailable(M5AiWorkflowQualificationClass::Experimental),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::NotApplicable,
        rollback_verified: false,
        evidence_packet_refs: vec!["evidence:local-symbol-inspector".to_owned()],
        explanation_label: "Local inspect-only pack; unsigned and never leaves the device"
            .to_owned(),
    }
}

fn quarantined_pack() -> RecipePackRow {
    RecipePackRow {
        pack_id: "quarantined-deploy".to_owned(),
        pack_label: "Quarantined deploy template".to_owned(),
        pack_family_label: "Deployment".to_owned(),
        pack_version: "3.2.0".to_owned(),
        manifest_content_address: "sha256:quarantined-deploy-v3".to_owned(),
        descriptor_pack_ref: "descriptor-pack:quarantined-deploy".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::SignedExtensionPublisher,
        publisher_identity_ref: "publisher-identity:deploy-ext".to_owned(),
        signature_class: RecipePackSignatureClass::ManagedOnlyChannelSignature,
        share_scope_class: RecipePackShareScopeClass::OrganizationManagedChannel,
        resolved_mode: RoutePolicyModeClass::EnterpriseGateway,
        automation_authority_class: AutomationAuthorityClass::ManagedOnlyTemplateAuthority,
        state: RecipePackStateClass::QuarantinedSignature,
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        step_disclosures: vec![
            inspect_disclosure(),
            RecipeStepDisclosure {
                side_effect_class: ToolSideEffectClass::ExternalIrreversiblePublish,
                replay_preview: ReplayPreviewClass::DiffPreviewBeforeReplay,
                approval_posture: ToolApprovalPostureClass::DeniedByPolicy,
                audit: RecipeStepAuditClass::AuditedToRunRecordTimeline,
                reversibility: RecipeStepReversibilityClass::IrreversibleExternalPublish,
                disclosure_label: "Deploy stays blocked while the pack signature is unverified"
                    .to_owned(),
            },
        ],
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Unavailable),
            provider_unavailable(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::NotApplicable,
        rollback_verified: false,
        evidence_packet_refs: vec!["evidence:quarantined-deploy".to_owned()],
        explanation_label: "Quarantined deploy template; signature failed so it claims nothing"
            .to_owned(),
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        RECIPE_PACK_SCHEMA_REF.to_owned(),
        RECIPE_PACK_DOC_REF.to_owned(),
        RECIPE_PACK_AUTOMATION_CONTRACT_REF.to_owned(),
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
        "fixture" => RecipePackGraduationPacket::new(RecipePackGraduationPacketInput {
            packet_id: "recipe-pack:fixture:blocked:0001".to_owned(),
            catalogue_label: "Blocked Recipe Pack Narrows Its Claim".to_owned(),
            packs: vec![org_template_pack(), quarantined_pack()],
            proof_freshness: RecipePackProofFreshness {
                proof_freshness_slo_hours: 168,
                last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
                auto_narrow_on_stale: true,
            },
            source_contract_refs: source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: "2026-06-09T00:00:00Z".to_owned(),
        }),
        _ => RecipePackGraduationPacket::new(RecipePackGraduationPacketInput {
            packet_id: "recipe-pack:stable:0001".to_owned(),
            catalogue_label: "Signed And Shared Recipe Packs".to_owned(),
            packs: vec![
                org_template_pack(),
                byok_publish_pack(),
                local_inspect_pack(),
                quarantined_pack(),
            ],
            proof_freshness: RecipePackProofFreshness {
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
