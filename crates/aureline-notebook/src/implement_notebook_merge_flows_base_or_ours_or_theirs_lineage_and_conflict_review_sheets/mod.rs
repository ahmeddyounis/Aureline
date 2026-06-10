//! Notebook merge flows, base/ours/theirs lineage, and conflict-review sheets.
//!
//! This module materializes the typed records that keep notebook merge,
//! lineage, and conflict review honest about base/ours/theirs/result
//! provenance, cell-aware resolution, and downgrade posture. The records and
//! closed vocabularies here mirror the boundary schema at
//! `/schemas/notebook/implement_notebook_merge_flows_base_or_ours_or_theirs_lineage_and_conflict_review_sheets.schema.json`
//! and reuse the merge-resolution vocabulary already frozen in
//! `/schemas/notebook/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback.schema.json`.
//!
//! The module exposes:
//!
//! - the [`NotebookMergeFlow`] record that describes a notebook merge operation
//!   (merge kind, base/ours/theirs refs, resolution strategy, unresolved count,
//!   and rollback checkpoint) so the chrome never presents an opaque merge
//!   identity;
//! - the [`NotebookMergeLineage`] record that carries per-cell base/ours/theirs
//!   lineage with a [`NotebookDiffMergeResolutionClass`] so provenance stays
//!   visible at cell granularity;
//! - the [`NotebookConflictReviewSheet`] record that surfaces a per-cell
//!   conflict review sheet (conflict class, suggested resolution, available
//!   actions, rollback path, and redaction profile) so reviewers can choose
//!   cell-aware or raw-merge actions without guessing lineage;
//! - the [`NotebookMergePacket`] checked-in artifact that downstream docs,
//!   help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

use crate::ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback::NotebookDiffMergeResolutionClass;

