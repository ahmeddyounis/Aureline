use super::*;

const PACKET_ID: &str = "durable-test-discovery:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn node(node_id: &str, node_kind: DurableTestNodeKind) -> DurableTestNode {
    DurableTestNode {
        node_id: node_id.to_owned(),
        node_kind,
        parent_node_id: None,
        display_label: format!("display:{node_id}"),
        identity_basis_token: format!("basis:{node_id}"),
        identity_class: TestItemIdentityClass::Stable,
        source_state: NodeSourceState::LocalResolved,
        template_node_id: None,
        invocation_key: None,
        notebook_linkage: None,
        prior_identity_chain: Vec::new(),
        evidence_refs: refs(&[&format!("evidence:node:{node_id}")]),
    }
}

fn invocation(node_id: &str, template_node_id: &str, key: &str) -> DurableTestNode {
    let mut n = node(node_id, DurableTestNodeKind::ConcreteInvocation);
    n.template_node_id = Some(template_node_id.to_owned());
    n.invocation_key = Some(key.to_owned());
    n
}

fn notebook_node(node_id: &str) -> DurableTestNode {
    let mut n = node(node_id, DurableTestNodeKind::NotebookLinkedTest);
    n.notebook_linkage = Some(NotebookLinkage {
        notebook_id: format!("nb:{node_id}"),
        cell_id: format!("cell:{node_id}"),
        cell_ordinal: 3,
    });
    n
}

fn moved_node(node_id: &str) -> DurableTestNode {
    node(node_id, DurableTestNodeKind::ConcreteCase)
        .degrade_to_needs_remap(format!("basis:{node_id}:relocated"))
}

fn snapshot(
    snapshot_id: &str,
    consumer: DiscoveryConsumerKind,
    partiality: DiscoveryPartiality,
    mapping_support_class: MappingSupportClass,
    nodes: Vec<DurableTestNode>,
    omitted_scopes: Vec<OmittedScope>,
) -> DiscoverySnapshot {
    DiscoverySnapshot {
        snapshot_id: snapshot_id.to_owned(),
        consumer,
        label: format!("label:{snapshot_id}"),
        partiality,
        mapping_support_class,
        nodes,
        omitted_scopes,
        imported_not_shown_as_local: true,
        evidence_refs: refs(&[&format!("evidence:snapshot:{snapshot_id}")]),
    }
}

fn omitted(scope_id: &str, reason: OmittedScopeReason) -> OmittedScope {
    OmittedScope {
        scope_id: scope_id.to_owned(),
        reason,
        label: format!("omitted because {}", reason.as_str()),
        recoverable: true,
    }
}

fn framework_snapshot() -> DiscoverySnapshot {
    let suite = node("framework:suite:0001", DurableTestNodeKind::Suite);
    let case = node("framework:case:0001", DurableTestNodeKind::ConcreteCase);
    let template = node(
        "framework:template:0001",
        DurableTestNodeKind::ParameterizedTemplate,
    );
    let inv_a = invocation(
        "framework:invocation:0001",
        "framework:template:0001",
        "x=1",
    );
    let inv_b = invocation(
        "framework:invocation:0002",
        "framework:template:0001",
        "x=2",
    );
    snapshot(
        "snapshot:framework:0001",
        DiscoveryConsumerKind::FrameworkPack,
        DiscoveryPartiality::Complete,
        MappingSupportClass::FullyMappedLocal,
        vec![suite, case, template, inv_a, inv_b],
        Vec::new(),
    )
}

fn notebook_snapshot() -> DiscoverySnapshot {
    snapshot(
        "snapshot:notebook:0001",
        DiscoveryConsumerKind::Notebook,
        DiscoveryPartiality::PartialVisible,
        MappingSupportClass::PartiallyMappedVisible,
        vec![notebook_node("notebook:test:0001")],
        vec![omitted(
            "omitted:notebook:0001",
            OmittedScopeReason::NotYetStreamed,
        )],
    )
}

