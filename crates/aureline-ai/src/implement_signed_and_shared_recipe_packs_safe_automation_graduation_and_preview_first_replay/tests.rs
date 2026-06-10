use super::*;

const PACKET_ID: &str = "recipe-pack:stable:0001";

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

fn proof_freshness() -> RecipePackProofFreshness {
    RecipePackProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> RecipePackGraduationPacket {
    RecipePackGraduationPacket::new(RecipePackGraduationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "Signed And Shared Recipe Packs".to_owned(),
        packs: vec![
            org_template_pack(),
            byok_publish_pack(),
            local_inspect_pack(),
            quarantined_pack(),
        ],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    })
}

fn assert_has(packet: &RecipePackGraduationPacket, expected: RecipePackViolation) {
    let violations = packet.validate();
    assert!(
        violations.contains(&expected),
        "expected {:?}, got {:?}",
        expected,
        violations
    );
}

#[test]
fn recipe_pack_packet_validates() {
    assert!(packet().validate().is_empty(), "{:?}", packet().validate());
}

#[test]
fn round_trips_through_json() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: RecipePackGraduationPacket = serde_json::from_str(&json).expect("packet parses");
    assert_eq!(packet, parsed);
}

#[test]
fn counts_match_fixture() {
    let packet = packet();
    // Three claimed packs (Stable, Beta, Preview); the Held quarantined one is not.
    assert_eq!(packet.claimed_pack_count(), 3);
    // The quarantined pack is the only blocked one.
    assert_eq!(packet.blocked_pack_count(), 1);
    // Org, BYOK, and quarantined packs carry mutating effects; local is inspect-only.
    assert_eq!(packet.mutating_pack_count(), 3);
    // Every pack except the unsigned local one is signed.
    assert_eq!(packet.signed_pack_count(), 3);
}

#[test]
fn preview_partition_holds() {
    assert!(ReplayPreviewClass::FullPreviewBeforeReplay.previews_before_replay());
    assert!(ReplayPreviewClass::DiffPreviewBeforeReplay.previews_before_replay());
    assert!(ReplayPreviewClass::DryRunPreviewBeforeReplay.previews_before_replay());
    assert!(!ReplayPreviewClass::InspectOnlyNoPreviewNeeded.previews_before_replay());
    assert!(!ReplayPreviewClass::PreviewUnavailableMustBlock.previews_before_replay());
}

#[test]
fn audit_partition_holds() {
    assert!(RecipeStepAuditClass::AuditedToRunRecordTimeline.is_externally_auditable());
    assert!(RecipeStepAuditClass::AuditedToSupportExport.is_externally_auditable());
    assert!(!RecipeStepAuditClass::AuditedLocalHistoryOnly.is_externally_auditable());
    assert!(RecipeStepAuditClass::AuditedLocalHistoryOnly.is_audited());
    assert!(!RecipeStepAuditClass::NotAudited.is_audited());
}

#[test]
fn authority_partition_holds() {
    assert!(!AutomationAuthorityClass::InspectOnlyNoAuthority.grants_mutation());
    assert!(AutomationAuthorityClass::LocalReversibleOnly.grants_mutation());
    assert!(AutomationAuthorityClass::ExternalIrreversibleAdminGated.admits_irreversible_publish());
    assert!(AutomationAuthorityClass::ManagedOnlyTemplateAuthority.admits_irreversible_publish());
    assert!(!AutomationAuthorityClass::ExternalReversibleWithApproval.admits_irreversible_publish());
    assert!(AutomationAuthorityClass::ManagedOnlyTemplateAuthority.requires_managed_mode());
    assert!(!AutomationAuthorityClass::LocalWithApproval.requires_managed_mode());
}

#[test]
fn signature_and_scope_partition_holds() {
    assert!(RecipePackSignatureClass::AuthorSignature.is_signed());
    assert!(!RecipePackSignatureClass::UnsignedLocalOnly.is_signed());
    assert!(RecipePackSignatureClass::ManagedOnlyChannelSignature.carries_organization_authority());
    assert!(!RecipePackSignatureClass::AuthorSignature.carries_organization_authority());
    assert!(RecipePackShareScopeClass::UserScopeLocalOnly.is_local_only());
    assert!(RecipePackShareScopeClass::OrganizationManagedChannel.is_shared_beyond_workspace());
    assert!(RecipePackShareScopeClass::OrganizationManagedChannel.requires_managed_channel());
    assert!(!RecipePackShareScopeClass::PortableProfileExport.requires_managed_channel());
}

