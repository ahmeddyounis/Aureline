use std::path::{Path, PathBuf};

use aureline_runtime::{
    BaselineCompatibilityStateClass, BaselineRecord, BaselineRecordRequest,
    EffectiveQualityProfile, QualityActionClass, QualityActionDisclosureClass,
    QualityActionProposal, QualityActionProposalRequest, QualityActorClass,
    QualityApplyPostureClass, QualityGovernanceError, QualityGovernanceSupportExport,
    QualityLockReasonClass, QualityLockStateClass, QualityMutationScopeClass, QualityOwnerClass,
    QualityPolicyLockStateClass, QualityPreviewRequirementClass, QualityProfileResolutionRequest,
    QualityProfileResolver, QualityProfileSourceCandidate, QualityProfileSourceLayer,
    QualityReopenRuleClass, QualitySafetyClass, QualitySession, QualitySessionOutcomeClass,
    QualitySessionRequest, QualitySessionTriggerClass, QualitySurfaceClass,
    QualityTargetScopeClass, QualityToolFamilyClass, QualityTruthMutationClass, SuppressionRecord,
    SuppressionRecordRequest,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/quality/profile_and_suppression_governance")
}

#[test]
fn policy_locked_profile_is_inspectable_on_required_surfaces() {
    let profile = governed_profile();

    assert_eq!(
        profile.lock_state_class,
        QualityLockStateClass::PolicyLocked
    );
    assert_eq!(profile.winning_source_ref, "policy:quality:regulated");
    assert!(profile.has_policy_overrides);
    assert!(profile.has_unmapped_imported_config);
    assert_eq!(
        profile.source_state_tokens(),
        vec![
            "selected_winner",
            "policy_overridden",
            "policy_overridden",
            "imported_read_only",
            "policy_overridden"
        ]
    );

    for surface in QualitySurfaceClass::required_profile_inspection_surfaces() {
        assert!(
            profile.is_visible_on_surface(surface),
            "{surface:?} should have profile projection"
        );
    }
}

#[test]
fn quality_actions_derive_preview_apply_and_rollback_posture() {
    let proposals = governed_proposals();
    let format = &proposals[0];
    assert_eq!(
        format.disclosure_class,
        QualityActionDisclosureClass::TriviaOnly
    );
    assert_eq!(
        format.apply_posture_class,
        QualityApplyPostureClass::AutoApplyAllowed
    );
    assert!(!format.preview_first_required);

    let broad = &proposals[1];
    assert_eq!(broad.disclosure_class, QualityActionDisclosureClass::Broad);
    assert_eq!(
        broad.preview_requirement_class,
        QualityPreviewRequirementClass::BatchScopePreview
    );
    assert!(broad.preview_first_required);
    assert!(!broad.apply_blocked);

    let policy = &proposals[2];
    assert_eq!(
        policy.disclosure_class,
        QualityActionDisclosureClass::PolicyEscalated
    );
    assert_eq!(
        policy.apply_posture_class,
        QualityApplyPostureClass::BlockedPendingPolicyOrTrust
    );
    assert!(policy.preview_first_required);
    assert!(policy.apply_blocked);

    let session = governed_session(proposals);
    assert_eq!(
        session.outcome_class,
        QualitySessionOutcomeClass::BlockedByPolicy
    );
    assert!(session.any_preview_first_required);
    assert!(session.any_apply_blocked);
}

