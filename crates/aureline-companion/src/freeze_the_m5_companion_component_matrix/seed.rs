//! Canonical seed builders for the frozen M5 companion component matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical companion component matrix.
pub const M5_COMPANION_COMPONENT_MATRIX_PACKET_ID: &str = "m5-companion-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5CompanionRequiredLabel> {
    M5CompanionRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5CompanionRequiredLabel]) -> Vec<M5CompanionRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every family-specific
/// vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5CompanionComponentFamily,
    qualification: M5CompanionQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5CompanionComponentRow {
    M5CompanionComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5CompanionSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CompanionDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        object_kinds: vec![],
        client_scopes: M5CompanionClientScope::ALL.to_vec(),
        freshness_classes: M5CompanionFreshness::ALL.to_vec(),
        dispositions: M5CompanionComponentDisposition::ALL.to_vec(),
        severities: vec![],
        review_kinds: vec![],
        ci_statuses: vec![],
        session_follow_states: vec![],
        handoff_targets: vec![],
        notification_categories: vec![],
        degraded_reasons: M5CompanionDegradedReason::ALL.to_vec(),
        accessibility_routes: M5CompanionAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5CompanionConsumerSurface::SupportExport,
            M5CompanionConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5CompanionDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_scope_or_freshness: false,
        hides_capability_boundary: false,
        invents_alternate_state_label: false,
        implies_desktop_action_is_companion_safe: false,
    }
}

