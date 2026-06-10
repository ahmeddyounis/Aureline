use super::*;

fn sample_comparison_equivalent() -> NotebookResultComparison {
    NotebookResultComparison {
        record_kind: NOTEBOOK_RESULT_COMPARISON_RECORD_KIND.to_owned(),
        notebook_result_comparison_schema_version: NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION,
        comparison_id: "nb.comparison.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        baseline_run_ref: "nb.run.baseline.01".to_owned(),
        current_run_ref: "nb.run.current.01".to_owned(),
        comparison_mode: NotebookComparisonMode::CellAware,
        comparison_scope: NotebookComparisonScopeClass::FullNotebook,
        outcome_class: NotebookComparisonOutcomeClass::Equivalent,
        confounder_refs: vec![],
        summary: "Full notebook cell-aware comparison shows equivalent results.".to_owned(),
    }
}

fn sample_comparison_different() -> NotebookResultComparison {
    NotebookResultComparison {
        record_kind: NOTEBOOK_RESULT_COMPARISON_RECORD_KIND.to_owned(),
        notebook_result_comparison_schema_version: NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION,
        comparison_id: "nb.comparison.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        baseline_run_ref: "nb.run.baseline.01".to_owned(),
        current_run_ref: "nb.run.current.02".to_owned(),
        comparison_mode: NotebookComparisonMode::OutputAware,
        comparison_scope: NotebookComparisonScopeClass::SelectedCells,
        outcome_class: NotebookComparisonOutcomeClass::Different,
        confounder_refs: vec!["nb.confounder.01".to_owned()],
        summary: "Selected cells differ; confounder surfaced.".to_owned(),
    }
}

fn sample_baseline_selected() -> NotebookBaselineSelection {
    NotebookBaselineSelection {
        record_kind: NOTEBOOK_BASELINE_SELECTION_RECORD_KIND.to_owned(),
        notebook_result_comparison_schema_version: NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION,
        selection_id: "nb.baseline.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        baseline_source: NotebookBaselineSourceClass::LastSuccessfulRun,
        selection_state: NotebookBaselineSelectionState::Selected,
        baseline_run_ref: Some("nb.run.baseline.01".to_owned()),
        pinned_by_actor_ref: None,
        summary: "Baseline selected from last successful run.".to_owned(),
    }
}

fn sample_baseline_unavailable() -> NotebookBaselineSelection {
    NotebookBaselineSelection {
        record_kind: NOTEBOOK_BASELINE_SELECTION_RECORD_KIND.to_owned(),
        notebook_result_comparison_schema_version: NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION,
        selection_id: "nb.baseline.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        baseline_source: NotebookBaselineSourceClass::PinnedExperiment,
        selection_state: NotebookBaselineSelectionState::Unavailable,
        baseline_run_ref: None,
        pinned_by_actor_ref: None,
        summary: "Pinned experiment baseline is currently unavailable.".to_owned(),
    }
}

fn sample_confounder_visible() -> NotebookConfounderVisibility {
    NotebookConfounderVisibility {
        record_kind: NOTEBOOK_CONFOUNDER_VISIBILITY_RECORD_KIND.to_owned(),
        notebook_result_comparison_schema_version: NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION,
        confounder_id: "nb.confounder.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        confounder_class: NotebookConfounderClass::EnvironmentDrift,
        visibility_class: NotebookConfounderVisibilityClass::Visible,
        evidence_refs: vec!["nb.env.fingerprint.01".to_owned()],
        summary: "Environment drift detected between baseline and current run.".to_owned(),
    }
}

fn sample_confounder_suppressed() -> NotebookConfounderVisibility {
    NotebookConfounderVisibility {
        record_kind: NOTEBOOK_CONFOUNDER_VISIBILITY_RECORD_KIND.to_owned(),
        notebook_result_comparison_schema_version: NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION,
        confounder_id: "nb.confounder.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        confounder_class: NotebookConfounderClass::DatasetChange,
        visibility_class: NotebookConfounderVisibilityClass::Suppressed,
        evidence_refs: vec![],
        summary: "Dataset change evidence is suppressed by redaction policy.".to_owned(),
    }
}