fn test_tree_snapshot() -> DiscoverySnapshot {
    snapshot(
        "snapshot:test-tree:0001",
        DiscoveryConsumerKind::TestTree,
        DiscoveryPartiality::Heuristic,
        MappingSupportClass::NeedsRemapPreserved,
        vec![
            node("tree:case:0001", DurableTestNodeKind::ConcreteCase),
            moved_node("tree:case:0002"),
        ],
        vec![omitted(
            "omitted:tree:0001",
            OmittedScopeReason::ParseErrorIsolated,
        )],
    )
}

fn imported_snapshot() -> DiscoverySnapshot {
    let mut imported = node("ci:case:0001", DurableTestNodeKind::ConcreteCase);
    imported.identity_class = TestItemIdentityClass::ImportedReadOnly;
    imported.source_state = NodeSourceState::ImportedReadOnly;
    snapshot(
        "snapshot:ci:0001",
        DiscoveryConsumerKind::ImportedCi,
        DiscoveryPartiality::ProviderImported,
        MappingSupportClass::ImportedReadOnlyMapped,
        vec![imported],
        vec![omitted(
            "omitted:ci:0001",
            OmittedScopeReason::ProviderOwnedScope,
        )],
    )
}

fn guardrails() -> DurableDiscoveryGuardrails {
    DurableDiscoveryGuardrails {
        display_labels_never_substitute_identity: true,
        templates_distinct_from_invocations: true,
        notebook_tests_distinct_from_file_tests: true,
        partial_discovery_stays_visible: true,
        imported_never_masquerades_as_local: true,
        remap_preserves_identity_chain: true,
    }
}

fn consumer_projection() -> DurableDiscoveryConsumerProjection {
    DurableDiscoveryConsumerProjection {
        framework_pack_ingests_nodes: true,
        notebook_ingests_nodes: true,
        test_tree_ingests_nodes: true,
        support_export_ingests_partiality: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        DURABLE_TEST_DISCOVERY_SCHEMA_REF,
        DURABLE_TEST_DISCOVERY_DOC_REF,
        DURABLE_TEST_DISCOVERY_ARTIFACT_REF,
    ])
}

