//! Deterministic builder for the M5 artifact-graph promotion-ledger register.
//!
//! [`build_m5_artifact_graph_promotion_ledger`] constructs the same register that
//! the checked-in JSON embeds, so the headless emitter can regenerate the artifact
//! and a test can prove the embedded JSON never drifts from the code.

use crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::M5ArtifactFamilyKind;
use crate::release_center_model::{
    ArtifactFamilyClass, ArtifactPayloadRefs, AuthSourceClass, BreakGlassDisclosure,
    BreakGlassStateClass, CompatibilityImpactClass, CompatibilityNote, ContinuityClass,
    ContinuityNote, EvidenceFreshnessClass, EvidenceRef, ImmutableDigest, PromotionEventClass,
    PromotionStage, PromotionTimelineStep, RolloutRing, SemanticChangeClass,
};
use crate::stable_claim_manifest::{FreshnessSlo, FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, StableClaimLevel,
};

use super::{
    ArtifactGraphNode, FamilyPromotionLedger, HistoryPointerClass, HistoryReconstructionParity,
    LedgerState, M5ArtifactGraphPromotionRegister, M5ArtifactGraphPromotionStopRule,
    M5ArtifactGraphPromotionSummary, NarrowingReason, ParityState, StopAction,
    M5_ARTIFACT_GRAPH_PROMOTION_RECORD_KIND, M5_ARTIFACT_GRAPH_PROMOTION_SCHEMA_VERSION,
};

const AS_OF: &str = "2026-06-15";
const SLO_REGISTER_REF: &str = "release/freshness_slo_register";
const TARGET_MAX_AGE_DAYS: u32 = 90;
const WARN_WITHIN_DAYS: u32 = 14;

