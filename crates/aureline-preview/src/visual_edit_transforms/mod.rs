//! Source-first visual-edit transform manifests, preview-diff sheets,
//! unsupported-construct cards, and code-first fallback for claimed
//! round-trip preview rows.
//!
//! Where
//! [`crate::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix`]
//! freezes the *qualification* of each claimed preview/runtime surface,
//! [`crate::preview_session_descriptors`] materializes the *per-session* state,
//! [`crate::inspect_to_source_tree`] materializes the *per-node* source-mapping
//! truth, and [`crate::browser_runtime_inspectors`] materializes the
//! *per-inspector* runtime truth, this module materializes the **per-edit**
//! truth packet behind every claimed visual-edit flow: one shared packet that
//! teaches each visual edit to say whether it is an exact round-trip apply, an
//! approximate round-trip apply, a code-first source suggestion, or an
//! inspect-only fallback — and to prove, before any source byte changes, that it
//! emitted a typed transform manifest, a real source preview diff, and a
//! rollback class.
//!
//! The packet is the one canonical answer to "for the visual action the user is
//! about to take, does this write back to source, what exact source diff will it
//! produce, can it be rolled back, and — if the construct is ambiguous or lossy —
//! does it degrade to a code-first suggestion or inspect-only mode instead of a
//! silent lossy rewrite?" A [`VisualEditTransformPacket`] binds the four
//! visual-edit outcomes onto the same governed vocabulary —
//! [`VisualEditOutcomeClass`], [`crate::RoundTripCapabilityClass`],
//! [`TransformConstructClass`], [`PreviewDiffClass`], [`RollbackClass`], and
//! [`ProtectedPathPosture`] — instead of provider-specific designer chrome.
//!
//! Source stays canonical and the transform packet is derivative — never a
//! second writable truth model. A [`VisualEditRow`] keeps the honesty rules the
//! spec freezes:
//!
//! - **Every apply previews the real source diff first.** An exact or
//!   approximate round-trip apply must carry a [`TransformManifest`] routed
//!   through the shared preview/apply/revert pipeline, a real-source
//!   [`PreviewDiffClass`], and a revertible [`RollbackClass`] — emitted before
//!   the apply, never after.
//! - **No silent lossy rewrite.** An ambiguous or lossy construct
//!   ([`TransformConstructClass::is_ambiguous_or_lossy`]) may never be an apply;
//!   it degrades to a code-first suggestion or inspect-only mode with its
//!   selection context preserved and a precise, non-generic
//!   [`UnsupportedConstructCard`].
//! - **No inspect-to-write auto-upgrade.** An inspect-only row carries no
//!   transform manifest, no real-source diff, and a no-mutation rollback class;
//!   it never silently becomes a write-capable designer flow.
//! - **Protected paths and ownership are preserved.** A
//!   [`ProtectedPathPosture::ProtectedBlocked`] target can never carry an apply;
//!   it must degrade like any other unsupported construct.
//! - **Preview-only stays distinguishable from round-trip.** Each row names its
//!   `framework_pack_family`, so release and support surfaces can tell a
//!   preview-only row from an exact-round-trip row on the very same framework
//!   pack family.
//!
//! Raw source bodies, diff hunks, file contents, credentials, and raw provider
//! payloads never cross this boundary; the packet carries only typed class
//! tokens, opaque span/selection/pipeline/evidence refs, booleans, and redacted
//! labels, so support and diagnostics exports can reconstruct exactly what a
//! visual action would have done without leaking source.
//!
//! The boundary schema is
//! [`schemas/preview/visual_edit_transforms.schema.json`](../../../../schemas/preview/visual_edit_transforms.schema.json).
//! The contract doc is
//! [`docs/preview/m5/visual_edit_transforms.md`](../../../../docs/preview/m5/visual_edit_transforms.md).
//! The protected fixture directory is
//! [`fixtures/preview/m5/visual_edit_transforms/`](../../../../fixtures/preview/m5/visual_edit_transforms/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{MutationReviewPosture, RoundTripCapabilityClass};

/// Stable record-kind tag carried by [`VisualEditTransformPacket`].
pub const VISUAL_EDIT_TRANSFORMS_RECORD_KIND: &str = "visual_edit_transforms";

/// Schema version for the visual-edit transform packet.
pub const VISUAL_EDIT_TRANSFORMS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const VISUAL_EDIT_TRANSFORMS_SCHEMA_REF: &str =
    "schemas/preview/visual_edit_transforms.schema.json";

/// Repo-relative path of the contract doc.
pub const VISUAL_EDIT_TRANSFORMS_DOC_REF: &str = "docs/preview/m5/visual_edit_transforms.md";

