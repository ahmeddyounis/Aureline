//! Fixture-driven tests for the install ownership-audit contract.

use std::path::{Path, PathBuf};

use aureline_install::{
    ChannelLayoutClass, DeepLinkRouteCheckClass, HandoffSurfaceClass, ManagedOwnershipClaim,
    OwnerVerdictClass, OwnershipAuditPacket, PortableOwnershipClaim, SideBySideDisclosureClass,
    OWNERSHIP_AUDIT_PACKET_RECORD_KIND, OWNERSHIP_AUDIT_SCHEMA_VERSION,
    OWNERSHIP_AUDIT_SHARED_CONTRACT_REF, OWNERSHIP_AUDIT_SUPPORT_EXPORT_RECORD_KIND,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/install/ownership_audit/ownership_audit_packet.json")
}

fn load_packet() -> OwnershipAuditPacket {
    let bytes = std::fs::read(fixture_path()).expect("read ownership audit fixture");
    serde_json::from_slice(&bytes).expect("parse ownership audit fixture")
}

#[test]
fn ownership_audit_fixture_passes_validation() {
    let packet = load_packet();
    assert_eq!(packet.record_kind, OWNERSHIP_AUDIT_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, OWNERSHIP_AUDIT_SCHEMA_VERSION);
    assert_eq!(
        packet.shared_contract_ref,
        OWNERSHIP_AUDIT_SHARED_CONTRACT_REF
    );

    let report = packet.validate();
    assert!(
        report.passed,
        "ownership audit fixture failed validation: {:#?}",
        report.findings
    );

    for required in [
        ChannelLayoutClass::StableAndPreviewSideBySide,
        ChannelLayoutClass::StableAndPortableBeside,
        ChannelLayoutClass::StableAndManagedBeside,
    ] {
        assert!(
            report.coverage.channel_layouts.contains(&required),
            "missing channel layout {required:?}"
        );
    }

    for required in [
        HandoffSurfaceClass::FileAssociation,
        HandoffSurfaceClass::ProtocolHandler,
        HandoffSurfaceClass::DefaultBrowserCallback,
        HandoffSurfaceClass::DeepLinkIntent,
    ] {
        assert!(
            report.coverage.handoff_surfaces.contains(&required),
            "missing handoff surface {required:?}"
        );
    }

    for required in [
        DeepLinkRouteCheckClass::OriginTrust,
        DeepLinkRouteCheckClass::ReviewedSheetPreview,
        DeepLinkRouteCheckClass::TargetScope,
    ] {
        assert!(
            report.coverage.deep_link_route_checks.contains(&required),
            "missing deep-link route check {required:?}"
        );
    }

    assert!(report.coverage.displaced_owner_diagnostic_covered);
}

#[test]
fn portable_rows_never_claim_ownership() {
    let packet = load_packet();
    let portable_rows: Vec<_> = packet.rows.iter().filter(|row| row.is_portable()).collect();
    assert!(
        !portable_rows.is_empty(),
        "fixture must cover portable rows"
    );

    for row in portable_rows {
        assert_eq!(row.owner_verdict, OwnerVerdictClass::NotRegistered);
        assert_eq!(
            row.portable_claim,
            PortableOwnershipClaim::NeverClaimsHostGlobalOwnership
        );
        assert!(row.silent_steal_blocked);
        assert!(row
            .side_by_side_disclosure
            .contains(&SideBySideDisclosureClass::PortableDoesNotStealInstalledOwnership));
    }
}

#[test]
fn managed_rows_disclose_admin_or_ring_owner() {
    let packet = load_packet();
    let managed_rows: Vec<_> = packet.rows.iter().filter(|row| row.is_managed()).collect();
    assert!(!managed_rows.is_empty(), "fixture must cover managed rows");

    for row in managed_rows {
        assert!(matches!(
            row.managed_claim,
            ManagedOwnershipClaim::AdminPolicyOwnsHandler
                | ManagedOwnershipClaim::ManagedRingOwnsHandler
                | ManagedOwnershipClaim::UserVisibleNotOverrideable
        ));
        assert!(matches!(
            row.owner_verdict,
            OwnerVerdictClass::AdminPolicyOwned
                | OwnerVerdictClass::ManagedFleetOwned
                | OwnerVerdictClass::NotRegistered
        ));
        assert!(row.silent_steal_blocked);
    }
}

