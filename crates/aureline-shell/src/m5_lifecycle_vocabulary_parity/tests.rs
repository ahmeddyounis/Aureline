//! Inline unit tests for the M5 lifecycle-vocabulary parity proof.

use super::*;

#[test]
fn seeded_packet_covers_every_term_and_is_clean() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    validate_m5_lifecycle_vocabulary_parity_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), REQUIRED_STATES.len());
    for state in REQUIRED_STATES {
        assert!(
            packet.row(state).is_some(),
            "missing row for {}",
            state.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_eleven_green_and_four_yellow_rows() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    assert_eq!(packet.green_row_count, 11);
    assert_eq!(packet.yellow_row_count, 4);
    assert_eq!(packet.red_row_count, 0);

    for state in [
        M5LifecycleState::Experimental,
        M5LifecycleState::ReadOnlyDegraded,
        M5LifecycleState::PolicyBlocked,
        M5LifecycleState::Deprecated,
    ] {
        assert_eq!(
            packet.row(state).unwrap().derived_status,
            VocabularyParityStatus::Yellow,
            "{} should auto-narrow to yellow",
            state.as_str()
        );
    }
}

#[test]
fn distinct_states_never_collapse_in_the_seed() {
    // RetestPending, Experimental, PolicyBlocked, and ReadOnlyDegraded must stay semantically
    // distinct rather than reading as a generic failure.
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    for state in [
        M5LifecycleState::RetestPending,
        M5LifecycleState::Experimental,
        M5LifecycleState::PolicyBlocked,
        M5LifecycleState::ReadOnlyDegraded,
    ] {
        let row = packet.row(state).unwrap();
        assert!(
            !matches!(
                row.semantic_distinction,
                SemanticDistinctionState::CollapsedIntoGenericFailure
            ),
            "{} must not collapse into generic failure",
            state.as_str()
        );
        assert!(
            !matches!(
                row.cross_surface_term,
                CrossSurfaceTermState::TermMeaningDriftedAcrossSurfaces
            ),
            "{} must not drift across surfaces",
            state.as_str()
        );
    }
}

#[test]
fn every_row_status_is_the_derived_value() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.state.as_str()
        );
        assert_eq!(row.term_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_certifies_all_declared_consumer_surfaces() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    for row in &packet.rows {
        assert!(
            row.consumer_surfaces_complete(),
            "row {} does not certify all declared consumer surfaces",
            row.state.as_str()
        );
        assert!(row.headless_parity_preserved);
    }
}

#[test]
fn required_consumer_surfaces_are_derived_from_the_matrix() {
    use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix::seeded_m5_lifecycle_matrix;

    let matrix = seeded_m5_lifecycle_matrix();
    let required = required_consumer_surfaces();
    assert!(!required.is_empty());
    // Every required surface is declared on some governed object family.
    for surface in &required {
        assert!(
            matrix
                .object_state_rows
                .iter()
                .any(|row| row.consumer_surfaces.contains(surface)),
            "required surface {} is not declared by the matrix",
            surface.as_str()
        );
    }
    // Every surface any object declares is in the required set.
    for row in &matrix.object_state_rows {
        for surface in &row.consumer_surfaces {
            assert!(
                required.contains(surface),
                "matrix surface {} missing from the required set",
                surface.as_str()
            );
        }
    }
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    let expected: Vec<String> = required.iter().map(|s| s.as_str().to_owned()).collect();
    assert_eq!(packet.required_consumer_surfaces, expected);
}

#[test]
fn admitting_families_are_pulled_from_the_matrix() {
    use crate::freeze_the_m5_lifecycle_state_and_journey_checkpoint_matrix::seeded_m5_lifecycle_matrix;

    let matrix = seeded_m5_lifecycle_matrix();
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    for row in &packet.rows {
        for family in &row.admitting_object_families {
            let matrix_row = matrix
                .object_state_rows
                .iter()
                .find(|r| r.object_family == *family)
                .unwrap();
            assert!(
                matrix_row.admitted_states.contains(&row.state),
                "{} claims family {} that does not admit it",
                row.state.as_str(),
                family.as_str()
            );
        }
    }
}

#[test]
fn narrowed_rows_disclose_a_reason() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, VocabularyParityStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.state.as_str()
            );
        }
    }
}

#[test]
fn disclosed_surface_paraphrase_carries_an_active_waiver() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    let experimental = packet.row(M5LifecycleState::Experimental).unwrap();
    assert!(matches!(
        experimental.cross_surface_term,
        CrossSurfaceTermState::DisclosedSurfaceParaphrase
    ));
    assert!(experimental.requires_waiver());
    assert!(experimental.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn disclosed_partial_export_keeps_policy_blocked_yellow_without_a_waiver() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    let policy = packet.row(M5LifecycleState::PolicyBlocked).unwrap();
    assert!(matches!(
        policy.export_code_parity,
        ExportCodeParityState::DisclosedPartialExport
    ));
    assert_eq!(policy.derived_status, VocabularyParityStatus::Yellow);
    assert!(policy.cross_surface_term.is_full());
    assert!(!policy.requires_waiver());
}

