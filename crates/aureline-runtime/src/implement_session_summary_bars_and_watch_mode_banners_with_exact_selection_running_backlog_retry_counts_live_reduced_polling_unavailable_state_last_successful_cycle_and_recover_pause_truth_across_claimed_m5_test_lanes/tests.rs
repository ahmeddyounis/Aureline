use super::*;

fn executing_watch_session() -> M5SessionSummaryResolutionInput {
    M5SessionSummaryResolutionInput {
        session_mode: M5SessionMode::WatchSession,
        activity_phase: M5SessionActivityPhase::ExecutingTests,
        selection_scope: M5SessionSelectionScope::SelectedSubsetSelection,
        session_outcome: M5TestSessionOutcome::InProgress,
        target_class: M5TestTargetClass::UnitTest,
        environment_lane: M5TestEnvironmentLane::LocalHost,
        attempt_lineage: M5AttemptLineageKind::RetriedFail,
        watch_fidelity: M5WatchFidelityState::Live,
        running_count: 7,
        backlog_count: 12,
        retry_count: 2,
        selection_label: "watch: selected auth + pricing subset".to_owned(),
        session_identity_ref: "session:explorer::watch-auth-pricing".to_owned(),
    }
}

fn live_watch() -> M5WatchBannerResolutionInput {
    M5WatchBannerResolutionInput {
        watch_fidelity: M5WatchFidelityState::Live,
        degrade_reason: None,
        last_successful_cycle: "2026-07-07T00:00:00Z".to_owned(),
        backlog_count: 0,
        watch_label: "watch: local host live".to_owned(),
        watch_identity_ref: "watch:explorer::local-live".to_owned(),
    }
}

// ---- session-summary-bar resolver ---------------------------------------

#[test]
fn executing_session_is_in_progress_reruns_only_when_settled() {
    let resolved = resolve_session_summary_bar(&executing_watch_session()).expect("resolves");
    assert_eq!(
        resolved.session_posture,
        M5SessionSummaryPosture::ExecutingSession
    );
    assert!(resolved.is_in_progress);
    assert!(!resolved.can_rerun);
    assert!(resolved.can_cancel);
    assert!(resolved.has_backlog);
    assert!(resolved.has_retries);
    assert!(!resolved.watch_is_degraded);
    assert!(resolved.needs_attention);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5SessionSummaryAction::RevealSessionDetails,
            M5SessionSummaryAction::CancelRunningSession,
            M5SessionSummaryAction::OpenWatchBanner,
            M5SessionSummaryAction::ExportSession,
        ]
    );
    assert_eq!(
        resolved.session_identity_ref,
        "session:explorer::watch-auth-pricing"
    );
}

#[test]
fn every_activity_phase_has_a_distinct_posture() {
    // The acceptance-criterion axis: discovery, execution, watch backlog, and
    // imported-status refresh never share one loading treatment.
    let cases = [
        (
            M5SessionActivityPhase::DiscoveringTests,
            M5SessionSummaryPosture::DiscoveringSession,
        ),
        (
            M5SessionActivityPhase::ExecutingTests,
            M5SessionSummaryPosture::ExecutingSession,
        ),
        (
            M5SessionActivityPhase::ProcessingWatchBacklog,
            M5SessionSummaryPosture::WatchBacklogSession,
        ),
        (
            M5SessionActivityPhase::RefreshingImportedStatus,
            M5SessionSummaryPosture::ImportedRefreshSession,
        ),
        (
            M5SessionActivityPhase::SettledComplete,
            M5SessionSummaryPosture::SettledSession,
        ),
    ];
    let mut postures = std::collections::BTreeSet::new();
    for (phase, expected) in cases {
        let resolved = resolve_session_summary_bar(&M5SessionSummaryResolutionInput {
            activity_phase: phase,
            ..executing_watch_session()
        })
        .expect("resolves");
        assert_eq!(resolved.session_posture, expected);
        postures.insert(resolved.session_posture);
    }
    assert_eq!(postures.len(), M5SessionSummaryPosture::ALL.len());
}