/// Builds the canonical M5 artifact-graph promotion-ledger register in code.
pub fn build_m5_artifact_graph_promotion_ledger() -> M5ArtifactGraphPromotionRegister {
    let rows = vec![
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::NotebookPack,
            slug: "notebook-pack",
            title: "Notebook pack promotion ledger",
            family_summary: "Records promotions of notebook packs and notebook-derived outputs.",
            claim_label: StableClaimLevel::Stable,
            owner: "notebook-release",
            final_stage: PromotionStage::Stable,
            final_event: PromotionEventClass::StablePublished,
            auth_source_class: AuthSourceClass::CiOidcReleaseIdentity,
            rollout_ring: RolloutRing::Stable,
            slo_state: FreshnessSloState::Current,
            with_break_glass: false,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::RequestDataAsset,
            slug: "request-data-asset",
            title: "Request/data asset promotion ledger",
            family_summary: "Records promotions of saved requests, datasets, and request fixtures.",
            claim_label: StableClaimLevel::Stable,
            owner: "data-release",
            final_stage: PromotionStage::Stable,
            final_event: PromotionEventClass::StablePublished,
            auth_source_class: AuthSourceClass::CiOidcReleaseIdentity,
            rollout_ring: RolloutRing::Stable,
            slo_state: FreshnessSloState::DueForRefresh,
            with_break_glass: false,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::ProfilerReplayArtifact,
            slug: "profiler-replay",
            title: "Profiler/replay artifact promotion ledger",
            family_summary:
                "Records promotions of profiler traces and replay recordings to the mirror feed.",
            claim_label: StableClaimLevel::Stable,
            owner: "profiler-release",
            final_stage: PromotionStage::MirrorPublished,
            final_event: PromotionEventClass::MirrorPublished,
            auth_source_class: AuthSourceClass::MirrorOperatorReceipt,
            rollout_ring: RolloutRing::MirrorOnly,
            slo_state: FreshnessSloState::Current,
            with_break_glass: false,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::FrameworkTemplatePack,
            slug: "framework-template-pack",
            title: "Framework/template pack promotion ledger",
            family_summary: "Records promotions of framework and template packs to the registry.",
            claim_label: StableClaimLevel::Stable,
            owner: "framework-release",
            final_stage: PromotionStage::RegistryPublished,
            final_event: PromotionEventClass::RegistryPublished,
            auth_source_class: AuthSourceClass::RegistryPublisherIdentity,
            rollout_ring: RolloutRing::Stable,
            slo_state: FreshnessSloState::Current,
            with_break_glass: false,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::DocsPack,
            slug: "docs-pack",
            title: "Docs pack promotion ledger",
            family_summary: "Records promotions of user-facing and embedded documentation packs.",
            claim_label: StableClaimLevel::Lts,
            owner: "docs-release",
            final_stage: PromotionStage::Lts,
            final_event: PromotionEventClass::LtsPublished,
            auth_source_class: AuthSourceClass::ReleaseVaultToken,
            rollout_ring: RolloutRing::Lts,
            slo_state: FreshnessSloState::Current,
            // The docs ledger captures a reconciled out-of-band correction in the
            // same timeline model, proving break-glass rides ordinary promotions.
            with_break_glass: true,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::ModelPack,
            slug: "model-pack",
            title: "Model pack promotion ledger",
            family_summary:
                "Records promotions of local model bundles and metadata to the mirror feed.",
            claim_label: StableClaimLevel::Stable,
            owner: "model-release",
            final_stage: PromotionStage::MirrorPublished,
            final_event: PromotionEventClass::MirrorPublished,
            auth_source_class: AuthSourceClass::MirrorOperatorReceipt,
            rollout_ring: RolloutRing::MirrorOnly,
            slo_state: FreshnessSloState::DueForRefresh,
            with_break_glass: false,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::CompanionOffboardingPacket,
            slug: "companion-offboarding",
            title: "Companion/offboarding packet promotion ledger",
            family_summary: "Records promotions of companion and offboarding packets.",
            claim_label: StableClaimLevel::Stable,
            owner: "companion-release",
            final_stage: PromotionStage::Stable,
            final_event: PromotionEventClass::StablePublished,
            auth_source_class: AuthSourceClass::CiOidcReleaseIdentity,
            rollout_ring: RolloutRing::Stable,
            slo_state: FreshnessSloState::Current,
            with_break_glass: false,
        }),
        managed_output_ledger(),
    ];

    let release_blocking_candidate_refs = rows
        .iter()
        .filter(|r| r.release_blocking)
        .map(|r| r.candidate_ref.clone())
        .collect();

    let mut register = M5ArtifactGraphPromotionRegister {
        schema_version: M5_ARTIFACT_GRAPH_PROMOTION_SCHEMA_VERSION,
        record_kind: M5_ARTIFACT_GRAPH_PROMOTION_RECORD_KIND.to_owned(),
        manifest_id: "m5-artifact-graph-promotion-ledgers".to_owned(),
        status: "frozen".to_owned(),
        overview_page:
            "docs/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.md"
                .to_owned(),
        as_of: AS_OF.to_owned(),
        claim_manifest_ref: "release/stable_claim_manifest".to_owned(),
        publication_matrix_ref:
            "release/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix"
                .to_owned(),
        family_release_graph_ref:
            "release/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family"
                .to_owned(),
        release_center_model_ref: "release/release_center_object_model".to_owned(),
        lifecycle_labels: StableClaimLevel::ALL.to_vec(),
        family_kinds: M5ArtifactFamilyKind::ALL.to_vec(),
        ledger_states: LedgerState::ALL.to_vec(),
        history_pointer_classes: HistoryPointerClass::ALL.to_vec(),
        parity_states: ParityState::ALL.to_vec(),
        narrowing_reasons: NarrowingReason::ALL.to_vec(),
        stop_rule_actions: StopAction::ALL.to_vec(),
        launch_cutline: launch_cutline(),
        release_blocking_candidate_refs,
        stop_rules: stop_rules(),
        rows,
        publication: PromotionDecisionRecord {
            promotion_gate: "m5_artifact_graph_promotion_ledgers".to_owned(),
            decision: PromotionDecision::Proceed,
            blocking_rule_ids: Vec::new(),
            blocking_claim_ids: Vec::new(),
            rationale:
                "Every release-blocking family at or above the cutline reconstructs the same immutable promotion history across release-center and headless flows, binds every step — ordinary and break-glass — to immutable artifact-graph digests, exposes an audit/postmortem replay, discloses a reversible window, and rides fresh evidence; the managed-output family already inherits a below-cutline claim, so it narrows without blocking the train."
                    .to_owned(),
        },
        summary: placeholder_summary(),
    };

    register.publication.decision = register.computed_publication_decision();
    register.publication.blocking_rule_ids = register.computed_blocking_rule_ids();
    register.publication.blocking_claim_ids = register.computed_blocking_entry_ids();
    register.summary = register.computed_summary();
    register
}

