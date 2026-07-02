//! Frozen M5 status-bar, transient-inspect, pane-control, and
//! durable-progress-component matrix.
//!
//! This module locks Aureline's canonical M5 shell primitives into one
//! export-safe packet. Every high-frequency shell primitive M5 claims — status
//! bar items, status overflow menus, tooltips, hovercards, peek panels,
//! pinned-preview promotions, splitter handles, pane-resize presets, progress
//! indicators, and durable job-row components — is named once here, bound to a
//! canonical shell zone, responsive class, and window class, and constrained by
//! the same freshness, accessibility, and serialization rules regardless of the
//! surface family that renders it.
//!
//! The shell topology this matrix binds against — the eight canonical shell
//! zones, the compact/standard/expanded responsive classes, the window classes,
//! and the ten claimed M5 surface families — is the one already frozen by
//! [`crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix`];
//! this matrix re-exports that vocabulary rather than minting parallel terms.
//! What this matrix adds is the stable vocabulary for the *primitives*
//! themselves: the primitive families, the status-item classes, the
//! overflow/severe-state displacement behaviors, the source/provider/freshness
//! labels, the hover/peek representation classes, the pinned-preview promotion
//! states, the splitter/pane-resize states, the progress/job-row states, the
//! non-visual accessibility routes, and the mandatory labels every primitive
//! must be able to show.
//!
//! The matrix is the single source of truth for whether a claimed M5 shell
//! primitive may publish a status, inspect, resize, or progress claim. Status
//! bars, hovercards, peek panels, splitters, and activity/progress centers all
//! consume this packet so ambient instrumentation stays overflow-safe, transient
//! inspect surfaces keep their source/provider/freshness truth after pinning,
//! panes resize precisely and serializably from the keyboard, and progress rows
//! stay durable, attributable, and reopenable. No critical truth is hover-only,
//! spinner-only, or lost when a surface compacts, pins, or promotes.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5ShellPrimitiveVocabularySet`] rather than minted per surface. Raw URLs,
//! raw local paths, raw usernames, raw hostnames, tokens, raw diagnostics,
//! private endpoints, credentials, and user text bodies stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/shell/m5-shell-primitives.schema.json`](../../../../schemas/shell/m5-shell-primitives.schema.json)
//! and the contract doc is
//! [`docs/shell/m5_shell_primitives_contract.md`](../../../../docs/shell/m5_shell_primitives_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-shell-primitives/`](../../../../fixtures/ui/m5-shell-primitives/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_shell_primitives_matrix,
    seeded_m5_shell_primitives_matrix_pane_resize_preset_beta_narrowed,
    seeded_m5_shell_primitives_matrix_pinned_preview_promotion_preview_narrowed,
    M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID,
};

// The canonical shell topology — zones, responsive classes, window classes,
// consumer surfaces, and the ten claimed M5 surface families — is frozen once,
// in the shell-zone matrix. This matrix reuses it verbatim so no shell primitive
// invents a parallel slot, layout class, window class, or surface family.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellSurfaceFamily, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ShellPrimitivesMatrixPacket`].
pub const M5_SHELL_PRIMITIVES_MATRIX_RECORD_KIND: &str =
    "freeze_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix";

/// Schema version for M5 shell-primitives-matrix records.
pub const M5_SHELL_PRIMITIVES_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the shell-primitives boundary schema.
pub const M5_SHELL_PRIMITIVES_SCHEMA_REF: &str = "schemas/shell/m5-shell-primitives.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SHELL_PRIMITIVES_DOC_REF: &str = "docs/shell/m5_shell_primitives_contract.md";

/// Repo-relative path of the frozen shell-zone schema this matrix binds against.
pub const M5_SHELL_PRIMITIVES_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen responsive-class schema this matrix binds
/// against.
pub const M5_SHELL_PRIMITIVES_RESPONSIVE_CLASS_REF: &str =
    "schemas/shell/m5-responsive-class.schema.json";

/// Repo-relative path of the frozen multi-window-parity schema this matrix binds
/// against.
pub const M5_SHELL_PRIMITIVES_MULTI_WINDOW_PARITY_REF: &str =
    "schemas/shell/m5-multi-window-parity.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SHELL_PRIMITIVES_FIXTURE_DIR: &str = "fixtures/ui/m5-shell-primitives";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SHELL_PRIMITIVES_ARTIFACT_REF: &str =
    "artifacts/release/m5-shell-primitives-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SHELL_PRIMITIVES_CSV_REF: &str =
    "artifacts/release/m5-shell-primitives-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_SHELL_PRIMITIVES_REPORT_REF: &str = "artifacts/shell/m5-shell-primitives.md";

