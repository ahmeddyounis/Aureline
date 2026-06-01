use super::*;

use aureline_auth::network_trust::seeded_network_trust_beta_page;

// ---------------------------------------------------------------------------
// Seeded page invariants
// ---------------------------------------------------------------------------

#[test]
fn seeded_page_qualifies_stable() {
    let page = seeded_harden_enterprise_network_proxy_page();
    assert!(
        page.qualifies_stable(),
        "seeded page must qualify stable; got '{}'; defects: {:?}",
        page.summary.overall_qualification_token,
        page.defects
    );
}

#[test]
fn seeded_page_has_zero_defects() {
    let page = seeded_harden_enterprise_network_proxy_page();
    assert!(
        page.defects.is_empty(),
        "seeded page must have zero defects; got: {:?}",
        page.defects
    );
}

#[test]
fn seeded_page_has_six_rows() {
    let page = seeded_harden_enterprise_network_proxy_page();
    assert_eq!(
        page.rows.len(),
        6,
        "seeded page must have 6 rows (one per proxy route)"
    );
}

#[test]
fn seeded_page_covers_all_routes() {
    let page = seeded_harden_enterprise_network_proxy_page();
    assert!(
        page.covers_all_required_routes(),
        "seeded page must cover all six required proxy routes"
    );
}

#[test]
fn seeded_page_all_rows_are_stable() {
    let page = seeded_harden_enterprise_network_proxy_page();
    for row in &page.rows {
        assert_eq!(
            row.qualification_token, "stable",
            "row '{}' must qualify stable in the seeded page",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_all_rows_exclude_raw_secret_material() {
    let page = seeded_harden_enterprise_network_proxy_page();
    assert!(
        page.all_rows_exclude_raw_secret_material(),
        "all seeded rows must exclude raw secret or private-key material"
    );
}

#[test]
fn seeded_page_all_required_fallbacks_declared() {
    let page = seeded_harden_enterprise_network_proxy_page();
    assert!(
        page.all_required_fallbacks_declared(),
        "all enterprise-bearing routes must declare a local-only fallback route"
    );
}

#[test]
fn seeded_page_all_rows_have_selector_reasons() {
    let page = seeded_harden_enterprise_network_proxy_page();
    assert!(
        page.all_rows_have_selector_reasons(),
        "all seeded rows must carry an explicit selector reason"
    );
}

#[test]
fn seeded_page_policy_pinned_row_has_managed_attribution() {
    let page = seeded_harden_enterprise_network_proxy_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.proxy_route == ProxyRouteClass::PolicyPinned)
        .expect("seeded page must have a policy_pinned row");
    assert!(
        !row.managed_attribution_ref.is_empty(),
        "policy_pinned row must carry a managed_attribution_ref"
    );
}

#[test]
fn seeded_page_mirror_only_row_has_managed_attribution() {
    let page = seeded_harden_enterprise_network_proxy_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.proxy_route == ProxyRouteClass::MirrorOnly)
        .expect("seeded page must have a mirror_only row");
    assert!(
        !row.managed_attribution_ref.is_empty(),
        "mirror_only row must carry a managed_attribution_ref"
    );
}

#[test]
fn seeded_page_mirror_only_and_offline_rows_explicit_on_local_core_continuity() {
    let page = seeded_harden_enterprise_network_proxy_page();
    for row in page
        .rows
        .iter()
        .filter(|r| matches!(r.proxy_route, ProxyRouteClass::MirrorOnly | ProxyRouteClass::Offline))
    {
        assert!(
            row.local_core_continuity_explicit,
            "row '{}' must carry local_core_continuity_explicit: true",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_policy_pinned_row_uses_custom_ca() {
    let page = seeded_harden_enterprise_network_proxy_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.proxy_route == ProxyRouteClass::PolicyPinned)
        .expect("seeded page must have a policy_pinned row");
    let has_custom_ca = row
        .bootstrap_credentials
        .iter()
        .any(|c| c.credential_kind == BootstrapCredentialKind::CustomCa);
    assert!(
        has_custom_ca,
        "policy_pinned row must list a custom_ca bootstrap credential"
    );
}

#[test]
fn seeded_page_policy_pinned_row_uses_client_certificate() {
    let page = seeded_harden_enterprise_network_proxy_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.proxy_route == ProxyRouteClass::PolicyPinned)
        .expect("seeded page must have a policy_pinned row");
    let has_client_cert = row
        .bootstrap_credentials
        .iter()
        .any(|c| c.credential_kind == BootstrapCredentialKind::ClientCertificate);
    assert!(
        has_client_cert,
        "policy_pinned row must list a client_certificate bootstrap credential"
    );
    assert_eq!(
        row.client_cert_posture,
        RouteClientCertPostureClass::PresentValid,
        "policy_pinned row client_cert_posture must be present_valid"
    );
}

#[test]
fn seeded_page_offline_row_has_no_required_credentials() {
    let page = seeded_harden_enterprise_network_proxy_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.proxy_route == ProxyRouteClass::Offline)
        .expect("seeded page must have an offline row");
    let has_none_required = row
        .bootstrap_credentials
        .iter()
        .any(|c| c.credential_kind == BootstrapCredentialKind::NoneRequired);
    assert!(
        has_none_required,
        "offline row must list none_required bootstrap credential"
    );
    assert_eq!(
        row.tls_verification_posture,
        TlsVerificationPostureClass::NotApplicable,
        "offline row TLS posture must be not_applicable"
    );
}

#[test]
fn seeded_page_all_rows_have_non_empty_precedence_rank_label() {
    let page = seeded_harden_enterprise_network_proxy_page();
    for row in &page.rows {
        assert!(
            !row.precedence_rank_label.is_empty(),
            "row '{}' must carry a non-empty precedence_rank_label",
            row.row_id
        );
    }
}

// ---------------------------------------------------------------------------
// Audit: raw secret material triggers withdrawal
// ---------------------------------------------------------------------------

#[test]
fn raw_secret_in_row_triggers_withdrawal() {
    let network_trust_page = seeded_network_trust_beta_page();
    let mut rows = vec![row_system()];
    rows[0].raw_secret_or_private_material_excluded = false;
    let page = HardenEnterpriseNetworkProxyPage::new(
        "test:withdrawal:raw-secret",
        "Test: raw secret material in row",
        "2026-06-01T00:00:00Z",
        rows,
        network_trust_page,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        HardenEnterpriseNetworkProxyQualificationClass::Withdrawn.as_str(),
        "raw secret in row must withdraw the packet"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == HardenEnterpriseNetworkProxyNarrowReasonClass::RawSecretOrPrivateMaterialExposed
        }),
        "defects must include raw_secret_or_private_material_exposed"
    );
}

