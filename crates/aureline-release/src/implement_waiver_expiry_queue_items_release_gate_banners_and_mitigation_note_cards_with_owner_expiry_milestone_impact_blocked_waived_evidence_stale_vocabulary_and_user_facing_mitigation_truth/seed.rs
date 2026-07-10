//! Canonical seed builders for the M5 waiver-expiry / release-gate / mitigation-note
//! controls packet.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The gated artifact generator and the inline tests both call them
//! so the in-code matrix, the artifact, the worked resolutions, and the fixtures never
//! drift.

use super::*;

/// Stable packet id for the canonical waiver/gate/mitigation controls packet.
pub const M5_WAIVER_GATE_CONTROLS_PACKET_ID: &str =
    "m5-waiver-gate-mitigation-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked waiver-expiry-item case from a full item state.
#[allow(clippy::too_many_arguments)]
fn witem(
    waiver_id: &str,
    held_failure: &str,
    waiver_state: M5WaiverExpiryState,
    affected_target: M5AffectedTargetKind,
    affected_target_repr: &str,
    mitigation_posture: M5MitigationPosture,
    owner_alias: &str,
    expiry: &str,
    evidence_freshness: M5EvidenceFreshness,
) -> M5WaiverExpiryItemCase {
    M5WaiverExpiryItemCase::resolved(M5WaiverExpiryItemResolutionInput {
        waiver_id_repr: waiver_id.to_owned(),
        held_failure_repr: held_failure.to_owned(),
        waiver_state,
        affected_target,
        affected_target_repr: affected_target_repr.to_owned(),
        mitigation_posture,
        owner_alias: owner_alias.to_owned(),
        expiry_repr: expiry.to_owned(),
        evidence_freshness,
    })
}

/// Builds a worked release-gate case from a full gate state.
#[allow(clippy::too_many_arguments)]
fn gate(
    gate_id: &str,
    blocker_count: u32,
    waived_count: u32,
    stale_evidence_count: u32,
    declared_decision: M5ReleaseGateDecision,
    mitigation_posture: M5MitigationPosture,
    user_facing_mitigation: &str,
    fallback_path: &str,
    evidence_freshness: M5EvidenceFreshness,
    owner_or_forum_resolved: bool,
) -> M5ReleaseGateCase {
    M5ReleaseGateCase::resolved(M5ReleaseGateResolutionInput {
        gate_id_repr: gate_id.to_owned(),
        blocker_count,
        waived_count,
        stale_evidence_count,
        declared_decision,
        mitigation_posture,
        user_facing_mitigation: user_facing_mitigation.to_owned(),
        fallback_path_repr: fallback_path.to_owned(),
        evidence_freshness,
        owner_or_forum_resolved,
    })
}

