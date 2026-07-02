//! Inline unit tests for the M5 durable-progress certification proof.

use super::*;

#[test]
fn seeded_packet_covers_every_family_and_is_clean() {
    let packet = seeded_m5_durable_progress_certification_packet();
    validate_m5_durable_progress_certification_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), M5DurableJobFamily::ALL.len());
    for family in M5DurableJobFamily::ALL {
        assert!(
            packet.row(family).is_some(),
            "missing row for {}",
            family.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_five_green_and_four_yellow_rows() {
    let packet = seeded_m5_durable_progress_certification_packet();
    assert_eq!(packet.green_row_count, 5);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5DurableJobFamily::Download,
        M5DurableJobFamily::Sync,
        M5DurableJobFamily::ProviderHandoff,
        M5DurableJobFamily::SupportExport,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            DurableProgressCertificationStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_durable_progress_certification_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.family.as_str()
        );
        assert_eq!(row.certification_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_keeps_complete_progress_states_and_labels() {
    let packet = seeded_m5_durable_progress_certification_packet();
    for row in &packet.rows {
        assert!(
            row.progress_states_complete(),
            "row {} does not certify every progress state",
            row.family.as_str()
        );
        assert!(
            row.required_labels_complete(),
            "row {} does not certify every durable-progress required label",
            row.family.as_str()
        );
        assert!(
            row.never_spinner_or_toast_only,
            "row {} allows a spinner-or-toast-only job",
            row.family.as_str()
        );
    }
}

#[test]
fn progress_matrix_union_covers_all_states_and_full_label_set() {
    // The union across the two progress families must cover the full frozen progress-state
    // vocabulary, else no family could be certified green. Progress rows carry
    // source/provider and freshness truth, so the required-label set is the full six —
    // including source_provider and freshness — unlike the pane-control lane.
    let packet = seeded_m5_durable_progress_certification_packet();
    let row = packet.row(M5DurableJobFamily::Indexing).unwrap();
    assert_eq!(row.certified_progress_states, M5ProgressState::ALL.to_vec());
    assert_eq!(
        row.required_labels,
        DURABLE_PROGRESS_REQUIRED_LABELS.to_vec()
    );
    assert!(row
        .required_labels
        .contains(&M5PrimitiveRequiredLabel::Freshness));
    assert!(row
        .required_labels
        .contains(&M5PrimitiveRequiredLabel::SourceProvider));
    assert!(!row.certified_source_freshness_labels.is_empty());
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_durable_progress_certification_packet();
    for row in &packet.rows {
        if !matches!(
            row.derived_status,
            DurableProgressCertificationStatus::Green
        ) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.family.as_str()
            );
        }
    }
}

#[test]
fn compacted_grouped_history_carries_an_active_waiver() {
    let packet = seeded_m5_durable_progress_certification_packet();
    let sync = packet.row(M5DurableJobFamily::Sync).unwrap();
    assert!(matches!(
        sync.grouped_history,
        GroupedHistoryState::DisclosedCompactedHistory
    ));
    assert!(sync.requires_waiver());
    assert!(sync.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn reduced_history_retention_narrows_but_does_not_block() {
    let packet = seeded_m5_durable_progress_certification_packet();
    let download = packet.row(M5DurableJobFamily::Download).unwrap();
    assert!(matches!(
        download.durable_presence,
        DurablePresenceState::DisclosedReducedHistoryRetention
    ));
    assert_eq!(
        download.derived_status,
        DurableProgressCertificationStatus::Yellow
    );
    assert!(download
        .certification_causes
        .iter()
        .any(|cause| cause.disclosed
            && matches!(
                cause.trigger,
                M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState
            )));
}

#[test]
fn transient_spinner_blocks_the_indexing_lane() {
    let packet =
        seeded_m5_durable_progress_certification_packet_indexing_transient_spinner_blocked();
    let row = packet.row(M5DurableJobFamily::Indexing).unwrap();
    assert_eq!(row.derived_status, DurableProgressCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        DurableProgressCertificationFinding::ProgressNotDurable { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState
        )));
    assert!(validate_m5_durable_progress_certification_packet(&packet).is_err());
}

#[test]
fn missing_attribution_blocks_the_notebook_lane() {
    let packet =
        seeded_m5_durable_progress_certification_packet_notebook_attribution_missing_blocked();
    let row = packet.row(M5DurableJobFamily::NotebookRuntime).unwrap();
    assert_eq!(row.derived_status, DurableProgressCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        DurableProgressCertificationFinding::AttributionMissing { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::GroupedProgressUnattributed
        )));
    assert!(validate_m5_durable_progress_certification_packet(&packet).is_err());
}

#[test]
fn lost_grouped_history_blocks_the_request_lane() {
    let packet = seeded_m5_durable_progress_certification_packet_request_history_lost_blocked();
    let row = packet.row(M5DurableJobFamily::RequestDataLoad).unwrap();
    assert_eq!(row.derived_status, DurableProgressCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        DurableProgressCertificationFinding::GroupedHistoryLost { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::ProgressLostOnLookAway
        )));
    assert!(validate_m5_durable_progress_certification_packet(&packet).is_err());
}

