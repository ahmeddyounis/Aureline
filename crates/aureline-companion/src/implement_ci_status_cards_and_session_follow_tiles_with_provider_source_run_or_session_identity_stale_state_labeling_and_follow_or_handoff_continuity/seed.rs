//! Canonical seed builders for the CI-status-card / session-follow-tile controls.
//!
//! These builders are the single producer of the checked-in support export and the
//! scenario fixtures. The headless emitter and the inline tests both call them so the
//! in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical CI-status-card / session-follow-tile packet.
pub const CI_STATUS_CARD_SESSION_FOLLOW_TILE_PACKET_ID: &str =
    "m5-ci-status-card-session-follow-tile-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn card_source_refs() -> Vec<String> {
    strings(&[
        M5_CI_STATUS_CARD_SCHEMA_REF,
        M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF,
    ])
}

fn tile_source_refs() -> Vec<String> {
    strings(&[
        M5_SESSION_FOLLOW_TILE_SCHEMA_REF,
        M5_COMPANION_COMPONENT_FOUNDATION_SESSION_FOLLOW_REF,
    ])
}

/// Builds a CI-status card, deriving the result class, the live claim, and the required
/// notes from the honest inputs so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn ci_status_card(
    card_id: &str,
    pipeline_label: &str,
    object_label: &str,
    object_landing_ref: &str,
    run_ref: &str,
    commit_ref: &str,
    client_scope: M5CompanionClientScope,
    scope_label: &str,
    provider_class: CiProviderClass,
    ci_status: M5CompanionCiStatus,
    failure_count: u32,
    freshness: M5CompanionFreshness,
    handoff_target: M5CompanionHandoffTarget,
    handoff_label: &str,
    status_verbs: Vec<CiStatusCardVerb>,
) -> CiStatusCard {
    let disclosure = resolve_ci_result(ci_status);
    CiStatusCard {
        component: M5CompanionComponentFamily::CiStatusCard,
        card_id: card_id.to_owned(),
        pipeline_label: pipeline_label.to_owned(),
        object_kind: M5CompanionObjectKind::CiRun,
        object_label: object_label.to_owned(),
        object_landing_ref: object_landing_ref.to_owned(),
        run_ref: run_ref.to_owned(),
        commit_ref: commit_ref.to_owned(),
        client_scope,
        scope_label: scope_label.to_owned(),
        provider_class,
        provider_label: format!("Source {}", provider_class.as_str()),
        ci_status,
        result_class: disclosure.result_class,
        claims_live_result: disclosure.is_live_result,
        failure_count,
        freshness,
        stale_note: if disclosure.needs_stale_note {
            format!(
                "CI status {}: shown stale, not a live result — refresh before acting",
                ci_status.as_str()
            )
        } else {
            String::new()
        },
        in_flight_note: if disclosure.needs_in_flight_note {
            "Pipeline is still in flight; the result is not final".to_owned()
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
            "pipeline_label",
            "object_label",
            "run_ref",
            "commit_ref",
            "client_scope",
            "provider_class",
            "ci_status",
            "failure_count",
            "freshness",
            "handoff_target",
        ]),
        source_contract_refs: card_source_refs(),
        masks_scope_or_freshness: false,
        hides_capability_boundary: false,
        invents_alternate_state_label: false,
        implies_desktop_action_is_companion_safe: false,
        routes_to_generic_activity_page: false,
    }
}

