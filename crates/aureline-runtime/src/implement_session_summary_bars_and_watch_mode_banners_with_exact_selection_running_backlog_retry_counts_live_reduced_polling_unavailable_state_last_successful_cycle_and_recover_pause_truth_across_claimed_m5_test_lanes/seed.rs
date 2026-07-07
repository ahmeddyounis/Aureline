//! Canonical seed builders for the M5 session-summary-bar / watch-mode-banner primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical session/watch-status primitive packet.
pub const M5_SESSION_WATCH_STATUS_PACKET_ID: &str =
    "m5-session-summary-watch-banner-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked session-summary-bar resolution case from a full session state.
#[allow(clippy::too_many_arguments)]
fn session_case(
    session_mode: M5SessionMode,
    activity_phase: M5SessionActivityPhase,
    selection_scope: M5SessionSelectionScope,
    session_outcome: M5TestSessionOutcome,
    target_class: M5TestTargetClass,
    environment_lane: M5TestEnvironmentLane,
    attempt_lineage: M5AttemptLineageKind,
    watch_fidelity: M5WatchFidelityState,
    running_count: u32,
    backlog_count: u32,
    retry_count: u32,
    selection_label: &str,
    session_identity_ref: &str,
) -> M5SessionSummaryResolutionCase {
    M5SessionSummaryResolutionCase::resolved(M5SessionSummaryResolutionInput {
        session_mode,
        activity_phase,
        selection_scope,
        session_outcome,
        target_class,
        environment_lane,
        attempt_lineage,
        watch_fidelity,
        running_count,
        backlog_count,
        retry_count,
        selection_label: selection_label.to_owned(),
        session_identity_ref: session_identity_ref.to_owned(),
    })
}

/// Builds a worked watch-mode-banner resolution case from a full watch state.
fn watch_case(
    watch_fidelity: M5WatchFidelityState,
    degrade_reason: Option<M5WatchDegradeReason>,
    last_successful_cycle: &str,
    backlog_count: u32,
    watch_label: &str,
    watch_identity_ref: &str,
) -> M5WatchBannerResolutionCase {
    M5WatchBannerResolutionCase::resolved(M5WatchBannerResolutionInput {
        watch_fidelity,
        degrade_reason,
        last_successful_cycle: last_successful_cycle.to_owned(),
        backlog_count,
        watch_label: watch_label.to_owned(),
        watch_identity_ref: watch_identity_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full session/watch anatomy, mode,
/// activity-phase, selection-scope, outcome, posture, watch-fidelity, degrade-reason,
/// action, export-field, and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5SessionWatchConsumerSurface,
    qualification: M5TestQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    session_examples: Vec<M5SessionSummaryResolutionCase>,
    watch_examples: Vec<M5WatchBannerResolutionCase>,
) -> M5SessionWatchConsumerRow {
    M5SessionWatchConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TestSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TestDeploymentLine::ALL.to_vec(),
        session_anatomy_parts: M5SessionSummaryAnatomyPart::ALL.to_vec(),
        watch_anatomy_parts: M5WatchBannerAnatomyPart::ALL.to_vec(),
        session_modes: M5SessionMode::ALL.to_vec(),
        activity_phases: M5SessionActivityPhase::ALL.to_vec(),
        selection_scopes: M5SessionSelectionScope::ALL.to_vec(),
        session_outcomes: M5TestSessionOutcome::ALL.to_vec(),
        session_postures: M5SessionSummaryPosture::ALL.to_vec(),
        attempt_lineage_kinds: M5AttemptLineageKind::ALL.to_vec(),
        watch_fidelity_states: M5WatchFidelityState::ALL.to_vec(),
        watch_degrade_reasons: M5WatchDegradeReason::ALL.to_vec(),
        watch_postures: M5WatchBannerPosture::ALL.to_vec(),
        session_actions: M5SessionSummaryAction::ALL.to_vec(),
        watch_actions: M5WatchBannerAction::ALL.to_vec(),
        session_export_fields: M5SessionSummaryExportField::ALL.to_vec(),
        watch_export_fields: M5WatchBannerExportField::ALL.to_vec(),
        accessibility_routes: M5TestAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TestConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TestDowngradeTrigger::WatchFidelityUnstated,
            M5TestDowngradeTrigger::WatchDegradeReasonHidden,
            M5TestDowngradeTrigger::AttemptLineageUnstated,
            M5TestDowngradeTrigger::AlternateStateLabelInvented,
            M5TestDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SESSION_WATCH_STATUS_SESSION_SCHEMA_REF,
            M5_SESSION_WATCH_STATUS_WATCH_SCHEMA_REF,
            M5_SESSION_WATCH_STATUS_TEST_SESSION_REF,
            M5_SESSION_WATCH_STATUS_WATCH_STATE_REF,
        ]),
        session_examples,
        watch_examples,
        collapses_activity_into_one_spinner: false,
        drops_retry_lineage: false,
        invents_alternate_watch_label: false,
        hides_watch_degrade_or_last_cycle: false,
    }
}

