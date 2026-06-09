use super::*;
use crate::{
    VariableExplorerEntry, VariableExplorerEntryActionClass, VariableExplorerFreshnessClass,
    VariableExplorerTruncationClass, NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
    VARIABLE_EXPLORER_ENTRY_RECORD_KIND,
};

fn sample_live_entry() -> VariableExplorerEntry {
    VariableExplorerEntry {
        record_kind: VARIABLE_EXPLORER_ENTRY_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        entry_id: "nb.var.entry.live.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        variable_handle_ref: "nb.var.handle.df".to_owned(),
        display_name_label: "df".to_owned(),
        type_descriptor_ref: "nb.type.pandas_dataframe".to_owned(),
        freshness_class: VariableExplorerFreshnessClass::LiveFromCurrentSession,
        truncation_class: VariableExplorerTruncationClass::NoTruncation,
        available_actions: vec![
            VariableExplorerEntryActionClass::OpenLiveViewer,
            VariableExplorerEntryActionClass::ExportWithRedaction,
        ],
        summary: "Live DataFrame variable 'df' from current session.".to_owned(),
    }
}

fn sample_snapshot_entry() -> VariableExplorerEntry {
    VariableExplorerEntry {
        record_kind: VARIABLE_EXPLORER_ENTRY_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        entry_id: "nb.var.entry.snapshot.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: None,
        variable_handle_ref: "nb.var.handle.model".to_owned(),
        display_name_label: "model".to_owned(),
        type_descriptor_ref: "nb.type.sklearn_pipeline".to_owned(),
        freshness_class: VariableExplorerFreshnessClass::SnapshotFromPriorSession,
        truncation_class: VariableExplorerTruncationClass::TruncatedForSize,
        available_actions: vec![
            VariableExplorerEntryActionClass::OpenSnapshotViewer,
            VariableExplorerEntryActionClass::ReviewBeforeExport,
        ],
        summary: "Snapshot pipeline variable 'model' from prior session.".to_owned(),
    }
}

fn sample_stale_entry() -> VariableExplorerEntry {
    VariableExplorerEntry {
        record_kind: VARIABLE_EXPLORER_ENTRY_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        entry_id: "nb.var.entry.stale.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        variable_handle_ref: "nb.var.handle.x".to_owned(),
        display_name_label: "x".to_owned(),
        type_descriptor_ref: "nb.type.numpy_array".to_owned(),
        freshness_class: VariableExplorerFreshnessClass::StaleAfterRestart,
        truncation_class: VariableExplorerTruncationClass::NoTruncation,
        available_actions: vec![VariableExplorerEntryActionClass::DismissFromExplorer],
        summary: "Stale array variable 'x' after kernel restart.".to_owned(),
    }
}

fn sample_no_kernel_entry() -> VariableExplorerEntry {
    VariableExplorerEntry {
        record_kind: VARIABLE_EXPLORER_ENTRY_RECORD_KIND.to_owned(),
        notebook_runtime_truth_schema_version: NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
        entry_id: "nb.var.entry.no_kernel.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: None,
        variable_handle_ref: "nb.var.handle.none".to_owned(),
        display_name_label: "(no variables)".to_owned(),
        type_descriptor_ref: "nb.type.none".to_owned(),
        freshness_class: VariableExplorerFreshnessClass::NoLiveVariablesNoKernel,
        truncation_class: VariableExplorerTruncationClass::UnsupportedTypeNoPreview,
        available_actions: vec![],
        summary: "No kernel selected; no live variables available.".to_owned(),
    }
}

fn sample_live_explorer() -> NotebookVariableExplorer {
    NotebookVariableExplorer {
        record_kind: NOTEBOOK_VARIABLE_EXPLORER_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        explorer_state_id: "nb.var.explorer.live.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        entries: vec![sample_live_entry(), sample_live_entry()],
        sort_class: VariableExplorerSortClass::NameAscending,
        filter_class: VariableExplorerFilterClass::NoFilter,
        search_query_label: None,
        entry_count_visible: 2,
        entry_count_total: 2,
        has_more_entries: false,
        truncation_notice_visible: false,
        summary: "Live variable explorer with 2 entries.".to_owned(),
    }
}

fn sample_filtered_live_explorer() -> NotebookVariableExplorer {
    NotebookVariableExplorer {
        record_kind: NOTEBOOK_VARIABLE_EXPLORER_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        explorer_state_id: "nb.var.explorer.filtered.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        entries: vec![sample_live_entry()],
        sort_class: VariableExplorerSortClass::FreshnessDescending,
        filter_class: VariableExplorerFilterClass::LiveOnly,
        search_query_label: None,
        entry_count_visible: 1,
        entry_count_total: 3,
        has_more_entries: false,
        truncation_notice_visible: false,
        summary: "Filtered to live variables only.".to_owned(),
    }
}

