use super::*;

use crate::diagnostics::{
    DiagnosticAnchorRemap, DiagnosticAnchorRemapStateClass, DiagnosticCausalLink,
    DiagnosticCausalLinkKind, DiagnosticEvidencePlaneClass, DiagnosticFreshnessClass,
    DiagnosticOriginClass, DiagnosticRecord, DiagnosticSeverityClass, DiagnosticSource,
    DiagnosticSourceConfidenceClass, DiagnosticSourceKind, DiagnosticSupportClass,
    DiagnosticSurfaceRefs,
};
use crate::quality::{BaselineCompatibilityStateClass, QualityDebtReopenStateClass};

const MINTED_AT: &str = "2026-06-19T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn surface_refs(diagnostic_id: &str) -> DiagnosticSurfaceRefs {
    DiagnosticSurfaceRefs {
        editor_decoration_ref: format!("editor:{diagnostic_id}"),
        problems_row_ref: format!("problems:{diagnostic_id}"),
        output_entry_ref: format!("output:{diagnostic_id}"),
        timeline_entry_ref: format!("timeline:{diagnostic_id}"),
        rerun_action_ref: format!("rerun:{diagnostic_id}"),
        review_packet_ref: format!("review:{diagnostic_id}"),
        cli_explain_ref: format!("cli:{diagnostic_id}"),
        ai_evidence_ref: format!("ai:{diagnostic_id}"),
        support_export_ref: format!("support:{diagnostic_id}"),
    }
}

fn record(diagnostic_id: &str, family: &str) -> DiagnosticRecord {
    let mut source = DiagnosticSource::new(
        format!("source:{diagnostic_id}"),
        DiagnosticSourceKind::LanguageService,
        DiagnosticEvidencePlaneClass::StaticAnalysis,
        DiagnosticOriginClass::LiveLocalSession,
        DiagnosticSourceConfidenceClass::Authoritative,
        DiagnosticSupportClass::Authoritative,
        format!("producer:{diagnostic_id}"),
        format!("tool:{diagnostic_id}"),
        Some(format!("tool-version:{diagnostic_id}")),
        "Normalized source descriptor.".to_owned(),
    );
    source.originating_session_ref = Some(format!("session:{diagnostic_id}"));
    let anchor_remap = DiagnosticAnchorRemap::new(
        format!("remap:{diagnostic_id}"),
        family.to_owned(),
        Some(format!("anchor:{diagnostic_id}:origin")),
        Some(format!("anchor:{diagnostic_id}:current")),
        DiagnosticAnchorRemapStateClass::Exact,
        format!("evidence:anchor:{diagnostic_id}"),
        MINTED_AT.to_owned(),
        "Append-only anchor remap evidence.".to_owned(),
    );
    let mut built = DiagnosticRecord::new(
        diagnostic_id.to_owned(),
        format!("rule:{diagnostic_id}"),
        format!("category:{diagnostic_id}"),
        DiagnosticSeverityClass::Warning,
        source,
        DiagnosticFreshnessClass::Current,
        anchor_remap,
        DiagnosticSupportClass::Authoritative,
        format!("message:{diagnostic_id}"),
        surface_refs(diagnostic_id),
        MINTED_AT.to_owned(),
        format!("Normalized diagnostic record {diagnostic_id}."),
    );
    built.causal_links = vec![DiagnosticCausalLink::new(
        DiagnosticCausalLinkKind::AdapterSession,
        format!("adapter-session:{diagnostic_id}"),
        "Producer adapter session emitted the finding.",
    )];
    built
}

fn handle(diagnostic_id: &str, surface_class: DiagnosticSurfaceClass) -> DiagnosticReopenHandle {
    DiagnosticReopenHandle {
        surface_class,
        stable_surface_ref: format!("{}:{diagnostic_id}", surface_class.as_str()),
        resolves_diagnostic_id: diagnostic_id.to_owned(),
        cites_canonical_id: true,
        preserves_detail: true,
    }
}

