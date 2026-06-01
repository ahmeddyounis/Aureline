//! Unit and fixture coverage for the stable marketplace catalog-truth packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/finalize-marketplace-catalog-truth-provenance-compatibility-labels-activation/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, narrowing verdict, and
//!    compatibility counts match the fixture's recorded expectation — proving the
//!    automatic narrowing below Stable.
//! 3. A `stable` effective tier only renders when the row pins the published catalog
//!    version, is evidence-backed, keeps a verified runtime class, ships compatibility
//!    scorecards with no inherited / unsupported / stale evidence, keeps its activation
//!    cost bounded, keeps a stable-grade non-profile-limited support class, keeps its
//!    publisher continuity current, and keeps its truth aligned across views.
//! 4. The compatibility summary is re-derived from the scorecards at validation time, so
//!    a stored packet cannot hide an inherited, unsupported, or stale scorecard.
//! 5. The hard guardrails (catalog-only trust, unbounded activation cost, inherited
//!    parity, ranking-implied trust) are surfaced and narrow the claim before users or
//!    admins rely on the row.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableMarketplaceCatalogTruthInput,
    expected: ExpectedPacket,
}

#[derive(Debug, Deserialize)]
struct ExpectedPacket {
    claimed_tier: String,
    effective_tier: String,
    support_claim_class: String,
    stable_claim: bool,
    downgraded: bool,
    downgrade_reasons: Vec<String>,
    downgraded_banner_required: bool,
    attribution_complete: bool,
    catalog_version_current: bool,
    lifecycle_installable: bool,
    compatibility_stable_backable: bool,
    views_aligned: bool,
    scorecard_count: usize,
    inherited_scorecard_count: usize,
    unsupported_scorecard_count: usize,
    blocks_stable_catalog_truth: bool,
}

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-marketplace-catalog-truth-provenance-compatibility-labels-activation/verified_publisher_stable_current.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-marketplace-catalog-truth-provenance-compatibility-labels-activation/scorecard_freshness_stale_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-marketplace-catalog-truth-provenance-compatibility-labels-activation/catalog_asserted_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-marketplace-catalog-truth-provenance-compatibility-labels-activation/quarantined_from_discovery_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/finalize-marketplace-catalog-truth-provenance-compatibility-labels-activation/unsupported_scorecard_withdrawn.json"
        )),
    ];
    raws.iter()
        .map(|raw| serde_json::from_str(raw).expect("fixture must parse"))
        .collect()
}

#[test]
fn every_fixture_builds_validates_and_matches_expectations() {
    let fixtures = all_fixtures();
    assert_eq!(fixtures.len(), 5, "all five canonical fixtures must load");

    for fixture in &fixtures {
        let packet = StableMarketplaceCatalogTruthPacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_marketplace_catalog_truth(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));

        let export = project_stable_marketplace_catalog_truth_support_export(&packet);

        let e = &fixture.expected;
        assert_eq!(packet.claim.claimed_tier, e.claimed_tier, "{}", fixture.case_name);
        assert_eq!(packet.claim.effective_tier, e.effective_tier, "{}", fixture.case_name);
        assert_eq!(
            packet.claim.support_claim_class, e.support_claim_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(packet.inspection.stable_claim, e.stable_claim, "{}", fixture.case_name);
        assert_eq!(packet.claim.downgraded, e.downgraded, "{}", fixture.case_name);

        let mut got = packet.claim.downgrade_reasons.clone();
        got.sort();
        let mut want = e.downgrade_reasons.clone();
        want.sort();
        assert_eq!(got, want, "fixture {} downgrade reasons", fixture.case_name);

        assert_eq!(
            packet.downgraded_banner.must_display, e.downgraded_banner_required,
            "fixture {} banner",
            fixture.case_name
        );
        assert_eq!(
            packet.attribution_complete(),
            e.attribution_complete,
            "fixture {} attribution",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.catalog_version_current, e.catalog_version_current,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.lifecycle_installable, e.lifecycle_installable,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.compatibility_stable_backable, e.compatibility_stable_backable,
            "{}",
            fixture.case_name
        );
        assert_eq!(packet.inspection.views_aligned, e.views_aligned, "{}", fixture.case_name);
        assert_eq!(
            packet.inspection.scorecard_count, e.scorecard_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.inherited_scorecard_count, e.inherited_scorecard_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.unsupported_scorecard_count, e.unsupported_scorecard_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            export.blocks_stable_catalog_truth, e.blocks_stable_catalog_truth,
            "{}",
            fixture.case_name
        );

        // Cross-cutting invariants for every fixture.
        assert!(
            packet.no_catalog_only_stable_claim(),
            "fixture {} must never imply stable from catalog trust",
            fixture.case_name
        );
        assert!(
            !packet.allows_catalog_only_trust
                && !packet.allows_unbounded_activation_cost
                && !packet.allows_inherited_parity_stable_claim
                && !packet.allows_ranking_implied_trust
        );

        // The compatibility summary always re-derives from the scorecards.
        let inherited = packet.scorecards.iter().filter(|s| s.parity_inherited()).count();
        assert_eq!(
            inherited, packet.compatibility_summary.inherited_count,
            "fixture {} inherited count",
            fixture.case_name
        );

        // A stable effective tier must satisfy the full posture.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert_eq!(packet.claim.claim_basis_class, "evidence_backed");
            assert!(packet.compatibility_summary.stable_backable());
            assert!(packet.attribution_complete());
            assert!(packet.activation_budget.within_budget());
            assert!(packet.publisher_continuity.current());
            assert!(packet.view_alignment.all_views_aligned());
            assert!(packet.support_class.stable_grade());
            assert!(!packet.support_class.profile_limited);
            assert!(!packet.discoverability.penalized());
            assert!(!packet.discoverability.quarantined());
            assert!(packet.surface_boundary.runtime_class_verified);
            assert!(!packet.downgraded_banner.must_display);
            assert!(!packet.claim.downgraded);
        }

        // The projection and export agree with the packet.
        assert_eq!(projection.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.scorecard_count, packet.scorecards.len());
        // Runtime-class truth is preserved into the support/mirror export.
        assert_eq!(export.runtime_class, packet.surface_boundary.runtime_class);
        assert_eq!(
            export.runtime_class_verified,
            packet.surface_boundary.runtime_class_verified
        );
        assert_eq!(export.support_profile_limited, packet.support_class.profile_limited);
    }
}

