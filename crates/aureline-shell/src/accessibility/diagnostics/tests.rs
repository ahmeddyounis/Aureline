use super::*;

#[test]
fn seeded_report_validates() {
    let packet = seeded_m5_dynamic_a11y_diagnostics_report();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_REPORT_PACKET_ID
    );
}

#[test]
fn seeded_report_covers_every_surface_family() {
    let packet = seeded_m5_dynamic_a11y_diagnostics_report();
    let present: std::collections::BTreeSet<_> =
        packet.surfaces.iter().map(|s| s.surface_family).collect();
    for family in M5SurfaceFamily::ALL {
        assert!(
            present.contains(&family),
            "missing surface family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_surface_runs_the_full_diagnostic_battery() {
    let packet = seeded_m5_dynamic_a11y_diagnostics_report();
    for surface in &packet.surfaces {
        let classes: std::collections::BTreeSet<_> =
            surface.checks.iter().map(|c| c.class).collect();
        assert_eq!(
            classes.len(),
            M5AtDiagnosticClass::ALL.len(),
            "surface {} does not run one check per class",
            surface.surface_id
        );
        // The focus class always exports the focus-contract disposition; no other does.
        for check in &surface.checks {
            let is_focus = check.class == M5AtDiagnosticClass::FocusReturnFailure;
            assert_eq!(
                is_focus,
                check.focus_return_disposition.is_some(),
                "focus disposition presence wrong on {} / {}",
                surface.surface_id,
                check.class.as_str()
            );
        }
    }
}

#[test]
fn every_diagnostic_class_is_exercised() {
    let packet = seeded_m5_dynamic_a11y_diagnostics_report();
    let present: std::collections::BTreeSet<_> = packet
        .surfaces
        .iter()
        .flat_map(|s| s.checks.iter().map(|c| c.class))
        .collect();
    for class in M5AtDiagnosticClass::ALL {
        assert!(
            present.contains(&class),
            "diagnostic class {} never exercised",
            class.as_str()
        );
    }
}

#[test]
fn green_report_passes_the_release_gate() {
    let packet = seeded_m5_dynamic_a11y_diagnostics_report();
    assert!(!packet.blocks_release());
    assert!(packet.blocked_surface_ids().is_empty());
    for surface in &packet.surfaces {
        assert_eq!(surface.gate.decision, M5ReleaseGateDecision::Pass);
        assert!(surface.gate.blocking_finding_classes.is_empty());
        assert!(surface.qualification.is_stable());
        assert!(!surface.current_degraded_state.is_degraded);
    }
}

#[test]
fn missing_surface_family_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet
        .surfaces
        .retain(|s| s.surface_family != M5SurfaceFamily::OverlaySheet);
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::RequiredSurfaceFamilyMissing));
}

#[test]
fn shared_vocabulary_drift_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.shared_vocabulary_set.bridge_states.pop();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::VocabularySetDrift));
}

#[test]
fn diagnostics_vocabulary_drift_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.diagnostics_vocabulary_set.diagnostic_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::VocabularySetDrift));
}

#[test]
fn duplicate_surface_id_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    let mut clone = packet.surfaces[0].clone();
    // Keep the family distinct so the duplicate-id check is what fires.
    clone.surface_family = M5SurfaceFamily::OverlaySheet;
    packet.surfaces.push(clone);
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::DuplicateSurfaceId));
}

#[test]
fn checks_not_one_per_class_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.surfaces[0].checks.pop();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::DiagnosticChecksNotOnePerClass));
}

#[test]
fn focus_disposition_mismatch_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    // Attach a focus disposition to a non-focus check.
    let check = packet.surfaces[0]
        .checks
        .iter_mut()
        .find(|c| c.class == M5AtDiagnosticClass::BridgeHealth)
        .expect("bridge health check present");
    check.focus_return_disposition = Some(A11yFocusReturnDisposition::ReturnedExact);
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::FocusDispositionMismatch));
}

