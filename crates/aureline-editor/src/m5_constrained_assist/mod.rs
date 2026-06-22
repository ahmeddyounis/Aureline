//! Canonical constrained-file and degraded-provider assist-narrowing model:
//! how every editor assist micro-surface narrows, downgrades, blocks, or routes
//! elsewhere when file state or provider certainty means Aureline cannot safely
//! offer the same completion / hint / hover / refactor behavior it offers on an
//! ordinary source file.
//!
//! Where the [editor-assist matrix](crate::m5_editor_assist) freezes the
//! per-*surface* degraded-state policy, the [completion-row model](crate::m5_completion_rows)
//! freezes the one shared suggestion row, and the
//! [hover/peek model](crate::m5_hover_peek) freezes the contextual-inspection
//! cards, this module freezes the orthogonal axis those three assume: the
//! canonical **constrained-file state classes** and the **degraded-provider**
//! posture, projected once into every assist channel. Before it, each pane
//! decided locally what to do with a generated, protected, read-only, projection,
//! captured-evidence, partially-indexed, restricted, or large file — one pane
//! greyed completion silently, another offered an apply that could never land,
//! a third dropped a side-effectful refactor with no explanation. This module
//! folds all of that into one governed narrowing model that, for every
//! constrained state, resolves per channel:
//!
//! 1. **A degraded-state verdict** — the canonical [`AssistDegradeClass`] the
//!    channel narrows to (full fidelity, source-labeled fallback, read-only,
//!    suppressed, pending, or blocked). The vocabulary is reused, never forked.
//! 2. **An inspectable reason** — a closed [`NarrowReasonClass`] plus a non-empty
//!    disabled-state diagnostic, so *why* an affordance is missing or reduced is
//!    itself product truth, never silently hidden (the guardrail).
//! 3. **A next-safe-action route** — whenever direct assist / apply is narrowed by
//!    a writable-boundary or state-class truth, a closed [`NextSafeActionClass`]
//!    (open generator source, regenerate, duplicate to a writable copy, request
//!    approval, edit the underlying source, wait for the index, open in the full
//!    editor, reconnect the provider, or inspect-only) names the nearest safe
//!    action and the command that reaches it.
//! 4. **Keyboard reachability** — every offered (non-blocked) cell stays
//!    keyboard-reachable, so the narrowed state and its reason are reachable
//!    without a pointer.
//! 5. **Provider provenance** — every profile carries the canonical
//!    [`AssistSourceDescriptor`], so provider identity, support posture, freshness,
//!    and degraded state travel with the narrowing; and a set of
//!    [`DegradedProviderCase`]s proves the same narrowing on an *ordinary* file
//!    when only the provider — not the file — is degraded.
//!
//! Each claimed constrained state resolves into a [`ConstrainedStateProfile`] with
//! exactly one [`AssistNarrowingCell`] per [`AssistChannelClass`]. A set of
//! [`ConsumerSurfaceProof`]s then binds the notebook, generated, request-artifact,
//! docs-code, and protected-config surfaces back to the shared state vocabulary,
//! proving they reuse it rather than inventing local special cases. The build is
//! static and deterministic: [`constrained_assist_model`] assembles the one
//! canonical record, the checked-in fixture plus the replay gate freeze it
//! byte-for-byte, and the model proves its own honesty invariants over its data.
//! It carries no file contents, credential bodies, or raw provider payloads, so
//! support, AI, and migration surfaces can consume it directly.

use serde::{Deserialize, Serialize};

use aureline_language::{
    RouterCompletenessClass, RouterDegradedStateClass, RouterFreshnessClass, RouterLocalityClass,
    RouterScopeClaimClass, RouterSupportClass, ScopeLimitClass,
};

use crate::assist::{AssistSourceDescriptor, AssistSourceFamily, AssistSourceLabelClass};
use crate::m5_editor_assist::{
    AssistChannelClass, AssistDegradeClass, ClassDescriptor, EditorSurfaceClass,
};

/// Schema version for the constrained-assist model record.
pub const M5_CONSTRAINED_ASSIST_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the constrained-assist model record.
pub const M5_CONSTRAINED_ASSIST_SCHEMA_REF: &str =
    "schemas/editor/m5-constrained-assist.schema.json";

/// Stable record-kind tag for the constrained-assist model record.
pub const M5_CONSTRAINED_ASSIST_RECORD_KIND: &str = "m5_constrained_assist_model";

/// Stable id for the canonical constrained-assist model.
pub const M5_CONSTRAINED_ASSIST_MODEL_ID: &str = "m5-constrained-assist:model:0001";

/// Capture stamp for the canonical model. Held as a constant so the projection
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_CONSTRAINED_ASSIST_AS_OF: &str = "2026-06-22T00:00:00Z";

const OPEN_GENERATOR_SOURCE_COMMAND: &str = "command.editor.assist.open_generator_source";
const REGENERATE_FROM_SOURCE_COMMAND: &str = "command.editor.assist.regenerate_from_source";
const DUPLICATE_EDITABLE_COPY_COMMAND: &str = "command.editor.assist.duplicate_editable_copy";
const REQUEST_APPROVAL_REVIEW_COMMAND: &str = "command.editor.assist.request_approval_review";
const EDIT_UNDERLYING_SOURCE_COMMAND: &str = "command.editor.assist.edit_underlying_source";
const SHOW_INDEX_PROGRESS_COMMAND: &str = "command.editor.assist.show_index_progress";
const OPEN_IN_FULL_EDITOR_COMMAND: &str = "command.editor.assist.open_in_full_editor";
const RECONNECT_PROVIDER_COMMAND: &str = "command.editor.assist.reconnect_provider";
const INSPECT_ONLY_COMMAND: &str = "command.editor.assist.inspect_only";

// ---------------------------------------------------------------------------
// Constrained-file state classes.
// ---------------------------------------------------------------------------

/// The constrained-file state classes already landed elsewhere in the product,
/// projected here into assist narrowing. Each variant documents the canonical
/// class it reuses; this module does **not** redefine those classes, it states how
/// each one narrows assist. They are orthogonal to the
/// [`EditorSurfaceClass`](crate::m5_editor_assist::EditorSurfaceClass) rows of the
/// surface matrix: one surface can exhibit one of these states, and several
/// surfaces share the same state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstrainedFileStateClass {
    /// A read-only writable boundary: reading is allowed, direct writes are not.
    /// Projects the canonical writable-boundary / limited-mode read-only posture.
    ReadOnlyBoundary,
    /// A generated artifact whose edits route through its generator. Projects the
    /// canonical generated-artifact state class.
    GeneratedArtifact,
    /// A tool-managed region whose content is owned by a managed-zone regenerator.
    /// Projects the canonical managed-zone state class.
    ManagedRegion,
    /// A read-only projection / virtual view of another source. Projects the
    /// canonical projection state class.
    ProjectionView,
    /// An immutable captured-evidence snapshot (a recorded response, run capture,
    /// or attached artifact). Projects the canonical captured-evidence state class.
    CapturedEvidence,
    /// A file whose semantic index is still building. Projects the canonical
    /// partial-index state class.
    PartialIndex,
    /// A restricted-mode / protected-path file whose writes require staged review.
    /// Projects the canonical restricted-mode state class.
    RestrictedMode,
    /// A file open in large-file / restricted-load mode where semantic assist is
    /// suppressed for safety. Projects the canonical large-file posture state class.
    LargeFile,
}

impl ConstrainedFileStateClass {
    /// All constrained-file state classes, in catalog order.
    pub const ALL: [Self; 8] = [
        Self::ReadOnlyBoundary,
        Self::GeneratedArtifact,
        Self::ManagedRegion,
        Self::ProjectionView,
        Self::CapturedEvidence,
        Self::PartialIndex,
        Self::RestrictedMode,
        Self::LargeFile,
    ];

