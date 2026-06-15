//! Deterministic builder for the M5 family release graph.
//!
//! [`build_m5_family_release_graph`] constructs the same graph that the
//! checked-in JSON embeds, so the headless emitter can regenerate the artifact
//! and a test can prove the embedded JSON never drifts from the code.

use crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::M5ArtifactFamilyKind;
use crate::stable_claim_manifest::{FreshnessSlo, FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, StableClaimLevel,
};

use super::{
    BlockerClass, BlockerRow, BundleMemberCard, BundleMemberKind, EvidenceFreshnessRow,
    FamilyGapReason, FamilyRemediationAction, FamilyStopRule, M5FamilyReleaseCandidate,
    M5FamilyReleaseGraph, MemberPresence, ScopedArtifactBundleCard,
    IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_RECORD_KIND,
    IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_SCHEMA_VERSION,
};

const AS_OF: &str = "2026-06-15";
const SLO_REGISTER_REF: &str = "release/freshness_slo_register";
const TARGET_MAX_AGE_DAYS: u32 = 90;
const WARN_WITHIN_DAYS: u32 = 14;

/// Builds the canonical M5 family release graph in code.
pub fn build_m5_family_release_graph() -> M5FamilyReleaseGraph {
    let candidates = vec![
        backed_candidate(BackedSpec {
            family_kind: M5ArtifactFamilyKind::NotebookPack,
            slug: "notebook-pack",
            title: "Notebook pack release candidate",
            artifact_summary: "Notebook packs and notebook-derived outputs.",
            claim_label: StableClaimLevel::Stable,
            owner: "notebook-release",
            na_members: &[],
        }),
        backed_candidate(BackedSpec {
            family_kind: M5ArtifactFamilyKind::RequestDataAsset,
            slug: "request-data-asset",
            title: "Request/data asset release candidate",
            artifact_summary: "Saved requests, datasets, and request/data fixtures.",
            claim_label: StableClaimLevel::Stable,
            owner: "data-release",
            na_members: &[],
        }),
        backed_candidate(BackedSpec {
            family_kind: M5ArtifactFamilyKind::ProfilerReplayArtifact,
            slug: "profiler-replay",
            title: "Profiler/replay artifact release candidate",
            artifact_summary: "Profiler traces and replay recordings.",
            claim_label: StableClaimLevel::Stable,
            owner: "profiler-release",
            na_members: &[BundleMemberKind::SdkArtifact],
        }),
        backed_candidate(BackedSpec {
            family_kind: M5ArtifactFamilyKind::FrameworkTemplatePack,
            slug: "framework-template-pack",
            title: "Framework/template pack release candidate",
            artifact_summary: "Framework and template packs.",
            claim_label: StableClaimLevel::Stable,
            owner: "framework-release",
            na_members: &[],
        }),
        backed_candidate(BackedSpec {
            family_kind: M5ArtifactFamilyKind::DocsPack,
            slug: "docs-pack",
            title: "Docs pack release candidate",
            artifact_summary: "User-facing and embedded documentation packs.",
            claim_label: StableClaimLevel::Lts,
            owner: "docs-release",
            na_members: &[BundleMemberKind::Symbols, BundleMemberKind::SdkArtifact],
        }),
        backed_candidate(BackedSpec {
            family_kind: M5ArtifactFamilyKind::ModelPack,
            slug: "model-pack",
            title: "Model pack release candidate",
            artifact_summary: "Local model bundles and model metadata.",
            claim_label: StableClaimLevel::Stable,
            owner: "model-release",
            na_members: &[],
        }),
        backed_candidate(BackedSpec {
            family_kind: M5ArtifactFamilyKind::CompanionOffboardingPacket,
            slug: "companion-offboarding",
            title: "Companion/offboarding packet release candidate",
            artifact_summary: "Companion and offboarding packets.",
            claim_label: StableClaimLevel::Stable,
            owner: "companion-release",
            na_members: &[BundleMemberKind::SdkArtifact],
        }),
        managed_output_candidate(),
    ];

    let release_blocking_artifact_refs = candidates
        .iter()
        .filter(|c| c.release_blocking)
        .map(|c| c.artifact_ref.clone())
        .collect();

    let mut graph = M5FamilyReleaseGraph {
        schema_version: IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_SCHEMA_VERSION,
        record_kind: IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_RECORD_KIND.to_owned(),
        graph_id: "m5-family-release-graph".to_owned(),
        status: "frozen".to_owned(),
        overview_page:
            "docs/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.md"
                .to_owned(),
        as_of: AS_OF.to_owned(),
        claim_manifest_ref: "release/stable_claim_manifest".to_owned(),
        artifact_graph_ref: "release/artifact_graph".to_owned(),
        publication_matrix_ref:
            "release/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix"
                .to_owned(),
        lifecycle_labels: StableClaimLevel::ALL.to_vec(),
        family_kinds: M5ArtifactFamilyKind::ALL.to_vec(),
        bundle_member_kinds: BundleMemberKind::ALL.to_vec(),
        member_presence_states: MemberPresence::ALL.to_vec(),
        blocker_classes: BlockerClass::ALL.to_vec(),
        freshness_states: FreshnessSloState::ALL.to_vec(),
        gap_reasons: FamilyGapReason::ALL.to_vec(),
        remediation_actions: FamilyRemediationAction::ALL.to_vec(),
        launch_cutline: launch_cutline(),
        release_blocking_artifact_refs,
        stop_rules: stop_rules(),
        candidates,
        publication: PromotionDecisionRecord {
            promotion_gate: "m5_family_release_graph".to_owned(),
            decision: PromotionDecision::Proceed,
            blocking_rule_ids: Vec::new(),
            blocking_claim_ids: Vec::new(),
            rationale:
                "Every release-blocking family holds an intact bundle, current evidence, and a recorded rollback target; the managed-output family already inherits a below-cutline claim, so it narrows without blocking the train."
                    .to_owned(),
        },
        summary: placeholder_summary(),
    };

    graph.publication.decision = graph.computed_publication_decision();
    graph.publication.blocking_rule_ids = graph.computed_blocking_rule_ids();
    graph.publication.blocking_claim_ids = graph.computed_blocking_candidate_ids();
    graph.summary = graph.computed_summary();
    graph
}

