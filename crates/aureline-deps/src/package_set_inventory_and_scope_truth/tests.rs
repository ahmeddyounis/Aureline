use super::*;

fn packet() -> PackageSetInventoryAndScopeTruth {
    current_package_set_inventory_and_scope_truth().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn the_three_scopes_stay_distinct() {
    let packet = packet();
    for kind in ScopeKind::ALL {
        assert!(
            packet.scope_views.iter().any(|row| row.scope_kind == kind),
            "missing scope kind {}",
            kind.as_str()
        );
    }
    // No scope claims hidden server-side widening.
    assert!(packet
        .scope_views
        .iter()
        .all(|row| !row.server_side_widening));
}

#[test]
fn scope_counts_are_honest_about_loaded_matching_total() {
    let packet = packet();
    for scope in &packet.scope_views {
        assert_eq!(scope.loaded_count, scope.member_package_ids.len());
        assert!(scope.loaded_count <= scope.matching_count);
        assert!(scope.matching_count <= scope.total_count);
        for member in &scope.member_package_ids {
            assert!(packet.package(member).is_some());
        }
    }
    // At least one scope is virtualized with loaded < total (large monorepo).
    assert!(packet
        .scope_views
        .iter()
        .any(|scope| scope.virtualized && scope.loaded_count < scope.total_count));
}

#[test]
fn every_package_has_owner_manifests_and_open_escapes() {
    let packet = packet();
    for row in &packet.packages {
        assert!(!row.owner_manifests.is_empty());
        assert!(!row.declared_versions.is_empty());
        let escape_kinds: std::collections::BTreeSet<_> =
            row.open_escapes.iter().map(|escape| escape.kind).collect();
        assert!(escape_kinds.contains(&OpenEscapeKind::OpenRaw));
        assert!(escape_kinds.contains(&OpenEscapeKind::OpenManifest));
    }
}

#[test]
fn convergence_states_match_resolved_versions() {
    let packet = packet();
    for state in ConvergenceState::ALL {
        assert!(
            packet
                .packages
                .iter()
                .any(|row| row.convergence_state == state),
            "missing convergence state {}",
            state.as_str()
        );
    }
    // A converged package agrees on one resolved version; a diverged one does not.
    let converged = packet
        .packages
        .iter()
        .find(|row| row.convergence_state == ConvergenceState::Converged)
        .expect("a converged package exists");
    let resolved: std::collections::BTreeSet<_> = converged
        .declared_versions
        .iter()
        .map(|claim| claim.resolved_exact_version_or_source.as_str())
        .collect();
    assert_eq!(resolved.len(), 1);
}

#[test]
fn dependency_edges_preserve_ownership_and_disclose_conflicts() {
    let packet = packet();
    for state in DuplicateConflictClass::ALL {
        assert!(
            packet
                .dependency_edges
                .iter()
                .any(|row| row.duplicate_conflict_class == state),
            "missing duplicate/conflict class {}",
            state.as_str()
        );
    }
    for edge in &packet.dependency_edges {
        assert!(!edge.owner_manifest.is_empty());
        assert!(packet.package(&edge.to_package_id).is_some());
        if edge.duplicate_conflict_class.is_disclosed() {
            assert!(!edge.disclosure_note.is_empty());
        }
    }
}

#[test]
fn mirror_and_offline_freshness_is_visible() {
    let packet = packet();
    for state in FreshnessState::ALL {
        assert!(
            packet
                .packages
                .iter()
                .any(|row| row.freshness_state == state),
            "missing freshness state {}",
            state.as_str()
        );
    }
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn export_projection_is_redaction_safe_and_complete() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(
        projection.rows.len(),
        packet.scope_views.len() + packet.packages.len()
    );
    assert!(!projection
        .rows
        .iter()
        .any(|row| row.summary.to_lowercase().contains("token:")));
    // The same scope and package vocabulary appears in the projection.
    assert!(projection
        .rows
        .iter()
        .any(|row| row.row_kind == "scope_view"));
    assert!(projection
        .rows
        .iter()
        .any(|row| row.row_kind == "package_inventory"));
}
