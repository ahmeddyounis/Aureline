//! Tests for the M05-843 project-entry component surface certification capstone.

use super::*;

fn packet() -> EntrySurfaceCertificationPacket {
    seeded_m5_project_entry_component_certification_packet()
}

// --------------------------------------------------------------------------
// Green-path tests
// --------------------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let violations = packet().validate();
    assert!(violations.is_empty(), "unexpected violations: {violations:?}");
}

#[test]
fn packet_record_kind_and_schema_version_are_stamped() {
    let p = packet();
    assert_eq!(p.record_kind, ENTRY_CERT_RECORD_KIND);
    assert_eq!(p.schema_version, ENTRY_CERT_SCHEMA_VERSION);
}

#[test]
fn every_claimed_surface_is_certified_exactly_once() {
    let p = packet();
    assert!(p.all_surfaces_present());
    for surface in M5ProjectEntryCertifiedSurface::ALL {
        let count = p.rows.iter().filter(|r| r.surface == surface).count();
        assert_eq!(count, 1, "surface {surface:?} certified {count} times");
    }
    assert_eq!(p.summary.surface_count, M5ProjectEntryCertifiedSurface::ALL.len());
}

#[test]
fn packet_has_four_green_and_five_yellow_and_no_red() {
    let p = packet();
    assert_eq!(p.summary.green_row_count, 4);
    assert_eq!(p.summary.yellow_row_count, 5);
    assert_eq!(p.summary.red_row_count, 0);
    assert!(p.summary.all_rows_publishable);
    assert!(p.summary.report_clean);
}

#[test]
fn every_row_scores_every_axis_exactly_once() {
    let p = packet();
    for row in &p.rows {
        assert!(row.covers_all_axes(), "row {} misses an axis", row.row_id);
        assert_eq!(row.axis_outcomes.len(), EntryCertificationAxis::ALL.len());
    }
    assert!(p.summary.every_axis_covered_on_every_row);
}

#[test]
fn export_parity_axis_is_certified_on_every_row() {
    let p = packet();
    for row in &p.rows {
        let export = row.axis(EntryCertificationAxis::ExportParity).expect("export axis");
        assert_eq!(export.state, AxisCertificationState::Certified);
        assert!(row.export_parity.is_complete());
    }
    assert!(p.summary.all_rows_export_parity_certified);
}

#[test]
fn every_row_cites_the_one_canonical_bundle() {
    let p = packet();
    assert_eq!(p.canonical_bundle_ref, ENTRY_CERT_CANONICAL_BUNDLE_REF);
    for row in &p.rows {
        assert_eq!(row.canonical_bundle_ref, ENTRY_CERT_CANONICAL_BUNDLE_REF);
    }
    assert!(p.summary.all_rows_cite_canonical_bundle);
}

#[test]
fn every_row_status_is_fresh() {
    let p = packet();
    for row in &p.rows {
        assert!(row.status_is_fresh(), "row {} status is stale", row.row_id);
    }
    assert!(p.summary.all_status_fresh);
}

#[test]
fn every_row_consumes_at_least_one_frozen_family() {
    for row in &packet().rows {
        assert!(!row.consumed_families.is_empty(), "row {} consumes no family", row.row_id);
    }
}

#[test]
fn yellow_rows_narrow_their_claim_and_bind_to_a_narrowed_axis() {
    let p = packet();
    for row in p.rows.iter().filter(|r| r.derived_status == SurfaceClaimStatus::Yellow) {
        assert!(row.is_tier_narrowed(), "yellow row {} did not narrow tier", row.row_id);
        let narrow = row
            .claim_auto_narrow
            .as_ref()
            .unwrap_or_else(|| panic!("yellow row {} has no claim_auto_narrow", row.row_id));
        assert_eq!(narrow.from_tier, row.claimed_tier);
        assert_eq!(narrow.to_tier, row.certified_tier);
        assert!(
            row.narrowed_axes().contains(&narrow.binding_axis),
            "row {} binds to an axis it did not narrow",
            row.row_id
        );
        assert!(!narrow.binding_axis.is_always_on());
    }
    assert_eq!(p.summary.narrowed_surface_count, p.summary.yellow_row_count);
}

#[test]
fn green_rows_have_no_narrowing_and_deliver_their_claim() {
    for row in packet().rows.iter().filter(|r| r.derived_status == SurfaceClaimStatus::Green) {
        assert_eq!(row.claimed_tier, row.certified_tier);
        assert!(row.claim_auto_narrow.is_none());
        assert!(row.narrowed_axes().is_empty());
    }
}