/// Repo-relative path of the protected fixture directory.
pub const VISUAL_EDIT_TRANSFORMS_FIXTURE_DIR: &str = "fixtures/preview/m5/visual_edit_transforms";

/// Repo-relative path of the checked support-export artifact.
pub const VISUAL_EDIT_TRANSFORMS_ARTIFACT_REF: &str =
    "artifacts/preview/m5/visual_edit_transforms/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const VISUAL_EDIT_TRANSFORMS_SUMMARY_REF: &str =
    "artifacts/preview/m5/visual_edit_transforms.md";

/// Closed visual-edit-outcome vocabulary. Names the disposition of a claimed
/// visual edit so an exact round-trip apply, an approximate round-trip apply, a
/// code-first source suggestion, and an inspect-only fallback all normalize onto
/// the same packet instead of bespoke per-provider designer chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualEditOutcomeClass {
    /// A supported exact-round-trip edit: it emits a transform manifest, a real
    /// source diff, and a rollback class, and applies the exact canonical-source
    /// diff through the shared preview/apply/revert pipeline.
    ExactRoundTripApply,
    /// An approximate-round-trip edit: it maps approximately to source but still
    /// emits a manifest, a real source diff, and a rollback class, and previews
    /// the diff before commit.
    ApproximateRoundTripApply,
    /// An ambiguous or lossy construct that degrades to a code-first source
    /// suggestion; it performs no visual write and points at a source edit.
    CodeFirstFallback,
    /// An unsupported construct that degrades to inspect-only mode; it performs
    /// no write and no suggestion, but preserves the selection context.
    InspectOnly,
}

impl VisualEditOutcomeClass {
    /// Every visual-edit outcome a claimed M5 visual-edit surface must
    /// demonstrate, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExactRoundTripApply,
        Self::ApproximateRoundTripApply,
        Self::CodeFirstFallback,
        Self::InspectOnly,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRoundTripApply => "exact_round_trip_apply",
            Self::ApproximateRoundTripApply => "approximate_round_trip_apply",
            Self::CodeFirstFallback => "code_first_fallback",
            Self::InspectOnly => "inspect_only",
        }
    }

    /// True when this outcome writes back to source through a transform manifest
    /// and therefore must emit a manifest, a real source diff, and a rollback
    /// class before apply.
    pub const fn applies_transform(self) -> bool {
        matches!(
            self,
            Self::ExactRoundTripApply | Self::ApproximateRoundTripApply
        )
    }

    /// True when this outcome is a non-writing degrade (code-first suggestion or
    /// inspect-only) that must carry an unsupported-construct card.
    pub const fn is_fallback(self) -> bool {
        matches!(self, Self::CodeFirstFallback | Self::InspectOnly)
    }

    /// True when this outcome is a preview-only row (inspect-only) with no
    /// write-back path at all, so release/support can distinguish it from a
    /// round-trip row on the same framework pack family.
    pub const fn is_preview_only(self) -> bool {
        matches!(self, Self::InspectOnly)
    }

    /// Whether the recorded round-trip capability is consistent with this
    /// outcome, so an outcome can never claim a round-trip class it contradicts.
    pub const fn consistent_with_round_trip(self, round_trip: RoundTripCapabilityClass) -> bool {
        matches!(
            (self, round_trip),
            (
                Self::ExactRoundTripApply,
                RoundTripCapabilityClass::ExactSourceRoundTrip
            ) | (
                Self::ApproximateRoundTripApply,
                RoundTripCapabilityClass::ApproximateSourceRoundTrip
            ) | (
                Self::CodeFirstFallback,
                RoundTripCapabilityClass::SourceOnlyFallback
            ) | (
                Self::InspectOnly,
                RoundTripCapabilityClass::InspectOnlyNoWrite
            ) | (Self::InspectOnly, RoundTripCapabilityClass::NoRoundTrip)
        )
    }

    /// Whether the recorded preview-diff class is consistent with this outcome: an
    /// apply previews a real source diff; a code-first fallback previews a
    /// suggestion diff; an inspect-only row shows no diff.
    pub const fn consistent_with_preview_diff(self, diff: PreviewDiffClass) -> bool {
        match self {
            Self::ExactRoundTripApply | Self::ApproximateRoundTripApply => {
                diff.is_real_source_diff()
            }
            Self::CodeFirstFallback => matches!(diff, PreviewDiffClass::CodeFirstSuggestionDiff),
            Self::InspectOnly => matches!(diff, PreviewDiffClass::NoDiffInspectOnly),
        }
    }

    /// Whether the recorded rollback class is consistent with this outcome: an
    /// apply carries a revertible rollback class; a fallback performs no mutation.
    pub const fn consistent_with_rollback(self, rollback: RollbackClass) -> bool {
        if self.applies_transform() {
            rollback.is_revertible()
        } else {
            rollback.is_no_mutation()
        }
    }
}

