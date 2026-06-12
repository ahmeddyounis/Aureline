use super::*;

fn packet() -> M5TopologyIdentityPacket {
    current_m5_topology_identity_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_TOPOLOGY_IDENTITY_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_TOPOLOGY_IDENTITY_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_body() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_surface_has_exactly_one_binding() {
    let packet = packet();
    assert_eq!(
        packet.surface_bindings.len(),
        TopologySurface::ALL.len(),
        "one binding per surface"
    );
    for surface in TopologySurface::ALL {
        assert!(
            packet.surface_binding(surface).is_some(),
            "missing binding for surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_binding_is_stamped_with_the_active_snapshot() {
    let packet = packet();
    for binding in &packet.surface_bindings {
        assert_eq!(binding.snapshot_id, packet.active_scope.snapshot_id);
        assert_eq!(binding.scope_id, packet.active_scope.scope_id);
    }
}

#[test]
fn every_object_carries_an_export_safe_permalink() {
    let packet = packet();
    assert!(packet.all_objects_have_permalink());
    for node in &packet.nodes {
        assert!(
            node.permalink_is_export_safe(),
            "node {} has an unsafe permalink",
            node.node_id
        );
        assert_eq!(
            packet.permalink_for_node(&node.node_id),
            Some(node.export_permalink.as_str())
        );
    }
    for edge in &packet.edges {
        assert!(
            edge.permalink_is_export_safe(),
            "edge {} has an unsafe permalink",
            edge.edge_id
        );
        assert_eq!(
            packet.permalink_for_edge(&edge.edge_id),
            Some(edge.export_permalink.as_str())
        );
    }
}

#[test]
fn permalinks_are_unique_across_nodes_and_edges() {
    let packet = packet();
    let permalinks: BTreeSet<&str> = packet
        .nodes
        .iter()
        .map(|n| n.export_permalink.as_str())
        .chain(packet.edges.iter().map(|e| e.export_permalink.as_str()))
        .collect();
    assert_eq!(permalinks.len(), packet.nodes.len() + packet.edges.len());
}

#[test]
fn every_nonexact_edge_is_labeled() {
    // The honesty guardrail: approximate, imported, partial, stale, and blocked edges all carry
    // an explicit disclosure reason rather than being implied away by presentation.
    let packet = packet();
    assert!(packet.all_nonexact_edges_labeled());
    for edge in &packet.edges {
        if edge.relation_fidelity.requires_disclosure() {
            assert!(
                edge.fidelity_reason
                    .as_ref()
                    .is_some_and(|reason| !reason.trim().is_empty()),
                "edge {} ({}) carries no fidelity_reason",
                edge.edge_id,
                edge.relation_fidelity.as_str()
            );
        }
    }
}

#[test]
fn every_relation_fidelity_is_exercised() {
    let packet = packet();
    let fidelities: BTreeSet<RelationFidelity> =
        packet.edges.iter().map(|e| e.relation_fidelity).collect();
    for fidelity in RelationFidelity::ALL {
        assert!(
            fidelities.contains(&fidelity),
            "no edge exercises fidelity {}",
            fidelity.as_str()
        );
    }
}

#[test]
fn every_node_and_edge_kind_is_exercised() {
    let packet = packet();
    let node_kinds: BTreeSet<TopologyNodeKind> = packet.nodes.iter().map(|n| n.kind).collect();
    for kind in TopologyNodeKind::ALL {
        assert!(
            node_kinds.contains(&kind),
            "no node exercises kind {}",
            kind.as_str()
        );
    }
    let edge_kinds: BTreeSet<TopologyEdgeKind> = packet.edges.iter().map(|e| e.kind).collect();
    for kind in TopologyEdgeKind::ALL {
        assert!(
            edge_kinds.contains(&kind),
            "no edge exercises kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn canvas_is_not_the_source_of_truth() {
    // The guardrail: every identity the canvas resolves is also resolvable from a non-canvas
    // accessible view.
    let packet = packet();
    assert!(packet.canvas_is_not_source_of_truth());
    assert!(packet.all_surfaces_resolve_shared_identity());
}

#[test]
fn every_declared_object_resolves_from_a_non_canvas_surface() {
    let packet = packet();
    let non_canvas_nodes: BTreeSet<&str> = packet
        .surface_bindings
        .iter()
        .filter(|b| !b.surface.is_canvas())
        .flat_map(|b| b.resolves_node_ids.iter())
        .map(String::as_str)
        .collect();
    for node in &packet.nodes {
        assert!(
            non_canvas_nodes.contains(node.node_id.as_str()),
            "node {} is only resolvable from the canvas",
            node.node_id
        );
    }
}

#[test]
fn packet_binds_to_canonical_upstream_packets() {
    let packet = packet();
    assert_eq!(
        packet.governance_matrix_ref,
        M5_TOPOLOGY_IDENTITY_GOVERNANCE_MATRIX_REF
    );
    assert_eq!(
        packet.scope_packet_ref,
        M5_TOPOLOGY_IDENTITY_SCOPE_PACKET_REF
    );
}

#[test]
fn export_projection_reflects_body_and_guardrails() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.snapshot_id, packet.active_scope.snapshot_id);
    assert_eq!(projection.scope_id, packet.active_scope.scope_id);
    assert_eq!(
        projection.objects.len(),
        packet.nodes.len() + packet.edges.len()
    );
    assert!(projection.all_objects_have_permalink);
    assert!(projection.all_nonexact_edges_labeled);
    assert!(projection.all_surfaces_resolve_shared_identity);
    assert!(projection.canvas_is_not_source_of_truth);
    let edge_rows = projection
        .objects
        .iter()
        .filter(|row| row.object_class == "edge")
        .count();
    assert_eq!(edge_rows, packet.edges.len());
    for row in &projection.objects {
        assert!(!row.permalink.trim().is_empty());
        assert!(row.permalink.contains(&row.object_id));
    }
}

#[test]
fn validate_flags_unlabeled_nonexact_relation() {
    let mut packet = packet();
    if let Some(edge) = packet
        .edges
        .iter_mut()
        .find(|e| e.relation_fidelity.requires_disclosure())
    {
        edge.fidelity_reason = None;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5TopologyIdentityViolation::UnlabeledNonExactRelation { .. }
        )));
    }
}

