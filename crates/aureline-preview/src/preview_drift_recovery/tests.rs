use super::*;

use crate::preview_origin::{
    DeviceCapabilityClass, PreviewOriginClass, PreviewTargetClass, SourceMappingQualityClass,
};
use crate::preview_session_descriptors::{PreviewConsumerSurface, PreviewDataPostureClass};

const PACKET_ID: &str = "m5-preview-drift-recovery-drills:stable:0001";

#[allow(clippy::too_many_arguments)]
fn snap(
    source_sync: SourceSyncClass,
    data: PreviewDataPostureClass,
    freshness: PreviewFreshnessClass,
    target: PreviewTargetClass,
    capability: DeviceCapabilityClass,
    mapping: SourceMappingQualityClass,
    origin: PreviewOriginClass,
    runtime_backed: bool,
    reconnect_required: bool,
) -> DriftTruthSnapshot {
    DriftTruthSnapshot {
        source_sync_class: source_sync,
        data_posture: data,
        freshness_class: freshness,
        target_kind: target,
        device_capability_class: capability,
        source_mapping_quality: mapping,
        runtime_origin_class: origin,
        runtime_backed,
        reconnect_required,
    }
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:drift-recovery:{id}")]
}

fn drills() -> Vec<PreviewDriftRecoveryDrill> {
    vec![
        PreviewDriftRecoveryDrill {
            drill_id: "drift-recovery:hot-reload-reset:0001".to_owned(),
            drift_event_class: DriftEventClass::HotReloadReset,
            consumer_surface: PreviewConsumerSurface::FrameworkPackPreview,
            label_summary:
                "Hot-reload reset discarded in-memory component state while the dev server stayed reachable and source-bound"
                    .to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            before: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Fresh,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Exact,
                PreviewOriginClass::LocalDevServer,
                true,
                false,
            ),
            after: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Fresh,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Exact,
                PreviewOriginClass::LocalDevServer,
                true,
                false,
            ),
            recovery_routes: vec![DriftRecoveryRoute::ReconnectSameRuntime],
            survives_reopen_export: true,
            downgrade_trigger: None,
            degraded_label: None,
            evidence_refs: ev("hot-reload-reset"),
        },
        PreviewDriftRecoveryDrill {
            drill_id: "drift-recovery:stale-source-map:0001".to_owned(),
            drift_event_class: DriftEventClass::StaleSourceMap,
            consumer_surface: PreviewConsumerSurface::FrameworkPackPreview,
            label_summary:
                "Source map drifted from the canonical source; source jumps are held below exact until the map is re-derived"
                    .to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            before: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Fresh,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Exact,
                PreviewOriginClass::LocalDevServer,
                true,
                false,
            ),
            after: snap(
                SourceSyncClass::DriftedFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Aging,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Stale,
                PreviewOriginClass::LocalDevServer,
                true,
                false,
            ),
            recovery_routes: vec![
                DriftRecoveryRoute::RemapSourceThenReload,
                DriftRecoveryRoute::HoldInspectOnlyUntilRemapped,
            ],
            survives_reopen_export: true,
            downgrade_trigger: Some(DriftRecoveryTrigger::SourceMapStale),
            degraded_label: Some(
                "Source map is stale relative to the canonical source; jumps land near, not on, the target until it is re-derived"
                    .to_owned(),
            ),
            evidence_refs: ev("stale-source-map"),
        },
        PreviewDriftRecoveryDrill {
            drill_id: "drift-recovery:dev-server-lost:0001".to_owned(),
            drift_event_class: DriftEventClass::DevServerLost,
            consumer_surface: PreviewConsumerSurface::PreviewRoute,
            label_summary:
                "Dev server disappeared; the preview fell back to its last capture and dropped its live-runtime claim"
                    .to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            before: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Fresh,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Heuristic,
                PreviewOriginClass::LocalDevServer,
                true,
                false,
            ),
            after: snap(
                SourceSyncClass::PendingRebuild,
                PreviewDataPostureClass::Captured,
                PreviewFreshnessClass::Stale,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Heuristic,
                PreviewOriginClass::LocalDevServer,
                false,
                true,
            ),
            recovery_routes: vec![
                DriftRecoveryRoute::RestartRuntime,
                DriftRecoveryRoute::ReimportCaptureSnapshot,
                DriftRecoveryRoute::ExportMetadataOnly,
            ],
            survives_reopen_export: true,
            downgrade_trigger: Some(DriftRecoveryTrigger::RuntimeUnavailable),
            degraded_label: Some(
                "Dev server is unreachable; showing the last capture, not a live view — restart the runtime to resume live preview"
                    .to_owned(),
            ),
            evidence_refs: ev("dev-server-lost"),
        },
        PreviewDriftRecoveryDrill {
            drill_id: "drift-recovery:device-reconnect:0001".to_owned(),
            drift_event_class: DriftEventClass::DeviceReconnect,
            consumer_surface: PreviewConsumerSurface::PreviewRoute,
            label_summary:
                "Tethered device dropped and is re-attaching; the same device target is preserved while reconnect completes"
                    .to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            before: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Fresh,
                PreviewTargetClass::PhysicalDeviceTarget,
                DeviceCapabilityClass::Mobile,
                SourceMappingQualityClass::Heuristic,
                PreviewOriginClass::RemoteOrContainerRuntime,
                true,
                false,
            ),
            after: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Aging,
                PreviewTargetClass::PhysicalDeviceTarget,
                DeviceCapabilityClass::Mobile,
                SourceMappingQualityClass::Heuristic,
                PreviewOriginClass::RemoteOrContainerRuntime,
                true,
                true,
            ),
            recovery_routes: vec![
                DriftRecoveryRoute::ReattachDeviceSession,
                DriftRecoveryRoute::ReconnectSameRuntime,
            ],
            survives_reopen_export: true,
            downgrade_trigger: Some(DriftRecoveryTrigger::ReconnectRequired),
            degraded_label: Some(
                "Device session dropped and must reconnect; the view is held until the same device re-attaches"
                    .to_owned(),
            ),
            evidence_refs: ev("device-reconnect"),
        },
        PreviewDriftRecoveryDrill {
            drill_id: "drift-recovery:browser-session-expired:0001".to_owned(),
            drift_event_class: DriftEventClass::BrowserSessionExpired,
            consumer_surface: PreviewConsumerSurface::PreviewRoute,
            label_summary:
                "Browser-runtime session expired; the last frame is retained as a capture and the live claim is dropped"
                    .to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            before: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Fresh,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Exact,
                PreviewOriginClass::LocalDevServer,
                true,
                false,
            ),
            after: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Captured,
                PreviewFreshnessClass::Aging,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Exact,
                PreviewOriginClass::LocalDevServer,
                false,
                true,
            ),
            recovery_routes: vec![
                DriftRecoveryRoute::ReconnectSameRuntime,
                DriftRecoveryRoute::ExportMetadataOnly,
            ],
            survives_reopen_export: true,
            downgrade_trigger: Some(DriftRecoveryTrigger::ReconnectRequired),
            degraded_label: Some(
                "Browser session expired; showing a captured last frame until the session is re-established"
                    .to_owned(),
            ),
            evidence_refs: ev("browser-session-expired"),
        },
        PreviewDriftRecoveryDrill {
            drill_id: "drift-recovery:runtime-replaced:0001".to_owned(),
            drift_event_class: DriftEventClass::RuntimeReplaced,
            consumer_surface: PreviewConsumerSurface::PreviewRoute,
            label_summary:
                "Runtime was replaced by a different runtime; the view is marked drifted from source until it is rebuilt"
                    .to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            before: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Fresh,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Exact,
                PreviewOriginClass::LocalDevServer,
                true,
                false,
            ),
            after: snap(
                SourceSyncClass::DriftedFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Aging,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Heuristic,
                PreviewOriginClass::RemoteOrContainerRuntime,
                true,
                false,
            ),
            recovery_routes: vec![
                DriftRecoveryRoute::RebuildThenReload,
                DriftRecoveryRoute::ReconnectSameRuntime,
            ],
            survives_reopen_export: true,
            downgrade_trigger: Some(DriftRecoveryTrigger::DriftedFromSource),
            degraded_label: Some(
                "A different runtime took over; the view has drifted from the canonical source and must be rebuilt before it is trusted"
                    .to_owned(),
            ),
            evidence_refs: ev("runtime-replaced"),
        },
        PreviewDriftRecoveryDrill {
            drill_id: "drift-recovery:data-posture-flip:0001".to_owned(),
            drift_event_class: DriftEventClass::DataPostureFlip,
            consumer_surface: PreviewConsumerSurface::NotebookAdjacentPreview,
            label_summary:
                "Data posture flipped from live to mock; the governed data chip changes while source-sync and target hold"
                    .to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            before: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Live,
                PreviewFreshnessClass::Fresh,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Exact,
                PreviewOriginClass::LocalDevServer,
                true,
                false,
            ),
            after: snap(
                SourceSyncClass::InSyncFromSource,
                PreviewDataPostureClass::Mock,
                PreviewFreshnessClass::Fresh,
                PreviewTargetClass::BrowserTabTarget,
                DeviceCapabilityClass::DesktopDefault,
                SourceMappingQualityClass::Exact,
                PreviewOriginClass::LocalDevServer,
                true,
                false,
            ),
            recovery_routes: vec![DriftRecoveryRoute::ReconnectSameRuntime],
            survives_reopen_export: true,
            downgrade_trigger: None,
            degraded_label: None,
            evidence_refs: ev("data-posture-flip"),
        },
    ]
}

