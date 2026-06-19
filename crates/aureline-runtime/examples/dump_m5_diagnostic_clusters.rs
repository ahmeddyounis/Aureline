//! Conformance dump for the M5 diagnostic-cluster set packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::cluster_m5_diagnostics_with_cross_source_dedupe_and_source_preserving_detail_sheets::*;
use aureline_runtime::diagnostics::{
    DiagnosticAnchorRemap, DiagnosticAnchorRemapStateClass, DiagnosticCausalLink,
    DiagnosticCausalLinkKind, DiagnosticEvidencePlaneClass, DiagnosticFreshnessClass,
    DiagnosticOriginClass, DiagnosticRecord, DiagnosticSeverityClass, DiagnosticSource,
    DiagnosticSourceConfidenceClass, DiagnosticSourceKind, DiagnosticSupportClass,
    DiagnosticSurfaceRefs,
};
use aureline_runtime::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix::M5DiagnosticSurface;

const PACKET_ID: &str = "m5-diagnostic-clusters:stable:0001";
const WORKSPACE_ID: &str = "workspace:m5:diagnostic-clusters";
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
fn source(
    diagnostic_id: &str,
    source_kind: DiagnosticSourceKind,
    evidence_plane: DiagnosticEvidencePlaneClass,
    origin: DiagnosticOriginClass,
    confidence: DiagnosticSourceConfidenceClass,
    support: DiagnosticSupportClass,
    tool: &str,
) -> DiagnosticSource {
    let mut built = DiagnosticSource::new(
        format!("source:{diagnostic_id}"),
        source_kind,
        evidence_plane,
        origin,
        confidence,
        support,
        format!("producer:{tool}"),
        format!("tool:{tool}"),
        Some(format!("tool-version:{tool}:1.0.0")),
        format!("Source descriptor for {tool} findings."),
    );
    built.adapter_ref = Some(format!("adapter:{tool}"));
    built.target_or_environment_ref = Some(format!("target:{diagnostic_id}"));
    if origin.is_imported_or_replayed() {
        built.import_ref = Some(format!("import-session:{diagnostic_id}"));
    } else {
        built.originating_session_ref = Some(format!("session:{diagnostic_id}"));
        built.run_ref = Some(format!("run:{diagnostic_id}"));
    }
    built
}