#[test]
fn settled_session_reruns_and_never_cancels() {
    let resolved = resolve_session_summary_bar(&M5SessionSummaryResolutionInput {
        session_mode: M5SessionMode::RunOnceSession,
        activity_phase: M5SessionActivityPhase::SettledComplete,
        session_outcome: M5TestSessionOutcome::AllPassed,
        backlog_count: 0,
        retry_count: 0,
        ..executing_watch_session()
    })
    .expect("resolves");
    assert_eq!(
        resolved.session_posture,
        M5SessionSummaryPosture::SettledSession
    );
    assert!(!resolved.is_in_progress);
    assert!(resolved.can_rerun);
    assert!(!resolved.can_cancel);
    assert!(resolved
        .available_actions
        .contains(&M5SessionSummaryAction::RerunExactSelection));
    assert!(!resolved
        .available_actions
        .contains(&M5SessionSummaryAction::CancelRunningSession));
    // A non-watch session never links the watch banner.
    assert!(!resolved
        .available_actions
        .contains(&M5SessionSummaryAction::OpenWatchBanner));
}

#[test]
fn degraded_watch_stays_visible_on_the_summary_bar() {
    for fidelity in [
        M5WatchFidelityState::Reduced,
        M5WatchFidelityState::Polling,
        M5WatchFidelityState::Unavailable,
        M5WatchFidelityState::Reconnecting,
    ] {
        let resolved = resolve_session_summary_bar(&M5SessionSummaryResolutionInput {
            watch_fidelity: fidelity,
            ..executing_watch_session()
        })
        .expect("resolves");
        assert!(
            resolved.watch_is_degraded,
            "watch fidelity {} not surfaced as degraded on the bar",
            fidelity.as_str()
        );
    }
}

#[test]
fn session_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_session_summary_bar(&M5SessionSummaryResolutionInput {
            selection_label: "  ".to_owned(),
            ..executing_watch_session()
        }),
        Err(M5SessionSummaryResolutionError::EmptySelectionLabel)
    );
    assert_eq!(
        resolve_session_summary_bar(&M5SessionSummaryResolutionInput {
            session_identity_ref: "".to_owned(),
            ..executing_watch_session()
        }),
        Err(M5SessionSummaryResolutionError::EmptySessionIdentity)
    );
    assert_eq!(
        resolve_session_summary_bar(&M5SessionSummaryResolutionInput {
            session_identity_ref: "session:https://ci.example/run".to_owned(),
            ..executing_watch_session()
        }),
        Err(M5SessionSummaryResolutionError::ForbiddenSessionMaterial)
    );
}

// ---- watch-mode-banner resolver -----------------------------------------

#[test]
fn live_watch_needs_no_attention_pauses_but_never_recovers() {
    let resolved = resolve_watch_mode_banner(&live_watch()).expect("resolves");
    assert_eq!(resolved.watch_posture, M5WatchBannerPosture::LiveWatch);
    assert!(!resolved.is_degraded);
    assert!(!resolved.can_recover);
    assert!(resolved.can_pause);
    assert!(resolved.explains_degradation);
    assert!(resolved.preserves_last_successful_cycle);
    assert!(!resolved.needs_attention);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5WatchBannerAction::RevealWatchDetails,
            M5WatchBannerAction::PauseWatch,
            M5WatchBannerAction::ExportWatchState,
        ]
    );
}

#[test]
fn watch_posture_is_one_to_one_with_fidelity() {
    for fidelity in M5WatchFidelityState::ALL {
        let degrade_reason = if watch_fidelity_is_degraded(fidelity) {
            Some(M5WatchDegradeReason::ResourcePressure)
        } else {
            None
        };
        let resolved = resolve_watch_mode_banner(&M5WatchBannerResolutionInput {
            watch_fidelity: fidelity,
            degrade_reason,
            ..live_watch()
        })
        .expect("resolves");
        assert_eq!(resolved.watch_posture.fidelity(), fidelity);
        assert_eq!(resolved.is_degraded, watch_fidelity_is_degraded(fidelity));
    }
}

#[test]
fn degraded_watch_must_explain_its_reason() {
    // A degraded watch with no reason is rejected — the banner never hides why it degraded.
    assert_eq!(
        resolve_watch_mode_banner(&M5WatchBannerResolutionInput {
            watch_fidelity: M5WatchFidelityState::Reduced,
            degrade_reason: None,
            ..live_watch()
        }),
        Err(M5WatchBannerResolutionError::MissingDegradeReason)
    );
    // With a reason it resolves and offers recover + pause.
    let resolved = resolve_watch_mode_banner(&M5WatchBannerResolutionInput {
        watch_fidelity: M5WatchFidelityState::Reduced,
        degrade_reason: Some(M5WatchDegradeReason::ResourcePressure),
        ..live_watch()
    })
    .expect("resolves");
    assert!(resolved.is_degraded);
    assert!(resolved.explains_degradation);
    assert!(resolved.can_recover);
    assert!(resolved.can_pause);
    assert!(resolved
        .available_actions
        .contains(&M5WatchBannerAction::RecoverWatch));
    assert!(resolved
        .available_actions
        .contains(&M5WatchBannerAction::PauseWatch));
}

