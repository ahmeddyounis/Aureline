use super::*;

use crate::diagnostics::{DiagnosticFreshnessClass, DiagnosticOriginClass, DiagnosticSourceKind};
use crate::m5_diagnostic_source_descriptors_and_collection_snapshots::DiagnosticCollectionScope;
use crate::quality::QualityTargetScopeClass;

const CAPTURED_AT: &str = "2026-06-19T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn scope(profile: &str) -> DiagnosticCollectionScope {
    DiagnosticCollectionScope {
        scope_class: QualityTargetScopeClass::Workspace,
        workspace_ref: "workspace:test".to_owned(),
        workset_ref: None,
        target_or_environment_ref: Some("target:test".to_owned()),
        active_profile_ref: Some(profile.to_owned()),
    }
}

fn tool_version(kind: DiagnosticSourceKind, family: &str) -> QualityToolVersionRow {
    QualityToolVersionRow {
        source_kind: kind,
        tool_ref: format!("tool:{family}"),
        tool_version: format!("{family}:1.0.0"),
        rule_pack_ref: format!("rule-pack:{family}"),
        rule_pack_version: format!("{family}-rules:1.0.0"),
        adapter_ref: None,
        summary: format!("{family} analyzer in force."),
    }
}

fn save_outcome(outcome: SaveParticipantOutcomeClass) -> SaveParticipantOutcomeRow {
    SaveParticipantOutcomeRow {
        participant_ref: "participant:format".to_owned(),
        proposal_ref: "proposal:format:0001".to_owned(),
        action_token: "format".to_owned(),
        outcome_class: outcome,
        preview_first_required: true,
        apply_blocked: false,
        observed_at: CAPTURED_AT.to_owned(),
        summary: "Last format save-participant outcome.".to_owned(),
    }
}

fn live_snapshot() -> DiagnosticQualitySnapshot {
    DiagnosticQualitySnapshot::new(DiagnosticQualitySnapshotInput {
        snapshot_id: "snapshot:live:0001".to_owned(),
        snapshot_label: "Live local quality snapshot".to_owned(),
        scope: scope("profile:default"),
        origin_class: DiagnosticOriginClass::LiveLocalSession,
        freshness_class: DiagnosticFreshnessClass::Current,
        captured_at: CAPTURED_AT.to_owned(),
        active_profile_ref: "profile:default".to_owned(),
        profile_fingerprint: "fingerprint:default".to_owned(),
        tool_versions: vec![tool_version(
            DiagnosticSourceKind::LanguageService,
            "language_service",
        )],
        recent_collection_refs: refs(&["collection:live:0001"]),
        suppression_refs: Vec::new(),
        baseline_refs: Vec::new(),
        release_visible_debt_count: 0,
        imported_scanner_session_refs: Vec::new(),
        save_participant_outcomes: vec![save_outcome(
            SaveParticipantOutcomeClass::PreviewedNotApplied,
        )],
        source_descriptor_refs: refs(&["source:language_service"]),
        imported_not_shown_as_live: true,
        export_safe_summary: "Live local quality state.".to_owned(),
    })
}

fn imported_snapshot() -> DiagnosticQualitySnapshot {
    DiagnosticQualitySnapshot::new(DiagnosticQualitySnapshotInput {
        snapshot_id: "snapshot:imported:0001".to_owned(),
        snapshot_label: "Imported scanner quality snapshot".to_owned(),
        scope: scope("profile:default"),
        origin_class: DiagnosticOriginClass::ImportedSnapshot,
        freshness_class: DiagnosticFreshnessClass::ImportedSnapshot,
        captured_at: CAPTURED_AT.to_owned(),
        active_profile_ref: "profile:default".to_owned(),
        profile_fingerprint: "fingerprint:default".to_owned(),
        tool_versions: vec![tool_version(
            DiagnosticSourceKind::ScannerImport,
            "scanner_import",
        )],
        recent_collection_refs: refs(&["collection:imported:0001"]),
        suppression_refs: refs(&["suppression:imported:0001"]),
        baseline_refs: refs(&["baseline:imported:0001"]),
        release_visible_debt_count: 2,
        imported_scanner_session_refs: refs(&["import-session:0001"]),
        save_participant_outcomes: vec![save_outcome(SaveParticipantOutcomeClass::Skipped)],
        source_descriptor_refs: refs(&["source:scanner_import"]),
        imported_not_shown_as_live: true,
        export_safe_summary: "Imported scanner quality state, read-only.".to_owned(),
    })
}

