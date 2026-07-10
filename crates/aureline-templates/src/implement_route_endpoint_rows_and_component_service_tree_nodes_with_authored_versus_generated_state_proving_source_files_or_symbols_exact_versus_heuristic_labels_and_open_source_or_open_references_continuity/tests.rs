use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_route_tree_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, ROUTE_TREE_CONTROLS_PACKET_ID);
    assert_eq!(packet.record_kind, ROUTE_TREE_CONTROLS_RECORD_KIND);
}

#[test]
fn every_component_carries_its_frozen_family() {
    let packet = seeded_route_tree_controls();
    assert!(!packet.route_rows.is_empty());
    assert!(!packet.tree_nodes.is_empty());
    for row in &packet.route_rows {
        assert_eq!(row.component, M5FrameworkComponentFamily::RouteEndpointRow);
    }
    for node in &packet.tree_nodes {
        assert_eq!(
            node.component,
            M5FrameworkComponentFamily::ComponentServiceTreeNode
        );
    }
}

#[test]
fn ac_certainty_posture_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin the exact certainty labels: exact from source, runtime confirmed,
    // heuristic, or partial / unresolved. Assert the exact tokens.
    let tokens: Vec<&str> = EvidenceCertaintyPosture::ALL
        .iter()
        .map(|p| p.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "exact_from_source",
            "runtime_confirmed",
            "heuristic",
            "partial_or_unresolved"
        ]
    );
}

#[test]
fn posture_is_derived_never_asserted() {
    let packet = seeded_route_tree_controls();
    for row in &packet.route_rows {
        let disclosure = row.posture_disclosure();
        assert_eq!(row.derived_certainty_posture, disclosure.certainty_posture);
        assert_eq!(
            row.derived_authorship_posture,
            disclosure.authorship_posture
        );
        assert_eq!(
            row.claims_exact_from_source,
            disclosure.is_exact_from_source
        );
        assert_eq!(row.claims_generated, disclosure.is_generated);
        assert_eq!(row.has_proving_source_form, disclosure.has_source_form);
    }
    for node in &packet.tree_nodes {
        let disclosure = node.posture_disclosure();
        assert_eq!(node.derived_certainty_posture, disclosure.certainty_posture);
        assert_eq!(
            node.claims_exact_from_source,
            disclosure.is_exact_from_source
        );
        assert_eq!(node.has_proving_source_form, disclosure.has_source_form);
    }
}

#[test]
fn only_exact_from_source_reads_as_exact() {
    for evidence in [
        M5RouteEvidenceClass::HeuristicConvention,
        M5RouteEvidenceClass::DerivedByConvention,
        M5RouteEvidenceClass::PartialEvidence,
        M5RouteEvidenceClass::Unresolved,
    ] {
        let disclosure = resolve_route_evidence_posture(evidence, M5RouteAuthorship::Authored);
        assert!(!disclosure.is_exact_from_source, "{evidence:?}");
        assert!(disclosure.must_not_read_as_exact, "{evidence:?}");
    }
    let disclosure = resolve_route_evidence_posture(
        M5RouteEvidenceClass::ExactFromSource,
        M5RouteAuthorship::Authored,
    );
    assert!(disclosure.is_exact_from_source);
    // Runtime confirmed is a distinct strong state, not exact-from-source.
    let runtime = resolve_route_evidence_posture(
        M5RouteEvidenceClass::RuntimeConfirmed,
        M5RouteAuthorship::FrameworkProvided,
    );
    assert!(!runtime.is_exact_from_source);
    assert!(runtime.is_runtime_confirmed);
    assert!(!runtime.must_not_read_as_exact);
}

#[test]
fn generated_authorship_is_always_visible() {
    for authorship in [
        M5RouteAuthorship::Generated,
        M5RouteAuthorship::GeneratedThenEdited,
    ] {
        let disclosure =
            resolve_route_evidence_posture(M5RouteEvidenceClass::ExactFromSource, authorship);
        assert!(disclosure.is_generated, "{authorship:?}");
        assert!(disclosure.needs_generated_note, "{authorship:?}");
    }
    for authorship in [
        M5RouteAuthorship::Authored,
        M5RouteAuthorship::FrameworkProvided,
    ] {
        let disclosure =
            resolve_route_evidence_posture(M5RouteEvidenceClass::ExactFromSource, authorship);
        assert!(!disclosure.is_generated, "{authorship:?}");
    }
}

#[test]
fn runtime_only_and_unknown_origin_have_no_source_form() {
    for authorship in [
        M5RouteAuthorship::RuntimeOnly,
        M5RouteAuthorship::UnknownOrigin,
    ] {
        let disclosure =
            resolve_route_evidence_posture(M5RouteEvidenceClass::PartialEvidence, authorship);
        assert!(!disclosure.has_source_form, "{authorship:?}");
        assert!(disclosure.needs_no_source_form_note, "{authorship:?}");
    }
}

