//! Inline unit coverage for the cross-surface event-reuse packet: seed
//! stability, the one-shared-history reuse contract, the reopen / export /
//! rerun-review / evidence-link flows that resolve to authoritative objects, and
//! the fail-closed guardrails against forked, log-reconstructed, id-rewritten, or
//! provenance-flattening surfaces.

use super::*;

fn seed() -> CrossSurfaceEventReusePacket {
    seeded_cross_surface_event_reuse_packet()
}

#[test]
fn seed_materializes_stable() {
    let packet = seed();
    assert!(
        packet.validate().is_empty(),
        "seed must validate clean: {:?}",
        packet.validate()
    );
    assert_eq!(
        packet.promotion_state,
        BuildTestInteropPromotionState::Stable
    );
    assert_eq!(packet.record_kind, CROSS_SURFACE_EVENT_REUSE_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        CROSS_SURFACE_EVENT_REUSE_SCHEMA_VERSION
    );
}

#[test]
fn seed_reuses_the_first_consumers_history_verbatim() {
    let packet = seed();
    let shared = current_stable_task_event_first_consumers_input().events;
    assert_eq!(
        packet.events, shared,
        "the shared history must be the canonical first-consumers record history"
    );
    assert_eq!(packet.shared_history_digest, shared_history_digest(&shared));
}

#[test]
fn seed_binds_every_consumer_surface() {
    let packet = seed();
    assert_eq!(
        packet.consumer_surface_tokens(),
        vec![
            "task_center",
            "test_tree",
            "coverage_flaky_snapshot",
            "pipeline_overlay",
            "notebook_run",
            "incident_runbook",
            "cli_headless_export",
            "support_export",
        ]
    );
}

#[test]
fn seed_carries_every_flow_kind() {
    let packet = seed();
    assert_eq!(
        packet.flow_kind_tokens(),
        vec!["reopen", "export", "rerun_review", "evidence_link"]
    );
}

#[test]
fn observed_counts_match_the_bound_traces() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        let expected = observed_event_count(&binding.bound_trace_ids, &packet.events);
        assert_eq!(
            binding.observed_event_count,
            expected,
            "{} observed count",
            binding.surface.as_str()
        );
    }
    // The task center, CLI/headless, and support exports read the whole history.
    let whole = packet.events.len();
    for binding in &packet.consumer_bindings {
        if matches!(
            binding.surface,
            ConsumerSurface::TaskCenter
                | ConsumerSurface::CliHeadlessExport
                | ConsumerSurface::SupportExport
        ) {
            assert_eq!(binding.observed_event_count, whole);
        }
    }
}

#[test]
fn every_flow_resolves_to_an_authoritative_shared_object() {
    let packet = seed();
    for flow in &packet.cross_surface_flows {
        let event = packet
            .event_for(&flow.authoritative_event_id)
            .expect("flow resolves to a shared event");
        assert_eq!(
            event.trace_id,
            flow.authoritative_trace_id,
            "{} flow must agree on the authoritative trace",
            flow.flow_kind.as_str()
        );
    }
}

#[test]
fn evidence_joins_explain_consistently() {
    let packet = seed();
    for surface in [
        ReuseEvidenceSurface::SupportBundle,
        ReuseEvidenceSurface::IncidentPacket,
        ReuseEvidenceSurface::AiEvidence,
    ] {
        let view = packet.evidence_join(surface, "view", "2026-06-17T00:01:00Z");
        assert!(
            view.explains_consistently(),
            "{} must explain",
            surface.as_str()
        );
        assert_eq!(view.shared_event_rows.len(), packet.events.len());
        assert_eq!(view.flow_rows.len(), packet.cross_surface_flows.len());
        assert_eq!(view.shared_history_digest, packet.shared_history_digest);
        for row in &view.flow_rows {
            assert!(row.resolves_to_shared_object, "flow row must resolve");
        }
    }
}