fn stale_imported_snapshot() -> DiagnosticQualitySnapshot {
    let mut snapshot = imported_snapshot();
    snapshot.snapshot_id = "snapshot:imported-stale:0007".to_owned();
    snapshot.freshness_class = DiagnosticFreshnessClass::Stale;
    snapshot.recent_collection_refs = refs(&["collection:imported-stale:0007"]);
    snapshot.imported_scanner_session_refs = refs(&["import-session:0007"]);
    snapshot.source_descriptor_refs = refs(&["source:scanner_import"]);
    snapshot
}

fn entry(
    id: &str,
    snapshot: DiagnosticQualitySnapshot,
    qualification: DiagnosticQualitySnapshotQualificationClass,
) -> DiagnosticQualitySnapshotEntry {
    DiagnosticQualitySnapshotEntry {
        entry_id: id.to_owned(),
        snapshot,
        claimed_qualification: qualification,
        effective_qualification: qualification,
        downgrade_trigger: None,
        degraded_label: None,
        evidence_refs: refs(&[&format!("evidence:{id}")]),
    }
}

fn downgraded_entry() -> DiagnosticQualitySnapshotEntry {
    DiagnosticQualitySnapshotEntry {
        entry_id: "entry:stale".to_owned(),
        snapshot: stale_imported_snapshot(),
        claimed_qualification: DiagnosticQualitySnapshotQualificationClass::Beta,
        effective_qualification: DiagnosticQualitySnapshotQualificationClass::Held,
        downgrade_trigger: Some(DiagnosticQualitySnapshotDowngradeTrigger::StaleGovernanceState),
        degraded_label: Some(
            "The imported scan is stale against the current rule-pack epoch and is held below preview until a fresh import"
                .to_owned(),
        ),
        evidence_refs: refs(&["evidence:stale"]),
    }
}

fn side(
    label: &str,
    origin: DiagnosticOriginClass,
    freshness: DiagnosticFreshnessClass,
    kind: DiagnosticSourceKind,
    snapshot_ref: &str,
) -> DiagnosticDeltaSide {
    DiagnosticDeltaSide {
        side_label: label.to_owned(),
        origin_class: origin,
        freshness_class: freshness,
        source_kind: kind,
        snapshot_ref: snapshot_ref.to_owned(),
        collection_ref: format!("collection:{snapshot_ref}"),
        active_profile_ref: "profile:default".to_owned(),
        tool_version_refs: refs(&["tool:any"]),
        summary: format!("{label}."),
    }
}

