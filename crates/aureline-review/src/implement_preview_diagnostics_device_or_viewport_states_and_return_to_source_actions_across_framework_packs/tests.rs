use super::*;

const PACKET_ID: &str = "preview-diagnostics:stable:0001";

fn framework_pack_rows() -> Vec<FrameworkPackRow> {
    vec![
        FrameworkPackRow {
            pack_id: "pack:react".to_owned(),
            pack_class: FrameworkPackClass::WebReact,
            pack_label: "React web framework pack".to_owned(),
            supports_source_mapping: true,
            supports_viewport_emulation: true,
            supports_hot_reload: true,
            coverage_label: "Full source-map, viewport, and hot-reload support".to_owned(),
            disclosure_label: "React pack with disclosed capabilities".to_owned(),
        },
        FrameworkPackRow {
            pack_id: "pack:static-html".to_owned(),
            pack_class: FrameworkPackClass::WebStaticHtml,
            pack_label: "Static HTML pack".to_owned(),
            supports_source_mapping: true,
            supports_viewport_emulation: true,
            supports_hot_reload: false,
            coverage_label: "Source-map and viewport support; no hot reload".to_owned(),
            disclosure_label: "Static HTML pack with disclosed capabilities".to_owned(),
        },
        FrameworkPackRow {
            pack_id: "pack:provider-owned".to_owned(),
            pack_class: FrameworkPackClass::UnknownPackProviderOwned,
            pack_label: "Provider-owned framework pack".to_owned(),
            supports_source_mapping: false,
            supports_viewport_emulation: false,
            supports_hot_reload: false,
            coverage_label: "Provider-owned pack with no disclosed capabilities".to_owned(),
            disclosure_label: "Unknown provider-owned pack; capabilities not assumed".to_owned(),
        },
    ]
}

