//! Unit and fixture coverage for the stable mirror/manual import-truth packet.
//!
//! These tests load every fixture under
//! `fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/`
//! and assert that:
//!
//! 1. Every fixture's input builds, validates, and projects without error.
//! 2. The effective tier, support claim, banner requirement, and narrowing verdict
//!    match the fixture's recorded expectation — proving the automatic narrowing below
//!    Stable.
//! 3. A `stable` effective tier only renders when the row pins the published import
//!    version, is evidence-backed, preserves its source class, stays explainable
//!    offline, pins a last-known-good, keeps its publisher-transfer continuity current
//!    (and gates high-trust auto-update behind delay / audit / notification for any
//!    transfer event), maps exactly or by a verified translation from the real
//!    artifact, never widens permissions, keeps verified compatibility, keeps its
//!    activation cost bounded, keeps a clean revocation posture, stays mirrorable, and
//!    is fully attributed.
//! 4. The effective tier, downgrade verdict, reasons, and banner are re-derived from
//!    the posture at validation time, so a stored packet cannot drift from its truth.
//! 5. The continuity scenarios (key rotation, ownership transfer, namespace dispute,
//!    orphan succession, mirror promotion) preserve deterministic user/admin copy,
//!    audit lineage, and rollback-to-last-known-good behavior.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: StableMirrorImportTruthInput,
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
    import_version_current: bool,
    lifecycle_installable: bool,
    blocks_stable_import_truth: bool,
}

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/verified_publisher_offline_bundle_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/approved_mirror_promotion_settled_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/signer_key_rotation_in_cooldown_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/ownership_transfer_pending_notification_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/namespace_dispute_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/orphan_succession_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/manual_artifact_shimmed_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/unsupported_mapping_failed_withdrawn.json"
        )),
    ];
    raws.iter()
        .map(|raw| serde_json::from_str(raw).expect("fixture must parse"))
        .collect()
}

#[test]
fn every_fixture_builds_validates_and_matches_expectations() {
    let fixtures = all_fixtures();
    assert_eq!(fixtures.len(), 8, "all eight canonical fixtures must load");

    for fixture in &fixtures {
        let packet = StableMirrorImportTruthPacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must re-validate");

        let payload = serde_json::to_string(&packet).expect("serialize");
        let projection = project_stable_mirror_import_truth(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));

        let export = project_stable_mirror_import_truth_support_export(&packet);

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
            packet.inspection.import_version_current, e.import_version_current,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.lifecycle_installable, e.lifecycle_installable,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            export.blocks_stable_import_truth, e.blocks_stable_import_truth,
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
                && !packet.allows_ambient_privilege
                && !packet.allows_unbounded_activation_cost
        );

        // A revoked or rehomed row stays explainable offline: a last-known-good ref is
        // always carried into the support/mirror export.
        assert_eq!(export.last_known_good_ref, packet.source_class.last_known_good_ref);
        assert_eq!(
            export.last_known_good_pinned,
            packet.source_class.last_known_good_pinned
        );

        // A stable effective tier must satisfy the full posture.
        if STABLE_TIERS.contains(&packet.claim.effective_tier.as_str()) {
            assert_eq!(packet.claim.claim_basis_class, "evidence_backed");
            assert!(packet.attribution_complete());
            assert!(packet.source_class.source_class_preserved);
            assert!(packet.source_class.offline_explainable);
            assert!(packet.source_class.last_known_good_pinned);
            assert!(packet.continuity.current());
            assert!(packet.continuity.auto_update_safely_gated());
            assert!(packet.continuity.notification_satisfied());
            assert!(packet.mapping_outcome.stable_grade());
            assert!(!packet.permission_posture.widened_on_import);
            assert!(packet.compatibility.compatibility_verified);
            assert!(packet.activation_budget.within_budget());
            assert!(packet.install_posture.revocation_clean());
            assert!(packet.install_posture.mirrorable());
            assert!(!packet.downgraded_banner.must_display);
            assert!(!packet.claim.downgraded);
        }

        // The projection and export agree with the packet.
        assert_eq!(projection.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(export.import_route_class, packet.source_class.import_route_class);
        assert_eq!(export.continuity_state_class, packet.continuity.continuity_state_class);
    }
}

