use super::*;

const PACKET_ID: &str = RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_PACKET_ID;

fn packet() -> RunComparisonTableCompareGuardBannerControlsPacket {
    seeded_run_comparison_table_compare_guard_banner_controls()
}

#[test]
fn seed_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(
        packet.record_kind,
        RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_VERSION
    );
}

#[test]
fn fairness_is_derived_not_asserted() {
    use ComparisonFairnessClass as Fair;
    use M5ComparabilityState as State;

    // Comparable → fair baseline.
    let d = resolve_run_comparison(State::Comparable);
    assert_eq!(d.fairness_class, Fair::FairBaseline);
    assert!(d.is_fair_baseline);

    // Comparable with caveats → caveated, needs caveat note.
    let d = resolve_run_comparison(State::ComparableWithCaveats);
    assert_eq!(d.fairness_class, Fair::CaveatedBaseline);
    assert!(!d.is_fair_baseline);
    assert!(d.needs_caveat_note);

    // Not comparable / confounded → unfair.
    for state in [State::NotComparable, State::Confounded] {
        let d = resolve_run_comparison(state);
        assert_eq!(d.fairness_class, Fair::UnfairBaseline);
        assert!(!d.is_fair_baseline);
    }
    assert!(resolve_run_comparison(State::NotComparable).needs_not_comparable_note);
    assert!(resolve_run_comparison(State::Confounded).needs_confounder_note);

    // Insufficient overlap / unknown → unproven.
    for state in [State::InsufficientOverlap, State::UnknownComparability] {
        let d = resolve_run_comparison(state);
        assert_eq!(d.fairness_class, Fair::UnprovenBaseline);
        assert!(!d.is_fair_baseline);
    }
    assert!(resolve_run_comparison(State::InsufficientOverlap).needs_insufficient_overlap_note);
    assert!(resolve_run_comparison(State::UnknownComparability).needs_unknown_comparability_note);
}

#[test]
fn guard_class_is_derived_not_asserted() {
    use GuardComparabilityClass as Guard;
    use M5CompareGuardState as State;

    // Permitted.
    let d = resolve_compare_guard(State::ComparisonPermitted);
    assert_eq!(d.guard_class, Guard::ComparablePermitted);
    assert!(d.permits_fair_comparison);

    // Caveated / acknowledged → partially comparable.
    for state in [State::ComparisonCaveated, State::GuardAcknowledged] {
        let d = resolve_compare_guard(state);
        assert_eq!(d.guard_class, Guard::PartiallyComparable);
        assert!(!d.permits_fair_comparison);
        assert!(d.needs_partial_comparability_note);
    }

    // Overridden → overridden, needs override warning.
    let d = resolve_compare_guard(State::GuardOverriddenByChoice);
    assert_eq!(d.guard_class, Guard::OverriddenComparison);
    assert!(!d.permits_fair_comparison);
    assert!(d.needs_override_warning);

    // Blocked → blocked, needs blocked note.
    let d = resolve_compare_guard(State::ComparisonBlocked);
    assert_eq!(d.guard_class, Guard::NotComparableBlocked);
    assert!(d.is_blocked);
    assert!(d.needs_blocked_note);

    // Unavailable → unavailable, needs unavailable note.
    let d = resolve_compare_guard(State::GuardUnavailable);
    assert_eq!(d.guard_class, Guard::GuardUnavailable);
    assert!(!d.permits_fair_comparison);
    assert!(d.needs_guard_unavailable_note);
}

#[test]
fn comparison_coverage_is_complete() {
    let packet = packet();
    let classes: std::collections::BTreeSet<_> = packet
        .comparison_tables
        .iter()
        .map(|t| t.fairness_disclosure().fairness_class)
        .collect();
    for class in ComparisonFairnessClass::ALL {
        assert!(classes.contains(&class), "missing fairness class {class:?}");
    }
    let axes: std::collections::BTreeSet<_> = packet
        .comparison_tables
        .iter()
        .map(|t| t.comparison_axis)
        .collect();
    for axis in M5ComparisonAxisClass::ALL {
        assert!(axes.contains(&axis), "missing comparison axis {axis:?}");
    }
    let states: std::collections::BTreeSet<_> = packet
        .comparison_tables
        .iter()
        .map(|t| t.comparability_state)
        .collect();
    for state in M5ComparabilityState::ALL {
        assert!(
            states.contains(&state),
            "missing comparability state {state:?}"
        );
    }
}