fn launch_cutline() -> LaunchCutline {
    LaunchCutline {
        cutline_level: StableClaimLevel::Stable,
        above_cutline_levels: StableClaimLevel::ABOVE_CUTLINE.to_vec(),
        below_cutline_levels: StableClaimLevel::BELOW_CUTLINE.to_vec(),
        description:
            "A family publishes at or above the cutline only when its scoped bundle is intact, its required evidence is within SLO, it has no open blocker, its rollback target and exact-build identity are recorded, its proof packet is within SLO, and it is owner-signed; otherwise it narrows below stable."
                .to_owned(),
    }
}

fn stop_rules() -> Vec<FamilyStopRule> {
    let rule = |id: &str,
                title: &str,
                trigger_reason: FamilyGapReason,
                default_action: FamilyRemediationAction,
                rationale: &str| FamilyStopRule {
        rule_id: id.to_owned(),
        title: title.to_owned(),
        trigger_reason,
        applies_to_labels: StableClaimLevel::ABOVE_CUTLINE.to_vec(),
        default_action,
        blocks_publication: true,
        rationale: rationale.to_owned(),
    };
    vec![
        rule(
            "stop-bundle-member-missing",
            "Bundle member missing",
            FamilyGapReason::BundleMemberMissing,
            FamilyRemediationAction::ProvideBundleMember,
            "A family at or above the cutline must provide every required bundle member.",
        ),
        rule(
            "stop-bundle-member-partial",
            "Bundle member partial",
            FamilyGapReason::BundleMemberPartial,
            FamilyRemediationAction::ProvideBundleMember,
            "A partial bundle member is not a publishable member.",
        ),
        rule(
            "stop-evidence-stale",
            "Required evidence stale",
            FamilyGapReason::EvidenceStale,
            FamilyRemediationAction::RefreshEvidence,
            "Required evidence that breached its freshness SLO narrows the family.",
        ),
        rule(
            "stop-evidence-missing",
            "Required evidence missing",
            FamilyGapReason::EvidenceMissing,
            FamilyRemediationAction::RecaptureEvidence,
            "Missing required evidence is a first-class blocker, not an omission.",
        ),
        rule(
            "stop-blocker-open",
            "Blocker open",
            FamilyGapReason::BlockerOpen,
            FamilyRemediationAction::ResolveBlocker,
            "An open blocker holds publication until it is resolved.",
        ),
        rule(
            "stop-rollback-target-missing",
            "Rollback target missing",
            FamilyGapReason::RollbackTargetMissing,
            FamilyRemediationAction::RecordRollbackTarget,
            "A family without a recorded rollback target cannot be revoked as one system.",
        ),
        rule(
            "stop-exact-build-identity-missing",
            "Exact-build identity missing",
            FamilyGapReason::ExactBuildIdentityMissing,
            FamilyRemediationAction::LinkExactBuildIdentity,
            "A family without exact-build identity is not rebuildable or symbolicated.",
        ),
        rule(
            "stop-proof-packet-stale",
            "Proof packet stale",
            FamilyGapReason::ProofPacketStale,
            FamilyRemediationAction::RefreshProofPacket,
            "A proof packet outside its freshness SLO narrows the family.",
        ),
        rule(
            "stop-proof-packet-missing",
            "Proof packet missing",
            FamilyGapReason::ProofPacketMissing,
            FamilyRemediationAction::RefreshProofPacket,
            "A family without a captured proof packet narrows below the cutline.",
        ),
        rule(
            "stop-waiver-expired",
            "Waiver expired",
            FamilyGapReason::WaiverExpired,
            FamilyRemediationAction::RenewWaiver,
            "A family relying on an expired waiver narrows below the cutline.",
        ),
        rule(
            "stop-owner-signoff-missing",
            "Owner sign-off missing",
            FamilyGapReason::OwnerSignoffMissing,
            FamilyRemediationAction::RequestOwnerSignoff,
            "A family without owner sign-off cannot hold its claimed label.",
        ),
    ]
}