/// One of the ten governed shell-primitive families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellPrimitiveFamily {
    /// A single status-bar item projecting one ambient truth.
    StatusBarItem,
    /// A status-bar overflow menu that holds displaced/low-priority items.
    StatusOverflowMenu,
    /// A plain tooltip (short label / shortcut hint).
    Tooltip,
    /// A rich hovercard (attributed inspectable detail on hover/focus).
    Hovercard,
    /// A peek panel (inline structured preview of a target).
    PeekPanel,
    /// A pinned-preview promotion (a transient peek promoted to a durable panel).
    PinnedPreviewPromotion,
    /// A splitter handle between resizable panes.
    SplitterHandle,
    /// A named pane-resize preset (a serializable layout ratio).
    PaneResizePreset,
    /// A progress indicator (determinate/indeterminate ambient progress).
    ProgressIndicator,
    /// A durable job-row component in an activity / progress center.
    DurableJobRow,
}

impl M5ShellPrimitiveFamily {
    /// Every governed primitive family, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::StatusBarItem,
        Self::StatusOverflowMenu,
        Self::Tooltip,
        Self::Hovercard,
        Self::PeekPanel,
        Self::PinnedPreviewPromotion,
        Self::SplitterHandle,
        Self::PaneResizePreset,
        Self::ProgressIndicator,
        Self::DurableJobRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatusBarItem => "status_bar_item",
            Self::StatusOverflowMenu => "status_overflow_menu",
            Self::Tooltip => "tooltip",
            Self::Hovercard => "hovercard",
            Self::PeekPanel => "peek_panel",
            Self::PinnedPreviewPromotion => "pinned_preview_promotion",
            Self::SplitterHandle => "splitter_handle",
            Self::PaneResizePreset => "pane_resize_preset",
            Self::ProgressIndicator => "progress_indicator",
            Self::DurableJobRow => "durable_job_row",
        }
    }

    /// `true` when this family is ambient status instrumentation and must
    /// therefore declare status-item classes and overflow behaviors.
    pub const fn is_ambient(self) -> bool {
        matches!(self, Self::StatusBarItem | Self::StatusOverflowMenu)
    }

    /// `true` when this family is a transient inspect surface and must therefore
    /// declare its hover/peek representation classes.
    pub const fn is_transient_inspect(self) -> bool {
        matches!(
            self,
            Self::Tooltip | Self::Hovercard | Self::PeekPanel | Self::PinnedPreviewPromotion
        )
    }

    /// `true` when this family promotes a transient surface to a durable one and
    /// must therefore declare its promotion states.
    pub const fn promotes(self) -> bool {
        matches!(self, Self::PeekPanel | Self::PinnedPreviewPromotion)
    }

    /// `true` when this family controls pane layout and must therefore declare
    /// its pane-resize states.
    pub const fn is_pane_control(self) -> bool {
        matches!(self, Self::SplitterHandle | Self::PaneResizePreset)
    }

    /// `true` when this family carries progress and must therefore declare its
    /// progress/job-row states.
    pub const fn is_progress(self) -> bool {
        matches!(self, Self::ProgressIndicator | Self::DurableJobRow)
    }

    /// `true` when this family carries source/provider/freshness truth and must
    /// therefore declare its source-freshness labels. Pure layout controls
    /// (splitters, presets) carry no freshness.
    pub const fn carries_freshness(self) -> bool {
        self.is_ambient() || self.is_transient_inspect() || self.is_progress()
    }
}

/// Controlled status-item class — the kind of ambient truth a status-bar item or
/// overflow entry projects. A status item may not surface bare chrome without one
/// of these named classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StatusItemClass {
    /// Background work / task activity summary.
    BackgroundWork,
    /// Connection / remote-target identity.
    ConnectionTarget,
    /// Active deployment profile.
    DeploymentProfile,
    /// Sync / freshness of the working set.
    SyncFreshness,
    /// Problem / diagnostic count.
    ProblemCount,
    /// Editor mode / cursor position indicator.
    ModeIndicator,
    /// Notification / attention summary.
    NotificationSummary,
    /// Capacity / quota / resource meter.
    CapacityMeter,
}