#[test]
fn guard_coverage_is_complete() {
    let packet = packet();
    let classes: std::collections::BTreeSet<_> = packet
        .guard_banners
        .iter()
        .map(|b| b.guard_disclosure().guard_class)
        .collect();
    for class in GuardComparabilityClass::ALL {
        assert!(classes.contains(&class), "missing guard class {class:?}");
    }
    let reasons: std::collections::BTreeSet<_> = packet
        .guard_banners
        .iter()
        .map(|b| b.guard_reason)
        .collect();
    for reason in M5CompareGuardReason::ALL {
        assert!(reasons.contains(&reason), "missing guard reason {reason:?}");
    }
    let states: std::collections::BTreeSet<_> =
        packet.guard_banners.iter().map(|b| b.guard_state).collect();
    for state in M5CompareGuardState::ALL {
        assert!(states.contains(&state), "missing guard state {state:?}");
    }
}

#[test]
fn every_table_names_baseline_and_candidate() {
    for table in packet().comparison_tables {
        assert!(
            !table.baseline_run_id.trim().is_empty() && !table.candidate_run_id.trim().is_empty(),
            "table {} has an anonymous run",
            table.table_id
        );
    }
}

#[test]
fn trust_labels_are_covered() {
    let packet = packet();
    let mut labels = std::collections::BTreeSet::new();
    for table in &packet.comparison_tables {
        labels.extend(table.dispositions.iter().copied());
    }
    for banner in &packet.guard_banners {
        labels.extend(banner.dispositions.iter().copied());
    }
    for required in [
        M5ExperimentDisposition::Reproducible,
        M5ExperimentDisposition::LikelyReproducible,
        M5ExperimentDisposition::NeedsRerun,
        M5ExperimentDisposition::ContextIncomplete,
    ] {
        assert!(
            labels.contains(&required),
            "missing trust label {required:?}"
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::MissingSourceContracts));
}

#[test]
fn empty_comparison_tables_fails() {
    let mut packet = packet();
    packet.comparison_tables.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::ComparisonTablesMissing));
}

#[test]
fn empty_guard_banners_fails() {
    let mut packet = packet();
    packet.guard_banners.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::GuardBannersMissing));
}

#[test]
fn comparison_table_wrong_component_class_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].component = M5ExperimentComponentFamily::CompareGuardBanner;
    assert!(packet.validate().contains(
        &RunComparisonTableCompareGuardBannerViolation::ComparisonTableWrongComponentClass
    ));
}

#[test]
fn guard_banner_wrong_component_class_fails() {
    let mut packet = packet();
    packet.guard_banners[0].component = M5ExperimentComponentFamily::RunComparisonTable;
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::GuardBannerWrongComponentClass));
}

#[test]
fn unfair_comparison_claiming_fair_fails() {
    let mut packet = packet();
    let table = packet
        .comparison_tables
        .iter_mut()
        .find(|t| t.fairness_class == ComparisonFairnessClass::UnfairBaseline)
        .expect("unfair comparison present");
    table.claims_fair_baseline = true;
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::FairnessMisrepresented));
}

#[test]
fn blocked_guard_claiming_permitted_fails() {
    let mut packet = packet();
    let banner = packet
        .guard_banners
        .iter_mut()
        .find(|b| b.guard_class == GuardComparabilityClass::NotComparableBlocked)
        .expect("blocked guard present");
    banner.claims_permits_fair_comparison = true;
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::GuardClassMisrepresented));
}

#[test]
fn table_without_candidate_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].candidate_run_id.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::BaselineOrCandidateMissing));
}

#[test]
fn table_without_difference_factor_fails() {
    let mut packet = packet();
    packet.comparison_tables[0]
        .environment_difference_note
        .clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::DifferenceFactorsMissing));
}

#[test]
fn table_without_delta_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].delta_note.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::MetricOrDeltaMissing));
}

#[test]
fn missing_confounder_note_fails() {
    let mut packet = packet();
    let table = packet
        .comparison_tables
        .iter_mut()
        .find(|t| t.comparability_state == M5ComparabilityState::Confounded)
        .expect("confounded comparison present");
    table.confounder_note.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::ConfounderNoteMissing));
}

#[test]
fn missing_not_comparable_note_fails() {
    let mut packet = packet();
    let table = packet
        .comparison_tables
        .iter_mut()
        .find(|t| t.comparability_state == M5ComparabilityState::NotComparable)
        .expect("not-comparable comparison present");
    table.not_comparable_note.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::NotComparableNoteMissing));
}

#[test]
fn missing_blocked_note_fails() {
    let mut packet = packet();
    let banner = packet
        .guard_banners
        .iter_mut()
        .find(|b| b.guard_state == M5CompareGuardState::ComparisonBlocked)
        .expect("blocked guard present");
    banner.blocked_note.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::BlockedNoteMissing));
}

#[test]
fn missing_override_warning_fails() {
    let mut packet = packet();
    let banner = packet
        .guard_banners
        .iter_mut()
        .find(|b| b.guard_state == M5CompareGuardState::GuardOverriddenByChoice)
        .expect("overridden guard present");
    banner.override_warning.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::OverrideWarningMissing));
}

