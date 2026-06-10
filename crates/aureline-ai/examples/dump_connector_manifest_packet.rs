//! Conformance dump for the external-tool connector-manifest packet.
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
use aureline_ai::ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure::*;
use aureline_ai::tool_gateway::{
    ToolApprovalPostureClass, ToolCapabilityClass, ToolCredentialPostureClass,
    ToolNetworkBehaviorClass, ToolOutputTrustPostureClass, ToolPublisherSourceClass,
    ToolRuntimeBoundaryClass, ToolSideEffectClass, TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
};

fn proof_stale(narrowed_to: M5AiWorkflowQualificationClass) -> ConnectorDowngradeRule {
    ConnectorDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn provider_unavailable(narrowed_to: M5AiWorkflowQualificationClass) -> ConnectorDowngradeRule {
    ConnectorDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProviderUnavailable,
        narrowed_to,
        auto_enforced: true,
        rationale: "Provider outage or quota exhaustion narrows the claim".to_owned(),
    }
}

fn inspect_disclosure() -> SideEffectDisclosure {
    SideEffectDisclosure {
        side_effect_class: ToolSideEffectClass::InspectOnly,
        preview: SideEffectPreviewClass::InspectOnlyNoPreviewNeeded,
        approval_posture: ToolApprovalPostureClass::AllowedWithoutPrompt,
        audit: SideEffectAuditClass::AuditedLocalHistoryOnly,
        reversibility: SideEffectReversibilityClass::NoSideEffect,
        disclosure_label: "Reads workspace context; no change is applied".to_owned(),
    }
}

