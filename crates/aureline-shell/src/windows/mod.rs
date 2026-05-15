//! Beta-grade workspace-management projection for split-window,
//! pane detach, cross-window move, and mixed-DPI restore.
//!
//! The windows module is the page-level surface above the
//! cross-window transfer fixtures, the layout serialization records,
//! and the restore-topology adjustments already frozen elsewhere in
//! the workspace. It does not invent its own topology truth or
//! shadow the live shell's geometry; it projects the acceptance
//! states M3 needs daily-beta users to be able to inspect:
//!
//! - split-window intents (axis, weights, focus after split, no
//!   silent buffer fork);
//! - pane-detach intents (origin pane, new window role, dirty / live
//!   posture preserved, command-fallback parity);
//! - cross-window move intents (tabs, editor groups, diff/review
//!   surfaces, inspectors) with canonical workspace truth preserved;
//! - restore-after-topology-change records (mixed DPI, monitor
//!   removal, docking) that label downgraded surfaces and the
//!   recovery-critical chrome that must stay reachable.
//!
//! The same projection feeds UI, the `aureline_shell_windows`
//! headless inspector, and the support-export wrapper. UI rows,
//! CLI rows, and support-export rows always come from the same
//! `case_id` and `shared_contract_ref`, so the live shell, the
//! review packet, and the support export report the same
//! workspace-management truth.

use serde::{Deserialize, Serialize};

/// Beta workspace-management schema version exported with every record.
pub const WINDOWS_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every beta workspace-management row.
pub const WINDOWS_BETA_SHARED_CONTRACT_REF: &str = "shell:windows_beta:v1";

/// Stable record kind for [`WindowsBetaPage`] payloads.
pub const WINDOWS_BETA_PAGE_RECORD_KIND: &str = "shell_windows_beta_page_record";

/// Stable record kind for [`WindowsBetaSupportExport`] payloads.
pub const WINDOWS_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_windows_beta_support_export_record";

/// Stable record kind for [`SplitWindowIntent`] payloads.
pub const SPLIT_WINDOW_INTENT_RECORD_KIND: &str = "shell_windows_beta_split_intent_record";

/// Stable record kind for [`PaneDetachIntent`] payloads.
pub const PANE_DETACH_INTENT_RECORD_KIND: &str = "shell_windows_beta_detach_intent_record";

/// Stable record kind for [`CrossWindowMoveIntent`] payloads.
pub const CROSS_WINDOW_MOVE_INTENT_RECORD_KIND: &str =
    "shell_windows_beta_cross_window_move_intent_record";

/// Stable record kind for [`RestoreTopologyOutcome`] payloads.
pub const RESTORE_TOPOLOGY_OUTCOME_RECORD_KIND: &str =
    "shell_windows_beta_restore_topology_outcome_record";

/// Split axis for the beta projection. Mirrors the live shell's
/// [`crate::layout::split_tree::SplitAxis`] vocabulary so the same
/// tokens flow from the in-memory split tree into review packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SplitAxisClass {
    /// Split the pane horizontally; new pane sits to the right.
    Vertical,
    /// Split the pane horizontally with the new pane below.
    Horizontal,
}

impl SplitAxisClass {
    /// Returns the stable schema token for this split axis.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

/// Surface role attached to a window participant in a beta record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowRoleClass {
    /// Primary workspace window (canonical workspace identity owner).
    PrimaryWorkspace,
    /// Secondary workspace window sharing the same workspace authority.
    SecondaryWorkspace,
    /// Specialized review or diff window.
    ReviewWindow,
    /// Specialized inspector or detail window.
    InspectorWindow,
}

impl WindowRoleClass {
    /// Returns the stable schema token for this role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryWorkspace => "primary_workspace",
            Self::SecondaryWorkspace => "secondary_workspace",
            Self::ReviewWindow => "review_window",
            Self::InspectorWindow => "inspector_window",
        }
    }
}

/// Surface kind that a transfer or split acts on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceKindClass {
    /// Editor tab.
    EditorTab,
    /// Editor group (split container holding tabs).
    EditorGroup,
    /// Diff or review surface.
    DiffReview,
    /// Inspector, detail pane, or other read-only surface.
    Inspector,
    /// Terminal pane.
    Terminal,
    /// Notebook surface.
    Notebook,
}

impl SurfaceKindClass {
    /// Returns the stable schema token for this surface kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorTab => "editor_tab",
            Self::EditorGroup => "editor_group",
            Self::DiffReview => "diff_review",
            Self::Inspector => "inspector",
            Self::Terminal => "terminal",
            Self::Notebook => "notebook",
        }
    }
}

/// Workspace continuity cues that must remain visible across a transfer.
///
/// These are the same cues the `cross_window_transfer_contract` keeps in
/// the design corpus: workspace authority, trust state, host/remote
/// state, profile, recovery cues, and the window-role badge. They are
/// modelled here so beta records can assert continuity without quoting
/// raw UI strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityCues {
    /// Opaque ref to the workspace identity.
    pub workspace_authority_ref: String,
    /// Trust state token (e.g. `trusted`, `restricted`, `quarantined`).
    pub trust_state: String,
    /// Host/remote state token (e.g. `local_host`, `managed_host`).
    pub host_remote_state: String,
    /// Profile ref carried with the surface.
    pub profile_ref: String,
    /// True when recovery cues stay visible across the transfer.
    pub recovery_cues_visible: bool,
    /// True when the trust badge stays visible in the destination chrome.
    pub trust_badge_visible: bool,
    /// True when the host/remote badge stays visible in the destination chrome.
    pub host_badge_visible: bool,
    /// Window-role badge token visible after the transfer.
    pub window_role_badge: WindowRoleClass,
}

