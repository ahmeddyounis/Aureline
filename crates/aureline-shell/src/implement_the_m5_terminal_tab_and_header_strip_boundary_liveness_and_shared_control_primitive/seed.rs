// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the M5 terminal-tab primitive.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code matrix, the artifact, the worked resolutions, and the fixtures
//! never drift.

use super::*;

/// Stable packet id for the canonical terminal-tab-primitive packet.
pub const M5_TERMINAL_TAB_PRIMITIVE_PACKET_ID: &str = "m5-terminal-tab-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full session state.
#[allow(clippy::too_many_arguments)]
fn case(
    session_title: &str,
    host_boundary: M5HostBoundaryClass,
    shell_integration: M5ShellIntegrationQuality,
    liveness: M5TerminalSessionLiveness,
    connection_state: Option<M5RemoteConnectionState>,
    cwd_repr: Option<&str>,
    last_known_cwd_repr: Option<&str>,
    collaboration_role: Option<M5CollaborationRole>,
    follow_state: Option<M5FollowState>,
    reauthorization_required: bool,
) -> M5TerminalTabResolutionCase {
    M5TerminalTabResolutionCase::resolved(M5TerminalTabResolutionInput {
        session_title: session_title.to_owned(),
        host_boundary,
        shell_integration,
        liveness,
        connection_state,
        cwd_repr: cwd_repr.map(str::to_owned),
        last_known_cwd_repr: last_known_cwd_repr.map(str::to_owned),
        collaboration_role,
        follow_state,
        reauthorization_required,
    })
}

/// A base row with the shared fields filled in and the full anatomy, input-posture,
/// cwd-state, shared-control-posture, export-field, and accessibility parity every
/// consumer carries.
fn base_row(
    console_surface: M5TerminalConsoleSurface,
    qualification: M5RuntimeBoundaryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    shell_zone_slot: M5ShellZoneSlot,
    proof_ref: &str,
    example_resolutions: Vec<M5TerminalTabResolutionCase>,
) -> M5TerminalConsoleRow {
    M5TerminalConsoleRow {
        console_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        shell_zone_slot,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        anatomy_parts: M5TerminalTabAnatomyPart::ALL.to_vec(),
        input_postures: M5TerminalInputPosture::ALL.to_vec(),
        cwd_display_states: M5CwdDisplayState::ALL.to_vec(),
        shared_control_postures: M5SharedControlPosture::ALL.to_vec(),
        export_fields: M5TerminalTabExportField::ALL.to_vec(),
        accessibility_routes: M5RuntimeBoundaryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5RuntimeBoundaryDowngradeTrigger::SessionLivenessAmbiguous,
            M5RuntimeBoundaryDowngradeTrigger::HostBoundaryMasked,
            M5RuntimeBoundaryDowngradeTrigger::ConnectionStateStale,
            M5RuntimeBoundaryDowngradeTrigger::ShellIntegrationQualityHidden,
            M5RuntimeBoundaryDowngradeTrigger::CollaborationRoleMasked,
            M5RuntimeBoundaryDowngradeTrigger::FollowStateAmbiguous,
            M5RuntimeBoundaryDowngradeTrigger::AuditTruthLostOffPrimarySurface,
            M5RuntimeBoundaryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TERMINAL_TAB_SCHEMA_REF,
            M5_TERMINAL_TAB_SESSION_RESTORE_REF,
            M5_TERMINAL_TAB_CONTROL_GRANT_REF,
        ]),
        example_resolutions,
        masks_host_or_runtime_boundary: false,
        conflates_live_and_restored_session: false,
        invents_private_terminal_grammar: false,
        infers_shared_control_from_background_metadata: false,
    }
}

