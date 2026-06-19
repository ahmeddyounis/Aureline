//! Append-only anchor-remap history for M5 findings that move under edits, cell
//! changes, generator churn, or imported-snapshot comparison.
//!
//! The normalization step in
//! [`crate::normalize_m5_diagnostic_records_with_stable_ids_and_suppression_baseline_joins`]
//! binds each M5 finding to one canonical
//! [`DiagnosticRecord`](crate::diagnostics::DiagnosticRecord) whose
//! [`DiagnosticAnchorRemap`](crate::diagnostics::DiagnosticAnchorRemap) carries the
//! finding's *current* anchor and remap state. That single record answers "where
//! is this finding now"; it does not answer "how did it get here". When a file is
//! edited, a notebook cell is re-identified, a generated artifact churns, or an
//! imported scan is compared against a later revision, the finding's anchor can
//! drift — and without an explicit, append-only trail that drift is silently
//! dropped, "fixed", or relabeled.
//!
//! This module owns that trail. An [`AnchorRemapHistory`] is the append-only
//! sequence of [`AnchorRemapHistoryEntry`] records for one anchor family: each
//! entry pairs an old anchor ref with a new anchor ref, the resulting remap state
//! ([`DiagnosticAnchorRemapStateClass`] — `exact`, `contextual`, `stale`,
//! `unmapped`, or `imported_static`), the typed evidence basis that admitted the
//! remap, a [`RevisionPair`] naming the from/to revisions, the
//! [`AnchorRemapActorClass`] and actor/tool ref that produced it, and the
//! [`AnchorDriftLaneClass`] that names *where* the drift happened (a file edit, a
//! notebook cell identity change, generated-artifact churn, or an imported
//! snapshot/replay comparison). The genesis entry records the finding's original
//! anchor; every later entry appends one transition.
//!
//! The three guarantees this delivery owns:
//!
//! 1. **Drift is explicit, never silent.** Anchor drift moves to one of the five
//!    explicit remap states with a typed evidence basis; it is never dropped,
//!    silently repaired, or relabeled. A row that claims `exact` must carry
//!    [`AnchorRemapEvidenceBasisClass::ExactRangePreserved`] evidence — an anchor
//!    cannot jump back to exact without it.
//! 2. **History is append-only and exportable.** Entries are sequence-ordered with
//!    continuous revision pairs and a continuous anchor chain, so support and
//!    review flows get a causal trail for every moved finding rather than a single
//!    overwritten "current" state.
//! 3. **One vocabulary across every lane.** File edits, notebook cell identity
//!    changes, generated-artifact churn, and imported scan/replay comparisons all
//!    reuse the same five remap states and the same evidence-basis vocabulary,
//!    rather than each inventing feature-specific drift states. The
//!    `imported_static` state and flag carry snapshot-only mappings that have not
//!    been locally revalidated.
//!
//! [`AnchorRemapHistorySetPacket::validate`] refuses a packet whose history is not
//! append-only, silently repairs an anchor (a remap state that disagrees with its
//! evidence basis), breaks the anchor or revision chain, lets a `current` state
//! disagree with the latest entry, renders an imported-static mapping
//! inconsistently, or hides the remap history from a required editor, Problems,
//! review, CLI, or support surface.
//!
//! Raw source bytes, raw provider payloads, raw scanner reports, credentials, and
//! raw artifact bodies never cross this boundary; the packet carries only typed
//! class tokens, booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/quality/anchor-remap-record.schema.json`](../../../../schemas/quality/anchor-remap-record.schema.json).
//! The reviewer-facing doc is
//! [`docs/help/anchor-remap-and-diagnostic-drift.md`](../../../../docs/help/anchor-remap-and-diagnostic-drift.md).
//! The protected fixture directory is
//! [`fixtures/quality/m5/anchor-remap/`](../../../../fixtures/quality/m5/anchor-remap/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::diagnostics::{
    DiagnosticAnchorRemap, DiagnosticAnchorRemapStateClass, DiagnosticRedactionClass,
    DiagnosticSurfaceClass,
};

/// Stable record-kind tag carried by [`AnchorRemapHistorySetPacket`].
pub const M5_ANCHOR_REMAP_HISTORY_SET_RECORD_KIND: &str = "m5_anchor_remap_history_set";

/// Stable record-kind tag for an [`AnchorRemapHistory`].
pub const M5_ANCHOR_REMAP_HISTORY_RECORD_KIND: &str = "m5_anchor_remap_history";

/// Stable record-kind tag for an [`AnchorRemapHistoryEntry`].
pub const M5_ANCHOR_REMAP_HISTORY_ENTRY_RECORD_KIND: &str = "m5_anchor_remap_history_entry";

/// Stable record-kind tag for an [`AnchorRemapSurfaceProjection`].
pub const M5_ANCHOR_REMAP_SURFACE_PROJECTION_RECORD_KIND: &str =
    "m5_anchor_remap_surface_projection";

/// Stable record-kind tag for an [`AnchorRemapSupportExport`].
pub const M5_ANCHOR_REMAP_SUPPORT_EXPORT_RECORD_KIND: &str = "m5_anchor_remap_support_export";

/// Schema version for the M5 anchor-remap history set.
pub const M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_REF: &str =
    "schemas/quality/anchor-remap-record.schema.json";

/// Repo-relative path of the reviewer-facing doc.
pub const M5_ANCHOR_REMAP_HISTORY_SET_DOC_REF: &str =
    "docs/help/anchor-remap-and-diagnostic-drift.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ANCHOR_REMAP_HISTORY_SET_ARTIFACT_REF: &str =
    "artifacts/m5/diagnostics/anchor-remap-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_ANCHOR_REMAP_HISTORY_SET_SUMMARY_REF: &str =
    "artifacts/m5/diagnostics/anchor-remap-proof/support_export.md";

