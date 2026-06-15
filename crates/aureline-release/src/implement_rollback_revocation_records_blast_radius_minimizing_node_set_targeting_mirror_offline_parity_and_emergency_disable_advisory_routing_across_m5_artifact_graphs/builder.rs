//! Deterministic builder for the M5 artifact-graph rollback/revocation register.
//!
//! [`build_m5_artifact_graph_recovery_register`] constructs the same register that
//! the checked-in JSON embeds, so the headless emitter can regenerate the artifact
//! and a test can prove the embedded JSON never drifts from the code.

use crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::M5ArtifactFamilyKind;
use crate::release_center_model::{
    ArtifactFamilyClass, ArtifactGraphConsistency, AuthSourceClass, BlastRadiusClass,
    BreakGlassDisclosure, BreakGlassStateClass, CompatibilityImpactClass, CompatibilityNote,
    ContinuityClass, ContinuityNote, EvidenceFreshnessClass, EvidenceRef, ImmutableDigest,
    RollbackOrRevocationKind, RollbackOrRevocationRecord, RolloutRing,
};
use crate::stable_claim_manifest::{FreshnessSlo, FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, StableClaimLevel,
};

use super::{
    ChannelDelivery, ChannelDeliveryParity, ChannelDeliveryState, DeliveryChannel,
    FamilyRecoveryLedger, M5ArtifactGraphRecoveryRegister, M5ArtifactGraphRecoveryStopRule,
    M5ArtifactGraphRecoverySummary, NarrowingReason, RecoveryGraphNode, RecoveryLedgerState,
    StopAction, M5_ARTIFACT_GRAPH_RECOVERY_RECORD_KIND, M5_ARTIFACT_GRAPH_RECOVERY_SCHEMA_VERSION,
};

const AS_OF: &str = "2026-06-15";
const SLO_REGISTER_REF: &str = "release/freshness_slo_register";
const TARGET_MAX_AGE_DAYS: u32 = 90;
const WARN_WITHIN_DAYS: u32 = 14;