/// Closed source-construct vocabulary. Names the kind of canonical-source
/// construct a visual edit targets so ambiguous or lossy constructs are typed and
/// can be refused a write rather than silently rewritten.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformConstructClass {
    /// A statically analyzable literal attribute / prop (e.g. a literal class or
    /// style attribute) that maps to an exact canonical-source span.
    StaticAttribute,
    /// A literal design-token or style value that maps to an exact source span.
    StaticStyleToken,
    /// A literal layout container / element node with an exact source span.
    StaticLayoutContainer,
    /// A value bound to a runtime or dynamic expression; the source target is
    /// ambiguous, so a write would be lossy.
    DynamicBoundExpression,
    /// A node generated by a conditional, loop, or map expression; it has no
    /// single hand-authored span to write back to.
    ConditionalOrLoopGenerated,
    /// A node from generated output or an external library with no hand-authored
    /// span at all.
    ExternalOrGeneratedArtifact,
}

impl TransformConstructClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaticAttribute => "static_attribute",
            Self::StaticStyleToken => "static_style_token",
            Self::StaticLayoutContainer => "static_layout_container",
            Self::DynamicBoundExpression => "dynamic_bound_expression",
            Self::ConditionalOrLoopGenerated => "conditional_or_loop_generated",
            Self::ExternalOrGeneratedArtifact => "external_or_generated_artifact",
        }
    }

    /// True when the construct maps to a single hand-authored span and can back an
    /// exact or approximate round-trip apply.
    pub const fn supports_round_trip(self) -> bool {
        matches!(
            self,
            Self::StaticAttribute | Self::StaticStyleToken | Self::StaticLayoutContainer
        )
    }

    /// True when the construct is ambiguous or lossy and so must never be the
    /// target of a write-back apply.
    pub const fn is_ambiguous_or_lossy(self) -> bool {
        !self.supports_round_trip()
    }
}

/// Closed preview-diff vocabulary. Names what diff the user is shown before a
/// visual action commits, so an apply can never hide which source bytes it will
/// change behind a generic confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewDiffClass {
    /// The real single-file source unified diff the apply will write.
    RealSourceUnifiedDiff,
    /// The real multi-file source diff the apply will write.
    RealSourceMultiFileDiff,
    /// A suggested source edit shown as a diff but never auto-applied (code-first
    /// fallback).
    CodeFirstSuggestionDiff,
    /// No diff is shown because the row is inspect-only and performs no write.
    NoDiffInspectOnly,
}

impl PreviewDiffClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RealSourceUnifiedDiff => "real_source_unified_diff",
            Self::RealSourceMultiFileDiff => "real_source_multi_file_diff",
            Self::CodeFirstSuggestionDiff => "code_first_suggestion_diff",
            Self::NoDiffInspectOnly => "no_diff_inspect_only",
        }
    }

    /// True when this diff is the real source diff the apply will write.
    pub const fn is_real_source_diff(self) -> bool {
        matches!(
            self,
            Self::RealSourceUnifiedDiff | Self::RealSourceMultiFileDiff
        )
    }
}

/// Closed rollback-class vocabulary. Names the rollback / checkpoint semantics an
/// apply path carries, so a write can never appear without a way back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackClass {
    /// A checkpoint is taken before apply; the edit is fully revertible.
    CheckpointRevertible,
    /// A working-tree snapshot backs the revert.
    SnapshotRevertible,
    /// An inverse transform manifest backs the revert.
    InverseTransformRevertible,
    /// The row performs no mutation, so no rollback class is needed (fallback /
    /// inspect-only paths).
    NoMutationNoRollback,
}

impl RollbackClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckpointRevertible => "checkpoint_revertible",
            Self::SnapshotRevertible => "snapshot_revertible",
            Self::InverseTransformRevertible => "inverse_transform_revertible",
            Self::NoMutationNoRollback => "no_mutation_no_rollback",
        }
    }

    /// True when this class makes the apply revertible.
    pub const fn is_revertible(self) -> bool {
        matches!(
            self,
            Self::CheckpointRevertible
                | Self::SnapshotRevertible
                | Self::InverseTransformRevertible
        )
    }

    /// True when this class records that the row performs no mutation.
    pub const fn is_no_mutation(self) -> bool {
        matches!(self, Self::NoMutationNoRollback)
    }
}