// ---------------------------------------------------------------------------
// Audit: missing route triggers preview narrowing
// ---------------------------------------------------------------------------

#[test]
fn missing_route_narrows_to_preview() {
    let network_trust_page = seeded_network_trust_beta_page();
    // Only include system and pac rows — missing manual, policy_pinned, mirror_only, offline.
    let rows = vec![row_system(), row_pac()];
    let page = HardenEnterpriseNetworkProxyPage::new(
        "test:preview:missing-route",
        "Test: required proxy route missing",
        "2026-06-01T00:00:00Z",
        rows,
        network_trust_page,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        HardenEnterpriseNetworkProxyQualificationClass::Preview.as_str(),
        "missing required route must narrow to preview"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == HardenEnterpriseNetworkProxyNarrowReasonClass::MissingRouteCoverage
        }),
        "defects must include missing_route_coverage"
    );
}

// ---------------------------------------------------------------------------
// Audit: empty selector reason narrows to beta
// ---------------------------------------------------------------------------

#[test]
fn empty_selector_reason_narrows_to_beta() {
    let network_trust_page = seeded_network_trust_beta_page();
    let mut rows = seeded_rows();
    if let Some(row) = rows.first_mut() {
        row.selector_reason_token.clear();
    }
    let page = HardenEnterpriseNetworkProxyPage::new(
        "test:beta:empty-selector-reason",
        "Test: empty selector reason",
        "2026-06-01T00:00:00Z",
        rows,
        network_trust_page,
    );
    assert!(
        page.summary.overall_qualification_token
            == HardenEnterpriseNetworkProxyQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == HardenEnterpriseNetworkProxyQualificationClass::Withdrawn.as_str(),
        "empty selector reason must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == HardenEnterpriseNetworkProxyNarrowReasonClass::EmptySelectorReason
        }),
        "defects must include empty_selector_reason"
    );
}

