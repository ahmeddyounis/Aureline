use super::*;

fn packet() -> LightRemoteEditSurfacesPacket {
    LightRemoteEditSurfacesPacket::materialize(seeded_stable_light_remote_edit_input())
}

#[test]
fn seeded_set_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, LIGHT_REMOTE_EDIT_RECORD_KIND);
    assert_eq!(packet.schema_version, LIGHT_REMOTE_EDIT_SCHEMA_VERSION);
}

#[test]
fn packet_covers_doc_comment_and_single_file_scopes() {
    let scopes: BTreeSet<LightRemoteEditScope> =
        packet().surfaces.iter().map(|s| s.scope).collect();
    for required in LightRemoteEditScope::REQUIRED {
        assert!(scopes.contains(&required), "missing scope {required:?}");
    }
}

#[test]
fn every_surface_carries_intent_return_path_authority_and_stale_state() {
    for surface in packet().surfaces {
        assert!(!surface.title.trim().is_empty());
        assert!(!surface.headline.trim().is_empty());
        assert!(!surface.edit_intent.note.trim().is_empty());
        assert!(!surface.return_path.return_ref.trim().is_empty());
        assert!(!surface.return_path.label.trim().is_empty());
        assert!(!surface.trust_disclosure_note.trim().is_empty());
        assert!(!surface.stale_state.note.trim().is_empty());
        assert!(!surface.open_raw_escape_ref.trim().is_empty());
        assert!(!surface.open_source_escape_ref.trim().is_empty());
        assert!(surface.scope.is_within_qualified_scope());
        // No hidden authority expansion in the seeded set.
        assert!(!surface.authority.is_expansion());
        assert!(surface.authority.effective <= surface.scope.max_authority());
        // Touch each token so it stays stable across refactors.
        let _ = (
            surface.scope.as_str(),
            surface.trust_class.as_str(),
            surface.edit_intent.intent_kind.as_str(),
            surface.return_path.return_kind.as_str(),
            surface.apply_posture.as_str(),
            surface.captured_vs_live.as_str(),
            surface.authority.granted.as_str(),
            surface.authority.effective.as_str(),
            surface.stale_state.base_state_kind.as_str(),
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
    let mut input = seeded_stable_light_remote_edit_input();
    let dropped = input
        .surfaces
        .iter()
        .position(|s| s.scope == LightRemoteEditScope::SingleFileTextEdit)
        .expect("single-file surface present");
    let dropped_id = input.surfaces.remove(dropped).surface_id;
    input.export.rows.retain(|r| r.surface_id_ref != dropped_id);
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        LightRemoteEditPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::RequiredScopeMissing));
}

#[test]
fn out_of_bounds_scope_blocks_stable() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.surfaces[0].scope = LightRemoteEditScope::MultiFileRefactor;
    input.export.rows[0].scope = LightRemoteEditScope::MultiFileRefactor;
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::SurfaceScopeOutOfBounds));
}

#[test]
fn missing_edit_intent_blocks_stable() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.surfaces[0].edit_intent.note = "  ".to_owned();
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::EditIntentMissing));
}

#[test]
fn missing_return_path_blocks_stable() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.surfaces[0].return_path.return_ref = "  ".to_owned();
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ReturnPathMissing));
}

#[test]
fn missing_trust_disclosure_blocks_stable() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.surfaces[0].trust_disclosure_note = "   ".to_owned();
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::TrustClassDisclosureMissing));
}

#[test]
fn untrusted_destination_at_high_confidence_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    let surface = input
        .surfaces
        .iter_mut()
        .find(|s| s.trust_class == EditTrustClass::LiveProviderEditSurface)
        .expect("live provider surface present");
    surface.chips.confidence = EditConfidence::High;
    let id = surface.surface_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == id {
            row.confidence = EditConfidence::High;
        }
    }
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::TrustClassDisclosureCollapsed));
}

#[test]
fn uncited_untrusted_surface_blocks_stable() {
    let mut input = seeded_stable_light_remote_edit_input();
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
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::SurfaceNotCited));
}

