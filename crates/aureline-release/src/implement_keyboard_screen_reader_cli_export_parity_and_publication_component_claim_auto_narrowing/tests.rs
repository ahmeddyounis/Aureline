//! Tests for the M05-866 publication-component accessibility fallback capstone:
//! the honest auto-narrowing logic, the per-family parity contract, and the
//! checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> PublicationAccessibilityRow {
    seeded_m5_publication_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_publication_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_publication_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5ReleaseCenterComponentFamily::ALL.len()
    );
    assert_eq!(packet.rows.len(), M5ReleaseCenterComponentFamily::ALL.len());
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_publication_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5PublicationClaimDimension::ALL.len()
    );
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_publication_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5PublicationSupportClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_publication_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5ReleaseCenterConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_four_yellow_zero_red() {
    let packet = seeded_m5_publication_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

// --- AC2: a stale / partial publication can no longer keep an old Certified / Supported label ---

#[test]
fn verified_candidate_card_is_certified_and_green() {
    let card = row("a11y:release-candidate-card");
    assert_eq!(
        card.full_support_claim,
        M5PublicationSupportClaim::Certified
    );
    assert_eq!(card.effective_claim(), M5PublicationSupportClaim::Certified);
    assert!(card.claim_narrow.is_none());
    assert_eq!(card.status(), PublicationAccessibilityStatus::Parity);
    assert!(card.effective_claim().asserts_certified());
}

#[test]
fn verified_version_bump_row_is_supported_and_green() {
    let bump = row("a11y:version-bump-row");
    assert_eq!(bump.effective_claim(), M5PublicationSupportClaim::Supported);
    assert!(bump.claim_narrow.is_none());
    assert_eq!(bump.status(), PublicationAccessibilityStatus::Parity);
    assert!(bump.effective_claim().asserts_full_self_sufficiency());
    assert!(!bump.effective_claim().asserts_certified());
}

#[test]
fn partial_auth_narrows_publish_target_to_degraded() {
    let target = row("a11y:publish-target-row");
    assert_eq!(
        target.effective_claim(),
        M5PublicationSupportClaim::Degraded
    );
    assert!(!target.effective_claim().asserts_full_self_sufficiency());
    let narrow = target.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ReleaseCenterDowngradeTrigger::TargetAuthSourceMasked
    );
    assert_eq!(
        narrow.binding_dimension,
        M5PublicationClaimDimension::TargetAuthPosture
    );
    assert!(target.claim_is_honest());
}

#[test]
fn unverified_provenance_narrows_to_unverified() {
    let card = row("a11y:artifact-provenance-bundle-card");
    assert_eq!(
        card.effective_claim(),
        M5PublicationSupportClaim::Unverified
    );
    assert!(!card.effective_claim().asserts_certified());
    assert!(!card.effective_claim().asserts_full_self_sufficiency());
    let narrow = card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ReleaseCenterDowngradeTrigger::SignatureOrAttestationOverclaimed
    );
    assert!(card.claim_is_honest());
}

#[test]
fn stale_mirror_narrows_promotion_to_provisional() {
    let step = row("a11y:promotion-timeline-step");
    assert_eq!(
        step.effective_claim(),
        M5PublicationSupportClaim::Provisional
    );
    assert!(!step.effective_claim().asserts_certified());
    assert!(step.claim_is_honest());
}

