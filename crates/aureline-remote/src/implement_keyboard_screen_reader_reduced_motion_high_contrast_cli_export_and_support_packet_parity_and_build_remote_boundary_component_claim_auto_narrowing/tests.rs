//! Tests for the M05-1081 build/remote-boundary component accessibility parity capstone: the
//! honest auto-narrowing logic, the per-family parity contract, and the checked-in support export
//! / CSV / report.

use super::*;

fn row(id: &str) -> BuildRemoteBoundaryAccessibilityRow {
    seeded_m5_build_remote_boundary_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_build_remote_boundary_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_build_remote_boundary_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5BuildRemoteBoundaryComponentFamily::ALL.len()
    );
    assert_eq!(
        packet.rows.len(),
        M5BuildRemoteBoundaryComponentFamily::ALL.len()
    );
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_build_remote_boundary_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5BuildRemoteClaimDimension::ALL.len()
    );
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_build_remote_boundary_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5BuildRemoteAccessClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_build_remote_boundary_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5_BUILD_REMOTE_BOUNDARY_A11Y_CONSUMER_SURFACES {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_six_yellow_zero_red() {
    let packet = seeded_m5_build_remote_boundary_a11y_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 6);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

// --- AC1: a stale / unverified / unsupported / partial boundary state can no longer keep an old
// fresh first-party full-truth label ---

#[test]
fn intact_host_boundary_strip_is_full_truth_and_green() {
    let strip = row("a11y:host-boundary-strip");
    assert_eq!(
        strip.full_support_claim,
        M5BuildRemoteAccessClaim::FullTruth
    );
    assert_eq!(strip.effective_claim(), M5BuildRemoteAccessClaim::FullTruth);
    assert!(strip.claim_narrow.is_none());
    assert_eq!(strip.status(), BuildRemoteAccessibilityStatus::Parity);
    assert!(strip.effective_claim().asserts_live_truth());
}

#[test]
fn intact_execution_origin_receipt_is_resolved_and_green() {
    let receipt = row("a11y:execution-origin-receipt-row");
    assert_eq!(
        receipt.effective_claim(),
        M5BuildRemoteAccessClaim::ResolvedTruth
    );
    assert!(receipt.claim_narrow.is_none());
    assert_eq!(receipt.status(), BuildRemoteAccessibilityStatus::Parity);
    assert!(receipt.effective_claim().asserts_full_self_sufficiency());
    assert!(!receipt.effective_claim().asserts_live_truth());
}

#[test]
fn partial_adapter_confidence_chip_narrows_to_degraded() {
    let chip = row("a11y:adapter-confidence-chip");
    assert_eq!(chip.effective_claim(), M5BuildRemoteAccessClaim::Degraded);
    assert!(!chip.effective_claim().asserts_full_self_sufficiency());
    let narrow = chip.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5BuildRemoteDowngradeTrigger::DiscoveryDriftHidden
    );
    assert_eq!(
        narrow.binding_dimension,
        M5BuildRemoteClaimDimension::DiscoveryConfidenceTruth
    );
    assert!(chip.claim_is_honest());
}

#[test]
fn stale_discovery_diff_card_narrows_to_stale() {
    let card = row("a11y:discovery-diff-card");
    assert_eq!(card.effective_claim(), M5BuildRemoteAccessClaim::Stale);
    assert!(!card.effective_claim().asserts_live_truth());
    assert!(card.claim_is_honest());
}

#[test]
fn unverified_review_sheet_narrows_to_unverified() {
    let sheet = row("a11y:suspend-resume-rebuild-review-sheet");
    assert_eq!(
        sheet.effective_claim(),
        M5BuildRemoteAccessClaim::Unverified
    );
    assert!(!sheet.effective_claim().asserts_full_self_sufficiency());
    assert!(sheet.claim_is_honest());
}

