//! Unit tests for the M5 OS-attention parity audit.

use super::*;

fn find_surface<'a>(report: &'a M5OsAttentionReport, surface_id: &str) -> &'a M5OsAttentionRow {
    report
        .rows
        .iter()
        .find(|row| row.descriptor.surface_id == surface_id)
        .unwrap_or_else(|| panic!("missing surface {surface_id}"))
}

fn binding_mut<'a>(
    row: &'a mut M5OsAttentionRow,
    guarantee: M5OsAttentionGuarantee,
) -> &'a mut M5OsAttentionBinding {
    row.bindings
        .iter_mut()
        .find(|binding| binding.guarantee == guarantee)
        .expect("guarantee binding present")
}

#[test]
fn seeded_report_is_clean_and_validates() {
    let report = seeded_m5_os_attention_report();
    assert!(report.report_clean, "seeded report must be clean");
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    validate_m5_os_attention_report(&report).expect("seeded report must validate");
}

#[test]
fn seeded_report_covers_every_guarantee() {
    let report = seeded_m5_os_attention_report();
    assert!(report.every_required_guarantee_qualified());
    assert_eq!(
        report.required_guarantees,
        M5OsAttentionGuarantee::required_guarantees().to_vec()
    );
}

#[test]
fn rows_are_sorted_by_surface_id() {
    let report = seeded_m5_os_attention_report();
    let mut sorted = report.rows.clone();
    sorted.sort_by(|a, b| a.descriptor.surface_id.cmp(&b.descriptor.surface_id));
    let ids: Vec<&str> = report
        .rows
        .iter()
        .map(|row| row.descriptor.surface_id.as_str())
        .collect();
    let sorted_ids: Vec<&str> = sorted
        .iter()
        .map(|row| row.descriptor.surface_id.as_str())
        .collect();
    assert_eq!(ids, sorted_ids);
}

#[test]
fn reuses_durable_job_families_and_count_classes() {
    let report = seeded_m5_os_attention_report();
    // Reused durable job-family vocabulary, not a desktop-only enum.
    assert_eq!(
        find_surface(&report, "os:task_run").descriptor.job_family,
        DurableAttentionJobFamily::TaskRun
    );
    // Reused durable count class for the badge.
    assert_eq!(
        find_surface(&report, "os:test_run")
            .envelope
            .badge_count_class,
        AggregateCountClass::FailedRuns
    );
    for row in &report.rows {
        assert!(row.descriptor.derived_from_durable_object);
        assert_eq!(
            row.envelope.durable_job_id_ref,
            row.descriptor.durable_job_id_ref
        );
    }
}

#[test]
fn progress_only_qualifies_for_surfaces_with_progress() {
    let report = seeded_m5_os_attention_report();
    let with_progress = binding_mut(
        &mut find_surface(&report, "os:task_run").clone(),
        M5OsAttentionGuarantee::ProgressNamedJobClass,
    )
    .qualification_status;
    assert_eq!(with_progress, M5OsQualificationStatus::Qualified);

    let approval = find_surface(&report, "os:ai_review");
    let progress = approval
        .bindings
        .iter()
        .find(|b| b.guarantee == M5OsAttentionGuarantee::ProgressNamedJobClass)
        .unwrap();
    assert_eq!(
        progress.qualification_status,
        M5OsQualificationStatus::NotApplicable
    );
    assert!(progress.narrowing_reason.is_some());
}

#[test]
fn lock_screen_leak_is_a_blocker() {
    let mut row = find_surface(&seeded_m5_os_attention_report(), "os:ai_review").clone();
    binding_mut(&mut row, M5OsAttentionGuarantee::PrivacySafeSummary).projected_lock_screen =
        Some(M5OsLockScreenDisclosure::LeaksProtectedDetail);
    let rebuilt = build_m5_os_attention_row(row.descriptor, row.envelope, row.bindings);
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "lock_screen_leak"));
}

#[test]
fn generic_progress_spinner_is_a_blocker() {
    let mut row = find_surface(&seeded_m5_os_attention_report(), "os:indexing").clone();
    binding_mut(&mut row, M5OsAttentionGuarantee::ProgressNamedJobClass).projected_progress_basis =
        Some(M5OsProgressBasis::GenericSpinner);
    let rebuilt = build_m5_os_attention_row(row.descriptor, row.envelope, row.bindings);
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "progress_generic_spinner"));
}

#[test]
fn raw_event_badge_is_a_blocker() {
    let mut row = find_surface(&seeded_m5_os_attention_report(), "os:test_run").clone();
    binding_mut(&mut row, M5OsAttentionGuarantee::BadgeDurableClass).projected_badge_basis =
        Some(M5OsBadgeBasis::RawEventFanout);
    let rebuilt = build_m5_os_attention_row(row.descriptor, row.envelope, row.bindings);
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "badge_raw_event_fanout"));
}

