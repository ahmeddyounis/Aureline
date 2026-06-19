//! Conformance dump for the M5 normalized diagnostic-record set packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::diagnostics::{
    DiagnosticAnchorRemap, DiagnosticAnchorRemapStateClass, DiagnosticCausalLink,
    DiagnosticCausalLinkKind, DiagnosticEvidencePlaneClass, DiagnosticFreshnessClass,
    DiagnosticOriginClass, DiagnosticRecord, DiagnosticSeverityClass, DiagnosticSource,
    DiagnosticSourceConfidenceClass, DiagnosticSourceKind, DiagnosticSupportClass,
    DiagnosticSurfaceClass, DiagnosticSurfaceRefs,
};
use aureline_runtime::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix::M5DiagnosticSurface;
use aureline_runtime::normalize_m5_diagnostic_records_with_stable_ids_and_suppression_baseline_joins::*;
use aureline_runtime::quality::{
    BaselineCompatibilityStateClass, QualityDebtReopenStateClass, QualityTargetScopeClass,
};

const PACKET_ID: &str = "m5-normalized-diagnostic-records:stable:0001";
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
        format!("Normalized source descriptor for {tool} findings."),
    );
    built.target_or_environment_ref = Some(format!("target:{diagnostic_id}"));
    if origin.is_imported_or_replayed() {
        built.import_ref = Some(format!("import-session:{diagnostic_id}"));
    } else {
        built.originating_session_ref = Some(format!("session:{diagnostic_id}"));
    }
    built
}

