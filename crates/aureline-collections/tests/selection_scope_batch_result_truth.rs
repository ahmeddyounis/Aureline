use aureline_collections::{
    current_selection_scope_packet, SelectionScopeFindingKind, SelectionScopePacket,
};

fn fixture(name: &str) -> SelectionScopePacket {
    let path = format!(
        "{}/../../fixtures/collections/m4/stabilize-selection-scope-and-batch-result-truth/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let contents = std::fs::read_to_string(path).expect("fixture should be readable");
    serde_json::from_str(&contents).expect("fixture should parse")
}

#[test]
fn checked_in_artifact_preserves_selection_scope_truth() {
    let packet = current_selection_scope_packet().expect("artifact should validate");
    assert!(packet.validate().is_empty());
    assert!(packet
        .support_export("support-export:test", "2026-06-07T00:00:00Z")
        .is_export_safe());
}

#[test]
fn baseline_fixture_matches_stable_contract() {
    let packet = fixture("baseline_stable.json");
    assert!(packet.validate().is_empty());
}

#[test]
fn implicit_all_matching_expansion_fixture_blocks_stable() {
    let packet = fixture("all_matching_expansion_implicit_blocks_stable.json");
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| {
        finding.finding_kind == SelectionScopeFindingKind::AllMatchingExpansionImplicit
    }));
}

#[test]
fn collapsed_descendant_fixture_blocks_stable() {
    let packet = fixture("collapsed_descendants_blocks_stable.json");
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| {
        finding.finding_kind == SelectionScopeFindingKind::TreeRangeCanIncludeCollapsedDescendants
    }));
}

#[test]
fn mixed_outcome_collapse_fixture_blocks_stable() {
    let packet = fixture("mixed_outcome_collapsed_blocks_stable.json");
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| {
        finding.finding_kind == SelectionScopeFindingKind::MixedOutcomeTruthCollapsed
    }));
}

#[test]
fn missing_package_data_grid_fixture_blocks_stable() {
    let packet = fixture("missing_package_data_grid_blocks_stable.json");
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| {
        finding.finding_kind == SelectionScopeFindingKind::MissingRequiredSurface
    }));
}
