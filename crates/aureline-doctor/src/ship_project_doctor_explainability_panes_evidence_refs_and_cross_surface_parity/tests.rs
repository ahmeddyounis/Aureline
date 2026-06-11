use super::*;

fn packet() -> ProjectDoctorExplainabilityParity {
    current_project_doctor_explainability_parity().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        PROJECT_DOCTOR_EXPLAINABILITY_PARITY_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        PROJECT_DOCTOR_EXPLAINABILITY_PARITY_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_records() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_pane_exposes_probe_version_and_evidence() {
    let packet = packet();
    for pane in &packet.panes {
        assert!(
            !pane.probe_id.trim().is_empty(),
            "{} probe_id",
            pane.pane_id
        );
        assert!(
            !pane.probe_version.trim().is_empty(),
            "{} probe_version",
            pane.pane_id
        );
        assert!(
            !pane.evidence_refs.is_empty(),
            "{} evidence_refs",
            pane.pane_id
        );
        assert!(
            pane.finding_code.starts_with(DOCTOR_FINDING_PREFIX),
            "{} finding code prefix",
            pane.pane_id
        );
    }
}

#[test]
fn exit_class_is_canonical_for_state() {
    let packet = packet();
    for pane in &packet.panes {
        assert!(
            pane.exit_class_is_canonical(),
            "{} exit class not canonical",
            pane.pane_id
        );
        assert_eq!(
            pane.cli_exit_class,
            pane.diagnosis_state.canonical_exit_class()
        );
    }
}

#[test]
fn exit_codes_are_stable() {
    assert_eq!(CliExitClass::OkHealthy.exit_code(), 0);
    assert_eq!(CliExitClass::AdvisoryFindings.exit_code(), 10);
    assert_eq!(CliExitClass::Blocked.exit_code(), 20);
    assert_eq!(CliExitClass::Unsupported.exit_code(), 30);
    assert_eq!(CliExitClass::PolicyDenied.exit_code(), 40);
}

#[test]
fn available_repairs_name_candidate_and_reversal() {
    let packet = packet();
    let mut available = 0;
    for pane in &packet.panes {
        if pane.repair_availability.is_available() {
            available += 1;
            assert!(
                pane.repair_candidate_id
                    .starts_with(DOCTOR_REPAIR_CANDIDATE_PREFIX),
                "{} repair prefix",
                pane.pane_id
            );
            assert!(
                pane.repair_unavailable_reason_code.trim().is_empty(),
                "{} available pane carries a block reason",
                pane.pane_id
            );
            assert!(
                pane.reversal_class.is_applicable(),
                "{} available repair lacks a reversal class",
                pane.pane_id
            );
        }
    }
    assert!(available >= 1, "corpus needs an available-repair pane");
}

#[test]
fn blocked_repairs_name_explicit_reason() {
    let packet = packet();
    let mut blocked = 0;
    for pane in &packet.panes {
        if pane.repair_availability.is_blocked() {
            blocked += 1;
            let reason = pane.repair_unavailable_reason_code.trim();
            assert!(!reason.is_empty(), "{} empty block reason", pane.pane_id);
            assert!(
                !GENERIC_DETAIL_TOKENS.contains(&reason.to_ascii_lowercase().as_str()),
                "{} generic block reason",
                pane.pane_id
            );
            assert!(
                pane.repair_candidate_id.trim().is_empty(),
                "{} blocked pane names a candidate",
                pane.pane_id
            );
            assert_eq!(
                pane.reversal_class,
                ReversalClass::NotApplicable,
                "{} blocked pane claims a reversal path",
                pane.pane_id
            );
        }
    }
    assert!(blocked >= 1, "corpus needs a blocked-repair pane");
}

#[test]
fn healthy_panes_carry_no_repair() {
    let packet = packet();
    for pane in packet.panes_in_state(DiagnosisState::Healthy) {
        assert_eq!(
            pane.repair_availability,
            RepairAvailability::NotApplicableHealthy
        );
        assert!(pane.repair_candidate_id.trim().is_empty());
        assert!(pane.repair_unavailable_reason_code.trim().is_empty());
        assert_eq!(pane.reversal_class, ReversalClass::NotApplicable);
        assert_eq!(pane.cli_exit_class, CliExitClass::OkHealthy);
    }
}

#[test]
fn panes_are_cross_surface_stable() {
    let packet = packet();
    for pane in &packet.panes {
        assert!(
            pane.is_cross_surface_stable(),
            "{} not cross-surface stable",
            pane.pane_id
        );
    }
    assert_eq!(
        packet
            .panes
            .iter()
            .filter(|p| p.is_cross_surface_stable())
            .count(),
        packet.panes.len()
    );
}

#[test]
fn panes_carry_locale_invariant_keys() {
    let packet = packet();
    for pane in &packet.panes {
        for required in REQUIRED_MACHINE_MEANING_KEYS {
            assert!(
                pane.machine_meaning_keys.iter().any(|k| k == required),
                "{} missing machine-meaning key {required}",
                pane.pane_id
            );
        }
    }
}

#[test]
fn panes_are_read_only_and_metadata_safe() {
    let packet = packet();
    for pane in &packet.panes {
        assert!(pane.raw_private_material_excluded);
        assert_eq!(pane.redaction_class, "metadata_safe_default");
    }
}

#[test]
fn diagnosis_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DiagnosisState> =
        packet.panes.iter().map(|p| p.diagnosis_state).collect();
    for state in DiagnosisState::ALL {
        assert!(present.contains(&state), "no pane in state {state:?}");
    }
}