impl M5StatusItemClass {
    /// Every status-item class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::BackgroundWork,
        Self::ConnectionTarget,
        Self::DeploymentProfile,
        Self::SyncFreshness,
        Self::ProblemCount,
        Self::ModeIndicator,
        Self::NotificationSummary,
        Self::CapacityMeter,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackgroundWork => "background_work",
            Self::ConnectionTarget => "connection_target",
            Self::DeploymentProfile => "deployment_profile",
            Self::SyncFreshness => "sync_freshness",
            Self::ProblemCount => "problem_count",
            Self::ModeIndicator => "mode_indicator",
            Self::NotificationSummary => "notification_summary",
            Self::CapacityMeter => "capacity_meter",
        }
    }
}

/// Controlled overflow / severe-state displacement behavior for ambient
/// instrumentation. The status bar reflows only through these named behaviors —
/// never around a spinner or a vanity item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverflowBehavior {
    /// A high-priority item is pinned and never displaced.
    PriorityPinned,
    /// Lower-priority items collapse into a keyboard-reachable overflow menu.
    CollapseToOverflowMenu,
    /// Related items group into one summary chip.
    GroupIntoSummary,
    /// A vanity / decorative item is dropped before any truth-bearing item.
    DropVanityItem,
    /// A severe state displaces a vanity item (never a truth-bearing peer).
    PromoteSevereState,
    /// Every overflowed item stays reachable without pointer hover.
    KeyboardReachableOverflow,
}

impl M5OverflowBehavior {
    /// Every overflow behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PriorityPinned,
        Self::CollapseToOverflowMenu,
        Self::GroupIntoSummary,
        Self::DropVanityItem,
        Self::PromoteSevereState,
        Self::KeyboardReachableOverflow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PriorityPinned => "priority_pinned",
            Self::CollapseToOverflowMenu => "collapse_to_overflow_menu",
            Self::GroupIntoSummary => "group_into_summary",
            Self::DropVanityItem => "drop_vanity_item",
            Self::PromoteSevereState => "promote_severe_state",
            Self::KeyboardReachableOverflow => "keyboard_reachable_overflow",
        }
    }
}

/// Controlled source / provider / freshness label. A transient inspect surface or
/// a progress row that shows cached, sampled, or stale content must name it with
/// one of these rather than let a stale preview read as live canonical content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceFreshnessLabel {
    /// Live, canonical content from the source of truth.
    LiveCanonical,
    /// A cached snapshot with a capture time.
    CachedSnapshot,
    /// Stale content explicitly invalidated / marked out of date.
    StaleInvalidated,
    /// Content attributed to a named provider / origin.
    ProviderAttributed,
    /// A sampled / approximate value (not exact).
    SampledApproximate,
    /// A refresh is in flight; the shown value is the prior one.
    RefreshInFlight,
}

impl M5SourceFreshnessLabel {
    /// Every source-freshness label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveCanonical,
        Self::CachedSnapshot,
        Self::StaleInvalidated,
        Self::ProviderAttributed,
        Self::SampledApproximate,
        Self::RefreshInFlight,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveCanonical => "live_canonical",
            Self::CachedSnapshot => "cached_snapshot",
            Self::StaleInvalidated => "stale_invalidated",
            Self::ProviderAttributed => "provider_attributed",
            Self::SampledApproximate => "sampled_approximate",
            Self::RefreshInFlight => "refresh_in_flight",
        }
    }
}

/// Controlled hover/peek representation class — what a transient inspect surface
/// shows and at what fidelity. Truncated content must always keep a reopen path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepresentationClass {
    /// A plain tooltip: short label / shortcut hint only.
    PlainTooltip,
    /// A rich hovercard: attributed inspectable detail.
    RichHovercard,
    /// A structured peek: an inline preview of the target's structure.
    StructuredPeek,
    /// A pinned peek: a promoted peek kept open with its representation truth.
    PinnedPeek,
    /// A provenance strip: source / provider / freshness attribution.
    ProvenanceStrip,
    /// Truncated content that keeps a keyboard-reachable reopen path.
    TruncatedWithReopen,
}

impl M5RepresentationClass {
    /// Every representation class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PlainTooltip,
        Self::RichHovercard,
        Self::StructuredPeek,
        Self::PinnedPeek,
        Self::ProvenanceStrip,
        Self::TruncatedWithReopen,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainTooltip => "plain_tooltip",
            Self::RichHovercard => "rich_hovercard",
            Self::StructuredPeek => "structured_peek",
            Self::PinnedPeek => "pinned_peek",
            Self::ProvenanceStrip => "provenance_strip",
            Self::TruncatedWithReopen => "truncated_with_reopen",
        }
    }
}

