use super::*;
use crate::m5_install_and_portability_governance::current_m5_install_portability_governance_matrix;

fn packet() -> M5InstallUpdateDiagnostics {
    current_m5_install_update_diagnostics().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_INSTALL_DIAGNOSTICS_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_INSTALL_DIAGNOSTICS_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_family_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.artifacts.len(), packet.artifact_families.len());
    for &family in &packet.artifact_families {
        assert!(
            packet.artifact_row(family).is_some(),
            "missing row for family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_artifact_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_artifacts_gate_consistent());
    for row in &packet.artifacts {
        assert_eq!(
            row.published_support,
            row.effective_support(),
            "artifact {} publishes beyond the gate",
            row.artifact_id
        );
        assert_eq!(
            row.narrow_reasons,
            row.computed_narrow_reasons(),
            "artifact {} narrow reasons diverge from the gate",
            row.artifact_id
        );
        assert_eq!(
            row.recovery_path,
            row.computed_recovery_path(),
            "artifact {} recovery path diverges from the gate",
            row.artifact_id
        );
    }
}

#[test]
fn artifacts_never_publish_beyond_their_governance_lane() {
    let packet = packet();
    let matrix = current_m5_install_portability_governance_matrix().expect("governance parses");
    assert_eq!(packet.governance_packet_id_ref, matrix.packet_id);
    for row in &packet.artifacts {
        let lane_row = matrix
            .lane_row(row.governs_lane)
            .expect("governance lane exists");
        assert_eq!(
            row.governs_assurance, lane_row.published_assurance,
            "artifact {} snapshots a stale governance assurance",
            row.artifact_id
        );
        assert!(
            row.published_support.rank() <= lane_row.published_assurance.rank(),
            "artifact {} claims support beyond its governance lane",
            row.artifact_id
        );
    }
}

#[test]
fn each_family_pins_to_its_canonical_lane() {
    let packet = packet();
    for row in &packet.artifacts {
        assert_eq!(
            row.governs_lane,
            row.artifact_family.governs_lane(),
            "artifact {} is not pinned to its canonical lane",
            row.artifact_id
        );
    }
}

#[test]
fn sensitive_roots_are_redacted_and_categorized() {
    let packet = packet();
    for row in &packet.artifacts {
        assert!(
            !row.artifact_roots.is_empty() && !row.mutable_state_roots.is_empty(),
            "artifact {} is missing required roots",
            row.artifact_id
        );
        assert!(
            row.roots_redaction_ok(),
            "artifact {} leaks a sensitive root",
            row.artifact_id
        );
        for root in &row.artifact_roots {
            assert_eq!(root.role.category(), RootCategory::Artifact);
        }
        for root in &row.mutable_state_roots {
            assert_eq!(root.role.category(), RootCategory::MutableState);
        }
        for root in &row.policy_roots {
            assert_eq!(root.role.category(), RootCategory::Policy);
        }
    }
    // At least one secret-bearing or machine-protected root is exercised and redacted.
    let redacted = packet
        .artifacts
        .iter()
        .flat_map(|a| a.all_roots())
        .filter(|r| r.sensitivity.requires_redaction())
        .count();
    assert!(redacted > 0, "no sensitive root is exercised");
}

#[test]
fn the_gate_admits_and_narrows_in_every_direction() {
    let packet = packet();
    let summary = &packet.summary;
    assert!(summary.verified_artifacts >= 1, "no verified artifact");
    assert!(
        summary.retest_pending_artifacts >= 1,
        "no retest-pending artifact"
    );
    assert!(summary.withheld_artifacts >= 1, "no withheld artifact");
    assert!(summary.downgraded_artifacts >= 1, "no downgraded artifact");
}

#[test]
fn every_required_incident_has_a_detected_drill() {
    let packet = packet();
    for incident in DiagnosticIncident::REQUIRED {
        let drill = packet
            .drills
            .iter()
            .find(|d| d.incident == incident)
            .unwrap_or_else(|| panic!("missing drill for incident {}", incident.as_str()));
        assert!(drill.detected, "drill {} does not detect", drill.drill_id);
        assert!(
            packet.artifact_row_by_id(&drill.artifact_id).is_some(),
            "drill {} targets an unknown artifact",
            drill.drill_id
        );
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for consumer in DiagnosticsConsumer::REQUIRED {
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
    assert!(projection.all_artifacts_gate_consistent);
    assert_eq!(projection.artifacts.len(), packet.artifacts.len());
    for (row, export) in packet.artifacts.iter().zip(projection.artifacts.iter()) {
        assert_eq!(export.published_support, row.published_support.as_str());
        assert_eq!(export.recovery_path, row.recovery_path.as_str());
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export("export-test", "2026-06-11T00:00:00Z");
    assert!(export.is_export_safe());
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.diagnostics_packet_id_ref, packet.packet_id);
}

#[test]
fn detects_overstated_support() {
    let mut packet = packet();
    let row = packet
        .artifacts
        .iter_mut()
        .find(|a| a.published_support == InstallAssurance::Withheld)
        .expect("a withheld artifact exists");
    row.published_support = InstallAssurance::Verified;
    assert!(packet
        .validate()
        .iter()
        .any(|v| matches!(v, M5InstallDiagnosticsViolation::OverstatedSupport { .. })));
}

#[test]
fn detects_unredacted_sensitive_root() {
    let mut packet = packet();
    'outer: for row in &mut packet.artifacts {
        for root in row
            .mutable_state_roots
            .iter_mut()
            .chain(row.policy_roots.iter_mut())
        {
            if root.sensitivity.requires_redaction() {
                root.redacted = false;
                break 'outer;
            }
        }
    }
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5InstallDiagnosticsViolation::UnredactedSensitiveRoot { .. }
    )));
}
