//! Canonical stable truth model for **focus, current item, selection, anchor,
//! activation, keyboard parity, and collection-state semantics** on a
//! claimed-stable dense shell surface.
//!
//! ## Why one governed record per interaction posture
//!
//! Dense list, tree, grid, palette, and inspector surfaces all converge on the
//! same risk: the shell quietly conflates focus with selection, loses the
//! selection anchor under virtualization, lets a streamed row or a background
//! index steal focus from active work, or drops focus to the document body when
//! a dialog closes — and the user, especially a keyboard-only or assistive-tech
//! user, loses their place with no announcement. The truth lives only in a
//! transient visual cue, if anywhere.
//!
//! This module mints one governed [`InteractionParityRecord`] per claimed-stable
//! interaction posture. The record binds, for a single dense-surface identity:
//!
//! - **Distinct coordination states** — `Focus`, `Current item`, `Selection`,
//!   `Anchor`, and `Activation` are modeled as separate state objects keyed by
//!   stable object identity, never collapsed onto a single value and never keyed
//!   by row index or transient DOM position.
//! - **Identity that survives asynchronous updates** — streaming inserts,
//!   sort/filter/pagination refresh, background indexing, and extension-view
//!   replacement preserve focus and selection by stable object id.
//! - **Complete focus return** — every dialog, sheet, palette, popover, inline
//!   rename, placeholder card, inspector, pane close, and split reflow records a
//!   focus-return target and returns to the invoking control, its row, or the
//!   nearest safe ancestor or sibling — never the document body and never an
//!   off-screen surface or a different window.
//! - **A complete keyboard model** — single-tab-stop or roving-tabindex with
//!   Arrow moving the current item, Space toggling selection where supported,
//!   Enter triggering the discoverable default action, and Home/End/Page
//!   preserving anchor semantics without silently firing destructive actions.
//! - **No focus theft** — background indexing, streamed rows, notifications,
//!   banners, diagnostics, and multi-window updates never steal focus from the
//!   active task; when the focused object disappears, focus moves to the nearest
//!   safe sibling or parent and the reason is announced.
//! - **Complete accessibility cues** — selected-count narration, position-in-set
//!   cues, and blocked/read-only row cues hold across normal, high-contrast, and
//!   zoomed layouts.
//! - **Per-OS conformance** — macOS, Windows, and Linux each carry current proof.
//! - **A public claim ceiling** and **automatic narrowing** below Stable with a
//!   named reason.
//! - **Recovery, route, and accessibility parity** and **no-account /
//!   no-managed-services availability**.
//!
//! The shell collection surface, the keyboard-help reference, the CLI inspector,
//! Help/About, and the diagnostics support export read this record verbatim
//! instead of cloning status text. The shared object-interaction vocabulary, the
//! batch-scope truth, the responsive identity cues, and the focus-return grammar
//! are **not** reinvented here: each record is a genuine projection of the live
//! interaction-integrity packet in [`crate::interaction_integrity`], hardened to
//! the Stable claim and extended to cover the palette-like and inspector/detail
//! families.
//!
//! The canonical artifacts for this lane (suggested-output stem
//! `harden-focus-selection-keyboard-parity-and-collection-state`) are:
//!
//! - [`model`](self) — the governed record, its closed vocabularies, the builder,
//!   and the honesty invariants. The boundary schema is
//!   `schemas/ux/harden-focus-selection-keyboard-parity-and-collection-state.schema.json`.
//! - [`corpus`](super::corpus) — the deterministic claimed-stable matrix,
//!   projected through the live interaction-integrity packet, and pinned on disk
//!   under
//!   `fixtures/ux/m4/harden-focus-selection-keyboard-parity-and-collection-state/`.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::notification_attention_stable::model::{
    is_canonical_object_ref, AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord,
    LayoutMode, LifecycleMarker, RecoveryActionRole, RecoveryRouteRecord, StableClaimClass,
};

/// Stable record-kind tag carried in serialized interaction-parity records.
pub const INTERACTION_PARITY_RECORD_KIND: &str = "interaction_parity_record";

/// Schema version for the [`InteractionParityRecord`] payload shape.
pub const INTERACTION_PARITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const INTERACTION_PARITY_SHARED_CONTRACT_REF: &str = "shell:interaction_parity_stable:v1";

/// Reviewer-facing notice rendered on every interaction-parity surface.
pub const INTERACTION_PARITY_NOTICE: &str =
    "Interaction-parity truth: each claimed-stable dense shell surface models Focus, Current item, \
     Selection, Anchor, and Activation as separate state objects keyed by stable object identity \
     rather than row index or transient position; streaming inserts, sort/filter/pagination \
     refresh, background indexing, and extension-view replacement preserve focus and selection by \
     stable id and never steal focus from the active task; when the focused object disappears, \
     focus moves to the nearest safe sibling or parent and the reason is announced; every dialog, \
     sheet, palette, popover, inline rename, placeholder card, inspector, pane close, and split \
     reflow records a focus-return target and returns to the invoking control, its row, or the \
     nearest safe ancestor or sibling, never the document body and never an off-screen surface or a \
     different window; the keyboard model is single-tab-stop or roving-tabindex with Arrow moving \
     the current item, Space toggling selection where supported, Enter triggering the discoverable \
     default action, and Home/End/Page preserving anchor semantics without silently firing \
     destructive actions; selected-count narration, position-in-set cues, and blocked/read-only \
     row cues hold across normal, high-contrast, and zoomed layouts; a posture that cannot prove a \
     pillar, or that sits on a binding surface whose own marker is below Stable, is narrowed below \
     Stable with a named reason rather than inheriting an adjacent green row; the same posture opens \
     from the activity center, command palette, status bar, and a menu command, keyboard-first; and \
     every posture stays available without an account or managed services.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a present (non-canonical) ref.
const MAX_REF_CHARS: usize = 200;

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn is_present_ref(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_REF_CHARS
}

fn require_canonical_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

fn require_present_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_present_ref(value) {
        Ok(())
    } else {
        Err(BuildError::MissingUpstreamRef { field })
    }
}

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Dense shell surface family a posture covers. The claimed-stable matrix must
/// cover at least one tree, one virtualized list, one grid, one palette-like
/// surface, and one inspector/detail workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionSurfaceClass {
    /// Hierarchical tree with stable node identity.
    Tree,
    /// Dense, virtualized, or paged row list.
    VirtualizedList,
    /// Columnar grid or table.
    Grid,
    /// Command-palette / quick-open style result list.
    PaletteLike,
    /// Inspector / detail workflow bound to a selected object.
    InspectorDetail,
}

impl InteractionSurfaceClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tree => "tree",
            Self::VirtualizedList => "virtualized_list",
            Self::Grid => "grid",
            Self::PaletteLike => "palette_like",
            Self::InspectorDetail => "inspector_detail",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Tree => "tree",
            Self::VirtualizedList => "virtualized list",
            Self::Grid => "grid",
            Self::PaletteLike => "palette-like surface",
            Self::InspectorDetail => "inspector/detail workflow",
        }
    }

    /// Every surface family the claimed-stable matrix must cover.
    pub const REQUIRED: [Self; 5] = [
        Self::Tree,
        Self::VirtualizedList,
        Self::Grid,
        Self::PaletteLike,
        Self::InspectorDetail,
    ];
}

