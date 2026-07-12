//! Implemented M5 diff-view and review-thread primitives.
//!
//! The frozen [editor-inline component matrix][matrix] names the reusable editor / review / AI inline
//! UI components and locks their controlled vocabulary. This module is the third implement lane over
//! that matrix (after the [editor-tab / gutter lane][tabgutter] and the
//! [diagnostic-decoration / code-action-chip lane][diagchip]): it turns the two inline *review-flow*
//! components — the **diff view** and the **review thread** — into resolvers that produce export-safe,
//! honest projections, so a user can read what a diff shows (change kind, moved-versus-hidden context,
//! source-versus-rendered truth, and stable hunk identity) and what a review comment means (draft,
//! published, resolved, outdated, re-anchored, locked, or pending-send state, comment-anchor
//! durability, and provider-local-versus-provider-hosted origin) *without* that truth collapsing into a
//! single immutable view, blurring outdated and resolved state, or drifting between desktop, browser
//! handoff, and exported review packets.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render diff views with moved or hidden-context indicators, source-versus-rendered truth where
//!   relevant, stable hunk identity, and export-safe structural summaries.** [`resolve_diff_view`]
//!   refuses to read as a clean diff when the hunk identity is unstated, the change kind is collapsed,
//!   the context visibility is unresolved, a moved region is hidden, collapsed or elided context is not
//!   disclosed, the source-versus-rendered relationship is unresolved or blurred, the hunk identity is
//!   unresolved or has silently drifted, the structural summary is opaque, or no command-backed detail
//!   path is reachable; it degrades instead.
//! * **Render review threads with draft, published, resolved, outdated, re-anchored, locked, and
//!   pending-send states using one controlled vocabulary.** [`resolve_review_thread`] degrades when the
//!   thread identity is unstated, the thread state is unresolved or encoded by color / provider-specific
//!   jargon, outdated and resolved state are blurred, the comment-anchor durability is unresolved or has
//!   silently drifted, the provider-local-versus-provider-hosted distinction is unresolved or implicit,
//!   a draft / pending-send thread reads as published, or no command-backed detail path is reachable.
//! * **Keep the provider-local-versus-provider-hosted distinction explicit so desktop, browser handoff,
//!   and exported review packets do not drift on comment truth.** The packet proves, by resolved
//!   examples, that the same thread-state grammar and anchor-durability behavior hold across surfaces,
//!   that diff consumers stay honest when context is moved, elided, collapsed, or re-anchored rather
//!   than pretending one immutable view, and that a user can distinguish outdated from resolved state
//!   without relying on color or provider-specific jargon.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EditorInlineDisposition`] inline-disposition vocabulary, the [`M5DiffChangeKind`] diff-change-kind
//! vocabulary, and the [`M5AnchorDurability`] anchor-durability vocabulary — so editor, diff, review,
//! notebook, support, and export surfaces can never fork their own change-kind or anchor wording. Raw
//! secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_editor_inline_component_matrix
//! [tabgutter]: crate::m5_editor_tab_and_gutter_state_and_marker_layering
//! [diagchip]: crate::m5_diagnostic_decoration_and_code_action_chip_state_and_fix_posture

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_diff_review_controls, seeded_m5_diff_review_controls_diff_ui_beta_narrowed,
    seeded_m5_diff_review_controls_review_ui_preview_narrowed, M5_DIFF_REVIEW_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_editor_inline_component_matrix::{
    M5AnchorDurability, M5DiffChangeKind, M5EditorInlineAccessibilityRoute,
    M5EditorInlineComponentFamily, M5EditorInlineConsumerSurface, M5EditorInlineDeploymentLine,
    M5EditorInlineDisposition, M5EditorInlineDowngradeTrigger, M5EditorInlineQualificationClass,
    M5EditorInlineRequiredLabel, M5_DIFF_VIEW_SCHEMA_REF, M5_EDITOR_INLINE_COMPONENT_DOC_REF,
    M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF, M5_REVIEW_THREAD_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5DiffReviewControlsPacket`].
pub const M5_DIFF_REVIEW_CONTROLS_RECORD_KIND: &str =
    "implement_m5_diff_view_and_review_thread_controls";

/// Schema version for M5 diff-view / review-thread controls records.
pub const M5_DIFF_REVIEW_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_DIFF_REVIEW_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-diff-view-review-thread-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_DIFF_REVIEW_CONTROLS_DOC_REF: &str =
    "docs/editor/m5_diff_view_and_review_thread_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DIFF_REVIEW_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-diff-view-review-thread-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_DIFF_REVIEW_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-diff-view-review-thread-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_DIFF_REVIEW_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-diff-view-review-thread-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_DIFF_REVIEW_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-diff-view-review-thread-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy
/// so no lane invents a parallel surface set.
pub type M5DiffReviewConsumerSurface = M5EditorInlineConsumerSurface;

/// Controlled context visibility a diff view names, so a diff consumer stays honest when context is
/// moved, elided, collapsed, or re-anchored rather than pretending one immutable view. Minted by this
/// lane because the frozen matrix carries diff *change kind* but not the moved-versus-hidden-context
/// axis the diff-view acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffContextVisibility {
    /// Full surrounding context is shown.
    FullContext,
    /// Surrounding context is collapsed to a summary.
    CollapsedContext,
    /// Surrounding context is elided (hidden with an explicit gap marker).
    ElidedContext,
    /// The region was moved from elsewhere.
    MovedContext,
    /// The context was re-anchored after the underlying text moved.
    ReAnchoredContext,
    /// The context visibility cannot currently be resolved.
    VisibilityUnresolved,
}

impl M5DiffContextVisibility {
    /// Every context visibility, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullContext,
        Self::CollapsedContext,
        Self::ElidedContext,
        Self::MovedContext,
        Self::ReAnchoredContext,
        Self::VisibilityUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullContext => "full_context",
            Self::CollapsedContext => "collapsed_context",
            Self::ElidedContext => "elided_context",
            Self::MovedContext => "moved_context",
            Self::ReAnchoredContext => "re_anchored_context",
            Self::VisibilityUnresolved => "visibility_unresolved",
        }
    }

    /// Whether this names a moved region whose provenance must be disclosed.
    pub const fn is_moved(self) -> bool {
        matches!(self, Self::MovedContext)
    }

    /// Whether this names hidden (collapsed or elided) context that must be disclosed.
    pub const fn is_context_hidden(self) -> bool {
        matches!(self, Self::CollapsedContext | Self::ElidedContext)
    }

    /// Whether the context visibility is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::VisibilityUnresolved)
    }
}

/// Controlled source-versus-rendered relationship a diff view names, so a rendered / transformed diff
/// is never mistaken for the exact source bytes. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffSourceRendering {
    /// The diff shows the exact source bytes.
    SourceExact,
    /// The diff is rendered but faithful to the source.
    RenderedFaithful,
    /// The diff is rendered and only approximate.
    RenderedApproximate,
    /// The diff is rendered through a transform (e.g. prettified / normalized).
    RenderedTransformed,
    /// The underlying content is binary or otherwise opaque.
    BinaryOrOpaque,
    /// The source-versus-rendered relationship cannot currently be resolved.
    RenderingUnresolved,
}

impl M5DiffSourceRendering {
    /// Every source-rendering relationship, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SourceExact,
        Self::RenderedFaithful,
        Self::RenderedApproximate,
        Self::RenderedTransformed,
        Self::BinaryOrOpaque,
        Self::RenderingUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceExact => "source_exact",
            Self::RenderedFaithful => "rendered_faithful",
            Self::RenderedApproximate => "rendered_approximate",
            Self::RenderedTransformed => "rendered_transformed",
            Self::BinaryOrOpaque => "binary_or_opaque",
            Self::RenderingUnresolved => "rendering_unresolved",
        }
    }

    /// Whether this relationship must disclose that the shown diff is not the exact source bytes.
    pub const fn needs_disclosure(self) -> bool {
        matches!(
            self,
            Self::RenderedApproximate | Self::RenderedTransformed | Self::BinaryOrOpaque
        )
    }

    /// Whether the source-rendering relationship is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::RenderingUnresolved)
    }
}

/// Controlled hunk identity a diff view names, so a hunk keeps a stable identity across rebases,
/// merges, and re-renders rather than silently drifting. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffHunkIdentity {
    /// A stable hunk id that has not moved.
    StableHunkId,
    /// A hunk id re-identified after a rebase.
    RebasedHunkId,
    /// A hunk id synthesized for a generated / rendered view.
    SynthesizedHunkId,
    /// A hunk id merged from multiple upstream hunks.
    MergedHunkId,
    /// A hunk id that has become unstable relative to its source.
    UnstableHunkId,
    /// The hunk identity cannot currently be resolved.
    HunkIdUnresolved,
}

impl M5DiffHunkIdentity {
    /// Every hunk identity, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StableHunkId,
        Self::RebasedHunkId,
        Self::SynthesizedHunkId,
        Self::MergedHunkId,
        Self::UnstableHunkId,
        Self::HunkIdUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableHunkId => "stable_hunk_id",
            Self::RebasedHunkId => "rebased_hunk_id",
            Self::SynthesizedHunkId => "synthesized_hunk_id",
            Self::MergedHunkId => "merged_hunk_id",
            Self::UnstableHunkId => "unstable_hunk_id",
            Self::HunkIdUnresolved => "hunk_id_unresolved",
        }
    }

    /// Whether this is a stable hunk id that needs no re-identification disclosure.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::StableHunkId)
    }

    /// Whether the hunk identity is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::HunkIdUnresolved)
    }
}

/// Controlled review-thread state a review thread names, using one shared vocabulary across every
/// claimed M5 review flow. Minted by this lane to carry the exact state list the review-thread
/// acceptance criteria require by name (draft, published, resolved, outdated, re-anchored, locked, and
/// pending-send).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewThreadState {
    /// An unsent draft comment.
    Draft,
    /// A published comment.
    Published,
    /// A resolved thread.
    Resolved,
    /// An outdated thread (its code moved on beneath it).
    Outdated,
    /// A thread re-anchored after the underlying text moved.
    ReAnchored,
    /// A locked thread.
    Locked,
    /// A comment queued for send but not yet published.
    PendingSend,
    /// The thread state cannot currently be resolved.
    StateUnknown,
}

impl M5ReviewThreadState {
    /// Every review-thread state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Draft,
        Self::Published,
        Self::Resolved,
        Self::Outdated,
        Self::ReAnchored,
        Self::Locked,
        Self::PendingSend,
        Self::StateUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Published => "published",
            Self::Resolved => "resolved",
            Self::Outdated => "outdated",
            Self::ReAnchored => "re_anchored",
            Self::Locked => "locked",
            Self::PendingSend => "pending_send",
            Self::StateUnknown => "state_unknown",
        }
    }

    /// Whether this names the resolved review state.
    pub const fn is_review_resolved(self) -> bool {
        matches!(self, Self::Resolved)
    }

    /// Whether this names the outdated review state.
    pub const fn is_outdated(self) -> bool {
        matches!(self, Self::Outdated)
    }

    /// Whether this names the outdated or resolved review state (the two that must stay distinct).
    pub const fn is_outdated_or_resolved(self) -> bool {
        matches!(self, Self::Outdated | Self::Resolved)
    }

    /// Whether this names a draft / pending-send thread that must never read as published.
    pub const fn needs_send(self) -> bool {
        matches!(self, Self::Draft | Self::PendingSend)
    }

    /// Whether the thread state is known (not the unknown sentinel).
    pub const fn is_known(self) -> bool {
        !matches!(self, Self::StateUnknown)
    }
}

/// Controlled provider locality a review thread names, so the provider-local-versus-provider-hosted
/// distinction stays explicit and desktop, browser handoff, and exported review packets never drift on
/// comment truth. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewProviderLocality {
    /// The thread is stored provider-locally (desktop / local review).
    ProviderLocal,
    /// The thread is hosted by the review provider.
    ProviderHosted,
    /// A locally mirrored copy of a hosted thread.
    MirroredLocal,
    /// A thread mid-handoff between desktop and browser.
    HandoffPending,
    /// A thread detached into an export packet.
    DetachedExport,
    /// The provider locality cannot currently be resolved.
    LocalityUnresolved,
}

impl M5ReviewProviderLocality {
    /// Every provider locality, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderLocal,
        Self::ProviderHosted,
        Self::MirroredLocal,
        Self::HandoffPending,
        Self::DetachedExport,
        Self::LocalityUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderLocal => "provider_local",
            Self::ProviderHosted => "provider_hosted",
            Self::MirroredLocal => "mirrored_local",
            Self::HandoffPending => "handoff_pending",
            Self::DetachedExport => "detached_export",
            Self::LocalityUnresolved => "locality_unresolved",
        }
    }

    /// Whether the thread is hosted by the review provider.
    pub const fn is_hosted(self) -> bool {
        matches!(self, Self::ProviderHosted)
    }

    /// Whether the provider locality is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::LocalityUnresolved)
    }
}

/// One mandatory rendered part a diff view or review thread must be able to show, so no change-kind,
/// context, source-rendering, hunk-identity, thread-state, anchor, or provider-locality fact is left
/// implicit behind compact chrome, a tooltip, or a secondary panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffReviewAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed inline disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The diff change kind (diff).
    ChangeKind,
    /// The moved-versus-hidden context visibility (diff).
    ContextVisibility,
    /// The source-versus-rendered relationship (diff).
    SourceRendering,
    /// The stable hunk identity (diff).
    HunkIdentity,
    /// The controlled review-thread state (thread).
    ThreadState,
    /// The comment-anchor durability (thread).
    AnchorDurability,
    /// The provider-local-versus-provider-hosted locality (thread).
    ProviderLocality,
    /// The outdated-versus-resolved distinction (thread).
    OutdatedResolvedDistinction,
    /// The command-backed path to trace the diff or thread (both components).
    StateCommand,
}

impl M5DiffReviewAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ChangeKind,
        Self::ContextVisibility,
        Self::SourceRendering,
        Self::HunkIdentity,
        Self::ThreadState,
        Self::AnchorDurability,
        Self::ProviderLocality,
        Self::OutdatedResolvedDistinction,
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
            Self::ChangeKind => "change_kind",
            Self::ContextVisibility => "context_visibility",
            Self::SourceRendering => "source_rendering",
            Self::HunkIdentity => "hunk_identity",
            Self::ThreadState => "thread_state",
            Self::AnchorDurability => "anchor_durability",
            Self::ProviderLocality => "provider_locality",
            Self::OutdatedResolvedDistinction => "outdated_resolved_distinction",
            Self::StateCommand => "state_command",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to trace a diff hunk
/// or understand a review thread behind a degraded component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffReviewNextAction {
    /// Open the command-backed component detail.
    OpenComponentDetail,
    /// Inspect the moved or hidden context behind the diff.
    InspectMovedOrHiddenContext,
    /// Review the controlled thread state.
    ReviewThreadState,
    /// Distinguish outdated from resolved review state.
    DistinguishOutdatedFromResolved,
    /// Review the provider-local-versus-provider-hosted locality.
    ReviewProviderLocality,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5DiffReviewNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenComponentDetail,
        Self::InspectMovedOrHiddenContext,
        Self::ReviewThreadState,
        Self::DistinguishOutdatedFromResolved,
        Self::ReviewProviderLocality,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenComponentDetail => "open_component_detail",
            Self::InspectMovedOrHiddenContext => "inspect_moved_or_hidden_context",
            Self::ReviewThreadState => "review_thread_state",
            Self::DistinguishOutdatedFromResolved => "distinguish_outdated_from_resolved",
            Self::ReviewProviderLocality => "review_provider_locality",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffReviewExportField {
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
    /// The diff context visibility named by the diff view.
    ContextVisibility,
    /// The source-versus-rendered relationship named by the diff view.
    SourceRendering,
    /// The stable hunk identity named by the diff view.
    HunkIdentity,
    /// The controlled review-thread state named by the review thread.
    ThreadState,
    /// The comment-anchor durability named by the review thread.
    AnchorDurability,
    /// The provider locality named by the review thread.
    ProviderLocality,
    /// The accountable owner role.
    OwnerRole,
}

impl M5DiffReviewExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::ContextVisibility,
        Self::SourceRendering,
        Self::HunkIdentity,
        Self::ThreadState,
        Self::AnchorDurability,
        Self::ProviderLocality,
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
            Self::ContextVisibility => "context_visibility",
            Self::SourceRendering => "source_rendering",
            Self::HunkIdentity => "hunk_identity",
            Self::ThreadState => "thread_state",
            Self::AnchorDurability => "anchor_durability",
            Self::ProviderLocality => "provider_locality",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a diff view degraded below a clean, legible state. The degrade-first ladder returns one of
/// these instead of ever letting an ambiguous diff read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffViewDegradeReason {
    /// The hunk identity / label is unstated; a user cannot tell what the hunk represents.
    DiffIdentityUnstated,
    /// The change kind is collapsed into an ambiguous generic change.
    ChangeKindCollapsed,
    /// The context visibility cannot currently be resolved.
    ContextVisibilityUnresolved,
    /// A moved region is hidden rather than disclosed as moved.
    MovedContextHidden,
    /// Collapsed or elided context is not disclosed.
    HiddenContextNotDisclosed,
    /// The source-versus-rendered relationship cannot currently be resolved.
    SourceRenderingUnresolved,
    /// A rendered / transformed diff is blurred with the exact source bytes.
    SourceVersusRenderedBlurred,
    /// The hunk identity cannot currently be resolved.
    HunkIdentityUnresolved,
    /// The hunk identity drifted without being disclosed as re-identified.
    HunkIdentityDrifted,
    /// The structural summary is opaque rather than an inspectable, export-safe structure.
    StructuralSummaryOpaque,
    /// No command-backed path to trace the diff is reachable.
    DiffDetailPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5DiffViewDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::DiffIdentityUnstated,
        Self::ChangeKindCollapsed,
        Self::ContextVisibilityUnresolved,
        Self::MovedContextHidden,
        Self::HiddenContextNotDisclosed,
        Self::SourceRenderingUnresolved,
        Self::SourceVersusRenderedBlurred,
        Self::HunkIdentityUnresolved,
        Self::HunkIdentityDrifted,
        Self::StructuralSummaryOpaque,
        Self::DiffDetailPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffIdentityUnstated => "diff_identity_unstated",
            Self::ChangeKindCollapsed => "change_kind_collapsed",
            Self::ContextVisibilityUnresolved => "context_visibility_unresolved",
            Self::MovedContextHidden => "moved_context_hidden",
            Self::HiddenContextNotDisclosed => "hidden_context_not_disclosed",
            Self::SourceRenderingUnresolved => "source_rendering_unresolved",
            Self::SourceVersusRenderedBlurred => "source_versus_rendered_blurred",
            Self::HunkIdentityUnresolved => "hunk_identity_unresolved",
            Self::HunkIdentityDrifted => "hunk_identity_drifted",
            Self::StructuralSummaryOpaque => "structural_summary_opaque",
            Self::DiffDetailPathMissing => "diff_detail_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DiffReviewNextAction {
        match self {
            Self::DiffIdentityUnstated
            | Self::ChangeKindCollapsed
            | Self::ContextVisibilityUnresolved
            | Self::MovedContextHidden
            | Self::HiddenContextNotDisclosed => {
                M5DiffReviewNextAction::InspectMovedOrHiddenContext
            }
            Self::SourceRenderingUnresolved
            | Self::SourceVersusRenderedBlurred
            | Self::HunkIdentityUnresolved
            | Self::HunkIdentityDrifted
            | Self::StructuralSummaryOpaque
            | Self::DiffDetailPathMissing
            | Self::ProofStale => M5DiffReviewNextAction::OpenComponentDetail,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EditorInlineDowngradeTrigger {
        match self {
            Self::ChangeKindCollapsed
            | Self::MovedContextHidden
            | Self::HiddenContextNotDisclosed => {
                M5EditorInlineDowngradeTrigger::DiffChangeKindCollapsed
            }
            Self::HunkIdentityDrifted => M5EditorInlineDowngradeTrigger::AnchorStateUnstated,
            Self::StructuralSummaryOpaque => {
                M5EditorInlineDowngradeTrigger::EvidenceTimelineOpaqueLog
            }
            Self::ProofStale => M5EditorInlineDowngradeTrigger::ProofStale,
            _ => M5EditorInlineDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// Reason a review thread degraded below a clean, legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewThreadDegradeReason {
    /// The thread identity / comment label is unstated.
    ThreadIdentityUnstated,
    /// The thread state cannot currently be resolved.
    ThreadStateUnresolved,
    /// The thread state is encoded by color or provider-specific jargon rather than named.
    ThreadStateEncodedByColorAlone,
    /// Outdated and resolved review state are blurred together.
    OutdatedResolvedBlurred,
    /// The comment-anchor durability cannot currently be resolved.
    AnchorDurabilityUnresolved,
    /// The comment anchor drifted, went outdated, or was orphaned without being disclosed.
    AnchorDriftHidden,
    /// The provider locality cannot currently be resolved.
    ProviderLocalityUnresolved,
    /// The provider-local-versus-provider-hosted distinction is left implicit.
    ProviderDistinctionImplicit,
    /// A draft / pending-send thread reads as published.
    PendingSendHidden,
    /// No command-backed path to trace the thread is reachable.
    ThreadDetailPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ReviewThreadDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ThreadIdentityUnstated,
        Self::ThreadStateUnresolved,
        Self::ThreadStateEncodedByColorAlone,
        Self::OutdatedResolvedBlurred,
        Self::AnchorDurabilityUnresolved,
        Self::AnchorDriftHidden,
        Self::ProviderLocalityUnresolved,
        Self::ProviderDistinctionImplicit,
        Self::PendingSendHidden,
        Self::ThreadDetailPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThreadIdentityUnstated => "thread_identity_unstated",
            Self::ThreadStateUnresolved => "thread_state_unresolved",
            Self::ThreadStateEncodedByColorAlone => "thread_state_encoded_by_color_alone",
            Self::OutdatedResolvedBlurred => "outdated_resolved_blurred",
            Self::AnchorDurabilityUnresolved => "anchor_durability_unresolved",
            Self::AnchorDriftHidden => "anchor_drift_hidden",
            Self::ProviderLocalityUnresolved => "provider_locality_unresolved",
            Self::ProviderDistinctionImplicit => "provider_distinction_implicit",
            Self::PendingSendHidden => "pending_send_hidden",
            Self::ThreadDetailPathMissing => "thread_detail_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DiffReviewNextAction {
        match self {
            Self::ThreadIdentityUnstated
            | Self::ThreadStateUnresolved
            | Self::ThreadStateEncodedByColorAlone
            | Self::PendingSendHidden => M5DiffReviewNextAction::ReviewThreadState,
            Self::OutdatedResolvedBlurred => {
                M5DiffReviewNextAction::DistinguishOutdatedFromResolved
            }
            Self::AnchorDurabilityUnresolved | Self::AnchorDriftHidden => {
                M5DiffReviewNextAction::ReviewThreadState
            }
            Self::ProviderLocalityUnresolved | Self::ProviderDistinctionImplicit => {
                M5DiffReviewNextAction::ReviewProviderLocality
            }
            Self::ThreadDetailPathMissing | Self::ProofStale => {
                M5DiffReviewNextAction::OpenComponentDetail
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EditorInlineDowngradeTrigger {
        match self {
            Self::ThreadStateEncodedByColorAlone => {
                M5EditorInlineDowngradeTrigger::TabMarkerDiagnosticColorOnly
            }
            Self::OutdatedResolvedBlurred => {
                M5EditorInlineDowngradeTrigger::OutdatedAndResolvedBlurred
            }
            Self::AnchorDurabilityUnresolved => M5EditorInlineDowngradeTrigger::AnchorStateUnstated,
            Self::AnchorDriftHidden => M5EditorInlineDowngradeTrigger::CommentAnchorDriftedSilently,
            Self::ProofStale => M5EditorInlineDowngradeTrigger::ProofStale,
            _ => M5EditorInlineDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// True when a comment anchor still points at a durable range (exact or cleanly re-anchored).
fn anchor_is_durable(anchor: M5AnchorDurability) -> bool {
    matches!(
        anchor,
        M5AnchorDurability::AnchoredExact | M5AnchorDurability::ReAnchored
    )
}

/// True when a comment anchor has drifted, gone outdated, or been orphaned.
fn anchor_is_drifted(anchor: M5AnchorDurability) -> bool {
    matches!(
        anchor,
        M5AnchorDurability::DriftedApproximate
            | M5AnchorDurability::OutdatedAnchor
            | M5AnchorDurability::OrphanedAnchor
    )
}

/// Input to [`resolve_diff_view`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DiffViewResolutionInput {
    /// Stable identity of the diff-view instance.
    pub diff_id: String,
    /// The hunk label / identity shown; empty means unstated.
    pub hunk_label: String,
    /// The diff change kind.
    pub change_kind: M5DiffChangeKind,
    /// True when the change kind is stated non-color-only (name / icon-with-label, never collapsed).
    pub change_kind_stated: bool,
    /// The moved-versus-hidden context visibility.
    pub context_visibility: M5DiffContextVisibility,
    /// True when a moved region is disclosed as moved, never hidden.
    pub moved_disclosed: bool,
    /// True when collapsed / elided context is disclosed as hidden, never pretending one full view.
    pub hidden_context_disclosed: bool,
    /// The source-versus-rendered relationship.
    pub source_rendering: M5DiffSourceRendering,
    /// True when a rendered / transformed / binary diff discloses that it is not the exact source.
    pub rendering_disclosed: bool,
    /// The stable hunk identity.
    pub hunk_identity: M5DiffHunkIdentity,
    /// True when a re-identified / drifted hunk is disclosed as such, never silently drifting.
    pub hunk_reidentification_disclosed: bool,
    /// True when the structural summary is an inspectable, export-safe structure, never an opaque blob.
    pub export_summary_structured: bool,
    /// True when a command-backed entrypoint to trace the diff is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe diff-view projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDiffView {
    /// Stable identity of the diff-view instance.
    pub diff_id: String,
    /// The hunk label / identity named by the diff view.
    pub hunk_label: String,
    /// The change-kind token named by the diff view.
    pub change_kind: String,
    /// Whether the change kind is stated non-color-only.
    pub change_kind_stated: bool,
    /// The context-visibility token named by the diff view.
    pub context_visibility: String,
    /// Whether this diff shows a moved region.
    pub context_is_moved: bool,
    /// Whether this diff hides (collapses / elides) context.
    pub context_is_hidden: bool,
    /// Whether a moved region is disclosed as moved.
    pub moved_disclosed: bool,
    /// Whether hidden context is disclosed.
    pub hidden_context_disclosed: bool,
    /// The source-rendering token named by the diff view.
    pub source_rendering: String,
    /// Whether the source-rendering relationship must disclose that the diff is not exact source.
    pub source_needs_disclosure: bool,
    /// Whether the rendered / transformed / binary relationship is disclosed.
    pub rendering_disclosed: bool,
    /// The hunk-identity token named by the diff view.
    pub hunk_identity: String,
    /// Whether the hunk identity is stable.
    pub hunk_is_stable: bool,
    /// Whether a re-identified / drifted hunk is disclosed.
    pub hunk_reidentification_disclosed: bool,
    /// Whether the structural summary is an inspectable, export-safe structure.
    pub export_summary_structured: bool,
    /// Whether a command-backed entrypoint to trace the diff is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the diff could not read as a clean, legible state.
    pub degrade_reason: Option<M5DiffViewDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DiffReviewNextAction,
    /// Whether the diff is legible at a glance (clean diff naming every fact).
    pub diff_legible_at_a_glance: bool,
}

impl M5ResolvedDiffView {
    /// Whether this diff reads as a clean, legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_review_thread`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReviewThreadResolutionInput {
    /// Stable identity of the review-thread instance.
    pub thread_id: String,
    /// The comment label / identity shown; empty means unstated.
    pub comment_label: String,
    /// The controlled review-thread state.
    pub thread_state: M5ReviewThreadState,
    /// True when the thread state is stated with a name, never color or provider-specific jargon.
    pub thread_state_stated: bool,
    /// True when outdated and resolved state are visibly distinguished, never blurred.
    pub outdated_resolved_distinguished: bool,
    /// The comment-anchor durability.
    pub anchor_durability: M5AnchorDurability,
    /// True when a drifted / outdated / orphaned anchor is disclosed, never silently drifted.
    pub anchor_drift_disclosed: bool,
    /// The provider locality.
    pub provider_locality: M5ReviewProviderLocality,
    /// True when the provider-local-versus-provider-hosted distinction is explicit.
    pub provider_distinction_explicit: bool,
    /// True when a draft / pending-send thread is disclosed as unsent, never reading as published.
    pub pending_send_disclosed: bool,
    /// True when a command-backed entrypoint to trace the thread is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe review-thread projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedReviewThread {
    /// Stable identity of the review-thread instance.
    pub thread_id: String,
    /// The comment label / identity named by the review thread.
    pub comment_label: String,
    /// The thread-state token named by the review thread.
    pub thread_state: String,
    /// Whether the thread state is stated (name, never color / jargon).
    pub thread_state_stated: bool,
    /// Whether the thread names the outdated review state.
    pub is_outdated: bool,
    /// Whether the thread names the resolved review state.
    pub is_resolved: bool,
    /// Whether the thread names outdated or resolved state (the two that must stay distinct).
    pub is_outdated_or_resolved: bool,
    /// Whether outdated and resolved state are visibly distinguished.
    pub outdated_resolved_distinguished: bool,
    /// Whether the thread is a draft / pending-send that must never read as published.
    pub needs_send: bool,
    /// Whether a draft / pending-send thread is disclosed as unsent.
    pub pending_send_disclosed: bool,
    /// The anchor-durability token named by the review thread.
    pub anchor_durability: String,
    /// Whether the comment anchor still points at a durable range.
    pub anchor_is_durable: bool,
    /// Whether the comment anchor has drifted / gone outdated / been orphaned.
    pub anchor_is_drifted: bool,
    /// Whether a drifted anchor is disclosed.
    pub anchor_drift_disclosed: bool,
    /// The provider-locality token named by the review thread.
    pub provider_locality: String,
    /// Whether the thread is hosted by the review provider.
    pub provider_is_hosted: bool,
    /// Whether the provider-local-versus-provider-hosted distinction is explicit.
    pub provider_distinction_explicit: bool,
    /// Whether a command-backed entrypoint to trace the thread is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the thread could not read as a clean, legible state.
    pub degrade_reason: Option<M5ReviewThreadDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DiffReviewNextAction,
    /// Whether the thread is legible at a glance (clean thread naming every fact).
    pub thread_legible_at_a_glance: bool,
}

impl M5ResolvedReviewThread {
    /// Whether this thread reads as a clean, legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5DiffReviewResolutionError {
    /// The diff id was empty.
    EmptyDiffId,
    /// The thread id was empty.
    EmptyThreadId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5DiffReviewResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyDiffId => "empty_diff_id",
            Self::EmptyThreadId => "empty_thread_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5DiffReviewResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 diff-view / review-thread resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DiffReviewResolutionError {}

/// Resolves a diff view so a hunk is legible at a glance and diff consumers stay honest when context is
/// moved, elided, collapsed, or re-anchored rather than pretending one immutable view: the diff names
/// its change kind (non-collapsed), context visibility (never hiding a moved or elided region),
/// source-versus-rendered relationship (never blurring a rendered diff with the exact source), and
/// stable hunk identity (never silently drifting), keeps an inspectable export-safe structural summary,
/// and always offers a command-backed detail entrypoint.
pub fn resolve_diff_view(
    input: M5DiffViewResolutionInput,
) -> Result<M5ResolvedDiffView, M5DiffReviewResolutionError> {
    if input.diff_id.trim().is_empty() {
        return Err(M5DiffReviewResolutionError::EmptyDiffId);
    }
    if string_is_forbidden(&input.diff_id) || string_is_forbidden(&input.hunk_label) {
        return Err(M5DiffReviewResolutionError::ForbiddenMaterial);
    }

    let context_is_moved = input.context_visibility.is_moved();
    let context_is_hidden = input.context_visibility.is_context_hidden();
    let source_needs_disclosure = input.source_rendering.needs_disclosure();
    let hunk_is_stable = input.hunk_identity.is_stable();

    let degrade_reason = if input.hunk_label.trim().is_empty() {
        Some(M5DiffViewDegradeReason::DiffIdentityUnstated)
    } else if !input.change_kind_stated {
        Some(M5DiffViewDegradeReason::ChangeKindCollapsed)
    } else if !input.context_visibility.is_resolved() {
        Some(M5DiffViewDegradeReason::ContextVisibilityUnresolved)
    } else if context_is_moved && !input.moved_disclosed {
        Some(M5DiffViewDegradeReason::MovedContextHidden)
    } else if context_is_hidden && !input.hidden_context_disclosed {
        Some(M5DiffViewDegradeReason::HiddenContextNotDisclosed)
    } else if !input.source_rendering.is_resolved() {
        Some(M5DiffViewDegradeReason::SourceRenderingUnresolved)
    } else if source_needs_disclosure && !input.rendering_disclosed {
        Some(M5DiffViewDegradeReason::SourceVersusRenderedBlurred)
    } else if !input.hunk_identity.is_resolved() {
        Some(M5DiffViewDegradeReason::HunkIdentityUnresolved)
    } else if !hunk_is_stable && !input.hunk_reidentification_disclosed {
        Some(M5DiffViewDegradeReason::HunkIdentityDrifted)
    } else if !input.export_summary_structured {
        Some(M5DiffViewDegradeReason::StructuralSummaryOpaque)
    } else if !input.detail_command_available {
        Some(M5DiffViewDegradeReason::DiffDetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5DiffViewDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DiffReviewNextAction::OpenComponentDetail,
    };

    Ok(M5ResolvedDiffView {
        diff_id: input.diff_id,
        hunk_label: input.hunk_label,
        change_kind: input.change_kind.as_str().to_owned(),
        change_kind_stated: input.change_kind_stated,
        context_visibility: input.context_visibility.as_str().to_owned(),
        context_is_moved,
        context_is_hidden,
        moved_disclosed: input.moved_disclosed,
        hidden_context_disclosed: input.hidden_context_disclosed,
        source_rendering: input.source_rendering.as_str().to_owned(),
        source_needs_disclosure,
        rendering_disclosed: input.rendering_disclosed,
        hunk_identity: input.hunk_identity.as_str().to_owned(),
        hunk_is_stable,
        hunk_reidentification_disclosed: input.hunk_reidentification_disclosed,
        export_summary_structured: input.export_summary_structured,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        diff_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a review thread so a user can read what a comment means using one controlled vocabulary,
/// distinguish outdated from resolved without color or provider-specific jargon, and see the same
/// thread-state grammar and anchor-durability behavior across desktop, browser handoff, and exported
/// packets: the thread names its state (non-color, non-jargon), its comment-anchor durability (never
/// silently drifting), and its provider-local-versus-provider-hosted locality (never implicit), never
/// reads a draft / pending-send comment as published, and always offers a command-backed detail
/// entrypoint.
pub fn resolve_review_thread(
    input: M5ReviewThreadResolutionInput,
) -> Result<M5ResolvedReviewThread, M5DiffReviewResolutionError> {
    if input.thread_id.trim().is_empty() {
        return Err(M5DiffReviewResolutionError::EmptyThreadId);
    }
    if string_is_forbidden(&input.thread_id) || string_is_forbidden(&input.comment_label) {
        return Err(M5DiffReviewResolutionError::ForbiddenMaterial);
    }

    let is_outdated = input.thread_state.is_outdated();
    let is_resolved = input.thread_state.is_review_resolved();
    let is_outdated_or_resolved = input.thread_state.is_outdated_or_resolved();
    let needs_send = input.thread_state.needs_send();
    let anchor_durable = anchor_is_durable(input.anchor_durability);
    let anchor_drifted = anchor_is_drifted(input.anchor_durability);
    let provider_is_hosted = input.provider_locality.is_hosted();

    let degrade_reason = if input.comment_label.trim().is_empty() {
        Some(M5ReviewThreadDegradeReason::ThreadIdentityUnstated)
    } else if !input.thread_state.is_known() {
        Some(M5ReviewThreadDegradeReason::ThreadStateUnresolved)
    } else if !input.thread_state_stated {
        Some(M5ReviewThreadDegradeReason::ThreadStateEncodedByColorAlone)
    } else if is_outdated_or_resolved && !input.outdated_resolved_distinguished {
        Some(M5ReviewThreadDegradeReason::OutdatedResolvedBlurred)
    } else if matches!(
        input.anchor_durability,
        M5AnchorDurability::AnchorUnresolved
    ) {
        Some(M5ReviewThreadDegradeReason::AnchorDurabilityUnresolved)
    } else if anchor_drifted && !input.anchor_drift_disclosed {
        Some(M5ReviewThreadDegradeReason::AnchorDriftHidden)
    } else if !input.provider_locality.is_resolved() {
        Some(M5ReviewThreadDegradeReason::ProviderLocalityUnresolved)
    } else if !input.provider_distinction_explicit {
        Some(M5ReviewThreadDegradeReason::ProviderDistinctionImplicit)
    } else if needs_send && !input.pending_send_disclosed {
        Some(M5ReviewThreadDegradeReason::PendingSendHidden)
    } else if !input.detail_command_available {
        Some(M5ReviewThreadDegradeReason::ThreadDetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5ReviewThreadDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DiffReviewNextAction::ReviewThreadState,
    };

    Ok(M5ResolvedReviewThread {
        thread_id: input.thread_id,
        comment_label: input.comment_label,
        thread_state: input.thread_state.as_str().to_owned(),
        thread_state_stated: input.thread_state_stated,
        is_outdated,
        is_resolved,
        is_outdated_or_resolved,
        outdated_resolved_distinguished: input.outdated_resolved_distinguished,
        needs_send,
        pending_send_disclosed: input.pending_send_disclosed,
        anchor_durability: input.anchor_durability.as_str().to_owned(),
        anchor_is_durable: anchor_durable,
        anchor_is_drifted: anchor_drifted,
        anchor_drift_disclosed: input.anchor_drift_disclosed,
        provider_locality: input.provider_locality.as_str().to_owned(),
        provider_is_hosted,
        provider_distinction_explicit: input.provider_distinction_explicit,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        thread_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved diff-view and review-thread examples it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiffReviewControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5DiffReviewConsumerSurface,
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
    pub anatomy_parts: Vec<M5DiffReviewAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5DiffReviewExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EditorInlineDowngradeTrigger>,
    /// Resolved diff-view examples.
    pub diff_examples: Vec<M5ResolvedDiffView>,
    /// Resolved review-thread examples.
    pub thread_examples: Vec<M5ResolvedReviewThread>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a moved or hidden diff context never pretends one immutable view.
    pub diff_moved_or_hidden_context_pretends_immutable_view: bool,
    /// Hard invariant: a diff hunk identity or source rendering never silently drifts.
    pub diff_hunk_identity_or_source_rendering_silently_drifts: bool,
    /// Hard invariant: outdated and resolved review state are never blurred.
    pub review_outdated_and_resolved_state_blurred: bool,
    /// Hard invariant: a review anchor or provider locality never silently drifts.
    pub review_anchor_or_provider_locality_silently_drifts: bool,
}

impl M5DiffReviewControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DiffReviewAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DiffReviewAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DiffReviewExportField> =
            self.export_fields.iter().copied().collect();
        M5DiffReviewExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.diff_moved_or_hidden_context_pretends_immutable_view
            && !self.diff_hunk_identity_or_source_rendering_silently_drifts
            && !self.review_outdated_and_resolved_state_blurred
            && !self.review_anchor_or_provider_locality_silently_drifts
    }

    /// True when every resolved example on this row is honest: no clean diff hides a moved or elided
    /// region, blurs a rendered diff with the exact source, drifts a hunk identity silently, leaves an
    /// opaque summary, or lacks a trace path; and no clean thread encodes its state by color / jargon,
    /// blurs outdated and resolved, drifts an anchor silently, leaves the provider distinction implicit,
    /// reads a draft as published, or lacks a trace path.
    fn examples_are_honest(&self) -> bool {
        self.diff_examples
            .iter()
            .all(|ex| !ex.is_clean() || diff_is_honest(ex))
            && self
                .thread_examples
                .iter()
                .all(|ex| !ex.is_clean() || thread_is_honest(ex))
    }
}

/// True when a clean diff view keeps every guardrail: change kind stated, no hidden moved / elided
/// context, no rendered-versus-source blur, no silent hunk drift, an inspectable summary, and a
/// reachable trace.
fn diff_is_honest(ex: &M5ResolvedDiffView) -> bool {
    ex.change_kind_stated
        && (ex.moved_disclosed || !ex.context_is_moved)
        && (ex.hidden_context_disclosed || !ex.context_is_hidden)
        && (ex.rendering_disclosed || !ex.source_needs_disclosure)
        && (ex.hunk_reidentification_disclosed || ex.hunk_is_stable)
        && ex.export_summary_structured
        && ex.detail_command_available
}

/// True when a clean review thread keeps every guardrail: state stated, no outdated-versus-resolved
/// blur, no silent anchor drift, an explicit provider distinction, no draft-as-published, and a
/// reachable trace.
fn thread_is_honest(ex: &M5ResolvedReviewThread) -> bool {
    ex.thread_state_stated
        && (ex.outdated_resolved_distinguished || !ex.is_outdated_or_resolved)
        && (ex.anchor_drift_disclosed || !ex.anchor_is_drifted)
        && ex.provider_distinction_explicit
        && (ex.pending_send_disclosed || !ex.needs_send)
        && ex.detail_command_available
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiffReviewVocabularySet {
    /// Inline-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Diff-change-kind tokens (bound from the frozen matrix).
    pub diff_change_kinds: Vec<String>,
    /// Anchor-durability tokens (bound from the frozen matrix).
    pub anchor_durabilities: Vec<String>,
    /// Diff context-visibility tokens (minted by this lane).
    pub diff_context_visibilities: Vec<String>,
    /// Diff source-rendering tokens (minted by this lane).
    pub diff_source_renderings: Vec<String>,
    /// Diff hunk-identity tokens (minted by this lane).
    pub diff_hunk_identities: Vec<String>,
    /// Review-thread-state tokens (minted by this lane).
    pub review_thread_states: Vec<String>,
    /// Review provider-locality tokens (minted by this lane).
    pub review_provider_localities: Vec<String>,
    /// Diff-view degrade-reason tokens.
    pub diff_view_degrade_reasons: Vec<String>,
    /// Review-thread degrade-reason tokens.
    pub review_thread_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5DiffReviewVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5EditorInlineDisposition::ALL, |v| v.as_str()),
            diff_change_kinds: tokens(&M5DiffChangeKind::ALL, |v| v.as_str()),
            anchor_durabilities: tokens(&M5AnchorDurability::ALL, |v| v.as_str()),
            diff_context_visibilities: tokens(&M5DiffContextVisibility::ALL, |v| v.as_str()),
            diff_source_renderings: tokens(&M5DiffSourceRendering::ALL, |v| v.as_str()),
            diff_hunk_identities: tokens(&M5DiffHunkIdentity::ALL, |v| v.as_str()),
            review_thread_states: tokens(&M5ReviewThreadState::ALL, |v| v.as_str()),
            review_provider_localities: tokens(&M5ReviewProviderLocality::ALL, |v| v.as_str()),
            diff_view_degrade_reasons: tokens(&M5DiffViewDegradeReason::ALL, |v| v.as_str()),
            review_thread_degrade_reasons: tokens(&M5ReviewThreadDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5DiffReviewAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5DiffReviewNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DiffReviewExportField::ALL, |v| v.as_str()),
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
pub struct M5DiffReviewGovernanceReview {
    /// The diff names its change kind, context visibility, and source-rendering with one vocabulary.
    pub diff_names_change_context_and_rendering: bool,
    /// The diff keeps a stable hunk identity across rebases / re-renders.
    pub diff_keeps_stable_hunk_identity: bool,
    /// Moved and hidden diff context are always disclosed, never pretending one immutable view.
    pub moved_and_hidden_context_always_disclosed: bool,
    /// The diff keeps an inspectable, export-safe structural summary.
    pub diff_keeps_inspectable_structural_summary: bool,
    /// The review thread names its state with one controlled vocabulary.
    pub thread_names_state_with_one_vocabulary: bool,
    /// Outdated and resolved review state are never blurred.
    pub outdated_and_resolved_never_blurred: bool,
    /// Comment anchors never silently drift.
    pub comment_anchors_never_silently_drift: bool,
    /// The provider-local-versus-provider-hosted distinction stays explicit.
    pub provider_locality_stays_explicit: bool,
    /// A draft / pending-send thread never reads as published.
    pub draft_or_pending_never_reads_as_published: bool,
    /// The same thread-state grammar holds across desktop, browser handoff, and exported packets.
    pub thread_grammar_holds_across_surfaces: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiffReviewConsumerProjection {
    /// Editor surfaces consume the shared diff and thread vocabulary.
    pub editor_surfaces_consume_diff_and_thread_vocabulary: bool,
    /// Diff surfaces consume the shared context / hunk vocabulary.
    pub diff_surfaces_consume_context_and_hunk_vocabulary: bool,
    /// Review surfaces consume the shared thread-state and anchor vocabulary.
    pub review_surfaces_consume_thread_state_and_anchor_vocabulary: bool,
    /// Browser handoff and export preserve provider locality and thread truth.
    pub browser_handoff_and_export_preserve_provider_locality: bool,
    /// Diff and review facts trace back to one canonical component contract.
    pub facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical editor-inline source.
    pub support_export_reads_single_editor_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiffReviewProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiffReviewReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DiffReviewControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DiffReviewControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5DiffReviewControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DiffReviewVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DiffReviewGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DiffReviewConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DiffReviewProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DiffReviewReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 diff-view / review-thread controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiffReviewControlsPacket {
    /// Record kind; must equal [`M5_DIFF_REVIEW_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DIFF_REVIEW_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5DiffReviewControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DiffReviewVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DiffReviewGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DiffReviewConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DiffReviewProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DiffReviewReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DiffReviewControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5DiffReviewControlsPacketInput) -> Self {
        Self {
            record_kind: M5_DIFF_REVIEW_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_DIFF_REVIEW_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5DiffReviewControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DIFF_REVIEW_CONTROLS_RECORD_KIND {
            violations.push(M5DiffReviewControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DIFF_REVIEW_CONTROLS_SCHEMA_VERSION {
            violations.push(M5DiffReviewControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DiffReviewControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5DiffReviewControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 diff-view / review-thread controls packet serializes"),
        ) {
            violations.push(M5DiffReviewControlsViolation::RawMaterialInExport);
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
            .expect("m5 diff-view / review-thread controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,diff_examples,thread_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .diff_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.thread_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.diff_examples.len(),
                row.thread_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Diff-View and Review-Thread Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Review thread states: {}\n",
            self.vocabulary_set.review_thread_states.join(", ")
        ));
        out.push_str(&format!(
            "- Diff context visibilities: {}\n",
            self.vocabulary_set.diff_context_visibilities.join(", ")
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
                "  - Diff examples: {} / thread examples: {}\n",
                row.diff_examples.len(),
                row.thread_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5DiffReviewControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DiffReviewControlsViolation>),
}

impl fmt::Display for M5DiffReviewControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 diff-view / review-thread controls export parse failed: {error}"
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
                    "m5 diff-view / review-thread controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DiffReviewControlsArtifactError {}

/// Validation failures emitted by [`M5DiffReviewControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DiffReviewControlsViolation {
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
    /// A controls row carries a dishonest clean example (hidden context, rendered-versus-source blur,
    /// silent hunk / anchor drift, blurred outdated-resolved, draft-as-published, or missing trace).
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
    /// Thread-state grammar / anchor durability is not proven: clean threads do not cover the shared
    /// thread-state and anchor grammar across desktop / handoff / export, or no color-only thread
    /// example degrades.
    ThreadStateGrammarAndAnchorNotProven,
    /// Diff context honesty is not proven: clean diffs do not cover distinct context visibilities, or no
    /// moved-context-hidden / hidden-context example degrades.
    DiffContextHonestyNotProven,
    /// The outdated-versus-resolved distinction is not proven: no clean outdated and resolved example,
    /// no blurred example degrades, or a clean diff and thread do not both offer a detail path.
    OutdatedVersusResolvedDistinctionNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DiffReviewControlsViolation {
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
            Self::ThreadStateGrammarAndAnchorNotProven => {
                "thread_state_grammar_and_anchor_not_proven"
            }
            Self::DiffContextHonestyNotProven => "diff_context_honesty_not_proven",
            Self::OutdatedVersusResolvedDistinctionNotProven => {
                "outdated_versus_resolved_distinction_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_diff_review_controls_export(
) -> Result<M5DiffReviewControlsPacket, M5DiffReviewControlsArtifactError> {
    let packet: M5DiffReviewControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-diff-view-review-thread-controls-proof/support_export.json"
    )))
    .map_err(M5DiffReviewControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DiffReviewControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DiffReviewControlsPacket,
    violations: &mut Vec<M5DiffReviewControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DIFF_REVIEW_CONTROLS_SCHEMA_REF,
        M5_DIFF_REVIEW_CONTROLS_DOC_REF,
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
        M5_DIFF_VIEW_SCHEMA_REF,
        M5_REVIEW_THREAD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DiffReviewControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5DiffReviewControlsPacket,
    violations: &mut Vec<M5DiffReviewControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5DiffReviewControlsViolation::NoControlsRows);
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
            violations.push(M5DiffReviewControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DiffReviewControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DiffReviewControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_DIFF_VIEW_SCHEMA_REF) || !refs.contains(M5_REVIEW_THREAD_SCHEMA_REF) {
            violations.push(M5DiffReviewControlsViolation::ComponentSchemaRefMissing);
        }
        if row.diff_examples.is_empty() || row.thread_examples.is_empty() {
            violations.push(M5DiffReviewControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5DiffReviewControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5DiffReviewControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5DiffReviewControlsPacket,
    violations: &mut Vec<M5DiffReviewControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.diff_names_change_context_and_rendering,
        review.diff_keeps_stable_hunk_identity,
        review.moved_and_hidden_context_always_disclosed,
        review.diff_keeps_inspectable_structural_summary,
        review.thread_names_state_with_one_vocabulary,
        review.outdated_and_resolved_never_blurred,
        review.comment_anchors_never_silently_drift,
        review.provider_locality_stays_explicit,
        review.draft_or_pending_never_reads_as_published,
        review.thread_grammar_holds_across_surfaces,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5DiffReviewControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DiffReviewControlsPacket,
    violations: &mut Vec<M5DiffReviewControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_surfaces_consume_diff_and_thread_vocabulary,
        projection.diff_surfaces_consume_context_and_hunk_vocabulary,
        projection.review_surfaces_consume_thread_state_and_anchor_vocabulary,
        projection.browser_handoff_and_export_preserve_provider_locality,
        projection.facts_trace_to_single_component_contract,
        projection.support_export_reads_single_editor_source,
    ] {
        if !ok {
            violations.push(M5DiffReviewControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DiffReviewControlsPacket,
    violations: &mut Vec<M5DiffReviewControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DiffReviewControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DiffReviewControlsPacket,
    violations: &mut Vec<M5DiffReviewControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DiffReviewControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5DiffReviewControlsPacket,
    violations: &mut Vec<M5DiffReviewControlsViolation>,
) {
    let diffs = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.diff_examples.iter())
    };
    let threads = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.thread_examples.iter())
    };

    // AC1: claimed M5 review flows expose the same thread-state grammar and anchor-durability behavior
    // across desktop, browser handoff, and exported packets. Clean threads cover at least two distinct
    // thread states and two distinct anchor durabilities, span provider-local and provider-hosted
    // localities, a color-only thread-state example degrades, and no clean thread is color-only.
    let clean_thread_states: BTreeSet<String> = threads()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.thread_state.clone())
        .collect();
    let clean_anchor_durabilities: BTreeSet<String> = threads()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.anchor_durability.clone())
        .collect();
    let clean_localities: BTreeSet<String> = threads()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.provider_locality.clone())
        .collect();
    let color_only_thread_degrades = threads().any(|ex| {
        ex.degrade_reason == Some(M5ReviewThreadDegradeReason::ThreadStateEncodedByColorAlone)
    });
    let spans_local_and_hosted =
        clean_localities.contains("provider_local") && clean_localities.contains("provider_hosted");
    let no_clean_color_only = threads().all(|ex| !ex.is_clean() || ex.thread_state_stated);
    if !(clean_thread_states.len() >= 2
        && clean_anchor_durabilities.len() >= 2
        && spans_local_and_hosted
        && color_only_thread_degrades
        && no_clean_color_only)
    {
        violations.push(M5DiffReviewControlsViolation::ThreadStateGrammarAndAnchorNotProven);
    }

    // AC2: diff consumers stay honest when context is moved, elided, collapsed, or re-anchored rather
    // than pretending one immutable view. Clean diffs cover at least two distinct context visibilities,
    // a moved-context-hidden example degrades, a hidden-context example degrades, and no clean diff
    // hides a moved or elided region.
    let clean_context_visibilities: BTreeSet<String> = diffs()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.context_visibility.clone())
        .collect();
    let moved_hidden_degrades =
        diffs().any(|ex| ex.degrade_reason == Some(M5DiffViewDegradeReason::MovedContextHidden));
    let hidden_context_degrades = diffs()
        .any(|ex| ex.degrade_reason == Some(M5DiffViewDegradeReason::HiddenContextNotDisclosed));
    let no_clean_context_hidden = diffs().all(|ex| {
        !ex.is_clean()
            || ((ex.moved_disclosed || !ex.context_is_moved)
                && (ex.hidden_context_disclosed || !ex.context_is_hidden))
    });
    if !(clean_context_visibilities.len() >= 2
        && moved_hidden_degrades
        && hidden_context_degrades
        && no_clean_context_hidden)
    {
        violations.push(M5DiffReviewControlsViolation::DiffContextHonestyNotProven);
    }

    // AC3: users can distinguish outdated from resolved state without relying on color or
    // provider-specific jargon. At least one clean outdated thread and one clean resolved thread exist,
    // an outdated-resolved-blurred example degrades, no clean thread blurs the two, and a clean diff and
    // clean thread both offer a command-backed detail entrypoint.
    let clean_outdated = threads().any(|ex| ex.is_clean() && ex.thread_state == "outdated");
    let clean_resolved = threads().any(|ex| ex.is_clean() && ex.thread_state == "resolved");
    let blurred_degrades = threads()
        .any(|ex| ex.degrade_reason == Some(M5ReviewThreadDegradeReason::OutdatedResolvedBlurred));
    let no_clean_blurred = threads().all(|ex| {
        !ex.is_clean() || ex.outdated_resolved_distinguished || !ex.is_outdated_or_resolved
    });
    let traceable_diff = diffs().any(|ex| ex.is_clean() && ex.detail_command_available);
    let traceable_thread = threads().any(|ex| ex.is_clean() && ex.detail_command_available);
    if !(clean_outdated
        && clean_resolved
        && blurred_degrades
        && no_clean_blurred
        && traceable_diff
        && traceable_thread)
    {
        violations.push(M5DiffReviewControlsViolation::OutdatedVersusResolvedDistinctionNotProven);
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
    M5EditorInlineComponentFamily::DiffView,
    M5EditorInlineComponentFamily::ReviewThread,
];