#[test]
fn cli_headless_view_reuses_every_binding() {
    let packet = seed();
    let view = packet.cli_headless_view(
        CROSS_SURFACE_EVENT_REUSE_CLI_HEADLESS_ID,
        "2026-06-17T00:01:00Z",
    );
    assert!(view.every_binding_reuses());
    assert_eq!(view.binding_rows.len(), packet.consumer_bindings.len());
    assert_eq!(view.flow_rows.len(), packet.cross_surface_flows.len());
    assert_eq!(view.shared_history_digest, packet.shared_history_digest);
}

#[test]
fn support_export_round_trips_and_stays_safe() {
    let packet = seed();
    let export = packet.support_export(
        CROSS_SURFACE_EVENT_REUSE_SUPPORT_EXPORT_ID,
        "2026-06-17T00:01:00Z",
    );
    assert!(export.is_export_safe());
    let json = serde_json::to_string(&export).expect("serialize");
    let parsed: CrossSurfaceEventReuseSupportExport = serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed, export);
    assert!(parsed.packet.is_stable());
    assert_eq!(
        parsed.packet.shared_history_digest,
        packet.shared_history_digest
    );
}

#[test]
fn consumer_reconstructing_from_logs_blocks_stable() {
    let mut input = current_stable_cross_surface_event_reuse_input();
    for binding in &mut input.consumer_bindings {
        if binding.surface == ConsumerSurface::CoverageFlakySnapshot {
            binding.reconstructs_from_logs = true;
        }
    }
    let packet = CrossSurfaceEventReusePacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BuildTestInteropPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CrossSurfaceFindingKind::ConsumerReconstructsFromLogs));
}

#[test]
fn consumer_forking_history_blocks_stable() {
    let mut input = current_stable_cross_surface_event_reuse_input();
    for binding in &mut input.consumer_bindings {
        if binding.surface == ConsumerSurface::TestTree {
            binding.reads_shared_history = false;
        }
    }
    let packet = CrossSurfaceEventReusePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CrossSurfaceFindingKind::ConsumerForksHistory));
}

#[test]
fn consumer_rewriting_ids_blocks_stable() {
    let mut input = current_stable_cross_surface_event_reuse_input();
    for binding in &mut input.consumer_bindings {
        if binding.surface == ConsumerSurface::NotebookRun {
            binding.preserves_stable_ids = false;
        }
    }
    let packet = CrossSurfaceEventReusePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CrossSurfaceFindingKind::ConsumerRewritesStableIds));
}

#[test]
fn missing_consumer_binding_blocks_stable() {
    let mut input = current_stable_cross_surface_event_reuse_input();
    input
        .consumer_bindings
        .retain(|binding| binding.surface != ConsumerSurface::NotebookRun);
    let packet = CrossSurfaceEventReusePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CrossSurfaceFindingKind::ConsumerBindingMissing));
}

#[test]
fn flow_target_missing_blocks_stable() {
    let mut input = current_stable_cross_surface_event_reuse_input();
    for flow in &mut input.cross_surface_flows {
        if flow.flow_kind == CrossSurfaceFlowKind::Reopen {
            flow.authoritative_event_id = "event:does-not-exist".to_owned();
        }
    }
    let packet = CrossSurfaceEventReusePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CrossSurfaceFindingKind::FlowTargetMissing));
}

#[test]
fn flow_dropping_provenance_blocks_stable() {
    let mut input = current_stable_cross_surface_event_reuse_input();
    for flow in &mut input.cross_surface_flows {
        if flow.flow_kind == CrossSurfaceFlowKind::EvidenceLink {
            flow.preserves_provenance = false;
        }
    }
    let packet = CrossSurfaceEventReusePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CrossSurfaceFindingKind::FlowDropsProvenance));
}

#[test]
fn binding_trace_unknown_blocks_stable() {
    let mut input = current_stable_cross_surface_event_reuse_input();
    for binding in &mut input.consumer_bindings {
        if binding.surface == ConsumerSurface::PipelineOverlay {
            binding.bound_trace_ids = vec!["trace:does-not-exist".to_owned()];
        }
    }
    let packet = CrossSurfaceEventReusePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CrossSurfaceFindingKind::BindingTraceUnknown));
}
