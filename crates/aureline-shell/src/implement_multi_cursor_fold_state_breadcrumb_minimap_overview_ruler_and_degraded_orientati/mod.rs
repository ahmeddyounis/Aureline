//! M5 orientation-aid packet: multi-cursor counts, fold-state summaries,
//! breadcrumb identity, minimap / overview-ruler markers, and degraded-
//! orientation truth across the claimed M5 editors, viewers, diffs, and
//! browser-runtime overlays.
//!
//! Aureline's switching promise depends on keyboard-first, recoverable, and
//! *orientable* interaction across every new M5 surface — editor, notebook,
//! data/API, preview, docs, review, runtime, and companion-adjacent panes. The
//! frozen keyboard-continuity matrix
//! [`crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix`]
//! pins those surfaces to their canonical interaction vocabulary and requires
//! that *orientation aids degrade honestly* — a collapse to no aids is a
//! downgrade, never a silent removal. This module discharges the orientation-aid
//! half of that contract: it takes the frozen [`OrientationAidClass`] posture
//! vocabulary and makes each multi-cursor count, fold summary, breadcrumb path,
//! minimap, and overview-ruler marker set **named, identity-aligned, and honestly
//! degraded** on the new M5 surfaces instead of silently disappearing or showing
//! stale markers.
//!
//! * an [`OrientationAidRecord`] binds a claimed M5 surface (keyed by a
//!   [`KeyboardSurfaceKind`] and a non-display [`KeyboardSurfaceSubject`]) to one
//!   orientation aid: which [`OrientationAidKind`] it renders, the
//!   [`OrientationMarkerSummary`] it reflects (a class, an opaque /
//!   workspace-relative object token aligned with the same object identity shown
//!   elsewhere, the marker and rendered counts, and a label), the
//!   [`OrientationAidClass`] posture it advertises, and the resolved
//!   [`OrientationDisclosureClass`];
//! * an aid is **never silently flattened to a fully-active claim when it is
//!   constrained**: a record whose markers exceed the live-render budget, whose
//!   breadcrumb identity is shared across surfaces, that sits on a constrained
//!   viewport, that runs under a reduced-motion profile, that reflects a large /
//!   unsafe artifact, that runs under a limited capability profile, or whose
//!   orientation proof is stale / missing fires one or more
//!   [`OrientationContractTrigger`]s. Each trigger imposes a minimum-disclosure
//!   floor on the resolution, so a triggered record can never resolve to a flat
//!   [`OrientationDisclosureClass::AidFullyActive`]; it must preserve an honest
//!   count summary, align identity across surfaces, disclose reduced detail,
//!   disclose suppressed motion, disclose a degraded aid, or disclose an
//!   unavailable aid — and it can never silently remove the aid or show a stale
//!   marker;
//! * a degraded or unavailable aid **always carries a precise reason label** an
//!   accessibility or support surface can read back, and never lets a
//!   provider-linked surface read as a locally verified orientation truth.
//!
//! [`OrientationAidPacket::validate`] refuses a packet that flattens a
//! constrained aid into a fully-active claim, that lowers a resolution below its
//! required disclosure floor, that silently removes an aid (the absent-downgraded
//! posture), that shows a stale marker, that drops the multi-cursor / fold /
//! breadcrumb / minimap / overview truth, or that lets a provider-linked surface
//! read as a locally verified orientation truth.
//!
//! Raw provider payloads, file contents, and absolute private paths never cross
//! this boundary; the packet carries only typed class tokens, booleans, opaque /
//! relative ids, fingerprint digests, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/interaction/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati.schema.json`](../../../../schemas/interaction/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati.schema.json).
//! The contract doc is
//! [`docs/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati.md`](../../../../docs/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati.md).
//! The protected fixture directory is
//! [`fixtures/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati/`](../../../../fixtures/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Re-export the frozen taxonomy this consumer binds, so product, help, support,
// and migration surfaces can name those types through this module rather than
// reaching into the matrix module by hand.
pub use crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix::{
    AxisProofCurrency, AxisVerification, KeyboardSurfaceKind, KeyboardSurfaceSubject,
    OrientationAidClass, SurfaceOriginClass, KEYBOARD_CONTINUITY_MATRIX_DOC_REF,
};

/// Stable record-kind tag carried by [`OrientationAidPacket`].
pub const ORIENTATION_AID_RECORD_KIND: &str =
    "m5_multi_cursor_fold_breadcrumb_minimap_overview_orientation_aid_packet";

/// Schema version for the orientation-aid packet.
pub const ORIENTATION_AID_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const ORIENTATION_AID_SCHEMA_REF: &str =
    "schemas/interaction/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati.schema.json";

/// Repo-relative path of the contract doc.
pub const ORIENTATION_AID_DOC_REF: &str =
    "docs/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati.md";

/// Repo-relative path of the checked support-export artifact.
pub const ORIENTATION_AID_ARTIFACT_REF: &str =
    "artifacts/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const ORIENTATION_AID_SUMMARY_REF: &str =
    "artifacts/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati.md";

/// Repo-relative path of the protected fixture directory.
pub const ORIENTATION_AID_FIXTURE_DIR: &str =
    "fixtures/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati";

/// Source contract ref of the overview / orientation-surface contract.
pub const ORIENTATION_AID_CONTRACT_REF: &str = "docs/ux/overview_surface_contract.md";

/// Which orientation aid a record reflects. Each aid kind is named rather than
/// collapsed into one generic "orientation" affordance, so help, migration, and
/// support can tell a multi-cursor count from a fold summary, a breadcrumb path,
/// a minimap, or an overview ruler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrientationAidKind {
    /// The visible multi-cursor / multi-selection count and primary anchor.
    MultiCursor,
    /// The fold-state summary (collapsed / expanded regions and counts).
    FoldState,
    /// The breadcrumb path identifying the current location.
    Breadcrumb,
    /// The minimap thumbnail and its markers.
    Minimap,
    /// The overview-ruler markers (diagnostics, changes, matches).
    OverviewRuler,
}

impl OrientationAidKind {
    /// Every aid kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MultiCursor,
        Self::FoldState,
        Self::Breadcrumb,
        Self::Minimap,
        Self::OverviewRuler,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MultiCursor => "multi_cursor",
            Self::FoldState => "fold_state",
            Self::Breadcrumb => "breadcrumb",
            Self::Minimap => "minimap",
            Self::OverviewRuler => "overview_ruler",
        }
    }
}

/// Class of object an orientation aid reflects. The class lets help, migration,
/// and support name the same underlying object identity the product exposes
/// elsewhere, so a breadcrumb or minimap aligns with the object IDs shown in the
/// editor, explorer, and outline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrientationObjectClass {
    /// An editor / diff text buffer.
    EditorBuffer,
    /// A notebook cell group.
    NotebookCellGroup,
    /// A data-grid / API result grid.
    DataResultGrid,
    /// A source-first preview document.
    PreviewDocument,
    /// A docs authoring outline.
    DocsOutline,
    /// A review / pull-request diff.
    ReviewDiff,
    /// An embedded browser-runtime overlay.
    RuntimeOverlay,
    /// A provider-linked companion transcript.
    CompanionTranscript,
}

impl OrientationObjectClass {
    /// Every object class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::EditorBuffer,
        Self::NotebookCellGroup,
        Self::DataResultGrid,
        Self::PreviewDocument,
        Self::DocsOutline,
        Self::ReviewDiff,
        Self::RuntimeOverlay,
        Self::CompanionTranscript,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorBuffer => "editor_buffer",
            Self::NotebookCellGroup => "notebook_cell_group",
            Self::DataResultGrid => "data_result_grid",
            Self::PreviewDocument => "preview_document",
            Self::DocsOutline => "docs_outline",
            Self::ReviewDiff => "review_diff",
            Self::RuntimeOverlay => "runtime_overlay",
            Self::CompanionTranscript => "companion_transcript",
        }
    }
}