/// Controlled promotion state for a transient surface that can be pinned or
/// promoted. Truth is preserved across every transition; a demotion never drops
/// what a pin surfaced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PromotionState {
    /// Transient: shown on hover/focus, dismissed on blur.
    Transient,
    /// Pinned: kept open in place with its representation truth.
    Pinned,
    /// Promoted to a docked panel.
    PromotedToPanel,
    /// Detached to its own window.
    DetachedToWindow,
    /// Demoted back to transient without losing content.
    DemotedToTransient,
    /// Dismissed, with its reopen path preserved.
    DismissedPreserved,
}

impl M5PromotionState {
    /// Every promotion state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Transient,
        Self::Pinned,
        Self::PromotedToPanel,
        Self::DetachedToWindow,
        Self::DemotedToTransient,
        Self::DismissedPreserved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Transient => "transient",
            Self::Pinned => "pinned",
            Self::PromotedToPanel => "promoted_to_panel",
            Self::DetachedToWindow => "detached_to_window",
            Self::DemotedToTransient => "demoted_to_transient",
            Self::DismissedPreserved => "dismissed_preserved",
        }
    }
}

/// Controlled splitter / pane-resize state. Resizing is precise,
/// keyboard-addressable, and serializable; a reset returns to a named default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PaneResizeState {
    /// The handle is idle at a serialized ratio.
    Idle,
    /// The handle is being dragged (pointer).
    Dragging,
    /// The handle is being stepped by keyboard.
    KeyboardStep,
    /// The ratio snapped to a named preset.
    SnappedToPreset,
    /// The ratio reset to its default.
    ResetToDefault,
    /// The pane clamped to its minimum width.
    ClampedToMinWidth,
    /// The pane collapsed to a rail / handle, keeping a reopen path.
    CollapsedToRail,
}

impl M5PaneResizeState {
    /// Every pane-resize state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Idle,
        Self::Dragging,
        Self::KeyboardStep,
        Self::SnappedToPreset,
        Self::ResetToDefault,
        Self::ClampedToMinWidth,
        Self::CollapsedToRail,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Dragging => "dragging",
            Self::KeyboardStep => "keyboard_step",
            Self::SnappedToPreset => "snapped_to_preset",
            Self::ResetToDefault => "reset_to_default",
            Self::ClampedToMinWidth => "clamped_to_min_width",
            Self::CollapsedToRail => "collapsed_to_rail",
        }
    }
}

/// Controlled progress / job-row state. Progress rows are durable, attributable,
/// and reopenable; a completed or failed row never vanishes silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProgressState {
    /// Queued, not yet started.
    Queued,
    /// Running with determinate or indeterminate progress.
    Running,
    /// A grouped batch of related jobs under one row.
    GroupedBatch,
    /// Paused / awaiting input.
    Paused,
    /// Succeeded; the outcome stays in history.
    Succeeded,
    /// Failed; the reason stays reopenable.
    Failed,
    /// Canceled by the user; the row stays in history.
    CanceledByUser,
    /// A completed row reopenable from durable history.
    ReopenableHistory,
}

impl M5ProgressState {
    /// Every progress state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Queued,
        Self::Running,
        Self::GroupedBatch,
        Self::Paused,
        Self::Succeeded,
        Self::Failed,
        Self::CanceledByUser,
        Self::ReopenableHistory,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::GroupedBatch => "grouped_batch",
            Self::Paused => "paused",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::CanceledByUser => "canceled_by_user",
            Self::ReopenableHistory => "reopenable_history",
        }
    }
}

/// Non-visual / accessibility route every primitive must offer so no truth is
/// hover-only, spinner-only, or pointer-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5AccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed shell primitive must be able to show. The first
/// three are hard requirements on every primitive per the guardrails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrimitiveRequiredLabel {
    /// The primitive's stable identity / what it represents.
    Identity,
    /// The primitive's current typed state.
    State,
    /// The non-visual keyboard route to the primitive.
    KeyboardRoute,
    /// The source / provider attribution of the shown content.
    SourceProvider,
    /// The freshness of the shown content.
    Freshness,
    /// The reopen / recover path for the primitive.
    ReopenPath,
}

impl M5PrimitiveRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::SourceProvider,
        Self::Freshness,
        Self::ReopenPath,
    ];

    /// The three labels every claimed primitive must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::SourceProvider => "source_provider",
            Self::Freshness => "freshness",
            Self::ReopenPath => "reopen_path",
        }
    }
}

