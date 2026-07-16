//! Tests for the M05-1264 constrained-object surface certification capstone.

use super::*;

fn packet() -> ConstrainedObjectProfileCertificationPacket {
    seeded_m5_constrained_object_surface_certification_packet()
}

// --------------------------------------------------------------------------
// Green-path tests
// --------------------------------------------------------------------------

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
    assert_eq!(p.record_kind, CONSTRAINED_OBJECT_CERT_RECORD_KIND);
    assert_eq!(p.schema_version, CONSTRAINED_OBJECT_CERT_SCHEMA_VERSION);
}

#[test]
fn every_claimed_profile_is_certified_exactly_once() {
    let p = packet();
    assert!(p.all_profiles_present());
    for profile in M5ConstrainedObjectCertifiedProfile::ALL {
        let count = p.rows.iter().filter(|r| r.profile == profile).count();
        assert_eq!(count, 1, "profile {profile:?} certified {count} times");
    }
    assert_eq!(
        p.summary.profile_count,
        M5ConstrainedObjectCertifiedProfile::ALL.len()
    );
}

#[test]
fn every_frozen_family_is_certified_on_some_profile() {
    let p = packet();
    assert!(p.all_families_covered());
    assert!(p.summary.all_families_covered);
    let families = p.represented_families();
    for family in M5ConstrainedFileStateObject::ALL {
        assert!(
            families.contains(&family),
            "family {family:?} not certified"
        );
    }
}

#[test]
fn packet_has_two_green_and_four_yellow_and_no_red() {
    let p = packet();
    assert_eq!(p.summary.green_row_count, 2);
    assert_eq!(p.summary.yellow_row_count, 4);
    assert_eq!(p.summary.red_row_count, 0);
    assert!(p.summary.all_rows_publishable);
    assert!(p.summary.report_clean);
}

#[test]
fn every_row_scores_every_axis_exactly_once() {
    let p = packet();
    for row in &p.rows {
        assert!(row.covers_all_axes(), "row {} misses an axis", row.row_id);
        assert_eq!(
            row.axis_outcomes.len(),
            ConstrainedObjectCertificationAxis::ALL.len()
        );
    }
    assert!(p.summary.every_axis_covered_on_every_row);
}

#[test]
fn cli_export_axis_is_certified_on_every_row() {
    let p = packet();
    for row in &p.rows {
        let export = row
            .axis(ConstrainedObjectCertificationAxis::CliExport)
            .expect("cli axis");
        assert_eq!(
            export.state,
            ConstrainedObjectAxisCertificationState::Certified
        );
        assert!(row.export_parity.is_complete());
    }
    assert!(p.summary.all_rows_export_parity_certified);
}

#[test]
fn every_row_cites_the_one_canonical_bundle() {
    let p = packet();
    assert_eq!(
        p.canonical_bundle_ref,
        CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF
    );
    for row in &p.rows {
        assert_eq!(
            row.canonical_bundle_ref,
            CONSTRAINED_OBJECT_CERT_CANONICAL_BUNDLE_REF
        );
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
fn every_row_holds_every_guardrail() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.guardrails.all_held(),
            "row {} breaks a guardrail",
            row.row_id
        );
    }
    assert!(p.summary.all_guardrails_held);
}

#[test]
fn every_row_consumes_at_least_one_frozen_family() {
    for row in &packet().rows {
        assert!(
            !row.consumed_families.is_empty(),
            "row {} consumes no family",
            row.row_id
        );
    }
}

#[test]
fn yellow_rows_narrow_their_claim_and_bind_to_a_narrowed_axis() {
    let p = packet();
    for row in p
        .rows
        .iter()
        .filter(|r| r.derived_status == ConstrainedObjectProfileClaimStatus::Yellow)
    {
        assert!(
            row.is_claim_narrowed(),
            "yellow row {} did not narrow claim",
            row.row_id
        );
        let narrow = row
            .claim_auto_narrow
            .as_ref()
            .unwrap_or_else(|| panic!("yellow row {} has no claim_auto_narrow", row.row_id));
        assert_eq!(narrow.from_claim, row.claimed_claim);
        assert_eq!(narrow.to_claim, row.certified_claim);
        assert!(
            row.narrowed_axes().contains(&narrow.binding_axis),
            "row {} binds to an axis it did not narrow",
            row.row_id
        );
        assert!(!narrow.binding_axis.is_always_on());
    }
    assert_eq!(p.summary.narrowed_profile_count, p.summary.yellow_row_count);
}