/// Resolved disclosure an aid carries for one orientation surface. This is the
/// canonical orientation vocabulary help, migration, and support name.
///
/// Only [`Self::AidFullyActive`] is the flat fully-active baseline; every other
/// resolution names an honestly-disclosed constraint — a preserved count summary,
/// a cross-surface identity alignment, a disclosed reduced detail, a disclosed
/// motion suppression, a disclosed degraded aid, or a disclosed unavailable aid.
/// The [`Self::disclosure_rank`] orders the resolutions so a triggered record can
/// be held at or above a required floor and never flattened into the bare
/// fully-active baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrientationDisclosureClass {
    /// The aid is fully active, every marker rendered live, nothing constrained.
    AidFullyActive,
    /// Markers exceed the live-render budget; an honest count summary is preserved.
    CountSummaryPreserved,
    /// The aid identity is aligned with the same object IDs shown elsewhere.
    IdentityAlignedAcrossSurfaces,
    /// A constrained viewport reduces detail, disclosed honestly.
    ReducedDetailDisclosed,
    /// A reduced-motion profile suppresses animation, disclosed honestly.
    MotionReducedDisclosed,
    /// A large / unsafe artifact or stale proof degrades the aid, disclosed honestly.
    DegradedDisclosed,
    /// A limited capability profile makes the aid unavailable, disclosed honestly.
    UnavailableDisclosed,
}

impl OrientationDisclosureClass {
    /// Every disclosure class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AidFullyActive,
        Self::CountSummaryPreserved,
        Self::IdentityAlignedAcrossSurfaces,
        Self::ReducedDetailDisclosed,
        Self::MotionReducedDisclosed,
        Self::DegradedDisclosed,
        Self::UnavailableDisclosed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AidFullyActive => "aid_fully_active",
            Self::CountSummaryPreserved => "count_summary_preserved",
            Self::IdentityAlignedAcrossSurfaces => "identity_aligned_across_surfaces",
            Self::ReducedDetailDisclosed => "reduced_detail_disclosed",
            Self::MotionReducedDisclosed => "motion_reduced_disclosed",
            Self::DegradedDisclosed => "degraded_disclosed",
            Self::UnavailableDisclosed => "unavailable_disclosed",
        }
    }

    /// Monotonic disclosure rank; higher is a more constrained / explicit
    /// disclosure, so a triggered record must hold a resolution whose rank meets
    /// its floor.
    pub const fn disclosure_rank(self) -> u8 {
        match self {
            Self::AidFullyActive => 0,
            Self::CountSummaryPreserved => 1,
            Self::IdentityAlignedAcrossSurfaces => 2,
            Self::ReducedDetailDisclosed => 3,
            Self::MotionReducedDisclosed => 4,
            Self::DegradedDisclosed => 5,
            Self::UnavailableDisclosed => 6,
        }
    }

    /// Whether this resolution is the flat fully-active baseline that a
    /// constrained aid must never silently collapse into.
    pub const fn is_flat_baseline(self) -> bool {
        matches!(self, Self::AidFullyActive)
    }

    /// The canonical [`OrientationAidClass`] posture this resolution implies. A
    /// fully-active aid reads as full aids; a summarized, identity-aligned, or
    /// reduced-detail aid reads as reduced-but-honest aids; a motion-reduced,
    /// degraded, or unavailable aid reads as degraded-but-honest aids. No valid
    /// resolution ever reads as the absent-downgraded (silently-removed) posture.
    pub const fn canonical_aid_class(self) -> OrientationAidClass {
        match self {
            Self::AidFullyActive => OrientationAidClass::FullOrientationAids,
            Self::CountSummaryPreserved
            | Self::IdentityAlignedAcrossSurfaces
            | Self::ReducedDetailDisclosed => OrientationAidClass::ReducedOrientationAidsHonest,
            Self::MotionReducedDisclosed | Self::DegradedDisclosed | Self::UnavailableDisclosed => {
                OrientationAidClass::OrientationAidsDegradedHonest
            }
        }
    }

    /// Whether this resolution must cite a `count_summary_label`.
    pub const fn requires_count_summary_label(self) -> bool {
        matches!(self, Self::CountSummaryPreserved)
    }

    /// Whether this resolution must cite an `identity_note`.
    pub const fn requires_identity_note(self) -> bool {
        matches!(self, Self::IdentityAlignedAcrossSurfaces)
    }

    /// Whether this resolution must cite a `reduced_detail_label`.
    pub const fn requires_reduced_detail_label(self) -> bool {
        matches!(self, Self::ReducedDetailDisclosed)
    }

    /// Whether this resolution must cite a `motion_reduced_label`.
    pub const fn requires_motion_reduced_label(self) -> bool {
        matches!(self, Self::MotionReducedDisclosed)
    }

    /// Whether this resolution must cite a `degrade_reason_label`.
    pub const fn requires_degrade_reason_label(self) -> bool {
        matches!(self, Self::DegradedDisclosed)
    }

    /// Whether this resolution must cite an `unavailable_reason_label`.
    pub const fn requires_unavailable_reason_label(self) -> bool {
        matches!(self, Self::UnavailableDisclosed)
    }
}

/// Why a record's aid was held off the flat fully-active lane. Each trigger
/// imposes a minimum-disclosure floor; the chrome quotes the trigger verbatim
/// instead of a generic label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrientationContractTrigger {
    /// Markers exceed the live-render budget, so a count summary must be preserved.
    HighCardinalityMarkers,
    /// The aid identity is shared across surfaces, so identity must be aligned.
    CrossSurfaceIdentityShared,
    /// A constrained viewport forces reduced detail, disclosed honestly.
    ConstrainedViewport,
    /// A reduced-motion profile forces suppressed motion, disclosed honestly.
    ReducedMotionProfile,
    /// A large / unsafe artifact forces a degraded aid, disclosed honestly.
    LargeOrUnsafeArtifact,
    /// A limited capability profile forces an unavailable aid, disclosed honestly.
    LimitedCapabilityProfile,
    /// The orientation proof backing this record is stale or missing.
    StaleOrMissingOrientationProof,
}

impl OrientationContractTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighCardinalityMarkers => "high_cardinality_markers",
            Self::CrossSurfaceIdentityShared => "cross_surface_identity_shared",
            Self::ConstrainedViewport => "constrained_viewport",
            Self::ReducedMotionProfile => "reduced_motion_profile",
            Self::LargeOrUnsafeArtifact => "large_or_unsafe_artifact",
            Self::LimitedCapabilityProfile => "limited_capability_profile",
            Self::StaleOrMissingOrientationProof => "stale_or_missing_orientation_proof",
        }
    }

    /// Minimum resolution disclosure rank this trigger imposes.
    ///
    /// High-cardinality markers require at least a preserved count summary; a
    /// cross-surface shared identity requires an identity alignment; a constrained
    /// viewport requires disclosed reduced detail; a reduced-motion profile
    /// requires disclosed motion suppression; a large / unsafe artifact or stale
    /// proof requires a disclosed degraded aid; a limited capability profile
    /// requires a disclosed unavailable aid.
    pub const fn minimum_resolution_rank(self) -> u8 {
        match self {
            Self::HighCardinalityMarkers => {
                OrientationDisclosureClass::CountSummaryPreserved.disclosure_rank()
            }
            Self::CrossSurfaceIdentityShared => {
                OrientationDisclosureClass::IdentityAlignedAcrossSurfaces.disclosure_rank()
            }
            Self::ConstrainedViewport => {
                OrientationDisclosureClass::ReducedDetailDisclosed.disclosure_rank()
            }
            Self::ReducedMotionProfile => {
                OrientationDisclosureClass::MotionReducedDisclosed.disclosure_rank()
            }
            Self::LargeOrUnsafeArtifact | Self::StaleOrMissingOrientationProof => {
                OrientationDisclosureClass::DegradedDisclosed.disclosure_rank()
            }
            Self::LimitedCapabilityProfile => {
                OrientationDisclosureClass::UnavailableDisclosed.disclosure_rank()
            }
        }
    }
}

/// Summary of the markers an orientation aid reflects. Preserved so the product
/// can show *how many* cursors, folds, or markers exist and *how many* are
/// rendered live rather than silently dropping the surplus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrientationMarkerSummary {
    /// Class of the underlying object the aid reflects.
    pub object_class: OrientationObjectClass,
    /// Opaque / workspace-relative object token. Never an absolute private path.
    /// Aligned with the same object identity shown elsewhere in the product.
    pub object_token: String,
    /// Count of distinct markers the aid covers (at least one).
    pub marker_count: u32,
    /// Count of markers actually rendered live (at most `marker_count`).
    pub rendered_count: u32,
    /// Reviewable marker-summary label.
    pub display_label: String,
}

