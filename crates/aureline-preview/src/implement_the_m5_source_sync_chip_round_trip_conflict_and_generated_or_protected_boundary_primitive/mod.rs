//! One reusable M5 source-round-trip honesty primitive: the source-sync chip, the
//! round-trip conflict banner, the unsupported-construct card, and the
//! generated-or-protected-file boundary notice for one designer target, resolved
//! once so a surface never implies broader or safer write authority than the
//! source model actually supports.
//!
//! Aureline's frozen visual-designer component matrix
//! ([`crate::freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix`])
//! names the source-sync chip, the round-trip conflict banner, and the
//! unsupported-construct card as governed component families and freezes their
//! state vocabulary. The selected-node primitive
//! ([`crate::implement_the_m5_design_canvas_structure_tree_and_property_inspector_selected_node_primitive`])
//! implements the canvas / tree / inspector families. This module *implements* the
//! remaining round-trip-honesty families as one reusable primitive: a resolver that
//! takes one designer target's round-trip situation and produces one
//! [`M5ResolvedRoundTripStatus`] carrying the chip, the conflict banner or
//! unsupported-construct card, the generated-or-protected boundary notice, and the
//! honest write authority — all sharing one target identity.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_round_trip_status`] — that takes one
//!    [`M5RoundTripStatusInput`] (a target's source-sync class, its round-trip
//!    capability, its source-boundary class, its protected-path posture, an
//!    optional round-trip conflict, and an optional unsupported construct) and
//!    produces one [`M5ResolvedRoundTripStatus`]. Unsupported constructs, manual
//!    source drift, generated sections, and protected files can never be silently
//!    normalized into a write path (AC1): whenever a hard block is present the
//!    resolved write authority is narrowed to a source-first fallback or read-only,
//!    never a plain writable surface. When round-trip support drops the resolver
//!    names an *exact* source-first fallback route instead of a best-effort
//!    writeback (AC2). Every narrowing or read-only outcome carries a typed
//!    downgrade trigger and a precise label so support and release packets can
//!    explain why a surface narrowed (AC3).
//! 2. A parity matrix — [`M5RoundTripHonestyPacket`] — that binds one row per
//!    claimed M5 visual-design surface family to the shared chip / banner / card /
//!    notice contract and carries worked resolution cases so the support / export
//!    packet can reconstruct round-trip truth from one shared model on every
//!    surface.
//!
//! The source-sync class ([`SourceSyncClass`]), the round-trip capability class
//! ([`RoundTripCapabilityClass`]), the sync-recovery route
//! ([`M5SyncRecoveryRoute`]), the round-trip conflict class
//! ([`M5RoundTripConflictClass`]), the conflict resolution route
//! ([`M5ConflictResolutionRoute`]), the unsupported-construct reason
//! ([`UnsupportedConstructReason`]), the protected-path posture
//! ([`ProtectedPathPosture`]), the downgrade triggers
//! ([`M5VisualDesignerDowngradeTrigger`]), and the surface families
//! ([`M5VisualDesignSurfaceFamily`]) are reused verbatim from the frozen matrix and
//! the sibling primitives. This module mints new vocabulary only for what the
//! round-trip-honesty primitive itself needs: the source-sync chip state, the
//! source-boundary class, the honest write authority, the source-first fallback
//! route, and the export fields. No M5 surface invents a second round-trip grammar.
//!
//! Raw source bodies, diff hunks, file contents, credentials, and raw provider
//! payloads never cross this boundary; the primitive carries only typed class
//! tokens, opaque target / span refs, booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-source-round-trip-honesty-primitive.schema.json`](../../../../schemas/ui/m5-source-round-trip-honesty-primitive.schema.json)
//! and the contract doc is
//! [`docs/designer/m5_source_round_trip_honesty_primitive_contract.md`](../../../../docs/designer/m5_source_round_trip_honesty_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-source-round-trip-honesty-primitive/`](../../../../fixtures/ui/m5-source-round-trip-honesty-primitive/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The source-sync class, round-trip capability, sync-recovery route, round-trip
// conflict class, conflict resolution route, unsupported-construct reason,
// protected-path posture, downgrade triggers, preview surface, and visual-design
// surface families are all frozen once, in the sibling matrices and primitives.
// This primitive reuses them verbatim so it never invents a parallel round-trip
// vocabulary.
pub use crate::{
    M5ConflictResolutionRoute, M5RoundTripConflictClass, M5SyncRecoveryRoute,
    M5VisualDesignSurfaceFamily, M5VisualDesignerDowngradeTrigger, PreviewSurface,
    ProtectedPathPosture, RoundTripCapabilityClass, SourceSyncClass, UnsupportedConstructReason,
};

/// Stable record-kind tag carried by [`M5RoundTripHonestyPacket`].
pub const M5_ROUND_TRIP_HONESTY_RECORD_KIND: &str =
    "implement_m5_source_sync_chip_round_trip_conflict_and_generated_or_protected_boundary_primitive";

/// Schema version for M5 round-trip-honesty-primitive records.
pub const M5_ROUND_TRIP_HONESTY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the round-trip-honesty-primitive boundary schema.
pub const M5_ROUND_TRIP_SCHEMA_REF: &str =
    "schemas/ui/m5-source-round-trip-honesty-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_ROUND_TRIP_DOC_REF: &str =
    "docs/designer/m5_source_round_trip_honesty_primitive_contract.md";

/// Repo-relative path of the frozen visual-designer component matrix this
/// primitive narrows from.
pub const M5_ROUND_TRIP_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-visual-designer-component-matrix.schema.json";

/// Repo-relative path of the visual-edit-transform contract this primitive binds
/// its round-trip capability against.
pub const M5_ROUND_TRIP_VISUAL_EDIT_REF: &str =
    "schemas/preview/visual_edit_transforms.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_ROUND_TRIP_FIXTURE_DIR: &str = "fixtures/ui/m5-source-round-trip-honesty-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_ROUND_TRIP_ARTIFACT_REF: &str =
    "artifacts/release/m5-source-round-trip-honesty-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_ROUND_TRIP_CSV_REF: &str =
    "artifacts/release/m5-source-round-trip-honesty-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_ROUND_TRIP_REPORT_REF: &str =
    "artifacts/components/m5-source-round-trip-honesty-primitive.md";

/// The state a source-sync chip discloses. These are the states the goal names —
/// in-sync, unsaved, needs-refresh, unsupported-construct, and conflict — so a
/// user always sees how the surface relates to canonical source before editing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceSyncChipState {
    /// The surface reflects the current canonical source.
    InSync,
    /// The surface has an unsaved visual edit not yet written to source.
    Unsaved,
    /// The surface drifted or is pending a rebuild and needs a refresh / re-attach.
    NeedsRefresh,
    /// The target is an unsupported construct that cannot round-trip.
    UnsupportedConstruct,
    /// A round-trip conflict is open on the target.
    Conflict,
}

impl M5SourceSyncChipState {
    /// Every chip state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InSync,
        Self::Unsaved,
        Self::NeedsRefresh,
        Self::UnsupportedConstruct,
        Self::Conflict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSync => "in_sync",
            Self::Unsaved => "unsaved",
            Self::NeedsRefresh => "needs_refresh",
            Self::UnsupportedConstruct => "unsupported_construct",
            Self::Conflict => "conflict",
        }
    }

    /// A precise, non-generic chip label safe to render on any surface.
    pub const fn label(self) -> &'static str {
        match self {
            Self::InSync => "In sync with source",
            Self::Unsaved => "Unsaved visual edit — not yet written to source",
            Self::NeedsRefresh => "Needs refresh — the view drifted from source",
            Self::UnsupportedConstruct => {
                "Unsupported construct — this target cannot round-trip to source"
            }
            Self::Conflict => "Round-trip conflict — source changed under this edit",
        }
    }
}

/// The source-boundary class of a designer target's file. Names whether the file
/// is author-owned writable source, a generated / managed zone, a protected
/// read-only file, a file with mixed managed regions, or external / vendored
/// source, so a designer edit can never silently widen into a managed or
/// non-writable source zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceBoundaryClass {
    /// Author-owned, writable canonical source.
    AuthorOwned,
    /// A generated / managed file the designer must not write directly.
    GeneratedManaged,
    /// A protected, read-only file.
    ProtectedReadOnly,
    /// A file with managed regions interleaved with author-owned regions; edits to
    /// author-owned spans are allowed but require review.
    MixedManagedRegion,
    /// External / vendored source with no writeback.
    ExternalVendored,
}