#[test]
fn comparison_equivalent_validates_clean() {
    let c = sample_comparison_equivalent();
    assert!(
        c.validate().is_empty(),
        "equivalent comparison should be clean: {:?}",
        c.validate()
    );
}

#[test]
fn comparison_different_validates_clean() {
    let c = sample_comparison_different();
    assert!(
        c.validate().is_empty(),
        "different comparison should be clean: {:?}",
        c.validate()
    );
}

#[test]
fn baseline_selected_validates_clean() {
    let b = sample_baseline_selected();
    assert!(
        b.validate().is_empty(),
        "selected baseline should be clean: {:?}",
        b.validate()
    );
}

#[test]
fn baseline_unavailable_validates_clean() {
    let b = sample_baseline_unavailable();
    assert!(
        b.validate().is_empty(),
        "unavailable baseline should be clean: {:?}",
        b.validate()
    );
}

#[test]
fn confounder_visible_validates_clean() {
    let c = sample_confounder_visible();
    assert!(
        c.validate().is_empty(),
        "visible confounder should be clean: {:?}",
        c.validate()
    );
}

#[test]
fn confounder_suppressed_validates_clean() {
    let c = sample_confounder_suppressed();
    assert!(
        c.validate().is_empty(),
        "suppressed confounder should be clean: {:?}",
        c.validate()
    );
}

#[test]
fn comparison_rejects_empty_baseline_run_ref() {
    let mut c = sample_comparison_equivalent();
    c.baseline_run_ref = "".to_owned();
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_result_comparison.baseline_run_ref_required"));
}

#[test]
fn comparison_rejects_empty_current_run_ref() {
    let mut c = sample_comparison_equivalent();
    c.current_run_ref = "".to_owned();
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_result_comparison.current_run_ref_required"));
}

#[test]
fn comparison_rejects_missing_confounder_refs_when_different() {
    let mut c = sample_comparison_different();
    c.confounder_refs = vec![];
    let findings = c.validate();
    assert!(findings.iter().any(
        |f| f.check_id == "notebook_result_comparison.confounder_refs_required_when_different"
    ));
}

#[test]
fn baseline_rejects_missing_run_ref_when_selected() {
    let mut b = sample_baseline_selected();
    b.baseline_run_ref = None;
    let findings = b.validate();
    assert!(findings.iter().any(|f| f.check_id
        == "notebook_baseline_selection.baseline_run_ref_required_when_selected_or_stale"));
}

#[test]
fn baseline_rejects_run_ref_when_unavailable() {
    let mut b = sample_baseline_unavailable();
    b.baseline_run_ref = Some("nb.run.baseline.01".to_owned());
    let findings = b.validate();
    assert!(findings.iter().any(|f| f.check_id == "notebook_baseline_selection.baseline_run_ref_must_be_none_when_unavailable_or_explicit_none"));
}

#[test]
fn confounder_rejects_empty_document_id_ref() {
    let mut c = sample_confounder_visible();
    c.document_id_ref = "".to_owned();
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_confounder_visibility.document_id_ref_required"));
}

#[test]
fn packet_from_embedded_json_validates_clean() {
    let packet = current_notebook_result_comparison_packet().expect("packet JSON must parse");
    assert!(
        packet.validate().is_empty(),
        "embedded packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookComparisonMode::CellAware.as_str(), "cell_aware");
    assert_eq!(
        NotebookComparisonOutcomeClass::Different.as_str(),
        "different"
    );
    assert_eq!(
        NotebookComparisonScopeClass::ActiveCell.as_str(),
        "active_cell"
    );
    assert_eq!(
        NotebookBaselineSourceClass::ManualUpload.as_str(),
        "manual_upload"
    );
    assert_eq!(
        NotebookBaselineSelectionState::ExplicitNone.as_str(),
        "explicit_none"
    );
    assert_eq!(
        NotebookConfounderClass::KernelRestart.as_str(),
        "kernel_restart"
    );
    assert_eq!(
        NotebookConfounderVisibilityClass::NotApplicable.as_str(),
        "not_applicable"
    );
}