/// Intent record describing a split-window operation.
///
/// A split intent never silently mutates buffer authority and never
/// invents a focus owner that is not in the source split tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitWindowIntent {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta workspace-management schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Window the split happens in.
    pub window_ref: String,
    /// Pane being split.
    pub pane_ref: String,
    /// Axis of the split.
    pub axis: SplitAxisClass,
    /// First-side weight (must be positive).
    pub first_weight: u16,
    /// Second-side weight (must be positive).
    pub second_weight: u16,
    /// Pane ref minted for the new side of the split.
    pub new_pane_ref: String,
    /// Pane that owns focus after the split commits.
    pub focus_owner_after_split: String,
    /// Continuity cues retained after the split.
    pub continuity: ContinuityCues,
    /// Command id wired to the equivalent command-palette fallback.
    pub command_fallback_ref: String,
    /// Required no-orphan invariants the surface must enforce.
    pub no_orphan_invariants: Vec<String>,
    /// Human-readable narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Intent record describing a pane detach into a new window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneDetachIntent {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta workspace-management schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Originating window the detach left.
    pub source_window_ref: String,
    /// Pane being detached.
    pub source_pane_ref: String,
    /// Surface kind being detached.
    pub surface_kind: SurfaceKindClass,
    /// Role of the new window the detach minted.
    pub new_window_role: WindowRoleClass,
    /// Window ref minted for the detached pane.
    pub new_window_ref: String,
    /// Restore posture preserved into the new window.
    pub restore_posture: String,
    /// Continuity cues retained on the detached surface.
    pub continuity: ContinuityCues,
    /// True when the source window's close history attributes the
    /// detach as an explicit user action (not an accidental tab close).
    pub source_close_attributed_to_detach: bool,
    /// Command id wired to the equivalent command-palette fallback.
    pub command_fallback_ref: String,
    /// Required no-orphan invariants the surface must enforce.
    pub no_orphan_invariants: Vec<String>,
    /// Human-readable narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Intent record describing a cross-window move of a tab, group, diff,
/// or inspector. Mirrors the existing cross-window transfer corpus
/// but in a shape consumable by the shell and support export at the
/// same time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossWindowMoveIntent {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta workspace-management schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Source window the surface left.
    pub source_window_ref: String,
    /// Target window the surface lands in.
    pub target_window_ref: String,
    /// Surface kind being moved.
    pub surface_kind: SurfaceKindClass,
    /// Source surface ref.
    pub source_surface_ref: String,
    /// Target slot ref the surface lands in.
    pub target_slot_ref: String,
    /// Continuity cues retained across the move.
    pub continuity: ContinuityCues,
    /// True when canonical workspace truth is preserved (no fork, no
    /// authority swap, no silent restore-state loss).
    pub canonical_workspace_truth_preserved: bool,
    /// Command id wired to the equivalent command-palette fallback.
    pub command_fallback_ref: String,
    /// Required no-orphan invariants the surface must enforce.
    pub no_orphan_invariants: Vec<String>,
    /// Human-readable narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Topology change class consumed by [`RestoreTopologyOutcome`].
///
/// Mirrors the typed `topology_change_class` corpus already pinned
/// under `fixtures/ux/restore_topology_cases/`. A record may carry
/// multiple classes when more than one drift is detected at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyChangeClass {
    /// One or more displays present at last save are no longer connected.
    DisplayRemoved,
    /// Safe visible bounds shrank or shifted relative to last save.
    SafeBoundsChanged,
    /// Scale bucket drifted (e.g. mixed-DPI dock/undock).
    ScaleChanged,
    /// Wake from sleep or display reconnect.
    WakeOrReconnect,
    /// Docking, undocking, or other monitor topology rearrangement.
    DockingChanged,
}

impl TopologyChangeClass {
    /// Returns the stable schema token for this topology change class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisplayRemoved => "display_removed",
            Self::SafeBoundsChanged => "safe_bounds_changed",
            Self::ScaleChanged => "scale_changed",
            Self::WakeOrReconnect => "wake_or_reconnect",
            Self::DockingChanged => "docking_changed",
        }
    }
}

/// Adjustment class applied to a window when restore reconciles a
/// topology change. Mirrors
/// [`aureline_workspace::DisplayAdjustmentClass`] so the same vocabulary
/// flows from the workspace portable-state package into the shell
/// review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreAdjustmentClass {
    /// Window was snapped into the safe visible region.
    SnappedToSafeBounds,
    /// Window was moved to the primary display.
    MovedToPrimaryDisplay,
    /// Window scale was normalized across mixed-DPI buckets.
    ScaleNormalized,
    /// Fullscreen state was cleared.
    FullscreenCleared,
    /// Owner dialog was recentered with its owner window.
    DialogRecenteredToOwner,
    /// Restore fell back to layout-only fidelity.
    LayoutOnlyFallback,
}

