use super::*;

fn packet() -> M5ExplainerArchitecturePacket {
    current_m5_explainer_and_architecture_maps_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_EXPLAINER_ARCHITECTURE_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_EXPLAINER_ARCHITECTURE_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_body() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_source_class_is_exercised() {
    // Generated-versus-curated stays visible: the corpus distinguishes curated, imported, and
    // generated explanations so generated prose never collapses into curated truth.
    let packet = packet();
    let classes: BTreeSet<ExplanationSourceClass> =
        packet.snapshots.iter().map(|s| s.source_class).collect();
    for class in ExplanationSourceClass::ALL {
        assert!(
            classes.contains(&class),
            "no snapshot exercises source class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_citation_kind_is_exercised() {
    // Files, symbols, docs packs, ADRs, curated notes, and graph objects are all citable.
    let packet = packet();
    let kinds: BTreeSet<CitationKind> = packet.citations.iter().map(|c| c.kind).collect();
    for kind in CitationKind::ALL {
        assert!(
            kinds.contains(&kind),
            "no citation exercises kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_visibility_is_exercised() {
    let packet = packet();
    let visibilities: BTreeSet<ExplainerVisibility> =
        packet.snapshots.iter().map(|s| s.visibility).collect();
    for visibility in ExplainerVisibility::ALL {
        assert!(
            visibilities.contains(&visibility),
            "no snapshot exercises visibility {}",
            visibility.as_str()
        );
    }
}

#[test]
fn every_snapshot_is_cited() {
    // Headline guardrail: a generated explanation never stands without a citation.
    let packet = packet();
    assert!(packet.every_snapshot_cited());
    for snapshot in &packet.snapshots {
        assert!(
            snapshot.is_cited(),
            "snapshot {} ships without a citation",
            snapshot.snapshot_id
        );
    }
}

#[test]
fn every_snapshot_carries_freshness_and_confidence() {
    let packet = packet();
    for snapshot in &packet.snapshots {
        assert!(
            snapshot.carries_freshness_and_confidence(),
            "snapshot {} drops a freshness or confidence cue",
            snapshot.snapshot_id
        );
    }
}

#[test]
fn every_snapshot_offers_non_canvas_navigation() {
    // Architecture maps are never canvas-only.
    let packet = packet();
    assert!(packet.every_snapshot_accessible());
    for snapshot in &packet.snapshots {
        assert!(
            snapshot.has_accessible_navigation(),
            "snapshot {} is canvas-only",
            snapshot.snapshot_id
        );
    }
}

#[test]
fn cited_and_follow_up_refs_resolve() {
    let packet = packet();
    for snapshot in &packet.snapshots {
        for citation_id in &snapshot.cited_ids {
            assert!(
                packet.citation(citation_id).is_some(),
                "snapshot {} cites undeclared citation {citation_id}",
                snapshot.snapshot_id
            );
        }
        for action_id in &snapshot.follow_up_ids {
            assert!(
                packet.follow_up(action_id).is_some(),
                "snapshot {} references undeclared follow-up {action_id}",
                snapshot.snapshot_id
            );
        }
    }
}

#[test]
fn every_surface_carries_one_binding_stamped_with_active_scope() {
    // Onboarding, review, docs, and AI all reference the same snapshot, stamped for replay.
    let packet = packet();
    for surface in ExplainerConsumerSurface::ALL {
        let binding = packet
            .consumer_binding(surface)
            .unwrap_or_else(|| panic!("no binding for surface {}", surface.as_str()));
        assert_eq!(binding.snapshot_id, packet.active_scope.snapshot_id);
        assert_eq!(binding.scope_id, packet.active_scope.scope_id);
        assert!(binding.preserves_source_labels);
    }
}

#[test]
fn support_export_carries_every_export_safe_snapshot_and_no_private() {
    let packet = packet();
    assert!(packet.every_export_safe_snapshot_in_support_export());
    assert!(packet.no_private_in_support_export());
}

#[test]
fn export_projection_redacts_private_snapshots() {
    let packet = packet();
    let projection = packet.export_projection();
    assert!(projection.every_snapshot_cited);
    assert!(projection.every_snapshot_accessible);
    assert!(projection.no_private_in_support_export);
    assert_eq!(
        projection.snapshots.len(),
        packet
            .snapshots
            .iter()
            .filter(|s| s.is_export_safe())
            .count()
    );
    assert!(projection.redacted_private_count >= 1);
    for row in &projection.snapshots {
        assert_ne!(row.visibility, ExplainerVisibility::Private.as_str());
    }
}

#[test]
fn validate_flags_snapshot_missing_citations() {
    let mut packet = packet();
    if let Some(snapshot) = packet.snapshots.first_mut() {
        snapshot.cited_ids.clear();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExplainerArchitectureViolation::SnapshotMissingCitations { .. }
        )));
    }
}

#[test]
fn validate_flags_canvas_only_navigation() {
    let mut packet = packet();
    if let Some(snapshot) = packet.snapshots.first_mut() {
        snapshot.navigation_affordances = vec![NavigationAffordance::Canvas];
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExplainerArchitectureViolation::CanvasOnlyNavigation { .. }
        )));
    }
}

