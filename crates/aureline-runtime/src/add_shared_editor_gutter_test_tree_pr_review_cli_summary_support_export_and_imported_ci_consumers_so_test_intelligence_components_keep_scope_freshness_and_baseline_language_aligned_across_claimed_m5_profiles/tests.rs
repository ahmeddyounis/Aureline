//! Tests for the M05-1033 test-intelligence component consumer adoption lane.

use super::*;

fn packet() -> IntelConsumerPacket {
    seeded_m5_test_intelligence_component_consumers_packet()
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
    assert_eq!(p.record_kind, TEST_INTEL_CONSUMER_RECORD_KIND);
    assert_eq!(p.schema_version, TEST_INTEL_CONSUMER_SCHEMA_VERSION);
}

#[test]
fn all_six_consumer_classes_present() {
    let p = packet();
    assert!(p.summary.editor_surface_present);
    assert!(p.summary.test_tree_present);
    assert!(p.summary.review_surface_present);
    assert!(p.summary.cli_summary_present);
    assert!(p.summary.imported_ci_detail_present);
    assert!(p.summary.support_export_present);
    assert_eq!(
        p.summary.consumer_class_count,
        SharedIntelConsumerClass::ALL.len()
    );
}

#[test]
fn every_frozen_family_is_adopted() {
    let p = packet();
    let families = p.represented_families();
    for family in M5TestIntelligenceComponentFamily::ALL {
        assert!(families.contains(&family), "missing family {family:?}");
    }
    assert_eq!(
        p.summary.component_family_count,
        M5TestIntelligenceComponentFamily::ALL.len()
    );
}

#[test]
fn every_consumer_surface_is_adopted() {
    let p = packet();
    let surfaces: BTreeSet<SharedIntelConsumerSurface> =
        p.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in SharedIntelConsumerSurface::ALL {
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
fn twin_primitives_share_one_release_packet() {
    use M5TestIntelligenceComponentFamily::*;
    assert_eq!(
        family_canonical_packet_ref(CoverageSummaryBar),
        family_canonical_packet_ref(CoverageOverlayMarker)
    );
    assert_eq!(
        family_canonical_packet_ref(FlakyStateBadge),
        family_canonical_packet_ref(RetryHistoryRow)
    );
    assert_eq!(
        family_canonical_packet_ref(SnapshotReviewCard),
        family_canonical_packet_ref(CoverageImportMergeSheet)
    );
    assert_eq!(
        family_canonical_schema_ref(CoverageSummaryBar),
        M5_COVERAGE_COMPONENTS_SUMMARY_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(TestGenerationSuggestionCard),
        M5_SUGGESTION_CARD_COMPONENTS_SCHEMA_REF
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
fn all_rows_preserve_required_actions() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.preserves_required_actions(),
            "row {} dropped a required action",
            row.row_id
        );
    }
    assert!(p.summary.all_rows_preserve_required_actions);
}

#[test]
fn all_row_invariants_hold() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.invariants.holds(),
            "row {} invariant broken",
            row.row_id
        );
    }
    assert!(p.summary.all_row_invariants_hold);
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
    for reason in M5IntelClaimNarrowReason::ALL {
        assert!(
            demonstrated.contains(&reason),
            "reason {reason:?} not demonstrated"
        );
    }
    assert!(p.summary.all_narrow_reasons_demonstrated);
}

#[test]
fn imported_and_current_both_present() {
    let p = packet();
    assert!(p.imported_and_current_both_present());
    assert!(p.summary.imported_and_current_both_present);
}

#[test]
fn weak_provenance_rows_carry_forced_narrow_and_never_claim_current() {
    for row in &packet().rows {
        if let Some(forced) = provenance_forced_reason(row.result_provenance) {
            assert!(
                row.claim_narrow_reasons.contains(&forced),
                "weak-provenance row {} does not carry {forced:?}",
                row.row_id
            );
            assert_ne!(
                row.label_parity,
                LabelParityState::Preserved,
                "weak-provenance row {} claims a full current-run parity",
                row.row_id
            );
        }
        if row.result_provenance == M5TestIntelligenceProvenanceClass::VerifiedCurrentRun {
            assert!(
                !row.claim_narrow_reasons
                    .contains(&M5IntelClaimNarrowReason::EvidenceImported)
                    && !row
                        .claim_narrow_reasons
                        .contains(&M5IntelClaimNarrowReason::ProvenanceStale),
                "verified current-run row {} claims imported or stale",
                row.row_id
            );
        }
        assert!(
            row.provenance_claim_consistent(),
            "row {} provenance/claim inconsistent",
            row.row_id
        );
    }
    assert!(packet().summary.all_rows_provenance_claim_consistent);
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
        .retain(|r| r.consumer_class != SharedIntelConsumerClass::SupportExport);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelConsumerViolation::MissingConsumerClass { .. })));
}