/// Schema version stamped on every merge/lineage/conflict record carried by
/// this module. Bumped only on breaking payload changes; additive-optional
/// fields do not bump this value.
pub const NOTEBOOK_MERGE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookMergeFlow`] payloads.
pub const NOTEBOOK_MERGE_FLOW_RECORD_KIND: &str = "notebook_merge_flow";

/// Stable record-kind tag for serialized [`NotebookMergeLineage`] payloads.
pub const NOTEBOOK_MERGE_LINEAGE_RECORD_KIND: &str = "notebook_merge_lineage";

/// Stable record-kind tag for serialized [`NotebookConflictReviewSheet`] payloads.
pub const NOTEBOOK_CONFLICT_REVIEW_SHEET_RECORD_KIND: &str = "notebook_conflict_review_sheet";

/// Stable record-kind tag for the checked-in [`NotebookMergePacket`].
pub const NOTEBOOK_MERGE_PACKET_RECORD_KIND: &str = "notebook_merge_packet";

/// Repo-relative path to the checked-in merge packet JSON.
pub const NOTEBOOK_MERGE_PACKET_PATH: &str =
    "artifacts/notebook/m5/implement_notebook_merge_flows_base_or_ours_or_theirs_lineage_and_conflict_review_sheets.json";

/// Embedded checked-in merge packet JSON.
pub const NOTEBOOK_MERGE_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/implement_notebook_merge_flows_base_or_ours_or_theirs_lineage_and_conflict_review_sheets.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Merge-kind class. Names the kind of merge or history mutation that
    /// produced the flow so the UI can show the right lineage and rollback
    /// affordances.
    NotebookMergeKind {
        ThreeWayMerge => "three_way_merge",
        FastForward => "fast_forward",
        Squash => "squash",
        Rebase => "rebase",
        CherryPick => "cherry_pick",
        Revert => "revert",
    }
);

closed_vocab!(
    /// Merge-resolution strategy class. Names whether the merge resolves at
    /// cell granularity, metadata-field granularity, or falls back to raw
    /// side-by-side merge.
    NotebookMergeResolutionStrategy {
        CellAware => "cell_aware",
        MetadataAware => "metadata_aware",
        RawFallback => "raw_fallback",
    }
);

closed_vocab!(
    /// Conflict-class class. Names the kind of conflict a review sheet
    /// surfaces for a cell or metadata field.
    NotebookConflictClass {
        SourceConflict => "source_conflict",
        MetadataConflict => "metadata_conflict",
        OutputConflict => "output_conflict",
        CellDeletedBoth => "cell_deleted_both",
        CellAddedBoth => "cell_added_both",
        TypeConflict => "type_conflict",
    }
);

closed_vocab!(
    /// Conflict-review-sheet action class. Names the actions available on a
    /// per-cell conflict review sheet.
    NotebookConflictReviewSheetAction {
        AcceptOurs => "accept_ours",
        AcceptTheirs => "accept_theirs",
        AcceptBase => "accept_base",
        MarkResolved => "mark_resolved",
        EditResult => "edit_result",
        RawMerge => "raw_merge",
        Abort => "abort",
    }
);

/// Generic finding shape used by every merge/lineage/conflict validator.
/// Mirrors the finding shapes other Aureline crates expose so a single
/// review/audit/support pipeline can consume them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeReviewFinding {
    /// Stable check id (e.g. `notebook_merge_flow.base_ref_required`).
    pub check_id: String,
    /// Subject row id (record id, flow id, lineage id, sheet id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl MergeReviewFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for a [`NotebookMergeFlow`].
pub type NotebookMergeFlowFinding = MergeReviewFinding;

/// Typed validation finding for a [`NotebookMergeLineage`].
pub type NotebookMergeLineageFinding = MergeReviewFinding;

/// Typed validation finding for a [`NotebookConflictReviewSheet`].
pub type NotebookConflictReviewSheetFinding = MergeReviewFinding;

/// Typed validation finding for a [`NotebookMergePacket`].
pub type NotebookMergePacketFinding = MergeReviewFinding;

/// Notebook merge-flow record. Describes a notebook merge operation, its
/// base/ours/theirs/result refs, resolution strategy, unresolved count, and
/// rollback checkpoint so the chrome never presents an opaque merge identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookMergeFlow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_merge_schema_version: u32,
    /// Stable opaque merge-flow id.
    pub merge_flow_id: String,
    /// Opaque notebook-document id this flow merges.
    pub document_id_ref: String,
    /// Merge-kind class.
    pub merge_kind: NotebookMergeKind,
    /// Opaque ref to the base revision/branch.
    pub base_ref: String,
    /// Opaque ref to the ours revision/branch.
    pub ours_ref: String,
    /// Opaque ref to the theirs revision/branch.
    pub theirs_ref: String,
    /// Opaque ref to the result revision, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_ref: Option<String>,
    /// Merge-resolution strategy class.
    pub resolution_strategy: NotebookMergeResolutionStrategy,
    /// Number of unresolved conflicts remaining in the flow.
    pub unresolved_count: u32,
    /// Opaque ref to the rollback checkpoint for this flow.
    pub rollback_checkpoint_ref: String,
    /// Opaque refs to [`NotebookMergeLineage`] records bound to this flow.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lineage_refs: Vec<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookMergeFlow {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookMergeFlowFinding> {
        let mut findings = Vec::new();
        let subject = self.merge_flow_id.as_str();

        if self.record_kind != NOTEBOOK_MERGE_FLOW_RECORD_KIND {
            findings.push(NotebookMergeFlowFinding::new(
                "notebook_merge_flow.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_MERGE_FLOW_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_merge_schema_version != NOTEBOOK_MERGE_SCHEMA_VERSION {
            findings.push(NotebookMergeFlowFinding::new(
                "notebook_merge_flow.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_MERGE_SCHEMA_VERSION}, found {}",
                    self.notebook_merge_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookMergeFlowFinding::new(
                "notebook_merge_flow.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.base_ref.trim().is_empty() {
            findings.push(NotebookMergeFlowFinding::new(
                "notebook_merge_flow.base_ref_required",
                subject,
                "base_ref must be non-empty",
            ));
        }
        if self.ours_ref.trim().is_empty() {
            findings.push(NotebookMergeFlowFinding::new(
                "notebook_merge_flow.ours_ref_required",
                subject,
                "ours_ref must be non-empty",
            ));
        }
        if self.theirs_ref.trim().is_empty() {
            findings.push(NotebookMergeFlowFinding::new(
                "notebook_merge_flow.theirs_ref_required",
                subject,
                "theirs_ref must be non-empty",
            ));
        }
        if self.rollback_checkpoint_ref.trim().is_empty() {
            findings.push(NotebookMergeFlowFinding::new(
                "notebook_merge_flow.rollback_checkpoint_ref_required",
                subject,
                "rollback_checkpoint_ref must be non-empty",
            ));
        }

        if self.unresolved_count > 0
            && self.resolution_strategy != NotebookMergeResolutionStrategy::RawFallback
        {
            // A non-raw merge strategy with outstanding conflicts should not
            // claim a result ref; the result is incomplete.
            if self.result_ref.is_some() {
                findings.push(NotebookMergeFlowFinding::new(
                    "notebook_merge_flow.result_ref_with_unresolved_conflicts",
                    subject,
                    "result_ref must be None when unresolved_count > 0 unless resolution_strategy is raw_fallback",
                ));
            }
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookMergeFlowFinding::new(
                "notebook_merge_flow.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Per-cell notebook merge-lineage record. Carries base/ours/theirs/result
/// cell refs and a merge-resolution class so provenance stays visible at cell
/// granularity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookMergeLineage {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_merge_schema_version: u32,
    /// Stable opaque lineage id.
    pub lineage_id: String,
    /// Opaque ref to the owning [`NotebookMergeFlow`].
    pub merge_flow_ref: String,
    /// Opaque ref to the stable cell id this lineage describes.
    pub cell_id_ref: String,
    /// Opaque ref to the base cell revision, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_cell_ref: Option<String>,
    /// Opaque ref to the ours cell revision, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ours_cell_ref: Option<String>,
    /// Opaque ref to the theirs cell revision, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theirs_cell_ref: Option<String>,
    /// Opaque ref to the result cell revision, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_cell_ref: Option<String>,
    /// Merge-resolution class for this cell.
    pub resolution_class: NotebookDiffMergeResolutionClass,
    /// Opaque refs to metadata-field lineage records bound to this cell.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata_field_lineage_refs: Vec<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookMergeLineage {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookMergeLineageFinding> {
        let mut findings = Vec::new();
        let subject = self.lineage_id.as_str();

        if self.record_kind != NOTEBOOK_MERGE_LINEAGE_RECORD_KIND {
            findings.push(NotebookMergeLineageFinding::new(
                "notebook_merge_lineage.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_MERGE_LINEAGE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_merge_schema_version != NOTEBOOK_MERGE_SCHEMA_VERSION {
            findings.push(NotebookMergeLineageFinding::new(
                "notebook_merge_lineage.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_MERGE_SCHEMA_VERSION}, found {}",
                    self.notebook_merge_schema_version
                ),
            ));
        }

        if self.merge_flow_ref.trim().is_empty() {
            findings.push(NotebookMergeLineageFinding::new(
                "notebook_merge_lineage.merge_flow_ref_required",
                subject,
                "merge_flow_ref must be non-empty",
            ));
        }
        if self.cell_id_ref.trim().is_empty() {
            findings.push(NotebookMergeLineageFinding::new(
                "notebook_merge_lineage.cell_id_ref_required",
                subject,
                "cell_id_ref must be non-empty",
            ));
        }
        if self.summary.trim().is_empty() {
            findings.push(NotebookMergeLineageFinding::new(
                "notebook_merge_lineage.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Per-cell conflict-review sheet. Surfaces the conflict class, suggested
/// resolution, available actions, rollback path, and redaction profile so
/// reviewers can choose cell-aware or raw-merge actions without guessing
/// lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookConflictReviewSheet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_merge_schema_version: u32,
    /// Stable opaque sheet id.
    pub sheet_id: String,
    /// Opaque ref to the owning [`NotebookMergeFlow`].
    pub merge_flow_ref: String,
    /// Opaque ref to the stable cell id where the conflict occurs.
    pub conflict_cell_ref: String,
    /// Conflict-class class.
    pub conflict_class: NotebookConflictClass,
    /// Opaque ref to a base preview record, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_preview_ref: Option<String>,
    /// Opaque ref to an ours preview record, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ours_preview_ref: Option<String>,
    /// Opaque ref to a theirs preview record, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theirs_preview_ref: Option<String>,
    /// Suggested merge-resolution class.
    pub suggested_resolution: NotebookDiffMergeResolutionClass,
    /// Available actions on this conflict-review sheet.
    pub available_actions: Vec<NotebookConflictReviewSheetAction>,
    /// Opaque ref to the rollback path for this conflict.
    pub rollback_path_ref: String,
    /// Opaque ref to the redaction profile applied when sharing this sheet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redaction_profile_ref: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookConflictReviewSheet {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookConflictReviewSheetFinding> {
        let mut findings = Vec::new();
        let subject = self.sheet_id.as_str();

        if self.record_kind != NOTEBOOK_CONFLICT_REVIEW_SHEET_RECORD_KIND {
            findings.push(NotebookConflictReviewSheetFinding::new(
                "notebook_conflict_review_sheet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_CONFLICT_REVIEW_SHEET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_merge_schema_version != NOTEBOOK_MERGE_SCHEMA_VERSION {
            findings.push(NotebookConflictReviewSheetFinding::new(
                "notebook_conflict_review_sheet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_MERGE_SCHEMA_VERSION}, found {}",
                    self.notebook_merge_schema_version
                ),
            ));
        }

        if self.merge_flow_ref.trim().is_empty() {
            findings.push(NotebookConflictReviewSheetFinding::new(
                "notebook_conflict_review_sheet.merge_flow_ref_required",
                subject,
                "merge_flow_ref must be non-empty",
            ));
        }
        if self.conflict_cell_ref.trim().is_empty() {
            findings.push(NotebookConflictReviewSheetFinding::new(
                "notebook_conflict_review_sheet.conflict_cell_ref_required",
                subject,
                "conflict_cell_ref must be non-empty",
            ));
        }
        if self.available_actions.is_empty() {
            findings.push(NotebookConflictReviewSheetFinding::new(
                "notebook_conflict_review_sheet.available_actions_required",
                subject,
                "available_actions must contain at least one action",
            ));
        }
        if self.rollback_path_ref.trim().is_empty() {
            findings.push(NotebookConflictReviewSheetFinding::new(
                "notebook_conflict_review_sheet.rollback_path_ref_required",
                subject,
                "rollback_path_ref must be non-empty",
            ));
        }
        if self.summary.trim().is_empty() {
            findings.push(NotebookConflictReviewSheetFinding::new(
                "notebook_conflict_review_sheet.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Checked-in merge/lineage/conflict packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookMergePacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: merge kinds.
    pub merge_kinds: Vec<NotebookMergeKind>,
    /// Closed vocabulary: resolution strategies.
    pub resolution_strategies: Vec<NotebookMergeResolutionStrategy>,
    /// Closed vocabulary: conflict classes.
    pub conflict_classes: Vec<NotebookConflictClass>,
    /// Closed vocabulary: sheet actions.
    pub sheet_actions: Vec<NotebookConflictReviewSheetAction>,
    /// Closed vocabulary: merge resolution classes.
    pub merge_resolution_classes: Vec<NotebookDiffMergeResolutionClass>,
    /// Worked example merge flows.
    pub example_merge_flows: Vec<NotebookMergeFlow>,
    /// Worked example lineage records.
    pub example_lineages: Vec<NotebookMergeLineage>,
    /// Worked example conflict-review sheets.
    pub example_conflict_sheets: Vec<NotebookConflictReviewSheet>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookMergePacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookMergePacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_MERGE_SCHEMA_VERSION {
            findings.push(NotebookMergePacketFinding::new(
                "notebook_merge_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_MERGE_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_MERGE_PACKET_RECORD_KIND {
            findings.push(NotebookMergePacketFinding::new(
                "notebook_merge_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_MERGE_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.merge_kinds.len() != NotebookMergeKind::ALL.len() {
            findings.push(NotebookMergePacketFinding::new(
                "notebook_merge_packet.merge_kinds_coverage",
                subject,
                "merge_kinds must list every variant",
            ));
        }
        if self.resolution_strategies.len() != NotebookMergeResolutionStrategy::ALL.len() {
            findings.push(NotebookMergePacketFinding::new(
                "notebook_merge_packet.resolution_strategies_coverage",
                subject,
                "resolution_strategies must list every variant",
            ));
        }
        if self.conflict_classes.len() != NotebookConflictClass::ALL.len() {
            findings.push(NotebookMergePacketFinding::new(
                "notebook_merge_packet.conflict_classes_coverage",
                subject,
                "conflict_classes must list every variant",
            ));
        }
        if self.sheet_actions.len() != NotebookConflictReviewSheetAction::ALL.len() {
            findings.push(NotebookMergePacketFinding::new(
                "notebook_merge_packet.sheet_actions_coverage",
                subject,
                "sheet_actions must list every variant",
            ));
        }
        if self.merge_resolution_classes.len() != NotebookDiffMergeResolutionClass::ALL.len() {
            findings.push(NotebookMergePacketFinding::new(
                "notebook_merge_packet.merge_resolution_classes_coverage",
                subject,
                "merge_resolution_classes must list every variant",
            ));
        }

        for flow in &self.example_merge_flows {
            findings.extend(
                flow.validate().into_iter().map(|f| {
                    NotebookMergePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
                }),
            );
        }
        for lineage in &self.example_lineages {
            findings.extend(
                lineage.validate().into_iter().map(|f| {
                    NotebookMergePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
                }),
            );
        }
        for sheet in &self.example_conflict_sheets {
            findings.extend(
                sheet.validate().into_iter().map(|f| {
                    NotebookMergePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
                }),
            );
        }

        findings
    }
}

/// Parses the checked-in merge packet JSON.
pub fn current_notebook_merge_packet() -> Result<NotebookMergePacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_MERGE_PACKET_JSON)
}

impl NotebookMergeKind {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ThreeWayMerge,
        Self::FastForward,
        Self::Squash,
        Self::Rebase,
        Self::CherryPick,
        Self::Revert,
    ];
}

impl NotebookMergeResolutionStrategy {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [Self::CellAware, Self::MetadataAware, Self::RawFallback];
}

impl NotebookConflictClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SourceConflict,
        Self::MetadataConflict,
        Self::OutputConflict,
        Self::CellDeletedBoth,
        Self::CellAddedBoth,
        Self::TypeConflict,
    ];
}

impl NotebookConflictReviewSheetAction {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AcceptOurs,
        Self::AcceptTheirs,
        Self::AcceptBase,
        Self::MarkResolved,
        Self::EditResult,
        Self::RawMerge,
        Self::Abort,
    ];
}

#[cfg(test)]
mod tests;
