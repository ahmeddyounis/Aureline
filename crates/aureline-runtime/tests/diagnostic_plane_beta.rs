use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use aureline_runtime::diagnostics::{
    DiagnosticAiEvidenceReferencePacket, DiagnosticAnchorRemap, DiagnosticAnchorRemapStateClass,
    DiagnosticCausalLink, DiagnosticEvidencePlaneClass, DiagnosticFreshnessClass,
    DiagnosticPlaneViolation, DiagnosticRecord, DiagnosticRedactionClass, DiagnosticSeverityClass,
    DiagnosticSource, DiagnosticSourceConfidenceClass, DiagnosticSourceKind,
    DiagnosticSupportClass, DiagnosticSupportExport, DiagnosticSurfaceClass, DiagnosticSurfaceRefs,
    UnifiedDiagnosticCluster, UnifiedDiagnosticPlaneSnapshot, UNIFIED_DIAGNOSTIC_SCHEMA_VERSION,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    workspace_id: String,
    snapshot_id: String,
    captured_at: String,
    source_cases: Vec<SourceCase>,
    cluster_groups: Vec<ClusterGroup>,
}

#[derive(Debug, Deserialize)]
struct SourceCase {
    case_id: String,
    diagnostic_id: String,
    rule_id_ref: String,
    category_ref: String,
    severity_class: DiagnosticSeverityClass,
    source_kind: DiagnosticSourceKind,
    evidence_plane_class: DiagnosticEvidencePlaneClass,
    origin_class: aureline_runtime::diagnostics::DiagnosticOriginClass,
    confidence_class: DiagnosticSourceConfidenceClass,
    support_class: DiagnosticSupportClass,
    producer_ref: String,
    tool_ref: String,
    tool_version_ref: Option<String>,
    adapter_ref: Option<String>,
    adapter_version_ref: Option<String>,
    target_or_environment_ref: Option<String>,
    originating_session_ref: Option<String>,
    import_ref: Option<String>,
    run_ref: Option<String>,
    task_ref: Option<String>,
    raw_payload_ref: Option<String>,
    freshness_class: DiagnosticFreshnessClass,
    remap_state_class: DiagnosticAnchorRemapStateClass,
    anchor_family_id: String,
    original_anchor_ref: Option<String>,
    current_anchor_ref: Option<String>,
    evidence_basis_ref: String,
    source_revision_ref: Option<String>,
    current_revision_ref: Option<String>,
    actor_tool_ref: Option<String>,
    message_ref: String,
    detail_ref: Option<String>,
    suppression_refs: Vec<String>,
    baseline_refs: Vec<String>,
    causal_links: Vec<DiagnosticCausalLink>,
    expected_disclosure_labels: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ClusterGroup {
    cluster_id: String,
    primary_diagnostic_id: String,
    dedupe_reason_ref: String,
    contributing_diagnostic_ids: Vec<String>,
    expected_preserved_source_kinds: Vec<DiagnosticSourceKind>,
    expected_preserved_freshness_classes: Vec<DiagnosticFreshnessClass>,
    expected_preserved_remap_states: Vec<DiagnosticAnchorRemapStateClass>,
}

#[test]
fn unified_diagnostic_plane_preserves_truth_across_surfaces() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "unified_diagnostic_plane_fixture");
    assert_eq!(fixture.schema_version, UNIFIED_DIAGNOSTIC_SCHEMA_VERSION);

    let records = fixture
        .source_cases
        .iter()
        .map(|case| build_record(&fixture, case))
        .collect::<Vec<_>>();
    let record_by_id = records
        .iter()
        .map(|record| (record.diagnostic_id.clone(), record.clone()))
        .collect::<BTreeMap<_, _>>();

    let covered_source_kinds = records
        .iter()
        .map(|record| record.source.source_kind)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        covered_source_kinds,
        DiagnosticSourceKind::ALL_BETA_CLAIMED
            .into_iter()
            .collect::<BTreeSet<_>>()
    );
    assert!(records.iter().all(DiagnosticRecord::can_emit_beta_source));

    let clusters = fixture
        .cluster_groups
        .iter()
        .map(|group| {
            let contributing = group
                .contributing_diagnostic_ids
                .iter()
                .map(|diagnostic_id| {
                    record_by_id
                        .get(diagnostic_id)
                        .unwrap_or_else(|| panic!("missing diagnostic {diagnostic_id}"))
                        .clone()
                })
                .collect::<Vec<_>>();
            let cluster = UnifiedDiagnosticCluster::from_records(
                group.cluster_id.clone(),
                group.primary_diagnostic_id.clone(),
                group.dedupe_reason_ref.clone(),
                &contributing,
                "Cluster preserves source, freshness, and remap truth.",
            );
            assert_eq!(
                set(cluster.preserved_source_kinds.clone()),
                set(group.expected_preserved_source_kinds.clone())
            );
            assert_eq!(
                set(cluster.preserved_freshness_classes.clone()),
                set(group.expected_preserved_freshness_classes.clone())
            );
            assert_eq!(
                set(cluster.preserved_remap_states.clone()),
                set(group.expected_preserved_remap_states.clone())
            );
            cluster
        })
        .collect::<Vec<_>>();

    let snapshot = UnifiedDiagnosticPlaneSnapshot::from_records(
        fixture.snapshot_id.clone(),
        fixture.workspace_id.clone(),
        fixture.captured_at.clone(),
        records.clone(),
        clusters,
    );

    let report = snapshot.validate();
    assert!(
        report.is_conformant(),
        "diagnostic plane validation failed: {:?}",
        report.violations
    );

    for record in &records {
        for surface in DiagnosticSurfaceClass::REQUIRED {
            let projection = snapshot
                .surface_projections
                .iter()
                .find(|projection| {
                    projection.diagnostic_id == record.diagnostic_id
                        && projection.surface_class == surface
                })
                .unwrap_or_else(|| {
                    panic!(
                        "missing projection for {} on {:?}",
                        record.diagnostic_id, surface
                    )
                });
            assert_eq!(projection.diagnostic_id, record.diagnostic_id);
            assert_eq!(projection.source_kind, record.source.source_kind);
            assert_eq!(projection.freshness_class, record.freshness_class);
            assert_eq!(
                projection.remap_state_class,
                record.anchor_remap.remap_state_class
            );
            assert!(!projection.raw_source_content_included);
            assert!(!projection.raw_payload_included);
            if surface == DiagnosticSurfaceClass::Problems {
                assert!(
                    projection.open_origin_ref.is_some(),
                    "Problems row stranded {} without an origin ref",
                    record.diagnostic_id
                );
            }
        }
    }

    for case in &fixture.source_cases {
        let projection = snapshot
            .surface_projections
            .iter()
            .find(|projection| {
                projection.diagnostic_id == case.diagnostic_id
                    && projection.surface_class == DiagnosticSurfaceClass::SupportExport
            })
            .unwrap();
        let actual_labels = projection
            .disclosure_labels
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        for expected_label in &case.expected_disclosure_labels {
            assert!(
                actual_labels.contains(expected_label),
                "{} missing disclosure label {expected_label}; labels were {:?}",
                case.case_id,
                actual_labels
            );
        }
    }

    assert_default_export_refs(&snapshot.support_export, &records);
    assert_default_ai_refs(&snapshot.ai_evidence, &records);

    let serialized = serde_json::to_string(&snapshot).expect("snapshot serializes");
    let round_trip: UnifiedDiagnosticPlaneSnapshot =
        serde_json::from_str(&serialized).expect("snapshot deserializes");
    assert_eq!(round_trip, snapshot);

    assert_schema_ids();
}