    /// Returns the stable schema token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyBoundary => "read_only_boundary",
            Self::GeneratedArtifact => "generated_artifact",
            Self::ManagedRegion => "managed_region",
            Self::ProjectionView => "projection_view",
            Self::CapturedEvidence => "captured_evidence",
            Self::PartialIndex => "partial_index",
            Self::RestrictedMode => "restricted_mode",
            Self::LargeFile => "large_file",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReadOnlyBoundary => "Read-only boundary",
            Self::GeneratedArtifact => "Generated artifact",
            Self::ManagedRegion => "Managed region",
            Self::ProjectionView => "Projection view",
            Self::CapturedEvidence => "Captured evidence",
            Self::PartialIndex => "Partial-index state",
            Self::RestrictedMode => "Restricted mode",
            Self::LargeFile => "Large-file mode",
        }
    }

    /// Note naming the canonical landed state class this variant projects, so a
    /// reader can trace it back to its source of truth.
    pub const fn canonical_class_note(self) -> &'static str {
        match self {
            Self::ReadOnlyBoundary => {
                "Reuses the canonical writable-boundary read-only posture; reading is allowed, \
                 direct writes are not."
            }
            Self::GeneratedArtifact => {
                "Reuses the canonical generated-artifact state class; edits route through the \
                 artifact's generator."
            }
            Self::ManagedRegion => {
                "Reuses the canonical managed-zone state class; the region is owned by a \
                 regenerator and re-emitted, not hand-edited."
            }
            Self::ProjectionView => {
                "Reuses the canonical projection state class; the view mirrors another source and \
                 edits belong to that source."
            }
            Self::CapturedEvidence => {
                "Reuses the canonical captured-evidence state class; the content is an immutable \
                 snapshot and is inspect-only."
            }
            Self::PartialIndex => {
                "Reuses the canonical partial-index state class; semantic answers are partial and \
                 labeled while the index builds."
            }
            Self::RestrictedMode => {
                "Reuses the canonical restricted-mode / protected-path state class; writes require \
                 staged review."
            }
            Self::LargeFile => {
                "Reuses the canonical large-file posture state class; semantic assist is suppressed \
                 for safety on the restricted load."
            }
        }
    }

    /// Whether a direct in-buffer apply is blocked on this state because writes
    /// route elsewhere, require approval, or are impossible.
    pub const fn blocks_direct_apply(self) -> bool {
        !matches!(self, Self::PartialIndex)
    }

    /// Whether semantic assist is suppressed for safety (large-file restricted
    /// load) rather than merely narrowed.
    pub const fn suppresses_semantic(self) -> bool {
        matches!(self, Self::LargeFile)
    }

    /// Whether the state is a still-building index rather than a writable-boundary
    /// constraint.
    pub const fn is_index_pending(self) -> bool {
        matches!(self, Self::PartialIndex)
    }

    /// The primary next-safe-action route offered when assist / apply is narrowed
    /// on this state.
    pub const fn primary_next_safe_action(self) -> NextSafeActionClass {
        match self {
            Self::ReadOnlyBoundary => NextSafeActionClass::DuplicateEditableCopy,
            Self::GeneratedArtifact => NextSafeActionClass::OpenGeneratorSource,
            Self::ManagedRegion => NextSafeActionClass::RegenerateFromSource,
            Self::ProjectionView => NextSafeActionClass::EditUnderlyingSource,
            Self::CapturedEvidence => NextSafeActionClass::ViewOnlyNoAction,
            Self::PartialIndex => NextSafeActionClass::WaitForIndex,
            Self::RestrictedMode => NextSafeActionClass::RequestApprovalReview,
            Self::LargeFile => NextSafeActionClass::OpenInFullEditor,
        }
    }
}

// ---------------------------------------------------------------------------
// Narrowing reason.
// ---------------------------------------------------------------------------

/// The closed vocabulary of reasons an assist channel is narrowed, blocked, or
/// has its apply routed elsewhere. The reason is product truth: it must always be
/// inspectable, never silently hidden behind a greyed control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowReasonClass {
    /// Writes route through the artifact's generator or managed-zone regenerator.
    WriteRoutesThroughGenerator,
    /// Edits belong to the underlying source the projection mirrors.
    ProjectionEditsRouteToSource,
    /// Writes require staged review on a protected / restricted path.
    WriteRequiresApproval,
    /// The file is a read-only writable boundary; reads are fine, writes are not.
    WriteBoundaryReadOnly,
    /// The content is an immutable captured-evidence snapshot.
    SnapshotImmutable,
    /// The semantic index is still building, so results are partial and labeled.
    IndexStillBuilding,
    /// Assist is suppressed for safety in large-file / restricted-load mode.
    SuppressedForSafety,
    /// The provider is degraded, so a labeled fallback is shown instead of a
    /// full-fidelity result.
    ProviderDegradedFallback,
}

impl NarrowReasonClass {
    /// All narrowing reasons, in catalog order.
    pub const ALL: [Self; 8] = [
        Self::WriteRoutesThroughGenerator,
        Self::ProjectionEditsRouteToSource,
        Self::WriteRequiresApproval,
        Self::WriteBoundaryReadOnly,
        Self::SnapshotImmutable,
        Self::IndexStillBuilding,
        Self::SuppressedForSafety,
        Self::ProviderDegradedFallback,
    ];

    /// Returns the stable schema token for this reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WriteRoutesThroughGenerator => "write_routes_through_generator",
            Self::ProjectionEditsRouteToSource => "projection_edits_route_to_source",
            Self::WriteRequiresApproval => "write_requires_approval",
            Self::WriteBoundaryReadOnly => "write_boundary_read_only",
            Self::SnapshotImmutable => "snapshot_immutable",
            Self::IndexStillBuilding => "index_still_building",
            Self::SuppressedForSafety => "suppressed_for_safety",
            Self::ProviderDegradedFallback => "provider_degraded_fallback",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WriteRoutesThroughGenerator => "Writes route through the generator",
            Self::ProjectionEditsRouteToSource => "Edits route to the underlying source",
            Self::WriteRequiresApproval => "Writes require approval",
            Self::WriteBoundaryReadOnly => "Read-only boundary",
            Self::SnapshotImmutable => "Immutable captured snapshot",
            Self::IndexStillBuilding => "Index still building",
            Self::SuppressedForSafety => "Suppressed for safety",
            Self::ProviderDegradedFallback => "Provider degraded — labeled fallback",
        }
    }
}

// ---------------------------------------------------------------------------
// Next-safe-action routes.
// ---------------------------------------------------------------------------

/// The closed vocabulary of next-safe-action routes offered when direct assist or
/// apply is narrowed. Each route names the nearest safe thing the user can do and
/// the command that reaches it, so a blocked affordance always points somewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NextSafeActionClass {
    /// Open the source that generates this artifact and edit there.
    OpenGeneratorSource,
    /// Regenerate the artifact / managed region from its source.
    RegenerateFromSource,
    /// Duplicate the file into a writable copy to edit freely.
    DuplicateEditableCopy,
    /// Request approval / staged review for a protected write.
    RequestApprovalReview,
    /// Edit the underlying source the projection mirrors.
    EditUnderlyingSource,
    /// Wait while the semantic index finishes, with visible progress.
    WaitForIndex,
    /// Open the file in the full editor once it can be loaded safely.
    OpenInFullEditor,
    /// Reconnect or restart the degraded provider to restore full fidelity.
    ReconnectProvider,
    /// Inspect-only: there is no safe edit route; reading and copying remain.
    ViewOnlyNoAction,
}

impl NextSafeActionClass {
    /// All next-safe-action routes, in catalog order.
    pub const ALL: [Self; 9] = [
        Self::OpenGeneratorSource,
        Self::RegenerateFromSource,
        Self::DuplicateEditableCopy,
        Self::RequestApprovalReview,
        Self::EditUnderlyingSource,
        Self::WaitForIndex,
        Self::OpenInFullEditor,
        Self::ReconnectProvider,
        Self::ViewOnlyNoAction,
    ];

    /// Returns the stable schema token for this route.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenGeneratorSource => "open_generator_source",
            Self::RegenerateFromSource => "regenerate_from_source",
            Self::DuplicateEditableCopy => "duplicate_editable_copy",
            Self::RequestApprovalReview => "request_approval_review",
            Self::EditUnderlyingSource => "edit_underlying_source",
            Self::WaitForIndex => "wait_for_index",
            Self::OpenInFullEditor => "open_in_full_editor",
            Self::ReconnectProvider => "reconnect_provider",
            Self::ViewOnlyNoAction => "view_only_no_action",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenGeneratorSource => "Open generator source",
            Self::RegenerateFromSource => "Regenerate from source",
            Self::DuplicateEditableCopy => "Duplicate to an editable copy",
            Self::RequestApprovalReview => "Request approval",
            Self::EditUnderlyingSource => "Edit the underlying source",
            Self::WaitForIndex => "Wait for the index",
            Self::OpenInFullEditor => "Open in the full editor",
            Self::ReconnectProvider => "Reconnect the provider",
            Self::ViewOnlyNoAction => "Inspect only",
        }
    }

    /// Canonical command id for this route.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::OpenGeneratorSource => OPEN_GENERATOR_SOURCE_COMMAND,
            Self::RegenerateFromSource => REGENERATE_FROM_SOURCE_COMMAND,
            Self::DuplicateEditableCopy => DUPLICATE_EDITABLE_COPY_COMMAND,
            Self::RequestApprovalReview => REQUEST_APPROVAL_REVIEW_COMMAND,
            Self::EditUnderlyingSource => EDIT_UNDERLYING_SOURCE_COMMAND,
            Self::WaitForIndex => SHOW_INDEX_PROGRESS_COMMAND,
            Self::OpenInFullEditor => OPEN_IN_FULL_EDITOR_COMMAND,
            Self::ReconnectProvider => RECONNECT_PROVIDER_COMMAND,
            Self::ViewOnlyNoAction => INSPECT_ONLY_COMMAND,
        }
    }

    /// Whether this route opens an onward path to a writable edit, as opposed to
    /// the inspect-only terminal route, which is the honest answer that no safe
    /// mutation exists.
    pub const fn offers_edit_path(self) -> bool {
        !matches!(self, Self::ViewOnlyNoAction)
    }
}