fn full_handles(diagnostic_id: &str) -> Vec<DiagnosticReopenHandle> {
    REQUIRED_REOPEN_SURFACES
        .iter()
        .map(|surface_class| handle(diagnostic_id, *surface_class))
        .collect()
}

fn full_identity(diagnostic_id: &str, family: &str) -> DiagnosticStableIdentityFamily {
    let observe = |context: DiagnosticIdentityContextClass| DiagnosticIdentityObservation {
        context_class: context,
        observed_diagnostic_id: diagnostic_id.to_owned(),
        observed_anchor_family_id: family.to_owned(),
        note: "Identity resolved to the same canonical id.".to_owned(),
    };
    DiagnosticStableIdentityFamily {
        diagnostic_id: diagnostic_id.to_owned(),
        anchor_family_id: family.to_owned(),
        observations: vec![
            observe(DiagnosticIdentityContextClass::InitialEmit),
            observe(DiagnosticIdentityContextClass::OrdinaryRefresh),
            observe(DiagnosticIdentityContextClass::SurfaceHop),
            observe(DiagnosticIdentityContextClass::PresentationChange),
        ],
    }
}

fn entry(diagnostic_id: &str, family: &str) -> NormalizedDiagnosticRecordEntry {
    NormalizedDiagnosticRecordEntry {
        entry_id: format!("entry:{diagnostic_id}"),
        surface: M5DiagnosticSurface::LanguageProviderDiagnostics,
        label_summary: "Normalized finding.".to_owned(),
        record: record(diagnostic_id, family),
        identity_family: full_identity(diagnostic_id, family),
        reopen_handles: full_handles(diagnostic_id),
        suppression_joins: Vec::new(),
        baseline_joins: Vec::new(),
        claimed_qualification: NormalizedRecordQualificationClass::Beta,
        effective_qualification: NormalizedRecordQualificationClass::Beta,
        downgrade_trigger: None,
        degraded_label: None,
        evidence_refs: refs(&[&format!("evidence:{diagnostic_id}")]),
        source_contract_refs: refs(&[M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_DOC_REF]),
    }
}

// ---- Checked artifact ----------------------------------------------------

#[test]
fn checked_export_validates_and_covers_all_surfaces() {
    let packet = current_m5_normalized_diagnostic_record_set_export()
        .expect("checked normalized diagnostic-record set export validates");
    assert!(packet.validate().is_empty());
    assert_eq!(packet.entries.len(), 9);
    let surfaces = packet.represented_surfaces();
    for required in M5DiagnosticSurface::ALL {
        assert!(surfaces.contains(&required), "missing surface {required:?}");
    }
    assert_eq!(packet.downgraded_entry_count(), 1);
    assert_eq!(packet.claimed_entry_count(), 9);
}

