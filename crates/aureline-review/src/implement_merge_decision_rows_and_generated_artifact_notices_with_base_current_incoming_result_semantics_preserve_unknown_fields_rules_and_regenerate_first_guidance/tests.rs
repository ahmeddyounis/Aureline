use super::*;

const PACKET_ID: &str = "merge-generated-controls:stable:0001";

const ARTIFACT_SOURCE: &str = "artifact:src/main.rs";
const ARTIFACT_GENERATED: &str = "artifact:generated/api_client.rs";
const ARTIFACT_LOCKFILE: &str = "artifact:package/Cargo.lock";
const ARTIFACT_MANIFEST: &str = "artifact:package/Cargo.toml";
const ARTIFACT_POLICY: &str = "artifact:policy/ownership.yaml";
const ARTIFACT_SCHEMA: &str = "artifact:generated/schema.json";

fn merge_decision_rows() -> Vec<MergeDecisionRow> {
    vec![
        MergeDecisionRow {
            component: M5ArtifactComponent::MergeDecisionRow,
            row_id: "merge:source-main".to_owned(),
            artifact_ref: ARTIFACT_SOURCE.to_owned(),
            object_path: "src/main.rs:fn run".to_owned(),
            conflict_class: MergeConflictClass::OrdinaryLineMerge,
            conflict_kind: "both modified".to_owned(),
            base_summary: "let cfg = load();".to_owned(),
            current_summary: "let cfg = load_with_retry();".to_owned(),
            incoming_summary: "let cfg = load_cached();".to_owned(),
            result_summary: "let cfg = load_with_retry();".to_owned(),
            preserve_unknown_fields_note: String::new(),
            available_guidance: vec![
                MergeResolutionGuidance::AcceptCurrent,
                MergeResolutionGuidance::AcceptIncoming,
                MergeResolutionGuidance::AcceptBoth,
                MergeResolutionGuidance::Manual,
            ],
            recommended_guidance: MergeResolutionGuidance::AcceptCurrent,
            write_back_safety_note:
                "Ordinary text merge: picking a side writes back and stays attributable".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            raw_context_action: "Open the raw three-way diff at src/main.rs".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::WriteBackAttributable,
            fields_shown: vec![
                "conflict_class".to_owned(),
                "base_summary".to_owned(),
                "current_summary".to_owned(),
                "incoming_summary".to_owned(),
                "result_summary".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF.to_owned()
            ],
        },
        MergeDecisionRow {
            component: M5ArtifactComponent::MergeDecisionRow,
            row_id: "merge:generated-client".to_owned(),
            artifact_ref: ARTIFACT_GENERATED.to_owned(),
            object_path: "generated/api_client.rs".to_owned(),
            conflict_class: MergeConflictClass::GeneratedArtifactConflict,
            conflict_kind: "both regenerated from divergent sources".to_owned(),
            base_summary: "client v2 (generated 2026-06-01)".to_owned(),
            current_summary: "client v3 (generated from ours)".to_owned(),
            incoming_summary: "client v3 (generated from theirs)".to_owned(),
            result_summary: "regenerate from the merged source of truth".to_owned(),
            preserve_unknown_fields_note:
                "Unknown generated attributes are preserved through regeneration".to_owned(),
            available_guidance: vec![
                MergeResolutionGuidance::RegenerateFromSource,
                MergeResolutionGuidance::Manual,
            ],
            recommended_guidance: MergeResolutionGuidance::RegenerateFromSource,
            write_back_safety_note:
                "Generated artifact: regenerate-first is safer than hand-merging the output"
                    .to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            raw_context_action: "Open the raw generated diff at generated/api_client.rs".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::RegenerateOnlyNoManualEdit,
            fields_shown: vec![
                "conflict_class".to_owned(),
                "recommended_guidance".to_owned(),
                "write_back_safety_note".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF.to_owned()
            ],
        },
        MergeDecisionRow {
            component: M5ArtifactComponent::MergeDecisionRow,
            row_id: "merge:lockfile".to_owned(),
            artifact_ref: ARTIFACT_LOCKFILE.to_owned(),
            object_path: "package/Cargo.lock:[serde]".to_owned(),
            conflict_class: MergeConflictClass::LockfileConflict,
            conflict_kind: "both changed pinned version".to_owned(),
            base_summary: "serde 1.0.100".to_owned(),
            current_summary: "serde 1.0.201".to_owned(),
            incoming_summary: "serde 1.0.203".to_owned(),
            result_summary: "regenerate the lockfile from the merged manifest".to_owned(),
            preserve_unknown_fields_note:
                "Unknown lockfile metadata is preserved through regeneration".to_owned(),
            available_guidance: vec![
                MergeResolutionGuidance::RegenerateFromSource,
                MergeResolutionGuidance::Manual,
            ],
            recommended_guidance: MergeResolutionGuidance::RegenerateFromSource,
            write_back_safety_note:
                "Lockfile: regenerate from the manifest rather than hand-picking a pin".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredPartial,
            raw_context_action: "Open the raw lockfile diff at [serde]".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::RegenerateOnlyNoManualEdit,
            fields_shown: vec![
                "conflict_class".to_owned(),
                "recommended_guidance".to_owned(),
                "write_back_safety_note".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF.to_owned()
            ],
        },
        MergeDecisionRow {
            component: M5ArtifactComponent::MergeDecisionRow,
            row_id: "merge:manifest".to_owned(),
            artifact_ref: ARTIFACT_MANIFEST.to_owned(),
            object_path: "package/Cargo.toml:[dependencies]".to_owned(),
            conflict_class: MergeConflictClass::ManifestConflict,
            conflict_kind: "both edited dependency set".to_owned(),
            base_summary: "tokio = 1.30".to_owned(),
            current_summary: "tokio = 1.35, tracing = 0.1".to_owned(),
            incoming_summary: "tokio = 1.34, anyhow = 1.0".to_owned(),
            result_summary: "reconcile the union of dependency edits by hand".to_owned(),
            preserve_unknown_fields_note:
                "Unknown manifest keys are preserved during manual reconciliation".to_owned(),
            available_guidance: vec![
                MergeResolutionGuidance::Manual,
                MergeResolutionGuidance::AcceptBoth,
            ],
            recommended_guidance: MergeResolutionGuidance::Manual,
            write_back_safety_note:
                "Manifest: manual reconciliation is safer than blindly picking a side".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            raw_context_action: "Open the raw manifest diff at [dependencies]".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::WriteBackAttributable,
            fields_shown: vec![
                "conflict_class".to_owned(),
                "recommended_guidance".to_owned(),
                "write_back_safety_note".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF.to_owned()
            ],
        },
        MergeDecisionRow {
            component: M5ArtifactComponent::MergeDecisionRow,
            row_id: "merge:policy".to_owned(),
            artifact_ref: ARTIFACT_POLICY.to_owned(),
            object_path: "policy/ownership.yaml:reviewers".to_owned(),
            conflict_class: MergeConflictClass::PolicyOwnedConflict,
            conflict_kind: "both edited a policy-owned field".to_owned(),
            base_summary: "reviewers: [team-core]".to_owned(),
            current_summary: "reviewers: [team-core, team-platform]".to_owned(),
            incoming_summary: "reviewers: [team-core, team-data]".to_owned(),
            result_summary: "reconcile under the ownership policy with an owner sign-off"
                .to_owned(),
            preserve_unknown_fields_note:
                "Unknown policy fields are preserved; only reviewers is reconciled".to_owned(),
            available_guidance: vec![MergeResolutionGuidance::Manual],
            recommended_guidance: MergeResolutionGuidance::Manual,
            write_back_safety_note:
                "Policy-owned: manual reconciliation under policy, not a direct side pick"
                    .to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            raw_context_action: "Open the raw policy diff at reviewers".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::EvidencePreservedNoRevert,
            fields_shown: vec![
                "conflict_class".to_owned(),
                "recommended_guidance".to_owned(),
                "write_back_safety_note".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF.to_owned()
            ],
        },
    ]
}