#[test]
fn state_blocked_partition_holds() {
    assert!(RecipePackStateClass::Admitted.admits_replay());
    assert!(!RecipePackStateClass::PendingFirstUseReview.admits_replay());
    for blocked in [
        RecipePackStateClass::PolicyBlocked,
        RecipePackStateClass::TrustBlocked,
        RecipePackStateClass::QuarantinedSignature,
        RecipePackStateClass::Withdrawn,
    ] {
        assert!(blocked.is_blocked());
        assert!(!blocked.admits_replay());
    }
}

#[test]
fn preview_first_holds_for_mutating_packs() {
    assert!(org_template_pack().is_preview_first());
    assert!(byok_publish_pack().is_preview_first());
    // Inspect-only packs are vacuously preview-first.
    assert!(local_inspect_pack().is_preview_first());
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "other".to_owned();
    assert_has(&packet, RecipePackViolation::WrongRecordKind);
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = packet();
    packet.schema_version = 99;
    assert_has(&packet, RecipePackViolation::WrongSchemaVersion);
}

#[test]
fn missing_identity_fails() {
    let mut packet = packet();
    packet.packet_id = "  ".to_owned();
    assert_has(&packet, RecipePackViolation::MissingIdentity);
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs = vec![RECIPE_PACK_SCHEMA_REF.to_owned()];
    assert_has(&packet, RecipePackViolation::MissingSourceContracts);
}

#[test]
fn no_packs_fails() {
    let mut packet = packet();
    packet.packs.clear();
    assert_has(&packet, RecipePackViolation::NoPacks);
}

#[test]
fn duplicate_pack_fails() {
    let mut packet = packet();
    packet.packs.push(org_template_pack());
    assert_has(&packet, RecipePackViolation::DuplicatePack);
}

#[test]
fn pack_row_incomplete_fails() {
    let mut packet = packet();
    packet.packs[0].pack_label = "  ".to_owned();
    assert_has(&packet, RecipePackViolation::PackRowIncomplete);
}

#[test]
fn pack_missing_content_address_fails() {
    let mut packet = packet();
    packet.packs[0].manifest_content_address = "  ".to_owned();
    assert_has(&packet, RecipePackViolation::PackMissingContentAddress);
}

#[test]
fn signed_pack_missing_publisher_identity_fails() {
    let mut packet = packet();
    packet.packs[0].publisher_identity_ref = "  ".to_owned();
    assert_has(
        &packet,
        RecipePackViolation::SignedPackMissingPublisherIdentity,
    );
}

#[test]
fn shared_pack_must_be_signed_fails() {
    let mut packet = packet();
    // Push the BYOK portable-export pack into the unsigned class; it stays shared.
    packet.packs[1].signature_class = RecipePackSignatureClass::UnsignedLocalOnly;
    // The unsigned MCP publisher still requires an identity, so isolate the
    // shared-must-be-signed violation by clearing the publisher identity too is
    // not needed; assert the specific violation is present.
    assert_has(&packet, RecipePackViolation::SharedPackMustBeSigned);
}

#[test]
fn managed_channel_scope_requires_managed_mode_fails() {
    let mut packet = packet();
    // The org template rides the managed channel; force a local mode.
    packet.packs[0].resolved_mode = RoutePolicyModeClass::Local;
    assert_has(
        &packet,
        RecipePackViolation::ManagedChannelScopeRequiresManagedMode,
    );
}

#[test]
fn managed_template_authority_requires_managed_mode_fails() {
    let mut packet = packet();
    // Move the BYOK pack to the managed-only template authority on a BYOK mode.
    packet.packs[1].automation_authority_class =
        AutomationAuthorityClass::ManagedOnlyTemplateAuthority;
    assert_has(
        &packet,
        RecipePackViolation::ManagedTemplateAuthorityRequiresManagedMode,
    );
}