impl M5SourceBoundaryClass {
    /// Every source-boundary class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AuthorOwned,
        Self::GeneratedManaged,
        Self::ProtectedReadOnly,
        Self::MixedManagedRegion,
        Self::ExternalVendored,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorOwned => "author_owned",
            Self::GeneratedManaged => "generated_managed",
            Self::ProtectedReadOnly => "protected_read_only",
            Self::MixedManagedRegion => "mixed_managed_region",
            Self::ExternalVendored => "external_vendored",
        }
    }

    /// True when the designer may write to this boundary at all. Author-owned
    /// source and the author-owned spans of a mixed region permit a write; a
    /// generated / managed, protected, or external file never does.
    pub const fn permits_designer_write(self) -> bool {
        matches!(self, Self::AuthorOwned | Self::MixedManagedRegion)
    }

    /// True when a write to this boundary must route through an owner / managed-file
    /// flow rather than a direct designer write.
    pub const fn requires_owner_flow(self) -> bool {
        matches!(
            self,
            Self::GeneratedManaged | Self::ProtectedReadOnly | Self::ExternalVendored
        )
    }

    /// A precise, non-generic boundary-notice label safe to render.
    pub const fn notice_label(self) -> &'static str {
        match self {
            Self::AuthorOwned => "Author-owned source — edits write back directly",
            Self::GeneratedManaged => {
                "Generated / managed file — the designer cannot write here; edit the generator"
            }
            Self::ProtectedReadOnly => {
                "Protected read-only file — the designer cannot write here"
            }
            Self::MixedManagedRegion => {
                "Mixed managed regions — only author-owned spans are writable and edits require review"
            }
            Self::ExternalVendored => {
                "External / vendored source — no writeback; changes belong upstream"
            }
        }
    }
}

/// The honest write authority a round-trip status carries. Names exactly how much
/// write authority the source model supports so a canvas edit never implies more.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteAuthority {
    /// A visual edit writes back to source through the shared apply / revert path.
    Writable,
    /// A visual edit writes back but requires review first.
    WritableWithReview,
    /// No visual writeback; edits fall back to editing the source directly.
    SourceOnlyFallback,
    /// The surface is read-only; it takes no write at all.
    ReadOnly,
}

impl M5WriteAuthority {
    /// Every write authority, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Writable,
        Self::WritableWithReview,
        Self::SourceOnlyFallback,
        Self::ReadOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Writable => "writable",
            Self::WritableWithReview => "writable_with_review",
            Self::SourceOnlyFallback => "source_only_fallback",
            Self::ReadOnly => "read_only",
        }
    }

    /// True when a visual edit under this authority writes back to source.
    pub const fn writes_back(self) -> bool {
        matches!(self, Self::Writable | Self::WritableWithReview)
    }

    /// True when this authority is a source-first fallback or read-only (no visual
    /// writeback).
    pub const fn is_narrowed(self) -> bool {
        matches!(self, Self::SourceOnlyFallback | Self::ReadOnly)
    }
}

/// The exact source-first fallback route offered when round-trip support drops, so
/// a user gets a real next step instead of a best-effort writeback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceFirstFallback {
    /// Open the canonical source and edit it directly.
    OpenSourceEditDirectly,
    /// Reload the canonical source, then re-apply the edit.
    ReloadSourceThenReapply,
    /// Keep source and discard the visual change.
    KeepSourceDiscardVisual,
    /// Open the managed-file / owner flow that governs this file.
    OpenManagedFileOwnerFlow,
    /// Inspect-only; the surface takes no write at all.
    InspectOnlyNoWrite,
}

impl M5SourceFirstFallback {
    /// Every source-first fallback route, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenSourceEditDirectly,
        Self::ReloadSourceThenReapply,
        Self::KeepSourceDiscardVisual,
        Self::OpenManagedFileOwnerFlow,
        Self::InspectOnlyNoWrite,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenSourceEditDirectly => "open_source_edit_directly",
            Self::ReloadSourceThenReapply => "reload_source_then_reapply",
            Self::KeepSourceDiscardVisual => "keep_source_discard_visual",
            Self::OpenManagedFileOwnerFlow => "open_managed_file_owner_flow",
            Self::InspectOnlyNoWrite => "inspect_only_no_write",
        }
    }
}

/// A field the support / export packet carries so round-trip truth is
/// reconstructable from the shared model. The first four in
/// [`M5RoundTripExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RoundTripExportField {
    /// The stable target identity, shared across chip / banner / card / notice.
    TargetId,
    /// The source-sync class.
    SyncClass,
    /// The source-sync chip state.
    ChipState,
    /// The honest write authority.
    WriteAuthority,
    /// The source-boundary class.
    BoundaryClass,
    /// The downgrade trigger, when the surface narrowed.
    DowngradeTrigger,
}

impl M5RoundTripExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TargetId,
        Self::SyncClass,
        Self::ChipState,
        Self::WriteAuthority,
        Self::BoundaryClass,
        Self::DowngradeTrigger,
    ];

    /// The export fields every round-trip export must carry.
    pub const MANDATORY: [Self; 4] = [
        Self::TargetId,
        Self::ChipState,
        Self::WriteAuthority,
        Self::BoundaryClass,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetId => "target_id",
            Self::SyncClass => "sync_class",
            Self::ChipState => "chip_state",
            Self::WriteAuthority => "write_authority",
            Self::BoundaryClass => "boundary_class",
            Self::DowngradeTrigger => "downgrade_trigger",
        }
    }
}

/// The full input to the round-trip resolver for one designer target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RoundTripStatusInput {
    /// The stable target identity that must survive across chip / banner / card /
    /// notice.
    pub target_id: String,
    /// The human-readable node label the banner / card names.
    pub node_label: String,
    /// The opaque affected-file label the banner / card / notice names; never a raw
    /// URL.
    pub file_label: String,
    /// How the surface relates to canonical source.
    pub sync_class: SourceSyncClass,
    /// The round-trip capability the surface claims for this target.
    pub round_trip: RoundTripCapabilityClass,
    /// The source-boundary class of the target's file.
    pub boundary: M5SourceBoundaryClass,
    /// How a protected target gates a write.
    pub protected_posture: ProtectedPathPosture,
    /// Whether the surface holds an unsaved visual edit.
    pub has_unsaved_visual_edit: bool,
    /// The opaque source-span ref, present when the target maps to source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span_ref: Option<String>,
    /// A round-trip conflict, when one is open.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict: Option<M5RoundTripConflictClass>,
    /// An unsupported construct, when the target cannot round-trip.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsupported: Option<UnsupportedConstructReason>,
}

/// The resolved source-sync chip for a target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSourceSyncChip {
    /// The target identity — identical to the banner / card / notice.
    pub target_id: String,
    /// The source-sync class the chip discloses.
    pub sync_class: SourceSyncClass,
    /// The chip state.
    pub chip_state: M5SourceSyncChipState,
    /// The recovery route offered, consistent with the sync class.
    pub recovery_route: M5SyncRecoveryRoute,
    /// The open-source action is offered when the target maps to source.
    pub open_source_action_available: bool,
    /// The open-diff action is offered when there is a diff to compare.
    pub open_diff_action_available: bool,
    /// A precise chip label.
    pub chip_label: String,
}

/// The resolved round-trip conflict banner. Present only when a conflict is open.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedConflictBanner {
    /// The target identity.
    pub target_id: String,
    /// The affected node the banner names.
    pub node_label: String,
    /// The affected file the banner names.
    pub file_label: String,
    /// Why the conflict banner appeared.
    pub conflict_class: M5RoundTripConflictClass,
    /// The conflict resolution route offered.
    pub resolution_route: M5ConflictResolutionRoute,
    /// The exact source-first fallback offered instead of a best-effort writeback.
    pub source_first_fallback: M5SourceFirstFallback,
    /// The refresh (reload-source) action is offered.
    pub refresh_action_available: bool,
    /// The compare (open-diff) action is offered.
    pub compare_action_available: bool,
    /// The conflict never collapses into a silent writeback; always `true`.
    pub never_silent_writeback: bool,
    /// The edit's selection context is preserved across the conflict; always
    /// `true`.
    pub preserves_selection_context: bool,
    /// A precise banner label.
    pub banner_label: String,
}