fn guardrails() -> DriftRecoveryGuardrails {
    DriftRecoveryGuardrails {
        source_canonical_no_second_writable_model: true,
        drift_never_silently_swaps_target: true,
        runtime_state_never_hides_source_mapping_uncertainty: true,
        inspect_only_never_auto_upgraded_to_write: true,
        embedded_boundaries_not_blurred_into_product: true,
        degraded_state_exports_truth_surviving_reopen: true,
    }
}

fn consumer_projection() -> DriftRecoveryConsumerProjection {
    DriftRecoveryConsumerProjection {
        product_ingests_drills: true,
        docs_help_ingests_drills: true,
        diagnostics_ingests_drills: true,
        support_export_ingests_drills: true,
        release_control_ingests_drills: true,
        degraded_state_labeled_below_current: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_REF.to_owned(),
        PREVIEW_DRIFT_RECOVERY_DRILL_SET_DOC_REF.to_owned(),
        PREVIEW_DRIFT_RECOVERY_DRILL_SET_ARTIFACT_REF.to_owned(),
        "schemas/preview/preview_session_descriptor_set.schema.json".to_owned(),
        "schemas/preview/preview_target_descriptor.schema.json".to_owned(),
        "schemas/preview/hot_reload_state.schema.json".to_owned(),
    ]
}

