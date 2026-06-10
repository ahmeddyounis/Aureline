use super::*;

fn packet() -> ScopedBrowserSurfacesPacket {
    ScopedBrowserSurfacesPacket::materialize(seeded_stable_scoped_browser_input())
}

#[test]
fn seeded_set_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, SCOPED_BROWSER_RECORD_KIND);
    assert_eq!(packet.schema_version, SCOPED_BROWSER_SCHEMA_VERSION);
}

#[test]
fn packet_covers_docs_and_review_scopes() {
    let scopes: BTreeSet<ScopedBrowserScope> = packet().surfaces.iter().map(|s| s.scope).collect();
    for required in ScopedBrowserScope::REQUIRED {
        assert!(scopes.contains(&required), "missing scope {required:?}");
    }
}

#[test]
fn every_surface_carries_handoff_reason_return_path_and_trust_disclosure() {
    for surface in packet().surfaces {
        assert!(!surface.title.trim().is_empty());
        assert!(!surface.headline.trim().is_empty());
        assert!(!surface.handoff_reason.note.trim().is_empty());
        assert!(!surface.return_path.return_ref.trim().is_empty());
        assert!(!surface.return_path.label.trim().is_empty());
        assert!(!surface.trust_disclosure_note.trim().is_empty());
        assert!(!surface.open_raw_escape_ref.trim().is_empty());
        assert!(!surface.open_source_escape_ref.trim().is_empty());
        assert!(surface.scope.is_within_qualified_scope());
        // Touch each token so it stays stable across refactors.
        let _ = (
            surface.scope.as_str(),
            surface.trust_class.as_str(),
            surface.handoff_reason.reason_kind.as_str(),
            surface.return_path.return_kind.as_str(),
            surface.handoff_capability.as_str(),
            surface.captured_vs_live.as_str(),
            surface.chips.source_class.as_str(),
            surface.chips.version_match.as_str(),
            surface.chips.freshness.as_str(),
            surface.chips.locality.as_str(),
            surface.chips.confidence.as_str(),
        );
    }
}

#[test]
fn missing_required_scope_blocks_stable() {
    let mut input = seeded_stable_scoped_browser_input();
    let dropped = input
        .surfaces
        .iter()
        .position(|s| s.scope == ScopedBrowserScope::Review)
        .expect("review surface present");
    let dropped_id = input.surfaces.remove(dropped).surface_id;
    input.export.rows.retain(|r| r.surface_id_ref != dropped_id);
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        ScopedBrowserPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::RequiredScopeMissing));
}

#[test]
fn out_of_bounds_scope_blocks_stable() {
    let mut input = seeded_stable_scoped_browser_input();
    input.surfaces[0].scope = ScopedBrowserScope::GeneralWeb;
    input.export.rows[0].scope = ScopedBrowserScope::GeneralWeb;
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::SurfaceScopeOutOfBounds));
}

#[test]
fn missing_handoff_reason_blocks_stable() {
    let mut input = seeded_stable_scoped_browser_input();
    input.surfaces[0].handoff_reason.note = "  ".to_owned();
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::HandoffReasonMissing));
}

#[test]
fn missing_return_path_blocks_stable() {
    let mut input = seeded_stable_scoped_browser_input();
    input.surfaces[0].return_path.return_ref = "  ".to_owned();
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::ReturnPathMissing));
}

#[test]
fn missing_trust_disclosure_blocks_stable() {
    let mut input = seeded_stable_scoped_browser_input();
    input.surfaces[0].trust_disclosure_note = "   ".to_owned();
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::TrustClassDisclosureMissing));
}

#[test]
fn untrusted_destination_at_high_confidence_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    let surface = input
        .surfaces
        .iter_mut()
        .find(|s| s.trust_class == ScopedBrowserTrustClass::LiveProviderHandoff)
        .expect("live provider surface present");
    surface.chips.confidence = ScopedBrowserConfidence::High;
    let id = surface.surface_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == id {
            row.confidence = ScopedBrowserConfidence::High;
        }
    }
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::TrustClassDisclosureCollapsed));
}

#[test]
fn uncited_untrusted_surface_blocks_stable() {
    let mut input = seeded_stable_scoped_browser_input();
    let surface = input
        .surfaces
        .iter_mut()
        .find(|s| s.trust_class.needs_citation())
        .expect("untrusted surface present");
    surface.cited = false;
    surface.citation_ref = None;
    let id = surface.surface_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == id {
            row.cited = false;
        }
    }
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::SurfaceNotCited));
}

#[test]
fn blocked_handoff_presented_live_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    let surface = input
        .surfaces
        .iter_mut()
        .find(|s| s.captured_vs_live == CapturedVsLive::Live)
        .expect("a live surface present");
    surface.handoff_capability = HandoffCapability::BlockedByPolicy;
    let id = surface.surface_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == id {
            row.handoff_capability = HandoffCapability::BlockedByPolicy;
        }
    }
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::BlockedHandoffPresentedAvailable));
}

#[test]
fn drifted_version_presented_as_confident_live_collapses_version_truth() {
    let mut input = seeded_stable_scoped_browser_input();
    // The light-edit surface is high-confidence + authoritative-live; drift its version.
    let surface = input
        .surfaces
        .iter_mut()
        .find(|s| s.chips.confidence == ScopedBrowserConfidence::High)
        .expect("high-confidence surface present");
    surface.chips.version_match = ScopedBrowserVersionMatch::IncompatibleDriftDetected;
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::VersionTruthCollapsed));
}