fn imported_vs_live_delta() -> DiagnosticDeltaPacket {
    DiagnosticDeltaPacket::new(DiagnosticDeltaPacketInput {
        delta_id: "delta:imported-vs-live:0001".to_owned(),
        delta_label: "Imported scan versus live rerun".to_owned(),
        comparison_basis_class: DiagnosticDeltaComparisonBasisClass::ImportedVsLiveRerun,
        base_side: side(
            "imported",
            DiagnosticOriginClass::ImportedSnapshot,
            DiagnosticFreshnessClass::ImportedSnapshot,
            DiagnosticSourceKind::ScannerImport,
            "snapshot:imported:0001",
        ),
        compare_side: side(
            "live",
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticFreshnessClass::Current,
            DiagnosticSourceKind::LanguageService,
            "snapshot:live:0001",
        ),
        compatibility_class: DiagnosticDeltaCompatibilityClass::CompatibleWithLocalConfirmation,
        compatibility_notes: vec![DiagnosticDeltaCompatibilityNote {
            note_class: DiagnosticDeltaCompatibilityNoteClass::FreshnessSkew,
            summary: "Imported side is a static snapshot.".to_owned(),
        }],
        delta_counts: DiagnosticDeltaCounts {
            added: 1,
            resolved: 0,
            persisting: 1,
            suppressed_or_waived: 0,
            unmapped: 0,
        },
        finding_deltas: vec![
            DiagnosticFindingDelta {
                finding_ref: "diagnostic:0001".to_owned(),
                delta_state: DiagnosticFindingDeltaState::Persisting,
                base_present: true,
                compare_present: true,
                comparable: true,
                summary: "Present on both.".to_owned(),
            },
            DiagnosticFindingDelta {
                finding_ref: "diagnostic:0002".to_owned(),
                delta_state: DiagnosticFindingDeltaState::Added,
                base_present: false,
                compare_present: true,
                comparable: true,
                summary: "Only on the live side.".to_owned(),
            },
        ],
        impersonation_guarded: true,
        export_safe_summary: "Comparable once confirmed.".to_owned(),
    })
}

fn blocked_delta() -> DiagnosticDeltaPacket {
    DiagnosticDeltaPacket::new(DiagnosticDeltaPacketInput {
        delta_id: "delta:ci-vs-local:0007".to_owned(),
        delta_label: "Stale CI scan versus local rerun".to_owned(),
        comparison_basis_class: DiagnosticDeltaComparisonBasisClass::CiVsLocalRerun,
        base_side: side(
            "ci",
            DiagnosticOriginClass::ImportedSnapshot,
            DiagnosticFreshnessClass::Stale,
            DiagnosticSourceKind::ScannerImport,
            "snapshot:imported-stale:0007",
        ),
        compare_side: side(
            "local",
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticFreshnessClass::Current,
            DiagnosticSourceKind::LanguageService,
            "snapshot:live:0001",
        ),
        compatibility_class: DiagnosticDeltaCompatibilityClass::BlockedRulePackMismatch,
        compatibility_notes: vec![DiagnosticDeltaCompatibilityNote {
            note_class: DiagnosticDeltaCompatibilityNoteClass::RulePackVersionSkew,
            summary: "Older rule pack on the CI side.".to_owned(),
        }],
        delta_counts: DiagnosticDeltaCounts {
            added: 0,
            resolved: 0,
            persisting: 0,
            suppressed_or_waived: 0,
            unmapped: 1,
        },
        finding_deltas: vec![DiagnosticFindingDelta {
            finding_ref: "diagnostic:ci:0001".to_owned(),
            delta_state: DiagnosticFindingDeltaState::Unmapped,
            base_present: true,
            compare_present: false,
            comparable: false,
            summary: "Cannot map across the mismatch.".to_owned(),
        }],
        impersonation_guarded: true,
        export_safe_summary: "Blocked by rule-pack mismatch.".to_owned(),
    })
}

fn release_debt() -> DiagnosticQualityReleaseDebtProjection {
    DiagnosticQualityReleaseDebtProjection {
        assembled_from_snapshots: true,
        owner_truth_preserved: true,
        expiry_truth_preserved: true,
        baseline_join_preserved: true,
        suppression_join_preserved: true,
        release_visible_debt_count: 4,
        debt_source_refs: refs(&["suppression:imported:0001", "baseline:imported:0001"]),
        summary: "Debt assembled from snapshots.".to_owned(),
    }
}

fn guardrails() -> DiagnosticQualityParityGuardrails {
    DiagnosticQualityParityGuardrails {
        unlike_sources_never_flattened: true,
        anchors_never_silently_repaired: true,
        imported_live_class_explicit: true,
        freshness_and_remap_states_explicit: true,
        policy_state_preserved: true,
        every_fix_route_is_typed_proposal: true,
        ids_and_completeness_exportable: true,
    }
}