#[test]
fn green_rows_have_no_narrowing_and_deliver_their_claim() {
    for row in packet()
        .rows
        .iter()
        .filter(|r| r.derived_status == ConstrainedObjectProfileClaimStatus::Green)
    {
        assert_eq!(row.claimed_claim, row.certified_claim);
        assert!(row.claim_auto_narrow.is_none());
        assert!(row.narrowed_axes().is_empty());
    }
}

#[test]
fn only_the_current_lane_profile_certifies_a_trusted_surface_claim() {
    for row in &packet().rows {
        if row
            .certified_claim
            .asserts_certified_constrained_object_truth()
        {
            assert!(
                row.profile.is_certified_constrained_object_lane(),
                "non-current-lane profile {} certifies a trusted claim",
                row.row_id
            );
        }
    }
}

#[test]
fn a_narrowed_profile_never_keeps_a_trusted_claim() {
    for row in &packet().rows {
        if row.is_claim_narrowed() {
            assert!(
                !row.certified_claim
                    .asserts_certified_constrained_object_truth(),
                "narrowed row {} still certifies a trusted claim",
                row.row_id
            );
        }
    }
}

#[test]
fn all_six_claim_tiers_appear_as_certified_claims() {
    let p = packet();
    let tiers: BTreeSet<M5ConstrainedObjectClaim> =
        p.rows.iter().map(|r| r.certified_claim).collect();
    for claim in M5ConstrainedObjectClaim::ALL {
        assert!(tiers.contains(&claim), "claim tier {claim:?} not certified");
    }
}

#[test]
fn profile_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5ConstrainedObjectCertifiedProfile::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), M5ConstrainedObjectCertifiedProfile::ALL.len());
}

#[test]
fn axis_tokens_are_distinct() {
    let axes: BTreeSet<&str> = ConstrainedObjectCertificationAxis::ALL
        .iter()
        .map(|a| a.as_str())
        .collect();
    assert_eq!(axes.len(), ConstrainedObjectCertificationAxis::ALL.len());
}

#[test]
fn only_cli_export_axis_is_always_on() {
    for axis in ConstrainedObjectCertificationAxis::ALL {
        assert_eq!(
            axis.is_always_on(),
            axis == ConstrainedObjectCertificationAxis::CliExport
        );
    }
}

#[test]
fn localization_axis_is_a_distinct_reach_axis() {
    // The B150 localization axis is present, distinct, and exercised by the
    // disclosed-imported-offline-partial profile as its binding narrowed axis.
    let p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| {
            r.profile
                == M5ConstrainedObjectCertifiedProfile::DisclosedGeneratedDivergencePartialProfile
        })
        .expect("disclosed-imported-offline-partial-profile row exists");
    let row = &p.rows[idx];
    let outcome = row
        .axis(ConstrainedObjectCertificationAxis::Localization)
        .expect("localization axis present");
    assert_eq!(
        outcome.state,
        ConstrainedObjectAxisCertificationState::DisclosedNarrowed
    );
    assert_eq!(
        row.claim_auto_narrow.as_ref().map(|n| n.binding_axis),
        Some(ConstrainedObjectCertificationAxis::Localization)
    );
}

#[test]
fn high_contrast_axis_is_present_and_certified_on_every_row() {
    // The B150 high-contrast reach axis stays certified across the seed (never a binding narrowed axis here).
    for row in &packet().rows {
        let outcome = row
            .axis(ConstrainedObjectCertificationAxis::HighContrast)
            .expect("high-contrast axis present");
        assert_eq!(
            outcome.state,
            ConstrainedObjectAxisCertificationState::Certified
        );
    }
}