fn valid_packet() -> DurableTestDiscoveryPacket {
    DurableTestDiscoveryPacket::new(DurableTestDiscoveryPacketInput {
        packet_id: PACKET_ID.to_owned(),
        label: "M5 Durable Test-Item Discovery".to_owned(),
        snapshots: vec![
            framework_snapshot(),
            notebook_snapshot(),
            test_tree_snapshot(),
            imported_snapshot(),
        ],
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

#[test]
fn valid_packet_has_no_violations() {
    let packet = valid_packet();
    assert!(
        packet.validate().is_empty(),
        "expected clean packet: {:?}",
        packet.validate()
    );
}

#[test]
fn all_node_kinds_and_consumers_are_represented() {
    let packet = valid_packet();
    assert_eq!(
        packet.represented_node_kinds().len(),
        DurableTestNodeKind::ALL.len()
    );
    assert!(packet
        .represented_consumers()
        .contains(&DiscoveryConsumerKind::FrameworkPack));
    assert!(packet
        .represented_consumers()
        .contains(&DiscoveryConsumerKind::Notebook));
    assert!(packet
        .represented_consumers()
        .contains(&DiscoveryConsumerKind::TestTree));
}

#[test]
fn template_and_invocation_keep_distinct_ids() {
    let snapshot = framework_snapshot();
    let template = snapshot
        .nodes
        .iter()
        .find(|n| n.is_template())
        .expect("template node");
    let invocations: Vec<_> = snapshot
        .nodes
        .iter()
        .filter(|n| n.is_invocation())
        .collect();
    assert_eq!(invocations.len(), 2);
    for inv in invocations {
        assert_ne!(inv.node_id, template.node_id);
        assert_eq!(
            inv.template_node_id.as_deref(),
            Some(template.node_id.as_str())
        );
    }
    assert!(snapshot.template_invocation_integrity());
}

#[test]
fn invocation_without_template_in_snapshot_fails_integrity() {
    let mut snapshot = framework_snapshot();
    // Remove the template node; the invocations now point at a missing template.
    snapshot.nodes.retain(|n| !n.is_template());
    assert!(!snapshot.template_invocation_integrity());

    let mut packet = valid_packet();
    packet.snapshots[0] = snapshot;
    let violations = packet.validate();
    assert!(violations.contains(&DurableTestDiscoveryViolation::TemplateCollapsedWithInvocation));
}

#[test]
fn invocation_aliased_to_template_id_is_rejected() {
    let mut snapshot = framework_snapshot();
    // Collapse a concrete invocation onto the template's own id.
    let template_id = "framework:template:0001".to_owned();
    let inv = snapshot
        .nodes
        .iter_mut()
        .find(|n| n.is_invocation())
        .expect("invocation");
    inv.template_node_id = Some(template_id.clone());
    inv.node_id = template_id;
    assert!(!snapshot.node_ids_unique() || !snapshot.nodes[3].linkage_consistent());

    let mut packet = valid_packet();
    packet.snapshots[0] = snapshot;
    assert!(!packet.validate().is_empty());
}

#[test]
fn notebook_node_distinct_from_file_backed_case() {
    let nb = notebook_node("nb:0001");
    assert!(nb.is_notebook_linked());
    assert!(nb.linkage_consistent());
    assert!(nb.notebook_linkage.is_some());

    let case = node("case:0001", DurableTestNodeKind::ConcreteCase);
    assert!(case.notebook_linkage.is_none());
    assert!(case.linkage_consistent());
}

#[test]
fn notebook_node_without_linkage_is_invalid() {
    let mut nb = notebook_node("nb:0001");
    nb.notebook_linkage = None;
    assert!(!nb.linkage_consistent());
    assert!(!nb.is_valid());
}

#[test]
fn display_label_cannot_substitute_identity() {
    let mut n = node("case:0001", DurableTestNodeKind::ConcreteCase);
    n.identity_basis_token = n.display_label.clone();
    assert!(!n.identity_independent_of_display_name());

    let mut packet = valid_packet();
    packet.snapshots[0].nodes[1] = n;
    let violations = packet.validate();
    assert!(violations.contains(&DurableTestDiscoveryViolation::DisplayLabelSubstitutesIdentity));
}

#[test]
fn partial_snapshot_without_omitted_scope_is_rejected() {
    let mut packet = valid_packet();
    // Notebook snapshot is PartialVisible; drop its omitted scope.
    packet.snapshots[1].omitted_scopes.clear();
    assert!(!packet.snapshots[1].partial_visibility_ok());
    let violations = packet.validate();
    assert!(violations.contains(&DurableTestDiscoveryViolation::PartialDiscoveryHidden));
}

#[test]
fn complete_snapshot_with_omitted_scope_is_rejected() {
    let mut packet = valid_packet();
    // Framework snapshot is Complete; an omitted scope contradicts completeness.
    packet.snapshots[0]
        .omitted_scopes
        .push(omitted("stray", OmittedScopeReason::DiscoveryTimedOut));
    assert!(!packet.snapshots[0].partial_visibility_ok());
    assert!(packet
        .validate()
        .contains(&DurableTestDiscoveryViolation::PartialDiscoveryHidden));
}

#[test]
fn imported_must_not_read_as_local() {
    let mut packet = valid_packet();
    packet.snapshots[3].imported_not_shown_as_local = false;
    assert!(!packet.snapshots[3].imported_separation_ok());
    assert!(packet
        .validate()
        .contains(&DurableTestDiscoveryViolation::ImportedShownAsLocal));
}

#[test]
fn source_move_degrades_to_needs_remap_and_preserves_chain() {
    let original = node("case:0001", DurableTestNodeKind::ConcreteCase);
    let original_basis = original.identity_basis_token.clone();
    let moved = original.degrade_to_needs_remap("basis:case:0001:v2".to_owned());

    assert_eq!(
        moved.identity_class,
        TestItemIdentityClass::RemapReviewRequired
    );
    assert_eq!(moved.source_state, NodeSourceState::SourceMovedNeedsRemap);
    assert_eq!(moved.identity_basis_token, "basis:case:0001:v2");
    assert_eq!(moved.prior_identity_chain, vec![original_basis]);
    assert!(moved.remap_preserves_chain());
    assert!(moved.is_valid());
}

#[test]
fn needs_remap_without_chain_is_rejected() {
    let mut moved = moved_node("case:0001");
    moved.prior_identity_chain.clear();
    assert!(!moved.remap_preserves_chain());

    let mut packet = valid_packet();
    packet.snapshots[2].nodes[1] = moved;
    assert!(packet
        .validate()
        .contains(&DurableTestDiscoveryViolation::RemapChainLost));
}

#[test]
fn mapping_support_class_must_match_truth() {
    let mut packet = valid_packet();
    // Claim a fully-mapped-local class on a partial notebook snapshot.
    packet.snapshots[1].mapping_support_class = MappingSupportClass::FullyMappedLocal;
    assert!(!packet.snapshots[1].mapping_support_consistent());
    assert!(packet
        .validate()
        .contains(&DurableTestDiscoveryViolation::MappingSupportInconsistent));
}

#[test]
fn missing_required_consumer_is_rejected() {
    let mut packet = valid_packet();
    // Drop the test-tree snapshot.
    packet
        .snapshots
        .retain(|s| s.consumer != DiscoveryConsumerKind::TestTree);
    assert!(packet
        .validate()
        .contains(&DurableTestDiscoveryViolation::RequiredConsumerMissing));
}

#[test]
fn from_canonical_item_maps_kinds() {
    use crate::testing_identity::CanonicalTestItem;

    let item = CanonicalTestItem {
        record_kind: crate::testing_identity::CANONICAL_TEST_ITEM_RECORD_KIND.to_owned(),
        schema_version: crate::testing_identity::TEST_IDENTITY_BETA_SCHEMA_VERSION,
        canonical_test_item_id: "item:0001".to_owned(),
        adapter_kind: crate::testing_identity::TestAdapterKind::Pytest,
        adapter_kind_token: "pytest".to_owned(),
        item_kind: CanonicalTestItemKind::ParameterizedInstance,
        item_kind_token: CanonicalTestItemKind::ParameterizedInstance
            .as_str()
            .to_owned(),
        adapter_item_ref: "tests/test_x.py::test_y[1]".to_owned(),
        logical_item_key: "tests/test_x.py::test_y::param".to_owned(),
        source_anchor_ref: "tests/test_x.py:10".to_owned(),
        source_anchor_digest: "digest".to_owned(),
        selector_ref: "tests/test_x.py::test_y[1]".to_owned(),
        display_label_digest: "label-digest".to_owned(),
        identity_class: TestItemIdentityClass::Stable,
        identity_class_token: TestItemIdentityClass::Stable.as_str().to_owned(),
        parameterized_family_ref: Some("item:family:0001".to_owned()),
        parameterized_instance_key: Some("1".to_owned()),
        remap_record_refs: Vec::new(),
    };

    let durable = DurableTestNode::from_canonical_item(
        &item,
        "test_y[1]".to_owned(),
        refs(&["evidence:item:0001"]),
    );
    assert_eq!(durable.node_kind, DurableTestNodeKind::ConcreteInvocation);
    assert_eq!(durable.node_id, "item:0001");
    assert_eq!(
        durable.template_node_id.as_deref(),
        Some("item:family:0001")
    );
    assert_eq!(durable.invocation_key.as_deref(), Some("1"));
    assert_eq!(
        durable.identity_basis_token,
        "tests/test_x.py::test_y::param"
    );
    assert!(durable.identity_independent_of_display_name());
}

#[test]
fn export_safe_json_round_trips() {
    let packet = valid_packet();
    let json = packet.export_safe_json();
    let parsed: DurableTestDiscoveryPacket =
        serde_json::from_str(&json).expect("round trip parses");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn markdown_summary_mentions_partiality_and_remap() {
    let packet = valid_packet();
    let md = packet.render_markdown_summary();
    assert!(md.contains("partiality"));
    assert!(md.contains("prior_chain"));
    assert!(md.contains("omitted"));
}

#[test]
fn guardrails_must_all_hold() {
    let mut packet = valid_packet();
    packet.guardrails.partial_discovery_stays_visible = false;
    assert!(packet
        .validate()
        .contains(&DurableTestDiscoveryViolation::GuardrailsIncomplete));
}