struct BackedSpec {
    family_kind: M5ArtifactFamilyKind,
    slug: &'static str,
    title: &'static str,
    artifact_summary: &'static str,
    claim_label: StableClaimLevel,
    owner: &'static str,
    na_members: &'static [BundleMemberKind],
}

fn backed_candidate(spec: BackedSpec) -> M5FamilyReleaseCandidate {
    let slug = spec.slug;
    let members = BundleMemberKind::ALL
        .into_iter()
        .map(|kind| {
            if spec.na_members.contains(&kind) {
                not_applicable_member(kind)
            } else {
                provided_member(kind, slug)
            }
        })
        .collect();
    M5FamilyReleaseCandidate {
        entry_id: format!("candidate-{slug}"),
        title: spec.title.to_owned(),
        family_kind: spec.family_kind,
        artifact_ref: format!("artifact/m5/{slug}"),
        artifact_summary: spec.artifact_summary.to_owned(),
        release_blocking: true,
        release_candidate_ref: format!("release/candidate/m5-{slug}"),
        candidate_version: "1.0.0".to_owned(),
        channel_family: "m5".to_owned(),
        claim_ref: format!("claim/m5-{slug}"),
        claim_label: spec.claim_label,
        exact_build_identity_ref: format!("exact_build/m5-{slug}"),
        rollback_target_ref: format!("rollback/m5-{slug}/last-known-good"),
        bundle: ScopedArtifactBundleCard {
            bundle_id: format!("bundle-{slug}"),
            artifact_graph_ref: "release/artifact_graph".to_owned(),
            exact_build_identity_ref: format!("exact_build/m5-{slug}"),
            members,
        },
        blockers: Vec::new(),
        evidence_rows: vec![
            evidence_row(
                format!("evidence-{slug}-rebuild"),
                "clean_room_rebuild",
                FreshnessSloState::Current,
                true,
                "Clean-room rebuild matches the published exact build.",
            ),
            evidence_row(
                format!("evidence-{slug}-compat"),
                "compatibility_report",
                FreshnessSloState::DueForRefresh,
                true,
                "Compatibility report is current and due for routine refresh.",
            ),
        ],
        known_issue_refs: vec![format!("known_issue/m5-{slug}/none-blocking")],
        proof_packet: proof_packet(slug, FreshnessSloState::Current),
        waiver: None,
        owner_signoff: signed(spec.owner),
        active_gap_reasons: Vec::new(),
        published_label: spec.claim_label,
        rationale:
            "Intact bundle, current evidence, recorded rollback target and exact-build identity, fresh proof packet, and owner sign-off; the family holds its claimed label."
                .to_owned(),
    }
}

