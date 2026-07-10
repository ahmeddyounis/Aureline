//! Canonical seed builders for the M5 decision-right / milestone controls packet.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The gated artifact generator and the inline tests both call them so
//! the in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical decision-right / milestone controls packet.
pub const M5_DECISION_RIGHT_MILESTONE_CONTROLS_PACKET_ID: &str =
    "m5-decision-right-milestone-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked decision-right-card case from a full card state.
#[allow(clippy::too_many_arguments)]
fn dc(
    card_id: &str,
    required_forum: M5DecisionForumClass,
    decision_state: M5DecisionRightState,
    reason: &str,
    target_milestone: &str,
    satisfaction_state: M5ReviewSatisfactionState,
    governance_review_required: bool,
    evidence_freshness: M5EvidenceFreshness,
) -> M5DecisionRightCardCase {
    M5DecisionRightCardCase::resolved(M5DecisionRightResolutionInput {
        card_id_repr: card_id.to_owned(),
        required_forum,
        decision_state,
        reason_for_review_repr: reason.to_owned(),
        target_milestone_repr: target_milestone.to_owned(),
        satisfaction_state,
        governance_review_required,
        evidence_freshness,
    })
}

/// Builds a worked milestone-dashboard-row case from a full row state.
#[allow(clippy::too_many_arguments)]
fn mr(
    milestone_id: &str,
    milestone_name: &str,
    owning_team: &str,
    owner_coverage: M5OwnershipCoverageState,
    blocker_count: u32,
    waiver_count: u32,
    gate_state: M5MilestoneGateState,
    nearest_review_forum: M5DecisionForumClass,
    next_review: &str,
    evidence_freshness: M5EvidenceFreshness,
) -> M5MilestoneRowCase {
    M5MilestoneRowCase::resolved(M5MilestoneRowResolutionInput {
        milestone_id_repr: milestone_id.to_owned(),
        milestone_name_repr: milestone_name.to_owned(),
        owning_team_alias: owning_team.to_owned(),
        owner_coverage,
        blocker_count,
        waiver_count,
        gate_state,
        nearest_review_forum,
        next_review_repr: next_review.to_owned(),
        evidence_freshness,
    })
}

