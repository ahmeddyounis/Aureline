//! Inline unit tests for the M5 embedded-boundary qualification audit.

use std::collections::BTreeSet;

use super::*;

#[test]
fn seeded_audit_passes_validation() {
    let report = seeded_m5_embedded_boundaries_audit();
    validate_m5_embedded_boundaries(&report).expect("seeded audit must validate");
}

#[test]
fn seeded_audit_qualifies_every_required_guarantee() {
    let report = seeded_m5_embedded_boundaries_audit();
    assert!(report.every_required_guarantee_qualified());
}

#[test]
fn seeded_audit_has_no_blocking_findings() {
    let report = seeded_m5_embedded_boundaries_audit();
    assert!(report.report_clean);
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    assert!(report.narrowable_marketed_rows.is_empty());
    for surface in &report.rows {
        assert!(
            surface.blocking_findings.is_empty(),
            "surface {} carried blocking findings: {:?}",
            surface.descriptor.surface_id,
            surface.blocking_findings
        );
    }
}

#[test]
fn seeded_audit_covers_every_embedded_surface() {
    let report = seeded_m5_embedded_boundaries_audit();
    let surfaces: BTreeSet<M5EmbeddedSurface> = report
        .rows
        .iter()
        .map(|surface| surface.descriptor.embedded_surface)
        .collect();
    for expected in [
        M5EmbeddedSurface::EmbeddedDocs,
        M5EmbeddedSurface::RequestRuntimeViewer,
        M5EmbeddedSurface::PreviewRoutePane,
        M5EmbeddedSurface::MarketplaceAccount,
        M5EmbeddedSurface::HelpCenterPane,
        M5EmbeddedSurface::ProviderReviewPane,
        M5EmbeddedSurface::CompanionBrowserHandoff,
        M5EmbeddedSurface::ProviderConsoleHandoff,
    ] {
        assert!(
            surfaces.contains(&expected),
            "embedded surface {} is not registered",
            expected.as_str()
        );
    }
}

#[test]
fn seeded_audit_qualifies_every_boundary_guarantee() {
    let report = seeded_m5_embedded_boundaries_audit();
    for guarantee in M5BoundaryGuarantee::required_guarantees() {
        let coverage = report
            .guarantee_coverage
            .iter()
            .find(|coverage| coverage.guarantee == guarantee)
            .expect("every required guarantee has a coverage summary");
        assert!(
            coverage.qualified_rows > 0,
            "guarantee {} must be qualified by at least one surface",
            guarantee.as_str()
        );
    }
}

#[test]
fn seeded_audit_covers_every_boundary_class() {
    let report = seeded_m5_embedded_boundaries_audit();
    let classes: BTreeSet<M5BoundaryClass> = report
        .rows
        .iter()
        .map(|surface| surface.descriptor.boundary_class)
        .collect();
    for expected in [
        M5BoundaryClass::FirstPartyLocal,
        M5BoundaryClass::EmbeddedWebview,
        M5BoundaryClass::ProviderOwned,
        M5BoundaryClass::ExternalHandoff,
    ] {
        assert!(
            classes.contains(&expected),
            "boundary class {} is not represented",
            expected.as_str()
        );
    }
}

#[test]
fn high_stakes_surfaces_project_a_return_anchor_on_qualified_rows() {
    let report = seeded_m5_embedded_boundaries_audit();
    assert!(report.high_stakes_surface_count > 0);
    for surface in &report.rows {
        if !surface.high_stakes {
            continue;
        }
        for binding in &surface.bindings {
            if binding.qualification_status == M5BoundaryStatus::Qualified {
                assert!(
                    binding.projected_return_anchor.is_some(),
                    "high-stakes surface {} must project a return anchor on {}",
                    surface.descriptor.surface_id,
                    binding.guarantee.as_str()
                );
            }
        }
    }
}

