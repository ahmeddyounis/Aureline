//! Conformance dump for the M5 extension-provider conformance packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_preview::extension_provider_conformance::*;
use aureline_preview::{AttachDepthClass, BrowserRuntimeTargetKind, InspectorMappingQualityClass};

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:conformance:{id}")]
}

fn strong_declaration() -> ProviderDeclaration {
    ProviderDeclaration {
        supported_target_kinds: vec![
            BrowserRuntimeTargetKind::EmbeddedPreview,
            BrowserRuntimeTargetKind::ExternalBrowser,
            BrowserRuntimeTargetKind::DeviceBrowser,
        ],
        supported_mapping_qualities: vec![
            InspectorMappingQualityClass::Exact,
            InspectorMappingQualityClass::Approximate,
            InspectorMappingQualityClass::GeneratedOnly,
            InspectorMappingQualityClass::RuntimeOnly,
        ],
        max_attach_depth: AttachDepthClass::DomStylesNetworkStorage,
        hot_reload: HotReloadDeclarationClass::Supported,
        client_scope_limit_token: "single_client".to_owned(),
    }
}

fn rows() -> Vec<ProviderConformanceRow> {
    vec![
        ProviderConformanceRow {
            row_id: "conformance:embedded-preview:0001".to_owned(),
            claimed_surface_label: "Embedded preview source-first inspect-and-edit".to_owned(),
            provider_id: "provider:first-party:embedded-preview".to_owned(),
            provider_origin: ProviderOriginClass::FirstParty,
            declaration: strong_declaration(),
            requirement: ClaimedRowRequirement {
                required_target_kind: BrowserRuntimeTargetKind::EmbeddedPreview,
                required_mapping_quality: InspectorMappingQualityClass::Exact,
                required_attach_depth: AttachDepthClass::DomOnly,
                requires_hot_reload: true,
            },
            prior_declaration: None,
            status: ProviderStatusClass::Conformant,
            operating_profile: OperatingProfileClass::Live,
            offers_write_capable_flow: true,
            repair: None,
            downgrade_trigger: None,
            degraded_label: None,
            label_summary: "First-party embedded-preview provider declared exact mapping, full attach, and hot reload; it backs the live source-first edit row".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            evidence_refs: ev("embedded-preview:0001"),
        },
        ProviderConformanceRow {
            row_id: "conformance:external-browser:0001".to_owned(),
            claimed_surface_label: "External browser preview parity".to_owned(),
            provider_id: "provider:first-party:external-browser".to_owned(),
            provider_origin: ProviderOriginClass::FirstParty,
            declaration: strong_declaration(),
            requirement: ClaimedRowRequirement {
                required_target_kind: BrowserRuntimeTargetKind::ExternalBrowser,
                required_mapping_quality: InspectorMappingQualityClass::Approximate,
                required_attach_depth: AttachDepthClass::DomOnly,
                requires_hot_reload: false,
            },
            prior_declaration: None,
            status: ProviderStatusClass::Conformant,
            operating_profile: OperatingProfileClass::MirrorOffline,
            offers_write_capable_flow: false,
            repair: Some(RepairGuidance {
                action: RepairActionClass::UseMirrorOffline,
                guidance_summary: "The external browser host is offline; reconnect it to leave the mirror snapshot, or keep working against the mirror as read-only bounded truth".to_owned(),
            }),
            downgrade_trigger: Some(ConformanceDowngradeTrigger::OfflineMirrorOnly),
            degraded_label: Some("Only a mirror/offline snapshot is reachable; this row reflects the last captured state, not a live external browser".to_owned()),
            label_summary: "First-party external-browser provider is conformant but its host is offline; the row presents a bounded mirror snapshot".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            evidence_refs: ev("external-browser:0001"),
        },
        ProviderConformanceRow {
            row_id: "conformance:device-browser:0001".to_owned(),
            claimed_surface_label: "Device browser inspect-to-source".to_owned(),
            provider_id: "provider:contributed:device-bridge".to_owned(),
            provider_origin: ProviderOriginClass::Contributed,
            declaration: ProviderDeclaration {
                supported_target_kinds: vec![BrowserRuntimeTargetKind::DeviceBrowser],
                supported_mapping_qualities: vec![
                    InspectorMappingQualityClass::Approximate,
                    InspectorMappingQualityClass::RuntimeOnly,
                ],
                max_attach_depth: AttachDepthClass::DomOnly,
                hot_reload: HotReloadDeclarationClass::RestartOnly,
                client_scope_limit_token: "shared_session_capped".to_owned(),
            },
            requirement: ClaimedRowRequirement {
                required_target_kind: BrowserRuntimeTargetKind::DeviceBrowser,
                required_mapping_quality: InspectorMappingQualityClass::Approximate,
                required_attach_depth: AttachDepthClass::DomOnly,
                requires_hot_reload: false,
            },
            prior_declaration: Some(ProviderDeclaration {
                supported_target_kinds: vec![BrowserRuntimeTargetKind::DeviceBrowser],
                supported_mapping_qualities: vec![
                    InspectorMappingQualityClass::Exact,
                    InspectorMappingQualityClass::Approximate,
                    InspectorMappingQualityClass::RuntimeOnly,
                ],
                max_attach_depth: AttachDepthClass::DomAndStyles,
                hot_reload: HotReloadDeclarationClass::Supported,
                client_scope_limit_token: "shared_session_capped".to_owned(),
            }),
            status: ProviderStatusClass::StaleDeclaration,
            operating_profile: OperatingProfileClass::InspectOnly,
            offers_write_capable_flow: false,
            repair: Some(RepairGuidance {
                action: RepairActionClass::ReverifyDeclaration,
                guidance_summary: "The device-bridge extension changed its declared capabilities; re-verify the provider declaration before this row can leave inspect-only".to_owned(),
            }),
            downgrade_trigger: Some(ConformanceDowngradeTrigger::ProviderDeclarationStale),
            degraded_label: Some("This contributed provider's capability declaration is stale and unverified; the row is held inspect-only until it is re-verified".to_owned()),
            label_summary: "Contributed device-bridge provider declaration went stale; the prior declaration is preserved and the row is held inspect-only".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            evidence_refs: ev("device-browser:0001"),
        },
        ProviderConformanceRow {
            row_id: "conformance:external-browser:0002".to_owned(),
            claimed_surface_label: "External browser network-depth inspection".to_owned(),
            provider_id: "provider:contributed:lite-bridge".to_owned(),
            provider_origin: ProviderOriginClass::Contributed,
            declaration: ProviderDeclaration {
                supported_target_kinds: vec![BrowserRuntimeTargetKind::ExternalBrowser],
                supported_mapping_qualities: vec![
                    InspectorMappingQualityClass::Approximate,
                    InspectorMappingQualityClass::RuntimeOnly,
                ],
                max_attach_depth: AttachDepthClass::DomOnly,
                hot_reload: HotReloadDeclarationClass::Unsupported,
                client_scope_limit_token: "single_client".to_owned(),
            },
            requirement: ClaimedRowRequirement {
                required_target_kind: BrowserRuntimeTargetKind::ExternalBrowser,
                required_mapping_quality: InspectorMappingQualityClass::Approximate,
                required_attach_depth: AttachDepthClass::DomOnly,
                requires_hot_reload: false,
            },
            prior_declaration: Some(ProviderDeclaration {
                supported_target_kinds: vec![
                    BrowserRuntimeTargetKind::ExternalBrowser,
                    BrowserRuntimeTargetKind::DeviceBrowser,
                ],
                supported_mapping_qualities: vec![
                    InspectorMappingQualityClass::Exact,
                    InspectorMappingQualityClass::Approximate,
                    InspectorMappingQualityClass::RuntimeOnly,
                ],
                max_attach_depth: AttachDepthClass::DomStylesNetwork,
                hot_reload: HotReloadDeclarationClass::Supported,
                client_scope_limit_token: "single_client".to_owned(),
            }),
            status: ProviderStatusClass::WeakerReplacement,
            operating_profile: OperatingProfileClass::PolicyLimited,
            offers_write_capable_flow: false,
            repair: Some(RepairGuidance {
                action: RepairActionClass::RestoreStrongerProvider,
                guidance_summary: "The lite-bridge extension declares weaker network depth and no hot reload than the provider it would replace; restore the stronger provider to regain full inspection".to_owned(),
            }),
            downgrade_trigger: Some(ConformanceDowngradeTrigger::WeakerProviderProposed),
            degraded_label: Some("A weaker contributed provider was proposed in place of a stronger one; it is not allowed to silently take over, so this row is policy-limited".to_owned()),
            label_summary: "A weaker contributed lite-bridge provider would replace a stronger one; the swap is refused and the row is policy-limited".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            evidence_refs: ev("external-browser:0002"),
        },
        ProviderConformanceRow {
            row_id: "conformance:remote-preview:0001".to_owned(),
            claimed_surface_label: "Remote preview session network inspection".to_owned(),
            provider_id: "provider:contributed:remote-runtime".to_owned(),
            provider_origin: ProviderOriginClass::Contributed,
            declaration: ProviderDeclaration {
                supported_target_kinds: vec![BrowserRuntimeTargetKind::RemotePreviewSession],
                supported_mapping_qualities: vec![InspectorMappingQualityClass::RuntimeOnly],
                max_attach_depth: AttachDepthClass::DomStylesNetwork,
                hot_reload: HotReloadDeclarationClass::NotApplicable,
                client_scope_limit_token: "shared_session_capped".to_owned(),
            },
            requirement: ClaimedRowRequirement {
                required_target_kind: BrowserRuntimeTargetKind::RemotePreviewSession,
                required_mapping_quality: InspectorMappingQualityClass::RuntimeOnly,
                required_attach_depth: AttachDepthClass::DomStylesNetwork,
                requires_hot_reload: false,
            },
            prior_declaration: Some(ProviderDeclaration {
                supported_target_kinds: vec![BrowserRuntimeTargetKind::RemotePreviewSession],
                supported_mapping_qualities: vec![InspectorMappingQualityClass::RuntimeOnly],
                max_attach_depth: AttachDepthClass::DomStylesNetwork,
                hot_reload: HotReloadDeclarationClass::NotApplicable,
                client_scope_limit_token: "shared_session_capped".to_owned(),
            }),
            status: ProviderStatusClass::Unavailable,
            operating_profile: OperatingProfileClass::MirrorOffline,
            offers_write_capable_flow: false,
            repair: Some(RepairGuidance {
                action: RepairActionClass::ReinstallProvider,
                guidance_summary: "The remote-runtime extension is no longer available; reinstall or re-enable it to restore live remote inspection, otherwise the mirror snapshot is read-only".to_owned(),
            }),
            downgrade_trigger: Some(ConformanceDowngradeTrigger::ProviderUnavailable),
            degraded_label: Some("The backing remote-runtime provider is unavailable; the row reflects its last captured declaration and a mirror snapshot, not a live session".to_owned()),
            label_summary: "Contributed remote-runtime provider became unavailable; its prior declaration is preserved and the row falls back to a mirror snapshot".to_owned(),
            observed_at: "2026-06-07T00:00:00Z".to_owned(),
            evidence_refs: ev("remote-preview:0001"),
        },
    ]
}