/// Qualification class for an M5 shell-primitive row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrimitiveQualificationClass {
    /// Primitive qualifies for the Stable claim.
    Stable,
    /// Primitive is narrowed to Beta.
    Beta,
    /// Primitive is narrowed to Preview.
    Preview,
    /// Primitive is experimental and not claimed.
    Experimental,
    /// Primitive is unavailable on this build.
    Unavailable,
    /// Primitive is held pending upstream resolution.
    Held,
}

impl M5PrimitiveQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the primitive may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a shell primitive below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellPrimitiveDowngradeTrigger {
    /// A status bar reflowed around a vanity or decorative item.
    VanityItemReflow,
    /// Critical state was visible only through a transient spinner.
    SpinnerOnlyState,
    /// Critical truth was reachable only via pointer hover.
    HoverOnlyCriticalTruth,
    /// A surface hid the source / provider / freshness of shown content.
    SourceFreshnessHidden,
    /// A stale / cached preview read as live canonical content.
    StalePreviewMistakenForLive,
    /// A promotion or pin dropped representation truth.
    PromotionDroppedTruth,
    /// A pane could only be resized with the pointer.
    PointerOnlyResize,
    /// A pane's resize state was not serializable / restorable.
    ResizeStateNotSerializable,
    /// Progress was lost when the user looked away.
    ProgressLostOnLookAway,
    /// A grouped-progress row was unattributed.
    GroupedProgressUnattributed,
    /// A severe state displaced truth instead of a vanity item.
    SevereStateDisplacedTruth,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5ShellPrimitiveDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::VanityItemReflow,
        Self::SpinnerOnlyState,
        Self::HoverOnlyCriticalTruth,
        Self::SourceFreshnessHidden,
        Self::StalePreviewMistakenForLive,
        Self::PromotionDroppedTruth,
        Self::PointerOnlyResize,
        Self::ResizeStateNotSerializable,
        Self::ProgressLostOnLookAway,
        Self::GroupedProgressUnattributed,
        Self::SevereStateDisplacedTruth,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VanityItemReflow => "vanity_item_reflow",
            Self::SpinnerOnlyState => "spinner_only_state",
            Self::HoverOnlyCriticalTruth => "hover_only_critical_truth",
            Self::SourceFreshnessHidden => "source_freshness_hidden",
            Self::StalePreviewMistakenForLive => "stale_preview_mistaken_for_live",
            Self::PromotionDroppedTruth => "promotion_dropped_truth",
            Self::PointerOnlyResize => "pointer_only_resize",
            Self::ResizeStateNotSerializable => "resize_state_not_serializable",
            Self::ProgressLostOnLookAway => "progress_lost_on_look_away",
            Self::GroupedProgressUnattributed => "grouped_progress_unattributed",
            Self::SevereStateDisplacedTruth => "severe_state_displaced_truth",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed shell-primitive family bound to its shell
/// zone, layout classes, and the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellPrimitiveRow {
    /// Governed primitive family.
    pub primitive_family: M5ShellPrimitiveFamily,
    /// Qualification class earned by this primitive.
    pub qualification: M5PrimitiveQualificationClass,
    /// Owner role accountable for keeping this primitive governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this primitive attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this primitive must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this primitive keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Claimed M5 surface families that render / consume this primitive.
    pub surface_families: Vec<M5ShellSurfaceFamily>,
    /// Mandatory labels this primitive must be able to show (must include the
    /// three [`M5PrimitiveRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Status-item classes this primitive projects (ambient only).
    pub status_item_classes: Vec<M5StatusItemClass>,
    /// Overflow / severe-state displacement behaviors (ambient only).
    pub overflow_behaviors: Vec<M5OverflowBehavior>,
    /// Source / provider / freshness labels this primitive can show.
    pub source_freshness_labels: Vec<M5SourceFreshnessLabel>,
    /// Hover/peek representation classes (transient inspect only).
    pub representation_classes: Vec<M5RepresentationClass>,
    /// Promotion states this primitive honours (promoting surfaces only).
    pub promotion_states: Vec<M5PromotionState>,
    /// Splitter / pane-resize states (pane control only).
    pub pane_resize_states: Vec<M5PaneResizeState>,
    /// Progress / job-row states (progress only).
    pub progress_states: Vec<M5ProgressState>,
    /// Non-visual accessibility routes this primitive offers.
    pub accessibility_routes: Vec<M5AccessibilityRoute>,
    /// Shell subsystems that consume this primitive's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this primitive.
    pub downgrade_triggers: Vec<M5ShellPrimitiveDowngradeTrigger>,
    /// Proof packet refs that keep this primitive current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this primitive.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this primitive never reflows around vanity items. MUST be
    /// `false`.
    pub reflows_around_vanity_items: bool,
    /// Hard invariant: this primitive never hides source / freshness truth. MUST
    /// be `false`.
    pub hides_source_or_freshness: bool,
    /// Hard invariant: this primitive never keeps critical truth hover-only. MUST
    /// be `false`.
    pub keeps_critical_truth_hover_only: bool,
    /// Hard invariant: this primitive is never resizable by pointer only. MUST be
    /// `false`.
    pub resizable_by_pointer_only: bool,
}