fn stable_input() -> StableMirrorImportTruthInput {
    all_fixtures()
        .into_iter()
        .find(|f| f.case_name == "verified_publisher_offline_bundle_stable")
        .expect("stable fixture present")
        .packet_input
}

#[test]
fn closed_vocabularies_hold_their_anchors() {
    assert!(CONTINUITY_EVENT_CLASSES.contains(&"signer_key_rotation"));
    assert!(CONTINUITY_STATE_CLASSES.contains(&"disputed"));
    assert!(MAPPING_OUTCOME_CLASSES.contains(&"unsupported"));
    assert!(ACTIVATION_BUDGET_CLASSES.contains(&"unbounded"));
    assert!(STABLE_TIERS.iter().all(|t| STABILITY_TIERS.contains(t)));
    // Every transfer continuity event is a valid continuity event.
    for ev in TRANSFER_CONTINUITY_EVENTS {
        assert!(CONTINUITY_EVENT_CLASSES.contains(ev));
    }
    // Every stable-grade mapping outcome is a valid mapping outcome.
    for m in STABLE_GRADE_MAPPING_OUTCOMES {
        assert!(MAPPING_OUTCOME_CLASSES.contains(m));
    }
    // Every downgrade reason is partitioned into exactly one severity bucket.
    for reason in MIRROR_IMPORT_DOWNGRADE_REASONS {
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
fn stable_fixture_holds_when_stabilized() {
    let packet = StableMirrorImportTruthPacket::from_input(stable_input()).expect("must build");
    assert_eq!(packet.claim.effective_tier, "stable");
    assert!(!packet.claim.downgraded);
    assert!(packet.inspection.stable_claim);
    assert!(!packet.downgraded_banner.must_display);
    assert!(packet.continuity.current());
    assert!(packet.install_posture.mirrorable());
}

#[test]
fn import_version_mismatch_narrows_below_stable() {
    let mut input = stable_input();
    input.identity.import_version = 99;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert!(packet.claim.downgraded);
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"import_version_not_published".to_string()));
}

#[test]
fn catalog_asserted_basis_cannot_back_stable() {
    let mut input = stable_input();
    input.claim.claim_basis_class = "catalog_asserted_only".to_string();
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"catalog_only_trust_not_evidence_backed".to_string()));
    assert!(packet.no_catalog_only_stable_claim());
}

#[test]
fn quarantined_trust_tier_narrows_to_preview() {
    let mut input = stable_input();
    input.identity.publisher_trust_tier_class = "quarantined".to_string();
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"trust_tier_quarantined".to_string()));
}

#[test]
fn non_installable_lifecycle_withdraws_the_row() {
    let mut input = stable_input();
    input.identity.lifecycle_state_class = "removed".to_string();
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"lifecycle_not_installable".to_string()));
}

#[test]
fn source_class_not_preserved_narrows_to_preview() {
    let mut input = stable_input();
    input.source_class.source_class_preserved = false;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"source_class_not_preserved".to_string()));
}

#[test]
fn not_offline_explainable_narrows_to_preview_and_raises_banner() {
    let mut input = stable_input();
    input.source_class.offline_explainable = false;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"offline_not_explainable".to_string()));
}

#[test]
fn missing_last_known_good_narrows_to_preview() {
    let mut input = stable_input();
    input.source_class.last_known_good_pinned = false;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"last_known_good_not_pinned".to_string()));
}

#[test]
fn ungated_auto_update_on_transfer_event_withdraws_the_row() {
    let mut input = stable_input();
    input.continuity.continuity_event_class = "ownership_transfer".to_string();
    input.continuity.high_trust_auto_update_gated = false;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"high_trust_auto_update_not_gated".to_string()));
}

#[test]
fn revoked_continuity_withdraws_the_row() {
    let mut input = stable_input();
    input.continuity.continuity_event_class = "ownership_transfer".to_string();
    input.continuity.continuity_state_class = "revoked".to_string();
    input.continuity.continuity_packet_ref = None;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"continuity_revoked".to_string()));
}

#[test]
fn incomplete_audit_lineage_narrows_to_preview() {
    let mut input = stable_input();
    input.continuity.audit_lineage_preserved = false;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"audit_lineage_incomplete".to_string()));
}

#[test]
fn missing_transfer_history_narrows_to_preview() {
    let mut input = stable_input();
    input.continuity.transfer_history_preserved = false;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"transfer_history_not_preserved".to_string()));
}

