//! Canonical seed builders for the M5 service-ownership / on-call controls packet.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The gated artifact generator and the inline tests both call them so
//! the in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical service-ownership / on-call controls packet.
pub const M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_PACKET_ID: &str =
    "m5-service-ownership-on-call-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-10T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked service-ownership-card case from a full card state.
#[allow(clippy::too_many_arguments)]
fn oc(
    service_id: &str,
    surface_identity: &str,
    owning_role: &str,
    support_class: M5ServiceSupportClass,
    coverage_state: M5OwnershipCoverageState,
    owner_source: M5OwnerSource,
    backup_owner: &str,
    escalation_route: M5EscalationRouteClass,
    owner_freshness: M5OwnerFreshness,
) -> M5ServiceOwnershipCardCase {
    M5ServiceOwnershipCardCase::resolved(M5ServiceOwnershipResolutionInput {
        service_id_repr: service_id.to_owned(),
        surface_identity_repr: surface_identity.to_owned(),
        owning_role_alias: owning_role.to_owned(),
        support_class,
        coverage_state,
        owner_source,
        backup_owner_alias: backup_owner.to_owned(),
        escalation_route,
        owner_freshness,
    })
}

/// Builds a worked on-call-strip case from a full strip state.
#[allow(clippy::too_many_arguments)]
fn ocs(
    strip_id: &str,
    role_alias: &str,
    coverage_state: M5OnCallCoverageState,
    availability_state: M5OnCallAvailabilityState,
    role_tier: M5OnCallRoleTier,
    escalation_route: M5EscalationRouteClass,
    handoff: &str,
    roster_freshness: M5OwnerFreshness,
) -> M5OnCallStripCase {
    M5OnCallStripCase::resolved(M5OnCallStripResolutionInput {
        strip_id_repr: strip_id.to_owned(),
        role_alias: role_alias.to_owned(),
        coverage_state,
        availability_state,
        role_tier,
        escalation_route,
        handoff_repr: handoff.to_owned(),
        roster_freshness,
    })
}