fn launch_cutline() -> LaunchCutline {
    LaunchCutline {
        cutline_level: StableClaimLevel::Stable,
        above_cutline_levels: StableClaimLevel::ABOVE_CUTLINE.to_vec(),
        below_cutline_levels: StableClaimLevel::BELOW_CUTLINE.to_vec(),
        description:
            "A family publishes at or above the cutline only when every promotion — including break-glass freezes, emergency publications, and out-of-band corrections — is captured as a complete timeline step bound to immutable artifact-graph digests, when release-center and headless flows reconstruct the same history, when an audit/postmortem export can replay it, when each step discloses a reversible window, when its evidence is fresh, when its proof packet is within SLO, and when it is owner-signed; otherwise it narrows below stable."
                .to_owned(),
    }
}

fn stop_rules() -> Vec<M5ArtifactGraphPromotionStopRule> {
    let rule = |id: &str,
                title: &str,
                trigger_reason: NarrowingReason,
                default_action: StopAction,
                rationale: &str| M5ArtifactGraphPromotionStopRule {
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
            "stop-timeline-capture-bypassed",
            "Timeline capture bypassed",
            NarrowingReason::TimelineCaptureBypassed,
            StopAction::CaptureTimelineStep,
            "A promotion or emergency action with no complete timeline step narrows the family; emergency flows may not bypass timeline capture.",
        ),
        rule(
            "stop-digest-binding-missing",
            "Digest binding missing",
            NarrowingReason::DigestBindingMissing,
            StopAction::BindImmutableDigest,
            "A timeline step that binds no immutable digest narrows the family; promotion history is anchored to immutable graph material.",
        ),
        rule(
            "stop-affected-node-set-incomplete",
            "Affected node set incomplete",
            NarrowingReason::AffectedNodeSetIncomplete,
            StopAction::CompleteAffectedNodeSet,
            "A step that cites a digest resolving to no node in the affected set narrows the family.",
        ),
        rule(
            "stop-mutable-latest-pointer",
            "Mutable latest pointer",
            NarrowingReason::MutableLatestPointer,
            StopAction::PinImmutableHistory,
            "A mutable 'latest' pointer may not stand in for immutable graph history.",
        ),
        rule(
            "stop-reconstruction-divergent",
            "Reconstruction divergent",
            NarrowingReason::ReconstructionDivergent,
            StopAction::ReconcileReconstruction,
            "Release-center and headless flows must reconstruct the same promotion history.",
        ),
        rule(
            "stop-audit-replay-unavailable",
            "Audit replay unavailable",
            NarrowingReason::AuditReplayUnavailable,
            StopAction::RestoreAuditReplay,
            "An audit/postmortem export must replay who promoted what, when, on which evidence, and with which reversible window.",
        ),
        rule(
            "stop-break-glass-unreconciled",
            "Break-glass unreconciled",
            NarrowingReason::BreakGlassUnreconciled,
            StopAction::ReconcileBreakGlass,
            "A break-glass action active past its reconciliation window narrows the family.",
        ),
        rule(
            "stop-reversible-window-undisclosed",
            "Reversible window undisclosed",
            NarrowingReason::ReversibleWindowUndisclosed,
            StopAction::DiscloseReversibleWindow,
            "A step that discloses neither a reversible window nor a rollback target narrows the family.",
        ),
        rule(
            "stop-evidence-stale",
            "Step evidence stale",
            NarrowingReason::EvidenceStale,
            StopAction::RecaptureEvidence,
            "A step riding stale or missing blocking evidence narrows the family.",
        ),
        rule(
            "stop-proof-packet-stale",
            "Proof packet stale",
            NarrowingReason::ProofPacketStale,
            StopAction::RefreshProofPacket,
            "A proof packet outside its freshness SLO narrows the family.",
        ),
        rule(
            "stop-proof-packet-missing",
            "Proof packet missing",
            NarrowingReason::ProofPacketMissing,
            StopAction::RefreshProofPacket,
            "A family without a captured proof packet narrows below the cutline.",
        ),
        rule(
            "stop-owner-signoff-missing",
            "Owner sign-off missing",
            NarrowingReason::OwnerManifestUnsigned,
            StopAction::RequestOwnerSignoff,
            "A family without owner sign-off cannot hold its claimed label.",
        ),
        rule(
            "stop-waiver-expired",
            "Waiver expired",
            NarrowingReason::WaiverExpired,
            StopAction::RenewWaiver,
            "A family relying on an expired waiver narrows below the cutline.",
        ),
    ]
}