fn diagnostic_rows() -> Vec<PreviewDiagnosticRow> {
    vec![
        PreviewDiagnosticRow {
            diagnostic_id: "diag:react-type-error".to_owned(),
            durable_anchor_id: "anchor:review:0001".to_owned(),
            pack_id: "pack:react".to_owned(),
            preview_target_label: "Preview of feature/login dashboard".to_owned(),
            viewport_state: DeviceViewportState {
                viewport_class: ViewportClass::Desktop,
                device_label: "Desktop viewport".to_owned(),
                dimensions_label: "1440×900".to_owned(),
                emulated: false,
                state_disclosed: true,
            },
            severity: DiagnosticSeverity::Error,
            diagnostic_kind: DiagnosticKind::TypeError,
            message_label: "Type error in dashboard component props".to_owned(),
            source_mapping: SourceMappingDisclosure {
                mapping_class: SourceMappingClass::ExactLineColumn,
                freshness_class: SourceMappingFreshness::FreshCurrentBuild,
                mapping_disclosed: true,
                mapping_label: "Exact line/column mapping from the current build".to_owned(),
            },
            return_to_source: ReturnToSourceAction {
                action_kind: ReturnToSourceActionKind::JumpToSourceLocal,
                action_disclosed: true,
                read_only: true,
                action_label: "Jump to source".to_owned(),
                handoff_ref: None,
            },
            blocked_class: ReturnToSourceBlockedClass::NotBlocked,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:diag-react-type-error:0001".to_owned(),
            attention_reasons: vec![
                "Error severity blocks a clean preview until resolved".to_owned()
            ],
            review_summary: "Type error with an exact, fresh return-to-source jump".to_owned(),
            source_contract_refs: vec![
                PREVIEW_DIAGNOSTICS_PREVIEW_TARGET_CONTRACT_REF.to_owned(),
                PREVIEW_DIAGNOSTICS_DEVICE_TARGET_CONTRACT_REF.to_owned(),
            ],
        },
        PreviewDiagnosticRow {
            diagnostic_id: "diag:static-hot-reload".to_owned(),
            durable_anchor_id: "anchor:review:0002".to_owned(),
            pack_id: "pack:static-html".to_owned(),
            preview_target_label: "Preview of docs/site landing page".to_owned(),
            viewport_state: DeviceViewportState {
                viewport_class: ViewportClass::Mobile,
                device_label: "Mobile device emulation".to_owned(),
                dimensions_label: "390×844".to_owned(),
                emulated: true,
                state_disclosed: true,
            },
            severity: DiagnosticSeverity::Warning,
            diagnostic_kind: DiagnosticKind::HotReloadFailure,
            message_label: "Hot reload unavailable; falling back to full reload".to_owned(),
            source_mapping: SourceMappingDisclosure {
                mapping_class: SourceMappingClass::ApproximateLine,
                freshness_class: SourceMappingFreshness::StalePriorBuild,
                mapping_disclosed: true,
                mapping_label: "Approximate line mapping from a prior build".to_owned(),
            },
            return_to_source: ReturnToSourceAction {
                action_kind: ReturnToSourceActionKind::OpenInBrowserHandoff,
                action_disclosed: true,
                read_only: false,
                action_label: "Open source in browser".to_owned(),
                handoff_ref: Some("handoff:diag-static-hot-reload".to_owned()),
            },
            blocked_class: ReturnToSourceBlockedClass::NotBlocked,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:diag-static-hot-reload:0002".to_owned(),
            attention_reasons: vec!["Source map is from a prior build and may be stale".to_owned()],
            review_summary: "Hot-reload warning with a stale-but-disclosed source map".to_owned(),
            source_contract_refs: vec![
                PREVIEW_DIAGNOSTICS_HOT_RELOAD_CONTRACT_REF.to_owned(),
                PREVIEW_DIAGNOSTICS_DEVICE_TARGET_CONTRACT_REF.to_owned(),
            ],
        },
        PreviewDiagnosticRow {
            diagnostic_id: "diag:provider-runtime-error".to_owned(),
            durable_anchor_id: "anchor:review:0003".to_owned(),
            pack_id: "pack:provider-owned".to_owned(),
            preview_target_label: "Preview of embed/third-party widget".to_owned(),
            viewport_state: DeviceViewportState {
                viewport_class: ViewportClass::UnknownViewportProviderOwned,
                device_label: "Provider-owned viewport".to_owned(),
                dimensions_label: "Undisclosed by provider".to_owned(),
                emulated: false,
                state_disclosed: true,
            },
            severity: DiagnosticSeverity::Fatal,
            diagnostic_kind: DiagnosticKind::RuntimeError,
            message_label: "Runtime error in a generated, unmapped bundle".to_owned(),
            source_mapping: SourceMappingDisclosure {
                mapping_class: SourceMappingClass::GeneratedNoSourceMap,
                freshness_class: SourceMappingFreshness::UnknownFreshnessProviderOwned,
                mapping_disclosed: true,
                mapping_label: "Generated content with no source map".to_owned(),
            },
            return_to_source: ReturnToSourceAction {
                action_kind: ReturnToSourceActionKind::UnsupportedNoSourceMap,
                action_disclosed: true,
                read_only: true,
                action_label: "Return to source unavailable; no source map".to_owned(),
                handoff_ref: None,
            },
            blocked_class: ReturnToSourceBlockedClass::BlockedNoSourceMap,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:diag-provider-runtime-error:0003".to_owned(),
            attention_reasons: vec![
                "Fatal runtime error halts the preview".to_owned(),
                "Generated content has no source map, so return-to-source is blocked".to_owned(),
                "Viewport and freshness are provider-owned and not assumed".to_owned(),
            ],
            review_summary: "Fatal error in generated content with no return-to-source target"
                .to_owned(),
            source_contract_refs: vec![
                PREVIEW_DIAGNOSTICS_PREVIEW_TARGET_CONTRACT_REF.to_owned(),
                PREVIEW_DIAGNOSTICS_TRUST_CLASS_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn trust_review() -> PreviewDiagnosticsTrustReview {
    PreviewDiagnosticsTrustReview {
        diagnostic_severity_explicit: true,
        device_viewport_state_disclosed: true,
        source_mapping_disclosed: true,
        source_mapping_freshness_disclosed: true,
        return_to_source_action_disclosed: true,
        return_to_source_read_only_unless_attributed: true,
        framework_pack_identity_explicit: true,
        every_diagnostic_anchored: true,
        every_action_attributable: true,
        no_hidden_write_scope: true,
        stale_source_map_narrows_action: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> PreviewDiagnosticsConsumerProjection {
    PreviewDiagnosticsConsumerProjection {
        preview_diagnostics_panel_shows_severity: true,
        preview_panel_shows_viewport_state: true,
        diagnostic_card_shows_framework_pack: true,
        diagnostic_card_shows_source_mapping: true,
        return_to_source_action_shows_freshness: true,
        review_workspace_header_shows_attribution: true,
        command_palette_shows_diagnostic_state: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
        preview_labs_label_for_unqualified: true,
    }
}

fn proof_freshness() -> PreviewDiagnosticsProofFreshness {
    PreviewDiagnosticsProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<PreviewDiagnosticsDowngradeTrigger> {
    vec![
        PreviewDiagnosticsDowngradeTrigger::ProofStale,
        PreviewDiagnosticsDowngradeTrigger::DiagnosticAttributionMissing,
        PreviewDiagnosticsDowngradeTrigger::SourceMapStale,
        PreviewDiagnosticsDowngradeTrigger::ViewportStateUndisclosed,
        PreviewDiagnosticsDowngradeTrigger::FrameworkPackUnknown,
        PreviewDiagnosticsDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<PreviewDiagnosticsConsumerSurface> {
    vec![
        PreviewDiagnosticsConsumerSurface::PreviewDiagnosticsPanel,
        PreviewDiagnosticsConsumerSurface::PreviewPanel,
        PreviewDiagnosticsConsumerSurface::DiagnosticCard,
        PreviewDiagnosticsConsumerSurface::ReturnToSourceAction,
        PreviewDiagnosticsConsumerSurface::CliHeadless,
        PreviewDiagnosticsConsumerSurface::SupportExport,
        PreviewDiagnosticsConsumerSurface::Diagnostics,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PREVIEW_DIAGNOSTICS_SCHEMA_REF.to_owned(),
        PREVIEW_DIAGNOSTICS_DOC_REF.to_owned(),
        PREVIEW_DIAGNOSTICS_PREVIEW_TARGET_CONTRACT_REF.to_owned(),
        PREVIEW_DIAGNOSTICS_DEVICE_TARGET_CONTRACT_REF.to_owned(),
        PREVIEW_DIAGNOSTICS_HOT_RELOAD_CONTRACT_REF.to_owned(),
        PREVIEW_DIAGNOSTICS_TRUST_CLASS_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> PreviewDiagnosticsPacket {
    PreviewDiagnosticsPacket::new(PreviewDiagnosticsPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Preview diagnostics, device/viewport states, and return-to-source actions"
            .to_owned(),
        framework_pack_rows: framework_pack_rows(),
        diagnostic_rows: diagnostic_rows(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn preview_diagnostics_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_framework_pack_rows_fails() {
    let mut packet = packet();
    packet.framework_pack_rows.clear();
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::FrameworkPackRowsMissing));
}

#[test]
fn incomplete_framework_pack_row_fails() {
    let mut packet = packet();
    packet.framework_pack_rows[0].pack_label = String::new();
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::FrameworkPackRowIncomplete));
}

#[test]
fn missing_diagnostic_rows_fails() {
    let mut packet = packet();
    packet.diagnostic_rows.clear();
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::DiagnosticRowsMissing));
}

#[test]
fn incomplete_diagnostic_row_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[0].preview_target_label = String::new();
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::DiagnosticRowIncomplete));
}

#[test]
fn orphan_pack_reference_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[0].pack_id = "pack:does-not-exist".to_owned();
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::OrphanPackReference));
}

#[test]
fn undisclosed_viewport_state_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[0].viewport_state.state_disclosed = false;
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::ViewportStateUndisclosed));
}

