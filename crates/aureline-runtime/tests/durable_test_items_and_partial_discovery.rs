use aureline_runtime::durable_test_items_and_partial_discovery::{
    current_durable_test_discovery_export, DiscoveryConsumerKind, DiscoveryPartiality,
    DurableTestDiscoveryPacket, DurableTestNodeKind, MappingSupportClass, NodeSourceState,
};
use aureline_runtime::testing_identity::TestItemIdentityClass;

fn fixture(name: &str) -> DurableTestDiscoveryPacket {
    let path = format!(
        "{}/../../fixtures/testing/m5/durable-test-items-and-partial-discovery/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_validates() {
    let packet = current_durable_test_discovery_export()
        .expect("checked-in durable test discovery export should validate");
    assert!(packet.validate().is_empty());

    for consumer in [
        DiscoveryConsumerKind::FrameworkPack,
        DiscoveryConsumerKind::Notebook,
        DiscoveryConsumerKind::TestTree,
    ] {
        assert!(
            packet.represented_consumers().contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }

    // Every durable node kind is represented, so templates, invocations, and
    // notebook-linked tests are all distinguishable through stable ids.
    assert_eq!(
        packet.represented_node_kinds().len(),
        DurableTestNodeKind::ALL.len()
    );
}

#[test]
fn template_and_invocation_have_distinct_stable_ids() {
    let packet = current_durable_test_discovery_export().expect("export validates");
    let framework = packet
        .snapshots
        .iter()
        .find(|s| s.consumer == DiscoveryConsumerKind::FrameworkPack)
        .expect("framework snapshot");
    assert!(framework.template_invocation_integrity());

    let template = framework
        .nodes
        .iter()
        .find(|n| n.is_template())
        .expect("template node");
    let invocations: Vec<_> = framework
        .nodes
        .iter()
        .filter(|n| n.is_invocation())
        .collect();
    assert!(invocations.len() >= 2);
    for inv in invocations {
        assert_ne!(
            inv.node_id, template.node_id,
            "invocation must not share the template's row identity"
        );
        assert_eq!(
            inv.template_node_id.as_deref(),
            Some(template.node_id.as_str())
        );
    }
}

#[test]
fn partial_discovery_stays_visible_in_fixture() {
    let packet = fixture("discovery_snapshot_preserves_identity_chain_on_source_move.json");
    assert!(packet.validate().is_empty());
    assert!(packet.partial_snapshot_count() >= 1);

    let notebook = packet
        .snapshots
        .iter()
        .find(|s| s.consumer == DiscoveryConsumerKind::Notebook)
        .expect("notebook snapshot");
    assert_ne!(notebook.partiality, DiscoveryPartiality::Complete);
    assert!(
        !notebook.omitted_scopes.is_empty(),
        "partial discovery must keep its uncovered scope visible"
    );
    assert!(notebook.partial_visibility_ok());
}

#[test]
fn source_move_keeps_prior_identity_chain() {
    let packet = fixture("discovery_snapshot_preserves_identity_chain_on_source_move.json");
    let tree = packet
        .snapshots
        .iter()
        .find(|s| s.consumer == DiscoveryConsumerKind::TestTree)
        .expect("test tree snapshot");
    let moved = tree
        .nodes
        .iter()
        .find(|n| n.source_state == NodeSourceState::SourceMovedNeedsRemap)
        .expect("a node whose source moved");
    assert_eq!(
        moved.identity_class,
        TestItemIdentityClass::RemapReviewRequired
    );
    assert!(
        !moved.prior_identity_chain.is_empty(),
        "needs-remap node must keep its prior durable identity chain"
    );
    assert!(moved.remap_preserves_chain());
}

#[test]
fn imported_overlay_never_reads_as_local() {
    let packet = fixture("discovery_snapshot_preserves_identity_chain_on_source_move.json");
    let imported = packet
        .snapshots
        .iter()
        .find(|s| s.consumer == DiscoveryConsumerKind::ImportedCi)
        .expect("imported ci snapshot");
    assert_eq!(
        imported.mapping_support_class,
        MappingSupportClass::ImportedReadOnlyMapped
    );
    assert!(imported.imported_not_shown_as_local);
    assert!(imported.imported_separation_ok());
}

#[test]
fn dropping_remap_chain_is_rejected() {
    let mut packet = fixture("discovery_snapshot_preserves_identity_chain_on_source_move.json");
    let tree = packet
        .snapshots
        .iter_mut()
        .find(|s| s.consumer == DiscoveryConsumerKind::TestTree)
        .expect("test tree snapshot");
    let moved = tree
        .nodes
        .iter_mut()
        .find(|n| n.source_state == NodeSourceState::SourceMovedNeedsRemap)
        .expect("a node whose source moved");
    moved.prior_identity_chain.clear();

    let violations = packet.validate();
    assert!(
        !violations.is_empty(),
        "dropping the chain must fail validation"
    );
}