fn generated_artifact_notices() -> Vec<GeneratedArtifactNotice> {
    vec![
        GeneratedArtifactNotice {
            component: M5ArtifactComponent::GeneratedArtifactNotice,
            notice_id: "notice:generated-client".to_owned(),
            artifact_ref: ARTIFACT_GENERATED.to_owned(),
            artifact_class_label: "generated API client".to_owned(),
            generated_from_relation: "generated from openapi/spec.yaml".to_owned(),
            source_of_truth_ref: "artifact:openapi/spec.yaml".to_owned(),
            generation_state: GeneratedArtifactState::Diverged,
            last_generated_label: "v3 (generated 2026-06-01T00:00:00Z)".to_owned(),
            divergence_note: "The output was hand-edited and no longer matches the spec".to_owned(),
            available_actions: vec![
                GeneratedNoticeAction::Regenerate,
                GeneratedNoticeAction::OpenSource,
                GeneratedNoticeAction::CompareAgainstSource,
                GeneratedNoticeAction::ViewLineage,
            ],
            write_back_restriction: GeneratedWriteBackRestriction::RegenerateOnly,
            write_back_restriction_note:
                "Regenerate-only: this client is rebuilt from the spec, never hand-edited"
                    .to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            raw_context_action: "Open the raw generated file".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::RegenerateOnlyNoManualEdit,
            fields_shown: vec![
                "generated_from_relation".to_owned(),
                "generation_state".to_owned(),
                "write_back_restriction".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF.to_owned()
            ],
        },
        GeneratedArtifactNotice {
            component: M5ArtifactComponent::GeneratedArtifactNotice,
            notice_id: "notice:lockfile".to_owned(),
            artifact_ref: ARTIFACT_LOCKFILE.to_owned(),
            artifact_class_label: "dependency lockfile".to_owned(),
            generated_from_relation: "generated from package/Cargo.toml".to_owned(),
            source_of_truth_ref: "artifact:package/Cargo.toml".to_owned(),
            generation_state: GeneratedArtifactState::Stale,
            last_generated_label: "generated 2026-05-20T00:00:00Z".to_owned(),
            divergence_note: String::new(),
            available_actions: vec![
                GeneratedNoticeAction::Regenerate,
                GeneratedNoticeAction::OpenSource,
                GeneratedNoticeAction::CompareAgainstSource,
            ],
            write_back_restriction: GeneratedWriteBackRestriction::RegenerateOnly,
            write_back_restriction_note:
                "Regenerate-only: the lockfile is rebuilt from the manifest".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredPartial,
            raw_context_action: "Open the raw lockfile".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::RegenerateOnlyNoManualEdit,
            fields_shown: vec![
                "generated_from_relation".to_owned(),
                "generation_state".to_owned(),
                "write_back_restriction".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF.to_owned()
            ],
        },
        GeneratedArtifactNotice {
            component: M5ArtifactComponent::GeneratedArtifactNotice,
            notice_id: "notice:schema".to_owned(),
            artifact_ref: ARTIFACT_SCHEMA.to_owned(),
            artifact_class_label: "generated JSON schema".to_owned(),
            generated_from_relation: "generated from schema/model.rs".to_owned(),
            source_of_truth_ref: "artifact:schema/model.rs".to_owned(),
            generation_state: GeneratedArtifactState::UpToDate,
            last_generated_label: "v7 (generated 2026-07-01T00:00:00Z)".to_owned(),
            divergence_note: String::new(),
            available_actions: vec![
                GeneratedNoticeAction::OpenSource,
                GeneratedNoticeAction::CompareAgainstSource,
                GeneratedNoticeAction::ViewLineage,
            ],
            write_back_restriction: GeneratedWriteBackRestriction::CompareOnly,
            write_back_restriction_note:
                "Compare-only: the schema is not written back from this view".to_owned(),
            schema_fidelity: M5ArtifactFidelityState::StructuredFaithful,
            raw_context_action: "Open the raw generated schema".to_owned(),
            rollback_posture: M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack,
            fields_shown: vec![
                "generated_from_relation".to_owned(),
                "generation_state".to_owned(),
                "write_back_restriction".to_owned(),
            ],
            source_contract_refs: vec![
                M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF.to_owned()
            ],
        },
    ]
}