/// Closed protected-path-posture vocabulary. Names how a protected target gates a
/// visual write so protected-file awareness and ownership are preserved on every
/// visual-edit path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedPathPosture {
    /// The target is not a protected path; an apply may proceed under its review.
    Unprotected,
    /// The target is a protected path; an apply requires review.
    ProtectedReviewRequired,
    /// The target is a protected path; an apply requires owner approval.
    ProtectedOwnerApprovalRequired,
    /// The target is a protected path that blocks the visual write entirely; the
    /// edit must degrade.
    ProtectedBlocked,
}

impl ProtectedPathPosture {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unprotected => "unprotected",
            Self::ProtectedReviewRequired => "protected_review_required",
            Self::ProtectedOwnerApprovalRequired => "protected_owner_approval_required",
            Self::ProtectedBlocked => "protected_blocked",
        }
    }

    /// Whether a write-back apply may proceed against this posture; a blocked
    /// protected path can never carry an apply.
    pub const fn permits_apply(self) -> bool {
        !matches!(self, Self::ProtectedBlocked)
    }
}

/// Closed unsupported-construct-reason vocabulary. Names why a row degraded to a
/// code-first or inspect-only fallback; the card quotes the reason verbatim
/// instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedConstructReason {
    /// The value is bound to a runtime / dynamic expression.
    DynamicBinding,
    /// The node originates in a conditional, loop, or map expression.
    ConditionalOrLoopOrigin,
    /// The node is generated output or from an external library with no span.
    GeneratedOrExternalArtifact,
    /// The source mapping is ambiguous; a write could land on the wrong span.
    AmbiguousSourceMapping,
    /// The transform would be lossy and was refused.
    LossyTransformRejected,
    /// The target is a blocked protected path.
    ProtectedPathBlocked,
}

impl UnsupportedConstructReason {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DynamicBinding => "dynamic_binding",
            Self::ConditionalOrLoopOrigin => "conditional_or_loop_origin",
            Self::GeneratedOrExternalArtifact => "generated_or_external_artifact",
            Self::AmbiguousSourceMapping => "ambiguous_source_mapping",
            Self::LossyTransformRejected => "lossy_transform_rejected",
            Self::ProtectedPathBlocked => "protected_path_blocked",
        }
    }
}

/// A typed transform manifest attached to a round-trip apply row. Its presence is
/// the write affordance; the spec requires that an apply can never appear without
/// one, that it applies the real source diff, and that it routes through the
/// shared preview/apply/revert pipeline rather than a private write path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransformManifest {
    /// Stable manifest id.
    pub manifest_id: String,
    /// Opaque ref naming the shared preview/apply/revert pipeline this manifest
    /// routes through; never a raw file path or command line.
    pub pipeline_ref: String,
    /// True when applying the manifest writes the real source diff previewed on
    /// the row, not a separate shadow model.
    pub applies_real_source_diff: bool,
    /// Whether an inverse transform is available to back a precise revert.
    pub inverse_available: bool,
}

impl TransformManifest {
    /// Whether the manifest is internally complete and honest.
    pub fn is_complete(&self) -> bool {
        !self.manifest_id.trim().is_empty()
            && !self.pipeline_ref.trim().is_empty()
            && self.applies_real_source_diff
    }
}

/// An unsupported-construct card attached to a fallback row. Its presence is the
/// honest degrade; the spec requires that an ambiguous or lossy construct degrade
/// to a code-first suggestion or inspect-only mode with its selection context
/// preserved and a precise, non-generic label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedConstructCard {
    /// Why the row degraded.
    pub reason: UnsupportedConstructReason,
    /// True when the selection context is preserved across the degrade so the
    /// user keeps their place; must always hold.
    pub preserves_selection_context: bool,
    /// Precise, non-generic card label safe to render on the card.
    pub card_label: String,
}

impl UnsupportedConstructCard {
    /// Whether the card is internally complete and honest.
    pub fn is_complete(&self) -> bool {
        self.preserves_selection_context && !label_is_generic(&self.card_label)
    }
}