#[test]
fn surface_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5ProjectEntryCertifiedSurface::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), M5ProjectEntryCertifiedSurface::ALL.len());
}

#[test]
fn axis_tokens_and_tier_ranks_are_distinct() {
    let axes: BTreeSet<&str> = EntryCertificationAxis::ALL.iter().map(|a| a.as_str()).collect();
    assert_eq!(axes.len(), EntryCertificationAxis::ALL.len());
    let ranks: BTreeSet<u8> = EntryClaimTier::ALL.iter().map(|t| t.capability_rank()).collect();
    assert_eq!(ranks.len(), EntryClaimTier::ALL.len());
}

#[test]
fn only_export_parity_axis_is_always_on() {
    for axis in EntryCertificationAxis::ALL {
        assert_eq!(
            axis.is_always_on(),
            axis == EntryCertificationAxis::ExportParity
        );
    }
}

#[test]
fn computed_summary_matches_stored_summary() {
    let p = packet();
    assert_eq!(p.summary, p.computed_summary());
}

// --------------------------------------------------------------------------
// Derivation / red-path tests
// --------------------------------------------------------------------------

#[test]
fn undisclosed_drift_blocks_the_surface() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.surface == M5ProjectEntryCertifiedSurface::StartCenter)
        .expect("start-center row exists");
    let row = &mut p.rows[idx];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == EntryCertificationAxis::TrustPosture {
            outcome.state = AxisCertificationState::UndisclosedDrift;
            outcome.narrowing_reason = Some("trust class silently stale".to_owned());
            outcome.downgrade_trigger = None;
        }
    }
    row.derived_status = row.derive_status();
    assert_eq!(row.derived_status, SurfaceClaimStatus::Red);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryCertificationViolation::SurfaceBlocked { .. })));
}

#[test]
fn degraded_axis_without_tier_narrowing_blocks() {
    // A disclosed-narrowed axis but the claim stays FullEntry => hidden overclaim.
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.surface == M5ProjectEntryCertifiedSurface::CommandPalette)
        .expect("command-palette row exists");
    let row = &mut p.rows[idx];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == EntryCertificationAxis::TrustPosture {
            *outcome = EntryAxisOutcome {
                axis: EntryCertificationAxis::TrustPosture,
                state: AxisCertificationState::DisclosedNarrowed,
                parity_note: "trust posture lagging".to_owned(),
                narrowing_reason: Some("trust posture is not current for this entry".to_owned()),
                downgrade_trigger: Some("review_required_before_open".to_owned()),
            };
        }
    }
    // Claim tier stays FullEntry == certified FullEntry, no claim_auto_narrow.
    assert_eq!(row.derive_status(), SurfaceClaimStatus::Red);
}

#[test]
fn export_parity_drop_blocks_the_surface() {
    let mut p = packet();
    let row = &mut p.rows[0];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == EntryCertificationAxis::ExportParity {
            outcome.state = AxisCertificationState::DisclosedNarrowed;
            outcome.narrowing_reason = Some("export parity not current for this surface".to_owned());
            outcome.downgrade_trigger = Some("review_required_before_open".to_owned());
        }
    }
    assert_eq!(row.derive_status(), SurfaceClaimStatus::Red);
}

#[test]
fn incomplete_copy_export_blocks_the_surface() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.export_parity.formats.retain(|f| f != "markdown");
    assert!(!row.export_parity.is_complete());
    assert_eq!(row.derive_status(), SurfaceClaimStatus::Red);
}

#[test]
fn spurious_claim_auto_narrow_without_tier_reduction_blocks() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.claim_auto_narrow = Some(EntryClaimAutoNarrow {
        binding_axis: EntryCertificationAxis::TrustPosture,
        from_tier: EntryClaimTier::FullEntry,
        to_tier: EntryClaimTier::FullEntry,
        visible_label: "a spurious narrowing that does not reduce the tier".to_owned(),
    });
    assert_eq!(row.derive_status(), SurfaceClaimStatus::Red);
}

#[test]
fn tier_narrowed_without_disclosure_blocks() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.certified_tier = EntryClaimTier::ReviewedEntry;
    row.claim_auto_narrow = None;
    assert_eq!(row.derive_status(), SurfaceClaimStatus::Red);
}

#[test]
fn certified_tier_above_claim_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.surface == M5ProjectEntryCertifiedSurface::CliHeadless)
        .expect("cli row exists");
    let row = &mut p.rows[idx];
    row.certified_tier = EntryClaimTier::FullEntry; // claimed is ExportOnly
    assert!(row.certified_tier.capability_rank() > row.claimed_tier.capability_rank());
    assert_eq!(row.derive_status(), SurfaceClaimStatus::Red);
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryCertificationViolation::CertifiedTierExceedsClaim { .. })));
}