#[test]
fn suppression_divergence_is_a_blocker() {
    let mut row = find_surface(&seeded_m5_os_attention_report(), "os:admin_policy").clone();
    binding_mut(&mut row, M5OsAttentionGuarantee::SuppressionParity).projected_suppression_parity =
        Some(M5OsSuppressionParity::DivergesFromInApp);
    let rebuilt = build_m5_os_attention_row(row.descriptor, row.envelope, row.bindings);
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "suppression_divergence"));
}

#[test]
fn missing_suppression_audit_is_a_blocker() {
    let mut row = find_surface(&seeded_m5_os_attention_report(), "os:admin_policy").clone();
    binding_mut(&mut row, M5OsAttentionGuarantee::SuppressionParity)
        .projected_suppression_audit_visible = Some(false);
    let rebuilt = build_m5_os_attention_row(row.descriptor, row.envelope, row.bindings);
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "suppression_audit_missing"));
}

#[test]
fn lost_reopen_target_is_a_blocker() {
    let mut row = find_surface(&seeded_m5_os_attention_report(), "os:task_run").clone();
    binding_mut(&mut row, M5OsAttentionGuarantee::ExactReopenParity).projected_reopen_outcome =
        Some(M5OsReopenOutcome::TargetLost);
    let rebuilt = build_m5_os_attention_row(row.descriptor, row.envelope, row.bindings);
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "reopen_target_lost"));
}

#[test]
fn truthful_placeholder_reopen_is_not_a_blocker() {
    let mut row = find_surface(&seeded_m5_os_attention_report(), "os:task_run").clone();
    binding_mut(&mut row, M5OsAttentionGuarantee::ExactReopenParity).projected_reopen_outcome =
        Some(M5OsReopenOutcome::TruthfulPlaceholder);
    let rebuilt = build_m5_os_attention_row(row.descriptor, row.envelope, row.bindings);
    assert!(rebuilt.blocking_findings.is_empty());
}

#[test]
fn desktop_only_synthesized_surface_is_a_blocker() {
    let mut row = find_surface(&seeded_m5_os_attention_report(), "os:task_run").clone();
    row.descriptor.derived_from_durable_object = false;
    let rebuilt = build_m5_os_attention_row(row.descriptor, row.envelope, row.bindings);
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "surface_not_derived_from_durable_object"));
}

#[test]
fn envelope_descriptor_mismatch_is_a_blocker() {
    let mut row = find_surface(&seeded_m5_os_attention_report(), "os:task_run").clone();
    row.envelope.durable_job_id_ref = "obj:durable-job:wrong".to_owned();
    let rebuilt = build_m5_os_attention_row(row.descriptor, row.envelope, row.bindings);
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "envelope_descriptor_mismatch"));
}

#[test]
fn high_stakes_surface_requires_reopen_on_every_guarantee() {
    let report = seeded_m5_os_attention_report();
    let row = find_surface(&report, "os:remote_reconnect");
    assert!(row.high_stakes);
    for binding in &row.bindings {
        if binding.qualification_status == M5OsQualificationStatus::Qualified {
            assert!(
                binding.projected_reopen_outcome.is_some(),
                "high-stakes qualified binding {:?} must carry a reopen outcome",
                binding.guarantee
            );
        }
    }
}

#[test]
fn missing_narrowing_reason_is_a_blocker() {
    let mut row = find_surface(&seeded_m5_os_attention_report(), "os:ai_review").clone();
    binding_mut(&mut row, M5OsAttentionGuarantee::ProgressNamedJobClass).narrowing_reason = None;
    let rebuilt = build_m5_os_attention_row(row.descriptor, row.envelope, row.bindings);
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "missing_narrowing_reason"));
}

#[test]
fn support_export_quotes_every_surface() {
    let report = seeded_m5_os_attention_report();
    let export =
        M5OsAttentionSupportExport::from_report(M5_OS_ATTENTION_SUPPORT_EXPORT_ID, report.clone());
    assert!(export.case_ids.contains(&report.report_id));
    for row in &report.rows {
        assert!(export.case_ids.contains(&row.descriptor.surface_id));
        assert!(export
            .case_ids
            .contains(&row.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn json_round_trips() {
    let report = seeded_m5_os_attention_report();
    let json = serde_json::to_string(&report).expect("serialize");
    let parsed: M5OsAttentionReport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(report, parsed);
}

#[test]
fn markdown_names_every_guarantee() {
    let md = seeded_m5_os_attention_report().render_markdown();
    for guarantee in M5OsAttentionGuarantee::required_guarantees() {
        assert!(
            md.contains(guarantee.display_label()),
            "markdown missing {}",
            guarantee.display_label()
        );
    }
}