#[test]
fn unavailable_watch_recovers_but_cannot_pause() {
    let resolved = resolve_watch_mode_banner(&M5WatchBannerResolutionInput {
        watch_fidelity: M5WatchFidelityState::Unavailable,
        degrade_reason: Some(M5WatchDegradeReason::OfflineHost),
        ..live_watch()
    })
    .expect("resolves");
    assert_eq!(
        resolved.watch_posture,
        M5WatchBannerPosture::UnavailableWatch
    );
    assert!(resolved.can_recover);
    assert!(!resolved.can_pause);
    assert!(!resolved
        .available_actions
        .contains(&M5WatchBannerAction::PauseWatch));
}

#[test]
fn paused_watch_recovers_but_is_not_a_degradation() {
    let resolved = resolve_watch_mode_banner(&M5WatchBannerResolutionInput {
        watch_fidelity: M5WatchFidelityState::Paused,
        degrade_reason: None,
        ..live_watch()
    })
    .expect("resolves");
    assert_eq!(resolved.watch_posture, M5WatchBannerPosture::PausedWatch);
    assert!(!resolved.is_degraded);
    assert!(resolved.can_recover);
    assert!(!resolved.can_pause);
}

#[test]
fn watch_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_watch_mode_banner(&M5WatchBannerResolutionInput {
            last_successful_cycle: "  ".to_owned(),
            ..live_watch()
        }),
        Err(M5WatchBannerResolutionError::EmptyLastSuccessfulCycle)
    );
    assert_eq!(
        resolve_watch_mode_banner(&M5WatchBannerResolutionInput {
            watch_identity_ref: "".to_owned(),
            ..live_watch()
        }),
        Err(M5WatchBannerResolutionError::EmptyWatchIdentity)
    );
    assert_eq!(
        resolve_watch_mode_banner(&M5WatchBannerResolutionInput {
            watch_label: "watch bearer token-lane".to_owned(),
            ..live_watch()
        }),
        Err(M5WatchBannerResolutionError::ForbiddenWatchMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_session_watch_status_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SESSION_WATCH_STATUS_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_session_watch_status_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5SessionWatchConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5SessionWatchConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_session_watch_status_packet();
    for row in &packet.rows {
        for part in M5SessionSummaryAnatomyPart::MANDATORY {
            assert!(row.session_anatomy_parts.contains(&part));
        }
        for part in M5WatchBannerAnatomyPart::MANDATORY {
            assert!(row.watch_anatomy_parts.contains(&part));
        }
        for field in M5SessionSummaryExportField::MANDATORY {
            assert!(row.session_export_fields.contains(&field));
        }
        for field in M5WatchBannerExportField::MANDATORY {
            assert!(row.watch_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TestAccessibilityRoute::KeyboardFocusable));
        assert!(!row.session_examples.is_empty());
        assert!(!row.watch_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_session_watch_status_packet();
    let sessions: Vec<&M5SessionSummaryResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.session_examples.iter())
        .collect();
    let watches: Vec<&M5WatchBannerResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.watch_examples.iter())
        .collect();

    for posture in M5SessionSummaryPosture::ALL {
        assert!(
            sessions
                .iter()
                .any(|c| c.resolved.session_posture == posture),
            "no example exercises session posture {}",
            posture.as_str()
        );
    }
    for action in M5SessionSummaryAction::ALL {
        assert!(
            sessions
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises session action {}",
            action.as_str()
        );
    }
    for posture in M5WatchBannerPosture::ALL {
        assert!(
            watches.iter().any(|c| c.resolved.watch_posture == posture),
            "no example exercises watch posture {}",
            posture.as_str()
        );
    }
    for action in M5WatchBannerAction::ALL {
        assert!(
            watches
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises watch action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_session_watch_status_packet();
    for row in &packet.rows {
        for case in &row.session_examples {
            assert!(
                case.is_self_consistent(),
                "session case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "session case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.watch_examples {
            assert!(
                case.is_self_consistent(),
                "watch case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "watch case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5SessionWatchConsumerSurface::RunPanelStatus);
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.vocabulary_set.watch_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::VocabularySetDrift));
}

#[test]
fn mandatory_session_anatomy_missing_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.rows[0]
        .session_anatomy_parts
        .retain(|p| *p != M5SessionSummaryAnatomyPart::WatchStateCue);
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::MandatorySessionAnatomyMissing));
}

#[test]
fn mandatory_watch_anatomy_missing_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.rows[0]
        .watch_anatomy_parts
        .retain(|p| *p != M5WatchBannerAnatomyPart::DegradeReasonCue);
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::MandatoryWatchAnatomyMissing));
}