#[test]
fn missing_escapes_block_stable() {
    let mut input = seeded_stable_scoped_browser_input();
    input.surfaces[0].open_raw_escape_ref = "  ".to_owned();
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::OpenRawOpenSourceEscapeMissing));
}

#[test]
fn export_dropping_return_path_preservation_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    input.export.preserves_return_path = false;
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::ExportDropsPreservation));
}

#[test]
fn export_scope_mismatch_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    input.export.rows[0].scope = ScopedBrowserScope::Review;
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::ExportScopeMismatch));
}

#[test]
fn export_trust_class_mismatch_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    input.export.rows[0].trust_class = ScopedBrowserTrustClass::DerivedInferenceOnly;
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::ExportTrustClassMismatch));
}

#[test]
fn export_source_class_mismatch_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    input.export.rows[0].source_class = ScopedBrowserSourceClass::ReviewHost;
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::ExportSourceClassMismatch));
}

#[test]
fn export_dropping_return_path_flag_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    input.export.rows[0].has_return_path = false;
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::ExportReturnPathMismatch));
}

#[test]
fn export_missing_coverage_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    input.export.rows.pop();
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::ExportCoverageMissing));
}

#[test]
fn export_orphan_row_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    input.export.rows[0].surface_id_ref = "surface:does-not-exist".to_owned();
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::ExportRowOrphan));
}

#[test]
fn narrowing_degradation_narrows_below_stable() {
    let mut input = seeded_stable_scoped_browser_input();
    input.browser_degradations.push(ScopedBrowserDegradation {
        degradation_class: ScopedBrowserDegradationClass::ScopeNarrowedRerun,
        severity: ScopedBrowserFindingSeverity::Narrowing,
        summary: "the review surface was rerun at a narrowed scope after a policy change"
            .to_owned(),
        surface_id_ref: None,
        evidence_ref: None,
    });
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        ScopedBrowserPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn blocking_degradation_blocks_stable() {
    let mut input = seeded_stable_scoped_browser_input();
    input.browser_degradations.push(ScopedBrowserDegradation {
        degradation_class: ScopedBrowserDegradationClass::QuarantinedSource,
        severity: ScopedBrowserFindingSeverity::Blocking,
        summary: "a surfaced source is quarantined and must not be presented as publishable"
            .to_owned(),
        surface_id_ref: Some("surface:docs:tokio_retry_guide".to_owned()),
        evidence_ref: None,
    });
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        ScopedBrowserPromotionState::BlocksStable
    );
}

#[test]
fn degradation_referencing_unknown_surface_is_orphan() {
    let mut input = seeded_stable_scoped_browser_input();
    input.browser_degradations[0].surface_id_ref = Some("surface:does-not-exist".to_owned());
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::DegradationOrphan));
}

#[test]
fn projection_dropping_return_paths_drifts() {
    let mut input = seeded_stable_scoped_browser_input();
    input.consumer_projections[0].preserves_return_paths = false;
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_scoped_browser_input();
    input
        .consumer_projections
        .retain(|p| p.surface != ScopedBrowserConsumerSurface::ReviewSurface);
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn duplicate_surface_id_is_flagged() {
    let mut input = seeded_stable_scoped_browser_input();
    let clone = input.surfaces[0].clone();
    input.surfaces.push(clone);
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::DuplicateSurfaceId));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_scoped_browser_input();
    input.surfaces[0].headline = "matched on bearer abc123 token in the source".to_owned();
    let packet = ScopedBrowserSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ScopedBrowserFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_surfaces_and_degradations() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for surface in &packet.surfaces {
        assert!(summary.contains(&surface.surface_id));
    }
    assert!(summary.contains("Handoff reason"));
    assert!(summary.contains("Return path"));
    assert!(summary.contains("Degradations"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-09T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: ScopedBrowserSupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        SCOPED_BROWSER_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_scoped_browser_export()
        .expect("checked scoped-browser export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:scoped_browser:retry_backoff_handoffs"
    );
    assert_eq!(
        export.packet.promotion_state,
        ScopedBrowserPromotionState::Stable
    );
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/scope_narrowed_rerun_narrowed.json"
            )),
            ScopedBrowserPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/out_of_bounds_scope_blocks_stable.json"
            )),
            ScopedBrowserPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/missing_return_path_blocks_stable.json"
            )),
            ScopedBrowserPromotionState::BlocksStable,
        ),
    ] {
        let fixture: ScopedBrowserFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = ScopedBrowserSurfacesPacket::materialize(fixture.input);
        assert_eq!(
            packet.promotion_state, expected,
            "fixture `{}` expected {:?}, findings: {:?}",
            fixture.case_name, expected, packet.validation_findings
        );
        for expected_kind in fixture.expect.expected_finding_kinds {
            assert!(
                packet
                    .validation_findings
                    .iter()
                    .any(|f| f.finding_kind.as_str() == expected_kind),
                "fixture `{}` expected finding `{}`",
                fixture.case_name,
                expected_kind
            );
        }
    }
}

#[derive(Debug, Deserialize)]
struct ScopedBrowserFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: ScopedBrowserSurfacesPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
