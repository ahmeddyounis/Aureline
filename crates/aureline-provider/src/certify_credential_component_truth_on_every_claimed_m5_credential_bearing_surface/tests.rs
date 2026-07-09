//! Tests for the M05-995 credential component surface certification capstone.

use super::*;

fn packet() -> CredentialSurfaceCertificationPacket {
    seeded_m5_credential_component_certification_packet()
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
    assert_eq!(p.record_kind, CREDENTIAL_CERT_RECORD_KIND);
    assert_eq!(p.schema_version, CREDENTIAL_CERT_SCHEMA_VERSION);
}

#[test]
fn every_claimed_surface_is_certified_exactly_once() {
    let p = packet();
    assert!(p.all_surfaces_present());
    for surface in M5CredentialCertifiedSurface::ALL {
        let count = p.rows.iter().filter(|r| r.surface == surface).count();
        assert_eq!(count, 1, "surface {surface:?} certified {count} times");
    }
    assert_eq!(
        p.summary.surface_count,
        M5CredentialCertifiedSurface::ALL.len()
    );
}

#[test]
fn every_frozen_family_is_certified_on_some_surface() {
    let p = packet();
    assert!(p.all_families_covered());
    assert!(p.summary.all_families_covered);
    let families = p.represented_families();
    for family in M5CredentialComponentFamily::ALL {
        assert!(
            families.contains(&family),
            "family {family:?} not certified"
        );
    }
}

#[test]
fn packet_has_four_green_and_four_yellow_and_no_red() {
    let p = packet();
    assert_eq!(p.summary.green_row_count, 4);
    assert_eq!(p.summary.yellow_row_count, 4);
    assert_eq!(p.summary.red_row_count, 0);
    assert!(p.summary.all_rows_publishable);
    assert!(p.summary.report_clean);
}

#[test]
fn every_surface_preserves_credential_truth() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.preserves_credential_truth_continuity(),
            "row {} drops credential truth",
            row.row_id
        );
        assert!(row.credential_truth_preserved);
    }
    assert!(p.summary.all_credential_truth_preserved);
}

#[test]
fn every_row_scores_every_axis_exactly_once() {
    let p = packet();
    for row in &p.rows {
        assert!(row.covers_all_axes(), "row {} misses an axis", row.row_id);
        assert_eq!(
            row.axis_outcomes.len(),
            CredentialCertificationAxis::ALL.len()
        );
    }
    assert!(p.summary.every_axis_covered_on_every_row);
}

#[test]
fn cli_export_axis_is_certified_on_every_row() {
    let p = packet();
    for row in &p.rows {
        let export = row
            .axis(CredentialCertificationAxis::CliExport)
            .expect("cli axis");
        assert_eq!(export.state, CredentialAxisCertificationState::Certified);
        assert!(row.export_parity.is_complete());
    }
    assert!(p.summary.all_rows_export_parity_certified);
}

