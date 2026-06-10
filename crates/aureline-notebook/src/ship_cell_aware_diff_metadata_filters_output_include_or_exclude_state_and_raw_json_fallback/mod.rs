//! Cell-aware diff, metadata filters, output include or exclude state, and raw JSON fallback.
//!
//! This module materializes the typed records that keep notebook review,
//! diff, merge, and collaboration honest about cell identity, metadata
//! boundaries, output visibility, and fallback posture. The records and
//! closed vocabularies here mirror the boundary schema at
//! `/schemas/notebook/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback.schema.json`
//! and reuse the diff, merge, and review axes already frozen in
//! `/schemas/notebook/notebook_metadata_aureline.schema.json`.
//!
//! The module exposes:
//!
//! - the [`NotebookDiffReviewSession`] record that binds a notebook review
//!   session to its diff mode, metadata-filter state, output-include state,
//!   stable cell/output anchors, and raw-fallback reason;
//! - the [`NotebookDiffCellChange`] record that carries per-cell diff facts
//!   (change class, source edit summary, output change summary, metadata
//!   filter state) so cell-aware review never degrades to line-only anchors
//!   silently;
//! - the [`NotebookDiffOutputSummary`] record that carries output
//!   add/remove/update/unchanged facts, include/exclude state, truncation
//!   notes, and trust/sandbox refs;
//! - the [`NotebookDiffMetadataFilter`] record that names which metadata
//!   namespaces are visible, hidden, or filtered in the current review mode;
//! - the [`NotebookRawJsonFallback`] record that explains why semantic
//!   review is unavailable and preserves the canonical-source note;
//! - the [`NotebookDiffPacket`] checked-in artifact that downstream docs,
//!   help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every diff/review record carried by this
/// module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const NOTEBOOK_DIFF_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookDiffReviewSession`] payloads.
pub const NOTEBOOK_DIFF_REVIEW_SESSION_RECORD_KIND: &str = "notebook_diff_review_session";

/// Stable record-kind tag for serialized [`NotebookDiffCellChange`] payloads.
pub const NOTEBOOK_DIFF_CELL_CHANGE_RECORD_KIND: &str = "notebook_diff_cell_change";

/// Stable record-kind tag for serialized [`NotebookDiffOutputSummary`] payloads.
pub const NOTEBOOK_DIFF_OUTPUT_SUMMARY_RECORD_KIND: &str = "notebook_diff_output_summary";

/// Stable record-kind tag for serialized [`NotebookDiffMetadataFilter`] payloads.
pub const NOTEBOOK_DIFF_METADATA_FILTER_RECORD_KIND: &str = "notebook_diff_metadata_filter";

/// Stable record-kind tag for serialized [`NotebookRawJsonFallback`] payloads.
pub const NOTEBOOK_RAW_JSON_FALLBACK_RECORD_KIND: &str = "notebook_raw_json_fallback";

/// Stable record-kind tag for the checked-in [`NotebookDiffPacket`].
pub const NOTEBOOK_DIFF_PACKET_RECORD_KIND: &str = "notebook_diff_packet";

/// Repo-relative path to the checked-in diff packet JSON.
pub const NOTEBOOK_DIFF_PACKET_PATH: &str =
    "artifacts/notebook/m5/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback.json";

/// Embedded checked-in diff packet JSON.
pub const NOTEBOOK_DIFF_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback.json"
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
    /// Diff mode class. Names whether the review surface is showing cell-aware
    /// structure, metadata focus, output focus, or raw JSON fallback.
    NotebookDiffMode {
        CellAware => "cell_aware",
        MetadataFocused => "metadata_focused",
        OutputAware => "output_aware",
        RawJsonFallback => "raw_json_fallback",
    }
);

closed_vocab!(
    /// Per-cell change class. Names what changed for a cell in the diff.
    NotebookDiffCellChangeClass {
        CellAdded => "cell_added",
        CellRemoved => "cell_removed",
        CellReordered => "cell_reordered",
        CellTypeChanged => "cell_type_changed",
        SourceChanged => "source_changed",
        MetadataChanged => "metadata_changed",
        OutputChanged => "output_changed",
        ExecutionChanged => "execution_changed",
        Unchanged => "unchanged",
    }
);

closed_vocab!(
    /// Per-output change class. Names what changed for an output in the diff.
    NotebookDiffOutputChangeClass {
        OutputAdded => "output_added",
        OutputRemoved => "output_removed",
        OutputUpdated => "output_updated",
        OutputUnchanged => "output_unchanged",
    }
);