/// Repo-relative path of the canonical normalized diagnostic-record set schema
/// this history lane sits above rather than replaces.
pub const CANONICAL_DIAGNOSTIC_RECORD_SET_SCHEMA_REF: &str =
    "schemas/quality/diagnostic-record.schema.json";

/// Consumer surfaces that must expose anchor-remap state and history so a user can
/// tell whether a finding still maps cleanly or only contextually survives.
pub const REMAP_EXPOSURE_SURFACES: [DiagnosticSurfaceClass; 5] = [
    DiagnosticSurfaceClass::Editor,
    DiagnosticSurfaceClass::Problems,
    DiagnosticSurfaceClass::Review,
    DiagnosticSurfaceClass::CliExplain,
    DiagnosticSurfaceClass::SupportExport,
];

/// The lane where a finding's anchor drifted.
///
/// Every M5 finding surface that can move an anchor reuses this one vocabulary so a
/// notebook cell change, a generated-artifact churn, and an imported scan
/// comparison do not each invent feature-specific drift states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorDriftLaneClass {
    /// A live edit to a file moved or removed the anchored range.
    FileEdit,
    /// A notebook cell's identity changed (split, merge, reorder, or re-key).
    NotebookCellIdentityChange,
    /// A generated artifact was regenerated and its anchored region churned.
    GeneratedArtifactChurn,
    /// An imported scan snapshot was compared against a later local revision.
    ImportedSnapshotComparison,
    /// A replayed support bundle was compared against a later local revision.
    ImportedReplayComparison,
}

impl AnchorDriftLaneClass {
    /// Every drift lane, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FileEdit,
        Self::NotebookCellIdentityChange,
        Self::GeneratedArtifactChurn,
        Self::ImportedSnapshotComparison,
        Self::ImportedReplayComparison,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileEdit => "file_edit",
            Self::NotebookCellIdentityChange => "notebook_cell_identity_change",
            Self::GeneratedArtifactChurn => "generated_artifact_churn",
            Self::ImportedSnapshotComparison => "imported_snapshot_comparison",
            Self::ImportedReplayComparison => "imported_replay_comparison",
        }
    }

    /// Returns true when the drift came from comparing imported or replayed
    /// evidence rather than from a live local change.
    pub const fn is_imported(self) -> bool {
        matches!(
            self,
            Self::ImportedSnapshotComparison | Self::ImportedReplayComparison
        )
    }
}

/// The typed evidence basis that admitted one remap.
///
/// Each basis maps to exactly one resulting remap state, so a row cannot claim a
/// state its evidence does not support — the guarantee that anchor drift is never
/// silently repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorRemapEvidenceBasisClass {
    /// The exact anchored range survived the change unmodified.
    ExactRangePreserved,
    /// The range was re-located by matching surrounding context after the change.
    SurroundingContextMatch,
    /// No newer mapping was found; the anchor is retained against an older epoch.
    StaleEpochRetained,
    /// The anchor could not be located in the new revision.
    NoMappingFound,
    /// An imported snapshot's static location has not been locally revalidated.
    ImportedStaticLocation,
}

impl AnchorRemapEvidenceBasisClass {
    /// Every evidence basis, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExactRangePreserved,
        Self::SurroundingContextMatch,
        Self::StaleEpochRetained,
        Self::NoMappingFound,
        Self::ImportedStaticLocation,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRangePreserved => "exact_range_preserved",
            Self::SurroundingContextMatch => "surrounding_context_match",
            Self::StaleEpochRetained => "stale_epoch_retained",
            Self::NoMappingFound => "no_mapping_found",
            Self::ImportedStaticLocation => "imported_static_location",
        }
    }

    /// The single remap state this evidence basis can admit.
    pub const fn resulting_state(self) -> DiagnosticAnchorRemapStateClass {
        match self {
            Self::ExactRangePreserved => DiagnosticAnchorRemapStateClass::Exact,
            Self::SurroundingContextMatch => DiagnosticAnchorRemapStateClass::Contextual,
            Self::StaleEpochRetained => DiagnosticAnchorRemapStateClass::Stale,
            Self::NoMappingFound => DiagnosticAnchorRemapStateClass::Unmapped,
            Self::ImportedStaticLocation => DiagnosticAnchorRemapStateClass::ImportedStatic,
        }
    }
}

/// The actor or tool that produced one remap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorRemapActorClass {
    /// The editor's edit tracker re-anchored against a live buffer change.
    EditorEditTracker,
    /// The notebook cell tracker re-anchored against a cell identity change.
    NotebookCellTracker,
    /// The generated-artifact reprojector re-anchored against generator churn.
    GeneratedArtifactReprojector,
    /// The imported-scan comparator mapped a snapshot finding onto a local range.
    ImportedScanComparator,
    /// The replay comparator mapped a replayed-bundle finding onto a local range.
    ReplayComparator,
}

impl AnchorRemapActorClass {
    /// Every actor class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EditorEditTracker,
        Self::NotebookCellTracker,
        Self::GeneratedArtifactReprojector,
        Self::ImportedScanComparator,
        Self::ReplayComparator,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorEditTracker => "editor_edit_tracker",
            Self::NotebookCellTracker => "notebook_cell_tracker",
            Self::GeneratedArtifactReprojector => "generated_artifact_reprojector",
            Self::ImportedScanComparator => "imported_scan_comparator",
            Self::ReplayComparator => "replay_comparator",
        }
    }
}

/// The from/to revision pair that frames one remap.
///
/// The from-revision is the revision the old anchor was valid against; the
/// to-revision is the revision the new anchor is valid against. Continuity across
/// entries — each entry's from-revision equals the prior entry's to-revision —
/// keeps the history append-only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevisionPair {
    /// Revision the old anchor was valid against.
    pub from_revision_ref: String,
    /// Revision the new anchor is valid against.
    pub to_revision_ref: String,
}