#[test]
fn mandatory_session_export_missing_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.rows[0]
        .session_export_fields
        .retain(|f| *f != M5SessionSummaryExportField::WatchFidelity);
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::MandatorySessionExportMissing));
}

#[test]
fn mandatory_watch_export_missing_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.rows[0]
        .watch_export_fields
        .retain(|f| *f != M5WatchBannerExportField::LastSuccessfulCycle);
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::MandatoryWatchExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.rows[0].session_examples[0].resolved.can_rerun = true;
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::ExampleResolutionDrift));
}

#[test]
fn example_missing_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.rows[1].watch_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::ExampleMissing));
}

#[test]
fn activity_phase_coverage_unproven_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    // Replace every session example with a settled one so most postures go uncovered.
    let settled = M5SessionSummaryResolutionCase::resolved(M5SessionSummaryResolutionInput {
        activity_phase: M5SessionActivityPhase::SettledComplete,
        ..executing_watch_session()
    });
    for row in &mut packet.rows {
        row.session_examples = vec![settled.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::ActivityPhaseCoverageUnproven));
}

#[test]
fn watch_fidelity_coverage_unproven_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    for row in &mut packet.rows {
        row.watch_examples = vec![M5WatchBannerResolutionCase::resolved(live_watch())];
    }
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::WatchFidelityCoverageUnproven));
}

#[test]
fn degradation_coverage_unproven_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    // Replace every watch example with a live one so the degraded half fires.
    for row in &mut packet.rows {
        row.watch_examples = vec![M5WatchBannerResolutionCase::resolved(live_watch())];
    }
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::DegradationCoverageUnproven));
}

#[test]
fn retry_backlog_coverage_unproven_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    // Replace every session example with a quiet one so the active half fires.
    let quiet = M5SessionSummaryResolutionCase::resolved(M5SessionSummaryResolutionInput {
        activity_phase: M5SessionActivityPhase::SettledComplete,
        backlog_count: 0,
        retry_count: 0,
        ..executing_watch_session()
    });
    for row in &mut packet.rows {
        row.session_examples = vec![quiet.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::RetryBacklogCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.rows[0].invents_alternate_watch_label = true;
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.governance_review.distinct_activity_never_one_spinner = false;
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet
        .consumer_projection
        .triage_and_support_read_same_watch_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_session_watch_status_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SessionWatchViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_session_watch_status_packet().render_markdown_summary();
    for surface in M5SessionWatchConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_session_watch_status_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5SessionWatchConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5SessionWatchConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_session_watch_status_export()
        .expect("checked M5 session watch status export validates");
    assert_eq!(from_disk.packet_id, M5_SESSION_WATCH_STATUS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_session_watch_status_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_session_watch_status_run_panel_status_preview_narrowed(),
        seeded_m5_session_watch_status_headless_cli_status_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5SessionWatchConsumerSurface::ALL.len());
    }

    let run_panel = seeded_m5_session_watch_status_run_panel_status_preview_narrowed();
    let row = run_panel
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5SessionWatchConsumerSurface::RunPanelStatus)
        .expect("run-panel-status row present");
    assert_eq!(row.qualification, M5TestQualificationClass::Preview);

    let headless = seeded_m5_session_watch_status_headless_cli_status_beta_narrowed();
    let row = headless
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5SessionWatchConsumerSurface::HeadlessCliStatus)
        .expect("headless-cli-status row present");
    assert_eq!(row.qualification, M5TestQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let run_panel: M5SessionWatchStatusPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-session-summary-watch-banner-primitive/run_panel_status_preview_narrowed.json"
    )))
    .expect("run-panel fixture parses");
    assert!(run_panel.validate().is_empty());
    assert_eq!(
        run_panel,
        seeded_m5_session_watch_status_run_panel_status_preview_narrowed()
    );

    let headless: M5SessionWatchStatusPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-session-summary-watch-banner-primitive/headless_cli_status_beta_narrowed.json"
    )))
    .expect("headless-cli fixture parses");
    assert!(headless.validate().is_empty());
    assert_eq!(
        headless,
        seeded_m5_session_watch_status_headless_cli_status_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_session_watch_status_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