fn trust_review() -> MergeGeneratedControlsTrustReview {
    MergeGeneratedControlsTrustReview {
        base_current_incoming_result_distinct: true,
        special_conflict_never_ordinary_line_merge: true,
        unknown_fields_preserved_explicitly: true,
        regenerate_or_manual_stated_when_safer: true,
        generated_from_relation_always_explicit: true,
        stale_or_diverged_state_disclosed: true,
        raw_context_always_reachable: true,
        compare_only_never_silently_writable: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> MergeGeneratedControlsConsumerProjection {
    MergeGeneratedControlsConsumerProjection {
        merge_row_shows_bcir_and_conflict_class: true,
        generated_notice_shows_relation_and_restriction: true,
        raw_context_reachable_from_both: true,
        regenerate_first_guidance_shown: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        help_about_shows_truth: true,
    }
}

fn proof_freshness() -> MergeGeneratedControlsProofFreshness {
    MergeGeneratedControlsProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<MergeGeneratedControlsDowngradeTrigger> {
    vec![
        MergeGeneratedControlsDowngradeTrigger::ProofStale,
        MergeGeneratedControlsDowngradeTrigger::GeneratedArtifactDrifted,
        MergeGeneratedControlsDowngradeTrigger::RegenerateFirstEnforced,
        MergeGeneratedControlsDowngradeTrigger::PolicyBlocked,
        MergeGeneratedControlsDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<MergeGeneratedControlsConsumerSurface> {
    vec![
        MergeGeneratedControlsConsumerSurface::DiffCompareView,
        MergeGeneratedControlsConsumerSurface::MergeConflictWorkspace,
        MergeGeneratedControlsConsumerSurface::ArtifactBrowser,
        MergeGeneratedControlsConsumerSurface::CliHeadless,
        MergeGeneratedControlsConsumerSurface::SupportExport,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        MERGE_GENERATED_CONTROLS_SCHEMA_REF.to_owned(),
        MERGE_GENERATED_CONTROLS_DOC_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF.to_owned(),
        M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> MergeGeneratedControlsPacket {
    MergeGeneratedControlsPacket::new(MergeGeneratedControlsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Merge decision rows and generated-artifact notices".to_owned(),
        merge_decision_rows: merge_decision_rows(),
        generated_artifact_notices: generated_artifact_notices(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

#[test]
fn merge_generated_controls_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn merge_resolver_derives_guidance_from_conflict_class() {
    let ordinary = resolve_merge_decision_disclosure(MergeConflictClass::OrdinaryLineMerge);
    assert!(ordinary.direct_write_back_safe);
    assert!(!ordinary.needs_preserve_unknown_fields_note);

    let generated =
        resolve_merge_decision_disclosure(MergeConflictClass::GeneratedArtifactConflict);
    assert!(generated.regenerate_first_safer);
    assert!(!generated.direct_write_back_safe);
    assert!(generated.needs_preserve_unknown_fields_note);

    let lockfile = resolve_merge_decision_disclosure(MergeConflictClass::LockfileConflict);
    assert!(lockfile.regenerate_first_safer);

    let manifest = resolve_merge_decision_disclosure(MergeConflictClass::ManifestConflict);
    assert!(manifest.manual_resolution_safer);
    assert!(!manifest.direct_write_back_safe);

    let policy = resolve_merge_decision_disclosure(MergeConflictClass::PolicyOwnedConflict);
    assert!(policy.manual_resolution_safer);
}

#[test]
fn generated_resolver_derives_from_state() {
    let up_to_date = resolve_generated_notice_disclosure(GeneratedArtifactState::UpToDate);
    assert!(!up_to_date.needs_regenerate_action);
    assert!(!up_to_date.needs_divergence_note);

    let stale = resolve_generated_notice_disclosure(GeneratedArtifactState::Stale);
    assert!(stale.needs_regenerate_action);
    assert!(!stale.needs_divergence_note);

    let diverged = resolve_generated_notice_disclosure(GeneratedArtifactState::Diverged);
    assert!(diverged.needs_regenerate_action);
    assert!(diverged.needs_divergence_note);
    assert!(diverged.regenerate_first_recommended);
}

#[test]
fn merge_semantics_missing_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[0].result_summary = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::MergeSemanticsMissing));
}

#[test]
fn preserve_unknown_fields_note_missing_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[1].preserve_unknown_fields_note = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::PreserveUnknownFieldsNoteMissing));
}

#[test]
fn write_back_safety_note_missing_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[0].write_back_safety_note = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::WriteBackSafetyNoteMissing));
}

#[test]
fn resolution_guidance_missing_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[0].available_guidance.clear();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::ResolutionGuidanceMissing));
}