/// A base row with the shared fields filled in and the full anatomy, label, readiness,
/// support-class, ownership-coverage, owner-source, freshness, degrade, on-call-coverage,
/// availability, role-tier, escalation, action, next-action, export-field, and
/// accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5OwnershipConsumerSurface,
    qualification: M5GovernanceQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    ownership_examples: Vec<M5ServiceOwnershipCardCase>,
    on_call_examples: Vec<M5OnCallStripCase>,
) -> M5OwnershipRow {
    M5OwnershipRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5GovernanceSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        anatomy_parts: M5OwnershipAnatomyPart::ALL.to_vec(),
        required_labels: M5GovernanceRequiredLabel::ALL.to_vec(),
        readiness_states: M5GovernanceReadinessState::ALL.to_vec(),
        support_classes: M5ServiceSupportClass::ALL.to_vec(),
        ownership_coverage_states: M5OwnershipCoverageState::ALL.to_vec(),
        owner_sources: M5OwnerSource::ALL.to_vec(),
        owner_freshness_states: M5OwnerFreshness::ALL.to_vec(),
        ownership_degrade_reasons: M5OwnershipDegradeReason::ALL.to_vec(),
        on_call_coverage_states: M5OnCallCoverageState::ALL.to_vec(),
        availability_states: M5OnCallAvailabilityState::ALL.to_vec(),
        role_tiers: M5OnCallRoleTier::ALL.to_vec(),
        escalation_route_classes: M5EscalationRouteClass::ALL.to_vec(),
        on_call_degrade_reasons: M5OnCallDegradeReason::ALL.to_vec(),
        card_actions: M5OwnershipCardAction::ALL.to_vec(),
        strip_actions: M5OnCallStripAction::ALL.to_vec(),
        next_actions: M5OwnershipNextAction::ALL.to_vec(),
        export_fields: M5OwnershipExportField::ALL.to_vec(),
        accessibility_routes: M5GovernanceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5GovernanceConsumerSurface::OperatorBoard,
            M5GovernanceConsumerSurface::ReleaseCenterUi,
            M5GovernanceConsumerSurface::ServiceHealth,
            M5GovernanceConsumerSurface::SupportExport,
            M5GovernanceConsumerSurface::CliInspect,
            M5GovernanceConsumerSurface::ShiproomPacket,
            M5GovernanceConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5GovernanceDowngradeTrigger::OwnerCoverageOverstated,
            M5GovernanceDowngradeTrigger::OnCallGapHidden,
            M5GovernanceDowngradeTrigger::EscalationRouteUnstated,
            M5GovernanceDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_REF,
            M5_SERVICE_OWNERSHIP_CARD_CONTRACT_REF,
            M5_ON_CALL_STRIP_CONTRACT_REF,
        ]),
        ownership_examples,
        on_call_examples,
        renders_unowned_or_backup_missing_as_covered: false,
        inherits_last_interacting_team_as_owner: false,
        hides_on_call_gap_or_escalation_route: false,
        invents_ownership_local_status_grammar: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn controls_rows() -> Vec<M5OwnershipRow> {
    use M5EscalationRouteClass as Route;
    use M5OnCallAvailabilityState as Avail;
    use M5OnCallCoverageState as OnCall;
    use M5OnCallRoleTier as Tier;
    use M5OwnerFreshness as Fresh;
    use M5OwnerSource as Source;
    use M5OwnershipCoverageState as Cover;
    use M5ServiceSupportClass as Support;

    let mut rows = Vec::new();

    // 1. Operator board — a backup-missing surface that reads as warning rather than
    //    covered (the AC-1 backup-missing example), a fully-owned clean service, an
    //    open on-call gap that never reads covered, and a clean on-call strip.
    rows.push(base_row(
        M5OwnershipConsumerSurface::OperatorBoard,
        M5GovernanceQualificationClass::Stable,
        "Operator-board owner",
        "The operator board renders the shared service-ownership card so a protected surface with a primary owner but no named backup reads as a warning with its coverage still visible rather than covered, and the on-call strip so an on-call gap never reads as covered while a fully-covered strip names its primary and escalation route",
        "evidence:m5-ownership-on-call-operator:001",
        vec![
            oc(
                "service:ingest-gateway",
                "surface:ingest-gateway-admin",
                "role:ingest-guild",
                Support::Tier1Critical,
                Cover::PrimaryOnlyNoBackup,
                Source::DeclaredOwnerRole,
                "",
                Route::EscalateToManager,
                Fresh::OwnerFresh,
            ),
            oc(
                "service:edge-router",
                "surface:edge-router-admin",
                "role:edge-guild",
                Support::Tier1Critical,
                Cover::OwnedWithBackup,
                Source::AuthoritativeRoster,
                "role:edge-backup-guild",
                Route::PagePrimary,
                Fresh::OwnerFresh,
            ),
        ],
        vec![
            ocs(
                "strip:ingest-gateway-oncall",
                "role:ingest-oncall",
                OnCall::OnCallGap,
                Avail::NoCoverage,
                Tier::SecondaryOnCall,
                Route::PageBackup,
                "handoff:ingest-runbook",
                Fresh::OwnerFresh,
            ),
            ocs(
                "strip:edge-router-oncall",
                "role:edge-oncall",
                OnCall::OnCallCovered,
                Avail::AvailableNow,
                Tier::PrimaryOnCall,
                Route::PagePrimary,
                "handoff:edge-runbook",
                Fresh::OwnerFresh,
            ),
        ],
    ));

    // 2. Release center — an owner inferred only from the last interacting team that reads
    //    as owner_unresolved (the AC-1 inherited-team example), a stale owner record, an
    //    on-call strip with no escalation path, and an escalation-only strip.
    rows.push(base_row(
        M5OwnershipConsumerSurface::ReleaseCenter,
        M5GovernanceQualificationClass::Stable,
        "Release-center owner",
        "The release center reuses the same role-based ownership card so an owner that is only inferred from the last interacting team reads as owner_unresolved rather than inheriting that team as false truth, and the on-call strip so a strip with no escalation path blocks and an escalation-only strip degrades — the same model support and operator surfaces use",
        "evidence:m5-ownership-on-call-release:001",
        vec![
            oc(
                "service:release-orchestrator",
                "surface:release-orchestrator-admin",
                "role:last-touch-team",
                Support::Tier2Standard,
                Cover::OwnerUnresolved,
                Source::LastInteractingTeamInference,
                "",
                Route::EscalateToManager,
                Fresh::OwnerFresh,
            ),
            oc(
                "service:artifact-store",
                "surface:artifact-store-admin",
                "role:supply-chain-guild",
                Support::Tier2Standard,
                Cover::OwnerStale,
                Source::AuthoritativeRoster,
                "role:supply-chain-backup",
                Route::PageBackup,
                Fresh::OwnerStale,
            ),
        ],
        vec![
            ocs(
                "strip:release-orchestrator-oncall",
                "role:release-oncall",
                OnCall::EscalationOnly,
                Avail::AvailableNow,
                Tier::ManagerEscalation,
                Route::NoEscalationPath,
                "handoff:release-runbook",
                Fresh::OwnerFresh,
            ),
            ocs(
                "strip:artifact-store-oncall",
                "role:artifact-oncall",
                OnCall::EscalationOnly,
                Avail::OffShift,
                Tier::ManagerEscalation,
                Route::EscalateToManager,
                "handoff:artifact-runbook",
                Fresh::OwnerFresh,
            ),
        ],
    ));

    // 3. Service health — a missing owner record that blocks, a policy-hidden ownership
    //    that warns, an on-call strip with no named responder, and a stale roster.
    rows.push(base_row(
        M5OwnershipConsumerSurface::ServiceHealth,
        M5GovernanceQualificationClass::Stable,
        "Service-health owner",
        "The service-health surface renders the shared service-ownership card so a missing owner record blocks and policy-hidden ownership warns rather than reading covered, and the on-call strip so a strip with no named responder reads owner_unresolved and a stale roster reads evidence_stale",
        "evidence:m5-ownership-on-call-service-health:001",
        vec![
            oc(
                "service:metrics-pipeline",
                "surface:metrics-pipeline-admin",
                "role:observability-guild",
                Support::Tier2Standard,
                Cover::OwnedWithBackup,
                Source::AuthoritativeRoster,
                "role:observability-backup",
                Route::PagePrimary,
                Fresh::OwnerMissing,
            ),
            oc(
                "service:audit-log",
                "surface:audit-log-admin",
                "role:security-guild",
                Support::Tier1Critical,
                Cover::PolicyHidden,
                Source::AuthoritativeRoster,
                "role:security-backup",
                Route::IncidentBridge,
                Fresh::OwnerFresh,
            ),
        ],
        vec![
            ocs(
                "strip:metrics-pipeline-oncall",
                "role:metrics-oncall",
                OnCall::EscalationOnly,
                Avail::AvailableNow,
                Tier::NoNamedResponder,
                Route::EscalateToManager,
                "handoff:metrics-runbook",
                Fresh::OwnerFresh,
            ),
            ocs(
                "strip:audit-log-oncall",
                "role:audit-oncall",
                OnCall::OnCallCovered,
                Avail::AvailableNow,
                Tier::PrimaryOnCall,
                Route::PagePrimary,
                "handoff:audit-runbook",
                Fresh::OwnerStale,
            ),
        ],
    ));

    // 4. Support / export — a not-yet-evaluated owner record, an aging owner record, an
    //    on-call strip with an unknown posture, and a strip with a pending handoff.
    rows.push(base_row(
        M5OwnershipConsumerSurface::SupportExport,
        M5GovernanceQualificationClass::Stable,
        "Support / export owner",
        "The support / export packet reuses the same role-based ownership card so a not-yet-evaluated owner record reads not_evaluated and an aging record warns, and the on-call strip so an unknown posture reads not_evaluated and a pending handoff warns — the same model operator and release surfaces read, reconstructable from the export",
        "evidence:m5-ownership-on-call-support:001",
        vec![
            oc(
                "service:query-router",
                "surface:query-router-admin",
                "role:query-guild",
                Support::Tier3BestEffort,
                Cover::OwnedWithBackup,
                Source::AuthoritativeRoster,
                "role:query-backup",
                Route::PagePrimary,
                Fresh::OwnerUnknown,
            ),
            oc(
                "service:search-index",
                "surface:search-index-admin",
                "role:search-guild",
                Support::Tier3BestEffort,
                Cover::OwnedWithBackup,
                Source::AuthoritativeRoster,
                "role:search-backup",
                Route::PageBackup,
                Fresh::OwnerAging,
            ),
        ],
        vec![
            ocs(
                "strip:query-router-oncall",
                "role:query-oncall",
                OnCall::OnCallCovered,
                Avail::AvailabilityUnknown,
                Tier::PrimaryOnCall,
                Route::PagePrimary,
                "handoff:query-runbook",
                Fresh::OwnerFresh,
            ),
            ocs(
                "strip:search-index-oncall",
                "role:search-oncall",
                OnCall::FollowTheSun,
                Avail::HandoffPending,
                Tier::PrimaryOnCall,
                Route::IncidentBridge,
                "handoff:search-runbook",
                Fresh::OwnerFresh,
            ),
        ],
    ));

    // 5. CLI inspect — a fully-owned clean service, a backup-missing surface, a clean
    //    follow-the-sun on-call strip, and a not-yet-evaluated strip — the same
    //    ownership/escalation vocabulary a headless reviewer reads elsewhere.
    rows.push(base_row(
        M5OwnershipConsumerSurface::CliInspect,
        M5GovernanceQualificationClass::Stable,
        "CLI-inspect owner",
        "The CLI inspect surface renders the shared service-ownership card so a fully-owned service reads passing and a backup-missing surface warns, and the on-call strip so a follow-the-sun covered strip reads passing and a not-yet-evaluated strip reads not_evaluated — the same ownership/escalation vocabulary a headless reviewer reads elsewhere",
        "evidence:m5-ownership-on-call-cli:001",
        vec![
            oc(
                "service:config-service",
                "surface:config-service-admin",
                "role:platform-guild",
                Support::Tier2Standard,
                Cover::OwnedWithBackup,
                Source::AuthoritativeRoster,
                "role:platform-backup",
                Route::PagePrimary,
                Fresh::OwnerFresh,
            ),
            oc(
                "service:notification-hub",
                "surface:notification-hub-admin",
                "role:messaging-guild",
                Support::CommunitySupported,
                Cover::PrimaryOnlyNoBackup,
                Source::DeclaredOwnerRole,
                "",
                Route::EscalateToManager,
                Fresh::OwnerFresh,
            ),
        ],
        vec![
            ocs(
                "strip:config-service-oncall",
                "role:config-oncall",
                OnCall::FollowTheSun,
                Avail::AvailableNow,
                Tier::PrimaryOnCall,
                Route::PagePrimary,
                "handoff:config-runbook",
                Fresh::OwnerFresh,
            ),
            ocs(
                "strip:notification-hub-oncall",
                "role:notification-oncall",
                OnCall::OnCallUnknown,
                Avail::AvailableNow,
                Tier::PrimaryOnCall,
                Route::PagePrimary,
                "handoff:notification-runbook",
                Fresh::OwnerUnknown,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5OwnershipReview {
    M5OwnershipReview {
        one_packet_carries_ownership_and_on_call_truth: true,
        service_identity_and_owning_role_always_shown: true,
        unowned_or_backup_missing_never_reads_covered: true,
        owner_never_inherited_from_last_interacting_team: true,
        support_class_and_freshness_always_shown: true,
        on_call_gap_never_reads_covered: true,
        escalation_route_always_explicit: true,
        readiness_state_drawn_from_frozen_vocabulary: true,
        support_operator_release_reuse_one_model: true,
        support_export_reconstructs_truth: true,
        every_row_declares_accessibility_route: true,
        owner_alias_is_role_not_person: true,
    }
}

fn consumer_projection() -> M5OwnershipConsumerProjection {
    M5OwnershipConsumerProjection {
        surfaces_consume_shared_packet: true,
        ownership_resolver_reads_single_source: true,
        on_call_resolver_reads_single_source: true,
        escalation_route_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5OwnershipProofFreshness {
    M5OwnershipProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5OwnershipReleasePosture {
    M5OwnershipReleasePosture {
        governance_packet_ref: M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_ARTIFACT_REF.to_owned(),
        assurance_audit_ref: M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_REF,
        M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_DOC_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_SCHEMA_REF,
        M5_GOVERNANCE_DASHBOARD_MATRIX_DOC_REF,
        M5_SERVICE_OWNERSHIP_CARD_CONTRACT_REF,
        M5_ON_CALL_STRIP_CONTRACT_REF,
    ])
}

/// Builds the canonical M5 service-ownership / on-call controls packet.
pub fn seeded_m5_service_ownership_on_call_controls_packet(
) -> M5ServiceOwnershipOnCallControlsPacket {
    M5ServiceOwnershipOnCallControlsPacket::new(M5ServiceOwnershipOnCallControlsPacketInput {
        packet_id: M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_PACKET_ID.to_owned(),
        matrix_label:
            "M5 service-ownership card and on-call strip controls: role-based owner/escalation aliases, support class, owner freshness, backup coverage, and export-safe handoff continuity across claimed M5 operator and release surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5OwnershipVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the service-health surface is held at Beta because a slice of
/// service-health cards do not yet render the owner-freshness cue on every export path;
/// every consumer stays visible.
pub fn seeded_m5_service_ownership_on_call_controls_service_health_beta_narrowed(
) -> M5ServiceOwnershipOnCallControlsPacket {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.packet_id = "m5-service-ownership-on-call-controls:service-health-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5OwnershipConsumerSurface::ServiceHealth)
        .expect("service-health row present");
    row.qualification = M5GovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the operator board is narrowed to Preview pending on-call-gap parity
/// proof across every export path; every consumer stays visible.
pub fn seeded_m5_service_ownership_on_call_controls_operator_board_preview_narrowed(
) -> M5ServiceOwnershipOnCallControlsPacket {
    let mut packet = seeded_m5_service_ownership_on_call_controls_packet();
    packet.packet_id =
        "m5-service-ownership-on-call-controls:operator-board-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5OwnershipConsumerSurface::OperatorBoard)
        .expect("operator-board row present");
    row.qualification = M5GovernanceQualificationClass::Preview;
    packet
}