#[test]
fn blocked_apply_presented_live_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    let surface = input
        .surfaces
        .iter_mut()
        .find(|s| s.captured_vs_live == CapturedVsLive::Live)
        .expect("a live surface present");
    surface.apply_posture = ApplyPosture::ApplyBlockedByPolicy;
    let id = surface.surface_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == id {
            row.apply_posture = ApplyPosture::ApplyBlockedByPolicy;
        }
    }
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::BlockedApplyPresentedAvailable));
}

#[test]
fn drifted_version_presented_as_confident_live_collapses_version_truth() {
    let mut input = seeded_stable_light_remote_edit_input();
    // The doc-comment surface is high-confidence + authoritative-live; drift its version.
    let surface = input
        .surfaces
        .iter_mut()
        .find(|s| s.chips.confidence == EditConfidence::High)
        .expect("high-confidence surface present");
    surface.chips.version_match = EditVersionMatch::IncompatibleDriftDetected;
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::VersionTruthCollapsed));
}

#[test]
fn stale_base_undisclosed_blocks_stable() {
    let mut input = seeded_stable_light_remote_edit_input();
    let surface = input
        .surfaces
        .iter_mut()
        .find(|s| s.scope == LightRemoteEditScope::SingleFileTextEdit)
        .expect("single-file surface present");
    surface.stale_state.base_state_kind = BaseStateKind::StaleSnapshot;
    surface.stale_state.disclosed = false;
    let id = surface.surface_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == id {
            row.base_state_kind = BaseStateKind::StaleSnapshot;
            row.stale_disclosed = false;
        }
    }
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::StaleStateNotDisclosed));
}

#[test]
fn stale_base_presented_confident_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    let surface = &mut input.surfaces[0];
    surface.stale_state.base_state_kind = BaseStateKind::StaleSnapshot;
    surface.stale_state.disclosed = true;
    surface.stale_state.note = "served from a known-stale snapshot; disclosed".to_owned();
    // chip confidence on surface[0] is High in the seeded set.
    let id = surface.surface_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == id {
            row.base_state_kind = BaseStateKind::StaleSnapshot;
        }
    }
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::StaleStatePresentedConfident));
}

#[test]
fn authority_expansion_beyond_grant_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    // The single-file surface has a SingleFileWrite ceiling; lower its grant and
    // keep effective at SingleFileWrite so only the expansion fires.
    let surface = input
        .surfaces
        .iter_mut()
        .find(|s| s.scope == LightRemoteEditScope::SingleFileTextEdit)
        .expect("single-file surface present");
    surface.authority.granted = AuthorityScope::SingleFieldWrite;
    let id = surface.surface_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == id {
            row.granted_authority = AuthorityScope::SingleFieldWrite;
        }
    }
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::AuthorityExpansionDetected));
}

#[test]
fn authority_beyond_scope_ceiling_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    // The review-reply scope permits only SingleFieldWrite; grant and exercise
    // SingleFileWrite so the scope ceiling is exceeded without a bare expansion.
    let surface = input
        .surfaces
        .iter_mut()
        .find(|s| s.scope == LightRemoteEditScope::ReviewReply)
        .expect("review-reply surface present");
    surface.authority.granted = AuthorityScope::SingleFileWrite;
    surface.authority.effective = AuthorityScope::SingleFileWrite;
    let id = surface.surface_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.surface_id_ref == id {
            row.granted_authority = AuthorityScope::SingleFileWrite;
            row.effective_authority = AuthorityScope::SingleFileWrite;
        }
    }
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ScopeAuthorityMismatch));
}

#[test]
fn missing_escapes_block_stable() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.surfaces[0].open_raw_escape_ref = "  ".to_owned();
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::OpenRawOpenSourceEscapeMissing));
}

#[test]
fn export_dropping_authority_preservation_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.export.preserves_authority = false;
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ExportDropsPreservation));
}

#[test]
fn export_authority_mismatch_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.export.rows[0].effective_authority = AuthorityScope::MultiFileWrite;
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ExportAuthorityMismatch));
}

#[test]
fn export_stale_state_mismatch_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.export.rows[0].base_state_kind = BaseStateKind::StaleSnapshot;
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ExportStaleStateMismatch));
}

