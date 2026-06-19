use super::*;

use crate::diagnostics::{
    DiagnosticAnchorRemap, DiagnosticAnchorRemapStateClass, DiagnosticCausalLink,
    DiagnosticCausalLinkKind, DiagnosticEvidencePlaneClass, DiagnosticFreshnessClass,
    DiagnosticOriginClass, DiagnosticRecord, DiagnosticSeverityClass, DiagnosticSource,
    DiagnosticSourceConfidenceClass, DiagnosticSourceKind, DiagnosticSupportClass,
    DiagnosticSurfaceRefs,
};

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

#[allow(clippy::too_many_arguments)]
fn record(
    diagnostic_id: &str,
    family: &str,
    source_kind: DiagnosticSourceKind,
    origin: DiagnosticOriginClass,
    severity: DiagnosticSeverityClass,
    freshness: DiagnosticFreshnessClass,
    remap_state: DiagnosticAnchorRemapStateClass,
) -> DiagnosticRecord {
    let confidence = if origin.is_imported_or_replayed() {
        DiagnosticSourceConfidenceClass::ImportedAuthoritative
    } else {
        DiagnosticSourceConfidenceClass::Authoritative
    };
    let mut source = DiagnosticSource::new(
        format!("source:{diagnostic_id}"),
        source_kind,
        DiagnosticEvidencePlaneClass::StaticAnalysis,
        origin,
        confidence,
        DiagnosticSupportClass::Authoritative,
        format!("producer:{diagnostic_id}"),
        format!("tool:{diagnostic_id}"),
        Some(format!("tool-version:{diagnostic_id}")),
        "Source descriptor.".to_owned(),
    );
    source.target_or_environment_ref = Some(format!("target:{diagnostic_id}"));
    if origin.is_imported_or_replayed() {
        source.import_ref = Some(format!("import:{diagnostic_id}"));
    } else {
        source.originating_session_ref = Some(format!("session:{diagnostic_id}"));
    }
    let anchor_remap = DiagnosticAnchorRemap::new(
        format!("remap:{diagnostic_id}"),
        family.to_owned(),
        Some(format!("anchor:{diagnostic_id}:origin")),
        Some(format!("anchor:{diagnostic_id}:current")),
        remap_state,
        format!("evidence:anchor:{diagnostic_id}"),
        MINTED_AT.to_owned(),
        "Append-only anchor remap evidence.".to_owned(),
    );
    let mut built = DiagnosticRecord::new(
        diagnostic_id.to_owned(),
        format!("rule:{diagnostic_id}"),
        format!("category:{diagnostic_id}"),
        severity,
        source,
        freshness,
        anchor_remap,
        DiagnosticSupportClass::Authoritative,
        format!("message:{diagnostic_id}"),
        surface_refs(diagnostic_id),
        MINTED_AT.to_owned(),
        format!("Diagnostic record {diagnostic_id}."),
    );
    built.causal_links = vec![DiagnosticCausalLink::new(
        DiagnosticCausalLinkKind::AdapterSession,
        format!("adapter-session:{diagnostic_id}"),
        "Producer adapter session emitted the finding.",
    )];
    built
}

fn member(surface: M5DiagnosticSurface, record: DiagnosticRecord) -> DiagnosticClusterMemberInput {
    let reopen_surface_ref = format!("problems:{}", record.diagnostic_id);
    DiagnosticClusterMemberInput {
        surface,
        record,
        reopen_surface_ref,
    }
}

fn cross_source_members() -> Vec<DiagnosticClusterMemberInput> {
    let family = "anchor-family:test-cross:0001";
    vec![
        member(
            M5DiagnosticSurface::LanguageProviderDiagnostics,
            record(
                "diagnostic:test:language:0001",
                family,
                DiagnosticSourceKind::LanguageService,
                DiagnosticOriginClass::LiveLocalSession,
                DiagnosticSeverityClass::Error,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
            ),
        ),
        member(
            M5DiagnosticSurface::ImportedScannerDiagnostics,
            record(
                "diagnostic:test:scanner:0001",
                family,
                DiagnosticSourceKind::ScannerImport,
                DiagnosticOriginClass::ImportedSnapshot,
                DiagnosticSeverityClass::Warning,
                DiagnosticFreshnessClass::ImportedSnapshot,
                DiagnosticAnchorRemapStateClass::ImportedStatic,
            ),
        ),
    ]
}

