use super::*;

use crate::registry::{
    ExternalToolExecutionLocusClass, ExternalToolRegistryEntry, ExternalToolTransportClass,
    PROVIDER_MODEL_REGISTRY_EXTERNAL_TOOL_ENTRY_RECORD_KIND,
    PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION,
};
use crate::routing::{DeploymentProfileClass, PolicyTrustState};

fn policy_context() -> RoutingPolicyContext {
    RoutingPolicyContext {
        policy_epoch_ref: "policy-epoch:0042".to_owned(),
        trust_state: PolicyTrustState::Trusted,
        deployment_profile_class: DeploymentProfileClass::IndividualLocal,
        execution_context_ref: Some("execution-context:ai.tool-gateway:0001".to_owned()),
    }
}

fn local_native_descriptor() -> ToolGatewayDescriptor {
    ToolGatewayDescriptor {
        record_kind: TOOL_GATEWAY_DESCRIPTOR_RECORD_KIND.to_owned(),
        schema_version: TOOL_GATEWAY_SCHEMA_VERSION,
        descriptor_id: "tool-gateway-descriptor:native-fs-snapshot:0001".to_owned(),
        external_tool_entry_ref: String::new(),
        display_label: "Aureline filesystem snapshot tool".to_owned(),
        tool_family_label: "Filesystem snapshot".to_owned(),
        tool_capability_version: "fs_snapshot_native.v1".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::FirstPartyNativePublisher,
        publisher_identity_ref: "publisher-identity:aureline-native:0001".to_owned(),
        runtime_boundary_class: ToolRuntimeBoundaryClass::LocalInProcess,
        network_behavior_class: ToolNetworkBehaviorClass::NoNetworkLocalOnly,
        credential_posture_class: ToolCredentialPostureClass::NoCredentialLocalOnly,
        output_trust_posture_class: ToolOutputTrustPostureClass::TrustedFirstPartyLocalToolOutput,
        availability_state_class: ToolAvailabilityStateClass::WarmAdmittedReady,
        lifecycle_state_class: ToolGatewayLifecycleStateClass::GenerallyAdmitted,
        approval_posture_class: ToolApprovalPostureClass::AllowedWithoutPrompt,
        first_use_review_state_class: FirstUseReviewStateClass::NeverRequiredNativeTool,
        first_use_review_ticket_ref: String::new(),
        capability_classes: vec![
            ToolCapabilityClass::InspectWorkspaceFiles,
            ToolCapabilityClass::InspectWorkspaceSymbols,
        ],
        allowed_side_effect_classes: vec![ToolSideEffectClass::InspectOnly],
        allowed_data_classes: vec![
            RegistryDataClass::WorkspaceCodeSliceAllowed,
            RegistryDataClass::WorkspaceSymbolAllowed,
        ],
        denied_data_classes: vec![
            RegistryDataClass::CredentialHandleDeniedAlways,
            RegistryDataClass::SecretProjectionDeniedAlways,
        ],
        supersedes_descriptor_refs: Vec::new(),
        policy_context: policy_context(),
        explanation_label: "Native filesystem snapshot tool stays local and inspect-only."
            .to_owned(),
        minted_at: "2026-05-18T00:00:00Z".to_owned(),
        last_refreshed_at: None,
    }
}