#[test]
fn the_four_yellow_rows_bind_four_distinct_axes_and_triggers() {
    // Each yellow profile discloses via a distinct binding axis and a distinct downgrade trigger, matching the
    // per-lane concern (generated divergence / canonical source / write-target review / actor parity).
    let p = packet();
    let axes: BTreeSet<ConstrainedObjectCertificationAxis> = p
        .rows
        .iter()
        .filter_map(|r| r.claim_auto_narrow.as_ref().map(|n| n.binding_axis))
        .collect();
    assert_eq!(axes.len(), 4, "yellow rows must bind four distinct axes");
    let triggers: std::collections::HashSet<M5ConstrainedFileStateDowngradeTrigger> = p
        .rows
        .iter()
        .flat_map(|r| r.axis_outcomes.iter().filter_map(|o| o.downgrade_trigger))
        .collect();
    assert_eq!(
        triggers.len(),
        4,
        "yellow rows must carry four distinct downgrade triggers"
    );
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
fn undisclosed_drift_blocks_the_profile() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| {
            r.profile == M5ConstrainedObjectCertifiedProfile::CertifiedConstrainedObjectLane
        })
        .expect("current-constrained-object-lane row exists");
    let row = &mut p.rows[idx];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == ConstrainedObjectCertificationAxis::Visual {
            outcome.state = ConstrainedObjectAxisCertificationState::UndisclosedDrift;
            outcome.narrowing_reason = Some("snapshot label silently dropped".to_owned());
            outcome.downgrade_trigger = None;
        }
    }
    row.derived_status = row.derive_status();
    assert_eq!(row.derived_status, ConstrainedObjectProfileClaimStatus::Red);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::ProfileBlocked { .. }
    )));
}

#[test]
fn guardrail_breach_blocks_the_profile() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| {
            r.profile == M5ConstrainedObjectCertifiedProfile::ReviewableConstrainedRecordStructure
        })
        .expect("reviewable-snapshot-record-structure row exists");
    let row = &mut p.rows[idx];
    row.guardrails
        .lets_one_constrained_state_class_hide_another_when_both_materially_affect_behavior = true;
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::GuardrailViolated { .. }
    )));
}

#[test]
fn non_live_profile_claiming_trusted_surface_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| {
            r.profile == M5ConstrainedObjectCertifiedProfile::ReviewableConstrainedRecordStructure
        })
        .expect("reviewable-snapshot-record-structure row exists");
    let row = &mut p.rows[idx];
    // Claim a trusted surface on a reviewable profile — the claimed ceiling is raised too so the block is the
    // non-current-lane rule, not a certified-exceeds-claim strengthening.
    row.claimed_claim = M5ConstrainedObjectClaim::CertifiedConstrainedObjectTruth;
    row.certified_claim = M5ConstrainedObjectClaim::CertifiedConstrainedObjectTruth;
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::NonLaneProfileClaimsCertifiedTruth { .. }
    )));
}

#[test]
fn degraded_axis_without_claim_narrowing_blocks() {
    // A disclosed-narrowed axis but the claim stays full => hidden overclaim.
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| {
            r.profile == M5ConstrainedObjectCertifiedProfile::ReviewableConstrainedRecordStructure
        })
        .expect("reviewable-snapshot-record-structure row exists");
    let row = &mut p.rows[idx];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == ConstrainedObjectCertificationAxis::DegradedState {
            *outcome = ConstrainedObjectAxisOutcome {
                axis: ConstrainedObjectCertificationAxis::DegradedState,
                state: ConstrainedObjectAxisCertificationState::DisclosedNarrowed,
                parity_note: "the live target is unresolved".to_owned(),
                narrowing_reason: Some(
                    "the live-target availability is partial for this profile".to_owned(),
                ),
                downgrade_trigger: Some(
                    M5ConstrainedFileStateDowngradeTrigger::SilentLossyDirectWriteFallback,
                ),
            };
        }
    }
    // Claim stays ReviewableConstrainedStateRecord == certified, no claim_auto_narrow.
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
}

#[test]
fn cli_export_drop_blocks_the_profile() {
    let mut p = packet();
    let row = &mut p.rows[0];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == ConstrainedObjectCertificationAxis::CliExport {
            outcome.state = ConstrainedObjectAxisCertificationState::DisclosedNarrowed;
            outcome.narrowing_reason =
                Some("export parity not current for this profile".to_owned());
            outcome.downgrade_trigger =
                Some(M5ConstrainedFileStateDowngradeTrigger::SilentLossyDirectWriteFallback);
        }
    }
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
}

#[test]
fn incomplete_copy_export_blocks_the_profile() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.export_parity.formats.retain(|f| f != "markdown");
    assert!(!row.export_parity.is_complete());
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
}

#[test]
fn spurious_claim_auto_narrow_without_claim_reduction_blocks() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.claim_auto_narrow = Some(ConstrainedObjectClaimAutoNarrow {
        binding_axis: ConstrainedObjectCertificationAxis::DegradedState,
        from_claim: M5ConstrainedObjectClaim::CertifiedConstrainedObjectTruth,
        to_claim: M5ConstrainedObjectClaim::CertifiedConstrainedObjectTruth,
        visible_label: "a spurious narrowing that does not reduce the claim".to_owned(),
    });
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
}

