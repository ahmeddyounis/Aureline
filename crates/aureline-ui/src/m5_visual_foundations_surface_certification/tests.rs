//! Tests for the M05-1147 visual-foundations surface certification capstone.

use super::*;

fn packet() -> VisualFoundationProfileCertificationPacket {
    seeded_m5_visual_foundations_surface_certification_packet()
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
    assert_eq!(p.record_kind, VISUAL_FOUNDATION_CERT_RECORD_KIND);
    assert_eq!(p.schema_version, VISUAL_FOUNDATION_CERT_SCHEMA_VERSION);
}

#[test]
fn every_claimed_profile_is_certified_exactly_once() {
    let p = packet();
    assert!(p.all_profiles_present());
    for profile in M5VisualFoundationCertifiedProfile::ALL {
        let count = p.rows.iter().filter(|r| r.profile == profile).count();
        assert_eq!(count, 1, "profile {profile:?} certified {count} times");
    }
    assert_eq!(
        p.summary.profile_count,
        M5VisualFoundationCertifiedProfile::ALL.len()
    );
}

#[test]
fn every_frozen_family_is_certified_on_some_profile() {
    let p = packet();
    assert!(p.all_families_covered());
    assert!(p.summary.all_families_covered);
    let families = p.represented_families();
    for family in M5VisualFoundationFamily::ALL {
        assert!(
            families.contains(&family),
            "family {family:?} not certified"
        );
    }
}

#[test]
fn packet_has_two_green_and_six_yellow_and_no_red() {
    let p = packet();
    assert_eq!(p.summary.green_row_count, 2);
    assert_eq!(p.summary.yellow_row_count, 6);
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
            VisualFoundationCertificationAxis::ALL.len()
        );
    }
    assert!(p.summary.every_axis_covered_on_every_row);
}

#[test]
fn cli_export_axis_is_certified_on_every_row() {
    let p = packet();
    for row in &p.rows {
        let export = row
            .axis(VisualFoundationCertificationAxis::CliExport)
            .expect("cli axis");
        assert_eq!(
            export.state,
            VisualFoundationAxisCertificationState::Certified
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
        VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF
    );
    for row in &p.rows {
        assert_eq!(
            row.canonical_bundle_ref,
            VISUAL_FOUNDATION_CERT_CANONICAL_BUNDLE_REF
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
        .filter(|r| r.derived_status == VisualFoundationProfileClaimStatus::Yellow)
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
        .filter(|r| r.derived_status == VisualFoundationProfileClaimStatus::Green)
    {
        assert_eq!(row.claimed_claim, row.certified_claim);
        assert!(row.claim_auto_narrow.is_none());
        assert!(row.narrowed_axes().is_empty());
    }
}

#[test]
fn only_the_live_profile_certifies_a_trusted_surface_claim() {
    for row in &packet().rows {
        if row.certified_claim.asserts_trusted_surface() {
            assert!(
                row.profile.is_live_trusted_visual_surface(),
                "non-live profile {} certifies a trusted claim",
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
                !row.certified_claim.asserts_trusted_surface(),
                "narrowed row {} still certifies a trusted claim",
                row.row_id
            );
        }
    }
}

#[test]
fn all_eight_claim_tiers_appear_as_certified_claims() {
    let p = packet();
    let tiers: BTreeSet<M5VisualFoundationA11yClaim> =
        p.rows.iter().map(|r| r.certified_claim).collect();
    for claim in M5VisualFoundationA11yClaim::ALL {
        assert!(tiers.contains(&claim), "claim tier {claim:?} not certified");
    }
}

#[test]
fn profile_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5VisualFoundationCertifiedProfile::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), M5VisualFoundationCertifiedProfile::ALL.len());
}

#[test]
fn axis_tokens_are_distinct() {
    let axes: BTreeSet<&str> = VisualFoundationCertificationAxis::ALL
        .iter()
        .map(|a| a.as_str())
        .collect();
    assert_eq!(axes.len(), VisualFoundationCertificationAxis::ALL.len());
}

#[test]
fn only_cli_export_axis_is_always_on() {
    for axis in VisualFoundationCertificationAxis::ALL {
        assert_eq!(
            axis.is_always_on(),
            axis == VisualFoundationCertificationAxis::CliExport
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
fn undisclosed_drift_blocks_the_profile() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5VisualFoundationCertifiedProfile::LiveTrustedVisualSurface)
        .expect("live-trusted-visual-surface row exists");
    let row = &mut p.rows[idx];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == VisualFoundationCertificationAxis::Visual {
            outcome.state = VisualFoundationAxisCertificationState::UndisclosedDrift;
            outcome.narrowing_reason = Some("color meaning silently dropped".to_owned());
            outcome.downgrade_trigger = None;
        }
    }
    row.derived_status = row.derive_status();
    assert_eq!(row.derived_status, VisualFoundationProfileClaimStatus::Red);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualFoundationCertificationViolation::ProfileBlocked { .. }
    )));
}