#[test]
fn validation_rejects_display_text_only_diagnostics() {
    let fixture = load_fixture();
    let mut records = fixture
        .source_cases
        .iter()
        .map(|case| build_record(&fixture, case))
        .collect::<Vec<_>>();
    records[0].source.tool_version_ref = None;
    records[0].causal_links.clear();
    let snapshot = UnifiedDiagnosticPlaneSnapshot::from_records(
        fixture.snapshot_id,
        fixture.workspace_id,
        fixture.captured_at,
        records,
        Vec::new(),
    );
    let report = snapshot.validate();
    assert!(report.violations.iter().any(|violation| matches!(
        violation,
        DiagnosticPlaneViolation::DisplayTextOnlyDiagnostic { diagnostic_id }
            if diagnostic_id == "diag:editor:unicode-bidi:config"
    )));
}

fn build_record(fixture: &Fixture, case: &SourceCase) -> DiagnosticRecord {
    let mut source = DiagnosticSource::new(
        format!("source:{}", case.case_id),
        case.source_kind,
        case.evidence_plane_class,
        case.origin_class,
        case.confidence_class,
        case.support_class,
        case.producer_ref.clone(),
        case.tool_ref.clone(),
        case.tool_version_ref.clone(),
        format!("{} source keeps provenance metadata.", case.case_id),
    );
    source.adapter_ref = case.adapter_ref.clone();
    source.adapter_version_ref = case.adapter_version_ref.clone();
    source.target_or_environment_ref = case.target_or_environment_ref.clone();
    source.originating_session_ref = case.originating_session_ref.clone();
    source.import_ref = case.import_ref.clone();
    source.run_ref = case.run_ref.clone();
    source.task_ref = case.task_ref.clone();
    source.raw_payload_ref = case.raw_payload_ref.clone();

    let mut anchor_remap = DiagnosticAnchorRemap::new(
        format!("remap:{}", case.case_id),
        case.anchor_family_id.clone(),
        case.original_anchor_ref.clone(),
        case.current_anchor_ref.clone(),
        case.remap_state_class,
        case.evidence_basis_ref.clone(),
        fixture.captured_at.clone(),
        format!("{} remap state is preserved.", case.case_id),
    );
    anchor_remap.source_revision_ref = case.source_revision_ref.clone();
    anchor_remap.current_revision_ref = case.current_revision_ref.clone();
    anchor_remap.actor_tool_ref = case.actor_tool_ref.clone();

    let mut record = DiagnosticRecord::new(
        case.diagnostic_id.clone(),
        case.rule_id_ref.clone(),
        case.category_ref.clone(),
        case.severity_class,
        source,
        case.freshness_class,
        anchor_remap,
        case.support_class,
        case.message_ref.clone(),
        surface_refs(&case.diagnostic_id),
        fixture.captured_at.clone(),
        format!(
            "{} normalized diagnostic preserves id, source, freshness, remap, and lineage.",
            case.case_id
        ),
    );
    record.detail_ref = case.detail_ref.clone();
    record.suppression_refs = case.suppression_refs.clone();
    record.baseline_refs = case.baseline_refs.clone();
    record.causal_links = case.causal_links.clone();
    record.redaction_class = DiagnosticRedactionClass::MetadataSafeDefault;
    record
}