impl RestoreAdjustmentClass {
    /// Returns the stable schema token for this adjustment class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SnappedToSafeBounds => "snapped_to_safe_bounds",
            Self::MovedToPrimaryDisplay => "moved_to_primary_display",
            Self::ScaleNormalized => "scale_normalized",
            Self::FullscreenCleared => "fullscreen_cleared",
            Self::DialogRecenteredToOwner => "dialog_recentered_to_owner",
            Self::LayoutOnlyFallback => "layout_only_fallback",
        }
    }
}

/// Restore fidelity floor admitted for the topology outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFidelityClass {
    /// Layout, focus, and bounds restored exactly.
    Exact,
    /// Layout intent preserved; bounds or scale adjusted.
    Compatible,
    /// Layout-only fallback; live surfaces replaced by placeholders.
    LayoutOnly,
}

impl RestoreFidelityClass {
    /// Returns the stable schema token for this fidelity class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact_restore",
            Self::Compatible => "compatible_restore",
            Self::LayoutOnly => "layout_only_restore",
        }
    }
}

/// Recovery-critical chrome that the topology outcome promises remains
/// reachable after restore. The list is intentionally small: it is the
/// recovery chrome a user needs to *get out of* a degraded restore,
/// not the entire shell surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryChromeAssurance {
    /// Title context / breadcrumb path remains visible.
    pub title_context_visible: bool,
    /// Restore prompt or restore details remain reachable.
    pub restore_details_reachable: bool,
    /// Command palette remains reachable via keyboard.
    pub command_palette_reachable: bool,
    /// A11y focus order remains reachable from the keyboard.
    pub keyboard_focus_reachable: bool,
    /// Activity center / status strip remains visible.
    pub activity_center_visible: bool,
}

impl RecoveryChromeAssurance {
    /// Returns true when every assurance is satisfied.
    pub fn all_satisfied(&self) -> bool {
        self.title_context_visible
            && self.restore_details_reachable
            && self.command_palette_reachable
            && self.keyboard_focus_reachable
            && self.activity_center_visible
    }
}

/// Pane outcome attached to a restore-topology record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorePaneOutcome {
    /// Pane ref the outcome applies to.
    pub pane_ref: String,
    /// Surface kind for the pane.
    pub surface_kind: SurfaceKindClass,
    /// Restore posture label.
    pub restore_posture: String,
    /// Placeholder label, if the pane could not hydrate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_label: Option<String>,
    /// True when the pane requires an explicit user action to rerun.
    pub requires_explicit_rerun: bool,
}

/// Restore outcome after a topology change. Carries the topology
/// classes, the typed adjustments applied, the resulting fidelity
/// floor, the per-pane outcomes, and the recovery-chrome assurance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreTopologyOutcome {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta workspace-management schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Window ref this outcome applies to.
    pub window_ref: String,
    /// Topology change classes detected at restore time.
    pub topology_change_classes: Vec<TopologyChangeClass>,
    /// Adjustments applied during restore.
    pub adjustments: Vec<RestoreAdjustmentClass>,
    /// Restore fidelity floor admitted to the user.
    pub restore_fidelity: RestoreFidelityClass,
    /// True when the user-visible "layout adjusted" note is required.
    pub user_visible_layout_adjusted_note_required: bool,
    /// Per-pane outcomes.
    pub pane_outcomes: Vec<RestorePaneOutcome>,
    /// Recovery-critical chrome assurance after the restore.
    pub recovery_chrome: RecoveryChromeAssurance,
    /// Required no-rerun invariants the restore must enforce.
    pub no_rerun_invariants: Vec<String>,
    /// Human-readable narrative used in fixtures and a11y exports.
    pub narrative: String,
}

/// Builder helpers for the four intent records.
impl SplitWindowIntent {
    /// Construct a beta split-window intent record with stable defaults.
    pub fn new(
        case_id: impl Into<String>,
        window_ref: impl Into<String>,
        pane_ref: impl Into<String>,
        axis: SplitAxisClass,
        new_pane_ref: impl Into<String>,
        continuity: ContinuityCues,
    ) -> Self {
        let new_pane_ref = new_pane_ref.into();
        let focus = new_pane_ref.clone();
        Self {
            record_kind: SPLIT_WINDOW_INTENT_RECORD_KIND.to_owned(),
            schema_version: WINDOWS_BETA_SCHEMA_VERSION,
            shared_contract_ref: WINDOWS_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: case_id.into(),
            window_ref: window_ref.into(),
            pane_ref: pane_ref.into(),
            axis,
            first_weight: 1,
            second_weight: 1,
            new_pane_ref,
            focus_owner_after_split: focus,
            continuity,
            command_fallback_ref: "cmd:window.split_pane".to_owned(),
            no_orphan_invariants: vec![
                "no_silent_buffer_fork".to_owned(),
                "focus_owner_in_split_tree".to_owned(),
            ],
            narrative: String::new(),
        }
    }

    /// Set the human-readable narrative used in fixtures and a11y exports.
    pub fn with_narrative(mut self, narrative: impl Into<String>) -> Self {
        self.narrative = narrative.into();
        self
    }
}