#[allow(clippy::too_many_arguments)]
fn record(
    diagnostic_id: &str,
    family: &str,
    severity: DiagnosticSeverityClass,
    freshness: DiagnosticFreshnessClass,
    remap_state: DiagnosticAnchorRemapStateClass,
    support: DiagnosticSupportClass,
    src: DiagnosticSource,
    suppression_refs: Vec<String>,
    baseline_refs: Vec<String>,
) -> DiagnosticRecord {
    let anchor_remap = DiagnosticAnchorRemap::new(
        format!("remap:{diagnostic_id}"),
        family.to_owned(),
        Some(format!("anchor:{diagnostic_id}:origin")),
        Some(format!("anchor:{diagnostic_id}:current")),
        remap_state,
        format!("evidence:anchor:{diagnostic_id}"),
        MINTED_AT.to_owned(),
        "Append-only anchor remap evidence for the finding.".to_owned(),
    );
    let mut built = DiagnosticRecord::new(
        diagnostic_id.to_owned(),
        format!("rule:{diagnostic_id}"),
        format!("category:{diagnostic_id}"),
        severity,
        src,
        freshness,
        anchor_remap,
        support,
        format!("message:{diagnostic_id}"),
        surface_refs(diagnostic_id),
        MINTED_AT.to_owned(),
        format!("Diagnostic record {diagnostic_id}."),
    );
    built.detail_ref = Some(format!("detail:{diagnostic_id}"));
    built.suppression_refs = suppression_refs;
    built.baseline_refs = baseline_refs;
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

/// Cross-source corroboration: the same underlying issue is reported by a language
/// service, an imported scanner, and a build task. Different sources are clustered
/// for display, but none is flattened into a synthetic finding.
fn cross_source_cluster() -> DiagnosticDisplayCluster {
    let family = "anchor-family:cross-source:0001";
    let language_id = "diagnostic:m5:cross-source:language-service:0001";
    let scanner_id = "diagnostic:m5:cross-source:imported-scanner:0001";
    let build_id = "diagnostic:m5:cross-source:build-task:0001";

    let members = vec![
        member(
            M5DiagnosticSurface::LanguageProviderDiagnostics,
            record(
                language_id,
                family,
                DiagnosticSeverityClass::Error,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
                DiagnosticSupportClass::Authoritative,
                source(
                    language_id,
                    DiagnosticSourceKind::LanguageService,
                    DiagnosticEvidencePlaneClass::StaticAnalysis,
                    DiagnosticOriginClass::LiveLocalSession,
                    DiagnosticSourceConfidenceClass::Authoritative,
                    DiagnosticSupportClass::Authoritative,
                    "language-service",
                ),
                Vec::new(),
                Vec::new(),
            ),
        ),
        member(
            M5DiagnosticSurface::ImportedScannerDiagnostics,
            record(
                scanner_id,
                family,
                DiagnosticSeverityClass::Warning,
                DiagnosticFreshnessClass::ImportedSnapshot,
                DiagnosticAnchorRemapStateClass::ImportedStatic,
                DiagnosticSupportClass::InspectOnly,
                source(
                    scanner_id,
                    DiagnosticSourceKind::ScannerImport,
                    DiagnosticEvidencePlaneClass::ImportedSnapshotEvidence,
                    DiagnosticOriginClass::ImportedSnapshot,
                    DiagnosticSourceConfidenceClass::ImportedAuthoritative,
                    DiagnosticSupportClass::InspectOnly,
                    "imported-scanner",
                ),
                refs(&["suppression:cross-source:imported-scanner:0001"]),
                Vec::new(),
            ),
        ),
        member(
            M5DiagnosticSurface::RequestToolingDiagnostics,
            record(
                build_id,
                family,
                DiagnosticSeverityClass::Warning,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Contextual,
                DiagnosticSupportClass::Authoritative,
                source(
                    build_id,
                    DiagnosticSourceKind::BuildOrTask,
                    DiagnosticEvidencePlaneClass::BuildTimeExecution,
                    DiagnosticOriginClass::LiveLocalSession,
                    DiagnosticSourceConfidenceClass::DerivedStructured,
                    DiagnosticSupportClass::Authoritative,
                    "request-validator",
                ),
                Vec::new(),
                Vec::new(),
            ),
        ),
    ];

    DiagnosticDisplayCluster::from_members(
        "cluster:m5:cross-source:0001",
        "Same issue corroborated by a language service, an imported scanner, and a build task",
        language_id,
        DiagnosticClusterMeaningClass::CrossSourceCorroboration,
        "Three distinct sources flagged the same anchor family; grouped for display while each member keeps its own provenance, freshness, remap state, and imported-versus-live class.",
        &members,
        "Cross-source corroboration cluster preserves three distinct sources without flattening them into one synthetic finding.",
    )
}

/// Exact duplicate: the same source reported the same finding twice (a re-run
/// emitted a second record). Grouped to a single ergonomic row.
fn exact_duplicate_cluster() -> DiagnosticDisplayCluster {
    let family = "anchor-family:exact-duplicate:0001";
    let first_id = "diagnostic:m5:notebook-cell:0001";
    let second_id = "diagnostic:m5:notebook-cell:0002";

    let make = |diagnostic_id: &str| {
        member(
            M5DiagnosticSurface::NotebookCellDiagnostics,
            record(
                diagnostic_id,
                family,
                DiagnosticSeverityClass::Error,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
                DiagnosticSupportClass::Authoritative,
                source(
                    diagnostic_id,
                    DiagnosticSourceKind::RuntimeOrTest,
                    DiagnosticEvidencePlaneClass::RuntimeOrTestExecution,
                    DiagnosticOriginClass::LiveLocalSession,
                    DiagnosticSourceConfidenceClass::Authoritative,
                    DiagnosticSupportClass::Authoritative,
                    "notebook-runner",
                ),
                Vec::new(),
                Vec::new(),
            ),
        )
    };

    DiagnosticDisplayCluster::from_members(
        "cluster:m5:exact-duplicate:0001",
        "Notebook cell error reported twice by the same runner",
        first_id,
        DiagnosticClusterMeaningClass::ExactDuplicate,
        "The same notebook runner emitted the same finding on two runs; grouped to one row while both contributing records stay recoverable.",
        &[make(first_id), make(second_id)],
        "Exact-duplicate cluster groups two same-source records without discarding either.",
    )
}

/// Related by location: an editor-structural guard and a package-lane policy
/// finding share one location.
fn related_by_location_cluster() -> DiagnosticDisplayCluster {
    let family = "anchor-family:related-location:0001";
    let structural_id = "diagnostic:m5:editor-structural:0001";
    let policy_id = "diagnostic:m5:package-lane:0001";

    let members = vec![
        member(
            M5DiagnosticSurface::EditorStructuralDiagnostics,
            record(
                structural_id,
                family,
                DiagnosticSeverityClass::Hint,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
                DiagnosticSupportClass::Authoritative,
                source(
                    structural_id,
                    DiagnosticSourceKind::EditorStructural,
                    DiagnosticEvidencePlaneClass::StaticAnalysis,
                    DiagnosticOriginClass::LiveLocalSession,
                    DiagnosticSourceConfidenceClass::Authoritative,
                    DiagnosticSupportClass::Authoritative,
                    "structural-guard",
                ),
                Vec::new(),
                Vec::new(),
            ),
        ),
        member(
            M5DiagnosticSurface::PackageLaneDiagnostics,
            record(
                policy_id,
                family,
                DiagnosticSeverityClass::Warning,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
                DiagnosticSupportClass::Authoritative,
                source(
                    policy_id,
                    DiagnosticSourceKind::Policy,
                    DiagnosticEvidencePlaneClass::PolicyOrTrustEvaluation,
                    DiagnosticOriginClass::LiveLocalSession,
                    DiagnosticSourceConfidenceClass::Authoritative,
                    DiagnosticSupportClass::Authoritative,
                    "package-policy",
                ),
                Vec::new(),
                refs(&["baseline:package-lane:0001"]),
            ),
        ),
    ];

    DiagnosticDisplayCluster::from_members(
        "cluster:m5:related-location:0001",
        "Editor-structural hint and package-lane policy finding share a location",
        policy_id,
        DiagnosticClusterMeaningClass::RelatedByLocation,
        "Two findings from different sources share one location; grouped for display while each keeps its own source kind and policy state.",
        &members,
        "Related-by-location cluster preserves the structural and policy members' distinct provenance.",
    )
}

/// Related by cause: a preview-runtime render finding and a request-tooling
/// assertion share one causal origin.
fn related_by_cause_cluster() -> DiagnosticDisplayCluster {
    let family = "anchor-family:related-cause:0001";
    let preview_id = "diagnostic:m5:preview-runtime:0001";
    let request_id = "diagnostic:m5:request-tooling:0001";

    let members = vec![
        member(
            M5DiagnosticSurface::PreviewRuntimeDiagnostics,
            record(
                preview_id,
                family,
                DiagnosticSeverityClass::Notice,
                DiagnosticFreshnessClass::Recent,
                DiagnosticAnchorRemapStateClass::Contextual,
                DiagnosticSupportClass::Advisory,
                source(
                    preview_id,
                    DiagnosticSourceKind::BuildOrTask,
                    DiagnosticEvidencePlaneClass::BuildTimeExecution,
                    DiagnosticOriginClass::LiveLocalSession,
                    DiagnosticSourceConfidenceClass::DerivedStructured,
                    DiagnosticSupportClass::Advisory,
                    "preview-renderer",
                ),
                Vec::new(),
                Vec::new(),
            ),
        ),
        member(
            M5DiagnosticSurface::RequestToolingDiagnostics,
            record(
                request_id,
                family,
                DiagnosticSeverityClass::Warning,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
                DiagnosticSupportClass::Authoritative,
                source(
                    request_id,
                    DiagnosticSourceKind::BuildOrTask,
                    DiagnosticEvidencePlaneClass::BuildTimeExecution,
                    DiagnosticOriginClass::LiveLocalSession,
                    DiagnosticSourceConfidenceClass::DerivedStructured,
                    DiagnosticSupportClass::Authoritative,
                    "request-validator",
                ),
                Vec::new(),
                Vec::new(),
            ),
        ),
    ];

    DiagnosticDisplayCluster::from_members(
        "cluster:m5:related-cause:0001",
        "Preview render notice and request-tooling assertion share one cause",
        request_id,
        DiagnosticClusterMeaningClass::RelatedByCause,
        "Both findings trace to one causal origin; grouped for display while each keeps its own freshness and remap state.",
        &members,
        "Related-by-cause cluster preserves both members' distinct freshness and remap state.",
    )
}

fn clusters() -> Vec<DiagnosticDisplayCluster> {
    vec![
        cross_source_cluster(),
        exact_duplicate_cluster(),
        related_by_location_cluster(),
        related_by_cause_cluster(),
    ]
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

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_REF,
        M5_DIAGNOSTIC_CLUSTER_SET_DOC_REF,
        M5_DIAGNOSTIC_CLUSTER_SET_ARTIFACT_REF,
        CANONICAL_DIAGNOSTIC_RECORD_SET_SCHEMA_REF,
        "schemas/quality/m5-diagnostic-truth-lane.schema.json",
        "schemas/quality/diagnostic-source-and-collection.schema.json",
    ])
}

fn packet() -> DiagnosticClusterSetPacket {
    DiagnosticClusterSetPacket::new(DiagnosticClusterSetPacketInput {
        packet_id: PACKET_ID.to_owned(),
        set_label: "M5 Diagnostic-Cluster Set".to_owned(),
        workspace_id: WORKSPACE_ID.to_owned(),
        clusters: clusters(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
