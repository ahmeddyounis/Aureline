use super::*;

// ---------------------------------------------------------------------------
// Browser / device-code handoff card set.
// ---------------------------------------------------------------------------

#[test]
fn seeded_card_set_validates() {
    let set = seeded_m5_browser_handoff_card_set();
    assert!(set.validate().is_ok(), "{:?}", set.validate());
    assert_eq!(set.set_id, M5_BROWSER_HANDOFF_CARD_SET_ID);
}

#[test]
fn seeded_card_set_names_every_kind_once() {
    let set = seeded_m5_browser_handoff_card_set();
    assert_eq!(set.cards.len(), BrowserHandoffKind::ALL.len());
    for kind in BrowserHandoffKind::ALL {
        let count = set.cards.iter().filter(|c| c.handoff_kind == kind).count();
        assert_eq!(count, 1, "kind {} not named exactly once", kind.as_str());
    }
}

#[test]
fn only_device_code_card_carries_device_code_disclosure() {
    let set = seeded_m5_browser_handoff_card_set();
    for card in &set.cards {
        let has = card.device_code_disclosure.is_some();
        if card.handoff_kind == BrowserHandoffKind::DeviceCodeAuth {
            assert!(has, "device-code card must disclose a device code");
        } else {
            assert!(
                !has,
                "kind {} must not carry a device code",
                card.handoff_kind.as_str()
            );
        }
    }
}

#[test]
fn every_card_leaves_native_chrome_and_never_impersonates() {
    let set = seeded_m5_browser_handoff_card_set();
    for card in &set.cards {
        assert!(card.opens_outside_native_chrome);
        assert!(!card.impersonates_native_chrome);
        assert!(card.presents_provider_owned_content_labeled);
        assert!(card.local_continuity.work_preserved_locally);
    }
}

#[test]
fn device_code_card_discloses_expiry() {
    let set = seeded_m5_browser_handoff_card_set();
    let card = set
        .cards
        .iter()
        .find(|c| c.handoff_kind == BrowserHandoffKind::DeviceCodeAuth)
        .expect("device-code card present");
    let disclosure = card
        .device_code_disclosure
        .as_ref()
        .expect("disclosure present");
    assert!(disclosure.expiry_disclosure.discloses_expiry());
    assert!(disclosure.code_shown_in_app_not_transmitted);
}

#[test]
fn card_reason_mismatch_fails() {
    let mut set = seeded_m5_browser_handoff_card_set();
    set.cards[0].handoff_reason = HandoffReasonClass::OpenVendorResource;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::HandoffReasonMismatch { .. })
    ));
}

#[test]
fn card_data_exit_mismatch_fails() {
    let mut set = seeded_m5_browser_handoff_card_set();
    let idx = set
        .cards
        .iter()
        .position(|c| c.handoff_kind == BrowserHandoffKind::ProviderContentView)
        .unwrap();
    set.cards[idx].data_exit_boundary = DataExitBoundary::SecurityPayloadsOnly;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::HandoffDataExitMismatch { .. })
    ));
}

#[test]
fn card_impersonating_native_chrome_fails() {
    let mut set = seeded_m5_browser_handoff_card_set();
    set.cards[0].impersonates_native_chrome = true;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::ImpersonatesNativeChrome { .. })
    ));
}

#[test]
fn card_not_leaving_native_chrome_fails() {
    let mut set = seeded_m5_browser_handoff_card_set();
    set.cards[0].opens_outside_native_chrome = false;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::HandoffDoesNotLeaveNativeChrome { .. })
    ));
}

#[test]
fn device_code_without_expiry_fails() {
    let mut set = seeded_m5_browser_handoff_card_set();
    let idx = set
        .cards
        .iter()
        .position(|c| c.handoff_kind == BrowserHandoffKind::DeviceCodeAuth)
        .unwrap();
    set.cards[idx]
        .device_code_disclosure
        .as_mut()
        .unwrap()
        .expiry_disclosure = ExpiryDisclosureClass::NoExpiryApplicable;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::DeviceCodeMissingExpiry { .. })
    ));
}

