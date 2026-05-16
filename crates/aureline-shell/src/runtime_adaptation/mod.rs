//! Beta-grade runtime-adaptation projection for suspend-resume,
//! battery/thermal pressure, and multi-monitor continuity.
//!
//! This module is the page-level surface that hardens the daily desktop
//! beta posture. It does not own its own power/thermal policy — that
//! still lives in [`crate::efficiency`] — and it does not invent any new
//! topology truth. It projects the acceptance states M3 needs daily-beta
//! users to be able to inspect:
//!
//! - the power/thermal posture of the host and which background work
//!   is paused or degraded under that posture;
//! - the protected foreground actions (editing, palette, direct
//!   navigation) that must remain responsive at every posture;
//! - suspend-resume cycles with an explicit continuity summary
//!   (reconnect, stale, resumed-work) instead of silent behavior drift;
//! - multi-monitor reconnect / detach / mixed-DPI events with an
//!   explicit continuity summary;
//! - a per-OS desktop matrix row that names the downgrade behavior on
//!   macOS, Windows, and Linux so support and design QA can pivot
//!   between OS rows without scraping UI text.
//!
//! The same projection feeds the live shell, the
//! `aureline_shell_runtime_adaptation` headless inspector, and the
//! support-export wrapper. UI rows, CLI rows, and support-export rows
//! always come from the same `case_id` and `shared_contract_ref`, so the
//! live shell, the review packet, and the support export report the
//! same runtime-adaptation truth.

use serde::{Deserialize, Serialize};

/// Beta runtime-adaptation schema version exported with every record.
pub const RUNTIME_ADAPTATION_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every beta runtime-adaptation row.
pub const RUNTIME_ADAPTATION_SHARED_CONTRACT_REF: &str = "shell:runtime_adaptation_beta:v1";

/// Stable record kind for [`RuntimeAdaptationPage`] payloads.
pub const RUNTIME_ADAPTATION_PAGE_RECORD_KIND: &str = "shell_runtime_adaptation_beta_page_record";

/// Stable record kind for [`RuntimeAdaptationSupportExport`] payloads.
pub const RUNTIME_ADAPTATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_runtime_adaptation_beta_support_export_record";

/// Stable record kind for [`PowerPostureRow`] payloads.
pub const POWER_POSTURE_RECORD_KIND: &str = "shell_runtime_adaptation_beta_power_posture_record";

/// Stable record kind for [`SuspendResumeRow`] payloads.
pub const SUSPEND_RESUME_RECORD_KIND: &str = "shell_runtime_adaptation_beta_suspend_resume_record";

/// Stable record kind for [`MonitorContinuityRow`] payloads.
pub const MONITOR_CONTINUITY_RECORD_KIND: &str =
    "shell_runtime_adaptation_beta_monitor_continuity_record";

/// Stable record kind for [`ForegroundProtectionRow`] payloads.
pub const FOREGROUND_PROTECTION_RECORD_KIND: &str =
    "shell_runtime_adaptation_beta_foreground_protection_record";

/// Stable record kind for [`DesktopMatrixRow`] payloads.
pub const DESKTOP_MATRIX_RECORD_KIND: &str = "shell_runtime_adaptation_beta_desktop_matrix_record";

/// Host operating-system row admitted to the beta desktop matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostOsClass {
    /// Apple macOS desktop row.
    Macos,
    /// Microsoft Windows desktop row.
    Windows,
    /// Linux desktop row (Wayland or X11).
    Linux,
}

impl HostOsClass {
    /// Returns the stable schema token for this host OS class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }
}

/// Power posture sampled from the efficiency-state runtime hook.
///
/// The values mirror the vocabulary used by [`crate::efficiency`] so
/// support and benchmark evidence can be cross-referenced without
/// re-deriving the posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerPostureClass {
    /// Host is on AC power with no material pressure.
    AcPower,
    /// Host is on battery with normal headroom.
    Battery,
    /// Battery is low; optional work should be reduced.
    LowBattery,
    /// Battery is critical; only core interaction remains funded.
    CriticalBattery,
    /// Operating-system battery saver or user low-power mode is active.
    OsBatterySaver,
    /// Host is under thermal pressure.
    ThermalPressure,
    /// Pressure cleared; deferred work is resuming in stages.
    Recovery,
}

impl PowerPostureClass {
    /// Returns the stable schema token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AcPower => "ac_power",
            Self::Battery => "battery",
            Self::LowBattery => "low_battery",
            Self::CriticalBattery => "critical_battery",
            Self::OsBatterySaver => "os_battery_saver",
            Self::ThermalPressure => "thermal_pressure",
            Self::Recovery => "recovery",
        }
    }

    /// True when the posture is expected to expose a degraded badge in
    /// the shell status strip and a "background work paused/degraded"
    /// summary on the page.
    pub const fn background_work_must_degrade(self) -> bool {
        matches!(
            self,
            Self::LowBattery | Self::CriticalBattery | Self::OsBatterySaver | Self::ThermalPressure
        )
    }
}

/// Workload family whose budget is shaped by a power posture decision.
///
/// The tokens map onto [`crate::efficiency::WorkloadFamily`] so the
/// beta projection does not duplicate the underlying classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackgroundWorkloadClass {
    /// AI warmups and background model preparation.
    AiWarmup,
    /// Speculative prefetch and cache widening.
    SpeculativePrefetch,
    /// Uploads, replication, and deferred transfer.
    UploadTransfer,
    /// Non-essential motion and decorative animation.
    NonEssentialAnimation,
    /// Indexing, semantic refresh, and workspace scan.
    IndexingRefresh,
    /// Extension timers, polling, and background refresh.
    ExtensionPolling,
    /// Preview, browser-runtime, and canvas refresh.
    PreviewRefresh,
    /// Graph enrichment and non-hot-set semantic widening.
    GraphEnrichment,
    /// Remote reconnect, heartbeat, and session helper work.
    RemoteSessionHelper,
}

impl BackgroundWorkloadClass {
    /// Returns the stable schema token for this workload class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiWarmup => "ai_warmup",
            Self::SpeculativePrefetch => "speculative_prefetch",
            Self::UploadTransfer => "upload_transfer",
            Self::NonEssentialAnimation => "non_essential_animation",
            Self::IndexingRefresh => "indexing_refresh",
            Self::ExtensionPolling => "extension_polling",
            Self::PreviewRefresh => "preview_refresh",
            Self::GraphEnrichment => "graph_enrichment",
            Self::RemoteSessionHelper => "remote_session_helper",
        }
    }
}