#[test]
fn claim_narrowed_without_disclosure_blocks() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.certified_claim = M5ConstrainedObjectClaim::GeneratedDivergenceDisclosedProjection;
    row.claim_auto_narrow = None;
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
}

#[test]
fn certified_claim_above_claim_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| {
            r.profile == M5ConstrainedObjectCertifiedProfile::ReviewableConstrainedRecordStructure
        })
        .expect("reviewable-snapshot-record-structure row exists");
    let row = &mut p.rows[idx];
    // claimed is ReviewableConstrainedStateRecord; certify a stronger certified operating line.
    row.certified_claim = M5ConstrainedObjectClaim::CertifiedConstrainedObjectTruth;
    assert!(row.certified_claim.capability_rank() > row.claimed_claim.capability_rank());
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::CertifiedClaimExceedsClaim { .. }
    )));
}

#[test]
fn claim_auto_narrow_bound_to_wrong_axis_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| {
            r.profile
                == M5ConstrainedObjectCertifiedProfile::DisclosedGeneratedDivergencePartialProfile
        })
        .expect("disclosed-imported-offline-partial-profile row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.binding_axis = ConstrainedObjectCertificationAxis::Visual;
    }
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
}

#[test]
fn claim_auto_narrow_bound_to_always_on_axis_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| {
            r.profile
                == M5ConstrainedObjectCertifiedProfile::DisclosedGeneratedDivergencePartialProfile
        })
        .expect("disclosed-imported-offline-partial-profile row exists");
    let row = &mut p.rows[idx];
    // Force the always-on CLI/export axis to be the narrowed + binding one.
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == ConstrainedObjectCertificationAxis::CliExport {
            outcome.state = ConstrainedObjectAxisCertificationState::DisclosedNarrowed;
            outcome.narrowing_reason = Some(
                "export parity is not current for the imported / offline-partial surface"
                    .to_owned(),
            );
            outcome.downgrade_trigger =
                Some(M5ConstrainedFileStateDowngradeTrigger::SilentLossyDirectWriteFallback);
        }
    }
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.binding_axis = ConstrainedObjectCertificationAxis::CliExport;
    }
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
}

#[test]
fn generic_narrow_label_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.derived_status == ConstrainedObjectProfileClaimStatus::Yellow)
        .expect("a yellow row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.visible_label = "generated".to_owned();
    }
    assert_eq!(
        row.derive_status(),
        ConstrainedObjectProfileClaimStatus::Red
    );
}

#[test]
fn certified_axis_carrying_a_reason_is_malformed() {
    let mut o = seed_certified(ConstrainedObjectCertificationAxis::Visual);
    o.narrowing_reason = Some("should not be here".to_owned());
    assert!(!o.well_formed());
}

#[test]
fn disclosed_axis_missing_trigger_is_malformed() {
    let mut o = seed_narrowed(
        ConstrainedObjectCertificationAxis::DegradedState,
        "note",
        "a genuine narrowing reason",
        M5ConstrainedFileStateDowngradeTrigger::SilentLossyDirectWriteFallback,
    );
    o.downgrade_trigger = None;
    assert!(!o.well_formed());
}

// --------------------------------------------------------------------------
// Structural / packet-level rejection tests
// --------------------------------------------------------------------------

#[test]
fn missing_profile_is_rejected() {
    let mut p = packet();
    p.rows.retain(|r| {
        r.profile != M5ConstrainedObjectCertifiedProfile::UnverifiedWriteTargetReviewProfile
    });
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::ProfileCoverageIncomplete
    )));
}

#[test]
fn missing_family_coverage_is_rejected() {
    // Strip the certified-archetype class from every row that carries it; coverage must fail.
    let mut p = packet();
    for row in &mut p.rows {
        row.consumed_families
            .retain(|f| *f != M5ConstrainedFileStateObject::ReadOnly);
        row.derived_status = row.derive_status();
    }
    p.summary = p.computed_summary();
    assert!(!p.all_families_covered());
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::FamilyCoverageIncomplete
    )));
}

#[test]
fn stale_derived_status_is_rejected() {
    let mut p = packet();
    p.rows[0].derived_status = ConstrainedObjectProfileClaimStatus::Yellow; // it is really Green
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::StatusDerivationStale { .. }
    )));
}

#[test]
fn wrong_canonical_bundle_is_rejected() {
    let mut p = packet();
    p.rows[0].canonical_bundle_ref = "artifacts/release/some-other-proof/packet.json".to_owned();
    p.rows[0].derived_status = p.rows[0].derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::RowMissingCanonicalBundle { .. }
    )));
}