#[test]
fn suppression_and_baseline_records_are_governed_and_reopenable() {
    let suppression = governed_suppression();
    assert_eq!(
        suppression.reopen_state_at("2026-06-02T00:00:00Z").as_str(),
        "expired_reopened"
    );
    assert!(suppression.release_visible);
    assert!(suppression.hidden_permanent_toggle_denied);

    let hidden = SuppressionRecord::from_request(SuppressionRecordRequest {
        suppression_id: "suppression:hidden:forever".into(),
        scope_class: QualityTargetScopeClass::CurrentFile,
        rule_refs: vec!["rule:style:unused".into()],
        finding_refs: vec!["finding:style:unused".into()],
        truth_mutation_class: QualityTruthMutationClass::LocalSessionVisibilityOnly,
        policy_lock_state_class: QualityPolicyLockStateClass::EditableLocal,
        owner_class: QualityOwnerClass::WorkspaceOwner,
        owner_ref: "owner:workspace".into(),
        actor_class: QualityActorClass::LocalUser,
        actor_ref: "actor:dev".into(),
        created_at: "2026-05-18T16:02:00Z".into(),
        expires_at: None,
        reason: "Hide this row locally.".into(),
        evidence_refs: vec!["evidence:local:row".into()],
        reopen_rule_class: QualityReopenRuleClass::ReopenOnExpiry,
        release_visible: false,
        summary: "Hidden local suppression should be denied.".into(),
    });
    assert_eq!(
        hidden.unwrap_err(),
        QualityGovernanceError::HiddenPermanentSuppressionDenied
    );

    let baseline = governed_baseline();
    assert!(!baseline.blocks_comparison());
    assert_eq!(baseline.reopen_state_for_comparison().as_str(), "active");

    let drifted = BaselineRecord::from_request(BaselineRecordRequest {
        compatibility_state_class: BaselineCompatibilityStateClass::ProfileDriftBlocked,
        ..baseline_request()
    })
    .expect("drifted baseline record is still governed");
    assert!(drifted.blocks_comparison());
    assert_eq!(
        drifted.reopen_state_for_comparison().as_str(),
        "reopened_for_profile_or_target_drift"
    );
}

#[test]
fn fixture_matches_runtime_quality_governance_records() {
    let path = fixture_root().join("governed_quality_plane.yaml");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    let fixture: QualityGovernanceFixture =
        serde_yaml::from_str(&payload).expect("fixture parses as quality governance records");

    let profile = governed_profile();
    let proposals = governed_proposals();
    let session = governed_session(proposals.clone());
    let suppression = governed_suppression();
    let baseline = governed_baseline();
    let export = governed_support_export(
        profile.clone(),
        session.clone(),
        suppression.clone(),
        baseline.clone(),
    );

    assert_eq!(fixture.profile, profile);
    assert_eq!(fixture.proposals, proposals);
    assert_eq!(fixture.session, session);
    assert_eq!(fixture.suppression, suppression);
    assert_eq!(fixture.baseline, baseline);
    assert_eq!(
        export.effective_profile_refs,
        vec!["quality.profile.effective.rust.governed"]
    );
    assert_eq!(
        export.quality_session_refs,
        vec!["quality.session.on-save.governed"]
    );
    assert_eq!(
        export.suppression_refs,
        vec!["suppression:secret:temporary:false-positive"]
    );
    assert_eq!(export.baseline_refs, vec!["baseline:secret:regulated"]);
    assert!(export.redaction_safe);
}

#[test]
fn quality_schemas_parse() {
    for rel in [
        "../../schemas/quality/effective_quality_profile.schema.json",
        "../../schemas/quality/quality_action_proposal.schema.json",
        "../../schemas/quality/quality_session.schema.json",
        "../../schemas/quality/suppression_record.schema.json",
    ] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel);
        let payload =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
        let schema: serde_json::Value =
            serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
        assert_eq!(
            schema["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
    }
}

