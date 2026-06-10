//! Notebook variable explorer, live or snapshot or stale labels, and typed export.
//!
//! This module materializes the composed variable-explorer surface that the
//! notebook chrome consumes to render the variable panel, freshness labels,
//! truncation notices, and typed-export actions. It reuses the closed
//! vocabularies and backing records already frozen in the
//! [`crate::runtime_truth`] module and adds the [`NotebookVariableExplorer`]
//! and [`VariableExplorerTypedExport`] records so the explorer never implies
//! durable project facts and never silently broadens capture on export.
//!
//! The module exposes:
//!
//! - the [`NotebookVariableExplorer`] record that carries the composed explorer
//!   state — entries, sort, filter, search, counts, and truncation notice — so
//!   the chrome can render the panel without touching raw kernel state;
//! - the [`VariableExplorerTypedExport`] record that carries export format,
//!   posture, scope, redaction requirements, and output path so the user always
//!   knows what will be exported, in what shape, and under what policy;
//! - the [`NotebookVariableExplorerPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookVariableExplorer`] payloads.
pub const NOTEBOOK_VARIABLE_EXPLORER_RECORD_KIND: &str = "notebook_variable_explorer";

/// Stable record-kind tag for serialized [`VariableExplorerTypedExport`]
/// payloads.
pub const VARIABLE_EXPLORER_TYPED_EXPORT_RECORD_KIND: &str =
    "notebook_variable_explorer_typed_export";

/// Stable record-kind tag for the checked-in [`NotebookVariableExplorerPacket`].
pub const NOTEBOOK_VARIABLE_EXPLORER_PACKET_RECORD_KIND: &str = "notebook_variable_explorer_packet";

/// Repo-relative path to the checked-in variable-explorer packet JSON.
pub const NOTEBOOK_VARIABLE_EXPLORER_PACKET_PATH: &str =
    "artifacts/notebook/m5/implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export.json";

/// Embedded checked-in variable-explorer packet JSON.
pub const NOTEBOOK_VARIABLE_EXPLORER_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export.json"
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
    /// How entries are sorted in the variable explorer. Pinned so the chrome
    /// never invents ad hoc sort orders.
    VariableExplorerSortClass {
        NameAscending => "name_ascending",
        NameDescending => "name_descending",
        TypeAscending => "type_ascending",
        TypeDescending => "type_descending",
        FreshnessAscending => "freshness_ascending",
        FreshnessDescending => "freshness_descending",
    }
);

closed_vocab!(
    /// What filter is applied to the variable-explorer entry list. Pinned so
    /// the user never mistakes a filtered view for the complete kernel state.
    VariableExplorerFilterClass {
        NoFilter => "no_filter",
        LiveOnly => "live_only",
        SnapshotOnly => "snapshot_only",
        StaleOnly => "stale_only",
        ByType => "by_type",
        ByName => "by_name",
    }
);

closed_vocab!(
    /// Export format for typed variable export. Pinned so the user always
    /// knows the structural shape of the exported data.
    VariableExplorerExportFormatClass {
        Csv => "csv",
        Json => "json",
        Tsv => "tsv",
        PythonDict => "python_dict",
        MarkdownTable => "markdown_table",
    }
);

closed_vocab!(
    /// Export posture for a typed variable export. Distinguishes ready,
    /// review-required, policy-blocked, and redaction-required states so the
    /// chrome never silently exports sensitive or stale values.
    VariableExplorerExportPostureClass {
        Ready => "ready",
        RequiresReview => "requires_review",
        BlockedByPolicy => "blocked_by_policy",
        RedactionRequired => "redaction_required",
    }
);