impl PaneDetachIntent {
    /// Construct a beta pane-detach intent record with stable defaults.
    pub fn new(
        case_id: impl Into<String>,
        source_window_ref: impl Into<String>,
        source_pane_ref: impl Into<String>,
        surface_kind: SurfaceKindClass,
        new_window_role: WindowRoleClass,
        new_window_ref: impl Into<String>,
        continuity: ContinuityCues,
    ) -> Self {
        Self {
            record_kind: PANE_DETACH_INTENT_RECORD_KIND.to_owned(),
            schema_version: WINDOWS_BETA_SCHEMA_VERSION,
            shared_contract_ref: WINDOWS_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: case_id.into(),
            source_window_ref: source_window_ref.into(),
            source_pane_ref: source_pane_ref.into(),
            surface_kind,
            new_window_role,
            new_window_ref: new_window_ref.into(),
            restore_posture: "live_authority_preserved".to_owned(),
            continuity,
            source_close_attributed_to_detach: true,
            command_fallback_ref: "cmd:window.detach_pane".to_owned(),
            no_orphan_invariants: vec![
                "dirty_state_visible_in_new_window".to_owned(),
                "no_implicit_close_history".to_owned(),
            ],
            narrative: String::new(),
        }
    }

    /// Set the human-readable narrative used in fixtures and a11y exports.
    pub fn with_narrative(mut self, narrative: impl Into<String>) -> Self {
        self.narrative = narrative.into();
        self
    }
}

impl CrossWindowMoveIntent {
    /// Construct a beta cross-window move intent record with stable defaults.
    pub fn new(
        case_id: impl Into<String>,
        source_window_ref: impl Into<String>,
        target_window_ref: impl Into<String>,
        surface_kind: SurfaceKindClass,
        source_surface_ref: impl Into<String>,
        target_slot_ref: impl Into<String>,
        continuity: ContinuityCues,
    ) -> Self {
        Self {
            record_kind: CROSS_WINDOW_MOVE_INTENT_RECORD_KIND.to_owned(),
            schema_version: WINDOWS_BETA_SCHEMA_VERSION,
            shared_contract_ref: WINDOWS_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: case_id.into(),
            source_window_ref: source_window_ref.into(),
            target_window_ref: target_window_ref.into(),
            surface_kind,
            source_surface_ref: source_surface_ref.into(),
            target_slot_ref: target_slot_ref.into(),
            continuity,
            canonical_workspace_truth_preserved: true,
            command_fallback_ref: "cmd:window.move_surface_to_window".to_owned(),
            no_orphan_invariants: vec![
                "no_authority_swap".to_owned(),
                "no_silent_restore_state_loss".to_owned(),
                "no_orphan_on_source_close".to_owned(),
            ],
            narrative: String::new(),
        }
    }

    /// Set the human-readable narrative used in fixtures and a11y exports.
    pub fn with_narrative(mut self, narrative: impl Into<String>) -> Self {
        self.narrative = narrative.into();
        self
    }
}

/// Aggregate summary banner for the beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WindowsBetaSummary {
    /// Number of split-window intents on the page.
    pub split_intent_count: usize,
    /// Number of pane-detach intents on the page.
    pub detach_intent_count: usize,
    /// Number of cross-window move intents on the page.
    pub move_intent_count: usize,
    /// Number of restore-topology outcomes on the page.
    pub restore_outcome_count: usize,
    /// Number of outcomes whose fidelity floor is layout-only.
    pub layout_only_outcomes: usize,
    /// Number of outcomes that required a user-visible "layout adjusted" note.
    pub layout_adjusted_notes: usize,
    /// Number of outcomes where recovery-critical chrome is fully reachable.
    pub recovery_chrome_fully_reachable: usize,
}

impl WindowsBetaSummary {
    fn from_page(
        splits: &[SplitWindowIntent],
        detaches: &[PaneDetachIntent],
        moves: &[CrossWindowMoveIntent],
        outcomes: &[RestoreTopologyOutcome],
    ) -> Self {
        let layout_only_outcomes = outcomes
            .iter()
            .filter(|o| matches!(o.restore_fidelity, RestoreFidelityClass::LayoutOnly))
            .count();
        let layout_adjusted_notes = outcomes
            .iter()
            .filter(|o| o.user_visible_layout_adjusted_note_required)
            .count();
        let recovery_chrome_fully_reachable = outcomes
            .iter()
            .filter(|o| o.recovery_chrome.all_satisfied())
            .count();
        Self {
            split_intent_count: splits.len(),
            detach_intent_count: detaches.len(),
            move_intent_count: moves.len(),
            restore_outcome_count: outcomes.len(),
            layout_only_outcomes,
            layout_adjusted_notes,
            recovery_chrome_fully_reachable,
        }
    }
}

/// Top-level beta workspace-management page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowsBetaPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta workspace-management schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Page label for chrome.
    pub page_label: String,
    /// Aggregate summary banner.
    pub summary: WindowsBetaSummary,
    /// Split-window intents on the page.
    pub split_intents: Vec<SplitWindowIntent>,
    /// Pane-detach intents on the page.
    pub detach_intents: Vec<PaneDetachIntent>,
    /// Cross-window move intents on the page.
    pub move_intents: Vec<CrossWindowMoveIntent>,
    /// Restore-topology outcomes on the page.
    pub restore_outcomes: Vec<RestoreTopologyOutcome>,
}