/// Decision class applied to a background workload at the current posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackgroundWorkloadDecisionClass {
    /// Workload continues at its published budget.
    KeepRunning,
    /// Workload continues but at a reduced rate or quality.
    Degrade,
    /// Workload pauses and will resume when posture clears.
    Pause,
    /// Workload is denied for the duration of the posture.
    Deny,
}

impl BackgroundWorkloadDecisionClass {
    /// Returns the stable schema token for this decision class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeepRunning => "keep_running",
            Self::Degrade => "degrade",
            Self::Pause => "pause",
            Self::Deny => "deny",
        }
    }
}

/// Foreground action protected at every posture. Editing the active
/// buffer, opening the command palette, and direct navigation between
/// surfaces must remain responsive even when background work is paused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedForegroundAction {
    /// Editing the active buffer (insert, delete, undo, save).
    EditActiveBuffer,
    /// Opening and dispatching from the command palette.
    OpenCommandPalette,
    /// Direct navigation to a known target (go-to-file, go-to-symbol).
    DirectNavigation,
}

impl ProtectedForegroundAction {
    /// Returns the stable schema token for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditActiveBuffer => "edit_active_buffer",
            Self::OpenCommandPalette => "open_command_palette",
            Self::DirectNavigation => "direct_navigation",
        }
    }
}

/// Single background workload decision for the current posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackgroundWorkloadDecision {
    /// Workload class the decision applies to.
    pub workload: BackgroundWorkloadClass,
    /// Decision class applied at this posture.
    pub decision: BackgroundWorkloadDecisionClass,
    /// Human-readable note recorded in support evidence.
    pub note: String,
}

/// Posture row describing background pause/degrade and foreground protection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerPostureRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta runtime-adaptation schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Power posture this row covers.
    pub posture: PowerPostureClass,
    /// Background workload decisions for this posture.
    pub workload_decisions: Vec<BackgroundWorkloadDecision>,
    /// Protected foreground actions that must remain responsive.
    pub protected_foreground_actions: Vec<ProtectedForegroundAction>,
    /// True when foreground responsiveness is preserved at this posture.
    pub foreground_responsiveness_preserved: bool,
    /// True when the user sees an explicit posture badge in the shell
    /// status strip and an "adapted" summary in the activity center.
    pub user_visible_posture_summary_required: bool,
    /// Human-readable narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Continuity summary class admitted by a suspend-resume or monitor event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuitySummaryClass {
    /// Local non-credentialed work continues; remote authority must be
    /// re-established before privileged or mutating work runs again.
    Reconnect,
    /// The cached view is older than the source of truth; an explicit
    /// refresh is required before privileged or mutating work runs again.
    Stale,
    /// Live work was paused at the boundary and a "resume" prompt
    /// brings it back without a silent rerun.
    ResumedWork,
}

impl ContinuitySummaryClass {
    /// Returns the stable schema token for this continuity summary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reconnect => "reconnect",
            Self::Stale => "stale",
            Self::ResumedWork => "resumed_work",
        }
    }
}

/// Suspend-resume lifecycle event class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuspendResumeEventClass {
    /// Host went to sleep and the shell paused its event loop.
    Sleep,
    /// Host woke from sleep and the shell event loop resumed.
    WakeFromSleep,
    /// Host hibernated and the shell process was paged out.
    Hibernate,
    /// Host resumed from hibernation; the process was paged back in.
    ResumeFromHibernate,
    /// Operating-system lock/unlock cycle interrupted the foreground.
    LockUnlockCycle,
}

impl SuspendResumeEventClass {
    /// Returns the stable schema token for this event class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sleep => "sleep",
            Self::WakeFromSleep => "wake_from_sleep",
            Self::Hibernate => "hibernate",
            Self::ResumeFromHibernate => "resume_from_hibernate",
            Self::LockUnlockCycle => "lock_unlock_cycle",
        }
    }
}

/// Row describing a suspend-resume cycle and the continuity summary it
/// produces. The named summary tokens are mirrored in the support
/// export so reviewers never have to scrape rendered prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspendResumeRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta runtime-adaptation schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Host OS this row applies to.
    pub host_os: HostOsClass,
    /// Lifecycle event class.
    pub event: SuspendResumeEventClass,
    /// Continuity summary classes emitted for the event.
    pub continuity_summary: Vec<ContinuitySummaryClass>,
    /// True when local non-credentialed work continues across the cycle.
    pub local_work_continues: bool,
    /// True when privileged or mutating work was paused at the boundary.
    pub privileged_or_mutating_work_paused: bool,
    /// True when no silent rerun or authority reacquisition is allowed.
    pub no_silent_rerun_or_authority_reuse: bool,
    /// True when the user-visible resume summary is required.
    pub user_visible_resume_summary_required: bool,
    /// Recovery action tokens surfaced to the user (e.g. continue local,
    /// reconnect, reauthenticate, refresh stale view, resume work).
    pub recovery_action_tokens: Vec<String>,
    /// Human-readable narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Multi-monitor topology event class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonitorContinuityEventClass {
    /// One or more displays disconnected.
    DisplayDisconnected,
    /// One or more displays reconnected.
    DisplayReconnected,
    /// Display scale or DPI bucket changed.
    ScaleChanged,
    /// Display arrangement or primary changed.
    ArrangementChanged,
    /// Dock or undock cycle altered the display topology.
    DockUndockCycle,
}

impl MonitorContinuityEventClass {
    /// Returns the stable schema token for this event class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisplayDisconnected => "display_disconnected",
            Self::DisplayReconnected => "display_reconnected",
            Self::ScaleChanged => "scale_changed",
            Self::ArrangementChanged => "arrangement_changed",
            Self::DockUndockCycle => "dock_undock_cycle",
        }
    }
}