fn stable_input() -> StableMarketplaceCatalogTruthInput {
    all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "verified_publisher_stable_current")
        .expect("stable fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(PROVENANCE_CLASSES.contains(&"under_review"));
    assert!(RANKING_STATE_CLASSES.contains(&"quarantined"));
    assert!(EVIDENCE_SOURCE_CLASSES.contains(&"inherited_from_adjacent"));
    assert!(ACTIVATION_BUDGET_CLASSES.contains(&"unbounded"));
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
    // Every downgrade reason is partitioned into exactly one severity bucket.
    for reason in MARKETPLACE_CATALOG_DOWNGRADE_REASONS {
        let in_withdrawn = WITHDRAWN_CLASS_REASONS.contains(reason);
        let in_preview = PREVIEW_CLASS_REASONS.contains(reason);
        let in_beta = BETA_CLASS_REASONS.contains(reason);
        assert!(
            (in_withdrawn as u8 + in_preview as u8 + in_beta as u8) == 1,
            "{reason} must be in exactly one severity bucket"
        );
    }
    // Every stable-grade support class is a valid support class.
    for class in STABLE_GRADE_SUPPORT_CLASSES {
        assert!(SUPPORT_CLASS_CLASSES.contains(class));
    }
    // Every surface-disclosure boundary is a valid host boundary.
    for boundary in SURFACE_DISCLOSURE_BOUNDARIES {
        assert!(HOST_BOUNDARY_CLASSES.contains(boundary));
    }
}

#[test]
fn stable_fixture_holds_when_stabilized() {
    let packet = StableMarketplaceCatalogTruthPacket::from_input(stable_input()).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.inspection.stable_claim);
    assert!(!packet.downgraded_banner.must_display);
    assert!(packet.compatibility_summary.has_scorecard);
    assert!(packet.compatibility_summary.no_inherited_parity);
    assert!(packet.view_alignment.all_views_aligned());
}

#[test]
fn catalog_version_mismatch_narrows_below_stable() {
    let mut input = stable_input();
    input.identity.catalog_version = 99;
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"catalog_version_not_published".to_string()));
}

#[test]
fn catalog_asserted_basis_cannot_back_stable() {
    let mut input = stable_input();
    input.claim.claim_basis_class = "catalog_asserted_only".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"catalog_only_trust_not_evidence_backed".to_string()));
    assert!(packet.no_catalog_only_stable_claim());
}

#[test]
fn provenance_under_review_withdraws_and_raises_banner() {
    let mut input = stable_input();
    input.provenance.provenance_class = "under_review".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"provenance_under_review".to_string()));
}

#[test]
fn quarantined_from_discovery_withdraws_the_row() {
    let mut input = stable_input();
    input.discoverability.ranking_state_class = "quarantined".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"quarantined_from_discovery".to_string()));
}

#[test]
fn penalized_ranking_narrows_to_beta() {
    let mut input = stable_input();
    input.discoverability.ranking_state_class = "penalized_staleness".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"ranking_penalized".to_string()));
}