#[test]
fn every_row_cites_the_one_canonical_bundle() {
    let p = packet();
    assert_eq!(p.canonical_bundle_ref, CREDENTIAL_CERT_CANONICAL_BUNDLE_REF);
    for row in &p.rows {
        assert_eq!(
            row.canonical_bundle_ref,
            CREDENTIAL_CERT_CANONICAL_BUNDLE_REF
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
        .filter(|r| r.derived_status == CredentialSurfaceClaimStatus::Yellow)
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
        assert!(narrow.preserves_credential_truth_continuity);
    }
    assert_eq!(p.summary.narrowed_surface_count, p.summary.yellow_row_count);
}

#[test]
fn green_rows_have_no_narrowing_and_deliver_their_claim() {
    for row in packet()
        .rows
        .iter()
        .filter(|r| r.derived_status == CredentialSurfaceClaimStatus::Green)
    {
        assert_eq!(row.claimed_claim, row.certified_claim);
        assert!(row.claim_auto_narrow.is_none());
        assert!(row.narrowed_axes().is_empty());
    }
}

#[test]
fn the_four_spec_auto_narrow_conditions_are_each_certified() {
    // The four spec narrowing conditions: unverified store, expired auth posture, drifted delegated
    // scope, and blocked reveal policy — must each be a certified yellow surface.
    let p = packet();
    let certified: BTreeSet<M5CredentialComponentClaim> = p
        .rows
        .iter()
        .filter(|r| r.derived_status == CredentialSurfaceClaimStatus::Yellow)
        .map(|r| r.certified_claim)
        .collect();
    assert!(certified.contains(&M5CredentialComponentClaim::UnverifiedStoreProjection));
    assert!(certified.contains(&M5CredentialComponentClaim::ExpiredAuthProjection));
    assert!(certified.contains(&M5CredentialComponentClaim::DriftedDelegationProjection));
    assert!(certified.contains(&M5CredentialComponentClaim::RevealBlockedProjection));
}

#[test]
fn certified_never_implies_verified_storage_current_auth_or_allowed_reveal() {
    // AC2 theme: a surface whose store is unverified, auth is expired, delegation has drifted, or
    // reveal is blocked must never certify a verified-brokered or fully self-sufficient projection.
    // The green surfaces stay honestly full; the yellow surfaces narrow below the broker/handle
    // ceiling so "certified" never implies verified storage, current auth, or an allowed reveal.
    let p = packet();
    for row in &p.rows {
        match row.derived_status {
            CredentialSurfaceClaimStatus::Green => {
                assert!(
                    row.certified_claim.asserts_full_projection(),
                    "green row {} should deliver a full projection",
                    row.row_id
                );
            }
            CredentialSurfaceClaimStatus::Yellow => {
                assert!(
                    !row.certified_claim.asserts_verified_brokered(),
                    "yellow row {} still asserts verified-brokered",
                    row.row_id
                );
                assert!(
                    !row.certified_claim.asserts_full_projection(),
                    "yellow row {} still asserts a full projection",
                    row.row_id
                );
            }
            CredentialSurfaceClaimStatus::Red => panic!("no red rows in the seeded packet"),
        }
    }
}

#[test]
fn surface_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5CredentialCertifiedSurface::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), M5CredentialCertifiedSurface::ALL.len());
}

#[test]
fn axis_tokens_are_distinct() {
    let axes: BTreeSet<&str> = CredentialCertificationAxis::ALL
        .iter()
        .map(|a| a.as_str())
        .collect();
    assert_eq!(axes.len(), CredentialCertificationAxis::ALL.len());
}

#[test]
fn only_cli_export_axis_is_always_on() {
    for axis in CredentialCertificationAxis::ALL {
        assert_eq!(
            axis.is_always_on(),
            axis == CredentialCertificationAxis::CliExport
        );
    }
}

#[test]
fn credential_claim_ladder_is_strictly_ordered() {
    let ranks: Vec<u8> = M5CredentialComponentClaim::ALL
        .iter()
        .map(|c| c.capability_rank())
        .collect();
    assert_eq!(ranks, vec![5, 4, 3, 2, 1, 0]);
    let tokens: BTreeSet<&str> = M5CredentialComponentClaim::ALL
        .iter()
        .map(|c| c.as_str())
        .collect();
    assert_eq!(tokens.len(), M5CredentialComponentClaim::ALL.len());
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
        .position(|r| r.surface == M5CredentialCertifiedSurface::ConnectorAuthorization)
        .expect("connector-authorization row exists");
    let row = &mut p.rows[idx];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == CredentialCertificationAxis::Visual {
            outcome.state = CredentialAxisCertificationState::UndisclosedDrift;
            outcome.narrowing_reason = Some("storage mode silently dropped".to_owned());
            outcome.downgrade_trigger = None;
        }
    }
    row.derived_status = row.derive_status();
    assert_eq!(row.derived_status, CredentialSurfaceClaimStatus::Red);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, CredentialCertificationViolation::SurfaceBlocked { .. })));
}

