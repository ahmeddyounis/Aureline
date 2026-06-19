use super::*;

use crate::diagnostics::{
    DiagnosticEvidencePlaneClass, DiagnosticOriginClass, DiagnosticSource,
    DiagnosticSourceConfidenceClass, DiagnosticSourceKind, DiagnosticSupportClass,
};
use crate::quality::QualityTargetScopeClass;

const MINTED_AT: &str = "2026-06-19T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn source(source_id: &str, kind: DiagnosticSourceKind) -> DiagnosticSource {
    let mut built = DiagnosticSource::new(
        source_id.to_owned(),
        kind,
        DiagnosticEvidencePlaneClass::StaticAnalysis,
        DiagnosticOriginClass::LiveLocalSession,
        DiagnosticSourceConfidenceClass::Authoritative,
        DiagnosticSupportClass::Authoritative,
        format!("producer:{source_id}"),
        format!("tool:{source_id}"),
        Some(format!("tool-version:{source_id}")),
        "Source descriptor for a normalized finding.".to_owned(),
    );
    built.target_or_environment_ref = Some(format!("target:{source_id}"));
    built.originating_session_ref = Some(format!("session:{source_id}"));
    built
}

fn scope() -> DiagnosticCollectionScope {
    DiagnosticCollectionScope {
        scope_class: QualityTargetScopeClass::Workspace,
        workspace_ref: "workspace:test".to_owned(),
        workset_ref: None,
        target_or_environment_ref: Some("target:test".to_owned()),
        active_profile_ref: Some("profile:test".to_owned()),
    }
}

fn settled_snapshot(snapshot_id: &str, source_id: &str) -> DiagnosticCollectionSnapshot {
    DiagnosticCollectionSnapshot::new(DiagnosticCollectionSnapshotInput {
        snapshot_id: snapshot_id.to_owned(),
        snapshot_label: "Complete settled enumeration.".to_owned(),
        surface: M5DiagnosticSurface::LanguageProviderDiagnostics,
        scope: scope(),
        completeness_class: DiagnosticCollectionCompletenessClass::CompleteEnumeration,
        freshness_class: DiagnosticFreshnessClass::Current,
        streaming_state: DiagnosticCollectionStreamingState::Settled,
        origin_class: DiagnosticOriginClass::LiveLocalSession,
        created_at: MINTED_AT.to_owned(),
        diagnostic_refs: refs(&[&format!("diagnostic:{snapshot_id}:0001")]),
        streaming_cursor: None,
        omitted_scopes: Vec::new(),
        contributing_source_ids: refs(&[source_id]),
        completeness_disclosed: false,
        imported_not_shown_as_live: true,
        export_safe_summary: "Whole-workspace settled enumeration.".to_owned(),
    })
}

fn entry(snapshot: DiagnosticCollectionSnapshot) -> DiagnosticCollectionSnapshotEntry {
    DiagnosticCollectionSnapshotEntry {
        entry_id: format!("entry:{}", snapshot.snapshot_id),
        snapshot,
        claimed_qualification: DiagnosticCollectionQualificationClass::Beta,
        effective_qualification: DiagnosticCollectionQualificationClass::Beta,
        downgrade_trigger: None,
        degraded_label: None,
        evidence_refs: refs(&["evidence:test:0001"]),
        source_contract_refs: refs(&[M5_SOURCE_AND_COLLECTION_DOC_REF]),
    }
}

fn guardrails() -> DiagnosticSourceAndCollectionGuardrails {
    DiagnosticSourceAndCollectionGuardrails {
        unlike_sources_never_flattened: true,
        source_descriptors_survive_normalization: true,
        imported_live_class_explicit: true,
        target_environment_refs_preserved: true,
        completeness_label_always_present: true,
        omitted_scopes_named_with_reasons: true,
        ids_and_completeness_exportable: true,
        snapshots_auto_downgrade_on_weak_truth: true,
    }
}

fn consumer_projection() -> DiagnosticSourceAndCollectionConsumerProjection {
    DiagnosticSourceAndCollectionConsumerProjection {
        problems_shows_source_and_completeness: true,
        review_carries_source_and_completeness: true,
        saved_views_preserve_source_and_completeness: true,
        cli_headless_prints_source_and_completeness: true,
        support_export_carries_source_and_completeness: true,
        omitted_scopes_visible_on_every_surface: true,
    }
}