impl RevisionPair {
    /// Builds a revision pair from opaque revision refs.
    pub fn new(from_revision_ref: impl Into<String>, to_revision_ref: impl Into<String>) -> Self {
        Self {
            from_revision_ref: from_revision_ref.into(),
            to_revision_ref: to_revision_ref.into(),
        }
    }

    /// Whether both revision refs are present.
    pub fn is_complete(&self) -> bool {
        !self.from_revision_ref.trim().is_empty() && !self.to_revision_ref.trim().is_empty()
    }
}

/// Constructor input for one append-only [`AnchorRemapHistoryEntry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnchorRemapHistoryEntryInput {
    /// Stable entry id.
    pub entry_id: String,
    /// Append-only sequence (0-based, contiguous within a history).
    pub sequence: u32,
    /// Lane where the drift occurred.
    pub drift_lane_class: AnchorDriftLaneClass,
    /// Old anchor ref, when one existed before this transition.
    pub old_anchor_ref: Option<String>,
    /// New anchor ref, when one can be shown after this transition.
    pub new_anchor_ref: Option<String>,
    /// Typed evidence basis admitting the remap.
    pub evidence_basis_class: AnchorRemapEvidenceBasisClass,
    /// Opaque ref to the evidence that admitted the remap.
    pub evidence_basis_ref: String,
    /// Revision pair framing the remap.
    pub revision_pair: RevisionPair,
    /// Actor class that produced the remap.
    pub actor_class: AnchorRemapActorClass,
    /// Opaque actor/tool ref that produced the remap.
    pub actor_tool_ref: String,
    /// Production timestamp.
    pub produced_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

/// One append-only entry in an anchor-remap history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRemapHistoryEntry {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable entry id.
    pub entry_id: String,
    /// Append-only sequence (0-based, contiguous within a history).
    pub sequence: u32,
    /// Lane where the drift occurred.
    pub drift_lane_class: AnchorDriftLaneClass,
    /// Old anchor ref, when one existed before this transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_anchor_ref: Option<String>,
    /// New anchor ref, when one can be shown after this transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_anchor_ref: Option<String>,
    /// Resulting remap state, derived from the evidence basis.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Typed evidence basis admitting the remap.
    pub evidence_basis_class: AnchorRemapEvidenceBasisClass,
    /// Opaque ref to the evidence that admitted the remap.
    pub evidence_basis_ref: String,
    /// Revision pair framing the remap.
    pub revision_pair: RevisionPair,
    /// Actor class that produced the remap.
    pub actor_class: AnchorRemapActorClass,
    /// Opaque actor/tool ref that produced the remap.
    pub actor_tool_ref: String,
    /// True when this entry carries a snapshot-only imported-static mapping.
    pub imported_static: bool,
    /// Production timestamp.
    pub produced_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl AnchorRemapHistoryEntry {
    /// Builds an append-only history entry, deriving the remap state from the
    /// evidence basis and the imported-static flag from the resulting state.
    pub fn new(input: AnchorRemapHistoryEntryInput) -> Self {
        let remap_state_class = input.evidence_basis_class.resulting_state();
        Self {
            record_kind: M5_ANCHOR_REMAP_HISTORY_ENTRY_RECORD_KIND.to_owned(),
            schema_version: M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_VERSION,
            entry_id: input.entry_id,
            sequence: input.sequence,
            drift_lane_class: input.drift_lane_class,
            old_anchor_ref: input.old_anchor_ref,
            new_anchor_ref: input.new_anchor_ref,
            remap_state_class,
            evidence_basis_class: input.evidence_basis_class,
            evidence_basis_ref: input.evidence_basis_ref,
            revision_pair: input.revision_pair,
            actor_class: input.actor_class,
            actor_tool_ref: input.actor_tool_ref,
            imported_static: remap_state_class == DiagnosticAnchorRemapStateClass::ImportedStatic,
            produced_at: input.produced_at,
            export_safe_summary: input.export_safe_summary,
        }
    }

    /// Whether the resulting remap state matches the evidence basis — the
    /// no-silent-repair check.
    pub fn state_matches_basis(&self) -> bool {
        self.remap_state_class == self.evidence_basis_class.resulting_state()
    }

    /// Whether the finding still maps to an exact range after this transition.
    pub fn maps_cleanly(&self) -> bool {
        self.remap_state_class == DiagnosticAnchorRemapStateClass::Exact
    }

    /// Whether the finding only contextually survives after this transition.
    pub fn only_contextually_survives(&self) -> bool {
        self.remap_state_class == DiagnosticAnchorRemapStateClass::Contextual
    }

    /// Whether the finding's anchor was dropped (no current range) by this
    /// transition.
    pub fn is_dropped(&self) -> bool {
        self.remap_state_class == DiagnosticAnchorRemapStateClass::Unmapped
    }

    /// Whether a surface must disclose this remap state.
    pub fn requires_disclosure(&self) -> bool {
        self.remap_state_class.requires_disclosure()
    }

    /// Whether the new-anchor presence agrees with the remap state: an unmapped
    /// transition has no new anchor; every other state names one.
    pub fn anchor_consistency(&self) -> bool {
        match self.remap_state_class {
            DiagnosticAnchorRemapStateClass::Unmapped => self.new_anchor_ref.is_none(),
            _ => self
                .new_anchor_ref
                .as_ref()
                .is_some_and(|value| !value.trim().is_empty()),
        }
    }

    /// Whether the imported-static flag agrees with the state and lane: the flag
    /// is set exactly for the imported-static state, which only an imported lane
    /// can produce.
    pub fn imported_static_consistency(&self) -> bool {
        let is_imported_static =
            self.remap_state_class == DiagnosticAnchorRemapStateClass::ImportedStatic;
        if self.imported_static != is_imported_static {
            return false;
        }
        !is_imported_static || self.drift_lane_class.is_imported()
    }

    /// Whether this entry holds every structural invariant.
    pub fn is_structurally_complete(&self) -> bool {
        !self.entry_id.trim().is_empty()
            && !self.evidence_basis_ref.trim().is_empty()
            && !self.actor_tool_ref.trim().is_empty()
            && !self.produced_at.trim().is_empty()
            && !self.export_safe_summary.trim().is_empty()
            && self.revision_pair.is_complete()
            && self.state_matches_basis()
            && self.anchor_consistency()
            && self.imported_static_consistency()
    }
}

