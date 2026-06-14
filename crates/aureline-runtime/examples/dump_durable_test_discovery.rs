//! Conformance dump for the M5 durable test-item discovery packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::durable_test_items_and_partial_discovery::*;
use aureline_runtime::testing_identity::TestItemIdentityClass;

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

fn child(node_id: &str, node_kind: DurableTestNodeKind, parent: &str) -> DurableTestNode {
    let mut n = node(node_id, node_kind);
    n.parent_node_id = Some(parent.to_owned());
    n
}

fn invocation(node_id: &str, template_node_id: &str, key: &str) -> DurableTestNode {
    let mut n = child(
        node_id,
        DurableTestNodeKind::ConcreteInvocation,
        template_node_id,
    );
    n.template_node_id = Some(template_node_id.to_owned());
    n.invocation_key = Some(key.to_owned());
    n
}

fn notebook_node(node_id: &str, notebook_id: &str, cell_id: &str, ordinal: u32) -> DurableTestNode {
    let mut n = node(node_id, DurableTestNodeKind::NotebookLinkedTest);
    n.notebook_linkage = Some(NotebookLinkage {
        notebook_id: notebook_id.to_owned(),
        cell_id: cell_id.to_owned(),
        cell_ordinal: ordinal,
    });
    n
}

fn omitted(scope_id: &str, reason: OmittedScopeReason, label: &str) -> OmittedScope {
    OmittedScope {
        scope_id: scope_id.to_owned(),
        reason,
        label: label.to_owned(),
        recoverable: true,
    }
}

fn framework_snapshot() -> DiscoverySnapshot {
    let suite = node("framework:suite:checkout", DurableTestNodeKind::Suite);
    let case = child(
        "framework:case:add-item",
        DurableTestNodeKind::ConcreteCase,
        "framework:suite:checkout",
    );
    let template = child(
        "framework:template:totals",
        DurableTestNodeKind::ParameterizedTemplate,
        "framework:suite:checkout",
    );
    let inv_a = invocation(
        "framework:invocation:totals:usd",
        "framework:template:totals",
        "currency=usd",
    );
    let inv_b = invocation(
        "framework:invocation:totals:eur",
        "framework:template:totals",
        "currency=eur",
    );
    DiscoverySnapshot {
        snapshot_id: "snapshot:framework-pack:checkout".to_owned(),
        consumer: DiscoveryConsumerKind::FrameworkPack,
        label: "Framework pack test explorer with a complete local enumeration".to_owned(),
        partiality: DiscoveryPartiality::Complete,
        mapping_support_class: MappingSupportClass::FullyMappedLocal,
        nodes: vec![suite, case, template, inv_a, inv_b],
        omitted_scopes: Vec::new(),
        imported_not_shown_as_local: true,
        evidence_refs: refs(&["evidence:snapshot:framework-pack:checkout"]),
    }
}

fn notebook_snapshot() -> DiscoverySnapshot {
    DiscoverySnapshot {
        snapshot_id: "snapshot:notebook:analysis".to_owned(),
        consumer: DiscoveryConsumerKind::Notebook,
        label: "Notebook test cells; later cells still streaming in".to_owned(),
        partiality: DiscoveryPartiality::Streaming,
        mapping_support_class: MappingSupportClass::PartiallyMappedVisible,
        nodes: vec![
            notebook_node(
                "notebook:test:analysis:cell-2",
                "notebook:analysis",
                "cell:0002",
                2,
            ),
            notebook_node(
                "notebook:test:analysis:cell-5",
                "notebook:analysis",
                "cell:0005",
                5,
            ),
        ],
        omitted_scopes: vec![omitted(
            "omitted:notebook:tail-cells",
            OmittedScopeReason::NotYetStreamed,
            "Cells after ordinal 5 have not yet streamed in; the tail of the notebook stays visibly pending",
        )],
        imported_not_shown_as_local: true,
        evidence_refs: refs(&["evidence:snapshot:notebook:analysis"]),
    }
}

fn test_tree_snapshot() -> DiscoverySnapshot {
    let resolved = node("tree:case:login", DurableTestNodeKind::ConcreteCase);
    let moved = node("tree:case:logout", DurableTestNodeKind::ConcreteCase)
        .degrade_to_needs_remap("basis:tree:case:logout:relocated".to_owned());
    DiscoverySnapshot {
        snapshot_id: "snapshot:test-tree:aggregate".to_owned(),
        consumer: DiscoveryConsumerKind::TestTree,
        label: "Aggregate test tree with one node whose source moved and now needs remap".to_owned(),
        partiality: DiscoveryPartiality::Heuristic,
        mapping_support_class: MappingSupportClass::NeedsRemapPreserved,
        nodes: vec![resolved, moved],
        omitted_scopes: vec![omitted(
            "omitted:test-tree:adapter-down",
            OmittedScopeReason::AdapterUnavailable,
            "One framework adapter was unavailable; its subtree is recorded as omitted rather than dropped",
        )],
        imported_not_shown_as_local: true,
        evidence_refs: refs(&["evidence:snapshot:test-tree:aggregate"]),
    }
}

fn imported_snapshot() -> DiscoverySnapshot {
    let mut imported = node("ci:case:smoke", DurableTestNodeKind::ConcreteCase);
    imported.identity_class = TestItemIdentityClass::ImportedReadOnly;
    imported.source_state = NodeSourceState::ImportedReadOnly;
    DiscoverySnapshot {
        snapshot_id: "snapshot:imported-ci:smoke".to_owned(),
        consumer: DiscoveryConsumerKind::ImportedCi,
        label: "Imported CI overlay; read-only and never shown as a live local rerun".to_owned(),
        partiality: DiscoveryPartiality::ProviderImported,
        mapping_support_class: MappingSupportClass::ImportedReadOnlyMapped,
        nodes: vec![imported],
        omitted_scopes: vec![omitted(
            "omitted:imported-ci:provider-scope",
            OmittedScopeReason::ProviderOwnedScope,
            "The full provider scope is completed CI-side; the local overlay shows only the imported subset",
        )],
        imported_not_shown_as_local: true,
        evidence_refs: refs(&["evidence:snapshot:imported-ci:smoke"]),
    }
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
        "schemas/testing/test_item_identity.schema.json",
    ])
}

fn packet() -> DurableTestDiscoveryPacket {
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

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