impl WindowsBetaPage {
    /// Build a beta page from the four intent / outcome lists.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        split_intents: Vec<SplitWindowIntent>,
        detach_intents: Vec<PaneDetachIntent>,
        move_intents: Vec<CrossWindowMoveIntent>,
        restore_outcomes: Vec<RestoreTopologyOutcome>,
    ) -> Self {
        let summary = WindowsBetaSummary::from_page(
            &split_intents,
            &detach_intents,
            &move_intents,
            &restore_outcomes,
        );
        Self {
            record_kind: WINDOWS_BETA_PAGE_RECORD_KIND.to_owned(),
            schema_version: WINDOWS_BETA_SCHEMA_VERSION,
            shared_contract_ref: WINDOWS_BETA_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            summary,
            split_intents,
            detach_intents,
            move_intents,
            restore_outcomes,
        }
    }
}

/// Support-export wrapper for the beta page. Mirrors the wrapper shape
/// used by `aureline-settings` so support reviewers can pivot from a
/// row to the page that owns it without separate query plumbing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowsBetaSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta workspace-management schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Embedded page.
    pub page: WindowsBetaPage,
    /// All case ids exposed by the page, in stable order.
    pub case_ids: Vec<String>,
}

impl WindowsBetaSupportExport {
    /// Build a support export wrapper from a beta page.
    pub fn from_page(export_id: impl Into<String>, page: WindowsBetaPage) -> Self {
        let mut case_ids: Vec<String> = Vec::new();
        for record in &page.split_intents {
            case_ids.push(record.case_id.clone());
        }
        for record in &page.detach_intents {
            case_ids.push(record.case_id.clone());
        }
        for record in &page.move_intents {
            case_ids.push(record.case_id.clone());
        }
        for record in &page.restore_outcomes {
            case_ids.push(record.case_id.clone());
        }
        Self {
            record_kind: WINDOWS_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: WINDOWS_BETA_SCHEMA_VERSION,
            shared_contract_ref: WINDOWS_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            page,
            case_ids,
        }
    }
}