// ---------------------------------------------------------------------------
// Narrowing cell.
// ---------------------------------------------------------------------------

/// How a single assist channel narrows on a single constrained-file state. One of
/// these is resolved for every [`AssistChannelClass`] on every state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistNarrowingCell {
    /// Assist channel this cell describes.
    pub channel: AssistChannelClass,
    /// Whether the channel is offered at all on this state. When false the channel
    /// is fully unavailable and the cell explains why.
    pub applicable: bool,
    /// Degraded-state verdict the channel narrows to.
    pub degrade_class: AssistDegradeClass,
    /// Whether a direct in-buffer apply is blocked for this channel here.
    pub apply_blocked: bool,
    /// Closed narrowing reason, when the channel is not at full fidelity.
    pub narrow_reason: Option<NarrowReasonClass>,
    /// Next-safe-action route offered when the channel is narrowed or apply is
    /// blocked.
    pub next_safe_action: Option<NextSafeActionClass>,
    /// Command id that reaches the next-safe-action route.
    pub next_safe_action_command_ref: Option<String>,
    /// Whether the channel stays keyboard-reachable when offered.
    pub keyboard_reachable: bool,
    /// The inspectable "why" copy shown as a tooltip, disabled-state diagnostic,
    /// and support-export line. Always present when the channel is narrowed.
    pub disabled_state_diagnostic: String,
}

impl AssistNarrowingCell {
    /// Returns true when the channel is narrowed below full fidelity.
    pub fn is_narrowed(&self) -> bool {
        self.degrade_class != AssistDegradeClass::FullFidelity
    }

    /// Returns true when the narrowing reason is inspectable: a narrowed channel
    /// always carries a non-empty disabled-state diagnostic, so the reason is never
    /// silently hidden.
    pub fn reason_inspectable(&self) -> bool {
        !self.is_narrowed() || !self.disabled_state_diagnostic.trim().is_empty()
    }

    /// Returns true when a blocked apply offers a concrete next-safe-action route.
    pub fn apply_block_offers_route(&self) -> bool {
        if !self.apply_blocked {
            return true;
        }
        self.next_safe_action.is_some()
            && self
                .next_safe_action_command_ref
                .as_ref()
                .is_some_and(|command| !command.trim().is_empty())
    }

    /// Returns true when a side-effectful (apply-capable) channel that is blocked
    /// or unavailable is never silently hidden: it both marks apply blocked and
    /// discloses why.
    pub fn no_silent_hidden_side_effect(&self) -> bool {
        if !self.channel.is_apply_capable() {
            return true;
        }
        if self.applicable && !self.apply_blocked {
            return true;
        }
        self.apply_blocked && !self.disabled_state_diagnostic.trim().is_empty()
    }

    /// Returns true when an offered cell stays keyboard-reachable.
    pub fn offered_cell_reachable(&self) -> bool {
        !self.degrade_class.is_offered() || self.keyboard_reachable
    }
}

// ---------------------------------------------------------------------------
// Resolution.
// ---------------------------------------------------------------------------

/// Resolves the narrowing cell for one channel on one constrained-file state. The
/// rules are deterministic and documented per state so the resolved matrix can be
/// audited against the honesty invariants the model proves over it.
fn resolve_cell(
    state: ConstrainedFileStateClass,
    channel: AssistChannelClass,
) -> AssistNarrowingCell {
    use AssistChannelClass as Ch;
    use AssistDegradeClass as Deg;
    use ConstrainedFileStateClass as St;

    // Decoration is editing-truth (diagnostics / conflict / review). It stays at
    // full fidelity everywhere except large-file mode, where the file is not fully
    // parsed and it narrows to a labeled lexical fallback rather than being dropped.
    if channel == Ch::Decoration {
        return match state {
            St::LargeFile => narrowed(
                channel,
                Deg::SourceLabeledFallback,
                false,
                NarrowReasonClass::SuppressedForSafety,
                NextSafeActionClass::OpenInFullEditor,
                "Editing-truth decorations are limited to lexical highlighting in large-file mode; \
                 open the file in the full editor for full diagnostics.",
            ),
            _ => full_fidelity(channel),
        };
    }

    let apply_capable = channel.is_apply_capable();
    let semantic = channel.is_semantic();
    // Code lenses are not apply-capable as a channel, but actionable lenses can
    // trigger edits, so on any write-blocking state they narrow to read-only with a
    // route, just like the apply-capable channels.
    let edit_bearing = apply_capable || channel == Ch::CodeLens;

    match state {
        St::LargeFile => {
            // Everything semantic or edit-bearing is suppressed; the rest is too,
            // because the file is not parsed. Cells stay reachable and disclose.
            narrowed(
                channel,
                Deg::SuppressedLargeFile,
                apply_capable,
                NarrowReasonClass::SuppressedForSafety,
                NextSafeActionClass::OpenInFullEditor,
                "Suppressed in large-file mode to keep the file responsive; open it in the full \
                 editor to restore assist.",
            )
        }
        St::PartialIndex => {
            if semantic {
                // Completion, signature, hover, peek, lens, inlay all narrow to a
                // labeled pending state while the index builds. Apply is not blocked
                // — a lexical fallback can still be accepted — but the semantic part
                // is partial and labeled.
                narrowed(
                    channel,
                    Deg::PendingPartialIndex,
                    false,
                    NarrowReasonClass::IndexStillBuilding,
                    NextSafeActionClass::WaitForIndex,
                    "Semantic results are partial while the index builds; a labeled fallback is \
                     shown and full results arrive when indexing completes.",
                )
            } else {
                // Snippet sessions and inline AI assist are local and stay at full
                // fidelity on a partially-indexed file.
                full_fidelity(channel)
            }
        }
        St::ReadOnlyBoundary => read_blocking_state(
            channel,
            edit_bearing,
            NarrowReasonClass::WriteBoundaryReadOnly,
            NextSafeActionClass::DuplicateEditableCopy,
            "This file is read-only; suggestions are shown for reading but cannot be applied. \
             Duplicate it to an editable copy to apply changes.",
        ),
        St::GeneratedArtifact => read_blocking_state(
            channel,
            edit_bearing,
            NarrowReasonClass::WriteRoutesThroughGenerator,
            NextSafeActionClass::OpenGeneratorSource,
            "This file is generated; edits would be overwritten. Open the generator source to make \
             changes that regenerate it.",
        ),
        St::ManagedRegion => read_blocking_state(
            channel,
            edit_bearing,
            NarrowReasonClass::WriteRoutesThroughGenerator,
            NextSafeActionClass::RegenerateFromSource,
            "This region is tool-managed; hand edits are re-emitted. Regenerate it from source to \
             change it.",
        ),
        St::ProjectionView => read_blocking_state(
            channel,
            edit_bearing,
            NarrowReasonClass::ProjectionEditsRouteToSource,
            NextSafeActionClass::EditUnderlyingSource,
            "This is a projection of another source; edits belong to that source. Open the \
             underlying source to change it.",
        ),
        St::RestrictedMode => read_blocking_state(
            channel,
            edit_bearing,
            NarrowReasonClass::WriteRequiresApproval,
            NextSafeActionClass::RequestApprovalReview,
            "This path is protected; writes require staged review. Request approval to apply \
             changes.",
        ),
        St::CapturedEvidence => {
            if apply_capable {
                // Captured evidence is immutable: editing channels are fully
                // unavailable, but reading and copying remain.
                AssistNarrowingCell {
                    channel,
                    applicable: false,
                    degrade_class: Deg::BlockedUnavailable,
                    apply_blocked: true,
                    narrow_reason: Some(NarrowReasonClass::SnapshotImmutable),
                    next_safe_action: Some(NextSafeActionClass::ViewOnlyNoAction),
                    next_safe_action_command_ref: Some(
                        NextSafeActionClass::ViewOnlyNoAction.command_id().to_owned(),
                    ),
                    keyboard_reachable: false,
                    disabled_state_diagnostic: "This is an immutable captured snapshot; editing \
                                                assist is not offered. You can inspect and copy its \
                                                contents."
                        .to_owned(),
                }
            } else if channel == Ch::CodeLens {
                // Actionable lenses on a snapshot are inspect-only.
                narrowed(
                    channel,
                    Deg::ReadOnlyNoApply,
                    true,
                    NarrowReasonClass::SnapshotImmutable,
                    NextSafeActionClass::ViewOnlyNoAction,
                    "This is an immutable captured snapshot; lens actions are inspect-only.",
                )
            } else {
                // Hover, peek, signature, inlay all read the snapshot fine.
                full_fidelity(channel)
            }
        }
    }
}