#[test]
fn return_anchor_index_covers_every_surface() {
    let report = seeded_m5_embedded_boundaries_audit();
    assert_eq!(report.return_anchor_index.len(), report.rows.len());
    for surface in &report.rows {
        let entry = report
            .return_anchor_index
            .iter()
            .find(|entry| entry.surface_id == surface.descriptor.surface_id)
            .expect("every surface must have a return-anchor entry");
        assert_eq!(
            entry.return_anchor_ref,
            surface.descriptor.return_anchor_ref
        );
        assert!(!entry.return_anchor_ref.is_empty());
    }
}

#[test]
fn high_stakes_surfaces_expose_boundary_chrome() {
    let report = seeded_m5_embedded_boundaries_audit();
    for surface in &report.rows {
        if surface.high_stakes {
            assert!(
                !surface.descriptor.boundary_chrome.is_empty(),
                "high-stakes surface {} must expose boundary chrome",
                surface.descriptor.surface_id
            );
        }
    }
}

#[test]
fn every_marketed_surface_declares_a_handoff_target() {
    let report = seeded_m5_embedded_boundaries_audit();
    for surface in &report.rows {
        if surface.marketed {
            assert!(
                !surface.descriptor.handoff_targets.is_empty(),
                "marketed surface {} must declare a handoff target",
                surface.descriptor.surface_id
            );
        }
    }
}

/// A minimal high-stakes descriptor used to exercise binding findings.
fn high_stakes_descriptor() -> M5EmbeddedSurfaceDescriptor {
    M5EmbeddedSurfaceDescriptor {
        surface_id: "embedded:marketplace_account".to_owned(),
        embedded_surface: M5EmbeddedSurface::MarketplaceAccount,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        return_anchor_ref: "embedded:return:marketplace_account".to_owned(),
        support_note: "note".to_owned(),
        boundary_class: M5BoundaryClass::ProviderOwned,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        boundary_chrome: vec![M5BoundaryChrome::OwnerBadge, M5BoundaryChrome::OriginLabel],
        handoff_targets: vec![M5HandoffTarget::SystemBrowser],
        marketed_on_desktop: true,
        routed_through_governed_boundary: true,
    }
}

/// A fully-projected qualified binding, ready to be mutated into a red result by
/// individual tests.
fn qualified_binding(guarantee: M5BoundaryGuarantee) -> M5BoundaryBinding {
    M5BoundaryBinding {
        guarantee,
        aspect: guarantee.canonical_aspect(),
        qualification_status: M5BoundaryStatus::Qualified,
        marketed_on_guarantee: true,
        projected_descriptor_ref: Some(
            "destination-descriptor:embedded:marketplace_account:owner_origin_disclosure"
                .to_owned(),
        ),
        projected_boundary_class: Some(M5BoundaryClass::ProviderOwned),
        projected_owner_origin: Some(M5OwnerOriginDisclosure::OwnerOriginDisclosed),
        projected_freshness: guarantee
            .requires_freshness_disclosure()
            .then_some(M5FreshnessDisclosure::FreshnessShown),
        projected_trust_chrome: guarantee
            .requires_trust_chrome()
            .then_some(M5TrustChrome::BoundedAttributed),
        projected_auth_channel: guarantee
            .requires_auth_channel()
            .then_some(M5AuthChannel::SystemBrowserDefault),
        projected_high_risk_handling: guarantee
            .requires_high_risk_handling()
            .then_some(M5HighRiskHandling::BlockedOrRouted),
        projected_return_anchor: Some(M5ReturnAnchorOutcome::ExactReturnResolved),
        projected_handoff_reason: guarantee
            .requires_handoff_reason()
            .then_some(M5HandoffReasonOutcome::ReasonPreserved),
        projected_support_parity: guarantee
            .requires_support_parity()
            .then_some(M5SupportParity::SameDescriptorReused),
        evidence_freshness: Some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: Some(GENERATED_AT.to_owned()),
        narrowing_reason: None,
        note: None,
    }
}

#[test]
fn unqualified_local_surface_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5BoundaryGuarantee::OwnerOriginDisclosure);
    binding.qualification_status = M5BoundaryStatus::UnqualifiedLocalSurface;
    let surface = build_m5_embedded_boundary_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5BoundaryBlockingFinding::UnqualifiedLocalSurface { .. })));
}