closed_vocab!(
    /// Metadata-filter state class. Names which metadata namespaces are
    /// visible in the current review mode. Filters are review conveniences
    /// only and never justify stripping unknown fields on save.
    NotebookMetadataFilterState {
        AllVisible => "all_visible",
        OfficialOnly => "official_only",
        AurelineOnly => "aureline_only",
        UnknownHidden => "unknown_hidden",
    }
);

closed_vocab!(
    /// Output-include state class. Names whether outputs are included,
    /// excluded, or collapsed in the current review mode so reviewers know
    /// whether outputs are absent, hidden, or intentionally excluded from
    /// comparison.
    NotebookOutputIncludeState {
        Included => "included",
        Excluded => "excluded",
        Collapsed => "collapsed",
    }
);

closed_vocab!(
    /// Raw-JSON fallback reason class. Names why semantic review is
    /// unavailable so the UI never silently degrades.
    RawJsonFallbackReason {
        ParseError => "parse_error",
        UnsupportedVersion => "unsupported_version",
        ExtensionMismatch => "extension_mismatch",
        ExplicitUserChoice => "explicit_user_choice",
        CorruptStructure => "corrupt_structure",
    }
);

closed_vocab!(
    /// Merge resolution class. Names how a merge resolved at cell or
    /// metadata-field granularity.
    NotebookDiffMergeResolutionClass {
        Base => "base",
        Ours => "ours",
        Theirs => "theirs",
        Result => "result",
        Unresolved => "unresolved",
    }
);

/// Generic finding shape used by every diff/review validator. Mirrors the
/// finding shapes other Aureline crates expose so a single review/audit/
/// support pipeline can consume them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffReviewFinding {
    /// Stable check id (e.g. `notebook_diff_review_session.mode_required`).
    pub check_id: String,
    /// Subject row id (record id, session id, cell id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl DiffReviewFinding {
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

/// Typed validation finding for a [`NotebookDiffReviewSession`].
pub type NotebookDiffReviewSessionFinding = DiffReviewFinding;

/// Typed validation finding for a [`NotebookDiffCellChange`].
pub type NotebookDiffCellChangeFinding = DiffReviewFinding;

/// Typed validation finding for a [`NotebookDiffOutputSummary`].
pub type NotebookDiffOutputSummaryFinding = DiffReviewFinding;

/// Typed validation finding for a [`NotebookDiffMetadataFilter`].
pub type NotebookDiffMetadataFilterFinding = DiffReviewFinding;

/// Typed validation finding for a [`NotebookRawJsonFallback`].
pub type NotebookRawJsonFallbackFinding = DiffReviewFinding;

/// Typed validation finding for a [`NotebookDiffPacket`].
pub type NotebookDiffPacketFinding = DiffReviewFinding;