impl M5ShellPrimitiveRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5PrimitiveRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5PrimitiveRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.reflows_around_vanity_items
            && !self.hides_source_or_freshness
            && !self.keeps_critical_truth_hover_only
            && !self.resizable_by_pointer_only
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellPrimitiveVocabularySet {
    /// Primitive-family tokens.
    pub primitive_families: Vec<String>,
    /// Status-item-class tokens.
    pub status_item_classes: Vec<String>,
    /// Overflow-behavior tokens.
    pub overflow_behaviors: Vec<String>,
    /// Source-freshness-label tokens.
    pub source_freshness_labels: Vec<String>,
    /// Representation-class tokens.
    pub representation_classes: Vec<String>,
    /// Promotion-state tokens.
    pub promotion_states: Vec<String>,
    /// Pane-resize-state tokens.
    pub pane_resize_states: Vec<String>,
    /// Progress-state tokens.
    pub progress_states: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5ShellPrimitiveVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            primitive_families: tokens(&M5ShellPrimitiveFamily::ALL, |v| v.as_str()),
            status_item_classes: tokens(&M5StatusItemClass::ALL, |v| v.as_str()),
            overflow_behaviors: tokens(&M5OverflowBehavior::ALL, |v| v.as_str()),
            source_freshness_labels: tokens(&M5SourceFreshnessLabel::ALL, |v| v.as_str()),
            representation_classes: tokens(&M5RepresentationClass::ALL, |v| v.as_str()),
            promotion_states: tokens(&M5PromotionState::ALL, |v| v.as_str()),
            pane_resize_states: tokens(&M5PaneResizeState::ALL, |v| v.as_str()),
            progress_states: tokens(&M5ProgressState::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5AccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5PrimitiveRequiredLabel::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellPrimitiveGovernanceReview {
    /// Ambient instrumentation stays stable and overflow-safe.
    pub ambient_instrumentation_overflow_safe: bool,
    /// Status bars never reflow around spinners or vanity items.
    pub no_status_reflow_around_vanity_items: bool,
    /// Transient inspect surfaces preserve source / provider / freshness.
    pub transient_inspect_preserves_source_and_freshness: bool,
    /// Pinned previews keep their representation truth after pinning.
    pub pinned_preview_keeps_representation_truth: bool,
    /// Pane resize is keyboard-addressable and serializable.
    pub pane_resize_keyboard_addressable_and_serializable: bool,
    /// Progress rows are durable, attributable, and reopenable.
    pub progress_rows_durable_and_reopenable: bool,
    /// No critical truth is hover-only or spinner-only.
    pub no_critical_truth_hover_or_spinner_only: bool,
    /// A severe state displaces a vanity item, never a truth-bearing peer.
    pub severe_state_displaces_vanity_not_truth: bool,
    /// Every primitive is bound to a canonical shell zone.
    pub every_primitive_bound_to_shell_zone: bool,
    /// Every primitive declares a non-visual accessibility route.
    pub every_primitive_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel shell-primitive vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellPrimitiveConsumerProjection {
    /// The status bar consumes the shared status-item / overflow vocabulary.
    pub status_bar_consumes_matrix: bool,
    /// Hovercards and peek panels consume the representation vocabulary.
    pub hovercard_peek_consumes_representation_vocabulary: bool,
    /// Splitters consume the pane-resize-state vocabulary.
    pub splitter_consumes_resize_state_vocabulary: bool,
    /// The activity / progress center consumes the progress vocabulary.
    pub activity_center_consumes_progress_vocabulary: bool,
    /// Support / export reads a single canonical shell-primitive source.
    pub support_export_reads_single_source: bool,
    /// The accessibility bridge reads a single canonical shell-primitive source.
    pub accessibility_bridge_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellPrimitiveProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the shell-primitives lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellPrimitiveReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting shell-primitives audit for the lane.
    pub shell_primitives_audit_ref: String,
    /// True when support/export parity is required for every primitive.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every primitive.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ShellPrimitivesMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ShellPrimitivesMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Primitive rows.
    pub primitive_rows: Vec<M5ShellPrimitiveRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ShellPrimitiveVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ShellPrimitiveGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ShellPrimitiveConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ShellPrimitiveProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ShellPrimitiveReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 shell-primitives matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellPrimitivesMatrixPacket {
    /// Record kind; must equal [`M5_SHELL_PRIMITIVES_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SHELL_PRIMITIVES_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Primitive rows.
    pub primitive_rows: Vec<M5ShellPrimitiveRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ShellPrimitiveVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ShellPrimitiveGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ShellPrimitiveConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ShellPrimitiveProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ShellPrimitiveReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ShellPrimitivesMatrixPacket {
    /// Builds an M5 shell-primitives matrix packet from stable-lane input.
    pub fn new(input: M5ShellPrimitivesMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_SHELL_PRIMITIVES_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_SHELL_PRIMITIVES_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            primitive_rows: input.primitive_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 shell-primitives matrix invariants.
    pub fn validate(&self) -> Vec<M5ShellPrimitivesMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SHELL_PRIMITIVES_MATRIX_RECORD_KIND {
            violations.push(M5ShellPrimitivesMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SHELL_PRIMITIVES_MATRIX_SCHEMA_VERSION {
            violations.push(M5ShellPrimitivesMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ShellPrimitivesMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_primitive_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 shell primitives matrix packet serializes"),
        ) {
            violations.push(M5ShellPrimitivesMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 shell primitives matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed
    /// primitive.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "primitive_family,qualification,owner,shell_zone_slot,responsive_classes,window_classes,surface_families,required_labels,consumer_surfaces\n",
        );
        for row in &self.primitive_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.primitive_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.responsive_classes, |v| v.as_str()),
                join_tokens(&row.window_classes, |v| v.as_str()),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_primitives = self
            .primitive_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Status-Bar, Transient-Inspect, Pane-Control, and Durable-Progress-Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Primitive families: {} ({} stable)\n",
            self.primitive_rows.len(),
            stable_primitives
        ));
        out.push_str(&format!(
            "- Status-item classes: {}\n",
            self.vocabulary_set.status_item_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Source/freshness labels: {}\n",
            self.vocabulary_set.source_freshness_labels.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Primitive families\n\n");
        for row in &self.primitive_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.primitive_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 shell-primitives matrix export.
#[derive(Debug)]
pub enum M5ShellPrimitivesMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ShellPrimitivesMatrixViolation>),
}

impl fmt::Display for M5ShellPrimitivesMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 shell primitives matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 shell primitives matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ShellPrimitivesMatrixArtifactError {}

/// Validation failures emitted by [`M5ShellPrimitivesMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ShellPrimitivesMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed primitive family is missing from the matrix.
    RequiredPrimitiveMissing,
    /// A primitive row is incomplete.
    PrimitiveRowIncomplete,
    /// A primitive row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// An ambient primitive declares no status-item classes.
    StatusItemClassMissing,
    /// An ambient primitive declares no overflow behaviors.
    OverflowBehaviorMissing,
    /// A transient-inspect primitive declares no representation classes.
    RepresentationClassMissing,
    /// A promoting primitive declares no promotion states.
    PromotionStateMissing,
    /// A pane-control primitive declares no pane-resize states.
    PaneResizeStateMissing,
    /// A progress primitive declares no progress states.
    ProgressStateMissing,
    /// A freshness-carrying primitive declares no source-freshness labels.
    SourceFreshnessMissing,
    /// A primitive declares no surface families.
    SurfaceFamilyMissing,
    /// A primitive declares no responsive classes.
    ResponsiveClassMissing,
    /// A primitive declares no window classes.
    WindowClassMissing,
    /// A primitive declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A primitive declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A primitive declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A primitive claiming Stable is missing required proof packet refs.
    StablePrimitiveMissingProof,
    /// A primitive violates a hard invariant (vanity reflow, hidden
    /// source/freshness, hover-only truth, or pointer-only resize).
    PrimitiveInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ShellPrimitivesMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredPrimitiveMissing => "required_primitive_missing",
            Self::PrimitiveRowIncomplete => "primitive_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::StatusItemClassMissing => "status_item_class_missing",
            Self::OverflowBehaviorMissing => "overflow_behavior_missing",
            Self::RepresentationClassMissing => "representation_class_missing",
            Self::PromotionStateMissing => "promotion_state_missing",
            Self::PaneResizeStateMissing => "pane_resize_state_missing",
            Self::ProgressStateMissing => "progress_state_missing",
            Self::SourceFreshnessMissing => "source_freshness_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ResponsiveClassMissing => "responsive_class_missing",
            Self::WindowClassMissing => "window_class_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StablePrimitiveMissingProof => "stable_primitive_missing_proof",
            Self::PrimitiveInvariantViolated => "primitive_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 shell-primitives matrix export.
pub fn current_stable_m5_shell_primitives_matrix_export(
) -> Result<M5ShellPrimitivesMatrixPacket, M5ShellPrimitivesMatrixArtifactError> {
    let packet: M5ShellPrimitivesMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shell-primitives-proof/support_export.json"
    )))
    .map_err(M5ShellPrimitivesMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ShellPrimitivesMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ShellPrimitivesMatrixPacket,
    violations: &mut Vec<M5ShellPrimitivesMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SHELL_PRIMITIVES_SCHEMA_REF,
        M5_SHELL_PRIMITIVES_DOC_REF,
        M5_SHELL_PRIMITIVES_SHELL_ZONE_REF,
        M5_SHELL_PRIMITIVES_RESPONSIVE_CLASS_REF,
        M5_SHELL_PRIMITIVES_MULTI_WINDOW_PARITY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ShellPrimitivesMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ShellPrimitivesMatrixPacket,
    violations: &mut Vec<M5ShellPrimitivesMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ShellPrimitivesMatrixViolation::VocabularySetDrift);
    }
}

fn validate_primitive_rows(
    packet: &M5ShellPrimitivesMatrixPacket,
    violations: &mut Vec<M5ShellPrimitivesMatrixViolation>,
) {
    let present: BTreeSet<M5ShellPrimitiveFamily> = packet
        .primitive_rows
        .iter()
        .map(|row| row.primitive_family)
        .collect();
    for required in M5ShellPrimitiveFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5ShellPrimitivesMatrixViolation::RequiredPrimitiveMissing);
            return;
        }
    }

    for row in &packet.primitive_rows {
        let family = row.primitive_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5ShellPrimitivesMatrixViolation::PrimitiveRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ShellPrimitivesMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_ambient() && row.status_item_classes.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::StatusItemClassMissing);
        }
        if family.is_ambient() && row.overflow_behaviors.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::OverflowBehaviorMissing);
        }
        if family.is_transient_inspect() && row.representation_classes.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::RepresentationClassMissing);
        }
        if family.promotes() && row.promotion_states.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::PromotionStateMissing);
        }
        if family.is_pane_control() && row.pane_resize_states.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::PaneResizeStateMissing);
        }
        if family.is_progress() && row.progress_states.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::ProgressStateMissing);
        }
        if family.carries_freshness() && row.source_freshness_labels.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::SourceFreshnessMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::SurfaceFamilyMissing);
        }
        if row.responsive_classes.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::ResponsiveClassMissing);
        }
        if row.window_classes.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::WindowClassMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ShellPrimitivesMatrixViolation::StablePrimitiveMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ShellPrimitivesMatrixViolation::PrimitiveInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ShellPrimitivesMatrixPacket,
    violations: &mut Vec<M5ShellPrimitivesMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.ambient_instrumentation_overflow_safe,
        review.no_status_reflow_around_vanity_items,
        review.transient_inspect_preserves_source_and_freshness,
        review.pinned_preview_keeps_representation_truth,
        review.pane_resize_keyboard_addressable_and_serializable,
        review.progress_rows_durable_and_reopenable,
        review.no_critical_truth_hover_or_spinner_only,
        review.severe_state_displaces_vanity_not_truth,
        review.every_primitive_bound_to_shell_zone,
        review.every_primitive_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ShellPrimitivesMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ShellPrimitivesMatrixPacket,
    violations: &mut Vec<M5ShellPrimitivesMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.status_bar_consumes_matrix,
        projection.hovercard_peek_consumes_representation_vocabulary,
        projection.splitter_consumes_resize_state_vocabulary,
        projection.activity_center_consumes_progress_vocabulary,
        projection.support_export_reads_single_source,
        projection.accessibility_bridge_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ShellPrimitivesMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ShellPrimitivesMatrixPacket,
    violations: &mut Vec<M5ShellPrimitivesMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ShellPrimitivesMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ShellPrimitivesMatrixPacket,
    violations: &mut Vec<M5ShellPrimitivesMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.shell_primitives_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ShellPrimitivesMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