fn component_rows() -> Vec<M5CompanionComponentRow> {
    use M5CompanionCiStatus as CI;
    use M5CompanionComponentDisposition as DP;
    use M5CompanionComponentFamily as F;
    use M5CompanionConsumerSurface as C;
    use M5CompanionDowngradeTrigger as D;
    use M5CompanionHandoffTarget as HT;
    use M5CompanionNotificationCategory as NC;
    use M5CompanionObjectKind as OK;
    use M5CompanionQualificationClass as Q;
    use M5CompanionRequiredLabel as L;
    use M5CompanionReviewKind as RK;
    use M5CompanionSessionFollowState as SF;
    use M5CompanionSeverity as SV;

    let mut rows = Vec::new();

    // 1. Notification row.
    let mut row = base_row(
        F::NotificationRow,
        Q::Stable,
        "Companion notification-row owner",
        "One notification-row model naming exactly which object a tap opens (a notification event bound to a build, review, agent, incident, sync, or mention), its severity, its client scope, and its freshness, so a user never has to infer what a tap opens or how urgent it is before acting from a browser or mobile companion",
        "evidence:m5-companion-notification-row-parity:001",
        &[
            M5_COMPANION_COMPONENT_SCHEMA_REF,
            M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF,
            M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF,
        ],
    );
    row.object_kinds = vec![OK::NotificationEvent, OK::HandoffIntent];
    row.severities = SV::ALL.to_vec();
    row.notification_categories = NC::ALL.to_vec();
    row.dispositions = vec![
        DP::ReviewOnly,
        DP::Cached,
        DP::Stale,
        DP::PolicyBlocked,
        DP::HandoffReady,
    ];
    row.required_labels = labels_with(&[L::ScopeAndFreshness, L::SeverityAndHandoffTarget]);
    row.consumer_surfaces = vec![
        C::NotificationTriageUi,
        C::StatusBarUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ObjectIdentityUnstated,
        D::SeverityUnstated,
        D::FreshnessHidden,
        D::StaleShownAsLive,
        D::GenericCompanionWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Mobile review card.
    let mut row = base_row(
        F::MobileReviewCard,
        Q::Stable,
        "Companion mobile-review-card owner",
        "One mobile-review-card model naming the review kind it carries (agent change, diff review, comment thread, approval request, policy gate, or merge readiness), its client scope, and whether it is review-only or comment-capable from the companion versus desktop-required, so a user never has to infer which actions are companion-safe before tapping",
        "evidence:m5-mobile-review-card-parity:001",
        &[
            M5_COMPANION_COMPONENT_SCHEMA_REF,
            M5_MOBILE_REVIEW_CARD_SCHEMA_REF,
            M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF,
        ],
    );
    row.object_kinds = vec![OK::ReviewItem, OK::HandoffIntent];
    row.review_kinds = RK::ALL.to_vec();
    row.dispositions = vec![
        DP::ReviewOnly,
        DP::CommentCapable,
        DP::DesktopRequired,
        DP::Cached,
        DP::Stale,
        DP::PolicyBlocked,
        DP::HandoffReady,
    ];
    row.required_labels = labels_with(&[L::ScopeAndFreshness, L::CapabilityBoundary]);
    row.consumer_surfaces = vec![
        C::ReviewQueueUi,
        C::DesktopHandoffUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CapabilityBoundaryUnstated,
        D::DispositionUnstated,
        D::DesktopRequiredActionOfferedInline,
        D::GenericCompanionWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. CI-status card.
    let mut row = base_row(
        F::CiStatusCard,
        Q::Stable,
        "Companion CI-status-card owner",
        "One ci-status-card model naming its pipeline status (passed, failed, running, queued, canceled, or stale), its repo/workspace scope, and its freshness (live, cached, or stale), so a stale pipeline status is never shown as live and a user always knows whether the status is current before acting",
        "evidence:m5-ci-status-card-parity:001",
        &[
            M5_COMPANION_COMPONENT_SCHEMA_REF,
            M5_CI_STATUS_CARD_SCHEMA_REF,
            M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF,
        ],
    );
    row.object_kinds = vec![OK::CiRun, OK::HandoffIntent];
    row.ci_statuses = CI::ALL.to_vec();
    row.dispositions = vec![DP::ReviewOnly, DP::Cached, DP::Stale, DP::HandoffReady];
    row.required_labels = labels_with(&[L::ScopeAndFreshness]);
    row.consumer_surfaces = vec![
        C::CiStatusUi,
        C::StatusBarUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::FreshnessHidden,
        D::StaleShownAsLive,
        D::ClientScopeUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Session-follow tile.
    let mut row = base_row(
        F::SessionFollowTile,
        Q::Stable,
        "Companion session-follow-tile owner",
        "One session-follow-tile model naming the followed session's state (live following, paused, diverged from host, host inactive, read-only mirror, or follow ended), its scope, and its freshness, so a diverged or stale followed session is never shown as live and the read/write boundary stays honest",
        "evidence:m5-session-follow-tile-parity:001",
        &[
            M5_COMPANION_COMPONENT_SCHEMA_REF,
            M5_SESSION_FOLLOW_TILE_SCHEMA_REF,
            M5_COMPANION_COMPONENT_FOUNDATION_SESSION_FOLLOW_REF,
        ],
    );
    row.object_kinds = vec![OK::FollowedSession, OK::HandoffIntent];
    row.session_follow_states = SF::ALL.to_vec();
    row.dispositions = vec![
        DP::ReviewOnly,
        DP::DesktopRequired,
        DP::Cached,
        DP::Stale,
        DP::HandoffReady,
    ];
    row.required_labels = labels_with(&[L::ScopeAndFreshness, L::CapabilityBoundary]);
    row.consumer_surfaces = vec![
        C::SessionFollowUi,
        C::DesktopHandoffUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::FreshnessHidden,
        D::StaleShownAsLive,
        D::CapabilityBoundaryUnstated,
        D::DesktopRequiredActionOfferedInline,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Incident-snapshot card.
    let mut row = base_row(
        F::IncidentSnapshotCard,
        Q::Stable,
        "Companion incident-snapshot-card owner",
        "One incident-snapshot-card model naming the incident's severity (critical, high, moderate, low, informational, or unspecified), its scope, and its freshness, so a stale incident snapshot is never shown as live and a user always sees how severe an incident is before escalating or handing off to desktop",
        "evidence:m5-incident-snapshot-card-parity:001",
        &[
            M5_COMPANION_COMPONENT_SCHEMA_REF,
            M5_INCIDENT_SNAPSHOT_CARD_SCHEMA_REF,
            M5_COMPANION_COMPONENT_FOUNDATION_SESSION_FOLLOW_REF,
        ],
    );
    row.object_kinds = vec![OK::IncidentRecord, OK::HandoffIntent];
    row.severities = SV::ALL.to_vec();
    row.dispositions = vec![
        DP::ReviewOnly,
        DP::DesktopRequired,
        DP::Cached,
        DP::Stale,
        DP::HandoffReady,
    ];
    row.required_labels = labels_with(&[L::ScopeAndFreshness, L::SeverityAndHandoffTarget]);
    row.consumer_surfaces = vec![
        C::IncidentAwarenessUi,
        C::StatusBarUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SeverityUnstated,
        D::FreshnessHidden,
        D::StaleShownAsLive,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Desktop-handoff sheet.
    let mut row = base_row(
        F::DesktopHandoffSheet,
        Q::Stable,
        "Companion desktop-handoff-sheet owner",
        "One desktop-handoff-sheet model naming the exact target it will open on desktop (a file location, a review panel, a CI pipeline run, an incident workspace, an agent session, or no handoff) and whether an active host is required, so a user always knows exactly what opens on desktop before a tap and a desktop-required action never reads as companion-safe",
        "evidence:m5-desktop-handoff-sheet-parity:001",
        &[
            M5_COMPANION_COMPONENT_SCHEMA_REF,
            M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
            M5_COMPANION_COMPONENT_FOUNDATION_MATRIX_REF,
        ],
    );
    row.object_kinds = vec![OK::HandoffIntent];
    row.handoff_targets = HT::ALL.to_vec();
    row.dispositions = vec![DP::DesktopRequired, DP::PolicyBlocked, DP::HandoffReady];
    row.required_labels = labels_with(&[L::CapabilityBoundary, L::SeverityAndHandoffTarget]);
    row.consumer_surfaces = vec![
        C::DesktopHandoffUi,
        C::ReviewQueueUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::HandoffTargetUnresolved,
        D::CapabilityBoundaryUnstated,
        D::DesktopRequiredActionOfferedInline,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5CompanionComponentGovernanceReview {
    M5CompanionComponentGovernanceReview {
        notification_row_shows_object_and_severity: true,
        mobile_review_card_shows_capability_boundary: true,
        ci_status_card_shows_status_and_freshness: true,
        session_follow_tile_shows_scope_and_freshness: true,
        incident_snapshot_card_shows_severity_and_freshness: true,
        desktop_handoff_sheet_shows_exact_target: true,
        no_surface_invents_alternate_state_label: true,
        object_identity_always_explicit: true,
        client_scope_named_once: true,
        freshness_always_explicit: true,
        capability_boundary_always_explicit: true,
        severity_always_explicit: true,
        exact_handoff_target_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5CompanionComponentConsumerProjection {
    M5CompanionComponentConsumerProjection {
        notification_surfaces_consume_severity_vocabulary: true,
        review_surfaces_consume_capability_vocabulary: true,
        ci_surfaces_consume_status_vocabulary: true,
        follow_surfaces_consume_freshness_vocabulary: true,
        handoff_surfaces_consume_target_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5CompanionComponentProofFreshness {
    M5CompanionComponentProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CompanionComponentReleasePosture {
    M5CompanionComponentReleasePosture {
        proof_packet_ref: M5_COMPANION_COMPONENT_ARTIFACT_REF.to_owned(),
        companion_audit_ref: M5_COMPANION_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COMPANION_COMPONENT_SCHEMA_REF,
        M5_COMPANION_COMPONENT_DOC_REF,
        M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF,
        M5_MOBILE_REVIEW_CARD_SCHEMA_REF,
        M5_CI_STATUS_CARD_SCHEMA_REF,
        M5_SESSION_FOLLOW_TILE_SCHEMA_REF,
        M5_INCIDENT_SNAPSHOT_CARD_SCHEMA_REF,
        M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
        M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF,
        M5_COMPANION_COMPONENT_FOUNDATION_SESSION_FOLLOW_REF,
        M5_COMPANION_COMPONENT_FOUNDATION_MATRIX_REF,
        M5_COMPANION_COMPONENT_FOUNDATION_SURFACE_CONTRACT_REF,
    ])
}

/// Builds the canonical frozen M5 companion component matrix packet.
pub fn seeded_m5_companion_component_matrix() -> M5CompanionComponentMatrixPacket {
    M5CompanionComponentMatrixPacket::new(M5CompanionComponentMatrixPacketInput {
        packet_id: M5_COMPANION_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 notification-row, mobile-review-card, ci-status-card, session-follow-tile, incident-snapshot-card, and desktop-handoff-sheet component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5CompanionComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the session-follow tile is held at Beta because the bounded light-edit
/// relay slice does not yet round-trip across every surface; every component stays visible.
pub fn seeded_m5_companion_component_matrix_session_follow_tile_beta_narrowed(
) -> M5CompanionComponentMatrixPacket {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.packet_id = "m5-companion-components:session-follow-tile-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CompanionComponentFamily::SessionFollowTile)
        .expect("session-follow-tile row present");
    row.qualification = M5CompanionQualificationClass::Beta;
    packet
}

/// Narrowed variant: the desktop-handoff sheet is narrowed to Preview pending exact-target
/// resolution parity across every surface; every component stays visible.
pub fn seeded_m5_companion_component_matrix_desktop_handoff_sheet_preview_narrowed(
) -> M5CompanionComponentMatrixPacket {
    let mut packet = seeded_m5_companion_component_matrix();
    packet.packet_id = "m5-companion-components:desktop-handoff-sheet-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CompanionComponentFamily::DesktopHandoffSheet)
        .expect("desktop-handoff-sheet row present");
    row.qualification = M5CompanionQualificationClass::Preview;
    packet
}
