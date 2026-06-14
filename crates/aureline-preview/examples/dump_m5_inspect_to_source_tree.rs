//! Conformance dump for the M5 inspect-to-source tree mapping packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_preview::inspect_to_source_tree::*;

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:node:{id}")]
}

fn base(
    node_id: &str,
    tree_kind: InspectTreeKind,
    label: &str,
    mapping_quality: NodeMappingQualityClass,
    runtime_backed: bool,
    source_navigation_offered: bool,
) -> InspectNode {
    InspectNode {
        node_id: node_id.to_owned(),
        tree_kind,
        label_summary: label.to_owned(),
        observed_at: "2026-06-07T00:00:00Z".to_owned(),
        mapping_quality,
        continuity_route: mapping_quality.required_continuity_route(),
        mapping_label_resolved: true,
        source_navigation_offered,
        mutation_offered: false,
        previews_source_diff_before_commit: false,
        runtime_backed,
        claims_saved_source: false,
        source_anchor_ref: if mapping_quality.is_source_backed() {
            Some(format!("source_anchor:{node_id}"))
        } else {
            None
        },
        downgrade_trigger: None,
        degraded_label: None,
        evidence_refs: ev(node_id),
    }
}

fn nodes() -> Vec<InspectNode> {
    vec![
        {
            let mut n = base(
                "inspect-node:component:0001",
                InspectTreeKind::Component,
                "Component node mapped exactly to its canonical-source span with a write-back round-trip",
                NodeMappingQualityClass::Exact,
                true,
                true,
            );
            n.mutation_offered = true;
            n.previews_source_diff_before_commit = true;
            n
        },
        base(
            "inspect-node:dom:0001",
            InspectTreeKind::DomElement,
            "DOM element mapped approximately to source; jump-to-source lands near the span with disclosure",
            NodeMappingQualityClass::Approximate,
            true,
            true,
        ),
        {
            let mut n = base(
                "inspect-node:widget:0001",
                InspectTreeKind::WidgetTreeNode,
                "Generated widget node with no hand-authored span; inspect-to-source falls back to the generator input",
                NodeMappingQualityClass::GeneratedOnly,
                false,
                true,
            );
            n.source_anchor_ref = Some("source_anchor:generator-input:widget:0001".to_owned());
            n
        },
        {
            let mut n = base(
                "inspect-node:dom:0002",
                InspectTreeKind::DomElement,
                "Runtime-only DOM node whose source map was lost on provider loss; explained as having no source to jump to",
                NodeMappingQualityClass::RuntimeOnly,
                true,
                false,
            );
            n.downgrade_trigger = Some(MappingDowngradeTrigger::ProviderLoss);
            n.degraded_label = Some(
                "Source map provider was lost on reconnect; this node is runtime-only and has no canonical source span to open"
                    .to_owned(),
            );
            n
        },
    ]
}

fn guardrails() -> TreeGuardrails {
    TreeGuardrails {
        source_canonical_no_second_writable_model: true,
        runtime_state_never_hides_source_mapping_uncertainty: true,
        inspect_only_never_auto_upgraded_to_write: true,
        embedded_boundaries_not_blurred_into_product: true,
        mapping_label_shown_before_navigation_or_mutation: true,
        continuity_preserved_without_silent_source_upgrade: true,
    }
}

fn consumer_projection() -> TreeConsumerProjection {
    TreeConsumerProjection {
        product_ingests_nodes: true,
        docs_help_ingests_nodes: true,
        diagnostics_ingests_nodes: true,
        support_export_ingests_nodes: true,
        release_control_ingests_nodes: true,
        support_export_reconstructs_mapping_quality: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        INSPECT_TO_SOURCE_TREE_SCHEMA_REF.to_owned(),
        INSPECT_TO_SOURCE_TREE_DOC_REF.to_owned(),
        INSPECT_TO_SOURCE_TREE_ARTIFACT_REF.to_owned(),
        "schemas/preview/preview_session_descriptor_set.schema.json".to_owned(),
        "schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json".to_owned(),
    ]
}

fn packet() -> InspectToSourceTreePacket {
    InspectToSourceTreePacket::new(InspectToSourceTreePacketInput {
        packet_id: "m5-inspect-to-source-tree:stable:0001".to_owned(),
        set_label: "M5 Inspect-to-Source Tree Mapping".to_owned(),
        nodes: nodes(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
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
