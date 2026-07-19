//! Tests for the M05-850 workflow-bundle component accessibility fallback capstone:
//! the honest auto-narrowing logic, the per-family parity contract, and the
//! checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> BundleAccessibilityRow {
    seeded_m5_bundle_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_bundle_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_bundle_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5WorkflowBundleComponentFamily::ALL.len()
    );
    assert_eq!(
        packet.rows.len(),
        M5WorkflowBundleComponentFamily::ALL.len()
    );
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_bundle_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5BundleClaimDimension::ALL.len()
    );
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_bundle_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5BundleSupportClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_bundle_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5BundleDisclosureSurfaceFamily::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_seven_yellow_zero_red() {
    let packet = seeded_m5_bundle_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 7);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

// --- AC1: a stale / partial bundle can no longer present as fully certified ---

#[test]
fn intact_start_center_card_is_certified_and_green() {
    let card = row("a11y:start-center-bundle-card");
    assert_eq!(card.full_support_claim, M5BundleSupportClaim::Certified);
    assert_eq!(card.effective_claim(), M5BundleSupportClaim::Certified);
    assert!(card.claim_narrow.is_none());
    assert_eq!(card.status(), BundleAccessibilityStatus::Parity);
    assert!(card.effective_claim().asserts_full_certification());
}

#[test]
fn stale_certification_narrows_to_retest_pending() {
    let badges = row("a11y:certified-archetype-badge-group");
    assert_eq!(
        badges.effective_claim(),
        M5BundleSupportClaim::RetestPending
    );
    assert!(!badges.effective_claim().asserts_full_certification());
    let narrow = badges.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5BundleComponentDowngradeTrigger::StaleCertification
    );
    assert_eq!(
        narrow.binding_dimension,
        M5BundleClaimDimension::CertificationEvidence
    );
    assert!(badges.claim_is_honest());
}

#[test]
fn policy_blocked_dependency_narrows_to_policy_blocked() {
    let detail = row("a11y:bundle-detail-page");
    assert_eq!(
        detail.effective_claim(),
        M5BundleSupportClaim::PolicyBlocked
    );
    assert!(!detail.effective_claim().asserts_full_self_sufficiency());
    assert!(detail.claim_is_honest());
}

#[test]
fn imported_provenance_never_reads_as_native_certified() {
    let card = row("a11y:bundle-class-disclosure-card");
    assert_eq!(card.effective_claim(), M5BundleSupportClaim::Imported);
    assert!(!card.effective_claim().asserts_full_certification());
    assert!(!card.effective_claim().asserts_full_self_sufficiency());
}

