//! Fixture-driven tests for cross-surface safe-preview trust classes.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use serde::Deserialize;

use aureline_shell::previews::trust_classes::{
    build_trust_class_alpha_packet, TrustClassActionFamily, TrustClassActionPosture,
    TrustClassFallbackMode, TrustClassSurfaceInput, TRUST_CLASS_ALPHA_PACKET_RECORD_KIND,
    TRUST_CLASS_ALPHA_PACKET_SCHEMA_VERSION,
};

#[derive(Debug, Deserialize)]
struct TrustClassFixture {
    packet_id: String,
    surfaces: Vec<TrustClassSurfaceInput>,
    expected: TrustClassFixtureExpected,
}

#[derive(Debug, Deserialize)]
struct TrustClassFixtureExpected {
    required_lane_tokens: Vec<String>,
    required_trust_class_tokens: Vec<String>,
    degraded_surface_ids: Vec<String>,
    required_pre_action_disclosures: Vec<String>,
    expected_surface_actions: BTreeMap<String, Vec<String>>,
}

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("previews")
        .join("trust_class_alpha")
}

fn load_fixture(file_name: &str) -> TrustClassFixture {
    let path = fixtures_dir().join(file_name);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|err| panic!("failed to read fixture {}: {err}", path.display()));
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("invalid JSON in fixture {}: {err}", path.display()))
}

#[test]
fn core_surfaces_share_trust_class_vocabulary_and_fallbacks() {
    let fixture = load_fixture("core_surface_packet.json");
    let packet = build_trust_class_alpha_packet(fixture.packet_id, fixture.surfaces);

    assert_eq!(packet.record_kind, TRUST_CLASS_ALPHA_PACKET_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        TRUST_CLASS_ALPHA_PACKET_SCHEMA_VERSION
    );

    let violations = packet.validate();
    let violation_tokens: Vec<&str> = violations
        .iter()
        .map(|violation| violation.token())
        .collect();
    assert!(
        violations.is_empty(),
        "trust-class packet violations: {violation_tokens:?} {violations:?}",
    );
    assert!(packet.covers_required_lanes());
    assert!(packet.covers_all_trust_classes());
    assert!(packet.all_actions_disclose_pre_action_state());
    assert!(packet.copy_export_actions_have_representation());
    assert!(packet.degraded_surfaces_fallback_truthfully());
    assert!(packet.mutation_actions_require_preview_or_block());

    let observed_lanes: BTreeSet<_> = packet
        .surfaces
        .iter()
        .map(|surface| surface.lane_token.as_str())
        .collect();
    for lane in &fixture.expected.required_lane_tokens {
        assert!(
            observed_lanes.contains(lane.as_str()),
            "missing lane token {lane}"
        );
    }

    let observed_trust_classes: BTreeSet<_> = packet
        .surfaces
        .iter()
        .flat_map(|surface| {
            [
                surface.nominal_trust_class_token.as_str(),
                surface.effective_trust_class_token.as_str(),
            ]
        })
        .collect();
    for trust_class in &fixture.expected.required_trust_class_tokens {
        assert!(
            observed_trust_classes.contains(trust_class.as_str()),
            "missing trust class {trust_class}"
        );
    }

    for required in &fixture.expected.required_pre_action_disclosures {
        for surface in &packet.surfaces {
            for action in surface.all_actions() {
                assert!(
                    action
                        .required_pre_action_disclosures
                        .iter()
                        .any(|field| field == required),
                    "action {} on {} missing disclosure {}",
                    action.action_id,
                    surface.surface_id,
                    required
                );
            }
        }
    }

    for (surface_id, expected_actions) in &fixture.expected.expected_surface_actions {
        let surface = packet
            .surfaces
            .iter()
            .find(|surface| surface.surface_id == *surface_id)
            .unwrap_or_else(|| panic!("missing surface {surface_id}"));
        let observed_actions: BTreeSet<_> = surface
            .all_actions()
            .map(|action| action.action_id.as_str())
            .collect();
        for action_id in expected_actions {
            assert!(
                observed_actions.contains(action_id.as_str()),
                "surface {surface_id} missing action {action_id}"
            );
        }
    }

    for degraded_id in &fixture.expected.degraded_surface_ids {
        let surface = packet
            .surfaces
            .iter()
            .find(|surface| surface.surface_id == *degraded_id)
            .unwrap_or_else(|| panic!("missing degraded surface {degraded_id}"));
        assert!(surface.is_degraded);
        assert_eq!(
            surface.fallback_mode,
            TrustClassFallbackMode::SanitizedStaticSnapshot
        );
        assert_eq!(surface.nominal_trust_class_token, "IsolatedRemoteActive");
        assert_eq!(surface.effective_trust_class_token, "SanitizedRich");
        let run_action = surface
            .mutation_actions
            .iter()
            .find(|action| action.action_id == "run_active_preview")
            .expect("degraded preview keeps blocked active action visible");
        assert_eq!(run_action.action_family, TrustClassActionFamily::ActiveOpen);
        assert_eq!(run_action.posture, TrustClassActionPosture::Blocked);
        assert!(run_action.blocked_reason.is_some());
    }

    let package_surface = packet
        .surfaces
        .iter()
        .find(|surface| surface.surface_id == "surface:package:install-review")
        .expect("package install surface present");
    let apply_action = package_surface
        .mutation_actions
        .iter()
        .find(|action| action.action_id == "apply_package_action")
        .expect("package apply action present");
    assert_eq!(apply_action.action_family, TrustClassActionFamily::Mutate);
    assert_eq!(
        apply_action.posture,
        TrustClassActionPosture::RequiresPreview
    );

    let plaintext = packet.render_plaintext();
    assert!(plaintext.contains("trust=RawText effective=RawText visible=raw"));
    assert!(plaintext.contains("trust=IsolatedRemoteActive effective=SanitizedRich"));
    assert!(plaintext.contains("copy_escaped family=copy"));
}
