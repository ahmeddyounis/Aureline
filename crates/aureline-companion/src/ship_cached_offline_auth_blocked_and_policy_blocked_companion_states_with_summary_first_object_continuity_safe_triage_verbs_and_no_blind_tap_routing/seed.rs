//! Canonical seed builders for the companion degraded-state continuity controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls,
//! the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical companion degraded-state continuity packet.
pub const COMPANION_DEGRADED_STATE_CONTINUITY_PACKET_ID: &str =
    "m5-companion-degraded-state-continuity-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The per-surface source refs: the surface's own canonical component schema plus the companion
/// foundation contract its family binds against.
fn row_source_refs(component: M5CompanionComponentFamily) -> Vec<String> {
    use M5CompanionComponentFamily as Family;
    let foundation = match component {
        Family::NotificationRow | Family::MobileReviewCard | Family::CiStatusCard => {
            M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF
        }
        Family::SessionFollowTile | Family::IncidentSnapshotCard => {
            M5_COMPANION_COMPONENT_FOUNDATION_SESSION_FOLLOW_REF
        }
        Family::DesktopHandoffSheet => M5_COMPANION_COMPONENT_FOUNDATION_MATRIX_REF,
    };
    strings(&[component.canonical_component_schema_ref(), foundation])
}

/// Builds a degraded companion surface, deriving the trust class, the live claim, the
/// next-safe-action, and the required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn degraded_surface(
    component: M5CompanionComponentFamily,
    surface_id: &str,
    surface_title: &str,
    object_kind: M5CompanionObjectKind,
    object_label: &str,
    object_landing_ref: &str,
    stable_object_ref: &str,
    client_scope: M5CompanionClientScope,
    scope_label: &str,
    availability_state: CompanionAvailabilityState,
    freshness: M5CompanionFreshness,
    handoff_target: M5CompanionHandoffTarget,
    handoff_label: &str,
    safe_verbs: Vec<CompanionSafeVerb>,
) -> CompanionDegradedSurfaceRow {
    let disclosure = resolve_availability(availability_state);
    CompanionDegradedSurfaceRow {
        component,
        surface_id: surface_id.to_owned(),
        surface_title: surface_title.to_owned(),
        object_kind,
        object_label: object_label.to_owned(),
        object_summary_note: format!("Last-known summary preserved for {object_label}"),
        object_landing_ref: object_landing_ref.to_owned(),
        stable_object_ref: stable_object_ref.to_owned(),
        client_scope,
        scope_label: scope_label.to_owned(),
        availability_state,
        trust_class: disclosure.trust_class,
        claims_live_data: disclosure.is_live,
        freshness,
        scope_and_freshness_note: format!(
            "Scoped to {}; freshness {}",
            client_scope.as_str(),
            freshness.as_str()
        ),
        state_explanation_note: if disclosure.needs_state_explanation {
            format!(
                "State {}: shown as {}, not live",
                availability_state.as_str(),
                disclosure.trust_class.as_str()
            )
        } else {
            String::new()
        },
        next_safe_action: disclosure.next_safe_action,
        next_safe_action_note: format!(
            "Next safe action: {}",
            disclosure.next_safe_action.as_str().replace('_', " ")
        ),
        desktop_fallback_note: if disclosure.needs_desktop_fallback {
            format!(
                "Desktop fallback: hand off to {} on desktop instead",
                handoff_target.as_str()
            )
        } else {
            String::new()
        },
        handoff_target,
        handoff_label: handoff_label.to_owned(),
        safe_verbs,
        degraded_reasons: M5CompanionDegradedReason::ALL.to_vec(),
        required_labels: M5CompanionRequiredLabel::ALL.to_vec(),
        surface_families: M5CompanionSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CompanionDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CompanionAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CompanionConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "surface_title",
            "object_label",
            "object_summary_note",
            "stable_object_ref",
            "client_scope",
            "availability_state",
            "trust_class",
            "freshness",
            "next_safe_action",
            "handoff_target",
        ]),
        source_contract_refs: row_source_refs(component),
        masks_scope_or_freshness: false,
        hides_capability_boundary: false,
        invents_alternate_state_label: false,
        implies_desktop_action_is_companion_safe: false,
        routes_to_generic_activity_page: false,
        routes_blindly_into_broken_or_overprivileged_path: false,
    }
}