/// The resolved unsupported-construct card. Present only when the target cannot
/// round-trip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedUnsupportedCard {
    /// The target identity.
    pub target_id: String,
    /// The affected node the card names.
    pub node_label: String,
    /// The affected file the card names.
    pub file_label: String,
    /// Why the target is unsupported.
    pub reason: UnsupportedConstructReason,
    /// The exact source-first fallback offered instead of a best-effort writeback.
    pub source_first_fallback: M5SourceFirstFallback,
    /// The open-source action is offered when the target maps to source.
    pub open_source_action_available: bool,
    /// The selection context is preserved across the degrade; always `true`.
    pub preserves_selection_context: bool,
    /// A precise card label.
    pub card_label: String,
}

/// The resolved generated-or-protected-file boundary notice. Present only when the
/// target's file is not author-owned writable source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBoundaryNotice {
    /// The target identity.
    pub target_id: String,
    /// The affected file the notice names.
    pub file_label: String,
    /// The source-boundary class.
    pub boundary_class: M5SourceBoundaryClass,
    /// Whether the designer may write to this boundary at all.
    pub designer_write_permitted: bool,
    /// Whether a write must route through an owner / managed-file flow.
    pub requires_owner_flow: bool,
    /// The designer edit never silently widens into this zone; always `true`.
    pub refuses_silent_widening: bool,
    /// A precise notice label.
    pub notice_label: String,
}

/// The resolved round-trip truth shared across chip, banner, card, and notice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRoundTripStatus {
    /// The stable target identity.
    pub target_id: String,
    /// The affected node label.
    pub node_label: String,
    /// The affected file label.
    pub file_label: String,
    /// The source-sync class disclosed.
    pub sync_class: SourceSyncClass,
    /// The round-trip capability claimed.
    pub round_trip: RoundTripCapabilityClass,
    /// The source-boundary class.
    pub boundary: M5SourceBoundaryClass,
    /// The honest write authority the source model supports.
    pub write_authority: M5WriteAuthority,
    /// The resolved source-sync chip.
    pub chip: M5ResolvedSourceSyncChip,
    /// The exact source-first fallback route, when the surface narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_first_fallback: Option<M5SourceFirstFallback>,
    /// The typed downgrade trigger, when the surface narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5VisualDesignerDowngradeTrigger>,
    /// The resolved conflict banner, when a conflict is open.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_banner: Option<M5ResolvedConflictBanner>,
    /// The resolved unsupported-construct card, when the target cannot round-trip.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsupported_card: Option<M5ResolvedUnsupportedCard>,
    /// The resolved boundary notice, when the file is not author-owned source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_notice: Option<M5ResolvedBoundaryNotice>,
    /// A visual edit never becomes a silent writeback; always `true`.
    pub no_silent_writeback: bool,
}

impl M5ResolvedRoundTripStatus {
    /// True when the target identity is identical across the chip, the conflict
    /// banner, the unsupported card, and the boundary notice.
    pub fn identity_consistent(&self) -> bool {
        let banner_ok = match &self.conflict_banner {
            Some(banner) => banner.target_id == self.target_id,
            None => true,
        };
        let card_ok = match &self.unsupported_card {
            Some(card) => card.target_id == self.target_id,
            None => true,
        };
        let notice_ok = match &self.boundary_notice {
            Some(notice) => notice.target_id == self.target_id,
            None => true,
        };
        self.chip.target_id == self.target_id && banner_ok && card_ok && notice_ok
    }

    /// True when a visual edit under this status writes back to source.
    pub fn writes_back(&self) -> bool {
        self.write_authority.writes_back()
    }

    /// True when this status narrows a round-trip that could otherwise write back —
    /// a real degrade from the surface's own capability, not a baseline read-only
    /// surface.
    pub fn is_narrowed(&self) -> bool {
        self.round_trip.writes_back_to_source() && self.write_authority.is_narrowed()
    }

    /// True when a hard block is present (a conflict, an unsupported construct, or a
    /// non-writable boundary).
    pub fn has_hard_block(&self) -> bool {
        self.conflict_banner.is_some()
            || self.unsupported_card.is_some()
            || self
                .boundary_notice
                .as_ref()
                .is_some_and(|notice| !notice.designer_write_permitted)
    }

    /// AC1: an unsupported construct, an open conflict, a generated section, or a
    /// protected file can never be silently normalized into a write path. Whenever
    /// a hard block is present the write authority is narrowed, never writable.
    pub fn refuses_silent_normalization(&self) -> bool {
        !self.has_hard_block() || self.write_authority.is_narrowed()
    }

    /// AC2: when round-trip support drops the surface names an exact source-first
    /// fallback instead of a best-effort writeback.
    pub fn offers_source_first_fallback(&self) -> bool {
        self.write_authority.writes_back() || self.source_first_fallback.is_some()
    }

    /// AC3: every narrowing or read-only outcome names a typed downgrade trigger so
    /// support and release packets can explain why the surface narrowed.
    pub fn narrowing_is_explained(&self) -> bool {
        !self.is_narrowed() || self.downgrade_trigger.is_some()
    }
}

/// Errors returned by [`resolve_round_trip_status`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5RoundTripResolutionError {
    /// The target identity was empty.
    EmptyTargetId,
    /// The node label was empty.
    EmptyNodeLabel,
    /// The file label was empty.
    EmptyFileLabel,
    /// A round-trip that writes back to source declared no source span.
    MissingSpanForSourceRoundTrip,
    /// A runtime-only-no-source surface carried a source span, contradicting its
    /// claim to have no saved-source backing.
    ContradictoryRuntimeSpan,
    /// A label carried forbidden material.
    ForbiddenMaterial,
}

impl M5RoundTripResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyTargetId => "empty_target_id",
            Self::EmptyNodeLabel => "empty_node_label",
            Self::EmptyFileLabel => "empty_file_label",
            Self::MissingSpanForSourceRoundTrip => "missing_span_for_source_round_trip",
            Self::ContradictoryRuntimeSpan => "contradictory_runtime_span",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5RoundTripResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "round-trip resolution error: {}", self.as_str())
    }
}

impl Error for M5RoundTripResolutionError {}