#[test]
fn recommended_guidance_not_offered_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[3].available_guidance = vec![MergeResolutionGuidance::AcceptBoth];
    // recommended_guidance is still Manual, which is no longer offered.
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::RecommendedGuidanceNotOffered));
}

#[test]
fn regenerate_first_guidance_missing_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[1].available_guidance = vec![MergeResolutionGuidance::Manual];
    packet.merge_decision_rows[1].recommended_guidance = MergeResolutionGuidance::Manual;
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::RegenerateFirstGuidanceMissing));
}

#[test]
fn manual_resolution_guidance_missing_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[4].available_guidance = vec![MergeResolutionGuidance::AcceptBoth];
    packet.merge_decision_rows[4].recommended_guidance = MergeResolutionGuidance::AcceptBoth;
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::ManualResolutionGuidanceMissing));
}

#[test]
fn ordinary_merge_misrepresented_fails() {
    let mut packet = packet();
    // A generated conflict recommended for a direct side-accept masquerades as
    // an ordinary line merge.
    packet.merge_decision_rows[1].available_guidance = vec![
        MergeResolutionGuidance::AcceptIncoming,
        MergeResolutionGuidance::RegenerateFromSource,
    ];
    packet.merge_decision_rows[1].recommended_guidance = MergeResolutionGuidance::AcceptIncoming;
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::OrdinaryMergeMisrepresented));
}