#[test]
fn policy_blocked_rollback_narrows_to_policy_blocked() {
    let rollback = row("a11y:rollback-revocation-row");
    assert_eq!(
        rollback.effective_claim(),
        M5PublicationSupportClaim::PolicyBlocked
    );
    assert!(!rollback.effective_claim().asserts_full_self_sufficiency());
    assert!(rollback.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: an unverified provenance
    // card claiming Certified.
    let mut card = row("a11y:artifact-provenance-bundle-card");
    card.claim_narrow = None;
    assert!(!card.claim_is_honest());
    assert_eq!(card.status(), PublicationAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_verified_row_is_rejected() {
    let mut card = row("a11y:release-candidate-card");
    card.claim_narrow = Some(PublicationClaimAutoNarrow {
        narrowed_to: M5PublicationSupportClaim::Degraded,
        binding_dimension: M5PublicationClaimDimension::EvidenceFreshness,
        trigger: M5ReleaseCenterDowngradeTrigger::BlockerFreshnessHidden,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
    });
    assert!(!card.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut target = row("a11y:publish-target-row");
    if let Some(narrow) = target.claim_narrow.as_mut() {
        narrow.binding_dimension = M5PublicationClaimDimension::EvidenceFreshness;
    }
    assert!(!target.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut target = row("a11y:publish-target-row");
    if let Some(narrow) = target.claim_narrow.as_mut() {
        narrow.trigger = M5ReleaseCenterDowngradeTrigger::BlockerFreshnessHidden;
    }
    assert!(!target.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut target = row("a11y:publish-target-row");
    if let Some(narrow) = target.claim_narrow.as_mut() {
        narrow.narrowed_label = "degraded".to_owned();
    }
    assert!(!target.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5PublicationConditionState as C;
    use M5PublicationSupportClaim as S;
    assert_eq!(C::Verified.permitted_ceiling(), S::Certified);
    assert_eq!(C::Partial.permitted_ceiling(), S::Degraded);
    assert_eq!(C::Stale.permitted_ceiling(), S::Provisional);
    assert_eq!(C::Unverified.permitted_ceiling(), S::Unverified);
    assert_eq!(C::PolicyBlocked.permitted_ceiling(), S::PolicyBlocked);
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5PublicationClaimDimension as D;
    use M5ReleaseCenterDowngradeTrigger as T;
    assert_eq!(
        D::EvidenceFreshness.default_trigger(),
        T::BlockerFreshnessHidden
    );
    assert_eq!(
        D::PublicSurfaceImpact.default_trigger(),
        T::VersionBumpImpactUnstated
    );
    assert_eq!(
        D::TargetAuthPosture.default_trigger(),
        T::TargetAuthSourceMasked
    );
    assert_eq!(
        D::SignatureAttestationState.default_trigger(),
        T::SignatureOrAttestationOverclaimed
    );
    assert_eq!(D::MirrorVerification.default_trigger(), T::ProofStale);
    assert_eq!(
        D::RollbackBlastRadius.default_trigger(),
        T::RollbackBlastRadiusUnderstated
    );
}

// --- AC1: accessibility / CLI / export reach the same canonical truth ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_publication_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_provenance_row_binds_a_non_visual_fallback() {
    let card = row("a11y:artifact-provenance-bundle-card");
    assert!(card.is_hierarchy_heavy());
    assert!(card.has_non_visual_fallback());
    assert!(card
        .fallback_modalities
        .contains(&M5PublicationFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut card = row("a11y:release-candidate-card");
    card.keyboard_reach = PublicationNonVisualReachState::ViewOnlyTrap;
    assert!(!card.reaches_canonical_truth_via_at());
    assert_eq!(card.status(), PublicationAccessibilityStatus::Stranded);
}

#[test]
fn empty_publication_context_ref_strands_a_row() {
    let mut card = row("a11y:release-candidate-card");
    card.publication_context_ref = "  ".to_owned();
    assert!(!card.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut card = row("a11y:release-candidate-card");
    card.export_summary = PublicationExportSummaryState::AbsentNeedsScreenshot;
    assert!(!card.export_preserves_meaning());
    assert_eq!(card.status(), PublicationAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut card = row("a11y:release-candidate-card");
    card.copy_export.formats.retain(|f| f != "markdown");
    assert!(!card.export_preserves_meaning());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut target = row("a11y:publish-target-row");
    target.narrowing_disclosures.clear();
    assert!(!target.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut target = row("a11y:publish-target-row");
    target.narrowing_disclosures[0].state = PublicationNarrowingDisclosureState::SilentlyDropped;
    assert!(!target.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut target = row("a11y:publish-target-row");
    target.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!target.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut card = row("a11y:release-candidate-card");
    card.required_labels
        .retain(|l| *l != M5ReleaseCenterRequiredLabel::Identity);
    assert!(!card.preserves_mandatory_labels());
    assert_eq!(card.status(), PublicationAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_publication_a11y_fallback_packet();
    packet.rows.retain(|r| r.row_id != "a11y:version-bump-row");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PublicationAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_publication_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5ReleaseCenterConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PublicationAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_publication_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PublicationAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_publication_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PublicationAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_publication_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PublicationAccessibilityViolation::RawReleaseMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:artifact-provenance-bundle-card").chip_tokens();
    assert!(chip.contains("family=artifact_provenance_bundle_card"));
    assert!(chip.contains("effective_claim=unverified"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_publication_a11y_fallback_packet();
    assert_eq!(packet.record_kind, PUBLICATION_A11Y_FALLBACK_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        PUBLICATION_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_publication_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_publication_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_publication_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_publication_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_publication_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-publication-component-accessibility-fallback-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_publication_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-publication-component-accessibility-fallback-proof/report.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so
/// it never runs in the normal suite. Run with
/// `GEN_PUBLICATION_A11Y_ARTIFACTS=1 cargo test -p aureline-release generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_PUBLICATION_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_publication_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-publication-component-accessibility-fallback-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-publication-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 publication-component accessibility fallback fixtures\n\n\
         Mirror of `artifacts/release/m5-publication-component-accessibility-fallback-proof/`.\n\
         Regenerate with `GEN_PUBLICATION_A11Y_ARTIFACTS=1 cargo test -p aureline-release generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