#[test]
fn guardrail_breach_blocks_the_profile() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5VisualFoundationCertifiedProfile::ReviewableGeometryStructure)
        .expect("reviewable-geometry-structure row exists");
    let row = &mut p.rows[idx];
    row.guardrails.collapses_status_or_trust_into_color_only = true;
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualFoundationCertificationViolation::GuardrailViolated { .. }
    )));
}

#[test]
fn non_live_profile_claiming_trusted_surface_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5VisualFoundationCertifiedProfile::ReviewableGeometryStructure)
        .expect("reviewable-geometry-structure row exists");
    let row = &mut p.rows[idx];
    // Claim a trusted surface on a reviewable profile — the claimed ceiling is raised too so the
    // block is the non-live rule, not a certified-exceeds-claim strengthening.
    row.claimed_claim = M5VisualFoundationA11yClaim::TrustedVisualSurface;
    row.certified_claim = M5VisualFoundationA11yClaim::TrustedVisualSurface;
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualFoundationCertificationViolation::NonLiveProfileClaimsTrustedSurface { .. }
    )));
}

#[test]
fn degraded_axis_without_claim_narrowing_blocks() {
    // A disclosed-narrowed axis but the claim stays full => hidden overclaim.
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5VisualFoundationCertifiedProfile::ReviewableGeometryStructure)
        .expect("reviewable-geometry-structure row exists");
    let row = &mut p.rows[idx];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == VisualFoundationCertificationAxis::DegradedState {
            *outcome = VisualFoundationAxisOutcome {
                axis: VisualFoundationCertificationAxis::DegradedState,
                state: VisualFoundationAxisCertificationState::DisclosedNarrowed,
                parity_note: "the contrast evidence is lagging".to_owned(),
                narrowing_reason: Some(
                    "the contrast evidence is partial for this profile".to_owned(),
                ),
                downgrade_trigger: Some(M5VisualFoundationDowngradeTrigger::ProofStale),
            };
        }
    }
    // Claim stays ReviewableVisualSurface == certified, no claim_auto_narrow.
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
}

#[test]
fn cli_export_drop_blocks_the_profile() {
    let mut p = packet();
    let row = &mut p.rows[0];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == VisualFoundationCertificationAxis::CliExport {
            outcome.state = VisualFoundationAxisCertificationState::DisclosedNarrowed;
            outcome.narrowing_reason =
                Some("export parity not current for this profile".to_owned());
            outcome.downgrade_trigger = Some(M5VisualFoundationDowngradeTrigger::ProofStale);
        }
    }
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
}

#[test]
fn incomplete_copy_export_blocks_the_profile() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.export_parity.formats.retain(|f| f != "markdown");
    assert!(!row.export_parity.is_complete());
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
}

#[test]
fn spurious_claim_auto_narrow_without_claim_reduction_blocks() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.claim_auto_narrow = Some(VisualFoundationClaimAutoNarrow {
        binding_axis: VisualFoundationCertificationAxis::DegradedState,
        from_claim: M5VisualFoundationA11yClaim::TrustedVisualSurface,
        to_claim: M5VisualFoundationA11yClaim::TrustedVisualSurface,
        visible_label: "a spurious narrowing that does not reduce the claim".to_owned(),
    });
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
}

#[test]
fn claim_narrowed_without_disclosure_blocks() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.certified_claim = M5VisualFoundationA11yClaim::ContrastUnverifiedProjection;
    row.claim_auto_narrow = None;
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
}

#[test]
fn certified_claim_above_claim_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5VisualFoundationCertifiedProfile::ReviewableGeometryStructure)
        .expect("reviewable-geometry-structure row exists");
    let row = &mut p.rows[idx];
    // claimed is ReviewableVisualSurface; certify a stronger trusted visual surface.
    row.certified_claim = M5VisualFoundationA11yClaim::TrustedVisualSurface;
    assert!(row.certified_claim.capability_rank() > row.claimed_claim.capability_rank());
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualFoundationCertificationViolation::CertifiedClaimExceedsClaim { .. }
    )));
}

#[test]
fn claim_auto_narrow_bound_to_wrong_axis_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5VisualFoundationCertifiedProfile::StaleContrastColorSurface)
        .expect("stale-contrast-color-surface row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.binding_axis = VisualFoundationCertificationAxis::Visual;
    }
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
}

#[test]
fn claim_auto_narrow_bound_to_always_on_axis_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5VisualFoundationCertifiedProfile::StaleContrastColorSurface)
        .expect("stale-contrast-color-surface row exists");
    let row = &mut p.rows[idx];
    // Force the always-on CLI/export axis to be the narrowed + binding one.
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == VisualFoundationCertificationAxis::CliExport {
            outcome.state = VisualFoundationAxisCertificationState::DisclosedNarrowed;
            outcome.narrowing_reason = Some(
                "export parity is not current for the stale contrast color surface".to_owned(),
            );
            outcome.downgrade_trigger = Some(M5VisualFoundationDowngradeTrigger::ProofStale);
        }
    }
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.binding_axis = VisualFoundationCertificationAxis::CliExport;
    }
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
}