#[test]
fn repair_availabilities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RepairAvailability> =
        packet.panes.iter().map(|p| p.repair_availability).collect();
    for availability in RepairAvailability::ALL {
        assert!(
            present.contains(&availability),
            "no pane with availability {}",
            availability.as_str()
        );
    }
}

#[test]
fn reversal_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ReversalClass> = packet.panes.iter().map(|p| p.reversal_class).collect();
    for class in ReversalClass::ALL {
        assert!(
            present.contains(&class),
            "no pane with reversal class {}",
            class.as_str()
        );
    }
}

#[test]
fn cli_exit_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CliExitClass> = packet.panes.iter().map(|p| p.cli_exit_class).collect();
    for class in CliExitClass::ALL {
        assert!(
            present.contains(&class),
            "no pane with exit class {}",
            class.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_panes() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.panes.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.available_repair_count,
        packet
            .panes
            .iter()
            .filter(|p| p.has_available_repair())
            .count()
    );
    assert_eq!(projection.cross_surface_stable_count, packet.panes.len());
    for row in &projection.rows {
        // Every row carries the stable exit code alongside the class token.
        assert!(row.cli_exit_code >= 0);
    }
}

#[test]
fn validate_flags_generic_blocked_reason() {
    let mut packet = packet();
    if let Some(pane) = packet
        .panes
        .iter_mut()
        .find(|p| p.repair_availability.is_blocked())
    {
        pane.repair_unavailable_reason_code = "unavailable".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorExplainabilityParityViolation::GenericBlockedReason { .. }
        )));
    }
}

#[test]
fn validate_flags_available_repair_missing_candidate() {
    let mut packet = packet();
    if let Some(pane) = packet
        .panes
        .iter_mut()
        .find(|p| p.repair_availability.is_available())
    {
        pane.repair_candidate_id = String::new();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorExplainabilityParityViolation::AvailableRepairMissingCandidate { .. }
        )));
    }
}

#[test]
fn validate_flags_unavailable_repair_with_candidate() {
    let mut packet = packet();
    if let Some(pane) = packet
        .panes
        .iter_mut()
        .find(|p| p.repair_availability.is_blocked())
    {
        pane.repair_candidate_id = "repair.illegal.candidate".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorExplainabilityParityViolation::UnavailableRepairHasCandidate { .. }
        )));
    }
}

#[test]
fn validate_flags_reversal_availability_mismatch() {
    let mut packet = packet();
    if let Some(pane) = packet
        .panes
        .iter_mut()
        .find(|p| p.repair_availability.is_available())
    {
        pane.reversal_class = ReversalClass::NotApplicable;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorExplainabilityParityViolation::ReversalAvailabilityMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_non_canonical_exit_class() {
    let mut packet = packet();
    if let Some(pane) = packet
        .panes
        .iter_mut()
        .find(|p| p.diagnosis_state == DiagnosisState::Unsupported)
    {
        pane.cli_exit_class = CliExitClass::OkHealthy;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorExplainabilityParityViolation::ExitClassNotCanonical { .. }
        )));
    }
}

#[test]
fn validate_flags_healthy_availability_mismatch() {
    let mut packet = packet();
    if let Some(pane) = packet
        .panes
        .iter_mut()
        .find(|p| p.diagnosis_state == DiagnosisState::Healthy)
    {
        pane.repair_availability = RepairAvailability::BlockedManagedPolicy;
        pane.repair_unavailable_reason_code = "policy_denies".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorExplainabilityParityViolation::HealthyAvailabilityMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_machine_meaning_key() {
    let mut packet = packet();
    if let Some(pane) = packet.panes.first_mut() {
        pane.machine_meaning_keys.retain(|k| k != "cli_exit_class");
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorExplainabilityParityViolation::MissingMachineMeaningKey { .. }
        )));
    }
}

#[test]
fn validate_flags_pane_not_cross_surface_stable() {
    let mut packet = packet();
    if let Some(pane) = packet.panes.first_mut() {
        pane.parity_surfaces
            .retain(|s| *s != ParitySurface::HeadlessJson);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorExplainabilityParityViolation::PaneNotCrossSurfaceStable { .. }
        )));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.pane_count = packet.summary.pane_count.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&ProjectDoctorExplainabilityParityViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(DiagnosisState::Unsupported.as_str(), "unsupported");
    assert_eq!(DiagnosisState::PolicyBlocked.as_str(), "policy_blocked");
    assert_eq!(RepairAvailability::Available.as_str(), "available");
    assert_eq!(
        RepairAvailability::BlockedManagedPolicy.as_str(),
        "blocked_managed_policy"
    );
    assert_eq!(
        ReversalClass::ReversibleTransactional.as_str(),
        "reversible_transactional"
    );
    assert_eq!(CliExitClass::PolicyDenied.as_str(), "policy_denied");
    assert_eq!(ParitySurface::DesktopPane.as_str(), "desktop_pane");
    assert_eq!(ParitySurface::PublicTruth.as_str(), "public_truth");
}

#[test]
fn diagnosis_state_exit_map_is_total_and_distinct_enough() {
    // Every state maps to a defined exit class, and the healthy state is the
    // only one that maps to ok_healthy.
    for state in DiagnosisState::ALL {
        let class = state.canonical_exit_class();
        let is_ok = class == CliExitClass::OkHealthy;
        assert_eq!(is_ok, state == DiagnosisState::Healthy, "{state} ok map");
    }
}