fn surface_refs(diagnostic_id: &str) -> DiagnosticSurfaceRefs {
    let token = diagnostic_id.replace(':', "-");
    DiagnosticSurfaceRefs {
        editor_decoration_ref: format!("editor-decoration:{token}"),
        problems_row_ref: format!("problems-row:{token}"),
        output_entry_ref: format!("output-entry:{token}"),
        timeline_entry_ref: format!("timeline-entry:{token}"),
        rerun_action_ref: format!("rerun-action:{token}"),
        review_packet_ref: format!("review-packet:{token}"),
        cli_explain_ref: format!("cli-explain:{token}"),
        ai_evidence_ref: format!("ai-evidence:{token}"),
        support_export_ref: format!("support-export:{token}"),
    }
}

fn assert_default_export_refs(export: &DiagnosticSupportExport, records: &[DiagnosticRecord]) {
    assert!(!export.raw_source_content_included);
    assert!(!export.raw_payload_included);
    assert_eq!(
        set(export.diagnostic_record_refs.clone()),
        records
            .iter()
            .map(|record| record.diagnostic_id.clone())
            .collect::<BTreeSet<_>>()
    );
}

fn assert_default_ai_refs(
    packet: &DiagnosticAiEvidenceReferencePacket,
    records: &[DiagnosticRecord],
) {
    assert!(!packet.raw_source_content_included);
    assert!(!packet.raw_payload_included);
    assert_eq!(
        set(packet.diagnostic_record_refs.clone()),
        records
            .iter()
            .map(|record| record.diagnostic_id.clone())
            .collect::<BTreeSet<_>>()
    );
}

fn assert_schema_ids() {
    let schema_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas/diagnostics");
    for (file_name, schema_id) in [
        (
            "diagnostic_record.schema.json",
            "https://aureline.dev/schemas/diagnostics/diagnostic_record.schema.json",
        ),
        (
            "diagnostic_source.schema.json",
            "https://aureline.dev/schemas/diagnostics/diagnostic_source.schema.json",
        ),
        (
            "diagnostic_anchor_remap.schema.json",
            "https://aureline.dev/schemas/diagnostics/diagnostic_anchor_remap.schema.json",
        ),
    ] {
        let path = schema_root.join(file_name);
        let payload =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
        let schema: serde_json::Value =
            serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"));
        assert_eq!(schema["$id"], schema_id);
    }
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/diagnostics/unified_diagnostic_plane/source_matrix.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn set<T: Ord>(values: Vec<T>) -> BTreeSet<T> {
    values.into_iter().collect()
}