struct HeldSpec {
    family_kind: M5ArtifactFamilyKind,
    slug: &'static str,
    title: &'static str,
    family_summary: &'static str,
    claim_label: StableClaimLevel,
    owner: &'static str,
    final_stage: PromotionStage,
    final_event: PromotionEventClass,
    auth_source_class: AuthSourceClass,
    rollout_ring: RolloutRing,
    slo_state: FreshnessSloState,
    with_break_glass: bool,
}

fn family_class(kind: M5ArtifactFamilyKind) -> ArtifactFamilyClass {
    match kind {
        M5ArtifactFamilyKind::NotebookPack => ArtifactFamilyClass::ExtensionPackage,
        M5ArtifactFamilyKind::RequestDataAsset => ArtifactFamilyClass::SchemaExport,
        M5ArtifactFamilyKind::ProfilerReplayArtifact => ArtifactFamilyClass::ReleaseEvidencePacket,
        M5ArtifactFamilyKind::FrameworkTemplatePack => ArtifactFamilyClass::ExtensionPackage,
        M5ArtifactFamilyKind::DocsPack => ArtifactFamilyClass::DocsPack,
        M5ArtifactFamilyKind::ModelPack => ArtifactFamilyClass::SdkArtifact,
        M5ArtifactFamilyKind::CompanionOffboardingPacket => ArtifactFamilyClass::PolicyBundle,
        M5ArtifactFamilyKind::ManagedOutput => ArtifactFamilyClass::ReleaseEvidencePacket,
    }
}

/// The two immutable-digest ids for a family's affected node set.
fn node_digest_ids(slug: &str) -> [String; 2] {
    [
        format!("digest/m5-{slug}/primary"),
        format!("digest/m5-{slug}/sidecar"),
    ]
}

fn node_set(kind: M5ArtifactFamilyKind, slug: &str) -> Vec<ArtifactGraphNode> {
    let ids = node_digest_ids(slug);
    let class = family_class(kind);
    vec![
        ArtifactGraphNode {
            node_id: ids[0].clone(),
            artifact_ref: format!("artifact/m5/{slug}/primary"),
            digest: ImmutableDigest {
                digest_id: ids[0].clone(),
                artifact_ref: format!("artifact/m5/{slug}/primary"),
                family_class: class,
                algorithm: "sha256".to_owned(),
                digest_ref: format!("sha256/m5-{slug}/primary"),
            },
            exact_build_identity_ref: format!("exact_build/m5-{slug}"),
            summary: "Primary artifact-graph node carrying the family's published material."
                .to_owned(),
        },
        ArtifactGraphNode {
            node_id: ids[1].clone(),
            artifact_ref: format!("artifact/m5/{slug}/sidecar"),
            digest: ImmutableDigest {
                digest_id: ids[1].clone(),
                artifact_ref: format!("artifact/m5/{slug}/sidecar"),
                family_class: class,
                algorithm: "sha256".to_owned(),
                digest_ref: format!("sha256/m5-{slug}/sidecar"),
            },
            exact_build_identity_ref: format!("exact_build/m5-{slug}"),
            summary: "Sidecar node carrying symbols, docs, and schema material for the family."
                .to_owned(),
        },
    ]
}

fn payload_refs(slug: &str) -> ArtifactPayloadRefs {
    ArtifactPayloadRefs {
        symbol_refs: vec![format!("symbols/m5-{slug}")],
        docs_pack_refs: vec![format!("docs/m5-{slug}")],
        schema_refs: vec![format!("schema/m5-{slug}")],
        compatibility_note_refs: vec![format!("compat/m5-{slug}")],
        advisory_refs: vec![format!("advisory/m5-{slug}")],
        mirror_metadata_refs: vec![format!("mirror/m5-{slug}/metadata")],
    }
}

