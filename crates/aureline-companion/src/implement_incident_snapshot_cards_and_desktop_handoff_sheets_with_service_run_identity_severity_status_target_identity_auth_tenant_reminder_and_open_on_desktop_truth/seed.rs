//! Canonical seed builders for the incident-snapshot-card / desktop-handoff-sheet controls.
//!
//! These builders are the single producer of the checked-in support export and the
//! scenario fixtures. The headless emitter and the inline tests both call them so the
//! in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical incident-snapshot-card / desktop-handoff-sheet packet.
pub const INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_PACKET_ID: &str =
    "m5-incident-snapshot-card-desktop-handoff-sheet-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn card_source_refs() -> Vec<String> {
    strings(&[
        M5_INCIDENT_SNAPSHOT_CARD_SCHEMA_REF,
        M5_COMPANION_COMPONENT_FOUNDATION_SESSION_FOLLOW_REF,
    ])
}

fn sheet_source_refs() -> Vec<String> {
    strings(&[
        M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
        M5_COMPANION_COMPONENT_FOUNDATION_MATRIX_REF,
    ])
}

/// Builds an incident-snapshot card, deriving the awareness class, the live claim, and the
/// required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn incident_snapshot_card(
    card_id: &str,
    incident_label: &str,
    object_label: &str,
    object_landing_ref: &str,
    service_ref: &str,
    run_ref: &str,
    client_scope: M5CompanionClientScope,
    scope_label: &str,
    service_class: IncidentServiceClass,
    severity: M5CompanionSeverity,
    incident_status: IncidentStatus,
    freshness: M5CompanionFreshness,
    handoff_target: M5CompanionHandoffTarget,
    handoff_label: &str,
    status_verbs: Vec<IncidentSnapshotCardVerb>,
) -> IncidentSnapshotCard {
    let disclosure = resolve_incident_awareness(incident_status);
    IncidentSnapshotCard {
        component: M5CompanionComponentFamily::IncidentSnapshotCard,
        card_id: card_id.to_owned(),
        incident_label: incident_label.to_owned(),
        object_kind: M5CompanionObjectKind::IncidentRecord,
        object_label: object_label.to_owned(),
        object_landing_ref: object_landing_ref.to_owned(),
        service_ref: service_ref.to_owned(),
        run_ref: run_ref.to_owned(),
        client_scope,
        scope_label: scope_label.to_owned(),
        service_class,
        service_label: format!("Service {}", service_class.as_str()),
        severity,
        severity_label: format!("Severity {}", severity.as_str()),
        incident_status,
        awareness_class: disclosure.awareness_class,
        claims_live_status: disclosure.is_live_status,
        freshness,
        stale_note: if disclosure.needs_stale_note {
            format!(
                "Incident status {}: shown stale, not a live incident — refresh before acting",
                incident_status.as_str()
            )
        } else {
            String::new()
        },
        awareness_note: if disclosure.needs_awareness_note {
            "Awareness only from the companion; remediation happens on desktop".to_owned()
        } else {
            String::new()
        },
        scope_and_freshness_note: format!(
            "Scoped to {}; freshness {}",
            client_scope.as_str(),
            freshness.as_str()
        ),
        handoff_target,
        handoff_label: handoff_label.to_owned(),
        status_verbs,
        degraded_reasons: M5CompanionDegradedReason::ALL.to_vec(),
        required_labels: M5CompanionRequiredLabel::ALL.to_vec(),
        surface_families: M5CompanionSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CompanionDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CompanionAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CompanionConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "incident_label",
            "object_label",
            "service_ref",
            "run_ref",
            "client_scope",
            "service_class",
            "severity",
            "incident_status",
            "freshness",
            "handoff_target",
        ]),
        source_contract_refs: card_source_refs(),
        masks_scope_or_freshness: false,
        hides_capability_boundary: false,
        invents_alternate_state_label: false,
        implies_desktop_action_is_companion_safe: false,
        routes_to_generic_activity_page: false,
        implies_companion_remediation: false,
    }
}