fn mcp_descriptor() -> ToolGatewayDescriptor {
    ToolGatewayDescriptor {
        record_kind: TOOL_GATEWAY_DESCRIPTOR_RECORD_KIND.to_owned(),
        schema_version: TOOL_GATEWAY_SCHEMA_VERSION,
        descriptor_id: "tool-gateway-descriptor:mcp-fs-snapshot:0001".to_owned(),
        external_tool_entry_ref: "tool-entry:mcp_server:user_registered:fs_snapshot:0001"
            .to_owned(),
        display_label: "User-registered MCP filesystem snapshot tool".to_owned(),
        tool_family_label: "Filesystem snapshot MCP server".to_owned(),
        tool_capability_version: "fs_snapshot_mcp.v1".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::McpServerPublisher,
        publisher_identity_ref: "publisher-identity:mcp_server:fs_snapshot:0001".to_owned(),
        runtime_boundary_class: ToolRuntimeBoundaryClass::LocalSubprocessSameDevice,
        network_behavior_class: ToolNetworkBehaviorClass::LocalLoopbackOnly,
        credential_posture_class: ToolCredentialPostureClass::SignedManifestOnly,
        output_trust_posture_class: ToolOutputTrustPostureClass::AlwaysTaintedExternalToolOutput,
        availability_state_class: ToolAvailabilityStateClass::WarmAdmittedReady,
        lifecycle_state_class: ToolGatewayLifecycleStateClass::GenerallyAdmitted,
        approval_posture_class: ToolApprovalPostureClass::AllowedWithPerInvocationPrompt,
        first_use_review_state_class: FirstUseReviewStateClass::ApprovedForUse,
        first_use_review_ticket_ref: "approval-ticket:mcp_fs_snapshot:admission:0001".to_owned(),
        capability_classes: vec![
            ToolCapabilityClass::InspectWorkspaceFiles,
            ToolCapabilityClass::EditWorkspaceFilesReversible,
        ],
        allowed_side_effect_classes: vec![
            ToolSideEffectClass::InspectOnly,
            ToolSideEffectClass::LocalReversibleEdit,
        ],
        allowed_data_classes: vec![
            RegistryDataClass::WorkspaceCodeSliceAllowed,
            RegistryDataClass::WorkspaceSymbolAllowed,
        ],
        denied_data_classes: vec![
            RegistryDataClass::CredentialHandleDeniedAlways,
            RegistryDataClass::SecretProjectionDeniedAlways,
        ],
        supersedes_descriptor_refs: Vec::new(),
        policy_context: policy_context(),
        explanation_label: "MCP filesystem snapshot tool runs locally; output remains tainted."
            .to_owned(),
        minted_at: "2026-05-18T00:00:00Z".to_owned(),
        last_refreshed_at: None,
    }
}

fn remote_managed_descriptor() -> ToolGatewayDescriptor {
    ToolGatewayDescriptor {
        record_kind: TOOL_GATEWAY_DESCRIPTOR_RECORD_KIND.to_owned(),
        schema_version: TOOL_GATEWAY_SCHEMA_VERSION,
        descriptor_id: "tool-gateway-descriptor:remote-issues:0001".to_owned(),
        external_tool_entry_ref: "tool-entry:remote_http:user_registered:issues_api:0001"
            .to_owned(),
        display_label: "Enterprise gateway issues tool".to_owned(),
        tool_family_label: "Issue tracking connector".to_owned(),
        tool_capability_version: "issues_connector.v1".to_owned(),
        publisher_source_class: ToolPublisherSourceClass::EnterpriseRegisteredPublisher,
        publisher_identity_ref: "publisher-identity:enterprise:issues:0001".to_owned(),
        runtime_boundary_class: ToolRuntimeBoundaryClass::EnterpriseGatewayBrokeredService,
        network_behavior_class: ToolNetworkBehaviorClass::RemoteHttps,
        credential_posture_class: ToolCredentialPostureClass::EnterpriseGatewayManaged,
        output_trust_posture_class: ToolOutputTrustPostureClass::AlwaysTaintedRemoteServiceOutput,
        availability_state_class: ToolAvailabilityStateClass::ColdAdmitPendingHandshake,
        lifecycle_state_class: ToolGatewayLifecycleStateClass::PreviewAdmitted,
        approval_posture_class: ToolApprovalPostureClass::RequiresAdminApproval,
        first_use_review_state_class: FirstUseReviewStateClass::ApprovedForUse,
        first_use_review_ticket_ref: "approval-ticket:enterprise-issues:admission:0001".to_owned(),
        capability_classes: vec![
            ToolCapabilityClass::FetchExternalData,
            ToolCapabilityClass::PublishExternalComment,
        ],
        allowed_side_effect_classes: vec![
            ToolSideEffectClass::InspectOnly,
            ToolSideEffectClass::ExternalReversibleComment,
        ],
        allowed_data_classes: vec![
            RegistryDataClass::UserSuppliedTextAllowed,
            RegistryDataClass::AiPriorTurnContextAllowed,
        ],
        denied_data_classes: vec![
            RegistryDataClass::CredentialHandleDeniedAlways,
            RegistryDataClass::SecretProjectionDeniedAlways,
        ],
        supersedes_descriptor_refs: Vec::new(),
        policy_context: policy_context(),
        explanation_label:
            "Enterprise gateway issues tool runs remotely; output is always tainted.".to_owned(),
        minted_at: "2026-05-18T00:00:00Z".to_owned(),
        last_refreshed_at: None,
    }
}

