//! Unit and fixture coverage for the stable manifest-hardening packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, narrowing verdict,
//!    and effective-permission counts match the fixture's recorded expectation —
//!    proving the automatic narrowing below Stable and the dependency-resolution
//!    permission truth.
//! 3. A `stable` effective tier only renders when the manifest pins the published
//!    schema version, is resolution-backed, satisfies its ranges, resolves every
//!    hard dependency, and never widens authority implicitly.
//! 4. The effective permission set is re-derived from declared + transitive truth
//!    at validation time, so a stored packet cannot hide a transitive permission.
//! 5. The hard guardrails (implicit authority widening, unresolved hard
//!    dependency, range conflict) are surfaced and narrow the claim before
//!    install, upgrade, or mirror promotion.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableManifestHardeningInput,
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
    downgraded_manifest_banner_required: bool,
    attribution_complete: bool,
    manifest_version_current: bool,
    ranges_satisfied: bool,
    lifecycle_installable: bool,
    implicit_authority_widening_present: bool,
    declared_permission_count: usize,
    transitive_permission_count: usize,
    effective_permission_count: usize,
    transitive_only_permission_count: usize,
    optional_integration_count: usize,
    hard_dependency_count: usize,
    unresolved_hard_dependency_count: usize,
    blocks_install: bool,
}

const FIXTURE_DIR: &str =
    "../../../../fixtures/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and";

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and/verified_publisher_stable_current.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and/runtime_range_above_max_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and/catalog_asserted_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and/implicit_widening_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/harden-extension-manifest-permission-display-lifecycle-labels-and/unresolved_hard_dependency_withdrawn.json"
        )),
    ];
    let _ = FIXTURE_DIR;
    raws.iter()
        .map(|raw| serde_json::from_str(raw).expect("fixture must parse"))
        .collect()
}

#[test]
fn every_fixture_builds_validates_and_matches_expectations() {
    let fixtures = all_fixtures();
    assert_eq!(fixtures.len(), 5, "all five canonical fixtures must load");

    for fixture in &fixtures {
        let packet = StableManifestHardeningPacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_manifest_hardening(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));

        let export = project_stable_manifest_hardening_support_export(&packet);

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
            packet.downgraded_manifest_banner.must_display, e.downgraded_manifest_banner_required,
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
            packet.inspection.manifest_version_current, e.manifest_version_current,
            "{}",
            fixture.case_name
        );
        assert_eq!(packet.inspection.ranges_satisfied, e.ranges_satisfied, "{}", fixture.case_name);
        assert_eq!(
            packet.inspection.lifecycle_installable, e.lifecycle_installable,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.implicit_authority_widening_present,
            e.implicit_authority_widening_present,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.declared_permission_count, e.declared_permission_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.transitive_permission_count, e.transitive_permission_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.effective_permission_count, e.effective_permission_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.transitive_only_permission_count, e.transitive_only_permission_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.optional_integration_count, e.optional_integration_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.hard_dependency_count, e.hard_dependency_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.unresolved_hard_dependency_count, e.unresolved_hard_dependency_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(export.blocks_install, e.blocks_install, "{}", fixture.case_name);

        // Cross-cutting invariants for every fixture.
        assert!(
            packet.no_catalog_only_stable_claim(),
            "fixture {} must never imply stable from catalog trust",
            fixture.case_name
        );
        assert!(
            !packet.allows_implicit_authority_widening
                && !packet.allows_catalog_only_trust
                && !packet.allows_hidden_range_conflict
                && !packet.allows_unsurfaced_transitive_permission
        );

        // The effective set always equals declared ∪ transitive, and carries one
        // diff entry per effective permission.
        let declared: std::collections::BTreeSet<&str> = packet
            .effective_permissions
            .declared_permission_refs
            .iter()
            .map(String::as_str)
            .collect();
        let transitive: std::collections::BTreeSet<&str> = packet
            .effective_permissions
            .transitive_permission_refs
            .iter()
            .map(String::as_str)
            .collect();
        let effective: std::collections::BTreeSet<&str> = packet
            .effective_permissions
            .effective_permission_refs
            .iter()
            .map(String::as_str)
            .collect();
        let union: std::collections::BTreeSet<&str> = declared.union(&transitive).copied().collect();
        assert_eq!(effective, union, "fixture {} effective union", fixture.case_name);
        assert_eq!(
            packet.effective_permissions.diff_entries.len(),
            packet.effective_permissions.effective_permission_refs.len(),
            "fixture {} diff coverage",
            fixture.case_name
        );

        // A stable effective tier must satisfy the full posture.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert_eq!(packet.claim.claim_basis_class, "resolution_backed");
            assert!(packet.compatibility_range.ranges_satisfied());
            assert!(packet.attribution_complete());
            assert!(!packet.effective_permissions.implicit_widening_present);
            assert!(!packet.downgraded_manifest_banner.must_display);
            assert!(!packet.claim.downgraded);
        }

        // The projection and export agree with the packet.
        assert_eq!(projection.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(
            export.effective_permission_count,
            packet.effective_permissions.effective_permission_refs.len()
        );
    }
}