#[test]
fn non_device_card_carrying_device_code_fails() {
    let mut set = seeded_m5_browser_handoff_card_set();
    let idx = set
        .cards
        .iter()
        .position(|c| c.handoff_kind == BrowserHandoffKind::VendorOutboundLink)
        .unwrap();
    set.cards[idx].device_code_disclosure = Some(DeviceCodeDisclosure {
        code_presentation_ref: "disclosure.spurious".to_owned(),
        code_presentation_label: "Spurious".to_owned(),
        expiry_disclosure: ExpiryDisclosureClass::ExpiresWithCountdown,
        expiry_note: "n/a".to_owned(),
        code_shown_in_app_not_transmitted: true,
    });
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::UnexpectedDeviceCodeDisclosure { .. })
    ));
}

#[test]
fn card_raw_ref_leak_fails() {
    let mut set = seeded_m5_browser_handoff_card_set();
    set.cards[0].provider_identity_ref = "https://provider.example/auth".to_owned();
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::RawRefLeak { .. }) | Err(AuthBoundaryError::RawMaterialInExport)
    ));
}

#[test]
fn card_missing_source_contract_fails() {
    let mut set = seeded_m5_browser_handoff_card_set();
    set.source_contract_refs
        .retain(|r| r != M5_AUTH_BOUNDARY_DEVICE_PERMISSION_REF);
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::MissingSourceContracts)
    ));
}

#[test]
fn card_matrix_csv_has_a_row_per_kind() {
    let set = seeded_m5_browser_handoff_card_set();
    let csv = set.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + BrowserHandoffKind::ALL.len());
    assert!(lines[0].starts_with("handoff_kind,"));
    for kind in BrowserHandoffKind::ALL {
        assert!(csv.contains(kind.as_str()), "csv missing {}", kind.as_str());
    }
}

#[test]
fn card_markdown_lists_every_kind() {
    let summary = seeded_m5_browser_handoff_card_set().render_markdown_summary();
    for kind in BrowserHandoffKind::ALL {
        assert!(
            summary.contains(kind.label()),
            "summary missing {}",
            kind.label()
        );
    }
}

// ---------------------------------------------------------------------------
// Webview origin bar set.
// ---------------------------------------------------------------------------

#[test]
fn seeded_bar_set_validates() {
    let set = seeded_m5_webview_origin_bar_set();
    assert!(set.validate().is_ok(), "{:?}", set.validate());
    assert_eq!(set.set_id, M5_WEBVIEW_ORIGIN_BAR_SET_ID);
}

#[test]
fn seeded_bar_set_names_every_owner_once() {
    let set = seeded_m5_webview_origin_bar_set();
    assert_eq!(set.bars.len(), WebviewOwnerClass::ALL.len());
    for owner in WebviewOwnerClass::ALL {
        let count = set.bars.iter().filter(|b| b.owner_class == owner).count();
        assert_eq!(count, 1, "owner {} not named exactly once", owner.as_str());
    }
}

#[test]
fn every_bar_is_labeled_and_never_impersonates_native_messaging() {
    let set = seeded_m5_webview_origin_bar_set();
    for bar in &set.bars {
        assert!(bar.labeled_as_embedded);
        assert!(!bar.impersonates_native_chrome);
        assert!(!bar.may_show_update_verification);
        assert!(!bar.may_show_device_permission_prompt);
        assert!(!bar.may_show_product_security_messaging);
        assert!(bar
            .capability_limits
            .iter()
            .any(|l| l.limit_class == CapabilityLimitClass::NotNativeTrustChrome));
    }
}

#[test]
fn capability_limit_vocabulary_is_covered() {
    let set = seeded_m5_webview_origin_bar_set();
    for limit in CapabilityLimitClass::ALL {
        assert!(
            set.bars
                .iter()
                .any(|b| b.capability_limits.iter().any(|l| l.limit_class == limit)),
            "limit {} not covered",
            limit.as_str()
        );
    }
}

#[test]
fn bar_origin_disclosure_mismatch_fails() {
    let mut set = seeded_m5_webview_origin_bar_set();
    let idx = set
        .bars
        .iter()
        .position(|b| b.owner_class == WebviewOwnerClass::ExtensionOwned)
        .unwrap();
    set.bars[idx].origin_disclosure = OriginDisclosureClass::FirstPartyOrigin;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::OriginDisclosureMismatch { .. })
    ));
}