fn managed_review_connector() -> ConnectorManifestRow {
    ConnectorManifestRow {
        manifest_id: "managed-review-comment".to_owned(),
        connector_label: "Managed review-comment connector".to_owned(),
        connector_family_label: "Code review".to_owned(),
        connector_capability_version: "1.4.0".to_owned(),
        descriptor_ref: "descriptor:managed-review-comment".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::EnterpriseRegisteredPublisher,
        publisher_identity_ref: "publisher-identity:enterprise-review".to_owned(),
        resolved_mode: RoutePolicyModeClass::Managed,
        runtime_boundary_class: ToolRuntimeBoundaryClass::RemoteVendorManagedService,
        network_behavior_class: ToolNetworkBehaviorClass::RemoteHttps,
        credential_posture_class: ToolCredentialPostureClass::EnterpriseGatewayManaged,
        output_trust_posture_class: ToolOutputTrustPostureClass::AlwaysTaintedRemoteServiceOutput,
        state: ConnectorManifestStateClass::Admitted,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        capability_classes: vec![
            ToolCapabilityClass::FetchExternalData,
            ToolCapabilityClass::PublishExternalComment,
        ],
        side_effect_disclosures: vec![
            inspect_disclosure(),
            SideEffectDisclosure {
                side_effect_class: ToolSideEffectClass::ExternalReversibleComment,
                preview: SideEffectPreviewClass::PreviewRequiredBeforeApply,
                approval_posture: ToolApprovalPostureClass::AllowedWithPerInvocationPrompt,
                audit: SideEffectAuditClass::AuditedToEvidenceTimeline,
                reversibility: SideEffectReversibilityClass::ReversibleInWorkspace,
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
        evidence_packet_refs: vec!["evidence:managed-review-comment".to_owned()],
        explanation_label: "Managed review connector; comments preview and audit before posting"
            .to_owned(),
    }
}

fn byok_publish_connector() -> ConnectorManifestRow {
    ConnectorManifestRow {
        manifest_id: "byok-issue-publish".to_owned(),
        connector_label: "BYOK issue-tracker publish connector".to_owned(),
        connector_family_label: "Issue tracking".to_owned(),
        connector_capability_version: "2.0.1".to_owned(),
        descriptor_ref: "descriptor:byok-issue-publish".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::McpServerPublisher,
        publisher_identity_ref: "publisher-identity:byok-issue".to_owned(),
        resolved_mode: RoutePolicyModeClass::Byok,
        runtime_boundary_class: ToolRuntimeBoundaryClass::RemoteSelfHostedService,
        network_behavior_class: ToolNetworkBehaviorClass::RemoteMcpOverStreamableHttp,
        credential_posture_class: ToolCredentialPostureClass::ByokSecretBroker,
        output_trust_posture_class: ToolOutputTrustPostureClass::AlwaysTaintedExternalToolOutput,
        state: ConnectorManifestStateClass::Admitted,
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        capability_classes: vec![
            ToolCapabilityClass::FetchExternalData,
            ToolCapabilityClass::PublishExternalArtifact,
        ],
        side_effect_disclosures: vec![
            inspect_disclosure(),
            SideEffectDisclosure {
                side_effect_class: ToolSideEffectClass::ExternalIrreversiblePublish,
                preview: SideEffectPreviewClass::DiffPreviewBeforeApply,
                approval_posture: ToolApprovalPostureClass::RequiresAdminApproval,
                audit: SideEffectAuditClass::AuditedToSupportExport,
                reversibility: SideEffectReversibilityClass::IrreversibleExternalPublish,
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
        explanation_label: "BYOK publish connector; irreversible filing is gated and audited"
            .to_owned(),
    }
}

fn local_inspect_connector() -> ConnectorManifestRow {
    ConnectorManifestRow {
        manifest_id: "local-symbol-inspector".to_owned(),
        connector_label: "Local symbol-inspector connector".to_owned(),
        connector_family_label: "Workspace inspection".to_owned(),
        connector_capability_version: "0.9.0".to_owned(),
        descriptor_ref: "descriptor:local-symbol-inspector".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::FirstPartyNativePublisher,
        publisher_identity_ref: "publisher-identity:first-party".to_owned(),
        resolved_mode: RoutePolicyModeClass::Local,
        runtime_boundary_class: ToolRuntimeBoundaryClass::LocalInProcess,
        network_behavior_class: ToolNetworkBehaviorClass::NoNetworkLocalOnly,
        credential_posture_class: ToolCredentialPostureClass::NoCredentialLocalOnly,
        output_trust_posture_class: ToolOutputTrustPostureClass::TrustedFirstPartyLocalToolOutput,
        state: ConnectorManifestStateClass::Admitted,
        claimed_qualification: M5AiWorkflowQualificationClass::Preview,
        capability_classes: vec![
            ToolCapabilityClass::InspectWorkspaceSymbols,
            ToolCapabilityClass::InspectWorkspaceFiles,
        ],
        side_effect_disclosures: vec![inspect_disclosure()],
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Experimental),
            provider_unavailable(M5AiWorkflowQualificationClass::Experimental),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::NotApplicable,
        rollback_verified: false,
        evidence_packet_refs: vec!["evidence:local-symbol-inspector".to_owned()],
        explanation_label: "Local first-party inspector; trusted output stays on-device".to_owned(),
    }
}

fn quarantined_connector() -> ConnectorManifestRow {
    ConnectorManifestRow {
        manifest_id: "quarantined-deploy".to_owned(),
        connector_label: "Quarantined deploy connector".to_owned(),
        connector_family_label: "Deployment".to_owned(),
        connector_capability_version: "3.2.0".to_owned(),
        descriptor_ref: "descriptor:quarantined-deploy".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::SignedExtensionPublisher,
        publisher_identity_ref: "publisher-identity:deploy-ext".to_owned(),
        resolved_mode: RoutePolicyModeClass::EnterpriseGateway,
        runtime_boundary_class: ToolRuntimeBoundaryClass::EnterpriseGatewayBrokeredService,
        network_behavior_class: ToolNetworkBehaviorClass::RemoteGrpcOverTls,
        credential_posture_class: ToolCredentialPostureClass::EnterpriseGatewayManaged,
        output_trust_posture_class: ToolOutputTrustPostureClass::AlwaysTaintedRemoteServiceOutput,
        state: ConnectorManifestStateClass::QuarantinedSignature,
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        capability_classes: vec![
            ToolCapabilityClass::ManageExternalResource,
            ToolCapabilityClass::PublishExternalArtifact,
        ],
        side_effect_disclosures: vec![
            inspect_disclosure(),
            SideEffectDisclosure {
                side_effect_class: ToolSideEffectClass::ExternalIrreversiblePublish,
                preview: SideEffectPreviewClass::DiffPreviewBeforeApply,
                approval_posture: ToolApprovalPostureClass::DeniedByPolicy,
                audit: SideEffectAuditClass::AuditedToEvidenceTimeline,
                reversibility: SideEffectReversibilityClass::IrreversibleExternalPublish,
                disclosure_label:
                    "Deploy stays blocked while the connector signature is unverified".to_owned(),
            },
        ],
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Unavailable),
            provider_unavailable(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::NotApplicable,
        rollback_verified: false,
        evidence_packet_refs: vec!["evidence:quarantined-deploy".to_owned()],
        explanation_label: "Quarantined deploy connector; signature failed so it claims nothing"
            .to_owned(),
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        CONNECTOR_MANIFEST_SCHEMA_REF.to_owned(),
        CONNECTOR_MANIFEST_DOC_REF.to_owned(),
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
        "fixture" => ConnectorManifestPacket::new(ConnectorManifestPacketInput {
            packet_id: "connector-manifest:fixture:blocked:0001".to_owned(),
            catalogue_label: "Blocked Connector Narrows Its Claim".to_owned(),
            connectors: vec![managed_review_connector(), quarantined_connector()],
            proof_freshness: ConnectorManifestProofFreshness {
                proof_freshness_slo_hours: 168,
                last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
                auto_narrow_on_stale: true,
            },
            source_contract_refs: source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: "2026-06-09T00:00:00Z".to_owned(),
        }),
        _ => ConnectorManifestPacket::new(ConnectorManifestPacketInput {
            packet_id: "connector-manifest:stable:0001".to_owned(),
            catalogue_label: "External-Tool Gateway Connector Manifests".to_owned(),
            connectors: vec![
                managed_review_connector(),
                byok_publish_connector(),
                local_inspect_connector(),
                quarantined_connector(),
            ],
            proof_freshness: ConnectorManifestProofFreshness {
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