#[test]
fn packet_level_wrong_bundle_is_rejected() {
    let mut p = packet();
    p.canonical_bundle_ref = "artifacts/release/other/packet.json".to_owned();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::WrongCanonicalBundle
    )));
}

#[test]
fn duplicate_row_id_is_rejected() {
    let mut p = packet();
    let dup = p.rows[0].clone();
    p.rows.push(dup);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::DuplicateId { .. }
    )));
}

#[test]
fn axis_coverage_gap_is_rejected() {
    let mut p = packet();
    p.rows[0]
        .axis_outcomes
        .retain(|o| o.axis != ConstrainedObjectCertificationAxis::ConstrainedObjectTruth);
    p.rows[0].derived_status = p.rows[0].derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::AxisCoverageIncomplete { .. }
    )));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut p = packet();
    p.summary.row_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, ConstrainedObjectCertificationViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut p = packet();
    p.rows[0]
        .evidence_refs
        .push("bearer abc123def456".to_owned());
    p.rows[0].derived_status = p.rows[0].derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ConstrainedObjectCertificationViolation::RawConstrainedObjectMaterialInExport
    )));
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
    let back: ConstrainedObjectProfileCertificationPacket =
        serde_json::from_str(&json).expect("round trips");
    assert_eq!(p, back);
    assert!(back.validate().is_empty());
}

#[test]
fn csv_has_header_and_one_line_per_row() {
    let p = packet();
    let csv = p.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), p.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,profile,claimed_claim,certified_claim,status"));
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

// --- byte-lock against the checked-in artifacts ---

#[test]
fn checked_in_export_matches_seeded_builder() {
    let on_disk =
        current_m5_constrained_object_surface_certification_export().expect("export is valid");
    assert_eq!(
        on_disk.export_safe_json(),
        packet().export_safe_json(),
        "checked-in certification export drifted from the seeded builder; regenerate the artifact"
    );
}

#[test]
fn checked_matrix_csv_matches_builder() {
    let expected = packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-constrained-object-surface-certification/matrix.csv"
    ));
    assert_eq!(
        on_disk, expected,
        "checked matrix CSV drifted from the builder"
    );
}

#[test]
fn checked_report_matches_builder() {
    let expected = packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-constrained-object-surface-certification.md"
    ));
    assert_eq!(on_disk, expected, "checked report drifted from the builder");
}

#[test]
fn checked_fixtures_mirror_the_release_artifacts() {
    let fixture_export = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-constrained-object-surface-certification/support_export.json"
    ));
    let packet_from_fixture: ConstrainedObjectProfileCertificationPacket =
        serde_json::from_str(fixture_export).expect("fixture export parses");
    assert!(packet_from_fixture.validate().is_empty());
    assert_eq!(packet_from_fixture, packet());

    let fixture_csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-constrained-object-surface-certification/matrix.csv"
    ));
    assert_eq!(fixture_csv, packet().render_matrix_csv());
}

// --- gated artifact regeneration ---
//
// Set `GEN_CONSTRAINED_OBJECT_CERT_ARTIFACTS=1` to (re)write the checked-in release artifacts and fixtures
// from the seed builder. Off by default so a normal `cargo test` never mutates the tree.
#[test]
fn regenerate_checked_artifacts_when_requested() {
    if std::env::var("GEN_CONSTRAINED_OBJECT_CERT_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_constrained_object_surface_certification_packet();
    assert!(
        packet.validate().is_empty(),
        "seed must validate before write"
    );

    let manifest = env!("CARGO_MANIFEST_DIR");
    let repo = Path::new(manifest).join("../..");

    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let md = packet.render_markdown_summary();

    let release_dir = repo.join("artifacts/release/m5-constrained-object-surface-certification");
    fs::create_dir_all(&release_dir).expect("create release dir");
    fs::write(release_dir.join("support_export.json"), &json).expect("write release export");
    fs::write(release_dir.join("matrix.csv"), &csv).expect("write release csv");
    fs::write(
        repo.join("artifacts/release/m5-constrained-object-surface-certification.md"),
        &md,
    )
    .expect("write release report");

    let fixture_dir = repo.join("fixtures/release/m5-constrained-object-surface-certification");
    fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    fs::write(fixture_dir.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixture_dir.join("matrix.csv"), &csv).expect("write fixture csv");
}