/// Row describing a multi-monitor continuity event and how the shell
/// reconciled the topology change. Restore-fidelity tokens mirror the
/// vocabulary used by [`crate::windows`] so a reviewer can pivot
/// between the windows-beta page and the runtime-adaptation page on
/// the same `case_id` when both are touched by one event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorContinuityRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta runtime-adaptation schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Host OS this row applies to.
    pub host_os: HostOsClass,
    /// Event class.
    pub event: MonitorContinuityEventClass,
    /// Continuity summary classes emitted for the event.
    pub continuity_summary: Vec<ContinuitySummaryClass>,
    /// Resulting restore fidelity token (mirrors `windows` beta vocabulary).
    pub resulting_fidelity_token: String,
    /// True when window placement was downgraded from exact geometry.
    pub topology_adjustment_downgraded_fidelity: bool,
    /// True when visible bounds are preserved after the adjustment.
    pub visible_bounds_preserved: bool,
    /// True when keyboard focus and the dominant pane intent survive.
    pub focus_intent_preserved: bool,
    /// True when the user-visible "topology changed" summary is required.
    pub user_visible_topology_summary_required: bool,
    /// Recovery action tokens surfaced to the user.
    pub recovery_action_tokens: Vec<String>,
    /// Optional ref to a `windows_beta` restore-topology outcome that
    /// shares the same topology event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows_beta_outcome_ref: Option<String>,
    /// Human-readable narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Row that pins the protected-foreground promise at a specific posture
/// and event class. The matrix is small on purpose: editing, palette,
/// and direct navigation must all stay responsive — anything less is a
/// contract bug.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForegroundProtectionRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta runtime-adaptation schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Host OS this row applies to.
    pub host_os: HostOsClass,
    /// Posture for the row.
    pub posture: PowerPostureClass,
    /// Protected actions covered by the row.
    pub protected_actions: Vec<ProtectedForegroundAction>,
    /// True when the action remains responsive at the posture.
    pub remains_responsive: bool,
    /// True when no input-event drops are admitted.
    pub no_input_event_drops: bool,
    /// True when no opaque blocking dialog is allowed to fire while
    /// the protected action runs.
    pub no_blocking_dialog: bool,
    /// Human-readable narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Per-OS desktop matrix row that names the downgrade behavior surfaced
/// to beta users on macOS, Windows, and Linux.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopMatrixRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta runtime-adaptation schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Host OS this row covers.
    pub host_os: HostOsClass,
    /// Suspend-resume row refs exercised on this OS row.
    pub suspend_resume_refs: Vec<String>,
    /// Monitor continuity row refs exercised on this OS row.
    pub monitor_continuity_refs: Vec<String>,
    /// Posture row refs exercised on this OS row.
    pub posture_refs: Vec<String>,
    /// Named downgrade behaviors surfaced to beta users on this OS.
    pub named_downgrade_behaviors: Vec<String>,
    /// True when foreground responsiveness held across the exercised rows.
    pub foreground_responsiveness_held: bool,
    /// True when no silent rerun or authority reuse was admitted.
    pub no_silent_rerun_or_authority_reuse: bool,
    /// Human-readable narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Aggregate summary banner for the beta runtime-adaptation page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RuntimeAdaptationSummary {
    /// Number of power-posture rows on the page.
    pub posture_row_count: usize,
    /// Number of suspend-resume rows on the page.
    pub suspend_resume_row_count: usize,
    /// Number of monitor-continuity rows on the page.
    pub monitor_continuity_row_count: usize,
    /// Number of foreground-protection rows on the page.
    pub foreground_protection_row_count: usize,
    /// Number of desktop-matrix rows on the page.
    pub desktop_matrix_row_count: usize,
    /// Number of postures admitting `pause` or `degrade` decisions.
    pub postures_with_background_degraded: usize,
    /// Number of continuity events requiring a user-visible summary.
    pub continuity_events_with_user_summary: usize,
    /// Number of OS rows admitting any named downgrade behavior.
    pub os_rows_with_named_downgrade: usize,
}

impl RuntimeAdaptationSummary {
    fn from_lists(
        postures: &[PowerPostureRow],
        suspends: &[SuspendResumeRow],
        monitors: &[MonitorContinuityRow],
        foreground: &[ForegroundProtectionRow],
        matrix: &[DesktopMatrixRow],
    ) -> Self {
        let postures_with_background_degraded = postures
            .iter()
            .filter(|row| {
                row.workload_decisions.iter().any(|decision| {
                    matches!(
                        decision.decision,
                        BackgroundWorkloadDecisionClass::Pause
                            | BackgroundWorkloadDecisionClass::Degrade
                            | BackgroundWorkloadDecisionClass::Deny
                    )
                })
            })
            .count();
        let continuity_events_with_user_summary = suspends
            .iter()
            .filter(|row| row.user_visible_resume_summary_required)
            .count()
            + monitors
                .iter()
                .filter(|row| row.user_visible_topology_summary_required)
                .count();
        let os_rows_with_named_downgrade = matrix
            .iter()
            .filter(|row| !row.named_downgrade_behaviors.is_empty())
            .count();
        Self {
            posture_row_count: postures.len(),
            suspend_resume_row_count: suspends.len(),
            monitor_continuity_row_count: monitors.len(),
            foreground_protection_row_count: foreground.len(),
            desktop_matrix_row_count: matrix.len(),
            postures_with_background_degraded,
            continuity_events_with_user_summary,
            os_rows_with_named_downgrade,
        }
    }
}

/// Top-level beta runtime-adaptation page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAdaptationPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta runtime-adaptation schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Page label for chrome.
    pub page_label: String,
    /// Aggregate summary banner.
    pub summary: RuntimeAdaptationSummary,
    /// Power-posture rows on the page.
    pub power_posture_rows: Vec<PowerPostureRow>,
    /// Suspend-resume rows on the page.
    pub suspend_resume_rows: Vec<SuspendResumeRow>,
    /// Monitor-continuity rows on the page.
    pub monitor_continuity_rows: Vec<MonitorContinuityRow>,
    /// Foreground-protection rows on the page.
    pub foreground_protection_rows: Vec<ForegroundProtectionRow>,
    /// Per-OS desktop matrix rows on the page.
    pub desktop_matrix_rows: Vec<DesktopMatrixRow>,
}