#[test]
fn missing_evidence_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5BoundaryGuarantee::OwnerOriginDisclosure);
    binding.qualification_status = M5BoundaryStatus::MissingEvidence;
    let surface = build_m5_embedded_boundary_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5BoundaryBlockingFinding::MissingEvidence { .. })));
}

#[test]
fn owner_origin_hidden_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5BoundaryGuarantee::OwnerOriginDisclosure);
    binding.projected_owner_origin = Some(M5OwnerOriginDisclosure::OwnerOriginHidden);
    let surface = build_m5_embedded_boundary_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5BoundaryBlockingFinding::OwnerOriginHidden { .. })));
}

#[test]
fn freshness_hidden_and_pretends_first_party_block() {
    let descriptor = high_stakes_descriptor();
    let mut fresh = qualified_binding(M5BoundaryGuarantee::FreshnessDisclosure);
    fresh.projected_freshness = Some(M5FreshnessDisclosure::FreshnessHidden);
    let mut trust = qualified_binding(M5BoundaryGuarantee::TrustBoundaryChrome);
    trust.projected_trust_chrome = Some(M5TrustChrome::PretendsFirstParty);
    let surface = build_m5_embedded_boundary_row(descriptor, vec![fresh, trust]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("freshness_hidden"));
    assert!(tokens.contains("pretends_first_party"));
}

#[test]
fn embedded_primary_auth_and_high_risk_approval_block() {
    let descriptor = high_stakes_descriptor();
    let mut auth = qualified_binding(M5BoundaryGuarantee::SystemBrowserAuthDefault);
    auth.projected_auth_channel = Some(M5AuthChannel::EmbeddedPrimaryApproval);
    let mut risk = qualified_binding(M5BoundaryGuarantee::NoEmbeddedHighRiskApproval);
    risk.projected_high_risk_handling = Some(M5HighRiskHandling::EmbeddedApprovalHidden);
    let surface = build_m5_embedded_boundary_row(descriptor, vec![auth, risk]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("embedded_primary_auth"));
    assert!(tokens.contains("embedded_high_risk_approval"));
}

#[test]
fn return_lost_and_handoff_reason_dropped_block() {
    let descriptor = high_stakes_descriptor();
    let mut ret = qualified_binding(M5BoundaryGuarantee::ReturnAnchorPresent);
    ret.projected_return_anchor = Some(M5ReturnAnchorOutcome::ReturnLost);
    let mut handoff = qualified_binding(M5BoundaryGuarantee::HandoffReasonPreserved);
    handoff.projected_handoff_reason = Some(M5HandoffReasonOutcome::ReasonDropped);
    let surface = build_m5_embedded_boundary_row(descriptor, vec![ret, handoff]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("return_anchor_lost"));
    assert!(tokens.contains("handoff_reason_dropped"));
}

#[test]
fn support_parity_divergent_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5BoundaryGuarantee::SupportExportParity);
    binding.projected_support_parity = Some(M5SupportParity::DivergentClone);
    let surface = build_m5_embedded_boundary_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5BoundaryBlockingFinding::SupportParityDivergent { .. })));
}

#[test]
fn stale_evidence_on_marketed_row_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5BoundaryGuarantee::OwnerOriginDisclosure);
    binding.evidence_freshness = Some(M5EvidenceFreshness::Stale);
    let surface = build_m5_embedded_boundary_row(descriptor, vec![binding]);
    assert!(surface.blocking_findings.iter().any(|f| matches!(
        f,
        M5BoundaryBlockingFinding::StaleEvidenceOnMarketedRow { .. }
    )));
}

#[test]
fn aspect_drift_blocks() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5BoundaryGuarantee::OwnerOriginDisclosure);
    binding.aspect = M5BoundaryAspect::Handoff;
    let surface = build_m5_embedded_boundary_row(descriptor, vec![binding]);
    assert!(surface
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5BoundaryBlockingFinding::AspectDrift { .. })));
}

#[test]
fn missing_projection_blocks_on_qualified_row() {
    let descriptor = high_stakes_descriptor();
    let mut binding = qualified_binding(M5BoundaryGuarantee::OwnerOriginDisclosure);
    binding.projected_descriptor_ref = None;
    let surface = build_m5_embedded_boundary_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("missing_projection"));
    assert!(tokens.contains("missing_descriptor_ref"));
}