#[test]
fn merge_conflict_class_coverage_missing_fails() {
    let mut packet = packet();
    packet
        .merge_decision_rows
        .retain(|row| row.conflict_class != MergeConflictClass::PolicyOwnedConflict);
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::MergeConflictClassCoverageMissing));
}

#[test]
fn conflict_kind_missing_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[0].conflict_kind = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::ConflictKindMissing));
}

#[test]
fn object_identity_missing_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[0].object_path = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::ObjectIdentityMissing));
}

#[test]
fn wrong_merge_component_class_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[0].component = M5ArtifactComponent::GeneratedArtifactNotice;
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::MergeDecisionRowWrongComponentClass));
}

#[test]
fn missing_raw_context_action_row_fails() {
    let mut packet = packet();
    packet.merge_decision_rows[0].raw_context_action = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::RawContextActionMissing));
}

#[test]
fn generated_from_relation_missing_fails() {
    let mut packet = packet();
    packet.generated_artifact_notices[2].generated_from_relation = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::GeneratedFromRelationMissing));
}

#[test]
fn source_of_truth_pointer_missing_fails() {
    let mut packet = packet();
    packet.generated_artifact_notices[2].source_of_truth_ref = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::SourceOfTruthPointerMissing));
}

#[test]
fn last_generated_label_missing_fails() {
    let mut packet = packet();
    packet.generated_artifact_notices[2].last_generated_label = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::LastGeneratedLabelMissing));
}

#[test]
fn regenerate_action_missing_fails() {
    let mut packet = packet();
    // The lockfile notice is stale and must offer a regenerate action.
    packet.generated_artifact_notices[1].available_actions =
        vec![GeneratedNoticeAction::OpenSource];
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::RegenerateActionMissing));
}

#[test]
fn open_source_action_missing_fails() {
    let mut packet = packet();
    packet.generated_artifact_notices[2].available_actions =
        vec![GeneratedNoticeAction::CompareAgainstSource];
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::OpenSourceActionMissing));
}

#[test]
fn divergence_note_missing_fails() {
    let mut packet = packet();
    // The generated-client notice is diverged and must explain the divergence.
    packet.generated_artifact_notices[0].divergence_note = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::DivergenceNoteMissing));
}

#[test]
fn write_back_restriction_note_missing_fails() {
    let mut packet = packet();
    packet.generated_artifact_notices[2].write_back_restriction_note = String::new();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::WriteBackRestrictionNoteMissing));
}

#[test]
fn write_back_restriction_inconsistent_fails() {
    let mut packet = packet();
    // Compare-only restriction with a writable posture is an inconsistency.
    packet.generated_artifact_notices[2].write_back_restriction =
        GeneratedWriteBackRestriction::WriteBackAllowed;
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::WriteBackRestrictionInconsistent));
}