fn governed_profile() -> EffectiveQualityProfile {
    QualityProfileResolver
        .resolve(QualityProfileResolutionRequest {
            effective_profile_id: "quality.profile.effective.rust.governed".into(),
            workspace_ref: "workspace:repo:example".into(),
            target_scope_class: QualityTargetScopeClass::Workspace,
            surface_classes: QualitySurfaceClass::required_profile_inspection_surfaces().to_vec(),
            resolved_at: "2026-05-18T16:00:00Z".into(),
            source_candidates: vec![
                QualityProfileSourceCandidate {
                    source_layer: QualityProfileSourceLayer::PolicyLockOrManagedProfile,
                    source_ref: "policy:quality:regulated".into(),
                    candidate_profile_ref: "quality.profile.policy.regulated.rust".into(),
                    tool_family_class: QualityToolFamilyClass::MixedQualityPack,
                    lock_state_class: QualityLockStateClass::PolicyLocked,
                    lock_reason_class: QualityLockReasonClass::AdminPolicyPinsRulePack,
                    available: true,
                    compatible: true,
                    imported_read_only: false,
                    unmapped_key_count: 0,
                    policy_overridden_key_count: 0,
                    summary: "Managed policy pins formatter and scanner rule pack.".into(),
                },
                QualityProfileSourceCandidate {
                    source_layer: QualityProfileSourceLayer::PerCommandOverride,
                    source_ref: "command:quality:format:override".into(),
                    candidate_profile_ref: "quality.profile.command.rustfmt-nightly".into(),
                    tool_family_class: QualityToolFamilyClass::Formatter,
                    lock_state_class: QualityLockStateClass::Unlocked,
                    lock_reason_class: QualityLockReasonClass::None,
                    available: true,
                    compatible: true,
                    imported_read_only: false,
                    unmapped_key_count: 0,
                    policy_overridden_key_count: 1,
                    summary: "Command override requested a formatter channel that policy narrowed."
                        .into(),
                },
                QualityProfileSourceCandidate {
                    source_layer: QualityProfileSourceLayer::WorkspaceQualitySettings,
                    source_ref: "workspace:settings:quality".into(),
                    candidate_profile_ref: "quality.profile.workspace.rust".into(),
                    tool_family_class: QualityToolFamilyClass::Formatter,
                    lock_state_class: QualityLockStateClass::Unlocked,
                    lock_reason_class: QualityLockReasonClass::None,
                    available: true,
                    compatible: true,
                    imported_read_only: false,
                    unmapped_key_count: 0,
                    policy_overridden_key_count: 1,
                    summary: "Workspace settings remain visible below the managed policy source."
                        .into(),
                },
                QualityProfileSourceCandidate {
                    source_layer: QualityProfileSourceLayer::ImportedToolConfig,
                    source_ref: "config:editorconfig:workspace".into(),
                    candidate_profile_ref: "quality.profile.imported.editorconfig".into(),
                    tool_family_class: QualityToolFamilyClass::Formatter,
                    lock_state_class: QualityLockStateClass::ReadOnlyImported,
                    lock_reason_class: QualityLockReasonClass::ImportedEvidenceReadOnly,
                    available: true,
                    compatible: true,
                    imported_read_only: true,
                    unmapped_key_count: 2,
                    policy_overridden_key_count: 0,
                    summary: "Imported editorconfig contributed read-only mapping notes.".into(),
                },
                QualityProfileSourceCandidate {
                    source_layer: QualityProfileSourceLayer::FallbackDefault,
                    source_ref: "default:quality:rust".into(),
                    candidate_profile_ref: "quality.profile.default.rust".into(),
                    tool_family_class: QualityToolFamilyClass::MixedQualityPack,
                    lock_state_class: QualityLockStateClass::Unlocked,
                    lock_reason_class: QualityLockReasonClass::None,
                    available: true,
                    compatible: true,
                    imported_read_only: false,
                    unmapped_key_count: 0,
                    policy_overridden_key_count: 0,
                    summary: "Built-in defaults are retained as the lowest explicit fallback."
                        .into(),
                },
            ],
            selected_tool_refs: vec![
                "tool:rustfmt:stable".into(),
                "tool:secret-scan:managed".into(),
            ],
            suppression_policy_refs: vec!["suppression.policy:regulated".into()],
            baseline_refs: vec!["baseline:secret:regulated".into()],
            profile_fingerprint: "digest:quality-profile:rust:governed".into(),
            resolution_summary:
                "Managed policy won the quality profile while lower sources remain inspectable."
                    .into(),
        })
        .expect("profile resolves")
}