#[test]
fn degraded_axis_without_claim_narrowing_blocks() {
    // A disclosed-narrowed axis but the claim stays full => hidden overclaim.
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.surface == M5CredentialCertifiedSurface::SupportExport)
        .expect("support-export row exists");
    let row = &mut p.rows[idx];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == CredentialCertificationAxis::DegradedState {
            *outcome = CredentialAxisOutcome {
                axis: CredentialCertificationAxis::DegradedState,
                state: CredentialAxisCertificationState::DisclosedNarrowed,
                parity_note: "delegated identity lagging".to_owned(),
                narrowing_reason: Some(
                    "the forwarded/delegated identity was not restated for this export".to_owned(),
                ),
                downgrade_trigger: Some(M5CredentialDowngradeTrigger::DelegatedIdentityUnstated),
            };
        }
    }
    // Claim stays HandleReadyProjection == certified HandleReadyProjection, no auto-narrow.
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
}

#[test]
fn cli_export_drop_blocks_the_surface() {
    let mut p = packet();
    let row = &mut p.rows[0];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == CredentialCertificationAxis::CliExport {
            outcome.state = CredentialAxisCertificationState::DisclosedNarrowed;
            outcome.narrowing_reason =
                Some("export parity not current for this surface".to_owned());
            outcome.downgrade_trigger = Some(M5CredentialDowngradeTrigger::ProofStale);
        }
    }
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
}

#[test]
fn incomplete_copy_export_blocks_the_surface() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.export_parity.formats.retain(|f| f != "markdown");
    assert!(!row.export_parity.is_complete());
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
}

#[test]
fn dropped_credential_truth_blocks_the_surface() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.credential_truth_preserved = false;
    assert!(!row.preserves_credential_truth_continuity());
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialCertificationViolation::CredentialTruthDropped { .. }
    )));
}

#[test]
fn narrowed_row_dropping_credential_truth_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.derived_status == CredentialSurfaceClaimStatus::Yellow)
        .expect("a yellow row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.preserves_credential_truth_continuity = false;
    }
    assert!(!row.preserves_credential_truth_continuity());
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
}

#[test]
fn spurious_claim_auto_narrow_without_claim_reduction_blocks() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.claim_auto_narrow = Some(CredentialClaimAutoNarrow {
        binding_axis: CredentialCertificationAxis::DegradedState,
        from_claim: M5CredentialComponentClaim::VerifiedBrokered,
        to_claim: M5CredentialComponentClaim::VerifiedBrokered,
        visible_label: "a spurious narrowing that does not reduce the claim".to_owned(),
        preserves_credential_truth_continuity: true,
    });
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
}

#[test]
fn claim_narrowed_without_disclosure_blocks() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.certified_claim = M5CredentialComponentClaim::RevealBlockedProjection;
    row.claim_auto_narrow = None;
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
}

#[test]
fn certified_claim_above_claim_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.surface == M5CredentialCertifiedSurface::SupportExport)
        .expect("support-export row exists");
    let row = &mut p.rows[idx];
    // claimed is HandleReadyProjection
    row.certified_claim = M5CredentialComponentClaim::VerifiedBrokered;
    assert!(row.certified_claim.capability_rank() > row.claimed_claim.capability_rank());
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialCertificationViolation::CertifiedClaimExceedsClaim { .. }
    )));
}

#[test]
fn claim_auto_narrow_bound_to_wrong_axis_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.surface == M5CredentialCertifiedSurface::DatabaseCredentialAttach)
        .expect("database-credential-attach row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.binding_axis = CredentialCertificationAxis::Visual;
    }
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
}

#[test]
fn claim_auto_narrow_bound_to_always_on_axis_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.surface == M5CredentialCertifiedSurface::DatabaseCredentialAttach)
        .expect("database-credential-attach row exists");
    let row = &mut p.rows[idx];
    // Force the always-on CLI/export axis to be the narrowed + binding one.
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == CredentialCertificationAxis::CliExport {
            outcome.state = CredentialAxisCertificationState::DisclosedNarrowed;
            outcome.narrowing_reason =
                Some("export parity is not current for the database attach".to_owned());
            outcome.downgrade_trigger = Some(M5CredentialDowngradeTrigger::ProofStale);
        }
    }
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.binding_axis = CredentialCertificationAxis::CliExport;
    }
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
}