/// The five coordination states that must be modeled distinctly on every dense
/// shell surface. They are tracked separately so activation never silently
/// mutates selection and so virtualization never collapses focus onto selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoordinationStateKind {
    /// Where keyboard input is delivered.
    Focus,
    /// The roving current item (the active row/node/cell).
    CurrentItem,
    /// The durable batch selection set.
    Selection,
    /// The selection range anchor.
    Anchor,
    /// The last discoverable default action fired.
    Activation,
}

impl CoordinationStateKind {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Focus => "focus",
            Self::CurrentItem => "current_item",
            Self::Selection => "selection",
            Self::Anchor => "anchor",
            Self::Activation => "activation",
        }
    }

    /// The five coordination states every Stable posture must model distinctly.
    pub const REQUIRED: [Self; 5] = [
        Self::Focus,
        Self::CurrentItem,
        Self::Selection,
        Self::Anchor,
        Self::Activation,
    ];
}

/// Asynchronous-update class that must preserve focus and selection by stable
/// identity and must never steal focus from the active task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AsyncUpdateClass {
    /// New rows streamed into a list while the user works.
    StreamingInsert,
    /// Sort, filter, or pagination refresh replaces the visible window.
    SortFilterRefresh,
    /// Background indexing changes counts, badges, or availability.
    BackgroundIndexing,
    /// An extension-owned view is replaced or re-mounted.
    ExtensionViewReplacement,
    /// A notification or banner posts while the user is focused elsewhere.
    NotificationBanner,
    /// A multi-window update arrives from another window.
    MultiWindowUpdate,
}

impl AsyncUpdateClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StreamingInsert => "streaming_insert",
            Self::SortFilterRefresh => "sort_filter_refresh",
            Self::BackgroundIndexing => "background_indexing",
            Self::ExtensionViewReplacement => "extension_view_replacement",
            Self::NotificationBanner => "notification_banner",
            Self::MultiWindowUpdate => "multi_window_update",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::StreamingInsert => "Streamed row insert",
            Self::SortFilterRefresh => "Sort/filter refresh",
            Self::BackgroundIndexing => "Background indexing",
            Self::ExtensionViewReplacement => "Extension-view replacement",
            Self::NotificationBanner => "Notification / banner",
            Self::MultiWindowUpdate => "Multi-window update",
        }
    }

    /// The asynchronous-update classes a Stable posture must prove.
    pub const REQUIRED: [Self; 5] = [
        Self::StreamingInsert,
        Self::SortFilterRefresh,
        Self::BackgroundIndexing,
        Self::ExtensionViewReplacement,
        Self::MultiWindowUpdate,
    ];
}

/// Where focus moves when the currently focused object disappears under an
/// asynchronous update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisappearanceResolution {
    /// The focused object cannot disappear under this update class.
    NotApplicable,
    /// Focus moves to the nearest safe sibling row/node/cell.
    NearestSafeSibling,
    /// Focus moves up to the parent node/group.
    ParentNode,
    /// Focus is dropped to the document body (forbidden).
    DocumentBody,
}

impl DisappearanceResolution {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::NearestSafeSibling => "nearest_safe_sibling",
            Self::ParentNode => "parent_node",
            Self::DocumentBody => "document_body",
        }
    }

    /// True when the resolution lands on a safe in-surface target.
    pub const fn is_safe(self) -> bool {
        matches!(self, Self::NearestSafeSibling | Self::ParentNode)
    }
}

/// Transient surface that steals focus and therefore owns a focus-return rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusReturnTrigger {
    /// Modal dialog confirm or cancel.
    DialogConfirmCancel,
    /// Sheet dismiss, cancel, or apply-without-navigation.
    SheetDismiss,
    /// Command palette / quick-open dismiss.
    PaletteDismiss,
    /// Popover dismiss.
    PopoverDismiss,
    /// Inline rename commit or cancel.
    InlineRenameCommitCancel,
    /// Inspector / detail pane dismiss.
    InspectorDismiss,
    /// Pane close.
    PaneClose,
    /// Split reflow / responsive fallback replacement.
    SplitReflow,
    /// Extension-view removal.
    ExtensionViewRemoval,
    /// Missing-dependency placeholder card replacement.
    MissingDependencyPlaceholderReplacement,
}

impl FocusReturnTrigger {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DialogConfirmCancel => "dialog_confirm_cancel",
            Self::SheetDismiss => "sheet_dismiss",
            Self::PaletteDismiss => "palette_dismiss",
            Self::PopoverDismiss => "popover_dismiss",
            Self::InlineRenameCommitCancel => "inline_rename_commit_cancel",
            Self::InspectorDismiss => "inspector_dismiss",
            Self::PaneClose => "pane_close",
            Self::SplitReflow => "split_reflow",
            Self::ExtensionViewRemoval => "extension_view_removal",
            Self::MissingDependencyPlaceholderReplacement => {
                "missing_dependency_placeholder_replacement"
            }
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DialogConfirmCancel => "Dialog confirm/cancel",
            Self::SheetDismiss => "Sheet dismiss",
            Self::PaletteDismiss => "Palette dismiss",
            Self::PopoverDismiss => "Popover dismiss",
            Self::InlineRenameCommitCancel => "Inline rename commit/cancel",
            Self::InspectorDismiss => "Inspector dismiss",
            Self::PaneClose => "Pane close",
            Self::SplitReflow => "Split reflow",
            Self::ExtensionViewRemoval => "Extension-view removal",
            Self::MissingDependencyPlaceholderReplacement => "Missing-dependency placeholder",
        }
    }

    /// The focus-return drills a Stable posture must cover.
    pub const REQUIRED: [Self; 7] = [
        Self::DialogConfirmCancel,
        Self::SheetDismiss,
        Self::PaneClose,
        Self::InlineRenameCommitCancel,
        Self::ExtensionViewRemoval,
        Self::MissingDependencyPlaceholderReplacement,
        Self::SplitReflow,
    ];
}

/// Keyboard navigation model a dense surface presents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardModelClass {
    /// One tab stop owns the surface; Arrow keys move the current item.
    SingleTabStop,
    /// Roving tabindex moves the single tab stop with the current item.
    RovingTabindex,
}

impl KeyboardModelClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleTabStop => "single_tab_stop",
            Self::RovingTabindex => "roving_tabindex",
        }
    }
}

/// Per-OS desktop profile a conformance row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformProfileClass {
    /// macOS (universal).
    #[serde(rename = "macos")]
    MacOs,
    /// Windows (x86_64).
    Windows,
    /// Linux (GNOME/Wayland, x86_64).
    Linux,
}

impl PlatformProfileClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MacOs => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }

    /// Every per-OS profile a Stable conformance posture must cover.
    pub const REQUIRED: [Self; 3] = [Self::MacOs, Self::Windows, Self::Linux];
}

/// Surface that ingests the shared interaction-parity record. The same record
/// drives the shell collection surface, the keyboard-help reference, the CLI
/// inspector, Help/About, and the support export rather than each cloning prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionTruthSurface {
    /// The live dense shell collection surface.
    ShellCollectionSurface,
    /// The keyboard-help / shortcut reference.
    KeyboardHelp,
    /// The CLI / headless inspector.
    CliInspect,
    /// The Help/About interaction posture.
    HelpAbout,
    /// The diagnostics support export.
    SupportExport,
}