fn current_evidence(slug: &str, step: &str) -> Vec<EvidenceRef> {
    vec![EvidenceRef {
        evidence_ref: format!("evidence/m5-{slug}/{step}"),
        evidence_kind: "promotion_evidence_bundle".to_owned(),
        freshness_class: EvidenceFreshnessClass::Current,
        generated_at: Some(AS_OF.to_owned()),
        required_for_promotion: true,
        summary: "Evidence bundle backing the promotion step.".to_owned(),
    }]
}

fn compat_notes(slug: &str) -> Vec<CompatibilityNote> {
    vec![CompatibilityNote {
        note_id: format!("compat/m5-{slug}/step"),
        impact_class: CompatibilityImpactClass::None,
        affected_surface: format!("artifact/m5/{slug}"),
        public_surface: false,
        summary: "No public-surface compatibility impact for the step.".to_owned(),
        source_refs: vec![format!("diff/m5-{slug}")],
    }]
}

fn continuity_notes(slug: &str) -> Vec<ContinuityNote> {
    vec![ContinuityNote {
        note_id: format!("continuity/m5-{slug}/step"),
        continuity_class: ContinuityClass::RollbackCoordinated,
        summary: "Rollback target and mirror continuity are coordinated for the step.".to_owned(),
        known_issue_refs: Vec::new(),
        support_refs: vec![format!("support/m5-{slug}")],
    }]
}

fn ordinary_step(
    spec: &HeldSpec,
    index: usize,
    source_stage: PromotionStage,
    destination_stage: PromotionStage,
    event_class: PromotionEventClass,
) -> PromotionTimelineStep {
    let slug = spec.slug;
    let ids = node_digest_ids(slug);
    PromotionTimelineStep {
        timeline_step_id: format!("step/m5-{slug}/{index:02}"),
        candidate_ref: format!("candidate/m5-{slug}"),
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        source_stage,
        destination_stage,
        event_class,
        semantic_change_class: SemanticChangeClass::Minor,
        publish_target_refs: vec![format!("publish_target/m5-{slug}")],
        artifact_bundle_refs: vec![format!("bundle/m5-{slug}")],
        digest_refs: vec![ids[0].clone(), ids[1].clone()],
        evidence_refs: current_evidence(slug, &format!("step-{index:02}")),
        approving_actor_refs: vec![
            format!("actor/{}/release-engineer", spec.owner),
            format!("actor/{}/release-approver", spec.owner),
        ],
        auth_source_class: spec.auth_source_class,
        rollout_ring: spec.rollout_ring,
        reversible_window: Some(format!(
            "72h reversible window with last-known-good rollback for {slug}"
        )),
        rollback_target_ref: format!("rollback/m5-{slug}/last-known-good"),
        payload_refs: payload_refs(slug),
        break_glass: BreakGlassDisclosure {
            state_class: BreakGlassStateClass::NotUsed,
            actor_class: None,
            break_glass_event_ref: None,
            reason_class: "ordinary_promotion".to_owned(),
            reconciliation_state: None,
            reconcile_by: None,
            follow_up_refs: Vec::new(),
        },
        compatibility_notes: compat_notes(slug),
        continuity_notes: continuity_notes(slug),
    }
}

/// A reconciled break-glass step: an out-of-band correction captured in the same
/// timeline model, bound to immutable digests, with reconciliation refs.
fn reconciled_break_glass_step(spec: &HeldSpec, index: usize) -> PromotionTimelineStep {
    let slug = spec.slug;
    let ids = node_digest_ids(slug);
    PromotionTimelineStep {
        timeline_step_id: format!("step/m5-{slug}/{index:02}"),
        candidate_ref: format!("candidate/m5-{slug}"),
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        source_stage: spec.final_stage,
        destination_stage: PromotionStage::Reconciled,
        event_class: PromotionEventClass::Reconciled,
        semantic_change_class: SemanticChangeClass::Hotfix,
        publish_target_refs: vec![format!("publish_target/m5-{slug}")],
        artifact_bundle_refs: vec![format!("bundle/m5-{slug}")],
        digest_refs: vec![ids[0].clone(), ids[1].clone()],
        evidence_refs: current_evidence(slug, "break-glass"),
        approving_actor_refs: vec![
            format!("actor/{}/incident-commander", spec.owner),
            format!("actor/{}/release-approver", spec.owner),
        ],
        auth_source_class: AuthSourceClass::SecurityEmergencyQuorum,
        rollout_ring: spec.rollout_ring,
        reversible_window: Some(format!(
            "24h reversible window for the out-of-band {slug} correction"
        )),
        rollback_target_ref: format!("rollback/m5-{slug}/last-known-good"),
        payload_refs: payload_refs(slug),
        break_glass: BreakGlassDisclosure {
            state_class: BreakGlassStateClass::Reconciled,
            actor_class: Some("security_emergency_quorum".to_owned()),
            break_glass_event_ref: Some(format!("break_glass/m5-{slug}/correction")),
            reason_class: "out_of_band_correction".to_owned(),
            reconciliation_state: Some("reconciled_by_signed_repin".to_owned()),
            reconcile_by: Some("2026-06-22".to_owned()),
            follow_up_refs: vec![format!("retrospective/m5-{slug}/correction")],
        },
        compatibility_notes: compat_notes(slug),
        continuity_notes: vec![ContinuityNote {
            note_id: format!("continuity/m5-{slug}/break-glass"),
            continuity_class: ContinuityClass::EmergencyReconciliation,
            summary: "Out-of-band correction reconciled into the immutable graph history."
                .to_owned(),
            known_issue_refs: Vec::new(),
            support_refs: vec![format!("support/m5-{slug}/incident")],
        }],
    }
}

