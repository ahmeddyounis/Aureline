use super::*;

use crate::preview_origin::{DeviceCapabilityClass, PreviewOriginClass, PreviewTargetClass};

const PACKET_ID: &str = "m5-preview-session-descriptors:stable:0001";

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
        packet_id: PACKET_ID.to_owned(),
        set_label: "M5 Preview-Session Descriptors".to_owned(),
        sessions: sessions(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn first_route_session(packet: &mut PreviewSessionDescriptorSet) -> &mut PreviewSessionDescriptor {
    packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "preview-session:preview-route:0001")
        .expect("preview-route mock session")
}

#[test]
fn session_set_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_first_real_surface_is_present() {
    let surfaces = packet().represented_surfaces();
    for surface in PreviewConsumerSurface::FIRST_REAL {
        assert!(
            surfaces.contains(&surface),
            "missing surface: {}",
            surface.as_str()
        );
    }
}

#[test]
fn missing_first_real_surface_fails() {
    let mut packet = packet();
    packet
        .sessions
        .retain(|s| s.consumer_surface != PreviewConsumerSurface::NotebookAdjacentPreview);
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::RequiredConsumerSurfaceMissing));
}

#[test]
fn posture_switch_is_demonstrated() {
    assert!(packet().demonstrates_posture_switch());
}

#[test]
fn missing_posture_switch_fails() {
    let mut packet = packet();
    // Collapse every posture to live, leaving no live-vs-mock-or-captured switch.
    for session in &mut packet.sessions {
        session.data_posture = PreviewDataPostureClass::Live;
        session.downgrade_trigger = None;
        session.degraded_label = None;
        // Live posture must be runtime-backed by a live runtime.
        session.runtime_backed = true;
        session.runtime_origin_class = PreviewOriginClass::LocalDevServer;
        session.source_sync_class = SourceSyncClass::InSyncFromSource;
        session.freshness_class = PreviewFreshnessClass::Fresh;
        session.write_capable = false;
        session.source_revision_ref = Some(format!("source_revision:{}", session.session_id));
    }
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::PostureSwitchCaseMissing));
}

#[test]
fn downgraded_case_is_present() {
    assert_eq!(packet().downgraded_session_count(), 2);
}

#[test]
fn downgraded_session_without_trigger_fails() {
    let mut packet = packet();
    let stale = packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "preview-session:preview-route:0002")
        .expect("stale session");
    stale.downgrade_trigger = None;
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::DowngradedSessionMissingLabelOrTrigger));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    let stale = packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "preview-session:preview-route:0002")
        .expect("stale session");
    stale.degraded_label = Some("stale".to_owned());
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::DowngradedSessionMissingLabelOrTrigger));
}

#[test]
fn non_downgraded_session_carrying_downgrade_fails() {
    let mut packet = packet();
    first_route_session(&mut packet).downgrade_trigger =
        Some(SessionDowngradeTrigger::PolicyNarrowed);
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::NonDowngradedSessionCarriesDowngrade));
}

#[test]
fn runtime_only_masquerading_as_source_fails() {
    let mut packet = packet();
    let runtime_only = packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "preview-session:preview-route:0003")
        .expect("runtime-only session");
    runtime_only.claims_saved_source = true;
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::RuntimeOnlyMasqueradesAsSource));
}

#[test]
fn runtime_only_carrying_source_revision_fails() {
    let mut packet = packet();
    let runtime_only = packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "preview-session:preview-route:0003")
        .expect("runtime-only session");
    runtime_only.source_revision_ref = Some("source_revision:leak".to_owned());
    let violations = packet.validate();
    assert!(
        violations.contains(&PreviewSessionDescriptorSetViolation::RuntimeOnlyMasqueradesAsSource)
    );
    assert!(violations
        .contains(&PreviewSessionDescriptorSetViolation::SourceRevisionPresenceInconsistent));
}

#[test]
fn live_posture_without_live_runtime_fails() {
    let mut packet = packet();
    let session = first_route_session(&mut packet);
    session.data_posture = PreviewDataPostureClass::Live;
    session.runtime_backed = false;
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::LiveDataPostureWithoutLiveRuntime));
}

#[test]
fn captured_posture_claiming_runtime_fails() {
    let mut packet = packet();
    let captured = packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "preview-session:notebook-adjacent:0001")
        .expect("captured session");
    captured.runtime_backed = true;
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::CapturedPostureClaimsRuntimeOrWrite));
}

#[test]
fn source_relative_session_without_revision_fails() {
    let mut packet = packet();
    first_route_session(&mut packet).source_revision_ref = None;
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::SourceRevisionPresenceInconsistent));
}

#[test]
fn write_capable_session_skipping_diff_preview_fails() {
    let mut packet = packet();
    let framework = packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "preview-session:framework-pack:0001")
        .expect("framework session");
    framework.previews_source_diff_before_commit = false;
    assert!(packet.validate().contains(
        &PreviewSessionDescriptorSetViolation::WriteCapableSessionUnbackedOrSkipsDiffPreview
    ));
}

#[test]
fn captured_session_claiming_write_fails() {
    let mut packet = packet();
    let captured = packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "preview-session:notebook-adjacent:0001")
        .expect("captured session");
    captured.write_capable = true;
    captured.previews_source_diff_before_commit = true;
    let violations = packet.validate();
    assert!(violations
        .contains(&PreviewSessionDescriptorSetViolation::CapturedPostureClaimsRuntimeOrWrite));
    assert!(violations.contains(
        &PreviewSessionDescriptorSetViolation::WriteCapableSessionUnbackedOrSkipsDiffPreview
    ));
}

#[test]
fn unpaired_viewport_dimensions_fail() {
    let mut packet = packet();
    let framework = packet
        .sessions
        .iter_mut()
        .find(|s| s.session_id == "preview-session:framework-pack:0001")
        .expect("framework session");
    framework.viewport_pixel_height = None;
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::ViewportDimensionMismatch));
}

#[test]
fn session_without_evidence_fails() {
    let mut packet = packet();
    packet.sessions[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::SessionEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != PREVIEW_SESSION_DESCRIPTOR_SET_DOC_REF);
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.stale_or_downgraded_sessions_export_truth = false;
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .downgraded_sessions_labeled_below_current = false;
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewSessionDescriptorSetViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: PreviewSessionDescriptorSet =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn chip_tokens_name_governed_chips() {
    let session = &packet().sessions[0];
    let chips = session.chip_tokens();
    assert!(chips.contains("data=live"));
    assert!(chips.contains("source_sync=in_sync_from_source"));
    assert!(chips.contains("freshness=fresh"));
}

#[test]
fn markdown_summary_names_sessions() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Preview-Session Descriptors"));
    assert!(summary.contains("framework_pack_preview"));
    assert!(summary.contains("Downgraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_preview_session_descriptor_set_export()
        .expect("checked preview session descriptor set export validates");
    assert_eq!(checked, packet());
}
