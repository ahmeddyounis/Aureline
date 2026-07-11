//! Tests for the M05-1091 notebook document / kernel / output / trust / recovery
//! component profile certification capstone.

use super::*;

fn packet() -> NotebookProfileCertificationPacket {
    seeded_m5_notebook_kernel_output_component_certification_packet()
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
    assert_eq!(p.record_kind, NOTEBOOK_CERT_RECORD_KIND);
    assert_eq!(p.schema_version, NOTEBOOK_CERT_SCHEMA_VERSION);
}

#[test]
fn every_claimed_profile_is_certified_exactly_once() {
    let p = packet();
    assert!(p.all_profiles_present());
    for profile in M5NotebookCertifiedProfile::ALL {
        let count = p.rows.iter().filter(|r| r.profile == profile).count();
        assert_eq!(count, 1, "profile {profile:?} certified {count} times");
    }
    assert_eq!(
        p.summary.profile_count,
        M5NotebookCertifiedProfile::ALL.len()
    );
}

#[test]
fn every_frozen_family_is_certified_on_some_profile() {
    let p = packet();
    assert!(p.all_families_covered());
    assert!(p.summary.all_families_covered);
    let families = p.represented_families();
    for family in M5NotebookKernelOutputComponentFamily::ALL {
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
fn every_row_scores_every_axis_exactly_once() {
    let p = packet();
    for row in &p.rows {
        assert!(row.covers_all_axes(), "row {} misses an axis", row.row_id);
        assert_eq!(
            row.axis_outcomes.len(),
            NotebookCertificationAxis::ALL.len()
        );
    }
    assert!(p.summary.every_axis_covered_on_every_row);
}

#[test]
fn cli_export_axis_is_certified_on_every_row() {
    let p = packet();
    for row in &p.rows {
        let export = row
            .axis(NotebookCertificationAxis::CliExport)
            .expect("cli axis");
        assert_eq!(export.state, NotebookAxisCertificationState::Certified);
        assert!(row.export_parity.is_complete());
    }
    assert!(p.summary.all_rows_export_parity_certified);
}

#[test]
fn every_row_holds_all_four_guardrails() {
    let p = packet();
    for row in &p.rows {
        assert!(
            row.guardrails.all_held(),
            "row {} breaches a guardrail",
            row.row_id
        );
    }
    assert!(p.summary.all_guardrails_held);
}

#[test]
fn every_row_cites_the_one_canonical_bundle() {
    let p = packet();
    assert_eq!(p.canonical_bundle_ref, NOTEBOOK_CERT_CANONICAL_BUNDLE_REF);
    for row in &p.rows {
        assert_eq!(row.canonical_bundle_ref, NOTEBOOK_CERT_CANONICAL_BUNDLE_REF);
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
fn only_local_profile_certifies_a_live_trusted_claim() {
    for row in &packet().rows {
        if row.certified_claim.asserts_live_trusted_result() {
            assert!(
                row.profile.is_local_first_party(),
                "non-local profile {} certifies a live-trusted claim",
                row.row_id
            );
        }
    }
}

#[test]
fn yellow_rows_narrow_their_claim_and_bind_to_a_narrowed_axis() {
    let p = packet();
    for row in p
        .rows
        .iter()
        .filter(|r| r.derived_status == NotebookProfileClaimStatus::Yellow)
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
        .filter(|r| r.derived_status == NotebookProfileClaimStatus::Green)
    {
        assert_eq!(row.claimed_claim, row.certified_claim);
        assert!(row.claim_auto_narrow.is_none());
        assert!(row.narrowed_axes().is_empty());
    }
}

#[test]
fn a_stale_disconnected_or_kernel_free_profile_never_keeps_a_fresh_live_trusted_claim() {
    // Every narrowed row narrows away from LiveTrustedResult (the only claim that asserts
    // a live, current, fresh trusted result). No profile keeps it while degraded.
    for row in &packet().rows {
        if row.is_claim_narrowed() {
            assert!(
                !row.certified_claim.asserts_live_trusted_result(),
                "narrowed row {} still certifies a fresh live-trusted claim",
                row.row_id
            );
        }
    }
}

#[test]
fn profile_tokens_are_unique() {
    let tokens: BTreeSet<&str> = M5NotebookCertifiedProfile::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), M5NotebookCertifiedProfile::ALL.len());
}

#[test]
fn axis_tokens_are_distinct() {
    let axes: BTreeSet<&str> = NotebookCertificationAxis::ALL
        .iter()
        .map(|a| a.as_str())
        .collect();
    assert_eq!(axes.len(), NotebookCertificationAxis::ALL.len());
}

#[test]
fn only_cli_export_axis_is_always_on() {
    for axis in NotebookCertificationAxis::ALL {
        assert_eq!(
            axis.is_always_on(),
            axis == NotebookCertificationAxis::CliExport
        );
    }
}

#[test]
fn only_local_trusted_kernel_is_first_party() {
    for profile in M5NotebookCertifiedProfile::ALL {
        assert_eq!(
            profile.is_local_first_party(),
            profile == M5NotebookCertifiedProfile::LocalTrustedKernel
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
        .position(|r| r.profile == M5NotebookCertifiedProfile::LocalTrustedKernel)
        .expect("local-trusted-kernel row exists");
    let row = &mut p.rows[idx];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == NotebookCertificationAxis::Visual {
            outcome.state = NotebookAxisCertificationState::UndisclosedDrift;
            outcome.narrowing_reason = Some("kernel origin silently dropped".to_owned());
            outcome.downgrade_trigger = None;
        }
    }
    row.derived_status = row.derive_status();
    assert_eq!(row.derived_status, NotebookProfileClaimStatus::Red);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, NotebookCertificationViolation::ProfileBlocked { .. })));
}