impl OrientationMarkerSummary {
    /// Whether the summary carries the identity an orientation aid needs.
    pub fn is_valid(&self) -> bool {
        !self.object_token.trim().is_empty()
            && !self.display_label.trim().is_empty()
            && self.marker_count >= 1
            && self.rendered_count <= self.marker_count
    }

    /// Whether the aid could not render every marker live, so it must preserve an
    /// honest count summary rather than a silent partial render.
    pub fn is_high_cardinality(&self) -> bool {
        self.rendered_count < self.marker_count
    }
}

/// Constructor input for [`OrientationAidRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrientationAidRecordInput {
    /// Stable record id.
    pub record_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the record covers.
    pub subject: KeyboardSurfaceSubject,
    /// Reviewable record label.
    pub label_summary: String,
    /// Which orientation aid this record reflects.
    pub aid_kind: OrientationAidKind,
    /// The markers the aid reflects.
    pub marker_summary: OrientationMarkerSummary,
    /// Canonical orientation-aid posture advertised by the surface.
    pub aid_posture: OrientationAidClass,
    /// Whether the aid identity is shared with the same object IDs shown elsewhere.
    pub cross_surface_identity_shared: bool,
    /// Whether a constrained viewport reduces the aid's detail.
    pub constrained_viewport: bool,
    /// Whether a reduced-motion profile suppresses the aid's animation.
    pub reduced_motion_profile: bool,
    /// Whether the aid reflects a large / unsafe artifact.
    pub large_or_unsafe_artifact: bool,
    /// Whether the aid runs under a limited capability profile.
    pub limited_capability_profile: bool,
    /// Reopenable verification proof backing the resolution.
    pub verification: AxisVerification,
    /// The resolved orientation disclosure.
    pub resolution: OrientationDisclosureClass,
    /// Triggers recorded as firing for this record.
    pub fired_triggers: Vec<OrientationContractTrigger>,
    /// Required when `resolution` is `count_summary_preserved`.
    pub count_summary_label: Option<String>,
    /// Required when `resolution` is `identity_aligned_across_surfaces`.
    pub identity_note: Option<String>,
    /// Required when `resolution` is `reduced_detail_disclosed`.
    pub reduced_detail_label: Option<String>,
    /// Required when `resolution` is `motion_reduced_disclosed`.
    pub motion_reduced_label: Option<String>,
    /// Required when `resolution` is `degraded_disclosed`.
    pub degrade_reason_label: Option<String>,
    /// Required when `resolution` is `unavailable_disclosed`.
    pub unavailable_reason_label: Option<String>,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

/// One orientation-aid record binding a claimed M5 surface to one aid with a
/// named marker summary, an honest disclosure resolution, and a degraded-
/// orientation posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrientationAidRecord {
    /// Stable record id.
    pub record_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the record covers.
    pub subject: KeyboardSurfaceSubject,
    /// Reviewable record label.
    pub label_summary: String,
    /// Which orientation aid this record reflects.
    pub aid_kind: OrientationAidKind,
    /// The markers the aid reflects.
    pub marker_summary: OrientationMarkerSummary,
    /// Canonical orientation-aid posture advertised by the surface.
    pub aid_posture: OrientationAidClass,
    /// Whether the aid identity is shared with the same object IDs shown elsewhere.
    pub cross_surface_identity_shared: bool,
    /// Whether a constrained viewport reduces the aid's detail.
    pub constrained_viewport: bool,
    /// Whether a reduced-motion profile suppresses the aid's animation.
    pub reduced_motion_profile: bool,
    /// Whether the aid reflects a large / unsafe artifact.
    pub large_or_unsafe_artifact: bool,
    /// Whether the aid runs under a limited capability profile.
    pub limited_capability_profile: bool,
    /// Reopenable verification proof backing the resolution.
    pub verification: AxisVerification,
    /// The resolved orientation disclosure.
    pub resolution: OrientationDisclosureClass,
    /// Triggers recorded as firing for this record. Must equal the computed set.
    pub fired_triggers: Vec<OrientationContractTrigger>,
    /// Required when `resolution` is `count_summary_preserved`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count_summary_label: Option<String>,
    /// Required when `resolution` is `identity_aligned_across_surfaces`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_note: Option<String>,
    /// Required when `resolution` is `reduced_detail_disclosed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_detail_label: Option<String>,
    /// Required when `resolution` is `motion_reduced_disclosed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motion_reduced_label: Option<String>,
    /// Required when `resolution` is `degraded_disclosed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degrade_reason_label: Option<String>,
    /// Required when `resolution` is `unavailable_disclosed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason_label: Option<String>,
    /// Guardrail: record does not carry raw provider payloads.
    pub raw_provider_payload_present: bool,
    /// Guardrail: record does not carry an absolute private path.
    pub absolute_private_path_present: bool,
    /// Guardrail: the aid was not silently removed (no absent-downgraded posture).
    pub aid_silently_removed: bool,
    /// Guardrail: no stale markers were shown for this aid.
    pub stale_markers_shown: bool,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

impl OrientationAidRecord {
    /// Builds a record from its input, defaulting the guardrail flags to their
    /// safe values.
    pub fn new(input: OrientationAidRecordInput) -> Self {
        Self {
            record_id: input.record_id,
            surface_kind: input.surface_kind,
            subject: input.subject,
            label_summary: input.label_summary,
            aid_kind: input.aid_kind,
            marker_summary: input.marker_summary,
            aid_posture: input.aid_posture,
            cross_surface_identity_shared: input.cross_surface_identity_shared,
            constrained_viewport: input.constrained_viewport,
            reduced_motion_profile: input.reduced_motion_profile,
            large_or_unsafe_artifact: input.large_or_unsafe_artifact,
            limited_capability_profile: input.limited_capability_profile,
            verification: input.verification,
            resolution: input.resolution,
            fired_triggers: input.fired_triggers,
            count_summary_label: input.count_summary_label,
            identity_note: input.identity_note,
            reduced_detail_label: input.reduced_detail_label,
            motion_reduced_label: input.motion_reduced_label,
            degrade_reason_label: input.degrade_reason_label,
            unavailable_reason_label: input.unavailable_reason_label,
            raw_provider_payload_present: false,
            absolute_private_path_present: false,
            aid_silently_removed: false,
            stale_markers_shown: false,
            evidence_refs: input.evidence_refs,
            minted_at: input.minted_at,
        }
    }

    /// Whether orientation truth for this record is provider-backed / imported.
    pub fn provider_or_imported(&self) -> bool {
        self.subject.origin_class.is_provider_or_imported()
    }

    /// Whether the verification proof backs a current orientation claim for this
    /// record's origin posture.
    pub fn orientation_proof_current(&self) -> bool {
        self.verification.backs_claim(self.provider_or_imported())
    }

    /// The set of triggers that actually fire for this record, computed from its
    /// marker cardinality, identity sharing, viewport, motion profile, artifact
    /// posture, capability profile, and proof.
    pub fn computed_triggers(&self) -> BTreeSet<OrientationContractTrigger> {
        let mut triggers = BTreeSet::new();
        if self.marker_summary.is_high_cardinality() {
            triggers.insert(OrientationContractTrigger::HighCardinalityMarkers);
        }
        if self.cross_surface_identity_shared {
            triggers.insert(OrientationContractTrigger::CrossSurfaceIdentityShared);
        }
        if self.constrained_viewport {
            triggers.insert(OrientationContractTrigger::ConstrainedViewport);
        }
        if self.reduced_motion_profile {
            triggers.insert(OrientationContractTrigger::ReducedMotionProfile);
        }
        if self.large_or_unsafe_artifact {
            triggers.insert(OrientationContractTrigger::LargeOrUnsafeArtifact);
        }
        if self.limited_capability_profile {
            triggers.insert(OrientationContractTrigger::LimitedCapabilityProfile);
        }
        if !self.orientation_proof_current() {
            triggers.insert(OrientationContractTrigger::StaleOrMissingOrientationProof);
        }
        triggers
    }