/// Constructor input for one [`AnchorRemapHistory`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnchorRemapHistoryInput {
    /// Stable history id.
    pub history_id: String,
    /// Anchor family the history tracks.
    pub anchor_family_id: String,
    /// Canonical diagnostic id the anchor belongs to.
    pub diagnostic_id: String,
    /// Append-only entries, ordered by sequence.
    pub entries: Vec<AnchorRemapHistoryEntry>,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

/// The append-only remap history for one anchor family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRemapHistory {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable history id.
    pub history_id: String,
    /// Anchor family the history tracks.
    pub anchor_family_id: String,
    /// Canonical diagnostic id the anchor belongs to.
    pub diagnostic_id: String,
    /// Append-only entries, ordered by sequence.
    pub entries: Vec<AnchorRemapHistoryEntry>,
    /// Append-only marker; must stay `true`.
    pub append_only: bool,
    /// Current remap state, derived from the latest entry.
    pub current_state_class: DiagnosticAnchorRemapStateClass,
    /// Current anchor ref, derived from the latest entry, when one can be shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_anchor_ref: Option<String>,
    /// Distinct drift lanes represented across the entries.
    pub drift_lanes: Vec<AnchorDriftLaneClass>,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl AnchorRemapHistory {
    /// Builds an append-only history, deriving the current state, current anchor,
    /// and represented drift lanes from the latest entry and entry set.
    pub fn new(input: AnchorRemapHistoryInput) -> Self {
        let current_state_class = input
            .entries
            .last()
            .map(|entry| entry.remap_state_class)
            .unwrap_or(DiagnosticAnchorRemapStateClass::Unmapped);
        let current_anchor_ref = input
            .entries
            .last()
            .and_then(|entry| entry.new_anchor_ref.clone());
        let drift_lanes = distinct_sorted(input.entries.iter().map(|entry| entry.drift_lane_class));
        Self {
            record_kind: M5_ANCHOR_REMAP_HISTORY_RECORD_KIND.to_owned(),
            schema_version: M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_VERSION,
            history_id: input.history_id,
            anchor_family_id: input.anchor_family_id,
            diagnostic_id: input.diagnostic_id,
            entries: input.entries,
            append_only: true,
            current_state_class,
            current_anchor_ref,
            drift_lanes,
            export_safe_summary: input.export_safe_summary,
        }
    }

    /// The genesis (first) entry, when the history is non-empty.
    pub fn genesis_entry(&self) -> Option<&AnchorRemapHistoryEntry> {
        self.entries.first()
    }

    /// The current (latest) entry, when the history is non-empty.
    pub fn current_entry(&self) -> Option<&AnchorRemapHistoryEntry> {
        self.entries.last()
    }

    /// Whether the entries carry contiguous 0-based sequence numbers.
    pub fn sequence_is_monotonic(&self) -> bool {
        self.entries
            .iter()
            .enumerate()
            .all(|(index, entry)| entry.sequence as usize == index)
    }

    /// Whether each entry's from-revision continues the prior entry's
    /// to-revision.
    pub fn revisions_are_continuous(&self) -> bool {
        self.entries.windows(2).all(|pair| {
            pair[0].revision_pair.to_revision_ref == pair[1].revision_pair.from_revision_ref
        })
    }

    /// Whether each entry's old anchor continues the prior entry's new anchor.
    pub fn anchor_chain_is_continuous(&self) -> bool {
        self.entries
            .windows(2)
            .all(|pair| pair[0].new_anchor_ref == pair[1].old_anchor_ref)
    }

    /// Whether every entry's remap state matches its evidence basis — no entry
    /// silently repaired an anchor.
    pub fn no_silent_repair(&self) -> bool {
        self.entries
            .iter()
            .all(AnchorRemapHistoryEntry::state_matches_basis)
    }

    /// Whether the stored current state and anchor agree with the latest entry.
    pub fn current_state_consistent(&self) -> bool {
        match self.current_entry() {
            Some(entry) => {
                self.current_state_class == entry.remap_state_class
                    && self.current_anchor_ref == entry.new_anchor_ref
            }
            None => false,
        }
    }

    /// Whether the stored drift-lane set agrees with the entries.
    pub fn drift_lanes_consistent(&self) -> bool {
        self.drift_lanes == distinct_sorted(self.entries.iter().map(|entry| entry.drift_lane_class))
    }

    /// Whether the history is append-only: explicitly marked, sequence-contiguous,
    /// and revision-continuous.
    pub fn is_append_only(&self) -> bool {
        self.append_only && self.sequence_is_monotonic() && self.revisions_are_continuous()
    }

    /// Whether this history demonstrates explicit drift: an anchor that moved at
    /// least once and landed in or passed through a non-exact state with evidence.
    pub fn demonstrates_explicit_drift(&self) -> bool {
        self.entries.len() >= 2
            && self.entries.iter().any(|entry| {
                entry.remap_state_class != DiagnosticAnchorRemapStateClass::Exact
                    && !entry.evidence_basis_ref.trim().is_empty()
            })
    }

    /// Whether this history holds every structural invariant.
    pub fn is_structurally_complete(&self) -> bool {
        !self.history_id.trim().is_empty()
            && !self.anchor_family_id.trim().is_empty()
            && !self.diagnostic_id.trim().is_empty()
            && !self.export_safe_summary.trim().is_empty()
            && !self.entries.is_empty()
            && self
                .entries
                .iter()
                .all(AnchorRemapHistoryEntry::is_structurally_complete)
            && self.append_only
            && self.sequence_is_monotonic()
            && self.revisions_are_continuous()
            && self.anchor_chain_is_continuous()
            && self.no_silent_repair()
            && self.current_state_consistent()
            && self.drift_lanes_consistent()
    }

    /// Projects the latest entry into a canonical
    /// [`DiagnosticAnchorRemap`](crate::diagnostics::DiagnosticAnchorRemap) so the
    /// shared diagnostic plane reuses this history's current state rather than
    /// forking its own.
    pub fn current_anchor_remap(&self) -> Option<DiagnosticAnchorRemap> {
        let genesis = self.genesis_entry()?;
        let current = self.current_entry()?;
        let mut remap = DiagnosticAnchorRemap::new(
            format!("remap:{}", self.anchor_family_id),
            self.anchor_family_id.clone(),
            genesis.old_anchor_ref.clone(),
            current.new_anchor_ref.clone(),
            current.remap_state_class,
            current.evidence_basis_ref.clone(),
            current.produced_at.clone(),
            current.export_safe_summary.clone(),
        );
        remap.source_revision_ref = Some(genesis.revision_pair.from_revision_ref.clone());
        remap.current_revision_ref = Some(current.revision_pair.to_revision_ref.clone());
        remap.actor_tool_ref = Some(current.actor_tool_ref.clone());
        Some(remap)
    }

    /// Builds the cross-surface projection of this history for one surface.
    pub fn surface_projection(
        &self,
        surface_class: DiagnosticSurfaceClass,
    ) -> AnchorRemapSurfaceProjection {
        let current = self.current_entry();
        AnchorRemapSurfaceProjection {
            record_kind: M5_ANCHOR_REMAP_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_VERSION,
            projection_id: format!(
                "anchor_remap_projection:{}:{}",
                surface_class.as_str(),
                sanitize_id(&self.history_id)
            ),
            history_id: self.history_id.clone(),
            anchor_family_id: self.anchor_family_id.clone(),
            diagnostic_id: self.diagnostic_id.clone(),
            surface_class,
            current_state_class: self.current_state_class,
            current_anchor_ref: self.current_anchor_ref.clone(),
            maps_cleanly: self.current_state_class == DiagnosticAnchorRemapStateClass::Exact,
            only_contextually_survives: self.current_state_class
                == DiagnosticAnchorRemapStateClass::Contextual,
            dropped: self.current_state_class == DiagnosticAnchorRemapStateClass::Unmapped,
            imported_static: self.current_state_class
                == DiagnosticAnchorRemapStateClass::ImportedStatic,
            entry_count: self.entries.len(),
            latest_evidence_basis_class: current.map(|entry| entry.evidence_basis_class),
            latest_drift_lane_class: current.map(|entry| entry.drift_lane_class),
            disclosure_required: self.current_state_class.requires_disclosure(),
            exposes_remap_history: true,
            exposes_current_state: true,
            raw_source_content_included: false,
            raw_payload_included: false,
            export_safe_summary: format!(
                "{} projection exposes the {} current state and all {} remap entries of anchor family {}.",
                surface_class.as_str(),
                self.current_state_class.as_str(),
                self.entries.len(),
                self.anchor_family_id
            ),
        }
    }
}