fn evidence_freshness() -> DiagnosticSourceAndCollectionEvidenceFreshness {
    DiagnosticSourceAndCollectionEvidenceFreshness {
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: MINTED_AT.to_owned(),
        auto_downgrade_on_stale: true,
    }
}

fn single_entry_packet(
    entry: DiagnosticCollectionSnapshotEntry,
) -> DiagnosticSourceAndCollectionPacket {
    let source_id = entry.snapshot.contributing_source_ids[0].clone();
    DiagnosticSourceAndCollectionPacket::new(DiagnosticSourceAndCollectionPacketInput {
        packet_id: "packet:test:0001".to_owned(),
        packet_label: "Test source-and-collection packet".to_owned(),
        source_descriptors: vec![source(&source_id, DiagnosticSourceKind::LanguageService)],
        snapshot_entries: vec![entry],
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        evidence_freshness: evidence_freshness(),
        source_contract_refs: refs(&[
            M5_SOURCE_AND_COLLECTION_SCHEMA_REF,
            M5_SOURCE_DESCRIPTOR_SCHEMA_REF,
            M5_COLLECTION_SNAPSHOT_SCHEMA_REF,
            M5_SOURCE_AND_COLLECTION_DOC_REF,
            M5_SOURCE_AND_COLLECTION_ARTIFACT_REF,
        ]),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

// ---- Checked artifact ----------------------------------------------------

#[test]
fn checked_export_validates_and_covers_all_surfaces_and_families() {
    let packet = current_m5_source_and_collection_export()
        .expect("checked source-and-collection export validates");
    assert!(packet.validate().is_empty());

    let surfaces = packet.represented_surfaces();
    for required in M5DiagnosticSurface::ALL {
        assert!(surfaces.contains(&required), "missing surface {required:?}");
    }
    let families = packet.represented_source_kinds();
    for required in DiagnosticSourceKind::ALL_BETA_CLAIMED {
        assert!(families.contains(&required), "missing family {required:?}");
    }
    assert_eq!(packet.downgraded_entry_count(), 1);
    assert_eq!(packet.claimed_entry_count(), packet.snapshot_entries.len());
}

#[test]
fn checked_export_round_trips() {
    let packet = current_m5_source_and_collection_export().expect("export validates");
    let json = packet.export_safe_json();
    let parsed: DiagnosticSourceAndCollectionPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_export_demonstrates_each_freshness_and_streaming_state() {
    let packet = current_m5_source_and_collection_export().expect("export validates");
    let mut streaming_states = BTreeSet::new();
    let mut completeness = BTreeSet::new();
    let mut freshness = BTreeSet::new();
    for entry in &packet.snapshot_entries {
        streaming_states.insert(entry.snapshot.streaming_state);
        completeness.insert(entry.snapshot.completeness_class);
        freshness.insert(entry.snapshot.freshness_class);
    }
    // Users can tell whether a set is settled, streaming, partial, or aborted.
    assert!(streaming_states.contains(&DiagnosticCollectionStreamingState::Streaming));
    // Imported snapshot and partial completeness are both demonstrated.
    assert!(completeness.contains(&DiagnosticCollectionCompletenessClass::ImportedSnapshotSet));
    assert!(completeness.contains(&DiagnosticCollectionCompletenessClass::PartialVisibleScan));
    // Stale and imported freshness states are demonstrated alongside current.
    assert!(freshness.contains(&DiagnosticFreshnessClass::ImportedSnapshot));
}

#[test]
fn markdown_summary_names_sources_snapshots_and_degrade() {
    let packet = current_m5_source_and_collection_export().expect("export validates");
    let summary = packet.render_markdown_summary();
    assert!(summary.contains("M5 Diagnostic Source Descriptors and Collection Snapshots"));
    assert!(summary.contains("## Source descriptors"));
    assert!(summary.contains("## Collection snapshots"));
    assert!(summary.contains("Degraded:"));
}

// ---- Completeness, freshness, streaming durability ------------------------

#[test]
fn complete_settled_entry_is_not_downgraded() {
    let entry = entry(settled_snapshot("snapshot:test:0001", "source:test:0001"));
    assert!(entry.snapshot.is_complete());
    assert!(entry.collection_truth_durable());
    assert!(!entry.needs_downgrade());
    assert!(entry.downgrade_consistent());
    assert!(entry.is_structurally_complete());
}

#[test]
fn unverified_freshness_forces_downgrade() {
    let mut snapshot = settled_snapshot("snapshot:test:0002", "source:test:0002");
    snapshot.freshness_class = DiagnosticFreshnessClass::Unverified;
    // Unverified freshness still needs disclosure, so flag it.
    snapshot.completeness_disclosed = true;
    let entry = entry(snapshot);
    assert!(entry.needs_downgrade());
    // Holding the claim is inconsistent.
    assert!(!entry.downgrade_consistent());
}

#[test]
fn aborted_collection_forces_downgrade() {
    let mut snapshot = settled_snapshot("snapshot:test:0003", "source:test:0003");
    snapshot.streaming_state = DiagnosticCollectionStreamingState::Aborted;
    snapshot.completeness_class = DiagnosticCollectionCompletenessClass::PartialVisibleScan;
    snapshot.completeness_disclosed = true;
    snapshot.omitted_scopes = vec![DiagnosticOmittedScope {
        scope_ref: "scope:not-reached".to_owned(),
        reason_class: DiagnosticOmittedScopeReasonClass::BudgetOrTimeoutCut,
        summary: "Analysis aborted before reaching this scope.".to_owned(),
    }];
    let mut entry = entry(snapshot);
    assert!(entry.needs_downgrade());
    // A proper downgrade is consistent.
    entry.effective_qualification = DiagnosticCollectionQualificationClass::Held;
    entry.downgrade_trigger = Some(DiagnosticCollectionDowngradeTrigger::AbortedCollection);
    entry.degraded_label =
        Some("Collection aborted before completion; held below preview.".to_owned());
    assert!(entry.downgrade_consistent());
    assert!(entry.is_structurally_complete());
}

#[test]
fn partial_scope_without_disclosure_is_flagged() {
    let mut snapshot = settled_snapshot("snapshot:test:0004", "source:test:0004");
    snapshot.completeness_class = DiagnosticCollectionCompletenessClass::PartialVisibleScan;
    snapshot.omitted_scopes = vec![DiagnosticOmittedScope {
        scope_ref: "scope:unscanned".to_owned(),
        reason_class: DiagnosticOmittedScopeReasonClass::NotYetScanned,
        summary: "This directory family has not been scanned yet.".to_owned(),
    }];
    // Disclosure cue missing for a partial scan.
    snapshot.completeness_disclosed = false;
    assert!(!snapshot.disclosure_ok());
    let violations = single_entry_packet(entry(snapshot)).validate();
    assert!(
        violations.contains(&DiagnosticSourceAndCollectionViolation::CollectionCompletenessHidden)
    );
}

#[test]
fn partial_scope_with_no_omitted_scope_is_flagged() {
    let mut snapshot = settled_snapshot("snapshot:test:0005", "source:test:0005");
    snapshot.completeness_class = DiagnosticCollectionCompletenessClass::PartialVisibleScan;
    snapshot.completeness_disclosed = true;
    // No omitted scope named for a partial scan.
    snapshot.omitted_scopes = Vec::new();
    assert!(!snapshot.omitted_scopes_sufficient());
    let violations = single_entry_packet(entry(snapshot)).validate();
    assert!(violations.contains(&DiagnosticSourceAndCollectionViolation::OmittedScopeMissing));
}

#[test]
fn malformed_omitted_scope_is_flagged() {
    let mut snapshot = settled_snapshot("snapshot:test:0006", "source:test:0006");
    snapshot.completeness_class = DiagnosticCollectionCompletenessClass::FilteredView;
    snapshot.completeness_disclosed = true;
    snapshot.omitted_scopes = vec![DiagnosticOmittedScope {
        scope_ref: "scope:filtered".to_owned(),
        reason_class: DiagnosticOmittedScopeReasonClass::FilteredBySuppression,
        // Generic non-answer summary.
        summary: "omitted".to_owned(),
    }];
    assert!(!snapshot.omitted_scopes_well_formed());
    let violations = single_entry_packet(entry(snapshot)).validate();
    assert!(violations.contains(&DiagnosticSourceAndCollectionViolation::OmittedScopeMalformed));
}

#[test]
fn streaming_without_cursor_is_inconsistent() {
    let mut snapshot = settled_snapshot("snapshot:test:0007", "source:test:0007");
    snapshot.streaming_state = DiagnosticCollectionStreamingState::Streaming;
    snapshot.completeness_class = DiagnosticCollectionCompletenessClass::PartialVisibleScan;
    snapshot.completeness_disclosed = true;
    snapshot.omitted_scopes = vec![DiagnosticOmittedScope {
        scope_ref: "scope:pending".to_owned(),
        reason_class: DiagnosticOmittedScopeReasonClass::NotYetScanned,
        summary: "Results still arriving for this scope.".to_owned(),
    }];
    // Streaming state with no resumable cursor.
    snapshot.streaming_cursor = None;
    assert!(!snapshot.streaming_consistent());
    let violations = single_entry_packet(entry(snapshot)).validate();
    assert!(
        violations.contains(&DiagnosticSourceAndCollectionViolation::StreamingStateInconsistent)
    );
}

#[test]
fn settled_state_with_cursor_is_inconsistent() {
    let mut snapshot = settled_snapshot("snapshot:test:0008", "source:test:0008");
    snapshot.streaming_cursor = Some(DiagnosticStreamingCursor {
        cursor_token: "cursor:test".to_owned(),
        emitted_count: 3,
        has_more: true,
        resume_hint_ref: None,
        summary: "Resume token.".to_owned(),
    });
    assert!(!snapshot.streaming_consistent());
}

#[test]
fn imported_shown_as_live_is_flagged() {
    let mut snapshot = settled_snapshot("snapshot:test:0009", "source:test:0009");
    snapshot.origin_class = DiagnosticOriginClass::ImportedSnapshot;
    snapshot.completeness_class = DiagnosticCollectionCompletenessClass::ImportedSnapshotSet;
    snapshot.freshness_class = DiagnosticFreshnessClass::ImportedSnapshot;
    snapshot.completeness_disclosed = true;
    // Imported evidence rendered as live local truth.
    snapshot.imported_not_shown_as_live = false;
    assert!(!snapshot.imported_separation_ok());
    let violations = single_entry_packet(entry(snapshot)).validate();
    assert!(violations.contains(&DiagnosticSourceAndCollectionViolation::ImportedShownAsLive));
}

#[test]
fn generic_degraded_label_is_rejected() {
    let mut snapshot = settled_snapshot("snapshot:test:0010", "source:test:0010");
    snapshot.streaming_state = DiagnosticCollectionStreamingState::Aborted;
    snapshot.completeness_class = DiagnosticCollectionCompletenessClass::PartialVisibleScan;
    snapshot.completeness_disclosed = true;
    snapshot.omitted_scopes = vec![DiagnosticOmittedScope {
        scope_ref: "scope:not-reached".to_owned(),
        reason_class: DiagnosticOmittedScopeReasonClass::BudgetOrTimeoutCut,
        summary: "Analysis aborted before reaching this scope.".to_owned(),
    }];
    let mut entry = entry(snapshot);
    entry.effective_qualification = DiagnosticCollectionQualificationClass::Held;
    entry.downgrade_trigger = Some(DiagnosticCollectionDowngradeTrigger::AbortedCollection);
    entry.degraded_label = Some("unavailable".to_owned());
    assert!(!entry.downgrade_consistent());
}

#[test]
fn entry_not_downgraded_on_weak_truth_is_flagged() {
    let mut snapshot = settled_snapshot("snapshot:test:0011", "source:test:0011");
    snapshot.completeness_class = DiagnosticCollectionCompletenessClass::UnknownRequiresReview;
    snapshot.completeness_disclosed = true;
    // The entry keeps its beta claim despite unknown completeness.
    let violations = single_entry_packet(entry(snapshot)).validate();
    assert!(
        violations.contains(&DiagnosticSourceAndCollectionViolation::EntryNotDowngradedOnWeakTruth)
    );
}

// ---- Source descriptor provenance ----------------------------------------

#[test]
fn source_descriptor_missing_target_fingerprint_is_flagged() {
    let packet = current_m5_source_and_collection_export().expect("export validates");
    let mut mutated = packet.clone();
    mutated.source_descriptors[0].target_or_environment_ref = None;
    let violations = mutated.validate();
    assert!(violations
        .contains(&DiagnosticSourceAndCollectionViolation::SourceDescriptorProvenanceMissing));
}

#[test]
fn source_descriptor_missing_tool_version_is_flagged() {
    let packet = current_m5_source_and_collection_export().expect("export validates");
    let mut mutated = packet.clone();
    mutated.source_descriptors[0].tool_version_ref = None;
    let violations = mutated.validate();
    assert!(violations
        .contains(&DiagnosticSourceAndCollectionViolation::SourceDescriptorProvenanceMissing));
}

#[test]
fn missing_source_family_is_flagged() {
    let packet = current_m5_source_and_collection_export().expect("export validates");
    let mut mutated = packet.clone();
    mutated
        .source_descriptors
        .retain(|source| source.source_kind != DiagnosticSourceKind::Heuristic);
    let violations = mutated.validate();
    assert!(
        violations.contains(&DiagnosticSourceAndCollectionViolation::RequiredSourceFamilyMissing)
    );
}

#[test]
fn contributing_source_not_in_packet_is_flagged() {
    let mut snapshot = settled_snapshot("snapshot:test:0012", "source:does-not-exist");
    snapshot.contributing_source_ids = refs(&["source:does-not-exist"]);
    // Packet's only source descriptor uses this id, so swap it to mismatch.
    let entry = entry(snapshot);
    let mut packet = single_entry_packet(entry);
    packet.source_descriptors = vec![source(
        "source:other",
        DiagnosticSourceKind::LanguageService,
    )];
    let violations = packet.validate();
    assert!(violations.contains(&DiagnosticSourceAndCollectionViolation::ContributingSourceMissing));
}

// ---- Packet-level governance ---------------------------------------------

#[test]
fn missing_required_surface_is_flagged() {
    let entry = entry(settled_snapshot("snapshot:test:0013", "source:test:0013"));
    let violations = single_entry_packet(entry).validate();
    assert!(violations.contains(&DiagnosticSourceAndCollectionViolation::RequiredSurfaceMissing));
    // A lone settled entry is no auto-downgrade demonstration either.
    assert!(
        violations.contains(&DiagnosticSourceAndCollectionViolation::DowngradedEntryCaseMissing)
    );
}

#[test]
fn missing_source_contract_is_flagged() {
    let packet = current_m5_source_and_collection_export().expect("export validates");
    let mut mutated = packet.clone();
    mutated
        .source_contract_refs
        .retain(|r| r != M5_COLLECTION_SNAPSHOT_SCHEMA_REF);
    let violations = mutated.validate();
    assert!(violations.contains(&DiagnosticSourceAndCollectionViolation::MissingSourceContracts));
}

#[test]
fn forbidden_boundary_material_is_flagged() {
    let packet = current_m5_source_and_collection_export().expect("export validates");
    let mut mutated = packet.clone();
    mutated.packet_label = "leaked password material".to_owned();
    let violations = mutated.validate();
    assert!(
        violations.contains(&DiagnosticSourceAndCollectionViolation::RawBoundaryMaterialInExport)
    );
}

#[test]
fn incomplete_guardrails_are_flagged() {
    let packet = current_m5_source_and_collection_export().expect("export validates");
    let mut mutated = packet.clone();
    mutated.guardrails.omitted_scopes_named_with_reasons = false;
    let violations = mutated.validate();
    assert!(violations.contains(&DiagnosticSourceAndCollectionViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_is_flagged() {
    let packet = current_m5_source_and_collection_export().expect("export validates");
    let mut mutated = packet.clone();
    mutated
        .consumer_projection
        .saved_views_preserve_source_and_completeness = false;
    let violations = mutated.validate();
    assert!(
        violations.contains(&DiagnosticSourceAndCollectionViolation::ConsumerProjectionIncomplete)
    );
}
