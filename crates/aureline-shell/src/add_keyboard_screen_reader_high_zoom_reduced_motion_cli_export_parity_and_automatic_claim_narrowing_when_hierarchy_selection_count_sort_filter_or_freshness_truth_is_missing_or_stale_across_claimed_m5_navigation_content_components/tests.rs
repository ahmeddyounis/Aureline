//! Tests for the M05-1114 navigation / content-component accessibility parity capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the stale/unresolved-never-current guarantee,
//! no-loss navigation / content-truth integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> NavContentComponentAccessibilityRow {
    seeded_m5_navigation_content_component_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_navigation_content_component_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_navigation_content_component_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5NavigationContentComponentFamily::ALL.len()
    );
    // Six rows cover the six families one-to-one (one certified fully green — the active-context-stated
    // tab strip — the other five narrowed-yellow: four auto-narrowed claims plus the dense tree view
    // whose screen-reader traversal narrows to a disclosed linear walk).
    assert_eq!(packet.rows.len(), 6);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_navigation_content_component_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5NavContentComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_navigation_content_component_a11y_packet();
    let states = packet.exercised_condition_states();
    for state in M5NavContentComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_navigation_content_component_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5NavContentComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_navigation_content_component_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5NavigationContentConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_one_green_five_yellow_zero_red() {
    let packet = seeded_m5_navigation_content_component_a11y_packet();
    assert_eq!(packet.summary.green_count, 1);
    assert_eq!(packet.summary.yellow_count, 5);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 6);
    assert_eq!(
        packet.summary.family_count,
        M5NavigationContentComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_truth_and_current_honesty() {
    let packet = seeded_m5_navigation_content_component_a11y_packet();
    assert!(packet.summary.all_truth_preserved);
    assert!(packet.summary.all_current_honesty_holds);
    assert!(packet.summary.all_claims_honest);
    assert!(packet.summary.all_export_summaries_preserve_meaning);
    assert!(packet.summary.all_reach_canonical_truth_via_at);
    assert!(packet.summary.all_narrowing_disclosed);
}

// --- auto-narrowing per family ---

#[test]
fn active_context_stated_tab_strip_is_current_and_green() {
    let tab_strip = row("a11y:tab-strip-active-context-stated");
    assert_eq!(
        tab_strip.full_ready_claim,
        M5NavContentComponentClaim::CurrentNavigationResult
    );
    assert_eq!(
        tab_strip.effective_claim(),
        M5NavContentComponentClaim::CurrentNavigationResult
    );
    assert!(tab_strip.claim_narrow.is_none());
    assert_eq!(
        tab_strip.status(),
        NavContentComponentAccessibilityStatus::Parity
    );
    assert!(tab_strip
        .effective_claim()
        .asserts_current_navigation_result());
}

#[test]
fn stale_hierarchy_breadcrumbs_narrows_and_is_never_current() {
    let trail = row("a11y:breadcrumbs-hierarchy-stale");
    assert_eq!(
        trail.effective_claim(),
        M5NavContentComponentClaim::HierarchyUnverifiedProjection
    );
    assert!(!trail.effective_claim().asserts_current_navigation_result());
    assert!(trail.current_honesty_holds());
    assert_eq!(
        trail.status(),
        NavContentComponentAccessibilityStatus::NarrowedDisclosed
    );
    let narrow = trail.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5NavigationContentDowngradeTrigger::HierarchyPathUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5NavContentComponentClaimDimension::HierarchyPathClarity
    );
}

#[test]
fn each_weak_family_narrows_to_its_permitted_ceiling() {
    for (id, expected) in [
        (
            "a11y:breadcrumbs-hierarchy-stale",
            M5NavContentComponentClaim::HierarchyUnverifiedProjection,
        ),
        (
            "a11y:list-view-count-scope-unresolved",
            M5NavContentComponentClaim::CountUnverifiedProjection,
        ),
        (
            "a11y:table-grid-sort-filter-provenance-stale",
            M5NavContentComponentClaim::SortFilterUnverifiedProjection,
        ),
        (
            "a11y:panel-header-source-freshness-partial",
            M5NavContentComponentClaim::SourceFreshnessProjection,
        ),
    ] {
        let r = row(id);
        assert_eq!(r.effective_claim(), expected, "row {id}");
        assert_eq!(r.permitted_claim(), expected, "row {id} permitted");
        assert!(r.claim_is_honest(), "row {id} honest");
        assert_eq!(
            r.status(),
            NavContentComponentAccessibilityStatus::NarrowedDisclosed,
            "row {id} yellow"
        );
    }
}

#[test]
fn cannot_be_shown_current_flags_exactly_the_overclaim_states() {
    use M5NavContentComponentConditionState as C;
    assert!(C::HierarchyPathStale.cannot_be_shown_current());
    assert!(C::CountScopeUnresolved.cannot_be_shown_current());
    assert!(C::SortFilterProvenanceStale.cannot_be_shown_current());
    // An honest disclosed-absence operation (a cached / partial freshness cue) is not a truth
    // overstatement.
    assert!(!C::SourceFreshnessPartial.cannot_be_shown_current());
    assert!(!C::FullyQualified.cannot_be_shown_current());
}

#[test]
fn tree_view_is_reviewable_hierarchy_heavy_and_yellow() {
    let tree = row("a11y:tree-view-disclosure-selection-stated");
    assert!(tree.is_hierarchy_heavy());
    assert!(tree.has_non_visual_fallback());
    assert!(tree
        .fallback_modalities
        .contains(&M5NavContentComponentFallbackModality::Structured));
    assert_eq!(
        tree.effective_claim(),
        M5NavContentComponentClaim::ReviewableStructureResult
    );
    assert!(tree.claim_narrow.is_none());
    assert_eq!(
        tree.status(),
        NavContentComponentAccessibilityStatus::NarrowedDisclosed
    );
}