#[test]
fn export_scope_mismatch_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.export.rows[0].scope = LightRemoteEditScope::ReviewReply;
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ExportScopeMismatch));
}

#[test]
fn export_trust_class_mismatch_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.export.rows[0].trust_class = EditTrustClass::DerivedSuggestionOnly;
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ExportTrustClassMismatch));
}

#[test]
fn export_source_class_mismatch_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.export.rows[0].source_class = EditSourceClass::ReviewHost;
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ExportSourceClassMismatch));
}

#[test]
fn export_dropping_return_path_flag_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.export.rows[0].has_return_path = false;
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ExportReturnPathMismatch));
}

#[test]
fn export_missing_coverage_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.export.rows.pop();
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ExportCoverageMissing));
}

#[test]
fn export_orphan_row_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.export.rows[0].surface_id_ref = "surface:does-not-exist".to_owned();
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ExportRowOrphan));
}

#[test]
fn narrowing_degradation_narrows_below_stable() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.edit_degradations.push(LightRemoteEditDegradation {
        degradation_class: LightRemoteEditDegradationClass::AuthorityNarrowed,
        severity: LightRemoteEditFindingSeverity::Narrowing,
        summary: "the remote edit's authority was narrowed to read-only after a policy change"
            .to_owned(),
        surface_id_ref: None,
        evidence_ref: None,
    });
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        LightRemoteEditPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn blocking_degradation_blocks_stable() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.edit_degradations.push(LightRemoteEditDegradation {
        degradation_class: LightRemoteEditDegradationClass::QuarantinedSource,
        severity: LightRemoteEditFindingSeverity::Blocking,
        summary: "a surfaced source is quarantined and must not be presented as applicable"
            .to_owned(),
        surface_id_ref: Some("surface:doc_comment_edit:retry_doc_comment".to_owned()),
        evidence_ref: None,
    });
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        LightRemoteEditPromotionState::BlocksStable
    );
}

#[test]
fn degradation_referencing_unknown_surface_is_orphan() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.edit_degradations[0].surface_id_ref = Some("surface:does-not-exist".to_owned());
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::DegradationOrphan));
}

#[test]
fn projection_dropping_authority_drifts() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.consumer_projections[0].preserves_authority = false;
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_light_remote_edit_input();
    input
        .consumer_projections
        .retain(|p| p.surface != LightRemoteEditConsumerSurface::LightEditSurface);
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn duplicate_surface_id_is_flagged() {
    let mut input = seeded_stable_light_remote_edit_input();
    let clone = input.surfaces[0].clone();
    input.surfaces.push(clone);
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::DuplicateSurfaceId));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_light_remote_edit_input();
    input.surfaces[0].headline = "matched on bearer abc123 token in the source".to_owned();
    let packet = LightRemoteEditSurfacesPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == LightRemoteEditFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_surfaces_and_degradations() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for surface in &packet.surfaces {
        assert!(summary.contains(&surface.surface_id));
    }
    assert!(summary.contains("Edit intent"));
    assert!(summary.contains("Return path"));
    assert!(summary.contains("Authority"));
    assert!(summary.contains("Base state"));
    assert!(summary.contains("Degradations"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-10T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: LightRemoteEditSupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        LIGHT_REMOTE_EDIT_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_light_remote_edit_export()
        .expect("checked light-remote-edit export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:light_remote_edit:retry_backoff_edits"
    );
    assert_eq!(
        export.packet.promotion_state,
        LightRemoteEditPromotionState::Stable
    );
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/authority_narrowed_rerun_narrowed.json"
            )),
            LightRemoteEditPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/stale_base_undisclosed_blocks_stable.json"
            )),
            LightRemoteEditPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/authority_expansion_blocks_stable.json"
            )),
            LightRemoteEditPromotionState::BlocksStable,
        ),
    ] {
        let fixture: LightRemoteEditFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = LightRemoteEditSurfacesPacket::materialize(fixture.input);
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
struct LightRemoteEditFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: LightRemoteEditSurfacesPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