#[test]
fn bar_untrusted_broad_permission_fails() {
    let mut set = seeded_m5_webview_origin_bar_set();
    let idx = set
        .bars
        .iter()
        .position(|b| b.owner_class == WebviewOwnerClass::UnknownUntrusted)
        .unwrap();
    set.bars[idx].permission_state = WebviewPermissionState::ScopedPermissionsGranted;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::UntrustedPermissionTooBroad { .. })
    ));
}

#[test]
fn bar_showing_update_verification_fails() {
    let mut set = seeded_m5_webview_origin_bar_set();
    set.bars[0].may_show_update_verification = true;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::EmbeddedImpersonatesNativeMessaging { .. })
    ));
}

#[test]
fn bar_showing_device_permission_prompt_fails() {
    let mut set = seeded_m5_webview_origin_bar_set();
    set.bars[0].may_show_device_permission_prompt = true;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::EmbeddedImpersonatesNativeMessaging { .. })
    ));
}

#[test]
fn bar_impersonating_native_chrome_fails() {
    let mut set = seeded_m5_webview_origin_bar_set();
    set.bars[0].impersonates_native_chrome = true;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::ImpersonatesNativeChrome { .. })
    ));
}

#[test]
fn bar_without_not_native_trust_limit_fails() {
    let mut set = seeded_m5_webview_origin_bar_set();
    set.bars[0]
        .capability_limits
        .retain(|l| l.limit_class != CapabilityLimitClass::NotNativeTrustChrome);
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::MissingNotNativeTrustLimit { .. })
            | Err(AuthBoundaryError::CapabilityLimitClassMissing { .. })
    ));
}

#[test]
fn bar_not_labeled_embedded_fails() {
    let mut set = seeded_m5_webview_origin_bar_set();
    set.bars[0].labeled_as_embedded = false;
    assert!(matches!(
        set.validate(),
        Err(AuthBoundaryError::EmbeddedSurfaceNotLabeled { .. })
    ));
}

#[test]
fn bar_matrix_csv_has_a_row_per_owner() {
    let set = seeded_m5_webview_origin_bar_set();
    let csv = set.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + WebviewOwnerClass::ALL.len());
    assert!(lines[0].starts_with("owner_class,"));
    for owner in WebviewOwnerClass::ALL {
        assert!(
            csv.contains(owner.as_str()),
            "csv missing {}",
            owner.as_str()
        );
    }
}

// ---------------------------------------------------------------------------
// Export safety + fixtures + checked-in round-trips.
// ---------------------------------------------------------------------------

#[test]
fn exports_carry_no_forbidden_material() {
    for json in [
        seeded_m5_browser_handoff_card_set().export_safe_json(),
        seeded_m5_webview_origin_bar_set().export_safe_json(),
    ] {
        let lower = json.to_lowercase();
        assert!(!lower.contains("api_key"));
        assert!(!lower.contains("password"));
        assert!(!lower.contains("secret"));
        assert!(!lower.contains("bearer "));
        assert!(!lower.contains("://"));
    }
}

#[test]
fn narrowed_fixtures_validate() {
    assert!(seeded_device_code_card_fixture().validate().is_ok());
    assert!(seeded_untrusted_webview_origin_bar_fixture()
        .validate()
        .is_ok());
}

#[test]
fn checked_browser_card_set_matches_seed() {
    let from_disk = current_stable_m5_browser_handoff_card_set()
        .expect("checked browser-handoff card set validates");
    assert_eq!(
        from_disk,
        seeded_m5_browser_handoff_card_set(),
        "checked browser-handoff card set drifted from the seed builder"
    );
}

#[test]
fn checked_webview_bar_set_matches_seed() {
    let from_disk = current_stable_m5_webview_origin_bar_set()
        .expect("checked webview origin bar set validates");
    assert_eq!(
        from_disk,
        seeded_m5_webview_origin_bar_set(),
        "checked webview origin bar set drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_match_seed_builders() {
    let card: BrowserHandoffCard = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/auth-boundary/device_code_card.json"
    )))
    .expect("device-code card fixture parses");
    assert_eq!(card, seeded_device_code_card_fixture());

    let bar: WebviewOriginBar = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/auth-boundary/untrusted_webview_origin_bar.json"
    )))
    .expect("untrusted webview bar fixture parses");
    assert_eq!(bar, seeded_untrusted_webview_origin_bar_fixture());
}