/// Resolves one designer target's round-trip situation into its shared source-sync
/// chip, conflict banner or unsupported-construct card, boundary notice, and honest
/// write authority.
///
/// The resolved status carries one target identity on every component so the user
/// keeps their place across the chip, banner, card, and notice. An unsupported
/// construct, an open conflict, a generated section, or a protected file can never
/// be silently normalized into a write path (AC1): the write authority is narrowed
/// to a source-first fallback or read-only whenever a hard block is present. When
/// round-trip support drops the resolver names the exact source-first fallback
/// route instead of a best-effort writeback (AC2). Every narrowing carries a typed
/// downgrade trigger and a precise label so support can explain it (AC3).
pub fn resolve_round_trip_status(
    input: &M5RoundTripStatusInput,
) -> Result<M5ResolvedRoundTripStatus, M5RoundTripResolutionError> {
    if input.target_id.trim().is_empty() {
        return Err(M5RoundTripResolutionError::EmptyTargetId);
    }
    if input.node_label.trim().is_empty() {
        return Err(M5RoundTripResolutionError::EmptyNodeLabel);
    }
    if input.file_label.trim().is_empty() {
        return Err(M5RoundTripResolutionError::EmptyFileLabel);
    }
    if label_is_forbidden(&input.node_label) || label_is_forbidden(&input.file_label) {
        return Err(M5RoundTripResolutionError::ForbiddenMaterial);
    }

    let span_present = input
        .source_span_ref
        .as_ref()
        .is_some_and(|span| !span.trim().is_empty());

    // A round-trip that writes back to source must name a span; you cannot claim a
    // write-back you have no span for.
    if input.round_trip.writes_back_to_source() && !span_present {
        return Err(M5RoundTripResolutionError::MissingSpanForSourceRoundTrip);
    }
    // A runtime-only-no-source surface must not carry a saved-source span.
    if input.sync_class.is_runtime_only() && span_present {
        return Err(M5RoundTripResolutionError::ContradictoryRuntimeSpan);
    }

    let write_authority = resolve_write_authority(input);
    let downgrade_trigger = resolve_downgrade_trigger(input);
    let source_first_fallback = resolve_source_first_fallback(input, write_authority);

    let open_source_available = span_present;
    let has_diff = input.has_unsaved_visual_edit
        || input.conflict.is_some()
        || matches!(
            input.sync_class,
            SourceSyncClass::DriftedFromSource | SourceSyncClass::PendingRebuild
        );

    let chip_state = resolve_chip_state(input);
    let chip = M5ResolvedSourceSyncChip {
        target_id: input.target_id.clone(),
        sync_class: input.sync_class,
        chip_state,
        recovery_route: recovery_route_for(input.sync_class),
        open_source_action_available: open_source_available,
        open_diff_action_available: has_diff,
        chip_label: chip_state.label().to_owned(),
    };

    let conflict_banner = input
        .conflict
        .map(|conflict_class| M5ResolvedConflictBanner {
            target_id: input.target_id.clone(),
            node_label: input.node_label.clone(),
            file_label: input.file_label.clone(),
            conflict_class,
            resolution_route: resolution_route_for(conflict_class),
            source_first_fallback: source_first_fallback
                .unwrap_or(M5SourceFirstFallback::OpenSourceEditDirectly),
            refresh_action_available: true,
            compare_action_available: true,
            never_silent_writeback: true,
            preserves_selection_context: true,
            banner_label: conflict_banner_label(conflict_class).to_owned(),
        });

    let unsupported_card = input.unsupported.map(|reason| M5ResolvedUnsupportedCard {
        target_id: input.target_id.clone(),
        node_label: input.node_label.clone(),
        file_label: input.file_label.clone(),
        reason,
        source_first_fallback: source_first_fallback
            .unwrap_or(M5SourceFirstFallback::OpenSourceEditDirectly),
        open_source_action_available: open_source_available,
        preserves_selection_context: true,
        card_label: unsupported_card_label(reason).to_owned(),
    });

    let boundary_notice = if input.boundary == M5SourceBoundaryClass::AuthorOwned {
        None
    } else {
        Some(M5ResolvedBoundaryNotice {
            target_id: input.target_id.clone(),
            file_label: input.file_label.clone(),
            boundary_class: input.boundary,
            designer_write_permitted: input.boundary.permits_designer_write()
                && input.protected_posture != ProtectedPathPosture::ProtectedBlocked,
            requires_owner_flow: input.boundary.requires_owner_flow(),
            refuses_silent_widening: true,
            notice_label: input.boundary.notice_label().to_owned(),
        })
    };

    Ok(M5ResolvedRoundTripStatus {
        target_id: input.target_id.clone(),
        node_label: input.node_label.clone(),
        file_label: input.file_label.clone(),
        sync_class: input.sync_class,
        round_trip: input.round_trip,
        boundary: input.boundary,
        write_authority,
        chip,
        source_first_fallback,
        downgrade_trigger,
        conflict_banner,
        unsupported_card,
        boundary_notice,
        no_silent_writeback: true,
    })
}

/// Derives the source-sync chip state from the situation. A conflict and an
/// unsupported construct take precedence over sync / unsaved state so the chip
/// always names the most severe honest state.
fn resolve_chip_state(input: &M5RoundTripStatusInput) -> M5SourceSyncChipState {
    if input.conflict.is_some() {
        M5SourceSyncChipState::Conflict
    } else if input.unsupported.is_some() {
        M5SourceSyncChipState::UnsupportedConstruct
    } else if matches!(
        input.sync_class,
        SourceSyncClass::DriftedFromSource
            | SourceSyncClass::PendingRebuild
            | SourceSyncClass::RuntimeOnlyNoSource
            | SourceSyncClass::UnidentifiedSourceSync
    ) {
        M5SourceSyncChipState::NeedsRefresh
    } else if input.has_unsaved_visual_edit {
        M5SourceSyncChipState::Unsaved
    } else {
        M5SourceSyncChipState::InSync
    }
}

/// The recovery route consistent with a sync class. Mirrors the frozen
/// `SourceSyncChipDescriptor::is_honest` mapping so the chip never points at a
/// route the sync class does not support.
fn recovery_route_for(sync_class: SourceSyncClass) -> M5SyncRecoveryRoute {
    match sync_class {
        SourceSyncClass::InSyncFromSource => M5SyncRecoveryRoute::NoneInSync,
        SourceSyncClass::PendingRebuild | SourceSyncClass::DriftedFromSource => {
            M5SyncRecoveryRoute::RebuildFromSource
        }
        SourceSyncClass::RuntimeOnlyNoSource => M5SyncRecoveryRoute::ReattachRuntime,
        SourceSyncClass::UnidentifiedSourceSync => M5SyncRecoveryRoute::InspectOnlyNoRecovery,
    }
}

/// Derives the honest write authority from the situation.
fn resolve_write_authority(input: &M5RoundTripStatusInput) -> M5WriteAuthority {
    // Hard read-only gates: the surface can take no write at all.
    if input.round_trip.is_inspect_only()
        || input.protected_posture == ProtectedPathPosture::ProtectedBlocked
        || input.boundary.requires_owner_flow()
        || matches!(
            input.sync_class,
            SourceSyncClass::RuntimeOnlyNoSource | SourceSyncClass::UnidentifiedSourceSync
        )
    {
        return M5WriteAuthority::ReadOnly;
    }
    // Source-first fallback gates: a write path exists in principle but round-trip
    // support dropped, so fall back to editing source directly.
    if input.conflict.is_some()
        || input.unsupported.is_some()
        || input.round_trip == RoundTripCapabilityClass::SourceOnlyFallback
        || matches!(
            input.sync_class,
            SourceSyncClass::DriftedFromSource | SourceSyncClass::PendingRebuild
        )
    {
        return M5WriteAuthority::SourceOnlyFallback;
    }
    // Writable-with-review gates: an approximate round-trip, a protected path that
    // permits a reviewed apply, or a mixed managed region.
    if input.round_trip == RoundTripCapabilityClass::ApproximateSourceRoundTrip
        || matches!(
            input.protected_posture,
            ProtectedPathPosture::ProtectedReviewRequired
                | ProtectedPathPosture::ProtectedOwnerApprovalRequired
        )
        || input.boundary == M5SourceBoundaryClass::MixedManagedRegion
    {
        return M5WriteAuthority::WritableWithReview;
    }
    // Fully writable: an exact round-trip, in sync, author-owned, unprotected.
    M5WriteAuthority::Writable
}

/// Derives the typed downgrade trigger, when any applies. Named so support and
/// release packets can explain why the surface narrowed.
fn resolve_downgrade_trigger(
    input: &M5RoundTripStatusInput,
) -> Option<M5VisualDesignerDowngradeTrigger> {
    if input.conflict.is_some() {
        return Some(M5VisualDesignerDowngradeTrigger::RoundTripConflictOpen);
    }
    if input.unsupported.is_some() {
        return Some(M5VisualDesignerDowngradeTrigger::UnsupportedConstruct);
    }
    if input.protected_posture == ProtectedPathPosture::ProtectedBlocked
        || matches!(
            input.boundary,
            M5SourceBoundaryClass::GeneratedManaged | M5SourceBoundaryClass::ProtectedReadOnly
        )
    {
        return Some(M5VisualDesignerDowngradeTrigger::ProtectedPathBlocked);
    }
    if input.boundary == M5SourceBoundaryClass::ExternalVendored
        || input.round_trip == RoundTripCapabilityClass::NoRoundTrip
    {
        return Some(M5VisualDesignerDowngradeTrigger::UnmappedSource);
    }
    if input.sync_class == SourceSyncClass::RuntimeOnlyNoSource {
        return Some(M5VisualDesignerDowngradeTrigger::RuntimeUnavailable);
    }
    if input.sync_class == SourceSyncClass::UnidentifiedSourceSync {
        return Some(M5VisualDesignerDowngradeTrigger::UnidentifiedPosture);
    }
    if matches!(
        input.sync_class,
        SourceSyncClass::DriftedFromSource | SourceSyncClass::PendingRebuild
    ) {
        return Some(M5VisualDesignerDowngradeTrigger::DriftedFromSource);
    }
    None
}