fn degraded_surfaces() -> Vec<CompanionDegradedSurfaceRow> {
    use CompanionAvailabilityState as State;
    use CompanionSafeVerb as Verb;
    use M5CompanionClientScope as Scope;
    use M5CompanionComponentFamily as Family;
    use M5CompanionFreshness as Fresh;
    use M5CompanionHandoffTarget as Handoff;
    use M5CompanionObjectKind as Kind;

    vec![
        // 1. Notification row, live: full companion capability, proceed in companion.
        degraded_surface(
            Family::NotificationRow,
            "surface-live-notification",
            "Review requested on change-4201",
            Kind::NotificationEvent,
            "Notification ntf-4201",
            "notification_event:ntf-4201",
            "notification_event:ntf-4201",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            State::Live,
            Fresh::Live,
            Handoff::ReviewPanel,
            "Open review panel on desktop",
            vec![
                Verb::Open,
                Verb::ViewSummary,
                Verb::Refresh,
                Verb::HandoffToDesktop,
            ],
        ),
        // 2. Mobile review card, cached: reduced trust, refresh for the latest, can still triage.
        degraded_surface(
            Family::MobileReviewCard,
            "surface-cached-review",
            "Diff review for change-4202 (cached)",
            Kind::ReviewItem,
            "Review change-4202",
            "review_item:change-4202",
            "review_item:change-4202",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            State::Cached,
            Fresh::Cached,
            Handoff::ReviewPanel,
            "Open review panel on desktop",
            vec![
                Verb::Open,
                Verb::ViewSummary,
                Verb::Refresh,
                Verb::HandoffToDesktop,
            ],
        ),
        // 3. CI-status card, offline: stale until reconnection, retry when online, desktop fallback.
        degraded_surface(
            Family::CiStatusCard,
            "surface-offline-ci",
            "CI run pipeline-4203 (offline)",
            Kind::CiRun,
            "CI run pipeline-4203",
            "ci_run:pipeline-4203",
            "ci_run:pipeline-4203",
            Scope::OrgScoped,
            "Organization aureline",
            State::Offline,
            Fresh::OfflineHeld,
            Handoff::CiPipelineRun,
            "Open CI pipeline run on desktop",
            vec![Verb::Open, Verb::ViewSummary, Verb::HandoffToDesktop],
        ),
        // 4. Session-follow tile, auth-blocked: over-privileged path, reauth on desktop first.
        degraded_surface(
            Family::SessionFollowTile,
            "surface-auth-blocked-session",
            "Follow session sess-4204 (reauth needed)",
            Kind::FollowedSession,
            "Followed session sess-4204",
            "followed_session:sess-4204",
            "followed_session:sess-4204",
            Scope::WorkspaceScoped,
            "Workspace platform",
            State::AuthBlocked,
            Fresh::Stale,
            Handoff::AgentSession,
            "Open agent session on desktop",
            vec![Verb::Open, Verb::ViewSummary, Verb::HandoffToDesktop],
        ),
        // 5. Incident-snapshot card, policy-blocked: publish path no longer allowed, open read-only
        //    on desktop.
        degraded_surface(
            Family::IncidentSnapshotCard,
            "surface-policy-blocked-incident",
            "Incident inc-4205 (companion publish blocked)",
            Kind::IncidentRecord,
            "Incident inc-4205",
            "incident_record:inc-4205",
            "incident_record:inc-4205",
            Scope::OrgScoped,
            "Organization aureline",
            State::PolicyBlocked,
            Fresh::Cached,
            Handoff::IncidentWorkspace,
            "Open incident workspace on desktop",
            vec![Verb::Open, Verb::ViewSummary, Verb::HandoffToDesktop],
        ),
        // 6. Desktop-handoff sheet, loading: detail not available yet, wait for load, desktop
        //    fallback offered.
        degraded_surface(
            Family::DesktopHandoffSheet,
            "surface-loading-handoff",
            "Open failing test on desktop (loading)",
            Kind::HandoffIntent,
            "Handoff hnd-4206",
            "handoff_intent:hnd-4206",
            "handoff_intent:hnd-4206",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            State::Loading,
            Fresh::UnknownFreshness,
            Handoff::FileLocation,
            "Open exact file location on desktop",
            vec![Verb::Open, Verb::ViewSummary, Verb::HandoffToDesktop],
        ),
        // 7. Desktop-handoff sheet, deleted-object: the target was deleted, so the surface
        //    preserves its summary and stops routing — no resolvable handoff, view summary only.
        degraded_surface(
            Family::DesktopHandoffSheet,
            "surface-deleted-handoff",
            "Handoff target no longer exists",
            Kind::HandoffIntent,
            "Handoff hnd-4207",
            "handoff_intent:hnd-4207",
            "handoff_intent:hnd-4207",
            Scope::AccountGlobal,
            "Account-wide",
            State::DeletedObject,
            Fresh::ExpiredSnapshot,
            Handoff::NoHandoff,
            "No desktop target: object was deleted",
            vec![Verb::Open, Verb::ViewSummary, Verb::Dismiss],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5CompanionDowngradeTrigger> {
    vec![
        M5CompanionDowngradeTrigger::ObjectIdentityUnstated,
        M5CompanionDowngradeTrigger::ClientScopeUnstated,
        M5CompanionDowngradeTrigger::FreshnessHidden,
        M5CompanionDowngradeTrigger::CapabilityBoundaryUnstated,
        M5CompanionDowngradeTrigger::HandoffTargetUnresolved,
        M5CompanionDowngradeTrigger::AlternateStateLabelInvented,
        M5CompanionDowngradeTrigger::GenericCompanionWordingUsed,
        M5CompanionDowngradeTrigger::StaleShownAsLive,
        M5CompanionDowngradeTrigger::DesktopRequiredActionOfferedInline,
        M5CompanionDowngradeTrigger::ProofStale,
    ]
}

fn glance_review() -> CompanionDegradedStateGlanceReview {
    CompanionDegradedStateGlanceReview {
        every_surface_names_object_summary_and_identity: true,
        every_surface_states_its_freshness: true,
        every_surface_states_next_safe_action: true,
        degraded_state_is_explicit_before_action: true,
        live_cached_offline_blocked_distinguishable: true,
        cached_or_stale_never_shown_as_live: true,
        trust_class_derived_never_asserted: true,
        safe_triage_verbs_preserved_when_detail_unavailable: true,
        blocked_publish_path_routes_to_desktop: true,
        no_surface_routes_blindly_into_broken_or_overprivileged_path: true,
        deleted_object_preserves_summary_and_stops_routing: true,
        every_broken_or_overprivileged_state_names_desktop_fallback: true,
        object_identity_always_explicit: true,
        client_scope_always_explicit: true,
        no_surface_invents_alternate_state_label: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> CompanionDegradedStateConsumerProjection {
    CompanionDegradedStateConsumerProjection {
        notification_surfaces_read_single_source: true,
        handoff_surfaces_read_single_source: true,
        first_glance_names_state_scope_and_freshness: true,
        next_safe_action_visible_before_tap: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> CompanionDegradedStateProofFreshness {
    CompanionDegradedStateProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        COMPANION_DEGRADED_STATE_CONTINUITY_SCHEMA_REF,
        COMPANION_DEGRADED_STATE_CONTINUITY_DOC_REF,
        M5_COMPANION_COMPONENT_SCHEMA_REF,
        M5_COMPANION_COMPONENT_DOC_REF,
        M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF,
        M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
    ])
}

/// Builds the canonical companion degraded-state continuity controls packet.
pub fn seeded_companion_degraded_state_continuity_controls(
) -> CompanionDegradedStateContinuityPacket {
    CompanionDegradedStateContinuityPacket::new(CompanionDegradedStateContinuityPacketInput {
        packet_id: COMPANION_DEGRADED_STATE_CONTINUITY_PACKET_ID.to_owned(),
        surface_label:
            "M5 companion degraded-state continuity: cached, offline, auth-blocked, policy-blocked, loading, and deleted-object states with summary-first object continuity, derived trust and next-safe-action, safe triage verbs, and a desktop fallback before any broken or over-privileged tap"
                .to_owned(),
        surfaces: degraded_surfaces(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: M5CompanionConsumerSurface::ALL.to_vec(),
        glance_review: glance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Scenario fixture: spotlights an auth-blocked / policy-blocked notification surface that must
/// never route blindly into a broken or over-privileged path — it names an explanatory state and
/// a desktop fallback before a tap. Every availability state and component family stays covered
/// so the fixture validates on its own.
pub fn seeded_companion_degraded_state_continuity_controls_notification_surface_blocked(
) -> CompanionDegradedStateContinuityPacket {
    let mut packet = seeded_companion_degraded_state_continuity_controls();
    packet.packet_id =
        "m5-companion-degraded-state-continuity-controls:fixture:notification-surface-blocked"
            .to_owned();
    packet.surface_label =
        "M5 companion surfaces: an auth-blocked or policy-blocked surface names an explanatory state and a desktop fallback rather than routing blindly"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a deleted-object handoff surface that must preserve its
/// last-known summary and stop routing instead of opening a target that no longer exists. Every
/// availability state and component family stays covered so the fixture validates on its own.
pub fn seeded_companion_degraded_state_continuity_controls_handoff_surface_deleted_object(
) -> CompanionDegradedStateContinuityPacket {
    let mut packet = seeded_companion_degraded_state_continuity_controls();
    packet.packet_id =
        "m5-companion-degraded-state-continuity-controls:fixture:handoff-surface-deleted-object"
            .to_owned();
    packet.surface_label =
        "M5 companion surfaces: a deleted-object handoff preserves its summary and stops routing instead of opening a target that no longer exists"
            .to_owned();
    packet
}
