//! Tests for the M05-913 test-explorer / watch / triage component consumer
//! adoption lane.

use super::*;

fn packet() -> TestConsumerPacket {
    seeded_m5_test_component_consumers_packet()
}

#[test]
fn seeded_packet_validates() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn packet_record_kind_and_schema_version_are_stamped() {
    let p = packet();
    assert_eq!(p.record_kind, TEST_CONSUMER_RECORD_KIND);
    assert_eq!(p.schema_version, TEST_CONSUMER_SCHEMA_VERSION);
}

#[test]
fn all_four_consumer_classes_present() {
    let p = packet();
    assert!(p.summary.day_to_day_editor_present);
    assert!(p.summary.quality_intelligence_present);
    assert!(p.summary.pipeline_imported_present);
    assert!(p.summary.support_export_present);
    assert_eq!(
        p.summary.consumer_class_count,
        SharedTestConsumerClass::ALL.len()
    );
}

#[test]
fn every_frozen_family_is_adopted() {
    let p = packet();
    let families = p.represented_families();
    for family in M5TestExplorerWatchTriageComponentFamily::ALL {
        assert!(families.contains(&family), "missing family {family:?}");
    }
    assert_eq!(
        p.summary.component_family_count,
        M5TestExplorerWatchTriageComponentFamily::ALL.len()
    );
}

#[test]
fn every_consumer_surface_is_adopted() {
    let p = packet();
    let surfaces: BTreeSet<SharedTestConsumerSurface> =
        p.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in SharedTestConsumerSurface::ALL {
        assert!(surfaces.contains(&surface), "missing surface {surface:?}");
    }
}

#[test]
fn at_least_one_family_reused_across_classes() {
    let p = packet();
    assert!(
        p.families_reused_across_classes() >= 1,
        "expected a family adopted by two or more consumer classes"
    );
    assert_eq!(
        p.summary.families_reused_across_classes,
        p.families_reused_across_classes()
    );
}

#[test]
fn all_rows_point_to_canonical_family() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.points_to_canonical_family(),
            "row {} does not point to canonical family",
            row.row_id
        );
    }
    assert!(p.summary.all_rows_point_to_canonical_family);
}

#[test]
fn canonical_refs_match_sibling_primitive_constants() {
    use M5TestExplorerWatchTriageComponentFamily::*;
    // Session-summary bar and watch-mode banner share the 911 status packet.
    assert_eq!(
        family_canonical_packet_ref(SessionSummaryBar),
        family_canonical_packet_ref(WatchModeBanner)
    );
    // Failure-triage, quarantine-review, and environment-matrix share the 912 packet.
    assert_eq!(
        family_canonical_packet_ref(FailureTriagePanel),
        family_canonical_packet_ref(QuarantineReviewSheet)
    );
    assert_eq!(
        family_canonical_packet_ref(FailureTriagePanel),
        family_canonical_packet_ref(EnvironmentMatrixCard)
    );
    assert_eq!(
        family_canonical_schema_ref(TestTreeRow),
        M5_TEST_TREE_ROW_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(InlineResultMarker),
        M5_INLINE_RESULT_MARKER_SCHEMA_REF
    );
}

#[test]
fn all_rows_preserve_labels() {
    let p = packet();
    for row in &p.rows {
        assert!(row.preserves_labels(), "row {} broke labels", row.row_id);
    }
    assert!(p.summary.all_rows_preserve_labels);
}

#[test]
fn label_family_coverage_is_complete() {
    let p = packet();
    let covered = p.covered_label_families();
    for family in REQUIRED_LABEL_FAMILIES {
        assert!(
            covered.contains(family),
            "label family {family} not covered"
        );
    }
    assert!(p.summary.label_family_coverage_complete);
}

#[test]
fn every_narrow_reason_is_demonstrated() {
    let p = packet();
    let demonstrated = p.demonstrated_narrow_reasons();
    for reason in M5TestClaimNarrowReason::ALL {
        assert!(
            demonstrated.contains(&reason),
            "reason {reason:?} not demonstrated"
        );
    }
    assert!(p.summary.all_narrow_reasons_demonstrated);
}