#[test]
fn checked_export_round_trips() {
    let packet = current_m5_normalized_diagnostic_record_set_export().expect("export validates");
    let json = packet.export_safe_json();
    let parsed: NormalizedDiagnosticRecordSetPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_export_carries_attached_suppression_and_baseline_joins() {
    let packet = current_m5_normalized_diagnostic_record_set_export().expect("export validates");
    let suppression = packet
        .entries
        .iter()
        .flat_map(|entry| entry.suppression_joins.iter())
        .count();
    let baseline = packet
        .entries
        .iter()
        .flat_map(|entry| entry.baseline_joins.iter())
        .count();
    assert!(suppression >= 1, "expected at least one suppression join");
    assert!(baseline >= 1, "expected at least one baseline join");
    // Every join in the checked set is attached to its record's own refs.
    for entry in &packet.entries {
        assert!(entry.joins_attached());
    }
}

#[test]
fn markdown_summary_names_entries_and_degrade() {
    let packet = current_m5_normalized_diagnostic_record_set_export().expect("export validates");
    let summary = packet.render_markdown_summary();
    assert!(summary.contains("M5 Normalized Diagnostic-Record Set"));
    assert!(summary.contains("diagnostic:m5:notebook-cell:0001"));
    assert!(summary.contains("Degraded:"));
}

// ---- Identity completeness and auto-downgrade ----------------------------

#[test]
fn complete_entry_is_not_downgraded() {
    let entry = entry("diagnostic:test:0001", "anchor-family:test:0001");
    assert!(entry.identity_complete());
    assert!(!entry.needs_downgrade());
    assert!(entry.downgrade_consistent());
    assert!(entry.is_structurally_complete());
}

#[test]
fn missing_reopen_surface_forces_downgrade() {
    let mut entry = entry("diagnostic:test:0002", "anchor-family:test:0002");
    entry
        .reopen_handles
        .retain(|handle| handle.surface_class != DiagnosticSurfaceClass::AiEvidence);
    assert!(!entry.reopen_complete());
    assert!(entry.needs_downgrade());

    // Holding the claim is inconsistent.
    assert!(!entry.downgrade_consistent());

    // A proper downgrade is consistent.
    entry.effective_qualification = NormalizedRecordQualificationClass::Held;
    entry.downgrade_trigger = Some(NormalizedRecordDowngradeTrigger::MissingReopenSurface);
    entry.degraded_label = Some("AI evidence cannot reopen this record yet.".to_owned());
    assert!(entry.downgrade_consistent());
    assert!(entry.is_structurally_complete());
}

#[test]
fn unproven_identity_forces_downgrade() {
    let mut entry = entry("diagnostic:test:0003", "anchor-family:test:0003");
    entry
        .identity_family
        .observations
        .retain(|obs| obs.context_class != DiagnosticIdentityContextClass::PresentationChange);
    assert!(!entry.identity_proven());
    assert!(entry.needs_downgrade());
}

#[test]
fn unstable_observation_breaks_identity() {
    let mut family = full_identity("diagnostic:test:0004", "anchor-family:test:0004");
    family.observations[1].observed_diagnostic_id = "diagnostic:other:9999".to_owned();
    assert!(!family.all_observations_stable());
    assert!(!family.is_proven());
}

#[test]
fn missing_normalized_provenance_forces_downgrade() {
    let mut entry = entry("diagnostic:test:0005", "anchor-family:test:0005");
    // Drop the tool version so the record can no longer emit a beta source.
    entry.record.source.tool_version_ref = None;
    assert!(!entry.normalized_provenance_ok());
    assert!(entry.needs_downgrade());
}

#[test]
fn generic_degraded_label_is_rejected() {
    let mut entry = entry("diagnostic:test:0006", "anchor-family:test:0006");
    entry
        .reopen_handles
        .retain(|handle| handle.surface_class != DiagnosticSurfaceClass::Review);
    entry.effective_qualification = NormalizedRecordQualificationClass::Held;
    entry.downgrade_trigger = Some(NormalizedRecordDowngradeTrigger::MissingReopenSurface);
    entry.degraded_label = Some("unavailable".to_owned());
    assert!(!entry.downgrade_consistent());
}

// ---- Suppression / baseline joins ----------------------------------------

fn guardrails() -> NormalizedDiagnosticRecordSetGuardrails {
    NormalizedDiagnosticRecordSetGuardrails {
        stable_ids_survive_refresh_and_surface_hop: true,
        unlike_sources_never_flattened: true,
        clustering_never_erases_provenance: true,
        imported_live_class_explicit: true,
        freshness_and_confidence_in_detail_paths: true,
        suppression_baseline_joins_attached_to_records: true,
        mutating_fixes_are_typed_proposals: true,
        records_auto_downgrade_on_incomplete_identity: true,
    }
}

fn consumer_projection() -> NormalizedDiagnosticRecordConsumerProjection {
    NormalizedDiagnosticRecordConsumerProjection {
        editor_reopens_record: true,
        problems_reopens_record: true,
        review_reopens_record: true,
        cli_headless_reopens_record: true,
        ai_evidence_reopens_record: true,
        support_export_reopens_record: true,
        compact_surfaces_preserve_class_in_detail: true,
    }
}

fn evidence_freshness() -> NormalizedDiagnosticRecordEvidenceFreshness {
    NormalizedDiagnosticRecordEvidenceFreshness {
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: MINTED_AT.to_owned(),
        auto_downgrade_on_stale: true,
    }
}

fn single_entry_packet(
    entry: NormalizedDiagnosticRecordEntry,
) -> NormalizedDiagnosticRecordSetPacket {
    NormalizedDiagnosticRecordSetPacket::new(NormalizedDiagnosticRecordSetPacketInput {
        packet_id: "packet:test:0001".to_owned(),
        set_label: "Test normalized diagnostic-record set".to_owned(),
        entries: vec![entry],
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        evidence_freshness: evidence_freshness(),
        source_contract_refs: refs(&[
            M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_SCHEMA_REF,
            M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_DOC_REF,
            M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_ARTIFACT_REF,
            CANONICAL_DIAGNOSTIC_RECORD_SCHEMA_REF,
        ]),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

#[test]
fn attached_suppression_join_passes() {
    let mut entry = entry("diagnostic:test:0010", "anchor-family:test:0010");
    let suppression_id = "suppression:test:0010".to_owned();
    entry.record.suppression_refs = vec![suppression_id.clone()];
    entry.suppression_joins = vec![DiagnosticSuppressionJoin {
        join_id: "suppression-join:test:0010".to_owned(),
        diagnostic_id: entry.record.diagnostic_id.clone(),
        suppression_id,
        scope_class: QualityTargetScopeClass::Workspace,
        reopen_state_class: QualityDebtReopenStateClass::Active,
        release_visible: true,
        attached_to_record: true,
        summary: "Governed suppression attached to the normalized record.".to_owned(),
    }];
    assert!(entry.joins_attached());
}

#[test]
fn detached_suppression_join_is_flagged() {
    let mut entry = entry("diagnostic:test:0011", "anchor-family:test:0011");
    let suppression_id = "suppression:test:0011".to_owned();
    entry.record.suppression_refs = vec![suppression_id.clone()];
    entry.suppression_joins = vec![DiagnosticSuppressionJoin {
        join_id: "suppression-join:test:0011".to_owned(),
        diagnostic_id: entry.record.diagnostic_id.clone(),
        suppression_id,
        scope_class: QualityTargetScopeClass::Workspace,
        reopen_state_class: QualityDebtReopenStateClass::Active,
        release_visible: true,
        // Declared detached: the join is not attached to the record.
        attached_to_record: false,
        summary: "Suppression hidden in feature-local metadata.".to_owned(),
    }];
    assert!(!entry.joins_attached());
    let violations = single_entry_packet(entry).validate();
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::JoinDetachedFromRecord));
}

#[test]
fn suppression_join_not_reflected_on_record_is_flagged() {
    let mut entry = entry("diagnostic:test:0012", "anchor-family:test:0012");
    // Join claims attachment but the record's own refs do not carry it.
    entry.suppression_joins = vec![DiagnosticSuppressionJoin {
        join_id: "suppression-join:test:0012".to_owned(),
        diagnostic_id: entry.record.diagnostic_id.clone(),
        suppression_id: "suppression:test:0012".to_owned(),
        scope_class: QualityTargetScopeClass::Workspace,
        reopen_state_class: QualityDebtReopenStateClass::Active,
        release_visible: true,
        attached_to_record: true,
        summary: "Join not reflected on the record refs.".to_owned(),
    }];
    assert!(!entry.joins_attached());
    let violations = single_entry_packet(entry).validate();
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::JoinDetachedFromRecord));
}