fn consumer_projection() -> DiagnosticQualityParityConsumerProjection {
    DiagnosticQualityParityConsumerProjection {
        problems_references_shared_model: true,
        review_references_shared_model: true,
        cli_headless_references_shared_model: true,
        support_export_references_shared_model: true,
        ai_evidence_references_shared_model: true,
        release_debt_references_shared_model: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_DIAGNOSTIC_QUALITY_PARITY_SCHEMA_REF,
        DIAGNOSTIC_QUALITY_SNAPSHOT_SCHEMA_REF,
        DIAGNOSTIC_DELTA_PACKET_SCHEMA_REF,
        M5_DIAGNOSTIC_QUALITY_PARITY_DOC_REF,
        M5_DIAGNOSTIC_QUALITY_PARITY_ARTIFACT_REF,
    ])
}

fn valid_packet() -> DiagnosticQualityParityPacket {
    DiagnosticQualityParityPacket::new(DiagnosticQualityParityPacketInput {
        packet_id: "packet:test:0001".to_owned(),
        packet_label: "Test parity packet".to_owned(),
        snapshot_entries: vec![
            entry(
                "entry:live",
                live_snapshot(),
                DiagnosticQualitySnapshotQualificationClass::Stable,
            ),
            entry(
                "entry:imported",
                imported_snapshot(),
                DiagnosticQualitySnapshotQualificationClass::Beta,
            ),
            downgraded_entry(),
        ],
        delta_packets: vec![imported_vs_live_delta(), blocked_delta()],
        release_debt_projection: release_debt(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: CAPTURED_AT.to_owned(),
    })
}

fn assert_has(packet: &DiagnosticQualityParityPacket, violation: DiagnosticQualityParityViolation) {
    let violations = packet.validate();
    assert!(
        violations.contains(&violation),
        "expected {violation:?}, got {violations:?}"
    );
}

#[test]
fn valid_packet_has_no_violations() {
    assert!(valid_packet().validate().is_empty());
}

#[test]
fn round_trips_through_json() {
    let packet = valid_packet();
    let json = packet.export_safe_json();
    let parsed: DiagnosticQualityParityPacket = serde_json::from_str(&json).unwrap();
    assert_eq!(packet, parsed);
}

#[test]
fn wrong_record_kind_is_caught() {
    let mut packet = valid_packet();
    packet.record_kind = "other".to_owned();
    assert_has(&packet, DiagnosticQualityParityViolation::WrongRecordKind);
}

#[test]
fn wrong_schema_version_is_caught() {
    let mut packet = valid_packet();
    packet.schema_version = 999;
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::WrongSchemaVersion,
    );
}

#[test]
fn missing_identity_is_caught() {
    let mut packet = valid_packet();
    packet.packet_id = "  ".to_owned();
    assert_has(&packet, DiagnosticQualityParityViolation::MissingIdentity);
}

#[test]
fn missing_source_contract_is_caught() {
    let mut packet = valid_packet();
    packet
        .source_contract_refs
        .retain(|r| r != M5_DIAGNOSTIC_QUALITY_PARITY_DOC_REF);
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::MissingSourceContracts,
    );
}

#[test]
fn missing_origin_coverage_is_caught() {
    let mut packet = valid_packet();
    // Make every snapshot a live origin: no imported side remains.
    for entry in &mut packet.snapshot_entries {
        entry.snapshot.origin_class = DiagnosticOriginClass::LiveLocalSession;
        entry.snapshot.freshness_class = DiagnosticFreshnessClass::Current;
        entry.snapshot.imported_scanner_session_refs = Vec::new();
    }
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::RequiredOriginCoverageMissing,
    );
}

#[test]
fn snapshot_missing_profile_binding_is_caught() {
    let mut packet = valid_packet();
    packet.snapshot_entries[0].snapshot.active_profile_ref = String::new();
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::SnapshotMissingProfileBinding,
    );
}