#[test]
fn generated_artifact_state_coverage_missing_fails() {
    let mut packet = packet();
    packet
        .generated_artifact_notices
        .retain(|notice| notice.generation_state != GeneratedArtifactState::Diverged);
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::GeneratedArtifactStateCoverageMissing));
}

#[test]
fn generated_conflict_notice_missing_fails() {
    let mut packet = packet();
    // A generated-artifact conflict whose artifact has no accompanying notice.
    packet.merge_decision_rows[1].artifact_ref = "artifact:generated/orphan.rs".to_owned();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::GeneratedConflictNoticeMissing));
}

#[test]
fn wrong_generated_component_class_fails() {
    let mut packet = packet();
    packet.generated_artifact_notices[0].component = M5ArtifactComponent::MergeDecisionRow;
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::GeneratedArtifactNoticeWrongComponentClass));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .special_conflict_never_ordinary_line_merge = false;
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.regenerate_first_guidance_shown = false;
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&MergeGeneratedControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Merge decision rows"));
    assert!(summary.contains("## Generated-artifact notices"));
    assert!(summary.contains("regenerate_from_source"));
    assert!(summary.contains("dependency lockfile"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_merge_generated_controls_export()
        .expect("checked merge generated controls export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-merge-decision-generated-notice-controls/generated_conflict_regenerate_first.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-merge-decision-generated-notice-controls/policy_conflict_manual_resolution.json"
        )),
    ] {
        let packet: MergeGeneratedControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as merge generated controls packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_MERGE_GENERATED_CONTROLS_ARTIFACTS` so ordinary test runs
/// never touch the working tree.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_MERGE_GENERATED_CONTROLS_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-merge-decision-generated-notice-controls-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-merge-decision-generated-notice-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: a generated-artifact conflict diverged from its source;
    // regenerate-first stays the recommended path and the notice accompanies it.
    let mut regenerate_first = packet.clone();
    regenerate_first.packet_id =
        "merge-generated-controls:fixture:generated-regenerate-first".to_owned();
    if let Some(notice) = regenerate_first
        .generated_artifact_notices
        .iter_mut()
        .find(|notice| notice.artifact_ref == ARTIFACT_GENERATED)
    {
        notice.divergence_note =
            "Hand-edits diverged the client from the spec; regenerate to reconcile".to_owned();
        notice.last_generated_label =
            "v3 (regenerate pending; source advanced 2026-07-05T00:00:00Z)".to_owned();
    }
    if let Some(row) = regenerate_first
        .merge_decision_rows
        .iter_mut()
        .find(|row| row.artifact_ref == ARTIFACT_GENERATED)
    {
        row.write_back_safety_note =
            "Regenerate-first: rebuild from the merged spec rather than hand-merging the output"
                .to_owned();
    }
    assert!(
        regenerate_first.validate().is_empty(),
        "{:?}",
        regenerate_first.validate()
    );
    std::fs::write(
        fixture_dir.join("generated_conflict_regenerate_first.json"),
        format!("{}\n", regenerate_first.export_safe_json()),
    )
    .expect("write generated-regenerate-first fixture");

    // Fixture 2: a policy-owned conflict that must be reconciled manually under
    // policy — never a direct side pick.
    let mut policy = packet.clone();
    policy.packet_id = "merge-generated-controls:fixture:policy-manual".to_owned();
    if let Some(row) = policy
        .merge_decision_rows
        .iter_mut()
        .find(|row| row.artifact_ref == ARTIFACT_POLICY)
    {
        row.conflict_kind =
            "both edited a policy-owned reviewers field under contention".to_owned();
        row.available_guidance = vec![MergeResolutionGuidance::Manual];
        row.recommended_guidance = MergeResolutionGuidance::Manual;
        row.write_back_safety_note =
            "Policy-owned: reconcile under the ownership policy with an owner sign-off, never a side pick"
                .to_owned();
    }
    assert!(policy.validate().is_empty(), "{:?}", policy.validate());
    std::fs::write(
        fixture_dir.join("policy_conflict_manual_resolution.json"),
        format!("{}\n", policy.export_safe_json()),
    )
    .expect("write policy-manual fixture");
}