impl RuntimeAdaptationPage {
    /// Construct a runtime-adaptation page from the five row lists.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        power_posture_rows: Vec<PowerPostureRow>,
        suspend_resume_rows: Vec<SuspendResumeRow>,
        monitor_continuity_rows: Vec<MonitorContinuityRow>,
        foreground_protection_rows: Vec<ForegroundProtectionRow>,
        desktop_matrix_rows: Vec<DesktopMatrixRow>,
    ) -> Self {
        let summary = RuntimeAdaptationSummary::from_lists(
            &power_posture_rows,
            &suspend_resume_rows,
            &monitor_continuity_rows,
            &foreground_protection_rows,
            &desktop_matrix_rows,
        );
        Self {
            record_kind: RUNTIME_ADAPTATION_PAGE_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
            shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            summary,
            power_posture_rows,
            suspend_resume_rows,
            monitor_continuity_rows,
            foreground_protection_rows,
            desktop_matrix_rows,
        }
    }
}

/// Support-export wrapper for the runtime-adaptation page. Quotes the
/// page plus every case id in stable order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAdaptationSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta runtime-adaptation schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Embedded page.
    pub page: RuntimeAdaptationPage,
    /// All case ids exposed by the page, in stable order.
    pub case_ids: Vec<String>,
}

impl RuntimeAdaptationSupportExport {
    /// Build a support-export wrapper from a runtime-adaptation page.
    pub fn from_page(export_id: impl Into<String>, page: RuntimeAdaptationPage) -> Self {
        let mut case_ids: Vec<String> = Vec::new();
        for record in &page.power_posture_rows {
            case_ids.push(record.case_id.clone());
        }
        for record in &page.suspend_resume_rows {
            case_ids.push(record.case_id.clone());
        }
        for record in &page.monitor_continuity_rows {
            case_ids.push(record.case_id.clone());
        }
        for record in &page.foreground_protection_rows {
            case_ids.push(record.case_id.clone());
        }
        for record in &page.desktop_matrix_rows {
            case_ids.push(record.case_id.clone());
        }
        Self {
            record_kind: RUNTIME_ADAPTATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
            shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            page,
            case_ids,
        }
    }
}

/// Validation errors raised when the runtime-adaptation page fails an
/// acceptance invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeAdaptationValidationError {
    /// A pressured posture admitted neither a paused nor degraded
    /// background workload, so background work cannot be observed to
    /// degrade under power/thermal pressure.
    PressuredPostureWithoutDegradedWorkload {
        /// Posture case id.
        case_id: String,
        /// Posture token.
        posture: String,
    },
    /// A posture row dropped foreground responsiveness.
    ForegroundResponsivenessDropped {
        /// Posture case id.
        case_id: String,
        /// Posture token.
        posture: String,
    },
    /// A posture row omitted one of the protected foreground actions.
    ProtectedForegroundActionMissing {
        /// Posture case id.
        case_id: String,
        /// Missing action token.
        action: String,
    },
    /// A suspend-resume row admitted a silent rerun or hidden
    /// authority reacquisition.
    SuspendResumeSilentRerun {
        /// Suspend-resume case id.
        case_id: String,
    },
    /// A suspend-resume row dropped the user-visible resume summary.
    SuspendResumeMissingSummary {
        /// Suspend-resume case id.
        case_id: String,
    },
    /// A monitor row dropped visible bounds or focus intent.
    MonitorContinuityBoundsOrFocusDropped {
        /// Monitor case id.
        case_id: String,
    },
    /// A monitor row dropped the user-visible topology summary.
    MonitorContinuityMissingSummary {
        /// Monitor case id.
        case_id: String,
    },
    /// A foreground-protection row admitted unresponsive, dropped, or
    /// blocked behavior on a protected action.
    ForegroundProtectionViolation {
        /// Foreground-protection case id.
        case_id: String,
    },
    /// A desktop-matrix OS row carried no named downgrade behavior, so
    /// the matrix is doc-only on that OS.
    DesktopMatrixMissingNamedDowngrade {
        /// Desktop matrix case id.
        case_id: String,
        /// Host OS token.
        host_os: String,
    },
    /// A desktop-matrix OS row admitted silent rerun or dropped
    /// foreground responsiveness.
    DesktopMatrixUnsafe {
        /// Desktop matrix case id.
        case_id: String,
    },
    /// The matrix did not cover macOS, Windows, or Linux.
    DesktopMatrixIncomplete {
        /// Missing OS token.
        missing_os: String,
    },
}

impl std::fmt::Display for RuntimeAdaptationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PressuredPostureWithoutDegradedWorkload { case_id, posture } => write!(
                f,
                "posture {case_id} ({posture}) admitted no paused or degraded workload"
            ),
            Self::ForegroundResponsivenessDropped { case_id, posture } => write!(
                f,
                "posture {case_id} ({posture}) dropped foreground responsiveness"
            ),
            Self::ProtectedForegroundActionMissing { case_id, action } => write!(
                f,
                "posture {case_id} did not promise the protected foreground action {action}"
            ),
            Self::SuspendResumeSilentRerun { case_id } => write!(
                f,
                "suspend-resume row {case_id} admitted a silent rerun or hidden authority reuse"
            ),
            Self::SuspendResumeMissingSummary { case_id } => write!(
                f,
                "suspend-resume row {case_id} dropped the user-visible resume summary"
            ),
            Self::MonitorContinuityBoundsOrFocusDropped { case_id } => write!(
                f,
                "monitor row {case_id} dropped visible bounds or focus intent"
            ),
            Self::MonitorContinuityMissingSummary { case_id } => write!(
                f,
                "monitor row {case_id} dropped the user-visible topology summary"
            ),
            Self::ForegroundProtectionViolation { case_id } => write!(
                f,
                "foreground-protection row {case_id} admitted a dropped or blocked action"
            ),
            Self::DesktopMatrixMissingNamedDowngrade { case_id, host_os } => write!(
                f,
                "desktop matrix {case_id} ({host_os}) carries no named downgrade behavior"
            ),
            Self::DesktopMatrixUnsafe { case_id } => write!(
                f,
                "desktop matrix {case_id} admitted silent rerun or dropped foreground responsiveness"
            ),
            Self::DesktopMatrixIncomplete { missing_os } => write!(
                f,
                "desktop matrix did not cover required OS row {missing_os}"
            ),
        }
    }
}

impl std::error::Error for RuntimeAdaptationValidationError {}