#[test]
fn validate_flags_unresolved_citation_ref() {
    let mut packet = packet();
    if let Some(snapshot) = packet.snapshots.first_mut() {
        snapshot.cited_ids.push("cite:does-not-exist".to_owned());
        packet.summary = packet.computed_summary();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExplainerArchitectureViolation::UnresolvedCitationRef { .. }
        )));
    }
}

#[test]
fn validate_flags_private_snapshot_in_support_export() {
    let mut packet = packet();
    let private_id = packet
        .snapshots
        .iter()
        .find(|s| !s.is_export_safe())
        .map(|s| s.snapshot_id.clone())
        .expect("a private snapshot exists");
    if let Some(binding) = packet
        .consumer_bindings
        .iter_mut()
        .find(|b| b.surface == ExplainerConsumerSurface::SupportExport)
    {
        binding.carries_snapshot_ids.push(private_id);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExplainerArchitectureViolation::PrivateSnapshotInSupportExport { .. }
        )));
    }
}

#[test]
fn validate_flags_visibility_exceeding_binding() {
    let mut packet = packet();
    if let Some(binding) = packet
        .consumer_bindings
        .iter_mut()
        .find(|b| b.surface == ExplainerConsumerSurface::SupportExport)
    {
        binding.max_visibility = ExplainerVisibility::Public;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExplainerArchitectureViolation::VisibilityExceedsBinding { .. }
        )));
    }
}

#[test]
fn validate_flags_source_labels_not_preserved() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.preserves_source_labels = false;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExplainerArchitectureViolation::SourceLabelsNotPreserved { .. }
        )));
    }
}

#[test]
fn validate_flags_export_safe_snapshot_missing_from_support_export() {
    let mut packet = packet();
    if let Some(binding) = packet
        .consumer_bindings
        .iter_mut()
        .find(|b| b.surface == ExplainerConsumerSurface::SupportExport)
    {
        binding.carries_snapshot_ids.clear();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExplainerArchitectureViolation::ExportSafeSnapshotMissingFromSupportExport { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_surface_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.surface != ExplainerConsumerSurface::DocsBrowser);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ExplainerArchitectureViolation::MissingSurfaceBinding { .. }
    )));
}

#[test]
fn validate_flags_snapshot_binding_mismatch() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.snapshot_id = "workset-scope:snapshot:stale".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExplainerArchitectureViolation::SnapshotBindingMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_unsafe_snapshot_permalink() {
    let mut packet = packet();
    if let Some(snapshot) = packet.snapshots.first_mut() {
        snapshot.export_permalink =
            "aureline://workspace:aureline/explainer/snapshot/mismatch".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExplainerArchitectureViolation::UnsafeSnapshotPermalink { .. }
        )));
    }
}

#[test]
fn validate_flags_governance_ref_mismatch() {
    let mut packet = packet();
    packet.governance_matrix_ref = "artifacts/graph/m5/not-the-matrix.json".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&M5ExplainerArchitectureViolation::GovernanceMatrixRefMismatch));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.snapshot_count = packet.summary.snapshot_count.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5ExplainerArchitectureViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(ExplanationSourceClass::Curated.as_str(), "curated");
    assert_eq!(ExplanationSourceClass::Imported.as_str(), "imported");
    assert_eq!(ExplanationSourceClass::Generated.as_str(), "generated");
    assert!(ExplanationSourceClass::Curated.is_curated());
    assert!(ExplanationSourceClass::Generated.is_generated());
    assert_eq!(CitationKind::DocPack.as_str(), "doc_pack");
    assert_eq!(CitationKind::Adr.as_str(), "adr");
    assert_eq!(CitationKind::CuratedNote.as_str(), "curated_note");
    assert_eq!(CitationKind::GraphObject.as_str(), "graph_object");
    assert_eq!(NavigationAffordance::ListTable.as_str(), "list_table");
    assert_eq!(NavigationAffordance::ScreenReader.as_str(), "screen_reader");
    assert!(NavigationAffordance::Keyboard.is_non_canvas());
    assert!(!NavigationAffordance::Canvas.is_non_canvas());
    assert_eq!(
        FollowUpActionClass::RequestCuratedReview.as_str(),
        "request_curated_review"
    );
    assert_eq!(ExplainerVisibility::Private.as_str(), "private");
    assert!(ExplainerVisibility::Internal.is_export_safe());
    assert!(!ExplainerVisibility::Private.is_export_safe());
    assert!(ExplainerVisibility::Public.fits_within(ExplainerVisibility::Internal));
    assert!(!ExplainerVisibility::Private.fits_within(ExplainerVisibility::Internal));
    assert_eq!(
        ExplainerConsumerSurface::AiContextInspector.as_str(),
        "ai_context_inspector"
    );
    assert!(ExplainerConsumerSurface::SupportExport.is_support_export());
}