fn packet() -> PreviewDriftRecoveryDrillSet {
    PreviewDriftRecoveryDrillSet::new(PreviewDriftRecoveryDrillSetInput {
        packet_id: PACKET_ID.to_owned(),
        set_label: "M5 Preview Drift-Recovery Drills".to_owned(),
        drills: drills(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn drill_mut<'a>(
    packet: &'a mut PreviewDriftRecoveryDrillSet,
    id: &str,
) -> &'a mut PreviewDriftRecoveryDrill {
    packet
        .drills
        .iter_mut()
        .find(|d| d.drill_id == id)
        .expect("drill present")
}

#[test]
fn drill_set_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_drift_event_is_present() {
    let events = packet().represented_events();
    for event in DriftEventClass::ALL {
        assert!(events.contains(&event), "missing event: {}", event.as_str());
    }
}

#[test]
fn missing_drift_event_fails() {
    let mut packet = packet();
    packet
        .drills
        .retain(|d| d.drift_event_class != DriftEventClass::DeviceReconnect);
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::RequiredDriftEventMissing));
}

#[test]
fn degraded_and_clean_cases_present() {
    let packet = packet();
    assert_eq!(packet.degraded_drill_count(), 5);
    assert_eq!(packet.clean_recovery_count(), 2);
}

#[test]
fn missing_degraded_case_fails() {
    // Collapse every degraded after-snapshot to a clean, in-sync, fresh state.
    let mut packet = packet();
    for drill in &mut packet.drills {
        drill.after = drill.before.clone();
        // DataPostureFlip must still change the posture to stay event-honest.
        if drill.drift_event_class == DriftEventClass::DataPostureFlip {
            drill.after.data_posture = PreviewDataPostureClass::Mock;
        }
        // RuntimeReplaced must not carry the in-sync claim forward unchanged.
        if drill.drift_event_class == DriftEventClass::RuntimeReplaced {
            drill.after.runtime_origin_class = PreviewOriginClass::RemoteOrContainerRuntime;
            drill.after.source_sync_class = SourceSyncClass::PendingRebuild;
        }
        drill.downgrade_trigger = None;
        drill.degraded_label = None;
    }
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::DegradedDrillCaseMissing));
}

#[test]
fn missing_clean_recovery_fails() {
    // Force every clean drill into a degraded after-snapshot.
    let mut packet = packet();
    for drill in &mut packet.drills {
        if !drill.after_is_degraded() {
            drill.after.reconnect_required = true;
            drill.downgrade_trigger = Some(DriftRecoveryTrigger::ReconnectRequired);
            drill.degraded_label = Some(
                "Forced reconnect-required state for the missing-clean-recovery drill".to_owned(),
            );
        }
    }
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::CleanRecoveryCaseMissing));
}

#[test]
fn target_silently_swapped_fails() {
    let mut packet = packet();
    drill_mut(&mut packet, "drift-recovery:device-reconnect:0001")
        .after
        .target_kind = PreviewTargetClass::DesignRendererTarget;
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::TargetKindSilentlySwapped));
}

#[test]
fn stale_source_map_keeping_exact_fails() {
    let mut packet = packet();
    drill_mut(&mut packet, "drift-recovery:stale-source-map:0001")
        .after
        .source_mapping_quality = SourceMappingQualityClass::Exact;
    let violations = packet.validate();
    assert!(violations.contains(&PreviewDriftRecoveryDrillSetViolation::EventAfterInconsistent));
}