/// Seeded fixture builder used by the headless bin and the integration
/// test. The seed mirrors the JSON checked in under
/// `fixtures/ux/m3/window_topology/` so the live shell, the review
/// packet, and the support export report the same workspace-management
/// truth.
pub fn seeded_windows_beta_page() -> WindowsBetaPage {
    let primary_continuity = ContinuityCues {
        workspace_authority_ref: "workspace-authority:payments".to_owned(),
        trust_state: "trusted".to_owned(),
        host_remote_state: "local_host".to_owned(),
        profile_ref: "profile:developer:local".to_owned(),
        recovery_cues_visible: true,
        trust_badge_visible: true,
        host_badge_visible: true,
        window_role_badge: WindowRoleClass::PrimaryWorkspace,
    };
    let secondary_continuity = ContinuityCues {
        window_role_badge: WindowRoleClass::SecondaryWorkspace,
        ..primary_continuity.clone()
    };
    let review_continuity = ContinuityCues {
        window_role_badge: WindowRoleClass::ReviewWindow,
        ..primary_continuity.clone()
    };
    let inspector_continuity = ContinuityCues {
        window_role_badge: WindowRoleClass::InspectorWindow,
        ..primary_continuity.clone()
    };

    let split = SplitWindowIntent::new(
        "shell:windows-beta:split:editor-vertical:01",
        "win:workspace:primary:01",
        "pane:editor:handler-rs:01",
        SplitAxisClass::Vertical,
        "pane:editor:handler-rs:02",
        primary_continuity.clone(),
    )
    .with_narrative(
        "Split the active editor pane vertically; the new pane takes focus while the source pane retains its tab membership and dirty markers.",
    );

    let detach = PaneDetachIntent::new(
        "shell:windows-beta:detach:terminal-to-secondary:01",
        "win:workspace:primary:01",
        "pane:terminal:repl:01",
        SurfaceKindClass::Terminal,
        WindowRoleClass::SecondaryWorkspace,
        "win:workspace:secondary:terminal:01",
        secondary_continuity.clone(),
    )
    .with_narrative(
        "Detach a terminal pane into a new secondary window. The source close history attributes the detach as an explicit user action; the dirty buffer authority remains visible in the new window.",
    );

    let move_tab = CrossWindowMoveIntent::new(
        "shell:windows-beta:move:dirty-tab-to-primary:01",
        "win:workspace:secondary:01",
        "win:workspace:primary:01",
        SurfaceKindClass::EditorTab,
        "tab:editor:handler-rs:dirty-recovered",
        "group:primary:editor-slot-right",
        primary_continuity.clone(),
    )
    .with_narrative(
        "Move a dirty recovered editor tab from a secondary window to the primary window. Workspace authority is unchanged; the trust badge, host/remote badge, and recovery cues remain visible in the target window.",
    );

    let move_review = CrossWindowMoveIntent::new(
        "shell:windows-beta:move:diff-review-to-review-window:01",
        "win:workspace:primary:01",
        "win:workspace:review:01",
        SurfaceKindClass::DiffReview,
        "diff:review:payments-feature:01",
        "group:review:primary-slot",
        review_continuity.clone(),
    )
    .with_narrative(
        "Move a diff/review surface from the primary workspace window into the specialized review window. The review window retains workspace authority and the review evidence continuation.",
    );

    let move_inspector = CrossWindowMoveIntent::new(
        "shell:windows-beta:move:inspector-to-inspector-window:01",
        "win:workspace:primary:01",
        "win:workspace:inspector:01",
        SurfaceKindClass::Inspector,
        "inspector:settings:security-egress:01",
        "group:inspector:primary-slot",
        inspector_continuity.clone(),
    )
    .with_narrative(
        "Move the settings effective-value inspector into a dedicated inspector window. The trust, host/remote, and profile cues remain visible; the inspector keeps reading the same effective-settings projection it had in the source window.",
    );

    let mixed_dpi = RestoreTopologyOutcome {
        record_kind: RESTORE_TOPOLOGY_OUTCOME_RECORD_KIND.to_owned(),
        schema_version: WINDOWS_BETA_SCHEMA_VERSION,
        shared_contract_ref: WINDOWS_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:windows-beta:restore:mixed-dpi-normalized:01".to_owned(),
        window_ref: "win:workspace:secondary:01".to_owned(),
        topology_change_classes: vec![
            TopologyChangeClass::ScaleChanged,
            TopologyChangeClass::SafeBoundsChanged,
        ],
        adjustments: vec![
            RestoreAdjustmentClass::ScaleNormalized,
            RestoreAdjustmentClass::SnappedToSafeBounds,
        ],
        restore_fidelity: RestoreFidelityClass::Compatible,
        user_visible_layout_adjusted_note_required: true,
        pane_outcomes: vec![RestorePaneOutcome {
            pane_ref: "pane:editor:handler-rs:01".to_owned(),
            surface_kind: SurfaceKindClass::EditorTab,
            restore_posture: "live_authority_preserved".to_owned(),
            placeholder_label: None,
            requires_explicit_rerun: false,
        }],
        recovery_chrome: RecoveryChromeAssurance {
            title_context_visible: true,
            restore_details_reachable: true,
            command_palette_reachable: true,
            keyboard_focus_reachable: true,
            activity_center_visible: true,
        },
        no_rerun_invariants: vec!["no_command_rerun".to_owned()],
        narrative:
            "Window restored after a mixed-DPI dock change. Scale was normalized before focus and keyboard routing; bounds were clamped into the current safe visible region; the user sees a layout-adjusted note in the restore history."
                .to_owned(),
    };

    let display_detach = RestoreTopologyOutcome {
        record_kind: RESTORE_TOPOLOGY_OUTCOME_RECORD_KIND.to_owned(),
        schema_version: WINDOWS_BETA_SCHEMA_VERSION,
        shared_contract_ref: WINDOWS_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:windows-beta:restore:display-detach-safe-bounds:01".to_owned(),
        window_ref: "win:workspace:secondary:01".to_owned(),
        topology_change_classes: vec![
            TopologyChangeClass::DisplayRemoved,
            TopologyChangeClass::SafeBoundsChanged,
        ],
        adjustments: vec![
            RestoreAdjustmentClass::MovedToPrimaryDisplay,
            RestoreAdjustmentClass::SnappedToSafeBounds,
            RestoreAdjustmentClass::DialogRecenteredToOwner,
        ],
        restore_fidelity: RestoreFidelityClass::Compatible,
        user_visible_layout_adjusted_note_required: true,
        pane_outcomes: vec![RestorePaneOutcome {
            pane_ref: "pane:terminal:repl:01".to_owned(),
            surface_kind: SurfaceKindClass::Terminal,
            restore_posture: "evidence_only_placeholder".to_owned(),
            placeholder_label: Some("Terminal transcript only; rerun requires explicit user action.".to_owned()),
            requires_explicit_rerun: true,
        }],
        recovery_chrome: RecoveryChromeAssurance {
            title_context_visible: true,
            restore_details_reachable: true,
            command_palette_reachable: true,
            keyboard_focus_reachable: true,
            activity_center_visible: true,
        },
        no_rerun_invariants: vec![
            "no_command_rerun".to_owned(),
            "no_hidden_authority_reacquire".to_owned(),
            "transcript_or_snapshot_only".to_owned(),
            "explicit_user_action_required".to_owned(),
        ],
        narrative:
            "Window restored after a previously connected external display detached. The window was remapped to the primary display and clamped to the current safe visible region; the owner dialog was recentered with its owner; the terminal pane restored as transcript-only with explicit-rerun guardrails."
                .to_owned(),
    };

    let docking_change = RestoreTopologyOutcome {
        record_kind: RESTORE_TOPOLOGY_OUTCOME_RECORD_KIND.to_owned(),
        schema_version: WINDOWS_BETA_SCHEMA_VERSION,
        shared_contract_ref: WINDOWS_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: "shell:windows-beta:restore:dock-undock-recovery-chrome:01".to_owned(),
        window_ref: "win:workspace:primary:01".to_owned(),
        topology_change_classes: vec![
            TopologyChangeClass::DockingChanged,
            TopologyChangeClass::WakeOrReconnect,
            TopologyChangeClass::SafeBoundsChanged,
        ],
        adjustments: vec![
            RestoreAdjustmentClass::FullscreenCleared,
            RestoreAdjustmentClass::SnappedToSafeBounds,
        ],
        restore_fidelity: RestoreFidelityClass::Compatible,
        user_visible_layout_adjusted_note_required: true,
        pane_outcomes: vec![RestorePaneOutcome {
            pane_ref: "pane:editor:handler-rs:01".to_owned(),
            surface_kind: SurfaceKindClass::EditorTab,
            restore_posture: "live_authority_preserved".to_owned(),
            placeholder_label: None,
            requires_explicit_rerun: false,
        }],
        recovery_chrome: RecoveryChromeAssurance {
            title_context_visible: true,
            restore_details_reachable: true,
            command_palette_reachable: true,
            keyboard_focus_reachable: true,
            activity_center_visible: true,
        },
        no_rerun_invariants: vec!["no_command_rerun".to_owned()],
        narrative:
            "Window restored after a dock/undock cycle and display reconnect. Fullscreen was cleared and bounds clamped before routing focus; the recovery-critical chrome (title context, restore details, command palette, keyboard focus, activity center) stays reachable across the adjustment."
                .to_owned(),
    };

    WindowsBetaPage::new(
        "all",
        "Multi-window workspace management (beta)",
        vec![split],
        vec![detach],
        vec![move_tab, move_review, move_inspector],
        vec![mixed_dpi, display_detach, docking_change],
    )
}