#[test]
fn absent_export_blocks_the_update_lane() {
    let packet =
        seeded_m5_durable_progress_certification_packet_update_progress_absent_from_capture_blocked(
        );
    let row = packet.row(M5DurableJobFamily::Update).unwrap();
    assert_eq!(row.derived_status, DurableProgressCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        DurableProgressCertificationFinding::ProgressStateAbsentFromCapture { .. }
    )));
    assert!(validate_m5_durable_progress_certification_packet(&packet).is_err());
}

#[test]
fn spinner_or_toast_only_invariant_blocks_the_branch_agent_lane() {
    let packet =
        seeded_m5_durable_progress_certification_packet_branch_agent_spinner_or_toast_only_blocked(
        );
    let row = packet.row(M5DurableJobFamily::BranchAgent).unwrap();
    assert_eq!(row.derived_status, DurableProgressCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        DurableProgressCertificationFinding::JobSpinnerOrToastOnly { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState
        )));
    assert!(validate_m5_durable_progress_certification_packet(&packet).is_err());
}

#[test]
fn every_row_pulls_progress_bindings_from_the_matrix() {
    let packet = seeded_m5_durable_progress_certification_packet();
    for row in &packet.rows {
        assert!(!row.certified_progress_states.is_empty());
        assert!(!row.certified_source_freshness_labels.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(!row.required_labels.is_empty());
        assert!(!row.consumer_surfaces.is_empty());
        assert!(!row.applicable_downgrade_triggers.is_empty());
        assert_eq!(row.shell_zone_slot, M5ShellZoneSlot::BottomPanel);
        assert_eq!(
            row.driven_primitive_families,
            vec![
                M5ShellPrimitiveFamily::ProgressIndicator,
                M5ShellPrimitiveFamily::DurableJobRow,
            ]
        );
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::ProgressLostOnLookAway));
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::GroupedProgressUnattributed));
    }
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_durable_progress_certification_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.shell_automation_refs.is_empty());

    let sync = dashboard
        .rows
        .iter()
        .find(|row| row.family == M5DurableJobFamily::Sync)
        .unwrap();
    assert_eq!(sync.status, DurableProgressCertificationStatus::Yellow);
    assert!(sync.has_active_waiver);
    assert!(sync.cause_tokens.contains(
        &M5ShellPrimitiveDowngradeTrigger::ProgressLostOnLookAway
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_durable_progress_certification_packet();
    let export = DurableProgressCertificationSupportExport::from_packet(
        M5_DURABLE_PROGRESS_CERTIFICATION_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.family.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_family() {
    let packet = seeded_m5_durable_progress_certification_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for family in M5DurableJobFamily::ALL {
        assert!(
            markdown.contains(family.label()),
            "markdown omits {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.as_str()),
            "csv omits {}",
            family.as_str()
        );
    }
    assert!(markdown.contains("m5_durable_progress_certification_fixtures"));
    assert!(markdown.contains("waiver:sync-compacted-grouped-history:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_durable_progress_certification_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("secret"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = DurableProgressCertificationWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        family: M5DurableJobFamily::Sync,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
