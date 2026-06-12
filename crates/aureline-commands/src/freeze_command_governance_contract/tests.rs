use super::*;

const PACKET_ID: &str = "command-governance-contract:stable:0001";

fn descriptor_fields() -> Vec<DescriptorFieldRow> {
    CommandDescriptorFieldClass::required_coverage()
        .into_iter()
        .map(|field_class| DescriptorFieldRow {
            field_class,
            field_label: format!("Descriptor field: {}", field_class.as_str()),
            source_pointer: format!("#/descriptor/{}", field_class.as_str()),
            exported: true,
        })
        .collect()
}

fn invocation_session_fields() -> Vec<InvocationSessionFieldRow> {
    InvocationSessionFieldClass::required_coverage()
        .into_iter()
        .map(|field_class| InvocationSessionFieldRow {
            field_class,
            field_label: format!("Invocation field: {}", field_class.as_str()),
            source_pointer: format!("#/invocation/{}", field_class.as_str()),
            required_on_every_surface: true,
        })
        .collect()
}

fn result_packet_fields() -> Vec<ResultPacketFieldRow> {
    ResultPacketFieldClass::required_coverage()
        .into_iter()
        .map(|field_class| ResultPacketFieldRow {
            field_class,
            field_label: format!("Result field: {}", field_class.as_str()),
            source_pointer: format!("#/result/{}", field_class.as_str()),
            required_on_every_result: true,
        })
        .collect()
}

fn lifecycle_dependency_vocabulary() -> Vec<LifecycleDependencyVocabularyRow> {
    LifecycleDependencyClass::required_coverage()
        .into_iter()
        .map(|dependency_class| LifecycleDependencyVocabularyRow {
            dependency_class,
            disclosure_label: format!("Dependency disclosure: {}", dependency_class.as_str()),
            source_pointer: format!("#/lifecycle/{}", dependency_class.as_str()),
            narrows_stable_claim: true,
        })
        .collect()
}

fn downgrade_rules() -> Vec<DowngradeRuleRow> {
    use DowngradeTriggerClass as Trigger;

    [
        (
            Trigger::DescriptorContractMissing,
            GovernanceClaimClass::Beta,
        ),
        (
            Trigger::InvocationSessionContractMissing,
            GovernanceClaimClass::Beta,
        ),
        (
            Trigger::ResultPacketContractMissing,
            GovernanceClaimClass::Beta,
        ),
        (
            Trigger::LifecycleDependencyPresent,
            GovernanceClaimClass::Preview,
        ),
        (Trigger::StaleEvidence, GovernanceClaimClass::Preview),
        (Trigger::MissingEvidence, GovernanceClaimClass::Unsupported),
        (
            Trigger::AuthorityParityBroken,
            GovernanceClaimClass::Preview,
        ),
        (
            Trigger::PreviewSemanticsStale,
            GovernanceClaimClass::Preview,
        ),
        (
            Trigger::DisabledReasonVocabularyMissing,
            GovernanceClaimClass::Preview,
        ),
    ]
    .into_iter()
    .map(|(trigger, narrowed_to)| DowngradeRuleRow {
        trigger,
        narrowed_to,
        consumer_surfaces: PublicationConsumerSurfaceClass::required_coverage().to_vec(),
        stable_copy_forbidden: true,
        summary: format!(
            "Consumers narrow to {} when {} fires.",
            narrowed_to.as_str(),
            trigger.as_str()
        ),
    })
    .collect()
}

fn dependency(
    marker_ref: &str,
    dependency_class: LifecycleDependencyClass,
    dependency_ref: &str,
    owner_ref: &str,
) -> LifecycleDependencyRow {
    LifecycleDependencyRow {
        marker_ref: marker_ref.to_owned(),
        dependency_class,
        dependency_ref: dependency_ref.to_owned(),
        owner_ref: owner_ref.to_owned(),
        disclosure_ref: format!("disclosure:{marker_ref}"),
        narrowing_required: true,
    }
}