/// Validation errors surfaced when a beta page or its records fail an
/// acceptance invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowsBetaValidationError {
    /// A split intent set focus to a pane not in the split tree.
    SplitFocusOutOfTree {
        /// Split-intent case id.
        case_id: String,
        /// Offending pane ref.
        pane_ref: String,
    },
    /// A split intent used a zero weight on one side.
    SplitZeroWeight {
        /// Split-intent case id.
        case_id: String,
    },
    /// A cross-window move dropped the canonical workspace truth.
    MoveLostCanonicalTruth {
        /// Move-intent case id.
        case_id: String,
    },
    /// A restore outcome admitted exact restore while still recording adjustments.
    ExactRestoreWithAdjustments {
        /// Outcome case id.
        case_id: String,
        /// Number of recorded adjustments.
        adjustment_count: usize,
    },
    /// A restore outcome admitted compatible or layout-only restore
    /// without the user-visible "layout adjusted" note.
    DowngradedWithoutNote {
        /// Outcome case id.
        case_id: String,
    },
    /// A restore outcome hid recovery-critical chrome.
    RecoveryChromeHidden {
        /// Outcome case id.
        case_id: String,
    },
    /// A restore outcome promised live rerun for a live-capability pane.
    LiveSurfaceSilentRerun {
        /// Outcome case id.
        case_id: String,
        /// Pane ref that promised silent rerun.
        pane_ref: String,
    },
}

impl std::fmt::Display for WindowsBetaValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SplitFocusOutOfTree { case_id, pane_ref } => write!(
                f,
                "split intent {case_id} placed focus on pane {pane_ref} not in the split tree"
            ),
            Self::SplitZeroWeight { case_id } => {
                write!(f, "split intent {case_id} used a zero weight on one side")
            }
            Self::MoveLostCanonicalTruth { case_id } => write!(
                f,
                "cross-window move {case_id} dropped canonical workspace truth"
            ),
            Self::ExactRestoreWithAdjustments {
                case_id,
                adjustment_count,
            } => write!(
                f,
                "restore outcome {case_id} admitted exact restore but recorded {adjustment_count} adjustments"
            ),
            Self::DowngradedWithoutNote { case_id } => write!(
                f,
                "restore outcome {case_id} downgraded fidelity without a user-visible note"
            ),
            Self::RecoveryChromeHidden { case_id } => {
                write!(f, "restore outcome {case_id} hid recovery-critical chrome")
            }
            Self::LiveSurfaceSilentRerun { case_id, pane_ref } => write!(
                f,
                "restore outcome {case_id} pane {pane_ref} promised silent rerun for a live-capability surface"
            ),
        }
    }
}

impl std::error::Error for WindowsBetaValidationError {}