/// Builds a cell for a write-blocking state: edit-bearing channels narrow to
/// read-only with a route, reading channels stay at full fidelity.
fn read_blocking_state(
    channel: AssistChannelClass,
    edit_bearing: bool,
    reason: NarrowReasonClass,
    action: NextSafeActionClass,
    diagnostic: &str,
) -> AssistNarrowingCell {
    if edit_bearing {
        narrowed(
            channel,
            AssistDegradeClass::ReadOnlyNoApply,
            true,
            reason,
            action,
            diagnostic,
        )
    } else {
        full_fidelity(channel)
    }
}

/// Builds a full-fidelity cell: nothing narrowed, nothing blocked.
fn full_fidelity(channel: AssistChannelClass) -> AssistNarrowingCell {
    AssistNarrowingCell {
        channel,
        applicable: true,
        degrade_class: AssistDegradeClass::FullFidelity,
        apply_blocked: false,
        narrow_reason: None,
        next_safe_action: None,
        next_safe_action_command_ref: None,
        keyboard_reachable: true,
        disabled_state_diagnostic: String::new(),
    }
}

/// Builds a narrowed cell with a reason, route, and inspectable diagnostic.
fn narrowed(
    channel: AssistChannelClass,
    degrade_class: AssistDegradeClass,
    apply_blocked: bool,
    reason: NarrowReasonClass,
    action: NextSafeActionClass,
    diagnostic: &str,
) -> AssistNarrowingCell {
    AssistNarrowingCell {
        channel,
        applicable: degrade_class.is_offered(),
        degrade_class,
        apply_blocked,
        narrow_reason: Some(reason),
        next_safe_action: Some(action),
        next_safe_action_command_ref: Some(action.command_id().to_owned()),
        keyboard_reachable: degrade_class.is_offered(),
        disabled_state_diagnostic: diagnostic.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Constrained-state profile.
// ---------------------------------------------------------------------------

/// One constrained-file state resolved into its per-channel narrowing, its primary
/// next-safe-action route, and the provider posture that travels with it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstrainedStateProfile {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Constrained-file state this profile describes.
    pub state_class: ConstrainedFileStateClass,
    /// Human-readable state label.
    pub label: String,
    /// Note naming the canonical landed class this state projects.
    pub canonical_class_note: String,
    /// Whether direct in-buffer apply is blocked on this state.
    pub blocks_direct_apply: bool,
    /// Primary next-safe-action route offered when assist / apply is narrowed.
    pub primary_next_safe_action: NextSafeActionClass,
    /// Command id for the primary next-safe-action route.
    pub primary_next_safe_action_command_ref: String,
    /// Provider / provenance descriptor whose posture travels with the narrowing.
    pub provider_posture: AssistSourceDescriptor,
    /// Exactly one cell per [`AssistChannelClass`], in channel order.
    pub cells: Vec<AssistNarrowingCell>,
    /// Export-safe summary of why and how this state narrows assist.
    pub blocked_reason_summary: String,
    /// Accessible summary for screen readers.
    pub accessibility_summary: String,
    /// Export-safe one-line summary.
    pub export_safe_summary: String,
}

impl ConstrainedStateProfile {
    /// Stable record-kind tag for constrained-state profiles.
    pub const RECORD_KIND: &'static str = "m5_constrained_state_profile";

    /// Returns the cell for the given channel, when present.
    pub fn cell(&self, channel: AssistChannelClass) -> Option<&AssistNarrowingCell> {
        self.cells.iter().find(|cell| cell.channel == channel)
    }

    /// Returns true when the provider posture for this state is degraded.
    pub fn provider_is_degraded(&self) -> bool {
        self.provider_posture.degraded_state_class != RouterDegradedStateClass::None
    }

    /// Returns true when at least one channel is narrowed below full fidelity.
    pub fn narrows_at_least_one_channel(&self) -> bool {
        self.cells.iter().any(|cell| cell.is_narrowed())
    }
}

// ---------------------------------------------------------------------------
// Degraded-provider case.
// ---------------------------------------------------------------------------

/// One case where an *ordinary* file narrows assist purely because the provider —
/// not the file — is degraded. This proves the degraded-provider half of the
/// model: the same source-labeled, inspectable, routed narrowing applies when the
/// file is fully writable but the provider cannot answer with full certainty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedProviderCase {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable case id.
    pub case_id: String,
    /// Provider degraded-state class (reuses the language router vocabulary).
    pub provider_state: RouterDegradedStateClass,
    /// Provider / provenance descriptor for the degraded result.
    pub source: AssistSourceDescriptor,
    /// Assist channel this case narrows.
    pub channel: AssistChannelClass,
    /// Degraded-state verdict the channel narrows to.
    pub degrade_class: AssistDegradeClass,
    /// Next-safe-action route to restore full fidelity.
    pub next_safe_action: NextSafeActionClass,
    /// Command id for the next-safe-action route.
    pub next_safe_action_command_ref: String,
    /// Whether the result is source-labeled rather than silently styled as
    /// full-fidelity.
    pub source_labeled_not_silent: bool,
    /// The inspectable "why" copy for the degraded provider.
    pub disabled_state_diagnostic: String,
    /// Export-safe note.
    pub note: String,
}

impl DegradedProviderCase {
    /// Stable record-kind tag for degraded-provider cases.
    pub const RECORD_KIND: &'static str = "m5_degraded_provider_case";

    /// Returns true when the case narrows below full fidelity, is source-labeled,
    /// offers a route, and discloses its reason.
    pub fn is_honest(&self) -> bool {
        self.degrade_class != AssistDegradeClass::FullFidelity
            && self.source_labeled_not_silent
            && !self.disabled_state_diagnostic.trim().is_empty()
            && !self.next_safe_action_command_ref.trim().is_empty()
    }
}

// ---------------------------------------------------------------------------
// Consumer surface proof.
// ---------------------------------------------------------------------------

/// One claimed editor surface bound back to the shared constrained-state
/// vocabulary, proving it reuses the model rather than inventing a local special
/// case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerSurfaceProof {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable consumer id.
    pub consumer_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// The canonical editor surface this consumer reuses, when applicable.
    pub base_editor_surface: Option<EditorSurfaceClass>,
    /// The constrained-file state this surface exhibits.
    pub exhibited_state: ConstrainedFileStateClass,
    /// A representative channel whose narrowing is asserted for this surface.
    pub representative_channel: AssistChannelClass,
    /// The degraded-state verdict the representative channel resolves to.
    pub resolved_degrade: AssistDegradeClass,
    /// The next-safe-action route the representative channel offers, if any.
    pub next_safe_action: Option<NextSafeActionClass>,
    /// Whether this surface reuses the shared vocabulary instead of a local case.
    pub reuses_shared_vocabulary: bool,
    /// Export-safe note.
    pub note: String,
}

impl ConsumerSurfaceProof {
    /// Stable record-kind tag for consumer surface proofs.
    pub const RECORD_KIND: &'static str = "m5_consumer_surface_proof";
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

/// One frozen honesty invariant the model must satisfy, with the result of
/// evaluating it over the model's own data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstrainedAssistInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// Human-readable statement of the invariant.
    pub statement: String,
    /// Whether the invariant holds on the built model.
    pub holds: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// The canonical, frozen, export-safe constrained-file and degraded-provider
/// assist-narrowing model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstrainedAssistModel {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_constrained_assist_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable model id.
    pub model_id: String,
    /// Capture stamp.
    pub as_of: String,
    /// Constrained-state catalog.
    pub state_classes: Vec<ClassDescriptor>,
    /// Assist-channel catalog (reuses the shared channel vocabulary).
    pub channel_classes: Vec<ClassDescriptor>,
    /// Degraded-state catalog (reuses the shared degrade vocabulary).
    pub degrade_classes: Vec<ClassDescriptor>,
    /// Narrowing-reason catalog.
    pub reason_classes: Vec<ClassDescriptor>,
    /// Next-safe-action catalog.
    pub next_safe_action_classes: Vec<ClassDescriptor>,
    /// One profile per constrained-file state.
    pub state_profiles: Vec<ConstrainedStateProfile>,
    /// Degraded-provider cases on otherwise-ordinary files.
    pub degraded_provider_cases: Vec<DegradedProviderCase>,
    /// Consumer surfaces bound to the shared vocabulary.
    pub consumer_proofs: Vec<ConsumerSurfaceProof>,
    /// Frozen invariants and whether each holds on this model.
    pub invariants: Vec<ConstrainedAssistInvariant>,
    /// Whether the model is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl ConstrainedAssistModel {
    /// Returns true when every frozen invariant holds on this model.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the model is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == M5_CONSTRAINED_ASSIST_SCHEMA_REF
            && self.record_kind == M5_CONSTRAINED_ASSIST_RECORD_KIND
    }

    /// Returns the profile for the given state, when present.
    pub fn profile(&self, state: ConstrainedFileStateClass) -> Option<&ConstrainedStateProfile> {
        self.state_profiles
            .iter()
            .find(|profile| profile.state_class == state)
    }

    /// Returns every narrowing cell across every profile.
    pub fn all_cells(&self) -> impl Iterator<Item = &AssistNarrowingCell> {
        self.state_profiles
            .iter()
            .flat_map(|profile| profile.cells.iter())
    }
}