/// Builds a desktop-handoff sheet, deriving the open class, the openable claim, and the
/// required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn desktop_handoff_sheet(
    sheet_id: &str,
    handoff_title: &str,
    object_label: &str,
    object_landing_ref: &str,
    target_ref: &str,
    target_object_label: &str,
    client_scope: M5CompanionClientScope,
    scope_label: &str,
    handoff_target: M5CompanionHandoffTarget,
    freshness: M5CompanionFreshness,
    auth_context: HandoffAuthContext,
    handoff_label: &str,
    handoff_verbs: Vec<DesktopHandoffSheetVerb>,
) -> DesktopHandoffSheet {
    let disclosure = resolve_handoff_open(handoff_target);
    DesktopHandoffSheet {
        component: M5CompanionComponentFamily::DesktopHandoffSheet,
        sheet_id: sheet_id.to_owned(),
        handoff_title: handoff_title.to_owned(),
        object_kind: M5CompanionObjectKind::HandoffIntent,
        object_label: object_label.to_owned(),
        object_landing_ref: object_landing_ref.to_owned(),
        target_ref: target_ref.to_owned(),
        target_object_label: target_object_label.to_owned(),
        client_scope,
        scope_label: scope_label.to_owned(),
        handoff_target,
        open_class: disclosure.open_class,
        claims_openable: disclosure.is_openable,
        freshness,
        opens_on_desktop_note: if disclosure.is_openable {
            format!(
                "Opens on desktop: {} ({})",
                target_object_label,
                handoff_target.as_str()
            )
        } else {
            format!("Nothing opens on desktop for {target_object_label}")
        },
        not_openable_note: if disclosure.needs_not_openable_note {
            "No desktop target resolves for this sheet; it cannot open on desktop".to_owned()
        } else {
            String::new()
        },
        auth_context,
        auth_tenant_reminder_note: if auth_context.needs_reminder() {
            format!(
                "Before opening on desktop: {}",
                auth_context.as_str().replace('_', " ")
            )
        } else {
            String::new()
        },
        scope_and_freshness_note: format!(
            "Scoped to {}; freshness {}",
            client_scope.as_str(),
            freshness.as_str()
        ),
        handoff_label: handoff_label.to_owned(),
        handoff_verbs,
        degraded_reasons: M5CompanionDegradedReason::ALL.to_vec(),
        required_labels: M5CompanionRequiredLabel::ALL.to_vec(),
        surface_families: M5CompanionSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CompanionDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CompanionAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CompanionConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "handoff_title",
            "object_label",
            "target_ref",
            "target_object_label",
            "client_scope",
            "handoff_target",
            "auth_context",
            "freshness",
        ]),
        source_contract_refs: sheet_source_refs(),
        masks_scope_or_freshness: false,
        hides_capability_boundary: false,
        invents_alternate_state_label: false,
        implies_desktop_action_is_companion_safe: false,
        routes_to_generic_activity_page: false,
    }
}

