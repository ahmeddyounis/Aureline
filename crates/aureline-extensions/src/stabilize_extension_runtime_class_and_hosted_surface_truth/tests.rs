//! Unit and fixture coverage for stable runtime-class and hosted-surface truth.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PacketFixture {
    case_name: String,
    packet_input: RuntimeClassTruthInput,
    expected: ExpectedPacket,
}

#[derive(Debug, Deserialize)]
struct ExpectedPacket {
    effective_tier: String,
    support_claim_class: String,
    downgrade_reasons: Vec<String>,
    required_consumers_present: bool,
    runtime_classes_verified: bool,
    active_inspectors_present: bool,
    inspector_attribution_complete: bool,
    inspector_actions_complete: bool,
    downgrade_banners_complete: bool,
    downgraded_host_active: bool,
    hosted_surface_boundaries_complete: bool,
    hosted_surface_handoffs_complete: bool,
    authoring_flows_use_public_vocabulary: bool,
    surface_coverage_complete: bool,
    blocks_stable_runtime_truth: bool,
}

fn all_fixtures() -> Vec<PacketFixture> {
    let raws: &[&str] = &[
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-extension-runtime-class-and-hosted-surface-truth/stable_full_truth_current.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-extension-runtime-class-and-hosted-surface-truth/downgraded_bridge_with_banner_narrows_to_beta.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-extension-runtime-class-and-hosted-surface-truth/hosted_surface_missing_chrome_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-extension-runtime-class-and-hosted-surface-truth/inspector_missing_actions_narrows_to_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-extension-runtime-class-and-hosted-surface-truth/local_dev_vocabulary_drift_narrows_to_preview.json"
        )),
    ];
    raws.iter()
        .map(|raw| serde_json::from_str(raw).expect("fixture must parse"))
        .collect()
}

#[test]
fn every_fixture_builds_validates_and_matches_expectations() {
    let fixtures = all_fixtures();
    assert_eq!(fixtures.len(), 5, "all canonical fixtures must load");

    for fixture in &fixtures {
        let packet = RuntimeClassTruthPacket::from_input(fixture.packet_input.clone())
            .unwrap_or_else(|e| panic!("fixture {:?} must build: {e}", fixture.case_name));
        packet.validate().expect("packet must validate");

        let payload = serde_json::to_string(&packet).expect("serialize packet");
        let projection = project_runtime_class_truth(&payload)
            .unwrap_or_else(|e| panic!("fixture {:?} must project: {e}", fixture.case_name));
        let export = project_runtime_class_truth_support_export(&packet);
        let e = &fixture.expected;

        assert_eq!(
            packet.claim.effective_tier, e.effective_tier,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.claim.support_claim_class, e.support_claim_class,
            "{}",
            fixture.case_name
        );

        let mut got = packet.claim.downgrade_reasons.clone();
        got.sort();
        let mut want = e.downgrade_reasons.clone();
        want.sort();
        assert_eq!(got, want, "fixture {} downgrade reasons", fixture.case_name);

        assert_eq!(
            packet.inspection.required_consumers_present, e.required_consumers_present,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.runtime_classes_verified, e.runtime_classes_verified,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.active_inspectors_present, e.active_inspectors_present,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.inspector_attribution_complete, e.inspector_attribution_complete,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.inspector_actions_complete, e.inspector_actions_complete,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.downgrade_banners_complete, e.downgrade_banners_complete,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.downgraded_host_active, e.downgraded_host_active,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.hosted_surface_boundaries_complete,
            e.hosted_surface_boundaries_complete,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.hosted_surface_handoffs_complete, e.hosted_surface_handoffs_complete,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.authoring_flows_use_public_vocabulary,
            e.authoring_flows_use_public_vocabulary,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.surface_coverage_complete, e.surface_coverage_complete,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            export.blocks_stable_runtime_truth, e.blocks_stable_runtime_truth,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            projection.blocks_stable_runtime_truth, e.blocks_stable_runtime_truth,
            "{}",
            fixture.case_name
        );

        assert!(!packet.allows_catalog_only_truth);
        assert!(!packet.allows_hidden_hosted_surface_boundary);
        assert!(!packet.allows_generic_extension_badge_only);

        if packet.claim.effective_tier == "stable" {
            assert!(packet.inspection.required_consumers_present);
            assert!(packet.inspection.runtime_classes_verified);
            assert!(packet.inspection.active_inspectors_present);
            assert!(packet.inspection.inspector_attribution_complete);
            assert!(packet.inspection.inspector_actions_complete);
            assert!(packet.inspection.hosted_surface_boundaries_complete);
            assert!(packet.inspection.hosted_surface_handoffs_complete);
            assert!(packet.inspection.authoring_flows_use_public_vocabulary);
            assert!(packet.inspection.surface_coverage_complete);
            assert!(packet.claim.downgrade_reasons.is_empty());
        }
    }
}

#[test]
fn runtime_class_labels_are_closed_and_user_facing() {
    assert_eq!(
        runtime_class_label("passive_package"),
        Some("Passive package")
    );
    assert_eq!(
        runtime_class_label("wasm_capability_sandbox"),
        Some("Wasm capability sandbox")
    );
    assert_eq!(
        runtime_class_label("declarative_host_rendered_view"),
        Some("Declarative/host-rendered view")
    );
    assert_eq!(runtime_class_label("external_host"), Some("External host"));
    assert_eq!(
        runtime_class_label("compatibility_bridge"),
        Some("Compatibility bridge")
    );
    assert_eq!(
        runtime_class_label("remote_side_component"),
        Some("Remote-side component")
    );
    assert_eq!(runtime_class_label("generic_extension"), None);
}