#[test]
fn dispatching_rows_run_same_checks_as_in_product_invocation() {
    let packet = load_packet();
    for row in &packet.rows {
        if !row.handoff_surface_class.participates_in_route_admission() {
            continue;
        }
        assert!(
            row.in_product_invocation_uses_same_checks,
            "row {} must reuse in-product checks",
            row.audit_row_id
        );
        assert!(
            !row.deep_link_route_checks.is_empty(),
            "row {} must list deep-link route checks",
            row.audit_row_id
        );
        for required in [
            DeepLinkRouteCheckClass::OriginTrust,
            DeepLinkRouteCheckClass::ReviewedSheetPreview,
            DeepLinkRouteCheckClass::TargetScope,
            DeepLinkRouteCheckClass::SingleUseReplay,
            DeepLinkRouteCheckClass::HandlerOwnershipVerification,
        ] {
            assert!(
                row.deep_link_route_checks.contains(&required),
                "row {} missing {required:?}",
                row.audit_row_id
            );
        }
    }
}

#[test]
fn surface_projection_and_support_export_round_trip() {
    let packet = load_packet();
    let projection = packet.surface_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.rows.len(), packet.rows.len());

    let export = packet.support_export_projection();
    assert_eq!(
        export.record_kind,
        OWNERSHIP_AUDIT_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.schema_version, OWNERSHIP_AUDIT_SCHEMA_VERSION);
    assert_eq!(
        export.shared_contract_ref,
        OWNERSHIP_AUDIT_SHARED_CONTRACT_REF
    );
    assert_eq!(export.redaction_class, "metadata_only_no_paths_or_secrets");
    assert_eq!(export.projection.rows.len(), packet.rows.len());

    // Surface projection and support export carry the same per-row truth.
    for (lhs, rhs) in projection.rows.iter().zip(export.projection.rows.iter()) {
        assert_eq!(lhs.audit_row_id, rhs.audit_row_id);
        assert_eq!(lhs.owner_verdict, rhs.owner_verdict);
        assert_eq!(lhs.deep_link_route_checks, rhs.deep_link_route_checks);
        assert_eq!(
            lhs.in_product_invocation_uses_same_checks,
            rhs.in_product_invocation_uses_same_checks
        );
    }
}

#[test]
fn portable_host_global_claim_is_rejected() {
    let mut packet = load_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.is_portable())
        .expect("portable row");
    row.owner_verdict = OwnerVerdictClass::SelectedOwner;
    row.selected_owner_channel = Some(row.owning_channel_class);
    row.silent_steal_blocked = false;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "ownership_audit.row.portable_owner_verdict"));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "ownership_audit.row.portable_silent_steal"));
}

#[test]
fn dispatching_row_dropping_in_product_parity_is_rejected() {
    let mut packet = load_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.handoff_surface_class.participates_in_route_admission())
        .expect("dispatching row");
    row.in_product_invocation_uses_same_checks = false;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "ownership_audit.row.in_product_check_parity_missing"));
}

#[test]
fn coexisting_layout_dropping_disclosure_is_rejected() {
    let mut packet = load_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.channel_layout.is_coexisting())
        .expect("coexisting row");
    row.side_by_side_disclosure = vec![SideBySideDisclosureClass::NotApplicable];

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id == "ownership_audit.row.side_by_side_disclosure_missing"));
}

#[test]
fn displaced_owner_row_must_reference_diagnostic() {
    let mut packet = load_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.owner_verdict == OwnerVerdictClass::DisplacedOwner)
        .expect("displaced owner row");
    row.displaced_owner_diagnostic_ref = None;

    let report = packet.validate();
    assert!(!report.passed);
    assert!(report.findings.iter().any(
        |finding| finding.check_id == "ownership_audit.row.displaced_owner_diagnostic_missing"
    ));
}
