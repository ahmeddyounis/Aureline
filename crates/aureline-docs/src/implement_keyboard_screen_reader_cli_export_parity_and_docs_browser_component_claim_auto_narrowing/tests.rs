//! Tests for the M05-874 docs-browser-component accessibility fallback capstone: the
//! honest auto-narrowing logic, the per-family parity contract, and the checked-in
//! support export / CSV / report.

use super::*;

fn row(id: &str) -> DocsBrowserAccessibilityRow {
    seeded_m5_docs_browser_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_docs_browser_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_docs_browser_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5DocsBrowserComponentFamily::ALL.len()
    );
    assert_eq!(packet.rows.len(), M5DocsBrowserComponentFamily::ALL.len());
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_docs_browser_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5DocsClaimDimension::ALL.len()
    );
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_docs_browser_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5DocsSupportClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_docs_browser_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5DocsConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_six_yellow_zero_red() {
    let packet = seeded_m5_docs_browser_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 6);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

// --- AC1: cached / adjacent / quarantined docs can no longer keep a current authoritative label ---

#[test]
fn current_search_bar_is_authoritative_and_green() {
    let bar = row("a11y:docs-search-bar");
    assert_eq!(
        bar.full_support_claim,
        M5DocsSupportClaim::CurrentAuthoritative
    );
    assert_eq!(
        bar.effective_claim(),
        M5DocsSupportClaim::CurrentAuthoritative
    );
    assert!(bar.claim_narrow.is_none());
    assert_eq!(bar.status(), DocsAccessibilityStatus::Parity);
    assert!(bar.effective_claim().asserts_current_authoritative());
}

#[test]
fn matched_scope_switcher_is_supported_and_green() {
    let scope = row("a11y:docs-scope-switcher");
    assert_eq!(
        scope.effective_claim(),
        M5DocsSupportClaim::SupportedReference
    );
    assert!(scope.claim_narrow.is_none());
    assert_eq!(scope.status(), DocsAccessibilityStatus::Parity);
    assert!(scope.effective_claim().asserts_full_self_sufficiency());
    assert!(!scope.effective_claim().asserts_current_authoritative());
}

#[test]
fn cached_result_narrows_to_cached_reference() {
    let result = row("a11y:docs-result-row");
    assert_eq!(
        result.effective_claim(),
        M5DocsSupportClaim::CachedReference
    );
    assert!(!result.effective_claim().asserts_full_self_sufficiency());
    let narrow = result.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(narrow.trigger, M5DocsDowngradeTrigger::FreshnessHidden);
    assert_eq!(
        narrow.binding_dimension,
        M5DocsClaimDimension::ResultFreshness
    );
    assert!(result.claim_is_honest());
}

#[test]
fn keyword_fallback_symbol_card_narrows_to_unverified() {
    let card = row("a11y:symbol-linked-reference-card");
    assert_eq!(
        card.effective_claim(),
        M5DocsSupportClaim::UnverifiedReference
    );
    assert!(!card.effective_claim().asserts_current_authoritative());
    assert!(!card.effective_claim().asserts_full_self_sufficiency());
    let narrow = card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5DocsDowngradeTrigger::SymbolAnchorUnresolvedHidden
    );
    assert!(card.claim_is_honest());
}

#[test]
fn adjacent_source_badge_narrows_to_version_adjacent() {
    let badge = row("a11y:docs-source-version-badge");
    assert_eq!(
        badge.effective_claim(),
        M5DocsSupportClaim::VersionAdjacentReference
    );
    assert!(!badge.effective_claim().asserts_current_authoritative());
    assert!(badge.claim_is_honest());
}

#[test]
fn quarantined_pack_narrows_to_policy_blocked() {
    let pack = row("a11y:docs-pack-row");
    assert_eq!(
        pack.effective_claim(),
        M5DocsSupportClaim::PolicyBlockedReference
    );
    assert!(!pack.effective_claim().asserts_full_self_sufficiency());
    assert!(pack.claim_is_honest());
}

#[test]
fn drifted_example_narrows_to_cached_reference() {
    let example = row("a11y:stale-example-finding-row");
    assert_eq!(
        example.effective_claim(),
        M5DocsSupportClaim::CachedReference
    );
    assert!(example.claim_is_honest());
}