#[test]
fn guardrail_breach_blocks_the_profile() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5NotebookCertifiedProfile::RestartedKernel)
        .expect("restarted-kernel row exists");
    let row = &mut p.rows[idx];
    row.guardrails.recovery_card_implies_rerun = true;
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, NotebookCertificationViolation::GuardrailViolated { .. })));
}

#[test]
fn non_local_profile_claiming_live_truth_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5NotebookCertifiedProfile::RemoteIsolatedKernel)
        .expect("remote-isolated-kernel row exists");
    let row = &mut p.rows[idx];
    // Claim a live-trusted result on a non-local profile.
    row.claimed_claim = M5NotebookComponentClaim::LiveTrustedResult;
    row.certified_claim = M5NotebookComponentClaim::LiveTrustedResult;
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookCertificationViolation::NonLocalProfileClaimsLiveTruth { .. }
    )));
}

#[test]
fn degraded_axis_without_claim_narrowing_blocks() {
    // A disclosed-narrowed axis but the claim stays full => hidden overclaim.
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5NotebookCertifiedProfile::ManagedKernel)
        .expect("managed-kernel row exists");
    let row = &mut p.rows[idx];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == NotebookCertificationAxis::DegradedState {
            *outcome = NotebookAxisOutcome {
                axis: NotebookCertificationAxis::DegradedState,
                state: NotebookAxisCertificationState::DisclosedNarrowed,
                parity_note: "the kernel reading is lagging".to_owned(),
                narrowing_reason: Some("the live proof is partial for this profile".to_owned()),
                downgrade_trigger: Some(M5NotebookKernelOutputDowngradeTrigger::ProofStale),
            };
        }
    }
    // Claim stays ReviewableResult == certified ReviewableResult, no claim_auto_narrow.
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
}

#[test]
fn cli_export_drop_blocks_the_profile() {
    let mut p = packet();
    let row = &mut p.rows[0];
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == NotebookCertificationAxis::CliExport {
            outcome.state = NotebookAxisCertificationState::DisclosedNarrowed;
            outcome.narrowing_reason =
                Some("export parity not current for this profile".to_owned());
            outcome.downgrade_trigger = Some(M5NotebookKernelOutputDowngradeTrigger::ProofStale);
        }
    }
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
}

#[test]
fn incomplete_copy_export_blocks_the_profile() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.export_parity.formats.retain(|f| f != "markdown");
    assert!(!row.export_parity.is_complete());
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
}

#[test]
fn spurious_claim_auto_narrow_without_claim_reduction_blocks() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.claim_auto_narrow = Some(NotebookClaimAutoNarrow {
        binding_axis: NotebookCertificationAxis::DegradedState,
        from_claim: M5NotebookComponentClaim::LiveTrustedResult,
        to_claim: M5NotebookComponentClaim::LiveTrustedResult,
        visible_label: "a spurious narrowing that does not reduce the claim".to_owned(),
    });
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
}

#[test]
fn claim_narrowed_without_disclosure_blocks() {
    let mut p = packet();
    let row = &mut p.rows[0];
    row.certified_claim = M5NotebookComponentClaim::StaleOutputProjection;
    row.claim_auto_narrow = None;
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
}

#[test]
fn certified_claim_above_claim_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5NotebookCertifiedProfile::ManagedKernel)
        .expect("managed-kernel row exists");
    let row = &mut p.rows[idx];
    row.certified_claim = M5NotebookComponentClaim::LiveTrustedResult; // claimed is ReviewableResult
    assert!(row.certified_claim.capability_rank() > row.claimed_claim.capability_rank());
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
    row.derived_status = row.derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookCertificationViolation::CertifiedClaimExceedsClaim { .. }
    )));
}

#[test]
fn claim_auto_narrow_bound_to_wrong_axis_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5NotebookCertifiedProfile::StaleOutput)
        .expect("stale-output row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.binding_axis = NotebookCertificationAxis::Visual;
    }
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
}