    /// The recorded triggers as a set.
    pub fn recorded_triggers(&self) -> BTreeSet<OrientationContractTrigger> {
        self.fired_triggers.iter().copied().collect()
    }

    /// The minimum resolution disclosure rank this record must meet, given its
    /// triggers.
    pub fn required_floor_rank(&self) -> u8 {
        self.computed_triggers()
            .iter()
            .map(|trigger| trigger.minimum_resolution_rank())
            .max()
            .unwrap_or(0)
    }

    /// Whether the aid must be held off the flat fully-active lane.
    pub fn must_not_flatten(&self) -> bool {
        self.required_floor_rank() > 0
    }

    /// Whether the recorded resolution meets the required disclosure floor.
    pub fn resolution_meets_floor(&self) -> bool {
        self.resolution.disclosure_rank() >= self.required_floor_rank()
    }

    /// Whether the recorded resolution silently flattens a constrained aid into
    /// the bare fully-active baseline.
    pub fn silently_flattens(&self) -> bool {
        self.resolution.is_flat_baseline() && self.must_not_flatten()
    }

    /// Whether the recorded trigger set matches the computed set.
    pub fn triggers_consistent(&self) -> bool {
        self.recorded_triggers() == self.computed_triggers()
    }

    /// Whether the advertised posture matches the one the resolution implies. No
    /// valid resolution ever advertises the absent-downgraded (silently-removed)
    /// posture.
    pub fn aid_posture_consistent(&self) -> bool {
        !self.aid_posture.is_downgraded()
            && self.aid_posture == self.resolution.canonical_aid_class()
    }

    /// Whether the resolution carries exactly the detail field it requires.
    pub fn resolution_detail_consistent(&self) -> bool {
        let present = |opt: &Option<String>| {
            opt.as_deref()
                .is_some_and(|value| !value.trim().is_empty() && !label_is_generic(value))
        };
        let check = |required: bool, value: &Option<String>| {
            if required {
                present(value)
            } else {
                value.is_none()
            }
        };
        check(
            self.resolution.requires_count_summary_label(),
            &self.count_summary_label,
        ) && check(
            self.resolution.requires_identity_note(),
            &self.identity_note,
        ) && check(
            self.resolution.requires_reduced_detail_label(),
            &self.reduced_detail_label,
        ) && check(
            self.resolution.requires_motion_reduced_label(),
            &self.motion_reduced_label,
        ) && check(
            self.resolution.requires_degrade_reason_label(),
            &self.degrade_reason_label,
        ) && check(
            self.resolution.requires_unavailable_reason_label(),
            &self.unavailable_reason_label,
        )
    }

    /// Whether the imported posture is consistent: a provider/imported surface
    /// never reads as a locally verified orientation truth, and a local surface
    /// never leans on imported proof.
    pub fn imported_posture_consistent(&self) -> bool {
        if self.provider_or_imported() {
            !self.verification.proof_currency.is_current_local()
        } else {
            !self.verification.proof_currency.is_imported_current()
        }
    }

    /// Whether no raw boundary material or silent-removal / stale-marker side
    /// effect is flagged present.
    pub fn no_raw_boundary_material(&self) -> bool {
        !self.raw_provider_payload_present
            && !self.absolute_private_path_present
            && !self.aid_silently_removed
            && !self.stale_markers_shown
    }

    /// Whether every field required to record this record is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.record_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.minted_at.trim().is_empty()
            && self.subject.is_valid()
            && self.marker_summary.is_valid()
            && self.verification.is_well_formed()
            && self.triggers_consistent()
            && !self.silently_flattens()
            && self.resolution_meets_floor()
            && self.resolution_detail_consistent()
            && self.aid_posture_consistent()
            && self.imported_posture_consistent()
            && self.no_raw_boundary_material()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrientationAidGuardrails {
    /// Aids degrade honestly and are never silently removed.
    pub aids_degrade_honestly_never_silently_removed: bool,
    /// Stale markers are never shown; a degraded aid drops them and says so.
    pub stale_markers_never_shown: bool,
    /// Multi-cursor and fold truth is preserved as named counts and summaries.
    pub multi_cursor_and_fold_truth_preserved: bool,
    /// Breadcrumb identity aligns with the same object IDs shown elsewhere.
    pub breadcrumb_identity_aligned_with_object_ids: bool,
    /// Minimap and overview-ruler markers are named, not collapsed to one blob.
    pub minimap_overview_markers_named: bool,
    /// Provider-linked orientation truth never reads as a locally verified truth.
    pub provider_orientation_never_reads_as_local: bool,
    /// No new editor core is introduced here.
    pub no_new_editor_core_introduced: bool,
}

impl OrientationAidGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.aids_degrade_honestly_never_silently_removed
            && self.stale_markers_never_shown
            && self.multi_cursor_and_fold_truth_preserved
            && self.breadcrumb_identity_aligned_with_object_ids
            && self.minimap_overview_markers_named
            && self.provider_orientation_never_reads_as_local
            && self.no_new_editor_core_introduced
    }
}

/// Consumer projection block: the surfaces that read this packet without cloning
/// orientation language by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrientationAidConsumerProjection {
    /// Product surfaces ingest this packet.
    pub product_ingests_packet: bool,
    /// Help / migration guidance ingests the same packet.
    pub help_migration_ingests_packet: bool,
    /// Support / export tooling ingests the same packet.
    pub support_export_ingests_packet: bool,
    /// Release-control surfaces ingest the same packet.
    pub release_control_ingests_packet: bool,
    /// Accessibility and support can name which aids were active, degraded, or
    /// unavailable on a claimed M5 surface from this packet.
    pub aid_states_nameable_for_accessibility_and_support: bool,
}

impl OrientationAidConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_packet
            && self.help_migration_ingests_packet
            && self.support_export_ingests_packet
            && self.release_control_ingests_packet
            && self.aid_states_nameable_for_accessibility_and_support
    }
}

/// Verification freshness block for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrientationAidFreshness {
    /// Verification-freshness SLO in hours.
    pub verification_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last verification refresh.
    pub last_verification_refresh: String,
    /// True when stale verification automatically forces records off the flat lane.
    pub auto_escalate_on_stale: bool,
}

impl OrientationAidFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.verification_freshness_slo_hours > 0
            && !self.last_verification_refresh.trim().is_empty()
    }
}