#[test]
fn claim_auto_narrow_bound_to_wrong_axis_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.surface == M5ProjectEntryCertifiedSurface::Restore)
        .expect("restore row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.binding_axis = EntryCertificationAxis::ProfileRemoteBadge;
    }
    assert_eq!(row.derive_status(), SurfaceClaimStatus::Red);
}

#[test]
fn generic_narrow_label_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.derived_status == SurfaceClaimStatus::Yellow)
        .expect("a yellow row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.visible_label = "reduced".to_owned();
    }
    assert_eq!(row.derive_status(), SurfaceClaimStatus::Red);
}

#[test]
fn certified_axis_carrying_a_reason_is_malformed() {
    let mut o = seed_certified(EntryCertificationAxis::TrustPosture);
    o.narrowing_reason = Some("should not be here".to_owned());
    assert!(!o.well_formed());
}

#[test]
fn disclosed_axis_missing_trigger_is_malformed() {
    let mut o = seed_narrowed(
        EntryCertificationAxis::TrustPosture,
        "note",
        "a genuine narrowing reason",
        "trigger",
    );
    o.downgrade_trigger = None;
    assert!(!o.well_formed());
}

// --------------------------------------------------------------------------
// Structural / packet-level rejection tests
// --------------------------------------------------------------------------

#[test]
fn missing_surface_is_rejected() {
    let mut p = packet();
    p.rows.retain(|r| r.surface != M5ProjectEntryCertifiedSurface::Restore);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryCertificationViolation::SurfaceCoverageIncomplete)));
}

#[test]
fn stale_derived_status_is_rejected() {
    let mut p = packet();
    p.rows[0].derived_status = SurfaceClaimStatus::Yellow; // it is really Green
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryCertificationViolation::StatusDerivationStale { .. })));
}

#[test]
fn wrong_canonical_bundle_is_rejected() {
    let mut p = packet();
    p.rows[0].canonical_bundle_ref = "artifacts/release/some-other-proof/packet.json".to_owned();
    p.rows[0].derived_status = p.rows[0].derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryCertificationViolation::RowMissingCanonicalBundle { .. })));
}

#[test]
fn packet_level_wrong_bundle_is_rejected() {
    let mut p = packet();
    p.canonical_bundle_ref = "artifacts/release/other/packet.json".to_owned();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryCertificationViolation::WrongCanonicalBundle)));
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
        .any(|v| matches!(v, EntryCertificationViolation::DuplicateId { .. })));
}

#[test]
fn axis_coverage_gap_is_rejected() {
    let mut p = packet();
    p.rows[0]
        .axis_outcomes
        .retain(|o| o.axis != EntryCertificationAxis::RestoreClass);
    p.rows[0].derived_status = p.rows[0].derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryCertificationViolation::AxisCoverageIncomplete { .. })));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut p = packet();
    p.summary.row_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryCertificationViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut p = packet();
    p.rows[0].evidence_refs.push("bearer abc123def456".to_owned());
    p.rows[0].derived_status = p.rows[0].derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, EntryCertificationViolation::RawBoundaryMaterialInExport)));
}

// --------------------------------------------------------------------------
// Rendering / round-trip tests
// --------------------------------------------------------------------------

#[test]
fn export_json_is_deterministic() {
    assert_eq!(packet().export_safe_json(), packet().export_safe_json());
}

#[test]
fn export_json_round_trips() {
    let p = packet();
    let json = p.export_safe_json();
    let back: EntrySurfaceCertificationPacket = serde_json::from_str(&json).expect("round trips");
    assert_eq!(p, back);
    assert!(back.validate().is_empty());
}

#[test]
fn csv_has_header_and_one_line_per_row() {
    let p = packet();
    let csv = p.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), p.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,surface,claimed_tier,certified_tier,status"));
}

#[test]
fn markdown_summary_lists_every_row() {
    let p = packet();
    let md = p.render_markdown_summary();
    for row in &p.rows {
        assert!(md.contains(&row.row_id), "missing {} in markdown", row.row_id);
    }
}

#[test]
fn checked_in_export_matches_seeded_builder() {
    let on_disk = current_m5_project_entry_component_certification_export().expect("export is valid");
    assert_eq!(
        on_disk.export_safe_json(),
        packet().export_safe_json(),
        "checked-in certification export drifted from the seeded builder; regenerate the artifact"
    );
}
