//! Inline unit tests for the M5 shell-primitive release proof.

use super::*;

#[test]
fn seeded_packet_covers_every_family_and_is_clean() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    validate_m5_shell_primitive_release_proof_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), M5ShellPrimitiveFamily::ALL.len());
    for family in M5ShellPrimitiveFamily::ALL {
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
fn seeded_packet_has_six_green_and_four_yellow_rows() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    assert_eq!(packet.green_row_count, 6);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for family in [
        M5ShellPrimitiveFamily::Hovercard,
        M5ShellPrimitiveFamily::PinnedPreviewPromotion,
        M5ShellPrimitiveFamily::PaneResizePreset,
        M5ShellPrimitiveFamily::ProgressIndicator,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            ShellPrimitiveReleaseStatus::Yellow,
            "{} should auto-narrow to yellow",
            family.as_str()
        );
    }
    for family in [
        M5ShellPrimitiveFamily::StatusBarItem,
        M5ShellPrimitiveFamily::StatusOverflowMenu,
        M5ShellPrimitiveFamily::Tooltip,
        M5ShellPrimitiveFamily::PeekPanel,
        M5ShellPrimitiveFamily::SplitterHandle,
        M5ShellPrimitiveFamily::DurableJobRow,
    ] {
        assert_eq!(
            packet.row(family).unwrap().derived_status,
            ShellPrimitiveReleaseStatus::Green,
            "{} should stay green",
            family.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.primitive_family.as_str()
        );
        assert_eq!(row.certification_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_keeps_complete_routes_labels_profiles_and_invariant() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    for row in &packet.rows {
        assert!(
            row.accessibility_routes_complete(),
            "row {} does not certify every accessibility route",
            row.primitive_family.as_str()
        );
        assert!(
            row.required_labels_complete(),
            "row {} does not certify every mandatory label",
            row.primitive_family.as_str()
        );
        assert!(
            row.profiles_complete(),
            "row {} does not certify every rendering profile",
            row.primitive_family.as_str()
        );
        assert!(
            row.never_hover_spinner_or_pointer_only,
            "row {} keeps a critical truth hover-/spinner-/pointer-only",
            row.primitive_family.as_str()
        );
    }
}

#[test]
fn rows_cover_all_four_truth_pillars() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    for pillar in M5ShellPrimitiveTruthPillar::ALL {
        assert!(
            packet.rows.iter().any(|row| row.truth_pillar == pillar),
            "no row covers the {} pillar",
            pillar.as_str()
        );
    }
    assert_eq!(
        packet.covered_truth_pillars,
        vec![
            "ambient_instrumentation".to_owned(),
            "durable_progress".to_owned(),
            "pane_control".to_owned(),
            "transient_inspect".to_owned(),
        ]
    );
}

#[test]
fn every_row_pulls_primitive_bindings_from_the_matrix() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    for row in &packet.rows {
        assert!(!row.accessibility_routes.is_empty());
        assert!(!row.required_labels.is_empty());
        assert!(!row.consumer_surfaces.is_empty());
        assert!(!row.applicable_downgrade_triggers.is_empty());
        assert!(!row.responsive_classes.is_empty());
        assert!(!row.window_classes.is_empty());
        assert!(!row.surface_families.is_empty());
        assert_eq!(
            row.certified_profiles.len(),
            M5ShellReleaseProfile::ALL.len()
        );
        assert!(row
            .applicable_downgrade_triggers
            .contains(&M5ShellPrimitiveDowngradeTrigger::ProofStale));
        assert_eq!(
            row.truth_pillar,
            M5ShellPrimitiveTruthPillar::from_family(row.primitive_family)
        );
    }
    // The ambient primitives project status-item classes and overflow behaviors.
    let status_bar = packet.row(M5ShellPrimitiveFamily::StatusBarItem).unwrap();
    assert!(!status_bar.certified_status_item_classes.is_empty());
    assert!(!status_bar.certified_overflow_behaviors.is_empty());
    // The transient-inspect primitives project representation classes.
    let hovercard = packet.row(M5ShellPrimitiveFamily::Hovercard).unwrap();
    assert!(!hovercard.certified_representation_classes.is_empty());
    // The pane controls project pane-resize states and no freshness.
    let splitter = packet.row(M5ShellPrimitiveFamily::SplitterHandle).unwrap();
    assert!(!splitter.certified_pane_resize_states.is_empty());
    assert!(splitter.certified_source_freshness_labels.is_empty());
    // The progress primitives project progress states.
    let job_row = packet.row(M5ShellPrimitiveFamily::DurableJobRow).unwrap();
    assert!(!job_row.certified_progress_states.is_empty());
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, ShellPrimitiveReleaseStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.primitive_family.as_str()
            );
        }
    }
}