fn cross_source_cluster() -> DiagnosticDisplayCluster {
    DiagnosticDisplayCluster::from_members(
        "cluster:test:cross:0001",
        "Cross-source corroboration test cluster",
        "diagnostic:test:language:0001",
        DiagnosticClusterMeaningClass::CrossSourceCorroboration,
        "Two distinct sources flagged the same anchor family.",
        &cross_source_members(),
        "Cross-source cluster preserves both members.",
    )
}

fn guardrails() -> DiagnosticClusterGuardrails {
    DiagnosticClusterGuardrails {
        unlike_sources_clustered_not_flattened: true,
        no_synthetic_findings: true,
        anchors_never_silently_repaired: true,
        imported_live_class_preserved_in_detail: true,
        target_environment_refs_preserved_in_detail: true,
        policy_state_preserved_in_detail: true,
        dedupe_reason_exposed_on_required_surfaces: true,
        diagnostic_ids_and_completeness_exportable: true,
        every_constituent_recoverable_from_detail_sheet: true,
    }
}

fn consumer_projection() -> DiagnosticClusterConsumerProjection {
    DiagnosticClusterConsumerProjection {
        problems_exposes_cluster_membership: true,
        review_exposes_cluster_membership: true,
        support_export_preserves_constituents: true,
        ai_evidence_exposes_cluster_membership: true,
        editor_detail_sheet_recovers_each_member: true,
        cli_headless_lists_dedupe_reason: true,
    }
}