#[test]
fn missing_changed_factors_note_fails() {
    let mut packet = packet();
    packet.guard_banners[0].changed_factors_note.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::ChangedFactorsNoteMissing));
}

#[test]
fn missing_redaction_note_fails() {
    let mut packet = packet();
    packet.guard_banners[0].redaction_note.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::GuardRedactionNoteMissing));
}

#[test]
fn table_missing_export_action_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].table_actions = vec![RunComparisonAction::OpenBaselineRun];
    assert!(packet.validate().contains(
        &RunComparisonTableCompareGuardBannerViolation::ComparisonTableActionsIncomplete
    ));
}

#[test]
fn guard_banner_missing_review_action_fails() {
    let mut packet = packet();
    packet.guard_banners[0].banner_actions = vec![CompareGuardAction::OpenFullLineage];
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::GuardBannerActionsIncomplete));
}

#[test]
fn trust_label_coverage_missing_fails() {
    let mut packet = packet();
    // Strip every reproducibility trust label so the coverage check trips.
    for table in &mut packet.comparison_tables {
        table
            .dispositions
            .retain(|d| matches!(d, M5ExperimentDisposition::LocalRun));
        if table.dispositions.is_empty() {
            table.dispositions.push(M5ExperimentDisposition::LocalRun);
        }
    }
    for banner in &mut packet.guard_banners {
        banner
            .dispositions
            .retain(|d| matches!(d, M5ExperimentDisposition::LocalRun));
        if banner.dispositions.is_empty() {
            banner.dispositions.push(M5ExperimentDisposition::LocalRun);
        }
    }
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::TrustLabelCoverageMissing));
}

#[test]
fn deep_link_action_without_target_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].deep_link_kind = DeepLinkKind::NoDeepLink;
    packet.comparison_tables[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::DeepLinkUnresolved));
}

#[test]
fn resolvable_deep_link_without_ref_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::DeepLinkRefMissing));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = packet();
    packet.guard_banners[0].context_note.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::ContextNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::DispositionsMissing));
}

#[test]
fn table_masking_provenance_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].masks_provenance_or_sensitivity_state = true;
    assert!(packet.validate().contains(
        &RunComparisonTableCompareGuardBannerViolation::ProvenanceOrSensitivityStateMasked
    ));
}

#[test]
fn table_hiding_identity_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].hides_baseline_or_candidate_identity = true;
    assert!(packet.validate().contains(
        &RunComparisonTableCompareGuardBannerViolation::BaselineOrCandidateIdentityHidden
    ));
}

#[test]
fn table_hiding_difference_factors_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].hides_difference_factors_beside_delta = true;
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::DifferenceFactorsHidden));
}

#[test]
fn banner_implying_apples_to_apples_fails() {
    let mut packet = packet();
    packet.guard_banners[0].implies_apples_to_apples_without_parity = true;
    assert!(packet.validate().contains(
        &RunComparisonTableCompareGuardBannerViolation::ApplesToApplesImpliedWithoutParity
    ));
}

#[test]
fn banner_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.guard_banners[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::AlternateStateLabelInvented));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].required_labels = vec![M5ExperimentRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.guard_banners[0].accessibility_routes =
        vec![M5ExperimentAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::AccessibilityRouteMissing));
}

#[test]
fn compare_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .compare_review
        .apples_to_apples_never_implied_without_parity = false;
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::CompareReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .difference_factors_visible_before_trust = false;
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.comparison_tables[0].deep_link_ref = "see https://internal.example/compare".to_owned();
    assert!(packet
        .validate()
        .contains(&RunComparisonTableCompareGuardBannerViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_components() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Run comparison tables"));
    assert!(summary.contains("## Compare guard banners"));
    assert!(summary.contains("fair baseline"));
    assert!(summary.contains("fair comparison"));
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 comparison tables + 6 guard banners
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("run_comparison_table"));
    assert!(csv.contains("compare_guard_banner"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_run_comparison_table_compare_guard_banner_export()
        .expect("checked run comparison compare guard export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-run-comparison-table-compare-guard-banner-controls/comparison_table_not_comparable.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-run-comparison-table-compare-guard-banner-controls/compare_guard_banner_blocked.json"
        )),
    ] {
        let packet: RunComparisonTableCompareGuardBannerControlsPacket =
            serde_json::from_str(raw)
                .expect("fixture parses as run comparison compare guard packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn scenario_fixtures_stay_valid_and_covered() {
    for packet in [
        seeded_run_comparison_table_compare_guard_banner_controls_comparison_table_not_comparable(),
        seeded_run_comparison_table_compare_guard_banner_controls_compare_guard_banner_blocked(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