fn inspect_only_timeline_entry() -> ToolCallTimelineEntry {
    ToolCallTimelineEntry {
        record_kind: TOOL_CALL_TIMELINE_ENTRY_RECORD_KIND.to_owned(),
        schema_version: TOOL_GATEWAY_SCHEMA_VERSION,
        tool_call_id: "tool-call:native-fs-snapshot:0001".to_owned(),
        descriptor_ref: "tool-gateway-descriptor:native-fs-snapshot:0001".to_owned(),
        runtime_boundary_class: ToolRuntimeBoundaryClass::LocalInProcess,
        boundary_label: "Local in-process native tool boundary".to_owned(),
        predicted_side_effect_class: ToolSideEffectClass::InspectOnly,
        observed_side_effect_class: ToolSideEffectClass::InspectOnly,
        outcome_class: ToolCallOutcomeClass::SucceededInspectOnly,
        outcome_summary_label: "Read-only workspace inspection succeeded.".to_owned(),
        taint_posture_class: ToolCallTaintPostureClass::TrustedFirstPartyLocalSigned,
        provenance_classification_state_class:
            ToolCallClassificationStateClass::ClassifiedEvidenceBacked,
        confidence_classification_state_class:
            ToolCallClassificationStateClass::ClassifiedEvidenceBacked,
        effect_classification_state_class:
            ToolCallClassificationStateClass::ClassifiedEvidenceBacked,
        data_classes_to_be_sent: vec![
            RegistryDataClass::WorkspaceCodeSliceAllowed,
            RegistryDataClass::WorkspaceSymbolAllowed,
        ],
        data_classes_returned: vec![RegistryDataClass::WorkspaceCodeSliceAllowed],
        inspect_action_ref: "action:tool-call:inspect:native-fs-snapshot:0001".to_owned(),
        remove_from_context_action_ref:
            "action:tool-call:remove-from-context:native-fs-snapshot:0001".to_owned(),
        replay_in_sandbox_action_ref: String::new(),
        renew_approval_action_ref: String::new(),
        originating_approval_ticket_ref: "approval-ticket:native-fs-snapshot:0001".to_owned(),
        originating_disclosure_ref: "disclosure:native-fs-snapshot:0001".to_owned(),
        tainted_context_fence_ref: String::new(),
        evidence_timeline_ref: "evidence-timeline:tool-gateway:0001".to_owned(),
        rerun_review_ref: "rerun-review:tool-gateway:0001".to_owned(),
        rollback_history_ref: String::new(),
        support_export_ref: "support-export:tool-gateway:0001".to_owned(),
        policy_context: policy_context(),
        emitted_at: "2026-05-18T00:00:01Z".to_owned(),
    }
}