#[test]
fn imported_and_live_both_present() {
    let p = packet();
    assert!(p.imported_and_live_both_present());
    assert!(p.summary.imported_and_live_both_present);
}

#[test]
fn imported_rows_carry_imported_narrow_and_never_claim_live() {
    for row in &packet().rows {
        if origin_is_imported(row.result_origin) {
            assert!(
                row.claim_narrow_reasons
                    .contains(&M5TestClaimNarrowReason::ResultsImported),
                "imported row {} does not carry ResultsImported",
                row.row_id
            );
            assert_ne!(
                row.label_parity,
                LabelParityState::Preserved,
                "imported row {} claims a full local-live parity",
                row.row_id
            );
        }
        if row.result_origin == M5TestResultOrigin::LiveLocal {
            assert!(
                !row.claim_narrow_reasons
                    .contains(&M5TestClaimNarrowReason::ResultsImported),
                "live-local row {} claims imported",
                row.row_id
            );
        }
        assert!(
            row.origin_claim_consistent(),
            "row {} origin/claim inconsistent",
            row.row_id
        );
    }
    assert!(packet().summary.all_rows_origin_claim_consistent);
}

#[test]
fn narrowed_rows_disclose_with_banner() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.discloses_narrowing(),
            "row {} does not disclose narrowing",
            row.row_id
        );
        if row.is_narrowed() {
            let banner = row
                .auto_narrow_banner
                .as_ref()
                .expect("narrowed row has a banner");
            let expected: Vec<String> = row
                .claim_narrow_reasons
                .iter()
                .map(|r| r.as_str().to_owned())
                .collect();
            assert_eq!(banner.reasons, expected);
            assert!(!banner.recovery_hints.is_empty());
            assert_eq!(row.label_parity, LabelParityState::DisclosedNarrowed);
        } else {
            assert!(row.auto_narrow_banner.is_none());
            assert_eq!(row.label_parity, LabelParityState::Preserved);
        }
    }
    assert!(p.summary.all_narrowed_rows_disclose);
}

#[test]
fn shared_lexicon_is_uniform() {
    let p = packet();
    assert!(p.shared_lexicon_uniform());
    assert!(p.summary.shared_lexicon_uniform);
    for row in &p.rows {
        assert_eq!(row.shared_state_lexicon, lexicon());
    }
}

#[test]
fn all_rows_have_copy_export_parity() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.copy_export.is_complete(),
            "row {} lacks copy/export parity",
            row.row_id
        );
    }
    assert!(p.summary.all_rows_have_copy_export);
}

#[test]
fn surface_class_is_consistent_for_every_row() {
    for row in &packet().rows {
        assert!(
            row.surface_class_consistent(),
            "row {} surface/class mismatch",
            row.row_id
        );
    }
}

#[test]
fn row_ids_are_unique() {
    let p = packet();
    let unique: BTreeSet<&str> = p.rows.iter().map(|r| r.row_id.as_str()).collect();
    assert_eq!(unique.len(), p.rows.len());
}

#[test]
fn computed_summary_matches_stored_summary() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

#[test]
fn missing_consumer_class_is_rejected() {
    let mut p = packet();
    p.rows
        .retain(|r| r.consumer_class != SharedTestConsumerClass::SupportExport);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::MissingConsumerClass { .. })));
}

#[test]
fn dropped_family_is_rejected() {
    let mut p = packet();
    p.rows.retain(|r| {
        r.component_family != M5TestExplorerWatchTriageComponentFamily::WatchModeBanner
    });
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::MissingFamilyCoverage { .. })));
}

#[test]
fn renamed_label_parity_is_rejected() {
    let mut p = packet();
    p.rows[0].label_parity = LabelParityState::RenamedOrDropped;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::LabelParityBroken { .. })));
}

#[test]
fn non_canonical_schema_ref_is_rejected() {
    let mut p = packet();
    p.rows[0].canonical_family_schema_ref = "schemas/ui/made-up.schema.json".to_owned();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::NotCanonicalFamily { .. })));
}