fn governed_proposals() -> Vec<QualityActionProposal> {
    vec![
        QualityActionProposal::from_request(QualityActionProposalRequest {
            proposal_id: "quality.proposal.format.document".into(),
            action_class: QualityActionClass::FormatDocument,
            target_scope_class: QualityTargetScopeClass::CurrentFile,
            mutation_scope_class: QualityMutationScopeClass::SingleFileLocalized,
            safety_class: QualitySafetyClass::TriviaSafe,
            effective_profile_ref: "quality.profile.effective.rust.governed".into(),
            triggering_finding_refs: Vec::new(),
            rule_refs: vec!["rule:format:rustfmt".into()],
            policy_lock_refs: Vec::new(),
            affected_file_count: 1,
            affected_anchor_count: 1,
            generated_path_count: 0,
            protected_path_count: 0,
            blocked_path_count: 0,
            semantic_current: true,
            profile_policy_locked: false,
            checkpoint_ref: None,
            preview_ref: None,
            revert_plan_ref: Some("revert:quality:format:document".into()),
            validation_refs: vec!["validation:formatter:idempotence".into()],
            summary: "Formatter is trivia-only and can auto-apply in the current file.".into(),
        }),
        QualityActionProposal::from_request(QualityActionProposalRequest {
            proposal_id: "quality.proposal.fix_all.secret".into(),
            action_class: QualityActionClass::FixAllRule,
            target_scope_class: QualityTargetScopeClass::Workspace,
            mutation_scope_class: QualityMutationScopeClass::MultiFileWorkspace,
            safety_class: QualitySafetyClass::CrossFileSemantic,
            effective_profile_ref: "quality.profile.effective.rust.governed".into(),
            triggering_finding_refs: vec!["finding:secret:token:001".into()],
            rule_refs: vec!["rule:secret:hardcoded-token".into()],
            policy_lock_refs: Vec::new(),
            affected_file_count: 4,
            affected_anchor_count: 4,
            generated_path_count: 0,
            protected_path_count: 0,
            blocked_path_count: 0,
            semantic_current: true,
            profile_policy_locked: false,
            checkpoint_ref: Some("checkpoint:quality:fix-all:secret".into()),
            preview_ref: Some("preview:quality:fix-all:secret".into()),
            revert_plan_ref: Some("revert:quality:fix-all:secret".into()),
            validation_refs: vec!["validation:secret-scan:rerun".into()],
            summary: "Fix-all spans multiple files and must open batch preview before apply."
                .into(),
        }),
        QualityActionProposal::from_request(QualityActionProposalRequest {
            proposal_id: "quality.proposal.suppress.secret".into(),
            action_class: QualityActionClass::SuppressionProposal,
            target_scope_class: QualityTargetScopeClass::Workspace,
            mutation_scope_class: QualityMutationScopeClass::ProtectedOrPolicyScoped,
            safety_class: QualitySafetyClass::GeneratedOrProtected,
            effective_profile_ref: "quality.profile.effective.rust.governed".into(),
            triggering_finding_refs: vec!["finding:secret:token:001".into()],
            rule_refs: vec!["rule:secret:hardcoded-token".into()],
            policy_lock_refs: vec!["policy:quality:regulated".into()],
            affected_file_count: 0,
            affected_anchor_count: 1,
            generated_path_count: 0,
            protected_path_count: 1,
            blocked_path_count: 0,
            semantic_current: true,
            profile_policy_locked: true,
            checkpoint_ref: None,
            preview_ref: Some("preview:quality:suppression:secret".into()),
            revert_plan_ref: Some("audit:quality:suppression:secret".into()),
            validation_refs: vec!["validation:suppression:expiry-scheduler".into()],
            summary: "Suppression proposal is policy-scoped and blocked pending policy review."
                .into(),
        }),
    ]
}