#[test]
fn empty_dimensions_label_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[0].viewport_state.dimensions_label = String::new();
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::ViewportStateUndisclosed));
}

#[test]
fn undisclosed_source_mapping_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[0].source_mapping.mapping_disclosed = false;
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::SourceMappingUndisclosed));
}

#[test]
fn undisclosed_return_to_source_action_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[0].return_to_source.action_disclosed = false;
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::ReturnToSourceUndisclosed));
}

#[test]
fn return_to_source_read_only_mismatch_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[0].return_to_source.read_only = false;
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::ReturnToSourceReadOnlyMismatch));
}

#[test]
fn handoff_action_without_ref_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[1].return_to_source.handoff_ref = None;
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::ReturnToSourceHandoffRefMissing));
}

#[test]
fn unsupported_action_without_block_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[2].blocked_class = ReturnToSourceBlockedClass::NotBlocked;
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::ReturnToSourceUnsupportedNotBlocked));
}

#[test]
fn missing_attribution_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[0].actor_attribution_label = String::new();
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::AttributionMissing));
}

#[test]
fn missing_audit_row_ref_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[1].audit_row_ref = String::new();
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::AttributionMissing));
}

#[test]
fn error_without_attention_reason_fails() {
    let mut packet = packet();
    packet.diagnostic_rows[0].attention_reasons.clear();
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::AttentionReasonMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_write_scope = false;
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .return_to_source_action_shows_freshness = false;
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&PreviewDiagnosticsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_section() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Framework packs"));
    assert!(summary.contains("## Diagnostics"));
    assert!(summary.contains("anchor:review:0001"));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_preview_diagnostics_export().expect("checked preview diagnostics export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs/stale_source_map_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs/unknown_pack_blocked.json"
        )),
    ] {
        let packet: PreviewDiagnosticsPacket =
            serde_json::from_str(raw).expect("fixture parses as preview diagnostics packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