/// Builds a session-follow tile, deriving the joinability class, the live/joinable claims,
/// and the required notes from the honest inputs so the seed is always self-consistent with
/// the resolver.
#[allow(clippy::too_many_arguments)]
fn session_follow_tile(
    tile_id: &str,
    session_label: &str,
    object_label: &str,
    object_landing_ref: &str,
    presenter_ref: &str,
    session_ref: &str,
    client_scope: M5CompanionClientScope,
    scope_label: &str,
    follow_state: M5CompanionSessionFollowState,
    freshness: M5CompanionFreshness,
    handoff_target: M5CompanionHandoffTarget,
    handoff_label: &str,
    follow_verbs: Vec<SessionFollowTileVerb>,
) -> SessionFollowTile {
    let disclosure = resolve_session_joinability(follow_state);
    SessionFollowTile {
        component: M5CompanionComponentFamily::SessionFollowTile,
        tile_id: tile_id.to_owned(),
        session_label: session_label.to_owned(),
        object_kind: M5CompanionObjectKind::FollowedSession,
        object_label: object_label.to_owned(),
        object_landing_ref: object_landing_ref.to_owned(),
        presenter_ref: presenter_ref.to_owned(),
        session_ref: session_ref.to_owned(),
        client_scope,
        scope_label: scope_label.to_owned(),
        follow_state,
        joinability: disclosure.joinability,
        claims_live_session: disclosure.is_live_session,
        claims_joinable: disclosure.is_joinable,
        freshness,
        joinability_note: format!(
            "Session joinability {} (live: {}, joinable: {})",
            disclosure.joinability.as_str(),
            disclosure.is_live_session,
            disclosure.is_joinable
        ),
        stale_note: if disclosure.needs_stale_note {
            format!(
                "Session {}: a stale read-only mirror, not a live session — rejoin from desktop",
                follow_state.as_str()
            )
        } else {
            String::new()
        },
        not_joinable_note: if disclosure.needs_not_joinable_note {
            format!(
                "Session {}: not joinable — the host session is not available to follow",
                follow_state.as_str()
            )
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
        follow_verbs,
        degraded_reasons: M5CompanionDegradedReason::ALL.to_vec(),
        required_labels: M5CompanionRequiredLabel::ALL.to_vec(),
        surface_families: M5CompanionSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CompanionDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CompanionAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CompanionConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "session_label",
            "object_label",
            "presenter_ref",
            "session_ref",
            "client_scope",
            "follow_state",
            "freshness",
            "handoff_target",
        ]),
        source_contract_refs: tile_source_refs(),
        masks_scope_or_freshness: false,
        hides_capability_boundary: false,
        invents_alternate_state_label: false,
        implies_desktop_action_is_companion_safe: false,
        routes_to_generic_activity_page: false,
    }
}

fn ci_status_cards() -> Vec<CiStatusCard> {
    use CiProviderClass as Provider;
    use CiStatusCardVerb as Verb;
    use M5CompanionCiStatus as Ci;
    use M5CompanionClientScope as Scope;
    use M5CompanionFreshness as Fresh;
    use M5CompanionHandoffTarget as Handoff;

    vec![
        // 1. Passed, hosted provider, live green: opens the run, follow/logs/rerun/handoff.
        ci_status_card(
            "ci-passed",
            "Release pipeline",
            "CI run #5001",
            "ci_run:pipeline-5001",
            "run-5001",
            "commit-a1b2c3",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Provider::HostedProvider,
            Ci::Passed,
            0,
            Fresh::Live,
            Handoff::CiPipelineRun,
            "Open CI run #5001 on desktop",
            vec![
                Verb::Open,
                Verb::Follow,
                Verb::OpenLogs,
                Verb::Rerun,
                Verb::HandoffToDesktop,
            ],
        ),
        // 2. Failed, self-hosted runner, live red with failure count: logs/rerun/handoff.
        ci_status_card(
            "ci-failed",
            "Integration pipeline",
            "CI run #5002",
            "ci_run:pipeline-5002",
            "run-5002",
            "commit-d4e5f6",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Provider::SelfHostedRunner,
            Ci::Failed,
            3,
            Fresh::Live,
            Handoff::CiPipelineRun,
            "Open failing CI run #5002 on desktop",
            vec![
                Verb::Open,
                Verb::OpenLogs,
                Verb::Rerun,
                Verb::HandoffToDesktop,
            ],
        ),
        // 3. Running, local core, in-flight: follow/logs/handoff, in-flight note required.
        ci_status_card(
            "ci-running",
            "Unit pipeline",
            "CI run #5003",
            "ci_run:pipeline-5003",
            "run-5003",
            "commit-071819",
            Scope::WorkspaceScoped,
            "Workspace platform",
            Provider::LocalCore,
            Ci::Running,
            0,
            Fresh::Live,
            Handoff::CiPipelineRun,
            "Open running CI run #5003 on desktop",
            vec![
                Verb::Open,
                Verb::Follow,
                Verb::OpenLogs,
                Verb::HandoffToDesktop,
            ],
        ),
        // 4. Queued, aggregated source, in-flight (cached): follow/handoff.
        ci_status_card(
            "ci-queued",
            "Nightly pipeline",
            "CI run #5004",
            "ci_run:pipeline-5004",
            "run-5004",
            "commit-202122",
            Scope::OrgScoped,
            "Organization aureline",
            Provider::AggregatedSource,
            Ci::Queued,
            0,
            Fresh::Cached,
            Handoff::CiPipelineRun,
            "Open queued CI run #5004 on desktop",
            vec![Verb::Open, Verb::Follow, Verb::HandoffToDesktop],
        ),
        // 5. Canceled, mirrored snapshot (stale freshness): logs/handoff.
        ci_status_card(
            "ci-canceled",
            "Fuzz pipeline",
            "CI run #5005",
            "ci_run:pipeline-5005",
            "run-5005",
            "commit-232425",
            Scope::RepoScoped,
            "Repository aureline/aureline",
            Provider::MirroredSnapshot,
            Ci::Canceled,
            0,
            Fresh::Stale,
            Handoff::CiPipelineRun,
            "Open canceled CI run #5005 on desktop",
            vec![Verb::Open, Verb::OpenLogs, Verb::HandoffToDesktop],
        ),
        // 6. Stale, unknown source, stale-unknown result with no desktop handoff: because
        //    handoff is unavailable, no rerun or handoff verb is offered — the card never
        //    invents a target it cannot resolve, and it is never shown as a live result.
        ci_status_card(
            "ci-stale",
            "Deploy pipeline",
            "CI run #5006",
            "ci_run:pipeline-5006",
            "run-5006",
            "commit-262728",
            Scope::AccountGlobal,
            "Account-wide",
            Provider::UnknownSource,
            Ci::Stale,
            0,
            Fresh::UnknownFreshness,
            Handoff::NoHandoff,
            "No desktop handoff for this stale CI run",
            vec![Verb::Open, Verb::OpenLogs, Verb::Dismiss],
        ),
    ]
}