#[test]
fn generic_narrow_label_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.derived_status == VisualFoundationProfileClaimStatus::Yellow)
        .expect("a yellow row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.visible_label = "stale".to_owned();
    }
    assert_eq!(row.derive_status(), VisualFoundationProfileClaimStatus::Red);
}

#[test]
fn certified_axis_carrying_a_reason_is_malformed() {
    let mut o = seed_certified(VisualFoundationCertificationAxis::Visual);
    o.narrowing_reason = Some("should not be here".to_owned());
    assert!(!o.well_formed());
}

#[test]
fn disclosed_axis_missing_trigger_is_malformed() {
    let mut o = seed_narrowed(
        VisualFoundationCertificationAxis::DegradedState,
        "note",
        "a genuine narrowing reason",
        M5VisualFoundationDowngradeTrigger::ProofStale,
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
    p.rows
        .retain(|r| r.profile != M5VisualFoundationCertifiedProfile::UndisclosedHitTargetSurface);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualFoundationCertificationViolation::ProfileCoverageIncomplete
    )));
}

#[test]
fn missing_family_coverage_is_rejected() {
    // Strip the syntax-token family from every row that carries it; coverage must fail.
    let mut p = packet();
    for row in &mut p.rows {
        row.consumed_families
            .retain(|f| *f != M5VisualFoundationFamily::SyntaxToken);
        row.derived_status = row.derive_status();
    }
    p.summary = p.computed_summary();
    assert!(!p.all_families_covered());
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualFoundationCertificationViolation::FamilyCoverageIncomplete
    )));
}

#[test]
fn stale_derived_status_is_rejected() {
    let mut p = packet();
    p.rows[0].derived_status = VisualFoundationProfileClaimStatus::Yellow; // it is really Green
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualFoundationCertificationViolation::StatusDerivationStale { .. }
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
        VisualFoundationCertificationViolation::RowMissingCanonicalBundle { .. }
    )));
}

#[test]
fn packet_level_wrong_bundle_is_rejected() {
    let mut p = packet();
    p.canonical_bundle_ref = "artifacts/release/other/packet.json".to_owned();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualFoundationCertificationViolation::WrongCanonicalBundle
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
        VisualFoundationCertificationViolation::DuplicateId { .. }
    )));
}

#[test]
fn axis_coverage_gap_is_rejected() {
    let mut p = packet();
    p.rows[0]
        .axis_outcomes
        .retain(|o| o.axis != VisualFoundationCertificationAxis::VisualFoundationComponentTruth);
    p.rows[0].derived_status = p.rows[0].derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VisualFoundationCertificationViolation::AxisCoverageIncomplete { .. }
    )));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut p = packet();
    p.summary.row_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VisualFoundationCertificationViolation::SummaryMismatch)));
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
        VisualFoundationCertificationViolation::RawFoundationMaterialInExport
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
    let back: VisualFoundationProfileCertificationPacket =
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
        current_m5_visual_foundations_surface_certification_export().expect("export is valid");
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
        "/../../artifacts/release/m5-visual-foundations-surface-certification/matrix.csv"
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
        "/../../artifacts/release/m5-visual-foundations-surface-certification.md"
    ));
    assert_eq!(on_disk, expected, "checked report drifted from the builder");
}

#[test]
fn checked_fixtures_mirror_the_release_artifacts() {
    let fixture_export = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-visual-foundations-surface-certification/support_export.json"
    ));
    let packet_from_fixture: VisualFoundationProfileCertificationPacket =
        serde_json::from_str(fixture_export).expect("fixture export parses");
    assert!(packet_from_fixture.validate().is_empty());
    assert_eq!(packet_from_fixture, packet());

    let fixture_csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-visual-foundations-surface-certification/matrix.csv"
    ));
    assert_eq!(fixture_csv, packet().render_matrix_csv());
}

// --- gated artifact regeneration ---
//
// Set `GEN_VISUAL_FOUNDATIONS_CERT_ARTIFACTS=1` to (re)write the checked-in release artifacts and
// fixtures from the seed builder. Off by default so a normal `cargo test` never mutates the tree.
#[test]
fn regenerate_checked_artifacts_when_requested() {
    if std::env::var("GEN_VISUAL_FOUNDATIONS_CERT_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_visual_foundations_surface_certification_packet();
    assert!(
        packet.validate().is_empty(),
        "seed must validate before write"
    );

    let manifest = env!("CARGO_MANIFEST_DIR");
    let repo = Path::new(manifest).join("../..");

    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let md = packet.render_markdown_summary();

    let release_dir = repo.join("artifacts/release/m5-visual-foundations-surface-certification");
    fs::create_dir_all(&release_dir).expect("create release dir");
    fs::write(release_dir.join("support_export.json"), &json).expect("write release export");
    fs::write(release_dir.join("matrix.csv"), &csv).expect("write release csv");
    fs::write(
        repo.join("artifacts/release/m5-visual-foundations-surface-certification.md"),
        &md,
    )
    .expect("write release report");

    let fixture_dir = repo.join("fixtures/ui/m5-visual-foundations-surface-certification");
    fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    fs::write(fixture_dir.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixture_dir.join("matrix.csv"), &csv).expect("write fixture csv");
}
