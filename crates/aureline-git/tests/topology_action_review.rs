//! Cross-crate coverage for reviewed topology-remediation action sheets.
//!
//! This exercises the public surface the way a downstream consumer (the review,
//! search, or AI lanes) would: load the canonical packet and the protected
//! fixtures, then confirm the no-wrong-root, distinct-verb, and reviewed-network
//! guarantees hold for every sheet.

use std::path::{Path, PathBuf};

use aureline_git::{
    current_topology_action_review_packet, TopologyActionApproval, TopologyActionKind,
    TopologyActionReviewPacket, TopologyOperationScope, WrongRootGuard,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/git/m5/widen-deepen-initialize-hydrate")
}

fn load_fixture(name: &str) -> TopologyActionReviewPacket {
    let path = fixtures_dir().join(name);
    let payload = std::fs::read_to_string(&path).expect("fixture must read");
    TopologyActionReviewPacket::parse_json(&payload)
        .unwrap_or_else(|error| panic!("fixture {path:?} must parse and validate: {error}"))
}

#[test]
fn checked_packet_validates_and_covers_every_verb() {
    let packet = current_topology_action_review_packet().expect("checked packet validates");
    let kinds: Vec<_> = packet
        .sheets
        .iter()
        .map(|sheet| sheet.action_kind)
        .collect();
    for kind in TopologyActionKind::ALL {
        assert!(kinds.contains(&kind), "verb {kind:?} is exercised");
    }
}

#[test]
fn each_verb_fixture_parses_and_keeps_its_distinct_verb() {
    for (name, expected) in [
        ("widen.json", TopologyActionKind::Widen),
        ("deepen.json", TopologyActionKind::Deepen),
        ("initialize.json", TopologyActionKind::Initialize),
        ("hydrate.json", TopologyActionKind::Hydrate),
    ] {
        let packet = load_fixture(name);
        assert_eq!(packet.sheets.len(), 1);
        assert_eq!(packet.sheets[0].action_kind, expected);
    }
}

#[test]
fn network_sheets_are_reviewed_and_local_widen_is_not() {
    let packet = current_topology_action_review_packet().expect("checked packet validates");
    for sheet in &packet.sheets {
        if sheet.action_kind.is_network_bearing() {
            assert!(sheet.network.reaches_network);
            assert!(sheet.network.egress_ref.is_some());
            assert!(matches!(
                sheet.approval,
                TopologyActionApproval::ApprovalRequired
                    | TopologyActionApproval::Approved
                    | TopologyActionApproval::PolicyBlocked
            ));
            assert!(!sheet.recovery.recovery_ref.is_empty());
        } else {
            assert!(!sheet.network.reaches_network);
        }
    }
}

#[test]
fn wrong_root_fixture_is_guarded_and_not_executable() {
    let packet = load_fixture("wrong_root.json");
    let sheet = &packet.sheets[0];
    assert_eq!(
        sheet.wrong_root_guard,
        WrongRootGuard::RetargetRequiredWrongRoot
    );
    assert_eq!(
        sheet.safe_operation_scope,
        TopologyOperationScope::MutationDenied
    );
    assert_ne!(sheet.approval, TopologyActionApproval::Approved);
    assert!(!sheet.is_executable());
}

#[test]
fn multi_root_fixture_broadens_only_behind_an_explicit_preview() {
    let packet = load_fixture("multi_root.json");
    let sheet = &packet.sheets[0];
    assert!(sheet.multi_root_preview.broadened);
    assert!(!sheet.multi_root_preview.additional_root_refs.is_empty());
    assert_eq!(
        sheet.safe_operation_scope,
        TopologyOperationScope::ExplicitMultiRootPreviewRequired
    );
}