/// Builds the canonical M5 artifact-graph rollback/revocation register in code.
pub fn build_m5_artifact_graph_recovery_register() -> M5ArtifactGraphRecoveryRegister {
    let rows = vec![
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::NotebookPack,
            slug: "notebook-pack",
            title: "Notebook pack recovery ledger",
            family_summary:
                "Rollback/revocation truth for notebook packs and notebook-derived outputs.",
            claim_label: StableClaimLevel::Stable,
            owner: "notebook-release",
            kind: RollbackOrRevocationKind::Rollback,
            auth_source_class: AuthSourceClass::CiOidcReleaseIdentity,
            rollout_ring: RolloutRing::Stable,
            slo_state: FreshnessSloState::Current,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::RequestDataAsset,
            slug: "request-data-asset",
            title: "Request/data asset recovery ledger",
            family_summary:
                "Rollback/revocation truth for saved requests, datasets, and request fixtures.",
            claim_label: StableClaimLevel::Stable,
            owner: "data-release",
            kind: RollbackOrRevocationKind::Revoke,
            auth_source_class: AuthSourceClass::CiOidcReleaseIdentity,
            rollout_ring: RolloutRing::Stable,
            slo_state: FreshnessSloState::DueForRefresh,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::ProfilerReplayArtifact,
            slug: "profiler-replay",
            title: "Profiler/replay artifact recovery ledger",
            family_summary:
                "Rollback/revocation truth for profiler traces and replay recordings on the mirror feed.",
            claim_label: StableClaimLevel::Stable,
            owner: "profiler-release",
            kind: RollbackOrRevocationKind::Yank,
            auth_source_class: AuthSourceClass::MirrorOperatorReceipt,
            rollout_ring: RolloutRing::MirrorOnly,
            slo_state: FreshnessSloState::Current,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::FrameworkTemplatePack,
            slug: "framework-template-pack",
            title: "Framework/template pack recovery ledger",
            family_summary:
                "Rollback/revocation truth for framework and template packs in the registry.",
            claim_label: StableClaimLevel::Stable,
            owner: "framework-release",
            kind: RollbackOrRevocationKind::Repin,
            auth_source_class: AuthSourceClass::RegistryPublisherIdentity,
            rollout_ring: RolloutRing::Stable,
            slo_state: FreshnessSloState::Current,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::DocsPack,
            slug: "docs-pack",
            title: "Docs pack recovery ledger",
            family_summary:
                "Rollback/revocation truth for user-facing and embedded documentation packs.",
            claim_label: StableClaimLevel::Lts,
            owner: "docs-release",
            // The docs ledger captures a reconciled emergency-disable in the same
            // record model, proving emergency response rides ordinary recovery.
            kind: RollbackOrRevocationKind::EmergencyDisable,
            auth_source_class: AuthSourceClass::SecurityEmergencyQuorum,
            rollout_ring: RolloutRing::Emergency,
            slo_state: FreshnessSloState::Current,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::ModelPack,
            slug: "model-pack",
            title: "Model pack recovery ledger",
            family_summary:
                "Rollback/revocation truth for local model bundles and metadata on the mirror feed.",
            claim_label: StableClaimLevel::Stable,
            owner: "model-release",
            kind: RollbackOrRevocationKind::Rollback,
            auth_source_class: AuthSourceClass::MirrorOperatorReceipt,
            rollout_ring: RolloutRing::MirrorOnly,
            slo_state: FreshnessSloState::DueForRefresh,
        }),
        held_ledger(HeldSpec {
            family_kind: M5ArtifactFamilyKind::CompanionOffboardingPacket,
            slug: "companion-offboarding",
            title: "Companion/offboarding packet recovery ledger",
            family_summary:
                "Rollback/revocation truth for companion and offboarding packets.",
            claim_label: StableClaimLevel::Stable,
            owner: "companion-release",
            kind: RollbackOrRevocationKind::Revoke,
            auth_source_class: AuthSourceClass::CiOidcReleaseIdentity,
            rollout_ring: RolloutRing::Stable,
            slo_state: FreshnessSloState::Current,
        }),
        managed_output_ledger(),
    ];

    let release_blocking_candidate_refs = rows
        .iter()
        .filter(|r| r.release_blocking)
        .map(|r| r.candidate_ref.clone())
        .collect();

    let mut register = M5ArtifactGraphRecoveryRegister {
        schema_version: M5_ARTIFACT_GRAPH_RECOVERY_SCHEMA_VERSION,
        record_kind: M5_ARTIFACT_GRAPH_RECOVERY_RECORD_KIND.to_owned(),
        manifest_id: "m5-artifact-graph-recovery-ledgers".to_owned(),
        status: "frozen".to_owned(),
        overview_page:
            "docs/m5/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs.md"
                .to_owned(),
        as_of: AS_OF.to_owned(),
        claim_manifest_ref: "release/stable_claim_manifest".to_owned(),
        publication_matrix_ref:
            "release/freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix"
                .to_owned(),
        family_release_graph_ref:
            "release/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family"
                .to_owned(),
        promotion_ledger_ref:
            "release/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs"
                .to_owned(),
        release_center_model_ref: "release/release_center_object_model".to_owned(),
        lifecycle_labels: StableClaimLevel::ALL.to_vec(),
        family_kinds: M5ArtifactFamilyKind::ALL.to_vec(),
        recovery_record_kinds: vec![
            RollbackOrRevocationKind::Rollback,
            RollbackOrRevocationKind::Revoke,
            RollbackOrRevocationKind::Yank,
            RollbackOrRevocationKind::Repin,
            RollbackOrRevocationKind::EmergencyDisable,
        ],
        delivery_channels: DeliveryChannel::ALL.to_vec(),
        channel_delivery_states: ChannelDeliveryState::ALL.to_vec(),
        ledger_states: RecoveryLedgerState::ALL.to_vec(),
        narrowing_reasons: NarrowingReason::ALL.to_vec(),
        stop_rule_actions: StopAction::ALL.to_vec(),
        launch_cutline: launch_cutline(),
        release_blocking_candidate_refs,
        stop_rules: stop_rules(),
        rows,
        publication: PromotionDecisionRecord {
            promotion_gate: "m5_artifact_graph_recovery_ledgers".to_owned(),
            decision: PromotionDecision::Proceed,
            blocking_rule_ids: Vec::new(),
            blocking_claim_ids: Vec::new(),
            rationale:
                "Every release-blocking family at or above the cutline targets the smallest affected node set, preserves every unaffected node as installable, keeps the artifact graph consistent, delivers the same recovery records and advisories to the hosted, mirrored, and offline channels, reconciles every emergency-disable, and rides fresh evidence; the managed-output family already inherits a below-cutline claim, so it narrows without blocking the train."
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
            "A family publishes at or above the cutline only when every recovery record targets the smallest affected node set and keeps unaffected nodes installable, when the artifact graph stays consistent, when the hosted, mirrored, and offline channels each receive the same recovery records and advisories, when every emergency-disable is advisory-routed and reconciled, when its evidence is fresh, when its proof packet is within SLO, and when it is owner-signed; otherwise it narrows below stable."
                .to_owned(),
    }
}