#[test]
fn bridge_probe_inconsistent_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.surfaces[0]
        .bridge_probe
        .semantic_node_coverage
        .missing_nodes += 3;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::BridgeProbeInconsistent));
}

#[test]
fn announcement_budget_inconsistent_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    // Observed traffic over budget but still flagged within-budget.
    packet.surfaces[0]
        .announcement_budget
        .observed_announcements_in_window = 100;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::AnnouncementBudgetInconsistent));
}

#[test]
fn budget_outcome_mismatch_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    // Over budget and honestly flagged, but the announcement checks still claim pass.
    let budget = &mut packet.surfaces[0].announcement_budget;
    budget.observed_announcements_in_window = 100;
    budget.observed_min_interval_ms = 10;
    budget.within_budget = false;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::BudgetOutcomeMismatch));
}

#[test]
fn visual_conformance_mismatch_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.surfaces[0].visual_conformance.high_zoom = M5DiagnosticOutcome::Regressed;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::VisualConformanceMismatch));
}

#[test]
fn degraded_state_inconsistent_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.surfaces[0].current_degraded_state.is_degraded = true;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::DegradedStateInconsistent));
}

#[test]
fn gate_decision_inconsistent_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    // A blocking regression with the gate left passing.
    let check = packet.surfaces[0]
        .checks
        .iter_mut()
        .find(|c| c.class == M5AtDiagnosticClass::LabelOrRoleDrift)
        .expect("label/role drift check present");
    check.outcome = M5DiagnosticOutcome::Regressed;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::GateDecisionInconsistent));
}

#[test]
fn narrowing_inconsistent_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    // An auto-narrowed check while the surface still claims Stable.
    let check = packet.surfaces[0]
        .checks
        .iter_mut()
        .find(|c| c.class == M5AtDiagnosticClass::BridgeHealth)
        .expect("bridge health check present");
    check.outcome = M5DiagnosticOutcome::AutoNarrowed;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::NarrowingInconsistent));
}

#[test]
fn message_id_prefix_missing_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.surfaces[0].checks[0].detail_message_id = "bridge_health".to_owned();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::MessageIdPrefixMissing));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.surfaces[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.surfaces[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.surfaces[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::ConsumerSurfacesMissing));
}

#[test]
fn non_reopenable_durable_fallback_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.surfaces[0].durable_fallback.reopenable = false;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::DurableFallbackMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::MissingSourceContracts));
}

#[test]
fn conformance_review_incomplete_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet
        .conformance_review
        .per_surface_diagnostics_not_replaced_by_aggregate_dashboard = false;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::ConformanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet
        .consumer_projection
        .release_public_truth_gates_on_diagnostics = false;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet
        .release_posture
        .stable_promotion_blocks_without_mapped_proof = false;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::ReleasePostureIncomplete));
}

#[test]
fn release_gate_aggregate_inconsistent_fails() {
    let mut packet = seeded_m5_dynamic_a11y_diagnostics_report();
    packet.release_gate.blocks_release = true;
    assert!(packet
        .validate()
        .contains(&M5DiagnosticsViolation::ReleaseGateAggregateInconsistent));
}

