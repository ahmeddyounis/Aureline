//! Protected tests for alpha route-origin support reconstruction.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::bundle::{ExactBuildCapture, ReleaseChannelClass};
use aureline_support::route_origin_alpha::{
    current_alpha_route_origin_matrix, current_command_route_reconstruction_packet,
    current_route_origin_fixture_corpus, current_route_origin_fixture_manifest,
    current_route_origin_support_preview, ALPHA_ROUTE_ORIGIN_MATRIX_PATH,
    ALPHA_ROUTE_ORIGIN_MATRIX_RECORD_KIND, COMMAND_ROUTE_RECONSTRUCTION_PACKET_PATH,
    COMMAND_ROUTE_RECONSTRUCTION_RECORD_KIND, ROUTE_ORIGIN_ALPHA_FIXTURE_MANIFEST_PATH,
    TRANSPORT_DECISION_ALPHA_RECORD_KIND,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_capture() -> ExactBuildCapture {
    ExactBuildCapture::for_fixture(
        "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:abcdef123456",
        "0.0.0",
        ReleaseChannelClass::DevLocal,
    )
}

#[test]
fn route_origin_matrix_packet_and_fixtures_validate_together() {
    let root = repo_root();
    let matrix = current_alpha_route_origin_matrix().expect("matrix parses");
    let packet = current_command_route_reconstruction_packet().expect("packet parses");
    let fixture_manifest = current_route_origin_fixture_manifest().expect("manifest parses");
    let corpus = current_route_origin_fixture_corpus().expect("fixtures parse");

    assert_eq!(matrix.record_kind, ALPHA_ROUTE_ORIGIN_MATRIX_RECORD_KIND);
    assert_eq!(packet.record_kind, COMMAND_ROUTE_RECONSTRUCTION_RECORD_KIND);
    assert!(root.join(ALPHA_ROUTE_ORIGIN_MATRIX_PATH).exists());
    assert!(root.join(COMMAND_ROUTE_RECONSTRUCTION_PACKET_PATH).exists());
    assert!(root.join(ROUTE_ORIGIN_ALPHA_FIXTURE_MANIFEST_PATH).exists());

    let violations = matrix
        .validate_packet_and_corpus(&packet, &fixture_manifest, &corpus)
        .into_iter()
        .chain(packet.validate_with_corpus(&corpus))
        .collect::<Vec<_>>();
    assert_eq!(violations, Vec::new());

    for entry in &corpus.entries {
        assert_eq!(
            entry.decision.record_kind,
            TRANSPORT_DECISION_ALPHA_RECORD_KIND
        );
        assert!(
            root.join(&entry.fixture_ref).exists(),
            "{} should exist on disk",
            entry.fixture_ref
        );
    }
}

#[test]
fn route_origin_support_preview_reconstructs_all_required_states() {
    let preview = current_route_origin_support_preview(fixture_capture(), "2026-05-14T16:10:00Z")
        .expect("preview builds");

    assert!(preview.manifest.has_exact_build_identity());
    assert_eq!(preview.manifest.preview_items.len(), 9);
    assert_eq!(preview.manifest.action_reconstruction_contexts.len(), 9);
    assert!(preview
        .manifest
        .redaction_controls
        .iter()
        .all(|control| !control.raw_content_export_allowed));

    let states = preview
        .manifest
        .action_reconstruction_contexts
        .iter()
        .map(|context| {
            context
                .route_truth_state
                .as_deref()
                .expect("route truth state")
        })
        .collect::<BTreeSet<_>>();
    for expected in [
        "route_authoritative",
        "route_changed_declared",
        "degraded_browser_handoff",
        "denied_wrong_target",
        "denied_wrong_origin",
        "denied_hidden_relay",
        "denied_hidden_fallback",
    ] {
        assert!(states.contains(expected), "missing state {expected}");
    }

    let hidden_relay = preview
        .manifest
        .action_reconstruction_contexts
        .iter()
        .find(|context| context.decision_outcome.as_deref() == Some("deny_hidden_relay"))
        .expect("hidden relay context");
    assert_eq!(
        hidden_relay.traffic_origin.as_deref(),
        Some("managed_service")
    );
    assert_eq!(
        hidden_relay.fallback_posture.as_deref(),
        Some("hidden_relay_denied")
    );

    let browser_handoff = preview
        .manifest
        .action_reconstruction_contexts
        .iter()
        .find(|context| context.route_choice.as_deref() == Some("browser_handoff"))
        .expect("browser handoff context");
    assert_eq!(
        browser_handoff.fallback_posture.as_deref(),
        Some("browser_handoff_required")
    );

    let tunnel = preview
        .manifest
        .action_reconstruction_contexts
        .iter()
        .find(|context| context.route_choice.as_deref() == Some("tunnel_exposed_route"))
        .expect("tunnel route context");
    assert_eq!(tunnel.action_target_class, "tunnel_exposed_target");
    assert_eq!(tunnel.action_exposure_class, "tunnel_exposed_public");
}