fn stable_surface(surface_class: CommandGovernanceSurfaceClass) -> SurfaceGovernanceRow {
    SurfaceGovernanceRow {
        surface_class,
        claimed_stable: true,
        effective_claim: GovernanceClaimClass::Stable,
        published_stable_copy_allowed: true,
        descriptor_contract_current: true,
        invocation_session_contract_current: true,
        result_packet_contract_current: true,
        lifecycle_dependency_disclosed: true,
        dependency_marker_refs: Vec::new(),
        authority_parity_preserved: true,
        preview_parity_preserved: true,
        approval_parity_preserved: true,
        disabled_reason_parity_preserved: true,
        route_truth_disclosed: true,
        evidence_freshness: EvidenceFreshnessClass::Current,
        downgrade_reasons: Vec::new(),
    }
}

fn narrowed_surface(
    surface_class: CommandGovernanceSurfaceClass,
    effective_claim: GovernanceClaimClass,
    marker_ref: Option<&str>,
    trigger: DowngradeTriggerClass,
) -> SurfaceGovernanceRow {
    SurfaceGovernanceRow {
        surface_class,
        claimed_stable: true,
        effective_claim,
        published_stable_copy_allowed: false,
        descriptor_contract_current: marker_ref.is_none(),
        invocation_session_contract_current: true,
        result_packet_contract_current: true,
        lifecycle_dependency_disclosed: true,
        dependency_marker_refs: marker_ref.into_iter().map(str::to_owned).collect(),
        authority_parity_preserved: trigger != DowngradeTriggerClass::AuthorityParityBroken,
        preview_parity_preserved: trigger != DowngradeTriggerClass::PreviewSemanticsStale,
        approval_parity_preserved: trigger != DowngradeTriggerClass::PreviewSemanticsStale,
        disabled_reason_parity_preserved: trigger
            != DowngradeTriggerClass::DisabledReasonVocabularyMissing,
        route_truth_disclosed: true,
        evidence_freshness: if trigger == DowngradeTriggerClass::StaleEvidence {
            EvidenceFreshnessClass::Stale
        } else {
            EvidenceFreshnessClass::Current
        },
        downgrade_reasons: vec![trigger],
    }
}