#[test]
fn generic_narrow_label_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.derived_status == CredentialSurfaceClaimStatus::Yellow)
        .expect("a yellow row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.visible_label = "unverified".to_owned();
    }
    assert_eq!(row.derive_status(), CredentialSurfaceClaimStatus::Red);
}

#[test]
fn certified_axis_carrying_a_reason_is_malformed() {
    let mut o = seed_certified(CredentialCertificationAxis::Visual);
    o.narrowing_reason = Some("should not be here".to_owned());
    assert!(!o.well_formed());
}

#[test]
fn disclosed_axis_missing_trigger_is_malformed() {
    let mut o = seed_narrowed(
        CredentialCertificationAxis::DegradedState,
        "note",
        "a genuine narrowing reason",
        M5CredentialDowngradeTrigger::RevealPostureUnstated,
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
    p.rows
        .retain(|r| r.surface != M5CredentialCertifiedSurface::SupportExport);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialCertificationViolation::SurfaceCoverageIncomplete
    )));
}

#[test]
fn missing_family_coverage_is_rejected() {
    // Strip the BrowserDeviceCodeHandoffCard family from every row that carries it; coverage must
    // fail (only the connector-authorization row consumes it).
    let mut p = packet();
    for row in &mut p.rows {
        row.consumed_families
            .retain(|f| *f != M5CredentialComponentFamily::BrowserDeviceCodeHandoffCard);
        row.derived_status = row.derive_status();
    }
    p.summary = p.computed_summary();
    assert!(!p.all_families_covered());
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialCertificationViolation::FamilyCoverageIncomplete
    )));
}

#[test]
fn stale_derived_status_is_rejected() {
    let mut p = packet();
    p.rows[0].derived_status = CredentialSurfaceClaimStatus::Yellow; // it is really Green
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialCertificationViolation::StatusDerivationStale { .. }
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
        CredentialCertificationViolation::RowMissingCanonicalBundle { .. }
    )));
}

#[test]
fn packet_level_wrong_bundle_is_rejected() {
    let mut p = packet();
    p.canonical_bundle_ref = "artifacts/release/other/packet.json".to_owned();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, CredentialCertificationViolation::WrongCanonicalBundle)));
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
        .any(|v| matches!(v, CredentialCertificationViolation::DuplicateId { .. })));
}

#[test]
fn axis_coverage_gap_is_rejected() {
    let mut p = packet();
    p.rows[0]
        .axis_outcomes
        .retain(|o| o.axis != CredentialCertificationAxis::CredentialBoundaryProvenance);
    p.rows[0].derived_status = p.rows[0].derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        CredentialCertificationViolation::AxisCoverageIncomplete { .. }
    )));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut p = packet();
    p.summary.row_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, CredentialCertificationViolation::SummaryMismatch)));
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
        CredentialCertificationViolation::RawCredentialMaterialInExport
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
    let back: CredentialSurfaceCertificationPacket =
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
    assert!(lines[0].starts_with("row_id,surface,claimed_claim,certified_claim,status"));
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
    let on_disk = current_m5_credential_component_certification_export().expect("export is valid");
    assert_eq!(
        on_disk.export_safe_json(),
        packet().export_safe_json(),
        "checked-in certification export drifted from the seeded builder; regenerate the artifact"
    );
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_CREDENTIAL_CERT_ARTIFACTS=1 cargo test -p aureline-provider \
///  certify_credential_component_truth...::tests::generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_CREDENTIAL_CERT_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_credential_component_certification_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art =
        Path::new(manifest).join("../../artifacts/release/m5-credential-component-certification");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-credential-component-certification");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
