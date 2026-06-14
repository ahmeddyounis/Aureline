//! Conformance dump for the M5 browser-runtime inspector packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_preview::browser_runtime_inspectors::*;
use aureline_preview::AttachDepthClass;

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:inspector:{id}")]
}

fn inspectors() -> Vec<InspectorRow> {
    vec![
        InspectorRow {
            inspector_id: "inspector:dom:0001".to_owned(),
            inspector_kind: InspectorKind::Dom,
            target_kind: BrowserRuntimeTargetKind::EmbeddedPreview,
            target_identity_ref: "target:embedded-preview:0001".to_owned(),
            session_id: "session:embedded-preview:0001".to_owned(),
            prior_session_ref: None,
            continuity: SessionContinuityClass::FreshAttach,
            attach_depth: AttachDepthClass::DomOnly,
            mapping_quality: InspectorMappingQualityClass::Exact,
            freshness: SessionFreshnessClass::Live,
            redaction_posture: RedactionPostureClass::NonSensitivePassthrough,
            label_summary: "DOM inspector on an embedded preview mapped exactly to its canonical-source span; the override previews the real source diff before commit".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            claims_saved_source: true,
            mutation: Some(MutationDescriptor {
                side_effect_class: SideEffectClass::DomMutation,
                review_posture: MutationReviewPosture::ReviewRequired,
            }),
            downgrade_trigger: None,
            degraded_label: None,
            evidence_refs: ev("dom:0001"),
        },
        InspectorRow {
            inspector_id: "inspector:dom:0002".to_owned(),
            inspector_kind: InspectorKind::Dom,
            target_kind: BrowserRuntimeTargetKind::ExternalBrowser,
            target_identity_ref: "target:external-browser:0001".to_owned(),
            session_id: "session:external-browser:0001".to_owned(),
            prior_session_ref: None,
            continuity: SessionContinuityClass::FreshAttach,
            attach_depth: AttachDepthClass::DomOnly,
            mapping_quality: InspectorMappingQualityClass::Approximate,
            freshness: SessionFreshnessClass::Live,
            redaction_posture: RedactionPostureClass::NonSensitivePassthrough,
            label_summary: "DOM inspector on an external browser mapped approximately to source; jump-to-source lands near the span".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            claims_saved_source: false,
            mutation: None,
            downgrade_trigger: None,
            degraded_label: None,
            evidence_refs: ev("dom:0002"),
        },
        InspectorRow {
            inspector_id: "inspector:css:0001".to_owned(),
            inspector_kind: InspectorKind::Css,
            target_kind: BrowserRuntimeTargetKind::SimulatorOrEmulator,
            target_identity_ref: "target:simulator:0001".to_owned(),
            session_id: "session:simulator:0001".to_owned(),
            prior_session_ref: None,
            continuity: SessionContinuityClass::FreshAttach,
            attach_depth: AttachDepthClass::DomAndStyles,
            mapping_quality: InspectorMappingQualityClass::GeneratedOnly,
            freshness: SessionFreshnessClass::Live,
            redaction_posture: RedactionPostureClass::NonSensitivePassthrough,
            label_summary: "CSS inspector on a simulator showing a generated stylesheet with no hand-authored span; inspect-to-source falls back to the generator input".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            claims_saved_source: false,
            mutation: None,
            downgrade_trigger: None,
            degraded_label: None,
            evidence_refs: ev("css:0001"),
        },
        InspectorRow {
            inspector_id: "inspector:console:0001".to_owned(),
            inspector_kind: InspectorKind::Console,
            target_kind: BrowserRuntimeTargetKind::DeviceBrowser,
            target_identity_ref: "target:device-browser:0001".to_owned(),
            session_id: "session:device-browser:0001".to_owned(),
            prior_session_ref: None,
            continuity: SessionContinuityClass::FreshAttach,
            attach_depth: AttachDepthClass::DomOnly,
            mapping_quality: InspectorMappingQualityClass::RuntimeOnly,
            freshness: SessionFreshnessClass::Live,
            redaction_posture: RedactionPostureClass::RedactedByDefault,
            label_summary: "Console inspector on a device browser; message bodies are redacted by default so tokens never leak into diagnostics".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            claims_saved_source: false,
            mutation: None,
            downgrade_trigger: None,
            degraded_label: None,
            evidence_refs: ev("console:0001"),
        },
        InspectorRow {
            inspector_id: "inspector:network:0001".to_owned(),
            inspector_kind: InspectorKind::Network,
            target_kind: BrowserRuntimeTargetKind::RemotePreviewSession,
            target_identity_ref: "target:remote-preview:0001".to_owned(),
            session_id: "session:remote-preview:0002".to_owned(),
            prior_session_ref: Some("session:remote-preview:0001".to_owned()),
            continuity: SessionContinuityClass::Reconnected,
            attach_depth: AttachDepthClass::DomStylesNetwork,
            mapping_quality: InspectorMappingQualityClass::RuntimeOnly,
            freshness: SessionFreshnessClass::Reconnected,
            redaction_posture: RedactionPostureClass::MetadataOnly,
            label_summary: "Network inspector on a remote preview re-attached after a transport drop; only request metadata crosses, and the prior session stays attributable".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            claims_saved_source: false,
            mutation: None,
            downgrade_trigger: None,
            degraded_label: None,
            evidence_refs: ev("network:0001"),
        },
        InspectorRow {
            inspector_id: "inspector:storage:0001".to_owned(),
            inspector_kind: InspectorKind::Storage,
            target_kind: BrowserRuntimeTargetKind::CapturedSnapshot,
            target_identity_ref: "target:captured-snapshot:0001".to_owned(),
            session_id: "session:captured-snapshot:0001".to_owned(),
            prior_session_ref: Some("session:remote-preview:0002".to_owned()),
            continuity: SessionContinuityClass::ImportedSnapshot,
            attach_depth: AttachDepthClass::DomStylesNetworkStorage,
            mapping_quality: InspectorMappingQualityClass::RuntimeOnly,
            freshness: SessionFreshnessClass::CapturedSnapshot,
            redaction_posture: RedactionPostureClass::HashedReference,
            label_summary: "Storage inspector over an imported captured snapshot; storage entries are carried as opaque hashes, not raw values".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            claims_saved_source: false,
            mutation: None,
            downgrade_trigger: Some(InspectorDowngradeTrigger::SnapshotImported),
            degraded_label: Some(
                "This view is an imported captured snapshot, not a live runtime; storage shown is from the capture and has no live session to mutate".to_owned(),
            ),
            evidence_refs: ev("storage:0001"),
        },
    ]
}