fn mcp_local_edit_timeline_entry() -> ToolCallTimelineEntry {
    ToolCallTimelineEntry {
        record_kind: TOOL_CALL_TIMELINE_ENTRY_RECORD_KIND.to_owned(),
        schema_version: TOOL_GATEWAY_SCHEMA_VERSION,
        tool_call_id: "tool-call:mcp-fs-snapshot:0001".to_owned(),
        descriptor_ref: "tool-gateway-descriptor:mcp-fs-snapshot:0001".to_owned(),
        runtime_boundary_class: ToolRuntimeBoundaryClass::LocalSubprocessSameDevice,
        boundary_label: "Local subprocess MCP server boundary".to_owned(),
        predicted_side_effect_class: ToolSideEffectClass::InspectOnly,
        observed_side_effect_class: ToolSideEffectClass::LocalReversibleEdit,
        outcome_class: ToolCallOutcomeClass::SucceededWithLocalReversibleEdit,
        outcome_summary_label:
            "MCP filesystem snapshot tool applied a reversible local edit after review.".to_owned(),
        taint_posture_class: ToolCallTaintPostureClass::TaintedExternalToolOutputDefault,
        provenance_classification_state_class:
            ToolCallClassificationStateClass::UnclassifiedPendingReview,
        confidence_classification_state_class:
            ToolCallClassificationStateClass::UnclassifiedPendingReview,
        effect_classification_state_class:
            ToolCallClassificationStateClass::ClassifiedEvidenceBacked,
        data_classes_to_be_sent: vec![RegistryDataClass::WorkspaceCodeSliceAllowed],
        data_classes_returned: vec![RegistryDataClass::WorkspaceCodeSliceAllowed],
        inspect_action_ref: "action:tool-call:inspect:mcp-fs-snapshot:0001".to_owned(),
        remove_from_context_action_ref: "action:tool-call:remove-from-context:mcp-fs-snapshot:0001"
            .to_owned(),
        replay_in_sandbox_action_ref: "action:tool-call:replay-in-sandbox:mcp-fs-snapshot:0001"
            .to_owned(),
        renew_approval_action_ref: "action:tool-call:renew-approval:mcp-fs-snapshot:0001"
            .to_owned(),
        originating_approval_ticket_ref: "approval-ticket:mcp_fs_snapshot:per_invocation:0001"
            .to_owned(),
        originating_disclosure_ref: "disclosure:tool:mcp_fs_snapshot:0001".to_owned(),
        tainted_context_fence_ref: "tainted-fence:tool-call:mcp-fs-snapshot:0001".to_owned(),
        evidence_timeline_ref: "evidence-timeline:tool-gateway:0001".to_owned(),
        rerun_review_ref: "rerun-review:tool-gateway:0001".to_owned(),
        rollback_history_ref: "rollback-history:tool-gateway:0001".to_owned(),
        support_export_ref: "support-export:tool-gateway:0001".to_owned(),
        policy_context: policy_context(),
        emitted_at: "2026-05-18T00:00:02Z".to_owned(),
    }
}

fn remote_publish_denied_timeline_entry() -> ToolCallTimelineEntry {
    ToolCallTimelineEntry {
        record_kind: TOOL_CALL_TIMELINE_ENTRY_RECORD_KIND.to_owned(),
        schema_version: TOOL_GATEWAY_SCHEMA_VERSION,
        tool_call_id: "tool-call:remote-issues:0001".to_owned(),
        descriptor_ref: "tool-gateway-descriptor:remote-issues:0001".to_owned(),
        runtime_boundary_class: ToolRuntimeBoundaryClass::EnterpriseGatewayBrokeredService,
        boundary_label: "Enterprise gateway brokered remote tool boundary".to_owned(),
        predicted_side_effect_class: ToolSideEffectClass::ExternalReversibleComment,
        observed_side_effect_class: ToolSideEffectClass::InspectOnly,
        outcome_class: ToolCallOutcomeClass::DeniedByApprovalMissing,
        outcome_summary_label:
            "Enterprise gateway invocation denied because admin approval ticket was missing."
                .to_owned(),
        taint_posture_class: ToolCallTaintPostureClass::TaintedUnknownEffectClass,
        provenance_classification_state_class:
            ToolCallClassificationStateClass::UnclassifiedPendingReview,
        confidence_classification_state_class:
            ToolCallClassificationStateClass::UnclassifiedPendingReview,
        effect_classification_state_class:
            ToolCallClassificationStateClass::ClassifiedUnknownMustTreatAsTainted,
        data_classes_to_be_sent: vec![RegistryDataClass::UserSuppliedTextAllowed],
        data_classes_returned: Vec::new(),
        inspect_action_ref: "action:tool-call:inspect:remote-issues:0001".to_owned(),
        remove_from_context_action_ref: "action:tool-call:remove-from-context:remote-issues:0001"
            .to_owned(),
        replay_in_sandbox_action_ref: String::new(),
        renew_approval_action_ref: "action:tool-call:renew-approval:remote-issues:0001".to_owned(),
        originating_approval_ticket_ref: "approval-ticket:remote-issues:per_invocation:0001"
            .to_owned(),
        originating_disclosure_ref: "disclosure:remote-issues:0001".to_owned(),
        tainted_context_fence_ref: "tainted-fence:tool-call:remote-issues:0001".to_owned(),
        evidence_timeline_ref: "evidence-timeline:tool-gateway:0001".to_owned(),
        rerun_review_ref: String::new(),
        rollback_history_ref: String::new(),
        support_export_ref: "support-export:tool-gateway:0001".to_owned(),
        policy_context: policy_context(),
        emitted_at: "2026-05-18T00:00:03Z".to_owned(),
    }
}