#[test]
fn dropped_family_is_rejected() {
    let mut p = packet();
    p.rows
        .retain(|r| r.component_family != M5TestIntelligenceComponentFamily::RetryHistoryRow);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelConsumerViolation::MissingFamilyCoverage { .. })));
}

#[test]
fn renamed_label_parity_is_rejected() {
    let mut p = packet();
    p.rows[0].label_parity = LabelParityState::RenamedOrDropped;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelConsumerViolation::LabelParityBroken { .. })));
}

#[test]
fn non_canonical_schema_ref_is_rejected() {
    let mut p = packet();
    p.rows[0].canonical_family_schema_ref = "schemas/ui/made-up.schema.json".to_owned();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelConsumerViolation::NotCanonicalFamily { .. })));
}

#[test]
fn dropped_required_action_is_rejected() {
    let mut p = packet();
    p.rows[0].preserved_actions.clear();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelConsumerViolation::MissingRequiredActions { .. })));
}

#[test]
fn imported_row_claiming_current_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| provenance_is_imported(r.result_provenance))
        .expect("an imported row exists");
    p.rows[idx].claim_narrow_reasons.clear();
    p.rows[idx].auto_narrow_banner = None;
    p.rows[idx].label_parity = LabelParityState::Preserved;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelConsumerViolation::ProvenanceClaimDivergent { .. })));
}

#[test]
fn current_row_claiming_imported_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.result_provenance == M5TestIntelligenceProvenanceClass::VerifiedCurrentRun)
        .expect("a verified current-run row exists");
    p.rows[idx]
        .claim_narrow_reasons
        .push(M5IntelClaimNarrowReason::EvidenceImported);
    p.rows[idx].label_parity = LabelParityState::DisclosedNarrowed;
    p.rows[idx].auto_narrow_banner = Some(AutoNarrowBanner {
        banner_id: "banner:forced".to_owned(),
        visible_label: "forced imported claim on a current-run row".to_owned(),
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
        .any(|v| matches!(v, IntelConsumerViolation::ProvenanceClaimDivergent { .. })));
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
        .any(|v| matches!(v, IntelConsumerViolation::NarrowedWithoutDisclosure { .. })));
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
        .any(|v| matches!(v, IntelConsumerViolation::NarrowedWithoutDisclosure { .. })));
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
        .any(|v| matches!(v, IntelConsumerViolation::NarrowedWithoutDisclosure { .. })));
}

#[test]
fn violated_hard_invariant_is_rejected() {
    let mut p = packet();
    p.rows[0]
        .invariants
        .collapses_shard_omission_into_single_percentage = true;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelConsumerViolation::RowInvariantViolated { .. })));
}

#[test]
fn bundled_generated_apply_invariant_is_rejected() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| {
            r.component_family == M5TestIntelligenceComponentFamily::TestGenerationSuggestionCard
        })
        .expect("a generation row exists");
    p.rows[idx]
        .invariants
        .bundles_generated_changes_into_opaque_apply = true;
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelConsumerViolation::RowInvariantViolated { .. })));
}

#[test]
fn divergent_shared_lexicon_is_rejected() {
    let mut p = packet();
    p.rows[0].shared_state_lexicon = vec!["local".to_owned(), "confirmed".to_owned()];
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelConsumerViolation::SharedLexiconDivergent)));
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
        .any(|v| matches!(v, IntelConsumerViolation::RawBoundaryMaterialInExport)));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut p = packet();
    p.summary.row_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, IntelConsumerViolation::SummaryMismatch)));
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
        .any(|v| matches!(v, IntelConsumerViolation::DuplicateId { .. })));
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
    let back: IntelConsumerPacket = serde_json::from_str(&json).expect("round trips");
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
    let on_disk =
        current_m5_test_intelligence_component_consumers_export().expect("export is valid");
    assert_eq!(
        on_disk.export_safe_json(),
        packet().export_safe_json(),
        "checked-in support export drifted from the seeded builder; regenerate the artifact"
    );
}

#[test]
fn surface_tokens_are_unique() {
    let tokens: BTreeSet<&str> = SharedIntelConsumerSurface::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), SharedIntelConsumerSurface::ALL.len());
}

#[test]
fn narrow_reason_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5IntelClaimNarrowReason::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(tokens.len(), M5IntelClaimNarrowReason::ALL.len());
}
