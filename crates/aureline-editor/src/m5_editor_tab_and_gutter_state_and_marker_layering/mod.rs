//! Implemented M5 editor-tab and gutter primitives.
//!
//! The frozen [editor-inline component matrix][matrix] names the reusable editor / review / AI inline
//! UI components and locks their controlled vocabulary. This module is the first implement lane over
//! that matrix: it turns the two left-edge, file/session-legibility components — the **editor tab**
//! and the **gutter** — into resolvers that produce export-safe, honest projections, so a user can
//! read which document context is active (versus merely open), what state a file/session carries, and
//! what a gutter glyph means, without any of that state being buried in a tooltip, encoded by color
//! alone, or reinvented as a feature-local badge.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render editor tabs with modified, preview, pinned, read-only, blocked, shared, generated, and
//!   remote state using one controlled vocabulary and no-color-only semantics.**
//!   [`resolve_editor_tab`] refuses to read as a clean, legible tab when the file/session identity is
//!   unstated, the tab context or item state is unresolved, a feature-local badge is invented for the
//!   same file/session state, an item state is encoded by color alone, a blocked tab hides behind a
//!   color/ellipsis cue, the pane context is unresolved, reopen/reveal continuity is lost across
//!   panes, or no command-backed path to trace the state is reachable; it degrades instead.
//! * **Render gutter markers with stable layering for diagnostics, breakpoints, change markers, and
//!   blame/trust cues, and command-backed reveal/detail actions.** [`resolve_gutter`] degrades when
//!   the anchor is unstated, the marker kind or layer is unresolved, a feature-local badge is
//!   invented, the marker or its severity is encoded by color alone, layer precedence is lost, the
//!   layering is not readable in a compact / high-zoom / exported representation, or no command-backed
//!   reveal/detail entrypoint is reachable.
//! * **Preserve reopen/reveal continuity and current-versus-selected semantics across single-editor,
//!   split-editor, diff, and notebook-backed code panes.** The packet proves, by resolved examples,
//!   that the same tab and gutter state grammar spans the panes, that continuity-breaking and
//!   badge-inventing examples degrade, and that a user can trace file/session and gutter state back to
//!   one canonical component contract and one command-backed detail entrypoint.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EditorInlineDisposition`] inline-disposition vocabulary, the [`M5EditorTabState`] tab-context
//! vocabulary, the [`M5GutterMarkerKind`] gutter-marker vocabulary, and the [`M5DiagnosticSeverity`]
//! diagnostic-severity vocabulary — so editor, diff, notebook, diagnostics, and support surfaces can
//! never fork their own file/session or marker wording or invent surface-local badges for the same
//! state. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_editor_inline_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_editor_tab_gutter_controls,
    seeded_m5_editor_tab_gutter_controls_diagnostics_ui_preview_narrowed,
    seeded_m5_editor_tab_gutter_controls_editor_ui_beta_narrowed,
    M5_EDITOR_TAB_GUTTER_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_editor_inline_component_matrix::{
    M5DiagnosticSeverity, M5EditorInlineAccessibilityRoute, M5EditorInlineComponentFamily,
    M5EditorInlineConsumerSurface, M5EditorInlineDeploymentLine, M5EditorInlineDisposition,
    M5EditorInlineDowngradeTrigger, M5EditorInlineQualificationClass, M5EditorInlineRequiredLabel,
    M5EditorTabState, M5GutterMarkerKind, M5_EDITOR_INLINE_COMPONENT_DOC_REF,
    M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_EDITOR_TAB_SCHEMA_REF, M5_GUTTER_MARKER_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5EditorTabGutterControlsPacket`].
pub const M5_EDITOR_TAB_GUTTER_CONTROLS_RECORD_KIND: &str =
    "implement_m5_editor_tab_and_gutter_controls";

/// Schema version for M5 editor-tab / gutter controls records.
pub const M5_EDITOR_TAB_GUTTER_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_EDITOR_TAB_GUTTER_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-editor-tab-gutter-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_EDITOR_TAB_GUTTER_CONTROLS_DOC_REF: &str =
    "docs/editor/m5_editor_tab_and_gutter_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EDITOR_TAB_GUTTER_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-editor-tab-gutter-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_EDITOR_TAB_GUTTER_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-editor-tab-gutter-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_EDITOR_TAB_GUTTER_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-editor-tab-gutter-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_EDITOR_TAB_GUTTER_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-editor-tab-gutter-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy
/// so no lane invents a parallel surface set.
pub type M5EditorTabGutterConsumerSurface = M5EditorInlineConsumerSurface;

/// Controlled per-tab item state an editor tab names with no-color-only semantics. Minted by this
/// lane because the frozen matrix [`M5EditorTabState`] carries active / background / preview-unpinned
/// tab *context* but not the modified / preview / pinned / read-only / blocked / shared / generated /
/// remote file/session **item state** the editor-tab acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorTabItemState {
    /// The tab has unsaved modifications.
    Modified,
    /// The tab is a single-click preview, not yet pinned.
    Preview,
    /// The tab is pinned open.
    Pinned,
    /// The tab is read-only / locked.
    ReadOnly,
    /// The tab is blocked (by policy, error, or precondition).
    Blocked,
    /// The tab is shared / co-edited with another actor.
    Shared,
    /// The tab is backed by machine-generated content.
    Generated,
    /// The tab is backed by a remote source.
    Remote,
    /// The tab item state cannot currently be resolved.
    StateUnknown,
}

impl M5EditorTabItemState {
    /// Every tab item state, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Modified,
        Self::Preview,
        Self::Pinned,
        Self::ReadOnly,
        Self::Blocked,
        Self::Shared,
        Self::Generated,
        Self::Remote,
        Self::StateUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Modified => "modified",
            Self::Preview => "preview",
            Self::Pinned => "pinned",
            Self::ReadOnly => "read_only",
            Self::Blocked => "blocked",
            Self::Shared => "shared",
            Self::Generated => "generated",
            Self::Remote => "remote",
            Self::StateUnknown => "state_unknown",
        }
    }

    /// Whether this state names a shared / co-edited tab.
    pub const fn is_shared(self) -> bool {
        matches!(self, Self::Shared)
    }

    /// Whether this state names a machine-generated tab.
    pub const fn is_generated(self) -> bool {
        matches!(self, Self::Generated)
    }

    /// Whether this state names a remote-backed tab.
    pub const fn is_remote(self) -> bool {
        matches!(self, Self::Remote)
    }

    /// Whether this state names a blocked tab that must never hide behind a color / ellipsis cue.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// Controlled gutter-marker layer — the precedence band a gutter glyph occupies, so layered gutter
/// state keeps a stable identity and precedence in compact, high-zoom, and exported representations.
/// Minted by this lane because the frozen matrix carries the marker *kind* but not the layering /
/// precedence band the gutter acceptance criteria require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GutterMarkerLayer {
    /// The diagnostic layer (errors / warnings / hints), highest precedence.
    Diagnostic,
    /// The breakpoint layer.
    Breakpoint,
    /// The change-marker layer (added / modified / removed).
    ChangeMarker,
    /// The blame / trust cue layer, where claimed.
    BlameOrTrustCue,
    /// The fold-region affordance layer, lowest precedence.
    FoldAffordance,
    /// The marker layer cannot currently be resolved.
    LayerUnresolved,
}

impl M5GutterMarkerLayer {
    /// Every gutter-marker layer, in declaration order (highest to lowest precedence).
    pub const ALL: [Self; 6] = [
        Self::Diagnostic,
        Self::Breakpoint,
        Self::ChangeMarker,
        Self::BlameOrTrustCue,
        Self::FoldAffordance,
        Self::LayerUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diagnostic => "diagnostic",
            Self::Breakpoint => "breakpoint",
            Self::ChangeMarker => "change_marker",
            Self::BlameOrTrustCue => "blame_or_trust_cue",
            Self::FoldAffordance => "fold_affordance",
            Self::LayerUnresolved => "layer_unresolved",
        }
    }

    /// Stable precedence rank (lower is higher precedence); resolved layers rank above the unresolved
    /// sentinel so precedence never collapses into ambiguity.
    pub const fn precedence(self) -> u8 {
        match self {
            Self::Diagnostic => 0,
            Self::Breakpoint => 1,
            Self::ChangeMarker => 2,
            Self::BlameOrTrustCue => 3,
            Self::FoldAffordance => 4,
            Self::LayerUnresolved => u8::MAX,
        }
    }

    /// Whether the layer is resolved to a concrete precedence band.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::LayerUnresolved)
    }
}

/// Controlled code-pane kind — which pane shape renders the tab / gutter, so reopen/reveal continuity
/// and current-versus-selected semantics stay honest across single-editor, split-editor, diff, and
/// notebook-backed code panes. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorPaneKind {
    /// A single, full-width editor pane.
    SingleEditor,
    /// One side of a split editor.
    SplitEditor,
    /// A diff / merge pane.
    DiffPane,
    /// A notebook-backed code cell pane.
    NotebookCodeCell,
    /// A peek / inline-preview pane.
    PeekPane,
    /// The pane kind cannot currently be resolved.
    PaneUnknown,
}

impl M5EditorPaneKind {
    /// Every pane kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleEditor,
        Self::SplitEditor,
        Self::DiffPane,
        Self::NotebookCodeCell,
        Self::PeekPane,
        Self::PaneUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleEditor => "single_editor",
            Self::SplitEditor => "split_editor",
            Self::DiffPane => "diff_pane",
            Self::NotebookCodeCell => "notebook_code_cell",
            Self::PeekPane => "peek_pane",
            Self::PaneUnknown => "pane_unknown",
        }
    }

    /// Whether the pane kind is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::PaneUnknown)
    }
}

/// One mandatory rendered part an editor tab or gutter must be able to show, so no file/session or
/// marker fact is left implicit behind compact chrome, a tooltip, or a secondary panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorTabGutterAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed inline disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The per-tab item state (editor tab).
    TabItemState,
    /// The pane / code-pane context (editor tab).
    PaneContext,
    /// The reopen/reveal continuity across panes (editor tab).
    ReopenRevealContinuity,
    /// The gutter marker kind (gutter).
    GutterMarkerKind,
    /// The gutter marker layer / precedence band (gutter).
    GutterMarkerLayer,
    /// The diagnostic severity behind a gutter diagnostic (gutter).
    DiagnosticSeverity,
    /// The layer precedence preserved in compact / exported views (gutter).
    LayerPrecedence,
    /// The command-backed path to trace the file/session or gutter state (both components).
    StateCommand,
}

impl M5EditorTabGutterAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::TabItemState,
        Self::PaneContext,
        Self::ReopenRevealContinuity,
        Self::GutterMarkerKind,
        Self::GutterMarkerLayer,
        Self::DiagnosticSeverity,
        Self::LayerPrecedence,
        Self::StateCommand,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::TabItemState => "tab_item_state",
            Self::PaneContext => "pane_context",
            Self::ReopenRevealContinuity => "reopen_reveal_continuity",
            Self::GutterMarkerKind => "gutter_marker_kind",
            Self::GutterMarkerLayer => "gutter_marker_layer",
            Self::DiagnosticSeverity => "diagnostic_severity",
            Self::LayerPrecedence => "layer_precedence",
            Self::StateCommand => "state_command",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to trace the
/// file/session or gutter state behind a degraded editor component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorTabGutterNextAction {
    /// Open the command-backed file/session or gutter state detail.
    OpenStateDetail,
    /// Inspect the per-tab item state behind the tab.
    InspectTabState,
    /// Inspect the gutter marker kind / layer behind the glyph.
    InspectGutterMarker,
    /// Review a blocked or hidden tab / marker.
    ReviewBlockedOrHidden,
    /// Review diagnostics for a stale or unresolved signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5EditorTabGutterNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenStateDetail,
        Self::InspectTabState,
        Self::InspectGutterMarker,
        Self::ReviewBlockedOrHidden,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenStateDetail => "open_state_detail",
            Self::InspectTabState => "inspect_tab_state",
            Self::InspectGutterMarker => "inspect_gutter_marker",
            Self::ReviewBlockedOrHidden => "review_blocked_or_hidden",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorTabGutterExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The inline dispositions carried.
    Dispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The per-tab item state named by the editor tab.
    TabItemState,
    /// The pane / code-pane context named by the editor tab.
    PaneKind,
    /// The gutter marker kind named by the gutter.
    GutterMarkerKind,
    /// The gutter marker layer named by the gutter.
    GutterMarkerLayer,
    /// The diagnostic severity named by the gutter.
    DiagnosticSeverity,
    /// The accountable owner role.
    OwnerRole,
}

impl M5EditorTabGutterExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::TabItemState,
        Self::PaneKind,
        Self::GutterMarkerKind,
        Self::GutterMarkerLayer,
        Self::DiagnosticSeverity,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::Dispositions => "dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::TabItemState => "tab_item_state",
            Self::PaneKind => "pane_kind",
            Self::GutterMarkerKind => "gutter_marker_kind",
            Self::GutterMarkerLayer => "gutter_marker_layer",
            Self::DiagnosticSeverity => "diagnostic_severity",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason an editor tab degraded below a clean, legible state. The degrade-first ladder returns one
/// of these instead of ever letting an ambiguous tab read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorTabDegradeReason {
    /// The file/session identity label is unstated; a user cannot tell which document is open.
    FileSessionIdentityUnstated,
    /// The tab context (current versus merely open) cannot currently be resolved.
    TabContextUnresolved,
    /// The per-tab item state cannot currently be resolved.
    ItemStateUnresolved,
    /// A feature-local badge was invented for a file/session state the shared grammar already names.
    FeatureLocalBadgeInvented,
    /// The item state is encoded by color / hover alone rather than named.
    StateEncodedByColorAlone,
    /// A blocked tab is hidden behind a color-only or ambiguous ellipsis cue.
    BlockedTabHiddenBehindColorOrEllipsis,
    /// The pane / code-pane context cannot currently be resolved.
    PaneContextUnresolved,
    /// Reopen/reveal continuity is lost across single / split / diff / notebook panes.
    ReopenRevealContinuityLost,
    /// No command-backed path to trace the file/session state is reachable.
    StateTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5EditorTabDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::FileSessionIdentityUnstated,
        Self::TabContextUnresolved,
        Self::ItemStateUnresolved,
        Self::FeatureLocalBadgeInvented,
        Self::StateEncodedByColorAlone,
        Self::BlockedTabHiddenBehindColorOrEllipsis,
        Self::PaneContextUnresolved,
        Self::ReopenRevealContinuityLost,
        Self::StateTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileSessionIdentityUnstated => "file_session_identity_unstated",
            Self::TabContextUnresolved => "tab_context_unresolved",
            Self::ItemStateUnresolved => "item_state_unresolved",
            Self::FeatureLocalBadgeInvented => "feature_local_badge_invented",
            Self::StateEncodedByColorAlone => "state_encoded_by_color_alone",
            Self::BlockedTabHiddenBehindColorOrEllipsis => {
                "blocked_tab_hidden_behind_color_or_ellipsis"
            }
            Self::PaneContextUnresolved => "pane_context_unresolved",
            Self::ReopenRevealContinuityLost => "reopen_reveal_continuity_lost",
            Self::StateTracePathMissing => "state_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5EditorTabGutterNextAction {
        match self {
            Self::FileSessionIdentityUnstated
            | Self::TabContextUnresolved
            | Self::ItemStateUnresolved => M5EditorTabGutterNextAction::InspectTabState,
            Self::FeatureLocalBadgeInvented
            | Self::PaneContextUnresolved
            | Self::ReopenRevealContinuityLost
            | Self::StateTracePathMissing => M5EditorTabGutterNextAction::OpenStateDetail,
            Self::StateEncodedByColorAlone | Self::BlockedTabHiddenBehindColorOrEllipsis => {
                M5EditorTabGutterNextAction::ReviewBlockedOrHidden
            }
            Self::ProofStale => M5EditorTabGutterNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EditorInlineDowngradeTrigger {
        match self {
            Self::StateEncodedByColorAlone | Self::BlockedTabHiddenBehindColorOrEllipsis => {
                M5EditorInlineDowngradeTrigger::TabMarkerDiagnosticColorOnly
            }
            Self::ProofStale => M5EditorInlineDowngradeTrigger::ProofStale,
            _ => M5EditorInlineDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// Reason a gutter degraded below a clean, readable, precedence-preserving state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GutterDegradeReason {
    /// The gutter anchor (line / range identity) is unstated.
    GutterAnchorUnstated,
    /// The marker kind cannot currently be resolved.
    MarkerKindUnresolved,
    /// The marker layer / precedence band cannot currently be resolved.
    MarkerLayerUnresolved,
    /// A feature-local badge was invented for a marker the shared grammar already names.
    FeatureLocalBadgeInvented,
    /// The marker kind is encoded by color alone rather than named.
    MarkerEncodedByColorAlone,
    /// The diagnostic severity behind the marker cannot currently be resolved.
    DiagnosticSeverityUnresolved,
    /// The diagnostic severity is encoded by color alone rather than named.
    SeverityEncodedByColorAlone,
    /// Layer precedence is lost; layered markers collapse into ambiguity.
    LayerPrecedenceLost,
    /// The marker layering is not readable in a compact / high-zoom / exported representation.
    MarkerLayeringNotReadableInCompactOrExport,
    /// No command-backed reveal / detail entrypoint for the marker is reachable.
    RevealTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5GutterDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::GutterAnchorUnstated,
        Self::MarkerKindUnresolved,
        Self::MarkerLayerUnresolved,
        Self::FeatureLocalBadgeInvented,
        Self::MarkerEncodedByColorAlone,
        Self::DiagnosticSeverityUnresolved,
        Self::SeverityEncodedByColorAlone,
        Self::LayerPrecedenceLost,
        Self::MarkerLayeringNotReadableInCompactOrExport,
        Self::RevealTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GutterAnchorUnstated => "gutter_anchor_unstated",
            Self::MarkerKindUnresolved => "marker_kind_unresolved",
            Self::MarkerLayerUnresolved => "marker_layer_unresolved",
            Self::FeatureLocalBadgeInvented => "feature_local_badge_invented",
            Self::MarkerEncodedByColorAlone => "marker_encoded_by_color_alone",
            Self::DiagnosticSeverityUnresolved => "diagnostic_severity_unresolved",
            Self::SeverityEncodedByColorAlone => "severity_encoded_by_color_alone",
            Self::LayerPrecedenceLost => "layer_precedence_lost",
            Self::MarkerLayeringNotReadableInCompactOrExport => {
                "marker_layering_not_readable_in_compact_or_export"
            }
            Self::RevealTracePathMissing => "reveal_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5EditorTabGutterNextAction {
        match self {
            Self::GutterAnchorUnstated
            | Self::MarkerKindUnresolved
            | Self::MarkerLayerUnresolved => M5EditorTabGutterNextAction::InspectGutterMarker,
            Self::FeatureLocalBadgeInvented | Self::RevealTracePathMissing => {
                M5EditorTabGutterNextAction::OpenStateDetail
            }
            Self::MarkerEncodedByColorAlone
            | Self::SeverityEncodedByColorAlone
            | Self::LayerPrecedenceLost
            | Self::MarkerLayeringNotReadableInCompactOrExport => {
                M5EditorTabGutterNextAction::ReviewBlockedOrHidden
            }
            Self::DiagnosticSeverityUnresolved | Self::ProofStale => {
                M5EditorTabGutterNextAction::ReviewDiagnostics
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EditorInlineDowngradeTrigger {
        match self {
            Self::GutterAnchorUnstated => M5EditorInlineDowngradeTrigger::AnchorStateUnstated,
            Self::MarkerEncodedByColorAlone | Self::SeverityEncodedByColorAlone => {
                M5EditorInlineDowngradeTrigger::TabMarkerDiagnosticColorOnly
            }
            Self::DiagnosticSeverityUnresolved => {
                M5EditorInlineDowngradeTrigger::DiagnosticFreshnessUnstated
            }
            Self::ProofStale => M5EditorInlineDowngradeTrigger::ProofStale,
            _ => M5EditorInlineDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// True when the tab-context state cannot be resolved.
fn tab_context_is_unresolved(state: M5EditorTabState) -> bool {
    matches!(state, M5EditorTabState::ContextUnresolved)
}

/// True when a diagnostic severity cannot be resolved.
fn severity_is_unresolved(severity: M5DiagnosticSeverity) -> bool {
    matches!(severity, M5DiagnosticSeverity::SeverityUnknown)
}

/// Input to [`resolve_editor_tab`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EditorTabResolutionInput {
    /// Stable identity of the tab instance.
    pub tab_id: String,
    /// The file/session identity label shown; empty means unstated.
    pub file_session_label: String,
    /// The tab context (current versus merely open).
    pub tab_context: M5EditorTabState,
    /// The per-tab item state.
    pub item_state: M5EditorTabItemState,
    /// True when the item state is stated non-color-only (name / icon-with-label, never color alone).
    pub item_state_stated: bool,
    /// The code-pane kind rendering the tab.
    pub pane_kind: M5EditorPaneKind,
    /// True when reopen/reveal continuity is preserved across single / split / diff / notebook panes.
    pub reopen_reveal_continuity_preserved: bool,
    /// True when at least one blocked tab is present in the strip.
    pub has_blocked_tab: bool,
    /// True when a present blocked tab is stated visibly, never hidden behind a color / ellipsis cue.
    pub blocked_tab_stated: bool,
    /// True when the tab invents a feature-local badge for a file/session state the shared grammar
    /// already names.
    pub invents_feature_local_badge: bool,
    /// True when a command-backed entrypoint to trace the file/session state is reachable, never
    /// menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe editor tab projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEditorTab {
    /// Stable identity of the tab instance.
    pub tab_id: String,
    /// The file/session identity label named by the tab.
    pub file_session_label: String,
    /// The tab-context token named by the tab.
    pub tab_context: String,
    /// The per-tab item-state token named by the tab.
    pub item_state: String,
    /// Whether the item state names a shared / co-edited tab.
    pub item_state_shared: bool,
    /// Whether the item state names a machine-generated tab.
    pub item_state_generated: bool,
    /// Whether the item state names a remote-backed tab.
    pub item_state_remote: bool,
    /// The code-pane-kind token named by the tab.
    pub pane_kind: String,
    /// Whether reopen/reveal continuity is preserved across panes.
    pub reopen_reveal_continuity_preserved: bool,
    /// Whether a blocked tab is present in the strip.
    pub has_blocked_tab: bool,
    /// Guardrail (MUST be `false` on a clean tab): a feature-local badge was invented.
    pub invents_feature_local_badge: bool,
    /// Whether a command-backed entrypoint to trace the file/session state is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the tab could not read as a clean, legible state.
    pub degrade_reason: Option<M5EditorTabDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5EditorTabGutterNextAction,
    /// Whether the file/session state is legible at a glance (clean tab naming every fact).
    pub state_legible_at_a_glance: bool,
}

impl M5ResolvedEditorTab {
    /// Whether this tab reads as a clean, legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_gutter`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GutterResolutionInput {
    /// Stable identity of the gutter instance.
    pub gutter_id: String,
    /// The gutter anchor (line / range identity) shown; empty means unstated.
    pub anchor_label: String,
    /// The gutter marker kind.
    pub marker_kind: M5GutterMarkerKind,
    /// The gutter marker layer / precedence band.
    pub marker_layer: M5GutterMarkerLayer,
    /// True when the marker kind is stated non-color-only (glyph-with-label, never color alone).
    pub marker_kind_stated: bool,
    /// The diagnostic severity behind the marker (used on the diagnostic layer).
    pub diagnostic_severity: M5DiagnosticSeverity,
    /// True when the diagnostic severity is stated non-color-only.
    pub severity_stated: bool,
    /// True when layer precedence is preserved (layered markers keep a stable precedence).
    pub layer_precedence_preserved: bool,
    /// True when the marker layering stays readable in compact / high-zoom / exported representations.
    pub readable_in_compact_and_export: bool,
    /// True when the gutter invents a feature-local badge for a marker the shared grammar names.
    pub invents_feature_local_badge: bool,
    /// True when a command-backed reveal / detail entrypoint is reachable, never hover-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe gutter projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedGutter {
    /// Stable identity of the gutter instance.
    pub gutter_id: String,
    /// The gutter anchor named by the gutter.
    pub anchor_label: String,
    /// The gutter-marker-kind token named by the gutter.
    pub marker_kind: String,
    /// The gutter-marker-layer token named by the gutter.
    pub marker_layer: String,
    /// The stable precedence rank of the layer (lower is higher precedence).
    pub layer_precedence: u8,
    /// Whether the layer is on the diagnostic precedence band.
    pub is_diagnostic_layer: bool,
    /// The diagnostic-severity token named by the gutter.
    pub diagnostic_severity: String,
    /// Whether layer precedence is preserved (layered markers keep a stable precedence).
    pub layer_precedence_preserved: bool,
    /// Whether the marker layering stays readable in compact / high-zoom / exported representations.
    pub readable_in_compact_and_export: bool,
    /// Guardrail (MUST be `false` on a clean gutter): a feature-local badge was invented.
    pub invents_feature_local_badge: bool,
    /// Whether a command-backed reveal / detail entrypoint is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the gutter could not read as a clean, readable state.
    pub degrade_reason: Option<M5GutterDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5EditorTabGutterNextAction,
    /// Whether the marker layering is legible at a glance (clean gutter naming every fact).
    pub layering_legible_at_a_glance: bool,
}

impl M5ResolvedGutter {
    /// Whether this gutter reads as a clean, readable state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5EditorTabGutterResolutionError {
    /// The tab id was empty.
    EmptyTabId,
    /// The gutter id was empty.
    EmptyGutterId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5EditorTabGutterResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyTabId => "empty_tab_id",
            Self::EmptyGutterId => "empty_gutter_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5EditorTabGutterResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 editor-tab / gutter resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5EditorTabGutterResolutionError {}

/// Resolves an editor tab so file/session state is legible at a glance: the tab names its context
/// (current versus merely open), per-tab item state (with no-color-only semantics), and pane context,
/// preserves reopen/reveal continuity across panes, never invents a feature-local badge for the same
/// file/session state, and never hides a blocked tab behind a color / ellipsis cue.
pub fn resolve_editor_tab(
    input: M5EditorTabResolutionInput,
) -> Result<M5ResolvedEditorTab, M5EditorTabGutterResolutionError> {
    if input.tab_id.trim().is_empty() {
        return Err(M5EditorTabGutterResolutionError::EmptyTabId);
    }
    if string_is_forbidden(&input.tab_id) || string_is_forbidden(&input.file_session_label) {
        return Err(M5EditorTabGutterResolutionError::ForbiddenMaterial);
    }

    let degrade_reason = if input.file_session_label.trim().is_empty() {
        Some(M5EditorTabDegradeReason::FileSessionIdentityUnstated)
    } else if tab_context_is_unresolved(input.tab_context) {
        Some(M5EditorTabDegradeReason::TabContextUnresolved)
    } else if matches!(input.item_state, M5EditorTabItemState::StateUnknown) {
        Some(M5EditorTabDegradeReason::ItemStateUnresolved)
    } else if input.invents_feature_local_badge {
        Some(M5EditorTabDegradeReason::FeatureLocalBadgeInvented)
    } else if !input.item_state_stated {
        Some(M5EditorTabDegradeReason::StateEncodedByColorAlone)
    } else if (input.has_blocked_tab || input.item_state.is_blocked()) && !input.blocked_tab_stated
    {
        Some(M5EditorTabDegradeReason::BlockedTabHiddenBehindColorOrEllipsis)
    } else if !input.pane_kind.is_resolved() {
        Some(M5EditorTabDegradeReason::PaneContextUnresolved)
    } else if !input.reopen_reveal_continuity_preserved {
        Some(M5EditorTabDegradeReason::ReopenRevealContinuityLost)
    } else if !input.detail_command_available {
        Some(M5EditorTabDegradeReason::StateTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5EditorTabDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5EditorTabGutterNextAction::OpenStateDetail,
    };

    Ok(M5ResolvedEditorTab {
        tab_id: input.tab_id,
        file_session_label: input.file_session_label,
        tab_context: input.tab_context.as_str().to_owned(),
        item_state: input.item_state.as_str().to_owned(),
        item_state_shared: input.item_state.is_shared(),
        item_state_generated: input.item_state.is_generated(),
        item_state_remote: input.item_state.is_remote(),
        pane_kind: input.pane_kind.as_str().to_owned(),
        reopen_reveal_continuity_preserved: input.reopen_reveal_continuity_preserved,
        has_blocked_tab: input.has_blocked_tab || input.item_state.is_blocked(),
        invents_feature_local_badge: input.invents_feature_local_badge,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        state_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a gutter so left-edge markers are legible at a glance: the gutter names its anchor, marker
/// kind and layer (with no-color-only semantics), and diagnostic severity, keeps layer precedence
/// stable, stays readable in compact / high-zoom / exported representations without losing identity or
/// precedence, never invents a feature-local badge, and always offers a command-backed reveal/detail
/// entrypoint.
pub fn resolve_gutter(
    input: M5GutterResolutionInput,
) -> Result<M5ResolvedGutter, M5EditorTabGutterResolutionError> {
    if input.gutter_id.trim().is_empty() {
        return Err(M5EditorTabGutterResolutionError::EmptyGutterId);
    }
    if string_is_forbidden(&input.gutter_id) || string_is_forbidden(&input.anchor_label) {
        return Err(M5EditorTabGutterResolutionError::ForbiddenMaterial);
    }

    let is_diagnostic_layer = matches!(input.marker_layer, M5GutterMarkerLayer::Diagnostic);

    let degrade_reason = if input.anchor_label.trim().is_empty() {
        Some(M5GutterDegradeReason::GutterAnchorUnstated)
    } else if matches!(input.marker_kind, M5GutterMarkerKind::MarkerUnresolved) {
        Some(M5GutterDegradeReason::MarkerKindUnresolved)
    } else if !input.marker_layer.is_resolved() {
        Some(M5GutterDegradeReason::MarkerLayerUnresolved)
    } else if input.invents_feature_local_badge {
        Some(M5GutterDegradeReason::FeatureLocalBadgeInvented)
    } else if !input.marker_kind_stated {
        Some(M5GutterDegradeReason::MarkerEncodedByColorAlone)
    } else if is_diagnostic_layer && severity_is_unresolved(input.diagnostic_severity) {
        Some(M5GutterDegradeReason::DiagnosticSeverityUnresolved)
    } else if is_diagnostic_layer && !input.severity_stated {
        Some(M5GutterDegradeReason::SeverityEncodedByColorAlone)
    } else if !input.layer_precedence_preserved {
        Some(M5GutterDegradeReason::LayerPrecedenceLost)
    } else if !input.readable_in_compact_and_export {
        Some(M5GutterDegradeReason::MarkerLayeringNotReadableInCompactOrExport)
    } else if !input.detail_command_available {
        Some(M5GutterDegradeReason::RevealTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5GutterDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5EditorTabGutterNextAction::OpenStateDetail,
    };

    Ok(M5ResolvedGutter {
        gutter_id: input.gutter_id,
        anchor_label: input.anchor_label,
        marker_kind: input.marker_kind.as_str().to_owned(),
        marker_layer: input.marker_layer.as_str().to_owned(),
        layer_precedence: input.marker_layer.precedence(),
        is_diagnostic_layer,
        diagnostic_severity: input.diagnostic_severity.as_str().to_owned(),
        layer_precedence_preserved: input.layer_precedence_preserved,
        readable_in_compact_and_export: input.readable_in_compact_and_export,
        invents_feature_local_badge: input.invents_feature_local_badge,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        layering_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved editor-tab and gutter examples it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorTabGutterControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5EditorTabGutterConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5EditorInlineQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5EditorInlineDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5EditorInlineRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5EditorInlineAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5EditorTabGutterAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5EditorTabGutterExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EditorInlineDowngradeTrigger>,
    /// Resolved editor-tab examples.
    pub editor_tab_examples: Vec<M5ResolvedEditorTab>,
    /// Resolved gutter examples.
    pub gutter_examples: Vec<M5ResolvedGutter>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a tab never invents a feature-local badge for the same file/session state.
    pub tabs_invent_feature_local_badges_for_file_session_state: bool,
    /// Hard invariant: a gutter marker never encodes its state by color alone.
    pub gutter_markers_encode_state_by_color_alone: bool,
    /// Hard invariant: gutter marker layering never loses identity or precedence.
    pub gutter_marker_layering_loses_identity_or_precedence: bool,
    /// Hard invariant: reopen/reveal continuity never breaks across single / split / diff / notebook
    /// panes.
    pub reopen_reveal_continuity_breaks_across_panes: bool,
}

impl M5EditorTabGutterControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5EditorTabGutterAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5EditorTabGutterAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5EditorTabGutterExportField> =
            self.export_fields.iter().copied().collect();
        M5EditorTabGutterExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.tabs_invent_feature_local_badges_for_file_session_state
            && !self.gutter_markers_encode_state_by_color_alone
            && !self.gutter_marker_layering_loses_identity_or_precedence
            && !self.reopen_reveal_continuity_breaks_across_panes
    }

    /// True when every resolved example on this row is honest: no clean tab invents a badge, loses
    /// reopen/reveal continuity, or lacks a trace path, and no clean gutter invents a badge, loses
    /// precedence, is unreadable in compact / export, or lacks a reveal path.
    fn examples_are_honest(&self) -> bool {
        self.editor_tab_examples.iter().all(|ex| {
            !ex.is_clean()
                || (!ex.invents_feature_local_badge
                    && ex.reopen_reveal_continuity_preserved
                    && ex.detail_command_available)
        }) && self.gutter_examples.iter().all(|ex| {
            !ex.is_clean()
                || (!ex.invents_feature_local_badge
                    && ex.layer_precedence_preserved
                    && ex.readable_in_compact_and_export
                    && ex.detail_command_available)
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorTabGutterVocabularySet {
    /// Inline-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Tab-context tokens (bound from the frozen matrix).
    pub tab_contexts: Vec<String>,
    /// Gutter-marker-kind tokens (bound from the frozen matrix).
    pub gutter_marker_kinds: Vec<String>,
    /// Diagnostic-severity tokens (bound from the frozen matrix).
    pub diagnostic_severities: Vec<String>,
    /// Tab item-state tokens (minted by this lane).
    pub tab_item_states: Vec<String>,
    /// Gutter-marker-layer tokens (minted by this lane).
    pub gutter_marker_layers: Vec<String>,
    /// Pane-kind tokens (minted by this lane).
    pub pane_kinds: Vec<String>,
    /// Editor-tab degrade-reason tokens.
    pub editor_tab_degrade_reasons: Vec<String>,
    /// Gutter degrade-reason tokens.
    pub gutter_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5EditorTabGutterVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5EditorInlineDisposition::ALL, |v| v.as_str()),
            tab_contexts: tokens(&M5EditorTabState::ALL, |v| v.as_str()),
            gutter_marker_kinds: tokens(&M5GutterMarkerKind::ALL, |v| v.as_str()),
            diagnostic_severities: tokens(&M5DiagnosticSeverity::ALL, |v| v.as_str()),
            tab_item_states: tokens(&M5EditorTabItemState::ALL, |v| v.as_str()),
            gutter_marker_layers: tokens(&M5GutterMarkerLayer::ALL, |v| v.as_str()),
            pane_kinds: tokens(&M5EditorPaneKind::ALL, |v| v.as_str()),
            editor_tab_degrade_reasons: tokens(&M5EditorTabDegradeReason::ALL, |v| v.as_str()),
            gutter_degrade_reasons: tokens(&M5GutterDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5EditorTabGutterAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5EditorTabGutterNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5EditorTabGutterExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5EditorInlineConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5EditorTabGutterGovernanceReview {
    /// The editor tab names its file/session context and per-tab item state.
    pub tab_names_context_and_item_state: bool,
    /// The editor tab states item state with no-color-only semantics.
    pub tab_item_state_no_color_only: bool,
    /// The gutter names its marker kind and layer, layering markers without relying on color alone.
    pub gutter_names_marker_kind_and_layer: bool,
    /// The gutter keeps layer precedence readable in compact / high-zoom / exported representations.
    pub gutter_layering_readable_across_representations: bool,
    /// Tabs never invent feature-local badges for the same file/session state.
    pub tabs_never_invent_feature_local_badges: bool,
    /// Gutters never invent feature-local badges for the same marker.
    pub gutters_never_invent_feature_local_badges: bool,
    /// State is never encoded by color alone on either component.
    pub state_never_encoded_by_color_alone: bool,
    /// Blocked tabs and hidden markers are never hidden behind a color / ellipsis cue.
    pub blocked_and_hidden_never_behind_color_or_ellipsis: bool,
    /// Reopen/reveal continuity is preserved across single / split / diff / notebook panes.
    pub reopen_reveal_continuity_preserved_across_panes: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorTabGutterConsumerProjection {
    /// Editor surfaces consume the shared tab and gutter vocabulary.
    pub editor_surfaces_consume_tab_and_gutter_vocabulary: bool,
    /// Diff surfaces consume the shared tab and gutter vocabulary.
    pub diff_surfaces_consume_tab_and_gutter_vocabulary: bool,
    /// The notebook consumes the shared tab and gutter vocabulary for code cells.
    pub notebook_consumes_tab_and_gutter_vocabulary: bool,
    /// Diagnostics consume the shared gutter marker and severity vocabulary.
    pub diagnostics_consume_marker_and_severity_vocabulary: bool,
    /// File/session and gutter facts trace back to one canonical component contract.
    pub state_facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical editor-inline source.
    pub support_export_reads_single_editor_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorTabGutterProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorTabGutterReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5EditorTabGutterControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EditorTabGutterControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5EditorTabGutterControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EditorTabGutterVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EditorTabGutterGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EditorTabGutterConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EditorTabGutterProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EditorTabGutterReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 editor-tab / gutter controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorTabGutterControlsPacket {
    /// Record kind; must equal [`M5_EDITOR_TAB_GUTTER_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EDITOR_TAB_GUTTER_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5EditorTabGutterControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EditorTabGutterVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EditorTabGutterGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EditorTabGutterConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EditorTabGutterProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EditorTabGutterReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EditorTabGutterControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5EditorTabGutterControlsPacketInput) -> Self {
        Self {
            record_kind: M5_EDITOR_TAB_GUTTER_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_EDITOR_TAB_GUTTER_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
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

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5EditorTabGutterControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EDITOR_TAB_GUTTER_CONTROLS_RECORD_KIND {
            violations.push(M5EditorTabGutterControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EDITOR_TAB_GUTTER_CONTROLS_SCHEMA_VERSION {
            violations.push(M5EditorTabGutterControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EditorTabGutterControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5EditorTabGutterControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 editor-tab / gutter controls packet serializes"),
        ) {
            violations.push(M5EditorTabGutterControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 editor-tab / gutter controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,tab_examples,gutter_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .editor_tab_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.gutter_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.editor_tab_examples.len(),
                row.gutter_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Editor-Tab and Gutter Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Tab item states: {}\n",
            self.vocabulary_set.tab_item_states.join(", ")
        ));
        out.push_str(&format!(
            "- Gutter marker layers: {}\n",
            self.vocabulary_set.gutter_marker_layers.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Editor-tab examples: {} / gutter examples: {}\n",
                row.editor_tab_examples.len(),
                row.gutter_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5EditorTabGutterControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5EditorTabGutterControlsViolation>),
}

impl fmt::Display for M5EditorTabGutterControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 editor-tab / gutter controls export parse failed: {error}"
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
                    "m5 editor-tab / gutter controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5EditorTabGutterControlsArtifactError {}

/// Validation failures emitted by [`M5EditorTabGutterControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EditorTabGutterControlsViolation {
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
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (badge, continuity loss, precedence loss,
    /// unreadable layering, or missing trace).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Tab / gutter state grammar is not proven: clean tabs do not cover the shared item-state
    /// grammar across surfaces, or no feature-local-badge example degrades.
    TabAndGutterStateGrammarNotProven,
    /// Marker-layering readability is not proven: no clean gutter preserves precedence and stays
    /// readable in compact / export, or no precedence-loss / unreadable example degrades.
    MarkerLayeringReadabilityNotProven,
    /// State traceability is not proven: no clean tab and clean gutter both offer a command-backed
    /// detail entrypoint.
    StateTraceabilityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5EditorTabGutterControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::TabAndGutterStateGrammarNotProven => "tab_and_gutter_state_grammar_not_proven",
            Self::MarkerLayeringReadabilityNotProven => "marker_layering_readability_not_proven",
            Self::StateTraceabilityNotProven => "state_traceability_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_editor_tab_gutter_controls_export(
) -> Result<M5EditorTabGutterControlsPacket, M5EditorTabGutterControlsArtifactError> {
    let packet: M5EditorTabGutterControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-editor-tab-gutter-controls-proof/support_export.json"
    )))
    .map_err(M5EditorTabGutterControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5EditorTabGutterControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5EditorTabGutterControlsPacket,
    violations: &mut Vec<M5EditorTabGutterControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EDITOR_TAB_GUTTER_CONTROLS_SCHEMA_REF,
        M5_EDITOR_TAB_GUTTER_CONTROLS_DOC_REF,
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
        M5_EDITOR_TAB_SCHEMA_REF,
        M5_GUTTER_MARKER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5EditorTabGutterControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5EditorTabGutterControlsPacket,
    violations: &mut Vec<M5EditorTabGutterControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5EditorTabGutterControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5EditorTabGutterControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5EditorTabGutterControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5EditorTabGutterControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_EDITOR_TAB_SCHEMA_REF) || !refs.contains(M5_GUTTER_MARKER_SCHEMA_REF) {
            violations.push(M5EditorTabGutterControlsViolation::ComponentSchemaRefMissing);
        }
        if row.editor_tab_examples.is_empty() || row.gutter_examples.is_empty() {
            violations.push(M5EditorTabGutterControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5EditorTabGutterControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5EditorTabGutterControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5EditorTabGutterControlsPacket,
    violations: &mut Vec<M5EditorTabGutterControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.tab_names_context_and_item_state,
        review.tab_item_state_no_color_only,
        review.gutter_names_marker_kind_and_layer,
        review.gutter_layering_readable_across_representations,
        review.tabs_never_invent_feature_local_badges,
        review.gutters_never_invent_feature_local_badges,
        review.state_never_encoded_by_color_alone,
        review.blocked_and_hidden_never_behind_color_or_ellipsis,
        review.reopen_reveal_continuity_preserved_across_panes,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5EditorTabGutterControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5EditorTabGutterControlsPacket,
    violations: &mut Vec<M5EditorTabGutterControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_surfaces_consume_tab_and_gutter_vocabulary,
        projection.diff_surfaces_consume_tab_and_gutter_vocabulary,
        projection.notebook_consumes_tab_and_gutter_vocabulary,
        projection.diagnostics_consume_marker_and_severity_vocabulary,
        projection.state_facts_trace_to_single_component_contract,
        projection.support_export_reads_single_editor_source,
    ] {
        if !ok {
            violations.push(M5EditorTabGutterControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5EditorTabGutterControlsPacket,
    violations: &mut Vec<M5EditorTabGutterControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5EditorTabGutterControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5EditorTabGutterControlsPacket,
    violations: &mut Vec<M5EditorTabGutterControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5EditorTabGutterControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5EditorTabGutterControlsPacket,
    violations: &mut Vec<M5EditorTabGutterControlsViolation>,
) {
    let tabs = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.editor_tab_examples.iter())
    };
    let gutters = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.gutter_examples.iter())
    };

    // AC1: claimed M5 editors show the same tab and gutter state grammar and do not invent
    // feature-local badges for the same file/session state. Clean tabs cover at least two distinct
    // item states from the shared vocabulary, a badge-invention example degrades on both the tab and
    // the gutter side, and no clean tab or gutter invents a badge.
    let clean_tab_states: BTreeSet<String> = tabs()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.item_state.clone())
        .collect();
    let badge_tab_degrades = tabs()
        .any(|ex| ex.degrade_reason == Some(M5EditorTabDegradeReason::FeatureLocalBadgeInvented));
    let badge_gutter_degrades = gutters()
        .any(|ex| ex.degrade_reason == Some(M5GutterDegradeReason::FeatureLocalBadgeInvented));
    let no_clean_badge = tabs().all(|ex| !(ex.is_clean() && ex.invents_feature_local_badge))
        && gutters().all(|ex| !(ex.is_clean() && ex.invents_feature_local_badge));
    if !(clean_tab_states.len() >= 2
        && badge_tab_degrades
        && badge_gutter_degrades
        && no_clean_badge)
    {
        violations.push(M5EditorTabGutterControlsViolation::TabAndGutterStateGrammarNotProven);
    }

    // AC2: marker layering remains readable in compact, high-zoom, and exported representations
    // without losing identity or precedence. At least one clean gutter preserves precedence and stays
    // readable across representations, a precedence-loss example degrades, an unreadable-layering
    // example degrades, and no clean gutter loses precedence or is unreadable.
    let readable_clean_gutter = gutters().any(|ex| {
        ex.is_clean()
            && ex.layer_precedence_preserved
            && ex.readable_in_compact_and_export
            && ex.layering_legible_at_a_glance
    });
    let precedence_loss_degrades =
        gutters().any(|ex| ex.degrade_reason == Some(M5GutterDegradeReason::LayerPrecedenceLost));
    let unreadable_degrades = gutters().any(|ex| {
        ex.degrade_reason == Some(M5GutterDegradeReason::MarkerLayeringNotReadableInCompactOrExport)
    });
    let no_clean_precedence_or_unreadable = gutters().all(|ex| {
        !(ex.is_clean() && (!ex.layer_precedence_preserved || !ex.readable_in_compact_and_export))
    });
    if !(readable_clean_gutter
        && precedence_loss_degrades
        && unreadable_degrades
        && no_clean_precedence_or_unreadable)
    {
        violations.push(M5EditorTabGutterControlsViolation::MarkerLayeringReadabilityNotProven);
    }

    // AC3: users can trace file/session and gutter state back to one canonical component contract and
    // one command-backed detail entrypoint. At least one clean tab and one clean gutter both expose a
    // command-backed detail entrypoint.
    let traceable_tab = tabs().any(|ex| ex.is_clean() && ex.detail_command_available);
    let traceable_gutter = gutters().any(|ex| ex.is_clean() && ex.detail_command_available);
    if !(traceable_tab && traceable_gutter) {
        violations.push(M5EditorTabGutterControlsViolation::StateTraceabilityNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5EditorInlineComponentFamily; 2] = [
    M5EditorInlineComponentFamily::EditorTab,
    M5EditorInlineComponentFamily::Gutter,
];