/// Cross-surface projection of one history that exposes its current remap state
/// and full append-only trail so a user can tell how a finding moved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRemapSurfaceProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// History id projected.
    pub history_id: String,
    /// Anchor family the history tracks.
    pub anchor_family_id: String,
    /// Canonical diagnostic id the anchor belongs to.
    pub diagnostic_id: String,
    /// Surface consuming the projection.
    pub surface_class: DiagnosticSurfaceClass,
    /// Current remap state copied from the history.
    pub current_state_class: DiagnosticAnchorRemapStateClass,
    /// Current anchor ref copied from the history, when one can be shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_anchor_ref: Option<String>,
    /// Whether the finding still maps to an exact range.
    pub maps_cleanly: bool,
    /// Whether the finding only contextually survives.
    pub only_contextually_survives: bool,
    /// Whether the finding's anchor was dropped.
    pub dropped: bool,
    /// Whether the current mapping is an imported-static snapshot mapping.
    pub imported_static: bool,
    /// Number of append-only entries available to the surface.
    pub entry_count: usize,
    /// Evidence basis of the latest entry, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_evidence_basis_class: Option<AnchorRemapEvidenceBasisClass>,
    /// Drift lane of the latest entry, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_drift_lane_class: Option<AnchorDriftLaneClass>,
    /// Whether the current state requires visible disclosure.
    pub disclosure_required: bool,
    /// Whether this projection exposes the full append-only history.
    pub exposes_remap_history: bool,
    /// Whether this projection exposes the current remap state.
    pub exposes_current_state: bool,
    /// Whether raw source content is included in this projection.
    pub raw_source_content_included: bool,
    /// Whether raw payload content is included in this projection.
    pub raw_payload_included: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl AnchorRemapSurfaceProjection {
    /// Whether this projection exposes the remap history and current state without
    /// raw content and agrees with its source history.
    pub fn is_honest(&self, history: &AnchorRemapHistory) -> bool {
        self.exposes_remap_history
            && self.exposes_current_state
            && !self.raw_source_content_included
            && !self.raw_payload_included
            && self.current_state_class == history.current_state_class
            && self.current_anchor_ref == history.current_anchor_ref
            && self.entry_count == history.entries.len()
            && self.maps_cleanly
                == (history.current_state_class == DiagnosticAnchorRemapStateClass::Exact)
    }
}