#[test]
fn dev_server_lost_keeping_runtime_fails() {
    let mut packet = packet();
    let drill = drill_mut(&mut packet, "drift-recovery:dev-server-lost:0001");
    drill.after.runtime_backed = true;
    drill.after.data_posture = PreviewDataPostureClass::Live;
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::EventAfterInconsistent));
}

#[test]
fn data_posture_flip_without_change_fails() {
    let mut packet = packet();
    drill_mut(&mut packet, "drift-recovery:data-posture-flip:0001")
        .after
        .data_posture = PreviewDataPostureClass::Live;
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::EventAfterInconsistent));
}

#[test]
fn runtime_replaced_carrying_in_sync_forward_fails() {
    let mut packet = packet();
    let drill = drill_mut(&mut packet, "drift-recovery:runtime-replaced:0001");
    drill.after.source_sync_class = SourceSyncClass::InSyncFromSource;
    drill.after.source_mapping_quality = SourceMappingQualityClass::Exact;
    drill.after.runtime_origin_class = PreviewOriginClass::LocalDevServer;
    drill.downgrade_trigger = None;
    drill.degraded_label = None;
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::EventAfterInconsistent));
}

#[test]
fn live_posture_without_live_runtime_fails() {
    let mut packet = packet();
    drill_mut(&mut packet, "drift-recovery:hot-reload-reset:0001")
        .after
        .runtime_backed = false;
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::SnapshotInconsistent));
}

#[test]
fn captured_posture_with_runtime_fails() {
    let mut packet = packet();
    let drill = drill_mut(&mut packet, "drift-recovery:browser-session-expired:0001");
    // Captured-with-runtime is both snapshot-inconsistent and re-asserts the
    // dropped runtime claim the event forbids.
    drill.after.runtime_backed = true;
    let violations = packet.validate();
    assert!(violations.contains(&PreviewDriftRecoveryDrillSetViolation::SnapshotInconsistent));
}

#[test]
fn degraded_drill_without_trigger_fails() {
    let mut packet = packet();
    drill_mut(&mut packet, "drift-recovery:stale-source-map:0001").downgrade_trigger = None;
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::DegradedDrillMissingLabelOrTrigger));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    drill_mut(&mut packet, "drift-recovery:stale-source-map:0001").degraded_label =
        Some("reconnecting".to_owned());
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::DegradedDrillMissingLabelOrTrigger));
}

#[test]
fn clean_drill_carrying_downgrade_fails() {
    let mut packet = packet();
    drill_mut(&mut packet, "drift-recovery:data-posture-flip:0001").downgrade_trigger =
        Some(DriftRecoveryTrigger::ReconnectRequired);
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::CleanDrillCarriesDowngrade));
}

#[test]
fn inadmissible_recovery_route_fails() {
    let mut packet = packet();
    // ReattachDeviceSession is not admissible for a hot-reload reset.
    drill_mut(&mut packet, "drift-recovery:hot-reload-reset:0001")
        .recovery_routes
        .push(DriftRecoveryRoute::ReattachDeviceSession);
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::RecoveryRouteNotAdmissible));
}

#[test]
fn empty_recovery_routes_fail() {
    let mut packet = packet();
    drill_mut(&mut packet, "drift-recovery:hot-reload-reset:0001")
        .recovery_routes
        .clear();
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::RecoveryRouteNotAdmissible));
}

#[test]
fn drift_state_not_surviving_reopen_fails() {
    let mut packet = packet();
    drill_mut(&mut packet, "drift-recovery:dev-server-lost:0001").survives_reopen_export = false;
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::DriftStateDoesNotSurviveReopen));
}

#[test]
fn drill_without_evidence_fails() {
    let mut packet = packet();
    packet.drills[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::DrillEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != PREVIEW_DRIFT_RECOVERY_DRILL_SET_DOC_REF);
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.drift_never_silently_swaps_target = false;
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .degraded_state_labeled_below_current = false;
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewDriftRecoveryDrillSetViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: PreviewDriftRecoveryDrillSet =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn chip_tokens_name_the_transition() {
    let drill = &packet().drills[1];
    let chips = drill.chip_tokens();
    assert!(chips.contains("event=stale_source_map"));
    assert!(chips.contains("before[sync=in_sync_from_source map=exact data=live]"));
    assert!(chips.contains("after[sync=drifted_from_source map=stale data=live]"));
}

#[test]
fn markdown_summary_names_drills() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Preview Drift-Recovery Drills"));
    assert!(summary.contains("hot_reload_reset"));
    assert!(summary.contains("Degraded:"));
    assert!(summary.contains("recovery:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_preview_drift_recovery_drill_set_export()
        .expect("checked preview drift recovery drill set export validates");
    assert_eq!(checked, packet());
}