fn rows() -> Vec<M5SessionWatchConsumerRow> {
    use M5AttemptLineageKind as Lineage;
    use M5SessionActivityPhase as Phase;
    use M5SessionMode as Mode;
    use M5SessionSelectionScope as Scope;
    use M5TestEnvironmentLane as Env;
    use M5TestSessionOutcome as Outcome;
    use M5TestTargetClass as Target;
    use M5WatchDegradeReason as Degrade;
    use M5WatchFidelityState as Fidelity;

    vec![
        // 1. Test-explorer status bar — a watch session executing a selected subset with a
        //    backlog and retries while watch is still live, and a live watch banner.
        base_row(
            M5SessionWatchConsumerSurface::TestExplorerStatusBar,
            M5TestQualificationClass::Stable,
            "Test explorer status owner",
            "The test-explorer status bar renders the shared session-summary bar so a watch session executing a selected subset with a backlog and retries reads as an executing session (never a generic spinner) whose exact selection, running/backlog counts, retry lineage, and live watch state are all explicit, and it renders the shared watch banner so a full-fidelity live watch reads as a live watch that can be paused",
            "evidence:m5-session-watch-explorer-status:001",
            vec![
                session_case(
                    Mode::WatchSession,
                    Phase::ExecutingTests,
                    Scope::SelectedSubsetSelection,
                    Outcome::InProgress,
                    Target::UnitTest,
                    Env::LocalHost,
                    Lineage::RetriedFail,
                    Fidelity::Live,
                    7,
                    12,
                    2,
                    "watch: selected auth + pricing subset",
                    "session:explorer::watch-auth-pricing",
                ),
                session_case(
                    Mode::RunOnceSession,
                    Phase::SettledComplete,
                    Scope::WholeSuiteSelection,
                    Outcome::AllPassed,
                    Target::UnitTest,
                    Env::LocalHost,
                    Lineage::FirstAttempt,
                    Fidelity::Live,
                    0,
                    0,
                    0,
                    "run once: whole suite",
                    "session:explorer::run-once-whole",
                ),
            ],
            vec![watch_case(
                Fidelity::Live,
                None,
                "2026-07-07T00:00:00Z",
                0,
                "watch: local host live",
                "watch:explorer::local-live",
            )],
        ),
        // 2. Editor status bar — a run-once session that has settled (rerun-able) with no
        //    backlog and no retries while watch polls, and a reduced-fidelity watch banner
        //    that explains its resource pressure and can recover or pause.
        base_row(
            M5SessionWatchConsumerSurface::EditorStatusBar,
            M5TestQualificationClass::Stable,
            "Editor status owner",
            "The editor status bar renders the shared session-summary bar so a settled run-once session reads as a settled session that offers rerun of its exact whole-suite selection, and it renders the shared watch banner so a reduced-fidelity watch reads as a reduced watch that explains its resource-pressure degradation, preserves its last successful cycle, and exposes both recover and pause — never a green banner over a degraded watch",
            "evidence:m5-session-watch-editor-status:001",
            vec![session_case(
                Mode::DebugSession,
                Phase::DiscoveringTests,
                Scope::SingleCaseSelection,
                Outcome::InProgress,
                Target::IntegrationTest,
                Env::Container,
                Lineage::FirstAttempt,
                Fidelity::Reduced,
                0,
                3,
                0,
                "debug: single failing case",
                "session:editor::debug-single-case",
            )],
            vec![
                watch_case(
                    Fidelity::Reduced,
                    Some(Degrade::ResourcePressure),
                    "2026-07-06T23:40:00Z",
                    4,
                    "watch: container reduced",
                    "watch:editor::container-reduced",
                ),
                watch_case(
                    Fidelity::Paused,
                    None,
                    "2026-07-06T23:10:00Z",
                    0,
                    "watch: user-paused",
                    "watch:editor::user-paused",
                ),
            ],
        ),
        // 3. Run-panel status — a coverage session draining a watch backlog (its own posture,
        //    not a generic spinner) and an imported replay refreshing imported status, plus a
        //    polling watch that explains its adapter limitation.
        base_row(
            M5SessionWatchConsumerSurface::RunPanelStatus,
            M5TestQualificationClass::Stable,
            "Run panel status owner",
            "The run-panel status renders the shared session-summary bar so a coverage session draining a watch backlog reads as a distinct watch-backlog session and an imported replay refreshing imported status reads as a distinct imported-refresh session — proving discovery, execution, watch-backlog, and imported-status refresh never share one loading treatment — and it renders the shared watch banner so a polling watch explains its adapter limitation",
            "evidence:m5-session-watch-run-panel-status:001",
            vec![
                session_case(
                    Mode::CoverageSession,
                    Phase::ProcessingWatchBacklog,
                    Scope::ChangedSinceSelection,
                    Outcome::InProgress,
                    Target::EndToEndTest,
                    Env::CiMatrix,
                    Lineage::RerunSelected,
                    Fidelity::Polling,
                    3,
                    9,
                    1,
                    "coverage: changed since last green",
                    "session:run-panel::coverage-changed",
                ),
                session_case(
                    Mode::ImportedReplaySession,
                    Phase::RefreshingImportedStatus,
                    Scope::ImportedReplaySelection,
                    Outcome::SomeFailed,
                    Target::ContractTest,
                    Env::RemoteRunner,
                    Lineage::ReplayedImport,
                    Fidelity::Polling,
                    0,
                    5,
                    0,
                    "replay: nightly imported results",
                    "session:run-panel::replay-nightly",
                ),
            ],
            vec![watch_case(
                Fidelity::Polling,
                Some(Degrade::AdapterLimited),
                "2026-07-06T22:50:00Z",
                6,
                "watch: ci matrix polling",
                "watch:run-panel::ci-polling",
            )],
        ),
        // 4. Headless / CLI status — a scheduled session that has settled after discovery with
        //    no backlog, and a reconnecting watch that explains its lost file-watch handle and
        //    can recover or pause; proves the same grammar works headless.
        base_row(
            M5SessionWatchConsumerSurface::HeadlessCliStatus,
            M5TestQualificationClass::Stable,
            "Headless CLI status owner",
            "The headless / CLI status renders the shared session-summary bar so a settled scheduled session reads as a settled session with an explicit whole-suite selection and no backlog, and it renders the shared watch banner so a reconnecting watch explains its lost file-watch handle, preserves its last successful cycle, and exposes recover and pause — proving the same status grammar works without a desktop surface",
            "evidence:m5-session-watch-headless-cli-status:001",
            vec![session_case(
                Mode::ScheduledSession,
                Phase::SettledComplete,
                Scope::WholeSuiteSelection,
                Outcome::AllPassed,
                Target::ContractTest,
                Env::RemoteRunner,
                Lineage::RerunFailedOnly,
                Fidelity::Reconnecting,
                0,
                0,
                0,
                "scheduled: nightly whole suite",
                "session:headless::scheduled-nightly",
            )],
            vec![watch_case(
                Fidelity::Reconnecting,
                Some(Degrade::FileWatchLost),
                "2026-07-06T22:30:00Z",
                2,
                "watch: reconnecting after handle loss",
                "watch:headless::reconnecting",
            )],
        ),
        // 5. Session / watch report export — an errored discovery session with retries, and an
        //    unavailable watch that explains its offline host and can recover but not pause;
        //    the same status a reviewer reads elsewhere.
        base_row(
            M5SessionWatchConsumerSurface::SessionWatchReportExport,
            M5TestQualificationClass::Stable,
            "Session watch report export owner",
            "The session / watch report export renders the shared session-summary bar so an errored discovery session with retries reads as a distinct discovering session whose retry lineage stays explicit, and it renders the shared watch banner so an unavailable watch explains its offline host, preserves its last successful cycle, and offers recover while honestly withholding a pause it cannot perform — the same status a reviewer reads in the tree and triage consumers",
            "evidence:m5-session-watch-report-export:001",
            vec![session_case(
                Mode::RunOnceSession,
                Phase::DiscoveringTests,
                Scope::FailedOnlySelection,
                Outcome::ErroredRun,
                Target::UiSnapshotTest,
                Env::BrowserMatrix,
                Lineage::RetriedFail,
                Fidelity::Unavailable,
                0,
                0,
                2,
                "rerun failed-only after error",
                "session:report::failed-only-error",
            )],
            vec![watch_case(
                Fidelity::Unavailable,
                Some(Degrade::OfflineHost),
                "2026-07-06T21:00:00Z",
                0,
                "watch: unavailable offline host",
                "watch:report::offline-unavailable",
            )],
        ),
    ]
}