#[test]
fn unsupported_local_safe_card_narrows_to_unsupported() {
    let card = row("a11y:local-safe-continuation-card");
    assert_eq!(
        card.effective_claim(),
        M5BuildRemoteAccessClaim::Unsupported
    );
    assert!(!card.effective_claim().asserts_full_self_sufficiency());
    assert!(card.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a partial adapter-confidence chip
    // claiming full truth.
    let mut chip = row("a11y:adapter-confidence-chip");
    chip.claim_narrow = None;
    assert!(!chip.claim_is_honest());
    assert_eq!(chip.status(), BuildRemoteAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_intact_row_is_rejected() {
    let mut strip = row("a11y:host-boundary-strip");
    strip.claim_narrow = Some(BuildRemoteClaimAutoNarrow {
        narrowed_to: M5BuildRemoteAccessClaim::Degraded,
        binding_dimension: M5BuildRemoteClaimDimension::HostOwnershipTruth,
        trigger: M5BuildRemoteDowngradeTrigger::HostBoundaryUnstated,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
    });
    assert!(!strip.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut chip = row("a11y:adapter-confidence-chip");
    if let Some(narrow) = chip.claim_narrow.as_mut() {
        narrow.binding_dimension = M5BuildRemoteClaimDimension::HostOwnershipTruth;
    }
    assert!(!chip.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut chip = row("a11y:adapter-confidence-chip");
    if let Some(narrow) = chip.claim_narrow.as_mut() {
        narrow.trigger = M5BuildRemoteDowngradeTrigger::HostBoundaryUnstated;
    }
    assert!(!chip.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut chip = row("a11y:adapter-confidence-chip");
    if let Some(narrow) = chip.claim_narrow.as_mut() {
        narrow.narrowed_label = "degraded".to_owned();
    }
    assert!(!chip.claim_is_honest());
    // Generic unverified / stale wording is also rejected.
    let mut chip2 = chip.clone();
    if let Some(narrow) = chip2.claim_narrow.as_mut() {
        narrow.narrowed_label = "Unverified".to_owned();
    }
    assert!(!chip2.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5BuildRemoteAccessClaim as S;
    use M5BuildRemoteConditionState as C;
    assert_eq!(C::Intact.permitted_ceiling(), S::FullTruth);
    assert_eq!(C::Partial.permitted_ceiling(), S::Degraded);
    assert_eq!(C::Stale.permitted_ceiling(), S::Stale);
    assert_eq!(C::Unverified.permitted_ceiling(), S::Unverified);
    assert_eq!(C::Unsupported.permitted_ceiling(), S::Unsupported);
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5BuildRemoteClaimDimension as D;
    use M5BuildRemoteDowngradeTrigger as T;
    assert_eq!(
        D::DiscoveryConfidenceTruth.default_trigger(),
        T::DiscoveryDriftHidden
    );
    assert_eq!(
        D::HostOwnershipTruth.default_trigger(),
        T::HostBoundaryUnstated
    );
    assert_eq!(
        D::ExecutionOriginTruth.default_trigger(),
        T::ExecutionOriginUnstated
    );
    assert_eq!(
        D::LifecycleStateTruth.default_trigger(),
        T::LifecycleStateUnstated
    );
    assert_eq!(
        D::ExpiryTimingTruth.default_trigger(),
        T::ExpiryTimingUnstated
    );
    assert_eq!(
        D::ContinuityTruth.default_trigger(),
        T::ExactContinuityOverclaimed
    );
}

// --- AC2: accessibility / CLI / export reach the same canonical truth ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_build_remote_boundary_a11y_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_review_sheet_binds_a_non_visual_fallback() {
    let sheet = row("a11y:suspend-resume-rebuild-review-sheet");
    assert!(sheet.is_hierarchy_heavy());
    assert!(sheet.has_non_visual_fallback());
    assert!(sheet
        .fallback_modalities
        .contains(&M5BuildRemoteFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut strip = row("a11y:host-boundary-strip");
    strip.keyboard_reach = BuildRemoteNonVisualReachState::ViewOnlyTrap;
    assert!(!strip.reaches_canonical_truth_via_at());
    assert_eq!(strip.status(), BuildRemoteAccessibilityStatus::Stranded);
}

#[test]
fn empty_boundary_context_ref_strands_a_row() {
    let mut strip = row("a11y:host-boundary-strip");
    strip.boundary_context_ref = "  ".to_owned();
    assert!(!strip.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut strip = row("a11y:host-boundary-strip");
    strip.export_summary = BuildRemoteExportSummaryState::AbsentNeedsScreenshot;
    assert!(!strip.export_preserves_meaning());
    assert_eq!(strip.status(), BuildRemoteAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut strip = row("a11y:host-boundary-strip");
    strip.copy_export.formats.retain(|f| f != "markdown");
    assert!(!strip.export_preserves_meaning());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut chip = row("a11y:adapter-confidence-chip");
    chip.narrowing_disclosures.clear();
    assert!(!chip.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut chip = row("a11y:adapter-confidence-chip");
    chip.narrowing_disclosures[0].state = BuildRemoteNarrowingDisclosureState::SilentlyDropped;
    assert!(!chip.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut chip = row("a11y:adapter-confidence-chip");
    chip.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!chip.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut strip = row("a11y:host-boundary-strip");
    strip
        .required_labels
        .retain(|l| *l != M5BuildRemoteRequiredLabel::Identity);
    assert!(!strip.preserves_mandatory_labels());
    assert_eq!(strip.status(), BuildRemoteAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_build_remote_boundary_a11y_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:adapter-confidence-chip");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        BuildRemoteBoundaryAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_build_remote_boundary_a11y_packet();
    packet.rows[0].consumer_surfaces = vec![M5BuildRemoteConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        BuildRemoteBoundaryAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_build_remote_boundary_a11y_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        BuildRemoteBoundaryAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_build_remote_boundary_a11y_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        BuildRemoteBoundaryAccessibilityViolation::SummaryMismatch
    )));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_build_remote_boundary_a11y_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        BuildRemoteBoundaryAccessibilityViolation::RawRemoteMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:discovery-diff-card").chip_tokens();
    assert!(chip.contains("family=discovery_diff_card"));
    assert!(chip.contains("effective_claim=stale"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_build_remote_boundary_a11y_packet();
    assert_eq!(packet.record_kind, BUILD_REMOTE_BOUNDARY_A11Y_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_build_remote_boundary_a11y_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_build_remote_boundary_a11y_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_build_remote_boundary_a11y_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_build_remote_boundary_a11y_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_build_remote_boundary_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-build-remote-boundary-component-accessibility-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_build_remote_boundary_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-build-remote-boundary-component-accessibility-proof/report.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_BUILD_REMOTE_BOUNDARY_A11Y_ARTIFACTS=1 cargo test -p aureline-remote generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_BUILD_REMOTE_BOUNDARY_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_build_remote_boundary_a11y_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-build-remote-boundary-component-accessibility-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-build-remote-boundary-component-accessibility-parity");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 build/remote-boundary component accessibility parity fixtures\n\n\
         Mirror of `artifacts/release/m5-build-remote-boundary-component-accessibility-proof/`.\n\
         Regenerate with `GEN_BUILD_REMOTE_BOUNDARY_A11Y_ARTIFACTS=1 cargo test -p aureline-remote generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