fn sample_stale_explorer() -> NotebookVariableExplorer {
    NotebookVariableExplorer {
        record_kind: NOTEBOOK_VARIABLE_EXPLORER_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        explorer_state_id: "nb.var.explorer.stale.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        entries: vec![sample_stale_entry()],
        sort_class: VariableExplorerSortClass::FreshnessAscending,
        filter_class: VariableExplorerFilterClass::StaleOnly,
        search_query_label: None,
        entry_count_visible: 1,
        entry_count_total: 1,
        has_more_entries: false,
        truncation_notice_visible: false,
        summary: "Stale variable explorer after restart.".to_owned(),
    }
}

fn sample_no_kernel_explorer() -> NotebookVariableExplorer {
    NotebookVariableExplorer {
        record_kind: NOTEBOOK_VARIABLE_EXPLORER_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        explorer_state_id: "nb.var.explorer.no_kernel.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: None,
        entries: vec![sample_no_kernel_entry()],
        sort_class: VariableExplorerSortClass::NameAscending,
        filter_class: VariableExplorerFilterClass::NoFilter,
        search_query_label: None,
        entry_count_visible: 1,
        entry_count_total: 1,
        has_more_entries: false,
        truncation_notice_visible: false,
        summary: "Variable explorer with no kernel.".to_owned(),
    }
}

fn sample_truncated_explorer() -> NotebookVariableExplorer {
    NotebookVariableExplorer {
        record_kind: NOTEBOOK_VARIABLE_EXPLORER_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        explorer_state_id: "nb.var.explorer.truncated.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        entries: vec![sample_live_entry()],
        sort_class: VariableExplorerSortClass::TypeAscending,
        filter_class: VariableExplorerFilterClass::NoFilter,
        search_query_label: None,
        entry_count_visible: 1,
        entry_count_total: 150,
        has_more_entries: true,
        truncation_notice_visible: true,
        summary: "Variable explorer truncated at 1 of 150 entries.".to_owned(),
    }
}

fn sample_snapshot_explorer() -> NotebookVariableExplorer {
    NotebookVariableExplorer {
        record_kind: NOTEBOOK_VARIABLE_EXPLORER_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        explorer_state_id: "nb.var.explorer.snapshot.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: None,
        entries: vec![sample_snapshot_entry()],
        sort_class: VariableExplorerSortClass::NameDescending,
        filter_class: VariableExplorerFilterClass::SnapshotOnly,
        search_query_label: None,
        entry_count_visible: 1,
        entry_count_total: 1,
        has_more_entries: false,
        truncation_notice_visible: false,
        summary: "Snapshot variable explorer from prior session.".to_owned(),
    }
}

fn sample_ready_csv_export() -> VariableExplorerTypedExport {
    VariableExplorerTypedExport {
        record_kind: VARIABLE_EXPLORER_TYPED_EXPORT_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        export_id: "nb.var.export.ready.csv.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        variable_handle_refs: vec!["nb.var.handle.df".to_owned()],
        export_format_class: VariableExplorerExportFormatClass::Csv,
        export_posture_class: VariableExplorerExportPostureClass::Ready,
        export_scope_class: VariableExplorerExportScopeClass::CurrentSessionOnly,
        redaction_required: false,
        output_path_token_ref: Some("nb.path.export.01".to_owned()),
        summary: "Ready CSV export of live DataFrame.".to_owned(),
    }
}

fn sample_review_json_export() -> VariableExplorerTypedExport {
    VariableExplorerTypedExport {
        record_kind: VARIABLE_EXPLORER_TYPED_EXPORT_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        export_id: "nb.var.export.review.json.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: None,
        variable_handle_refs: vec!["nb.var.handle.model".to_owned()],
        export_format_class: VariableExplorerExportFormatClass::Json,
        export_posture_class: VariableExplorerExportPostureClass::RequiresReview,
        export_scope_class: VariableExplorerExportScopeClass::SnapshotSessionOnly,
        redaction_required: false,
        output_path_token_ref: None,
        summary: "JSON export of snapshot model awaiting review.".to_owned(),
    }
}

fn sample_blocked_policy_export() -> VariableExplorerTypedExport {
    VariableExplorerTypedExport {
        record_kind: VARIABLE_EXPLORER_TYPED_EXPORT_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        export_id: "nb.var.export.blocked.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        variable_handle_refs: vec!["nb.var.handle.secret".to_owned()],
        export_format_class: VariableExplorerExportFormatClass::PythonDict,
        export_posture_class: VariableExplorerExportPostureClass::BlockedByPolicy,
        export_scope_class: VariableExplorerExportScopeClass::AllVisible,
        redaction_required: true,
        output_path_token_ref: None,
        summary: "Export blocked by policy due to sensitive variable.".to_owned(),
    }
}

