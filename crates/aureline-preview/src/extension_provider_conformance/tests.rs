use super::*;

const PACKET_ID: &str = "m5-extension-provider-conformance:stable:0001";

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:conformance:{id}")]
}

/// The strong, fully-capable first-party declaration that backs the live row and
/// stands in as the preserved prior declaration where a weaker provider regressed.
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
        // Live, conformant, first-party provider whose declaration satisfies the
        // claimed row; the only row that may offer a write-capable designer flow.
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
        // Conformant first-party provider whose host is offline, so the row degrades
        // to a mirror/offline snapshot as bounded truth rather than a blank surface.
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
        // Contributed provider whose declaration went stale and must be re-verified;
        // the prior declaration is preserved and the row stays inspect-only.
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
        // Contributed weaker provider that would replace a stronger one; the swap is
        // refused, the prior stronger declaration is preserved, and the row degrades
        // to a policy-limited profile with restore guidance.
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
        // Contributed provider that became unavailable; the last declaration and
        // limitation notes are preserved as unresolved state on a mirror profile.
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
        packet_id: PACKET_ID.to_owned(),
        set_label: "M5 Extension-Provider Conformance".to_owned(),
        rows: rows(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

fn row_mut<'a>(
    packet: &'a mut ProviderConformancePacket,
    row_id: &str,
) -> &'a mut ProviderConformanceRow {
    packet
        .rows
        .iter_mut()
        .find(|r| r.row_id == row_id)
        .unwrap_or_else(|| panic!("row {row_id}"))
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_operating_profile_is_present() {
    let profiles = packet().represented_operating_profiles();
    for profile in OperatingProfileClass::ALL {
        assert!(
            profiles.contains(&profile),
            "missing operating profile: {}",
            profile.as_str()
        );
    }
}

#[test]
fn every_provider_status_is_present() {
    let statuses = packet().represented_statuses();
    for status in ProviderStatusClass::ALL {
        assert!(
            statuses.contains(&status),
            "missing provider status: {}",
            status.as_str()
        );
    }
}

#[test]
fn both_provider_origins_are_present() {
    let origins = packet().represented_provider_origins();
    for origin in ProviderOriginClass::ALL {
        assert!(
            origins.contains(&origin),
            "missing provider origin: {}",
            origin.as_str()
        );
    }
}

#[test]
fn write_capable_and_disclosure_cases_present() {
    let packet = packet();
    assert_eq!(packet.write_capable_row_count(), 1);
    assert_eq!(packet.disclosed_row_count(), 4);
}

#[test]
fn missing_operating_profile_fails() {
    let mut packet = packet();
    packet
        .rows
        .retain(|r| r.operating_profile != OperatingProfileClass::InspectOnly);
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::RequiredOperatingProfileMissing));
}

#[test]
fn missing_provider_status_fails() {
    let mut packet = packet();
    packet
        .rows
        .retain(|r| r.status != ProviderStatusClass::WeakerReplacement);
    let violations = packet.validate();
    assert!(violations.contains(&ProviderConformanceViolation::RequiredProviderStatusMissing));
    assert!(violations.contains(&ProviderConformanceViolation::WeakerReplacementCaseMissing));
}

#[test]
fn missing_contributed_origin_fails() {
    let mut packet = packet();
    packet
        .rows
        .retain(|r| r.provider_origin != ProviderOriginClass::Contributed);
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::ProviderOriginCoverageMissing));
}

#[test]
fn live_row_with_unsatisfying_declaration_fails() {
    let mut packet = packet();
    // Require a target kind the live provider never declared.
    row_mut(&mut packet, "conformance:embedded-preview:0001")
        .requirement
        .required_target_kind = BrowserRuntimeTargetKind::CapturedSnapshot;
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::LiveRequiresConformance));
}

#[test]
fn live_row_requiring_unmet_attach_depth_fails() {
    let mut packet = packet();
    let row = row_mut(&mut packet, "conformance:device-browser:0001");
    // Make the stale device row live; its declaration only attaches dom_only but the
    // requirement asks for storage depth.
    row.status = ProviderStatusClass::Conformant;
    row.operating_profile = OperatingProfileClass::Live;
    row.prior_declaration = None;
    row.repair = None;
    row.downgrade_trigger = None;
    row.degraded_label = None;
    row.requirement.required_attach_depth = AttachDepthClass::DomStylesNetworkStorage;
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::LiveRequiresConformance));
}

#[test]
fn nonconformant_row_on_live_profile_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:device-browser:0001").operating_profile =
        OperatingProfileClass::Live;
    let violations = packet.validate();
    assert!(violations.contains(&ProviderConformanceViolation::NonconformantRowStillLive));
}