#[test]
fn term_meaning_drift_blocks_reconnecting() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet_reconnecting_term_drift_blocked();
    let row = packet.row(M5LifecycleState::Reconnecting).unwrap();
    assert_eq!(row.derived_status, VocabularyParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.red_row_count >= 1);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, VocabularyParityFinding::TermMeaningDrifted { .. })));
    assert!(row.term_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_lifecycle_vocabulary_parity_packet(&packet).is_err());
}

#[test]
fn generic_collapse_blocks_retest_pending() {
    let packet =
        seeded_m5_lifecycle_vocabulary_parity_packet_retest_pending_generic_collapse_blocked();
    let row = packet.row(M5LifecycleState::RetestPending).unwrap();
    assert_eq!(row.derived_status, VocabularyParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        VocabularyParityFinding::CollapsedIntoGenericFailure { .. }
    )));
    assert!(row.term_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_lifecycle_vocabulary_parity_packet(&packet).is_err());
}

#[test]
fn unexportable_status_code_blocks_policy_blocked() {
    let packet =
        seeded_m5_lifecycle_vocabulary_parity_packet_policy_blocked_status_code_unexportable_blocked();
    let row = packet.row(M5LifecycleState::PolicyBlocked).unwrap();
    assert_eq!(row.derived_status, VocabularyParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        VocabularyParityFinding::StatusCodeUnexportable { .. }
    )));
    assert!(row.term_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StatusCodeUnexportable
    )));
    assert!(validate_m5_lifecycle_vocabulary_parity_packet(&packet).is_err());
}

#[test]
fn stale_copy_blocks_deprecated() {
    let packet =
        seeded_m5_lifecycle_vocabulary_parity_packet_deprecated_stale_copy_overclaims_blocked();
    let row = packet.row(M5LifecycleState::Deprecated).unwrap();
    assert_eq!(row.derived_status, VocabularyParityStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, VocabularyParityFinding::StaleCopyOverclaims { .. })));
    assert!(row
        .term_causes
        .iter()
        .any(|cause| matches!(cause.trigger, M5LifecycleDowngradeTrigger::ProofStale)));
    assert!(validate_m5_lifecycle_vocabulary_parity_packet(&packet).is_err());
}

#[test]
fn headless_parity_loss_blocks_experimental() {
    let packet =
        seeded_m5_lifecycle_vocabulary_parity_packet_experimental_headless_parity_lost_blocked();
    let row = packet.row(M5LifecycleState::Experimental).unwrap();
    assert_eq!(row.derived_status, VocabularyParityStatus::Red);
    assert!(!row.headless_parity_preserved);
    assert!(!packet.report_clean);
    assert!(packet
        .blocking_findings
        .iter()
        .any(|finding| matches!(finding, VocabularyParityFinding::HeadlessParityLost { .. })));
    assert!(row.term_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5LifecycleDowngradeTrigger::StateVocabularyDrift
    )));
    assert!(validate_m5_lifecycle_vocabulary_parity_packet(&packet).is_err());
}

#[test]
fn incomplete_consumer_surface_certification_blocks() {
    // Hand-mutate a green row so it certifies fewer than all declared consumer surfaces — the
    // completeness lint must block it.
    let mut packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.state == M5LifecycleState::Ready)
        .unwrap();
    row.evaluated_consumer_surfaces.pop();
    assert!(!row.consumer_surfaces_complete());
    assert_eq!(row.recompute_status(), VocabularyParityStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        VocabularyParityFinding::ConsumerSurfacesIncomplete { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_and_counts() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert!(!dashboard.lifecycle_automation_refs.is_empty());

    let experimental = dashboard
        .rows
        .iter()
        .find(|row| row.state == M5LifecycleState::Experimental)
        .unwrap();
    assert_eq!(experimental.status, VocabularyParityStatus::Yellow);
    assert!(experimental.has_active_waiver);
    assert!(matches!(
        experimental.cross_surface_term,
        CrossSurfaceTermState::DisclosedSurfaceParaphrase
    ));
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    let export = VocabularyParitySupportExport::from_packet(
        M5_LIFECYCLE_VOCABULARY_PARITY_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.state.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_term() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for state in REQUIRED_STATES {
        assert!(csv.contains(state.as_str()), "csv omits {}", state.as_str());
    }
    assert!(markdown.contains("m5_lifecycle_vocabulary_parity_fixtures"));
    assert!(markdown.contains("waiver:experimental-surface-paraphrase:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_lifecycle_vocabulary_parity_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = VocabularyParityWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        state: M5LifecycleState::Experimental,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