fn held_ledger(spec: HeldSpec) -> FamilyPromotionLedger {
    let slug = spec.slug;
    let mut timeline = vec![
        ordinary_step(
            &spec,
            1,
            PromotionStage::Draft,
            PromotionStage::LocalPreview,
            PromotionEventClass::LocalBuildRecorded,
        ),
        ordinary_step(
            &spec,
            2,
            PromotionStage::LocalPreview,
            PromotionStage::InternalRing,
            PromotionEventClass::CandidatePromoted,
        ),
        ordinary_step(
            &spec,
            3,
            PromotionStage::StableCandidate,
            spec.final_stage,
            spec.final_event,
        ),
    ];
    if spec.with_break_glass {
        timeline.push(reconciled_break_glass_step(&spec, 4));
    }
    let step_ids: Vec<String> = timeline
        .iter()
        .map(|step| step.timeline_step_id.clone())
        .collect();
    let history_digest = format!("sha256/history/m5-{slug}");

    FamilyPromotionLedger {
        entry_id: format!("ledger-{slug}"),
        title: spec.title.to_owned(),
        family_kind: spec.family_kind,
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        candidate_ref: format!("candidate/m5-{slug}"),
        family_summary: spec.family_summary.to_owned(),
        release_blocking: true,
        claim_ref: format!("claim/m5-{slug}"),
        claim_label: spec.claim_label,
        ledger_state: LedgerState::Reconstructable,
        history_pointer_class: HistoryPointerClass::ImmutableGraphHistory,
        affected_node_set: node_set(spec.family_kind, slug),
        timeline,
        reconstruction: HistoryReconstructionParity {
            release_center_history_ref: format!("release_center/history/m5-{slug}"),
            headless_history_ref: format!("headless/history/m5-{slug}"),
            release_center_history_digest: history_digest.clone(),
            headless_history_digest: history_digest.clone(),
            audit_export_ref: format!("audit/replay/m5-{slug}"),
            audit_export_digest: history_digest,
            reconstructed_step_ids: step_ids,
            parity_state: ParityState::Matched,
        },
        proof_packet: proof_packet(slug, spec.slo_state),
        waiver: None,
        owner_signoff: signed(spec.owner),
        active_narrowing_reasons: Vec::new(),
        published_label: spec.claim_label,
        rationale: if spec.with_break_glass {
            "Immutable promotion history — including a reconciled out-of-band break-glass correction — reconstructs identically across release-center and headless flows, every step is digest-bound to the affected node set, an audit/postmortem replay is available, each step discloses a reversible window, evidence is fresh, the proof packet is within SLO, and the owner signed; the family holds its claimed label.".to_owned()
        } else {
            "Immutable promotion history reconstructs identically across release-center and headless flows, every step is digest-bound to the affected node set, an audit/postmortem replay is available, each step discloses a reversible window, evidence is fresh, the proof packet is within SLO, and the owner signed; the family holds its claimed label.".to_owned()
        },
        publication_destinations: vec![
            "release_center".to_owned(),
            "support_export".to_owned(),
            "audit_export".to_owned(),
            "diagnostics".to_owned(),
        ],
    }
}

