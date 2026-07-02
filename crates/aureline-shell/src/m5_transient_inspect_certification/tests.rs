//! Inline unit tests for the M5 transient-inspect certification proof.

use super::*;

#[test]
fn seeded_packet_covers_every_context_and_is_clean() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    validate_m5_transient_inspect_certification_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), M5InspectContext::ALL.len());
    for context in M5InspectContext::ALL {
        assert!(
            packet.row(context).is_some(),
            "missing row for {}",
            context.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_three_green_and_four_yellow_rows() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    assert_eq!(packet.green_row_count, 3);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for context in [
        M5InspectContext::ReviewChange,
        M5InspectContext::DataGrid,
        M5InspectContext::Profiler,
        M5InspectContext::Operator,
    ] {
        assert_eq!(
            packet.row(context).unwrap().derived_status,
            TransientInspectCertificationStatus::Yellow,
            "{} should auto-narrow to yellow",
            context.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.context.as_str()
        );
        assert_eq!(row.certification_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_keeps_complete_representation_promotion_and_labels() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    for row in &packet.rows {
        assert!(
            row.representation_classes_complete(),
            "row {} does not certify every representation class",
            row.context.as_str()
        );
        assert!(
            row.promotion_states_complete(),
            "row {} does not certify every promotion state",
            row.context.as_str()
        );
        assert!(
            row.required_labels_complete(),
            "row {} does not certify every required label",
            row.context.as_str()
        );
        assert!(
            row.stale_labels_present(),
            "row {} does not certify the stale/cached labels",
            row.context.as_str()
        );
        assert!(
            row.tooltip_never_sole_critical_instruction,
            "row {} lets a tooltip carry the sole critical instruction",
            row.context.as_str()
        );
    }
}

#[test]
fn transient_matrix_union_covers_all_representation_and_promotion_vocab() {
    // The union across the four transient families must cover the full frozen
    // representation-class and promotion-state vocabulary, else no context could be
    // certified green.
    let packet = seeded_m5_transient_inspect_certification_packet();
    let row = packet.row(M5InspectContext::SearchResults).unwrap();
    assert_eq!(
        row.certified_representation_classes,
        M5RepresentationClass::ALL.to_vec()
    );
    assert_eq!(
        row.certified_promotion_states,
        M5PromotionState::ALL.to_vec()
    );
    assert_eq!(row.required_labels, M5PrimitiveRequiredLabel::ALL.to_vec());
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    for row in &packet.rows {
        if !matches!(
            row.derived_status,
            TransientInspectCertificationStatus::Green
        ) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.context.as_str()
            );
        }
    }
}

#[test]
fn reduced_promotion_path_carries_an_active_waiver() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    let profiler = packet.row(M5InspectContext::Profiler).unwrap();
    assert!(matches!(
        profiler.promotion_continuity,
        PromotionContinuityState::DisclosedReducedPromotionPath
    ));
    assert!(profiler.requires_waiver());
    assert!(profiler.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn reduced_reach_route_narrows_but_does_not_block() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    let operator = packet.row(M5InspectContext::Operator).unwrap();
    assert!(matches!(
        operator.non_hover_reach,
        NonHoverReachState::DisclosedReducedReachRoute
    ));
    assert_eq!(
        operator.derived_status,
        TransientInspectCertificationStatus::Yellow
    );
    assert!(operator
        .certification_causes
        .iter()
        .any(|cause| cause.disclosed
            && matches!(
                cause.trigger,
                M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
            )));
}

#[test]
fn hidden_representation_blocks_the_search_lane() {
    let packet =
        seeded_m5_transient_inspect_certification_packet_search_representation_hidden_blocked();
    let row = packet.row(M5InspectContext::SearchResults).unwrap();
    assert_eq!(row.derived_status, TransientInspectCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TransientInspectCertificationFinding::RepresentationTruthHidden { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::SourceFreshnessHidden
        )));
    assert!(validate_m5_transient_inspect_certification_packet(&packet).is_err());
}

