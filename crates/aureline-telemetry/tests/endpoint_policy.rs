use std::borrow::Cow;

use aureline_telemetry::endpoint_policy::{
    EndpointOptInState, EndpointPolicyRow, TraceEventClass, TraceRedactionClass,
};
use aureline_telemetry::trace_event::{BuildIdentityRecord, TraceEventRecord};

fn trace_event(event_class: &'static str, note: Option<String>) -> TraceEventRecord {
    TraceEventRecord {
        schema: Cow::Borrowed(TraceEventRecord::SCHEMA),
        schema_version: TraceEventRecord::SCHEMA_VERSION,
        record_kind: Cow::Borrowed(TraceEventRecord::RECORD_KIND),
        event_id: format!("event:test:{event_class}"),
        trace_id: "trace:endpoint-policy-test".to_string(),
        span_id: format!("span:test:{event_class}"),
        parent_span_id: None,
        span_kind: Cow::Borrowed("point_event"),
        event_class: Cow::Borrowed(event_class),
        protected_journey: Cow::Borrowed("observability"),
        dispatch_layer: Cow::Borrowed("observability_hook"),
        journey_segment_id: Cow::Borrowed("seg.observability.endpoint_policy.test"),
        budget_ref: Cow::Borrowed("path.observability.endpoint_policy"),
        attempt_class: Cow::Borrowed("first_attempt"),
        outcome_class: Cow::Borrowed("completed"),
        degraded_posture: Cow::Borrowed("healthy"),
        fallback_posture: Cow::Borrowed("none"),
        backend: Cow::Borrowed("headless"),
        host_os: Cow::Borrowed("unknown"),
        build: BuildIdentityRecord {
            crate_name: "aureline-telemetry".to_string(),
            crate_version: "0.0.0".to_string(),
            rustc_target_triple: "fixture-target".to_string(),
        },
        exact_build_identity_ref: None,
        hardware_definition_ref: None,
        environment_ref: None,
        fixture_ref: None,
        corpus_manifest: None,
        sampling_profile: Cow::Borrowed("developer_local"),
        sampling_profile_ref: "profile.trace_sampling.developer_local".to_string(),
        retention_class: Cow::Borrowed("hot_path_volatile"),
        export_posture: Cow::Borrowed("excluded_by_default"),
        redaction_class: Cow::Borrowed("operator_only_restricted"),
        started_tick: 1,
        finished_tick: Some(2),
        duration_ticks: Some(1),
        linked_spike_trace_refs: Vec::new(),
        linked_journey_trace_refs: Vec::new(),
        evidence_refs: Vec::new(),
        requirement_refs: Vec::new(),
        note,
    }
}

#[test]
fn opt_out_endpoint_produces_zero_event_projection() {
    let row = EndpointPolicyRow::optional_upload(EndpointOptInState::OptedOut);
    let event = trace_event(TraceEventClass::Startup.as_str(), None);

    let projection = row.project_trace_events(&[event]);

    assert_eq!(projection.event_count, 0);
    assert!(projection.events.is_empty());
    assert_eq!(
        projection.endpoint_policy_row.current_opt_in_state,
        EndpointOptInState::OptedOut
    );
}

#[test]
fn redaction_class_is_applied_before_serialization() {
    let row = EndpointPolicyRow::new(
        "endpoint_policy.trace_event.support_bundle_test",
        aureline_telemetry::endpoint_policy::EndpointIdentity::new(
            "endpoint.telemetry.trace_event.support_bundle_test",
            "Trace-event support bundle test",
            "support_bundle",
        ),
        TraceRedactionClass::MetadataSafeDefault,
        vec![TraceEventClass::Startup],
        EndpointOptInState::OptedIn,
    );
    let event = trace_event(
        TraceEventClass::Startup.as_str(),
        Some("RAW_EVENT_CONTENT_SHOULD_NOT_LEAK".to_string()),
    );

    let projection = row.project_trace_events(&[event]);
    let json = serde_json::to_string(&projection).expect("projection serializes");

    assert_eq!(projection.event_count, 1);
    assert_eq!(
        projection.events[0].redaction_class.as_ref(),
        TraceRedactionClass::MetadataSafeDefault.as_str()
    );
    assert!(projection.events[0].note.is_none());
    assert!(!json.contains("RAW_EVENT_CONTENT_SHOULD_NOT_LEAK"));
    assert!(json.contains("\"redaction_class\":\"metadata_safe_default\""));
}