#[test]
fn pack_missing_step_disclosures_fails() {
    let mut packet = packet();
    packet.packs[2].step_disclosures.clear();
    assert_has(&packet, RecipePackViolation::PackMissingStepDisclosures);
}

#[test]
fn step_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet.packs[0].step_disclosures[1].disclosure_label = "  ".to_owned();
    assert_has(&packet, RecipePackViolation::StepDisclosureIncomplete);
}

#[test]
fn duplicate_step_disclosure_fails() {
    let mut packet = packet();
    let dup = packet.packs[0].step_disclosures[0].clone();
    packet.packs[0].step_disclosures.push(dup);
    assert_has(&packet, RecipePackViolation::DuplicateStepDisclosure);
}

#[test]
fn mutating_step_without_preview_first_replay_fails() {
    let mut packet = packet();
    packet.packs[0].step_disclosures[1].replay_preview =
        ReplayPreviewClass::PreviewUnavailableMustBlock;
    assert_has(
        &packet,
        RecipePackViolation::MutatingStepWithoutPreviewFirstReplay,
    );
}

#[test]
fn mutating_step_without_approval_fails() {
    let mut packet = packet();
    packet.packs[0].step_disclosures[1].approval_posture =
        ToolApprovalPostureClass::AllowedWithoutPrompt;
    assert_has(&packet, RecipePackViolation::MutatingStepWithoutApproval);
}

#[test]
fn mutating_step_without_audit_fails() {
    let mut packet = packet();
    packet.packs[0].step_disclosures[1].audit = RecipeStepAuditClass::NotAudited;
    assert_has(&packet, RecipePackViolation::MutatingStepWithoutAudit);
}

#[test]
fn inspect_only_authority_has_mutating_step_fails() {
    let mut packet = packet();
    // The local pack is inspect-only authority; add a mutating step to it.
    packet.packs[2].step_disclosures.push(RecipeStepDisclosure {
        side_effect_class: ToolSideEffectClass::LocalReversibleEdit,
        replay_preview: ReplayPreviewClass::DiffPreviewBeforeReplay,
        approval_posture: ToolApprovalPostureClass::AllowedWithPerInvocationPrompt,
        audit: RecipeStepAuditClass::AuditedToRunRecordTimeline,
        reversibility: RecipeStepReversibilityClass::ReversibleInWorkspace,
        disclosure_label: "Edits a file; previewed and reversible".to_owned(),
    });
    assert_has(
        &packet,
        RecipePackViolation::InspectOnlyAuthorityHasMutatingStep,
    );
}

#[test]
fn irreversible_publish_without_admin_authority_fails() {
    let mut packet = packet();
    // The BYOK pack discloses an irreversible publish; drop it to a reversible
    // authority that cannot graduate that effect.
    packet.packs[1].automation_authority_class =
        AutomationAuthorityClass::ExternalReversibleWithApproval;
    assert_has(
        &packet,
        RecipePackViolation::IrreversiblePublishWithoutAdminAuthority,
    );
}

#[test]
fn irreversible_publish_not_externally_audited_fails() {
    let mut packet = packet();
    packet.packs[1].step_disclosures[1].audit = RecipeStepAuditClass::AuditedLocalHistoryOnly;
    assert_has(
        &packet,
        RecipePackViolation::IrreversiblePublishNotExternallyAudited,
    );
}

#[test]
fn step_reversibility_mismatch_fails() {
    let mut packet = packet();
    // Inspect-only must declare no side effect.
    packet.packs[2].step_disclosures[0].reversibility =
        RecipeStepReversibilityClass::ReversibleInWorkspace;
    assert_has(&packet, RecipePackViolation::StepReversibilityMismatch);
}

#[test]
fn blocked_pack_claims_qualification_fails() {
    let mut packet = packet();
    packet.packs[3].claimed_qualification = M5AiWorkflowQualificationClass::Beta;
    assert_has(&packet, RecipePackViolation::BlockedPackClaimsQualification);
}

#[test]
fn pending_review_claims_stable_fails() {
    let mut packet = packet();
    packet.packs[0].state = RecipePackStateClass::PendingFirstUseReview;
    assert_has(&packet, RecipePackViolation::PendingReviewClaimsStable);
}