/// Validate the acceptance invariants on a beta runtime-adaptation page.
pub fn validate_runtime_adaptation_page(
    page: &RuntimeAdaptationPage,
) -> Result<(), Vec<RuntimeAdaptationValidationError>> {
    let mut errors: Vec<RuntimeAdaptationValidationError> = Vec::new();

    for posture in &page.power_posture_rows {
        if posture.posture.background_work_must_degrade() {
            let degraded = posture.workload_decisions.iter().any(|decision| {
                matches!(
                    decision.decision,
                    BackgroundWorkloadDecisionClass::Pause
                        | BackgroundWorkloadDecisionClass::Degrade
                        | BackgroundWorkloadDecisionClass::Deny
                )
            });
            if !degraded {
                errors.push(
                    RuntimeAdaptationValidationError::PressuredPostureWithoutDegradedWorkload {
                        case_id: posture.case_id.clone(),
                        posture: posture.posture.as_str().to_owned(),
                    },
                );
            }
        }
        if !posture.foreground_responsiveness_preserved {
            errors.push(
                RuntimeAdaptationValidationError::ForegroundResponsivenessDropped {
                    case_id: posture.case_id.clone(),
                    posture: posture.posture.as_str().to_owned(),
                },
            );
        }
        for required in [
            ProtectedForegroundAction::EditActiveBuffer,
            ProtectedForegroundAction::OpenCommandPalette,
            ProtectedForegroundAction::DirectNavigation,
        ] {
            if !posture.protected_foreground_actions.contains(&required) {
                errors.push(
                    RuntimeAdaptationValidationError::ProtectedForegroundActionMissing {
                        case_id: posture.case_id.clone(),
                        action: required.as_str().to_owned(),
                    },
                );
            }
        }
    }

    for row in &page.suspend_resume_rows {
        if !row.no_silent_rerun_or_authority_reuse {
            errors.push(RuntimeAdaptationValidationError::SuspendResumeSilentRerun {
                case_id: row.case_id.clone(),
            });
        }
        if !row.user_visible_resume_summary_required {
            errors.push(
                RuntimeAdaptationValidationError::SuspendResumeMissingSummary {
                    case_id: row.case_id.clone(),
                },
            );
        }
    }

    for row in &page.monitor_continuity_rows {
        if !row.visible_bounds_preserved || !row.focus_intent_preserved {
            errors.push(
                RuntimeAdaptationValidationError::MonitorContinuityBoundsOrFocusDropped {
                    case_id: row.case_id.clone(),
                },
            );
        }
        if !row.user_visible_topology_summary_required {
            errors.push(
                RuntimeAdaptationValidationError::MonitorContinuityMissingSummary {
                    case_id: row.case_id.clone(),
                },
            );
        }
    }

    for row in &page.foreground_protection_rows {
        if !row.remains_responsive || !row.no_input_event_drops || !row.no_blocking_dialog {
            errors.push(
                RuntimeAdaptationValidationError::ForegroundProtectionViolation {
                    case_id: row.case_id.clone(),
                },
            );
        }
    }

    let mut covered_os: Vec<HostOsClass> = Vec::new();
    for row in &page.desktop_matrix_rows {
        covered_os.push(row.host_os);
        if row.named_downgrade_behaviors.is_empty() {
            errors.push(
                RuntimeAdaptationValidationError::DesktopMatrixMissingNamedDowngrade {
                    case_id: row.case_id.clone(),
                    host_os: row.host_os.as_str().to_owned(),
                },
            );
        }
        if !row.foreground_responsiveness_held || !row.no_silent_rerun_or_authority_reuse {
            errors.push(RuntimeAdaptationValidationError::DesktopMatrixUnsafe {
                case_id: row.case_id.clone(),
            });
        }
    }
    for required in [HostOsClass::Macos, HostOsClass::Windows, HostOsClass::Linux] {
        if !covered_os.contains(&required) {
            errors.push(RuntimeAdaptationValidationError::DesktopMatrixIncomplete {
                missing_os: required.as_str().to_owned(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seeded fixture builder used by the headless inspector bin and the
/// integration test. The seed mirrors the JSON checked in under
/// `fixtures/ux/m3/desktop_continuity_matrix/` so the live shell, the
/// review packet, and the support export report the same
/// runtime-adaptation truth.
pub fn seeded_runtime_adaptation_page() -> RuntimeAdaptationPage {
    let protected_actions = vec![
        ProtectedForegroundAction::EditActiveBuffer,
        ProtectedForegroundAction::OpenCommandPalette,
        ProtectedForegroundAction::DirectNavigation,
    ];

    let posture_ac = PowerPostureRow {
        record_kind: POWER_POSTURE_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:posture:ac-power:01".to_owned(),
        posture: PowerPostureClass::AcPower,
        workload_decisions: vec![
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::IndexingRefresh,
                decision: BackgroundWorkloadDecisionClass::KeepRunning,
                note: "Indexing runs at published budget on AC power.".to_owned(),
            },
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::AiWarmup,
                decision: BackgroundWorkloadDecisionClass::KeepRunning,
                note: "AI warmups continue at published budget on AC power.".to_owned(),
            },
        ],
        protected_foreground_actions: protected_actions.clone(),
        foreground_responsiveness_preserved: true,
        user_visible_posture_summary_required: false,
        narrative: "AC power; ordinary published budgets apply and the shell does not surface an adapted-posture summary.".to_owned(),
    };

    let posture_low_battery = PowerPostureRow {
        record_kind: POWER_POSTURE_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:posture:low-battery:01".to_owned(),
        posture: PowerPostureClass::LowBattery,
        workload_decisions: vec![
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::SpeculativePrefetch,
                decision: BackgroundWorkloadDecisionClass::Pause,
                note: "Speculative prefetch pauses until posture clears.".to_owned(),
            },
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::IndexingRefresh,
                decision: BackgroundWorkloadDecisionClass::Degrade,
                note: "Indexing refresh continues at a reduced rate.".to_owned(),
            },
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::NonEssentialAnimation,
                decision: BackgroundWorkloadDecisionClass::Pause,
                note: "Non-essential animation pauses to preserve interaction headroom.".to_owned(),
            },
        ],
        protected_foreground_actions: protected_actions.clone(),
        foreground_responsiveness_preserved: true,
        user_visible_posture_summary_required: true,
        narrative: "Battery is low; background work pauses or degrades and the shell surfaces an adapted-posture summary in the activity center.".to_owned(),
    };

    let posture_thermal = PowerPostureRow {
        record_kind: POWER_POSTURE_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:posture:thermal-pressure:01".to_owned(),
        posture: PowerPostureClass::ThermalPressure,
        workload_decisions: vec![
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::AiWarmup,
                decision: BackgroundWorkloadDecisionClass::Pause,
                note: "AI warmups pause while the host is under thermal pressure.".to_owned(),
            },
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::PreviewRefresh,
                decision: BackgroundWorkloadDecisionClass::Degrade,
                note: "Preview refresh ticks at a reduced cadence.".to_owned(),
            },
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::GraphEnrichment,
                decision: BackgroundWorkloadDecisionClass::Pause,
                note: "Graph enrichment pauses until pressure clears.".to_owned(),
            },
        ],
        protected_foreground_actions: protected_actions.clone(),
        foreground_responsiveness_preserved: true,
        user_visible_posture_summary_required: true,
        narrative: "Host is under thermal pressure; the shell pauses speculative work and surfaces an adapted-posture summary while protected foreground actions stay responsive.".to_owned(),
    };

    let posture_critical = PowerPostureRow {
        record_kind: POWER_POSTURE_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:posture:critical-battery:01".to_owned(),
        posture: PowerPostureClass::CriticalBattery,
        workload_decisions: vec![
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::UploadTransfer,
                decision: BackgroundWorkloadDecisionClass::Pause,
                note: "Uploads pause; the user is prompted to plug in before resuming.".to_owned(),
            },
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::ExtensionPolling,
                decision: BackgroundWorkloadDecisionClass::Deny,
                note: "Extension background polling is denied at critical battery.".to_owned(),
            },
            BackgroundWorkloadDecision {
                workload: BackgroundWorkloadClass::IndexingRefresh,
                decision: BackgroundWorkloadDecisionClass::Pause,
                note: "Indexing refresh pauses until the host is back on AC power.".to_owned(),
            },
        ],
        protected_foreground_actions: protected_actions.clone(),
        foreground_responsiveness_preserved: true,
        user_visible_posture_summary_required: true,
        narrative: "Battery is critical; only core interaction is funded and the user-visible posture summary explains what was paused or denied.".to_owned(),
    };

    let suspend_macos_wake = SuspendResumeRow {
        record_kind: SUSPEND_RESUME_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:suspend-resume:macos-wake:01".to_owned(),
        host_os: HostOsClass::Macos,
        event: SuspendResumeEventClass::WakeFromSleep,
        continuity_summary: vec![
            ContinuitySummaryClass::Reconnect,
            ContinuitySummaryClass::ResumedWork,
        ],
        local_work_continues: true,
        privileged_or_mutating_work_paused: true,
        no_silent_rerun_or_authority_reuse: true,
        user_visible_resume_summary_required: true,
        recovery_action_tokens: vec![
            "continue_local".to_owned(),
            "reconnect".to_owned(),
            "reauthenticate".to_owned(),
            "resume_paused_work".to_owned(),
        ],
        narrative: "macOS wake from sleep: the local editor buffer and palette are immediately responsive; remote authority is held until reconnect; paused background work surfaces a resume prompt.".to_owned(),
    };

    let suspend_windows_lock = SuspendResumeRow {
        record_kind: SUSPEND_RESUME_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:suspend-resume:windows-lock-unlock:01".to_owned(),
        host_os: HostOsClass::Windows,
        event: SuspendResumeEventClass::LockUnlockCycle,
        continuity_summary: vec![ContinuitySummaryClass::Stale],
        local_work_continues: true,
        privileged_or_mutating_work_paused: true,
        no_silent_rerun_or_authority_reuse: true,
        user_visible_resume_summary_required: true,
        recovery_action_tokens: vec![
            "continue_local".to_owned(),
            "refresh_stale_view".to_owned(),
            "reauthenticate".to_owned(),
        ],
        narrative: "Windows lock/unlock cycle: cached views are tagged stale on unlock; the user sees a refresh prompt before any privileged or mutating work is rerun.".to_owned(),
    };

    let suspend_linux_hibernate = SuspendResumeRow {
        record_kind: SUSPEND_RESUME_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:suspend-resume:linux-hibernate-resume:01".to_owned(),
        host_os: HostOsClass::Linux,
        event: SuspendResumeEventClass::ResumeFromHibernate,
        continuity_summary: vec![
            ContinuitySummaryClass::Reconnect,
            ContinuitySummaryClass::Stale,
        ],
        local_work_continues: true,
        privileged_or_mutating_work_paused: true,
        no_silent_rerun_or_authority_reuse: true,
        user_visible_resume_summary_required: true,
        recovery_action_tokens: vec![
            "continue_local".to_owned(),
            "reconnect".to_owned(),
            "refresh_stale_view".to_owned(),
            "reauthenticate".to_owned(),
            "export_restore_details".to_owned(),
        ],
        narrative: "Linux resume from hibernate: cached views are tagged stale and remote authority is held until reconnect; the user-visible resume summary explains the held actions.".to_owned(),
    };

    let monitor_macos_detach = MonitorContinuityRow {
        record_kind: MONITOR_CONTINUITY_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:monitor-continuity:macos-detach:01".to_owned(),
        host_os: HostOsClass::Macos,
        event: MonitorContinuityEventClass::DisplayDisconnected,
        continuity_summary: vec![ContinuitySummaryClass::ResumedWork],
        resulting_fidelity_token: "compatible_restore".to_owned(),
        topology_adjustment_downgraded_fidelity: true,
        visible_bounds_preserved: true,
        focus_intent_preserved: true,
        user_visible_topology_summary_required: true,
        recovery_action_tokens: vec![
            "recenter_window".to_owned(),
            "review_layout".to_owned(),
            "restore_detached_window".to_owned(),
        ],
        windows_beta_outcome_ref: Some(
            "shell:windows-beta:restore:display-detach-safe-bounds:01".to_owned(),
        ),
        narrative: "macOS external display detached: stranded windows snap to the primary display's safe bounds; the user sees a layout-adjusted summary and an explicit restore-detached-window action.".to_owned(),
    };

    let monitor_windows_dock = MonitorContinuityRow {
        record_kind: MONITOR_CONTINUITY_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:monitor-continuity:windows-dock-undock:01".to_owned(),
        host_os: HostOsClass::Windows,
        event: MonitorContinuityEventClass::DockUndockCycle,
        continuity_summary: vec![ContinuitySummaryClass::ResumedWork],
        resulting_fidelity_token: "compatible_restore".to_owned(),
        topology_adjustment_downgraded_fidelity: true,
        visible_bounds_preserved: true,
        focus_intent_preserved: true,
        user_visible_topology_summary_required: true,
        recovery_action_tokens: vec![
            "exit_fullscreen_or_snapped_mode".to_owned(),
            "review_layout".to_owned(),
            "keep_current_pane_visible".to_owned(),
        ],
        windows_beta_outcome_ref: Some(
            "shell:windows-beta:restore:dock-undock-recovery-chrome:01".to_owned(),
        ),
        narrative: "Windows dock/undock: fullscreen clears and bounds clamp into the current safe region before focus is routed; the activity center shows a topology-changed summary.".to_owned(),
    };

    let monitor_linux_mixed_dpi = MonitorContinuityRow {
        record_kind: MONITOR_CONTINUITY_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:monitor-continuity:linux-mixed-dpi:01".to_owned(),
        host_os: HostOsClass::Linux,
        event: MonitorContinuityEventClass::ScaleChanged,
        continuity_summary: vec![ContinuitySummaryClass::ResumedWork],
        resulting_fidelity_token: "compatible_restore".to_owned(),
        topology_adjustment_downgraded_fidelity: true,
        visible_bounds_preserved: true,
        focus_intent_preserved: true,
        user_visible_topology_summary_required: true,
        recovery_action_tokens: vec![
            "recenter_window".to_owned(),
            "review_layout".to_owned(),
        ],
        windows_beta_outcome_ref: Some(
            "shell:windows-beta:restore:mixed-dpi-normalized:01".to_owned(),
        ),
        narrative: "Linux mixed-DPI: scale normalization runs before focus and keyboard routing; the layout-adjusted summary stays in the activity center until the user acknowledges.".to_owned(),
    };

    let foreground_macos_low_battery = ForegroundProtectionRow {
        record_kind: FOREGROUND_PROTECTION_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:foreground:macos-low-battery:01".to_owned(),
        host_os: HostOsClass::Macos,
        posture: PowerPostureClass::LowBattery,
        protected_actions: protected_actions.clone(),
        remains_responsive: true,
        no_input_event_drops: true,
        no_blocking_dialog: true,
        narrative: "macOS at low battery: editing, palette, and direct navigation remain responsive; no blocking dialog fires while the posture is active.".to_owned(),
    };

    let foreground_windows_thermal = ForegroundProtectionRow {
        record_kind: FOREGROUND_PROTECTION_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:foreground:windows-thermal:01".to_owned(),
        host_os: HostOsClass::Windows,
        posture: PowerPostureClass::ThermalPressure,
        protected_actions: protected_actions.clone(),
        remains_responsive: true,
        no_input_event_drops: true,
        no_blocking_dialog: true,
        narrative: "Windows under thermal pressure: editing, palette, and direct navigation remain responsive while speculative work is paused or degraded.".to_owned(),
    };

    let foreground_linux_resume = ForegroundProtectionRow {
        record_kind: FOREGROUND_PROTECTION_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:foreground:linux-post-resume:01".to_owned(),
        host_os: HostOsClass::Linux,
        posture: PowerPostureClass::Recovery,
        protected_actions: protected_actions.clone(),
        remains_responsive: true,
        no_input_event_drops: true,
        no_blocking_dialog: true,
        narrative: "Linux immediately after resume-from-hibernate: editing, palette, and direct navigation are responsive before background reconnect finishes.".to_owned(),
    };

    let matrix_macos = DesktopMatrixRow {
        record_kind: DESKTOP_MATRIX_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:desktop-matrix:macos:01".to_owned(),
        host_os: HostOsClass::Macos,
        suspend_resume_refs: vec![suspend_macos_wake.case_id.clone()],
        monitor_continuity_refs: vec![monitor_macos_detach.case_id.clone()],
        posture_refs: vec![
            posture_low_battery.case_id.clone(),
            posture_thermal.case_id.clone(),
            posture_critical.case_id.clone(),
        ],
        named_downgrade_behaviors: vec![
            "speculative_prefetch_paused".to_owned(),
            "ai_warmup_paused".to_owned(),
            "non_essential_animation_paused".to_owned(),
            "stranded_window_snapped_to_primary".to_owned(),
            "remote_authority_held_pending_reconnect".to_owned(),
        ],
        foreground_responsiveness_held: true,
        no_silent_rerun_or_authority_reuse: true,
        narrative: "macOS beta row: wake-from-sleep replays through the reconnect/resumed-work summaries; external-display detach snaps windows safely; low-battery, thermal, and critical postures degrade background work without dropping foreground responsiveness.".to_owned(),
    };

    let matrix_windows = DesktopMatrixRow {
        record_kind: DESKTOP_MATRIX_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:desktop-matrix:windows:01".to_owned(),
        host_os: HostOsClass::Windows,
        suspend_resume_refs: vec![suspend_windows_lock.case_id.clone()],
        monitor_continuity_refs: vec![monitor_windows_dock.case_id.clone()],
        posture_refs: vec![
            posture_low_battery.case_id.clone(),
            posture_thermal.case_id.clone(),
            posture_critical.case_id.clone(),
        ],
        named_downgrade_behaviors: vec![
            "speculative_prefetch_paused".to_owned(),
            "indexing_refresh_degraded".to_owned(),
            "extension_polling_denied".to_owned(),
            "fullscreen_cleared_on_dock_change".to_owned(),
            "cached_view_marked_stale_on_unlock".to_owned(),
        ],
        foreground_responsiveness_held: true,
        no_silent_rerun_or_authority_reuse: true,
        narrative: "Windows beta row: lock/unlock marks cached views stale; dock/undock clears fullscreen before routing focus; pressure postures pause speculative work and surface a posture summary in the activity center.".to_owned(),
    };

    let matrix_linux = DesktopMatrixRow {
        record_kind: DESKTOP_MATRIX_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_ADAPTATION_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_ADAPTATION_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:runtime-adaptation:desktop-matrix:linux:01".to_owned(),
        host_os: HostOsClass::Linux,
        suspend_resume_refs: vec![suspend_linux_hibernate.case_id.clone()],
        monitor_continuity_refs: vec![monitor_linux_mixed_dpi.case_id.clone()],
        posture_refs: vec![
            posture_low_battery.case_id.clone(),
            posture_thermal.case_id.clone(),
            posture_critical.case_id.clone(),
        ],
        named_downgrade_behaviors: vec![
            "speculative_prefetch_paused".to_owned(),
            "graph_enrichment_paused".to_owned(),
            "preview_refresh_degraded".to_owned(),
            "scale_normalized_before_focus".to_owned(),
            "remote_authority_held_pending_reconnect".to_owned(),
        ],
        foreground_responsiveness_held: true,
        no_silent_rerun_or_authority_reuse: true,
        narrative: "Linux beta row: hibernate/resume replays through reconnect and stale summaries; mixed-DPI changes normalize scale before focus routing; pressure postures pause graph enrichment and degrade preview refresh.".to_owned(),
    };

    RuntimeAdaptationPage::new(
        "all",
        "Desktop continuity (beta): suspend-resume, battery/thermal, multi-monitor",
        vec![
            posture_ac,
            posture_low_battery,
            posture_thermal,
            posture_critical,
        ],
        vec![
            suspend_macos_wake,
            suspend_windows_lock,
            suspend_linux_hibernate,
        ],
        vec![
            monitor_macos_detach,
            monitor_windows_dock,
            monitor_linux_mixed_dpi,
        ],
        vec![
            foreground_macos_low_battery,
            foreground_windows_thermal,
            foreground_linux_resume,
        ],
        vec![matrix_macos, matrix_windows, matrix_linux],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_passes_validation() {
        let page = seeded_runtime_adaptation_page();
        validate_runtime_adaptation_page(&page).expect("seeded page must validate");
    }

    #[test]
    fn seeded_page_summary_matches_record_counts() {
        let page = seeded_runtime_adaptation_page();
        assert_eq!(
            page.summary.posture_row_count,
            page.power_posture_rows.len()
        );
        assert_eq!(
            page.summary.suspend_resume_row_count,
            page.suspend_resume_rows.len()
        );
        assert_eq!(
            page.summary.monitor_continuity_row_count,
            page.monitor_continuity_rows.len()
        );
        assert_eq!(
            page.summary.foreground_protection_row_count,
            page.foreground_protection_rows.len()
        );
        assert_eq!(
            page.summary.desktop_matrix_row_count,
            page.desktop_matrix_rows.len()
        );
        assert_eq!(page.summary.os_rows_with_named_downgrade, 3);
        assert!(page.summary.postures_with_background_degraded >= 3);
        assert!(page.summary.continuity_events_with_user_summary >= 6);
    }

    #[test]
    fn validation_flags_pressured_posture_without_degraded_workload() {
        let mut page = seeded_runtime_adaptation_page();
        let row = page
            .power_posture_rows
            .iter_mut()
            .find(|row| matches!(row.posture, PowerPostureClass::LowBattery))
            .expect("seed has a low-battery posture");
        for decision in &mut row.workload_decisions {
            decision.decision = BackgroundWorkloadDecisionClass::KeepRunning;
        }
        let errors = validate_runtime_adaptation_page(&page)
            .expect_err("must flag pressured posture without degradation");
        assert!(errors.iter().any(|e| matches!(
            e,
            RuntimeAdaptationValidationError::PressuredPostureWithoutDegradedWorkload { .. }
        )));
    }

    #[test]
    fn validation_flags_foreground_drop() {
        let mut page = seeded_runtime_adaptation_page();
        page.power_posture_rows[0].foreground_responsiveness_preserved = false;
        let errors = validate_runtime_adaptation_page(&page)
            .expect_err("must flag foreground responsiveness drop");
        assert!(errors.iter().any(|e| matches!(
            e,
            RuntimeAdaptationValidationError::ForegroundResponsivenessDropped { .. }
        )));
    }

    #[test]
    fn validation_flags_silent_rerun() {
        let mut page = seeded_runtime_adaptation_page();
        page.suspend_resume_rows[0].no_silent_rerun_or_authority_reuse = false;
        let errors = validate_runtime_adaptation_page(&page).expect_err("must flag silent rerun");
        assert!(errors.iter().any(|e| matches!(
            e,
            RuntimeAdaptationValidationError::SuspendResumeSilentRerun { .. }
        )));
    }

    #[test]
    fn validation_flags_monitor_bounds_drop() {
        let mut page = seeded_runtime_adaptation_page();
        page.monitor_continuity_rows[0].visible_bounds_preserved = false;
        let errors =
            validate_runtime_adaptation_page(&page).expect_err("must flag monitor bounds drop");
        assert!(errors.iter().any(|e| matches!(
            e,
            RuntimeAdaptationValidationError::MonitorContinuityBoundsOrFocusDropped { .. }
        )));
    }

    #[test]
    fn validation_flags_incomplete_desktop_matrix() {
        let mut page = seeded_runtime_adaptation_page();
        page.desktop_matrix_rows
            .retain(|row| !matches!(row.host_os, HostOsClass::Linux));
        let errors = validate_runtime_adaptation_page(&page).expect_err("must flag missing OS row");
        assert!(errors.iter().any(|e| matches!(
            e,
            RuntimeAdaptationValidationError::DesktopMatrixIncomplete { .. }
        )));
    }

    #[test]
    fn support_export_quotes_every_case_id() {
        let page = seeded_runtime_adaptation_page();
        let export = RuntimeAdaptationSupportExport::from_page(
            "support-export:runtime-adaptation:001",
            page,
        );
        assert_eq!(
            export.shared_contract_ref,
            RUNTIME_ADAPTATION_SHARED_CONTRACT_REF
        );
        let mut expected: Vec<String> = Vec::new();
        for r in &export.page.power_posture_rows {
            expected.push(r.case_id.clone());
        }
        for r in &export.page.suspend_resume_rows {
            expected.push(r.case_id.clone());
        }
        for r in &export.page.monitor_continuity_rows {
            expected.push(r.case_id.clone());
        }
        for r in &export.page.foreground_protection_rows {
            expected.push(r.case_id.clone());
        }
        for r in &export.page.desktop_matrix_rows {
            expected.push(r.case_id.clone());
        }
        assert_eq!(export.case_ids, expected);
    }
}