/// Constructor input for [`OrientationAidPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrientationAidPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface orientation-aid records.
    pub records: Vec<OrientationAidRecord>,
    /// Guardrail invariants block.
    pub guardrails: OrientationAidGuardrails,
    /// Consumer projection block.
    pub consumer_projection: OrientationAidConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: OrientationAidFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe orientation-aid packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrientationAidPacket {
    /// Record kind; must equal [`ORIENTATION_AID_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ORIENTATION_AID_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface orientation-aid records.
    pub records: Vec<OrientationAidRecord>,
    /// Guardrail invariants block.
    pub guardrails: OrientationAidGuardrails,
    /// Consumer projection block.
    pub consumer_projection: OrientationAidConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: OrientationAidFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl OrientationAidPacket {
    /// Builds an orientation-aid packet.
    pub fn new(input: OrientationAidPacketInput) -> Self {
        Self {
            record_kind: ORIENTATION_AID_RECORD_KIND.to_owned(),
            schema_version: ORIENTATION_AID_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            records: input.records,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            verification_freshness: input.verification_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surface kinds represented by some record in this packet.
    pub fn represented_surface_kinds(&self) -> BTreeSet<KeyboardSurfaceKind> {
        self.records
            .iter()
            .map(|record| record.surface_kind)
            .collect()
    }

    /// Object classes represented across records.
    pub fn represented_object_classes(&self) -> BTreeSet<OrientationObjectClass> {
        self.records
            .iter()
            .map(|record| record.marker_summary.object_class)
            .collect()
    }

    /// Aid kinds represented across records.
    pub fn represented_aid_kinds(&self) -> BTreeSet<OrientationAidKind> {
        self.records.iter().map(|record| record.aid_kind).collect()
    }

    /// Resolution classes represented across records.
    pub fn represented_resolutions(&self) -> BTreeSet<OrientationDisclosureClass> {
        self.records
            .iter()
            .map(|record| record.resolution)
            .collect()
    }

    /// Count of records held off the flat fully-active lane.
    pub fn forced_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.must_not_flatten())
            .count()
    }

    /// Count of records resolved to the flat fully-active baseline.
    pub fn flat_baseline_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.resolution.is_flat_baseline())
            .count()
    }

    /// Count of records whose aid is degraded or unavailable but disclosed.
    pub fn degraded_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| {
                matches!(
                    record.resolution,
                    OrientationDisclosureClass::DegradedDisclosed
                        | OrientationDisclosureClass::UnavailableDisclosed
                )
            })
            .count()
    }

    /// Count of provider-linked / imported records.
    pub fn provider_or_imported_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.provider_or_imported())
            .count()
    }

    /// Resolves a record by its id.
    pub fn record(&self, record_id: &str) -> Option<&OrientationAidRecord> {
        self.records
            .iter()
            .find(|record| record.record_id == record_id)
    }

    /// Validates the orientation-aid invariants.
    pub fn validate(&self) -> Vec<OrientationAidViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ORIENTATION_AID_RECORD_KIND {
            violations.push(OrientationAidViolation::WrongRecordKind);
        }
        if self.schema_version != ORIENTATION_AID_SCHEMA_VERSION {
            violations.push(OrientationAidViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(OrientationAidViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_records(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(OrientationAidViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(OrientationAidViolation::ConsumerProjectionIncomplete);
        }
        if !self.verification_freshness.is_valid() {
            violations.push(OrientationAidViolation::VerificationFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("orientation aid packet serializes"),
        ) {
            violations.push(OrientationAidViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("orientation aid packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Orientation Aids: Multi-Cursor, Fold-State, Breadcrumb, Minimap, Overview-Ruler, and Degraded-Orientation Truth\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Records: {} ({} flat baseline, {} forced off flat, {} degraded/unavailable, {} provider/imported)\n",
            self.records.len(),
            self.flat_baseline_record_count(),
            self.forced_record_count(),
            self.degraded_record_count(),
            self.provider_or_imported_record_count()
        ));
        out.push_str(&format!(
            "- Surface kinds: {} / {}\n",
            self.represented_surface_kinds().len(),
            KeyboardSurfaceKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Aid kinds: {} / {}\n",
            self.represented_aid_kinds().len(),
            OrientationAidKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Disclosure classes: {} / {}\n",
            self.represented_resolutions().len(),
            OrientationDisclosureClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Verification freshness SLO: {} hours (last refresh: {})\n",
            self.verification_freshness.verification_freshness_slo_hours,
            self.verification_freshness.last_verification_refresh
        ));
        out.push_str("\n## Records\n\n");
        for record in &self.records {
            out.push_str(&format!(
                "- **{}** ({}): aid `{}`, disclosure `{}`, posture `{}`\n",
                record.record_id,
                record.surface_kind.as_str(),
                record.aid_kind.as_str(),
                record.resolution.as_str(),
                record.aid_posture.as_str()
            ));
            out.push_str(&format!("  - {}\n", record.label_summary));
            out.push_str(&format!(
                "  - {} of `{}` ({}), markers {}/{} rendered\n",
                record.aid_kind.as_str(),
                record.marker_summary.object_token,
                record.marker_summary.object_class.as_str(),
                record.marker_summary.rendered_count,
                record.marker_summary.marker_count
            ));
            let triggers = record
                .fired_triggers
                .iter()
                .map(|trigger| trigger.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "  - triggers: [{}]\n",
                if triggers.is_empty() {
                    "none"
                } else {
                    &triggers
                }
            ));
            if let Some(label) = &record.count_summary_label {
                out.push_str(&format!("  - Count summary: {label}\n"));
            }
            if let Some(note) = &record.identity_note {
                out.push_str(&format!("  - Identity alignment: {note}\n"));
            }
            if let Some(label) = &record.reduced_detail_label {
                out.push_str(&format!("  - Reduced detail: {label}\n"));
            }
            if let Some(label) = &record.motion_reduced_label {
                out.push_str(&format!("  - Motion reduced: {label}\n"));
            }
            if let Some(label) = &record.degrade_reason_label {
                out.push_str(&format!("  - Degraded: {label}\n"));
            }
            if let Some(label) = &record.unavailable_reason_label {
                out.push_str(&format!("  - Unavailable: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum OrientationAidArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<OrientationAidViolation>),
}

impl fmt::Display for OrientationAidArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "orientation aid export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "orientation aid export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for OrientationAidArtifactError {}

/// Validation failures emitted by [`OrientationAidPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrientationAidViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed surface kind is represented by no record.
    RequiredSurfaceKindMissing,
    /// A required orientation-object class is represented by no record.
    RequiredObjectClassMissing,
    /// The required orientation-aid kinds are not all represented.
    AidKindCoverageMissing,
    /// The required disclosure classes are not all represented.
    ResolutionCoverageMissing,
    /// No record demonstrates an aid held off the flat fully-active lane.
    ForcedRecordCaseMissing,
    /// No clean flat-baseline fully-active record is present.
    FlatBaselineMissing,
    /// No degraded-but-disclosed aid record is present.
    DegradedCaseMissing,
    /// No unavailable-but-disclosed aid record is present.
    UnavailableCaseMissing,
    /// No provider-linked / imported record is present.
    ProviderOrImportedCaseMissing,
    /// A record is incomplete.
    RecordIncomplete,
    /// A constrained aid was flattened into the bare fully-active baseline.
    SilentFlatteningOfAid,
    /// A record's resolution ranks below its required disclosure floor.
    ResolutionBelowRequiredFloor,
    /// A record's recorded triggers do not match the computed set.
    TriggerSetInconsistent,
    /// A record's resolution detail field is missing, generic, or unexpected.
    ResolutionDetailInconsistent,
    /// A record's advertised posture does not match the one its resolution implies.
    AidPostureInconsistent,
    /// A record dropped its marker summary.
    MarkerSummaryMissing,
    /// A provider/imported record reads as a locally verified orientation truth.
    ImportedReadsAsLocal,
    /// A record's verification proof is not reopenable.
    VerificationProofNotReopenable,
    /// A record lacks evidence refs.
    RecordEvidenceMissing,
    /// A record's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A record flags raw boundary material / silent-removal side effect present.
    RawBoundaryMaterialPresent,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Verification freshness block is incomplete.
    VerificationFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl OrientationAidViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::RequiredObjectClassMissing => "required_object_class_missing",
            Self::AidKindCoverageMissing => "aid_kind_coverage_missing",
            Self::ResolutionCoverageMissing => "resolution_coverage_missing",
            Self::ForcedRecordCaseMissing => "forced_record_case_missing",
            Self::FlatBaselineMissing => "flat_baseline_missing",
            Self::DegradedCaseMissing => "degraded_case_missing",
            Self::UnavailableCaseMissing => "unavailable_case_missing",
            Self::ProviderOrImportedCaseMissing => "provider_or_imported_case_missing",
            Self::RecordIncomplete => "record_incomplete",
            Self::SilentFlatteningOfAid => "silent_flattening_of_aid",
            Self::ResolutionBelowRequiredFloor => "resolution_below_required_floor",
            Self::TriggerSetInconsistent => "trigger_set_inconsistent",
            Self::ResolutionDetailInconsistent => "resolution_detail_inconsistent",
            Self::AidPostureInconsistent => "aid_posture_inconsistent",
            Self::MarkerSummaryMissing => "marker_summary_missing",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::VerificationProofNotReopenable => "verification_proof_not_reopenable",
            Self::RecordEvidenceMissing => "record_evidence_missing",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::VerificationFreshnessIncomplete => "verification_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_orientation_aid_export() -> Result<OrientationAidPacket, OrientationAidArtifactError>
{
    let packet: OrientationAidPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/interaction/m5/implement-multi-cursor-fold-state-breadcrumb-minimap-overview-ruler-and-degraded-orientati/support_export.json"
    )))
    .map_err(OrientationAidArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(OrientationAidArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &OrientationAidPacket,
    violations: &mut Vec<OrientationAidViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ORIENTATION_AID_SCHEMA_REF,
        ORIENTATION_AID_DOC_REF,
        ORIENTATION_AID_ARTIFACT_REF,
        KEYBOARD_CONTINUITY_MATRIX_DOC_REF,
        ORIENTATION_AID_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(OrientationAidViolation::MissingSourceContracts);
            break;
        }
    }
}

/// Surface kinds that must appear so the packet proves orientation-aid parity
/// across the new M5 editor, viewer, diff, and browser-runtime surfaces, plus the
/// editor-core baseline.
const REQUIRED_SURFACE_KINDS: [KeyboardSurfaceKind; 7] = [
    KeyboardSurfaceKind::EditorCore,
    KeyboardSurfaceKind::NotebookSurface,
    KeyboardSurfaceKind::DataApiSurface,
    KeyboardSurfaceKind::PreviewSurface,
    KeyboardSurfaceKind::DocsSurface,
    KeyboardSurfaceKind::ReviewSurface,
    KeyboardSurfaceKind::RuntimeSurface,
];

/// Orientation-object classes whose aid parity this packet must demonstrate.
const REQUIRED_OBJECT_CLASSES: [OrientationObjectClass; 7] = [
    OrientationObjectClass::EditorBuffer,
    OrientationObjectClass::NotebookCellGroup,
    OrientationObjectClass::DataResultGrid,
    OrientationObjectClass::PreviewDocument,
    OrientationObjectClass::DocsOutline,
    OrientationObjectClass::ReviewDiff,
    OrientationObjectClass::RuntimeOverlay,
];

fn validate_coverage(packet: &OrientationAidPacket, violations: &mut Vec<OrientationAidViolation>) {
    let surface_kinds = packet.represented_surface_kinds();
    for required in REQUIRED_SURFACE_KINDS {
        if !surface_kinds.contains(&required) {
            violations.push(OrientationAidViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    let object_classes = packet.represented_object_classes();
    for required in REQUIRED_OBJECT_CLASSES {
        if !object_classes.contains(&required) {
            violations.push(OrientationAidViolation::RequiredObjectClassMissing);
            break;
        }
    }

    let aid_kinds = packet.represented_aid_kinds();
    for required in OrientationAidKind::ALL {
        if !aid_kinds.contains(&required) {
            violations.push(OrientationAidViolation::AidKindCoverageMissing);
            break;
        }
    }

    let resolutions = packet.represented_resolutions();
    for required in OrientationDisclosureClass::ALL {
        if !resolutions.contains(&required) {
            violations.push(OrientationAidViolation::ResolutionCoverageMissing);
            break;
        }
    }

    if !packet
        .records
        .iter()
        .any(|record| record.must_not_flatten() && record.is_complete())
    {
        violations.push(OrientationAidViolation::ForcedRecordCaseMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution.is_flat_baseline() && !record.must_not_flatten() && record.is_complete()
    }) {
        violations.push(OrientationAidViolation::FlatBaselineMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution == OrientationDisclosureClass::DegradedDisclosed && record.is_complete()
    }) {
        violations.push(OrientationAidViolation::DegradedCaseMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution == OrientationDisclosureClass::UnavailableDisclosed
            && record.is_complete()
    }) {
        violations.push(OrientationAidViolation::UnavailableCaseMissing);
    }

    if packet.provider_or_imported_record_count() == 0 {
        violations.push(OrientationAidViolation::ProviderOrImportedCaseMissing);
    }
}

fn validate_records(packet: &OrientationAidPacket, violations: &mut Vec<OrientationAidViolation>) {
    for record in &packet.records {
        if !record.is_complete() {
            violations.push(OrientationAidViolation::RecordIncomplete);
        }
        if record.silently_flattens() {
            violations.push(OrientationAidViolation::SilentFlatteningOfAid);
        }
        if !record.resolution_meets_floor() {
            violations.push(OrientationAidViolation::ResolutionBelowRequiredFloor);
        }
        if !record.triggers_consistent() {
            violations.push(OrientationAidViolation::TriggerSetInconsistent);
        }
        if !record.resolution_detail_consistent() {
            violations.push(OrientationAidViolation::ResolutionDetailInconsistent);
        }
        if !record.aid_posture_consistent() {
            violations.push(OrientationAidViolation::AidPostureInconsistent);
        }
        if !record.marker_summary.is_valid() {
            violations.push(OrientationAidViolation::MarkerSummaryMissing);
        }
        if !record.imported_posture_consistent() {
            violations.push(OrientationAidViolation::ImportedReadsAsLocal);
        }
        if !record.verification.is_well_formed() {
            violations.push(OrientationAidViolation::VerificationProofNotReopenable);
        }
        if record.evidence_refs.is_empty()
            || record.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(OrientationAidViolation::RecordEvidenceMissing);
        }
        if !record.subject.fingerprint_independent_of_id() {
            violations.push(OrientationAidViolation::FingerprintSubstitutesIdentity);
        }
        if !record.no_raw_boundary_material() {
            violations.push(OrientationAidViolation::RawBoundaryMaterialPresent);
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "reduced"
            | "hidden"
            | "off"
            | "disabled"
            | "summary"
            | "more"
            | "markers"
            | "breadcrumb"
            | "minimap"
            | "overview"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key") || lower.contains("password") || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Stable packet id minted by [`seeded_orientation_aid_packet`].
pub const SEED_ORIENTATION_AID_PACKET_ID: &str = "m5-orientation-aids:stable:0001";

/// Mint timestamp used by [`seeded_orientation_aid_packet`].
pub const SEED_ORIENTATION_AID_MINTED_AT: &str = "2026-06-14T00:00:00Z";

/// Builds the canonical, validating orientation-aid packet that the checked-in
/// support export, the Markdown summary, and the conformance tests all share, so
/// the in-crate builder stays byte-aligned with the artifact.
///
/// The seed anchors a clean fully-active baseline (an editor-core multi-cursor
/// count), then exercises each non-default disclosure on a distinct M5 surface: a
/// notebook fold summary whose many folds exceed the live-render budget and are
/// preserved as an honest count summary, a preview breadcrumb whose identity is
/// aligned with the same object IDs shown elsewhere, a data/API minimap reduced
/// to honest detail under a constrained viewport, a docs overview ruler whose
/// motion is suppressed and disclosed under a reduced-motion profile, a review
/// diff minimap degraded and disclosed for a large / unsafe artifact, a
/// browser-runtime overlay fold aid made unavailable and disclosed under a limited
/// capability profile, and a provider-linked companion breadcrumb whose imported
/// orientation truth never reads as a local one.
pub fn seeded_orientation_aid_packet() -> OrientationAidPacket {
    OrientationAidPacket::new(OrientationAidPacketInput {
        packet_id: SEED_ORIENTATION_AID_PACKET_ID.to_owned(),
        label: "M5 Orientation Aids: Multi-Cursor, Fold-State, Breadcrumb, Minimap, Overview-Ruler, and Degraded-Orientation Truth"
            .to_owned(),
        records: seeded_records(),
        guardrails: OrientationAidGuardrails {
            aids_degrade_honestly_never_silently_removed: true,
            stale_markers_never_shown: true,
            multi_cursor_and_fold_truth_preserved: true,
            breadcrumb_identity_aligned_with_object_ids: true,
            minimap_overview_markers_named: true,
            provider_orientation_never_reads_as_local: true,
            no_new_editor_core_introduced: true,
        },
        consumer_projection: OrientationAidConsumerProjection {
            product_ingests_packet: true,
            help_migration_ingests_packet: true,
            support_export_ingests_packet: true,
            release_control_ingests_packet: true,
            aid_states_nameable_for_accessibility_and_support: true,
        },
        verification_freshness: OrientationAidFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
            auto_escalate_on_stale: true,
        },
        source_contract_refs: vec![
            ORIENTATION_AID_SCHEMA_REF.to_owned(),
            ORIENTATION_AID_DOC_REF.to_owned(),
            ORIENTATION_AID_ARTIFACT_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned(),
            ORIENTATION_AID_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
    })
}

fn seeded_records() -> Vec<OrientationAidRecord> {
    vec![
        editor_core_fully_active_record(),
        notebook_count_summary_record(),
        preview_identity_aligned_record(),
        data_api_reduced_detail_record(),
        docs_motion_reduced_record(),
        review_degraded_record(),
        runtime_unavailable_record(),
        companion_identity_aligned_record(),
    ]
}

/// Builds a verification proof keyed by a non-display fingerprint distinct from
/// the record id.
fn proof_for(record_id: &str, currency: AxisProofCurrency, summary: &str) -> AxisVerification {
    let (proof_ref, proof_fingerprint_token) = if currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{record_id}")),
            Some(format!("fp:proof:{record_id}")),
        )
    };
    AxisVerification {
        proof_currency: currency,
        proof_ref,
        proof_fingerprint_token,
        summary: summary.to_owned(),
    }
}

/// Builds a subject whose fingerprint is independent of its surface id.
fn subject_for(record_id: &str, origin_class: SurfaceOriginClass) -> KeyboardSurfaceSubject {
    KeyboardSurfaceSubject {
        surface_id: format!("surface:{record_id}"),
        origin_class,
        surface_fingerprint_token: format!("fp:surface:{record_id}"),
    }
}

fn markers(
    object_class: OrientationObjectClass,
    object_token: &str,
    marker_count: u32,
    rendered_count: u32,
    display_label: &str,
) -> OrientationMarkerSummary {
    OrientationMarkerSummary {
        object_class,
        object_token: object_token.to_owned(),
        marker_count,
        rendered_count,
        display_label: display_label.to_owned(),
    }
}

fn editor_core_fully_active_record() -> OrientationAidRecord {
    let record_id = "orientation:editor-core:0001";
    OrientationAidRecord::new(OrientationAidRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::EditorCore,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Editor-core multi-cursor count rendered fully live, every caret visible"
            .to_owned(),
        aid_kind: OrientationAidKind::MultiCursor,
        marker_summary: markers(
            OrientationObjectClass::EditorBuffer,
            "buffer:src/lib.rs",
            3,
            3,
            "3 cursors in src/lib.rs",
        ),
        aid_posture: OrientationAidClass::FullOrientationAids,
        cross_surface_identity_shared: false,
        constrained_viewport: false,
        reduced_motion_profile: false,
        large_or_unsafe_artifact: false,
        limited_capability_profile: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Editor-core multi-cursor aid verified to render all three carets live with an accurate count",
        ),
        resolution: OrientationDisclosureClass::AidFullyActive,
        fired_triggers: vec![],
        count_summary_label: None,
        identity_note: None,
        reduced_detail_label: None,
        motion_reduced_label: None,
        degrade_reason_label: None,
        unavailable_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
    })
}

fn notebook_count_summary_record() -> OrientationAidRecord {
    let record_id = "orientation:notebook:0001";
    OrientationAidRecord::new(OrientationAidRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::NotebookSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Notebook fold-state summary preserves an honest count when folds exceed the live-render budget"
                .to_owned(),
        aid_kind: OrientationAidKind::FoldState,
        marker_summary: markers(
            OrientationObjectClass::NotebookCellGroup,
            "cells:notebook/analysis#folds",
            60,
            40,
            "60 fold regions across notebook cells",
        ),
        aid_posture: OrientationAidClass::ReducedOrientationAidsHonest,
        cross_surface_identity_shared: false,
        constrained_viewport: false,
        reduced_motion_profile: false,
        large_or_unsafe_artifact: false,
        limited_capability_profile: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Notebook fold aid verified to render 40 fold gutters live and summarize the remaining 20 by an honest count",
        ),
        resolution: OrientationDisclosureClass::CountSummaryPreserved,
        fired_triggers: vec![OrientationContractTrigger::HighCardinalityMarkers],
        count_summary_label: Some(
            "60 fold regions exist; 40 are drawn in the gutter and the remaining 20 are kept as an honest '+20 more folds' count rather than dropped"
                .to_owned(),
        ),
        identity_note: None,
        reduced_detail_label: None,
        motion_reduced_label: None,
        degrade_reason_label: None,
        unavailable_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
    })
}

fn preview_identity_aligned_record() -> OrientationAidRecord {
    let record_id = "orientation:preview:0001";
    OrientationAidRecord::new(OrientationAidRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::PreviewSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Preview breadcrumb identity aligned with the same document object IDs shown in the editor and outline"
                .to_owned(),
        aid_kind: OrientationAidKind::Breadcrumb,
        marker_summary: markers(
            OrientationObjectClass::PreviewDocument,
            "document:preview/home#section-pricing",
            4,
            4,
            "Preview breadcrumb: home › sections › pricing › heading",
        ),
        aid_posture: OrientationAidClass::ReducedOrientationAidsHonest,
        cross_surface_identity_shared: true,
        constrained_viewport: false,
        reduced_motion_profile: false,
        large_or_unsafe_artifact: false,
        limited_capability_profile: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Preview breadcrumb verified to resolve each segment to the same document object ID the editor and outline use",
        ),
        resolution: OrientationDisclosureClass::IdentityAlignedAcrossSurfaces,
        fired_triggers: vec![OrientationContractTrigger::CrossSurfaceIdentityShared],
        count_summary_label: None,
        identity_note: Some(
            "Each breadcrumb segment resolves to the same preview-document object ID shown in the editor outline, so jumping from the breadcrumb lands on the identical object rather than a re-derived guess"
                .to_owned(),
        ),
        reduced_detail_label: None,
        motion_reduced_label: None,
        degrade_reason_label: None,
        unavailable_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
    })
}

fn data_api_reduced_detail_record() -> OrientationAidRecord {
    let record_id = "orientation:data-api:0001";
    OrientationAidRecord::new(OrientationAidRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DataApiSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Data/API minimap reduced to honest detail under a constrained viewport, disclosed rather than removed"
                .to_owned(),
        aid_kind: OrientationAidKind::Minimap,
        marker_summary: markers(
            OrientationObjectClass::DataResultGrid,
            "grid:query/orders-by-region#run-12",
            12,
            12,
            "Result-grid minimap with 12 section markers",
        ),
        aid_posture: OrientationAidClass::ReducedOrientationAidsHonest,
        cross_surface_identity_shared: false,
        constrained_viewport: true,
        reduced_motion_profile: false,
        large_or_unsafe_artifact: false,
        limited_capability_profile: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Data/API minimap verified to fall back to a narrow marker strip when the viewport is too narrow, with the reduction disclosed",
        ),
        resolution: OrientationDisclosureClass::ReducedDetailDisclosed,
        fired_triggers: vec![OrientationContractTrigger::ConstrainedViewport],
        count_summary_label: None,
        identity_note: None,
        reduced_detail_label: Some(
            "The viewport is too narrow for the full minimap thumbnail; it collapses to a labeled marker strip that still shows all 12 section markers and says it is reduced"
                .to_owned(),
        ),
        motion_reduced_label: None,
        degrade_reason_label: None,
        unavailable_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
    })
}

