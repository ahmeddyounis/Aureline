//! Tests for the M05-946 badge-family accessibility fallback capstone: the honest
//! auto-narrowing logic, the per-family parity contract, and the checked-in support
//! export / CSV / report.

use super::*;

fn row(id: &str) -> BadgeAccessibilityRow {
    seeded_m5_badge_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_badge_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_badge_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5BadgeFamily::ALL.len()
    );
    assert_eq!(packet.rows.len(), M5BadgeFamily::ALL.len());
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_badge_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5BadgeClaimDimension::ALL.len()
    );
}

#[test]
fn every_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_badge_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5BadgeSupportClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_badge_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5BadgeConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_four_yellow_zero_red() {
    let packet = seeded_m5_badge_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

// --- AC2: a stale / limited / imported / policy-blocked badge can no longer keep the
// family's full claim ---

#[test]
fn current_support_class_is_full_and_green() {
    let badge = row("a11y:support-class-badge");
    assert_eq!(badge.full_support_claim, M5BadgeSupportClaim::FullClaim);
    assert_eq!(badge.effective_claim(), M5BadgeSupportClaim::FullClaim);
    assert!(badge.claim_narrow.is_none());
    assert_eq!(badge.status(), BadgeAccessibilityStatus::Parity);
    assert!(badge.effective_claim().asserts_full_claim());
}

#[test]
fn current_lifecycle_is_supported_and_green() {
    let badge = row("a11y:lifecycle-badge");
    assert_eq!(badge.effective_claim(), M5BadgeSupportClaim::Supported);
    assert!(badge.claim_narrow.is_none());
    assert_eq!(badge.status(), BadgeAccessibilityStatus::Parity);
    assert!(badge.effective_claim().asserts_trustworthy_posture());
    assert!(!badge.effective_claim().asserts_full_claim());
}

#[test]
fn stale_freshness_narrows_to_provisional() {
    let badge = row("a11y:evidence-freshness-badge");
    assert_eq!(badge.effective_claim(), M5BadgeSupportClaim::Provisional);
    assert!(!badge.effective_claim().asserts_full_claim());
    let narrow = badge.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5BadgeDowngradeTrigger::EvidenceFreshnessHidden
    );
    assert_eq!(
        narrow.binding_dimension,
        M5BadgeClaimDimension::EvidenceFreshness
    );
    assert!(badge.claim_is_honest());
}

#[test]
fn limited_channel_narrows_to_limited() {
    let badge = row("a11y:channel-badge");
    assert_eq!(badge.effective_claim(), M5BadgeSupportClaim::Limited);
    assert!(!badge.effective_claim().asserts_trustworthy_posture());
    let narrow = badge.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5BadgeDowngradeTrigger::ChannelValueUnstated
    );
    assert!(badge.claim_is_honest());
}

#[test]
fn imported_deployment_scope_narrows_to_imported() {
    let badge = row("a11y:deployment-scope-badge");
    assert_eq!(badge.effective_claim(), M5BadgeSupportClaim::Imported);
    assert!(!badge.effective_claim().asserts_full_claim());
    let narrow = badge.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5BadgeDowngradeTrigger::DeploymentScopeUnstated
    );
    assert!(badge.claim_is_honest());
}