/// One row of a history's preserved append-only trail in a support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRemapHistoryExportRow {
    /// History id this row preserves.
    pub history_id: String,
    /// Anchor family the history tracks.
    pub anchor_family_id: String,
    /// Canonical diagnostic id the anchor belongs to.
    pub diagnostic_id: String,
    /// Append-only entry ids preserved for the history, in order.
    pub entry_ids: Vec<String>,
    /// Current remap state preserved for the history.
    pub current_state_class: DiagnosticAnchorRemapStateClass,
}

/// Support export that preserves the full append-only remap trail per anchor
/// family so support and review flows get a causal trail for moved findings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRemapSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support export id.
    pub export_id: String,
    /// Workspace id covered by the export.
    pub workspace_id: String,
    /// History ids cited by the export.
    pub history_refs: Vec<String>,
    /// Per-history append-only trail preserved by the export.
    pub history_trails: Vec<AnchorRemapHistoryExportRow>,
    /// True when the export preserves each history's append-only trail.
    pub preserves_append_only_history: bool,
    /// Redaction posture for the export.
    pub redaction_class: DiagnosticRedactionClass,
    /// Whether raw source content is included by default.
    pub raw_source_content_included: bool,
    /// Whether raw payload content is included by default.
    pub raw_payload_included: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl AnchorRemapSupportExport {
    /// Builds a metadata-only support export from a set of histories.
    pub fn from_histories(
        export_id: impl Into<String>,
        workspace_id: impl Into<String>,
        histories: &[AnchorRemapHistory],
    ) -> Self {
        let history_refs = histories
            .iter()
            .map(|history| history.history_id.clone())
            .collect::<Vec<_>>();
        let history_trails = histories
            .iter()
            .map(|history| AnchorRemapHistoryExportRow {
                history_id: history.history_id.clone(),
                anchor_family_id: history.anchor_family_id.clone(),
                diagnostic_id: history.diagnostic_id.clone(),
                entry_ids: history
                    .entries
                    .iter()
                    .map(|entry| entry.entry_id.clone())
                    .collect(),
                current_state_class: history.current_state_class,
            })
            .collect::<Vec<_>>();
        let entry_total: usize = histories.iter().map(|history| history.entries.len()).sum();

        Self {
            record_kind: M5_ANCHOR_REMAP_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_VERSION,
            export_id: export_id.into(),
            workspace_id: workspace_id.into(),
            history_refs,
            history_trails,
            preserves_append_only_history: true,
            redaction_class: DiagnosticRedactionClass::MetadataSafeDefault,
            raw_source_content_included: false,
            raw_payload_included: false,
            export_safe_summary: format!(
                "Support export preserves {} anchor-remap histories and {} append-only entries with raw content omitted by default.",
                histories.len(),
                entry_total
            ),
        }
    }

    /// Whether the export preserves every history's id and ordered entry trail.
    pub fn preserves(&self, histories: &[AnchorRemapHistory]) -> bool {
        if !self.preserves_append_only_history {
            return false;
        }
        histories.iter().all(|history| {
            self.history_refs.contains(&history.history_id)
                && self.history_trails.iter().any(|row| {
                    row.history_id == history.history_id
                        && row.current_state_class == history.current_state_class
                        && row.entry_ids
                            == history
                                .entries
                                .iter()
                                .map(|entry| entry.entry_id.clone())
                                .collect::<Vec<_>>()
                })
        })
    }
}

/// Set-level guardrail invariants that must all hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRemapGuardrails {
    /// Anchor drift moves to explicit states with evidence, never silently
    /// dropped or relabeled.
    pub drift_never_silently_dropped: bool,
    /// File-edit, notebook, generated-artifact, and imported lanes reuse the same
    /// remap vocabulary.
    pub same_remap_vocabulary_across_lanes: bool,
    /// Remap history is append-only.
    pub history_is_append_only: bool,
    /// Remap history is exportable.
    pub history_is_exportable: bool,
    /// Imported-static state carries snapshot-only mappings.
    pub imported_static_supported_for_snapshot_only: bool,
    /// Anchors are never silently repaired: every remap names its evidence basis.
    pub no_silent_anchor_repair: bool,
    /// Every remap records a revision pair.
    pub revision_pair_recorded_per_remap: bool,
}

impl AnchorRemapGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.drift_never_silently_dropped
            && self.same_remap_vocabulary_across_lanes
            && self.history_is_append_only
            && self.history_is_exportable
            && self.imported_static_supported_for_snapshot_only
            && self.no_silent_anchor_repair
            && self.revision_pair_recorded_per_remap
    }
}

/// Declares which consumer surfaces expose remap state and history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRemapConsumerProjection {
    /// Editor decorations show the current remap state.
    pub editor_shows_remap_state: bool,
    /// Problems rows show the current remap state.
    pub problems_shows_remap_state: bool,
    /// Review annotations expose the append-only remap history.
    pub review_shows_remap_history: bool,
    /// CLI / headless output lists the current remap state.
    pub cli_shows_remap_state: bool,
    /// Support export preserves the append-only remap history.
    pub support_export_preserves_history: bool,
}

impl AnchorRemapConsumerProjection {
    /// Whether every consumer projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.editor_shows_remap_state
            && self.problems_shows_remap_state
            && self.review_shows_remap_history
            && self.cli_shows_remap_state
            && self.support_export_preserves_history
    }
}