#[test]
fn baseline_join_referential_mismatch_is_flagged() {
    let mut entry = entry("diagnostic:test:0013", "anchor-family:test:0013");
    let baseline_id = "baseline:test:0013".to_owned();
    entry.record.baseline_refs = vec![baseline_id.clone()];
    entry.baseline_joins = vec![DiagnosticBaselineJoin {
        join_id: "baseline-join:test:0013".to_owned(),
        // Wrong diagnostic id: this join belongs to another record.
        diagnostic_id: "diagnostic:other:9999".to_owned(),
        baseline_id,
        compatibility_state_class: BaselineCompatibilityStateClass::Compatible,
        accepted_in_baseline: true,
        attached_to_record: true,
        summary: "Baseline join with a mismatched diagnostic id.".to_owned(),
    }];
    assert!(!entry.referential_integrity_ok());
    let violations = single_entry_packet(entry).validate();
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::RecordReferentialMismatch));
}

// ---- Referential and structural integrity --------------------------------

#[test]
fn identity_family_mismatch_is_flagged() {
    let mut entry = entry("diagnostic:test:0020", "anchor-family:test:0020");
    entry.identity_family.diagnostic_id = "diagnostic:other:0020".to_owned();
    assert!(!entry.referential_integrity_ok());
    let violations = single_entry_packet(entry).validate();
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::RecordReferentialMismatch));
}