// --- negative / red-detection cases ---

#[test]
fn over_asserting_a_weak_state_is_stranded() {
    let mut trail = row("a11y:breadcrumbs-hierarchy-stale");
    // Drop the narrow so the stale-hierarchy state keeps a current claim.
    trail.claim_narrow = None;
    assert!(!trail.claim_is_honest());
    assert!(!trail.current_honesty_holds());
    assert_eq!(
        trail.status(),
        NavContentComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn a_view_only_hover_trap_is_stranded() {
    let mut tree = row("a11y:tree-view-disclosure-selection-stated");
    tree.keyboard_reach = NavContentComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!tree.reaches_canonical_truth_via_at());
    assert_eq!(
        tree.status(),
        NavContentComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn a_raw_payload_only_export_is_stranded() {
    let mut tab_strip = row("a11y:tab-strip-active-context-stated");
    tab_strip.export_summary = NavContentComponentExportSummaryState::RequiresRawPayload;
    assert!(!tab_strip.export_preserves_meaning());
    assert_eq!(
        tab_strip.status(),
        NavContentComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn a_generic_narrow_label_is_dishonest() {
    let mut trail = row("a11y:breadcrumbs-hierarchy-stale");
    trail.claim_narrow.as_mut().unwrap().narrowed_label = "stale".to_owned();
    assert!(!trail.claim_is_honest());
}

#[test]
fn dropping_a_family_reports_missing_coverage() {
    let mut packet = seeded_m5_navigation_content_component_a11y_packet();
    packet.rows.pop();
    let mut packet =
        NavContentComponentAccessibilityPacket::new(NavContentComponentAccessibilityPacketInput {
            packet_id: packet.packet_id,
            as_of: packet.as_of,
            matrix_ref: packet.matrix_ref,
            rows: packet.rows,
        });
    packet.summary = packet.computed_summary();
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"missing_family_coverage"));
}

// --- forbidden material ---

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut packet = seeded_m5_navigation_content_component_a11y_packet();
    packet.rows[0]
        .copy_export
        .export_fields
        .push("bearer abc".to_owned());
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"raw_nav_content_material_in_export"));
}

// --- rendering ---

#[test]
fn csv_has_a_row_per_certified_component() {
    let packet = seeded_m5_navigation_content_component_a11y_packet();
    let csv = packet.render_matrix_csv();
    // header + 6 rows + trailing newline
    assert_eq!(csv.lines().count(), 7);
    for row in &packet.rows {
        assert!(csv.contains(&row.row_id));
    }
}

#[test]
fn markdown_summary_lists_every_row_and_narrowing() {
    let packet = seeded_m5_navigation_content_component_a11y_packet();
    let md = packet.render_markdown_summary();
    for row in &packet.rows {
        assert!(md.contains(&row.row_id));
    }
    assert!(md.contains("Auto-narrow"));
}

// --- byte-lock against the checked-in artifacts ---

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_m5_navigation_content_component_a11y_export()
        .expect("checked M5 navigation-content a11y export validates");
    assert_eq!(from_disk.packet_id, NAV_CONTENT_A11Y_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_navigation_content_component_a11y_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_matrix_csv_matches_builder() {
    let expected = seeded_m5_navigation_content_component_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-navigation-content-component-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        on_disk, expected,
        "checked matrix CSV drifted from the builder"
    );
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_navigation_content_component_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-navigation-content-component-accessibility-parity.md"
    ));
    assert_eq!(on_disk, expected, "checked report drifted from the builder");
}

#[test]
fn checked_fixtures_mirror_the_release_artifacts() {
    let fixture_export = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-navigation-content-component-accessibility-parity/support_export.json"
    ));
    let packet: NavContentComponentAccessibilityPacket =
        serde_json::from_str(fixture_export).expect("fixture export parses");
    assert!(packet.validate().is_empty());
    assert_eq!(packet, seeded_m5_navigation_content_component_a11y_packet());

    let fixture_csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-navigation-content-component-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        fixture_csv,
        seeded_m5_navigation_content_component_a11y_packet().render_matrix_csv()
    );
}

// --- gated artifact regeneration ---
//
// Set `GEN_NAVIGATION_CONTENT_A11Y_ARTIFACTS=1` to (re)write the checked-in release artifacts and
// fixtures from the seed builder. Off by default so a normal `cargo test` never mutates the tree.
#[test]
fn regenerate_checked_artifacts_when_requested() {
    if std::env::var("GEN_NAVIGATION_CONTENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_navigation_content_component_a11y_packet();
    assert!(
        packet.validate().is_empty(),
        "seed must validate before write"
    );

    let manifest = env!("CARGO_MANIFEST_DIR");
    let repo = Path::new(manifest).join("../..");

    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let md = packet.render_markdown_summary();

    let release_dir =
        repo.join("artifacts/release/m5-navigation-content-component-accessibility-parity");
    fs::create_dir_all(&release_dir).expect("create release dir");
    fs::write(release_dir.join("support_export.json"), &json).expect("write release export");
    fs::write(release_dir.join("matrix.csv"), &csv).expect("write release csv");
    fs::write(
        repo.join("artifacts/release/m5-navigation-content-component-accessibility-parity.md"),
        &md,
    )
    .expect("write release report");

    let fixture_dir = repo.join("fixtures/ui/m5-navigation-content-component-accessibility-parity");
    fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    fs::write(fixture_dir.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixture_dir.join("matrix.csv"), &csv).expect("write fixture csv");
}