// ---------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn provider_posture(
    state: ConstrainedFileStateClass,
    family: AssistSourceFamily,
    provider_id: Option<&str>,
    provider_label: &str,
    support: RouterSupportClass,
    freshness: RouterFreshnessClass,
    scope: RouterScopeClaimClass,
    completeness: RouterCompletenessClass,
    locality: RouterLocalityClass,
    degraded: RouterDegradedStateClass,
    scope_limits: Vec<ScopeLimitClass>,
    summary: &str,
) -> AssistSourceDescriptor {
    AssistSourceDescriptor {
        source_descriptor_id: format!(
            "constrained-assist-source:{}:{}",
            state.as_str(),
            family.as_str()
        ),
        source_family: family,
        source_label_class: AssistSourceLabelClass::from_source_family(family),
        source_label: provider_label.to_owned(),
        provider_id: provider_id.map(str::to_owned),
        router_decision_ref: provider_id
            .map(|id| format!("router-decision:{}:{id}", state.as_str())),
        source_ref: None,
        support_class: support,
        freshness_class: freshness,
        scope_claim_class: scope,
        completeness_class: completeness,
        scope_limit_classes: scope_limits,
        locality_class: locality,
        degraded_state_class: degraded,
        summary: summary.to_owned(),
    }
}

fn class_descriptor(token: &str, label: &str, note: &str) -> ClassDescriptor {
    ClassDescriptor {
        class_token: token.to_owned(),
        label: label.to_owned(),
        note: note.to_owned(),
    }
}

/// Stable snake_case token for a router degraded-state class, matching its serde
/// representation, for the human-readable projection.
const fn degraded_state_token(class: RouterDegradedStateClass) -> &'static str {
    match class {
        RouterDegradedStateClass::None => "none",
        RouterDegradedStateClass::DegradedProviderUnavailable => "degraded_provider_unavailable",
        RouterDegradedStateClass::DegradedCrashLoopQuarantine => "degraded_crash_loop_quarantine",
        RouterDegradedStateClass::DegradedCachedFallback => "degraded_cached_fallback",
        RouterDegradedStateClass::DegradedHeuristicFallback => "degraded_heuristic_fallback",
        RouterDegradedStateClass::DegradedScopeNarrowed => "degraded_scope_narrowed",
        RouterDegradedStateClass::DegradedRemoteUnreachable => "degraded_remote_unreachable",
        RouterDegradedStateClass::DegradedCoordinateMappingMissing => {
            "degraded_coordinate_mapping_missing"
        }
        RouterDegradedStateClass::DegradedPolicyNarrowed => "degraded_policy_narrowed",
    }
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds the one canonical constrained-file and degraded-provider
/// assist-narrowing model.
///
/// The build is deterministic and self-contained: it materializes one
/// [`ConstrainedStateProfile`] per claimed constrained-file state (each resolving
/// one [`AssistNarrowingCell`] per assist channel and carrying its provider
/// posture), a set of [`DegradedProviderCase`]s for the provider axis, and a set
/// of [`ConsumerSurfaceProof`]s binding the claimed surfaces back to the shared
/// vocabulary, then evaluates every frozen honesty invariant over the assembled
/// data so the record's `invariants[].holds` reflect real checks.
pub fn constrained_assist_model() -> ConstrainedAssistModel {
    let state_profiles = build_state_profiles();
    let degraded_provider_cases = build_degraded_provider_cases();
    let consumer_proofs = build_consumer_proofs();
    let invariants =
        evaluate_invariants(&state_profiles, &degraded_provider_cases, &consumer_proofs);

    let qualified = invariants.iter().all(|invariant| invariant.holds);
    let summary = if qualified {
        format!(
            "Constrained-assist model frozen: {states} constrained-file states each resolve a \
             per-channel narrowing over {channels} assist channels, plus {provider} \
             degraded-provider cases and {consumers} consumer-surface proofs. Every narrowed \
             channel discloses an inspectable reason and stays keyboard-reachable; every blocked \
             apply offers a next-safe-action route (open generator source, regenerate, duplicate, \
             request approval, edit source, wait for index, open in full editor, reconnect \
             provider, or inspect-only); editing-truth decorations stay full fidelity except in \
             large-file mode; and notebook, generated, request-artifact, docs-code, and protected \
             surfaces reuse the shared vocabulary. All {invariants} invariants hold.",
            states = state_profiles.len(),
            channels = AssistChannelClass::ALL.len(),
            provider = degraded_provider_cases.len(),
            consumers = consumer_proofs.len(),
            invariants = invariants.len(),
        )
    } else {
        format!(
            "Constrained-assist model INVALID: {failing} of {total} invariants do not hold.",
            failing = invariants.iter().filter(|i| !i.holds).count(),
            total = invariants.len(),
        )
    };

    ConstrainedAssistModel {
        record_kind: M5_CONSTRAINED_ASSIST_RECORD_KIND.to_owned(),
        m5_constrained_assist_schema_version: M5_CONSTRAINED_ASSIST_SCHEMA_VERSION,
        schema_ref: M5_CONSTRAINED_ASSIST_SCHEMA_REF.to_owned(),
        model_id: M5_CONSTRAINED_ASSIST_MODEL_ID.to_owned(),
        as_of: M5_CONSTRAINED_ASSIST_AS_OF.to_owned(),
        state_classes: build_state_catalog(),
        channel_classes: build_channel_catalog(),
        degrade_classes: build_degrade_catalog(),
        reason_classes: build_reason_catalog(),
        next_safe_action_classes: build_next_safe_action_catalog(),
        state_profiles,
        degraded_provider_cases,
        consumer_proofs,
        invariants,
        raw_payload_excluded: true,
        summary,
    }
}

/// Builds the human-readable projection of the model for support and headless use.
pub fn constrained_assist_model_lines(model: &ConstrainedAssistModel) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Constrained-assist model — {} ({})",
        model.model_id, model.as_of
    ));
    lines.push(format!(
        "schema_ref={} version={}",
        model.schema_ref, model.m5_constrained_assist_schema_version
    ));

    lines.push("State profiles:".to_owned());
    for profile in &model.state_profiles {
        lines.push(format!(
            "  {state}: primary_action={action} apply_blocked={blocked} provider={provider}",
            state = profile.state_class.as_str(),
            action = profile.primary_next_safe_action.as_str(),
            blocked = profile.blocks_direct_apply,
            provider = profile.provider_posture.source_label,
        ));
        for cell in &profile.cells {
            lines.push(format!(
                "    {channel}: degrade={degrade} apply_blocked={apply} reason={reason} \
                 next={next}",
                channel = cell.channel.as_str(),
                degrade = cell.degrade_class.as_str(),
                apply = cell.apply_blocked,
                reason = cell.narrow_reason.map(|r| r.as_str()).unwrap_or("none"),
                next = cell.next_safe_action.map(|a| a.as_str()).unwrap_or("none"),
            ));
        }
    }

    lines.push("Degraded-provider cases:".to_owned());
    for case in &model.degraded_provider_cases {
        lines.push(format!(
            "  {id}: provider_state={state} channel={channel} degrade={degrade} next={next}",
            id = case.case_id,
            state = degraded_state_token(case.provider_state),
            channel = case.channel.as_str(),
            degrade = case.degrade_class.as_str(),
            next = case.next_safe_action.as_str(),
        ));
    }

    lines.push("Consumer proofs:".to_owned());
    for proof in &model.consumer_proofs {
        lines.push(format!(
            "  {id}: state={state} channel={channel} degrade={degrade} reuses={reuses}",
            id = proof.consumer_id,
            state = proof.exhibited_state.as_str(),
            channel = proof.representative_channel.as_str(),
            degrade = proof.resolved_degrade.as_str(),
            reuses = proof.reuses_shared_vocabulary,
        ));
    }

    lines.push("Invariants:".to_owned());
    for invariant in &model.invariants {
        lines.push(format!(
            "  {id} holds={holds}",
            id = invariant.invariant_id,
            holds = invariant.holds,
        ));
    }

    lines.push(model.summary.clone());
    lines
}

