use super::*;

fn packet() -> ExportSafeDependencyReports {
    current_export_safe_dependency_reports().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        EXPORT_SAFE_DEPENDENCY_REPORTS_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        EXPORT_SAFE_DEPENDENCY_REPORTS_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn packet_has_report_context() {
    let packet = packet();
    assert!(!packet.report_context.build_id.is_empty());
    assert!(!packet.report_context.workspace_scope_ref.is_empty());
    assert!(!packet.report_context.lockfile_fingerprint.is_empty());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn export_projection_includes_all_rows() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.can_claim_clean, packet.can_claim_clean());
}

#[test]
fn report_kinds_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ReportKind> = packet.rows.iter().map(|r| r.report_kind).collect();
    for kind in ReportKind::ALL {
        assert!(
            present.contains(&kind),
            "missing report kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn claim_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ClaimClass> = packet.rows.iter().map(|r| r.claim_class).collect();
    for claim in ClaimClass::ALL {
        assert!(
            present.contains(&claim),
            "missing claim class {}",
            claim.as_str()
        );
    }
}

#[test]
fn source_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SourceClass> = packet.rows.iter().map(|r| r.source_class).collect();
    for source in SourceClass::ALL {
        assert!(
            present.contains(&source),
            "missing source class {}",
            source.as_str()
        );
    }
}

#[test]
fn declares_an_sbom_capable_export_format() {
    let packet = packet();
    assert!(packet.has_sbom_export());
}

#[test]
fn verified_claims_require_local_current_source() {
    assert!(ClaimClass::Verified.permitted_for(SourceClass::LocalAnalysis, FreshnessClass::Current));
    assert!(!ClaimClass::Verified.permitted_for(SourceClass::ImportedFeed, FreshnessClass::Current));
    assert!(!ClaimClass::Verified.permitted_for(SourceClass::LocalAnalysis, FreshnessClass::Stale));
    assert!(
        ClaimClass::Mirrored.permitted_for(SourceClass::EnterpriseMirror, FreshnessClass::Stale)
    );
    assert!(
        !ClaimClass::Mirrored.permitted_for(SourceClass::LocalAnalysis, FreshnessClass::Current)
    );
}

#[test]
fn clean_claim_requires_online_and_genuinely_empty() {
    let disclosure = ConnectivityDisclosure {
        connectivity_state: ConnectivityState::Online,
        empty_result_reason: EmptyResultReason::GenuinelyEmpty,
        last_known_good_at: None,
        evidence_refs: Vec::new(),
        note: "online".to_owned(),
    };
    assert!(disclosure.can_claim_clean());

    let mirror = ConnectivityDisclosure {
        connectivity_state: ConnectivityState::MirrorOnly,
        empty_result_reason: EmptyResultReason::MirrorStale,
        last_known_good_at: Some("2026-06-01T00:00:00Z".to_owned()),
        evidence_refs: Vec::new(),
        note: "mirror".to_owned(),
    };
    assert!(!mirror.can_claim_clean());
}

#[test]
fn validate_flags_overstated_claim() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.claim_class != ClaimClass::Verified)
    {
        row.claim_class = ClaimClass::Verified;
        row.source_class = SourceClass::ImportedFeed;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ExportSafeDependencyReportsViolation::OverstatedClaim { .. }
        )));
    }
}

#[test]
fn validate_flags_undeclared_export_format() {
    let mut packet = packet();
    assert!(
        packet.rows.iter().any(|r| !r.export_formats.is_empty()),
        "fixture must exercise per-row export formats"
    );
    // Drop every export-format descriptor so any row reference dangles.
    packet.export_formats.clear();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExportSafeDependencyReportsViolation::UndeclaredExportFormat { .. }
    )));
}

#[test]
fn validate_flags_secret_leak_risk() {
    let mut packet = packet();
    if let Some(descriptor) = packet.export_formats.first_mut() {
        descriptor.redacts_secrets = false;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ExportSafeDependencyReportsViolation::SecretLeakRisk { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_last_known_good() {
    let mut packet = packet();
    packet.connectivity.connectivity_state = ConnectivityState::AirGapped;
    packet.connectivity.last_known_good_at = None;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ExportSafeDependencyReportsViolation::MissingLastKnownGood { .. }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_rows = packet.summary.total_rows.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&ExportSafeDependencyReportsViolation::SummaryMismatch));
}

#[test]
fn report_kind_tokens_are_stable() {
    assert_eq!(ReportKind::Advisory.as_str(), "advisory");
    assert_eq!(ReportKind::Vulnerability.as_str(), "vulnerability");
    assert_eq!(ReportKind::License.as_str(), "license");
    assert_eq!(ReportKind::Notice.as_str(), "notice");
    assert_eq!(ReportKind::Sbom.as_str(), "sbom");
}

#[test]
fn empty_result_reason_distinguishes_clean_from_degraded() {
    assert!(EmptyResultReason::GenuinelyEmpty.can_claim_clean());
    assert!(!EmptyResultReason::MirrorStale.can_claim_clean());
    assert!(!EmptyResultReason::AuthRequired.can_claim_clean());
    assert!(!EmptyResultReason::SnapshotOnly.can_claim_clean());
    assert!(!EmptyResultReason::FeedUnreachable.can_claim_clean());
}
