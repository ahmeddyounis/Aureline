use super::*;

const PACKET_ID: &str = "m5-preview-inspection-matrix:stable:0001";

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
        packet_id: PACKET_ID.to_owned(),
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

#[test]
fn preview_inspection_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_claimed_surface_is_present() {
    let surfaces = packet().represented_surfaces();
    for surface in PreviewSurface::ALL {
        assert!(
            surfaces.contains(&surface),
            "missing surface: {}",
            surface.as_str()
        );
    }
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.surface != PreviewSurface::VisualEditTransform);
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::RequiredSurfaceMissing));
}

#[test]
fn auto_narrow_case_is_present() {
    assert_eq!(packet().narrowed_row_count(), 1);
}

#[test]
fn missing_narrowed_case_fails_validation() {
    let mut packet = packet();
    // Identify the mapping-quality class on the support-export row and restore its
    // full claim, leaving no demonstrated auto-narrowing case.
    let export_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == PreviewSurface::SupportExportProjection)
        .expect("support-export row");
    export_row.mapping_quality = Some(SourceMappingQualityClass::Unavailable);
    export_row.effective_qualification = export_row.claimed_qualification;
    export_row.narrow_trigger = None;
    export_row.degraded_label = None;
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::NarrowedRowCaseMissing));
}

#[test]
fn unidentified_dimension_without_narrowing_fails() {
    let mut packet = packet();
    // Drop the target kind on a current row but leave its effective claim untouched.
    let composer_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == PreviewSurface::SourceFirstFrameworkPreview)
        .expect("source-first row");
    composer_row.target_kind = None;
    let violations = packet.validate();
    assert!(violations
        .contains(&PreviewInspectionMatrixViolation::RowNotNarrowedOnUnidentifiedDimension));
    assert!(
        violations.contains(&PreviewInspectionMatrixViolation::NarrowedRowMissingLabelOrTrigger)
    );
}

#[test]
fn runtime_only_masquerading_as_source_fails() {
    let mut packet = packet();
    let runtime_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == PreviewSurface::BrowserRuntimeInspection)
        .expect("browser-runtime row");
    runtime_row.claims_saved_source = true;
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::RuntimeOnlyMasqueradesAsSource));
}

#[test]
fn inspect_only_row_claiming_write_fails() {
    let mut packet = packet();
    let embedded_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == PreviewSurface::EmbeddedWebviewPreview)
        .expect("embedded webview row");
    embedded_row.write_capable = true;
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::InspectOnlyRowClaimsWrite));
}

#[test]
fn write_capable_without_source_diff_preview_fails() {
    let mut packet = packet();
    let edit_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == PreviewSurface::VisualEditTransform)
        .expect("visual-edit row");
    edit_row.previews_source_diff_before_commit = false;
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::WriteCapableRowSkipsSourceDiffPreview));
}

#[test]
fn browser_target_without_attach_depth_fails() {
    let mut packet = packet();
    let runtime_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == PreviewSurface::BrowserRuntimeInspection)
        .expect("browser-runtime row");
    runtime_row.attach_depth_class = AttachDepthClass::NotApplicableNonBrowser;
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::AttachDepthInconsistentWithTarget));
}

#[test]
fn exact_round_trip_without_exact_mapping_fails() {
    let mut packet = packet();
    let edit_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == PreviewSurface::VisualEditTransform)
        .expect("visual-edit row");
    edit_row.mapping_quality = Some(SourceMappingQualityClass::Heuristic);
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::ExactRoundTripWithoutExactMapping));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.rows[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != M5_PREVIEW_INSPECTION_MATRIX_DOC_REF);
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.rows_auto_narrow_on_unidentified_dimension = false;
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .extension_provider_conformance_ingests_matrix = false;
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn incomplete_evidence_freshness_fails() {
    let mut packet = packet();
    packet.evidence_freshness.evidence_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::EvidenceFreshnessIncomplete));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    let export_row = packet
        .rows
        .iter_mut()
        .find(|row| row.surface == PreviewSurface::SupportExportProjection)
        .expect("support-export row");
    export_row.degraded_label = Some("unavailable".to_owned());
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::NarrowedRowMissingLabelOrTrigger));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewInspectionMatrixViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: PreviewInspectionMatrixPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Source-First Preview / Browser-Runtime Inspection Matrix"));
    assert!(summary.contains("source_first_framework_preview"));
    assert!(summary.contains("Degraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_preview_inspection_matrix_export()
        .expect("checked preview inspection matrix export validates");
    assert_eq!(checked, packet());
}