/// Derives the exact source-first fallback route, when the surface does not write
/// back. A writable surface names no fallback.
fn resolve_source_first_fallback(
    input: &M5RoundTripStatusInput,
    write_authority: M5WriteAuthority,
) -> Option<M5SourceFirstFallback> {
    if write_authority.writes_back() {
        return None;
    }
    if let Some(conflict) = input.conflict {
        return Some(match conflict {
            M5RoundTripConflictClass::SourceChangedUnderEdit
            | M5RoundTripConflictClass::ConcurrentExternalEdit => {
                M5SourceFirstFallback::ReloadSourceThenReapply
            }
            M5RoundTripConflictClass::LossyTransformRefused => {
                M5SourceFirstFallback::KeepSourceDiscardVisual
            }
            M5RoundTripConflictClass::GeneratedFileProtected => {
                M5SourceFirstFallback::OpenManagedFileOwnerFlow
            }
            M5RoundTripConflictClass::AmbiguousMapping => {
                M5SourceFirstFallback::OpenSourceEditDirectly
            }
        });
    }
    if input.boundary.requires_owner_flow()
        || input.protected_posture == ProtectedPathPosture::ProtectedBlocked
    {
        return Some(M5SourceFirstFallback::OpenManagedFileOwnerFlow);
    }
    if input.round_trip.is_inspect_only() || input.sync_class.is_runtime_only() {
        return Some(M5SourceFirstFallback::InspectOnlyNoWrite);
    }
    // An unsupported construct, a drift, or a source-only round-trip falls back to
    // editing the source directly.
    Some(M5SourceFirstFallback::OpenSourceEditDirectly)
}

/// The conflict resolution route consistent with a conflict class.
fn resolution_route_for(conflict: M5RoundTripConflictClass) -> M5ConflictResolutionRoute {
    match conflict {
        M5RoundTripConflictClass::SourceChangedUnderEdit => {
            M5ConflictResolutionRoute::ReloadSourceReapply
        }
        M5RoundTripConflictClass::ConcurrentExternalEdit
        | M5RoundTripConflictClass::AmbiguousMapping => {
            M5ConflictResolutionRoute::OpenSourceManualMerge
        }
        M5RoundTripConflictClass::GeneratedFileProtected => {
            M5ConflictResolutionRoute::InspectOnlyNoWrite
        }
        M5RoundTripConflictClass::LossyTransformRefused => {
            M5ConflictResolutionRoute::KeepSourceDiscardVisual
        }
    }
}

/// A precise, non-generic conflict-banner label per conflict class.
fn conflict_banner_label(conflict: M5RoundTripConflictClass) -> &'static str {
    match conflict {
        M5RoundTripConflictClass::SourceChangedUnderEdit => {
            "Source changed under this edit — reload source, then re-apply"
        }
        M5RoundTripConflictClass::GeneratedFileProtected => {
            "Target is a generated / protected file — the visual edit cannot write here"
        }
        M5RoundTripConflictClass::AmbiguousMapping => {
            "Source mapping is ambiguous — open source to merge rather than guess a span"
        }
        M5RoundTripConflictClass::ConcurrentExternalEdit => {
            "A concurrent external edit touched this span — open source to merge"
        }
        M5RoundTripConflictClass::LossyTransformRefused => {
            "The transform would be lossy and was refused — keep source and discard the visual change"
        }
    }
}

