//! Frozen M5 editor-tab, gutter, diagnostic-decoration, code-action-chip, diff-view, review-thread,
//! AI-message-card, and evidence-timeline inline component matrix.
//!
//! This module locks Aureline's reusable editor / review / AI inline UI components into one
//! export-safe packet. Every claimed M5 surface that still ships its own editor tab, gutter marker,
//! diagnostic decoration, code-action chip, diff view, review thread, AI message card, or evidence
//! timeline — across the editor, diff/merge, review, notebook, AI, diagnostics, and support surfaces —
//! is named once here and constrained by the same modified / preview / pinned / read-only / shared /
//! generated / remote item state, breakpoint / change-marker layering, problem severity and freshness,
//! exact-versus-inferred fix posture, comment-anchor durability, AI source-context / confidence /
//! available-action, and evidence-timeline readability / collapsibility vocabulary regardless of the
//! surface family that renders it.
//!
//! The matrix does not re-architect the diff engine, the notebook runtime, the hosted-review provider
//! model, or the AI planner / runtime — it is the shared inline-component-honesty contract layered on
//! top of them. The controlled vocabularies are frozen in one self-describing
//! [`M5EditorInlineVocabularySet`] rather than minted per surface. The single controlled inline
//! disposition vocabulary consumers bind to — modified, preview, pinned, read-only, shared, generated,
//! remote, exact-fix, inferred-fix, outdated, resolved, re-anchored, blocked-by-policy, streaming,
//! review-required, applied, reverted, failed, and export-safe-evidence — keeps tab / marker /
//! diagnostic state from being encoded by color alone, keeps comment anchors and AI evidence pointers
//! from silently drifting, keeps outdated and resolved review state from blurring together, keeps
//! inferred fixes from being presented as exact, and keeps evidence timelines from hiding in opaque
//! logs without inspectable structure. Raw secret values and private endpoints stay outside the export
//! boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_editor_inline_component_matrix,
    seeded_m5_editor_inline_component_matrix_diff_view_beta_narrowed,
    seeded_m5_editor_inline_component_matrix_review_thread_preview_narrowed,
    M5_EDITOR_INLINE_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5EditorInlineComponentMatrixPacket`].
pub const M5_EDITOR_INLINE_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_editor_tab_gutter_diagnostic_decoration_code_action_chip_diff_view_review_thread_ai_message_card_and_evidence_timeline_component_matrix";

/// Schema version for M5 editor-inline component-matrix records.
pub const M5_EDITOR_INLINE_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined editor-inline component-matrix schema.
pub const M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-editor-inline-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EDITOR_INLINE_COMPONENT_DOC_REF: &str =
    "docs/editor/m5_editor_inline_components_contract.md";

/// Repo-relative path of the editor-tab canonical component schema.
pub const M5_EDITOR_TAB_SCHEMA_REF: &str = "schemas/ui/m5-editor-tab.schema.json";

/// Repo-relative path of the gutter-marker canonical component schema.
pub const M5_GUTTER_MARKER_SCHEMA_REF: &str = "schemas/ui/m5-gutter-marker.schema.json";

/// Repo-relative path of the diagnostic-decoration canonical component schema.
pub const M5_DIAGNOSTIC_DECORATION_SCHEMA_REF: &str =
    "schemas/ui/m5-diagnostic-decoration.schema.json";

/// Repo-relative path of the code-action-chip canonical component schema.
pub const M5_CODE_ACTION_CHIP_SCHEMA_REF: &str = "schemas/ui/m5-code-action-chip.schema.json";

/// Repo-relative path of the diff-view canonical component schema.
pub const M5_DIFF_VIEW_SCHEMA_REF: &str = "schemas/ui/m5-diff-view.schema.json";

/// Repo-relative path of the review-thread canonical component schema.
pub const M5_REVIEW_THREAD_SCHEMA_REF: &str = "schemas/ui/m5-review-thread.schema.json";

/// Repo-relative path of the AI-message-card canonical component schema.
pub const M5_AI_MESSAGE_CARD_SCHEMA_REF: &str = "schemas/ui/m5-ai-message-card.schema.json";

/// Repo-relative path of the evidence-timeline canonical component schema.
pub const M5_EVIDENCE_TIMELINE_SCHEMA_REF: &str = "schemas/ui/m5-evidence-timeline.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_EDITOR_INLINE_COMPONENT_FIXTURE_DIR: &str = "fixtures/ui/m5-editor-inline-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EDITOR_INLINE_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-editor-inline-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_EDITOR_INLINE_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-editor-inline-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_EDITOR_INLINE_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-editor-inline-component-matrix.md";

/// One of the eight governed editor / review / AI inline component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineComponentFamily {
    /// An editor tab naming the open-document context and its modified / preview / pinned /
    /// read-only / shared / generated / remote item state.
    EditorTab,
    /// A gutter naming breakpoint, change-marker, and fold layering next to the code without relying
    /// on color alone.
    Gutter,
    /// A diagnostic decoration naming problem severity and freshness anchored to a durable range.
    DiagnosticDecoration,
    /// A code-action chip naming exact-versus-inferred fix posture and applied / reverted / blocked
    /// state.
    CodeActionChip,
    /// A diff view naming added / removed / modified / moved / conflicted change kinds.
    DiffView,
    /// A review thread naming comment-anchor durability and outdated-versus-resolved review state.
    ReviewThread,
    /// An AI message card naming source context, confidence, and available actions.
    AiMessageCard,
    /// An evidence timeline naming inspectable, collapsible, export-safe evidence lineage.
    EvidenceTimeline,
}

impl M5EditorInlineComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::EditorTab,
        Self::Gutter,
        Self::DiagnosticDecoration,
        Self::CodeActionChip,
        Self::DiffView,
        Self::ReviewThread,
        Self::AiMessageCard,
        Self::EvidenceTimeline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorTab => "editor_tab",
            Self::Gutter => "gutter",
            Self::DiagnosticDecoration => "diagnostic_decoration",
            Self::CodeActionChip => "code_action_chip",
            Self::DiffView => "diff_view",
            Self::ReviewThread => "review_thread",
            Self::AiMessageCard => "ai_message_card",
            Self::EvidenceTimeline => "evidence_timeline",
        }
    }

    /// The canonical per-component schema ref a downstream row points at instead of restating this
    /// component's inline-state truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::EditorTab => M5_EDITOR_TAB_SCHEMA_REF,
            Self::Gutter => M5_GUTTER_MARKER_SCHEMA_REF,
            Self::DiagnosticDecoration => M5_DIAGNOSTIC_DECORATION_SCHEMA_REF,
            Self::CodeActionChip => M5_CODE_ACTION_CHIP_SCHEMA_REF,
            Self::DiffView => M5_DIFF_VIEW_SCHEMA_REF,
            Self::ReviewThread => M5_REVIEW_THREAD_SCHEMA_REF,
            Self::AiMessageCard => M5_AI_MESSAGE_CARD_SCHEMA_REF,
            Self::EvidenceTimeline => M5_EVIDENCE_TIMELINE_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled editor-tab state.
    pub const fn declares_tab_state(self) -> bool {
        matches!(self, Self::EditorTab)
    }

    /// `true` when this family must name a controlled gutter-marker kind.
    pub const fn declares_gutter_marker(self) -> bool {
        matches!(self, Self::Gutter)
    }

    /// `true` when this family must name a controlled diagnostic severity.
    pub const fn declares_diagnostic_severity(self) -> bool {
        matches!(self, Self::Gutter | Self::DiagnosticDecoration)
    }

    /// `true` when this family must name a controlled fix posture.
    pub const fn declares_fix_posture(self) -> bool {
        matches!(self, Self::CodeActionChip)
    }

    /// `true` when this family must name a controlled diff change kind.
    pub const fn declares_diff_change_kind(self) -> bool {
        matches!(self, Self::DiffView)
    }

    /// `true` when this family must name a controlled anchor durability.
    pub const fn declares_anchor_durability(self) -> bool {
        matches!(self, Self::DiagnosticDecoration | Self::ReviewThread)
    }

    /// `true` when this family must name a controlled AI confidence.
    pub const fn declares_ai_confidence(self) -> bool {
        matches!(self, Self::AiMessageCard)
    }

    /// `true` when this family must name a controlled evidence-disclosure state.
    pub const fn declares_evidence_disclosure(self) -> bool {
        matches!(self, Self::AiMessageCard | Self::EvidenceTimeline)
    }
}

/// The single controlled inline-disposition vocabulary every editor, diff, review, notebook, AI,
/// diagnostics, or support consumer binds to. These are the exact acceptance-criteria tokens that keep
/// tab / marker / diagnostic state from being color-only, keep anchors and evidence pointers from
/// drifting, keep outdated and resolved state distinct, keep inferred fixes from reading as exact, and
/// keep evidence export-safe. No editor / review / AI surface invents a parallel word for any of these
/// dispositions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineDisposition {
    /// The item has unsaved modifications.
    Modified,
    /// The item is a preview (single-click, not yet pinned open).
    Preview,
    /// The item is pinned open.
    Pinned,
    /// The item is read-only.
    ReadOnly,
    /// The item is shared with other participants.
    Shared,
    /// The item was machine-generated.
    Generated,
    /// The item is backed by a remote source.
    Remote,
    /// The offered fix is an exact, verified transform.
    ExactFix,
    /// The offered fix is inferred / heuristic and not guaranteed exact.
    InferredFix,
    /// The state shown is outdated relative to the source of truth.
    Outdated,
    /// A review item is resolved.
    Resolved,
    /// An anchor was re-anchored after the underlying text moved.
    ReAnchored,
    /// The item is blocked by policy.
    BlockedByPolicy,
    /// The item is still streaming in.
    Streaming,
    /// The item requires review before it takes effect.
    ReviewRequired,
    /// The action has been applied.
    Applied,
    /// The action has been reverted.
    Reverted,
    /// The action failed.
    Failed,
    /// Evidence is present and export-safe.
    ExportSafeEvidence,
}

impl M5EditorInlineDisposition {
    /// Every disposition token, in declaration order.
    pub const ALL: [Self; 19] = [
        Self::Modified,
        Self::Preview,
        Self::Pinned,
        Self::ReadOnly,
        Self::Shared,
        Self::Generated,
        Self::Remote,
        Self::ExactFix,
        Self::InferredFix,
        Self::Outdated,
        Self::Resolved,
        Self::ReAnchored,
        Self::BlockedByPolicy,
        Self::Streaming,
        Self::ReviewRequired,
        Self::Applied,
        Self::Reverted,
        Self::Failed,
        Self::ExportSafeEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Modified => "modified",
            Self::Preview => "preview",
            Self::Pinned => "pinned",
            Self::ReadOnly => "read_only",
            Self::Shared => "shared",
            Self::Generated => "generated",
            Self::Remote => "remote",
            Self::ExactFix => "exact_fix",
            Self::InferredFix => "inferred_fix",
            Self::Outdated => "outdated",
            Self::Resolved => "resolved",
            Self::ReAnchored => "re_anchored",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::Streaming => "streaming",
            Self::ReviewRequired => "review_required",
            Self::Applied => "applied",
            Self::Reverted => "reverted",
            Self::Failed => "failed",
            Self::ExportSafeEvidence => "export_safe_evidence",
        }
    }

    /// Whether this disposition names a fix posture that must never read exact when it is inferred.
    pub const fn is_fix_posture(self) -> bool {
        matches!(self, Self::ExactFix | Self::InferredFix)
    }
}

/// Controlled editor-tab state — which open document is current versus merely open, and whether it is
/// an unpinned preview, has unsaved changes, or is read-only, so a background or preview tab is never
/// presented as the active saved document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorTabState {
    /// The current, focused tab.
    ActiveCurrent,
    /// An open background tab.
    BackgroundOpen,
    /// An unpinned preview tab.
    PreviewUnpinned,
    /// A tab with unsaved modifications.
    ModifiedUnsaved,
    /// A read-only / locked tab.
    ReadOnlyLocked,
    /// The tab's document context cannot currently be resolved.
    ContextUnresolved,
}

impl M5EditorTabState {
    /// Every editor-tab state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ActiveCurrent,
        Self::BackgroundOpen,
        Self::PreviewUnpinned,
        Self::ModifiedUnsaved,
        Self::ReadOnlyLocked,
        Self::ContextUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveCurrent => "active_current",
            Self::BackgroundOpen => "background_open",
            Self::PreviewUnpinned => "preview_unpinned",
            Self::ModifiedUnsaved => "modified_unsaved",
            Self::ReadOnlyLocked => "read_only_locked",
            Self::ContextUnresolved => "context_unresolved",
        }
    }
}

/// Controlled gutter-marker kind — what a gutter glyph means (breakpoint, change marker, fold), so
/// layered gutter state is never encoded by color alone and a change marker is never confused with a
/// breakpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GutterMarkerKind {
    /// A breakpoint.
    Breakpoint,
    /// An added-line change marker.
    ChangeAdded,
    /// A modified-line change marker.
    ChangeModified,
    /// A removed-line change marker.
    ChangeRemoved,
    /// A fold region affordance.
    FoldRegion,
    /// The marker kind cannot currently be resolved.
    MarkerUnresolved,
}

impl M5GutterMarkerKind {
    /// Every gutter-marker kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Breakpoint,
        Self::ChangeAdded,
        Self::ChangeModified,
        Self::ChangeRemoved,
        Self::FoldRegion,
        Self::MarkerUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Breakpoint => "breakpoint",
            Self::ChangeAdded => "change_added",
            Self::ChangeModified => "change_modified",
            Self::ChangeRemoved => "change_removed",
            Self::FoldRegion => "fold_region",
            Self::MarkerUnresolved => "marker_unresolved",
        }
    }
}

/// Controlled diagnostic severity — the severity and freshness of a problem, so a stale diagnostic is
/// never presented as a current one and severity is never encoded by color alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticSeverity {
    /// An error.
    Error,
    /// A warning.
    Warning,
    /// Informational.
    Info,
    /// A hint.
    Hint,
    /// A diagnostic that is stale relative to the current buffer.
    StaleDiagnostic,
    /// The severity cannot currently be resolved.
    SeverityUnknown,
}

impl M5DiagnosticSeverity {
    /// Every diagnostic severity, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Error,
        Self::Warning,
        Self::Info,
        Self::Hint,
        Self::StaleDiagnostic,
        Self::SeverityUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Hint => "hint",
            Self::StaleDiagnostic => "stale_diagnostic",
            Self::SeverityUnknown => "severity_unknown",
        }
    }
}

/// Controlled fix posture — whether an offered code action is an exact transform or an inferred /
/// heuristic suggestion, so an inferred fix is never presented as an exact one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FixPosture {
    /// An exact, verified fix.
    ExactFix,
    /// An inferred fix (correct shape, not guaranteed exact).
    InferredFix,
    /// A heuristic suggestion.
    HeuristicSuggestion,
    /// Multiple candidate fixes are offered.
    MultipleCandidates,
    /// No fix applies.
    NotApplicable,
    /// The fix posture cannot currently be resolved.
    PostureUnknown,
}

impl M5FixPosture {
    /// Every fix posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactFix,
        Self::InferredFix,
        Self::HeuristicSuggestion,
        Self::MultipleCandidates,
        Self::NotApplicable,
        Self::PostureUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactFix => "exact_fix",
            Self::InferredFix => "inferred_fix",
            Self::HeuristicSuggestion => "heuristic_suggestion",
            Self::MultipleCandidates => "multiple_candidates",
            Self::NotApplicable => "not_applicable",
            Self::PostureUnknown => "posture_unknown",
        }
    }
}

/// Controlled diff change kind — what a diff hunk represents, so an added, removed, modified, moved,
/// or conflicted change is never collapsed into an ambiguous generic change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffChangeKind {
    /// An added region.
    Added,
    /// A removed region.
    Removed,
    /// A modified region.
    Modified,
    /// A moved region.
    Moved,
    /// A conflicted region.
    Conflicted,
    /// Unchanged context.
    UnchangedContext,
}

impl M5DiffChangeKind {
    /// Every diff change kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Added,
        Self::Removed,
        Self::Modified,
        Self::Moved,
        Self::Conflicted,
        Self::UnchangedContext,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Modified => "modified",
            Self::Moved => "moved",
            Self::Conflicted => "conflicted",
            Self::UnchangedContext => "unchanged_context",
        }
    }
}

/// Controlled anchor durability — how durable a comment or decoration anchor is against edits, so an
/// anchor is never presented as exact when it has drifted, gone stale, or been orphaned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AnchorDurability {
    /// The anchor still points at its exact original range.
    AnchoredExact,
    /// The anchor was re-anchored after the text moved.
    ReAnchored,
    /// The anchor drifted to an approximate range.
    DriftedApproximate,
    /// The anchor is outdated relative to the current text.
    OutdatedAnchor,
    /// The anchor was orphaned (its range no longer exists).
    OrphanedAnchor,
    /// The anchor state cannot currently be resolved.
    AnchorUnresolved,
}

impl M5AnchorDurability {
    /// Every anchor durability, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AnchoredExact,
        Self::ReAnchored,
        Self::DriftedApproximate,
        Self::OutdatedAnchor,
        Self::OrphanedAnchor,
        Self::AnchorUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AnchoredExact => "anchored_exact",
            Self::ReAnchored => "re_anchored",
            Self::DriftedApproximate => "drifted_approximate",
            Self::OutdatedAnchor => "outdated_anchor",
            Self::OrphanedAnchor => "orphaned_anchor",
            Self::AnchorUnresolved => "anchor_unresolved",
        }
    }
}

/// Controlled AI confidence — how well grounded an AI message is, so an unverified or low-confidence
/// message is never presented with the same weight as a grounded one, and a still-streaming partial is
/// never read as final.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiConfidence {
    /// Grounded with high confidence.
    GroundedHigh,
    /// Grounded with medium confidence.
    GroundedMedium,
    /// Low confidence.
    LowConfidence,
    /// Unverified (no grounding).
    Unverified,
    /// A still-streaming partial.
    StreamingPartial,
    /// The confidence cannot currently be resolved.
    ConfidenceUnknown,
}

impl M5AiConfidence {
    /// Every AI confidence, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GroundedHigh,
        Self::GroundedMedium,
        Self::LowConfidence,
        Self::Unverified,
        Self::StreamingPartial,
        Self::ConfidenceUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GroundedHigh => "grounded_high",
            Self::GroundedMedium => "grounded_medium",
            Self::LowConfidence => "low_confidence",
            Self::Unverified => "unverified",
            Self::StreamingPartial => "streaming_partial",
            Self::ConfidenceUnknown => "confidence_unknown",
        }
    }
}

/// Controlled evidence-disclosure state — how much of an evidence timeline is shown, so evidence is
/// never hidden in an opaque log and a redacted export is never mistaken for a complete one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceDisclosure {
    /// Fully expanded.
    ExpandedFull,
    /// Collapsed to a summary.
    CollapsedSummary,
    /// Partially loaded.
    PartiallyLoaded,
    /// Redacted for an export-safe boundary.
    RedactedExportSafe,
    /// No evidence is present.
    EmptyNoEvidence,
    /// The disclosure state cannot currently be resolved.
    DisclosureUnknown,
}

impl M5EvidenceDisclosure {
    /// Every evidence-disclosure state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandedFull,
        Self::CollapsedSummary,
        Self::PartiallyLoaded,
        Self::RedactedExportSafe,
        Self::EmptyNoEvidence,
        Self::DisclosureUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandedFull => "expanded_full",
            Self::CollapsedSummary => "collapsed_summary",
            Self::PartiallyLoaded => "partially_loaded",
            Self::RedactedExportSafe => "redacted_export_safe",
            Self::EmptyNoEvidence => "empty_no_evidence",
            Self::DisclosureUnknown => "disclosure_unknown",
        }
    }
}

/// Claimed M5 surface family that renders / consumes an editor-inline component. No component may
/// invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineSurfaceFamily {
    /// The code editor.
    Editor,
    /// The diff / merge surface.
    DiffMerge,
    /// The review surface.
    Review,
    /// The notebook.
    Notebook,
    /// The AI chat / assist surface.
    AiChat,
    /// The diagnostics surface.
    Diagnostics,
    /// The support export.
    SupportExport,
}

impl M5EditorInlineSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Editor,
        Self::DiffMerge,
        Self::Review,
        Self::Notebook,
        Self::AiChat,
        Self::Diagnostics,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::DiffMerge => "diff_merge",
            Self::Review => "review",
            Self::Notebook => "notebook",
            Self::AiChat => "ai_chat",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's state, severity,
/// anchor, confidence, or evidence truth never silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5EditorInlineDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineConsumerSurface {
    /// The editor UI.
    EditorUi,
    /// The diff / merge UI.
    DiffUi,
    /// The review UI.
    ReviewUi,
    /// The notebook UI.
    NotebookUi,
    /// The AI UI.
    AiUi,
    /// The diagnostics UI.
    DiagnosticsUi,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5EditorInlineConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::EditorUi,
        Self::DiffUi,
        Self::ReviewUi,
        Self::NotebookUi,
        Self::AiUi,
        Self::DiagnosticsUi,
        Self::CliExport,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorUi => "editor_ui",
            Self::DiffUi => "diff_ui",
            Self::ReviewUi => "review_ui",
            Self::NotebookUi => "notebook_ui",
            Self::AiUi => "ai_ui",
            Self::DiagnosticsUi => "diagnostics_ui",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no editor / review / AI truth is
/// hover-only, pointer-only, motion-only, or visually encoded alone. Records the keyboard,
/// screen-reader, high-zoom, reduced-motion, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Legible and usable with reduced motion.
    ReducedMotionSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5EditorInlineAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::ReducedMotionSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::ReducedMotionSafe => "reduced_motion_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason an editor-inline component has degraded below its qualified state. Required on every row so a
/// stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The anchor signal is unavailable.
    AnchorSignalUnavailable,
    /// The severity signal is unavailable.
    SeveritySignalUnavailable,
    /// The confidence signal is unavailable.
    ConfidenceSignalUnavailable,
    /// The evidence signal is unavailable.
    EvidenceSignalUnavailable,
    /// The fix-posture signal is unavailable.
    FixPostureUnavailable,
}

impl M5EditorInlineDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::AnchorSignalUnavailable,
        Self::SeveritySignalUnavailable,
        Self::ConfidenceSignalUnavailable,
        Self::EvidenceSignalUnavailable,
        Self::FixPostureUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::AnchorSignalUnavailable => "anchor_signal_unavailable",
            Self::SeveritySignalUnavailable => "severity_signal_unavailable",
            Self::ConfidenceSignalUnavailable => "confidence_signal_unavailable",
            Self::EvidenceSignalUnavailable => "evidence_signal_unavailable",
            Self::FixPostureUnavailable => "fix_posture_unavailable",
        }
    }
}

/// Mandatory label a claimed editor-inline component must be able to show. The first three are hard
/// requirements on every component; the remaining three close the acceptance-criteria ambiguity about
/// anchor / freshness, confidence / source, and evidence-lineage labeling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state / disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The anchor durability and freshness behind the component.
    AnchorAndFreshness,
    /// The confidence and source context behind the component.
    ConfidenceAndSource,
    /// The evidence lineage behind the component.
    EvidenceLineage,
}

impl M5EditorInlineRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::AnchorAndFreshness,
        Self::ConfidenceAndSource,
        Self::EvidenceLineage,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::AnchorAndFreshness => "anchor_and_freshness",
            Self::ConfidenceAndSource => "confidence_and_source",
            Self::EvidenceLineage => "evidence_lineage",
        }
    }
}

/// Qualification class for an M5 editor-inline component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5EditorInlineQualificationClass {
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

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows an editor-inline component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineDowngradeTrigger {
    /// Tab, marker, or diagnostic state was encoded by color alone.
    TabMarkerDiagnosticColorOnly,
    /// A comment anchor drifted silently.
    CommentAnchorDriftedSilently,
    /// An AI evidence pointer drifted silently.
    EvidencePointerDriftedSilently,
    /// Outdated and resolved review state were blurred together.
    OutdatedAndResolvedBlurred,
    /// An inferred fix was shown as exact.
    InferredFixShownAsExact,
    /// An evidence timeline was hidden in an opaque log.
    EvidenceTimelineOpaqueLog,
    /// A diagnostic left its freshness unstated.
    DiagnosticFreshnessUnstated,
    /// An AI message left its confidence unstated.
    AiConfidenceUnstated,
    /// A diff view collapsed its change kinds.
    DiffChangeKindCollapsed,
    /// A component left its anchor state unstated.
    AnchorStateUnstated,
    /// Generic chrome wording concealed inline truth.
    GenericChromeWordingUsed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5EditorInlineDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::TabMarkerDiagnosticColorOnly,
        Self::CommentAnchorDriftedSilently,
        Self::EvidencePointerDriftedSilently,
        Self::OutdatedAndResolvedBlurred,
        Self::InferredFixShownAsExact,
        Self::EvidenceTimelineOpaqueLog,
        Self::DiagnosticFreshnessUnstated,
        Self::AiConfidenceUnstated,
        Self::DiffChangeKindCollapsed,
        Self::AnchorStateUnstated,
        Self::GenericChromeWordingUsed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TabMarkerDiagnosticColorOnly => "tab_marker_diagnostic_color_only",
            Self::CommentAnchorDriftedSilently => "comment_anchor_drifted_silently",
            Self::EvidencePointerDriftedSilently => "evidence_pointer_drifted_silently",
            Self::OutdatedAndResolvedBlurred => "outdated_and_resolved_blurred",
            Self::InferredFixShownAsExact => "inferred_fix_shown_as_exact",
            Self::EvidenceTimelineOpaqueLog => "evidence_timeline_opaque_log",
            Self::DiagnosticFreshnessUnstated => "diagnostic_freshness_unstated",
            Self::AiConfidenceUnstated => "ai_confidence_unstated",
            Self::DiffChangeKindCollapsed => "diff_change_kind_collapsed",
            Self::AnchorStateUnstated => "anchor_state_unstated",
            Self::GenericChromeWordingUsed => "generic_chrome_wording_used",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed editor-inline component family bound to the surface-specific
/// truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorInlineComponentRow {
    /// Governed component family.
    pub component_family: M5EditorInlineComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5EditorInlineQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5EditorInlineSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5EditorInlineDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5EditorInlineRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5EditorInlineRequiredLabel>,
    /// Inline dispositions this component can carry (the frozen AC vocabulary; required on every
    /// component).
    pub dispositions: Vec<M5EditorInlineDisposition>,
    /// Editor-tab states this component names (tab-state-bearing families only).
    pub tab_states: Vec<M5EditorTabState>,
    /// Gutter-marker kinds this component names (gutter-bearing families only).
    pub gutter_marker_kinds: Vec<M5GutterMarkerKind>,
    /// Diagnostic severities this component names (severity-bearing families only).
    pub diagnostic_severities: Vec<M5DiagnosticSeverity>,
    /// Fix postures this component names (fix-bearing families only).
    pub fix_postures: Vec<M5FixPosture>,
    /// Diff change kinds this component names (diff-bearing families only).
    pub diff_change_kinds: Vec<M5DiffChangeKind>,
    /// Anchor durabilities this component names (anchor-bearing families only).
    pub anchor_durabilities: Vec<M5AnchorDurability>,
    /// AI confidences this component names (confidence-bearing families only).
    pub ai_confidences: Vec<M5AiConfidence>,
    /// Evidence-disclosure states this component names (evidence-bearing families only).
    pub evidence_disclosures: Vec<M5EvidenceDisclosure>,
    /// Degraded reasons this component can name (required on every component).
    pub degraded_reasons: Vec<M5EditorInlineDegradedReason>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5EditorInlineAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5EditorInlineConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5EditorInlineDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component (must include its own canonical component
    /// schema so downstream rows have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never encodes tab / marker / diagnostic state by color alone.
    /// MUST be `false`.
    pub encodes_tab_marker_or_diagnostic_state_by_color_alone: bool,
    /// Hard invariant: this component never lets a comment anchor or evidence pointer silently drift.
    /// MUST be `false`.
    pub lets_comment_anchor_or_evidence_pointer_silently_drift: bool,
    /// Hard invariant: this component never blurs outdated and resolved review state together. MUST be
    /// `false`.
    pub blurs_outdated_and_resolved_review_state: bool,
    /// Hard invariant: this component never presents an inferred fix as exact. MUST be `false`.
    pub presents_inferred_fix_as_exact: bool,
    /// Hard invariant: this component never hides an evidence timeline in an opaque log without
    /// inspectable structure. MUST be `false`.
    pub hides_evidence_timeline_in_opaque_log: bool,
}

impl M5EditorInlineComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5EditorInlineRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5EditorInlineRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.encodes_tab_marker_or_diagnostic_state_by_color_alone
            && !self.lets_comment_anchor_or_evidence_pointer_silently_drift
            && !self.blurs_outdated_and_resolved_review_state
            && !self.presents_inferred_fix_as_exact
            && !self.hides_evidence_timeline_in_opaque_log
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorInlineVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Inline-disposition tokens.
    pub dispositions: Vec<String>,
    /// Editor-tab-state tokens.
    pub tab_states: Vec<String>,
    /// Gutter-marker-kind tokens.
    pub gutter_marker_kinds: Vec<String>,
    /// Diagnostic-severity tokens.
    pub diagnostic_severities: Vec<String>,
    /// Fix-posture tokens.
    pub fix_postures: Vec<String>,
    /// Diff-change-kind tokens.
    pub diff_change_kinds: Vec<String>,
    /// Anchor-durability tokens.
    pub anchor_durabilities: Vec<String>,
    /// AI-confidence tokens.
    pub ai_confidences: Vec<String>,
    /// Evidence-disclosure tokens.
    pub evidence_disclosures: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5EditorInlineVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5EditorInlineComponentFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5EditorInlineDisposition::ALL, |v| v.as_str()),
            tab_states: tokens(&M5EditorTabState::ALL, |v| v.as_str()),
            gutter_marker_kinds: tokens(&M5GutterMarkerKind::ALL, |v| v.as_str()),
            diagnostic_severities: tokens(&M5DiagnosticSeverity::ALL, |v| v.as_str()),
            fix_postures: tokens(&M5FixPosture::ALL, |v| v.as_str()),
            diff_change_kinds: tokens(&M5DiffChangeKind::ALL, |v| v.as_str()),
            anchor_durabilities: tokens(&M5AnchorDurability::ALL, |v| v.as_str()),
            ai_confidences: tokens(&M5AiConfidence::ALL, |v| v.as_str()),
            evidence_disclosures: tokens(&M5EvidenceDisclosure::ALL, |v| v.as_str()),
            surface_families: tokens(&M5EditorInlineSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5EditorInlineDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5EditorInlineConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5EditorInlineAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5EditorInlineDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5EditorInlineRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5EditorInlineDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5EditorInlineGovernanceReview {
    /// The editor tab shows its state and document context.
    pub editor_tab_shows_state_and_context: bool,
    /// The gutter layers markers without relying on color alone.
    pub gutter_layers_markers_without_color_only: bool,
    /// The diagnostic decoration shows severity and freshness.
    pub diagnostic_decoration_shows_severity_and_freshness: bool,
    /// The code-action chip distinguishes exact from inferred fixes.
    pub code_action_chip_distinguishes_exact_from_inferred: bool,
    /// The diff view names every change kind.
    pub diff_view_names_every_change_kind: bool,
    /// The review thread shows anchor durability and resolution truth.
    pub review_thread_shows_anchor_and_resolution_truth: bool,
    /// The AI message card shows source, confidence, and actions.
    pub ai_message_card_shows_source_confidence_and_actions: bool,
    /// The evidence timeline is inspectable, not an opaque log.
    pub evidence_timeline_is_inspectable_not_opaque: bool,
    /// State is never encoded by color alone.
    pub state_never_encoded_by_color_alone: bool,
    /// Comment anchors never drift silently.
    pub comment_anchors_never_drift_silently: bool,
    /// Evidence pointers never drift silently.
    pub evidence_pointers_never_drift_silently: bool,
    /// Outdated and resolved review state is never blurred.
    pub outdated_and_resolved_never_blurred: bool,
    /// Inferred fixes are never presented as exact.
    pub inferred_fix_never_presented_as_exact: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel inline vocabulary.
    pub later_rows_cannot_invent_parallel_inline_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorInlineConsumerProjection {
    /// Editor surfaces consume the shared tab and gutter vocabulary.
    pub editor_surfaces_consume_tab_and_gutter_vocabulary: bool,
    /// Diff surfaces consume the shared change-kind vocabulary.
    pub diff_surfaces_consume_change_kind_vocabulary: bool,
    /// Review consumes the shared anchor and resolution vocabulary.
    pub review_consumes_anchor_and_resolution_vocabulary: bool,
    /// AI surfaces consume the shared confidence and evidence vocabulary.
    pub ai_surfaces_consume_confidence_and_evidence_vocabulary: bool,
    /// The notebook consumes the shared inline-component vocabulary.
    pub notebook_consumes_inline_component_vocabulary: bool,
    /// Support / export reads a single canonical inline source.
    pub support_export_reads_single_inline_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorInlineProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the editor-inline component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorInlineReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting editor-inline component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5EditorInlineComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EditorInlineComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5EditorInlineComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EditorInlineVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EditorInlineGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EditorInlineConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EditorInlineProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EditorInlineReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 editor-inline component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EditorInlineComponentMatrixPacket {
    /// Record kind; must equal [`M5_EDITOR_INLINE_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EDITOR_INLINE_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5EditorInlineComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5EditorInlineVocabularySet,
    /// Governance-review block.
    pub governance_review: M5EditorInlineGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5EditorInlineConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5EditorInlineProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5EditorInlineReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5EditorInlineComponentMatrixPacket {
    /// Builds an M5 editor-inline component matrix packet from stable-lane input.
    pub fn new(input: M5EditorInlineComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_EDITOR_INLINE_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_EDITOR_INLINE_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 editor-inline component matrix invariants.
    pub fn validate(&self) -> Vec<M5EditorInlineComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EDITOR_INLINE_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5EditorInlineComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EDITOR_INLINE_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5EditorInlineComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5EditorInlineComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 editor-inline component matrix serializes"),
        ) {
            violations.push(M5EditorInlineComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 editor-inline component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.component_family.canonical_component_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Editor-Tab, Gutter, Diagnostic-Decoration, Code-Action-Chip, Diff-View, Review-Thread, AI-Message-Card, and Evidence-Timeline Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Inline dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Fix postures: {}\n",
            self.vocabulary_set.fix_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.component_family.canonical_component_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
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

/// Errors emitted when reading the checked-in M5 editor-inline matrix export.
#[derive(Debug)]
pub enum M5EditorInlineComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5EditorInlineComponentMatrixViolation>),
}

impl fmt::Display for M5EditorInlineComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 editor-inline component matrix export parse failed: {error}"
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
                    "m5 editor-inline component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5EditorInlineComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5EditorInlineComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5EditorInlineComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row does not point at its own canonical component schema.
    ComponentSchemaRefMissing,
    /// A component declares no inline dispositions.
    DispositionMissing,
    /// A tab-state-bearing component declares no tab states.
    TabStateMissing,
    /// A gutter-bearing component declares no gutter-marker kinds.
    GutterMarkerMissing,
    /// A severity-bearing component declares no diagnostic severities.
    DiagnosticSeverityMissing,
    /// A fix-bearing component declares no fix postures.
    FixPostureMissing,
    /// A diff-bearing component declares no diff change kinds.
    DiffChangeKindMissing,
    /// An anchor-bearing component declares no anchor durabilities.
    AnchorDurabilityMissing,
    /// A confidence-bearing component declares no AI confidences.
    AiConfidenceMissing,
    /// An evidence-bearing component declares no evidence-disclosure states.
    EvidenceDisclosureMissing,
    /// A component declares no degraded reasons.
    DegradedReasonMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (encodes state by color alone, lets an anchor or evidence
    /// pointer drift, blurs outdated and resolved review state, presents an inferred fix as exact, or
    /// hides an evidence timeline in an opaque log).
    ComponentInvariantViolated,
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

impl M5EditorInlineComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::DispositionMissing => "disposition_missing",
            Self::TabStateMissing => "tab_state_missing",
            Self::GutterMarkerMissing => "gutter_marker_missing",
            Self::DiagnosticSeverityMissing => "diagnostic_severity_missing",
            Self::FixPostureMissing => "fix_posture_missing",
            Self::DiffChangeKindMissing => "diff_change_kind_missing",
            Self::AnchorDurabilityMissing => "anchor_durability_missing",
            Self::AiConfidenceMissing => "ai_confidence_missing",
            Self::EvidenceDisclosureMissing => "evidence_disclosure_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 editor-inline matrix export.
pub fn current_stable_m5_editor_inline_component_matrix_export(
) -> Result<M5EditorInlineComponentMatrixPacket, M5EditorInlineComponentMatrixArtifactError> {
    let packet: M5EditorInlineComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-editor-inline-proof/support_export.json"
    )))
    .map_err(M5EditorInlineComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5EditorInlineComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5EditorInlineComponentMatrixPacket,
    violations: &mut Vec<M5EditorInlineComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
        M5_EDITOR_TAB_SCHEMA_REF,
        M5_GUTTER_MARKER_SCHEMA_REF,
        M5_DIAGNOSTIC_DECORATION_SCHEMA_REF,
        M5_CODE_ACTION_CHIP_SCHEMA_REF,
        M5_DIFF_VIEW_SCHEMA_REF,
        M5_REVIEW_THREAD_SCHEMA_REF,
        M5_AI_MESSAGE_CARD_SCHEMA_REF,
        M5_EVIDENCE_TIMELINE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5EditorInlineComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5EditorInlineComponentMatrixPacket,
    violations: &mut Vec<M5EditorInlineComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5EditorInlineComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5EditorInlineComponentMatrixPacket,
    violations: &mut Vec<M5EditorInlineComponentMatrixViolation>,
) {
    let present: BTreeSet<M5EditorInlineComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5EditorInlineComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5EditorInlineComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5EditorInlineComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5EditorInlineComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations.push(M5EditorInlineComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::DispositionMissing);
        }
        if family.declares_tab_state() && row.tab_states.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::TabStateMissing);
        }
        if family.declares_gutter_marker() && row.gutter_marker_kinds.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::GutterMarkerMissing);
        }
        if family.declares_diagnostic_severity() && row.diagnostic_severities.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::DiagnosticSeverityMissing);
        }
        if family.declares_fix_posture() && row.fix_postures.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::FixPostureMissing);
        }
        if family.declares_diff_change_kind() && row.diff_change_kinds.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::DiffChangeKindMissing);
        }
        if family.declares_anchor_durability() && row.anchor_durabilities.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::AnchorDurabilityMissing);
        }
        if family.declares_ai_confidence() && row.ai_confidences.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::AiConfidenceMissing);
        }
        if family.declares_evidence_disclosure() && row.evidence_disclosures.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::EvidenceDisclosureMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5EditorInlineComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5EditorInlineComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5EditorInlineComponentMatrixPacket,
    violations: &mut Vec<M5EditorInlineComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.editor_tab_shows_state_and_context,
        review.gutter_layers_markers_without_color_only,
        review.diagnostic_decoration_shows_severity_and_freshness,
        review.code_action_chip_distinguishes_exact_from_inferred,
        review.diff_view_names_every_change_kind,
        review.review_thread_shows_anchor_and_resolution_truth,
        review.ai_message_card_shows_source_confidence_and_actions,
        review.evidence_timeline_is_inspectable_not_opaque,
        review.state_never_encoded_by_color_alone,
        review.comment_anchors_never_drift_silently,
        review.evidence_pointers_never_drift_silently,
        review.outdated_and_resolved_never_blurred,
        review.inferred_fix_never_presented_as_exact,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_inline_vocabulary,
    ] {
        if !ok {
            violations.push(M5EditorInlineComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5EditorInlineComponentMatrixPacket,
    violations: &mut Vec<M5EditorInlineComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_surfaces_consume_tab_and_gutter_vocabulary,
        projection.diff_surfaces_consume_change_kind_vocabulary,
        projection.review_consumes_anchor_and_resolution_vocabulary,
        projection.ai_surfaces_consume_confidence_and_evidence_vocabulary,
        projection.notebook_consumes_inline_component_vocabulary,
        projection.support_export_reads_single_inline_source,
    ] {
        if !ok {
            violations.push(M5EditorInlineComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5EditorInlineComponentMatrixPacket,
    violations: &mut Vec<M5EditorInlineComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5EditorInlineComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5EditorInlineComponentMatrixPacket,
    violations: &mut Vec<M5EditorInlineComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5EditorInlineComponentMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses editor / review / AI words; what is rejected is a raw secret *value*
/// shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