#[test]
fn stale_row_without_prior_declaration_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:device-browser:0001").prior_declaration = None;
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::PriorDeclarationInconsistent));
}

#[test]
fn conformant_row_carrying_prior_declaration_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:embedded-preview:0001").prior_declaration =
        Some(strong_declaration());
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::PriorDeclarationInconsistent));
}

#[test]
fn weaker_replacement_that_is_not_weaker_fails() {
    let mut packet = packet();
    // Replace the weaker current declaration with the strong one so it is no longer
    // weaker than the preserved prior.
    row_mut(&mut packet, "conformance:external-browser:0002").declaration = ProviderDeclaration {
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
    };
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::WeakerReplacementNotWeaker));
}

#[test]
fn bounded_row_without_repair_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:external-browser:0001").repair = None;
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::RepairGuidanceInconsistent));
}

#[test]
fn clean_live_row_with_repair_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:embedded-preview:0001").repair = Some(RepairGuidance {
        action: RepairActionClass::ReverifyDeclaration,
        guidance_summary: "An unexpected repair note on a clean live row".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::RepairGuidanceInconsistent));
}

#[test]
fn generic_repair_guidance_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:external-browser:0001")
        .repair
        .as_mut()
        .unwrap()
        .guidance_summary = "try again".to_owned();
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::RepairGuidanceInconsistent));
}

#[test]
fn bounded_row_without_downgrade_fails() {
    let mut packet = packet();
    let row = row_mut(&mut packet, "conformance:external-browser:0001");
    row.downgrade_trigger = None;
    row.degraded_label = None;
    let violations = packet.validate();
    assert!(violations.contains(&ProviderConformanceViolation::DowngradeInconsistent));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:external-browser:0001").degraded_label =
        Some("disconnected".to_owned());
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::DowngradeInconsistent));
}

#[test]
fn degraded_label_without_trigger_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:embedded-preview:0001").degraded_label =
        Some("A precise but unexpected label on a clean live row".to_owned());
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::DowngradeInconsistent));
}

#[test]
fn write_capable_inspect_only_row_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:device-browser:0001").offers_write_capable_flow = true;
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::WriteCapabilityUnbacked));
}

#[test]
fn declaration_without_target_kinds_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:embedded-preview:0001")
        .declaration
        .supported_target_kinds
        .clear();
    let violations = packet.validate();
    assert!(violations.contains(&ProviderConformanceViolation::DeclarationIncomplete));
}

#[test]
fn declaration_without_client_scope_token_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "conformance:embedded-preview:0001")
        .declaration
        .client_scope_limit_token = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::DeclarationIncomplete));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.rows[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != EXTENSION_PROVIDER_CONFORMANCE_DOC_REF);
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet
        .guardrails
        .weaker_provider_never_silently_swaps_semantics = false;
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .support_export_reconstructs_operating_profile = false;
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&ProviderConformanceViolation::WrongRecordKind));
}

#[test]
fn declaration_satisfies_and_strength_helpers() {
    let strong = strong_declaration();
    let req = ClaimedRowRequirement {
        required_target_kind: BrowserRuntimeTargetKind::EmbeddedPreview,
        required_mapping_quality: InspectorMappingQualityClass::Exact,
        required_attach_depth: AttachDepthClass::DomStylesNetworkStorage,
        requires_hot_reload: true,
    };
    assert!(strong.satisfies(&req));

    let weaker = ProviderDeclaration {
        supported_target_kinds: vec![BrowserRuntimeTargetKind::ExternalBrowser],
        supported_mapping_qualities: vec![InspectorMappingQualityClass::RuntimeOnly],
        max_attach_depth: AttachDepthClass::DomOnly,
        hot_reload: HotReloadDeclarationClass::Unsupported,
        client_scope_limit_token: "single_client".to_owned(),
    };
    assert!(!weaker.satisfies(&req));
    assert!(weaker.is_weaker_than(&strong));
    assert!(strong.at_least_as_strong_as(&weaker));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: ProviderConformancePacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn chip_tokens_name_governed_chips() {
    let row = &packet().rows[0];
    let chips = row.chip_tokens();
    assert!(chips.contains("origin=first_party"));
    assert!(chips.contains("status=conformant"));
    assert!(chips.contains("profile=live"));
    assert!(chips.contains("target=embedded_preview"));
    assert!(chips.contains("hot_reload=supported"));
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Extension-Provider Conformance"));
    assert!(summary.contains("conformance:external-browser:0002"));
    assert!(summary.contains("Repair:"));
    assert!(summary.contains("Disclosed:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_extension_provider_conformance_export()
        .expect("checked provider conformance export validates");
    assert_eq!(checked, packet());
}