/// One visual edit: the shared truth packet a single visual action presents
/// before any source byte changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualEditRow {
    /// Stable edit id.
    pub edit_id: String,
    /// Opaque framework-pack-family token (e.g. `react`, `flutter`); never a raw
    /// path. Lets release/support distinguish preview-only rows from
    /// exact-round-trip rows on the same family.
    pub framework_pack_family: String,
    /// Human-readable label of the surface the edit acts on.
    pub surface_label: String,
    /// The disposition of this visual edit.
    pub outcome: VisualEditOutcomeClass,
    /// The round-trip capability this edit claims; must agree with the outcome.
    pub round_trip: RoundTripCapabilityClass,
    /// The kind of source construct the edit targets.
    pub construct_class: TransformConstructClass,
    /// Opaque ref to the selection context preserved across a degrade; never raw
    /// source.
    pub selection_context_ref: String,
    /// Opaque ref to the canonical-source target span; never raw source bytes.
    pub source_target_ref: String,
    /// What diff the user is shown before commit.
    pub preview_diff: PreviewDiffClass,
    /// The rollback / checkpoint semantics this row carries.
    pub rollback_class: RollbackClass,
    /// How a protected target gates the write.
    pub protected_path_posture: ProtectedPathPosture,
    /// The review / confirmation an apply requires; required for an apply outcome
    /// and absent for a fallback outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_posture: Option<MutationReviewPosture>,
    /// The typed transform manifest backing an apply; required for an apply and
    /// absent for a fallback.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_manifest: Option<TransformManifest>,
    /// The unsupported-construct card backing a degrade; required for a fallback
    /// and absent for an apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsupported_card: Option<UnsupportedConstructCard>,
    /// Human-readable label summary safe to render on the row.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the visual-edit state was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
}

impl VisualEditRow {
    /// Whether the round-trip capability agrees with the outcome.
    pub fn round_trip_ok(&self) -> bool {
        self.outcome.consistent_with_round_trip(self.round_trip)
    }

    /// Whether the construct class permits the outcome: an apply may target only a
    /// round-trip-capable construct, so an ambiguous or lossy construct is never
    /// silently rewritten.
    pub fn construct_ok(&self) -> bool {
        if self.outcome.applies_transform() {
            self.construct_class.supports_round_trip()
        } else {
            true
        }
    }

    /// Whether the preview diff agrees with the outcome.
    pub fn preview_diff_ok(&self) -> bool {
        self.outcome.consistent_with_preview_diff(self.preview_diff)
    }

    /// Whether the rollback class agrees with the outcome.
    pub fn rollback_ok(&self) -> bool {
        self.outcome.consistent_with_rollback(self.rollback_class)
    }

    /// Whether the protected-path posture permits the outcome: a blocked protected
    /// path can never carry an apply.
    pub fn protected_path_ok(&self) -> bool {
        if self.outcome.applies_transform() {
            self.protected_path_posture.permits_apply()
        } else {
            true
        }
    }

    /// Whether the review posture presence matches the outcome: an apply carries a
    /// review/confirmation posture; a fallback carries none.
    pub fn review_posture_ok(&self) -> bool {
        if self.outcome.applies_transform() {
            self.review_posture.is_some()
        } else {
            self.review_posture.is_none()
        }
    }

    /// Whether the transform manifest presence and shape match the outcome: an
    /// apply carries a complete manifest that applies the real source diff; a
    /// fallback carries none.
    pub fn transform_manifest_ok(&self) -> bool {
        match (&self.transform_manifest, self.outcome.applies_transform()) {
            (Some(manifest), true) => manifest.is_complete(),
            (None, false) => true,
            _ => false,
        }
    }

    /// Whether the unsupported-construct card presence and shape match the
    /// outcome: a fallback carries a complete card that preserves selection
    /// context; an apply carries none.
    pub fn unsupported_card_ok(&self) -> bool {
        match (&self.unsupported_card, self.outcome.is_fallback()) {
            (Some(card), true) => card.is_complete(),
            (None, false) => true,
            _ => false,
        }
    }

    /// Whether this row is a complete, honest round-trip apply that emitted a
    /// manifest, a real source diff, and a revertible rollback class before apply.
    pub fn is_round_trip_apply(&self) -> bool {
        self.outcome.applies_transform()
            && self.transform_manifest_ok()
            && self.preview_diff.is_real_source_diff()
            && self.rollback_class.is_revertible()
            && self.is_complete()
    }