fn incident_snapshot_cards() -> Vec<IncidentSnapshotCard> {
    use IncidentServiceClass as Service;
    use IncidentSnapshotCardVerb as Verb;
    use IncidentStatus as Status;
    use M5CompanionClientScope as Scope;
    use M5CompanionFreshness as Fresh;
    use M5CompanionHandoffTarget as Handoff;
    use M5CompanionSeverity as Sev;

    vec![
        // 1. Firing, hosted service, critical, live active-unacknowledged: open/ack/handoff.
        incident_snapshot_card(
            "incident-firing",
            "Checkout latency spike",
            "Incident inc-7001",
            "incident_record:inc-7001",
            "svc-checkout",
            "occurrence-7001",
            Scope::OrgScoped,
            "Organization aureline",
            Service::HostedService,
            Sev::Critical,
            Status::Firing,
            Fresh::Live,
            Handoff::IncidentWorkspace,
            "Open incident inc-7001 on desktop",
            vec![
                Verb::Open,
                Verb::Acknowledge,
                Verb::ViewTimeline,
                Verb::Follow,
                Verb::HandoffToDesktop,
            ],
        ),
        // 2. Acknowledged, self-hosted service, high, active-acknowledged: open/timeline/handoff.
        incident_snapshot_card(
            "incident-acknowledged",
            "Queue backlog growing",
            "Incident inc-7002",
            "incident_record:inc-7002",
            "svc-queue",
            "occurrence-7002",
            Scope::WorkspaceScoped,
            "Workspace platform",
            Service::SelfHostedService,
            Sev::High,
            Status::Acknowledged,
            Fresh::Live,
            Handoff::IncidentWorkspace,
            "Open incident inc-7002 on desktop",
            vec![
                Verb::Open,
                Verb::ViewTimeline,
                Verb::Follow,
                Verb::HandoffToDesktop,
            ],
        ),
        // 3. Investigating, local core, moderate, active-acknowledged (cached): open/timeline/handoff.
        incident_snapshot_card(
            "incident-investigating",
            "Elevated error rate",
            "Incident inc-7003",
            "incident_record:inc-7003",
            "svc-api",
            "occurrence-7003",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Service::LocalCoreService,
            Sev::Moderate,
            Status::Investigating,
            Fresh::Cached,
            Handoff::IncidentWorkspace,
            "Open incident inc-7003 on desktop",
            vec![Verb::Open, Verb::ViewTimeline, Verb::HandoffToDesktop],
        ),
        // 4. Mitigating, aggregated source, low, mitigating: open/timeline/handoff.
        incident_snapshot_card(
            "incident-mitigating",
            "Cache warm-up degraded",
            "Incident inc-7004",
            "incident_record:inc-7004",
            "svc-cache",
            "occurrence-7004",
            Scope::OrgScoped,
            "Organization aureline",
            Service::AggregatedSource,
            Sev::Low,
            Status::Mitigating,
            Fresh::Live,
            Handoff::IncidentWorkspace,
            "Open incident inc-7004 on desktop",
            vec![Verb::Open, Verb::ViewTimeline, Verb::HandoffToDesktop],
        ),
        // 5. Resolved, mirrored snapshot, informational, resolved (stale freshness): open/timeline/handoff.
        incident_snapshot_card(
            "incident-resolved",
            "Nightly job flake",
            "Incident inc-7005",
            "incident_record:inc-7005",
            "svc-scheduler",
            "occurrence-7005",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Service::MirroredSnapshot,
            Sev::Informational,
            Status::Resolved,
            Fresh::Stale,
            Handoff::IncidentWorkspace,
            "Open resolved incident inc-7005 on desktop",
            vec![Verb::Open, Verb::ViewTimeline, Verb::HandoffToDesktop],
        ),
        // 6. Stale, unknown source, unspecified, stale-unknown with no desktop handoff: because
        //    handoff is unavailable, no handoff verb is offered — the card never invents a target
        //    it cannot resolve, and it is never shown as a live incident.
        incident_snapshot_card(
            "incident-stale",
            "Unclassified signal",
            "Incident inc-7006",
            "incident_record:inc-7006",
            "svc-unknown",
            "occurrence-7006",
            Scope::AccountGlobal,
            "Account-wide",
            Service::UnknownSource,
            Sev::Unspecified,
            Status::Stale,
            Fresh::UnknownFreshness,
            Handoff::NoHandoff,
            "No desktop handoff for this stale incident",
            vec![Verb::Open, Verb::ViewTimeline, Verb::Dismiss],
        ),
    ]
}

