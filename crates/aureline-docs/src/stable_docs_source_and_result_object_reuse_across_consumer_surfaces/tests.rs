use super::*;

fn packet() -> DocsObjectReusePacket {
    DocsObjectReusePacket::materialize(seeded_stable_docs_source_result_reuse_input())
}

#[test]
fn seeded_packet_is_stable() {
    let packet = packet();
    assert_eq!(packet.promotion_state, DocsObjectPromotionState::Stable);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_stable());
}

#[test]
fn seeded_packet_covers_every_consumer_surface() {
    let packet = packet();
    let present: BTreeSet<DocsObjectConsumerSurface> = packet
        .surface_projections
        .iter()
        .map(|projection| projection.consumer_surface)
        .collect();
    for surface in DocsObjectConsumerSurface::REQUIRED {
        assert!(
            present.contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn seeded_packet_keeps_source_classes_distinguishable() {
    let packet = packet();
    let classes: std::collections::HashSet<CitationSourceClass> = packet
        .sources
        .iter()
        .map(|source| source.source_class)
        .collect();
    for required in [
        CitationSourceClass::ProjectDocs,
        CitationSourceClass::MirroredOfficialDocs,
        CitationSourceClass::CuratedKnowledgePack,
        CitationSourceClass::VendorProviderDocs,
        CitationSourceClass::DerivedExplanation,
    ] {
        assert!(
            classes.contains(&required),
            "missing class {}",
            required.as_str()
        );
    }
}

#[test]
fn project_docs_relabeled_as_vendor_blocks_stable() {
    let mut packet = packet();
    packet.sources[0].trust_class = DocsObjectTrustClass::LiveProviderHandoff;
    let kinds: Vec<_> = packet
        .validate()
        .into_iter()
        .map(|finding| finding.finding_kind)
        .collect();
    assert!(kinds.contains(&DocsObjectFindingKind::SourceTrustClassMismatch));
}

#[test]
fn missing_distinguishable_class_blocks_stable() {
    let mut packet = packet();
    packet
        .sources
        .retain(|source| source.source_class != CitationSourceClass::DerivedExplanation);
    // Results/projections referencing the dropped source also go.
    packet
        .results
        .retain(|result| result.source_class != CitationSourceClass::DerivedExplanation);
    packet
        .surface_projections
        .retain(|projection| projection.consumer_surface != DocsObjectConsumerSurface::AiCitation);
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsObjectFindingKind::SourceClassDistinguishabilityMissing));
}

#[test]
fn derived_explanation_claiming_precedence_blocks_stable() {
    let mut packet = packet();
    let derived = packet
        .sources
        .iter_mut()
        .find(|source| source.source_class == CitationSourceClass::DerivedExplanation)
        .expect("derived source present");
    derived.precedence_class = SourcePrecedenceClass::ProjectOutranksVendorDefault;
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DocsObjectFindingKind::DerivedExplanationMasqueradesAsPrimary));
}

#[test]
fn live_external_without_handoff_blocks_stable() {
    let mut packet = packet();
    let vendor = packet
        .sources
        .iter_mut()
        .find(|source| source.source_class == CitationSourceClass::VendorProviderDocs)
        .expect("vendor source present");
    vendor.mirror_offline_posture = DocsMirrorOfflinePosture::CachedLocal;
    assert!(packet.validate().iter().any(
        |finding| finding.finding_kind == DocsObjectFindingKind::LiveExternalDocsHandoffMissing
    ));
}

#[test]
fn result_freshness_drift_blocks_stable() {
    let mut packet = packet();
    packet.results[0].freshness_state = DocsFreshnessClass::Unverified;
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind == DocsObjectFindingKind::SourceResultTruthMismatch));
}

#[test]
fn result_pointing_at_unknown_source_blocks_stable() {
    let mut packet = packet();
    packet.results[0].docs_source_ref = "docs-source:nonexistent".to_owned();
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind == DocsObjectFindingKind::ResultSourceRefUnresolved));
}

#[test]
fn snippet_forcing_full_content_blocks_stable() {
    let mut packet = packet();
    packet.results[0].snippet.full_content_excluded = false;
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind == DocsObjectFindingKind::SnippetForcesFullContent));
}

#[test]
fn missing_consumer_surface_blocks_stable() {
    let mut packet = packet();
    packet.surface_projections.retain(|projection| {
        projection.consumer_surface != DocsObjectConsumerSurface::SupportExport
    });
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind == DocsObjectFindingKind::MissingConsumerSurface));
}

#[test]
fn projection_dropping_truth_blocks_stable() {
    let mut packet = packet();
    packet.surface_projections[0].shows_source_class = false;
    assert!(packet.validate().iter().any(
        |finding| finding.finding_kind == DocsObjectFindingKind::ConsumerSurfaceProjectionDrift
    ));
}

#[test]
fn projection_using_local_badge_vocabulary_blocks_stable() {
    let mut packet = packet();
    packet.surface_projections[1].local_badge_vocabulary_used = true;
    assert!(packet.validate().iter().any(
        |finding| finding.finding_kind == DocsObjectFindingKind::ConsumerSurfaceProjectionDrift
    ));
}

#[test]
fn missing_source_contracts_blocks_stable() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind == DocsObjectFindingKind::MissingSourceContracts));
}

#[test]
fn promotion_state_mismatch_is_detected() {
    let mut packet = packet();
    packet.promotion_state = DocsObjectPromotionState::BlocksStable;
    assert!(packet
        .validate()
        .iter()
        .any(|finding| finding.finding_kind == DocsObjectFindingKind::PromotionStateMismatch));
}

#[test]
fn markdown_summary_lists_every_surface() {
    let summary = packet().render_markdown_summary();
    for surface in DocsObjectConsumerSurface::REQUIRED {
        assert!(
            summary.contains(surface.as_str()),
            "summary missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn support_export_round_trips_safely() {
    let packet = packet();
    let export = packet.support_export(
        "support-export:stable_docs_source_and_result_object_reuse:test",
        "2026-06-26T00:30:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.export_packet_id_ref, packet.packet_id);
    assert_eq!(export.export_packet, packet);
}

#[test]
fn checked_support_export_validates() {
    let export =
        current_stable_docs_source_result_reuse_export().expect("checked support export validates");
    assert!(export.is_export_safe());
    assert_eq!(
        export.export_packet.promotion_state,
        DocsObjectPromotionState::Stable
    );
}

#[test]
fn current_packet_helper_validates() {
    let packet = current_stable_docs_source_result_reuse_packet().expect("seeded packet validates");
    assert_eq!(packet.packet_id, packet.packet_id);
    assert!(packet.validate().is_empty());
}