#[test]
fn unknown_or_unresolved_topology_has_no_source_form() {
    let unknown = resolve_topology_evidence_posture(
        M5TopologyNodeKind::UnknownNode,
        M5TopologyEvidenceClass::PartialEvidence,
    );
    assert!(!unknown.has_source_form);
    let unresolved = resolve_topology_evidence_posture(
        M5TopologyNodeKind::ComponentNode,
        M5TopologyEvidenceClass::Unresolved,
    );
    assert!(!unresolved.has_source_form);
    let exact = resolve_topology_evidence_posture(
        M5TopologyNodeKind::ComponentNode,
        M5TopologyEvidenceClass::ExactFromSource,
    );
    assert!(exact.has_source_form);
    assert!(exact.is_exact_from_source);
}

#[test]
fn components_cover_every_frozen_and_derived_vocabulary() {
    let packet = seeded_route_tree_controls();
    for evidence in M5RouteEvidenceClass::ALL {
        assert!(
            packet
                .route_rows
                .iter()
                .any(|r| r.route_evidence_class == evidence),
            "missing route evidence {}",
            evidence.as_str()
        );
    }
    for authorship in M5RouteAuthorship::ALL {
        assert!(
            packet
                .route_rows
                .iter()
                .any(|r| r.route_authorship == authorship),
            "missing authorship {}",
            authorship.as_str()
        );
    }
    for kind in RouteKind::ALL {
        assert!(
            packet.route_rows.iter().any(|r| r.route_kind == kind),
            "missing route kind {}",
            kind.as_str()
        );
    }
    for freshness in RowFreshnessState::ALL {
        assert!(
            packet
                .route_rows
                .iter()
                .any(|r| r.freshness_state == freshness),
            "missing freshness {}",
            freshness.as_str()
        );
    }
    for node_kind in M5TopologyNodeKind::ALL {
        assert!(
            packet
                .tree_nodes
                .iter()
                .any(|n| n.topology_node_kind == node_kind),
            "missing node kind {}",
            node_kind.as_str()
        );
    }
    for evidence in M5TopologyEvidenceClass::ALL {
        assert!(
            packet
                .tree_nodes
                .iter()
                .any(|n| n.topology_evidence_class == evidence),
            "missing topology evidence {}",
            evidence.as_str()
        );
    }
    for relation in NodeRelationKind::ALL {
        assert!(
            packet
                .tree_nodes
                .iter()
                .any(|n| n.relation_kind == relation),
            "missing relation {}",
            relation.as_str()
        );
    }
    for posture in EvidenceCertaintyPosture::ALL {
        assert!(
            packet
                .route_rows
                .iter()
                .any(|r| r.derived_certainty_posture == posture)
                || packet
                    .tree_nodes
                    .iter()
                    .any(|n| n.derived_certainty_posture == posture),
            "missing certainty posture {}",
            posture.as_str()
        );
    }
    for link in ProvingSourceLink::ALL {
        assert!(
            packet
                .route_rows
                .iter()
                .any(|r| r.proving_source_kind == link)
                || packet
                    .tree_nodes
                    .iter()
                    .any(|n| n.proving_source_kind == link),
            "missing proving source link {}",
            link.as_str()
        );
    }
}

#[test]
fn every_component_offers_mandatory_actions_labels_and_keyboard_route() {
    let packet = seeded_route_tree_controls();
    for row in &packet.route_rows {
        for action in RouteRowAction::MANDATORY {
            assert!(row.row_actions.contains(&action));
        }
        assert!(row.declares_mandatory_labels());
        assert!(row
            .accessibility_routes
            .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable));
    }
    for node in &packet.tree_nodes {
        for action in TreeNodeAction::MANDATORY {
            assert!(node.node_actions.contains(&action));
        }
        assert!(node.declares_mandatory_labels());
        assert!(node
            .accessibility_routes
            .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn misrepresented_route_posture_fails() {
    let mut packet = seeded_route_tree_controls();
    packet.route_rows[0].claims_generated = true;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::RoutePostureMisrepresented));
}

#[test]
fn heuristic_route_claiming_exact_fails() {
    let mut packet = seeded_route_tree_controls();
    let row = packet
        .route_rows
        .iter_mut()
        .find(|r| r.posture_disclosure().must_not_read_as_exact)
        .expect("a heuristic or partial route");
    row.claims_exact_from_source = true;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::HeuristicClaimsExact));
}

#[test]
fn runtime_only_route_claiming_a_proving_source_fails() {
    let mut packet = seeded_route_tree_controls();
    let row = packet
        .route_rows
        .iter_mut()
        .find(|r| !r.has_proving_source_form)
        .expect("a runtime-only route");
    row.proving_source_kind = ProvingSourceLink::SourceFile;
    row.proving_source_ref = "src:fake/path.rs".to_owned();
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::ProvingSourceClaimedWithoutForm));
}

