use super::*;

fn packet() -> M5InstallConfigAuthCertification {
    current_m5_install_config_auth_certification().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_INSTALL_CONFIG_AUTH_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        M5_INSTALL_CONFIG_AUTH_CERTIFICATION_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_profile_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.rows.len(), packet.profiles.len());
    for &profile in &packet.profiles {
        assert!(
            packet.row(profile).is_some(),
            "missing row for profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn every_row_qualifies_all_four_domains() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            row.covers_all_domains(),
            "row {} does not cover all domains",
            row.row_id
        );
        for domain in CertificationDomain::REQUIRED {
            let qual = row
                .domains
                .iter()
                .find(|d| d.domain == domain)
                .unwrap_or_else(|| panic!("row {} missing domain {}", row.row_id, domain.as_str()));
            assert_eq!(
                qual.source_packet_ref,
                qual.source_packet.contract_ref(),
                "row {} domain {} source ref drifted",
                row.row_id,
                domain.as_str()
            );
            assert_eq!(qual.source_packet.domain(), domain);
        }
    }
}

#[test]
fn every_row_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_rows_gate_consistent());
    for row in &packet.rows {
        assert_eq!(
            row.published_support,
            row.effective_support(),
            "row {} publishes beyond the gate",
            row.row_id
        );
        assert_eq!(
            row.narrow_reasons,
            row.computed_narrow_reasons(),
            "row {} narrow reasons diverge from the gate",
            row.row_id
        );
        assert_eq!(
            row.downgrade_path,
            row.computed_downgrade_path(),
            "row {} downgrade path diverges from the gate",
            row.row_id
        );
    }
}

#[test]
fn every_source_packet_is_aggregated() {
    let packet = packet();
    for source in SourcePacket::ALL {
        assert!(
            packet
                .rows
                .iter()
                .any(|r| r.domains.iter().any(|d| d.source_packet == source)),
            "source packet {} is aggregated by no domain row",
            source.as_str()
        );
    }
}

#[test]
fn the_gate_admits_and_narrows_in_every_direction() {
    let packet = packet();
    let s = &packet.summary;
    assert!(s.verified_rows >= 1, "no verified row");
    assert!(s.bounded_rows >= 1, "no bounded row");
    assert!(s.retest_pending_rows >= 1, "no retest-pending row");
    assert!(s.withheld_rows >= 1, "no withheld row");
    assert!(s.downgraded_rows >= 1, "no downgraded row");
}

#[test]
fn every_required_drill_class_has_a_detected_drill() {
    let packet = packet();
    for class in CertificationDrillClass::REQUIRED {
        let drill = packet
            .drills
            .iter()
            .find(|d| d.drill_class == class)
            .unwrap_or_else(|| panic!("missing drill for class {}", class.as_str()));
        assert!(drill.detected, "drill {} does not detect", drill.drill_id);
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for consumer in CertificationConsumer::REQUIRED {
        assert!(
            packet.has_binding_for(consumer),
            "missing binding for consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn export_projection_preserves_published_support() {
    let packet = packet();
    let projection = packet.export_projection();
    assert!(projection.all_rows_gate_consistent);
    assert_eq!(projection.rows.len(), packet.rows.len());
    for (row, export) in packet.rows.iter().zip(projection.rows.iter()) {
        assert_eq!(export.published_support, row.published_support.as_str());
        assert_eq!(export.downgrade_path, row.downgrade_path.as_str());
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export("export-test", "2026-06-12T00:00:00Z");
    assert!(export.is_export_safe());
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.certification_packet_id_ref, packet.packet_id);
}

#[test]
fn detects_overstated_row_support() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.published_support == InstallAssurance::Withheld)
        .expect("a withheld row exists");
    row.published_support = InstallAssurance::Verified;
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5InstallConfigAuthCertificationViolation::OverstatedSupport { .. }
    )));
}

#[test]
fn detects_stale_evidence_downgrade() {
    let mut packet = packet();
    // Flip a current domain to missing on a currently-verified row: the gate must withhold it and flag
    // the overstated support.
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.is_verified())
        .expect("a verified row exists");
    row.domains[0].evidence_freshness = EvidenceFreshness::Missing;
    assert_eq!(row.effective_support(), InstallAssurance::Withheld);
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5InstallConfigAuthCertificationViolation::OverstatedSupport { .. }
    )));
}

#[test]
fn detects_missing_domain_coverage() {
    let mut packet = packet();
    packet.rows[0].domains.remove(0);
    assert_eq!(
        packet.rows[0].computed_downgrade_path(),
        CertificationDowngradePath::CompleteDomainCoverage
    );
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5InstallConfigAuthCertificationViolation::MissingDomain { .. }
    )));
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5InstallConfigAuthCertificationViolation::DowngradePathMismatch { .. }
    )));
}

#[test]
fn detects_source_ref_drift() {
    let mut packet = packet();
    packet.rows[0].domains[0].source_packet_ref = "wrong:ref:v9".to_owned();
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5InstallConfigAuthCertificationViolation::SourceRefMismatch { .. }
    )));
}
