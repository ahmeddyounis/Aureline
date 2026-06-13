//! Protected tests for incident workspace and runbook packet alpha.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_crash::{
    CrashDumpManifest, CrashEnvelope, CrashIncidentTrail, CrashIncidentTrailInputs,
};
use aureline_incident::{
    evidence_kinds, fixture_exact_build_capture, DiagnosisLatencyMeasurement, EvidenceAvailability,
    IncidentActionContext, IncidentEvidenceAttachment, IncidentEvidenceKind, IncidentRunbookPacket,
    IncidentWorkspaceBuilder, LocalContinuityState, MissingSpan, MissingSpanImpactClass,
    MissingSpanKind, MissingSpanReasonClass, ProviderLaneState,
    SUPPORT_ITEM_INCIDENT_DIAGNOSIS_LATENCY_SCORECARD, SUPPORT_ITEM_INCIDENT_MISSING_SPANS,
    SUPPORT_RUNBOOK_PACKET_SCHEMA_REF,
};
use aureline_provider::{project_work_item_object_row, seeded_work_item_transition_beta_page};
use aureline_support::bundle::{
    ActionPolicySourceContext, ActionabilityImpactClass, DiagnosticDataClass, HighRiskContentClass,
    PreviewItemSeed, RedactionState, SizeEstimate, SupportBundlePreview,
    SupportBundlePreviewBuilder,
};
use serde::Deserialize;

const GENERATED_AT: &str = "2026-05-14T09:00:00Z";

#[derive(Debug, Deserialize)]
struct IncidentFixture {
    workspace_id: String,
    title: String,
    summary: String,
    provider_lane_state: ProviderLaneState,
    local_continuity_state: LocalContinuityState,
    log_slice_ref: String,
    trace_window_ref: String,
    runbook_packet_fixture: String,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repo root")
        .to_path_buf()
}