#[test]
fn snapshot_missing_tool_versions_is_caught() {
    let mut packet = valid_packet();
    packet.snapshot_entries[0].snapshot.tool_versions = Vec::new();
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::SnapshotMissingToolVersions,
    );
}

#[test]
fn snapshot_missing_recent_collection_is_caught() {
    let mut packet = valid_packet();
    packet.snapshot_entries[0].snapshot.recent_collection_refs = Vec::new();
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::SnapshotMissingRecentCollection,
    );
}

#[test]
fn imported_shown_as_live_is_caught() {
    let mut packet = valid_packet();
    packet.snapshot_entries[1]
        .snapshot
        .imported_not_shown_as_live = false;
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::SnapshotImportedShownAsLive,
    );
}

#[test]
fn imported_without_session_ref_is_caught() {
    let mut packet = valid_packet();
    packet.snapshot_entries[1]
        .snapshot
        .imported_scanner_session_refs = Vec::new();
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::SnapshotImportedShownAsLive,
    );
}

#[test]
fn suppression_baseline_truth_missing_is_caught() {
    let mut packet = valid_packet();
    let snapshot = &mut packet.snapshot_entries[1].snapshot;
    snapshot.release_visible_debt_count = 3;
    snapshot.suppression_refs = Vec::new();
    snapshot.baseline_refs = Vec::new();
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::SnapshotSuppressionBaselineTruthMissing,
    );
}

#[test]
fn missing_downgrade_case_is_caught() {
    let mut packet = valid_packet();
    // Drop the only downgraded entry.
    packet.snapshot_entries.truncate(2);
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::DowngradedSnapshotCaseMissing,
    );
}

#[test]
fn weak_truth_not_downgraded_is_caught() {
    let mut packet = valid_packet();
    // Claim beta on a stale snapshot but keep effective at beta.
    let downgraded = packet.snapshot_entries.last_mut().unwrap();
    downgraded.effective_qualification = DiagnosticQualitySnapshotQualificationClass::Beta;
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::SnapshotNotDowngradedOnWeakTruth,
    );
}

#[test]
fn downgrade_without_trigger_is_caught() {
    let mut packet = valid_packet();
    let downgraded = packet.snapshot_entries.last_mut().unwrap();
    downgraded.downgrade_trigger = None;
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::DowngradedSnapshotMissingLabelOrTrigger,
    );
}

#[test]
fn downgrade_with_generic_label_is_caught() {
    let mut packet = valid_packet();
    let downgraded = packet.snapshot_entries.last_mut().unwrap();
    downgraded.degraded_label = Some("downgraded".to_owned());
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::DowngradedSnapshotMissingLabelOrTrigger,
    );
}

#[test]
fn missing_delta_packets_is_caught() {
    let mut packet = valid_packet();
    packet.delta_packets = Vec::new();
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::DeltaPacketMissing,
    );
}

#[test]
fn missing_imported_vs_live_delta_is_caught() {
    let mut packet = valid_packet();
    // Keep a delta packet but none that crosses the imported/live boundary.
    let mut runtime_vs_static = imported_vs_live_delta();
    runtime_vs_static.comparison_basis_class =
        DiagnosticDeltaComparisonBasisClass::LiveSnapshotVsLiveSnapshot;
    runtime_vs_static.base_side.origin_class = DiagnosticOriginClass::LiveLocalSession;
    runtime_vs_static.base_side.freshness_class = DiagnosticFreshnessClass::Recent;
    packet.delta_packets = vec![runtime_vs_static];
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::ImportedVsLiveDeltaCaseMissing,
    );
}

#[test]
fn missing_blocked_delta_is_caught() {
    let mut packet = valid_packet();
    // Only the compatible imported-vs-live delta remains.
    packet.delta_packets = vec![imported_vs_live_delta()];
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::BlockedDeltaCaseMissing,
    );
}

#[test]
fn non_distinct_delta_sides_are_caught() {
    let mut packet = valid_packet();
    let delta = &mut packet.delta_packets[0];
    delta.compare_side = delta.base_side.clone();
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::DeltaSidesNotDistinct,
    );
}