/// A precise, non-generic unsupported-construct-card label per reason.
fn unsupported_card_label(reason: UnsupportedConstructReason) -> &'static str {
    match reason {
        UnsupportedConstructReason::DynamicBinding => {
            "Value is a runtime / dynamic binding — edit the source expression directly"
        }
        UnsupportedConstructReason::ConditionalOrLoopOrigin => {
            "Node originates in a conditional / loop — edit the source directly"
        }
        UnsupportedConstructReason::GeneratedOrExternalArtifact => {
            "Node is generated output or from an external library — no source span to write"
        }
        UnsupportedConstructReason::AmbiguousSourceMapping => {
            "Source mapping is ambiguous — a write could land on the wrong span"
        }
        UnsupportedConstructReason::LossyTransformRejected => {
            "The transform would be lossy and was rejected"
        }
        UnsupportedConstructReason::ProtectedPathBlocked => {
            "Target is a blocked protected path — the visual edit cannot write here"
        }
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs round-trip truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RoundTripStatusCase {
    /// The resolver input.
    pub input: M5RoundTripStatusInput,
    /// The resolved round-trip truth. Must equal
    /// `resolve_round_trip_status(&input)`.
    pub resolved: M5ResolvedRoundTripStatus,
}

impl M5RoundTripStatusCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5RoundTripStatusInput) -> Self {
        let resolved = resolve_round_trip_status(&input).expect("seed round-trip case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_round_trip_status(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one visual-design surface family bound to the
/// shared round-trip-honesty contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RoundTripSurfaceRow {
    /// The visual-design surface family.
    pub surface_family: M5VisualDesignSurfaceFamily,
    /// The claimed preview surface this row maps onto (reused vocabulary).
    pub preview_surface: PreviewSurface,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Source-sync classes this surface can disclose (must be non-empty).
    pub sync_classes: Vec<SourceSyncClass>,
    /// Source-boundary classes this surface can encounter (must be non-empty).
    pub boundary_classes: Vec<M5SourceBoundaryClass>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5RoundTripExportField>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5VisualDesignerDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection.
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface.
    pub example_statuses: Vec<M5RoundTripStatusCase>,
    /// Hard invariant: this row never silently normalizes an unsupported construct.
    /// MUST be `false`.
    pub normalizes_unsupported_silently: bool,
    /// Hard invariant: this row never widens write scope without disclosure. MUST
    /// be `false`.
    pub widens_write_scope_without_disclosure: bool,
    /// Hard invariant: this row never hides a read-only narrowing. MUST be `false`.
    pub hides_read_only_narrowing: bool,
    /// Hard invariant: this row never invents a private round-trip grammar. MUST be
    /// `false`.
    pub invents_private_round_trip_grammar: bool,
}

impl M5RoundTripSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RoundTripExportField> =
            self.export_fields.iter().copied().collect();
        M5RoundTripExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.normalizes_unsupported_silently
            && !self.widens_write_scope_without_disclosure
            && !self.hides_read_only_narrowing
            && !self.invents_private_round_trip_grammar
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RoundTripVocabularySet {
    /// Visual-design surface-family tokens (reused).
    pub surface_families: Vec<String>,
    /// Source-sync-class tokens (reused).
    pub sync_classes: Vec<String>,
    /// Source-sync chip-state tokens.
    pub chip_states: Vec<String>,
    /// Source-boundary-class tokens.
    pub boundary_classes: Vec<String>,
    /// Write-authority tokens.
    pub write_authorities: Vec<String>,
    /// Source-first-fallback tokens.
    pub source_first_fallbacks: Vec<String>,
    /// Round-trip conflict-class tokens (reused).
    pub conflict_classes: Vec<String>,
    /// Unsupported-construct-reason tokens (reused).
    pub unsupported_reasons: Vec<String>,
    /// Downgrade-trigger tokens (reused).
    pub downgrade_triggers: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
}

impl M5RoundTripVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5VisualDesignSurfaceFamily::ALL, |v| v.as_str()),
            sync_classes: tokens(&SYNC_CLASS_ALL, |v| v.as_str()),
            chip_states: tokens(&M5SourceSyncChipState::ALL, |v| v.as_str()),
            boundary_classes: tokens(&M5SourceBoundaryClass::ALL, |v| v.as_str()),
            write_authorities: tokens(&M5WriteAuthority::ALL, |v| v.as_str()),
            source_first_fallbacks: tokens(&M5SourceFirstFallback::ALL, |v| v.as_str()),
            conflict_classes: tokens(&CONFLICT_CLASS_ALL, |v| v.as_str()),
            unsupported_reasons: tokens(&UNSUPPORTED_REASON_ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&DOWNGRADE_TRIGGER_ALL, |v| v.as_str()),
            export_fields: tokens(&M5RoundTripExportField::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The source-sync classes this primitive discloses, in a stable order. The frozen
/// [`SourceSyncClass`] enum is a pure token set, so the order is pinned here.
const SYNC_CLASS_ALL: [SourceSyncClass; 5] = [
    SourceSyncClass::InSyncFromSource,
    SourceSyncClass::PendingRebuild,
    SourceSyncClass::DriftedFromSource,
    SourceSyncClass::RuntimeOnlyNoSource,
    SourceSyncClass::UnidentifiedSourceSync,
];

/// The round-trip conflict classes this primitive handles, in a stable order.
const CONFLICT_CLASS_ALL: [M5RoundTripConflictClass; 5] = [
    M5RoundTripConflictClass::SourceChangedUnderEdit,
    M5RoundTripConflictClass::GeneratedFileProtected,
    M5RoundTripConflictClass::AmbiguousMapping,
    M5RoundTripConflictClass::ConcurrentExternalEdit,
    M5RoundTripConflictClass::LossyTransformRefused,
];

/// The unsupported-construct reasons this primitive handles, in a stable order.
const UNSUPPORTED_REASON_ALL: [UnsupportedConstructReason; 6] = [
    UnsupportedConstructReason::DynamicBinding,
    UnsupportedConstructReason::ConditionalOrLoopOrigin,
    UnsupportedConstructReason::GeneratedOrExternalArtifact,
    UnsupportedConstructReason::AmbiguousSourceMapping,
    UnsupportedConstructReason::LossyTransformRejected,
    UnsupportedConstructReason::ProtectedPathBlocked,
];

/// The downgrade triggers this primitive emits, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5VisualDesignerDowngradeTrigger; 7] = [
    M5VisualDesignerDowngradeTrigger::DriftedFromSource,
    M5VisualDesignerDowngradeTrigger::UnmappedSource,
    M5VisualDesignerDowngradeTrigger::RuntimeUnavailable,
    M5VisualDesignerDowngradeTrigger::ProtectedPathBlocked,
    M5VisualDesignerDowngradeTrigger::UnsupportedConstruct,
    M5VisualDesignerDowngradeTrigger::RoundTripConflictOpen,
    M5VisualDesignerDowngradeTrigger::UnidentifiedPosture,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RoundTripGovernanceReview {
    /// One primitive carries the chip, banner, card, and notice on every surface.
    pub one_primitive_carries_chip_banner_card_notice: bool,
    /// Unsupported constructs and conflicts are never silently normalized.
    pub unsupported_and_conflicts_never_silently_normalized: bool,
    /// A source-first fallback is named whenever round-trip support drops.
    pub source_first_fallback_named_when_round_trip_drops: bool,
    /// Generated / protected boundaries block a silent write-scope widening.
    pub generated_and_protected_boundaries_block_silent_widening: bool,
    /// Narrowing and read-only outcomes are explained in the support export.
    pub narrowing_and_read_only_explained_in_support_export: bool,
    /// No surface invents a second round-trip grammar.
    pub no_surface_invents_second_round_trip_grammar: bool,
    /// Later M5 rows cannot invent parallel round-trip vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RoundTripConsumerProjection {
    /// Desktop / preview / browser / framework / shell / support surfaces all
    /// consume the shared primitive.
    pub visual_surfaces_consume_shared_round_trip_primitive: bool,
    /// The round-trip resolver reads a single canonical model.
    pub resolver_reads_single_round_trip_model: bool,
    /// The chip reads a single canonical source-sync source.
    pub chip_reads_single_sync_source: bool,
    /// Support / export reads a single canonical round-trip source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the round-trip-honesty primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RoundTripReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting round-trip audit.
    pub round_trip_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RoundTripHonestyPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RoundTripHonestyPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5RoundTripSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RoundTripVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RoundTripGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RoundTripConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5RoundTripReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 round-trip-honesty-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RoundTripHonestyPacket {
    /// Record kind; must equal [`M5_ROUND_TRIP_HONESTY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ROUND_TRIP_HONESTY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5RoundTripSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RoundTripVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RoundTripGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RoundTripConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5RoundTripReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RoundTripHonestyPacket {
    /// Builds an M5 round-trip-honesty packet from stable-lane input.
    pub fn new(input: M5RoundTripHonestyPacketInput) -> Self {
        Self {
            record_kind: M5_ROUND_TRIP_HONESTY_RECORD_KIND.to_owned(),
            schema_version: M5_ROUND_TRIP_HONESTY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 round-trip-honesty invariants.
    pub fn validate(&self) -> Vec<M5RoundTripHonestyViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ROUND_TRIP_HONESTY_RECORD_KIND {
            violations.push(M5RoundTripHonestyViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ROUND_TRIP_HONESTY_SCHEMA_VERSION {
            violations.push(M5RoundTripHonestyViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RoundTripHonestyViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 round-trip honesty packet serializes"),
        ) {
            violations.push(M5RoundTripHonestyViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 round-trip honesty packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,preview_surface,owner,sync_classes,boundary_classes,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.preview_surface.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.sync_classes, |v| v.as_str()),
                join_tokens(&row.boundary_classes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_statuses.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Source-Round-Trip Honesty Primitive: Sync Chip, Conflict Banner, Unsupported Card, and Boundary Notice\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Visual-design surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5VisualDesignSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Chip states: {}\n",
            self.vocabulary_set.chip_states.join(", ")
        ));
        out.push_str(&format!(
            "- Write authorities: {}\n",
            self.vocabulary_set.write_authorities.join(", ")
        ));
        out.push_str(&format!(
            "- Boundary classes: {}\n",
            self.vocabulary_set.boundary_classes.join(", ")
        ));
        out.push_str("\n## Visual-design surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.surface_family.label(),
                row.preview_surface.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked statuses: {}\n",
                row.example_statuses.len()
            ));
            for case in &row.example_statuses {
                out.push_str(&format!(
                    "    - `{}` → node `{}` chip `{}`, authority `{}`, boundary `{}`\n",
                    case.resolved.target_id,
                    case.resolved.node_label,
                    case.resolved.chip.chip_state.as_str(),
                    case.resolved.write_authority.as_str(),
                    case.resolved.boundary.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 round-trip-honesty export.
#[derive(Debug)]
pub enum M5RoundTripHonestyArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RoundTripHonestyViolation>),
}

impl fmt::Display for M5RoundTripHonestyArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 round-trip honesty export parse failed: {error}"
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
                    "m5 round-trip honesty export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RoundTripHonestyArtifactError {}

/// Validation failures emitted by [`M5RoundTripHonestyPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RoundTripHonestyViolation {
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
    /// A required visual-design surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no source-sync classes.
    SyncClassMissing,
    /// A surface row declares no source-boundary classes.
    BoundaryClassMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked status cases.
    ExampleStatusMissing,
    /// A worked status case does not match a fresh resolve of its input.
    ExampleStatusDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked status proves an unsupported construct, conflict, or protected
    /// boundary refused a silent write (AC1).
    SilentNormalizationUnproven,
    /// No worked status proves an exact source-first fallback named when round-trip
    /// support dropped (AC2).
    SourceFirstFallbackUnproven,
    /// No worked status proves a narrowing / read-only outcome explained with a
    /// downgrade trigger (AC3).
    NarrowingExplanationUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5RoundTripHonestyViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::SyncClassMissing => "sync_class_missing",
            Self::BoundaryClassMissing => "boundary_class_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleStatusMissing => "example_status_missing",
            Self::ExampleStatusDrift => "example_status_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::SilentNormalizationUnproven => "silent_normalization_unproven",
            Self::SourceFirstFallbackUnproven => "source_first_fallback_unproven",
            Self::NarrowingExplanationUnproven => "narrowing_explanation_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 round-trip-honesty export.
pub fn current_stable_m5_round_trip_honesty_export(
) -> Result<M5RoundTripHonestyPacket, M5RoundTripHonestyArtifactError> {
    let packet: M5RoundTripHonestyPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-source-round-trip-honesty-proof/support_export.json"
    )))
    .map_err(M5RoundTripHonestyArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RoundTripHonestyArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5RoundTripHonestyPacket,
    violations: &mut Vec<M5RoundTripHonestyViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ROUND_TRIP_SCHEMA_REF,
        M5_ROUND_TRIP_DOC_REF,
        M5_ROUND_TRIP_COMPONENT_MATRIX_REF,
        M5_ROUND_TRIP_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RoundTripHonestyViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5RoundTripHonestyPacket,
    violations: &mut Vec<M5RoundTripHonestyViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5RoundTripHonestyViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5RoundTripHonestyPacket,
    violations: &mut Vec<M5RoundTripHonestyViolation>,
) {
    let present: BTreeSet<M5VisualDesignSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5VisualDesignSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5RoundTripHonestyViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5RoundTripHonestyViolation::SurfaceRowIncomplete);
        }
        if row.sync_classes.is_empty() {
            violations.push(M5RoundTripHonestyViolation::SyncClassMissing);
        }
        if row.boundary_classes.is_empty() {
            violations.push(M5RoundTripHonestyViolation::BoundaryClassMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5RoundTripHonestyViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5RoundTripHonestyViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5RoundTripHonestyViolation::ConsumerSurfacesMissing);
        }
        if row.example_statuses.is_empty() {
            violations.push(M5RoundTripHonestyViolation::ExampleStatusMissing);
        }
        if row
            .example_statuses
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5RoundTripHonestyViolation::ExampleStatusDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5RoundTripHonestyViolation::SurfaceInvariantViolated);
        }
    }
}

/// The three acceptance criteria must each be demonstrated by at least one worked
/// status across the matrix: an unsupported construct / conflict / protected
/// boundary refusing a silent write (AC1), an exact source-first fallback named
/// when round-trip support dropped (AC2), and a narrowing / read-only outcome
/// explained with a downgrade trigger (AC3). The stronger per-status invariants are
/// also enforced across every case.
fn validate_acceptance_criteria_covered(
    packet: &M5RoundTripHonestyPacket,
    violations: &mut Vec<M5RoundTripHonestyViolation>,
) {
    let cases: Vec<&M5ResolvedRoundTripStatus> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_statuses.iter().map(|case| &case.resolved))
        .collect();

    let silent_norm_proven = cases
        .iter()
        .any(|resolved| resolved.has_hard_block() && resolved.refuses_silent_normalization())
        && cases
            .iter()
            .all(|resolved| resolved.refuses_silent_normalization());
    if !silent_norm_proven {
        violations.push(M5RoundTripHonestyViolation::SilentNormalizationUnproven);
    }

    let fallback_proven = cases
        .iter()
        .any(|resolved| !resolved.writes_back() && resolved.source_first_fallback.is_some())
        && cases
            .iter()
            .all(|resolved| resolved.offers_source_first_fallback());
    if !fallback_proven {
        violations.push(M5RoundTripHonestyViolation::SourceFirstFallbackUnproven);
    }

    let narrowing_proven = cases
        .iter()
        .any(|resolved| resolved.is_narrowed() && resolved.downgrade_trigger.is_some())
        && cases
            .iter()
            .all(|resolved| resolved.narrowing_is_explained() && resolved.identity_consistent());
    if !narrowing_proven {
        violations.push(M5RoundTripHonestyViolation::NarrowingExplanationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5RoundTripHonestyPacket,
    violations: &mut Vec<M5RoundTripHonestyViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_chip_banner_card_notice,
        review.unsupported_and_conflicts_never_silently_normalized,
        review.source_first_fallback_named_when_round_trip_drops,
        review.generated_and_protected_boundaries_block_silent_widening,
        review.narrowing_and_read_only_explained_in_support_export,
        review.no_surface_invents_second_round_trip_grammar,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5RoundTripHonestyViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RoundTripHonestyPacket,
    violations: &mut Vec<M5RoundTripHonestyViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.visual_surfaces_consume_shared_round_trip_primitive,
        projection.resolver_reads_single_round_trip_model,
        projection.chip_reads_single_sync_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5RoundTripHonestyViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5RoundTripHonestyPacket,
    violations: &mut Vec<M5RoundTripHonestyViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.round_trip_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RoundTripHonestyViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
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

/// True when a label carries obviously forbidden material.
fn label_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => label_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in M5 round-trip-honesty packet. This is the one
/// source of truth shared by the tests, the example dump, and the on-disk support
/// export so all three stay byte-aligned.
pub fn seeded_m5_round_trip_honesty_packet() -> M5RoundTripHonestyPacket {
    M5RoundTripHonestyPacket::new(M5RoundTripHonestyPacketInput {
        packet_id: "m5-source-round-trip-honesty-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Source-Round-Trip Honesty Primitive: Sync Chip, Conflict Banner, Unsupported Card, and Boundary Notice"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5RoundTripVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-03T00:00:00Z".to_owned(),
    })
}

fn all_export_fields() -> Vec<M5RoundTripExportField> {
    M5RoundTripExportField::ALL.to_vec()
}

fn seeded_surface_rows() -> Vec<M5RoundTripSurfaceRow> {
    vec![
        // Desktop designer — an author-owned, in-sync, exact-round-trip element that
        // writes back cleanly (baseline writable), plus an unsaved variant.
        M5RoundTripSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::DesktopDesigner,
            preview_surface: PreviewSurface::VisualSurfaceMapping,
            owner_role: "Visual Designer Platform".to_owned(),
            scope_summary:
                "Desktop designer source-sync chip and round-trip status for an author-owned element"
                    .to_owned(),
            sync_classes: SYNC_CLASS_ALL.to_vec(),
            boundary_classes: M5SourceBoundaryClass::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::DriftedFromSource,
                M5VisualDesignerDowngradeTrigger::RoundTripConflictOpen,
            ],
            consumer_surfaces: vec!["product_designer".to_owned(), "support_export".to_owned()],
            source_contract_refs: vec![M5_ROUND_TRIP_COMPONENT_MATRIX_REF.to_owned()],
            example_statuses: vec![
                M5RoundTripStatusCase::resolved(M5RoundTripStatusInput {
                    target_id: "target:desktop:hero-heading:0001".to_owned(),
                    node_label: "HeroHeading".to_owned(),
                    file_label: "src/components/Hero.tsx".to_owned(),
                    sync_class: SourceSyncClass::InSyncFromSource,
                    round_trip: RoundTripCapabilityClass::ExactSourceRoundTrip,
                    boundary: M5SourceBoundaryClass::AuthorOwned,
                    protected_posture: ProtectedPathPosture::Unprotected,
                    has_unsaved_visual_edit: false,
                    source_span_ref: Some("span:desktop:hero-heading".to_owned()),
                    conflict: None,
                    unsupported: None,
                }),
                M5RoundTripStatusCase::resolved(M5RoundTripStatusInput {
                    target_id: "target:desktop:hero-cta:0002".to_owned(),
                    node_label: "HeroCta".to_owned(),
                    file_label: "src/components/Hero.tsx".to_owned(),
                    sync_class: SourceSyncClass::InSyncFromSource,
                    round_trip: RoundTripCapabilityClass::ExactSourceRoundTrip,
                    boundary: M5SourceBoundaryClass::AuthorOwned,
                    protected_posture: ProtectedPathPosture::Unprotected,
                    has_unsaved_visual_edit: true,
                    source_span_ref: Some("span:desktop:hero-cta".to_owned()),
                    conflict: None,
                    unsupported: None,
                }),
            ],
            normalizes_unsupported_silently: false,
            widens_write_scope_without_disclosure: false,
            hides_read_only_narrowing: false,
            invents_private_round_trip_grammar: false,
        },
        // Source-first preview — a round-trip conflict (source changed under the
        // edit): the banner fires, write authority narrows to source-first fallback
        // (reload then re-apply). Proves AC1 + AC2 + AC3.
        M5RoundTripSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::SourceFirstPreview,
            preview_surface: PreviewSurface::SourceFirstFrameworkPreview,
            owner_role: "Source-First Preview".to_owned(),
            scope_summary:
                "Source-first preview conflict banner when canonical source changed under a visual edit"
                    .to_owned(),
            sync_classes: SYNC_CLASS_ALL.to_vec(),
            boundary_classes: M5SourceBoundaryClass::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::RoundTripConflictOpen,
                M5VisualDesignerDowngradeTrigger::DriftedFromSource,
            ],
            consumer_surfaces: vec!["preview_runtime".to_owned(), "support_export".to_owned()],
            source_contract_refs: vec![M5_ROUND_TRIP_VISUAL_EDIT_REF.to_owned()],
            example_statuses: vec![M5RoundTripStatusCase::resolved(M5RoundTripStatusInput {
                target_id: "target:preview:pricing-card:0001".to_owned(),
                node_label: "PricingCard".to_owned(),
                file_label: "src/routes/pricing.tsx".to_owned(),
                sync_class: SourceSyncClass::DriftedFromSource,
                round_trip: RoundTripCapabilityClass::ExactSourceRoundTrip,
                boundary: M5SourceBoundaryClass::AuthorOwned,
                protected_posture: ProtectedPathPosture::Unprotected,
                has_unsaved_visual_edit: true,
                source_span_ref: Some("span:preview:pricing-card".to_owned()),
                conflict: Some(M5RoundTripConflictClass::SourceChangedUnderEdit),
                unsupported: None,
            })],
            normalizes_unsupported_silently: false,
            widens_write_scope_without_disclosure: false,
            hides_read_only_narrowing: false,
            invents_private_round_trip_grammar: false,
        },
        // Browser-runtime inspector — a runtime-only-no-source node: inspect-only,
        // read-only, the chip needs a re-attach, and the fallback is inspect-only.
        M5RoundTripSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::BrowserRuntimeInspector,
            preview_surface: PreviewSurface::BrowserRuntimeInspection,
            owner_role: "Browser Runtime Inspector".to_owned(),
            scope_summary:
                "Browser-runtime inspector source-sync chip for a runtime-only node with no saved source"
                    .to_owned(),
            sync_classes: SYNC_CLASS_ALL.to_vec(),
            boundary_classes: M5SourceBoundaryClass::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::RuntimeUnavailable,
                M5VisualDesignerDowngradeTrigger::UnmappedSource,
            ],
            consumer_surfaces: vec!["browser_runtime".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: vec![M5_ROUND_TRIP_VISUAL_EDIT_REF.to_owned()],
            example_statuses: vec![M5RoundTripStatusCase::resolved(M5RoundTripStatusInput {
                target_id: "target:runtime:status-badge:0001".to_owned(),
                node_label: "StatusBadge".to_owned(),
                file_label: "runtime/dom/status-badge".to_owned(),
                sync_class: SourceSyncClass::RuntimeOnlyNoSource,
                round_trip: RoundTripCapabilityClass::InspectOnlyNoWrite,
                boundary: M5SourceBoundaryClass::AuthorOwned,
                protected_posture: ProtectedPathPosture::Unprotected,
                has_unsaved_visual_edit: false,
                source_span_ref: None,
                conflict: None,
                unsupported: None,
            })],
            normalizes_unsupported_silently: false,
            widens_write_scope_without_disclosure: false,
            hides_read_only_narrowing: false,
            invents_private_round_trip_grammar: false,
        },
        // Framework-pack preview — an unsupported construct (a dynamic binding) on an
        // approximate round-trip: the card fires, write authority narrows to a
        // source-first fallback (edit source directly). Proves AC1 + AC2 + AC3.
        M5RoundTripSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::FrameworkPackPreview,
            preview_surface: PreviewSurface::VisualEditTransform,
            owner_role: "Framework Packs".to_owned(),
            scope_summary:
                "Framework-pack preview unsupported-construct card for a dynamically bound value"
                    .to_owned(),
            sync_classes: SYNC_CLASS_ALL.to_vec(),
            boundary_classes: M5SourceBoundaryClass::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::UnsupportedConstruct,
                M5VisualDesignerDowngradeTrigger::ProtectedPathBlocked,
            ],
            consumer_surfaces: vec!["framework_pack".to_owned(), "support_export".to_owned()],
            source_contract_refs: vec![M5_ROUND_TRIP_COMPONENT_MATRIX_REF.to_owned()],
            example_statuses: vec![M5RoundTripStatusCase::resolved(M5RoundTripStatusInput {
                target_id: "target:framework:cart-count:0001".to_owned(),
                node_label: "CartCount".to_owned(),
                file_label: "src/widgets/CartBadge.dart".to_owned(),
                sync_class: SourceSyncClass::InSyncFromSource,
                round_trip: RoundTripCapabilityClass::ApproximateSourceRoundTrip,
                boundary: M5SourceBoundaryClass::AuthorOwned,
                protected_posture: ProtectedPathPosture::Unprotected,
                has_unsaved_visual_edit: false,
                source_span_ref: Some("span:framework:cart-count".to_owned()),
                conflict: None,
                unsupported: Some(UnsupportedConstructReason::DynamicBinding),
            })],
            normalizes_unsupported_silently: false,
            widens_write_scope_without_disclosure: false,
            hides_read_only_narrowing: false,
            invents_private_round_trip_grammar: false,
        },
        // Embedded shell designer — a generated / managed file target: even though
        // an exact round-trip is claimed, the generated-file boundary blocks the
        // write, so authority narrows to read-only with an owner-flow fallback. Proves
        // the generated-file boundary blocks a silent widening (AC1 + AC3).
        M5RoundTripSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::EmbeddedShellDesigner,
            preview_surface: PreviewSurface::EmbeddedWebviewPreview,
            owner_role: "Embedded Designer".to_owned(),
            scope_summary:
                "Embedded shell designer boundary notice for a generated / managed file target"
                    .to_owned(),
            sync_classes: SYNC_CLASS_ALL.to_vec(),
            boundary_classes: M5SourceBoundaryClass::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::ProtectedPathBlocked,
                M5VisualDesignerDowngradeTrigger::UnidentifiedPosture,
            ],
            consumer_surfaces: vec!["app_shell".to_owned(), "support_export".to_owned()],
            source_contract_refs: vec![M5_ROUND_TRIP_COMPONENT_MATRIX_REF.to_owned()],
            example_statuses: vec![M5RoundTripStatusCase::resolved(M5RoundTripStatusInput {
                target_id: "target:shell:generated-route:0001".to_owned(),
                node_label: "GeneratedRouteTable".to_owned(),
                file_label: "src/app/generated/routes.gen.ts".to_owned(),
                sync_class: SourceSyncClass::InSyncFromSource,
                round_trip: RoundTripCapabilityClass::ExactSourceRoundTrip,
                boundary: M5SourceBoundaryClass::GeneratedManaged,
                protected_posture: ProtectedPathPosture::Unprotected,
                has_unsaved_visual_edit: false,
                source_span_ref: Some("span:shell:generated-route".to_owned()),
                conflict: None,
                unsupported: None,
            })],
            normalizes_unsupported_silently: false,
            widens_write_scope_without_disclosure: false,
            hides_read_only_narrowing: false,
            invents_private_round_trip_grammar: false,
        },
        // Support-export replay — a drifted, exact-round-trip node on a mixed managed
        // region: the source-only fallback fires (edit source directly), the notice
        // discloses the mixed region, and the narrowing is explained. Proves AC1 + AC2
        // + AC3.
        M5RoundTripSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::SupportExportReplay,
            preview_surface: PreviewSurface::SupportExportProjection,
            owner_role: "Support Export".to_owned(),
            scope_summary:
                "Support-export replay of a captured round-trip status for a drifted node on a mixed managed region"
                    .to_owned(),
            sync_classes: SYNC_CLASS_ALL.to_vec(),
            boundary_classes: M5SourceBoundaryClass::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::DriftedFromSource,
                M5VisualDesignerDowngradeTrigger::UnmappedSource,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: vec![M5_ROUND_TRIP_ARTIFACT_REF.to_owned()],
            example_statuses: vec![M5RoundTripStatusCase::resolved(M5RoundTripStatusInput {
                target_id: "target:support:list-item:0001".to_owned(),
                node_label: "ListItemRow".to_owned(),
                file_label: "src/features/list/ListItem.tsx".to_owned(),
                sync_class: SourceSyncClass::DriftedFromSource,
                round_trip: RoundTripCapabilityClass::ExactSourceRoundTrip,
                boundary: M5SourceBoundaryClass::MixedManagedRegion,
                protected_posture: ProtectedPathPosture::ProtectedReviewRequired,
                has_unsaved_visual_edit: true,
                source_span_ref: Some("span:support:list-item".to_owned()),
                conflict: None,
                unsupported: None,
            })],
            normalizes_unsupported_silently: false,
            widens_write_scope_without_disclosure: false,
            hides_read_only_narrowing: false,
            invents_private_round_trip_grammar: false,
        },
    ]
}