fn load_fixture() -> IncidentFixture {
    let path = repo_root()
        .join("fixtures")
        .join("support")
        .join("incident_workspace_alpha")
        .join("provider_unavailable_missing_span.yaml");
    serde_yaml::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn load_json<T>(path: PathBuf) -> T
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn crash_trail_missing_symbolication() -> CrashIncidentTrail {
    let root = repo_root()
        .join("fixtures")
        .join("support")
        .join("incident_trail_alpha");
    CrashIncidentTrail::from_inputs(CrashIncidentTrailInputs {
        incident_trail_id: "crash-incident-trail:renderer-panic:missing-symbols".into(),
        generated_at: GENERATED_AT.into(),
        alpha_channel_ref: "alpha-channel:preview:design-partner-linux".into(),
        crash_envelope: load_json::<CrashEnvelope>(root.join("crash_envelope.json")),
        crash_dump_manifest: load_json::<CrashDumpManifest>(root.join("crash_dump_manifest.json")),
        symbolication_report: None,
        support_bundle_manifest_ref: Some(
            "support.bundle.manifest.incident_workspace.local_review".into(),
        ),
        support_preview_snapshot_ref: Some(
            "preview-snapshot:support-bundle:incident-workspace:local-review".into(),
        ),
    })
}

fn support_preview() -> SupportBundlePreview {
    let mut builder = SupportBundlePreviewBuilder::new(
        "support-bundle:incident-workspace:local-review",
        "Incident workspace local support bundle",
        GENERATED_AT,
        fixture_exact_build_capture(),
    );
    builder.add_item(PreviewItemSeed {
        support_pack_item_id: "support.item.build_identity".into(),
        title: "Exact build identity".into(),
        data_class: DiagnosticDataClass::MetadataOnly,
        high_risk_content_class: HighRiskContentClass::NotApplicable,
        bundle_section_class: "build_and_install_truth".into(),
        artifact_kind_class: "exact_build_identity_manifest".into(),
        manifest_path_ref: "preview_items[0]".into(),
        bundle_member_path_ref: Some("manifest/build_identity.json".into()),
        source_refs: vec!["build-id:aureline:preview:fixture".into()],
        size_estimate: SizeEstimate {
            estimated_bytes: Some(4096),
            confidence_class: "estimated".into(),
            display_label: "4 KB".into(),
            size_source_class: "collector_estimate".into(),
        },
        impact_class: ActionabilityImpactClass::High,
        impact_summary: "Build identity joins incident, support, and runbook evidence.".into(),
        notes: "Metadata-only support preview row.".into(),
    });
    builder.build().expect("build support preview")
}

fn action_context() -> IncidentActionContext {
    IncidentActionContext {
        command_id: "cmd:tasks.rerun_failed".into(),
        command_descriptor_ref: "cmd-rev:tasks.rerun_failed:alpha".into(),
        invocation_session_id: "inv:tasks.rerun_failed:fixture".into(),
        target_identity_ref: "target:local:workspace".into(),
        action_route_packet_ref: Some("route-packet:tasks.rerun_failed:fixture".into()),
        action_origin_class: "user_keystroke_local".into(),
        action_target_class: "local_host_target".into(),
        action_route_class: "local_workspace_route".into(),
        action_exposure_class: "local_only_no_external_exposure".into(),
        policy_source: ActionPolicySourceContext {
            policy_source_ref: "policy-source:local-default:1".into(),
            policy_epoch: "1".into(),
            trust_state: "trusted".into(),
            policy_bundle_ref: Some("policy-bundle:local-default:0001".into()),
            source_class: "invocation_policy_context".into(),
        },
        route_summary:
            "Task history preserves command, invocation, target, route, and policy refs.".into(),
        reviewed_enforcement_ref: Some("reviewed-enforcement:tasks.rerun_failed".into()),
        redaction_class: "metadata_safe_default".into(),
    }
}

fn runbook_packet(fixture: &IncidentFixture) -> IncidentRunbookPacket {
    let path = repo_root().join(&fixture.runbook_packet_fixture);
    let yaml =
        fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    IncidentRunbookPacket::from_support_runbook_yaml(
        &yaml,
        fixture_exact_build_capture().exact_build_refs,
    )
    .expect("runbook packet parses")
}

fn incident_workspace() -> aureline_incident::IncidentWorkspacePacket {
    let fixture = load_fixture();
    let mut builder = IncidentWorkspaceBuilder::new(
        fixture.workspace_id.clone(),
        fixture.title.clone(),
        fixture.summary.clone(),
        GENERATED_AT,
        fixture_exact_build_capture(),
    )
    .with_provider_lane_state(fixture.provider_lane_state)
    .with_local_continuity_state(fixture.local_continuity_state);

    builder.add_evidence(IncidentEvidenceAttachment::log_slice(
        "log-slice:renderer:startup",
        fixture.log_slice_ref.clone(),
        Some(8192),
    ));
    builder.add_evidence(IncidentEvidenceAttachment::task_history(
        "task-history:rerun-failed",
        "task-history:rerun-failed:summary",
        action_context(),
    ));
    builder.attach_crash_trail(&crash_trail_missing_symbolication());
    builder.attach_support_bundle_preview(&support_preview());
    let provider_page = seeded_work_item_transition_beta_page();
    builder.add_linked_work_item_row(project_work_item_object_row(
        provider_page
            .detail_records
            .iter()
            .find(|record| record.canonical_id == "INC-246")
            .expect("incident work-item row"),
    ));
    builder.add_missing_span(
        MissingSpan::new(
            "missing-span:trace:renderer:startup",
            MissingSpanKind::TraceWindow,
            MissingSpanReasonClass::ProviderLaneUnavailable,
            true,
            MissingSpanImpactClass::WeakensFirstDiagnosis,
            "Provider trace lane was unavailable; local logs and crash refs remain attached.",
        )
        .with_expected_source_ref(fixture.trace_window_ref.clone()),
    );
    builder.add_runbook_packet(runbook_packet(&fixture));
    builder.build()
}

#[test]
fn incident_workspace_attaches_evidence_without_claiming_missing_spans_are_present() {
    let packet = incident_workspace();

    assert!(packet.has_evidence_kind(IncidentEvidenceKind::LogSlice));
    assert!(packet.has_evidence_kind(IncidentEvidenceKind::CrashReference));
    assert!(packet.has_evidence_kind(IncidentEvidenceKind::TaskHistory));
    assert!(packet.has_evidence_kind(IncidentEvidenceKind::SupportBundle));
    assert!(packet.has_missing_required_spans());
    assert!(packet.span_coverage_is_honest());
    assert!(!packet.span_coverage.complete_coverage_claimed);
    assert!(packet
        .missing_spans
        .iter()
        .any(|span| span.span_kind == MissingSpanKind::SymbolicationReport));

    let kinds = evidence_kinds(&packet.evidence_attachments);
    assert!(kinds.contains(&IncidentEvidenceKind::LogSlice));
    assert!(kinds.contains(&IncidentEvidenceKind::TaskHistory));
    assert_eq!(packet.linked_work_item_rows.len(), 1);
    assert_eq!(packet.linked_work_item_rows[0].canonical_id, "INC-246");
    assert_eq!(
        packet.lifecycle_binding.export_outcome_token,
        "omitted_by_redaction"
    );
    assert_eq!(packet.lifecycle_binding.delete_outcome_token, "completed");
    assert_eq!(
        packet.lifecycle_binding.destruction_receipt_ref.as_deref(),
        Some("receipt:incident-packet:0001")
    );
}

#[test]
fn diagnosis_latency_scorecard_uses_missing_span_markers_without_fabricating_values() {
    let missing_runbook_span = MissingSpan::new(
        "missing-span:runbook:invocation",
        MissingSpanKind::TaskHistory,
        MissingSpanReasonClass::NotCollected,
        true,
        MissingSpanImpactClass::WeakensFirstDiagnosis,
        "Runbook invocation event was not collected for this incident.",
    )
    .with_expected_source_ref("task-history:runbook:invocation".to_owned());

    let mut builder = IncidentWorkspaceBuilder::new(
        "incident-workspace:synthetic:latency",
        "Synthetic diagnosis latency incident",
        "Synthetic incident with one missing diagnosis-latency checkpoint.",
        GENERATED_AT,
        fixture_exact_build_capture(),
    );
    builder.add_missing_span(missing_runbook_span.clone());
    builder.record_time_to_first_signal(DiagnosisLatencyMeasurement::observed(
        125,
        "incident:start",
        "signal:first-log",
        vec!["log-slice:synthetic:first-signal".into()],
    ));
    builder.record_time_to_first_hypothesis(DiagnosisLatencyMeasurement::observed(
        340,
        "incident:start",
        "hypothesis:first",
        vec!["doctor:finding:synthetic".into()],
    ));
    builder.record_time_to_redacted_export(DiagnosisLatencyMeasurement::observed(
        920,
        "incident:start",
        "support:preview:redacted",
        vec!["support.bundle.manifest.synthetic".into()],
    ));
    builder.record_time_to_runbook_invocation(DiagnosisLatencyMeasurement::missing(
        missing_runbook_span,
    ));

    let packet = builder.build();
    let scorecard = &packet.diagnosis_latency_scorecard;

    assert_eq!(scorecard.missing_measurement_count, 1);
    assert_eq!(scorecard.time_to_first_signal.elapsed_millis(), Some(125));
    assert_eq!(
        scorecard.time_to_first_hypothesis.elapsed_millis(),
        Some(340)
    );
    assert_eq!(
        scorecard.time_to_redacted_export.elapsed_millis(),
        Some(920)
    );
    assert!(scorecard.time_to_runbook_invocation.is_missing());
    assert!(scorecard.contains_missing_span("missing-span:runbook:invocation"));
    assert!(!scorecard.raw_content_exported);
}

#[test]
fn runbook_packet_summary_consumes_support_runbook_fixture_and_exact_build_refs() {
    let fixture = load_fixture();
    let runbook = runbook_packet(&fixture);

    assert_eq!(runbook.step_count, 4);
    assert_eq!(runbook.mutating_step_count, 2);
    assert_eq!(
        runbook.support_schema_ref,
        SUPPORT_RUNBOOK_PACKET_SCHEMA_REF
    );
    assert!(!runbook.exact_build_refs.is_empty());
    assert!(runbook.exportable_with_redaction_controls);
}

#[test]
fn redacted_export_preview_preserves_exact_build_redaction_controls_and_missing_span_honesty() {
    let packet = incident_workspace();
    let preview = packet
        .redacted_export_preview("2026-05-14T09:05:00Z")
        .expect("build redacted export preview");

    assert!(preview.manifest.has_exact_build_identity());
    assert!(preview.honesty_marker_present());
    assert!(preview
        .manifest
        .preview_classification_summary
        .included_support_pack_item_ids
        .contains(&SUPPORT_ITEM_INCIDENT_MISSING_SPANS.to_owned()));
    assert!(preview
        .manifest
        .preview_classification_summary
        .included_support_pack_item_ids
        .contains(&SUPPORT_ITEM_INCIDENT_DIAGNOSIS_LATENCY_SCORECARD.to_owned()));
    assert_eq!(preview.manifest.diagnosis_latency_scorecards.len(), 1);
    let latency_scorecard = &preview.manifest.diagnosis_latency_scorecards[0];
    assert_eq!(
        latency_scorecard.support_pack_item_id,
        SUPPORT_ITEM_INCIDENT_DIAGNOSIS_LATENCY_SCORECARD
    );
    assert!(latency_scorecard.time_to_first_signal.is_missing());
    assert!(latency_scorecard
        .time_to_first_signal
        .missing_span_id
        .as_deref()
        .is_some_and(|id| id.contains("trace")));
    assert!(!latency_scorecard.raw_content_exported);
    assert!(preview
        .manifest
        .preview_export_parity
        .reconstruction_fields
        .iter()
        .any(|field| field == "diagnosis_latency_scorecards[]"));
    assert_eq!(preview.manifest.action_reconstruction_contexts.len(), 1);
    assert!(preview.manifest.redaction_controls.iter().all(|control| {
        !control.raw_content_export_allowed && control.broadening_requires_review
    }));

    let log_row = preview
        .manifest
        .preview_items
        .iter()
        .find(|item| item.file_section_identity.artifact_kind_class == "incident_log_slice_ref")
        .expect("log row");
    assert_eq!(
        log_row.redaction.redaction_state,
        RedactionState::OmittedPendingOptIn
    );

    let runbook_row = preview
        .manifest
        .preview_items
        .iter()
        .find(|item| {
            item.file_section_identity.artifact_kind_class == "support_runbook_packet_record"
        })
        .expect("runbook row");
    assert_eq!(
        runbook_row.redaction.redaction_state,
        RedactionState::NotRequiredMetadata
    );
}

#[test]
fn provider_unavailable_workspace_stays_local_and_export_preview_reopens_without_network() {
    let packet = incident_workspace();
    assert_eq!(packet.provider_lane_state, ProviderLaneState::Unavailable);
    assert!(packet.stays_usable_without_provider_lane());
    assert!(packet
        .support_bundle_links
        .iter()
        .all(|link| link.can_reopen_without_network));

    let preview = packet
        .redacted_export_preview("2026-05-14T09:06:00Z")
        .expect("build export preview");
    assert!(
        preview
            .manifest
            .reopen_after_export_path
            .can_reopen_without_network
    );
    assert!(packet.evidence_attachments.iter().all(|evidence| {
        evidence.availability != EvidenceAvailability::Missing
            || packet
                .missing_spans
                .iter()
                .any(|span| span.required_for_first_diagnosis)
    }));
}
