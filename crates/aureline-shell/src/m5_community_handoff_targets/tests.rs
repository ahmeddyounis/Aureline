use super::*;

#[test]
fn seeded_set_validates() {
    let set = seeded_m5_community_handoff_target_sheet_set();
    assert!(set.validate().is_ok(), "{:?}", set.validate());
    assert_eq!(set.sheet_set_id, M5_COMMUNITY_HANDOFF_TARGET_SHEET_SET_ID);
}

#[test]
fn seeded_set_names_every_route_once() {
    let set = seeded_m5_community_handoff_target_sheet_set();
    assert_eq!(set.sheets.len(), CommunityHandoffRouteClass::ALL.len());
    for route in CommunityHandoffRouteClass::ALL {
        let count = set.sheets.iter().filter(|s| s.route_class == route).count();
        assert_eq!(count, 1, "route {} not named exactly once", route.as_str());
    }
}

#[test]
fn every_trust_class_is_carried() {
    let set = seeded_m5_community_handoff_target_sheet_set();
    for trust in DestinationTrustClass::ALL {
        let primary = set.sheets.iter().any(|s| s.trust_class == trust);
        let fallback = set
            .sheets
            .iter()
            .any(|s| s.local_safe_fallback.trust_class == trust);
        assert!(primary || fallback, "trust class {} not carried", trust.as_str());
    }
}

#[test]
fn official_and_community_routes_are_distinguishable() {
    let set = seeded_m5_community_handoff_target_sheet_set();
    assert!(set.sheets.iter().any(|s| s.trust_class.is_official()));
    assert!(set.sheets.iter().any(|s| s.trust_class.is_community()));
    // No sheet collapses official and community into the same trust class for
    // distinct routes: each route's trust class is explicit.
    for sheet in &set.sheets {
        assert!(sheet.route_class.allows_trust(sheet.trust_class));
    }
}

#[test]
fn only_official_support_is_a_guaranteed_commitment() {
    let set = seeded_m5_community_handoff_target_sheet_set();
    for sheet in &set.sheets {
        let guaranteed = sheet.commitment_honesty.guaranteed_product_commitment;
        if sheet.route_class == CommunityHandoffRouteClass::OfficialSupport {
            assert!(guaranteed, "official support must be a guaranteed commitment");
        } else {
            assert!(
                !guaranteed,
                "route {} must not masquerade as a guaranteed commitment",
                sheet.route_class.as_str()
            );
        }
    }
}

#[test]
fn world_readable_routes_require_review_and_never_auto_open() {
    let set = seeded_m5_community_handoff_target_sheet_set();
    for sheet in &set.sheets {
        if sheet.trust_class.is_world_readable() {
            assert!(sheet.requires_prior_review_before_open);
            assert!(!sheet.auto_open_from_critical_alert_allowed);
        }
    }
}

#[test]
fn private_security_route_requires_unsupported_profile_disclosure() {
    let set = seeded_m5_community_handoff_target_sheet_set();
    let sheet = set
        .sheets
        .iter()
        .find(|s| s.trust_class == DestinationTrustClass::PrivateSecurity)
        .expect("security route present");
    assert!(sheet.unsupported_profile_disclosure_required);
}

#[test]
fn route_targeting_wrong_trust_fails() {
    let mut set = seeded_m5_community_handoff_target_sheet_set();
    // Force the public-issue route onto a private/security trust class.
    let sheet = set
        .sheets
        .iter_mut()
        .find(|s| s.route_class == CommunityHandoffRouteClass::PublicIssue)
        .expect("public issue route present");
    sheet.trust_class = DestinationTrustClass::PrivateSecurity;
    assert!(matches!(
        set.validate(),
        Err(CommunityHandoffTargetError::RouteTrustMismatch { .. })
    ));
}

#[test]
fn community_route_marked_guaranteed_fails() {
    let mut set = seeded_m5_community_handoff_target_sheet_set();
    let sheet = set
        .sheets
        .iter_mut()
        .find(|s| s.route_class == CommunityHandoffRouteClass::CommunitySupport)
        .expect("community support route present");
    sheet.commitment_honesty.guaranteed_product_commitment = true;
    assert!(matches!(
        set.validate(),
        Err(CommunityHandoffTargetError::CommitmentMasqueradesAsGuarantee { .. })
    ));
}

#[test]
fn world_readable_auto_open_fails() {
    let mut set = seeded_m5_community_handoff_target_sheet_set();
    let sheet = set
        .sheets
        .iter_mut()
        .find(|s| s.trust_class.is_world_readable())
        .expect("world-readable route present");
    sheet.auto_open_from_critical_alert_allowed = true;
    assert!(matches!(
        set.validate(),
        Err(CommunityHandoffTargetError::WorldReadableAutoOpens { .. })
    ));
}