#[test]
fn reopen_handle_wrong_id_is_flagged() {
    let mut entry = entry("diagnostic:test:0021", "anchor-family:test:0021");
    entry.reopen_handles[0].resolves_diagnostic_id = "diagnostic:other:0021".to_owned();
    let violations = single_entry_packet(entry).validate();
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::ReopenHandleInvalid));
}

#[test]
fn complete_single_entry_has_no_entry_level_violations() {
    let entry = entry("diagnostic:test:0022", "anchor-family:test:0022");
    assert!(entry.is_structurally_complete());
    // A lone entry cannot cover all surfaces, so packet-level coverage
    // violations remain; the entry itself must raise no entry-level violation.
    let violations = single_entry_packet(entry).validate();
    for forbidden in [
        NormalizedDiagnosticRecordViolation::EntryStructurallyIncomplete,
        NormalizedDiagnosticRecordViolation::RecordReferentialMismatch,
        NormalizedDiagnosticRecordViolation::JoinDetachedFromRecord,
        NormalizedDiagnosticRecordViolation::ReopenHandleInvalid,
        NormalizedDiagnosticRecordViolation::EntryEvidenceMissing,
        NormalizedDiagnosticRecordViolation::EntryNotDowngradedOnIncompleteIdentity,
    ] {
        assert!(!violations.contains(&forbidden), "unexpected {forbidden:?}");
    }
}

#[test]
fn missing_required_surface_is_flagged() {
    // A set whose only entry is language-provider does not cover all M5 surfaces.
    let entry = entry("diagnostic:test:0023", "anchor-family:test:0023");
    let violations = single_entry_packet(entry).validate();
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::RequiredSurfaceMissing));
    // It also lacks a downgraded demonstration entry.
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::DowngradedEntryCaseMissing));
}

#[test]
fn duplicate_diagnostic_id_is_flagged() {
    let packet = current_m5_normalized_diagnostic_record_set_export().expect("export validates");
    let mut mutated = packet.clone();
    let first = mutated.entries[0].clone();
    mutated.entries.push(first);
    let violations = mutated.validate();
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::DuplicateDiagnosticId));
}

#[test]
fn missing_source_contract_is_flagged() {
    let packet = current_m5_normalized_diagnostic_record_set_export().expect("export validates");
    let mut mutated = packet.clone();
    mutated
        .source_contract_refs
        .retain(|r| r != CANONICAL_DIAGNOSTIC_RECORD_SCHEMA_REF);
    let violations = mutated.validate();
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::MissingSourceContracts));
}

#[test]
fn forbidden_boundary_material_is_flagged() {
    let packet = current_m5_normalized_diagnostic_record_set_export().expect("export validates");
    let mut mutated = packet.clone();
    mutated.set_label = "leaked password material".to_owned();
    let violations = mutated.validate();
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::RawBoundaryMaterialInExport));
}

#[test]
fn incomplete_guardrails_are_flagged() {
    let packet = current_m5_normalized_diagnostic_record_set_export().expect("export validates");
    let mut mutated = packet.clone();
    mutated
        .guardrails
        .suppression_baseline_joins_attached_to_records = false;
    let violations = mutated.validate();
    assert!(violations.contains(&NormalizedDiagnosticRecordViolation::GuardrailsIncomplete));
}
