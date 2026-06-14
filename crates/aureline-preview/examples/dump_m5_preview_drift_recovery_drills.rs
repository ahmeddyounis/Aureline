//! Conformance dump for the M5 preview drift-recovery drill set packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_preview::preview_drift_recovery::*;
use aureline_preview::{
    DeviceCapabilityClass, PreviewConsumerSurface, PreviewDataPostureClass, PreviewFreshnessClass,
    PreviewOriginClass, PreviewTargetClass, SourceMappingQualityClass, SourceSyncClass,
};

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
        // Hot-reload reset: runtime stays reachable, in-memory state discarded,
        // source-sync and target preserved — a clean recovery.
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
        // Stale source map: the runtime is fine but jumps can no longer claim an
        // exact map; the view holds drifted until it is remapped.
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
        // Dev server lost: the live feed is gone, the view falls back to its last
        // capture and can no longer claim a runtime.
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
        // Device reconnect: the same tethered device dropped and is re-attaching;
        // target and runtime identity are preserved while reconnect is required.
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
        // Browser session expired: the runtime claim drops, the last frame is
        // retained as a capture, reconnect required.
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
        // Runtime replaced: a different runtime took over; the view is honestly
        // drifted from source until rebuilt rather than carrying the old in-sync
        // claim forward.
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
        // Data-posture flip: the user switched live to mock; the governed data
        // chip changes and nothing else degrades — a clean recovery.
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
        packet_id: "m5-preview-drift-recovery-drills:stable:0001".to_owned(),
        set_label: "M5 Preview Drift-Recovery Drills".to_owned(),
        drills: drills(),
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