// ---------------------------------------------------------------------------
// Audit: missing local fallback on enterprise route narrows to beta
// ---------------------------------------------------------------------------

#[test]
fn missing_local_fallback_on_enterprise_route_narrows_to_beta() {
    let network_trust_page = seeded_network_trust_beta_page();
    let mut rows = seeded_rows();
    if let Some(row) = rows
        .iter_mut()
        .find(|r| r.proxy_route == ProxyRouteClass::PolicyPinned)
    {
        row.local_only_fallback_route_token.clear();
    }
    let page = HardenEnterpriseNetworkProxyPage::new(
        "test:beta:missing-fallback",
        "Test: missing local fallback on policy_pinned route",
        "2026-06-01T00:00:00Z",
        rows,
        network_trust_page,
    );
    assert!(
        page.summary.overall_qualification_token
            == HardenEnterpriseNetworkProxyQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == HardenEnterpriseNetworkProxyQualificationClass::Withdrawn.as_str(),
        "missing local fallback on enterprise route must narrow to at least beta"
    );
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == HardenEnterpriseNetworkProxyNarrowReasonClass::EmptyLocalFallback),
        "defects must include empty_local_fallback"
    );
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

#[test]
fn support_export_always_excludes_raw_private_material() {
    let page = seeded_harden_enterprise_network_proxy_page();
    let export = HardenEnterpriseNetworkProxySupportExport::from_page(
        "export:test:001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(
        export.raw_secret_or_private_material_excluded,
        "support export must always report raw_secret_or_private_material_excluded: true"
    );
}

#[test]
fn support_export_has_correct_record_kind() {
    let page = seeded_harden_enterprise_network_proxy_page();
    let export = HardenEnterpriseNetworkProxySupportExport::from_page(
        "export:test:002",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert_eq!(
        export.record_kind,
        HARDEN_ENTERPRISE_NETWORK_PROXY_SUPPORT_EXPORT_RECORD_KIND
    );
}

// ---------------------------------------------------------------------------
// Re-audit helpers are consistent
// ---------------------------------------------------------------------------

#[test]
fn validate_returns_ok_for_seeded_page() {
    let page = seeded_harden_enterprise_network_proxy_page();
    let result = validate_harden_enterprise_network_proxy_page(&page);
    assert!(
        result.is_ok(),
        "validate must return Ok for the seeded page; defects: {:?}",
        result.err()
    );
}

#[test]
fn audit_returns_zero_defects_for_seeded_page() {
    let page = seeded_harden_enterprise_network_proxy_page();
    let defects = audit_harden_enterprise_network_proxy_page(&page);
    assert!(
        defects.is_empty(),
        "re-audit of seeded page must return zero defects; got: {:?}",
        defects
    );
}

// ---------------------------------------------------------------------------
// Precedence model invariants
// ---------------------------------------------------------------------------

#[test]
fn policy_pinned_has_highest_precedence_rank() {
    let page = seeded_harden_enterprise_network_proxy_page();
    let pinned = page
        .rows
        .iter()
        .find(|r| r.proxy_route == ProxyRouteClass::PolicyPinned)
        .expect("must have policy_pinned row");
    for other in page
        .rows
        .iter()
        .filter(|r| r.proxy_route != ProxyRouteClass::PolicyPinned)
    {
        assert!(
            pinned.precedence_rank <= other.precedence_rank,
            "policy_pinned row must have the highest (lowest-numbered) precedence rank; \
             got {} vs {} for route '{}'",
            pinned.precedence_rank,
            other.precedence_rank,
            other.proxy_route_token
        );
    }
}