fn managed_output_candidate() -> M5FamilyReleaseCandidate {
    let slug = "managed-output";
    let members = BundleMemberKind::ALL
        .into_iter()
        .map(|kind| match kind {
            BundleMemberKind::Symbols => not_applicable_member(kind),
            BundleMemberKind::Schema => partial_member(kind, slug),
            BundleMemberKind::SdkArtifact => not_provided_member(kind),
            other => provided_member(other, slug),
        })
        .collect();
    M5FamilyReleaseCandidate {
        entry_id: format!("candidate-{slug}"),
        title: "Managed output release candidate".to_owned(),
        family_kind: M5ArtifactFamilyKind::ManagedOutput,
        artifact_ref: format!("artifact/m5/{slug}"),
        artifact_summary: "Managed outputs produced by managed/tenant-scoped lanes.".to_owned(),
        release_blocking: true,
        release_candidate_ref: format!("release/candidate/m5-{slug}"),
        candidate_version: "1.0.0".to_owned(),
        channel_family: "m5".to_owned(),
        claim_ref: format!("claim/m5-{slug}"),
        claim_label: StableClaimLevel::Beta,
        exact_build_identity_ref: format!("exact_build/m5-{slug}"),
        rollback_target_ref: format!("rollback/m5-{slug}/last-known-good"),
        bundle: ScopedArtifactBundleCard {
            bundle_id: format!("bundle-{slug}"),
            artifact_graph_ref: "release/artifact_graph".to_owned(),
            exact_build_identity_ref: format!("exact_build/m5-{slug}"),
            members,
        },
        blockers: vec![BlockerRow {
            blocker_id: format!("blocker-{slug}-grid-defect"),
            class: BlockerClass::OpenDefect,
            blocks_promotion: true,
            source_ref: format!("defect/m5-{slug}/result-grid"),
            summary: "Open defect in the managed result-grid export path.".to_owned(),
        }],
        evidence_rows: vec![
            evidence_row(
                format!("evidence-{slug}-rebuild"),
                "clean_room_rebuild",
                FreshnessSloState::Current,
                true,
                "Clean-room rebuild matches the published exact build.",
            ),
            evidence_row(
                format!("evidence-{slug}-compat"),
                "compatibility_report",
                FreshnessSloState::Breached,
                true,
                "Compatibility report aged out beyond its freshness SLO.",
            ),
        ],
        known_issue_refs: vec![format!("known_issue/m5-{slug}/result-grid-export")],
        proof_packet: proof_packet(slug, FreshnessSloState::Breached),
        waiver: None,
        owner_signoff: signed("managed-release"),
        active_gap_reasons: vec![
            FamilyGapReason::BundleMemberMissing,
            FamilyGapReason::BundleMemberPartial,
            FamilyGapReason::EvidenceStale,
            FamilyGapReason::BlockerOpen,
            FamilyGapReason::ProofPacketStale,
        ],
        published_label: StableClaimLevel::Preview,
        rationale:
            "The managed-output family is missing its SDK artifact, ships a partial schema, has stale compatibility evidence, an open result-grid defect, and a breached proof packet; it inherits its below-cutline beta claim and narrows to preview, naming every gap."
                .to_owned(),
    }
}

fn provided_member(kind: BundleMemberKind, slug: &str) -> BundleMemberCard {
    BundleMemberCard {
        member_kind: kind,
        presence: MemberPresence::Provided,
        artifact_ref: format!("artifact/m5/{slug}/{}", kind.as_str()),
        digest_algorithm: "sha256".to_owned(),
        digest_ref: format!("sha256/m5/{slug}/{}", kind.as_str()),
        summary: format!("{} joined by immutable digest.", kind.as_str()),
    }
}

