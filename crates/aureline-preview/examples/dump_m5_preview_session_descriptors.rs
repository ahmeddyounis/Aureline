//! Conformance dump for the M5 preview-session descriptor set packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_preview::preview_session_descriptors::*;
use aureline_preview::{
    DeviceCapabilityClass, PreviewOriginClass, PreviewSessionClass, PreviewTargetClass,
    SourceSyncClass,
};

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:session:{id}")]
}

#[allow(clippy::too_many_arguments)]
fn base(
    session_id: &str,
    surface: PreviewConsumerSurface,
    label: &str,
    session: PreviewSessionClass,
    source_sync: SourceSyncClass,
    data: PreviewDataPostureClass,
    freshness: PreviewFreshnessClass,
    origin: PreviewOriginClass,
    target: PreviewTargetClass,
    capability: DeviceCapabilityClass,
    runtime_backed: bool,
) -> PreviewSessionDescriptor {
    PreviewSessionDescriptor {
        session_id: session_id.to_owned(),
        consumer_surface: surface,
        label_summary: label.to_owned(),
        observed_at: "2026-06-07T00:00:00Z".to_owned(),
        preview_session_class: session,
        source_sync_class: source_sync,
        data_posture: data,
        freshness_class: freshness,
        runtime_origin_class: origin,
        target_kind: target,
        device_capability_class: capability,
        source_revision_ref: Some(format!("source_revision:{session_id}")),
        viewport_pixel_width: None,
        viewport_pixel_height: None,
        freshness_slo_seconds: 60,
        runtime_backed,
        claims_saved_source: false,
        write_capable: false,
        previews_source_diff_before_commit: false,
        downgrade_trigger: None,
        degraded_label: None,
        evidence_refs: ev(session_id),
        source_contract_refs: vec![PREVIEW_SESSION_DESCRIPTOR_SET_DOC_REF.to_owned()],
    }
}

fn sessions() -> Vec<PreviewSessionDescriptor> {
    vec![
        {
            let mut s = base(
                "preview-session:framework-pack:0001",
                PreviewConsumerSurface::FrameworkPackPreview,
                "Framework-pack live preview rendered from the canonical source with an exact round-trip",
                PreviewSessionClass::SourceBoundLivePreview,
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Fresh,
                PreviewOriginClass::LocalDevServer,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                true,
            );
            s.viewport_pixel_width = Some(1280);
            s.viewport_pixel_height = Some(800);
            s.write_capable = true;
            s.previews_source_diff_before_commit = true;
            s
        },
        base(
            "preview-session:preview-route:0001",
            PreviewConsumerSurface::PreviewRoute,
            "Preview-route session showing mock data over a source-bound live preview",
            PreviewSessionClass::SourceBoundLivePreview,
            SourceSyncClass::InSyncFromSource,
            PreviewDataPostureClass::Mock,
            PreviewFreshnessClass::Fresh,
            PreviewOriginClass::LocalDevServer,
            PreviewTargetClass::BrowserTabTarget,
            DeviceCapabilityClass::DesktopDefault,
            true,
        ),
        base(
            "preview-session:notebook-adjacent:0001",
            PreviewConsumerSurface::NotebookAdjacentPreview,
            "Notebook-adjacent captured cell output replayed from a pinned source revision",
            PreviewSessionClass::SnapshotProjection,
            SourceSyncClass::InSyncFromSource,
            PreviewDataPostureClass::Captured,
            PreviewFreshnessClass::Fresh,
            PreviewOriginClass::ImportedOrStaticEvidence,
            PreviewTargetClass::DesignRendererTarget,
            DeviceCapabilityClass::NotApplicable,
            false,
        ),
        {
            let mut s = base(
                "preview-session:preview-route:0002",
                PreviewConsumerSurface::PreviewRoute,
                "Preview-route session whose captured view drifted from source and is past its freshness SLO",
                PreviewSessionClass::SnapshotProjection,
                SourceSyncClass::DriftedFromSource,
                PreviewDataPostureClass::Captured,
                PreviewFreshnessClass::Stale,
                PreviewOriginClass::ImportedOrStaticEvidence,
                PreviewTargetClass::DesignRendererTarget,
                DeviceCapabilityClass::NotApplicable,
                false,
            );
            s.downgrade_trigger = Some(SessionDowngradeTrigger::StaleFreshness);
            s.degraded_label = Some(
                "Captured preview is past its freshness SLO and has drifted from the canonical source; rebuild before relying on it"
                    .to_owned(),
            );
            s
        },
        {
            let mut s = base(
                "preview-session:preview-route:0003",
                PreviewConsumerSurface::PreviewRoute,
                "Preview-route runtime-only inspection of a remote runtime with no saved-source backing",
                PreviewSessionClass::RuntimeBackedInspection,
                SourceSyncClass::RuntimeOnlyNoSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Fresh,
                PreviewOriginClass::RemoteOrContainerRuntime,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                true,
            );
            s.source_revision_ref = None;
            s
        },
        {
            let mut s = base(
                "preview-session:support-export:0001",
                PreviewConsumerSurface::SupportExportProjection,
                "Support/export projection of a session whose data posture is not yet identified",
                PreviewSessionClass::SnapshotProjection,
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Unidentified,
                PreviewFreshnessClass::Fresh,
                PreviewOriginClass::ImportedOrStaticEvidence,
                PreviewTargetClass::ViewportPresetOnly,
                DeviceCapabilityClass::NotApplicable,
                false,
            );
            s.downgrade_trigger = Some(SessionDowngradeTrigger::UnidentifiedDataPosture);
            s.degraded_label = Some(
                "Data posture not yet identified for this projected session; held below a live/mock/captured chip until it is classified"
                    .to_owned(),
            );
            s
        },
    ]
}

fn guardrails() -> SessionGuardrails {
    SessionGuardrails {
        source_canonical_no_second_writable_model: true,
        runtime_state_never_hides_source_mapping_uncertainty: true,
        inspect_only_never_auto_upgraded_to_write: true,
        embedded_boundaries_not_blurred_into_product: true,
        posture_switch_changes_governed_chips_not_bespoke_copy: true,
        stale_or_downgraded_sessions_export_truth: true,
    }
}

fn consumer_projection() -> SessionConsumerProjection {
    SessionConsumerProjection {
        product_ingests_sessions: true,
        docs_help_ingests_sessions: true,
        diagnostics_ingests_sessions: true,
        support_export_ingests_sessions: true,
        release_control_ingests_sessions: true,
        downgraded_sessions_labeled_below_current: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_REF.to_owned(),
        PREVIEW_SESSION_DESCRIPTOR_SET_DOC_REF.to_owned(),
        PREVIEW_SESSION_DESCRIPTOR_SET_ARTIFACT_REF.to_owned(),
        "schemas/preview/preview_target_descriptor.schema.json".to_owned(),
        "schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json".to_owned(),
    ]
}

fn packet() -> PreviewSessionDescriptorSet {
    PreviewSessionDescriptorSet::new(PreviewSessionDescriptorSetInput {
        packet_id: "m5-preview-session-descriptors:stable:0001".to_owned(),
        set_label: "M5 Preview-Session Descriptors".to_owned(),
        sessions: sessions(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
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