/// A base row with the shared fields filled in and the full anatomy, label, readiness,
/// decision-forum, decision-right, satisfaction, degrade, milestone-gate, owner-coverage,
/// evidence-freshness, action, next-action, export-field, and accessibility parity every
/// consumer carries.
fn base_row(
    consumer_surface: M5DecisionMilestoneConsumerSurface,
    qualification: M5GovernanceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    decision_examples: Vec<M5DecisionRightCardCase>,
    milestone_examples: Vec<M5MilestoneRowCase>,
) -> M5DecisionMilestoneRow {
    M5DecisionMilestoneRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5GovernanceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5DecisionMilestoneAnatomyPart::ALL.to_vec(),
        required_labels: M5GovernanceRequiredLabel::ALL.to_vec(),
        readiness_states: M5GovernanceReadinessState::ALL.to_vec(),
        decision_forum_classes: M5DecisionForumClass::ALL.to_vec(),
        decision_right_states: M5DecisionRightState::ALL.to_vec(),
        satisfaction_states: M5ReviewSatisfactionState::ALL.to_vec(),
        decision_degrade_reasons: M5DecisionRightDegradeReason::ALL.to_vec(),
        milestone_gate_states: M5MilestoneGateState::ALL.to_vec(),
        owner_coverage_states: M5OwnershipCoverageState::ALL.to_vec(),
        milestone_degrade_reasons: M5MilestoneDegradeReason::ALL.to_vec(),
        evidence_freshness_states: M5EvidenceFreshness::ALL.to_vec(),
        card_actions: M5DecisionCardAction::ALL.to_vec(),
        row_actions: M5MilestoneRowAction::ALL.to_vec(),
        next_actions: M5DecisionMilestoneNextAction::ALL.to_vec(),
        export_fields: M5DecisionMilestoneExportField::ALL.to_vec(),
        accessibility_routes: M5GovernanceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5GovernanceConsumerSurface::ShiproomPacket,
            M5GovernanceConsumerSurface::OperatorBoard,
            M5GovernanceConsumerSurface::ReleaseCenterUi,
            M5GovernanceConsumerSurface::SupportExport,
            M5GovernanceConsumerSurface::CliInspect,
            M5GovernanceConsumerSurface::AssuranceDashboard,
            M5GovernanceConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5GovernanceDowngradeTrigger::DecisionForumMasked,
            M5GovernanceDowngradeTrigger::AdvisoryForumReadsAuthoritative,
            M5GovernanceDowngradeTrigger::MilestoneGateOverstated,
            M5GovernanceDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_REF,
            M5_DECISION_RIGHT_CARD_CONTRACT_REF,
            M5_MILESTONE_DASHBOARD_ROW_CONTRACT_REF,
        ]),
        decision_examples,
        milestone_examples,
        lets_ready_hide_a_blocking_forum_or_gate: false,
        lets_advisory_forum_read_authoritative: false,
        drifts_milestone_readiness_from_ownership_and_counts: false,
        invents_decision_local_status_grammar: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn controls_rows() -> Vec<M5DecisionMilestoneRow> {
    use M5DecisionForumClass as Forum;
    use M5DecisionRightState as Right;
    use M5EvidenceFreshness as Fresh;
    use M5MilestoneGateState as Gate;
    use M5OwnershipCoverageState as Cover;
    use M5ReviewSatisfactionState as Sat;

    let mut rows = Vec::new();

    // 1. Shiproom board — a required release-council review that is still pending and
    //    therefore never reads ready while the named council can still block it (the AC-1
    //    pending example), a clean satisfied decision, a milestone with open blockers that
    //    never reads a met gate (the AC-2 blocker example), and a clean met milestone.
    rows.push(base_row(
        M5DecisionMilestoneConsumerSurface::ShiproomBoard,
        M5GovernanceQualificationClass::Stable,
        "Shiproom-board owner",
        "The shiproom board renders the shared decision-right card so a milestone gated on a still-pending release-council review never reads ready while the council can still block it, its required forum and reason stay visible, and the milestone row so an exit gate with open blockers never reads met while its owning team and blocker/waiver counts stay visible",
        "evidence:m5-decision-milestone-shiproom:001",
        vec![
            dc(
                "card:shiproom-ga-cut",
                Forum::ReleaseCouncil,
                Right::AuthoritativeForum,
                "await release-council go/no-go sign-off before the GA cut",
                "milestone:m5-ga",
                Sat::ReviewPending,
                true,
                Fresh::EvidenceFresh,
            ),
            dc(
                "card:shiproom-docs-cut",
                Forum::ServiceOwner,
                Right::AuthoritativeForum,
                "service-owner confirmation that docs are complete",
                "milestone:m5-docs",
                Sat::ReviewSatisfied,
                true,
                Fresh::EvidenceFresh,
            ),
        ],
        vec![
            mr(
                "milestone:m5-ga",
                "M5 GA",
                "role:release-guild",
                Cover::OwnedWithBackup,
                2,
                0,
                Gate::ExitGateBlocked,
                Forum::ReleaseCouncil,
                "next-review:shiproom-2026-07-17",
                Fresh::EvidenceFresh,
            ),
            mr(
                "milestone:m5-docs",
                "M5 Docs",
                "role:docs-guild",
                Cover::OwnedWithBackup,
                0,
                0,
                Gate::ExitGateMet,
                Forum::ServiceOwner,
                "next-review:shiproom-2026-07-24",
                Fresh::EvidenceFresh,
            ),
        ],
    ));

    // 2. Operator board — an advisory-only architecture forum that reads warning rather
    //    than authoritative (the AC-1 advisory example), a required review with no
    //    authorized forum that reads forum_unresolved, a milestone held under a waiver that
    //    never reads met (the AC-2 waiver example), and a pending exit gate.
    rows.push(base_row(
        M5DecisionMilestoneConsumerSurface::OperatorBoard,
        M5GovernanceQualificationClass::Stable,
        "Operator-board owner",
        "The operator board reuses the same decision-right card so an advisory-only forum reads warning rather than authoritative and a required review with no authorized forum reads forum_unresolved, and the milestone row so a gate held under a waiver never reads met while its waiver count stays visible — the same model shiproom and support surfaces use",
        "evidence:m5-decision-milestone-operator:001",
        vec![
            dc(
                "card:operator-arch-advisory",
                Forum::ArchitectureForum,
                Right::AdvisoryOnly,
                "architecture forum advisory input on the rollout topology",
                "milestone:m5-waived",
                Sat::ReviewPending,
                true,
                Fresh::EvidenceFresh,
            ),
            dc(
                "card:operator-no-forum",
                Forum::NoAuthorizedForum,
                Right::ForumUnresolved,
                "no authorized forum is resolved for this escalation",
                "milestone:m5-pending",
                Sat::ReviewPending,
                true,
                Fresh::EvidenceFresh,
            ),
        ],
        vec![
            mr(
                "milestone:m5-waived",
                "M5 Waived Gate",
                "role:release-guild",
                Cover::OwnedWithBackup,
                0,
                1,
                Gate::ExitGateWaived,
                Forum::ReleaseCouncil,
                "next-review:operator-2026-07-15",
                Fresh::EvidenceFresh,
            ),
            mr(
                "milestone:m5-pending",
                "M5 Pending Gate",
                "role:platform-guild",
                Cover::OwnedWithBackup,
                0,
                0,
                Gate::ExitGatePending,
                Forum::ServiceOwner,
                "next-review:operator-2026-07-16",
                Fresh::EvidenceFresh,
            ),
        ],
    ));

    // 3. Release center — a decision delegated to another forum that reads warning, a
    //    stale-evidence decision, a milestone with an unresolved owner that reads
    //    owner_unresolved rather than drifting into a summary pass (the AC-2 ownership
    //    example), and a stale-gate milestone.
    rows.push(base_row(
        M5DecisionMilestoneConsumerSurface::ReleaseCenter,
        M5GovernanceQualificationClass::Stable,
        "Release-center owner",
        "The release center renders the shared decision-right card so a decision delegated elsewhere reads warning and a stale-evidence decision reads evidence_stale, and the milestone row so a milestone with no resolved owning team reads owner_unresolved rather than drifting into a summary-only pass and a stale exit gate reads evidence_stale",
        "evidence:m5-decision-milestone-release:001",
        vec![
            dc(
                "card:release-delegated",
                Forum::ServiceOwner,
                Right::DelegatedDecision,
                "decision delegated to the standing service owner",
                "milestone:m5-orphan",
                Sat::ReviewPending,
                true,
                Fresh::EvidenceFresh,
            ),
            dc(
                "card:release-stale-evidence",
                Forum::ReleaseCouncil,
                Right::AuthoritativeForum,
                "release-council review with evidence pending refresh",
                "milestone:m5-stale",
                Sat::ReviewSatisfied,
                true,
                Fresh::EvidenceStale,
            ),
        ],
        vec![
            mr(
                "milestone:m5-orphan",
                "M5 Orphan Lane",
                "role:unassigned-lane",
                Cover::OwnerUnresolved,
                0,
                0,
                Gate::ExitGatePending,
                Forum::ReleaseCouncil,
                "next-review:release-2026-07-18",
                Fresh::EvidenceFresh,
            ),
            mr(
                "milestone:m5-stale",
                "M5 Stale Gate",
                "role:release-guild",
                Cover::OwnedWithBackup,
                0,
                0,
                Gate::ExitGateStale,
                Forum::ReleaseCouncil,
                "next-review:release-2026-07-19",
                Fresh::EvidenceStale,
            ),
        ],
    ));

    // 4. Support / export — a waived decision review that reads waived, a not-yet-evaluated
    //    decision, a milestone with missing gate evidence that reads evidence_stale, and a
    //    milestone with no nearest review forum that reads forum_unresolved —
    //    reconstructable from the export, the same model shiproom and operator surfaces
    //    read.
    rows.push(base_row(
        M5DecisionMilestoneConsumerSurface::SupportExport,
        M5GovernanceQualificationClass::Stable,
        "Support / export owner",
        "The support / export packet reuses the same decision-right card so a waived review reads waived and a not-yet-evaluated decision reads not_evaluated, and the milestone row so missing gate evidence reads evidence_stale and a milestone with no nearest review forum reads forum_unresolved — the same model operator and shiproom surfaces read, reconstructable from the export",
        "evidence:m5-decision-milestone-support:001",
        vec![
            dc(
                "card:support-waived-review",
                Forum::SecurityReviewBoard,
                Right::AuthoritativeForum,
                "security review board sign-off held under a disclosed waiver",
                "milestone:m5-missing",
                Sat::ReviewWaived,
                true,
                Fresh::EvidenceFresh,
            ),
            dc(
                "card:support-not-evaluated",
                Forum::ReleaseCouncil,
                Right::NotEvaluatedHere,
                "decision right not evaluated on this build",
                "milestone:m5-no-forum",
                Sat::ReviewNotRequired,
                false,
                Fresh::EvidenceUnknown,
            ),
        ],
        vec![
            mr(
                "milestone:m5-missing",
                "M5 Missing Evidence",
                "role:security-guild",
                Cover::OwnedWithBackup,
                0,
                0,
                Gate::ExitGateMet,
                Forum::ServiceOwner,
                "next-review:support-2026-07-20",
                Fresh::EvidenceMissing,
            ),
            mr(
                "milestone:m5-no-forum",
                "M5 No Review Forum",
                "role:platform-guild",
                Cover::OwnedWithBackup,
                0,
                0,
                Gate::ExitGatePending,
                Forum::NoAuthorizedForum,
                "next-review:support-2026-07-21",
                Fresh::EvidenceFresh,
            ),
        ],
    ));

    // 5. CLI inspect — a clean decision whose review is not required, a decision with
    //    missing evidence that blocks, a clean met milestone, and an aging-evidence
    //    milestone — the same decision-right/milestone vocabulary a headless reviewer reads
    //    elsewhere.
    rows.push(base_row(
        M5DecisionMilestoneConsumerSurface::CliInspect,
        M5GovernanceQualificationClass::Stable,
        "CLI-inspect owner",
        "The CLI inspect surface renders the shared decision-right card so a decision with no required review reads passing and a decision with missing evidence blocks, and the milestone row so a met gate with zero blockers and zero waivers reads passing and an aging-evidence milestone reads warning — the same decision-right/milestone vocabulary a headless reviewer reads elsewhere",
        "evidence:m5-decision-milestone-cli:001",
        vec![
            dc(
                "card:cli-clean",
                Forum::ServiceOwner,
                Right::AuthoritativeForum,
                "service-owner decision with no required governance review",
                "milestone:m5-cli-clean",
                Sat::ReviewNotRequired,
                false,
                Fresh::EvidenceFresh,
            ),
            dc(
                "card:cli-missing-evidence",
                Forum::ReleaseCouncil,
                Right::AuthoritativeForum,
                "release-council review with the decision evidence missing",
                "milestone:m5-cli-aging",
                Sat::ReviewSatisfied,
                true,
                Fresh::EvidenceMissing,
            ),
        ],
        vec![
            mr(
                "milestone:m5-cli-clean",
                "M5 CLI Clean",
                "role:platform-guild",
                Cover::OwnedWithBackup,
                0,
                0,
                Gate::ExitGateMet,
                Forum::ServiceOwner,
                "next-review:cli-2026-07-22",
                Fresh::EvidenceFresh,
            ),
            mr(
                "milestone:m5-cli-aging",
                "M5 CLI Aging",
                "role:platform-guild",
                Cover::PrimaryOnlyNoBackup,
                0,
                0,
                Gate::ExitGateMet,
                Forum::ServiceOwner,
                "next-review:cli-2026-07-23",
                Fresh::EvidenceAging,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5DecisionMilestoneReview {
    M5DecisionMilestoneReview {
        one_packet_carries_decision_and_milestone_truth: true,
        required_forum_and_reason_always_shown: true,
        ready_never_hides_a_blocking_forum_or_gate: true,
        advisory_forum_never_reads_authoritative: true,
        satisfaction_state_and_target_always_shown: true,
        milestone_readiness_paired_with_ownership: true,
        blocker_and_waiver_counts_always_shown: true,
        readiness_state_drawn_from_frozen_vocabulary: true,
        shiproom_operator_support_reuse_one_model: true,
        support_export_reconstructs_truth: true,
        every_row_declares_accessibility_route: true,
        owner_alias_is_role_not_person: true,
    }
}

fn consumer_projection() -> M5DecisionMilestoneConsumerProjection {
    M5DecisionMilestoneConsumerProjection {
        surfaces_consume_shared_packet: true,
        decision_resolver_reads_single_source: true,
        milestone_resolver_reads_single_source: true,
        nearest_forum_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5DecisionMilestoneProofFreshness {
    M5DecisionMilestoneProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DecisionMilestoneReleasePosture {
    M5DecisionMilestoneReleasePosture {
        governance_packet_ref: M5_DECISION_RIGHT_MILESTONE_CONTROLS_ARTIFACT_REF.to_owned(),
        assurance_audit_ref: M5_DECISION_RIGHT_MILESTONE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_REF,
        M5_DECISION_RIGHT_MILESTONE_CONTROLS_DOC_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF,
        M5_DECISION_RIGHT_CARD_CONTRACT_REF,
        M5_MILESTONE_DASHBOARD_ROW_CONTRACT_REF,
    ])
}

/// Builds the canonical M5 decision-right / milestone controls packet.
pub fn seeded_m5_decision_right_milestone_controls_packet() -> M5DecisionRightMilestoneControlsPacket
{
    M5DecisionRightMilestoneControlsPacket::new(M5DecisionRightMilestoneControlsPacketInput {
        packet_id: M5_DECISION_RIGHT_MILESTONE_CONTROLS_PACKET_ID.to_owned(),
        matrix_label:
            "M5 decision-right card and milestone dashboard row controls: required forum/reason, satisfied/pending state, target milestone, owning team, blocker/waiver counts, gate state, nearest review forum, and next-review continuity across claimed M5 shiproom and operator surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5DecisionMilestoneVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shiproom board is held at Beta because a slice of shiproom cards
/// do not yet render the reason-for-review cue on every export path; every consumer stays
/// visible.
pub fn seeded_m5_decision_right_milestone_controls_shiproom_board_beta_narrowed(
) -> M5DecisionRightMilestoneControlsPacket {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.packet_id = "m5-decision-right-milestone-controls:shiproom-board-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionMilestoneConsumerSurface::ShiproomBoard)
        .expect("shiproom-board row present");
    row.qualification = M5GovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the operator board is narrowed to Preview pending blocker/waiver-count
/// parity proof across every export path; every consumer stays visible.
pub fn seeded_m5_decision_right_milestone_controls_operator_board_preview_narrowed(
) -> M5DecisionRightMilestoneControlsPacket {
    let mut packet = seeded_m5_decision_right_milestone_controls_packet();
    packet.packet_id =
        "m5-decision-right-milestone-controls:operator-board-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5DecisionMilestoneConsumerSurface::OperatorBoard)
        .expect("operator-board row present");
    row.qualification = M5GovernanceQualificationClass::Preview;
    packet
}
