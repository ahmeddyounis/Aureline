use super::*;

const PACKET_ID: &str = "m5-browser-runtime-inspectors:stable:0001";

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
        packet_id: PACKET_ID.to_owned(),
        set_label: "M5 Browser-Runtime Inspectors".to_owned(),
        inspectors: inspectors(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn row_mut<'a>(
    packet: &'a mut BrowserRuntimeInspectorPacket,
    inspector_id: &str,
) -> &'a mut InspectorRow {
    packet
        .inspectors
        .iter_mut()
        .find(|r| r.inspector_id == inspector_id)
        .unwrap_or_else(|| panic!("inspector {inspector_id}"))
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_inspector_kind_is_present() {
    let kinds = packet().represented_inspector_kinds();
    for kind in InspectorKind::ALL {
        assert!(
            kinds.contains(&kind),
            "missing inspector kind: {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_target_kind_is_present() {
    let kinds = packet().represented_target_kinds();
    for kind in BrowserRuntimeTargetKind::ALL {
        assert!(
            kinds.contains(&kind),
            "missing target kind: {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_mapping_quality_is_present() {
    let qualities = packet().represented_mapping_qualities();
    for quality in InspectorMappingQualityClass::ALL {
        assert!(
            qualities.contains(&quality),
            "missing mapping quality: {}",
            quality.as_str()
        );
    }
}

#[test]
fn mutation_and_downgrade_cases_present() {
    let packet = packet();
    assert_eq!(packet.mutation_row_count(), 1);
    assert_eq!(packet.downgraded_row_count(), 1);
}

#[test]
fn missing_target_kind_fails() {
    let mut packet = packet();
    packet
        .inspectors
        .retain(|r| r.target_kind != BrowserRuntimeTargetKind::CapturedSnapshot);
    let violations = packet.validate();
    assert!(violations.contains(&BrowserRuntimeInspectorViolation::RequiredTargetKindMissing));
    assert!(violations.contains(&BrowserRuntimeInspectorViolation::DowngradedRowCaseMissing));
}

#[test]
fn missing_inspector_kind_fails() {
    let mut packet = packet();
    packet
        .inspectors
        .retain(|r| r.inspector_kind != InspectorKind::Storage);
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::RequiredInspectorKindMissing));
}

#[test]
fn missing_mutation_case_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:dom:0001").mutation = None;
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::MutationCaseMissing));
}

#[test]
fn shallow_attach_for_storage_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:storage:0001").attach_depth =
        AttachDepthClass::DomStylesNetwork;
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::AttachDepthInsufficient));
}

#[test]
fn shallow_attach_for_network_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:network:0001").attach_depth = AttachDepthClass::DomAndStyles;
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::AttachDepthInsufficient));
}

#[test]
fn freshness_continuity_mismatch_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:dom:0001").freshness = SessionFreshnessClass::Stale;
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::FreshnessContinuityMismatch));
}

#[test]
fn captured_snapshot_target_with_live_freshness_fails() {
    let mut packet = packet();
    let row = row_mut(&mut packet, "inspector:dom:0001");
    row.target_kind = BrowserRuntimeTargetKind::CapturedSnapshot;
    // freshness stays Live, contradicting the captured-snapshot target.
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::TargetFreshnessMismatch));
}

#[test]
fn reconnect_without_prior_session_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:network:0001").prior_session_ref = None;
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::PriorSessionInconsistent));
}

#[test]
fn fresh_attach_carrying_prior_session_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:dom:0001").prior_session_ref = Some("session:leak".to_owned());
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::PriorSessionInconsistent));
}

#[test]
fn sensitive_inspector_with_passthrough_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:console:0001").redaction_posture =
        RedactionPostureClass::NonSensitivePassthrough;
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::SensitiveValuesUnredacted));
}

#[test]
fn runtime_only_claiming_saved_source_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:console:0001").claims_saved_source = true;
    let violations = packet.validate();
    assert!(violations.contains(&BrowserRuntimeInspectorViolation::RuntimeOnlyMasqueradesAsSource));
    assert!(
        violations.contains(&BrowserRuntimeInspectorViolation::NonSourceBackedClaimsSavedSource)
    );
}

#[test]
fn generated_only_claiming_saved_source_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:css:0001").claims_saved_source = true;
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::NonSourceBackedClaimsSavedSource));
}

#[test]
fn mutation_against_captured_snapshot_fails() {
    let mut packet = packet();
    let row = row_mut(&mut packet, "inspector:storage:0001");
    row.mutation = Some(MutationDescriptor {
        side_effect_class: SideEffectClass::StorageWrite,
        review_posture: MutationReviewPosture::ConfirmationRequired,
    });
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::MutationAffordanceUnbacked));
}

#[test]
fn mutation_with_mismatched_side_effect_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:dom:0001").mutation = Some(MutationDescriptor {
        side_effect_class: SideEffectClass::StorageWrite,
        review_posture: MutationReviewPosture::ReviewRequired,
    });
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::MutationAffordanceUnbacked));
}

#[test]
fn mutation_without_target_identity_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:dom:0001").target_identity_ref = "  ".to_owned();
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::MutationAffordanceUnbacked));
}

#[test]
fn imported_snapshot_without_downgrade_fails() {
    let mut packet = packet();
    let row = row_mut(&mut packet, "inspector:storage:0001");
    row.downgrade_trigger = None;
    row.degraded_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&BrowserRuntimeInspectorViolation::DowngradeInconsistent));
    assert!(violations.contains(&BrowserRuntimeInspectorViolation::DowngradedRowCaseMissing));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:storage:0001").degraded_label = Some("disconnected".to_owned());
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::DowngradeInconsistent));
}

#[test]
fn degraded_label_without_trigger_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "inspector:dom:0002").degraded_label =
        Some("Some precise but unexpected label".to_owned());
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::DowngradeInconsistent));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.inspectors[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != BROWSER_RUNTIME_INSPECTORS_DOC_REF);
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.sensitive_values_redacted_by_default = false;
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .support_export_reconstructs_redaction_posture = false;
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&BrowserRuntimeInspectorViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: BrowserRuntimeInspectorPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn chip_tokens_name_governed_chips() {
    let row = &packet().inspectors[0];
    let chips = row.chip_tokens();
    assert!(chips.contains("inspector=dom"));
    assert!(chips.contains("target=embedded_preview"));
    assert!(chips.contains("attach=dom_only"));
    assert!(chips.contains("redaction=non_sensitive_passthrough"));
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Browser-Runtime Inspectors"));
    assert!(summary.contains("inspector:storage:0001"));
    assert!(summary.contains("Mutation:"));
    assert!(summary.contains("Downgraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_browser_runtime_inspectors_export()
        .expect("checked browser-runtime inspector export validates");
    assert_eq!(checked, packet());
}