fn governance_review() -> M5SessionWatchGovernanceReview {
    M5SessionWatchGovernanceReview {
        bar_shows_mode_and_exact_selection: true,
        bar_shows_target_and_environment: true,
        bar_shows_running_and_backlog_counts: true,
        bar_shows_retry_state: true,
        bar_shows_current_watch_state: true,
        distinct_activity_never_one_spinner: true,
        banner_uses_controlled_watch_vocabulary: true,
        banner_explains_degradation_reason: true,
        banner_preserves_last_successful_cycle: true,
        banner_exposes_recover_and_pause: true,
        watch_degradation_visible_everywhere: true,
        components_stable_across_deployment_lines: true,
        components_stable_across_consumer_surfaces: true,
        every_component_declares_accessibility_route: true,
        support_export_reconstructs_status_truth: true,
        later_components_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5SessionWatchConsumerProjection {
    M5SessionWatchConsumerProjection {
        tree_and_status_surfaces_consume_status_vocabulary: true,
        session_posture_reads_single_source: true,
        watch_posture_reads_single_source: true,
        triage_and_support_read_same_watch_vocabulary: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5SessionWatchProofFreshness {
    M5SessionWatchProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SessionWatchReleasePosture {
    M5SessionWatchReleasePosture {
        release_packet_ref: M5_SESSION_WATCH_STATUS_ARTIFACT_REF.to_owned(),
        test_evidence_audit_ref: M5_SESSION_WATCH_STATUS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SESSION_WATCH_STATUS_SESSION_SCHEMA_REF,
        M5_SESSION_WATCH_STATUS_WATCH_SCHEMA_REF,
        M5_SESSION_WATCH_STATUS_DOC_REF,
        M5_SESSION_WATCH_STATUS_COMPONENT_MATRIX_REF,
        M5_SESSION_WATCH_STATUS_TEST_SESSION_REF,
        M5_SESSION_WATCH_STATUS_WATCH_STATE_REF,
    ])
}

/// Builds the canonical M5 session/watch-status packet.
pub fn seeded_m5_session_watch_status_packet() -> M5SessionWatchStatusPacket {
    M5SessionWatchStatusPacket::new(M5SessionWatchStatusPacketInput {
        packet_id: M5_SESSION_WATCH_STATUS_PACKET_ID.to_owned(),
        matrix_label:
            "M5 session-summary-bar / watch-mode-banner primitive: session mode, exact selection, target/environment shorthand, running/backlog/retry counts, distinct discovering/executing/watch-backlog/imported-refresh/settled activity postures, current watch state, controlled live/reduced/polling/reconnecting/paused/unavailable watch postures, explained degradation, preserved last successful cycle, and bounded reveal/rerun/cancel/open-watch and reveal/recover/pause/export actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5SessionWatchVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the run-panel status consumer is narrowed to Preview pending
/// watch-backlog-versus-imported-refresh posture parity proof across every deployment line;
/// every consumer stays visible.
pub fn seeded_m5_session_watch_status_run_panel_status_preview_narrowed(
) -> M5SessionWatchStatusPacket {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.packet_id =
        "m5-session-summary-watch-banner-primitive:run-panel-status-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SessionWatchConsumerSurface::RunPanelStatus)
        .expect("run-panel-status row present");
    row.qualification = M5TestQualificationClass::Preview;
    packet
}

/// Narrowed variant: the headless / CLI status consumer is held at Beta because a slice of
/// headless surfaces do not yet render the keyboard route cue on every profile; every
/// consumer stays visible.
pub fn seeded_m5_session_watch_status_headless_cli_status_beta_narrowed(
) -> M5SessionWatchStatusPacket {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.packet_id =
        "m5-session-summary-watch-banner-primitive:headless-cli-status-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SessionWatchConsumerSurface::HeadlessCliStatus)
        .expect("headless-cli-status row present");
    row.qualification = M5TestQualificationClass::Beta;
    packet
}