#[test]
fn markdown_summary_lists_surfaces_and_gate() {
    let packet = seeded_m5_dynamic_a11y_diagnostics_report();
    let summary = packet.render_markdown_summary();
    assert!(summary.contains("Release gate:"));
    for surface in &packet.surfaces {
        assert!(
            summary.contains(&surface.surface_id),
            "summary missing surface {}",
            surface.surface_id
        );
        assert!(
            summary.contains(&surface.object_identity_ref),
            "summary missing object identity {}",
            surface.object_identity_ref
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_dynamic_a11y_diagnostics_export()
        .expect("checked M5 diagnostics export validates");
    assert_eq!(
        packet.packet_id,
        M5_DYNAMIC_A11Y_DIAGNOSTICS_REPORT_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_dynamic_a11y_diagnostics_export()
        .expect("checked M5 diagnostics export validates");
    assert_eq!(
        from_disk,
        seeded_m5_dynamic_a11y_diagnostics_report(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn drills_validate_and_fail_release_for_blocking_regressions() {
    // The auto-narrowing drill keeps shipping: it narrows but does not block.
    let narrowed = seeded_m5_dynamic_a11y_diagnostics_report_bridge_unavailable_narrowed();
    assert!(narrowed.validate().is_empty(), "{:?}", narrowed.validate());
    assert!(!narrowed.blocks_release());
    assert_eq!(narrowed.surfaces.len(), M5SurfaceFamily::ALL.len());
    let editor = narrowed
        .surfaces
        .iter()
        .find(|s| s.surface_family == M5SurfaceFamily::EditorCanvas)
        .expect("editor surface present");
    assert_eq!(
        editor.qualification,
        M5DynamicSurfaceA11yQualificationClass::Beta
    );
    assert_eq!(editor.gate.decision, M5ReleaseGateDecision::Pass);
    assert!(editor.current_degraded_state.is_degraded);

    // The blocking drills fail the release gate for the right reason on the right surface.
    let cases = [
        (
            seeded_m5_dynamic_a11y_diagnostics_report_bridge_regression_blocked(),
            M5SurfaceFamily::TerminalCanvas,
            M5AtDiagnosticClass::BridgeHealth,
        ),
        (
            seeded_m5_dynamic_a11y_diagnostics_report_announcement_spam_blocked(),
            M5SurfaceFamily::DenseCollection,
            M5AtDiagnosticClass::AnnouncementRate,
        ),
        (
            seeded_m5_dynamic_a11y_diagnostics_report_visual_regression_blocked(),
            M5SurfaceFamily::ReviewDiff,
            M5AtDiagnosticClass::HighContrastRegression,
        ),
    ];
    for (packet, family, blocking_class) in cases {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
        assert!(packet.blocks_release());
        assert_eq!(packet.surfaces.len(), M5SurfaceFamily::ALL.len());
        let surface = packet
            .surfaces
            .iter()
            .find(|s| s.surface_family == family)
            .expect("target surface present");
        assert_eq!(surface.gate.decision, M5ReleaseGateDecision::Blocked);
        assert!(
            surface
                .gate
                .blocking_finding_classes
                .contains(&blocking_class),
            "gate for {} missing blocking class {}",
            surface.surface_id,
            blocking_class.as_str()
        );
        assert_eq!(
            packet.blocked_surface_ids(),
            vec![surface.surface_id.as_str()]
        );
    }
}

#[test]
fn advisory_regression_does_not_block_release() {
    let packet = seeded_m5_dynamic_a11y_diagnostics_report_visual_regression_blocked();
    let review = packet
        .surfaces
        .iter()
        .find(|s| s.surface_family == M5SurfaceFamily::ReviewDiff)
        .expect("review surface present");
    // Reduced-motion regressed but is advisory, so it is recorded yet never gates.
    let motion = review
        .check(M5AtDiagnosticClass::ReducedMotionRegression)
        .expect("reduced-motion check present");
    assert_eq!(motion.outcome, M5DiagnosticOutcome::Regressed);
    assert_eq!(motion.severity, M5DiagnosticSeverity::Advisory);
    assert!(!review
        .gate
        .blocking_finding_classes
        .contains(&M5AtDiagnosticClass::ReducedMotionRegression));
}

#[test]
fn checked_drill_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-bridge-and-announcement-drills/bridge_unavailable_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-bridge-and-announcement-drills/bridge_regression_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-bridge-and-announcement-drills/announcement_spam_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-bridge-and-announcement-drills/visual_regression_blocked.json"
        )),
    ] {
        let packet: M5DynamicA11yDiagnosticsPacket =
            serde_json::from_str(raw).expect("fixture parses as diagnostics report packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_dynamic_a11y_diagnostics_report().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