#[test]
fn dropped_promotion_blocks_the_docs_lane() {
    let packet = seeded_m5_transient_inspect_certification_packet_docs_promotion_dropped_blocked();
    let row = packet.row(M5InspectContext::DocsHelp).unwrap();
    assert_eq!(row.derived_status, TransientInspectCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TransientInspectCertificationFinding::PromotionDroppedTruth { .. }
    )));
    assert!(validate_m5_transient_inspect_certification_packet(&packet).is_err());
}

#[test]
fn hover_only_information_blocks_the_editor_lane() {
    let packet = seeded_m5_transient_inspect_certification_packet_editor_hover_only_blocked();
    let row = packet.row(M5InspectContext::Editor).unwrap();
    assert_eq!(row.derived_status, TransientInspectCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TransientInspectCertificationFinding::InformationHoverOnly { .. }
    )));
    assert!(validate_m5_transient_inspect_certification_packet(&packet).is_err());
}

#[test]
fn stale_reads_as_live_blocks_the_data_grid_lane() {
    let packet = seeded_m5_transient_inspect_certification_packet_data_stale_reads_live_blocked();
    let row = packet.row(M5InspectContext::DataGrid).unwrap();
    assert_eq!(row.derived_status, TransientInspectCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TransientInspectCertificationFinding::StalePreviewMistakenForLive { .. }
    )));
    assert!(validate_m5_transient_inspect_certification_packet(&packet).is_err());
}

#[test]
fn sole_instruction_tooltip_blocks_the_operator_lane() {
    let packet =
        seeded_m5_transient_inspect_certification_packet_operator_tooltip_sole_instruction_blocked(
        );
    let row = packet.row(M5InspectContext::Operator).unwrap();
    assert_eq!(row.derived_status, TransientInspectCertificationStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        TransientInspectCertificationFinding::TooltipSoleCriticalInstruction { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
        )));
    assert!(validate_m5_transient_inspect_certification_packet(&packet).is_err());
}

#[test]
fn every_row_pulls_transient_bindings_from_the_matrix() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    for row in &packet.rows {
        assert!(!row.certified_representation_classes.is_empty());
        assert!(!row.certified_promotion_states.is_empty());
        assert!(!row.source_freshness_labels.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(!row.consumer_surfaces.is_empty());
        assert!(!row.applicable_downgrade_triggers.is_empty());
        assert_eq!(row.shell_zone_slot, M5ShellZoneSlot::TransientOverlay);
        assert_eq!(
            row.driven_primitive_families,
            vec![
                M5ShellPrimitiveFamily::Tooltip,
                M5ShellPrimitiveFamily::Hovercard,
                M5ShellPrimitiveFamily::PeekPanel,
                M5ShellPrimitiveFamily::PinnedPreviewPromotion,
            ]
        );
    }
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.shell_automation_refs.is_empty());

    let profiler = dashboard
        .rows
        .iter()
        .find(|row| row.context == M5InspectContext::Profiler)
        .unwrap();
    assert_eq!(profiler.status, TransientInspectCertificationStatus::Yellow);
    assert!(profiler.has_active_waiver);
    assert!(profiler.cause_tokens.contains(
        &M5ShellPrimitiveDowngradeTrigger::PromotionDroppedTruth
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    let export = TransientInspectCertificationSupportExport::from_packet(
        M5_TRANSIENT_INSPECT_CERTIFICATION_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.context.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_context() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for context in M5InspectContext::ALL {
        assert!(
            markdown.contains(context.label()),
            "markdown omits {}",
            context.as_str()
        );
        assert!(
            csv.contains(context.as_str()),
            "csv omits {}",
            context.as_str()
        );
    }
    assert!(markdown.contains("m5_transient_inspect_certification_fixtures"));
    assert!(markdown.contains("waiver:profiler-reduced-promotion-path:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_transient_inspect_certification_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("secret"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = TransientInspectCertificationWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        context: M5InspectContext::Profiler,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