#[test]
fn claimed_pack_missing_evidence_fails() {
    let mut packet = packet();
    packet.packs[0].evidence_packet_refs.clear();
    assert_has(&packet, RecipePackViolation::ClaimedPackMissingEvidence);
}

#[test]
fn claimed_rollback_unverified_fails() {
    let mut packet = packet();
    packet.packs[0].rollback_verified = false;
    assert_has(&packet, RecipePackViolation::ClaimedRollbackUnverified);
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.packs[0].downgrade_rules.clear();
    assert_has(&packet, RecipePackViolation::DowngradeRulesMissing);
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.packs[0].downgrade_rules =
        vec![provider_unavailable(M5AiWorkflowQualificationClass::Beta)];
    assert_has(&packet, RecipePackViolation::DowngradeRuleMissingProofStale);
}

#[test]
fn downgrade_rule_missing_provider_unavailable_fails() {
    let mut packet = packet();
    packet.packs[0].downgrade_rules = vec![proof_stale(M5AiWorkflowQualificationClass::Beta)];
    assert_has(
        &packet,
        RecipePackViolation::DowngradeRuleMissingProviderUnavailable,
    );
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    // Narrowing "to" Stable is not below a Stable claim.
    packet.packs[0].downgrade_rules = vec![
        proof_stale(M5AiWorkflowQualificationClass::Stable),
        provider_unavailable(M5AiWorkflowQualificationClass::Beta),
    ];
    assert_has(&packet, RecipePackViolation::DowngradeRuleNotNarrowing);
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert_has(&packet, RecipePackViolation::ProofFreshnessIncomplete);
}

#[test]
fn raw_automation_material_in_export_fails() {
    let mut packet = packet();
    packet.packs[0].explanation_label = "runs $(curl https://example.com)".to_owned();
    assert_has(&packet, RecipePackViolation::RawAutomationMaterialInExport);
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let pack = byok_publish_pack();
    assert_eq!(
        pack.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Preview
    );
    assert_eq!(
        pack.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Held
    );
    // An unmatched trigger leaves the claim unchanged.
    assert_eq!(
        pack.narrowed_qualification(M5AiWorkflowDowngradeTrigger::TrustNarrowing),
        M5AiWorkflowQualificationClass::Beta
    );
}

#[test]
fn render_inspector_lists_every_disclosure() {
    let pack = org_template_pack();
    let card = pack.render_inspector();
    assert!(card.contains("external_reversible_comment"));
    assert!(card.contains("full_preview_before_replay"));
    assert!(card.contains("audited_to_run_record_timeline"));
    assert!(card.contains("managed_only_template_authority"));
}

#[test]
fn markdown_summary_lists_every_pack() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("org-review-template"));
    assert!(summary.contains("byok-issue-publish"));
    assert!(summary.contains("local-symbol-inspector"));
    assert!(summary.contains("quarantined-deploy"));
}

#[test]
fn blocked_pack_narrows_fixture_validates() {
    let packet: RecipePackGraduationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/blocked_pack_narrows.json"
    )))
    .expect("blocked pack fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The quarantined pack dropped out of every claimed lane.
    let quarantined = packet
        .pack("quarantined-deploy")
        .expect("quarantined pack present");
    assert!(!quarantined.is_claimed());
    assert!(quarantined.state.is_blocked());
    assert_eq!(
        quarantined.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Unavailable
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_recipe_pack_export().expect("checked recipe pack export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.packs.is_empty());
    // Every mutating pack previews before replay, gates, and audits its effects.
    for pack in &packet.packs {
        assert!(pack.is_preview_first());
        for disclosure in &pack.step_disclosures {
            if disclosure.is_mutating() {
                assert!(disclosure.replay_preview.previews_before_replay());
                assert!(disclosure.has_approval_gate());
                assert!(disclosure.audit.is_audited());
            }
        }
    }
    // The quarantined pack dropped out of every claimed lane and is blocked.
    let quarantined = packet
        .pack("quarantined-deploy")
        .expect("quarantined pack present");
    assert!(!quarantined.is_claimed());
    assert!(quarantined.state.is_blocked());
}