fn anchor_remap(
    diagnostic_id: &str,
    family: &str,
    state: DiagnosticAnchorRemapStateClass,
) -> DiagnosticAnchorRemap {
    DiagnosticAnchorRemap::new(
        format!("remap:{diagnostic_id}"),
        family.to_owned(),
        Some(format!("anchor:{diagnostic_id}:origin")),
        Some(format!("anchor:{diagnostic_id}:current")),
        state,
        format!("evidence:anchor:{diagnostic_id}"),
        MINTED_AT.to_owned(),
        "Append-only anchor remap evidence for the finding.".to_owned(),
    )
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
    let mut built = DiagnosticRecord::new(
        diagnostic_id.to_owned(),
        format!("rule:{diagnostic_id}"),
        format!("category:{diagnostic_id}"),
        severity,
        src,
        freshness,
        anchor_remap(diagnostic_id, family, remap_state),
        support,
        format!("message:{diagnostic_id}"),
        surface_refs(diagnostic_id),
        MINTED_AT.to_owned(),
        format!("Normalized diagnostic record {diagnostic_id}."),
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

fn identity_family(diagnostic_id: &str, family: &str) -> DiagnosticStableIdentityFamily {
    let observe =
        |context: DiagnosticIdentityContextClass, note: &str| DiagnosticIdentityObservation {
            context_class: context,
            observed_diagnostic_id: diagnostic_id.to_owned(),
            observed_anchor_family_id: family.to_owned(),
            note: note.to_owned(),
        };
    DiagnosticStableIdentityFamily {
        diagnostic_id: diagnostic_id.to_owned(),
        anchor_family_id: family.to_owned(),
        observations: vec![
            observe(
                DiagnosticIdentityContextClass::InitialEmit,
                "Finding first emitted with this canonical id.",
            ),
            observe(
                DiagnosticIdentityContextClass::OrdinaryRefresh,
                "Re-analysis kept the same id within the anchor family.",
            ),
            observe(
                DiagnosticIdentityContextClass::AdapterRefresh,
                "Adapter refresh did not reissue the id.",
            ),
            observe(
                DiagnosticIdentityContextClass::SurfaceHop,
                "Reopened from another surface and resolved to the same id.",
            ),
            observe(
                DiagnosticIdentityContextClass::PresentationChange,
                "Clustering and density change kept the same id.",
            ),
            observe(
                DiagnosticIdentityContextClass::ReExport,
                "Re-export into a support bundle kept the same id.",
            ),
        ],
    }
}

fn reopen_handle(
    diagnostic_id: &str,
    surface_class: DiagnosticSurfaceClass,
) -> DiagnosticReopenHandle {
    DiagnosticReopenHandle {
        surface_class,
        stable_surface_ref: format!("{}:{diagnostic_id}", surface_class.as_str()),
        resolves_diagnostic_id: diagnostic_id.to_owned(),
        cites_canonical_id: true,
        preserves_detail: true,
    }
}

fn reopen_handles(diagnostic_id: &str) -> Vec<DiagnosticReopenHandle> {
    REQUIRED_REOPEN_SURFACES
        .iter()
        .map(|surface_class| reopen_handle(diagnostic_id, *surface_class))
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn entry(
    surface: M5DiagnosticSurface,
    diagnostic_id: &str,
    family: &str,
    label: &str,
    record: DiagnosticRecord,
    reopen_handles: Vec<DiagnosticReopenHandle>,
    suppression_joins: Vec<DiagnosticSuppressionJoin>,
    baseline_joins: Vec<DiagnosticBaselineJoin>,
    claimed: NormalizedRecordQualificationClass,
) -> NormalizedDiagnosticRecordEntry {
    NormalizedDiagnosticRecordEntry {
        entry_id: format!("entry:{diagnostic_id}"),
        surface,
        label_summary: label.to_owned(),
        record,
        identity_family: identity_family(diagnostic_id, family),
        reopen_handles,
        suppression_joins,
        baseline_joins,
        claimed_qualification: claimed,
        effective_qualification: claimed,
        downgrade_trigger: None,
        degraded_label: None,
        evidence_refs: refs(&[&format!("evidence:{diagnostic_id}")]),
        source_contract_refs: refs(&[M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_DOC_REF]),
    }
}

fn downgraded_data_tooling_entry() -> NormalizedDiagnosticRecordEntry {
    let diagnostic_id = "diagnostic:m5:data-tooling:0001";
    let family = "anchor-family:data-tooling:0001";
    let record = record(
        diagnostic_id,
        family,
        DiagnosticSeverityClass::Warning,
        DiagnosticFreshnessClass::Recent,
        DiagnosticAnchorRemapStateClass::Exact,
        DiagnosticSupportClass::Authoritative,
        source(
            diagnostic_id,
            DiagnosticSourceKind::BuildOrTask,
            DiagnosticEvidencePlaneClass::BuildTimeExecution,
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticSourceConfidenceClass::DerivedStructured,
            DiagnosticSupportClass::Authoritative,
            "query-validator",
        ),
        Vec::new(),
        Vec::new(),
    );
    // Drop the AI-evidence reopen handle so this record cannot be reopened from
    // every required consumer surface and must auto-downgrade.
    let handles = REQUIRED_REOPEN_SURFACES
        .iter()
        .filter(|surface_class| **surface_class != DiagnosticSurfaceClass::AiEvidence)
        .map(|surface_class| reopen_handle(diagnostic_id, *surface_class))
        .collect();
    let mut downgraded = entry(
        M5DiagnosticSurface::DataToolingDiagnostics,
        diagnostic_id,
        family,
        "Data-tooling findings whose AI-evidence reopen handle is not yet published",
        record,
        handles,
        Vec::new(),
        Vec::new(),
        NormalizedRecordQualificationClass::Beta,
    );
    downgraded.effective_qualification = NormalizedRecordQualificationClass::Held;
    downgraded.downgrade_trigger = Some(NormalizedRecordDowngradeTrigger::MissingReopenSurface);
    downgraded.degraded_label = Some(
        "No AI-evidence reopen handle yet resolves this record; held below preview until the AI-evidence surface can reopen the same canonical diagnostic id"
            .to_owned(),
    );
    downgraded
}

fn entries() -> Vec<NormalizedDiagnosticRecordEntry> {
    let framework_id = "diagnostic:m5:framework-pack:0001";
    let framework_family = "anchor-family:framework-pack:0001";
    let framework_baseline = "baseline:framework-pack:0001";
    let scanner_id = "diagnostic:m5:imported-scanner:0001";
    let scanner_family = "anchor-family:imported-scanner:0001";
    let scanner_suppression = "suppression:imported-scanner:0001";

    vec![
        entry(
            M5DiagnosticSurface::NotebookCellDiagnostics,
            "diagnostic:m5:notebook-cell:0001",
            "anchor-family:notebook-cell:0001",
            "Notebook cell diagnostic from a live local run",
            record(
                "diagnostic:m5:notebook-cell:0001",
                "anchor-family:notebook-cell:0001",
                DiagnosticSeverityClass::Error,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
                DiagnosticSupportClass::Authoritative,
                source(
                    "diagnostic:m5:notebook-cell:0001",
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
            reopen_handles("diagnostic:m5:notebook-cell:0001"),
            Vec::new(),
            Vec::new(),
            NormalizedRecordQualificationClass::Beta,
        ),
        entry(
            M5DiagnosticSurface::FrameworkPackDiagnostics,
            framework_id,
            framework_family,
            "Framework-pack analyzer finding accepted into a compatible baseline",
            record(
                framework_id,
                framework_family,
                DiagnosticSeverityClass::Warning,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
                DiagnosticSupportClass::Authoritative,
                source(
                    framework_id,
                    DiagnosticSourceKind::LanguageService,
                    DiagnosticEvidencePlaneClass::StaticAnalysis,
                    DiagnosticOriginClass::LiveLocalSession,
                    DiagnosticSourceConfidenceClass::Authoritative,
                    DiagnosticSupportClass::Authoritative,
                    "framework-analyzer",
                ),
                Vec::new(),
                refs(&[framework_baseline]),
            ),
            reopen_handles(framework_id),
            Vec::new(),
            vec![DiagnosticBaselineJoin {
                join_id: format!("baseline-join:{framework_id}"),
                diagnostic_id: framework_id.to_owned(),
                baseline_id: framework_baseline.to_owned(),
                compatibility_state_class: BaselineCompatibilityStateClass::Compatible,
                accepted_in_baseline: true,
                attached_to_record: true,
                summary: "Finding accepted into the compatible framework baseline.".to_owned(),
            }],
            NormalizedRecordQualificationClass::Beta,
        ),
        entry(
            M5DiagnosticSurface::RequestToolingDiagnostics,
            "diagnostic:m5:request-tooling:0001",
            "anchor-family:request-tooling:0001",
            "Request/API tooling assertion whose fix previews before apply",
            record(
                "diagnostic:m5:request-tooling:0001",
                "anchor-family:request-tooling:0001",
                DiagnosticSeverityClass::Warning,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
                DiagnosticSupportClass::Authoritative,
                source(
                    "diagnostic:m5:request-tooling:0001",
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
            reopen_handles("diagnostic:m5:request-tooling:0001"),
            Vec::new(),
            Vec::new(),
            NormalizedRecordQualificationClass::Beta,
        ),
        downgraded_data_tooling_entry(),
        entry(
            M5DiagnosticSurface::PreviewRuntimeDiagnostics,
            "diagnostic:m5:preview-runtime:0001",
            "anchor-family:preview-runtime:0001",
            "Preview-runtime render finding whose range is contextually remapped",
            record(
                "diagnostic:m5:preview-runtime:0001",
                "anchor-family:preview-runtime:0001",
                DiagnosticSeverityClass::Notice,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Contextual,
                DiagnosticSupportClass::Advisory,
                source(
                    "diagnostic:m5:preview-runtime:0001",
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
            reopen_handles("diagnostic:m5:preview-runtime:0001"),
            Vec::new(),
            Vec::new(),
            NormalizedRecordQualificationClass::Beta,
        ),
        entry(
            M5DiagnosticSurface::PackageLaneDiagnostics,
            "diagnostic:m5:package-lane:0001",
            "anchor-family:package-lane:0001",
            "Package-lane policy finding with an unmapped, non-line-anchored range",
            record(
                "diagnostic:m5:package-lane:0001",
                "anchor-family:package-lane:0001",
                DiagnosticSeverityClass::Warning,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Unmapped,
                DiagnosticSupportClass::Authoritative,
                source(
                    "diagnostic:m5:package-lane:0001",
                    DiagnosticSourceKind::Policy,
                    DiagnosticEvidencePlaneClass::PolicyOrTrustEvaluation,
                    DiagnosticOriginClass::LiveLocalSession,
                    DiagnosticSourceConfidenceClass::Authoritative,
                    DiagnosticSupportClass::Authoritative,
                    "package-policy",
                ),
                Vec::new(),
                Vec::new(),
            ),
            reopen_handles("diagnostic:m5:package-lane:0001"),
            Vec::new(),
            Vec::new(),
            NormalizedRecordQualificationClass::Beta,
        ),
        entry(
            M5DiagnosticSurface::LanguageProviderDiagnostics,
            "diagnostic:m5:language-provider:0001",
            "anchor-family:language-provider:0001",
            "Language-service finding with exact anchors and stable identity",
            record(
                "diagnostic:m5:language-provider:0001",
                "anchor-family:language-provider:0001",
                DiagnosticSeverityClass::Error,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
                DiagnosticSupportClass::Authoritative,
                source(
                    "diagnostic:m5:language-provider:0001",
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
            reopen_handles("diagnostic:m5:language-provider:0001"),
            Vec::new(),
            Vec::new(),
            NormalizedRecordQualificationClass::Stable,
        ),
        entry(
            M5DiagnosticSurface::EditorStructuralDiagnostics,
            "diagnostic:m5:editor-structural:0001",
            "anchor-family:editor-structural:0001",
            "Editor-structural guard with a complete local enumeration",
            record(
                "diagnostic:m5:editor-structural:0001",
                "anchor-family:editor-structural:0001",
                DiagnosticSeverityClass::Hint,
                DiagnosticFreshnessClass::Current,
                DiagnosticAnchorRemapStateClass::Exact,
                DiagnosticSupportClass::Authoritative,
                source(
                    "diagnostic:m5:editor-structural:0001",
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
            reopen_handles("diagnostic:m5:editor-structural:0001"),
            Vec::new(),
            Vec::new(),
            NormalizedRecordQualificationClass::Stable,
        ),
        entry(
            M5DiagnosticSurface::ImportedScannerDiagnostics,
            scanner_id,
            scanner_family,
            "Imported scanner finding held read-only and suppressed with a governed join",
            record(
                scanner_id,
                scanner_family,
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
                refs(&[scanner_suppression]),
                Vec::new(),
            ),
            reopen_handles(scanner_id),
            vec![DiagnosticSuppressionJoin {
                join_id: format!("suppression-join:{scanner_id}"),
                diagnostic_id: scanner_id.to_owned(),
                suppression_id: scanner_suppression.to_owned(),
                scope_class: QualityTargetScopeClass::BaselineFamily,
                reopen_state_class: QualityDebtReopenStateClass::Active,
                release_visible: true,
                attached_to_record: true,
                summary: "Imported finding suppressed under a governed, release-visible record."
                    .to_owned(),
            }],
            Vec::new(),
            NormalizedRecordQualificationClass::Beta,
        ),
    ]
}

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

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_SCHEMA_REF,
        M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_DOC_REF,
        M5_NORMALIZED_DIAGNOSTIC_RECORD_SET_ARTIFACT_REF,
        CANONICAL_DIAGNOSTIC_RECORD_SCHEMA_REF,
        "schemas/quality/suppression_record.schema.json",
        "schemas/quality/m5-diagnostic-truth-lane.schema.json",
    ])
}

fn packet() -> NormalizedDiagnosticRecordSetPacket {
    NormalizedDiagnosticRecordSetPacket::new(NormalizedDiagnosticRecordSetPacketInput {
        packet_id: PACKET_ID.to_owned(),
        set_label: "M5 Normalized Diagnostic-Record Set".to_owned(),
        entries: entries(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        evidence_freshness: evidence_freshness(),
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