fn stable_input() -> StableManifestHardeningInput {
    let fixtures = all_fixtures();
    fixtures
        .into_iter()
        .find(|f| f.case_name == "verified_publisher_stable_current")
        .expect("stable fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(DEPENDENCY_CLASSES.contains(&"hard_dependency"));
    assert!(DEPENDENCY_CLASSES.contains(&"optional_integration"));
    assert!(RANGE_RESOLUTION_CLASSES.contains(&"satisfied"));
    assert!(RANGE_RESOLUTION_CLASSES.contains(&"range_conflict"));
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
    // Every downgrade reason is partitioned into exactly one severity bucket.
    for reason in MANIFEST_HARDENING_DOWNGRADE_REASONS {
        let in_withdrawn = WITHDRAWN_CLASS_REASONS.contains(reason);
        let in_preview = PREVIEW_CLASS_REASONS.contains(reason);
        let in_beta = BETA_CLASS_REASONS.contains(reason);
        assert!(
            (in_withdrawn as u8 + in_preview as u8 + in_beta as u8) == 1,
            "{reason} must be in exactly one severity bucket"
        );
    }
}

#[test]
fn stable_fixture_holds_when_hardened() {
    let packet = StableManifestHardeningPacket::from_input(stable_input()).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.inspection.stable_claim);
    assert!(!packet.downgraded_manifest_banner.must_display);
    // The transitive permission from the hard dependency is folded into the
    // effective set and labelled as transitive-only.
    let transitive = packet
        .effective_permissions
        .diff_entries
        .iter()
        .find(|d| d.permission_ref == "perm:net.connect.registry")
        .expect("transitive permission present");
    assert_eq!(transitive.source_class, "transitive_hard_dependency");
    assert!(transitive
        .contributed_by_refs
        .contains(&"dep:acme.formatter.requires.acme.lsp_bridge".to_string()));
}

#[test]
fn manifest_schema_version_mismatch_narrows_below_stable() {
    let mut input = stable_input();
    input.identity.manifest_schema_version = 99;
    let packet = StableManifestHardeningPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"manifest_schema_version_mismatch".to_string()));
}

#[test]
fn quarantined_trust_tier_raises_banner_and_narrows() {
    let mut input = stable_input();
    input.identity.publisher_trust_tier_class = "quarantined".to_string();
    let packet = StableManifestHardeningPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert!(!STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()));
    assert!(packet.downgraded_manifest_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"trust_tier_quarantined".to_string()));
}

#[test]
fn implicit_widening_narrows_and_still_surfaces_the_permission() {
    let mut input = stable_input();
    // The hard dependency now contributes a permission it does not self-declare.
    input.dependencies[0].permission_implications_machine_readable = false;
    let packet = StableManifestHardeningPacket::from_input(input).expect("must build");
    assert!(packet.effective_permissions.implicit_widening_present);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet.downgraded_manifest_banner.must_display);
    // The undeclared transitive permission is still surfaced in the effective set.
    assert!(packet
        .effective_permissions
        .effective_permission_refs
        .contains(&"perm:net.connect.registry".to_string()));
}

#[test]
fn unresolved_hard_dependency_withdraws_and_excludes_its_permissions() {
    let mut input = stable_input();
    input.dependencies[0].resolution_state_class = "unresolved_missing".to_string();
    let packet = StableManifestHardeningPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"unresolved_hard_dependency".to_string()));
    // An unresolved hard dependency contributes nothing to the effective set.
    assert!(!packet
        .effective_permissions
        .effective_permission_refs
        .contains(&"perm:net.connect.registry".to_string()));
}

#[test]
fn runtime_range_below_minimum_withdraws_the_claim() {
    let mut input = stable_input();
    input.compatibility_range.runtime_range_resolution_class = "below_minimum".to_string();
    let packet = StableManifestHardeningPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_manifest_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"runtime_range_below_min_unsupported".to_string()));
}

#[test]
fn honest_beta_claim_passes_through() {
    let mut input = stable_input();
    input.claim.claimed_tier = "beta".to_string();
    // Even with a runtime above-max, an honest beta claim is not narrowed.
    input.compatibility_range.runtime_range_resolution_class = "above_maximum".to_string();
    let packet = StableManifestHardeningPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
}

#[test]
fn optional_integration_permissions_are_not_folded_into_effective() {
    let packet = StableManifestHardeningPacket::from_input(stable_input()).expect("must build");
    // The optional telemetry integration is surfaced separately, not effective.
    assert!(packet
        .effective_permissions
        .optional_integration_permission_refs
        .contains(&"perm:telemetry.usage".to_string()));
    assert!(!packet
        .effective_permissions
        .effective_permission_refs
        .contains(&"perm:telemetry.usage".to_string()));
}

#[test]
fn incomplete_attribution_narrows_below_stable() {
    let mut input = stable_input();
    input.identity.source_package_ref = "   ".to_string();
    let result = StableManifestHardeningPacket::from_input(input);
    // An identity without a source package fails the non-empty input check.
    assert!(result.is_err());
}

#[test]
fn unknown_capability_class_is_rejected() {
    let mut input = stable_input();
    input.declared_permissions[0].capability_class = "root_everything".to_string();
    let result = StableManifestHardeningPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn lifecycle_label_must_match_identity_lifecycle() {
    let mut input = stable_input();
    input.lifecycle_label.lifecycle_state_class = "disabled".to_string();
    let result = StableManifestHardeningPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn support_export_quotes_permission_and_dependency_counts() {
    let packet = StableManifestHardeningPacket::from_input(stable_input()).expect("must build");
    let export = project_stable_manifest_hardening_support_export(&packet);
    assert_eq!(export.effective_permission_count, 3);
    assert_eq!(export.hard_dependency_count, 1);
    assert_eq!(export.optional_integration_count, 1);
    assert!(!export.blocks_install);
    assert!(export.export_safe_summary.contains("effective=3"));
}