fn feature_family_rows() -> Vec<FeatureFamilyGovernanceRow> {
    use CommandGovernanceSurfaceClass as Surface;
    use DowngradeTriggerClass as Trigger;
    use FeatureFamilyClass as Family;
    use GovernanceClaimClass as Claim;
    use LifecycleDependencyClass as Dependency;

    vec![
        FeatureFamilyGovernanceRow {
            family_class: Family::Notebook,
            display_label: "Notebook command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::Preview,
            published_stable_copy_allowed: false,
            descriptor_proof_ref: "proof:notebook:descriptor".to_owned(),
            descriptor_contract_current: true,
            invocation_session_proof_ref: "proof:notebook:invocation".to_owned(),
            invocation_session_contract_current: true,
            result_packet_proof_ref: "proof:notebook:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: true,
            preview_semantics_current: true,
            rollout_owner_ref: "owner:notebook".to_owned(),
            evidence_packet_ref: "evidence:notebook".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::Current,
            lifecycle_dependencies: vec![dependency(
                "marker:notebook:kernel-preview",
                Dependency::PreviewDependency,
                "cap:notebook.kernel.preview",
                "owner:notebook-kernel",
            )],
            downgrade_reasons: vec![Trigger::LifecycleDependencyPresent],
            surface_rows: CommandGovernanceSurfaceClass::required_coverage()
                .into_iter()
                .map(|surface| {
                    narrowed_surface(
                        surface,
                        Claim::Preview,
                        Some("marker:notebook:kernel-preview"),
                        Trigger::LifecycleDependencyPresent,
                    )
                })
                .collect(),
        },
        FeatureFamilyGovernanceRow {
            family_class: Family::DataApi,
            display_label: "Data/API command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::Stable,
            published_stable_copy_allowed: true,
            descriptor_proof_ref: "proof:data_api:descriptor".to_owned(),
            descriptor_contract_current: true,
            invocation_session_proof_ref: "proof:data_api:invocation".to_owned(),
            invocation_session_contract_current: true,
            result_packet_proof_ref: "proof:data_api:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: true,
            preview_semantics_current: true,
            rollout_owner_ref: "owner:data_api".to_owned(),
            evidence_packet_ref: "evidence:data_api".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::Current,
            lifecycle_dependencies: Vec::new(),
            downgrade_reasons: Vec::new(),
            surface_rows: CommandGovernanceSurfaceClass::required_coverage()
                .into_iter()
                .map(stable_surface)
                .collect(),
        },
        FeatureFamilyGovernanceRow {
            family_class: Family::Profiler,
            display_label: "Profiler command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::Preview,
            published_stable_copy_allowed: false,
            descriptor_proof_ref: "proof:profiler:descriptor".to_owned(),
            descriptor_contract_current: true,
            invocation_session_proof_ref: "proof:profiler:invocation".to_owned(),
            invocation_session_contract_current: true,
            result_packet_proof_ref: "proof:profiler:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: true,
            preview_semantics_current: true,
            rollout_owner_ref: "owner:profiler".to_owned(),
            evidence_packet_ref: "evidence:profiler".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::Stale,
            lifecycle_dependencies: Vec::new(),
            downgrade_reasons: vec![Trigger::StaleEvidence],
            surface_rows: CommandGovernanceSurfaceClass::required_coverage()
                .into_iter()
                .map(|surface| {
                    narrowed_surface(surface, Claim::Preview, None, Trigger::StaleEvidence)
                })
                .collect(),
        },
        FeatureFamilyGovernanceRow {
            family_class: Family::DocsBrowser,
            display_label: "Docs/browser command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::Beta,
            published_stable_copy_allowed: false,
            descriptor_proof_ref: "proof:docs_browser:descriptor".to_owned(),
            descriptor_contract_current: false,
            invocation_session_proof_ref: "proof:docs_browser:invocation".to_owned(),
            invocation_session_contract_current: true,
            result_packet_proof_ref: "proof:docs_browser:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: true,
            preview_semantics_current: true,
            rollout_owner_ref: "owner:docs_browser".to_owned(),
            evidence_packet_ref: "evidence:docs_browser".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::Current,
            lifecycle_dependencies: Vec::new(),
            downgrade_reasons: vec![Trigger::DescriptorContractMissing],
            surface_rows: CommandGovernanceSurfaceClass::required_coverage()
                .into_iter()
                .map(|surface| {
                    let mut row = narrowed_surface(
                        surface,
                        Claim::Beta,
                        None,
                        Trigger::DescriptorContractMissing,
                    );
                    row.descriptor_contract_current = false;
                    row
                })
                .collect(),
        },
        FeatureFamilyGovernanceRow {
            family_class: Family::Pipeline,
            display_label: "Pipeline command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::Preview,
            published_stable_copy_allowed: false,
            descriptor_proof_ref: "proof:pipeline:descriptor".to_owned(),
            descriptor_contract_current: true,
            invocation_session_proof_ref: "proof:pipeline:invocation".to_owned(),
            invocation_session_contract_current: true,
            result_packet_proof_ref: "proof:pipeline:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: true,
            preview_semantics_current: true,
            rollout_owner_ref: "owner:pipeline".to_owned(),
            evidence_packet_ref: "evidence:pipeline".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::Current,
            lifecycle_dependencies: vec![dependency(
                "marker:pipeline:beta-runner",
                Dependency::BetaDependency,
                "cap:pipeline.runner.beta",
                "owner:pipeline-runner",
            )],
            downgrade_reasons: vec![Trigger::LifecycleDependencyPresent],
            surface_rows: CommandGovernanceSurfaceClass::required_coverage()
                .into_iter()
                .map(|surface| {
                    narrowed_surface(
                        surface,
                        Claim::Preview,
                        Some("marker:pipeline:beta-runner"),
                        Trigger::LifecycleDependencyPresent,
                    )
                })
                .collect(),
        },
        FeatureFamilyGovernanceRow {
            family_class: Family::FrameworkPack,
            display_label: "Framework-pack command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::Stable,
            published_stable_copy_allowed: true,
            descriptor_proof_ref: "proof:framework_pack:descriptor".to_owned(),
            descriptor_contract_current: true,
            invocation_session_proof_ref: "proof:framework_pack:invocation".to_owned(),
            invocation_session_contract_current: true,
            result_packet_proof_ref: "proof:framework_pack:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: true,
            preview_semantics_current: true,
            rollout_owner_ref: "owner:framework_pack".to_owned(),
            evidence_packet_ref: "evidence:framework_pack".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::Current,
            lifecycle_dependencies: Vec::new(),
            downgrade_reasons: Vec::new(),
            surface_rows: CommandGovernanceSurfaceClass::required_coverage()
                .into_iter()
                .map(stable_surface)
                .collect(),
        },
        FeatureFamilyGovernanceRow {
            family_class: Family::Companion,
            display_label: "Companion command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::Preview,
            published_stable_copy_allowed: false,
            descriptor_proof_ref: "proof:companion:descriptor".to_owned(),
            descriptor_contract_current: true,
            invocation_session_proof_ref: "proof:companion:invocation".to_owned(),
            invocation_session_contract_current: true,
            result_packet_proof_ref: "proof:companion:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: true,
            preview_semantics_current: false,
            rollout_owner_ref: "owner:companion".to_owned(),
            evidence_packet_ref: "evidence:companion".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::Current,
            lifecycle_dependencies: Vec::new(),
            downgrade_reasons: vec![Trigger::PreviewSemanticsStale],
            surface_rows: vec![
                stable_surface(Surface::Desktop),
                stable_surface(Surface::Cli),
                stable_surface(Surface::Ai),
                stable_surface(Surface::Recipe),
                stable_surface(Surface::Extension),
                narrowed_surface(
                    Surface::BrowserCompanion,
                    Claim::Preview,
                    None,
                    Trigger::PreviewSemanticsStale,
                ),
            ],
        },
        FeatureFamilyGovernanceRow {
            family_class: Family::Sync,
            display_label: "Sync command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::Stable,
            published_stable_copy_allowed: true,
            descriptor_proof_ref: "proof:sync:descriptor".to_owned(),
            descriptor_contract_current: true,
            invocation_session_proof_ref: "proof:sync:invocation".to_owned(),
            invocation_session_contract_current: true,
            result_packet_proof_ref: "proof:sync:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: true,
            preview_semantics_current: true,
            rollout_owner_ref: "owner:sync".to_owned(),
            evidence_packet_ref: "evidence:sync".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::DueForRefresh,
            lifecycle_dependencies: Vec::new(),
            downgrade_reasons: Vec::new(),
            surface_rows: CommandGovernanceSurfaceClass::required_coverage()
                .into_iter()
                .map(|surface| {
                    let mut row = stable_surface(surface);
                    row.evidence_freshness = EvidenceFreshnessClass::DueForRefresh;
                    row
                })
                .collect(),
        },
        FeatureFamilyGovernanceRow {
            family_class: Family::Incident,
            display_label: "Incident command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::PolicyBlocked,
            published_stable_copy_allowed: false,
            descriptor_proof_ref: "proof:incident:descriptor".to_owned(),
            descriptor_contract_current: true,
            invocation_session_proof_ref: "proof:incident:invocation".to_owned(),
            invocation_session_contract_current: true,
            result_packet_proof_ref: "proof:incident:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: true,
            preview_semantics_current: true,
            rollout_owner_ref: "owner:incident".to_owned(),
            evidence_packet_ref: "evidence:incident".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::Current,
            lifecycle_dependencies: vec![dependency(
                "marker:incident:policy-gate",
                Dependency::PolicyGatedDependency,
                "cap:incident.response.policy_gate",
                "owner:incident-response",
            )],
            downgrade_reasons: vec![Trigger::LifecycleDependencyPresent],
            surface_rows: CommandGovernanceSurfaceClass::required_coverage()
                .into_iter()
                .map(|surface| {
                    narrowed_surface(
                        surface,
                        Claim::PolicyBlocked,
                        Some("marker:incident:policy-gate"),
                        Trigger::LifecycleDependencyPresent,
                    )
                })
                .collect(),
        },
        FeatureFamilyGovernanceRow {
            family_class: Family::SecretBroker,
            display_label: "Secret-broker command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::Preview,
            published_stable_copy_allowed: false,
            descriptor_proof_ref: "proof:secret_broker:descriptor".to_owned(),
            descriptor_contract_current: true,
            invocation_session_proof_ref: "proof:secret_broker:invocation".to_owned(),
            invocation_session_contract_current: true,
            result_packet_proof_ref: "proof:secret_broker:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: true,
            preview_semantics_current: true,
            rollout_owner_ref: "owner:secret_broker".to_owned(),
            evidence_packet_ref: "evidence:secret_broker".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::Current,
            lifecycle_dependencies: vec![dependency(
                "marker:secret_broker:rotation-preview",
                Dependency::PreviewDependency,
                "cap:secret_broker.rotation.preview",
                "owner:secret-broker",
            )],
            downgrade_reasons: vec![Trigger::LifecycleDependencyPresent],
            surface_rows: CommandGovernanceSurfaceClass::required_coverage()
                .into_iter()
                .map(|surface| {
                    narrowed_surface(
                        surface,
                        Claim::Preview,
                        Some("marker:secret_broker:rotation-preview"),
                        Trigger::LifecycleDependencyPresent,
                    )
                })
                .collect(),
        },
        FeatureFamilyGovernanceRow {
            family_class: Family::Infrastructure,
            display_label: "Infrastructure command surfaces".to_owned(),
            claimed_stable: true,
            effective_claim: Claim::Preview,
            published_stable_copy_allowed: false,
            descriptor_proof_ref: "proof:infrastructure:descriptor".to_owned(),
            descriptor_contract_current: true,
            invocation_session_proof_ref: "proof:infrastructure:invocation".to_owned(),
            invocation_session_contract_current: false,
            result_packet_proof_ref: "proof:infrastructure:result".to_owned(),
            result_packet_contract_current: true,
            disabled_reason_vocabulary_current: false,
            preview_semantics_current: true,
            rollout_owner_ref: "owner:infrastructure".to_owned(),
            evidence_packet_ref: "evidence:infrastructure".to_owned(),
            evidence_freshness: EvidenceFreshnessClass::Missing,
            lifecycle_dependencies: vec![dependency(
                "marker:infrastructure:underqualified",
                Dependency::UnderqualifiedDependency,
                "cap:infrastructure.ownerless_experiment",
                "owner:temporary-control-plane",
            )],
            downgrade_reasons: vec![
                Trigger::InvocationSessionContractMissing,
                Trigger::DisabledReasonVocabularyMissing,
                Trigger::MissingEvidence,
                Trigger::LifecycleDependencyPresent,
            ],
            surface_rows: CommandGovernanceSurfaceClass::required_coverage()
                .into_iter()
                .map(|surface| {
                    let mut row =
                        narrowed_surface(surface, Claim::Preview, None, Trigger::MissingEvidence);
                    row.invocation_session_contract_current = false;
                    row.disabled_reason_parity_preserved = false;
                    row.evidence_freshness = EvidenceFreshnessClass::Missing;
                    row.dependency_marker_refs =
                        vec!["marker:infrastructure:underqualified".to_owned()];
                    row
                })
                .collect(),
        },
    ]
}