fn packet_with(clusters: Vec<DiagnosticDisplayCluster>) -> DiagnosticClusterSetPacket {
    DiagnosticClusterSetPacket::new(DiagnosticClusterSetPacketInput {
        packet_id: "packet:test:0001".to_owned(),
        set_label: "Test diagnostic-cluster set".to_owned(),
        workspace_id: "workspace:test".to_owned(),
        clusters,
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: refs(&[
            M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_REF,
            M5_DIAGNOSTIC_CLUSTER_SET_DOC_REF,
            M5_DIAGNOSTIC_CLUSTER_SET_ARTIFACT_REF,
            CANONICAL_DIAGNOSTIC_RECORD_SET_SCHEMA_REF,
        ]),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

// ---- Checked artifact ----------------------------------------------------

#[test]
fn checked_export_validates() {
    let packet = current_m5_diagnostic_cluster_set_export()
        .expect("checked diagnostic-cluster set export validates");
    assert!(packet.validate().is_empty());
    assert_eq!(packet.clusters.len(), 4);
    assert!(packet.cross_source_cluster_count() >= 1);
}

#[test]
fn checked_export_round_trips() {
    let packet = current_m5_diagnostic_cluster_set_export().expect("export validates");
    let json = packet.export_safe_json();
    let parsed: DiagnosticClusterSetPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_export_preserves_cross_source_members() {
    let packet = current_m5_diagnostic_cluster_set_export().expect("export validates");
    let cross = packet
        .clusters
        .iter()
        .find(|cluster| cluster.is_cross_source())
        .expect("a cross-source cluster exists");
    // Distinct source kinds are preserved, not flattened.
    assert!(cross.preserved_source_kinds.len() >= 2);
    assert!(cross.not_flattened());
    assert!(cross.recovers_every_member());
    // An imported member keeps its imported class explicit.
    assert!(cross
        .member_detail_sheets
        .iter()
        .any(|sheet| sheet.imported_live_class.is_imported_or_replayed()));
    assert!(cross
        .member_detail_sheets
        .iter()
        .any(|sheet| sheet.imported_live_class.is_live()));
}

#[test]
fn checked_export_surface_projections_expose_dedupe_and_membership() {
    let packet = current_m5_diagnostic_cluster_set_export().expect("export validates");
    for cluster in &packet.clusters {
        for surface_class in CLUSTER_EXPOSURE_SURFACES {
            let projection = packet
                .projection_for(&cluster.cluster_id, surface_class)
                .expect("required surface projection exists");
            assert!(projection.is_honest(cluster));
            assert_eq!(
                projection.member_diagnostic_ids,
                cluster.contributing_diagnostic_ids
            );
        }
    }
}

#[test]
fn checked_export_support_export_preserves_constituents() {
    let packet = current_m5_diagnostic_cluster_set_export().expect("export validates");
    assert!(packet.support_export.preserves(&packet.clusters));
    assert!(!packet.support_export.raw_source_content_included);
    assert!(!packet.support_export.raw_payload_included);
    for cluster in &packet.clusters {
        for id in &cluster.contributing_diagnostic_ids {
            assert!(packet
                .support_export
                .all_constituent_diagnostic_refs
                .contains(id));
        }
    }
}

#[test]
fn markdown_summary_names_clusters_and_members() {
    let packet = current_m5_diagnostic_cluster_set_export().expect("export validates");
    let summary = packet.render_markdown_summary();
    assert!(summary.contains("M5 Diagnostic-Cluster Set"));
    assert!(summary.contains("cross_source_corroboration"));
    assert!(summary.contains("Cross-source clusters:"));
}

// ---- Detail sheet truth --------------------------------------------------

#[test]
fn detail_sheet_preserves_source_target_and_imported_class() {
    let rec = record(
        "diagnostic:test:scanner:9001",
        "anchor-family:test:9001",
        DiagnosticSourceKind::ScannerImport,
        DiagnosticOriginClass::ImportedSnapshot,
        DiagnosticSeverityClass::Warning,
        DiagnosticFreshnessClass::ImportedSnapshot,
        DiagnosticAnchorRemapStateClass::ImportedStatic,
    );
    let sheet = DiagnosticClusterMemberDetailSheet::from_record(
        "detail:test:9001",
        M5DiagnosticSurface::ImportedScannerDiagnostics,
        &rec,
        "problems:diagnostic:test:scanner:9001",
    );
    assert_eq!(sheet.member_diagnostic_id, "diagnostic:test:scanner:9001");
    assert_eq!(
        sheet.imported_live_class,
        DiagnosticImportedLiveClass::Imported
    );
    assert!(sheet.imported_live_consistent());
    assert_eq!(
        sheet.target_or_environment_ref.as_deref(),
        Some("target:diagnostic:test:scanner:9001")
    );
    assert!(sheet.disclosure_required);
    assert!(sheet.recovers_member());
}

#[test]
fn cross_source_cluster_is_not_flattened() {
    let cluster = cross_source_cluster();
    assert!(!cluster.synthetic_finding);
    assert!(cluster.is_cross_source());
    assert!(cluster.not_flattened());
    assert!(cluster.preserves_provenance());
    assert!(cluster.aggregate_counts_consistent());
    assert!(cluster.dominant_state_consistent());
    assert!(cluster.is_structurally_complete());
    // The most severe member (error) dominates the compact row.
    assert_eq!(
        cluster.dominant_display_state.dominant_severity_class,
        DiagnosticSeverityClass::Error
    );
    // The most cautionary freshness (imported snapshot) dominates.
    assert_eq!(
        cluster.dominant_display_state.dominant_freshness_class,
        DiagnosticFreshnessClass::ImportedSnapshot
    );
}

// ---- Validation: flattening and provenance -------------------------------

#[test]
fn synthetic_finding_is_flagged() {
    let mut cluster = cross_source_cluster();
    cluster.synthetic_finding = true;
    let violations = packet_with(vec![cluster]).validate();
    assert!(violations.contains(&DiagnosticClusterViolation::SyntheticFindingFlattening));
}

#[test]
fn dropped_member_detail_sheet_is_flagged() {
    let mut cluster = cross_source_cluster();
    // Drop one detail sheet while keeping its contributing id: the member can no
    // longer be recovered and the cluster reads as flattened.
    cluster.member_detail_sheets.pop();
    let violations = packet_with(vec![cluster]).validate();
    assert!(violations.contains(&DiagnosticClusterViolation::SyntheticFindingFlattening));
    assert!(violations.contains(&DiagnosticClusterViolation::MemberNotRecoverable));
}

#[test]
fn dropped_preserved_source_kind_is_flagged() {
    let mut cluster = cross_source_cluster();
    // Drop the imported scanner source kind from the preserved set: provenance
    // was erased by clustering.
    cluster
        .preserved_source_kinds
        .retain(|kind| *kind != DiagnosticSourceKind::ScannerImport);
    assert!(!cluster.preserves_provenance());
    let violations = packet_with(vec![cluster]).validate();
    assert!(violations.contains(&DiagnosticClusterViolation::ClusterDroppedProvenance));
}

#[test]
fn unrecoverable_member_is_flagged() {
    let mut cluster = cross_source_cluster();
    cluster.member_detail_sheets[0].recoverable = false;
    assert!(!cluster.recovers_every_member());
    let violations = packet_with(vec![cluster]).validate();
    assert!(violations.contains(&DiagnosticClusterViolation::MemberNotRecoverable));
}

#[test]
fn primary_not_a_member_is_flagged() {
    let mut cluster = cross_source_cluster();
    cluster.primary_diagnostic_id = "diagnostic:test:not-a-member:0001".to_owned();
    let violations = packet_with(vec![cluster]).validate();
    assert!(violations.contains(&DiagnosticClusterViolation::PrimaryNotAMember));
}

#[test]
fn inconsistent_aggregate_counts_are_flagged() {
    let mut cluster = cross_source_cluster();
    cluster.aggregate_counts.member_count += 5;
    assert!(!cluster.aggregate_counts_consistent());
    let violations = packet_with(vec![cluster]).validate();
    assert!(violations.contains(&DiagnosticClusterViolation::AggregateCountsInconsistent));
}

#[test]
fn inconsistent_dominant_state_is_flagged() {
    let mut cluster = cross_source_cluster();
    cluster.dominant_display_state.dominant_severity_class = DiagnosticSeverityClass::Hint;
    assert!(!cluster.dominant_state_consistent());
    let violations = packet_with(vec![cluster]).validate();
    assert!(violations.contains(&DiagnosticClusterViolation::DominantStateInconsistent));
}

#[test]
fn missing_cross_source_cluster_is_flagged() {
    // A lone single-source, single-member cluster cannot demonstrate cross-source
    // clustering.
    let family = "anchor-family:test-single:0001";
    let cluster = DiagnosticDisplayCluster::from_members(
        "cluster:test:single:0001",
        "Single-source single-member cluster",
        "diagnostic:test:single:0001",
        DiagnosticClusterMeaningClass::NoClustering,
        "One record, no clustering applied.",
        &[member(
            M5DiagnosticSurface::LanguageProviderDiagnostics,
            record(
                "diagnostic:test:single:0001",
                family,
                DiagnosticSourceKind::LanguageService,
                DiagnosticOriginClass::LiveLocalSession,
                DiagnosticSeverityClass::Warning,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
            ),
        )],
        "Single member, no clustering.",
    );
    let violations = packet_with(vec![cluster]).validate();
    assert!(violations.contains(&DiagnosticClusterViolation::CrossSourceClusterMissing));
}

// ---- Validation: projections and export ----------------------------------

#[test]
fn missing_surface_projection_is_flagged() {
    let mut packet = packet_with(vec![cross_source_cluster()]);
    packet
        .surface_projections
        .retain(|projection| projection.surface_class != DiagnosticSurfaceClass::AiEvidence);
    let violations = packet.validate();
    assert!(violations.contains(&DiagnosticClusterViolation::SurfaceProjectionMissing));
}

#[test]
fn projection_dropping_membership_is_flagged() {
    let mut packet = packet_with(vec![cross_source_cluster()]);
    let projection = packet
        .surface_projections
        .iter_mut()
        .find(|projection| projection.surface_class == DiagnosticSurfaceClass::Problems)
        .expect("problems projection exists");
    projection.member_diagnostic_ids.clear();
    let violations = packet.validate();
    assert!(
        violations.contains(&DiagnosticClusterViolation::SurfaceProjectionDropsDedupeOrMembership)
    );
}

#[test]
fn raw_content_in_support_export_is_flagged() {
    let mut packet = packet_with(vec![cross_source_cluster()]);
    packet.support_export.raw_source_content_included = true;
    let violations = packet.validate();
    assert!(violations.contains(&DiagnosticClusterViolation::SupportExportIncludesRawContent));
}

#[test]
fn lossy_support_export_is_flagged() {
    let mut packet = packet_with(vec![cross_source_cluster()]);
    packet.support_export.clustered_constituent_refs.clear();
    let violations = packet.validate();
    assert!(violations.contains(&DiagnosticClusterViolation::SupportExportLossy));
}

// ---- Validation: packet-level invariants ---------------------------------

#[test]
fn empty_cluster_set_is_flagged() {
    let violations = packet_with(Vec::new()).validate();
    assert!(violations.contains(&DiagnosticClusterViolation::NoClusters));
}

#[test]
fn missing_source_contract_is_flagged() {
    let mut packet = packet_with(vec![cross_source_cluster()]);
    packet
        .source_contract_refs
        .retain(|r| r != CANONICAL_DIAGNOSTIC_RECORD_SET_SCHEMA_REF);
    let violations = packet.validate();
    assert!(violations.contains(&DiagnosticClusterViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_are_flagged() {
    let mut packet = packet_with(vec![cross_source_cluster()]);
    packet.guardrails.no_synthetic_findings = false;
    let violations = packet.validate();
    assert!(violations.contains(&DiagnosticClusterViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_is_flagged() {
    let mut packet = packet_with(vec![cross_source_cluster()]);
    packet
        .consumer_projection
        .support_export_preserves_constituents = false;
    let violations = packet.validate();
    assert!(violations.contains(&DiagnosticClusterViolation::ConsumerProjectionIncomplete));
}

#[test]
fn forbidden_boundary_material_is_flagged() {
    let mut packet = packet_with(vec![cross_source_cluster()]);
    packet.set_label = "leaked password material".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&DiagnosticClusterViolation::RawBoundaryMaterialInExport));
}

#[test]
fn complete_packet_has_no_violations() {
    let packet = packet_with(clusters_for_complete_packet());
    assert!(packet.validate().is_empty());
}

fn clusters_for_complete_packet() -> Vec<DiagnosticDisplayCluster> {
    vec![cross_source_cluster(), exact_duplicate_test_cluster()]
}

fn exact_duplicate_test_cluster() -> DiagnosticDisplayCluster {
    let family = "anchor-family:test-dup:0001";
    let make = |id: &str| {
        member(
            M5DiagnosticSurface::NotebookCellDiagnostics,
            record(
                id,
                family,
                DiagnosticSourceKind::RuntimeOrTest,
                DiagnosticOriginClass::LiveLocalSession,
                DiagnosticSeverityClass::Error,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
            ),
        )
    };
    DiagnosticDisplayCluster::from_members(
        "cluster:test:dup:0001",
        "Exact duplicate test cluster",
        "diagnostic:test:dup:0001",
        DiagnosticClusterMeaningClass::ExactDuplicate,
        "Same source reported the same finding twice.",
        &[
            make("diagnostic:test:dup:0001"),
            make("diagnostic:test:dup:0002"),
        ],
        "Exact-duplicate cluster keeps both records.",
    )
}
