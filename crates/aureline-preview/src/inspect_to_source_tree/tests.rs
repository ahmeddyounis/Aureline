use super::*;

const PACKET_ID: &str = "m5-inspect-to-source-tree:stable:0001";

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:node:{id}")]
}

#[allow(clippy::too_many_arguments)]
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
        packet_id: PACKET_ID.to_owned(),
        set_label: "M5 Inspect-to-Source Tree Mapping".to_owned(),
        nodes: nodes(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn node_mut<'a>(packet: &'a mut InspectToSourceTreePacket, node_id: &str) -> &'a mut InspectNode {
    packet
        .nodes
        .iter_mut()
        .find(|n| n.node_id == node_id)
        .unwrap_or_else(|| panic!("node {node_id}"))
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_tree_kind_is_present() {
    let kinds = packet().represented_tree_kinds();
    for kind in InspectTreeKind::ALL {
        assert!(
            kinds.contains(&kind),
            "missing tree kind: {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_mapping_quality_is_present() {
    let qualities = packet().represented_mapping_qualities();
    for quality in NodeMappingQualityClass::ALL {
        assert!(
            qualities.contains(&quality),
            "missing mapping quality: {}",
            quality.as_str()
        );
    }
}

#[test]
fn missing_tree_kind_fails() {
    let mut packet = packet();
    packet
        .nodes
        .retain(|n| n.tree_kind != InspectTreeKind::WidgetTreeNode);
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::RequiredTreeKindMissing));
}

#[test]
fn missing_mapping_quality_fails() {
    let mut packet = packet();
    packet
        .nodes
        .retain(|n| n.mapping_quality != NodeMappingQualityClass::RuntimeOnly);
    let violations = packet.validate();
    assert!(violations.contains(&InspectToSourceTreeViolation::RequiredMappingQualityMissing));
    assert!(violations.contains(&InspectToSourceTreeViolation::DowngradedNodeCaseMissing));
}

#[test]
fn downgraded_case_is_present() {
    assert_eq!(packet().downgraded_node_count(), 1);
}

#[test]
fn continuity_route_mismatch_fails() {
    let mut packet = packet();
    node_mut(&mut packet, "inspect-node:component:0001").continuity_route =
        ContinuityRoute::RuntimeOnlyExplanation;
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::ContinuityRouteMismatch));
}

#[test]
fn affordance_before_label_fails() {
    let mut packet = packet();
    let node = node_mut(&mut packet, "inspect-node:dom:0001");
    node.mapping_label_resolved = false;
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::AffordanceBeforeLabel));
}

#[test]
fn source_backed_node_without_anchor_fails() {
    let mut packet = packet();
    node_mut(&mut packet, "inspect-node:component:0001").source_anchor_ref = None;
    let violations = packet.validate();
    assert!(violations.contains(&InspectToSourceTreeViolation::SourceAnchorPresenceInconsistent));
}

#[test]
fn runtime_only_carrying_source_anchor_fails() {
    let mut packet = packet();
    node_mut(&mut packet, "inspect-node:dom:0002").source_anchor_ref =
        Some("source_anchor:leak".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&InspectToSourceTreeViolation::SourceAnchorPresenceInconsistent));
    assert!(violations.contains(&InspectToSourceTreeViolation::RuntimeOnlyMasqueradesAsSource));
}

#[test]
fn runtime_only_claiming_saved_source_fails() {
    let mut packet = packet();
    node_mut(&mut packet, "inspect-node:dom:0002").claims_saved_source = true;
    let violations = packet.validate();
    assert!(violations.contains(&InspectToSourceTreeViolation::RuntimeOnlyMasqueradesAsSource));
    assert!(violations.contains(&InspectToSourceTreeViolation::NonSourceBackedClaimsSavedSource));
}

#[test]
fn generated_only_claiming_saved_source_fails() {
    let mut packet = packet();
    node_mut(&mut packet, "inspect-node:widget:0001").claims_saved_source = true;
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::NonSourceBackedClaimsSavedSource));
}

#[test]
fn runtime_only_offering_mutation_fails() {
    let mut packet = packet();
    let node = node_mut(&mut packet, "inspect-node:dom:0002");
    node.mutation_offered = true;
    node.previews_source_diff_before_commit = true;
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::MutationAffordanceUnbacked));
}

#[test]
fn mutation_without_diff_preview_fails() {
    let mut packet = packet();
    node_mut(&mut packet, "inspect-node:component:0001").previews_source_diff_before_commit = false;
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::MutationAffordanceUnbacked));
}

#[test]
fn generated_only_offering_mutation_fails() {
    let mut packet = packet();
    let node = node_mut(&mut packet, "inspect-node:widget:0001");
    node.mutation_offered = true;
    node.previews_source_diff_before_commit = true;
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::MutationAffordanceUnbacked));
}

#[test]
fn downgrade_without_label_fails() {
    let mut packet = packet();
    node_mut(&mut packet, "inspect-node:dom:0002").degraded_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&InspectToSourceTreeViolation::DowngradeInconsistent));
    assert!(violations.contains(&InspectToSourceTreeViolation::DowngradedNodeCaseMissing));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    node_mut(&mut packet, "inspect-node:dom:0002").degraded_label = Some("runtime only".to_owned());
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::DowngradeInconsistent));
}

#[test]
fn degraded_label_without_trigger_fails() {
    let mut packet = packet();
    node_mut(&mut packet, "inspect-node:component:0001").degraded_label =
        Some("Some precise but unexpected label".to_owned());
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::DowngradeInconsistent));
}

#[test]
fn node_without_evidence_fails() {
    let mut packet = packet();
    packet.nodes[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::NodeEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != INSPECT_TO_SOURCE_TREE_DOC_REF);
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet
        .guardrails
        .continuity_preserved_without_silent_source_upgrade = false;
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .support_export_reconstructs_mapping_quality = false;
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&InspectToSourceTreeViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: InspectToSourceTreePacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn chip_tokens_name_governed_chips() {
    let node = &packet().nodes[0];
    let chips = node.chip_tokens();
    assert!(chips.contains("tree=component"));
    assert!(chips.contains("mapping=exact"));
    assert!(chips.contains("continuity=exact_jump"));
}

#[test]
fn markdown_summary_names_nodes() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Inspect-to-Source Tree Mapping"));
    assert!(summary.contains("inspect-node:component:0001"));
    assert!(summary.contains("Downgraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_inspect_to_source_tree_export()
        .expect("checked inspect-to-source tree export validates");
    assert_eq!(checked, packet());
}