fn evidence_export() -> GovernanceEvidenceExport {
    GovernanceEvidenceExport {
        evidence_id: "command-governance-evidence:stable:0001".to_owned(),
        json_export_ref: FREEZE_COMMAND_GOVERNANCE_CONTRACT_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: FREEZE_COMMAND_GOVERNANCE_CONTRACT_SUMMARY_REF.to_owned(),
        help_projection_ref: "help:command-governance:0001".to_owned(),
        release_projection_ref: "release:command-governance:0001".to_owned(),
        support_projection_ref: "support:command-governance:0001".to_owned(),
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        FREEZE_COMMAND_GOVERNANCE_CONTRACT_DOC_REF.to_owned(),
        FREEZE_COMMAND_GOVERNANCE_CONTRACT_SCHEMA_REF.to_owned(),
        FREEZE_COMMAND_GOVERNANCE_DESCRIPTOR_CONTRACT_REF.to_owned(),
        FREEZE_COMMAND_GOVERNANCE_INVOCATION_PARITY_CONTRACT_REF.to_owned(),
        FREEZE_COMMAND_GOVERNANCE_LIFECYCLE_ADR_REF.to_owned(),
        FREEZE_COMMAND_GOVERNANCE_DISABLED_REASON_REF.to_owned(),
    ]
}