#[test]
fn source_form_route_without_proving_source_fails() {
    let mut packet = seeded_route_tree_controls();
    let row = packet
        .route_rows
        .iter_mut()
        .find(|r| r.has_proving_source_form)
        .expect("a route with a source form");
    row.proving_source_kind = ProvingSourceLink::NoProvingSource;
    row.proving_source_ref = String::new();
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::ProvingSourceUnresolved));
}

#[test]
fn missing_generated_note_fails() {
    let mut packet = seeded_route_tree_controls();
    let row = packet
        .route_rows
        .iter_mut()
        .find(|r| r.posture_disclosure().is_generated)
        .expect("a generated route");
    row.generated_note = String::new();
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::GeneratedNoteMissing));
}

#[test]
fn missing_no_source_form_note_fails() {
    let mut packet = seeded_route_tree_controls();
    let row = packet
        .route_rows
        .iter_mut()
        .find(|r| !r.has_proving_source_form)
        .expect("a runtime-only route");
    row.no_source_form_note = String::new();
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::NoSourceFormNoteMissing));
}

#[test]
fn missing_params_or_guards_fails() {
    let mut packet = seeded_route_tree_controls();
    packet.route_rows[0].guards_notes = String::new();
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::ParamsOrGuardsMissing));
}

#[test]
fn missing_related_links_fails() {
    let mut packet = seeded_route_tree_controls();
    packet.tree_nodes[0].related_links_label = String::new();
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::RelatedLinksMissing));
}

#[test]
fn misrepresented_topology_posture_fails() {
    let mut packet = seeded_route_tree_controls();
    packet.tree_nodes[0].claims_exact_from_source = false;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::TopologyPostureMisrepresented));
}

#[test]
fn missing_mandatory_route_action_fails() {
    let mut packet = seeded_route_tree_controls();
    packet.route_rows[0]
        .row_actions
        .retain(|a| *a != RouteRowAction::OpenProvingSource);
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::RouteRowActionsIncomplete));
}

#[test]
fn missing_mandatory_node_action_fails() {
    let mut packet = seeded_route_tree_controls();
    packet.tree_nodes[0]
        .node_actions
        .retain(|a| *a != TreeNodeAction::OpenProvingSource);
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::TreeNodeActionsIncomplete));
}

#[test]
fn each_hard_invariant_fails_when_set() {
    let mut packet = seeded_route_tree_controls();
    packet.route_rows[0].lets_heuristic_masquerade_as_exact = true;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::HeuristicMasqueradesAsExact));

    let mut packet = seeded_route_tree_controls();
    packet.route_rows[0].hides_authored_versus_generated_state = true;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::AuthoredVersusGeneratedHidden));

    let mut packet = seeded_route_tree_controls();
    packet.route_rows[0].acts_as_hidden_parallel_model = true;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::HiddenParallelModel));

    let mut packet = seeded_route_tree_controls();
    packet.tree_nodes[0].hides_partial_or_derived_state = true;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::PartialOrDerivedHidden));

    let mut packet = seeded_route_tree_controls();
    packet.tree_nodes[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::AlternateStateLabelInvented));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = seeded_route_tree_controls();
    packet.tree_nodes[0].context_note = String::new();
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::ContextNoteMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_route_tree_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::MissingSourceContracts));
}

#[test]
fn route_topology_review_incomplete_fails() {
    let mut packet = seeded_route_tree_controls();
    packet.route_topology_review.heuristic_never_shown_as_exact = false;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::RouteTopologyReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_route_tree_controls();
    packet
        .consumer_projection
        .proving_source_reachable_before_trust = false;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_route_tree_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RouteTreeControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_component() {
    let packet = seeded_route_tree_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.route_rows {
        assert!(summary.contains(&row.route_or_matcher_label));
    }
    for node in &packet.tree_nodes {
        assert!(summary.contains(&node.entity_label));
    }
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let packet = seeded_route_tree_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + packet.route_rows.len() + packet.tree_nodes.len()
    );
    assert!(lines[0].starts_with("component,id,evidence_class,"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_route_tree_controls_export().expect("checked route tree controls export validates");
    assert_eq!(
        from_disk,
        seeded_route_tree_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn scenario_fixtures_validate_and_keep_full_coverage() {
    for packet in [
        seeded_route_tree_controls_heuristic_generated_route(),
        seeded_route_tree_controls_inferred_node(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn checked_scenario_fixtures_validate_and_match_seed_builders() {
    let route: RouteEndpointTreeNodeControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-route-endpoint-component-service-tree-controls/heuristic_generated_route.json"
    )))
    .expect("heuristic-generated-route fixture parses");
    assert!(route.validate().is_empty());
    assert_eq!(
        route,
        seeded_route_tree_controls_heuristic_generated_route()
    );

    let node: RouteEndpointTreeNodeControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-route-endpoint-component-service-tree-controls/inferred_node.json"
    )))
    .expect("inferred-node fixture parses");
    assert!(node.validate().is_empty());
    assert_eq!(node, seeded_route_tree_controls_inferred_node());
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_route_tree_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("secret"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