fn surface_rows(
    descriptor_refs: Vec<String>,
    entry_refs: Vec<String>,
) -> Vec<ToolGatewaySurfaceRow> {
    vec![
        ToolGatewaySurfaceRow::new(
            ToolGatewaySurfaceClass::Composer,
            "projection:tool-gateway:composer:0001",
            descriptor_refs.clone(),
            entry_refs.clone(),
        ),
        ToolGatewaySurfaceRow::new(
            ToolGatewaySurfaceClass::ContextInspector,
            "projection:tool-gateway:context-inspector:0001",
            descriptor_refs.clone(),
            entry_refs.clone(),
        ),
        ToolGatewaySurfaceRow::new(
            ToolGatewaySurfaceClass::ReviewWorkspace,
            "projection:tool-gateway:review-workspace:0001",
            descriptor_refs.clone(),
            entry_refs.clone(),
        ),
        ToolGatewaySurfaceRow::new(
            ToolGatewaySurfaceClass::DocsHelp,
            "projection:tool-gateway:docs-help:0001",
            descriptor_refs.clone(),
            entry_refs.clone(),
        ),
        ToolGatewaySurfaceRow::new(
            ToolGatewaySurfaceClass::SupportExport,
            "projection:tool-gateway:support-export:0001",
            descriptor_refs,
            entry_refs,
        ),
    ]
}

fn build_packet() -> ToolGatewayConformancePacket {
    let descriptors = vec![
        local_native_descriptor(),
        mcp_descriptor(),
        remote_managed_descriptor(),
    ];
    let timeline_entries = vec![
        inspect_only_timeline_entry(),
        mcp_local_edit_timeline_entry(),
        remote_publish_denied_timeline_entry(),
    ];
    let descriptor_refs: Vec<String> = descriptors
        .iter()
        .map(|descriptor| descriptor.descriptor_id.clone())
        .collect();
    let entry_refs: Vec<String> = timeline_entries
        .iter()
        .map(|entry| entry.tool_call_id.clone())
        .collect();
    ToolGatewayConformancePacket::new(ToolGatewayConformancePacketInput {
        packet_id: "tool-gateway-conformance:m3:0001".to_owned(),
        display_label: "Tool gateway M3 conformance".to_owned(),
        descriptors,
        timeline_entries,
        surface_rows: surface_rows(descriptor_refs, entry_refs),
        source_contract_refs: vec![
            TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF.to_owned(),
            TOOL_CALL_TIMELINE_ENTRY_SCHEMA_REF.to_owned(),
            TOOL_GATEWAY_CONFORMANCE_ARTIFACT_REF.to_owned(),
        ],
        policy_context: policy_context(),
        minted_at: "2026-05-18T00:00:00Z".to_owned(),
    })
}

#[test]
fn canonical_packet_validates_clean() {
    let packet = build_packet();
    let violations = packet.validate();
    assert!(violations.is_empty(), "{violations:?}");
}

#[test]
fn export_excludes_raw_endpoint_material() {
    let packet = build_packet();
    let json = packet.export_safe_json();
    assert!(!json.contains("://"));
    assert!(!json.to_ascii_lowercase().contains("api_key"));
    assert!(!json.to_ascii_lowercase().contains("oauth_token"));
}

#[test]
fn markdown_summary_reports_boundary_and_taint_posture_coverage() {
    let packet = build_packet();
    let markdown = packet.render_markdown_summary();
    assert!(markdown.starts_with("# AI Tool-Gateway Conformance Report"));
    assert!(markdown.contains("local_in_process"));
    assert!(markdown.contains("local_subprocess_same_device"));
    assert!(markdown.contains("enterprise_gateway_brokered_service"));
    assert!(markdown.contains("trusted_first_party_local_signed"));
    assert!(markdown.contains("tainted_external_tool_output_default"));
}