#[test]
fn imported_row_claiming_live_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| origin_is_imported(r.result_origin))
        .expect("an imported row exists");
    p.rows[idx].claim_narrow_reasons.clear();
    p.rows[idx].auto_narrow_banner = None;
    p.rows[idx].label_parity = LabelParityState::Preserved;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::OriginClaimDivergent { .. })));
}

#[test]
fn live_row_claiming_imported_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.result_origin == M5TestResultOrigin::LiveLocal)
        .expect("a live-local row exists");
    p.rows[idx]
        .claim_narrow_reasons
        .push(M5TestClaimNarrowReason::ResultsImported);
    p.rows[idx].label_parity = LabelParityState::DisclosedNarrowed;
    p.rows[idx].auto_narrow_banner = Some(AutoNarrowBanner {
        banner_id: "banner:forced".to_owned(),
        visible_label: "forced imported claim on a live row".to_owned(),
        reasons: p.rows[idx]
            .claim_narrow_reasons
            .iter()
            .map(|r| r.as_str().to_owned())
            .collect(),
        recovery_hints: vec!["rerun locally".to_owned()],
    });
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::OriginClaimDivergent { .. })));
}

#[test]
fn narrowed_without_banner_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("a narrowed row exists");
    p.rows[idx].auto_narrow_banner = None;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::NarrowedWithoutDisclosure { .. })));
}

#[test]
fn banner_reason_mismatch_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("a narrowed row exists");
    if let Some(banner) = p.rows[idx].auto_narrow_banner.as_mut() {
        banner.reasons = vec!["some_other_reason".to_owned()];
    }
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::NarrowedWithoutDisclosure { .. })));
}

#[test]
fn generic_banner_label_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.is_narrowed())
        .expect("a narrowed row exists");
    if let Some(banner) = p.rows[idx].auto_narrow_banner.as_mut() {
        banner.visible_label = "imported".to_owned();
    }
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::NarrowedWithoutDisclosure { .. })));
}

#[test]
fn divergent_shared_lexicon_is_rejected() {
    let mut p = packet();
    p.rows[0].shared_state_lexicon = vec!["red".to_owned(), "retry".to_owned()];
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::SharedLexiconDivergent)));
}

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut p = packet();
    p.rows[0]
        .evidence_refs
        .push("bearer abc123def456".to_owned());
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::RawBoundaryMaterialInExport)));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut p = packet();
    p.summary.row_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::SummaryMismatch)));
}

#[test]
fn duplicate_row_id_is_rejected() {
    let mut p = packet();
    let dup = p.rows[0].clone();
    p.rows.push(dup);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TestConsumerViolation::DuplicateId { .. })));
}

#[test]
fn export_json_is_deterministic() {
    let a = packet().export_safe_json();
    let b = packet().export_safe_json();
    assert_eq!(a, b);
}

#[test]
fn export_json_round_trips() {
    let p = packet();
    let json = p.export_safe_json();
    let back: TestConsumerPacket = serde_json::from_str(&json).expect("round trips");
    assert_eq!(p, back);
    assert!(back.validate().is_empty());
}

#[test]
fn csv_has_header_and_one_line_per_row() {
    let p = packet();
    let csv = p.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), p.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,consumer_class,consumer_surface"));
}

#[test]
fn markdown_summary_lists_every_row() {
    let p = packet();
    let md = p.render_markdown_summary();
    for row in &p.rows {
        assert!(
            md.contains(&row.row_id),
            "missing {} in markdown",
            row.row_id
        );
    }
}

#[test]
fn checked_in_export_matches_seeded_builder() {
    let on_disk = current_m5_test_component_consumers_export().expect("export is valid");
    assert_eq!(
        on_disk.export_safe_json(),
        packet().export_safe_json(),
        "checked-in support export drifted from the seeded builder; regenerate the artifact"
    );
}

#[test]
fn surface_tokens_are_unique() {
    let tokens: BTreeSet<&str> = SharedTestConsumerSurface::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), SharedTestConsumerSurface::ALL.len());
}

#[test]
fn narrow_reason_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5TestClaimNarrowReason::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(tokens.len(), M5TestClaimNarrowReason::ALL.len());
}