#[test]
fn claim_auto_narrow_bound_to_always_on_axis_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.profile == M5NotebookCertifiedProfile::StaleOutput)
        .expect("stale-output row exists");
    let row = &mut p.rows[idx];
    // Force the always-on CLI/export axis to be the narrowed + binding one.
    for outcome in &mut row.axis_outcomes {
        if outcome.axis == NotebookCertificationAxis::CliExport {
            outcome.state = NotebookAxisCertificationState::DisclosedNarrowed;
            outcome.narrowing_reason =
                Some("export parity is not current for the output".to_owned());
            outcome.downgrade_trigger = Some(M5NotebookKernelOutputDowngradeTrigger::ProofStale);
        }
    }
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.binding_axis = NotebookCertificationAxis::CliExport;
    }
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
}

#[test]
fn generic_narrow_label_blocks() {
    let mut p = packet();
    let idx = p
        .rows
        .iter()
        .position(|r| r.derived_status == NotebookProfileClaimStatus::Yellow)
        .expect("a yellow row exists");
    let row = &mut p.rows[idx];
    if let Some(narrow) = row.claim_auto_narrow.as_mut() {
        narrow.visible_label = "stale".to_owned();
    }
    assert_eq!(row.derive_status(), NotebookProfileClaimStatus::Red);
}

#[test]
fn certified_axis_carrying_a_reason_is_malformed() {
    let mut o = seed_certified(NotebookCertificationAxis::Visual);
    o.narrowing_reason = Some("should not be here".to_owned());
    assert!(!o.well_formed());
}

#[test]
fn disclosed_axis_missing_trigger_is_malformed() {
    let mut o = seed_narrowed(
        NotebookCertificationAxis::DegradedState,
        "note",
        "a genuine narrowing reason",
        M5NotebookKernelOutputDowngradeTrigger::ProofStale,
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
        .retain(|r| r.profile != M5NotebookCertifiedProfile::DisconnectedReconnectingKernel);
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, NotebookCertificationViolation::ProfileCoverageIncomplete)));
}

#[test]
fn missing_family_coverage_is_rejected() {
    // Strip the kernel-recovery-card family from every row that carries it; coverage must
    // fail.
    let mut p = packet();
    for row in &mut p.rows {
        row.consumed_families
            .retain(|f| *f != M5NotebookKernelOutputComponentFamily::KernelRecoveryCard);
        row.derived_status = row.derive_status();
    }
    p.summary = p.computed_summary();
    assert!(!p.all_families_covered());
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, NotebookCertificationViolation::FamilyCoverageIncomplete)));
}

#[test]
fn stale_derived_status_is_rejected() {
    let mut p = packet();
    p.rows[0].derived_status = NotebookProfileClaimStatus::Yellow; // it is really Green
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookCertificationViolation::StatusDerivationStale { .. }
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
        NotebookCertificationViolation::RowMissingCanonicalBundle { .. }
    )));
}

#[test]
fn packet_level_wrong_bundle_is_rejected() {
    let mut p = packet();
    p.canonical_bundle_ref = "artifacts/release/other/packet.json".to_owned();
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, NotebookCertificationViolation::WrongCanonicalBundle)));
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
        .any(|v| matches!(v, NotebookCertificationViolation::DuplicateId { .. })));
}

#[test]
fn axis_coverage_gap_is_rejected() {
    let mut p = packet();
    p.rows[0]
        .axis_outcomes
        .retain(|o| o.axis != NotebookCertificationAxis::NotebookTruth);
    p.rows[0].derived_status = p.rows[0].derive_status();
    p.summary = p.computed_summary();
    let violations = p.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        NotebookCertificationViolation::AxisCoverageIncomplete { .. }
    )));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut p = packet();
    p.summary.row_count += 1;
    let violations = p.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, NotebookCertificationViolation::SummaryMismatch)));
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
        NotebookCertificationViolation::RawNotebookMaterialInExport
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
    let back: NotebookProfileCertificationPacket =
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

#[test]
fn checked_in_export_matches_seeded_builder() {
    let on_disk = current_m5_notebook_kernel_output_component_certification_export()
        .expect("export is valid");
    assert_eq!(
        on_disk.export_safe_json(),
        packet().export_safe_json(),
        "checked-in certification export drifted from the seeded builder; regenerate the artifact"
    );
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it
/// never runs in the normal suite. Run with
/// `GEN_NOTEBOOK_KERNEL_OUTPUT_CERT_ARTIFACTS=1 cargo test -p aureline-notebook --lib \
///  certify_notebook_document_kernel...::tests::generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_NOTEBOOK_KERNEL_OUTPUT_CERT_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_notebook_kernel_output_component_certification_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-notebook-kernel-output-component-certification-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-notebook-kernel-output-component-certification");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 notebook document/kernel/output component profile certification fixtures\n\n\
         Mirror of `artifacts/release/m5-notebook-kernel-output-component-certification-proof/`.\n\
         Regenerate with `GEN_NOTEBOOK_KERNEL_OUTPUT_CERT_ARTIFACTS=1 cargo test -p aureline-notebook \
         --lib certify_notebook_document_kernel...::tests::generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
