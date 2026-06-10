use super::*;

fn packet() -> SavedQueryPrivacyPacket {
    SavedQueryPrivacyPacket::materialize(seeded_stable_saved_query_privacy_input())
}

#[test]
fn seeded_set_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, SAVED_QUERY_PRIVACY_RECORD_KIND);
    assert_eq!(packet.schema_version, SAVED_QUERY_PRIVACY_SCHEMA_VERSION);
}

#[test]
fn packet_covers_private_local_and_shared_team_classes() {
    let classes: BTreeSet<QueryPrivacyClass> =
        packet().entries.iter().map(|e| e.privacy_class).collect();
    for required in QueryPrivacyClass::REQUIRED {
        assert!(classes.contains(&required), "missing class {required:?}");
    }
}

#[test]
fn every_entry_carries_privacy_retention_export_and_escapes() {
    for entry in packet().entries {
        assert!(!entry.title.trim().is_empty());
        assert!(!entry.query_label.trim().is_empty());
        assert!(!entry.trust_disclosure_note.trim().is_empty());
        assert!(!entry.retention.note.trim().is_empty());
        assert!(!entry.export_safety.note.trim().is_empty());
        assert!(!entry.open_raw_escape_ref.trim().is_empty());
        assert!(!entry.open_source_escape_ref.trim().is_empty());
        assert!(entry.privacy_class.is_within_qualified_scope());
        // No hidden visibility expansion in the seeded set.
        assert!(!entry.visibility.is_expansion());
        assert!(entry.visibility.effective <= entry.privacy_class.max_visibility());
        // The retention tier never exposes the entry beyond its privacy ceiling.
        assert!(entry.retention.posture.shared_boundary() <= entry.privacy_class.max_visibility());
        // Export-safe entries carry an actually-safe redaction class.
        if entry.export_safety.export_safe {
            assert!(entry.export_safety.redaction_class.is_export_safe());
        }
        // Touch each token so it stays stable across refactors.
        let _ = (
            entry.entry_kind.as_str(),
            entry.privacy_class.as_str(),
            entry.trust_class.as_str(),
            entry.retention.posture.as_str(),
            entry.export_safety.redaction_class.as_str(),
            entry.share_posture.as_str(),
            entry.captured_vs_live.as_str(),
            entry.visibility.granted.as_str(),
            entry.visibility.effective.as_str(),
            entry.chips.source_class.as_str(),
            entry.chips.version_match.as_str(),
            entry.chips.freshness.as_str(),
            entry.chips.locality.as_str(),
            entry.chips.confidence.as_str(),
        );
    }
}

#[test]
fn missing_required_class_blocks_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    let dropped = input
        .entries
        .iter()
        .position(|e| e.privacy_class == QueryPrivacyClass::SharedTeam)
        .expect("shared-team entry present");
    let dropped_id = input.entries.remove(dropped).entry_id;
    input.export.rows.retain(|r| r.entry_id_ref != dropped_id);
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        SavedQueryPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::RequiredPrivacyClassMissing));
}

#[test]
fn out_of_bounds_privacy_class_blocks_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.entries[0].privacy_class = QueryPrivacyClass::PublicListing;
    input.export.rows[0].privacy_class = QueryPrivacyClass::PublicListing;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::PrivacyClassOutOfBounds));
}

#[test]
fn missing_title_or_label_blocks_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.entries[0].query_label = "  ".to_owned();
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::EntryTitleOrLabelMissing));
}

#[test]
fn missing_trust_disclosure_blocks_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.entries[0].trust_disclosure_note = "   ".to_owned();
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::TrustClassDisclosureMissing));
}

#[test]
fn untrusted_origin_at_high_confidence_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.trust_class == QueryTrustClass::LiveSyncedSuggestion)
        .expect("live synced suggestion present");
    entry.chips.confidence = QueryConfidence::High;
    let id = entry.entry_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == id {
            row.confidence = QueryConfidence::High;
        }
    }
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::TrustClassDisclosureCollapsed));
}