// ---------------------------------------------------------------------------
// Catalog builders.
// ---------------------------------------------------------------------------

fn build_state_catalog() -> Vec<ClassDescriptor> {
    ConstrainedFileStateClass::ALL
        .iter()
        .map(|state| class_descriptor(state.as_str(), state.label(), state.canonical_class_note()))
        .collect()
}

fn build_channel_catalog() -> Vec<ClassDescriptor> {
    AssistChannelClass::ALL
        .iter()
        .map(|channel| {
            let note = if channel.is_apply_capable() {
                "Apply-capable channel; narrows to read-only or blocked with a route on \
                 write-constrained states."
            } else if channel.is_semantic() {
                "Semantic channel; narrows to a labeled fallback or pending state when the index \
                 or provider is degraded."
            } else {
                "Editing-truth channel; stays full fidelity except where the file is not parsed."
            };
            class_descriptor(channel.as_str(), channel.label(), note)
        })
        .collect()
}

fn build_degrade_catalog() -> Vec<ClassDescriptor> {
    AssistDegradeClass::ALL
        .iter()
        .map(|degrade| {
            let note = if degrade.is_full() {
                "Full-fidelity assist."
            } else if degrade.is_offered() {
                "Offered but narrowed; keyboard-reachable and disclosed."
            } else {
                "Not offered on this state; the channel discloses why it is unavailable."
            };
            class_descriptor(degrade.as_str(), degrade.label(), note)
        })
        .collect()
}

fn build_reason_catalog() -> Vec<ClassDescriptor> {
    NarrowReasonClass::ALL
        .iter()
        .map(|reason| {
            class_descriptor(
                reason.as_str(),
                reason.label(),
                "Inspectable narrowing reason; surfaced as a tooltip, disabled-state diagnostic, \
                 and support-export line.",
            )
        })
        .collect()
}