#[test]
fn tainted_entry_without_fence_is_rejected() {
    let mut packet = build_packet();
    let entry = packet
        .timeline_entries
        .iter_mut()
        .find(|entry| entry.tool_call_id == "tool-call:mcp-fs-snapshot:0001")
        .expect("mcp entry exists");
    entry.tainted_context_fence_ref.clear();
    let violations = packet.validate();
    assert!(violations.contains(&ToolGatewayViolation::TimelineEntryTaintedWithoutFence));
}

#[test]
fn trusted_entry_must_be_local_and_classified() {
    let mut packet = build_packet();
    let entry = packet
        .timeline_entries
        .iter_mut()
        .find(|entry| entry.tool_call_id == "tool-call:native-fs-snapshot:0001")
        .expect("native entry exists");
    entry.runtime_boundary_class = ToolRuntimeBoundaryClass::RemoteVendorManagedService;
    let violations = packet.validate();
    assert!(violations.contains(&ToolGatewayViolation::TimelineEntryTrustedNotLocalBoundary));
}

#[test]
fn descriptor_with_unknown_disclosure_must_not_admit_invocations() {
    let mut packet = build_packet();
    let descriptor = packet
        .descriptors
        .iter_mut()
        .find(|descriptor| descriptor.descriptor_id == "tool-gateway-descriptor:remote-issues:0001")
        .expect("remote descriptor exists");
    descriptor.network_behavior_class = ToolNetworkBehaviorClass::UnknownMustDisclose;
    descriptor.availability_state_class = ToolAvailabilityStateClass::WarmAdmittedReady;
    let violations = packet.validate();
    assert!(violations.contains(&ToolGatewayViolation::DescriptorUnknownDisclosureMustBeNarrowed));
}

#[test]
fn descriptor_must_keep_credential_and_secret_data_classes_denied() {
    let mut packet = build_packet();
    let descriptor = packet
        .descriptors
        .iter_mut()
        .find(|descriptor| {
            descriptor.descriptor_id == "tool-gateway-descriptor:mcp-fs-snapshot:0001"
        })
        .expect("mcp descriptor exists");
    descriptor.denied_data_classes.clear();
    let violations = packet.validate();
    assert!(violations.contains(&ToolGatewayViolation::DescriptorMissingDeniedDataClasses));
}

#[test]
fn descriptor_must_not_admit_credential_or_secret_classes_as_input() {
    let mut packet = build_packet();
    let descriptor = packet
        .descriptors
        .iter_mut()
        .find(|descriptor| {
            descriptor.descriptor_id == "tool-gateway-descriptor:mcp-fs-snapshot:0001"
        })
        .expect("mcp descriptor exists");
    descriptor
        .allowed_data_classes
        .push(RegistryDataClass::CredentialHandleDeniedAlways);
    let violations = packet.validate();
    assert!(violations.contains(&ToolGatewayViolation::DescriptorAllowsForbiddenDataClass));
}

#[test]
fn timeline_entry_must_reference_a_known_descriptor() {
    let mut packet = build_packet();
    packet.timeline_entries[0].descriptor_ref = "tool-gateway-descriptor:does-not-exist".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&ToolGatewayViolation::TimelineEntryReferencesUnknownDescriptor));
}

#[test]
fn timeline_entry_side_effect_must_be_in_descriptor_allowlist() {
    let mut packet = build_packet();
    let entry = packet
        .timeline_entries
        .iter_mut()
        .find(|entry| entry.tool_call_id == "tool-call:native-fs-snapshot:0001")
        .expect("native entry exists");
    entry.predicted_side_effect_class = ToolSideEffectClass::ExternalIrreversiblePublish;
    let violations = packet.validate();
    assert!(violations.contains(&ToolGatewayViolation::TimelineEntryPredictedSideEffectNotAllowed));
}