fn guardrails() -> ConformanceGuardrails {
    ConformanceGuardrails {
        source_canonical_no_second_writable_model: true,
        runtime_state_never_hides_source_mapping_uncertainty: true,
        inspect_only_never_auto_upgraded_to_write: true,
        embedded_boundaries_not_blurred_into_product: true,
        weaker_provider_never_silently_swaps_semantics: true,
        stale_or_unavailable_provider_preserves_history: true,
        bounded_profiles_explicit_and_exportable: true,
    }
}

fn consumer_projection() -> ConformanceConsumerProjection {
    ConformanceConsumerProjection {
        product_ingests_conformance: true,
        docs_help_ingests_conformance: true,
        diagnostics_ingests_conformance: true,
        support_export_ingests_conformance: true,
        release_control_ingests_conformance: true,
        support_export_reconstructs_operating_profile: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_REF.to_owned(),
        EXTENSION_PROVIDER_CONFORMANCE_DOC_REF.to_owned(),
        EXTENSION_PROVIDER_CONFORMANCE_ARTIFACT_REF.to_owned(),
        "schemas/preview/browser_runtime_inspectors.schema.json".to_owned(),
        "schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json".to_owned(),
    ]
}

fn packet() -> ProviderConformancePacket {
    ProviderConformancePacket::new(ProviderConformancePacketInput {
        packet_id: "m5-extension-provider-conformance:stable:0001".to_owned(),
        set_label: "M5 Extension-Provider Conformance".to_owned(),
        rows: rows(),
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