fn stop_rules() -> Vec<M5ArtifactGraphRecoveryStopRule> {
    let rule = |id: &str,
                title: &str,
                trigger_reason: NarrowingReason,
                default_action: StopAction,
                rationale: &str| M5ArtifactGraphRecoveryStopRule {
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
            "stop-blast-radius-unscoped",
            "Blast radius unscoped",
            NarrowingReason::BlastRadiusUnscoped,
            StopAction::MinimizeBlastRadius,
            "A recovery record that names no affected node or does not classify every node in the affected set narrows the family.",
        ),
        rule(
            "stop-unaffected-nodes-not-preserved",
            "Unaffected nodes not preserved",
            NarrowingReason::UnaffectedNodesNotPreserved,
            StopAction::PreserveUnaffectedNodes,
            "A recovery record that does not list an installable node in its preserved set narrows the family; unaffected nodes must stay installable.",
        ),
        rule(
            "stop-graph-consistency-broken",
            "Graph consistency broken",
            NarrowingReason::GraphConsistencyBroken,
            StopAction::RestoreGraphConsistency,
            "A recovery record that leaves the artifact graph broken narrows the family.",
        ),
        rule(
            "stop-last-known-good-missing",
            "Last-known-good missing",
            NarrowingReason::LastKnownGoodMissing,
            StopAction::BindLastKnownGood,
            "A rollback or repin record that cites no last-known-good target narrows the family.",
        ),
        rule(
            "stop-mirror-parity-missing",
            "Mirror parity missing",
            NarrowingReason::MirrorParityMissing,
            StopAction::DeliverMirrorTruth,
            "A family whose mirrored channel has no delivery path for the recovery truth narrows; mirrored customers are not second-class.",
        ),
        rule(
            "stop-offline-parity-missing",
            "Offline parity missing",
            NarrowingReason::OfflineParityMissing,
            StopAction::DeliverOfflineTruth,
            "A family whose offline channel has no delivery path for the recovery truth narrows; offline customers are not second-class.",
        ),
        rule(
            "stop-channel-delivery-stale",
            "Channel delivery stale",
            NarrowingReason::ChannelDeliveryStale,
            StopAction::RefreshChannelDelivery,
            "A family whose delivery channel carries stale recovery truth narrows.",
        ),
        rule(
            "stop-advisory-routing-missing",
            "Advisory routing missing",
            NarrowingReason::AdvisoryRoutingMissing,
            StopAction::RouteAdvisory,
            "A revocation, yank, or emergency-disable record that routes no security advisory narrows the family.",
        ),
        rule(
            "stop-emergency-disable-unreconciled",
            "Emergency disable unreconciled",
            NarrowingReason::EmergencyDisableUnreconciled,
            StopAction::ReconcileEmergencyDisable,
            "An emergency-disable record active past its reconciliation window narrows the family.",
        ),
        rule(
            "stop-evidence-stale",
            "Record evidence stale",
            NarrowingReason::EvidenceStale,
            StopAction::RecaptureEvidence,
            "A recovery record riding stale or missing blocking evidence narrows the family.",
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
    kind: RollbackOrRevocationKind,
    auth_source_class: AuthSourceClass,
    rollout_ring: RolloutRing,
    slo_state: FreshnessSloState,
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

fn primary_ref(slug: &str) -> String {
    format!("artifact/m5/{slug}/primary")
}

fn sidecar_ref(slug: &str) -> String {
    format!("artifact/m5/{slug}/sidecar")
}

/// A two-node set: the primary node is the one the recovery action targets (pulled),
/// the sidecar node stays installable (explicitly preserved) — the smallest affected
/// node set with the rest of the graph kept installable.
fn node_set(kind: M5ArtifactFamilyKind, slug: &str) -> Vec<RecoveryGraphNode> {
    let class = family_class(kind);
    vec![
        RecoveryGraphNode {
            node_id: format!("digest/m5-{slug}/primary"),
            artifact_ref: primary_ref(slug),
            digest: ImmutableDigest {
                digest_id: format!("digest/m5-{slug}/primary"),
                artifact_ref: primary_ref(slug),
                family_class: class,
                algorithm: "sha256".to_owned(),
                digest_ref: format!("sha256/m5-{slug}/primary"),
            },
            exact_build_identity_ref: format!("exact_build/m5-{slug}"),
            installable_after_action: false,
            summary: "Primary node the recovery action targets; pulled after the action."
                .to_owned(),
        },
        RecoveryGraphNode {
            node_id: format!("digest/m5-{slug}/sidecar"),
            artifact_ref: sidecar_ref(slug),
            digest: ImmutableDigest {
                digest_id: format!("digest/m5-{slug}/sidecar"),
                artifact_ref: sidecar_ref(slug),
                family_class: class,
                algorithm: "sha256".to_owned(),
                digest_ref: format!("sha256/m5-{slug}/sidecar"),
            },
            exact_build_identity_ref: format!("exact_build/m5-{slug}"),
            installable_after_action: true,
            summary: "Unaffected sidecar node preserved as installable after the action."
                .to_owned(),
        },
    ]
}

fn current_evidence(slug: &str) -> Vec<EvidenceRef> {
    vec![EvidenceRef {
        evidence_ref: format!("evidence/m5-{slug}/recovery"),
        evidence_kind: "recovery_evidence_bundle".to_owned(),
        freshness_class: EvidenceFreshnessClass::Current,
        generated_at: Some(AS_OF.to_owned()),
        required_for_promotion: true,
        summary: "Evidence bundle backing the recovery action.".to_owned(),
    }]
}

fn compat_notes(slug: &str) -> Vec<CompatibilityNote> {
    vec![CompatibilityNote {
        note_id: format!("compat/m5-{slug}/recovery"),
        impact_class: CompatibilityImpactClass::None,
        affected_surface: primary_ref(slug),
        public_surface: false,
        summary: "No public-surface compatibility impact for the unaffected nodes.".to_owned(),
        source_refs: vec![format!("diff/m5-{slug}")],
    }]
}

fn continuity_notes(slug: &str) -> Vec<ContinuityNote> {
    vec![
        ContinuityNote {
            note_id: format!("continuity/m5-{slug}/rollback"),
            continuity_class: ContinuityClass::RollbackCoordinated,
            summary: "Rollback target and last-known-good are coordinated across the graph."
                .to_owned(),
            known_issue_refs: Vec::new(),
            support_refs: vec![format!("support/m5-{slug}")],
        },
        ContinuityNote {
            note_id: format!("continuity/m5-{slug}/mirror"),
            continuity_class: ContinuityClass::MirrorContinuity,
            summary: "Mirror and offline feeds receive the same recovery record and advisory."
                .to_owned(),
            known_issue_refs: Vec::new(),
            support_refs: vec![format!("support/m5-{slug}/mirror")],
        },
    ]
}

fn held_record(spec: &HeldSpec) -> RollbackOrRevocationRecord {
    let slug = spec.slug;
    let break_glass = if spec.kind == RollbackOrRevocationKind::EmergencyDisable {
        BreakGlassDisclosure {
            state_class: BreakGlassStateClass::Reconciled,
            actor_class: Some("security_emergency_quorum".to_owned()),
            break_glass_event_ref: Some(format!("break_glass/m5-{slug}/disable")),
            reason_class: "emergency_disable".to_owned(),
            reconciliation_state: Some("reconciled_by_signed_repin".to_owned()),
            reconcile_by: Some("2026-06-22".to_owned()),
            follow_up_refs: vec![format!("retrospective/m5-{slug}/disable")],
        }
    } else {
        BreakGlassDisclosure {
            state_class: BreakGlassStateClass::NotUsed,
            actor_class: None,
            break_glass_event_ref: None,
            reason_class: "ordinary_recovery".to_owned(),
            reconciliation_state: None,
            reconcile_by: None,
            follow_up_refs: Vec::new(),
        }
    };
    RollbackOrRevocationRecord {
        record_id: format!("record/m5-{slug}/01"),
        kind: spec.kind,
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        affected_artifact_refs: vec![primary_ref(slug)],
        unaffected_artifact_refs: vec![sidecar_ref(slug)],
        blast_radius_class: BlastRadiusClass::SingleArtifactNode,
        last_known_good_ref: format!("rollback/m5-{slug}/last-known-good"),
        rollback_manifest_ref: Some(format!("rollback_manifest/m5-{slug}")),
        revocation_record_refs: vec![format!("revocation/m5-{slug}/01")],
        advisory_refs: vec![format!("advisory/m5-{slug}")],
        known_issue_refs: vec![format!("known_issue/m5-{slug}")],
        support_export_refs: vec![format!("support/m5-{slug}")],
        auth_source_class: spec.auth_source_class,
        rollout_ring: spec.rollout_ring,
        artifact_graph_consistency: ArtifactGraphConsistency::ConsistentScopedException,
        evidence_refs: current_evidence(slug),
        break_glass,
        compatibility_notes: compat_notes(slug),
        continuity_notes: continuity_notes(slug),
    }
}

fn current_channel_parity(slug: &str, record_ids: &[String]) -> ChannelDeliveryParity {
    let channel = |channel: DeliveryChannel, feed: &str| ChannelDelivery {
        channel,
        delivery_state: ChannelDeliveryState::Current,
        feed_ref: format!("feed/m5-{slug}/{feed}"),
        delivered_record_ids: record_ids.to_vec(),
        advisory_refs: vec![format!("advisory/m5-{slug}")],
        delivered_at: Some(AS_OF.to_owned()),
        summary: format!("{feed} channel received the current recovery record set and advisory."),
    };
    ChannelDeliveryParity {
        channels: vec![
            channel(DeliveryChannel::Hosted, "hosted"),
            channel(DeliveryChannel::Mirrored, "mirror"),
            channel(DeliveryChannel::Offline, "offline"),
        ],
        summary: "Hosted, mirrored, and offline channels are at recovery parity.".to_owned(),
    }
}

fn held_ledger(spec: HeldSpec) -> FamilyRecoveryLedger {
    let slug = spec.slug;
    let record = held_record(&spec);
    let record_ids = vec![record.record_id.clone()];
    let emergency = spec.kind == RollbackOrRevocationKind::EmergencyDisable;
    FamilyRecoveryLedger {
        entry_id: format!("ledger-{slug}"),
        title: spec.title.to_owned(),
        family_kind: spec.family_kind,
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        candidate_ref: format!("candidate/m5-{slug}"),
        family_summary: spec.family_summary.to_owned(),
        release_blocking: true,
        claim_ref: format!("claim/m5-{slug}"),
        claim_label: spec.claim_label,
        ledger_state: RecoveryLedgerState::Contained,
        affected_node_set: node_set(spec.family_kind, slug),
        recovery_records: vec![record],
        channel_parity: current_channel_parity(slug, &record_ids),
        proof_packet: proof_packet(slug, spec.slo_state),
        waiver: None,
        owner_signoff: signed(spec.owner),
        active_narrowing_reasons: Vec::new(),
        published_label: spec.claim_label,
        rationale: if emergency {
            "A reconciled emergency-disable, captured in the same record model as an ordinary rollback, targets only the affected node while the sidecar stays installable, routes a security advisory to the hosted, mirrored, and offline channels alike, keeps the graph consistent, rides fresh evidence within SLO, and is owner-signed; the family holds its claimed label.".to_owned()
        } else {
            "The recovery record targets the smallest affected node set while the sidecar stays installable, keeps the artifact graph consistent, delivers the same record and advisory to the hosted, mirrored, and offline channels, rides fresh evidence within SLO, and is owner-signed; the family holds its claimed label.".to_owned()
        },
        publication_destinations: vec![
            "release_center".to_owned(),
            "update_surface".to_owned(),
            "advisory_export".to_owned(),
            "support_export".to_owned(),
            "diagnostics".to_owned(),
        ],
    }
}

/// The managed-output ledger narrows: a revoke record leaves the graph broken, an
/// emergency-disable expired without reconciliation, a record rides stale blocking
/// evidence, every delivery channel carries stale recovery truth, and its proof
/// packet breached its SLO. No record over-revokes a preservable node and no channel
/// is excluded while another has the truth, so neither guardrail trips.
fn managed_output_ledger() -> FamilyRecoveryLedger {
    let slug = "managed-output";
    let kind = M5ArtifactFamilyKind::ManagedOutput;

    let expired_disable = RollbackOrRevocationRecord {
        record_id: format!("record/m5-{slug}/01"),
        kind: RollbackOrRevocationKind::EmergencyDisable,
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        affected_artifact_refs: vec![primary_ref(slug)],
        unaffected_artifact_refs: vec![sidecar_ref(slug)],
        blast_radius_class: BlastRadiusClass::SingleArtifactNode,
        last_known_good_ref: format!("rollback/m5-{slug}/last-known-good"),
        rollback_manifest_ref: Some(format!("rollback_manifest/m5-{slug}")),
        revocation_record_refs: vec![format!("revocation/m5-{slug}/01")],
        advisory_refs: vec![format!("advisory/m5-{slug}/disable")],
        known_issue_refs: vec![format!("known_issue/m5-{slug}/disable")],
        support_export_refs: vec![format!("support/m5-{slug}/incident")],
        auth_source_class: AuthSourceClass::SecurityEmergencyQuorum,
        rollout_ring: RolloutRing::Emergency,
        artifact_graph_consistency: ArtifactGraphConsistency::PendingReconciliation,
        evidence_refs: vec![EvidenceRef {
            evidence_ref: format!("evidence/m5-{slug}/stale"),
            evidence_kind: "recovery_evidence_bundle".to_owned(),
            freshness_class: EvidenceFreshnessClass::StaleBlocking,
            generated_at: Some("2026-01-02".to_owned()),
            required_for_promotion: true,
            summary: "Recovery evidence is stale and blocks the claim.".to_owned(),
        }],
        break_glass: BreakGlassDisclosure {
            state_class: BreakGlassStateClass::ExpiredWithoutReconciliation,
            actor_class: Some("security_emergency_quorum".to_owned()),
            break_glass_event_ref: Some(format!("break_glass/m5-{slug}/disable")),
            reason_class: "emergency_disable".to_owned(),
            reconciliation_state: Some("expired_without_reconciliation".to_owned()),
            reconcile_by: Some("2026-02-01".to_owned()),
            follow_up_refs: Vec::new(),
        },
        compatibility_notes: compat_notes(slug),
        continuity_notes: vec![ContinuityNote {
            note_id: format!("continuity/m5-{slug}/disable"),
            continuity_class: ContinuityClass::EmergencyReconciliation,
            summary: "Emergency disable awaiting reconciliation; window expired.".to_owned(),
            known_issue_refs: vec![format!("known_issue/m5-{slug}/disable")],
            support_refs: vec![format!("support/m5-{slug}/incident")],
        }],
    };

    let broken_revoke = RollbackOrRevocationRecord {
        record_id: format!("record/m5-{slug}/02"),
        kind: RollbackOrRevocationKind::Revoke,
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        affected_artifact_refs: vec![primary_ref(slug)],
        unaffected_artifact_refs: vec![sidecar_ref(slug)],
        blast_radius_class: BlastRadiusClass::ArtifactFamilyScoped,
        last_known_good_ref: format!("rollback/m5-{slug}/last-known-good"),
        rollback_manifest_ref: None,
        revocation_record_refs: vec![format!("revocation/m5-{slug}/02")],
        advisory_refs: vec![format!("advisory/m5-{slug}/revoke")],
        known_issue_refs: vec![format!("known_issue/m5-{slug}/revoke")],
        support_export_refs: vec![format!("support/m5-{slug}/revoke")],
        auth_source_class: AuthSourceClass::MirrorOperatorReceipt,
        rollout_ring: RolloutRing::MirrorOnly,
        artifact_graph_consistency: ArtifactGraphConsistency::Broken,
        evidence_refs: current_evidence(slug),
        break_glass: BreakGlassDisclosure {
            state_class: BreakGlassStateClass::NotUsed,
            actor_class: None,
            break_glass_event_ref: None,
            reason_class: "ordinary_recovery".to_owned(),
            reconciliation_state: None,
            reconcile_by: None,
            follow_up_refs: Vec::new(),
        },
        compatibility_notes: compat_notes(slug),
        continuity_notes: continuity_notes(slug),
    };

    let stale_channel = |channel: DeliveryChannel, feed: &str| ChannelDelivery {
        channel,
        delivery_state: ChannelDeliveryState::Stale,
        feed_ref: format!("feed/m5-{slug}/{feed}"),
        delivered_record_ids: vec![format!("record/m5-{slug}/01")],
        advisory_refs: vec![format!("advisory/m5-{slug}/disable")],
        delivered_at: Some("2026-02-10".to_owned()),
        summary: format!("{feed} channel carries stale recovery truth and must be refreshed."),
    };

    FamilyRecoveryLedger {
        entry_id: format!("ledger-{slug}"),
        title: "Managed output recovery ledger".to_owned(),
        family_kind: kind,
        artifact_graph_ref: format!("artifact_graph/m5-{slug}"),
        candidate_ref: format!("candidate/m5-{slug}"),
        family_summary: "Rollback/revocation truth for managed outputs from managed/tenant-scoped lanes."
            .to_owned(),
        release_blocking: true,
        claim_ref: format!("claim/m5-{slug}"),
        claim_label: StableClaimLevel::Beta,
        ledger_state: RecoveryLedgerState::RecoveryGap,
        affected_node_set: node_set(kind, slug),
        recovery_records: vec![expired_disable, broken_revoke],
        channel_parity: ChannelDeliveryParity {
            channels: vec![
                stale_channel(DeliveryChannel::Hosted, "hosted"),
                stale_channel(DeliveryChannel::Mirrored, "mirror"),
                stale_channel(DeliveryChannel::Offline, "offline"),
            ],
            summary: "All delivery channels carry equally stale recovery truth and must be refreshed."
                .to_owned(),
        },
        proof_packet: proof_packet(slug, FreshnessSloState::Breached),
        waiver: None,
        owner_signoff: signed("managed-release"),
        active_narrowing_reasons: vec![
            NarrowingReason::GraphConsistencyBroken,
            NarrowingReason::EmergencyDisableUnreconciled,
            NarrowingReason::ChannelDeliveryStale,
            NarrowingReason::EvidenceStale,
            NarrowingReason::ProofPacketStale,
        ],
        published_label: StableClaimLevel::Preview,
        rationale:
            "The managed-output family leaves the artifact graph broken after a revoke, holds an emergency-disable that expired without reconciliation, rides stale blocking evidence, carries equally stale recovery truth on every delivery channel, and breached its proof-packet SLO; it inherits its below-cutline beta claim and narrows to preview, naming every gap. The smallest affected node set is still preserved and no channel is excluded while another has the truth, so neither guardrail trips."
                .to_owned(),
        publication_destinations: vec![
            "release_center".to_owned(),
            "update_surface".to_owned(),
            "advisory_export".to_owned(),
            "support_export".to_owned(),
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

fn placeholder_summary() -> M5ArtifactGraphRecoverySummary {
    M5ArtifactGraphRecoverySummary {
        total_entries: 0,
        total_artifact_graphs: 0,
        entries_contained: 0,
        entries_narrowed: 0,
        entries_on_active_waiver: 0,
        entries_with_blast_radius_gap: 0,
        entries_with_graph_consistency_gap: 0,
        entries_with_channel_parity_gap: 0,
        entries_with_advisory_gap: 0,
        entries_with_emergency_gap: 0,
        release_blocking_total: 0,
        release_blocking_contained: 0,
        release_blocking_narrowed: 0,
        notebook_pack_entries: 0,
        request_data_asset_entries: 0,
        profiler_replay_entries: 0,
        framework_template_entries: 0,
        docs_pack_entries: 0,
        model_pack_entries: 0,
        companion_offboarding_entries: 0,
        managed_output_entries: 0,
        channels_current: 0,
        channels_pending: 0,
        channels_stale: 0,
        channels_undelivered: 0,
        records_rollback: 0,
        records_revoke: 0,
        records_yank: 0,
        records_repin: 0,
        records_emergency_disable: 0,
        packets_current: 0,
        packets_due_for_refresh: 0,
        packets_breached: 0,
        packets_missing: 0,
        total_recovery_records: 0,
        total_emergency_disable_records: 0,
        total_active_narrowing_reasons: 0,
        rules_firing: 0,
    }
}