fn guardrails() -> InspectorGuardrails {
    InspectorGuardrails {
        source_canonical_no_second_writable_model: true,
        runtime_state_never_hides_source_mapping_uncertainty: true,
        inspect_only_never_auto_upgraded_to_write: true,
        embedded_boundaries_not_blurred_into_product: true,
        sensitive_values_redacted_by_default: true,
        mutation_requires_side_effect_class_and_review: true,
        session_identity_attributable_across_reconnect: true,
    }
}

fn consumer_projection() -> InspectorConsumerProjection {
    InspectorConsumerProjection {
        product_ingests_inspectors: true,
        docs_help_ingests_inspectors: true,
        diagnostics_ingests_inspectors: true,
        support_export_ingests_inspectors: true,
        release_control_ingests_inspectors: true,
        support_export_reconstructs_redaction_posture: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        BROWSER_RUNTIME_INSPECTORS_SCHEMA_REF.to_owned(),
        BROWSER_RUNTIME_INSPECTORS_DOC_REF.to_owned(),
        BROWSER_RUNTIME_INSPECTORS_ARTIFACT_REF.to_owned(),
        "schemas/preview/inspect_to_source_tree_mapping.schema.json".to_owned(),
        "schemas/preview/preview_session_descriptor_set.schema.json".to_owned(),
        "schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json".to_owned(),
    ]
}

fn packet() -> BrowserRuntimeInspectorPacket {
    BrowserRuntimeInspectorPacket::new(BrowserRuntimeInspectorPacketInput {
        packet_id: "m5-browser-runtime-inspectors:stable:0001".to_owned(),
        set_label: "M5 Browser-Runtime Inspectors".to_owned(),
        inspectors: inspectors(),
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