/// Validate the acceptance invariants on a beta workspace-management page.
pub fn validate_windows_beta_page(
    page: &WindowsBetaPage,
) -> Result<(), Vec<WindowsBetaValidationError>> {
    let mut errors: Vec<WindowsBetaValidationError> = Vec::new();

    for split in &page.split_intents {
        if split.first_weight == 0 || split.second_weight == 0 {
            errors.push(WindowsBetaValidationError::SplitZeroWeight {
                case_id: split.case_id.clone(),
            });
        }
        let focus = split.focus_owner_after_split.as_str();
        if focus != split.pane_ref && focus != split.new_pane_ref {
            errors.push(WindowsBetaValidationError::SplitFocusOutOfTree {
                case_id: split.case_id.clone(),
                pane_ref: focus.to_owned(),
            });
        }
    }

    for move_intent in &page.move_intents {
        if !move_intent.canonical_workspace_truth_preserved {
            errors.push(WindowsBetaValidationError::MoveLostCanonicalTruth {
                case_id: move_intent.case_id.clone(),
            });
        }
    }

    for outcome in &page.restore_outcomes {
        if matches!(outcome.restore_fidelity, RestoreFidelityClass::Exact)
            && !outcome.adjustments.is_empty()
        {
            errors.push(WindowsBetaValidationError::ExactRestoreWithAdjustments {
                case_id: outcome.case_id.clone(),
                adjustment_count: outcome.adjustments.len(),
            });
        }
        if matches!(
            outcome.restore_fidelity,
            RestoreFidelityClass::Compatible | RestoreFidelityClass::LayoutOnly
        ) && !outcome.user_visible_layout_adjusted_note_required
        {
            errors.push(WindowsBetaValidationError::DowngradedWithoutNote {
                case_id: outcome.case_id.clone(),
            });
        }
        if !outcome.recovery_chrome.all_satisfied() {
            errors.push(WindowsBetaValidationError::RecoveryChromeHidden {
                case_id: outcome.case_id.clone(),
            });
        }
        for pane in &outcome.pane_outcomes {
            let is_live_surface = matches!(
                pane.surface_kind,
                SurfaceKindClass::Terminal | SurfaceKindClass::Notebook
            );
            let promises_silent_rerun = !pane.requires_explicit_rerun
                && pane.restore_posture == "evidence_only_placeholder";
            if is_live_surface && promises_silent_rerun {
                errors.push(WindowsBetaValidationError::LiveSurfaceSilentRerun {
                    case_id: outcome.case_id.clone(),
                    pane_ref: pane.pane_ref.clone(),
                });
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_summary_matches_record_counts() {
        let page = seeded_windows_beta_page();
        assert_eq!(page.shared_contract_ref, WINDOWS_BETA_SHARED_CONTRACT_REF);
        assert_eq!(page.summary.split_intent_count, page.split_intents.len());
        assert_eq!(page.summary.detach_intent_count, page.detach_intents.len());
        assert_eq!(page.summary.move_intent_count, page.move_intents.len());
        assert_eq!(
            page.summary.restore_outcome_count,
            page.restore_outcomes.len()
        );
        assert_eq!(page.summary.layout_only_outcomes, 0);
        assert_eq!(
            page.summary.layout_adjusted_notes,
            page.restore_outcomes.len()
        );
        assert_eq!(
            page.summary.recovery_chrome_fully_reachable,
            page.restore_outcomes.len()
        );
    }

    #[test]
    fn seeded_page_passes_validation() {
        let page = seeded_windows_beta_page();
        assert!(validate_windows_beta_page(&page).is_ok());
    }

    #[test]
    fn validation_flags_focus_out_of_tree() {
        let mut page = seeded_windows_beta_page();
        let split = page.split_intents.get_mut(0).expect("seed has a split");
        split.focus_owner_after_split = "pane:unrelated".to_owned();
        let errors = validate_windows_beta_page(&page).expect_err("must flag focus drift");
        assert!(errors.iter().any(|e| matches!(
            e,
            WindowsBetaValidationError::SplitFocusOutOfTree { .. }
        )));
    }

    #[test]
    fn validation_flags_canonical_truth_loss() {
        let mut page = seeded_windows_beta_page();
        page.move_intents
            .get_mut(0)
            .expect("seed has a move")
            .canonical_workspace_truth_preserved = false;
        let errors = validate_windows_beta_page(&page).expect_err("must flag canonical truth loss");
        assert!(errors.iter().any(|e| matches!(
            e,
            WindowsBetaValidationError::MoveLostCanonicalTruth { .. }
        )));
    }

    #[test]
    fn validation_flags_recovery_chrome_hidden() {
        let mut page = seeded_windows_beta_page();
        page.restore_outcomes
            .get_mut(0)
            .expect("seed has an outcome")
            .recovery_chrome
            .restore_details_reachable = false;
        let errors = validate_windows_beta_page(&page).expect_err("must flag hidden chrome");
        assert!(errors.iter().any(|e| matches!(
            e,
            WindowsBetaValidationError::RecoveryChromeHidden { .. }
        )));
    }

    #[test]
    fn validation_flags_silent_rerun_on_live_surface() {
        let mut page = seeded_windows_beta_page();
        let outcome = page
            .restore_outcomes
            .iter_mut()
            .find(|o| {
                o.pane_outcomes
                    .iter()
                    .any(|p| matches!(p.surface_kind, SurfaceKindClass::Terminal))
            })
            .expect("seed has a terminal outcome");
        let pane = outcome
            .pane_outcomes
            .iter_mut()
            .find(|p| matches!(p.surface_kind, SurfaceKindClass::Terminal))
            .expect("terminal pane present");
        pane.requires_explicit_rerun = false;
        let errors = validate_windows_beta_page(&page).expect_err("must flag silent rerun");
        assert!(errors.iter().any(|e| matches!(
            e,
            WindowsBetaValidationError::LiveSurfaceSilentRerun { .. }
        )));
    }

    #[test]
    fn support_export_quotes_every_case_id() {
        let page = seeded_windows_beta_page();
        let export = WindowsBetaSupportExport::from_page("support-export:windows-beta:001", page);
        assert_eq!(export.shared_contract_ref, WINDOWS_BETA_SHARED_CONTRACT_REF);
        let mut expected: Vec<String> = Vec::new();
        for r in &export.page.split_intents {
            expected.push(r.case_id.clone());
        }
        for r in &export.page.detach_intents {
            expected.push(r.case_id.clone());
        }
        for r in &export.page.move_intents {
            expected.push(r.case_id.clone());
        }
        for r in &export.page.restore_outcomes {
            expected.push(r.case_id.clone());
        }
        assert_eq!(export.case_ids, expected);
    }
}