#[test]
fn uncited_untrusted_entry_blocks_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.trust_class.needs_citation())
        .expect("untrusted entry present");
    entry.cited = false;
    entry.citation_ref = None;
    let id = entry.entry_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == id {
            row.cited = false;
        }
    }
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::EntryNotCited));
}

#[test]
fn blocked_share_presented_live_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.privacy_class == QueryPrivacyClass::SharedTeam)
        .expect("a shared-team entry present");
    entry.share_posture = SharePosture::ShareBlockedByPolicy;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::BlockedSharePresentedAvailable));
}

#[test]
fn drifted_version_presented_as_confident_live_collapses_version_truth() {
    let mut input = seeded_stable_saved_query_privacy_input();
    // The private-local entry is high-confidence + authoritative-live; drift its version.
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.chips.confidence == QueryConfidence::High)
        .expect("high-confidence entry present");
    entry.chips.version_match = QueryVersionMatch::IncompatibleDriftDetected;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::VersionTruthCollapsed));
}

#[test]
fn retention_leak_blocks_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.privacy_class == QueryPrivacyClass::PrivateLocal)
        .expect("private-local entry present");
    entry.retention.posture = RetentionPosture::SharedStore;
    let id = entry.entry_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == id {
            row.retention_posture = RetentionPosture::SharedStore;
        }
    }
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::RetentionPrivacyMismatch));
}

#[test]
fn undisclosed_shared_retention_blocks_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.retention.posture == RetentionPosture::SharedStore)
        .expect("shared-store entry present");
    entry.retention.disclosed = false;
    let id = entry.entry_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == id {
            row.retention_disclosed = false;
        }
    }
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::RetentionDisclosureMissing));
}

#[test]
fn visibility_expansion_beyond_grant_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    // The shared-team entry has a Team ceiling; lower its grant and keep
    // effective at Team so only the expansion fires.
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.privacy_class == QueryPrivacyClass::SharedTeam)
        .expect("shared-team entry present");
    entry.visibility.granted = Visibility::OwnerDevices;
    let id = entry.entry_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == id {
            row.granted_visibility = Visibility::OwnerDevices;
        }
    }
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::VisibilityExpansionDetected));
}

#[test]
fn visibility_beyond_privacy_ceiling_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    // The private-synced entry permits only OwnerDevices; grant and expose Team
    // so the privacy ceiling is exceeded without a bare expansion.
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.privacy_class == QueryPrivacyClass::PrivateSynced)
        .expect("private-synced entry present");
    entry.visibility.granted = Visibility::Team;
    entry.visibility.effective = Visibility::Team;
    let id = entry.entry_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == id {
            row.granted_visibility = Visibility::Team;
            row.effective_visibility = Visibility::Team;
        }
    }
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::PrivacyVisibilityMismatch));
}

#[test]
fn export_safe_with_unsafe_redaction_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    let entry = &mut input.entries[0];
    entry.export_safety.redaction_class = QueryRedactionClass::NeedsRedaction;
    entry.export_safety.export_safe = true;
    let id = entry.entry_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == id {
            row.redaction_class = QueryRedactionClass::NeedsRedaction;
        }
    }
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::SupportExportUnsafe));
}

#[test]
fn missing_escapes_block_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.entries[0].open_raw_escape_ref = "  ".to_owned();
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::OpenRawOpenSourceEscapeMissing));
}

#[test]
fn export_dropping_retention_preservation_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.export.preserves_retention = false;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ExportDropsPreservation));
}

#[test]
fn export_visibility_mismatch_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.export.rows[0].effective_visibility = Visibility::Everyone;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ExportVisibilityMismatch));
}

#[test]
fn export_retention_mismatch_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.export.rows[0].retention_posture = RetentionPosture::SharedStore;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ExportRetentionMismatch));
}

#[test]
fn export_privacy_class_mismatch_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.export.rows[0].privacy_class = QueryPrivacyClass::SharedOrg;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ExportPrivacyClassMismatch));
}