impl InteractionTruthSurface {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellCollectionSurface => "shell_collection_surface",
            Self::KeyboardHelp => "keyboard_help",
            Self::CliInspect => "cli_inspect",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
        }
    }

    /// The five surfaces that must all bind the shared record.
    pub const REQUIRED: [Self; 5] = [
        Self::ShellCollectionSurface,
        Self::KeyboardHelp,
        Self::CliInspect,
        Self::HelpAbout,
        Self::SupportExport,
    ];
}

/// Closed recovery-action vocabulary exposed on an interaction posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionRecoveryAction {
    /// Open the keyboard-help / shortcut reference for this surface.
    OpenKeyboardHelp,
    /// Review the current selection scope (included / excluded / blocked).
    ReviewSelectionScope,
    /// Restore focus to the last known safe target.
    RestoreFocus,
    /// Open the runtime diagnostics center.
    OpenDiagnostics,
    /// Export a redacted interaction-support packet.
    ExportInteractionSupport,
}

impl InteractionRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenKeyboardHelp => "open_keyboard_help",
            Self::ReviewSelectionScope => "review_selection_scope",
            Self::RestoreFocus => "restore_focus",
            Self::OpenDiagnostics => "open_diagnostics",
            Self::ExportInteractionSupport => "export_interaction_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenKeyboardHelp => "Open keyboard help",
            Self::ReviewSelectionScope => "Review selection scope",
            Self::RestoreFocus => "Restore focus",
            Self::OpenDiagnostics => "Open diagnostics",
            Self::ExportInteractionSupport => "Export interaction support",
        }
    }

    /// Placement / confirmation role for this action.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenKeyboardHelp | Self::ReviewSelectionScope => RecoveryActionRole::Primary,
            Self::RestoreFocus => RecoveryActionRole::Recovery,
            Self::OpenDiagnostics | Self::ExportInteractionSupport => RecoveryActionRole::Secondary,
        }
    }

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }

    /// The recovery actions every posture must expose regardless of surface.
    pub const REQUIRED: [Self; 3] = [
        Self::OpenKeyboardHelp,
        Self::ReviewSelectionScope,
        Self::ExportInteractionSupport,
    ];
}

/// Returns the recovery routes a posture must expose, in rendered order, given
/// whether focus restoration is offered for this surface.
pub fn required_recovery_routes(focus_restorable: bool) -> Vec<RecoveryRouteRecord> {
    let mut actions = vec![
        InteractionRecoveryAction::OpenKeyboardHelp,
        InteractionRecoveryAction::ReviewSelectionScope,
    ];
    if focus_restorable {
        actions.push(InteractionRecoveryAction::RestoreFocus);
    }
    actions.push(InteractionRecoveryAction::OpenDiagnostics);
    actions.push(InteractionRecoveryAction::ExportInteractionSupport);
    actions
        .into_iter()
        .map(InteractionRecoveryAction::route)
        .collect()
}

/// Closed reason a posture is narrowed below Stable. Required whenever the claim
/// class is below the cutline; forbidden when it is Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionNarrowingReason {
    /// Focus, current item, selection, anchor, and activation are not modeled
    /// as distinct stable state objects.
    CoordinationStatesCollapsed,
    /// An asynchronous update loses focus or selection by stable identity.
    IdentityLostUnderAsyncUpdate,
    /// A focus-return drill is missing or returns to an unsafe target.
    FocusReturnIncomplete,
    /// The keyboard model is incomplete or fires destructive actions silently.
    KeyboardModelIncomplete,
    /// An asynchronous update steals focus from the active task.
    AsyncUpdateStealsFocus,
    /// Selected-count, position-in-set, or blocked/read-only cues are missing.
    AccessibilityCuesIncomplete,
    /// Per-OS conformance is incomplete.
    PlatformConformanceIncomplete,
    /// The binding surface's own lifecycle marker is below Stable, so it must not
    /// inherit Stable by adjacency.
    SurfaceNotYetStable,
}

impl InteractionNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoordinationStatesCollapsed => "coordination_states_collapsed",
            Self::IdentityLostUnderAsyncUpdate => "identity_lost_under_async_update",
            Self::FocusReturnIncomplete => "focus_return_incomplete",
            Self::KeyboardModelIncomplete => "keyboard_model_incomplete",
            Self::AsyncUpdateStealsFocus => "async_update_steals_focus",
            Self::AccessibilityCuesIncomplete => "accessibility_cues_incomplete",
            Self::PlatformConformanceIncomplete => "platform_conformance_incomplete",
            Self::SurfaceNotYetStable => "surface_not_yet_stable",
        }
    }
}

// ---------------------------------------------------------------------------
// Per-pillar evidence blocks
// ---------------------------------------------------------------------------

/// The distinct coordination-state model for one dense surface. Focus, current
/// item, selection, anchor, and activation are tracked separately, keyed by
/// stable object identity rather than row index or transient position.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoordinationStateModel {
    /// Stable object id of the focused object, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_object_id_ref: Option<String>,
    /// Stable object id of the current (roving) item, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_item_id_ref: Option<String>,
    /// Stable object ids in the durable selection set, in canonical order.
    pub selection_object_id_refs: Vec<String>,
    /// Stable object id of the selection anchor, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_object_id_ref: Option<String>,
    /// Stable object id of the last activated object, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_activated_object_id_ref: Option<String>,
    /// Whether the five coordination states are modeled as distinct state
    /// objects (never collapsed onto a single value).
    pub states_modeled_distinctly: bool,
    /// Whether activation acts on the current object without changing selection.
    pub activation_preserves_selection: bool,
    /// Whether identity is keyed by stable object id, not row index or position.
    pub identity_by_stable_id_not_index: bool,
}

impl CoordinationStateModel {
    /// True when the coordination model keeps the five states distinct, keeps
    /// activation from mutating selection, and keys identity by stable id.
    pub fn holds(&self) -> bool {
        self.states_modeled_distinctly
            && self.activation_preserves_selection
            && self.identity_by_stable_id_not_index
    }
}

/// One asynchronous-update row's focus/selection-preservation evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsyncUpdateRow {
    /// The asynchronous-update class.
    pub update_class: AsyncUpdateClass,
    /// Whether focus is preserved by stable object id across the update.
    pub preserves_focus_by_stable_id: bool,
    /// Whether selection is preserved by stable object id across the update.
    pub preserves_selection_by_stable_id: bool,
    /// Whether the selection anchor is preserved across the update.
    pub preserves_anchor: bool,
    /// Whether the update steals focus from the active task (must be false).
    pub steals_focus_from_active_task: bool,
    /// Whether the focused object can disappear under this update class.
    pub focused_object_can_disappear: bool,
    /// Where focus moves when the focused object disappears.
    pub disappearance_resolution: DisappearanceResolution,
    /// Whether the reason for the focus move is announced.
    pub announces_focus_move_reason: bool,
    /// Reviewable user-impact sentence.
    pub user_impact_label: String,
}

impl AsyncUpdateRow {
    /// True when focus, selection, and anchor are preserved by stable identity.
    pub fn preserves_identity(&self) -> bool {
        self.preserves_focus_by_stable_id
            && self.preserves_selection_by_stable_id
            && self.preserves_anchor
    }