#[test]
fn unverified_runtime_class_withdraws_the_row() {
    let mut input = stable_input();
    input.surface_boundary.runtime_class_verified = false;
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"runtime_class_unverified".to_string()));
}

#[test]
fn undisclosed_hosted_surface_narrows_to_preview() {
    let mut input = stable_input();
    input.surface_boundary.host_boundary_class = "hosted_remote_surface".to_string();
    input.surface_boundary.hosted_surface_implication = true;
    input.surface_boundary.surface_boundary_disclosed = false;
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"hosted_surface_boundary_undisclosed".to_string()));
}

#[test]
fn inherited_parity_scorecard_narrows_below_stable() {
    let mut input = stable_input();
    input.scorecards[0].evidence_source_class = "inherited_from_adjacent".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert_eq!(packet.compatibility_summary.inherited_count, 1);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"scorecard_parity_inherited".to_string()));
}

#[test]
fn unsupported_scorecard_withdraws_the_row() {
    let mut input = stable_input();
    input.scorecards[0].parity_band_class = "unsupported".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert_eq!(packet.compatibility_summary.unsupported_count, 1);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"compatibility_unsupported".to_string()));
}

#[test]
fn missing_scorecard_is_rejected_at_input() {
    let mut input = stable_input();
    input.scorecards.clear();
    let result = StableMarketplaceCatalogTruthPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn unbounded_activation_cost_withdraws_the_row() {
    let mut input = stable_input();
    input.activation_budget.budget_class = "unbounded".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"activation_cost_unbounded".to_string()));
}

#[test]
fn over_budget_activation_cost_narrows_to_beta() {
    let mut input = stable_input();
    input.activation_budget.budget_class = "over_budget".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"activation_cost_over_budget".to_string()));
}

#[test]
fn profile_limited_support_narrows_to_beta() {
    let mut input = stable_input();
    input.support_class.profile_limited = true;
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"support_class_profile_limited".to_string()));
}

#[test]
fn experimental_support_class_narrows_to_preview() {
    let mut input = stable_input();
    input.support_class.support_class_class = "experimental".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"support_class_below_stable_grade".to_string()));
}

#[test]
fn stale_publisher_continuity_narrows_to_beta() {
    let mut input = stable_input();
    input.publisher_continuity.continuity_state_class = "stale".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"publisher_continuity_stale".to_string()));
}

#[test]
fn revoked_publisher_continuity_withdraws_the_row() {
    let mut input = stable_input();
    input.publisher_continuity.continuity_state_class = "revoked".to_string();
    input.publisher_continuity.continuity_packet_ref = None;
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"publisher_continuity_revoked".to_string()));
}

#[test]
fn unaligned_views_narrow_below_stable() {
    let mut input = stable_input();
    input.view_alignment.aligned_views = vec!["public_registry".to_string()];
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(!packet.view_alignment.all_views_aligned());
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"views_not_aligned".to_string()));
}

#[test]
fn honest_beta_claim_passes_through() {
    let mut input = stable_input();
    input.claim.claimed_tier = "beta".to_string();
    // Even with a stale scorecard, an honest beta claim is not narrowed.
    input.scorecards[0].freshness_window_class = "stale".to_string();
    let packet = StableMarketplaceCatalogTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
}

#[test]
fn current_continuity_requires_a_continuity_packet_ref() {
    let mut input = stable_input();
    input.publisher_continuity.continuity_packet_ref = None;
    let result = StableMarketplaceCatalogTruthPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn unknown_provenance_class_is_rejected() {
    let mut input = stable_input();
    input.provenance.provenance_class = "super_trusted".to_string();
    let result = StableMarketplaceCatalogTruthPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn non_mechanically_sourced_provenance_is_rejected() {
    let mut input = stable_input();
    input.provenance.mechanically_sourced = false;
    let result = StableMarketplaceCatalogTruthPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn ranking_not_explainable_without_install_count_is_rejected() {
    let mut input = stable_input();
    input.discoverability.ranking_explained_without_install_count = false;
    let result = StableMarketplaceCatalogTruthPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn support_export_preserves_runtime_class_truth() {
    let packet = StableMarketplaceCatalogTruthPacket::from_input(stable_input()).expect("must build");
    let export = project_stable_marketplace_catalog_truth_support_export(&packet);
    assert!(!export.blocks_stable_catalog_truth);
    assert_eq!(export.runtime_class, packet.surface_boundary.runtime_class);
    assert!(export.runtime_class_verified);
    assert!(!export.support_profile_limited);
    assert!(export.views_aligned);
    assert!(export.export_safe_summary.contains("Runtime="));
}