fn governed_session(proposals: Vec<QualityActionProposal>) -> QualitySession {
    QualitySession::from_request(QualitySessionRequest {
        session_id: "quality.session.on-save.governed".into(),
        trigger_class: QualitySessionTriggerClass::OnSave,
        target_scope_class: QualityTargetScopeClass::Workspace,
        effective_profile_ref: "quality.profile.effective.rust.governed".into(),
        execution_context_ref: Some("execution:quality:local".into()),
        started_at: "2026-05-18T16:00:01Z".into(),
        ended_at: Some("2026-05-18T16:00:05Z".into()),
        proposals,
        validation_refs: vec!["validation:quality:session".into()],
        rollback_refs: vec![
            "revert:quality:format:document".into(),
            "revert:quality:fix-all:secret".into(),
        ],
        summary:
            "On-save quality session used the governed profile and blocked policy-scoped debt mutation."
                .into(),
    })
}

fn governed_suppression() -> SuppressionRecord {
    SuppressionRecord::from_request(SuppressionRecordRequest {
        suppression_id: "suppression:secret:temporary:false-positive".into(),
        scope_class: QualityTargetScopeClass::Workspace,
        rule_refs: vec!["rule:secret:hardcoded-token".into()],
        finding_refs: vec!["finding:secret:token:001".into()],
        truth_mutation_class: QualityTruthMutationClass::WorkspaceRepoArtifact,
        policy_lock_state_class: QualityPolicyLockStateClass::EditableWithReview,
        owner_class: QualityOwnerClass::SecurityOwner,
        owner_ref: "owner:security".into(),
        actor_class: QualityActorClass::LocalUser,
        actor_ref: "actor:dev".into(),
        created_at: "2026-05-18T16:02:00Z".into(),
        expires_at: Some("2026-06-01T16:02:00Z".into()),
        reason: "False positive for generated test fixture token with linked review evidence."
            .into(),
        evidence_refs: vec!["evidence:review:secret:001".into()],
        reopen_rule_class: QualityReopenRuleClass::ReopenOnExpiry,
        release_visible: true,
        summary: "Time-bounded suppression remains release-visible and reopens on expiry.".into(),
    })
    .expect("suppression is governed")
}

fn baseline_request() -> BaselineRecordRequest {
    BaselineRecordRequest {
        baseline_id: "baseline:secret:regulated".into(),
        compatible_profile_family_ref: "quality.profile.family:regulated-secret".into(),
        target_scope_ref: "target:workspace:example".into(),
        target_scope_class: QualityTargetScopeClass::Workspace,
        accepted_finding_refs: vec!["finding:secret:known-test-fixture".into()],
        created_at: "2026-05-18T16:03:00Z".into(),
        actor_class: QualityActorClass::LocalUser,
        actor_ref: "actor:dev".into(),
        owner_class: QualityOwnerClass::SecurityOwner,
        owner_ref: "owner:security".into(),
        evidence_refs: vec!["evidence:baseline:secret:001".into()],
        review_refs: vec!["review:baseline:secret:001".into()],
        supersedes_refs: Vec::new(),
        compatibility_state_class: BaselineCompatibilityStateClass::Compatible,
        policy_lock_state_class: QualityPolicyLockStateClass::EditableWithReview,
        reopen_rule_class: QualityReopenRuleClass::ReopenOnProfileOrTargetDrift,
        release_visible: true,
        summary: "Baseline anchors known secret-scan debt to a compatible regulated profile."
            .into(),
    }
}

fn governed_baseline() -> BaselineRecord {
    BaselineRecord::from_request(baseline_request()).expect("baseline is governed")
}

fn governed_support_export(
    profile: EffectiveQualityProfile,
    session: QualitySession,
    suppression: SuppressionRecord,
    baseline: BaselineRecord,
) -> QualityGovernanceSupportExport {
    QualityGovernanceSupportExport::from_records(
        "support:quality:governed",
        "2026-05-18T16:04:00Z",
        vec![profile],
        vec![session],
        vec![suppression],
        vec![baseline],
        "Support export preserves effective profile, action, session, suppression, and baseline truth.",
    )
}

#[derive(Debug, Deserialize)]
struct QualityGovernanceFixture {
    profile: EffectiveQualityProfile,
    proposals: Vec<QualityActionProposal>,
    session: QualitySession,
    suppression: SuppressionRecord,
    baseline: BaselineRecord,
}