#[test]
fn unverified_handoff_narrows_to_unverified_reference() {
    let banner = row("a11y:docs-handoff-banner");
    assert_eq!(
        banner.effective_claim(),
        M5DocsSupportClaim::UnverifiedReference
    );
    assert!(banner.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: an unverified symbol card
    // claiming CurrentAuthoritative.
    let mut card = row("a11y:symbol-linked-reference-card");
    card.claim_narrow = None;
    assert!(!card.claim_is_honest());
    assert_eq!(card.status(), DocsAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_current_row_is_rejected() {
    let mut bar = row("a11y:docs-search-bar");
    bar.claim_narrow = Some(DocsClaimAutoNarrow {
        narrowed_to: M5DocsSupportClaim::CachedReference,
        binding_dimension: M5DocsClaimDimension::CorpusReachability,
        trigger: M5DocsDowngradeTrigger::CorpusClassUnstated,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
    });
    assert!(!bar.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut result = row("a11y:docs-result-row");
    if let Some(narrow) = result.claim_narrow.as_mut() {
        narrow.binding_dimension = M5DocsClaimDimension::CorpusReachability;
    }
    assert!(!result.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut result = row("a11y:docs-result-row");
    if let Some(narrow) = result.claim_narrow.as_mut() {
        narrow.trigger = M5DocsDowngradeTrigger::CorpusClassUnstated;
    }
    assert!(!result.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut result = row("a11y:docs-result-row");
    if let Some(narrow) = result.claim_narrow.as_mut() {
        narrow.narrowed_label = "cached".to_owned();
    }
    assert!(!result.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5DocsConditionState as C;
    use M5DocsSupportClaim as S;
    assert_eq!(C::Current.permitted_ceiling(), S::CurrentAuthoritative);
    assert_eq!(C::Adjacent.permitted_ceiling(), S::VersionAdjacentReference);
    assert_eq!(C::Cached.permitted_ceiling(), S::CachedReference);
    assert_eq!(C::Unverified.permitted_ceiling(), S::UnverifiedReference);
    assert_eq!(
        C::Quarantined.permitted_ceiling(),
        S::PolicyBlockedReference
    );
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5DocsClaimDimension as D;
    use M5DocsDowngradeTrigger as T;
    assert_eq!(
        D::CorpusReachability.default_trigger(),
        T::CorpusClassUnstated
    );
    assert_eq!(D::VersionMatch.default_trigger(), T::VersionScopeUnstated);
    assert_eq!(D::ResultFreshness.default_trigger(), T::FreshnessHidden);
    assert_eq!(
        D::SymbolLinkage.default_trigger(),
        T::SymbolAnchorUnresolvedHidden
    );
    assert_eq!(
        D::SourceProvenance.default_trigger(),
        T::SourceProviderMasked
    );
    assert_eq!(
        D::PackVerification.default_trigger(),
        T::PackStateMisrepresented
    );
    assert_eq!(
        D::ExampleDrift.default_trigger(),
        T::StaleExampleShownAsCurrent
    );
    assert_eq!(D::HandoffState.default_trigger(), T::HandoffReasonUnstated);
}

// --- AC2: accessibility / CLI / export reach the same canonical truth ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_docs_browser_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_symbol_card_binds_a_non_visual_fallback() {
    let card = row("a11y:symbol-linked-reference-card");
    assert!(card.is_hierarchy_heavy());
    assert!(card.has_non_visual_fallback());
    assert!(card
        .fallback_modalities
        .contains(&M5DocsFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut bar = row("a11y:docs-search-bar");
    bar.keyboard_reach = DocsNonVisualReachState::ViewOnlyTrap;
    assert!(!bar.reaches_canonical_truth_via_at());
    assert_eq!(bar.status(), DocsAccessibilityStatus::Stranded);
}

#[test]
fn empty_docs_context_ref_strands_a_row() {
    let mut bar = row("a11y:docs-search-bar");
    bar.docs_context_ref = "  ".to_owned();
    assert!(!bar.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut bar = row("a11y:docs-search-bar");
    bar.export_summary = DocsExportSummaryState::AbsentNeedsScreenshot;
    assert!(!bar.export_preserves_meaning());
    assert_eq!(bar.status(), DocsAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut bar = row("a11y:docs-search-bar");
    bar.copy_export.formats.retain(|f| f != "markdown");
    assert!(!bar.export_preserves_meaning());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut result = row("a11y:docs-result-row");
    result.narrowing_disclosures.clear();
    assert!(!result.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut result = row("a11y:docs-result-row");
    result.narrowing_disclosures[0].state = DocsNarrowingDisclosureState::SilentlyDropped;
    assert!(!result.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut result = row("a11y:docs-result-row");
    result.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!result.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut bar = row("a11y:docs-search-bar");
    bar.required_labels
        .retain(|l| *l != M5DocsRequiredLabel::Identity);
    assert!(!bar.preserves_mandatory_labels());
    assert_eq!(bar.status(), DocsAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_docs_browser_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:docs-scope-switcher");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        DocsBrowserAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_docs_browser_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5DocsConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        DocsBrowserAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_docs_browser_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, DocsBrowserAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_docs_browser_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, DocsBrowserAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_docs_browser_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        DocsBrowserAccessibilityViolation::RawDocsMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:symbol-linked-reference-card").chip_tokens();
    assert!(chip.contains("family=symbol_linked_reference_card"));
    assert!(chip.contains("effective_claim=unverified_reference"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_docs_browser_a11y_fallback_packet();
    assert_eq!(packet.record_kind, DOCS_BROWSER_A11Y_FALLBACK_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        DOCS_BROWSER_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_docs_browser_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_docs_browser_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_docs_browser_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_docs_browser_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_docs_browser_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_docs_browser_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it
/// never runs in the normal suite. Run with
/// `GEN_DOCS_BROWSER_A11Y_ARTIFACTS=1 cargo test -p aureline-docs generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_DOCS_BROWSER_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_docs_browser_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest)
            .join("../../artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback.md"),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/docs/m5/m5-docs-browser-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 docs-browser-component accessibility fallback fixtures\n\n\
         Mirror of `artifacts/docs/m5/m5-docs-browser-component-accessibility-fallback/`.\n\
         Regenerate with `GEN_DOCS_BROWSER_A11Y_ARTIFACTS=1 cargo test -p aureline-docs generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