fn build_next_safe_action_catalog() -> Vec<ClassDescriptor> {
    NextSafeActionClass::ALL
        .iter()
        .map(|action| {
            let note = if action.offers_edit_path() {
                "Routes to a safe edit path away from the constrained surface."
            } else {
                "Inspect-only terminal route; the honest answer that no safe mutation exists here."
            };
            class_descriptor(action.as_str(), action.label(), note)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Profile assembly.
// ---------------------------------------------------------------------------

struct ProfileSpec {
    state: ConstrainedFileStateClass,
    provider_posture: AssistSourceDescriptor,
    blocked_reason_summary: &'static str,
}

fn assemble_profile(spec: ProfileSpec) -> ConstrainedStateProfile {
    let state = spec.state;
    let cells: Vec<AssistNarrowingCell> = AssistChannelClass::ALL
        .iter()
        .map(|channel| resolve_cell(state, *channel))
        .collect();

    let primary = state.primary_next_safe_action();
    let accessibility_summary = format!(
        "{state}: {summary} Nearest safe action: {action}.",
        state = state.label(),
        summary = spec.blocked_reason_summary,
        action = primary.label(),
    );
    let export_safe_summary = format!(
        "{state} narrows assist; primary route {action}, apply_blocked={blocked}.",
        state = state.as_str(),
        action = primary.as_str(),
        blocked = state.blocks_direct_apply(),
    );

    ConstrainedStateProfile {
        record_kind: ConstrainedStateProfile::RECORD_KIND.to_owned(),
        state_class: state,
        label: state.label().to_owned(),
        canonical_class_note: state.canonical_class_note().to_owned(),
        blocks_direct_apply: state.blocks_direct_apply(),
        primary_next_safe_action: primary,
        primary_next_safe_action_command_ref: primary.command_id().to_owned(),
        provider_posture: spec.provider_posture,
        cells,
        blocked_reason_summary: spec.blocked_reason_summary.to_owned(),
        accessibility_summary,
        export_safe_summary,
    }
}

fn build_state_profiles() -> Vec<ConstrainedStateProfile> {
    vec![
        assemble_profile(ProfileSpec {
            state: ConstrainedFileStateClass::ReadOnlyBoundary,
            provider_posture: provider_posture(
                ConstrainedFileStateClass::ReadOnlyBoundary,
                AssistSourceFamily::LanguageServer,
                Some("rust-analyzer"),
                "rust-analyzer",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::WholeWorkspace,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Reads are authoritative; the constraint is the writable boundary, not the provider.",
            ),
            blocked_reason_summary: "Reading is full fidelity; suggestions cannot be applied because \
                                     the file is read-only.",
        }),
        assemble_profile(ProfileSpec {
            state: ConstrainedFileStateClass::GeneratedArtifact,
            provider_posture: provider_posture(
                ConstrainedFileStateClass::GeneratedArtifact,
                AssistSourceFamily::FrameworkPack,
                Some("generated-source-bridge"),
                "Generated-source bridge",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Generated output reads fine; edits route through the generator.",
            ),
            blocked_reason_summary: "Reading is full fidelity; edits route through the generator and \
                                     would otherwise be overwritten.",
        }),
        assemble_profile(ProfileSpec {
            state: ConstrainedFileStateClass::ManagedRegion,
            provider_posture: provider_posture(
                ConstrainedFileStateClass::ManagedRegion,
                AssistSourceFamily::FrameworkPack,
                Some("managed-zone-bridge"),
                "Managed-zone bridge",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Managed region reads fine; hand edits are re-emitted by the regenerator.",
            ),
            blocked_reason_summary: "Reading is full fidelity; the region is tool-managed and must \
                                     be regenerated from source.",
        }),
        assemble_profile(ProfileSpec {
            state: ConstrainedFileStateClass::ProjectionView,
            provider_posture: provider_posture(
                ConstrainedFileStateClass::ProjectionView,
                AssistSourceFamily::LanguageServer,
                Some("projection-bridge"),
                "Projection bridge",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                Vec::new(),
                "The projection mirrors a live source; edits belong to that source.",
            ),
            blocked_reason_summary: "Reading is full fidelity; the view is a projection and edits \
                                     belong to the underlying source.",
        }),
        assemble_profile(ProfileSpec {
            state: ConstrainedFileStateClass::CapturedEvidence,
            provider_posture: provider_posture(
                ConstrainedFileStateClass::CapturedEvidence,
                AssistSourceFamily::ToolAdapter,
                Some("captured-evidence-snapshot"),
                "Captured-evidence snapshot",
                RouterSupportClass::Advisory,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                vec![ScopeLimitClass::SingleFileOnly],
                "An immutable snapshot; readable and copyable but never editable.",
            ),
            blocked_reason_summary: "The snapshot is inspect-only; editing assist is not offered, \
                                     reading and copying remain.",
        }),
        assemble_profile(ProfileSpec {
            state: ConstrainedFileStateClass::PartialIndex,
            provider_posture: provider_posture(
                ConstrainedFileStateClass::PartialIndex,
                AssistSourceFamily::LanguageServer,
                Some("rust-analyzer"),
                "rust-analyzer (indexing)",
                RouterSupportClass::Advisory,
                RouterFreshnessClass::Unverified,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::DegradedScopeNarrowed,
                vec![ScopeLimitClass::SingleFileOnly],
                "Semantic answers are partial and labeled while the index builds.",
            ),
            blocked_reason_summary: "Semantic results are partial while the index builds; a labeled \
                                     fallback is shown and apply stays available.",
        }),
        assemble_profile(ProfileSpec {
            state: ConstrainedFileStateClass::RestrictedMode,
            provider_posture: provider_posture(
                ConstrainedFileStateClass::RestrictedMode,
                AssistSourceFamily::FrameworkPack,
                Some("schema-pack:policy"),
                "Policy schema pack",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::WholeWorkspace,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Reads are authoritative; the constraint is the protected-path write gate.",
            ),
            blocked_reason_summary: "Reading is full fidelity; writes require staged review on this \
                                     protected path.",
        }),
        assemble_profile(ProfileSpec {
            state: ConstrainedFileStateClass::LargeFile,
            provider_posture: provider_posture(
                ConstrainedFileStateClass::LargeFile,
                AssistSourceFamily::FallbackLexical,
                None,
                "Large-file mode",
                RouterSupportClass::Unsupported,
                RouterFreshnessClass::Unverified,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::UnavailableForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::DegradedScopeNarrowed,
                vec![ScopeLimitClass::SingleFileOnly],
                "Semantic assist is suppressed for safety; only lexical highlighting remains.",
            ),
            blocked_reason_summary: "Semantic assist is suppressed to keep the file responsive; open \
                                     it in the full editor to restore assist.",
        }),
    ]
}

// ---------------------------------------------------------------------------
// Degraded-provider cases.
// ---------------------------------------------------------------------------

fn build_degraded_provider_cases() -> Vec<DegradedProviderCase> {
    vec![
        DegradedProviderCase {
            record_kind: DegradedProviderCase::RECORD_KIND.to_owned(),
            case_id: "degraded-provider:provider_unavailable".to_owned(),
            provider_state: RouterDegradedStateClass::DegradedProviderUnavailable,
            source: provider_posture(
                ConstrainedFileStateClass::ReadOnlyBoundary,
                AssistSourceFamily::FallbackLexical,
                None,
                "Lexical fallback",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::DegradedProviderUnavailable,
                vec![ScopeLimitClass::SingleFileOnly],
                "The language server is unavailable; completion falls back to labeled lexical \
                 matches.",
            ),
            channel: AssistChannelClass::Completion,
            degrade_class: AssistDegradeClass::SourceLabeledFallback,
            next_safe_action: NextSafeActionClass::ReconnectProvider,
            next_safe_action_command_ref: NextSafeActionClass::ReconnectProvider
                .command_id()
                .to_owned(),
            source_labeled_not_silent: true,
            disabled_state_diagnostic: "The language server is not responding; completion is showing \
                                        labeled lexical matches. Reconnect the provider to restore \
                                        full results."
                .to_owned(),
            note: "An ordinary writable file narrows purely because the provider is unavailable."
                .to_owned(),
        },
        DegradedProviderCase {
            record_kind: DegradedProviderCase::RECORD_KIND.to_owned(),
            case_id: "degraded-provider:scope_narrowed".to_owned(),
            provider_state: RouterDegradedStateClass::DegradedScopeNarrowed,
            source: provider_posture(
                ConstrainedFileStateClass::PartialIndex,
                AssistSourceFamily::LanguageServer,
                Some("rust-analyzer"),
                "rust-analyzer (warming up)",
                RouterSupportClass::Advisory,
                RouterFreshnessClass::Unverified,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::DegradedScopeNarrowed,
                vec![ScopeLimitClass::SingleFileOnly],
                "The provider answers for the current file only while the workspace index warms up.",
            ),
            channel: AssistChannelClass::Hover,
            degrade_class: AssistDegradeClass::PendingPartialIndex,
            next_safe_action: NextSafeActionClass::WaitForIndex,
            next_safe_action_command_ref: NextSafeActionClass::WaitForIndex.command_id().to_owned(),
            source_labeled_not_silent: true,
            disabled_state_diagnostic: "Hover is answering for this file only while the workspace \
                                        index warms up; full cross-file results arrive when indexing \
                                        completes."
                .to_owned(),
            note: "An ordinary writable file narrows because the provider's scope is temporarily \
                   narrowed."
                .to_owned(),
        },
        DegradedProviderCase {
            record_kind: DegradedProviderCase::RECORD_KIND.to_owned(),
            case_id: "degraded-provider:stale_awaiting_refresh".to_owned(),
            provider_state: RouterDegradedStateClass::DegradedScopeNarrowed,
            source: provider_posture(
                ConstrainedFileStateClass::ReadOnlyBoundary,
                AssistSourceFamily::LanguageServer,
                Some("rust-analyzer"),
                "rust-analyzer (stale)",
                RouterSupportClass::Advisory,
                RouterFreshnessClass::Stale,
                RouterScopeClaimClass::WholeWorkspace,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::DegradedScopeNarrowed,
                Vec::new(),
                "A previous result is shown while a refresh is pending.",
            ),
            channel: AssistChannelClass::SignatureHelp,
            degrade_class: AssistDegradeClass::SourceLabeledFallback,
            next_safe_action: NextSafeActionClass::ReconnectProvider,
            next_safe_action_command_ref: NextSafeActionClass::ReconnectProvider
                .command_id()
                .to_owned(),
            source_labeled_not_silent: true,
            disabled_state_diagnostic: "Signature help is showing a stale result while a refresh is \
                                        pending; it is labeled rather than styled as live."
                .to_owned(),
            note: "An ordinary writable file narrows because the provider's result is stale."
                .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Consumer surface proofs.
// ---------------------------------------------------------------------------

fn consumer_proof(
    consumer_id: &str,
    surface_label: &str,
    base_editor_surface: Option<EditorSurfaceClass>,
    exhibited_state: ConstrainedFileStateClass,
    representative_channel: AssistChannelClass,
    note: &str,
) -> ConsumerSurfaceProof {
    let cell = resolve_cell(exhibited_state, representative_channel);
    ConsumerSurfaceProof {
        record_kind: ConsumerSurfaceProof::RECORD_KIND.to_owned(),
        consumer_id: consumer_id.to_owned(),
        surface_label: surface_label.to_owned(),
        base_editor_surface,
        exhibited_state,
        representative_channel,
        resolved_degrade: cell.degrade_class,
        next_safe_action: cell.next_safe_action,
        reuses_shared_vocabulary: true,
        note: note.to_owned(),
    }
}

fn build_consumer_proofs() -> Vec<ConsumerSurfaceProof> {
    vec![
        consumer_proof(
            "consumer:notebook_cell",
            "Notebook cell",
            Some(EditorSurfaceClass::NotebookCell),
            ConstrainedFileStateClass::PartialIndex,
            AssistChannelClass::Completion,
            "A notebook cell whose cross-cell index is still building reuses the partial-index \
             narrowing: completion is labeled pending and routes to wait-for-index.",
        ),
        consumer_proof(
            "consumer:generated_file",
            "Generated file",
            Some(EditorSurfaceClass::GeneratedFile),
            ConstrainedFileStateClass::GeneratedArtifact,
            AssistChannelClass::Completion,
            "A generated file reuses the generated-artifact narrowing: completion is read-only and \
             routes to open-generator-source.",
        ),
        consumer_proof(
            "consumer:request_artifact",
            "Request response artifact",
            Some(EditorSurfaceClass::RequestEditor),
            ConstrainedFileStateClass::CapturedEvidence,
            AssistChannelClass::Completion,
            "A captured request / response artifact reuses the captured-evidence narrowing: editing \
             completion is unavailable and routes to inspect-only.",
        ),
        consumer_proof(
            "consumer:docs_code_block",
            "Docs-code block",
            Some(EditorSurfaceClass::DocsCodeBlock),
            ConstrainedFileStateClass::ProjectionView,
            AssistChannelClass::Completion,
            "A fenced docs-code block reuses the projection narrowing: completion is read-only and \
             routes to edit-the-underlying-source.",
        ),
        consumer_proof(
            "consumer:protected_config",
            "Protected config",
            Some(EditorSurfaceClass::ProtectedFile),
            ConstrainedFileStateClass::RestrictedMode,
            AssistChannelClass::Completion,
            "A protected config reuses the restricted-mode narrowing: completion is read-only and \
             routes to request-approval.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariant evaluation.
// ---------------------------------------------------------------------------

fn evaluate_invariants(
    profiles: &[ConstrainedStateProfile],
    provider_cases: &[DegradedProviderCase],
    consumers: &[ConsumerSurfaceProof],
) -> Vec<ConstrainedAssistInvariant> {
    let cells: Vec<&AssistNarrowingCell> = profiles
        .iter()
        .flat_map(|profile| profile.cells.iter())
        .collect();

    let mut invariants = Vec::new();

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "every_state_resolves_one_cell_per_channel".into(),
        statement: "Each claimed constrained-file state resolves exactly one narrowing cell per \
                    assist channel."
            .into(),
        holds: !profiles.is_empty()
            && ConstrainedFileStateClass::ALL
                .iter()
                .all(|state| profiles.iter().filter(|p| p.state_class == *state).count() == 1)
            && profiles.iter().all(|profile| {
                AssistChannelClass::ALL.iter().all(|channel| {
                    profile
                        .cells
                        .iter()
                        .filter(|cell| cell.channel == *channel)
                        .count()
                        == 1
                })
            }),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "narrowed_reasons_are_inspectable".into(),
        statement: "Every channel narrowed below full fidelity carries a non-empty disabled-state \
                    diagnostic, so the reason is never silently hidden."
            .into(),
        holds: cells.iter().all(|cell| cell.reason_inspectable()),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "blocked_apply_offers_next_safe_action".into(),
        statement: "Every cell that blocks apply offers a concrete next-safe-action route with a \
                    command."
            .into(),
        holds: cells.iter().all(|cell| cell.apply_block_offers_route()),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "no_silently_hidden_side_effectful_assist".into(),
        statement: "No apply-capable channel is blocked or unavailable without marking apply \
                    blocked and disclosing why."
            .into(),
        holds: cells.iter().all(|cell| cell.no_silent_hidden_side_effect()),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "offered_cells_stay_keyboard_reachable".into(),
        statement: "Every offered (non-blocked) cell stays keyboard-reachable.".into(),
        holds: cells.iter().all(|cell| cell.offered_cell_reachable()),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "large_file_suppresses_semantic_and_apply".into(),
        statement:
            "On the large-file state every semantic and apply-capable channel is suppressed \
                    or blocked, never full fidelity, and discloses it."
                .into(),
        holds: profiles
            .iter()
            .find(|p| p.state_class == ConstrainedFileStateClass::LargeFile)
            .is_some_and(|profile| {
                profile.cells.iter().all(|cell| {
                    if cell.channel.is_semantic() || cell.channel.is_apply_capable() {
                        cell.degrade_class == AssistDegradeClass::SuppressedLargeFile
                            && !cell.disabled_state_diagnostic.trim().is_empty()
                    } else {
                        true
                    }
                })
            }),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "partial_index_narrows_semantic_to_pending".into(),
        statement:
            "On the partial-index state every semantic channel narrows to a labeled pending \
                    state, never silently full fidelity."
                .into(),
        holds: profiles
            .iter()
            .find(|p| p.state_class == ConstrainedFileStateClass::PartialIndex)
            .is_some_and(|profile| {
                profile
                    .cells
                    .iter()
                    .filter(|cell| cell.channel.is_semantic())
                    .all(|cell| cell.degrade_class == AssistDegradeClass::PendingPartialIndex)
            }),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "generated_and_managed_route_writes_to_source".into(),
        statement: "On generated and managed states every apply-capable channel blocks apply and \
                    routes to open-generator-source or regenerate-from-source."
            .into(),
        holds: [
            (
                ConstrainedFileStateClass::GeneratedArtifact,
                NextSafeActionClass::OpenGeneratorSource,
            ),
            (
                ConstrainedFileStateClass::ManagedRegion,
                NextSafeActionClass::RegenerateFromSource,
            ),
        ]
        .iter()
        .all(|(state, action)| {
            profiles
                .iter()
                .find(|p| p.state_class == *state)
                .is_some_and(|profile| {
                    profile
                        .cells
                        .iter()
                        .filter(|cell| cell.channel.is_apply_capable())
                        .all(|cell| cell.apply_blocked && cell.next_safe_action == Some(*action))
                })
        }),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "restricted_routes_writes_to_approval".into(),
        statement:
            "On the restricted state every apply-capable channel blocks apply and routes to \
                    request-approval."
                .into(),
        holds: profiles
            .iter()
            .find(|p| p.state_class == ConstrainedFileStateClass::RestrictedMode)
            .is_some_and(|profile| {
                profile
                    .cells
                    .iter()
                    .filter(|cell| cell.channel.is_apply_capable())
                    .all(|cell| {
                        cell.apply_blocked
                            && cell.next_safe_action
                                == Some(NextSafeActionClass::RequestApprovalReview)
                    })
            }),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "captured_evidence_is_inspect_only".into(),
        statement: "On the captured-evidence state every apply-capable channel is unavailable and \
                    routes to inspect-only, while hover and peek still read."
            .into(),
        holds: profiles
            .iter()
            .find(|p| p.state_class == ConstrainedFileStateClass::CapturedEvidence)
            .is_some_and(|profile| {
                let edit_unavailable = profile
                    .cells
                    .iter()
                    .filter(|cell| cell.channel.is_apply_capable())
                    .all(|cell| {
                        !cell.applicable
                            && cell.degrade_class == AssistDegradeClass::BlockedUnavailable
                            && cell.next_safe_action == Some(NextSafeActionClass::ViewOnlyNoAction)
                    });
                let reads_ok = [AssistChannelClass::Hover, AssistChannelClass::Peek]
                    .iter()
                    .all(|channel| {
                        profile.cell(*channel).is_some_and(|cell| {
                            cell.degrade_class == AssistDegradeClass::FullFidelity
                        })
                    });
                edit_unavailable && reads_ok
            }),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "read_only_and_projection_allow_read_block_write".into(),
        statement: "On read-only and projection states hover and peek stay full fidelity while \
                    apply-capable channels block apply with a duplicate or edit-source route."
            .into(),
        holds: [
            (
                ConstrainedFileStateClass::ReadOnlyBoundary,
                NextSafeActionClass::DuplicateEditableCopy,
            ),
            (
                ConstrainedFileStateClass::ProjectionView,
                NextSafeActionClass::EditUnderlyingSource,
            ),
        ]
        .iter()
        .all(|(state, action)| {
            profiles
                .iter()
                .find(|p| p.state_class == *state)
                .is_some_and(|profile| {
                    let reads_ok = [AssistChannelClass::Hover, AssistChannelClass::Peek]
                        .iter()
                        .all(|channel| {
                            profile.cell(*channel).is_some_and(|cell| {
                                cell.degrade_class == AssistDegradeClass::FullFidelity
                            })
                        });
                    let writes_blocked = profile
                        .cells
                        .iter()
                        .filter(|cell| cell.channel.is_apply_capable())
                        .all(|cell| cell.apply_blocked && cell.next_safe_action == Some(*action));
                    reads_ok && writes_blocked
                })
        }),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "decoration_truth_preserved_except_large_file".into(),
        statement:
            "Editing-truth decorations stay full fidelity on every state except large-file, \
                    where they narrow to a labeled fallback rather than being dropped."
                .into(),
        holds: profiles.iter().all(|profile| {
            profile
                .cell(AssistChannelClass::Decoration)
                .is_some_and(|cell| {
                    if profile.state_class == ConstrainedFileStateClass::LargeFile {
                        cell.degrade_class == AssistDegradeClass::SourceLabeledFallback
                            && !cell.disabled_state_diagnostic.trim().is_empty()
                    } else {
                        cell.degrade_class == AssistDegradeClass::FullFidelity
                    }
                })
        }),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "degraded_provider_cases_source_labeled_not_silent".into(),
        statement: "Every degraded-provider case narrows below full fidelity, is source-labeled, \
                    discloses its reason, and offers a route."
            .into(),
        holds: !provider_cases.is_empty() && provider_cases.iter().all(|case| case.is_honest()),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "degraded_provider_posture_narrows_assist".into(),
        statement: "Every profile whose provider posture is degraded narrows at least one assist \
                    channel."
            .into(),
        holds: profiles
            .iter()
            .filter(|profile| profile.provider_is_degraded())
            .all(|profile| profile.narrows_at_least_one_channel()),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "consumer_surfaces_reuse_shared_vocabulary".into(),
        statement:
            "Every consumer surface reuses a constrained-file state and a catalogued degrade \
                    class instead of a local special case, and its asserted narrowing matches the \
                    resolved model."
                .into(),
        holds: !consumers.is_empty()
            && consumers.iter().all(|proof| {
                proof.reuses_shared_vocabulary
                    && profiles
                        .iter()
                        .find(|p| p.state_class == proof.exhibited_state)
                        .and_then(|p| p.cell(proof.representative_channel))
                        .is_some_and(|cell| {
                            cell.degrade_class == proof.resolved_degrade
                                && cell.next_safe_action == proof.next_safe_action
                        })
            }),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "claimed_consumer_surfaces_present".into(),
        statement: "The notebook, generated, request-artifact, docs-code, and protected surfaces \
                    each prove the shared constrained-assist vocabulary."
            .into(),
        holds: [
            EditorSurfaceClass::NotebookCell,
            EditorSurfaceClass::GeneratedFile,
            EditorSurfaceClass::RequestEditor,
            EditorSurfaceClass::DocsCodeBlock,
            EditorSurfaceClass::ProtectedFile,
        ]
        .iter()
        .all(|surface| {
            consumers
                .iter()
                .any(|proof| proof.base_editor_surface == Some(*surface))
        }),
    });

    invariants.push(ConstrainedAssistInvariant {
        invariant_id: "every_profile_screen_reader_meaningful".into(),
        statement: "Every constrained-state profile carries a non-empty screen-reader summary."
            .into(),
        holds: profiles
            .iter()
            .all(|profile| !profile.accessibility_summary.trim().is_empty()),
    });

    invariants
}

#[cfg(test)]
mod tests;