#[test]
fn mirror_and_offline_artifact_availability_narrow_distinctly() {
    assert_eq!(
        row("a11y:bundle-local-override-row").effective_claim(),
        M5BundleSupportClaim::MirrorOnly
    );
    assert_eq!(
        row("a11y:bundle-rollback-remove-card").effective_claim(),
        M5BundleSupportClaim::OfflineCacheOnly
    );
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a stale bundle claiming
    // Certified.
    let mut card = row("a11y:certified-archetype-badge-group");
    card.claim_narrow = None;
    assert!(!card.claim_is_honest());
    assert_eq!(card.status(), BundleAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_intact_row_is_rejected() {
    let mut card = row("a11y:start-center-bundle-card");
    card.claim_narrow = Some(BundleClaimAutoNarrow {
        narrowed_to: M5BundleSupportClaim::Limited,
        binding_dimension: M5BundleClaimDimension::CertificationEvidence,
        trigger: M5BundleComponentDowngradeTrigger::StaleCertification,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
    });
    assert!(!card.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut badges = row("a11y:certified-archetype-badge-group");
    if let Some(narrow) = badges.claim_narrow.as_mut() {
        narrow.binding_dimension = M5BundleClaimDimension::DependencyPosture;
    }
    assert!(!badges.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut badges = row("a11y:certified-archetype-badge-group");
    if let Some(narrow) = badges.claim_narrow.as_mut() {
        narrow.narrowed_label = "degraded".to_owned();
    }
    assert!(!badges.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5BundleClaimConditionState as C;
    use M5BundleSupportClaim as S;
    assert_eq!(C::Intact.permitted_ceiling(), S::Certified);
    assert_eq!(C::Partial.permitted_ceiling(), S::Limited);
    assert_eq!(C::Stale.permitted_ceiling(), S::RetestPending);
    assert_eq!(C::Imported.permitted_ceiling(), S::Imported);
    assert_eq!(C::MirrorStale.permitted_ceiling(), S::MirrorOnly);
    assert_eq!(C::OfflineOnly.permitted_ceiling(), S::OfflineCacheOnly);
    assert_eq!(C::PolicyBlocked.permitted_ceiling(), S::PolicyBlocked);
}

// --- AC2: accessibility / CLI / export reach the same canonical truth ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_bundle_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_detail_page_binds_a_non_visual_fallback() {
    let detail = row("a11y:bundle-detail-page");
    assert!(detail.is_hierarchy_heavy());
    assert!(detail.has_non_visual_fallback());
    assert!(detail
        .fallback_modalities
        .contains(&M5BundleFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut card = row("a11y:start-center-bundle-card");
    card.keyboard_reach = BundleNonVisualReachState::ViewOnlyTrap;
    assert!(!card.reaches_canonical_truth_via_at());
    assert_eq!(card.status(), BundleAccessibilityStatus::Stranded);
}

#[test]
fn empty_bundle_context_ref_strands_a_row() {
    let mut card = row("a11y:start-center-bundle-card");
    card.bundle_context_ref = "  ".to_owned();
    assert!(!card.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut card = row("a11y:start-center-bundle-card");
    card.export_summary = BundleExportSummaryState::AbsentNeedsScreenshot;
    assert!(!card.export_preserves_meaning());
    assert_eq!(card.status(), BundleAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut card = row("a11y:start-center-bundle-card");
    card.copy_export.formats.retain(|f| f != "markdown");
    assert!(!card.export_preserves_meaning());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut card = row("a11y:certified-archetype-badge-group");
    card.narrowing_disclosures.clear();
    assert!(!card.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut card = row("a11y:certified-archetype-badge-group");
    card.narrowing_disclosures[0].state = BundleNarrowingDisclosureState::SilentlyDropped;
    assert!(!card.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut card = row("a11y:certified-archetype-badge-group");
    card.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!card.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut card = row("a11y:start-center-bundle-card");
    card.required_labels
        .retain(|l| *l != M5BundleRequiredLabel::BundleIdentity);
    assert!(!card.preserves_mandatory_labels());
    assert_eq!(card.status(), BundleAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_bundle_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:bundle-detail-page");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        BundleAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_bundle_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces =
        vec![M5BundleDisclosureSurfaceFamily::DiagnosticsClassReport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        BundleAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_bundle_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BundleAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_bundle_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BundleAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_bundle_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BundleAccessibilityViolation::RawBoundaryMaterialInExport)));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:certified-archetype-badge-group").chip_tokens();
    assert!(chip.contains("family=certified_archetype_badge_group"));
    assert!(chip.contains("effective_claim=retest_pending"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_bundle_a11y_fallback_packet();
    assert_eq!(packet.record_kind, BUNDLE_A11Y_FALLBACK_RECORD_KIND);
    assert_eq!(packet.schema_version, BUNDLE_A11Y_FALLBACK_SCHEMA_VERSION);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_bundle_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_bundle_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_bundle_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_bundle_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_bundle_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workflow-bundle-component-accessibility-fallback-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_bundle_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workflow-bundle-component-accessibility-fallback-proof/report.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var
/// so it never runs in the normal suite. Run with
/// `GEN_BUNDLE_A11Y_ARTIFACTS=1 cargo test -p aureline-workspace generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_BUNDLE_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_bundle_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-workflow-bundle-component-accessibility-fallback-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-workflow-bundle-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 workflow-bundle component accessibility fallback fixtures\n\n\
         Mirror of `artifacts/release/m5-workflow-bundle-component-accessibility-fallback-proof/`.\n\
         Regenerate with `GEN_BUNDLE_A11Y_ARTIFACTS=1 cargo test -p aureline-workspace generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