/// A base row with the shared fields filled in and the full anatomy, label, readiness,
/// waiver-state, affected-target, mitigation-posture, mitigation-clarity, evidence,
/// degrade, decision, action, next-action, export-field, and accessibility parity every
/// consumer carries.
fn base_row(
    consumer_surface: M5WaiverGateConsumerSurface,
    qualification: M5GovernanceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    waiver_expiry_examples: Vec<M5WaiverExpiryItemCase>,
    release_gate_examples: Vec<M5ReleaseGateCase>,
) -> M5WaiverGateRow {
    M5WaiverGateRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5GovernanceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5WaiverGateAnatomyPart::ALL.to_vec(),
        required_labels: M5GovernanceRequiredLabel::ALL.to_vec(),
        readiness_states: M5GovernanceReadinessState::ALL.to_vec(),
        waiver_expiry_states: M5WaiverExpiryState::ALL.to_vec(),
        affected_target_kinds: M5AffectedTargetKind::ALL.to_vec(),
        mitigation_postures: M5MitigationPosture::ALL.to_vec(),
        mitigation_clarities: M5MitigationClarity::ALL.to_vec(),
        evidence_freshness_states: M5EvidenceFreshness::ALL.to_vec(),
        waiver_degrade_reasons: M5WaiverDegradeReason::ALL.to_vec(),
        gate_decisions: M5ReleaseGateDecision::ALL.to_vec(),
        gate_degrade_reasons: M5GateDegradeReason::ALL.to_vec(),
        item_actions: M5WaiverItemAction::ALL.to_vec(),
        gate_actions: M5GateAction::ALL.to_vec(),
        next_actions: M5WaiverGateNextAction::ALL.to_vec(),
        export_fields: M5WaiverGateExportField::ALL.to_vec(),
        accessibility_routes: M5GovernanceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5GovernanceConsumerSurface::AssuranceDashboard,
            M5GovernanceConsumerSurface::OperatorBoard,
            M5GovernanceConsumerSurface::ShiproomPacket,
            M5GovernanceConsumerSurface::ServiceHealth,
            M5GovernanceConsumerSurface::SupportExport,
            M5GovernanceConsumerSurface::CliInspect,
            M5GovernanceConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5GovernanceDowngradeTrigger::WaiverExpiryHidden,
            M5GovernanceDowngradeTrigger::ReleaseGateReasonGeneric,
            M5GovernanceDowngradeTrigger::MitigationHiddenBehindJargon,
            M5GovernanceDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_WAIVER_GATE_CONTROLS_SCHEMA_REF,
            M5_WAIVER_EXPIRY_QUEUE_ITEM_CONTRACT_REF,
            M5_RELEASE_GATE_BANNER_CONTRACT_REF,
            M5_MITIGATION_NOTE_CARD_CONTRACT_REF,
        ]),
        waiver_expiry_examples,
        release_gate_examples,
        renders_waived_or_expired_as_clean_pass: false,
        hides_waiver_expiry_or_owner: false,
        hides_mitigation_behind_internal_jargon: false,
        invents_gate_local_status_grammar: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn controls_rows() -> Vec<M5WaiverGateRow> {
    use M5AffectedTargetKind as Target;
    use M5EvidenceFreshness as Fresh;
    use M5MitigationPosture as Mit;
    use M5ReleaseGateDecision as Decision;
    use M5WaiverExpiryState as Waiver;

    let mut rows = Vec::new();

    // 1. Assurance dashboard — an expiring waiver that stays waived and visible (the
    //    AC-1 example), an active waiver, a clean go with a plain-language mitigation
    //    (the AC-2 example), and a gate with open blockers that never stays go.
    rows.push(base_row(
        M5WaiverGateConsumerSurface::AssuranceDashboard,
        M5GovernanceQualificationClass::Stable,
        "Assurance-dashboard owner",
        "The assurance dashboard renders the shared waiver-expiry queue item so an expiring waiver reads as waived with its expiry still visible rather than passing, and the release-gate banner so a go declared over open blockers never stays go while its plain-language mitigation stays understandable",
        "evidence:m5-waiver-gate-assurance:001",
        vec![
            witem(
                "waiver:accessibility-audit-hold",
                "check:accessibility-audit",
                Waiver::ExpiringSoon,
                Target::MilestoneTarget,
                "milestone:m5-exit-gate",
                Mit::PartiallyMitigated,
                "role:accessibility-guild",
                "expiry:2026-07-20T00:00:00Z",
                Fresh::EvidenceFresh,
            ),
            witem(
                "waiver:cold-start-budget-hold",
                "check:cold-start-budget",
                Waiver::ActiveWaiver,
                Target::ReleaseTrainTarget,
                "train:m5-train-07",
                Mit::Mitigated,
                "role:performance-guild",
                "expiry:2026-08-15T00:00:00Z",
                Fresh::EvidenceFresh,
            ),
        ],
        vec![
            gate(
                "gate:m5-assurance-ship",
                0,
                0,
                0,
                Decision::Go,
                Mit::Mitigated,
                "The audited accessibility screens ship behind a feature flag while the remaining defects are fixed in the next patch.",
                "fallback:disable-feature-flag",
                Fresh::EvidenceFresh,
                true,
            ),
            gate(
                "gate:m5-assurance-blocked",
                2,
                0,
                0,
                Decision::Go,
                Mit::Unmitigated,
                "Two crash-on-launch blockers remain open and must be resolved before this lane can ship.",
                "fallback:hold-at-m4",
                Fresh::EvidenceFresh,
                true,
            ),
        ],
    ));

    // 2. Operator board — an expired waiver and a revoked waiver that both read as
    //    expired_waiver, a gate held by waived items, and a gate held on stale evidence.
    rows.push(base_row(
        M5WaiverGateConsumerSurface::OperatorBoard,
        M5GovernanceQualificationClass::Stable,
        "Operator-board owner",
        "The operator board renders the shared waiver-expiry queue item so an expired or revoked waiver reads as expired_waiver rather than covering its failure, and the release-gate banner so a gate held by waived items reads as conditional and a gate on stale evidence is held pending evidence",
        "evidence:m5-waiver-gate-operator:001",
        vec![
            witem(
                "waiver:memory-ceiling-lapsed",
                "check:memory-ceiling",
                Waiver::ExpiredWaiver,
                Target::ServiceTarget,
                "service:ingest-worker",
                Mit::PartiallyMitigated,
                "role:reliability-guild",
                "expiry:2026-07-01T00:00:00Z",
                Fresh::EvidenceFresh,
            ),
            witem(
                "waiver:error-budget-revoked",
                "check:error-budget-burn",
                Waiver::RevokedWaiver,
                Target::FleetTarget,
                "fleet:global",
                Mit::Unmitigated,
                "role:reliability-guild",
                "expiry:2026-07-05T00:00:00Z",
                Fresh::EvidenceFresh,
            ),
        ],
        vec![
            gate(
                "gate:m5-operator-conditional",
                0,
                1,
                0,
                Decision::ConditionalGo,
                Mit::Mitigated,
                "One blocker is held under a still-valid waiver and the affected path is guarded by a documented fallback.",
                "fallback:route-around-ingest",
                Fresh::EvidenceFresh,
                true,
            ),
            gate(
                "gate:m5-operator-stale",
                0,
                0,
                3,
                Decision::Go,
                Mit::Mitigated,
                "Three evidence artifacts are stale and must be refreshed before the ship decision is trusted.",
                "fallback:hold-for-rerun",
                Fresh::EvidenceStale,
                true,
            ),
        ],
    ));

    // 3. Shiproom packet — a retired exception with no active waiver that is a clean
    //    pass, an unwaived failure that blocks, a gate whose mitigation collapsed into
    //    internal jargon, and a gate held by an unresolved owner or forum.
    rows.push(base_row(
        M5WaiverGateConsumerSurface::ShiproomPacket,
        M5GovernanceQualificationClass::Stable,
        "Shiproom-packet owner",
        "The shiproom packet renders the shared waiver-expiry queue item so a fully-mitigated retired exception reads as passing while an unwaived failure blocks, and the release-gate banner so a mitigation that collapsed into internal jargon degrades and a gate with no authorized owner or forum reads as blocked",
        "evidence:m5-waiver-gate-shiproom:001",
        vec![
            witem(
                "waiver:startup-crash-cleared",
                "check:startup-crash-rate",
                Waiver::NoWaiver,
                Target::ServiceTarget,
                "service:mobile-shell",
                Mit::Mitigated,
                "role:quality-guild",
                "expiry:none",
                Fresh::EvidenceFresh,
            ),
            witem(
                "waiver:license-compliance-open",
                "check:license-compliance",
                Waiver::NoWaiver,
                Target::TargetUnrecorded,
                "target:unrecorded",
                Mit::Unmitigated,
                "role:supply-chain-guild",
                "expiry:none",
                Fresh::EvidenceFresh,
            ),
        ],
        vec![
            gate(
                "gate:m5-shiproom-jargon",
                0,
                0,
                0,
                Decision::ConditionalGo,
                Mit::PartiallyMitigated,
                "wontfix; see internal.",
                "fallback:escalate-to-shiproom",
                Fresh::EvidenceFresh,
                true,
            ),
            gate(
                "gate:m5-shiproom-forumless",
                0,
                0,
                0,
                Decision::Go,
                Mit::Mitigated,
                "The release forum has not yet claimed this lane and must be resolved before a ship decision.",
                "fallback:assign-decision-forum",
                Fresh::EvidenceFresh,
                false,
            ),
        ],
    ));

    // 4. CLI inspect — a queue item with stale evidence, a queue item blocked on missing
    //    evidence, a gate not yet evaluated, and a gate whose risk is accepted but whose
    //    plain-language mitigation stays understandable.
    rows.push(base_row(
        M5WaiverGateConsumerSurface::CliInspect,
        M5GovernanceQualificationClass::Stable,
        "CLI-inspect owner",
        "The CLI inspect surface renders the shared waiver-expiry queue item so a stale-evidence item reads as evidence_stale and a missing-evidence item blocks, and the release-gate banner so a not-yet-evaluated gate reads as not_evaluated and an accepted-risk gate keeps a plain-language mitigation — the same waiver/gate vocabulary a headless reviewer reads elsewhere",
        "evidence:m5-waiver-gate-cli:001",
        vec![
            witem(
                "waiver:build-reproducibility-hold",
                "check:build-reproducibility",
                Waiver::ActiveWaiver,
                Target::ReleaseTrainTarget,
                "train:m5-train-08",
                Mit::PartiallyMitigated,
                "role:supply-chain-guild",
                "expiry:2026-08-01T00:00:00Z",
                Fresh::EvidenceStale,
            ),
            witem(
                "waiver:index-freshness-hold",
                "check:index-freshness",
                Waiver::ActiveWaiver,
                Target::ServiceTarget,
                "service:search-index",
                Mit::PartiallyMitigated,
                "role:performance-guild",
                "expiry:2026-08-02T00:00:00Z",
                Fresh::EvidenceMissing,
            ),
        ],
        vec![
            gate(
                "gate:m5-cli-not-run",
                0,
                0,
                0,
                Decision::Go,
                Mit::Mitigated,
                "This gate has not been evaluated on the current build and must be run before a ship decision.",
                "fallback:run-gate-evaluation",
                Fresh::EvidenceUnknown,
                true,
            ),
            gate(
                "gate:m5-cli-risk-accepted",
                0,
                0,
                0,
                Decision::ConditionalGo,
                Mit::RiskAccepted,
                "The residual localization gap is accepted for this release and tracked for the following train.",
                "fallback:track-in-next-train",
                Fresh::EvidenceFresh,
                true,
            ),
        ],
    ));

    // 5. Support / export — a queue item with an unresolved owner, an expiring waiver
    //    that stays waived and visible, a gate whose mitigation note is absent, and a
    //    clean go.
    rows.push(base_row(
        M5WaiverGateConsumerSurface::SupportExport,
        M5GovernanceQualificationClass::Stable,
        "Support / export owner",
        "The support / export packet renders the shared waiver-expiry queue item so an item with no resolved owner reads as owner_unresolved and an expiring waiver stays waived and visible, and the release-gate banner so a gate with no user-facing mitigation degrades and a clean go still names its fallback — the same waiver/gate vocabulary a support or release reviewer reads elsewhere",
        "evidence:m5-waiver-gate-support:001",
        vec![
            witem(
                "waiver:query-throughput-hold",
                "check:query-throughput",
                Waiver::ActiveWaiver,
                Target::ServiceTarget,
                "service:query-router",
                Mit::PartiallyMitigated,
                "",
                "expiry:2026-08-10T00:00:00Z",
                Fresh::EvidenceFresh,
            ),
            witem(
                "waiver:crash-rate-expiring",
                "check:crash-rate",
                Waiver::ExpiringSoon,
                Target::MilestoneTarget,
                "milestone:m5-exit-gate",
                Mit::Mitigated,
                "role:quality-guild",
                "expiry:2026-07-18T00:00:00Z",
                Fresh::EvidenceFresh,
            ),
        ],
        vec![
            gate(
                "gate:m5-support-absent-note",
                0,
                0,
                0,
                Decision::ConditionalGo,
                Mit::Unmitigated,
                "",
                "fallback:request-mitigation-note",
                Fresh::EvidenceFresh,
                true,
            ),
            gate(
                "gate:m5-support-clean",
                0,
                0,
                0,
                Decision::Go,
                Mit::Mitigated,
                "All release blockers are resolved and the documented fallback path is verified for this lane.",
                "fallback:rollback-to-previous-train",
                Fresh::EvidenceFresh,
                true,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5WaiverGateReview {
    M5WaiverGateReview {
        one_packet_carries_waiver_gate_and_mitigation_truth: true,
        identity_and_gate_decision_always_shown: true,
        waived_or_expiring_never_reads_clean_pass: true,
        waiver_expiry_always_visible: true,
        ownerless_or_forumless_blocker_never_resolved: true,
        blocker_waived_stale_counts_always_shown: true,
        mitigation_stays_understandable: true,
        readiness_state_drawn_from_frozen_vocabulary: true,
        support_export_reconstructs_truth: true,
        no_surface_invents_second_grammar: true,
        every_row_declares_accessibility_route: true,
        owner_alias_is_role_not_person: true,
    }
}

fn consumer_projection() -> M5WaiverGateConsumerProjection {
    M5WaiverGateConsumerProjection {
        surfaces_consume_shared_packet: true,
        readiness_resolver_reads_single_source: true,
        mitigation_clarity_reads_single_source: true,
        waiver_expiry_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5WaiverGateProofFreshness {
    M5WaiverGateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5WaiverGateReleasePosture {
    M5WaiverGateReleasePosture {
        governance_packet_ref: M5_WAIVER_GATE_CONTROLS_ARTIFACT_REF.to_owned(),
        assurance_audit_ref: M5_WAIVER_GATE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_WAIVER_GATE_CONTROLS_SCHEMA_REF,
        M5_WAIVER_GATE_CONTROLS_DOC_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF,
        M5_WAIVER_EXPIRY_QUEUE_ITEM_CONTRACT_REF,
        M5_RELEASE_GATE_BANNER_CONTRACT_REF,
        M5_MITIGATION_NOTE_CARD_CONTRACT_REF,
    ])
}

/// Builds the canonical M5 waiver/gate/mitigation controls packet.
pub fn seeded_m5_waiver_gate_controls_packet() -> M5WaiverGateControlsPacket {
    M5WaiverGateControlsPacket::new(M5WaiverGateControlsPacketInput {
        packet_id: M5_WAIVER_GATE_CONTROLS_PACKET_ID.to_owned(),
        matrix_label:
            "M5 waiver-expiry queue item, release-gate banner, and mitigation note card controls: owner, expiry, milestone impact, blocked-versus-waived-versus-evidence-stale vocabulary, and user-facing mitigation truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5WaiverGateVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shiproom packet is held at Beta because a slice of
/// shiproom-packet cards do not yet render the fallback path on every export path; every
/// consumer stays visible.
pub fn seeded_m5_waiver_gate_controls_shiproom_packet_beta_narrowed() -> M5WaiverGateControlsPacket
{
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.packet_id = "m5-waiver-gate-mitigation-controls:shiproom-packet-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WaiverGateConsumerSurface::ShiproomPacket)
        .expect("shiproom-packet row present");
    row.qualification = M5GovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the operator board is narrowed to Preview pending
/// mitigation-clarity parity proof across every export path; every consumer stays
/// visible.
pub fn seeded_m5_waiver_gate_controls_operator_board_preview_narrowed() -> M5WaiverGateControlsPacket
{
    let mut packet = seeded_m5_waiver_gate_controls_packet();
    packet.packet_id = "m5-waiver-gate-mitigation-controls:operator-board-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WaiverGateConsumerSurface::OperatorBoard)
        .expect("operator-board row present");
    row.qualification = M5GovernanceQualificationClass::Preview;
    packet
}