    /// True when the update never steals focus, and when the focused object can
    /// disappear it resolves to a safe in-surface target with an announcement.
    pub fn never_steals_focus(&self) -> bool {
        if self.steals_focus_from_active_task {
            return false;
        }
        if self.focused_object_can_disappear {
            self.disappearance_resolution.is_safe() && self.announces_focus_move_reason
        } else {
            true
        }
    }

    /// True when the row both preserves identity and never steals focus.
    pub fn holds(&self) -> bool {
        self.preserves_identity() && self.never_steals_focus()
    }
}

/// One focus-return rule for a transient surface that steals focus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FocusReturnRow {
    /// The triggering transient surface.
    pub trigger: FocusReturnTrigger,
    /// Stable rule id.
    pub rule_id: String,
    /// Stable object id that owns the focus before the transient surface opens.
    pub origin_object_id_ref: String,
    /// Stable id of the invoking control that opened the transient surface.
    pub invoking_control_id_ref: String,
    /// Stable id of the preferred post-close focus target.
    pub expected_return_target_id_ref: String,
    /// Stable id of the fallback target when the original target vanished.
    pub fallback_return_target_id_ref: String,
    /// Whether focus returns to the invoker, its row, or a safe ancestor/sibling.
    pub returns_to_invoker_or_safe_ancestor: bool,
    /// Whether focus never returns to the document body.
    pub never_returns_to_document_body: bool,
    /// Whether focus never returns to an off-screen surface.
    pub never_returns_to_offscreen_surface: bool,
    /// Whether focus never warps to a different window.
    pub never_warps_across_windows: bool,
    /// Whether selection or cursor state is preserved across the return.
    pub preserves_selection_or_cursor_state: bool,
    /// Screen-reader announcement emitted on return.
    pub screen_reader_announcement: String,
}

impl FocusReturnRow {
    /// True when the rule returns to a safe target and never drops focus to the
    /// document body, an off-screen surface, or a different window.
    pub fn holds(&self) -> bool {
        self.returns_to_invoker_or_safe_ancestor
            && self.never_returns_to_document_body
            && self.never_returns_to_offscreen_surface
            && self.never_warps_across_windows
            && self.preserves_selection_or_cursor_state
    }
}

/// The keyboard-navigation model for one dense surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardModelRow {
    /// The keyboard model class.
    pub model_class: KeyboardModelClass,
    /// Whether the surface presents a single tab stop.
    pub single_tab_stop: bool,
    /// Whether Arrow keys move the current item.
    pub arrow_moves_current_item: bool,
    /// Whether the surface supports per-item selection.
    pub selection_supported: bool,
    /// Whether Space toggles selection (where selection is supported).
    pub space_toggles_selection: bool,
    /// Whether Enter triggers the default action.
    pub enter_triggers_default_action: bool,
    /// Whether the default action is discoverable (named, not hover-only).
    pub default_action_discoverable: bool,
    /// Whether Home/End/Page navigation preserves anchor semantics.
    pub home_end_page_preserves_anchor: bool,
    /// Whether navigation never silently fires a destructive action.
    pub no_silent_destructive_activation: bool,
}

impl KeyboardModelRow {
    /// True when the keyboard model is complete: single tab stop / roving
    /// tabindex, Arrow moves current item, Space toggles selection where
    /// supported, Enter triggers a discoverable default, Home/End/Page preserve
    /// the anchor, and navigation never silently activates a destructive action.
    pub fn holds(&self) -> bool {
        let selection_ok = !self.selection_supported || self.space_toggles_selection;
        self.single_tab_stop
            && self.arrow_moves_current_item
            && selection_ok
            && self.enter_triggers_default_action
            && self.default_action_discoverable
            && self.home_end_page_preserves_anchor
            && self.no_silent_destructive_activation
    }
}

/// Accessibility cue evidence for one dense surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionA11yCues {
    /// Whether the selected count is narrated.
    pub selected_count_narrated: bool,
    /// Reviewable selected-count label (e.g. "3 of 128 selected").
    pub selected_count_label: String,
    /// Whether position-in-set is narrated.
    pub position_in_set_narrated: bool,
    /// Position of the current item within the set (1-based).
    pub position_in_set: u64,
    /// Total size of the set.
    pub set_size: u64,
    /// Reviewable position-in-set label (e.g. "3 of 128").
    pub position_in_set_label: String,
    /// Whether a cue is present for blocked rows.
    pub blocked_row_cue_present: bool,
    /// Whether a cue is present for read-only rows.
    pub read_only_row_cue_present: bool,
    /// Whether the roving-tabindex behavior is narrated.
    pub roving_tabindex_narrated: bool,
}

impl InteractionA11yCues {
    /// True when selected-count, position-in-set, blocked/read-only, and
    /// roving-tabindex cues are all present.
    pub fn holds(&self) -> bool {
        self.selected_count_narrated
            && self.position_in_set_narrated
            && self.blocked_row_cue_present
            && self.read_only_row_cue_present
            && self.roving_tabindex_narrated
    }
}

/// Per-OS conformance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformConformanceRow {
    /// The per-OS profile.
    pub profile: PlatformProfileClass,
    /// Stable profile id (e.g. `macos_15_plus_universal`).
    pub profile_id: String,
    /// Whether the profile is covered with current proof.
    pub covered: bool,
    /// Source proof ref.
    pub proof_ref: String,
    /// Named focus / keyboard behaviors exercised on this profile.
    pub named_behaviors: Vec<String>,
}

/// Input form of one binding-surface projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractionSurfaceProjectionInput {
    /// The binding surface.
    pub surface: InteractionTruthSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Whether the surface reads the shared record rather than cloning prose.
    pub reads_shared_record: bool,
}

/// Output form of one binding-surface projection, with a derived summary line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionSurfaceProjection {
    /// The binding surface.
    pub surface: InteractionTruthSurface,
    /// The surface's own lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Whether the surface reads the shared record rather than cloning prose.
    pub reads_shared_record: bool,
    /// Derived, deterministic summary line the surface renders.
    pub summary_line: String,
}

/// The proven pillars of one interaction posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionPillars {
    /// Whether focus, current item, selection, anchor, and activation are
    /// modeled as distinct stable state objects.
    pub coordination_states_distinct: bool,
    /// Whether focus and selection survive asynchronous updates by stable id.
    pub identity_survives_async_updates: bool,
    /// Whether every focus-return drill is present and returns to a safe target.
    pub focus_return_complete: bool,
    /// Whether the keyboard model is complete and safe.
    pub keyboard_model_complete: bool,
    /// Whether asynchronous updates never steal focus from the active task.
    pub async_never_steals_focus: bool,
    /// Whether selected-count, position-in-set, and blocked/read-only cues hold.
    pub accessibility_cues_complete: bool,
    /// Whether per-OS conformance is complete.
    pub platform_conformance_complete: bool,
}

/// The public claim ceiling: what a posture is allowed to assert. Each field
/// must be provable from the posture's real evidence; the builder enforces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InteractionClaimCeiling {
    /// Whether the posture may claim distinct coordination states.
    pub asserts_coordination_states_distinct: bool,
    /// Whether the posture may claim identity survives asynchronous updates.
    pub asserts_identity_survives_async_updates: bool,
    /// Whether the posture may claim focus-return completeness.
    pub asserts_focus_return_complete: bool,
    /// Whether the posture may claim keyboard-model completeness.
    pub asserts_keyboard_model_complete: bool,
    /// Whether the posture may claim asynchronous updates never steal focus.
    pub asserts_async_never_steals_focus: bool,
    /// Whether the posture may claim complete accessibility cues.
    pub asserts_accessibility_cues_complete: bool,
    /// Whether the posture may claim complete per-OS conformance.
    pub asserts_platform_conformance_complete: bool,
}