closed_vocab!(
    /// Export scope for a typed variable export. Distinguishes all visible
    /// entries, selected entries only, current-session live entries only, and
    /// snapshot-session entries only so the user knows exactly which values
    /// cross the boundary.
    VariableExplorerExportScopeClass {
        AllVisible => "all_visible",
        SelectedOnly => "selected_only",
        CurrentSessionOnly => "current_session_only",
        SnapshotSessionOnly => "snapshot_session_only",
    }
);

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableExplorerFinding {
    /// Stable check id (e.g. `notebook_variable_explorer.record_kind`).
    pub check_id: String,
    /// Subject row id (record id, explorer id, export id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl VariableExplorerFinding {
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

/// Typed validation finding for a [`NotebookVariableExplorer`].
pub type NotebookVariableExplorerFinding = VariableExplorerFinding;

/// Typed validation finding for a [`VariableExplorerTypedExport`].
pub type VariableExplorerTypedExportFinding = VariableExplorerFinding;

/// Typed validation finding for a [`NotebookVariableExplorerPacket`].
pub type NotebookVariableExplorerPacketFinding = VariableExplorerFinding;

/// Canonical notebook variable-explorer record. The composed UI surface for
/// the variable panel: entries, sort, filter, counts, and truncation notice.
///
/// This record never carries raw variable values; it only carries opaque refs
/// and closed-vocabulary tokens so the chrome renders truthfully without
/// exposing kernel state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookVariableExplorer {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_variable_explorer_schema_version: u32,
    /// Stable opaque explorer-state id.
    pub explorer_state_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque kernel-session id this explorer is attributed to; null when the
    /// explorer shows no live variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Variable-explorer entries rendered in the panel.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<crate::VariableExplorerEntry>,
    /// How entries are currently sorted.
    pub sort_class: VariableExplorerSortClass,
    /// What filter is currently applied.
    pub filter_class: VariableExplorerFilterClass,
    /// Export-safe search query label; null when no search is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_query_label: Option<String>,
    /// Number of entries currently visible after sort/filter/search.
    pub entry_count_visible: u32,
    /// Total number of entries before sort/filter/search.
    pub entry_count_total: u32,
    /// Whether there are more entries available than are currently shown.
    pub has_more_entries: bool,
    /// Whether a truncation notice is visible in the chrome.
    pub truncation_notice_visible: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookVariableExplorer {
    /// Returns typed truth-rule findings; an empty vector means the explorer
    /// state is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookVariableExplorerFinding> {
        let mut findings = Vec::new();
        let subject = self.explorer_state_id.as_str();

        if self.record_kind != NOTEBOOK_VARIABLE_EXPLORER_RECORD_KIND {
            findings.push(NotebookVariableExplorerFinding::new(
                "notebook_variable_explorer.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_VARIABLE_EXPLORER_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_variable_explorer_schema_version
            != NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION
        {
            findings.push(NotebookVariableExplorerFinding::new(
                "notebook_variable_explorer.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION}, found {}",
                    self.notebook_variable_explorer_schema_version
                ),
            ));
        }

        if self.entry_count_visible > self.entry_count_total {
            findings.push(NotebookVariableExplorerFinding::new(
                "notebook_variable_explorer.visible_exceeds_total",
                subject,
                "entry_count_visible must not exceed entry_count_total",
            ));
        }

        // If the filter narrows to a specific freshness class, every entry must match.
        match self.filter_class {
            VariableExplorerFilterClass::LiveOnly => {
                for entry in &self.entries {
                    if !matches!(
                        entry.freshness_class,
                        crate::VariableExplorerFreshnessClass::LiveFromCurrentSession
                    ) {
                        findings.push(NotebookVariableExplorerFinding::new(
                            "notebook_variable_explorer.live_only_inconsistent",
                            subject,
                            "live_only filter must not contain non-live entries",
                        ));
                        break;
                    }
                }
            }
            VariableExplorerFilterClass::SnapshotOnly => {
                for entry in &self.entries {
                    if !matches!(
                        entry.freshness_class,
                        crate::VariableExplorerFreshnessClass::SnapshotFromPriorSession
                            | crate::VariableExplorerFreshnessClass::ImportedSnapshot
                    ) {
                        findings.push(NotebookVariableExplorerFinding::new(
                            "notebook_variable_explorer.snapshot_only_inconsistent",
                            subject,
                            "snapshot_only filter must not contain non-snapshot entries",
                        ));
                        break;
                    }
                }
            }
            VariableExplorerFilterClass::StaleOnly => {
                for entry in &self.entries {
                    if !matches!(
                        entry.freshness_class,
                        crate::VariableExplorerFreshnessClass::StaleAfterRestart
                    ) {
                        findings.push(NotebookVariableExplorerFinding::new(
                            "notebook_variable_explorer.stale_only_inconsistent",
                            subject,
                            "stale_only filter must not contain non-stale entries",
                        ));
                        break;
                    }
                }
            }
            _ => {}
        }

        // Count consistency: visible count must match the filtered entry list length.
        let entries_len = self.entries.len() as u32;
        if self.entry_count_visible != entries_len {
            findings.push(NotebookVariableExplorerFinding::new(
                "notebook_variable_explorer.visible_count_mismatch",
                subject,
                "entry_count_visible must match the length of the entries list",
            ));
        }

        // Truncation notice must be visible when has_more_entries is true.
        if self.has_more_entries && !self.truncation_notice_visible {
            findings.push(NotebookVariableExplorerFinding::new(
                "notebook_variable_explorer.truncation_notice_required",
                subject,
                "has_more_entries=true requires truncation_notice_visible=true",
            ));
        }

        // No-kernel explorer must not claim a kernel session.
        if self.entries.is_empty() && self.kernel_session_id_ref.is_some() {
            // This is allowed — the kernel may be connected but have no variables.
        }

        for entry in &self.entries {
            findings.extend(entry.validate().into_iter().map(|f| {
                NotebookVariableExplorerFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

/// Canonical typed-export record for the variable explorer.
///
/// Carries exactly what will be exported, in what format, under what posture,
/// and with what redaction policy — so the user never accidentally exports a
/// snapshot as live or a redacted field in the clear.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableExplorerTypedExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_variable_explorer_schema_version: u32,
    /// Stable opaque export id.
    pub export_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque kernel-session id this export is attributed to; null when the
    /// export scope does not require a live session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Opaque variable-handle refs selected for export.
    pub variable_handle_refs: Vec<String>,
    /// Export format class.
    pub export_format_class: VariableExplorerExportFormatClass,
    /// Export posture class.
    pub export_posture_class: VariableExplorerExportPostureClass,
    /// Export scope class.
    pub export_scope_class: VariableExplorerExportScopeClass,
    /// Whether redaction is required before export can proceed.
    pub redaction_required: bool,
    /// Opaque output-path token ref; null when the export is not yet
    /// destined for a specific path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_path_token_ref: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl VariableExplorerTypedExport {
    /// Returns typed truth-rule findings; an empty vector means the export
    /// record is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<VariableExplorerTypedExportFinding> {
        let mut findings = Vec::new();
        let subject = self.export_id.as_str();

        if self.record_kind != VARIABLE_EXPLORER_TYPED_EXPORT_RECORD_KIND {
            findings.push(VariableExplorerTypedExportFinding::new(
                "variable_explorer_typed_export.record_kind",
                subject,
                format!(
                    "record_kind must be '{VARIABLE_EXPLORER_TYPED_EXPORT_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_variable_explorer_schema_version
            != NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION
        {
            findings.push(VariableExplorerTypedExportFinding::new(
                "variable_explorer_typed_export.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION}, found {}",
                    self.notebook_variable_explorer_schema_version
                ),
            ));
        }

        if self.variable_handle_refs.is_empty() {
            findings.push(VariableExplorerTypedExportFinding::new(
                "variable_explorer_typed_export.variable_handles_required",
                subject,
                "typed export must select at least one variable",
            ));
        }

        // Posture / redaction consistency.
        match self.export_posture_class {
            VariableExplorerExportPostureClass::Ready => {
                if self.redaction_required {
                    findings.push(VariableExplorerTypedExportFinding::new(
                        "variable_explorer_typed_export.ready_but_redaction_required",
                        subject,
                        "ready posture must not require redaction",
                    ));
                }
            }
            VariableExplorerExportPostureClass::RedactionRequired => {
                if !self.redaction_required {
                    findings.push(VariableExplorerTypedExportFinding::new(
                        "variable_explorer_typed_export.redaction_posture_mismatch",
                        subject,
                        "redaction_required posture must set redaction_required=true",
                    ));
                }
            }
            VariableExplorerExportPostureClass::BlockedByPolicy => {
                if self.output_path_token_ref.is_some() {
                    findings.push(VariableExplorerTypedExportFinding::new(
                        "variable_explorer_typed_export.blocked_no_output_path",
                        subject,
                        "blocked_by_policy must not carry an output_path_token_ref",
                    ));
                }
            }
            VariableExplorerExportPostureClass::RequiresReview => {}
        }

        // Scope / kernel-session consistency.
        match self.export_scope_class {
            VariableExplorerExportScopeClass::CurrentSessionOnly => {
                if self.kernel_session_id_ref.is_none() {
                    findings.push(VariableExplorerTypedExportFinding::new(
                        "variable_explorer_typed_export.current_session_requires_kernel",
                        subject,
                        "current_session_only scope requires a kernel_session_id_ref",
                    ));
                }
            }
            VariableExplorerExportScopeClass::SnapshotSessionOnly => {
                if self.kernel_session_id_ref.is_some() {
                    findings.push(VariableExplorerTypedExportFinding::new(
                        "variable_explorer_typed_export.snapshot_session_no_kernel",
                        subject,
                        "snapshot_session_only scope must not carry a kernel_session_id_ref",
                    ));
                }
            }
            _ => {}
        }

        findings
    }
}

/// Checked-in variable-explorer packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookVariableExplorerPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: variable-explorer sort classes.
    pub sort_classes: Vec<VariableExplorerSortClass>,
    /// Closed vocabulary: variable-explorer filter classes.
    pub filter_classes: Vec<VariableExplorerFilterClass>,
    /// Closed vocabulary: variable-explorer export format classes.
    pub export_format_classes: Vec<VariableExplorerExportFormatClass>,
    /// Closed vocabulary: variable-explorer export posture classes.
    pub export_posture_classes: Vec<VariableExplorerExportPostureClass>,
    /// Closed vocabulary: variable-explorer export scope classes.
    pub export_scope_classes: Vec<VariableExplorerExportScopeClass>,
    /// Worked example variable-explorer states.
    pub example_variable_explorers: Vec<NotebookVariableExplorer>,
    /// Worked example typed-export records.
    pub example_typed_exports: Vec<VariableExplorerTypedExport>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookVariableExplorerPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookVariableExplorerPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION {
            findings.push(NotebookVariableExplorerPacketFinding::new(
                "notebook_variable_explorer_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_VARIABLE_EXPLORER_PACKET_RECORD_KIND {
            findings.push(NotebookVariableExplorerPacketFinding::new(
                "notebook_variable_explorer_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_VARIABLE_EXPLORER_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.sort_classes.len() != VariableExplorerSortClass::ALL.len() {
            findings.push(NotebookVariableExplorerPacketFinding::new(
                "notebook_variable_explorer_packet.sort_classes_coverage",
                subject,
                "sort_classes must list every variant",
            ));
        }
        if self.filter_classes.len() != VariableExplorerFilterClass::ALL.len() {
            findings.push(NotebookVariableExplorerPacketFinding::new(
                "notebook_variable_explorer_packet.filter_classes_coverage",
                subject,
                "filter_classes must list every variant",
            ));
        }
        if self.export_format_classes.len() != VariableExplorerExportFormatClass::ALL.len() {
            findings.push(NotebookVariableExplorerPacketFinding::new(
                "notebook_variable_explorer_packet.export_format_classes_coverage",
                subject,
                "export_format_classes must list every variant",
            ));
        }
        if self.export_posture_classes.len() != VariableExplorerExportPostureClass::ALL.len() {
            findings.push(NotebookVariableExplorerPacketFinding::new(
                "notebook_variable_explorer_packet.export_posture_classes_coverage",
                subject,
                "export_posture_classes must list every variant",
            ));
        }
        if self.export_scope_classes.len() != VariableExplorerExportScopeClass::ALL.len() {
            findings.push(NotebookVariableExplorerPacketFinding::new(
                "notebook_variable_explorer_packet.export_scope_classes_coverage",
                subject,
                "export_scope_classes must list every variant",
            ));
        }

        for explorer in &self.example_variable_explorers {
            findings.extend(explorer.validate().into_iter().map(|f| {
                NotebookVariableExplorerPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for export in &self.example_typed_exports {
            findings.extend(export.validate().into_iter().map(|f| {
                NotebookVariableExplorerPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

impl VariableExplorerSortClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NameAscending,
        Self::NameDescending,
        Self::TypeAscending,
        Self::TypeDescending,
        Self::FreshnessAscending,
        Self::FreshnessDescending,
    ];
}

impl VariableExplorerFilterClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoFilter,
        Self::LiveOnly,
        Self::SnapshotOnly,
        Self::StaleOnly,
        Self::ByType,
        Self::ByName,
    ];
}

impl VariableExplorerExportFormatClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Csv,
        Self::Json,
        Self::Tsv,
        Self::PythonDict,
        Self::MarkdownTable,
    ];
}

impl VariableExplorerExportPostureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Ready,
        Self::RequiresReview,
        Self::BlockedByPolicy,
        Self::RedactionRequired,
    ];
}

impl VariableExplorerExportScopeClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AllVisible,
        Self::SelectedOnly,
        Self::CurrentSessionOnly,
        Self::SnapshotSessionOnly,
    ];
}

/// Parses the checked-in variable-explorer packet JSON.
pub fn current_notebook_variable_explorer_packet(
) -> Result<NotebookVariableExplorerPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_VARIABLE_EXPLORER_PACKET_JSON)
}

#[cfg(test)]
mod tests;