fn desktop_handoff_sheets() -> Vec<DesktopHandoffSheet> {
    use DesktopHandoffSheetVerb as Verb;
    use HandoffAuthContext as Auth;
    use M5CompanionClientScope as Scope;
    use M5CompanionFreshness as Fresh;
    use M5CompanionHandoffTarget as Handoff;

    vec![
        // 1. File location, same auth, live opens-exact-location: open/open-on-desktop/copy/preview.
        desktop_handoff_sheet(
            "handoff-file",
            "Open failing test on desktop",
            "Handoff hnd-8001",
            "handoff_intent:hnd-8001",
            "file:src/lib.rs#L42",
            "src/lib.rs at line 42",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Handoff::FileLocation,
            Fresh::Live,
            Auth::SameAuthNoReminder,
            "Open exact file location on desktop",
            vec![
                Verb::Open,
                Verb::OpenOnDesktop,
                Verb::CopyReference,
                Verb::PreviewTarget,
            ],
        ),
        // 2. Review panel, reauth required, live opens-exact-panel: open/open-on-desktop/share.
        desktop_handoff_sheet(
            "handoff-review",
            "Open review on desktop",
            "Handoff hnd-8002",
            "handoff_intent:hnd-8002",
            "review:change-8002",
            "Review change-8002",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Handoff::ReviewPanel,
            Fresh::Live,
            Auth::ReauthRequired,
            "Open review panel on desktop",
            vec![Verb::Open, Verb::OpenOnDesktop, Verb::Share],
        ),
        // 3. CI pipeline run, tenant switch required, cached opens-exact-panel: open/open-on-desktop.
        desktop_handoff_sheet(
            "handoff-ci",
            "Open CI run on desktop",
            "Handoff hnd-8003",
            "handoff_intent:hnd-8003",
            "ci_run:pipeline-8003",
            "CI run pipeline-8003",
            Scope::OrgScoped,
            "Organization aureline",
            Handoff::CiPipelineRun,
            Fresh::Cached,
            Auth::TenantSwitchRequired,
            "Open CI pipeline run on desktop",
            vec![Verb::Open, Verb::OpenOnDesktop, Verb::CopyReference],
        ),
        // 4. Incident workspace, account mismatch warning, live opens-exact-workspace.
        desktop_handoff_sheet(
            "handoff-incident",
            "Open incident workspace on desktop",
            "Handoff hnd-8004",
            "handoff_intent:hnd-8004",
            "incident_workspace:inc-8004",
            "Incident workspace inc-8004",
            Scope::OrgScoped,
            "Organization aureline",
            Handoff::IncidentWorkspace,
            Fresh::Live,
            Auth::AccountMismatchWarning,
            "Open incident workspace on desktop",
            vec![Verb::Open, Verb::OpenOnDesktop, Verb::PreviewTarget],
        ),
        // 5. Agent session, scope elevation required, offline-held opens-exact-workspace.
        desktop_handoff_sheet(
            "handoff-agent",
            "Open agent session on desktop",
            "Handoff hnd-8005",
            "handoff_intent:hnd-8005",
            "agent_session:sess-8005",
            "Agent session sess-8005",
            Scope::WorkspaceScoped,
            "Workspace platform",
            Handoff::AgentSession,
            Fresh::OfflineHeld,
            Auth::ScopeElevationRequired,
            "Open agent session on desktop",
            vec![Verb::Open, Verb::OpenOnDesktop, Verb::Share],
        ),
        // 6. No handoff, same auth, expired snapshot not-openable: because nothing resolves, no
        //    open-on-desktop verb is offered — open/copy/dismiss only, not-openable note required.
        desktop_handoff_sheet(
            "handoff-none",
            "No desktop target available",
            "Handoff hnd-8006",
            "handoff_intent:hnd-8006",
            "handoff_intent:hnd-8006",
            "Unresolved handoff target",
            Scope::AccountGlobal,
            "Account-wide",
            Handoff::NoHandoff,
            Fresh::ExpiredSnapshot,
            Auth::SameAuthNoReminder,
            "No desktop target to open",
            vec![Verb::Open, Verb::CopyReference, Verb::Dismiss],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5CompanionDowngradeTrigger> {
    vec![
        M5CompanionDowngradeTrigger::ObjectIdentityUnstated,
        M5CompanionDowngradeTrigger::ClientScopeUnstated,
        M5CompanionDowngradeTrigger::FreshnessHidden,
        M5CompanionDowngradeTrigger::CapabilityBoundaryUnstated,
        M5CompanionDowngradeTrigger::SeverityUnstated,
        M5CompanionDowngradeTrigger::HandoffTargetUnresolved,
        M5CompanionDowngradeTrigger::AlternateStateLabelInvented,
        M5CompanionDowngradeTrigger::GenericCompanionWordingUsed,
        M5CompanionDowngradeTrigger::StaleShownAsLive,
        M5CompanionDowngradeTrigger::DesktopRequiredActionOfferedInline,
        M5CompanionDowngradeTrigger::ProofStale,
    ]
}

fn glance_review() -> IncidentSnapshotCardDesktopHandoffSheetGlanceReview {
    IncidentSnapshotCardDesktopHandoffSheetGlanceReview {
        incident_card_shows_service_and_run_identity: true,
        incident_card_shows_severity: true,
        incident_card_states_latest_status: true,
        incident_card_stays_awareness_only: true,
        handoff_sheet_shows_target_object_and_identity: true,
        handoff_sheet_states_what_opens_on_desktop: true,
        handoff_sheet_shows_auth_or_tenant_reminder: true,
        object_identity_always_explicit: true,
        client_scope_always_explicit: true,
        freshness_always_explicit: true,
        awareness_and_openability_derived_never_asserted: true,
        stale_never_shown_as_live: true,
        every_verb_traces_to_one_object: true,
        every_handoff_names_exact_target: true,
        desktop_only_action_never_implied_companion_safe: true,
        no_surface_invents_alternate_state_label: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> IncidentSnapshotCardDesktopHandoffSheetConsumerProjection {
    IncidentSnapshotCardDesktopHandoffSheetConsumerProjection {
        incident_awareness_ui_reads_single_source: true,
        desktop_handoff_ui_reads_single_source: true,
        first_glance_names_object_scope_and_freshness: true,
        remediation_and_open_posture_visible_before_tap: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> IncidentSnapshotCardDesktopHandoffSheetProofFreshness {
    IncidentSnapshotCardDesktopHandoffSheetProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
        INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_DOC_REF,
        M5_COMPANION_COMPONENT_SCHEMA_REF,
        M5_COMPANION_COMPONENT_DOC_REF,
        M5_INCIDENT_SNAPSHOT_CARD_SCHEMA_REF,
        M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
    ])
}

/// Builds the canonical incident-snapshot-card / desktop-handoff-sheet controls packet.
pub fn seeded_incident_snapshot_card_desktop_handoff_sheet_controls(
) -> IncidentSnapshotCardDesktopHandoffSheetControlsPacket {
    IncidentSnapshotCardDesktopHandoffSheetControlsPacket::new(
        IncidentSnapshotCardDesktopHandoffSheetControlsPacketInput {
            packet_id: INCIDENT_SNAPSHOT_CARD_DESKTOP_HANDOFF_SHEET_PACKET_ID.to_owned(),
            surface_label:
                "M5 incident-snapshot cards and desktop-handoff sheets: service/run identity, severity, latest status, freshness, bounded acknowledge/handoff actions, companion-versus-desktop capability boundary, exact desktop target, and auth/tenant reminder where relevant"
                    .to_owned(),
            incident_snapshot_cards: incident_snapshot_cards(),
            desktop_handoff_sheets: desktop_handoff_sheets(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5CompanionConsumerSurface::ALL.to_vec(),
            glance_review: glance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a stale incident-snapshot card that must never read as a live
/// incident. Every awareness class and incident status stays covered so the fixture validates
/// on its own.
pub fn seeded_incident_snapshot_card_desktop_handoff_sheet_controls_incident_snapshot_card_stale(
) -> IncidentSnapshotCardDesktopHandoffSheetControlsPacket {
    let mut packet = seeded_incident_snapshot_card_desktop_handoff_sheet_controls();
    packet.packet_id =
        "m5-incident-snapshot-card-desktop-handoff-sheet-controls:fixture:incident-snapshot-card-stale"
            .to_owned();
    packet.surface_label =
        "M5 incident-snapshot cards: a stale incident never reads as a live incident and stays awareness-only"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a not-openable desktop-handoff sheet that must degrade to an
/// explicit not-openable state instead of implying a desktop client will open the intended
/// object. Every open class and handoff target stays covered so the fixture validates on its
/// own.
pub fn seeded_incident_snapshot_card_desktop_handoff_sheet_controls_desktop_handoff_sheet_not_openable(
) -> IncidentSnapshotCardDesktopHandoffSheetControlsPacket {
    let mut packet = seeded_incident_snapshot_card_desktop_handoff_sheet_controls();
    packet.packet_id =
        "m5-incident-snapshot-card-desktop-handoff-sheet-controls:fixture:desktop-handoff-sheet-not-openable"
            .to_owned();
    packet.surface_label =
        "M5 desktop-handoff sheets: a sheet with no resolvable target degrades to an explicit not-openable state"
            .to_owned();
    packet
}
