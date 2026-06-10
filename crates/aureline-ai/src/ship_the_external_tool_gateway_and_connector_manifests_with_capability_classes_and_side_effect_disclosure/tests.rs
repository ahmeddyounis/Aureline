use super::*;

const PACKET_ID: &str = "connector-manifest:stable:0001";

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

fn managed_stable_connector() -> ConnectorManifestRow {
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

fn local_preview_connector() -> ConnectorManifestRow {
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

fn proof_freshness() -> ConnectorManifestProofFreshness {
    ConnectorManifestProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> ConnectorManifestPacket {
    ConnectorManifestPacket::new(ConnectorManifestPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "External-Tool Gateway Connector Manifests".to_owned(),
        connectors: vec![
            managed_stable_connector(),
            byok_publish_connector(),
            local_preview_connector(),
            quarantined_connector(),
        ],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    })
}

fn assert_has(packet: &ConnectorManifestPacket, expected: ConnectorManifestViolation) {
    let violations = packet.validate();
    assert!(
        violations.contains(&expected),
        "expected {:?}, got {:?}",
        expected,
        violations
    );
}

#[test]
fn connector_packet_validates() {
    assert!(packet().validate().is_empty(), "{:?}", packet().validate());
}

#[test]
fn round_trips_through_json() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: ConnectorManifestPacket = serde_json::from_str(&json).expect("packet parses");
    assert_eq!(packet, parsed);
}

#[test]
fn counts_match_fixture() {
    let packet = packet();
    // Three claimed connectors (Stable, Beta, Preview); Held quarantined one is not.
    assert_eq!(packet.claimed_connector_count(), 3);
    // The quarantined connector is the only blocked one.
    assert_eq!(packet.blocked_connector_count(), 1);
    // Managed, BYOK, and quarantined connectors carry mutating effects; local is inspect-only.
    assert_eq!(packet.mutating_connector_count(), 3);
}

#[test]
fn preview_partition_holds() {
    assert!(SideEffectPreviewClass::PreviewRequiredBeforeApply.previews_before_apply());
    assert!(SideEffectPreviewClass::DiffPreviewBeforeApply.previews_before_apply());
    assert!(SideEffectPreviewClass::DryRunPreviewBeforeApply.previews_before_apply());
    assert!(!SideEffectPreviewClass::InspectOnlyNoPreviewNeeded.previews_before_apply());
    assert!(!SideEffectPreviewClass::PreviewUnavailableMustBlock.previews_before_apply());
}

#[test]
fn audit_partition_holds() {
    assert!(SideEffectAuditClass::AuditedToEvidenceTimeline.is_externally_auditable());
    assert!(SideEffectAuditClass::AuditedToSupportExport.is_externally_auditable());
    assert!(!SideEffectAuditClass::AuditedLocalHistoryOnly.is_externally_auditable());
    assert!(SideEffectAuditClass::AuditedLocalHistoryOnly.is_audited());
    assert!(!SideEffectAuditClass::NotAudited.is_audited());
}

#[test]
fn state_blocked_partition_holds() {
    assert!(ConnectorManifestStateClass::Admitted.admits_invocation());
    assert!(!ConnectorManifestStateClass::PendingFirstUseReview.admits_invocation());
    for blocked in [
        ConnectorManifestStateClass::PolicyBlocked,
        ConnectorManifestStateClass::TrustBlocked,
        ConnectorManifestStateClass::QuarantinedSignature,
        ConnectorManifestStateClass::Withdrawn,
    ] {
        assert!(blocked.is_blocked());
        assert!(!blocked.admits_invocation());
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "other".to_owned();
    assert_has(&packet, ConnectorManifestViolation::WrongRecordKind);
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = packet();
    packet.schema_version = 99;
    assert_has(&packet, ConnectorManifestViolation::WrongSchemaVersion);
}

#[test]
fn missing_identity_fails() {
    let mut packet = packet();
    packet.packet_id = "  ".to_owned();
    assert_has(&packet, ConnectorManifestViolation::MissingIdentity);
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs = vec![CONNECTOR_MANIFEST_SCHEMA_REF.to_owned()];
    assert_has(&packet, ConnectorManifestViolation::MissingSourceContracts);
}

#[test]
fn no_connectors_fails() {
    let mut packet = packet();
    packet.connectors.clear();
    assert_has(&packet, ConnectorManifestViolation::NoConnectors);
}

#[test]
fn duplicate_connector_fails() {
    let mut packet = packet();
    packet.connectors.push(managed_stable_connector());
    assert_has(&packet, ConnectorManifestViolation::DuplicateConnector);
}

#[test]
fn connector_row_incomplete_fails() {
    let mut packet = packet();
    packet.connectors[0].connector_label = "  ".to_owned();
    assert_has(&packet, ConnectorManifestViolation::ConnectorRowIncomplete);
}

#[test]
fn connector_missing_publisher_identity_fails() {
    let mut packet = packet();
    packet.connectors[0].publisher_identity_ref = "  ".to_owned();
    assert_has(
        &packet,
        ConnectorManifestViolation::ConnectorMissingPublisherIdentity,
    );
}

#[test]
fn connector_missing_capabilities_fails() {
    let mut packet = packet();
    packet.connectors[2].capability_classes.clear();
    assert_has(
        &packet,
        ConnectorManifestViolation::ConnectorMissingCapabilities,
    );
}

#[test]
fn connector_missing_side_effect_disclosures_fails() {
    let mut packet = packet();
    packet.connectors[2].side_effect_disclosures.clear();
    assert_has(
        &packet,
        ConnectorManifestViolation::ConnectorMissingSideEffectDisclosures,
    );
}

#[test]
fn side_effect_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet.connectors[0].side_effect_disclosures[1].disclosure_label = "  ".to_owned();
    assert_has(
        &packet,
        ConnectorManifestViolation::SideEffectDisclosureIncomplete,
    );
}

#[test]
fn duplicate_side_effect_disclosure_fails() {
    let mut packet = packet();
    let dup = packet.connectors[0].side_effect_disclosures[0].clone();
    packet.connectors[0].side_effect_disclosures.push(dup);
    assert_has(
        &packet,
        ConnectorManifestViolation::DuplicateSideEffectDisclosure,
    );
}

#[test]
fn mutating_side_effect_without_preview_fails() {
    let mut packet = packet();
    packet.connectors[0].side_effect_disclosures[1].preview =
        SideEffectPreviewClass::PreviewUnavailableMustBlock;
    assert_has(
        &packet,
        ConnectorManifestViolation::MutatingSideEffectWithoutPreview,
    );
}

#[test]
fn mutating_side_effect_without_approval_fails() {
    let mut packet = packet();
    packet.connectors[0].side_effect_disclosures[1].approval_posture =
        ToolApprovalPostureClass::AllowedWithoutPrompt;
    assert_has(
        &packet,
        ConnectorManifestViolation::MutatingSideEffectWithoutApproval,
    );
}

#[test]
fn mutating_side_effect_without_audit_fails() {
    let mut packet = packet();
    packet.connectors[0].side_effect_disclosures[1].audit = SideEffectAuditClass::NotAudited;
    assert_has(
        &packet,
        ConnectorManifestViolation::MutatingSideEffectWithoutAudit,
    );
}

#[test]
fn irreversible_publish_not_externally_audited_fails() {
    let mut packet = packet();
    packet.connectors[1].side_effect_disclosures[1].audit =
        SideEffectAuditClass::AuditedLocalHistoryOnly;
    assert_has(
        &packet,
        ConnectorManifestViolation::IrreversiblePublishNotExternallyAudited,
    );
}

#[test]
fn side_effect_reversibility_mismatch_fails() {
    let mut packet = packet();
    // Inspect-only must declare no side effect.
    packet.connectors[2].side_effect_disclosures[0].reversibility =
        SideEffectReversibilityClass::ReversibleInWorkspace;
    assert_has(
        &packet,
        ConnectorManifestViolation::SideEffectReversibilityMismatch,
    );
}

#[test]
fn trusted_output_requires_local_boundary_fails() {
    let mut packet = packet();
    // Push the trusted-output local connector onto a remote boundary; the
    // network behavior stays local so only the boundary invariant trips.
    packet.connectors[2].runtime_boundary_class =
        ToolRuntimeBoundaryClass::RemoteVendorManagedService;
    assert_has(
        &packet,
        ConnectorManifestViolation::TrustedOutputRequiresLocalBoundary,
    );
}

#[test]
fn trusted_output_requires_signed_publisher_fails() {
    let mut packet = packet();
    packet.connectors[2].publisher_identity_ref = "  ".to_owned();
    assert_has(
        &packet,
        ConnectorManifestViolation::TrustedOutputRequiresSignedPublisher,
    );
}

#[test]
fn local_connector_advertises_remote_network_fails() {
    let mut packet = packet();
    packet.connectors[2].network_behavior_class = ToolNetworkBehaviorClass::RemoteHttps;
    assert_has(
        &packet,
        ConnectorManifestViolation::LocalConnectorAdvertisesRemoteNetwork,
    );
}

#[test]
fn remote_connector_output_not_tainted_fails() {
    let mut packet = packet();
    packet.connectors[0].output_trust_posture_class =
        ToolOutputTrustPostureClass::TrustedFirstPartyLocalToolOutput;
    assert_has(
        &packet,
        ConnectorManifestViolation::RemoteConnectorOutputNotTainted,
    );
}

#[test]
fn blocked_connector_claims_qualification_fails() {
    let mut packet = packet();
    packet.connectors[3].claimed_qualification = M5AiWorkflowQualificationClass::Beta;
    assert_has(
        &packet,
        ConnectorManifestViolation::BlockedConnectorClaimsQualification,
    );
}

#[test]
fn pending_review_claims_stable_fails() {
    let mut packet = packet();
    packet.connectors[0].state = ConnectorManifestStateClass::PendingFirstUseReview;
    assert_has(
        &packet,
        ConnectorManifestViolation::PendingReviewClaimsStable,
    );
}

#[test]
fn claimed_connector_missing_evidence_fails() {
    let mut packet = packet();
    packet.connectors[0].evidence_packet_refs.clear();
    assert_has(
        &packet,
        ConnectorManifestViolation::ClaimedConnectorMissingEvidence,
    );
}

#[test]
fn claimed_rollback_unverified_fails() {
    let mut packet = packet();
    packet.connectors[0].rollback_verified = false;
    assert_has(
        &packet,
        ConnectorManifestViolation::ClaimedRollbackUnverified,
    );
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.connectors[0].downgrade_rules.clear();
    assert_has(&packet, ConnectorManifestViolation::DowngradeRulesMissing);
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.connectors[0].downgrade_rules =
        vec![provider_unavailable(M5AiWorkflowQualificationClass::Beta)];
    assert_has(
        &packet,
        ConnectorManifestViolation::DowngradeRuleMissingProofStale,
    );
}

#[test]
fn downgrade_rule_missing_provider_unavailable_fails() {
    let mut packet = packet();
    packet.connectors[0].downgrade_rules = vec![proof_stale(M5AiWorkflowQualificationClass::Beta)];
    assert_has(
        &packet,
        ConnectorManifestViolation::DowngradeRuleMissingProviderUnavailable,
    );
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    // Narrowing "to" Stable is not below a Stable claim.
    packet.connectors[0].downgrade_rules = vec![
        proof_stale(M5AiWorkflowQualificationClass::Stable),
        provider_unavailable(M5AiWorkflowQualificationClass::Beta),
    ];
    assert_has(
        &packet,
        ConnectorManifestViolation::DowngradeRuleNotNarrowing,
    );
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert_has(
        &packet,
        ConnectorManifestViolation::ProofFreshnessIncomplete,
    );
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = packet();
    packet.connectors[0].explanation_label = "reaches https://api.example.com".to_owned();
    assert_has(
        &packet,
        ConnectorManifestViolation::RawBoundaryMaterialInExport,
    );
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let connector = byok_publish_connector();
    assert_eq!(
        connector.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Preview
    );
    assert_eq!(
        connector.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Held
    );
    // An unmatched trigger leaves the claim unchanged.
    assert_eq!(
        connector.narrowed_qualification(M5AiWorkflowDowngradeTrigger::TrustNarrowing),
        M5AiWorkflowQualificationClass::Beta
    );
}

#[test]
fn render_inspector_lists_every_disclosure() {
    let connector = managed_stable_connector();
    let card = connector.render_inspector();
    assert!(card.contains("external_reversible_comment"));
    assert!(card.contains("preview_required_before_apply"));
    assert!(card.contains("audited_to_evidence_timeline"));
    assert!(card.contains("fetch_external_data"));
}

#[test]
fn markdown_summary_lists_every_connector() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("managed-review-comment"));
    assert!(summary.contains("byok-issue-publish"));
    assert!(summary.contains("local-symbol-inspector"));
    assert!(summary.contains("quarantined-deploy"));
}

#[test]
fn blocked_connector_narrows_fixture_validates() {
    let packet: ConnectorManifestPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/blocked_connector_narrows.json"
    )))
    .expect("blocked connector fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The quarantined connector dropped out of every claimed lane.
    let quarantined = packet
        .connector("quarantined-deploy")
        .expect("quarantined connector present");
    assert!(!quarantined.is_claimed());
    assert!(quarantined.state.is_blocked());
    assert_eq!(
        quarantined.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Unavailable
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_connector_manifest_export().expect("checked connector export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.connectors.is_empty());
    // Every mutating connector previews, gates, and audits its mutating effects.
    for connector in &packet.connectors {
        for disclosure in &connector.side_effect_disclosures {
            if disclosure.is_mutating() {
                assert!(disclosure.preview.previews_before_apply());
                assert!(disclosure.has_approval_gate());
                assert!(disclosure.audit.is_audited());
            }
        }
    }
    // The quarantined connector dropped out of every claimed lane and is blocked.
    let quarantined = packet
        .connector("quarantined-deploy")
        .expect("quarantined connector present");
    assert!(!quarantined.is_claimed());
    assert!(quarantined.state.is_blocked());
}