#[test]
fn validate_flags_unsafe_node_permalink() {
    let mut packet = packet();
    if let Some(node) = packet.nodes.first_mut() {
        node.export_permalink = "aureline://workspace:aureline/topology/node/mismatch".to_owned();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5TopologyIdentityViolation::UnsafeNodePermalink { .. })));
    }
}

#[test]
fn validate_flags_duplicate_permalink() {
    let mut packet = packet();
    let shared = packet.nodes[0].export_permalink.clone();
    packet.nodes[1].export_permalink = shared;
    let violations = packet.validate();
    // The mismatched node id also trips the unsafe-permalink check; assert the duplicate is seen.
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5TopologyIdentityViolation::DuplicatePermalink { .. })));
}

#[test]
fn validate_flags_dangling_edge_endpoint() {
    let mut packet = packet();
    if let Some(edge) = packet.edges.first_mut() {
        edge.to_node_id = "node:does-not-exist".to_owned();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5TopologyIdentityViolation::DanglingEdgeEndpoint { .. })));
    }
}

#[test]
fn validate_flags_unresolved_node_ref() {
    let mut packet = packet();
    if let Some(binding) = packet.surface_bindings.iter_mut().find(|b| !b.is_canvas) {
        binding.resolves_node_ids.push("node:phantom".to_owned());
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5TopologyIdentityViolation::UnresolvedNodeRef { .. })));
    }
}

#[test]
fn validate_flags_canvas_only_identity() {
    // Add a node that only the canvas resolves; the guardrail must reject it.
    let mut packet = packet();
    let canvas_only = TopologyNodeIdentity {
        node_id: "node:canvas-only".to_owned(),
        kind: TopologyNodeKind::File,
        display_label: "canvas only".to_owned(),
        namespace_ref: "repo:core".to_owned(),
        workspace_ref: "workspace:aureline".to_owned(),
        freshness: "authoritative".to_owned(),
        confidence: "high".to_owned(),
        source_class: TopologySourceClass::Indexed,
        contract_badges: ContractBadge::ALL.to_vec(),
        export_permalink: "aureline://workspace:aureline/topology/node/node:canvas-only".to_owned(),
    };
    packet.nodes.push(canvas_only);
    if let Some(binding) = packet.surface_bindings.iter_mut().find(|b| b.is_canvas) {
        binding
            .resolves_node_ids
            .push("node:canvas-only".to_owned());
    }
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5TopologyIdentityViolation::NodeMissingNonCanvasSurface { .. }
            | M5TopologyIdentityViolation::CanvasOnlyNodeIdentity { .. }
    )));
}

#[test]
fn validate_flags_missing_required_badges() {
    let mut packet = packet();
    if let Some(node) = packet.nodes.first_mut() {
        node.contract_badges
            .retain(|b| *b != ContractBadge::StableIdentity);
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5TopologyIdentityViolation::MissingRequiredBadges { .. })));
    }
}

#[test]
fn validate_flags_snapshot_binding_mismatch() {
    let mut packet = packet();
    if let Some(binding) = packet.surface_bindings.first_mut() {
        binding.snapshot_id = "workset-scope:snapshot:stale".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5TopologyIdentityViolation::SnapshotBindingMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_surface_canvas_flag_mismatch() {
    let mut packet = packet();
    if let Some(binding) = packet.surface_bindings.iter_mut().find(|b| !b.is_canvas) {
        binding.is_canvas = true;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5TopologyIdentityViolation::SurfaceCanvasFlagMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_surface_binding() {
    let mut packet = packet();
    packet
        .surface_bindings
        .retain(|b| b.surface != TopologySurface::Breadcrumb);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5TopologyIdentityViolation::MissingSurfaceBinding { .. })));
}

#[test]
fn validate_flags_governance_ref_mismatch() {
    let mut packet = packet();
    packet.governance_matrix_ref = "artifacts/graph/m5/not-the-matrix.json".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&M5TopologyIdentityViolation::GovernanceMatrixRefMismatch));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.node_count = packet.summary.node_count.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5TopologyIdentityViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        TopologyNodeKind::ProviderResource.as_str(),
        "provider_resource"
    );
    assert_eq!(TopologyNodeKind::WorksetScope.as_str(), "workset_scope");
    assert_eq!(TopologyEdgeKind::OwnedBy.as_str(), "owned_by");
    assert_eq!(TopologyEdgeKind::DependsOn.as_str(), "depends_on");
    assert_eq!(RelationFidelity::Approximate.as_str(), "approximate");
    assert_eq!(RelationFidelity::Blocked.as_str(), "blocked");
    assert_eq!(TopologySourceClass::Annotation.as_str(), "annotation");
    assert_eq!(TopologySurface::SupportExport.as_str(), "support_export");
    assert_eq!(
        ContractBadge::ExportSafePermalink.as_str(),
        "export_safe_permalink"
    );
    assert!(TopologySurface::MapCanvas.is_canvas());
    assert!(!TopologySurface::Table.is_canvas());
    assert!(RelationFidelity::Exact.is_exact());
    assert!(RelationFidelity::Stale.requires_disclosure());
}