#[test]
fn policy_blocked_compatibility_narrows_to_policy_blocked() {
    let badge = row("a11y:compatibility-state-badge");
    assert_eq!(badge.effective_claim(), M5BadgeSupportClaim::PolicyBlocked);
    assert!(!badge.effective_claim().asserts_trustworthy_posture());
    assert!(badge.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a stale freshness badge
    // claiming its full posture.
    let mut badge = row("a11y:evidence-freshness-badge");
    badge.claim_narrow = None;
    assert!(!badge.claim_is_honest());
    assert_eq!(badge.status(), BadgeAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_current_row_is_rejected() {
    let mut badge = row("a11y:support-class-badge");
    badge.claim_narrow = Some(BadgeClaimAutoNarrow {
        narrowed_to: M5BadgeSupportClaim::Limited,
        binding_dimension: M5BadgeClaimDimension::SupportClassPosture,
        trigger: M5BadgeDowngradeTrigger::SupportClassValueUnstated,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
    });
    assert!(!badge.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut badge = row("a11y:evidence-freshness-badge");
    if let Some(narrow) = badge.claim_narrow.as_mut() {
        narrow.binding_dimension = M5BadgeClaimDimension::SupportClassPosture;
    }
    assert!(!badge.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut badge = row("a11y:evidence-freshness-badge");
    if let Some(narrow) = badge.claim_narrow.as_mut() {
        narrow.trigger = M5BadgeDowngradeTrigger::SupportClassValueUnstated;
    }
    assert!(!badge.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut badge = row("a11y:evidence-freshness-badge");
    if let Some(narrow) = badge.claim_narrow.as_mut() {
        narrow.narrowed_label = "limited".to_owned();
    }
    assert!(!badge.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5BadgeConditionState as C;
    use M5BadgeSupportClaim as S;
    assert_eq!(C::Current.permitted_ceiling(), S::FullClaim);
    assert_eq!(C::Limited.permitted_ceiling(), S::Limited);
    assert_eq!(C::Stale.permitted_ceiling(), S::Provisional);
    assert_eq!(C::Imported.permitted_ceiling(), S::Imported);
    assert_eq!(C::PolicyBlocked.permitted_ceiling(), S::PolicyBlocked);
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5BadgeClaimDimension as D;
    use M5BadgeDowngradeTrigger as T;
    assert_eq!(
        D::SupportClassPosture.default_trigger(),
        T::SupportClassValueUnstated
    );
    assert_eq!(
        D::EvidenceFreshness.default_trigger(),
        T::EvidenceFreshnessHidden
    );
    assert_eq!(
        D::LifecycleStage.default_trigger(),
        T::LifecycleValueUnstated
    );
    assert_eq!(D::ChannelPosture.default_trigger(), T::ChannelValueUnstated);
    assert_eq!(
        D::DeploymentScope.default_trigger(),
        T::DeploymentScopeUnstated
    );
    assert_eq!(
        D::CompatibilityState.default_trigger(),
        T::CompatibilityStateUnstated
    );
}

#[test]
fn certified_never_implies_fresh() {
    // The support-class badge is FullClaim (certified-equivalent) but the freshness badge
    // is an independent axis that has narrowed to provisional — proving Certified does
    // not imply Fresh.
    let support = row("a11y:support-class-badge");
    let freshness = row("a11y:evidence-freshness-badge");
    assert!(support.effective_claim().asserts_full_claim());
    assert_eq!(
        freshness.effective_claim(),
        M5BadgeSupportClaim::Provisional
    );
    assert_ne!(support.badge_family, freshness.badge_family);
}

// --- AC1: accessibility / CLI / export reach the same badge truth ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_badge_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_compatibility_row_binds_a_non_visual_fallback() {
    let badge = row("a11y:compatibility-state-badge");
    assert!(badge.is_hierarchy_heavy());
    assert!(badge.has_non_visual_fallback());
    assert!(badge
        .fallback_modalities
        .contains(&M5BadgeFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut badge = row("a11y:support-class-badge");
    badge.keyboard_reach = BadgeNonVisualReachState::ViewOnlyTrap;
    assert!(!badge.reaches_canonical_truth_via_at());
    assert_eq!(badge.status(), BadgeAccessibilityStatus::Stranded);
}

#[test]
fn empty_badge_context_ref_strands_a_row() {
    let mut badge = row("a11y:support-class-badge");
    badge.badge_context_ref = "  ".to_owned();
    assert!(!badge.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut badge = row("a11y:support-class-badge");
    badge.export_summary = BadgeExportSummaryState::AbsentNeedsScreenshot;
    assert!(!badge.export_preserves_meaning());
    assert_eq!(badge.status(), BadgeAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut badge = row("a11y:support-class-badge");
    badge.copy_export.formats.retain(|f| f != "markdown");
    assert!(!badge.export_preserves_meaning());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut badge = row("a11y:evidence-freshness-badge");
    badge.narrowing_disclosures.clear();
    assert!(!badge.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut badge = row("a11y:evidence-freshness-badge");
    badge.narrowing_disclosures[0].state = BadgeNarrowingDisclosureState::SilentlyDropped;
    assert!(!badge.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut badge = row("a11y:evidence-freshness-badge");
    badge.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!badge.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut badge = row("a11y:support-class-badge");
    badge
        .required_labels
        .retain(|l| *l != M5BadgeRequiredLabel::Identity);
    assert!(!badge.preserves_mandatory_labels());
    assert_eq!(badge.status(), BadgeAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_badge_a11y_fallback_packet();
    packet.rows.retain(|r| r.row_id != "a11y:lifecycle-badge");
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BadgeAccessibilityViolation::MissingFamilyCoverage { .. })));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_badge_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5BadgeConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BadgeAccessibilityViolation::MissingConsumerParity { .. })));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_badge_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BadgeAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_badge_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BadgeAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_badge_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, BadgeAccessibilityViolation::RawReleaseMaterialInExport)));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:compatibility-state-badge").chip_tokens();
    assert!(chip.contains("family=compatibility_state"));
    assert!(chip.contains("effective_claim=policy_blocked"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_badge_a11y_fallback_packet();
    assert_eq!(packet.record_kind, BADGE_A11Y_FALLBACK_RECORD_KIND);
    assert_eq!(packet.schema_version, BADGE_A11Y_FALLBACK_SCHEMA_VERSION);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_badge_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_badge_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,badge_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_badge_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_badge_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_badge_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-badge-family-accessibility-fallback-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_badge_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-badge-family-accessibility-fallback-proof/report.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it
/// never runs in the normal suite. Run with
/// `GEN_BADGE_A11Y_ARTIFACTS=1 cargo test -p aureline-release generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_BADGE_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_badge_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-badge-family-accessibility-fallback-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-badge-family-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 badge-family accessibility fallback fixtures\n\n\
         Mirror of `artifacts/release/m5-badge-family-accessibility-fallback-proof/`.\n\
         Regenerate with `GEN_BADGE_A11Y_ARTIFACTS=1 cargo test -p aureline-release generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