#[test]
fn pane_resize_reduced_reach_carries_an_active_waiver() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    let preset = packet
        .row(M5ShellPrimitiveFamily::PaneResizePreset)
        .unwrap();
    assert!(matches!(
        preset.interaction_reach,
        InteractionReachState::DisclosedReducedReachOrResize
    ));
    assert!(preset.requires_waiver());
    assert!(preset.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn reduced_truth_scope_narrows_but_does_not_block() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    let progress = packet
        .row(M5ShellPrimitiveFamily::ProgressIndicator)
        .unwrap();
    assert!(matches!(
        progress.primitive_truth,
        PrimitiveTruthState::DisclosedReducedTruthScope
    ));
    assert_eq!(progress.derived_status, ShellPrimitiveReleaseStatus::Yellow);
    assert!(progress
        .certification_causes
        .iter()
        .any(|cause| cause.disclosed
            && matches!(
                cause.trigger,
                M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState
            )));
}

#[test]
fn collapsed_truth_blocks_the_status_bar_item() {
    let packet =
        seeded_m5_shell_primitive_release_proof_packet_status_bar_truth_collapsed_blocked();
    let row = packet.row(M5ShellPrimitiveFamily::StatusBarItem).unwrap();
    assert_eq!(row.derived_status, ShellPrimitiveReleaseStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ShellPrimitiveReleaseFinding::PrimitiveTruthCollapsed { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState
        )));
    assert!(validate_m5_shell_primitive_release_proof_packet(&packet).is_err());
}

#[test]
fn hidden_source_freshness_blocks_the_hovercard() {
    let packet =
        seeded_m5_shell_primitive_release_proof_packet_hovercard_source_freshness_hidden_blocked();
    let row = packet.row(M5ShellPrimitiveFamily::Hovercard).unwrap();
    assert_eq!(row.derived_status, ShellPrimitiveReleaseStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ShellPrimitiveReleaseFinding::SourceOrFreshnessHidden { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::SourceFreshnessHidden
        )));
    assert!(validate_m5_shell_primitive_release_proof_packet(&packet).is_err());
}

#[test]
fn pointer_only_resize_blocks_the_splitter_handle() {
    let packet =
        seeded_m5_shell_primitive_release_proof_packet_splitter_pointer_only_resize_blocked();
    let row = packet.row(M5ShellPrimitiveFamily::SplitterHandle).unwrap();
    assert_eq!(row.derived_status, ShellPrimitiveReleaseStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ShellPrimitiveReleaseFinding::InteractionPointerOrHoverOnly { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
        )));
    assert!(validate_m5_shell_primitive_release_proof_packet(&packet).is_err());
}

#[test]
fn stale_export_blocks_the_durable_job_row() {
    let packet =
        seeded_m5_shell_primitive_release_proof_packet_job_row_exported_proof_stale_blocked();
    let row = packet.row(M5ShellPrimitiveFamily::DurableJobRow).unwrap();
    assert_eq!(row.derived_status, ShellPrimitiveReleaseStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ShellPrimitiveReleaseFinding::ExportedProofStale { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(cause.trigger, M5ShellPrimitiveDowngradeTrigger::ProofStale)));
    assert!(validate_m5_shell_primitive_release_proof_packet(&packet).is_err());
}

#[test]
fn hover_spinner_only_invariant_blocks_the_progress_indicator() {
    let packet =
        seeded_m5_shell_primitive_release_proof_packet_progress_hover_spinner_only_blocked();
    let row = packet
        .row(M5ShellPrimitiveFamily::ProgressIndicator)
        .unwrap();
    assert_eq!(row.derived_status, ShellPrimitiveReleaseStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        ShellPrimitiveReleaseFinding::CriticalTruthHoverSpinnerOrPointerOnly { .. }
    )));
    assert!(row.certification_causes.iter().any(|cause| !cause.disclosed
        && matches!(
            cause.trigger,
            M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
        )));
    assert!(validate_m5_shell_primitive_release_proof_packet(&packet).is_err());
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.shell_automation_refs.is_empty());

    let preset = dashboard
        .rows
        .iter()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::PaneResizePreset)
        .unwrap();
    assert_eq!(preset.status, ShellPrimitiveReleaseStatus::Yellow);
    assert!(preset.has_active_waiver);
    assert!(preset.cause_tokens.contains(
        &M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth
            .as_str()
            .to_owned()
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    let export = ShellPrimitiveReleaseSupportExport::from_packet(
        M5_SHELL_PRIMITIVE_RELEASE_PROOF_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export
            .case_ids
            .contains(&row.primitive_family.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_family() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for family in M5ShellPrimitiveFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv omits {}",
            family.as_str()
        );
    }
    assert!(markdown.contains("m5_shell_primitive_release_proof_fixtures"));
    assert!(markdown.contains("waiver:pane-resize-reduced-reach:0001"));
    for profile in M5ShellReleaseProfile::ALL {
        assert!(
            markdown.contains(profile.as_str()),
            "markdown omits profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_shell_primitive_release_proof_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("secret"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = ShellPrimitiveReleaseWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        primitive_family: M5ShellPrimitiveFamily::PaneResizePreset,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