fn docs_motion_reduced_record() -> OrientationAidRecord {
    let record_id = "orientation:docs:0001";
    OrientationAidRecord::new(OrientationAidRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DocsSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Docs overview-ruler markers keep position but suppress animation under a reduced-motion profile, disclosed"
                .to_owned(),
        aid_kind: OrientationAidKind::OverviewRuler,
        marker_summary: markers(
            OrientationObjectClass::DocsOutline,
            "outline:docs/guide#getting-started",
            8,
            8,
            "Overview ruler with 8 heading and match markers",
        ),
        aid_posture: OrientationAidClass::OrientationAidsDegradedHonest,
        cross_surface_identity_shared: false,
        constrained_viewport: false,
        reduced_motion_profile: true,
        large_or_unsafe_artifact: false,
        limited_capability_profile: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Docs overview ruler verified to keep all marker positions while dropping the scroll-tracking animation under reduced motion, disclosed",
        ),
        resolution: OrientationDisclosureClass::MotionReducedDisclosed,
        fired_triggers: vec![OrientationContractTrigger::ReducedMotionProfile],
        count_summary_label: None,
        identity_note: None,
        reduced_detail_label: None,
        motion_reduced_label: Some(
            "Reduced-motion is on, so the overview ruler renders all 8 markers statically and drops the animated scroll-position glide, and says the animation was suppressed"
                .to_owned(),
        ),
        degrade_reason_label: None,
        unavailable_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
    })
}