    /// Whether this row is a complete, honest fallback that degraded with a card.
    pub fn is_fallback(&self) -> bool {
        self.outcome.is_fallback() && self.unsupported_card_ok() && self.is_complete()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "outcome={outcome} round_trip={round_trip} construct={construct} \
diff={diff} rollback={rollback} protected={protected} family={family}",
            outcome = self.outcome.as_str(),
            round_trip = self.round_trip.as_str(),
            construct = self.construct_class.as_str(),
            diff = self.preview_diff.as_str(),
            rollback = self.rollback_class.as_str(),
            protected = self.protected_path_posture.as_str(),
            family = self.framework_pack_family,
        )
    }

    /// Whether every dimension required to record this row is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.edit_id.trim().is_empty()
            && !self.framework_pack_family.trim().is_empty()
            && !self.surface_label.trim().is_empty()
            && !self.selection_context_ref.trim().is_empty()
            && !self.source_target_ref.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.round_trip_ok()
            && self.construct_ok()
            && self.preview_diff_ok()
            && self.rollback_ok()
            && self.protected_path_ok()
            && self.review_posture_ok()
            && self.transform_manifest_ok()
            && self.unsupported_card_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the visual-edit transform packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualEditTransformGuardrails {
    /// Source remains canonical; the transform packet is derivative, never a
    /// second writable truth model.
    pub source_canonical_no_second_writable_model: bool,
    /// Runtime state or extension-private wording never hides source-mapping
    /// uncertainty.
    pub private_wording_never_hides_mapping_uncertainty: bool,
    /// Inspect-only rows are never auto-upgraded into write-capable designer flows.
    pub inspect_only_never_auto_upgraded_to_write: bool,
    /// Embedded preview / browser boundaries are not blurred into product authority.
    pub embedded_boundaries_not_blurred_into_product: bool,
    /// Every apply emits a transform manifest, a real source diff, and a rollback
    /// class before apply.
    pub every_apply_emits_manifest_diff_and_rollback: bool,
    /// Ambiguous or lossy constructs are never silently rewritten; they degrade.
    pub ambiguous_constructs_never_silently_rewritten: bool,
    /// Protected-file awareness, ownership, and rollback semantics are preserved.
    pub protected_paths_and_ownership_preserved: bool,
    /// Applies route through the shared preview/apply/revert pipeline used by
    /// other wide-scope mutations.
    pub apply_routes_through_shared_pipeline: bool,
}

impl VisualEditTransformGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.source_canonical_no_second_writable_model
            && self.private_wording_never_hides_mapping_uncertainty
            && self.inspect_only_never_auto_upgraded_to_write
            && self.embedded_boundaries_not_blurred_into_product
            && self.every_apply_emits_manifest_diff_and_rollback
            && self.ambiguous_constructs_never_silently_rewritten
            && self.protected_paths_and_ownership_preserved
            && self.apply_routes_through_shared_pipeline
    }
}

/// Consumer-projection block for the visual-edit transform packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualEditTransformConsumerProjection {
    /// Product surfaces ingest these transform rows instead of cloning chip text.
    pub product_ingests_transforms: bool,
    /// Docs/help ingests the same transform rows.
    pub docs_help_ingests_transforms: bool,
    /// Diagnostics ingests the same transform rows.
    pub diagnostics_ingests_transforms: bool,
    /// Support export ingests the same transform rows.
    pub support_export_ingests_transforms: bool,
    /// Release-control surfaces ingest the same transform rows.
    pub release_control_ingests_transforms: bool,
    /// Release/support surfaces can distinguish preview-only rows from
    /// exact-round-trip rows on the same framework pack family.
    pub release_distinguishes_preview_only_from_round_trip: bool,
}

impl VisualEditTransformConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_transforms
            && self.docs_help_ingests_transforms
            && self.diagnostics_ingests_transforms
            && self.support_export_ingests_transforms
            && self.release_control_ingests_transforms
            && self.release_distinguishes_preview_only_from_round_trip
    }
}