fn input() -> CommandGovernanceContractPacketInput {
    CommandGovernanceContractPacketInput {
        packet_id: PACKET_ID.to_owned(),
        display_label: "Frozen command governance contract".to_owned(),
        policy_epoch_ref: "policy-epoch:command-governance:0001".to_owned(),
        contract_refs: GovernanceContractRefs::canonical(),
        descriptor_fields: descriptor_fields(),
        invocation_session_fields: invocation_session_fields(),
        result_packet_fields: result_packet_fields(),
        lifecycle_dependency_vocabulary: lifecycle_dependency_vocabulary(),
        downgrade_rules: downgrade_rules(),
        feature_family_rows: feature_family_rows(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-12T00:00:00Z".to_owned(),
    }
}

fn packet() -> CommandGovernanceContractPacket {
    CommandGovernanceContractPacket::new(input())
}

#[test]
fn command_governance_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::WrongRecordKind));
}

#[test]
fn missing_descriptor_field_coverage_is_rejected() {
    let mut packet = packet();
    packet
        .descriptor_fields
        .retain(|row| row.field_class != CommandDescriptorFieldClass::OriginMetadata);
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::DescriptorFieldCoverageMissing));
}

#[test]
fn missing_invocation_field_coverage_is_rejected() {
    let mut packet = packet();
    packet
        .invocation_session_fields
        .retain(|row| row.field_class != InvocationSessionFieldClass::TimingCostBands);
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::InvocationSessionFieldCoverageMissing));
}