fn review_degraded_record() -> OrientationAidRecord {
    let record_id = "orientation:review:0001";
    OrientationAidRecord::new(OrientationAidRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::ReviewSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Review-diff minimap degraded and disclosed for a large diff artifact, dropping stale markers rather than showing them"
                .to_owned(),
        aid_kind: OrientationAidKind::Minimap,
        marker_summary: markers(
            OrientationObjectClass::ReviewDiff,
            "diff:review/pr-204#large",
            1,
            1,
            "Review-diff minimap over a large pull-request diff",
        ),
        aid_posture: OrientationAidClass::OrientationAidsDegradedHonest,
        cross_surface_identity_shared: false,
        constrained_viewport: false,
        reduced_motion_profile: false,
        large_or_unsafe_artifact: true,
        limited_capability_profile: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Review-diff minimap verified to switch to a degraded summary band for a large diff and to drop markers it can no longer keep current",
        ),
        resolution: OrientationDisclosureClass::DegradedDisclosed,
        fired_triggers: vec![OrientationContractTrigger::LargeOrUnsafeArtifact],
        count_summary_label: None,
        identity_note: None,
        reduced_detail_label: None,
        motion_reduced_label: None,
        degrade_reason_label: Some(
            "The diff is too large to keep a live minimap; the aid degrades to a coarse change-density band and explicitly drops per-line markers it can no longer keep current rather than showing stale ones"
                .to_owned(),
        ),
        unavailable_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
    })
}