#[test]
fn blocked_delta_without_note_is_caught() {
    let mut packet = valid_packet();
    packet.delta_packets[1].compatibility_notes = Vec::new();
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::DeltaCompatibilityNoteMissing,
    );
}

#[test]
fn inconsistent_delta_counts_are_caught() {
    let mut packet = valid_packet();
    packet.delta_packets[0].delta_counts.added = 99;
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::DeltaCountsInconsistent,
    );
}

#[test]
fn unguarded_delta_is_caught() {
    let mut packet = valid_packet();
    packet.delta_packets[0].impersonation_guarded = false;
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::DeltaImpersonationRisk,
    );
}

#[test]
fn same_origin_crossing_delta_is_caught() {
    let mut packet = valid_packet();
    // An imported-vs-live basis whose two sides share an origin impersonates.
    packet.delta_packets[0].base_side.origin_class = DiagnosticOriginClass::LiveLocalSession;
    packet.delta_packets[0].base_side.freshness_class = DiagnosticFreshnessClass::Recent;
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::DeltaImpersonationRisk,
    );
}

#[test]
fn release_debt_truth_dropped_is_caught() {
    let mut packet = valid_packet();
    packet.release_debt_projection.owner_truth_preserved = false;
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::ReleaseDebtTruthDropped,
    );
}

#[test]
fn incomplete_guardrails_are_caught() {
    let mut packet = valid_packet();
    packet.guardrails.anchors_never_silently_repaired = false;
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::GuardrailsIncomplete,
    );
}

#[test]
fn incomplete_consumer_projection_is_caught() {
    let mut packet = valid_packet();
    packet
        .consumer_projection
        .release_debt_references_shared_model = false;
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::ConsumerProjectionIncomplete,
    );
}

#[test]
fn raw_boundary_material_is_caught() {
    let mut packet = valid_packet();
    packet.snapshot_entries[0].snapshot.export_safe_summary = "embedded api_key=abc123".to_owned();
    assert_has(
        &packet,
        DiagnosticQualityParityViolation::RawBoundaryMaterialInExport,
    );
}

#[test]
fn downgraded_snapshot_reports_expected_trigger() {
    let entry = downgraded_entry();
    assert!(entry.needs_downgrade());
    assert!(entry.downgrade_consistent());
    assert_eq!(
        entry.expected_downgrade_trigger(),
        Some(DiagnosticQualitySnapshotDowngradeTrigger::StaleGovernanceState)
    );
}

#[test]
fn imported_snapshot_is_disclosed() {
    assert!(imported_snapshot().imported_disclosed());
    assert!(live_snapshot().imported_disclosed());
}

#[test]
fn origin_coverage_spans_imported_and_live() {
    let packet = valid_packet();
    let origins = packet.represented_origin_classes();
    assert!(origins.contains(&DiagnosticOriginClass::LiveLocalSession));
    assert!(origins.contains(&DiagnosticOriginClass::ImportedSnapshot));
}

#[test]
fn blocked_delta_count_is_one() {
    assert_eq!(valid_packet().blocked_delta_count(), 1);
}

#[test]
fn markdown_summary_names_records() {
    let summary = valid_packet().render_markdown_summary();
    assert!(summary.contains("snapshot:imported-stale:0007"));
    assert!(summary.contains("delta:ci-vs-local:0007"));
    assert!(summary.contains("Release-visible debt"));
}

#[test]
fn current_export_loads_and_validates() {
    let packet = current_m5_diagnostic_quality_parity_export()
        .expect("checked artifact loads and validates");
    assert!(packet.validate().is_empty());
    assert_eq!(packet.record_kind, M5_DIAGNOSTIC_QUALITY_PARITY_RECORD_KIND);
    // The checked artifact must demonstrate the auto-downgrade and blocked-delta
    // cases the lane depends on.
    assert!(packet.downgraded_snapshot_count() >= 1);
    assert!(packet.blocked_delta_count() >= 1);
}