#[test]
fn missing_result_field_coverage_is_rejected() {
    let mut packet = packet();
    packet
        .result_packet_fields
        .retain(|row| row.field_class != ResultPacketFieldClass::EvidenceRefs);
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::ResultPacketFieldCoverageMissing));
}

#[test]
fn missing_lifecycle_dependency_vocabulary_is_rejected() {
    let mut packet = packet();
    packet
        .lifecycle_dependency_vocabulary
        .retain(|row| row.dependency_class != LifecycleDependencyClass::UnderqualifiedDependency);
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::LifecycleDependencyCoverageMissing));
}

#[test]
fn missing_downgrade_rule_coverage_is_rejected() {
    let mut packet = packet();
    packet
        .downgrade_rules
        .retain(|row| row.trigger != DowngradeTriggerClass::MissingEvidence);
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::DowngradeRuleCoverageMissing));
}

#[test]
fn missing_feature_family_coverage_is_rejected() {
    let mut packet = packet();
    packet
        .feature_family_rows
        .retain(|row| row.family_class != FeatureFamilyClass::Companion);
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::FeatureFamilyCoverageMissing));
}

#[test]
fn missing_surface_coverage_is_rejected() {
    let mut packet = packet();
    packet.feature_family_rows[0]
        .surface_rows
        .retain(|row| row.surface_class != CommandGovernanceSurfaceClass::Ai);
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::SurfaceCoverageMissing));
}

#[test]
fn stable_claim_must_auto_narrow_when_descriptor_proof_is_missing() {
    let mut packet = packet();
    let row = packet
        .feature_family_rows
        .iter_mut()
        .find(|row| row.family_class == FeatureFamilyClass::DocsBrowser)
        .unwrap();
    row.effective_claim = GovernanceClaimClass::Stable;
    row.published_stable_copy_allowed = true;
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::StableClaimNotAutoNarrowed));
}

#[test]
fn lifecycle_dependencies_must_be_disclosed() {
    let mut packet = packet();
    let row = packet
        .feature_family_rows
        .iter_mut()
        .find(|row| row.family_class == FeatureFamilyClass::Pipeline)
        .unwrap();
    row.surface_rows[0].lifecycle_dependency_disclosed = false;
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::LifecycleDependencyUndisclosed));
}

#[test]
fn stable_surface_may_not_hide_authority_parity_breakage() {
    let mut packet = packet();
    let row = packet
        .feature_family_rows
        .iter_mut()
        .find(|row| row.family_class == FeatureFamilyClass::DataApi)
        .unwrap();
    row.surface_rows[0].authority_parity_preserved = false;
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::SurfaceAuthorityParityBroken));
}

#[test]
fn raw_material_is_rejected() {
    let mut packet = packet();
    packet.feature_family_rows[0].display_label = "Open https://provider.example".to_owned();
    assert!(packet
        .validate()
        .contains(&CommandGovernanceContractViolation::RawMaterialInExport));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_frozen_command_governance_contract_export()
        .expect("checked command governance export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/commands/freeze_command_governance_contract");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        format!("{dir}/support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
    std::fs::write(
        format!("{dir}/summary.md"),
        packet.render_markdown_summary(),
    )
    .unwrap();
    let fixture_dir = format!("{root}/fixtures/commands/freeze_command_governance_contract");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/command_governance_contract_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