fn sample_redaction_required_export() -> VariableExplorerTypedExport {
    VariableExplorerTypedExport {
        record_kind: VARIABLE_EXPLORER_TYPED_EXPORT_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        export_id: "nb.var.export.redaction.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        variable_handle_refs: vec!["nb.var.handle.pii".to_owned()],
        export_format_class: VariableExplorerExportFormatClass::Tsv,
        export_posture_class: VariableExplorerExportPostureClass::RedactionRequired,
        export_scope_class: VariableExplorerExportScopeClass::SelectedOnly,
        redaction_required: true,
        output_path_token_ref: Some("nb.path.export.02".to_owned()),
        summary: "TSV export requires redaction of PII fields.".to_owned(),
    }
}

fn sample_markdown_table_export() -> VariableExplorerTypedExport {
    VariableExplorerTypedExport {
        record_kind: VARIABLE_EXPLORER_TYPED_EXPORT_RECORD_KIND.to_owned(),
        notebook_variable_explorer_schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        export_id: "nb.var.export.markdown.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        kernel_session_id_ref: Some("nb.kernel.session.01".to_owned()),
        variable_handle_refs: vec!["nb.var.handle.df".to_owned(), "nb.var.handle.summary".to_owned()],
        export_format_class: VariableExplorerExportFormatClass::MarkdownTable,
        export_posture_class: VariableExplorerExportPostureClass::Ready,
        export_scope_class: VariableExplorerExportScopeClass::AllVisible,
        redaction_required: false,
        output_path_token_ref: Some("nb.path.export.03".to_owned()),
        summary: "Markdown table export of all visible variables.".to_owned(),
    }
}

#[test]
fn live_explorer_validates_clean() {
    let explorer = sample_live_explorer();
    assert!(
        explorer.validate().is_empty(),
        "live explorer should be clean: {:?}",
        explorer.validate()
    );
}

#[test]
fn filtered_live_explorer_validates_clean() {
    let explorer = sample_filtered_live_explorer();
    assert!(
        explorer.validate().is_empty(),
        "filtered live explorer should be clean: {:?}",
        explorer.validate()
    );
}

#[test]
fn stale_explorer_validates_clean() {
    let explorer = sample_stale_explorer();
    assert!(
        explorer.validate().is_empty(),
        "stale explorer should be clean: {:?}",
        explorer.validate()
    );
}

#[test]
fn no_kernel_explorer_validates_clean() {
    let explorer = sample_no_kernel_explorer();
    assert!(
        explorer.validate().is_empty(),
        "no-kernel explorer should be clean: {:?}",
        explorer.validate()
    );
}

#[test]
fn truncated_explorer_validates_clean() {
    let explorer = sample_truncated_explorer();
    assert!(
        explorer.validate().is_empty(),
        "truncated explorer should be clean: {:?}",
        explorer.validate()
    );
}

#[test]
fn snapshot_explorer_validates_clean() {
    let explorer = sample_snapshot_explorer();
    assert!(
        explorer.validate().is_empty(),
        "snapshot explorer should be clean: {:?}",
        explorer.validate()
    );
}

#[test]
fn visible_count_exceeds_total_is_rejected() {
    let mut explorer = sample_live_explorer();
    explorer.entry_count_visible = 5;
    explorer.entry_count_total = 2;
    let findings = explorer.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_variable_explorer.visible_exceeds_total"));
}

#[test]
fn visible_count_mismatch_is_rejected() {
    let mut explorer = sample_live_explorer();
    explorer.entry_count_visible = 5;
    let findings = explorer.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_variable_explorer.visible_count_mismatch"));
}

#[test]
fn live_only_with_stale_entry_is_rejected() {
    let mut explorer = sample_live_explorer();
    explorer.filter_class = VariableExplorerFilterClass::LiveOnly;
    explorer.entries = vec![sample_stale_entry()];
    explorer.entry_count_visible = 1;
    let findings = explorer.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_variable_explorer.live_only_inconsistent"));
}

#[test]
fn stale_only_with_live_entry_is_rejected() {
    let mut explorer = sample_live_explorer();
    explorer.filter_class = VariableExplorerFilterClass::StaleOnly;
    explorer.entries = vec![sample_live_entry()];
    explorer.entry_count_visible = 1;
    let findings = explorer.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_variable_explorer.stale_only_inconsistent"));
}

#[test]
fn has_more_without_truncation_notice_is_rejected() {
    let mut explorer = sample_truncated_explorer();
    explorer.truncation_notice_visible = false;
    let findings = explorer.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_variable_explorer.truncation_notice_required"));
}

#[test]
fn ready_csv_export_validates_clean() {
    let export = sample_ready_csv_export();
    assert!(
        export.validate().is_empty(),
        "ready CSV export should be clean: {:?}",
        export.validate()
    );
}