#[test]
fn surface_projection_drift_is_detected() {
    let mut packet = build_packet();
    packet
        .surface_rows
        .iter_mut()
        .find(|row| row.surface_class == ToolGatewaySurfaceClass::Composer)
        .expect("composer surface row exists")
        .preserves_taint_posture = false;
    let violations = packet.validate();
    assert!(violations.contains(&ToolGatewayViolation::SurfaceProjectionDrift));
}

#[test]
fn descriptor_from_registry_entry_inherits_identity_and_denies_secrets() {
    let registry_entry = ExternalToolRegistryEntry {
        record_kind: PROVIDER_MODEL_REGISTRY_EXTERNAL_TOOL_ENTRY_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION,
        tool_entry_id: "tool-entry:remote_mcp:user_registered:fs_snapshot:0001".to_owned(),
        display_label: "User-registered MCP filesystem snapshot tool".to_owned(),
        tool_family_label: "Filesystem snapshot MCP server".to_owned(),
        tool_capability_version: "fs_snapshot_mcp.v1".to_owned(),
        tool_transport_class: ExternalToolTransportClass::LocalStdioSpawn,
        tool_execution_locus_class: ExternalToolExecutionLocusClass::LocalSubprocessSameDevice,
        tool_auth_mode_class: crate::registry::RegistryAuthModeClass::SignedManifestOnlyLocalPack,
        tool_output_trust_posture_label: "always_tainted_external_tool_output".to_owned(),
        allowed_side_effect_classes: vec![
            crate::registry::ExternalToolSideEffectClass::InspectOnly,
            crate::registry::ExternalToolSideEffectClass::LocalReversibleEdit,
        ],
        allowed_data_classes: vec![
            RegistryDataClass::WorkspaceCodeSliceAllowed,
            RegistryDataClass::WorkspaceSymbolAllowed,
        ],
        denied_data_classes: Vec::new(),
        policy_allowed_route_choices: vec![
            crate::registry::RegistryRoutingPolicyClass::CheapestQualifying,
        ],
        lifecycle_state_class: RegistryLifecycleStateClass::GenerallyAdmitted,
        approval_posture_class: RegistryApprovalPostureClass::AllowedWithPerInvocationPrompt,
        required_approval_ticket_ref: Some(
            "approval-ticket:mcp_fs_snapshot:admission:0001".to_owned(),
        ),
        explanation_label: "MCP filesystem snapshot tool runs locally; output remains tainted."
            .to_owned(),
    };

    let descriptor = ToolGatewayDescriptor::from_registry_entry(
        "tool-gateway-descriptor:mcp-fs-snapshot:0001",
        &registry_entry,
        ToolPublisherSourceClass::McpServerPublisher,
        "publisher-identity:mcp_server:fs_snapshot:0001",
        ToolRuntimeBoundaryClass::LocalSubprocessSameDevice,
        ToolNetworkBehaviorClass::LocalLoopbackOnly,
        ToolCredentialPostureClass::SignedManifestOnly,
        ToolOutputTrustPostureClass::AlwaysTaintedExternalToolOutput,
        ToolAvailabilityStateClass::WarmAdmittedReady,
        FirstUseReviewStateClass::ApprovedForUse,
        vec![ToolCapabilityClass::InspectWorkspaceFiles],
        policy_context(),
        "2026-05-18T00:00:00Z",
    );

    assert_eq!(
        descriptor.external_tool_entry_ref,
        "tool-entry:remote_mcp:user_registered:fs_snapshot:0001"
    );
    assert!(descriptor
        .denied_data_classes
        .contains(&RegistryDataClass::CredentialHandleDeniedAlways));
    assert!(descriptor
        .denied_data_classes
        .contains(&RegistryDataClass::SecretProjectionDeniedAlways));
    assert!(descriptor.has_mutating_side_effect());
    assert!(descriptor.outputs_are_tainted_by_default());
    assert!(descriptor.is_local_boundary());
    assert!(descriptor.admits_material_run());
}

#[test]
fn checked_in_fixture_matches_canonical_packet() {
    let packet = current_beta_tool_gateway_conformance_packet()
        .expect("checked-in tool gateway conformance fixture validates");
    assert_eq!(packet.packet_id, "tool-gateway-conformance:m3:0001");
    let canonical = build_packet();
    assert_eq!(packet, canonical);
}
