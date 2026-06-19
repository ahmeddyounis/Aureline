//! Conformance dump for the M5 diagnostic source-descriptor and
//! collection-snapshot packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_runtime::diagnostics::{
    DiagnosticEvidencePlaneClass, DiagnosticOriginClass, DiagnosticSource,
    DiagnosticSourceConfidenceClass, DiagnosticSourceKind, DiagnosticSupportClass,
};
use aureline_runtime::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix::{
    DiagnosticCollectionCompletenessClass, M5DiagnosticSurface,
};
use aureline_runtime::diagnostics::DiagnosticFreshnessClass;
use aureline_runtime::m5_diagnostic_source_descriptors_and_collection_snapshots::*;
use aureline_runtime::quality::QualityTargetScopeClass;

const PACKET_ID: &str = "m5-diagnostic-source-and-collection:stable:0001";
const MINTED_AT: &str = "2026-06-19T00:00:00Z";
const WORKSPACE_REF: &str = "workspace:primary";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[allow(clippy::too_many_arguments)]
fn source(
    family: &str,
    kind: DiagnosticSourceKind,
    evidence_plane: DiagnosticEvidencePlaneClass,
    origin: DiagnosticOriginClass,
    confidence: DiagnosticSourceConfidenceClass,
    support: DiagnosticSupportClass,
) -> DiagnosticSource {
    let source_id = format!("source:{family}");
    let mut built = DiagnosticSource::new(
        source_id.clone(),
        kind,
        evidence_plane,
        origin,
        confidence,
        support,
        format!("producer:{family}"),
        format!("tool:{family}"),
        Some(format!("tool-version:{family}:1.0.0")),
        format!("Source descriptor for {family} findings."),
    );
    built.target_or_environment_ref = Some(format!("target:{family}"));
    if origin.is_imported_or_replayed() {
        built.import_ref = Some(format!("import-session:{family}"));
    } else {
        built.originating_session_ref = Some(format!("session:{family}"));
    }
    built
}

fn source_descriptors() -> Vec<DiagnosticSource> {
    vec![
        source(
            "editor_structural",
            DiagnosticSourceKind::EditorStructural,
            DiagnosticEvidencePlaneClass::StaticAnalysis,
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticSourceConfidenceClass::Authoritative,
            DiagnosticSupportClass::Authoritative,
        ),
        source(
            "language_service",
            DiagnosticSourceKind::LanguageService,
            DiagnosticEvidencePlaneClass::StaticAnalysis,
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticSourceConfidenceClass::Authoritative,
            DiagnosticSupportClass::Authoritative,
        ),
        source(
            "build_or_task",
            DiagnosticSourceKind::BuildOrTask,
            DiagnosticEvidencePlaneClass::BuildTimeExecution,
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticSourceConfidenceClass::DerivedStructured,
            DiagnosticSupportClass::Authoritative,
        ),
        source(
            "runtime_or_test",
            DiagnosticSourceKind::RuntimeOrTest,
            DiagnosticEvidencePlaneClass::RuntimeOrTestExecution,
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticSourceConfidenceClass::Authoritative,
            DiagnosticSupportClass::Authoritative,
        ),
        source(
            "scanner_import",
            DiagnosticSourceKind::ScannerImport,
            DiagnosticEvidencePlaneClass::ImportedSnapshotEvidence,
            DiagnosticOriginClass::ImportedSnapshot,
            DiagnosticSourceConfidenceClass::ImportedAuthoritative,
            DiagnosticSupportClass::InspectOnly,
        ),
        source(
            "policy",
            DiagnosticSourceKind::Policy,
            DiagnosticEvidencePlaneClass::PolicyOrTrustEvaluation,
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticSourceConfidenceClass::Authoritative,
            DiagnosticSupportClass::Authoritative,
        ),
        source(
            "heuristic",
            DiagnosticSourceKind::Heuristic,
            DiagnosticEvidencePlaneClass::HeuristicFallback,
            DiagnosticOriginClass::LiveLocalSession,
            DiagnosticSourceConfidenceClass::HeuristicParsed,
            DiagnosticSupportClass::Advisory,
        ),
    ]
}