/// Constructor input for an [`AnchorRemapHistorySetPacket`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnchorRemapHistorySetPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Workspace id covered by the set.
    pub workspace_id: String,
    /// Append-only histories in the set.
    pub histories: Vec<AnchorRemapHistory>,
    /// Guardrail invariants block.
    pub guardrails: AnchorRemapGuardrails,
    /// Consumer projection block.
    pub consumer_projection: AnchorRemapConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 anchor-remap history set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRemapHistorySetPacket {
    /// Record kind; must equal [`M5_ANCHOR_REMAP_HISTORY_SET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Workspace id covered by the set.
    pub workspace_id: String,
    /// Append-only histories in the set.
    pub histories: Vec<AnchorRemapHistory>,
    /// Cross-surface projections, one per history per exposure surface.
    pub surface_projections: Vec<AnchorRemapSurfaceProjection>,
    /// Default support export preserving the append-only trails.
    pub support_export: AnchorRemapSupportExport,
    /// Guardrail invariants block.
    pub guardrails: AnchorRemapGuardrails,
    /// Consumer projection block.
    pub consumer_projection: AnchorRemapConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AnchorRemapHistorySetPacket {
    /// Builds an M5 anchor-remap history set packet, deriving cross-surface
    /// projections and the default support export from the histories.
    pub fn new(input: AnchorRemapHistorySetPacketInput) -> Self {
        let surface_projections = input
            .histories
            .iter()
            .flat_map(|history| {
                REMAP_EXPOSURE_SURFACES
                    .into_iter()
                    .map(|surface| history.surface_projection(surface))
            })
            .collect::<Vec<_>>();
        let support_export = AnchorRemapSupportExport::from_histories(
            format!(
                "anchor_remap_support_export:{}",
                sanitize_id(&input.packet_id)
            ),
            input.workspace_id.clone(),
            &input.histories,
        );

        Self {
            record_kind: M5_ANCHOR_REMAP_HISTORY_SET_RECORD_KIND.to_owned(),
            schema_version: M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            workspace_id: input.workspace_id,
            histories: input.histories,
            surface_projections,
            support_export,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Distinct drift lanes represented across the set.
    pub fn represented_drift_lanes(&self) -> BTreeSet<AnchorDriftLaneClass> {
        self.histories
            .iter()
            .flat_map(|history| history.entries.iter().map(|entry| entry.drift_lane_class))
            .collect()
    }

    /// Distinct remap states represented across the set.
    pub fn represented_states(&self) -> BTreeSet<DiagnosticAnchorRemapStateClass> {
        self.histories
            .iter()
            .flat_map(|history| history.entries.iter().map(|entry| entry.remap_state_class))
            .collect()
    }

    /// Whether the set covers every drift lane.
    pub fn covers_all_drift_lanes(&self) -> bool {
        self.represented_drift_lanes().len() == AnchorDriftLaneClass::ALL.len()
    }

    /// The projection matching one history and surface, when present.
    pub fn projection_for(
        &self,
        history_id: &str,
        surface_class: DiagnosticSurfaceClass,
    ) -> Option<&AnchorRemapSurfaceProjection> {
        self.surface_projections.iter().find(|projection| {
            projection.history_id == history_id && projection.surface_class == surface_class
        })
    }

    /// Validates the M5 anchor-remap history set invariants.
    pub fn validate(&self) -> Vec<AnchorRemapViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ANCHOR_REMAP_HISTORY_SET_RECORD_KIND {
            violations.push(AnchorRemapViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_VERSION {
            violations.push(AnchorRemapViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.workspace_id.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AnchorRemapViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_histories(self, &mut violations);
        validate_support_export(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(AnchorRemapViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(AnchorRemapViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("anchor-remap history set serializes"),
        ) {
            violations.push(AnchorRemapViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("anchor-remap history set serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Anchor-Remap History Set\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!("- Workspace: `{}`\n", self.workspace_id));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(&format!("- Histories: {}\n", self.histories.len()));
        out.push_str(&format!(
            "- Drift lanes covered: {}\n\n",
            self.represented_drift_lanes().len()
        ));

        out.push_str(
            "| History | Anchor family | Entries | Current state | Current anchor | Lanes | Disclosure |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for history in &self.histories {
            out.push_str(&format!(
                "| `{}` | `{}` | {} | {} | {} | {} | {} |\n",
                history.history_id,
                history.anchor_family_id,
                history.entries.len(),
                history.current_state_class.as_str(),
                history.current_anchor_ref.as_deref().unwrap_or("(dropped)"),
                history
                    .drift_lanes
                    .iter()
                    .map(|lane| lane.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                history.current_state_class.requires_disclosure(),
            ));
        }

        out.push('\n');
        for history in &self.histories {
            out.push_str(&format!(
                "- `{}` — {} ({})\n",
                history.history_id, history.export_safe_summary, history.diagnostic_id
            ));
            for entry in &history.entries {
                out.push_str(&format!(
                    "  - [{}] {} / {} / {} → {}\n",
                    entry.sequence,
                    entry.drift_lane_class.as_str(),
                    entry.evidence_basis_class.as_str(),
                    entry.remap_state_class.as_str(),
                    entry.new_anchor_ref.as_deref().unwrap_or("(dropped)"),
                ));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum AnchorRemapArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<AnchorRemapViolation>),
}

impl fmt::Display for AnchorRemapArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(
                    f,
                    "anchor-remap history set support export parse error: {err}"
                )
            }
            Self::Validation(violations) => write!(
                f,
                "anchor-remap history set support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for AnchorRemapArtifactError {}

/// Invariant violations reported by [`AnchorRemapHistorySetPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorRemapViolation {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Packet identity fields are missing.
    MissingIdentity,
    /// Required canonical source contracts are missing.
    MissingSourceContracts,
    /// The set has no histories.
    NoHistories,
    /// A history failed its structural completeness invariants.
    HistoryStructurallyIncomplete,
    /// A history is not append-only (sequence or revision continuity broken).
    HistoryNotAppendOnly,
    /// A history's anchor chain is broken across entries.
    AnchorChainBroken,
    /// A remap state disagrees with its evidence basis (silent anchor repair).
    SilentAnchorRepair,
    /// A history's current state disagrees with its latest entry.
    CurrentStateInconsistent,
    /// An entry's imported-static flag, state, or lane disagree.
    ImportedStaticInconsistent,
    /// An entry's new anchor presence disagrees with its remap state.
    AnchorRefInconsistent,
    /// No history demonstrates explicit drift with evidence.
    ExplicitDriftProofMissing,
    /// A required exposure-surface projection is missing for a history.
    SurfaceProjectionMissing,
    /// A surface projection drops the remap history or current state.
    SurfaceProjectionDropsHistory,
    /// The support export lost an append-only trail.
    SupportExportLossy,
    /// The support export includes raw source or payload content by default.
    SupportExportIncludesRawContent,
    /// Guardrail block is incomplete.
    GuardrailsIncomplete,
    /// Consumer projection block is incomplete.
    ConsumerProjectionIncomplete,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl AnchorRemapViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoHistories => "no_histories",
            Self::HistoryStructurallyIncomplete => "history_structurally_incomplete",
            Self::HistoryNotAppendOnly => "history_not_append_only",
            Self::AnchorChainBroken => "anchor_chain_broken",
            Self::SilentAnchorRepair => "silent_anchor_repair",
            Self::CurrentStateInconsistent => "current_state_inconsistent",
            Self::ImportedStaticInconsistent => "imported_static_inconsistent",
            Self::AnchorRefInconsistent => "anchor_ref_inconsistent",
            Self::ExplicitDriftProofMissing => "explicit_drift_proof_missing",
            Self::SurfaceProjectionMissing => "surface_projection_missing",
            Self::SurfaceProjectionDropsHistory => "surface_projection_drops_history",
            Self::SupportExportLossy => "support_export_lossy",
            Self::SupportExportIncludesRawContent => "support_export_includes_raw_content",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked support-export artifact.
///
/// This is the canonical entry point downstream editor, Problems, review, CLI, and
/// support surfaces use to ingest the append-only remap history instead of forking
/// per-surface drift state.
///
/// # Errors
///
/// Returns [`AnchorRemapArtifactError`] when the artifact cannot be parsed or fails
/// validation.
pub fn current_m5_anchor_remap_history_set_export(
) -> Result<AnchorRemapHistorySetPacket, AnchorRemapArtifactError> {
    let packet: AnchorRemapHistorySetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/diagnostics/anchor-remap-proof/support_export.json"
    )))
    .map_err(AnchorRemapArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AnchorRemapArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &AnchorRemapHistorySetPacket,
    violations: &mut Vec<AnchorRemapViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_REF,
        M5_ANCHOR_REMAP_HISTORY_SET_DOC_REF,
        M5_ANCHOR_REMAP_HISTORY_SET_ARTIFACT_REF,
        CANONICAL_DIAGNOSTIC_RECORD_SET_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(AnchorRemapViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_histories(
    packet: &AnchorRemapHistorySetPacket,
    violations: &mut Vec<AnchorRemapViolation>,
) {
    if packet.histories.is_empty() {
        violations.push(AnchorRemapViolation::NoHistories);
    }

    for history in &packet.histories {
        if !history.is_structurally_complete() {
            violations.push(AnchorRemapViolation::HistoryStructurallyIncomplete);
        }
        if !history.is_append_only() {
            violations.push(AnchorRemapViolation::HistoryNotAppendOnly);
        }
        if !history.anchor_chain_is_continuous() {
            violations.push(AnchorRemapViolation::AnchorChainBroken);
        }
        if !history.no_silent_repair() {
            violations.push(AnchorRemapViolation::SilentAnchorRepair);
        }
        if !history.current_state_consistent() {
            violations.push(AnchorRemapViolation::CurrentStateInconsistent);
        }

        for entry in &history.entries {
            if !entry.imported_static_consistency() {
                violations.push(AnchorRemapViolation::ImportedStaticInconsistent);
            }
            if !entry.anchor_consistency() {
                violations.push(AnchorRemapViolation::AnchorRefInconsistent);
            }
        }

        for surface_class in REMAP_EXPOSURE_SURFACES {
            match packet.projection_for(&history.history_id, surface_class) {
                Some(projection) => {
                    if !projection.is_honest(history) {
                        violations.push(AnchorRemapViolation::SurfaceProjectionDropsHistory);
                    }
                }
                None => violations.push(AnchorRemapViolation::SurfaceProjectionMissing),
            }
        }
    }

    if !packet.histories.is_empty()
        && !packet
            .histories
            .iter()
            .any(AnchorRemapHistory::demonstrates_explicit_drift)
    {
        violations.push(AnchorRemapViolation::ExplicitDriftProofMissing);
    }
}

fn validate_support_export(
    packet: &AnchorRemapHistorySetPacket,
    violations: &mut Vec<AnchorRemapViolation>,
) {
    if packet.support_export.raw_source_content_included
        || packet.support_export.raw_payload_included
    {
        violations.push(AnchorRemapViolation::SupportExportIncludesRawContent);
    }
    if !packet.support_export.preserves(&packet.histories) {
        violations.push(AnchorRemapViolation::SupportExportLossy);
    }
}

fn distinct_sorted<T>(values: impl Iterator<Item = T>) -> Vec<T>
where
    T: Ord,
{
    values.collect::<BTreeSet<_>>().into_iter().collect()
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