/// Per-output diff summary. Carries add/remove/update/unchanged facts,
/// include/exclude state, and trust/sandbox refs so reviewers know whether
/// an output is absent, hidden, or intentionally excluded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDiffOutputSummary {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_diff_schema_version: u32,
    /// Stable opaque output summary id.
    pub output_summary_id: String,
    /// Opaque ref to the owning cell.
    pub owner_cell_ref: String,
    /// Output change class.
    pub output_change_class: NotebookDiffOutputChangeClass,
    /// Output-include state in the current review mode.
    pub output_include_state: NotebookOutputIncludeState,
    /// Whether the output is truncated or virtualized in the review view.
    pub truncated_in_review: bool,
    /// Opaque ref to the output trust record for this output.
    pub output_trust_state_ref: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookDiffOutputSummary {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookDiffOutputSummaryFinding> {
        let mut findings = Vec::new();
        let subject = self.output_summary_id.as_str();

        if self.record_kind != NOTEBOOK_DIFF_OUTPUT_SUMMARY_RECORD_KIND {
            findings.push(NotebookDiffOutputSummaryFinding::new(
                "notebook_diff_output_summary.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_DIFF_OUTPUT_SUMMARY_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_diff_schema_version != NOTEBOOK_DIFF_SCHEMA_VERSION {
            findings.push(NotebookDiffOutputSummaryFinding::new(
                "notebook_diff_output_summary.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DIFF_SCHEMA_VERSION}, found {}",
                    self.notebook_diff_schema_version
                ),
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookDiffOutputSummaryFinding::new(
                "notebook_diff_output_summary.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Metadata-filter state for a review session. Names which namespaces are
/// visible, hidden, or filtered. Filters are review conveniences only and
/// never justify stripping unknown fields on save.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDiffMetadataFilter {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_diff_schema_version: u32,
    /// Stable opaque metadata-filter id.
    pub metadata_filter_id: String,
    /// Metadata-filter state class.
    pub metadata_filter_state: NotebookMetadataFilterState,
    /// Opaque refs to visible metadata namespace ids.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visible_namespace_refs: Vec<String>,
    /// Opaque refs to hidden metadata namespace ids.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hidden_namespace_refs: Vec<String>,
    /// Whether unknown vendor namespaces are preserved on save despite being
    /// hidden in review.
    pub unknown_namespaces_preserved_on_save: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookDiffMetadataFilter {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookDiffMetadataFilterFinding> {
        let mut findings = Vec::new();
        let subject = self.metadata_filter_id.as_str();

        if self.record_kind != NOTEBOOK_DIFF_METADATA_FILTER_RECORD_KIND {
            findings.push(NotebookDiffMetadataFilterFinding::new(
                "notebook_diff_metadata_filter.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_DIFF_METADATA_FILTER_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_diff_schema_version != NOTEBOOK_DIFF_SCHEMA_VERSION {
            findings.push(NotebookDiffMetadataFilterFinding::new(
                "notebook_diff_metadata_filter.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DIFF_SCHEMA_VERSION}, found {}",
                    self.notebook_diff_schema_version
                ),
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookDiffMetadataFilterFinding::new(
                "notebook_diff_metadata_filter.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Per-cell diff change record. Carries change class, source-edit summary,
/// output change summary, metadata filter state, and stable cell anchors so
/// cell-aware review never degrades to line-only anchors silently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDiffCellChange {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_diff_schema_version: u32,
    /// Stable opaque cell-change id.
    pub cell_change_id: String,
    /// Opaque ref to the stable cell id this change describes.
    pub cell_id_ref: String,
    /// Cell change class.
    pub cell_change_class: NotebookDiffCellChangeClass,
    /// Opaque ref to the source-edit summary record, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_edit_summary_ref: Option<String>,
    /// Opaque refs to output summary records bound to this cell.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_summary_refs: Vec<String>,
    /// Opaque ref to the metadata-filter state for this cell.
    pub metadata_filter_ref: String,
    /// Merge resolution class when this cell is part of a merge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_resolution_class: Option<NotebookDiffMergeResolutionClass>,
    /// Whether this cell is collapsed in the diff view.
    pub collapsed_in_diff: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookDiffCellChange {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookDiffCellChangeFinding> {
        let mut findings = Vec::new();
        let subject = self.cell_change_id.as_str();

        if self.record_kind != NOTEBOOK_DIFF_CELL_CHANGE_RECORD_KIND {
            findings.push(NotebookDiffCellChangeFinding::new(
                "notebook_diff_cell_change.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_DIFF_CELL_CHANGE_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_diff_schema_version != NOTEBOOK_DIFF_SCHEMA_VERSION {
            findings.push(NotebookDiffCellChangeFinding::new(
                "notebook_diff_cell_change.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DIFF_SCHEMA_VERSION}, found {}",
                    self.notebook_diff_schema_version
                ),
            ));
        }

        if self.cell_id_ref.trim().is_empty() {
            findings.push(NotebookDiffCellChangeFinding::new(
                "notebook_diff_cell_change.cell_id_ref_required",
                subject,
                "cell_id_ref must be non-empty",
            ));
        }

        findings
    }
}

/// Raw JSON fallback record. Explains why semantic review is unavailable and
/// preserves the canonical-source note so the UI never silently degrades.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookRawJsonFallback {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_diff_schema_version: u32,
    /// Stable opaque fallback id.
    pub fallback_id: String,
    /// Raw-JSON fallback reason class.
    pub fallback_reason: RawJsonFallbackReason,
    /// Human-readable explanation shown in the UI.
    pub fallback_explanation: String,
    /// Whether the raw JSON view is the explicit user choice.
    pub explicit_user_choice: bool,
    /// Opaque ref to the canonical notebook document.
    pub canonical_document_ref: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookRawJsonFallback {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookRawJsonFallbackFinding> {
        let mut findings = Vec::new();
        let subject = self.fallback_id.as_str();

        if self.record_kind != NOTEBOOK_RAW_JSON_FALLBACK_RECORD_KIND {
            findings.push(NotebookRawJsonFallbackFinding::new(
                "notebook_raw_json_fallback.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_RAW_JSON_FALLBACK_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_diff_schema_version != NOTEBOOK_DIFF_SCHEMA_VERSION {
            findings.push(NotebookRawJsonFallbackFinding::new(
                "notebook_raw_json_fallback.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DIFF_SCHEMA_VERSION}, found {}",
                    self.notebook_diff_schema_version
                ),
            ));
        }

        if self.fallback_explanation.trim().is_empty() {
            findings.push(NotebookRawJsonFallbackFinding::new(
                "notebook_raw_json_fallback.fallback_explanation_required",
                subject,
                "fallback_explanation must be non-empty",
            ));
        }

        findings
    }
}

/// Notebook diff/review session record. Binds a notebook review session to
/// its diff mode, metadata-filter state, output-include state, stable
/// cell/output anchors, and raw-fallback reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDiffReviewSession {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_diff_schema_version: u32,
    /// Stable opaque session id.
    pub session_id: String,
    /// Opaque notebook-document id this session reviews.
    pub document_id_ref: String,
    /// Diff mode class.
    pub diff_mode: NotebookDiffMode,
    /// Opaque ref to the metadata-filter state for this session.
    pub metadata_filter_ref: String,
    /// Output-include state for this session.
    pub output_include_state: NotebookOutputIncludeState,
    /// Opaque refs to cell-change records in diff order.
    pub cell_change_refs: Vec<String>,
    /// Opaque ref to the raw-JSON fallback record, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_json_fallback_ref: Option<String>,
    /// Whether comments and review anchors bind to stable cell IDs.
    pub stable_cell_anchors: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookDiffReviewSession {
    /// Returns typed truth-rule findings; an empty vector means the session is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookDiffReviewSessionFinding> {
        let mut findings = Vec::new();
        let subject = self.session_id.as_str();

        if self.record_kind != NOTEBOOK_DIFF_REVIEW_SESSION_RECORD_KIND {
            findings.push(NotebookDiffReviewSessionFinding::new(
                "notebook_diff_review_session.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_DIFF_REVIEW_SESSION_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_diff_schema_version != NOTEBOOK_DIFF_SCHEMA_VERSION {
            findings.push(NotebookDiffReviewSessionFinding::new(
                "notebook_diff_review_session.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DIFF_SCHEMA_VERSION}, found {}",
                    self.notebook_diff_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookDiffReviewSessionFinding::new(
                "notebook_diff_review_session.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }

        if self.diff_mode == NotebookDiffMode::RawJsonFallback && self.raw_json_fallback_ref.is_none() {
            findings.push(NotebookDiffReviewSessionFinding::new(
                "notebook_diff_review_session.raw_json_fallback_ref_required",
                subject,
                "raw_json_fallback_ref is required when diff_mode is raw_json_fallback",
            ));
        }

        if self.diff_mode != NotebookDiffMode::RawJsonFallback && self.raw_json_fallback_ref.is_some() {
            findings.push(NotebookDiffReviewSessionFinding::new(
                "notebook_diff_review_session.raw_json_fallback_ref_unexpected",
                subject,
                "raw_json_fallback_ref must be None when diff_mode is not raw_json_fallback",
            ));
        }

        findings
    }
}

/// Checked-in diff/review packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDiffPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: diff modes.
    pub diff_modes: Vec<NotebookDiffMode>,
    /// Closed vocabulary: cell change classes.
    pub cell_change_classes: Vec<NotebookDiffCellChangeClass>,
    /// Closed vocabulary: output change classes.
    pub output_change_classes: Vec<NotebookDiffOutputChangeClass>,
    /// Closed vocabulary: metadata filter states.
    pub metadata_filter_states: Vec<NotebookMetadataFilterState>,
    /// Closed vocabulary: output include states.
    pub output_include_states: Vec<NotebookOutputIncludeState>,
    /// Closed vocabulary: raw JSON fallback reasons.
    pub raw_json_fallback_reasons: Vec<RawJsonFallbackReason>,
    /// Closed vocabulary: merge resolution classes.
    pub merge_resolution_classes: Vec<NotebookDiffMergeResolutionClass>,
    /// Worked example review sessions.
    pub example_review_sessions: Vec<NotebookDiffReviewSession>,
    /// Worked example cell changes.
    pub example_cell_changes: Vec<NotebookDiffCellChange>,
    /// Worked example output summaries.
    pub example_output_summaries: Vec<NotebookDiffOutputSummary>,
    /// Worked example metadata filters.
    pub example_metadata_filters: Vec<NotebookDiffMetadataFilter>,
    /// Worked example raw JSON fallbacks.
    pub example_raw_json_fallbacks: Vec<NotebookRawJsonFallback>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookDiffPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookDiffPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_DIFF_SCHEMA_VERSION {
            findings.push(NotebookDiffPacketFinding::new(
                "notebook_diff_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DIFF_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_DIFF_PACKET_RECORD_KIND {
            findings.push(NotebookDiffPacketFinding::new(
                "notebook_diff_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_DIFF_PACKET_RECORD_KIND, self.record_kind
                ),
            ));
        }

        if self.diff_modes.len() != NotebookDiffMode::ALL.len() {
            findings.push(NotebookDiffPacketFinding::new(
                "notebook_diff_packet.diff_modes_coverage",
                subject,
                "diff_modes must list every variant",
            ));
        }
        if self.cell_change_classes.len() != NotebookDiffCellChangeClass::ALL.len() {
            findings.push(NotebookDiffPacketFinding::new(
                "notebook_diff_packet.cell_change_classes_coverage",
                subject,
                "cell_change_classes must list every variant",
            ));
        }
        if self.output_change_classes.len() != NotebookDiffOutputChangeClass::ALL.len() {
            findings.push(NotebookDiffPacketFinding::new(
                "notebook_diff_packet.output_change_classes_coverage",
                subject,
                "output_change_classes must list every variant",
            ));
        }
        if self.metadata_filter_states.len() != NotebookMetadataFilterState::ALL.len() {
            findings.push(NotebookDiffPacketFinding::new(
                "notebook_diff_packet.metadata_filter_states_coverage",
                subject,
                "metadata_filter_states must list every variant",
            ));
        }
        if self.output_include_states.len() != NotebookOutputIncludeState::ALL.len() {
            findings.push(NotebookDiffPacketFinding::new(
                "notebook_diff_packet.output_include_states_coverage",
                subject,
                "output_include_states must list every variant",
            ));
        }
        if self.raw_json_fallback_reasons.len() != RawJsonFallbackReason::ALL.len() {
            findings.push(NotebookDiffPacketFinding::new(
                "notebook_diff_packet.raw_json_fallback_reasons_coverage",
                subject,
                "raw_json_fallback_reasons must list every variant",
            ));
        }
        if self.merge_resolution_classes.len() != NotebookDiffMergeResolutionClass::ALL.len() {
            findings.push(NotebookDiffPacketFinding::new(
                "notebook_diff_packet.merge_resolution_classes_coverage",
                subject,
                "merge_resolution_classes must list every variant",
            ));
        }

        for session in &self.example_review_sessions {
            findings.extend(session.validate().into_iter().map(|f| {
                NotebookDiffPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for change in &self.example_cell_changes {
            findings.extend(change.validate().into_iter().map(|f| {
                NotebookDiffPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for summary in &self.example_output_summaries {
            findings.extend(summary.validate().into_iter().map(|f| {
                NotebookDiffPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for filter in &self.example_metadata_filters {
            findings.extend(filter.validate().into_iter().map(|f| {
                NotebookDiffPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for fallback in &self.example_raw_json_fallbacks {
            findings.extend(fallback.validate().into_iter().map(|f| {
                NotebookDiffPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

/// Parses the checked-in diff packet JSON.
pub fn current_notebook_diff_packet() -> Result<NotebookDiffPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_DIFF_PACKET_JSON)
}

impl NotebookDiffMode {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CellAware,
        Self::MetadataFocused,
        Self::OutputAware,
        Self::RawJsonFallback,
    ];
}

impl NotebookDiffCellChangeClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::CellAdded,
        Self::CellRemoved,
        Self::CellReordered,
        Self::CellTypeChanged,
        Self::SourceChanged,
        Self::MetadataChanged,
        Self::OutputChanged,
        Self::ExecutionChanged,
        Self::Unchanged,
    ];
}

impl NotebookDiffOutputChangeClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OutputAdded,
        Self::OutputRemoved,
        Self::OutputUpdated,
        Self::OutputUnchanged,
    ];
}

impl NotebookMetadataFilterState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AllVisible,
        Self::OfficialOnly,
        Self::AurelineOnly,
        Self::UnknownHidden,
    ];
}

impl NotebookOutputIncludeState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [Self::Included, Self::Excluded, Self::Collapsed];
}

impl RawJsonFallbackReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ParseError,
        Self::UnsupportedVersion,
        Self::ExtensionMismatch,
        Self::ExplicitUserChoice,
        Self::CorruptStructure,
    ];
}

impl NotebookDiffMergeResolutionClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Base,
        Self::Ours,
        Self::Theirs,
        Self::Result,
        Self::Unresolved,
    ];
}

#[cfg(test)]
mod tests;