#[test]
fn descriptor_level_findings_fire() {
    let descriptor = M5EmbeddedSurfaceDescriptor {
        surface_id: "embedded:marketplace_account".to_owned(),
        embedded_surface: M5EmbeddedSurface::MarketplaceAccount,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        return_anchor_ref: "  ".to_owned(),
        support_note: "  ".to_owned(),
        boundary_class: M5BoundaryClass::ProviderOwned,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        boundary_chrome: vec![],
        handoff_targets: vec![],
        marketed_on_desktop: true,
        routed_through_governed_boundary: false,
    };
    let surface = build_m5_embedded_boundary_row(descriptor, vec![]);
    let tokens: BTreeSet<&str> = surface
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("descriptor_missing_return_anchor"));
    assert!(tokens.contains("missing_support_note"));
    assert!(tokens.contains("surface_not_on_governed_boundary"));
    assert!(tokens.contains("missing_boundary_chrome"));
    assert!(tokens.contains("no_declared_handoff_target"));
}

#[test]
fn missing_narrowing_reason_blocks() {
    let mut report = seeded_m5_embedded_boundaries_audit();
    let surface = report
        .rows
        .iter_mut()
        .find(|surface| surface.descriptor.surface_id == "embedded:provider_console_handoff")
        .expect("provider console handoff surface present");
    let binding = surface
        .bindings
        .iter_mut()
        .find(|binding| binding.qualification_status == M5BoundaryStatus::DeclaredCaptureGap)
        .expect("declared capture gap binding present");
    binding.narrowing_reason = None;
    let rebuilt =
        build_m5_embedded_boundary_row(surface.descriptor.clone(), surface.bindings.clone());
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5BoundaryBlockingFinding::MissingNarrowingReason { .. })));
}

#[test]
fn missing_required_guarantee_blocks_validation() {
    let mut report = seeded_m5_embedded_boundaries_audit();
    report.rows[0]
        .bindings
        .retain(|binding| binding.guarantee != M5BoundaryGuarantee::OwnerOriginDisclosure);
    let result = validate_m5_embedded_boundaries(&report);
    assert!(result.is_err());
}

#[test]
fn narrowable_rows_surface_a_blocking_marketed_row() {
    let report = seeded_m5_embedded_boundaries_audit();
    let mut surfaces = report.rows.clone();
    let surface = surfaces
        .iter_mut()
        .find(|surface| surface.descriptor.surface_id == "embedded:marketplace_account")
        .expect("marketplace surface present");
    let mut bindings = surface.bindings.clone();
    let binding = bindings
        .iter_mut()
        .find(|binding| binding.guarantee == M5BoundaryGuarantee::SystemBrowserAuthDefault)
        .expect("system-browser auth binding present");
    binding.projected_auth_channel = Some(M5AuthChannel::EmbeddedPrimaryApproval);
    *surface = build_m5_embedded_boundary_row(surface.descriptor.clone(), bindings);
    let rebuilt = build_m5_embedded_boundaries_audit(surfaces);
    assert!(!rebuilt.report_clean);
    assert!(rebuilt.narrowable_marketed_rows.iter().any(|narrowable| {
        narrowable.surface_id == "embedded:marketplace_account"
            && narrowable.guarantee == M5BoundaryGuarantee::SystemBrowserAuthDefault
    }));
}

#[test]
fn support_export_quotes_every_surface_id() {
    let report = seeded_m5_embedded_boundaries_audit();
    let export =
        M5EmbeddedBoundarySupportExport::from_report(M5_EMBEDDED_SUPPORT_EXPORT_ID, report.clone());
    assert!(export.case_ids.contains(&report.report_id));
    for surface in &report.rows {
        assert!(export.case_ids.contains(&surface.descriptor.surface_id));
        assert!(export
            .case_ids
            .contains(&surface.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn render_markdown_is_deterministic() {
    let report = seeded_m5_embedded_boundaries_audit();
    assert_eq!(report.render_markdown(), report.render_markdown());
}