fn partial_member(kind: BundleMemberKind, slug: &str) -> BundleMemberCard {
    BundleMemberCard {
        member_kind: kind,
        presence: MemberPresence::Partial,
        artifact_ref: format!("artifact/m5/{slug}/{}", kind.as_str()),
        digest_algorithm: String::new(),
        digest_ref: String::new(),
        summary: format!(
            "{} is present but incomplete; digest not yet sealed.",
            kind.as_str()
        ),
    }
}

fn not_provided_member(kind: BundleMemberKind) -> BundleMemberCard {
    BundleMemberCard {
        member_kind: kind,
        presence: MemberPresence::NotProvided,
        artifact_ref: String::new(),
        digest_algorithm: String::new(),
        digest_ref: String::new(),
        summary: format!("{} is not provided for this family.", kind.as_str()),
    }
}

fn not_applicable_member(kind: BundleMemberKind) -> BundleMemberCard {
    BundleMemberCard {
        member_kind: kind,
        presence: MemberPresence::NotApplicable,
        artifact_ref: String::new(),
        digest_algorithm: String::new(),
        digest_ref: String::new(),
        summary: format!("{} does not apply to this family.", kind.as_str()),
    }
}

fn evidence_row(
    id: String,
    kind: &str,
    slo_state: FreshnessSloState,
    required: bool,
    summary: &str,
) -> EvidenceFreshnessRow {
    let captured_at = if slo_state == FreshnessSloState::Missing {
        None
    } else {
        Some(AS_OF.to_owned())
    };
    EvidenceFreshnessRow {
        evidence_id: id.clone(),
        evidence_kind: kind.to_owned(),
        slo_state,
        required_for_promotion: required,
        evidence_ref: format!("evidence/{id}"),
        captured_at,
        summary: summary.to_owned(),
    }
}

fn proof_packet(slug: &str, slo_state: FreshnessSloState) -> ProofPacket {
    let captured_at = if slo_state == FreshnessSloState::Missing {
        None
    } else {
        Some(AS_OF.to_owned())
    };
    let evidence_refs = if slo_state == FreshnessSloState::Missing {
        Vec::new()
    } else {
        vec![format!("evidence/proof/m5-{slug}")]
    };
    ProofPacket {
        packet_id: format!("packet-m5-{slug}"),
        packet_ref: format!("proof/m5-{slug}"),
        proof_index_ref: format!("proof_index/m5-{slug}"),
        captured_at,
        freshness_slo: FreshnessSlo {
            target_max_age_days: TARGET_MAX_AGE_DAYS,
            warn_within_days: WARN_WITHIN_DAYS,
            slo_register_ref: SLO_REGISTER_REF.to_owned(),
        },
        slo_state,
        evidence_refs,
    }
}

fn signed(owner: &str) -> OwnerSignoff {
    OwnerSignoff {
        owner_ref: owner.to_owned(),
        signed_off: true,
        signed_at: Some(AS_OF.to_owned()),
    }
}

fn placeholder_summary() -> super::M5FamilyReleaseGraphSummary {
    super::M5FamilyReleaseGraphSummary {
        total_candidates: 0,
        total_release_candidates: 0,
        candidates_backed: 0,
        candidates_narrowed: 0,
        candidates_on_active_waiver: 0,
        release_blocking_total: 0,
        release_blocking_backed: 0,
        release_blocking_narrowed: 0,
        notebook_pack_candidates: 0,
        request_data_asset_candidates: 0,
        profiler_replay_candidates: 0,
        framework_template_candidates: 0,
        docs_pack_candidates: 0,
        model_pack_candidates: 0,
        companion_offboarding_candidates: 0,
        managed_output_candidates: 0,
        bundles_intact: 0,
        bundles_with_missing_member: 0,
        bundles_with_partial_member: 0,
        total_blockers: 0,
        blocking_blockers: 0,
        total_evidence_rows: 0,
        blocking_evidence_rows: 0,
        packets_current: 0,
        packets_due_for_refresh: 0,
        packets_breached: 0,
        packets_missing: 0,
        total_active_gap_reasons: 0,
        rules_firing: 0,
    }
}