fn scope(
    scope_class: QualityTargetScopeClass,
    workset: Option<&str>,
    target: Option<&str>,
) -> DiagnosticCollectionScope {
    DiagnosticCollectionScope {
        scope_class,
        workspace_ref: WORKSPACE_REF.to_owned(),
        workset_ref: workset.map(str::to_owned),
        target_or_environment_ref: target.map(str::to_owned),
        active_profile_ref: Some("profile:default".to_owned()),
    }
}

fn omitted(
    scope_ref: &str,
    reason_class: DiagnosticOmittedScopeReasonClass,
    summary: &str,
) -> DiagnosticOmittedScope {
    DiagnosticOmittedScope {
        scope_ref: scope_ref.to_owned(),
        reason_class,
        summary: summary.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn snapshot(
    snapshot_id: &str,
    snapshot_label: &str,
    surface: M5DiagnosticSurface,
    scope: DiagnosticCollectionScope,
    completeness_class: DiagnosticCollectionCompletenessClass,
    freshness_class: DiagnosticFreshnessClass,
    streaming_state: DiagnosticCollectionStreamingState,
    origin_class: DiagnosticOriginClass,
    streaming_cursor: Option<DiagnosticStreamingCursor>,
    omitted_scopes: Vec<DiagnosticOmittedScope>,
    contributing_source_ids: Vec<String>,
    completeness_disclosed: bool,
) -> DiagnosticCollectionSnapshot {
    DiagnosticCollectionSnapshot::new(DiagnosticCollectionSnapshotInput {
        snapshot_id: snapshot_id.to_owned(),
        snapshot_label: snapshot_label.to_owned(),
        surface,
        scope,
        completeness_class,
        freshness_class,
        streaming_state,
        origin_class,
        created_at: MINTED_AT.to_owned(),
        diagnostic_refs: refs(&[
            &format!("diagnostic:{snapshot_id}:0001"),
            &format!("diagnostic:{snapshot_id}:0002"),
        ]),
        streaming_cursor,
        omitted_scopes,
        contributing_source_ids,
        completeness_disclosed,
        imported_not_shown_as_live: true,
        export_safe_summary: format!("Collection snapshot for {snapshot_id}."),
    })
}

fn entry(
    snapshot: DiagnosticCollectionSnapshot,
    claimed: DiagnosticCollectionQualificationClass,
) -> DiagnosticCollectionSnapshotEntry {
    let snapshot_id = snapshot.snapshot_id.clone();
    DiagnosticCollectionSnapshotEntry {
        entry_id: format!("entry:{snapshot_id}"),
        snapshot,
        claimed_qualification: claimed,
        effective_qualification: claimed,
        downgrade_trigger: None,
        degraded_label: None,
        evidence_refs: refs(&[&format!("evidence:{snapshot_id}")]),
        source_contract_refs: refs(&[M5_SOURCE_AND_COLLECTION_DOC_REF]),
    }
}

fn downgraded_data_tooling_entry() -> DiagnosticCollectionSnapshotEntry {
    let snapshot_id = "snapshot:m5:data-tooling:0001";
    let snapshot = snapshot(
        snapshot_id,
        "Data-tooling scan aborted before reaching the rest of the workspace",
        M5DiagnosticSurface::DataToolingDiagnostics,
        scope(QualityTargetScopeClass::Workspace, None, None),
        DiagnosticCollectionCompletenessClass::PartialVisibleScan,
        DiagnosticFreshnessClass::Stale,
        DiagnosticCollectionStreamingState::Aborted,
        DiagnosticOriginClass::LiveLocalSession,
        None,
        vec![omitted(
            "scope:workspace:remaining-datasets",
            DiagnosticOmittedScopeReasonClass::BudgetOrTimeoutCut,
            "The dataset scan aborted on a timeout before reaching the remaining datasets.",
        )],
        refs(&["source:build_or_task"]),
        true,
    );
    let mut downgraded = entry(snapshot, DiagnosticCollectionQualificationClass::Beta);
    downgraded.effective_qualification = DiagnosticCollectionQualificationClass::Held;
    downgraded.downgrade_trigger = Some(DiagnosticCollectionDowngradeTrigger::AbortedCollection);
    downgraded.degraded_label = Some(
        "The dataset scan aborted before completing; held below preview until a full scan can re-establish whole-workspace coverage"
            .to_owned(),
    );
    downgraded
}

fn snapshot_entries() -> Vec<DiagnosticCollectionSnapshotEntry> {
    vec![
        entry(
            snapshot(
                "snapshot:m5:notebook-cell:0001",
                "Notebook cell findings from a complete live local run",
                M5DiagnosticSurface::NotebookCellDiagnostics,
                scope(QualityTargetScopeClass::CurrentRoot, None, None),
                DiagnosticCollectionCompletenessClass::CompleteEnumeration,
                DiagnosticFreshnessClass::Current,
                DiagnosticCollectionStreamingState::Settled,
                DiagnosticOriginClass::LiveLocalSession,
                None,
                Vec::new(),
                refs(&["source:runtime_or_test"]),
                false,
            ),
            DiagnosticCollectionQualificationClass::Beta,
        ),
        entry(
            snapshot(
                "snapshot:m5:framework-pack:0001",
                "Framework-pack analysis still streaming across the workspace",
                M5DiagnosticSurface::FrameworkPackDiagnostics,
                scope(QualityTargetScopeClass::Workspace, None, None),
                DiagnosticCollectionCompletenessClass::PartialVisibleScan,
                DiagnosticFreshnessClass::Current,
                DiagnosticCollectionStreamingState::Streaming,
                DiagnosticOriginClass::LiveLocalSession,
                Some(DiagnosticStreamingCursor {
                    cursor_token: "cursor:framework-pack:0001".to_owned(),
                    emitted_count: 12,
                    has_more: true,
                    resume_hint_ref: Some("resume:framework-pack:0001".to_owned()),
                    summary: "More framework-pack findings are still arriving.".to_owned(),
                }),
                vec![omitted(
                    "scope:workspace:not-yet-scanned-packs",
                    DiagnosticOmittedScopeReasonClass::NotYetScanned,
                    "Framework packs after the current cursor have not been scanned yet.",
                )],
                refs(&["source:language_service"]),
                true,
            ),
            DiagnosticCollectionQualificationClass::Beta,
        ),
        entry(
            snapshot(
                "snapshot:m5:request-tooling:0001",
                "Request/API tooling assertions over the selected workset",
                M5DiagnosticSurface::RequestToolingDiagnostics,
                scope(
                    QualityTargetScopeClass::SelectedWorkset,
                    Some("workset:api-suite"),
                    None,
                ),
                DiagnosticCollectionCompletenessClass::CompleteEnumeration,
                DiagnosticFreshnessClass::Current,
                DiagnosticCollectionStreamingState::Settled,
                DiagnosticOriginClass::LiveLocalSession,
                None,
                Vec::new(),
                refs(&["source:build_or_task"]),
                false,
            ),
            DiagnosticCollectionQualificationClass::Beta,
        ),
        downgraded_data_tooling_entry(),
        entry(
            snapshot(
                "snapshot:m5:preview-runtime:0001",
                "Preview-runtime findings with a suppression-filtered view",
                M5DiagnosticSurface::PreviewRuntimeDiagnostics,
                scope(
                    QualityTargetScopeClass::CurrentRoot,
                    None,
                    Some("target:preview-runtime"),
                ),
                DiagnosticCollectionCompletenessClass::FilteredView,
                DiagnosticFreshnessClass::Current,
                DiagnosticCollectionStreamingState::Settled,
                DiagnosticOriginClass::LiveLocalSession,
                None,
                vec![omitted(
                    "scope:suppressed:known-preview-warnings",
                    DiagnosticOmittedScopeReasonClass::FilteredBySuppression,
                    "Known preview warnings are suppressed by the active profile and withheld from this view.",
                )],
                refs(&["source:build_or_task", "source:heuristic"]),
                true,
            ),
            DiagnosticCollectionQualificationClass::Beta,
        ),
        entry(
            snapshot(
                "snapshot:m5:package-lane:0001",
                "Package-lane policy findings, complete for the workspace",
                M5DiagnosticSurface::PackageLaneDiagnostics,
                scope(QualityTargetScopeClass::Workspace, None, None),
                DiagnosticCollectionCompletenessClass::CompleteEnumeration,
                DiagnosticFreshnessClass::Current,
                DiagnosticCollectionStreamingState::Settled,
                DiagnosticOriginClass::LiveLocalSession,
                None,
                Vec::new(),
                refs(&["source:policy"]),
                false,
            ),
            DiagnosticCollectionQualificationClass::Beta,
        ),
        entry(
            snapshot(
                "snapshot:m5:language-provider:0001",
                "Language-service findings, complete and current",
                M5DiagnosticSurface::LanguageProviderDiagnostics,
                scope(QualityTargetScopeClass::Workspace, None, None),
                DiagnosticCollectionCompletenessClass::CompleteEnumeration,
                DiagnosticFreshnessClass::Current,
                DiagnosticCollectionStreamingState::Settled,
                DiagnosticOriginClass::LiveLocalSession,
                None,
                Vec::new(),
                refs(&["source:language_service"]),
                false,
            ),
            DiagnosticCollectionQualificationClass::Stable,
        ),
        entry(
            snapshot(
                "snapshot:m5:editor-structural:0001",
                "Editor-structural guard, incremental since the last save",
                M5DiagnosticSurface::EditorStructuralDiagnostics,
                scope(QualityTargetScopeClass::CurrentRoot, None, None),
                DiagnosticCollectionCompletenessClass::IncrementalSinceLast,
                DiagnosticFreshnessClass::Recent,
                DiagnosticCollectionStreamingState::Settled,
                DiagnosticOriginClass::LiveLocalSession,
                None,
                Vec::new(),
                refs(&["source:editor_structural"]),
                true,
            ),
            DiagnosticCollectionQualificationClass::Stable,
        ),
        entry(
            snapshot(
                "snapshot:m5:imported-scanner:0001",
                "Imported scanner snapshot held read-only, not live local truth",
                M5DiagnosticSurface::ImportedScannerDiagnostics,
                scope(
                    QualityTargetScopeClass::Workspace,
                    None,
                    Some("target:ci-import"),
                ),
                DiagnosticCollectionCompletenessClass::ImportedSnapshotSet,
                DiagnosticFreshnessClass::ImportedSnapshot,
                DiagnosticCollectionStreamingState::Settled,
                DiagnosticOriginClass::ImportedSnapshot,
                None,
                vec![omitted(
                    "scope:workspace:files-not-in-import",
                    DiagnosticOmittedScopeReasonClass::ExcludedFromSelection,
                    "Files outside the imported CI scan are not represented in this snapshot.",
                )],
                refs(&["source:scanner_import"]),
                true,
            ),
            DiagnosticCollectionQualificationClass::Beta,
        ),
    ]
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

fn source_contract_refs() -> Vec<String> {
    refs(&[
        M5_SOURCE_AND_COLLECTION_SCHEMA_REF,
        M5_SOURCE_DESCRIPTOR_SCHEMA_REF,
        M5_COLLECTION_SNAPSHOT_SCHEMA_REF,
        M5_SOURCE_AND_COLLECTION_DOC_REF,
        M5_SOURCE_AND_COLLECTION_ARTIFACT_REF,
        "schemas/quality/diagnostic-record.schema.json",
        "schemas/quality/m5-diagnostic-truth-lane.schema.json",
    ])
}

fn packet() -> DiagnosticSourceAndCollectionPacket {
    DiagnosticSourceAndCollectionPacket::new(DiagnosticSourceAndCollectionPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "M5 Diagnostic Source Descriptors and Collection Snapshots".to_owned(),
        source_descriptors: source_descriptors(),
        snapshot_entries: snapshot_entries(),
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