#[test]
fn mapping_not_from_real_artifact_narrows_to_preview() {
    let mut input = stable_input();
    input.mapping_outcome.generated_from_real_artifact = false;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"mapping_not_from_real_artifact".to_string()));
}

#[test]
fn failed_mapping_without_checkpoint_withdraws_the_row() {
    let mut input = stable_input();
    input.mapping_outcome.mapping_failed = true;
    input.mapping_outcome.checkpoint_preserved = false;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"mapping_rollback_checkpoint_missing".to_string()));
}

#[test]
fn widened_permissions_on_import_withdraws_the_row() {
    let mut input = stable_input();
    input.permission_posture.widened_on_import = true;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"permission_widened_on_import".to_string()));
}

#[test]
fn parity_limited_compatibility_narrows_to_beta() {
    let mut input = stable_input();
    input.compatibility.compatibility_label_class = "partial_parity".to_string();
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"compatibility_parity_limited".to_string()));
}

#[test]
fn inherited_compatibility_evidence_narrows_to_preview() {
    let mut input = stable_input();
    input.compatibility.evidence_source_class = "inherited_from_adjacent".to_string();
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"compatibility_evidence_inherited".to_string()));
}

#[test]
fn unbounded_activation_cost_withdraws_the_row() {
    let mut input = stable_input();
    input.activation_budget.budget_class = "unbounded".to_string();
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
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
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"activation_cost_over_budget".to_string()));
}

#[test]
fn undisclosed_install_scope_narrows_to_preview() {
    let mut input = stable_input();
    input.install_posture.install_scope_disclosed = false;
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"install_scope_not_disclosed".to_string()));
}

#[test]
fn quarantined_revocation_posture_withdraws_the_row() {
    let mut input = stable_input();
    input.install_posture.revocation_posture_class = "quarantined".to_string();
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "withdrawn");
    assert!(packet.downgraded_banner.must_display);
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"revocation_posture_quarantined".to_string()));
}

#[test]
fn advisory_revocation_posture_narrows_to_beta() {
    let mut input = stable_input();
    input.install_posture.revocation_posture_class = "advisory".to_string();
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"revocation_posture_advisory".to_string()));
}

#[test]
fn not_mirrorable_narrows_to_beta() {
    let mut input = stable_input();
    input.install_posture.mirrorability_class = "not_mirrorable".to_string();
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(packet
        .claim
        .downgrade_reasons
        .contains(&"not_mirrorable".to_string()));
}

#[test]
fn honest_beta_claim_passes_through() {
    let mut input = stable_input();
    input.claim.claimed_tier = "beta".to_string();
    // Even with a shimmed mapping, an honest beta claim is not narrowed further.
    input.mapping_outcome.outcome_class = "shimmed".to_string();
    let packet = StableMirrorImportTruthPacket::from_input(input).expect("must build");
    assert_eq!(packet.claim.effective_tier, "beta");
    assert!(!packet.claim.downgraded);
}

#[test]
fn current_continuity_requires_a_continuity_packet_ref() {
    let mut input = stable_input();
    input.continuity.continuity_packet_ref = None;
    let result = StableMirrorImportTruthPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn unknown_continuity_event_is_rejected() {
    let mut input = stable_input();
    input.continuity.continuity_event_class = "vanity_rename".to_string();
    let result = StableMirrorImportTruthPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn unknown_import_route_is_rejected() {
    let mut input = stable_input();
    input.source_class.import_route_class = "torrent".to_string();
    let result = StableMirrorImportTruthPacket::from_input(input);
    assert!(result.is_err());
}

#[test]
fn support_export_preserves_offline_and_continuity_truth() {
    let packet = StableMirrorImportTruthPacket::from_input(stable_input()).expect("must build");
    let export = project_stable_mirror_import_truth_support_export(&packet);
    assert!(!export.blocks_stable_import_truth);
    assert_eq!(export.import_route_class, packet.source_class.import_route_class);
    assert!(export.offline_explainable);
    assert!(export.last_known_good_pinned);
    assert!(export.audit_lineage_preserved);
    assert!(export.transfer_history_preserved);
    assert!(export.export_safe_summary.contains("Continuity"));
    assert!(export.export_safe_summary.contains("Mapping="));
}