/// Constructor input for [`VisualEditTransformPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualEditTransformPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-edit descriptors.
    pub edits: Vec<VisualEditRow>,
    /// Guardrail invariants block.
    pub guardrails: VisualEditTransformGuardrails,
    /// Consumer projection block.
    pub consumer_projection: VisualEditTransformConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe visual-edit transform packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualEditTransformPacket {
    /// Record kind; must equal [`VISUAL_EDIT_TRANSFORMS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VISUAL_EDIT_TRANSFORMS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-edit descriptors.
    pub edits: Vec<VisualEditRow>,
    /// Guardrail invariants block.
    pub guardrails: VisualEditTransformGuardrails,
    /// Consumer projection block.
    pub consumer_projection: VisualEditTransformConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl VisualEditTransformPacket {
    /// Builds a visual-edit transform packet.
    pub fn new(input: VisualEditTransformPacketInput) -> Self {
        Self {
            record_kind: VISUAL_EDIT_TRANSFORMS_RECORD_KIND.to_owned(),
            schema_version: VISUAL_EDIT_TRANSFORMS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            edits: input.edits,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Outcomes represented by some row in this packet.
    pub fn represented_outcomes(&self) -> BTreeSet<VisualEditOutcomeClass> {
        self.edits.iter().map(|r| r.outcome).collect()
    }

    /// Round-trip classes represented by some row in this packet.
    pub fn represented_round_trips(&self) -> BTreeSet<RoundTripCapabilityClass> {
        self.edits.iter().map(|r| r.round_trip).collect()
    }

    /// Count of rows that are complete, honest round-trip applies.
    pub fn apply_row_count(&self) -> usize {
        self.edits
            .iter()
            .filter(|r| r.is_round_trip_apply())
            .count()
    }

    /// Count of rows that are complete, honest fallbacks.
    pub fn fallback_row_count(&self) -> usize {
        self.edits.iter().filter(|r| r.is_fallback()).count()
    }

    /// Whether some framework pack family carries both an exact-round-trip apply
    /// row and a preview-only (inspect-only) row, proving release/support can
    /// distinguish the two on the same family.
    pub fn has_family_with_round_trip_and_preview_only(&self) -> bool {
        let round_trip_families: BTreeSet<&str> = self
            .edits
            .iter()
            .filter(|r| {
                r.outcome == VisualEditOutcomeClass::ExactRoundTripApply && r.is_round_trip_apply()
            })
            .map(|r| r.framework_pack_family.as_str())
            .collect();
        self.edits
            .iter()
            .filter(|r| r.outcome.is_preview_only() && r.is_fallback())
            .any(|r| round_trip_families.contains(r.framework_pack_family.as_str()))
    }

    /// Validates the visual-edit transform packet invariants.
    pub fn validate(&self) -> Vec<VisualEditTransformViolation> {
        let mut violations = Vec::new();

        if self.record_kind != VISUAL_EDIT_TRANSFORMS_RECORD_KIND {
            violations.push(VisualEditTransformViolation::WrongRecordKind);
        }
        if self.schema_version != VISUAL_EDIT_TRANSFORMS_SCHEMA_VERSION {
            violations.push(VisualEditTransformViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(VisualEditTransformViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("visual-edit transform packet serializes"),
        ) {
            violations.push(VisualEditTransformViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("visual-edit transform packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Visual-Edit Transforms\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Edits: {} ({} round-trip applies, {} fallbacks)\n",
            self.edits.len(),
            self.apply_row_count(),
            self.fallback_row_count()
        ));
        out.push_str(&format!(
            "- Outcomes: {} / {}\n",
            self.represented_outcomes().len(),
            VisualEditOutcomeClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Round-trip-vs-preview-only on a shared family: {}\n",
            self.has_family_with_round_trip_and_preview_only()
        ));
        out.push_str("\n## Edits\n\n");
        for row in &self.edits {
            out.push_str(&format!(
                "- **{}** ({}) [{}]\n",
                row.edit_id,
                row.outcome.as_str(),
                row.framework_pack_family
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!("  - {}\n", row.chip_tokens()));
            if let Some(manifest) = &row.transform_manifest {
                out.push_str(&format!(
                    "  - Manifest: `{}` pipeline=`{}` inverse={}\n",
                    manifest.manifest_id, manifest.pipeline_ref, manifest.inverse_available,
                ));
            }
            if let Some(card) = &row.unsupported_card {
                out.push_str(&format!(
                    "  - Fallback: reason={} — {}\n",
                    card.reason.as_str(),
                    card.card_label,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in visual-edit transform export.
#[derive(Debug)]
pub enum VisualEditTransformArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VisualEditTransformViolation>),
}

impl fmt::Display for VisualEditTransformArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "visual-edit transform export parse failed: {error}"
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
                    "visual-edit transform export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for VisualEditTransformArtifactError {}

/// Validation failures emitted by [`VisualEditTransformPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisualEditTransformViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required visual-edit outcome is represented by no row.
    RequiredOutcomeMissing,
    /// The packet demonstrates no complete round-trip apply row.
    RoundTripApplyCaseMissing,
    /// The packet demonstrates no complete fallback row.
    FallbackCaseMissing,
    /// No framework pack family carries both a round-trip and a preview-only row.
    RoundTripVsPreviewOnlyCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row's round-trip capability disagrees with its outcome.
    RoundTripMismatch,
    /// A row's construct class cannot back its outcome (a lossy construct on apply).
    ConstructUnsupportedForOutcome,
    /// A row's preview diff disagrees with its outcome.
    PreviewDiffMismatch,
    /// A row's rollback class disagrees with its outcome.
    RollbackMismatch,
    /// A row applies a write against a blocked protected path.
    ProtectedPathViolation,
    /// A row's review-posture presence is inconsistent with its outcome.
    ReviewPostureInconsistent,
    /// A row's transform manifest presence or shape is inconsistent with its outcome.
    TransformManifestInconsistent,
    /// A row's unsupported-construct card presence or shape is inconsistent.
    UnsupportedCardInconsistent,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl VisualEditTransformViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredOutcomeMissing => "required_outcome_missing",
            Self::RoundTripApplyCaseMissing => "round_trip_apply_case_missing",
            Self::FallbackCaseMissing => "fallback_case_missing",
            Self::RoundTripVsPreviewOnlyCaseMissing => "round_trip_vs_preview_only_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RoundTripMismatch => "round_trip_mismatch",
            Self::ConstructUnsupportedForOutcome => "construct_unsupported_for_outcome",
            Self::PreviewDiffMismatch => "preview_diff_mismatch",
            Self::RollbackMismatch => "rollback_mismatch",
            Self::ProtectedPathViolation => "protected_path_violation",
            Self::ReviewPostureInconsistent => "review_posture_inconsistent",
            Self::TransformManifestInconsistent => "transform_manifest_inconsistent",
            Self::UnsupportedCardInconsistent => "unsupported_card_inconsistent",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in visual-edit transform export.
pub fn current_m5_visual_edit_transforms_export(
) -> Result<VisualEditTransformPacket, VisualEditTransformArtifactError> {
    let packet: VisualEditTransformPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/preview/m5/visual_edit_transforms/support_export.json"
    )))
    .map_err(VisualEditTransformArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VisualEditTransformArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &VisualEditTransformPacket,
    violations: &mut Vec<VisualEditTransformViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        VISUAL_EDIT_TRANSFORMS_SCHEMA_REF,
        VISUAL_EDIT_TRANSFORMS_DOC_REF,
        VISUAL_EDIT_TRANSFORMS_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(VisualEditTransformViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &VisualEditTransformPacket,
    violations: &mut Vec<VisualEditTransformViolation>,
) {
    let outcomes = packet.represented_outcomes();
    for required in VisualEditOutcomeClass::ALL {
        if !outcomes.contains(&required) {
            violations.push(VisualEditTransformViolation::RequiredOutcomeMissing);
            break;
        }
    }

    if packet.apply_row_count() == 0 {
        violations.push(VisualEditTransformViolation::RoundTripApplyCaseMissing);
    }
    if packet.fallback_row_count() == 0 {
        violations.push(VisualEditTransformViolation::FallbackCaseMissing);
    }
    if !packet.has_family_with_round_trip_and_preview_only() {
        violations.push(VisualEditTransformViolation::RoundTripVsPreviewOnlyCaseMissing);
    }
}

fn validate_rows(
    packet: &VisualEditTransformPacket,
    violations: &mut Vec<VisualEditTransformViolation>,
) {
    for row in &packet.edits {
        if !row.is_complete() {
            violations.push(VisualEditTransformViolation::RowIncomplete);
        }
        if !row.round_trip_ok() {
            violations.push(VisualEditTransformViolation::RoundTripMismatch);
        }
        if !row.construct_ok() {
            violations.push(VisualEditTransformViolation::ConstructUnsupportedForOutcome);
        }
        if !row.preview_diff_ok() {
            violations.push(VisualEditTransformViolation::PreviewDiffMismatch);
        }
        if !row.rollback_ok() {
            violations.push(VisualEditTransformViolation::RollbackMismatch);
        }
        if !row.protected_path_ok() {
            violations.push(VisualEditTransformViolation::ProtectedPathViolation);
        }
        if !row.review_posture_ok() {
            violations.push(VisualEditTransformViolation::ReviewPostureInconsistent);
        }
        if !row.transform_manifest_ok() {
            violations.push(VisualEditTransformViolation::TransformManifestInconsistent);
        }
        if !row.unsupported_card_ok() {
            violations.push(VisualEditTransformViolation::UnsupportedCardInconsistent);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(VisualEditTransformViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &VisualEditTransformPacket,
    violations: &mut Vec<VisualEditTransformViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(VisualEditTransformViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &VisualEditTransformPacket,
    violations: &mut Vec<VisualEditTransformViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(VisualEditTransformViolation::ConsumerProjectionIncomplete);
    }
}

/// Whether a degraded / card label is a generic non-answer rather than a precise
/// label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "cannot edit"
            | "no mapping"
            | "blocked"
            | "fallback"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