/// The managed-output ledger narrows: it is driven by a mutable latest pointer,
/// its release-center and headless reconstructions diverge, no audit replay is
/// available, it rides stale evidence, it carries an expired break-glass freeze,
/// and its proof packet breached its SLO. Every gap is captured — the break-glass
/// step is still digest-bound, so the guardrail against bypassing capture holds.
fn managed_output_ledger() -> FamilyPromotionLedger {
    let slug = "managed-output";
    let ids = node_digest_ids(slug);
    let ordinary = PromotionTimelineStep {
        timeline_step_id: format!("step/m5-{slug}/01"),
        candidate_ref: format!("candidate/m5-{slug}"),
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        source_stage: PromotionStage::Draft,
        destination_stage: PromotionStage::MirrorStaged,
        event_class: PromotionEventClass::LocalBuildRecorded,
        semantic_change_class: SemanticChangeClass::Major,
        publish_target_refs: vec![format!("publish_target/m5-{slug}")],
        artifact_bundle_refs: vec![format!("bundle/m5-{slug}")],
        digest_refs: vec![ids[0].clone(), ids[1].clone()],
        evidence_refs: vec![EvidenceRef {
            evidence_ref: format!("evidence/m5-{slug}/stale"),
            evidence_kind: "promotion_evidence_bundle".to_owned(),
            freshness_class: EvidenceFreshnessClass::StaleBlocking,
            generated_at: Some("2026-01-02".to_owned()),
            required_for_promotion: true,
            summary: "Promotion evidence is stale and blocks promotion.".to_owned(),
        }],
        approving_actor_refs: vec![format!("actor/managed-release/release-engineer")],
        auth_source_class: AuthSourceClass::MirrorOperatorReceipt,
        rollout_ring: RolloutRing::MirrorOnly,
        reversible_window: Some("48h reversible window for the managed-output stage".to_owned()),
        rollback_target_ref: format!("rollback/m5-{slug}/last-known-good"),
        payload_refs: payload_refs(slug),
        break_glass: BreakGlassDisclosure {
            state_class: BreakGlassStateClass::NotUsed,
            actor_class: None,
            break_glass_event_ref: None,
            reason_class: "ordinary_promotion".to_owned(),
            reconciliation_state: None,
            reconcile_by: None,
            follow_up_refs: Vec::new(),
        },
        compatibility_notes: compat_notes(slug),
        continuity_notes: continuity_notes(slug),
    };
    let expired_break_glass = PromotionTimelineStep {
        timeline_step_id: format!("step/m5-{slug}/02"),
        candidate_ref: format!("candidate/m5-{slug}"),
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        source_stage: PromotionStage::MirrorStaged,
        destination_stage: PromotionStage::EmergencyActive,
        event_class: PromotionEventClass::MirrorPublished,
        semantic_change_class: SemanticChangeClass::Hotfix,
        publish_target_refs: vec![format!("publish_target/m5-{slug}")],
        artifact_bundle_refs: vec![format!("bundle/m5-{slug}")],
        // The emergency step is still captured and digest-bound: the guardrail
        // against bypassing capture holds even though the freeze is unreconciled.
        digest_refs: vec![ids[0].clone(), ids[1].clone()],
        evidence_refs: current_evidence(slug, "break-glass"),
        approving_actor_refs: vec![format!("actor/managed-release/incident-commander")],
        auth_source_class: AuthSourceClass::SecurityEmergencyQuorum,
        rollout_ring: RolloutRing::Emergency,
        reversible_window: Some("emergency freeze window for the managed-output lane".to_owned()),
        rollback_target_ref: format!("rollback/m5-{slug}/last-known-good"),
        payload_refs: payload_refs(slug),
        break_glass: BreakGlassDisclosure {
            state_class: BreakGlassStateClass::ExpiredWithoutReconciliation,
            actor_class: Some("security_emergency_quorum".to_owned()),
            break_glass_event_ref: Some(format!("break_glass/m5-{slug}/freeze")),
            reason_class: "emergency_freeze".to_owned(),
            reconciliation_state: Some("expired_without_reconciliation".to_owned()),
            reconcile_by: Some("2026-02-01".to_owned()),
            follow_up_refs: Vec::new(),
        },
        compatibility_notes: compat_notes(slug),
        continuity_notes: vec![ContinuityNote {
            note_id: format!("continuity/m5-{slug}/break-glass"),
            continuity_class: ContinuityClass::EmergencyReconciliation,
            summary: "Emergency freeze awaiting reconciliation; window expired.".to_owned(),
            known_issue_refs: vec![format!("known_issue/m5-{slug}/freeze")],
            support_refs: vec![format!("support/m5-{slug}/incident")],
        }],
    };

    FamilyPromotionLedger {
        entry_id: format!("ledger-{slug}"),
        title: "Managed output promotion ledger".to_owned(),
        family_kind: M5ArtifactFamilyKind::ManagedOutput,
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        candidate_ref: format!("candidate/m5-{slug}"),
        family_summary: "Records promotions of managed outputs from managed/tenant-scoped lanes."
            .to_owned(),
        release_blocking: true,
        claim_ref: format!("claim/m5-{slug}"),
        claim_label: StableClaimLevel::Beta,
        ledger_state: LedgerState::HistoryGap,
        history_pointer_class: HistoryPointerClass::MutableLatestPointer,
        affected_node_set: node_set(M5ArtifactFamilyKind::ManagedOutput, slug),
        timeline: vec![ordinary, expired_break_glass],
        reconstruction: HistoryReconstructionParity {
            release_center_history_ref: format!("release_center/history/m5-{slug}"),
            headless_history_ref: format!("headless/history/m5-{slug}"),
            release_center_history_digest: format!("sha256/history/m5-{slug}/release-center"),
            headless_history_digest: format!("sha256/history/m5-{slug}/headless"),
            audit_export_ref: format!("audit/replay/m5-{slug}"),
            // No reconstructable digest: the divergent history cannot be replayed.
            audit_export_digest: String::new(),
            reconstructed_step_ids: Vec::new(),
            parity_state: ParityState::Divergent,
        },
        proof_packet: proof_packet(slug, FreshnessSloState::Breached),
        waiver: None,
        owner_signoff: signed("managed-release"),
        active_narrowing_reasons: vec![
            NarrowingReason::MutableLatestPointer,
            NarrowingReason::ReconstructionDivergent,
            NarrowingReason::AuditReplayUnavailable,
            NarrowingReason::BreakGlassUnreconciled,
            NarrowingReason::EvidenceStale,
            NarrowingReason::ProofPacketStale,
        ],
        published_label: StableClaimLevel::Preview,
        rationale:
            "The managed-output family drives publication from a mutable 'latest' pointer, its release-center and headless reconstructions diverge, no audit/postmortem replay is available, a break-glass freeze expired without reconciliation, a step rides stale blocking evidence, and its proof packet breached its SLO; it inherits its below-cutline beta claim and narrows to preview, naming every gap."
                .to_owned(),
        publication_destinations: vec![
            "release_center".to_owned(),
            "support_export".to_owned(),
            "audit_export".to_owned(),
            "diagnostics".to_owned(),
        ],
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

fn placeholder_summary() -> M5ArtifactGraphPromotionSummary {
    M5ArtifactGraphPromotionSummary {
        total_entries: 0,
        total_artifact_graphs: 0,
        entries_reconstructable: 0,
        entries_narrowed: 0,
        entries_on_active_waiver: 0,
        entries_with_timeline_gap: 0,
        entries_with_node_set_gap: 0,
        entries_with_reconstruction_gap: 0,
        entries_with_break_glass_gap: 0,
        entries_with_mutable_pointer_gap: 0,
        release_blocking_total: 0,
        release_blocking_reconstructable: 0,
        release_blocking_narrowed: 0,
        notebook_pack_entries: 0,
        request_data_asset_entries: 0,
        profiler_replay_entries: 0,
        framework_template_entries: 0,
        docs_pack_entries: 0,
        model_pack_entries: 0,
        companion_offboarding_entries: 0,
        managed_output_entries: 0,
        parity_matched: 0,
        parity_divergent: 0,
        parity_missing: 0,
        history_immutable: 0,
        history_mutable: 0,
        packets_current: 0,
        packets_due_for_refresh: 0,
        packets_breached: 0,
        packets_missing: 0,
        total_timeline_steps: 0,
        total_break_glass_steps: 0,
        total_active_narrowing_reasons: 0,
        rules_firing: 0,
    }
}