/// The derived stable-claim verdict for a posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionQualification {
    /// The derived claim class (Stable when fully qualified, else narrowed).
    pub claim_class: StableClaimClass,
    /// Whether the posture qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// The reasons the posture is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<InteractionNarrowingReason>,
}

/// Upstream ids the record is a genuine projection of, kept for support
/// traceability. These are upstream source refs, not canonical durable objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionUpstream {
    /// Interaction-integrity packet id this record projects from.
    pub interaction_packet_ref: String,
    /// Interaction-integrity shared contract ref this record projects from.
    pub interaction_contract_ref: String,
    /// Contributing object ids the coordination model projects from, in order.
    pub contributing_object_refs: Vec<String>,
}

/// Validated input used to mint an [`InteractionParityRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractionParityInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The dense surface family this posture covers.
    pub surface_class: InteractionSurfaceClass,
    /// Stable id of the live dense shell surface.
    pub surface_id_ref: String,
    /// The distinct coordination-state model.
    pub coordination: CoordinationStateModel,
    /// The asynchronous-update rows, in canonical order.
    pub async_updates: Vec<AsyncUpdateRow>,
    /// The focus-return rows, in canonical order.
    pub focus_returns: Vec<FocusReturnRow>,
    /// The keyboard-navigation model.
    pub keyboard_model: KeyboardModelRow,
    /// The accessibility cue evidence.
    pub a11y_cues: InteractionA11yCues,
    /// The per-OS conformance rows.
    pub platform_conformance: Vec<PlatformConformanceRow>,
    /// The binding-surface projections.
    pub surface_projections: Vec<InteractionSurfaceProjectionInput>,
    /// Public claim ceiling.
    pub claim_ceiling: InteractionClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the same posture.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the posture stays available without an account.
    pub available_without_account: bool,
    /// Whether the posture stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: InteractionUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed interaction-parity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionParityRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The dense surface family this posture covers.
    pub surface_class: InteractionSurfaceClass,
    /// Stable id of the live dense shell surface.
    pub surface_id_ref: String,
    /// The lowest binding-surface lifecycle marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// The distinct coordination-state model.
    pub coordination: CoordinationStateModel,
    /// The asynchronous-update rows, in canonical order.
    pub async_updates: Vec<AsyncUpdateRow>,
    /// The focus-return rows, in canonical order.
    pub focus_returns: Vec<FocusReturnRow>,
    /// The keyboard-navigation model.
    pub keyboard_model: KeyboardModelRow,
    /// The accessibility cue evidence.
    pub a11y_cues: InteractionA11yCues,
    /// The per-OS conformance rows, in canonical order.
    pub platform_conformance: Vec<PlatformConformanceRow>,
    /// The binding-surface projections, in canonical order.
    pub surface_projections: Vec<InteractionSurfaceProjection>,
    /// The proven pillars.
    pub pillars: InteractionPillars,
    /// Public claim ceiling.
    pub claim_ceiling: InteractionClaimCeiling,
    /// The derived stable-claim verdict.
    pub stable_qualification: InteractionQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the same posture.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the posture stays available without an account.
    pub available_without_account: bool,
    /// Whether the posture stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or below-stable to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: InteractionUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons an [`InteractionParityRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence {
        /// The offending field.
        field: &'static str,
    },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef {
        /// The offending field.
        field: &'static str,
        /// The value that failed.
        value: String,
    },
    /// A required upstream ref was missing.
    MissingUpstreamRef {
        /// The offending field.
        field: &'static str,
    },
    /// A required asynchronous-update class was missing.
    MissingAsyncUpdateClass {
        /// The missing class.
        class: AsyncUpdateClass,
    },
    /// A required focus-return trigger was missing.
    MissingFocusReturnTrigger {
        /// The missing trigger.
        trigger: FocusReturnTrigger,
    },
    /// A required per-OS profile was missing.
    MissingPlatformProfile {
        /// The missing profile.
        profile: PlatformProfileClass,
    },
    /// A per-OS profile lacked current proof.
    PlatformProofMissing {
        /// The under-proven profile.
        profile: PlatformProfileClass,
    },
    /// The posture over-claims distinct coordination states.
    OverclaimsCoordinationStates,
    /// The posture over-claims identity survival under asynchronous updates.
    OverclaimsIdentitySurvival,
    /// The posture over-claims focus-return completeness.
    OverclaimsFocusReturn,
    /// The posture over-claims keyboard-model completeness.
    OverclaimsKeyboardModel,
    /// The posture over-claims that asynchronous updates never steal focus.
    OverclaimsAsyncNeverStealsFocus,
    /// The posture over-claims accessibility-cue completeness.
    OverclaimsAccessibilityCues,
    /// The posture over-claims per-OS conformance.
    OverclaimsPlatformConformance,
    /// A required recovery route was missing.
    MissingRecoveryRoute {
        /// The missing action.
        action: InteractionRecoveryAction,
    },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable {
        /// The offending action id.
        action_id: String,
    },
    /// A binding surface was projected more than once.
    DuplicateSurfaceProjection {
        /// The duplicated surface.
        surface: InteractionTruthSurface,
    },
    /// A binding surface clones prose instead of reading the shared record.
    SurfaceClonesProse {
        /// The offending surface.
        surface: InteractionTruthSurface,
    },
    /// A required binding surface was missing.
    SurfaceProjectionMissing {
        /// The missing surface.
        surface: InteractionTruthSurface,
    },
    /// An entry-route surface was projected more than once.
    DuplicateRouteSurface {
        /// The duplicated surface.
        surface: AttentionRouteSurface,
    },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable {
        /// The offending surface.
        surface: AttentionRouteSurface,
    },
    /// An entry route activates a different item.
    RouteTargetsDifferentItem {
        /// The offending surface.
        surface: AttentionRouteSurface,
    },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing {
        /// The missing surface.
        surface: AttentionRouteSurface,
    },
    /// Accessibility action labels drift from the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// A required layout-mode disclosure was missing.
    AccessibilityLayoutModeMissing {
        /// The missing mode.
        mode: LayoutMode,
    },
    /// A layout-mode disclosure was unreachable.
    AccessibilityLayoutModeUnreachable {
        /// The unreachable mode.
        mode: LayoutMode,
    },
    /// The posture would be hidden without an account.
    HiddenWithoutAccount,
    /// The posture would be hidden without managed services.
    HiddenWithoutManagedServices,
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field {field} is not a reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field {field} value {value:?} is not a canonical object ref"
                )
            }
            Self::MissingUpstreamRef { field } => write!(f, "missing upstream ref for {field}"),
            Self::MissingAsyncUpdateClass { class } => {
                write!(f, "missing async-update class {}", class.as_str())
            }
            Self::MissingFocusReturnTrigger { trigger } => {
                write!(f, "missing focus-return trigger {}", trigger.as_str())
            }
            Self::MissingPlatformProfile { profile } => {
                write!(f, "missing per-OS profile {}", profile.as_str())
            }
            Self::PlatformProofMissing { profile } => {
                write!(f, "per-OS profile {} lacks current proof", profile.as_str())
            }
            Self::OverclaimsCoordinationStates => {
                write!(f, "claims distinct coordination states not proven")
            }
            Self::OverclaimsIdentitySurvival => {
                write!(f, "claims identity survival under async updates not proven")
            }
            Self::OverclaimsFocusReturn => write!(f, "claims focus-return completeness not proven"),
            Self::OverclaimsKeyboardModel => {
                write!(f, "claims keyboard-model completeness not proven")
            }
            Self::OverclaimsAsyncNeverStealsFocus => {
                write!(f, "claims async updates never steal focus, but one does")
            }
            Self::OverclaimsAccessibilityCues => {
                write!(f, "claims accessibility-cue completeness not proven")
            }
            Self::OverclaimsPlatformConformance => {
                write!(f, "claims per-OS conformance not complete")
            }
            Self::MissingRecoveryRoute { action } => {
                write!(f, "missing recovery route {}", action.as_str())
            }
            Self::RecoveryRouteNotKeyboardReachable { action_id } => {
                write!(f, "recovery route {action_id} not keyboard reachable")
            }
            Self::DuplicateSurfaceProjection { surface } => {
                write!(f, "duplicate surface projection {}", surface.as_str())
            }
            Self::SurfaceClonesProse { surface } => {
                write!(f, "surface {} clones prose", surface.as_str())
            }
            Self::SurfaceProjectionMissing { surface } => {
                write!(f, "missing binding surface {}", surface.as_str())
            }
            Self::DuplicateRouteSurface { surface } => {
                write!(f, "duplicate route surface {}", surface.as_str())
            }
            Self::RouteNotKeyboardReachable { surface } => {
                write!(f, "route {} not keyboard reachable", surface.as_str())
            }
            Self::RouteTargetsDifferentItem { surface } => {
                write!(f, "route {} activates a different item", surface.as_str())
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "missing route surface {}", surface.as_str())
            }
            Self::AccessibilityActionLabelsMismatch => {
                write!(f, "accessibility action labels drift from recovery routes")
            }
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(f, "missing layout mode {}", mode.as_str())
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => {
                write!(f, "layout mode {} unreachable", mode.as_str())
            }
            Self::HiddenWithoutAccount => write!(f, "posture hidden without an account"),
            Self::HiddenWithoutManagedServices => {
                write!(f, "posture hidden without managed services")
            }
        }
    }
}