fn runtime_unavailable_record() -> OrientationAidRecord {
    let record_id = "orientation:runtime:0001";
    OrientationAidRecord::new(OrientationAidRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::RuntimeSurface,
        subject: subject_for(record_id, SurfaceOriginClass::EmbeddedRuntimeSurface),
        label_summary:
            "Browser-runtime overlay fold aid unavailable under a limited capability profile, disclosed rather than silently removed"
                .to_owned(),
        aid_kind: OrientationAidKind::FoldState,
        marker_summary: markers(
            OrientationObjectClass::RuntimeOverlay,
            "overlay:runtime/dev-server#dom-tree",
            1,
            1,
            "Browser-runtime DOM-tree fold overlay",
        ),
        aid_posture: OrientationAidClass::OrientationAidsDegradedHonest,
        cross_surface_identity_shared: false,
        constrained_viewport: false,
        reduced_motion_profile: false,
        large_or_unsafe_artifact: false,
        limited_capability_profile: true,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Browser-runtime overlay verified to mark its fold aid unavailable under the limited embedded profile and to say so in place rather than leaving a blank gutter",
        ),
        resolution: OrientationDisclosureClass::UnavailableDisclosed,
        fired_triggers: vec![OrientationContractTrigger::LimitedCapabilityProfile],
        count_summary_label: None,
        identity_note: None,
        reduced_detail_label: None,
        motion_reduced_label: None,
        degrade_reason_label: None,
        unavailable_reason_label: Some(
            "The embedded browser-runtime profile cannot host the DOM-tree fold overlay; the aid is marked unavailable in place with a reason, never silently removed from the gutter"
                .to_owned(),
        ),
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
    })
}

fn companion_identity_aligned_record() -> OrientationAidRecord {
    let record_id = "orientation:companion:0001";
    OrientationAidRecord::new(OrientationAidRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::CompanionSurface,
        subject: subject_for(record_id, SurfaceOriginClass::ProviderLinkedSurface),
        label_summary:
            "Provider-linked companion breadcrumb aligns identity across surfaces; imported orientation proof never reads as local"
                .to_owned(),
        aid_kind: OrientationAidKind::Breadcrumb,
        marker_summary: markers(
            OrientationObjectClass::CompanionTranscript,
            "transcript:companion/assistant#thread-77",
            3,
            3,
            "Companion breadcrumb: assistant › thread 77 › message",
        ),
        aid_posture: OrientationAidClass::ReducedOrientationAidsHonest,
        cross_surface_identity_shared: true,
        constrained_viewport: false,
        reduced_motion_profile: false,
        large_or_unsafe_artifact: false,
        limited_capability_profile: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::ImportedCurrent,
            "Provider-backed companion breadcrumb verified with imported proof to resolve each segment to the same provider thread object ID, labeled imported rather than local",
        ),
        resolution: OrientationDisclosureClass::IdentityAlignedAcrossSurfaces,
        fired_triggers: vec![OrientationContractTrigger::CrossSurfaceIdentityShared],
        count_summary_label: None,
        identity_note: Some(
            "Each companion breadcrumb segment resolves to the same provider-backed thread object ID shown in the companion list; the alignment is provider-backed and labeled imported, never presented as a locally verified breadcrumb"
                .to_owned(),
        ),
        reduced_detail_label: None,
        motion_reduced_label: None,
        degrade_reason_label: None,
        unavailable_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
    })
}

/// Packet id minted by [`fixture_orientation_aid_packet`].
pub const FIXTURE_ORIENTATION_AID_PACKET_ID: &str =
    "m5-orientation-aids:fixture:stale-proof-forces-degraded:0001";

/// Builds the protected fixture variant: it keeps the full seeded record set —
/// including the clean fully-active baseline — and adds one drill record for an
/// editor-core multi-cursor aid that would otherwise be fully active but is forced
/// off the flat lane because its orientation proof aged outside the freshness
/// window.
///
/// The fixture is a *valid* packet: the drill record correctly records the
/// [`OrientationContractTrigger::StaleOrMissingOrientationProof`] trigger and
/// resolves to [`OrientationDisclosureClass::DegradedDisclosed`] with a precise
/// degrade-reason label, so it validates while demonstrating that stale evidence —
/// not just high cardinality, a shared identity, a constrained viewport, reduced
/// motion, a large artifact, or a limited profile — forces an aid off the flat
/// fully-active lane and drops stale markers rather than showing them.
pub fn fixture_orientation_aid_packet() -> OrientationAidPacket {
    let mut packet = seeded_orientation_aid_packet();
    packet.packet_id = FIXTURE_ORIENTATION_AID_PACKET_ID.to_owned();
    packet.label =
        "M5 Orientation Aids fixture: stale orientation proof forces a fully-active aid into a disclosed degraded state"
            .to_owned();
    packet.records.push(stale_proof_drill_record());
    packet
}

/// An editor-core multi-cursor aid that would render fully live, but whose
/// orientation proof has aged outside its freshness window, so it is forced into a
/// disclosed degraded state that drops stale markers.
fn stale_proof_drill_record() -> OrientationAidRecord {
    let record_id = "orientation:editor-core:stale-proof:0001";
    OrientationAidRecord::new(OrientationAidRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::EditorCore,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Editor-core multi-cursor aid whose stale orientation proof forces a disclosed degraded state"
                .to_owned(),
        aid_kind: OrientationAidKind::MultiCursor,
        marker_summary: markers(
            OrientationObjectClass::EditorBuffer,
            "buffer:src/lib.rs#stale",
            5,
            5,
            "5 cursors in src/lib.rs with stale proof",
        ),
        aid_posture: OrientationAidClass::OrientationAidsDegradedHonest,
        cross_surface_identity_shared: false,
        constrained_viewport: false,
        reduced_motion_profile: false,
        large_or_unsafe_artifact: false,
        limited_capability_profile: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::StaleExpired,
            "Editor-core multi-cursor aid proof aged outside its freshness window",
        ),
        resolution: OrientationDisclosureClass::DegradedDisclosed,
        fired_triggers: vec![OrientationContractTrigger::StaleOrMissingOrientationProof],
        count_summary_label: None,
        identity_note: None,
        reduced_detail_label: None,
        motion_reduced_label: None,
        degrade_reason_label: Some(
            "Orientation proof aged outside its freshness window; the multi-cursor aid degrades to a re-verifying state and drops its prior caret markers rather than showing them as if current"
                .to_owned(),
        ),
        unavailable_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_ORIENTATION_AID_MINTED_AT.to_owned(),
    })
}