#[test]
fn private_route_without_disclosure_fails() {
    let mut set = seeded_m5_community_handoff_target_sheet_set();
    let sheet = set
        .sheets
        .iter_mut()
        .find(|s| s.trust_class == DestinationTrustClass::PrivateSecurity)
        .expect("security route present");
    sheet.unsupported_profile_disclosure_required = false;
    assert!(matches!(
        set.validate(),
        Err(CommunityHandoffTargetError::PrivateRouteMissingDisclosure { .. })
    ));
}

#[test]
fn local_fallback_that_leaves_product_fails() {
    let mut set = seeded_m5_community_handoff_target_sheet_set();
    set.sheets[0].local_safe_fallback.data_exit_boundary =
        DataExitBoundary::MetadataSafeObjectRefs;
    assert!(matches!(
        set.validate(),
        Err(CommunityHandoffTargetError::LocalFallbackNotLocal { .. })
    ));
}

#[test]
fn missing_route_fails() {
    let mut set = seeded_m5_community_handoff_target_sheet_set();
    set.sheets
        .retain(|s| s.route_class != CommunityHandoffRouteClass::OfficialSupport);
    assert!(matches!(
        set.validate(),
        Err(CommunityHandoffTargetError::RouteMissing { .. })
    ));
}

#[test]
fn missing_source_contract_fails() {
    let mut set = seeded_m5_community_handoff_target_sheet_set();
    set.source_contract_refs
        .retain(|r| r != M5_COMMUNITY_HANDOFF_PUBLIC_MATRIX_REF);
    assert!(matches!(
        set.validate(),
        Err(CommunityHandoffTargetError::MissingSourceContracts)
    ));
}

#[test]
fn raw_ref_leak_fails() {
    let mut set = seeded_m5_community_handoff_target_sheet_set();
    set.sheets[0].destination_identity_ref = "https://example.com/issues".to_owned();
    assert!(matches!(
        set.validate(),
        Err(CommunityHandoffTargetError::RawRefLeak { .. })
            | Err(CommunityHandoffTargetError::RawMaterialInExport)
    ));
}

#[test]
fn duplicate_target_id_fails() {
    let mut set = seeded_m5_community_handoff_target_sheet_set();
    let dup = set.sheets[0].clone();
    set.sheets.push(dup);
    assert!(matches!(
        set.validate(),
        Err(CommunityHandoffTargetError::DuplicateTargetId { .. })
            | Err(CommunityHandoffTargetError::RouteMissing { .. })
    ));
}

#[test]
fn matrix_csv_has_a_row_per_route() {
    let set = seeded_m5_community_handoff_target_sheet_set();
    let csv = set.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + CommunityHandoffRouteClass::ALL.len());
    assert!(lines[0].starts_with("route,trust_class,"));
    for route in CommunityHandoffRouteClass::ALL {
        assert!(csv.contains(route.as_str()), "csv missing {}", route.as_str());
    }
}

#[test]
fn markdown_summary_lists_every_route() {
    let summary = seeded_m5_community_handoff_target_sheet_set().render_markdown_summary();
    for route in CommunityHandoffRouteClass::ALL {
        assert!(
            summary.contains(route.as_str()),
            "summary missing {}",
            route.as_str()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_community_handoff_target_sheet_set().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}

#[test]
fn narrowed_fixture_sheets_validate() {
    for sheet in [
        seeded_security_disclosure_sheet_unsupported_profile(),
        seeded_community_support_sheet_no_commitment(),
    ] {
        assert!(sheet.validate().is_ok(), "{:?}", sheet.validate());
    }
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_community_handoff_target_set()
        .expect("checked community-handoff target set validates");
    assert_eq!(
        from_disk,
        seeded_m5_community_handoff_target_sheet_set(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_match_seed_builders() {
    let unsupported: CommunityHandoffTargetSheet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/community-handoff/security_disclosure_local_safe_fallback.json"
    )))
    .expect("unsupported-profile fixture parses");
    assert_eq!(
        unsupported,
        seeded_security_disclosure_sheet_unsupported_profile()
    );

    let no_commitment: CommunityHandoffTargetSheet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/community-handoff/community_support_not_a_commitment.json"
    )))
    .expect("no-commitment fixture parses");
    assert_eq!(no_commitment, seeded_community_support_sheet_no_commitment());
}