impl std::error::Error for BuildError {}

impl InteractionParityRecord {
    /// Builds a governed interaction-parity record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about coordination-state distinctness, identity survival under
    /// asynchronous updates, focus-return completeness, the keyboard model,
    /// focus theft, accessibility cues, per-OS coverage, recovery routes,
    /// binding surfaces, route reachability, or accessibility. The stable claim
    /// class is *derived* from the evidence, so a posture can never publish a
    /// claim wider than its proof.
    pub fn build(input: InteractionParityInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        if !is_reviewable_sentence(&input.title) {
            return Err(BuildError::InvalidSentence { field: "title" });
        }
        if !is_reviewable_sentence(&input.summary) {
            return Err(BuildError::InvalidSentence { field: "summary" });
        }
        if !is_reviewable_sentence(&input.posture_label) {
            return Err(BuildError::InvalidSentence {
                field: "posture_label",
            });
        }
        require_canonical_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_canonical_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_canonical_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_canonical_ref("narrative_refs", narrative)?;
        }
        require_present_ref(
            "upstream.interaction_packet_ref",
            &input.upstream.interaction_packet_ref,
        )?;
        require_present_ref(
            "upstream.interaction_contract_ref",
            &input.upstream.interaction_contract_ref,
        )?;

        // --- coverage: required async-update classes -------------------------
        let present_async: BTreeSet<AsyncUpdateClass> = input
            .async_updates
            .iter()
            .map(|row| row.update_class)
            .collect();
        for required in AsyncUpdateClass::REQUIRED {
            if !present_async.contains(&required) {
                return Err(BuildError::MissingAsyncUpdateClass { class: required });
            }
        }

        // --- coverage: required focus-return triggers ------------------------
        let present_triggers: BTreeSet<FocusReturnTrigger> =
            input.focus_returns.iter().map(|row| row.trigger).collect();
        for required in FocusReturnTrigger::REQUIRED {
            if !present_triggers.contains(&required) {
                return Err(BuildError::MissingFocusReturnTrigger { trigger: required });
            }
        }

        // --- per-OS conformance: every profile present with current proof ----
        for required in PlatformProfileClass::REQUIRED {
            let row = input
                .platform_conformance
                .iter()
                .find(|row| row.profile == required)
                .ok_or(BuildError::MissingPlatformProfile { profile: required })?;
            if !row.covered || row.proof_ref.trim().is_empty() {
                return Err(BuildError::PlatformProofMissing { profile: required });
            }
        }
        let platform_conformance_complete = PlatformProfileClass::REQUIRED.iter().all(|profile| {
            input.platform_conformance.iter().any(|row| {
                row.profile == *profile && row.covered && !row.proof_ref.trim().is_empty()
            })
        });

        // --- derive the pillars from the evidence ----------------------------
        let coordination_states_distinct = input.coordination.holds();
        let identity_survives_async_updates = input
            .async_updates
            .iter()
            .all(AsyncUpdateRow::preserves_identity)
            && input.coordination.identity_by_stable_id_not_index;
        let focus_return_complete = input.focus_returns.iter().all(FocusReturnRow::holds)
            && FocusReturnTrigger::REQUIRED
                .iter()
                .all(|t| present_triggers.contains(t));
        let keyboard_model_complete = input.keyboard_model.holds();
        let async_never_steals_focus = input
            .async_updates
            .iter()
            .all(AsyncUpdateRow::never_steals_focus);
        let accessibility_cues_complete = input.a11y_cues.holds();

        // --- claim ceiling: never claim what the product cannot prove --------
        if input.claim_ceiling.asserts_coordination_states_distinct && !coordination_states_distinct
        {
            return Err(BuildError::OverclaimsCoordinationStates);
        }
        if input.claim_ceiling.asserts_identity_survives_async_updates
            && !identity_survives_async_updates
        {
            return Err(BuildError::OverclaimsIdentitySurvival);
        }
        if input.claim_ceiling.asserts_focus_return_complete && !focus_return_complete {
            return Err(BuildError::OverclaimsFocusReturn);
        }
        if input.claim_ceiling.asserts_keyboard_model_complete && !keyboard_model_complete {
            return Err(BuildError::OverclaimsKeyboardModel);
        }
        if input.claim_ceiling.asserts_async_never_steals_focus && !async_never_steals_focus {
            return Err(BuildError::OverclaimsAsyncNeverStealsFocus);
        }
        if input.claim_ceiling.asserts_accessibility_cues_complete && !accessibility_cues_complete {
            return Err(BuildError::OverclaimsAccessibilityCues);
        }
        if input.claim_ceiling.asserts_platform_conformance_complete
            && !platform_conformance_complete
        {
            return Err(BuildError::OverclaimsPlatformConformance);
        }