#[test]
fn export_redaction_mismatch_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.export.rows[0].redaction_class = QueryRedactionClass::DigestOnly;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ExportRedactionMismatch));
}

#[test]
fn export_export_safe_mismatch_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.export.rows[0].export_safe = false;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ExportExportSafeMismatch));
}

#[test]
fn export_trust_class_mismatch_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.export.rows[0].trust_class = QueryTrustClass::DerivedSuggestionOnly;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ExportTrustClassMismatch));
}

#[test]
fn export_source_class_mismatch_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.export.rows[0].source_class = QuerySourceClass::TeamSharedLibrary;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ExportSourceClassMismatch));
}

#[test]
fn export_missing_coverage_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.export.rows.pop();
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ExportCoverageMissing));
}

#[test]
fn export_orphan_row_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.export.rows[0].entry_id_ref = "entry:does-not-exist".to_owned();
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ExportRowOrphan));
}

#[test]
fn narrowing_degradation_narrows_below_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.query_degradations.push(SavedQueryDegradation {
        degradation_class: SavedQueryDegradationClass::PrivacyNarrowed,
        severity: SavedQueryFindingSeverity::Narrowing,
        summary: "the shared query was narrowed back to private after a policy change".to_owned(),
        entry_id_ref: None,
        evidence_ref: None,
    });
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        SavedQueryPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn blocking_degradation_blocks_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.query_degradations.push(SavedQueryDegradation {
        degradation_class: SavedQueryDegradationClass::QuarantinedSource,
        severity: SavedQueryFindingSeverity::Blocking,
        summary: "a source store is quarantined and must not be presented as available".to_owned(),
        entry_id_ref: Some("entry:saved_query:retry_backoff_symbol_search".to_owned()),
        evidence_ref: None,
    });
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        SavedQueryPromotionState::BlocksStable
    );
}

#[test]
fn degradation_referencing_unknown_entry_is_orphan() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.query_degradations[0].entry_id_ref = Some("entry:does-not-exist".to_owned());
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::DegradationOrphan));
}

#[test]
fn projection_dropping_retention_drifts() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.consumer_projections[0].preserves_retention = false;
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input
        .consumer_projections
        .retain(|p| p.surface != SavedQueryConsumerSurface::HistoryPanel);
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn duplicate_entry_id_is_flagged() {
    let mut input = seeded_stable_saved_query_privacy_input();
    let clone = input.entries[0].clone();
    input.entries.push(clone);
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::DuplicateEntryId));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_saved_query_privacy_input();
    input.entries[0].query_label = "matched on bearer abc123 token in the source".to_owned();
    let packet = SavedQueryPrivacyPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SavedQueryFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_entries_and_degradations() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for entry in &packet.entries {
        assert!(summary.contains(&entry.entry_id));
    }
    assert!(summary.contains("Visibility"));
    assert!(summary.contains("Retention"));
    assert!(summary.contains("Export safety"));
    assert!(summary.contains("Degradations"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-10T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: SavedQueryPrivacySupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        SAVED_QUERY_PRIVACY_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_saved_query_privacy_export()
        .expect("checked saved-query-privacy export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:saved_query_privacy:retry_backoff_searches"
    );
    assert_eq!(
        export.packet.promotion_state,
        SavedQueryPromotionState::Stable
    );
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/privacy_narrowed_rerun_narrowed.json"
            )),
            SavedQueryPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/retention_leak_blocks_stable.json"
            )),
            SavedQueryPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/ship_saved_query_privacy_controls_local_versus_shared_retention_and_support_export_safe_search_history/support_export_unsafe_blocks_stable.json"
            )),
            SavedQueryPromotionState::BlocksStable,
        ),
    ] {
        let fixture: SavedQueryPrivacyFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = SavedQueryPrivacyPacket::materialize(fixture.input);
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
struct SavedQueryPrivacyFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: SavedQueryPrivacyPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