fn console_rows() -> Vec<M5TerminalConsoleRow> {
    use M5CollaborationRole as Role;
    use M5FollowState as Follow;
    use M5HostBoundaryClass as Host;
    use M5RemoteConnectionState as Conn;
    use M5ShellIntegrationQuality as Integ;
    use M5TerminalSessionLiveness as Live;

    let mut rows = Vec::with_capacity(5);

    // 1. Terminal panel — a live local write-capable session, and a remote restored
    //    transcript that is read-only (the AC1 live-versus-restored proof).
    rows.push(base_row(
        M5TerminalConsoleSurface::TerminalPanel,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Terminal panel owner",
        "The terminal panel renders the shared terminal tab so a live local PTY reads as write-capable with its live cwd, while a remote session restored from a transcript reads as read-only with its last-known cwd — never confused with a live write-capable shell",
        M5ShellZoneSlot::BottomPanel,
        "evidence:m5-terminal-tab-panel:001",
        vec![
            case(
                "app-server",
                Host::LocalHost,
                Integ::FullyIntegrated,
                Live::LiveAttached,
                None,
                Some("workspace/app"),
                None,
                None,
                None,
                false,
            ),
            case(
                "api-server",
                Host::RemoteSshHost,
                Integ::CwdReportingOnly,
                Live::RestoredFromTranscript,
                Some(Conn::OfflineCached),
                None,
                Some("workspace/api"),
                None,
                None,
                false,
            ),
        ],
    ));

    // 2. Notebook console — a shared control-held detached-running session, and a
    //    reconnecting session shown read-only with its last-known cwd.
    rows.push(base_row(
        M5TerminalConsoleSurface::NotebookConsole,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Notebook console owner",
        "The notebook kernel console renders the shared terminal tab so a container-hosted detached-running kernel with a held control token reads as shared-control-held and write-capable, while a dropped session reads as read-only-reconnecting with its last-known cwd",
        M5ShellZoneSlot::BottomPanel,
        "evidence:m5-terminal-tab-notebook:001",
        vec![
            case(
                "kernel-py",
                Host::ContainerHost,
                Integ::IntegrationDegraded,
                Live::LiveDetachedRunning,
                Some(Conn::Connected),
                Some("notebook/session"),
                None,
                Some(Role::ControlHolder),
                Some(Follow::BeingFollowed),
                false,
            ),
            case(
                "kernel-r",
                Host::RemoteSshHost,
                Integ::FullyIntegrated,
                Live::Reconnecting,
                Some(Conn::Reconnecting),
                None,
                Some("notebook/api"),
                None,
                None,
                false,
            ),
        ],
    ));

    // 3. Request console — an observer on a managed host (inspect-only), and a live
    //    local collaborator following the presenter with no cwd available.
    rows.push(base_row(
        M5TerminalConsoleSurface::RequestConsole,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Request console owner",
        "The request/REPL console renders the shared terminal tab so an observer on a managed workspace host reads as inspect-only with cwd-not-reported by the shell, while a collaborator following the presenter reads as shared-following-presenter with cwd-unavailable rather than a stale value",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-terminal-tab-request:001",
        vec![
            case(
                "repl-managed",
                Host::ManagedWorkspaceHost,
                Integ::CommandMarksOnly,
                Live::LiveAttached,
                Some(Conn::Connected),
                None,
                None,
                Some(Role::Observer),
                Some(Follow::FollowingPresenter),
                false,
            ),
            case(
                "repl-local",
                Host::LocalHost,
                Integ::FullyIntegrated,
                Live::LiveAttached,
                None,
                None,
                None,
                Some(Role::Collaborator),
                Some(Follow::FollowingPresenter),
                false,
            ),
        ],
    ));

    // 4. Preview dev-server — a closed exited session with its last-known cwd, and a
    //    wasm-sandbox session blocked pending reauthorization (the AC3 proof).
    rows.push(base_row(
        M5TerminalConsoleSurface::PreviewDevServer,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Preview dev-server owner",
        "The preview dev-server console renders the shared terminal tab so a closed dev-server reads as closed-no-input with its last-known cwd, while a shared wasm-sandbox session pending reauthorization reads as reauthorization-blocked and reauthorization-required rather than silently allowing input",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-terminal-tab-preview:001",
        vec![
            case(
                "vite-preview",
                Host::VirtualMachineHost,
                Integ::FullyIntegrated,
                Live::ClosedExited,
                Some(Conn::Disconnected),
                None,
                Some("preview/build"),
                None,
                None,
                false,
            ),
            case(
                "wasm-preview",
                Host::WasmSandboxHost,
                Integ::BasicPtyNoIntegration,
                Live::LiveAttached,
                Some(Conn::Connected),
                None,
                None,
                Some(Role::SessionHost),
                None,
                true,
            ),
        ],
    ));

    // 5. Incident shell — a presenter driving a live remote session, and a
    //    container-hosted restored incident transcript (a second read-only proof).
    rows.push(base_row(
        M5TerminalConsoleSurface::IncidentShell,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Incident shell owner",
        "The incident/break-glass shell renders the shared terminal tab so a presenter driving a live remote session reads as shared-control-held and write-capable with its live cwd, while a restored incident transcript reads as read-only with its last-known cwd — boundary and liveness legible before any keystroke",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-terminal-tab-incident:001",
        vec![
            case(
                "incident-triage",
                Host::RemoteSshHost,
                Integ::CwdReportingOnly,
                Live::LiveDetachedRunning,
                Some(Conn::Connected),
                Some("incident/triage"),
                None,
                Some(Role::Presenter),
                Some(Follow::PresentingToOthers),
                false,
            ),
            case(
                "incident-log",
                Host::ContainerHost,
                Integ::FullyIntegrated,
                Live::RestoredFromTranscript,
                Some(Conn::OfflineCached),
                None,
                Some("incident/log"),
                None,
                None,
                false,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5TerminalTabGovernanceReview {
    M5TerminalTabGovernanceReview {
        one_primitive_carries_boundary_and_liveness: true,
        session_title_and_host_boundary_always_shown: true,
        live_versus_restored_never_conflated: true,
        restored_transcript_never_write_capable: true,
        cwd_or_last_known_cwd_always_disclosed: true,
        shared_control_and_reauthorization_always_explicit: true,
        support_export_reconstructs_boundary_truth: true,
        no_surface_invents_second_terminal_grammar: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5TerminalTabConsumerProjection {
    M5TerminalTabConsumerProjection {
        terminal_console_surfaces_consume_shared_primitive: true,
        liveness_resolver_reads_single_source: true,
        shared_control_reads_single_collaboration_source: true,
        boundary_badge_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5TerminalTabProofFreshness {
    M5TerminalTabProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TerminalTabReleasePosture {
    M5TerminalTabReleasePosture {
        release_packet_ref: M5_TERMINAL_TAB_ARTIFACT_REF.to_owned(),
        terminal_tab_audit_ref: M5_TERMINAL_TAB_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TERMINAL_TAB_SCHEMA_REF,
        M5_TERMINAL_TAB_DOC_REF,
        M5_TERMINAL_TAB_SHELL_ZONE_REF,
        M5_TERMINAL_TAB_COMPONENT_MATRIX_REF,
        M5_TERMINAL_TAB_SESSION_RESTORE_REF,
        M5_TERMINAL_TAB_CONTROL_GRANT_REF,
    ])
}

/// Builds the canonical M5 terminal-tab-primitive packet.
pub fn seeded_m5_terminal_tab_primitive_packet() -> M5TerminalTabPrimitivePacket {
    M5TerminalTabPrimitivePacket::new(M5TerminalTabPrimitivePacketInput {
        packet_id: M5_TERMINAL_TAB_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 terminal-tab and header-strip primitive: session title, host boundary, shell-integration quality, cwd-or-transcript state, and shared control"
                .to_owned(),
        console_rows: console_rows(),
        vocabulary_set: M5TerminalTabVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the incident shell is held at Beta because a slice of
/// break-glass sessions do not yet render the reauthorization cue on every profile;
/// every consumer stays visible.
pub fn seeded_m5_terminal_tab_primitive_incident_shell_beta_narrowed(
) -> M5TerminalTabPrimitivePacket {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.packet_id = "m5-terminal-tab-primitive:incident-shell-beta:0001".to_owned();
    let row = packet
        .console_rows
        .iter_mut()
        .find(|row| row.console_surface == M5TerminalConsoleSurface::IncidentShell)
        .expect("incident shell row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the preview dev-server console is narrowed to Preview pending
/// last-known-cwd parity proof across every export path; every consumer stays
/// visible.
pub fn seeded_m5_terminal_tab_primitive_preview_dev_server_preview_narrowed(
) -> M5TerminalTabPrimitivePacket {
    let mut packet = seeded_m5_terminal_tab_primitive_packet();
    packet.packet_id = "m5-terminal-tab-primitive:preview-dev-server-preview:0001".to_owned();
    let row = packet
        .console_rows
        .iter_mut()
        .find(|row| row.console_surface == M5TerminalConsoleSurface::PreviewDevServer)
        .expect("preview dev-server row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Preview;
    packet
}