#[test]
fn review_json_export_validates_clean() {
    let export = sample_review_json_export();
    assert!(
        export.validate().is_empty(),
        "review JSON export should be clean: {:?}",
        export.validate()
    );
}

#[test]
fn blocked_policy_export_validates_clean() {
    let export = sample_blocked_policy_export();
    assert!(
        export.validate().is_empty(),
        "blocked policy export should be clean: {:?}",
        export.validate()
    );
}

#[test]
fn redaction_required_export_validates_clean() {
    let export = sample_redaction_required_export();
    assert!(
        export.validate().is_empty(),
        "redaction required export should be clean: {:?}",
        export.validate()
    );
}

#[test]
fn markdown_table_export_validates_clean() {
    let export = sample_markdown_table_export();
    assert!(
        export.validate().is_empty(),
        "markdown table export should be clean: {:?}",
        export.validate()
    );
}

#[test]
fn empty_variable_handles_is_rejected() {
    let mut export = sample_ready_csv_export();
    export.variable_handle_refs = vec![];
    let findings = export.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "variable_explorer_typed_export.variable_handles_required"));
}

#[test]
fn ready_but_redaction_required_is_rejected() {
    let mut export = sample_ready_csv_export();
    export.redaction_required = true;
    let findings = export.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "variable_explorer_typed_export.ready_but_redaction_required"));
}

#[test]
fn redaction_posture_mismatch_is_rejected() {
    let mut export = sample_redaction_required_export();
    export.redaction_required = false;
    let findings = export.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "variable_explorer_typed_export.redaction_posture_mismatch"));
}

#[test]
fn blocked_with_output_path_is_rejected() {
    let mut export = sample_blocked_policy_export();
    export.output_path_token_ref = Some("nb.path.disallowed".to_owned());
    let findings = export.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "variable_explorer_typed_export.blocked_no_output_path"));
}

#[test]
fn current_session_without_kernel_is_rejected() {
    let mut export = sample_ready_csv_export();
    export.kernel_session_id_ref = None;
    let findings = export.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "variable_explorer_typed_export.current_session_requires_kernel"));
}

#[test]
fn snapshot_session_with_kernel_is_rejected() {
    let mut export = sample_review_json_export();
    export.kernel_session_id_ref = Some("nb.kernel.session.01".to_owned());
    let findings = export.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "variable_explorer_typed_export.snapshot_session_no_kernel"));
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookVariableExplorerPacket {
        schema_version: NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION,
        record_kind: NOTEBOOK_VARIABLE_EXPLORER_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.var.explorer.packet.m5.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        sort_classes: VariableExplorerSortClass::ALL.to_vec(),
        filter_classes: VariableExplorerFilterClass::ALL.to_vec(),
        export_format_classes: VariableExplorerExportFormatClass::ALL.to_vec(),
        export_posture_classes: VariableExplorerExportPostureClass::ALL.to_vec(),
        export_scope_classes: VariableExplorerExportScopeClass::ALL.to_vec(),
        example_variable_explorers: vec![
            sample_live_explorer(),
            sample_filtered_live_explorer(),
            sample_stale_explorer(),
            sample_no_kernel_explorer(),
            sample_truncated_explorer(),
            sample_snapshot_explorer(),
        ],
        example_typed_exports: vec![
            sample_ready_csv_export(),
            sample_review_json_export(),
            sample_blocked_policy_export(),
            sample_redaction_required_export(),
            sample_markdown_table_export(),
        ],
        summary: "Notebook variable explorer, live or snapshot or stale labels, and typed export packet v1."
            .to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_variable_explorer_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, NOTEBOOK_VARIABLE_EXPLORER_PACKET_RECORD_KIND);
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(VariableExplorerSortClass::NameAscending.as_str(), "name_ascending");
    assert_eq!(VariableExplorerSortClass::FreshnessDescending.as_str(), "freshness_descending");

    assert_eq!(VariableExplorerFilterClass::LiveOnly.as_str(), "live_only");
    assert_eq!(VariableExplorerFilterClass::ByName.as_str(), "by_name");

    assert_eq!(VariableExplorerExportFormatClass::Csv.as_str(), "csv");
    assert_eq!(VariableExplorerExportFormatClass::MarkdownTable.as_str(), "markdown_table");

    assert_eq!(VariableExplorerExportPostureClass::Ready.as_str(), "ready");
    assert_eq!(VariableExplorerExportPostureClass::BlockedByPolicy.as_str(), "blocked_by_policy");

    assert_eq!(VariableExplorerExportScopeClass::AllVisible.as_str(), "all_visible");
    assert_eq!(
        VariableExplorerExportScopeClass::SnapshotSessionOnly.as_str(),
        "snapshot_session_only"
    );
}