fn seeded_governance_review() -> M5RoundTripGovernanceReview {
    M5RoundTripGovernanceReview {
        one_primitive_carries_chip_banner_card_notice: true,
        unsupported_and_conflicts_never_silently_normalized: true,
        source_first_fallback_named_when_round_trip_drops: true,
        generated_and_protected_boundaries_block_silent_widening: true,
        narrowing_and_read_only_explained_in_support_export: true,
        no_surface_invents_second_round_trip_grammar: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5RoundTripConsumerProjection {
    M5RoundTripConsumerProjection {
        visual_surfaces_consume_shared_round_trip_primitive: true,
        resolver_reads_single_round_trip_model: true,
        chip_reads_single_sync_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5RoundTripReleasePosture {
    M5RoundTripReleasePosture {
        release_packet_ref:
            "artifacts/release/m5-source-round-trip-honesty-proof/support_export.json".to_owned(),
        round_trip_audit_ref: "artifacts/components/m5-source-round-trip-honesty-primitive.md"
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    vec![
        M5_ROUND_TRIP_SCHEMA_REF.to_owned(),
        M5_ROUND_TRIP_DOC_REF.to_owned(),
        M5_ROUND_TRIP_COMPONENT_MATRIX_REF.to_owned(),
        M5_ROUND_TRIP_ARTIFACT_REF.to_owned(),
        M5_ROUND_TRIP_VISUAL_EDIT_REF.to_owned(),
    ]
}