fn session_follow_tiles() -> Vec<SessionFollowTile> {
    use M5CompanionClientScope as Scope;
    use M5CompanionFreshness as Fresh;
    use M5CompanionHandoffTarget as Handoff;
    use M5CompanionSessionFollowState as Follow;
    use SessionFollowTileVerb as Verb;

    vec![
        // 1. Live following, live and joinable: open/follow/pause/handoff.
        session_follow_tile(
            "follow-live",
            "Live pairing session",
            "Session sess-9001",
            "followed_session:sess-9001",
            "Presenter alex",
            "sess-9001",
            Scope::WorkspaceScoped,
            "Workspace platform",
            Follow::LiveFollowing,
            Fresh::Live,
            Handoff::AgentSession,
            "Open session sess-9001 on desktop",
            vec![
                Verb::Open,
                Verb::Follow,
                Verb::PauseFollow,
                Verb::HandoffToDesktop,
            ],
        ),
        // 2. Paused follow, resumable and joinable (cached): open/resume/handoff.
        session_follow_tile(
            "follow-paused",
            "Paused pairing session",
            "Session sess-9002",
            "followed_session:sess-9002",
            "Presenter blair",
            "sess-9002",
            Scope::WorkspaceScoped,
            "Workspace platform",
            Follow::PausedFollow,
            Fresh::Cached,
            Handoff::AgentSession,
            "Open paused session sess-9002 on desktop",
            vec![Verb::Open, Verb::ResumeFollow, Verb::HandoffToDesktop],
        ),
        // 3. Diverged from host, stale read-only (not joinable): open/handoff only — no
        //    join verb is offered into a diverged session.
        session_follow_tile(
            "follow-diverged",
            "Diverged pairing session",
            "Session sess-9003",
            "followed_session:sess-9003",
            "Presenter cameron",
            "sess-9003",
            Scope::DeviceScoped,
            "This device",
            Follow::DivergedFromHost,
            Fresh::Stale,
            Handoff::AgentSession,
            "Open session sess-9003 on desktop to rejoin",
            vec![Verb::Open, Verb::HandoffToDesktop],
        ),
        // 4. Host inactive, not joinable (offline-held): open/leave/handoff — not-joinable
        //    note required.
        session_follow_tile(
            "follow-host-inactive",
            "Idle pairing session",
            "Session sess-9004",
            "followed_session:sess-9004",
            "Presenter devon",
            "sess-9004",
            Scope::WorkspaceScoped,
            "Workspace platform",
            Follow::HostInactive,
            Fresh::OfflineHeld,
            Handoff::AgentSession,
            "Open session sess-9004 on desktop when the host returns",
            vec![Verb::Open, Verb::LeaveFollow, Verb::HandoffToDesktop],
        ),
        // 5. Read-only mirror, stale read-only (expired snapshot): open/handoff only.
        session_follow_tile(
            "follow-mirror",
            "Mirror pairing session",
            "Session sess-9005",
            "followed_session:sess-9005",
            "Presenter emerson",
            "sess-9005",
            Scope::OrgScoped,
            "Organization aureline",
            Follow::ReadOnlyMirror,
            Fresh::ExpiredSnapshot,
            Handoff::AgentSession,
            "Open session sess-9005 on desktop to rejoin",
            vec![Verb::Open, Verb::HandoffToDesktop],
        ),
        // 6. Follow ended, not joinable with no desktop handoff: because the session is
        //    over, no join or handoff verb is offered — open/leave only.
        session_follow_tile(
            "follow-ended",
            "Ended pairing session",
            "Session sess-9006",
            "followed_session:sess-9006",
            "Presenter finley",
            "sess-9006",
            Scope::AccountGlobal,
            "Account-wide",
            Follow::FollowEnded,
            Fresh::UnknownFreshness,
            Handoff::NoHandoff,
            "No desktop handoff for this ended session",
            vec![Verb::Open, Verb::LeaveFollow],
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

fn glance_review() -> CiStatusCardSessionFollowTileGlanceReview {
    CiStatusCardSessionFollowTileGlanceReview {
        ci_card_shows_run_and_commit_identity: true,
        ci_card_shows_provider_source: true,
        ci_card_shows_failure_count: true,
        session_tile_shows_presenter_and_session_identity: true,
        session_tile_states_joinability: true,
        session_tile_degrades_to_explicit_state: true,
        object_identity_always_explicit: true,
        client_scope_always_explicit: true,
        freshness_always_explicit: true,
        result_and_joinability_derived_never_asserted: true,
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

fn consumer_projection() -> CiStatusCardSessionFollowTileConsumerProjection {
    CiStatusCardSessionFollowTileConsumerProjection {
        ci_status_ui_reads_single_source: true,
        session_follow_ui_reads_single_source: true,
        first_glance_names_object_scope_and_freshness: true,
        rerun_and_join_posture_visible_before_tap: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> CiStatusCardSessionFollowTileProofFreshness {
    CiStatusCardSessionFollowTileProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_REF,
        CI_STATUS_CARD_SESSION_FOLLOW_TILE_DOC_REF,
        M5_COMPANION_COMPONENT_SCHEMA_REF,
        M5_COMPANION_COMPONENT_DOC_REF,
        M5_CI_STATUS_CARD_SCHEMA_REF,
        M5_SESSION_FOLLOW_TILE_SCHEMA_REF,
    ])
}

/// Builds the canonical CI-status-card / session-follow-tile controls packet.
pub fn seeded_ci_status_card_session_follow_tile_controls(
) -> CiStatusCardSessionFollowTileControlsPacket {
    CiStatusCardSessionFollowTileControlsPacket::new(
        CiStatusCardSessionFollowTileControlsPacketInput {
            packet_id: CI_STATUS_CARD_SESSION_FOLLOW_TILE_PACKET_ID.to_owned(),
            surface_label:
                "M5 CI-status cards and session-follow tiles: provider/source class, run/commit/session identity, freshness, failure counts, keyboard-complete follow/open-logs/handoff quick actions, companion-versus-desktop capability boundary, and an exact desktop-handoff target"
                    .to_owned(),
            ci_status_cards: ci_status_cards(),
            session_follow_tiles: session_follow_tiles(),
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

/// Scenario fixture: spotlights a stale CI-status card that must never read as a live
/// result. Every result class and CI status stays covered so the fixture validates on its
/// own.
pub fn seeded_ci_status_card_session_follow_tile_controls_ci_status_card_stale(
) -> CiStatusCardSessionFollowTileControlsPacket {
    let mut packet = seeded_ci_status_card_session_follow_tile_controls();
    packet.packet_id =
        "m5-ci-status-card-session-follow-tile-controls:fixture:ci-status-card-stale".to_owned();
    packet.surface_label =
        "M5 CI-status cards: a stale CI status never reads as a live pass or fail".to_owned();
    packet
}

/// Scenario fixture: spotlights a not-joinable session-follow tile that must degrade to an
/// explicit not-joinable state instead of an ambiguous empty card. Every joinability class
/// and follow state stays covered so the fixture validates on its own.
pub fn seeded_ci_status_card_session_follow_tile_controls_session_follow_tile_not_joinable(
) -> CiStatusCardSessionFollowTileControlsPacket {
    let mut packet = seeded_ci_status_card_session_follow_tile_controls();
    packet.packet_id =
        "m5-ci-status-card-session-follow-tile-controls:fixture:session-follow-tile-not-joinable"
            .to_owned();
    packet.surface_label =
        "M5 session-follow tiles: a not-joinable session degrades to an explicit state, never an ambiguous empty card"
            .to_owned();
    packet
}