        // --- recovery routes -------------------------------------------------
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in InteractionRecoveryAction::REQUIRED {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- surface projections ---------------------------------------------
        let mut seen_surfaces: BTreeSet<InteractionTruthSurface> = BTreeSet::new();
        for projection in &input.surface_projections {
            if !seen_surfaces.insert(projection.surface) {
                return Err(BuildError::DuplicateSurfaceProjection {
                    surface: projection.surface,
                });
            }
            if !projection.reads_shared_record {
                return Err(BuildError::SurfaceClonesProse {
                    surface: projection.surface,
                });
            }
        }
        for required in InteractionTruthSurface::REQUIRED {
            if !seen_surfaces.contains(&required) {
                return Err(BuildError::SurfaceProjectionMissing { surface: required });
            }
        }
        let mut surface_projections: Vec<InteractionSurfaceProjection> = Vec::new();
        for required in InteractionTruthSurface::REQUIRED {
            let projection = input
                .surface_projections
                .iter()
                .find(|p| p.surface == required)
                .expect("surface presence checked above");
            surface_projections.push(InteractionSurfaceProjection {
                surface: required,
                surface_marker: projection.surface_marker,
                reads_shared_record: projection.reads_shared_record,
                summary_line: surface_summary_line(required, &input),
            });
        }
        let surface_lifecycle_marker = surface_projections
            .iter()
            .map(|projection| projection.surface_marker)
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- entry routes ----------------------------------------------------
        let mut seen_route_surfaces: Vec<AttentionRouteSurface> = Vec::new();
        for route in &input.routes {
            if seen_route_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_route_surfaces.push(route.surface);
            require_canonical_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_item {
                return Err(BuildError::RouteTargetsDifferentItem {
                    surface: route.surface,
                });
            }
        }
        for required in AttentionRouteSurface::REQUIRED {
            if !seen_route_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability ----------------------------------------------------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- pillars ---------------------------------------------------------
        let pillars = InteractionPillars {
            coordination_states_distinct,
            identity_survives_async_updates,
            focus_return_complete,
            keyboard_model_complete,
            async_never_steals_focus,
            accessibility_cues_complete,
            platform_conformance_complete,
        };

        // --- normalise per-OS conformance + upstream refs --------------------
        let mut platform_conformance = input.platform_conformance;
        platform_conformance.sort_by_key(|row| row.profile);
        let mut contributing_object_refs = input.upstream.contributing_object_refs.clone();
        contributing_object_refs.sort();
        contributing_object_refs.dedup();

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !coordination_states_distinct {
            narrowing_reasons.push(InteractionNarrowingReason::CoordinationStatesCollapsed);
        }
        if !identity_survives_async_updates {
            narrowing_reasons.push(InteractionNarrowingReason::IdentityLostUnderAsyncUpdate);
        }
        if !focus_return_complete {
            narrowing_reasons.push(InteractionNarrowingReason::FocusReturnIncomplete);
        }
        if !keyboard_model_complete {
            narrowing_reasons.push(InteractionNarrowingReason::KeyboardModelIncomplete);
        }
        if !async_never_steals_focus {
            narrowing_reasons.push(InteractionNarrowingReason::AsyncUpdateStealsFocus);
        }
        if !accessibility_cues_complete {
            narrowing_reasons.push(InteractionNarrowingReason::AccessibilityCuesIncomplete);
        }
        if !platform_conformance_complete {
            narrowing_reasons.push(InteractionNarrowingReason::PlatformConformanceIncomplete);
        }
        if surface_lifecycle_marker.is_below_stable() {
            narrowing_reasons.push(InteractionNarrowingReason::SurfaceNotYetStable);
        }
        let qualifies_stable = narrowing_reasons.is_empty();
        let claim_class = if qualifies_stable {
            StableClaimClass::Stable
        } else if narrowing_reasons.len() == 1
            && narrowing_reasons[0] == InteractionNarrowingReason::SurfaceNotYetStable
        {
            match surface_lifecycle_marker {
                LifecycleMarker::Preview => StableClaimClass::Preview,
                _ => StableClaimClass::Beta,
            }
        } else {
            StableClaimClass::Beta
        };
        let stable_qualification = InteractionQualification {
            claim_class,
            qualifies_stable,
            narrowing_reasons,
        };
        let honesty_marker_present =
            !qualifies_stable || surface_lifecycle_marker.is_below_stable();

        Ok(Self {
            record_kind: INTERACTION_PARITY_RECORD_KIND.to_string(),
            schema_version: INTERACTION_PARITY_SCHEMA_VERSION,
            notice: INTERACTION_PARITY_NOTICE.to_string(),
            shared_contract_ref: INTERACTION_PARITY_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            surface_class: input.surface_class,
            surface_id_ref: input.surface_id_ref,
            surface_lifecycle_marker,
            coordination: input.coordination,
            async_updates: input.async_updates,
            focus_returns: input.focus_returns,
            keyboard_model: input.keyboard_model,
            a11y_cues: input.a11y_cues,
            platform_conformance,
            surface_projections,
            pillars,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: InteractionUpstream {
                interaction_packet_ref: input.upstream.interaction_packet_ref,
                interaction_contract_ref: input.upstream.interaction_contract_ref,
                contributing_object_refs,
            },
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("interaction_parity: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!("posture: {} ({})", self.posture_id, self.posture_label),
            format!(
                "surface: {} class={}",
                self.surface_id_ref,
                self.surface_class.as_str()
            ),
            format!(
                "surface_lifecycle_marker: {}",
                self.surface_lifecycle_marker.as_str()
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "pillars: coordination_distinct={} identity_survives={} focus_return={} keyboard_model={} async_no_steal={} a11y_cues={} platform_conformance={}",
                self.pillars.coordination_states_distinct,
                self.pillars.identity_survives_async_updates,
                self.pillars.focus_return_complete,
                self.pillars.keyboard_model_complete,
                self.pillars.async_never_steals_focus,
                self.pillars.accessibility_cues_complete,
                self.pillars.platform_conformance_complete
            ),
            format!(
                "coordination: focus={} current_item={} anchor={} last_activated={} selection=[{}] distinct={} activation_preserves_selection={} stable_id_not_index={}",
                self.coordination.focus_object_id_ref.as_deref().unwrap_or("-"),
                self.coordination.current_item_id_ref.as_deref().unwrap_or("-"),
                self.coordination.anchor_object_id_ref.as_deref().unwrap_or("-"),
                self.coordination
                    .last_activated_object_id_ref
                    .as_deref()
                    .unwrap_or("-"),
                self.coordination.selection_object_id_refs.join(", "),
                self.coordination.states_modeled_distinctly,
                self.coordination.activation_preserves_selection,
                self.coordination.identity_by_stable_id_not_index
            ),
            format!(
                "keyboard_model: model={} single_tab_stop={} arrow_moves_current={} space_toggles_selection={} enter_default={} default_discoverable={} home_end_page_preserves_anchor={} no_silent_destructive={}",
                self.keyboard_model.model_class.as_str(),
                self.keyboard_model.single_tab_stop,
                self.keyboard_model.arrow_moves_current_item,
                self.keyboard_model.space_toggles_selection,
                self.keyboard_model.enter_triggers_default_action,
                self.keyboard_model.default_action_discoverable,
                self.keyboard_model.home_end_page_preserves_anchor,
                self.keyboard_model.no_silent_destructive_activation
            ),
            format!(
                "a11y_cues: selected_count={:?} position_in_set={:?} ({} of {}) blocked_cue={} read_only_cue={} roving_tabindex_narrated={}",
                self.a11y_cues.selected_count_label,
                self.a11y_cues.position_in_set_label,
                self.a11y_cues.position_in_set,
                self.a11y_cues.set_size,
                self.a11y_cues.blocked_row_cue_present,
                self.a11y_cues.read_only_row_cue_present,
                self.a11y_cues.roving_tabindex_narrated
            ),
        ];
        lines.push("async_updates:".to_string());
        for row in &self.async_updates {
            lines.push(format!(
                "  - {} preserves_focus={} preserves_selection={} preserves_anchor={} steals_focus={} disappearance={} announces={} :: {}",
                row.update_class.as_str(),
                row.preserves_focus_by_stable_id,
                row.preserves_selection_by_stable_id,
                row.preserves_anchor,
                row.steals_focus_from_active_task,
                row.disappearance_resolution.as_str(),
                row.announces_focus_move_reason,
                row.user_impact_label
            ));
        }
        lines.push("focus_returns:".to_string());
        for row in &self.focus_returns {
            lines.push(format!(
                "  - {} return={} fallback={} safe_ancestor={} not_body={} not_offscreen={} not_cross_window={} :: {}",
                row.trigger.as_str(),
                row.expected_return_target_id_ref,
                row.fallback_return_target_id_ref,
                row.returns_to_invoker_or_safe_ancestor,
                row.never_returns_to_document_body,
                row.never_returns_to_offscreen_surface,
                row.never_warps_across_windows,
                row.screen_reader_announcement
            ));
        }
        lines.push("platform_conformance:".to_string());
        for row in &self.platform_conformance {
            lines.push(format!(
                "  - {} profile_id={} covered={} behaviors=[{}] :: {}",
                row.profile.as_str(),
                row.profile_id,
                row.covered,
                row.named_behaviors.join(", "),
                row.proof_ref
            ));
        }
        lines.push("surface_projections:".to_string());
        for projection in &self.surface_projections {
            lines.push(format!(
                "  - {} marker={} reads_shared_record={} :: {}",
                projection.surface.as_str(),
                projection.surface_marker.as_str(),
                projection.reads_shared_record,
                projection.summary_line
            ));
        }
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

fn surface_summary_line(
    surface: InteractionTruthSurface,
    input: &InteractionParityInput,
) -> String {
    let prefix = match surface {
        InteractionTruthSurface::ShellCollectionSurface => "Shell surface",
        InteractionTruthSurface::KeyboardHelp => "Keyboard help",
        InteractionTruthSurface::CliInspect => "CLI inspect",
        InteractionTruthSurface::HelpAbout => "Help/About",
        InteractionTruthSurface::SupportExport => "Support export",
    };
    let selected = input.coordination.selection_object_id_refs.len();
    format!(
        "{prefix}: {} — {} keyboard model, {} selected, focus/current/selection/anchor/activation distinct.",
        input.surface_class.label(),
        input.keyboard_model.model_class.as_str(),
        selected,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordination_model_holds_when_distinct() {
        let model = CoordinationStateModel {
            focus_object_id_ref: Some("o:1".to_string()),
            current_item_id_ref: Some("o:1".to_string()),
            selection_object_id_refs: vec!["o:1".to_string(), "o:2".to_string()],
            anchor_object_id_ref: Some("o:1".to_string()),
            last_activated_object_id_ref: Some("o:1".to_string()),
            states_modeled_distinctly: true,
            activation_preserves_selection: true,
            identity_by_stable_id_not_index: true,
        };
        assert!(model.holds());
        let collapsed = CoordinationStateModel {
            states_modeled_distinctly: false,
            ..model.clone()
        };
        assert!(!collapsed.holds());
    }

    #[test]
    fn async_row_never_steals_when_resolves_safely() {
        let row = AsyncUpdateRow {
            update_class: AsyncUpdateClass::StreamingInsert,
            preserves_focus_by_stable_id: true,
            preserves_selection_by_stable_id: true,
            preserves_anchor: true,
            steals_focus_from_active_task: false,
            focused_object_can_disappear: true,
            disappearance_resolution: DisappearanceResolution::NearestSafeSibling,
            announces_focus_move_reason: true,
            user_impact_label: "label".to_string(),
        };
        assert!(row.holds());
        let body = AsyncUpdateRow {
            disappearance_resolution: DisappearanceResolution::DocumentBody,
            ..row.clone()
        };
        assert!(!body.never_steals_focus());
        let steals = AsyncUpdateRow {
            steals_focus_from_active_task: true,
            ..row.clone()
        };
        assert!(!steals.holds());
    }

    #[test]
    fn focus_return_row_holds_when_safe() {
        let row = FocusReturnRow {
            trigger: FocusReturnTrigger::DialogConfirmCancel,
            rule_id: "r:1".to_string(),
            origin_object_id_ref: "o:1".to_string(),
            invoking_control_id_ref: "c:1".to_string(),
            expected_return_target_id_ref: "o:1".to_string(),
            fallback_return_target_id_ref: "o:parent".to_string(),
            returns_to_invoker_or_safe_ancestor: true,
            never_returns_to_document_body: true,
            never_returns_to_offscreen_surface: true,
            never_warps_across_windows: true,
            preserves_selection_or_cursor_state: true,
            screen_reader_announcement: "back".to_string(),
        };
        assert!(row.holds());
        let to_body = FocusReturnRow {
            never_returns_to_document_body: false,
            ..row.clone()
        };
        assert!(!to_body.holds());
    }

    #[test]
    fn keyboard_model_requires_space_only_when_selection_supported() {
        let base = KeyboardModelRow {
            model_class: KeyboardModelClass::RovingTabindex,
            single_tab_stop: true,
            arrow_moves_current_item: true,
            selection_supported: false,
            space_toggles_selection: false,
            enter_triggers_default_action: true,
            default_action_discoverable: true,
            home_end_page_preserves_anchor: true,
            no_silent_destructive_activation: true,
        };
        assert!(base.holds());
        let selecting = KeyboardModelRow {
            selection_supported: true,
            space_toggles_selection: false,
            ..base.clone()
        };
        assert!(!selecting.holds());
    }

    #[test]
    fn required_recovery_routes_expand_with_restore() {
        let base = required_recovery_routes(false);
        let ids: Vec<&str> = base.iter().map(|r| r.action_id.as_str()).collect();
        for required in InteractionRecoveryAction::REQUIRED {
            assert!(ids.contains(&required.as_str()));
        }
        assert!(!ids.contains(&"restore_focus"));
        let full = required_recovery_routes(true);
        let ids: Vec<String> = full.iter().map(|r| r.action_id.clone()).collect();
        assert!(ids.iter().any(|id| id == "restore_focus"));
    }

    #[test]
    fn required_surface_classes_cover_all_families() {
        assert_eq!(InteractionSurfaceClass::REQUIRED.len(), 5);
        assert!(InteractionSurfaceClass::REQUIRED.contains(&InteractionSurfaceClass::PaletteLike));
        assert!(
            InteractionSurfaceClass::REQUIRED.contains(&InteractionSurfaceClass::InspectorDetail)
        );
    }
}
