//! Conformance dump for the M5 source-first preview / browser-runtime inspection
//! matrix packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_preview::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix::*;
use aureline_preview::{PreviewTargetClass, SourceMappingQualityClass};

fn ev(refs: &[&str]) -> Vec<String> {
    refs.iter().map(|r| (*r).to_owned()).collect()
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    surface: PreviewSurface,
    label: &str,
    session: PreviewSessionClass,
    source_sync: SourceSyncClass,
    target: Option<PreviewTargetClass>,
    mapping: Option<SourceMappingQualityClass>,
    attach: AttachDepthClass,
    round_trip: RoundTripCapabilityClass,
    claimed: PreviewMatrixQualificationClass,
    runtime_backed: bool,
    write_capable: bool,
) -> PreviewInspectionRow {
    PreviewInspectionRow {
        row_id: row_id.to_owned(),
        surface,
        label_summary: label.to_owned(),
        preview_session_class: session,
        source_sync_class: source_sync,
        target_kind: target,
        mapping_quality: mapping,
        attach_depth_class: attach,
        round_trip_capability: round_trip,
        claimed_qualification: claimed,
        effective_qualification: claimed,
        runtime_backed,
        claims_saved_source: false,
        write_capable,
        previews_source_diff_before_commit: write_capable,
        narrow_trigger: None,
        degraded_label: None,
        evidence_refs: ev(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: vec![M5_PREVIEW_INSPECTION_MATRIX_DOC_REF.to_owned()],
    }
}

fn narrowed_support_export_row() -> PreviewInspectionRow {
    let mut export_row = row(
        "preview-row:support-export:0001",
        PreviewSurface::SupportExportProjection,
        "Support/export projection of a preview row whose mapping-quality class is not yet identified",
        PreviewSessionClass::SnapshotProjection,
        SourceSyncClass::DriftedFromSource,
        Some(PreviewTargetClass::ViewportPresetOnly),
        None,
        AttachDepthClass::NotApplicableNonBrowser,
        RoundTripCapabilityClass::NoRoundTrip,
        PreviewMatrixQualificationClass::Beta,
        false,
        false,
    );
    export_row.effective_qualification = PreviewMatrixQualificationClass::Held;
    export_row.narrow_trigger = Some(PreviewMatrixDowngradeTrigger::UnidentifiedMappingQuality);
    export_row.degraded_label = Some(
        "Mapping-quality class not yet identified for this projected row; held below preview until a source map is published"
            .to_owned(),
    );
    export_row
}

fn rows() -> Vec<PreviewInspectionRow> {
    vec![
        row(
            "preview-row:source-first-framework:0001",
            PreviewSurface::SourceFirstFrameworkPreview,
            "Source-first framework preview rendered from the canonical source by the design renderer",
            PreviewSessionClass::SourceBoundLivePreview,
            SourceSyncClass::InSyncFromSource,
            Some(PreviewTargetClass::DesignRendererTarget),
            Some(SourceMappingQualityClass::Exact),
            AttachDepthClass::NotApplicableNonBrowser,
            RoundTripCapabilityClass::ExactSourceRoundTrip,
            PreviewMatrixQualificationClass::Stable,
            false,
            true,
        ),
        row(
            "preview-row:visual-surface-mapping:0001",
            PreviewSurface::VisualSurfaceMapping,
            "Component/DOM/widget mapping inspector over a source-bound design render",
            PreviewSessionClass::SourceBoundLivePreview,
            SourceSyncClass::InSyncFromSource,
            Some(PreviewTargetClass::DesignRendererTarget),
            Some(SourceMappingQualityClass::Heuristic),
            AttachDepthClass::NotApplicableNonBrowser,
            RoundTripCapabilityClass::ApproximateSourceRoundTrip,
            PreviewMatrixQualificationClass::Beta,
            false,
            true,
        ),
        {
            let mut runtime_row = row(
                "preview-row:browser-runtime-inspection:0001",
                PreviewSurface::BrowserRuntimeInspection,
                "Browser-runtime DOM/CSS/network/storage inspection over an attached local browser tab",
                PreviewSessionClass::RuntimeBackedInspection,
                SourceSyncClass::RuntimeOnlyNoSource,
                Some(PreviewTargetClass::BrowserTabTarget),
                Some(SourceMappingQualityClass::Partial),
                AttachDepthClass::DomStylesNetworkStorage,
                RoundTripCapabilityClass::InspectOnlyNoWrite,
                PreviewMatrixQualificationClass::Beta,
                true,
                false,
            );
            runtime_row.previews_source_diff_before_commit = false;
            runtime_row
        },
        row(
            "preview-row:device-or-simulator:0001",
            PreviewSurface::DeviceOrSimulatorPreview,
            "Simulator preview tethered over a workspace-bound transport with an exact source map",
            PreviewSessionClass::DeviceTetheredSession,
            SourceSyncClass::InSyncFromSource,
            Some(PreviewTargetClass::SimulatorTarget),
            Some(SourceMappingQualityClass::Exact),
            AttachDepthClass::NotApplicableNonBrowser,
            RoundTripCapabilityClass::ExactSourceRoundTrip,
            PreviewMatrixQualificationClass::Beta,
            true,
            true,
        ),
        {
            let mut full_stack = row(
                "preview-row:full-stack-loop:0001",
                PreviewSurface::FullStackPreviewLoop,
                "Full-stack preview loop where edits fall back to editing the source directly",
                PreviewSessionClass::SourceBoundLivePreview,
                SourceSyncClass::PendingRebuild,
                Some(PreviewTargetClass::BrowserTabTarget),
                Some(SourceMappingQualityClass::Stale),
                AttachDepthClass::DomStylesNetwork,
                RoundTripCapabilityClass::SourceOnlyFallback,
                PreviewMatrixQualificationClass::Beta,
                true,
                false,
            );
            full_stack.previews_source_diff_before_commit = false;
            full_stack
        },
        {
            let mut embedded = row(
                "preview-row:embedded-webview:0001",
                PreviewSurface::EmbeddedWebviewPreview,
                "Embedded webview preview hosted in the shell with inspect-only DOM/CSS depth",
                PreviewSessionClass::EmbeddedRendererSession,
                SourceSyncClass::InSyncFromSource,
                Some(PreviewTargetClass::EmbeddedWebviewTarget),
                Some(SourceMappingQualityClass::Heuristic),
                AttachDepthClass::DomAndStyles,
                RoundTripCapabilityClass::InspectOnlyNoWrite,
                PreviewMatrixQualificationClass::Preview,
                false,
                false,
            );
            embedded.previews_source_diff_before_commit = false;
            embedded
        },
        row(
            "preview-row:visual-edit-transform:0001",
            PreviewSurface::VisualEditTransform,
            "Visual-edit transform that previews the real source diff before committing an exact round-trip",
            PreviewSessionClass::SourceBoundLivePreview,
            SourceSyncClass::InSyncFromSource,
            Some(PreviewTargetClass::DesignRendererTarget),
            Some(SourceMappingQualityClass::Exact),
            AttachDepthClass::NotApplicableNonBrowser,
            RoundTripCapabilityClass::ExactSourceRoundTrip,
            PreviewMatrixQualificationClass::Beta,
            false,
            true,
        ),
        narrowed_support_export_row(),
    ]
}

fn guardrails() -> MatrixGuardrails {
    MatrixGuardrails {
        source_canonical_preview_derivative: true,
        runtime_inspection_never_masquerades_as_source: true,
        mapping_uncertainty_never_hidden: true,
        inspect_only_never_auto_upgraded_to_write: true,
        visual_edits_preview_source_diff_before_commit: true,
        embedded_boundaries_not_blurred_into_product: true,
        rows_auto_narrow_on_unidentified_dimension: true,
    }
}

fn consumer_projection() -> MatrixConsumerProjection {
    MatrixConsumerProjection {
        product_ingests_matrix: true,
        docs_help_ingests_matrix: true,
        diagnostics_ingests_matrix: true,
        extension_provider_conformance_ingests_matrix: true,
        release_control_ingests_matrix: true,
        narrowed_rows_labeled_below_current: true,
    }
}

fn evidence_freshness() -> MatrixEvidenceFreshness {
    MatrixEvidenceFreshness {
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_REF.to_owned(),
        M5_PREVIEW_INSPECTION_MATRIX_DOC_REF.to_owned(),
        M5_PREVIEW_INSPECTION_MATRIX_ARTIFACT_REF.to_owned(),
        "schemas/preview/preview_target_descriptor.schema.json".to_owned(),
        "schemas/preview/hot_reload_state.schema.json".to_owned(),
        "schemas/browser_runtime/session_origin.schema.json".to_owned(),
    ]
}

fn packet() -> PreviewInspectionMatrixPacket {
    PreviewInspectionMatrixPacket::new(PreviewInspectionMatrixPacketInput {
        packet_id: "m5-preview-inspection-matrix:stable:0001".to_owned(),
        matrix_label: "M5 Source-First Preview / Browser-Runtime Inspection Matrix".to_owned(),
        rows: rows(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        evidence_freshness: evidence_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
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
