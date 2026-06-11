use super::*;

fn packet() -> ProjectDoctorFeatureLaneProbes {
    current_project_doctor_feature_lane_probes().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        PROJECT_DOCTOR_FEATURE_LANE_PROBES_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        PROJECT_DOCTOR_FEATURE_LANE_PROBES_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_records() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_lane_has_exactly_one_family() {
    let packet = packet();
    assert!(packet.all_lanes_covered());
    assert_eq!(packet.families.len(), DoctorLane::ALL.len());
    for lane in DoctorLane::ALL {
        let family = packet
            .family(lane)
            .unwrap_or_else(|| panic!("missing {lane}"));
        assert_eq!(family.lane, lane);
        assert!(family.prefix_matches_lane(), "lane {lane} prefix");
    }
}

#[test]
fn every_finding_binds_to_its_lane_family() {
    let packet = packet();
    for finding in &packet.findings {
        let family = packet
            .family(finding.lane)
            .expect("finding lane has a family");
        assert_eq!(finding.family_ref, family.family_id);
        assert!(family
            .supported_finding_codes
            .contains(&finding.finding_code));
        assert!(family.supported_states.contains(&finding.diagnosis_state));
        assert!(family
            .supported_scope_kinds
            .contains(&finding.affected_scope.scope_kind));
    }
}

#[test]
fn findings_are_read_only_and_metadata_safe() {
    let packet = packet();
    // There is no mutating posture: read-only by construction.
    for family in &packet.families {
        assert!(matches!(
            family.read_only_posture,
            ReadOnlyPosture::ReadOnlyNoMutation | ReadOnlyPosture::MetadataLocalEvidenceOnly
        ));
    }
    for finding in &packet.findings {
        assert!(finding.raw_private_material_excluded);
        assert_eq!(finding.redaction_class, "metadata_safe_default");
    }
}

#[test]
fn non_healthy_states_report_explicit_detail() {
    let packet = packet();
    for finding in &packet.findings {
        if finding.diagnosis_state.requires_explicit_detail() {
            let detail = finding.state_detail_code.trim();
            assert!(!detail.is_empty(), "{} empty detail", finding.finding_id);
            assert!(
                !GENERIC_DETAIL_TOKENS.contains(&detail.to_ascii_lowercase().as_str()),
                "{} generic detail",
                finding.finding_id
            );
        } else {
            assert!(
                finding.state_detail_code.trim().is_empty(),
                "{} healthy has detail",
                finding.finding_id
            );
        }
    }
}

#[test]
fn findings_are_cross_context_stable() {
    let packet = packet();
    for finding in &packet.findings {
        assert!(
            finding.is_cross_context_stable(),
            "{} not cross-context stable",
            finding.finding_id
        );
    }
    assert_eq!(
        packet
            .findings
            .iter()
            .filter(|f| f.is_cross_context_stable())
            .count(),
        packet.findings.len()
    );
}

#[test]
fn repair_candidates_stay_within_emitting_lanes() {
    let packet = packet();
    for finding in &packet.findings {
        if finding.has_repair_candidate() {
            let family = packet.family(finding.lane).expect("family");
            assert!(
                family.emits_repair_candidates,
                "{} attaches repair in a non-emitting lane",
                finding.finding_id
            );
            for id in &finding.repair_candidate_ids {
                assert!(id.starts_with(DOCTOR_REPAIR_CANDIDATE_PREFIX));
            }
        }
    }
    assert!(
        packet.findings.iter().any(|f| f.has_repair_candidate()),
        "corpus needs a repair-candidate finding"
    );
}

#[test]
fn diagnosis_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DiagnosisState> =
        packet.findings.iter().map(|f| f.diagnosis_state).collect();
    for state in DiagnosisState::ALL {
        assert!(present.contains(&state), "no finding in state {state:?}");
    }
}

#[test]
fn severity_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FindingSeverity> =
        packet.findings.iter().map(|f| f.severity_class).collect();
    for severity in FindingSeverity::ALL {
        assert!(
            present.contains(&severity),
            "no finding with severity {}",
            severity.as_str()
        );
    }
}

#[test]
fn confidence_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FindingConfidence> =
        packet.findings.iter().map(|f| f.confidence_class).collect();
    for confidence in FindingConfidence::ALL {
        assert!(
            present.contains(&confidence),
            "no finding with confidence {}",
            confidence.as_str()
        );
    }
}

#[test]
fn scope_kinds_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ScopeKind> = packet
        .findings
        .iter()
        .map(|f| f.affected_scope.scope_kind)
        .collect();
    for scope in ScopeKind::ALL {
        assert!(
            present.contains(&scope),
            "no finding with scope {}",
            scope.as_str()
        );
    }
}

#[test]
fn unsupported_state_and_severity_agree() {
    let packet = packet();
    for finding in &packet.findings {
        let unsupported_state = finding.diagnosis_state == DiagnosisState::Unsupported;
        let unsupported_severity = finding.severity_class == FindingSeverity::Unsupported;
        assert_eq!(
            unsupported_state, unsupported_severity,
            "{} severity/state disagree",
            finding.finding_id
        );
    }
}

#[test]
fn export_projection_reflects_findings() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.findings.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.all_lanes_covered, packet.all_lanes_covered());
    assert_eq!(
        projection.repair_candidate_count,
        packet
            .findings
            .iter()
            .filter(|f| f.has_repair_candidate())
            .count()
    );
    assert_eq!(projection.cross_context_stable_count, packet.findings.len());
}

#[test]
fn validate_flags_generic_unsupported_detail() {
    let mut packet = packet();
    if let Some(finding) = packet
        .findings
        .iter_mut()
        .find(|f| f.diagnosis_state.requires_explicit_detail())
    {
        finding.state_detail_code = "unavailable".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorFeatureLaneProbesViolation::GenericUnsupportedDetail { .. }
        )));
    }
}

#[test]
fn validate_flags_healthy_finding_with_detail() {
    let mut packet = packet();
    if let Some(finding) = packet
        .findings
        .iter_mut()
        .find(|f| f.diagnosis_state == DiagnosisState::Healthy)
    {
        finding.state_detail_code = "should_not_be_here".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorFeatureLaneProbesViolation::HealthyFindingHasDetail { .. }
        )));
    }
}

#[test]
fn validate_flags_severity_state_mismatch() {
    let mut packet = packet();
    if let Some(finding) = packet
        .findings
        .iter_mut()
        .find(|f| f.diagnosis_state == DiagnosisState::Unsupported)
    {
        finding.severity_class = FindingSeverity::Blocking;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorFeatureLaneProbesViolation::SeverityStateMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_repair_candidate_in_non_emitting_lane() {
    let mut packet = packet();
    if let Some(finding) = packet
        .findings
        .iter_mut()
        .find(|f| f.lane == DoctorLane::ProfilerReplay && f.repair_candidate_ids.is_empty())
    {
        finding
            .repair_candidate_ids
            .push("repair.profiler_replay.reattach".to_owned());
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorFeatureLaneProbesViolation::RepairCandidateNotPermitted { .. }
        )));
    }
}

#[test]
fn validate_flags_repair_candidate_prefix() {
    let mut packet = packet();
    if let Some(finding) = packet
        .findings
        .iter_mut()
        .find(|f| f.has_repair_candidate())
    {
        finding.repair_candidate_ids[0] = "not-a-repair-id".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorFeatureLaneProbesViolation::RepairCandidatePrefix { .. }
        )));
    }
}

#[test]
fn validate_flags_finding_not_cross_context_stable() {
    let mut packet = packet();
    if let Some(finding) = packet.findings.first_mut() {
        finding
            .render_surfaces
            .retain(|s| *s != RenderSurface::HeadlessJsonRow);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorFeatureLaneProbesViolation::FindingNotCrossContextStable { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_lane_family() {
    let mut packet = packet();
    let removed = packet.families.pop();
    assert!(removed.is_some());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ProjectDoctorFeatureLaneProbesViolation::MissingLaneFamily { .. }
    )));
}

#[test]
fn validate_flags_finding_code_not_supported() {
    let mut packet = packet();
    if let Some(finding) = packet.findings.first_mut() {
        finding.finding_code = format!("{}unknown_code", finding.lane.finding_code_prefix());
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorFeatureLaneProbesViolation::FindingCodeNotSupported { .. }
        )));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.finding_count = packet.summary.finding_count.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&ProjectDoctorFeatureLaneProbesViolation::SummaryMismatch));
}

#[test]
fn validate_flags_unknown_family_ref() {
    let mut packet = packet();
    if let Some(finding) = packet.findings.first_mut() {
        finding.family_ref = "probe-family:nope".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorFeatureLaneProbesViolation::FindingFamilyRefUnknown { .. }
        )));
    }
}

#[test]
fn tokens_are_stable() {
    assert_eq!(DoctorLane::NotebookKernel.as_str(), "notebook_kernel");
    assert_eq!(DoctorLane::RequestApi.as_str(), "request_api");
    assert_eq!(DoctorLane::IncidentPacket.as_str(), "incident_packet");
    assert_eq!(
        DoctorLane::NotebookKernel.finding_code_prefix(),
        "doctor.finding.notebook_kernel."
    );
    assert_eq!(DiagnosisState::Unsupported.as_str(), "unsupported");
    assert_eq!(DiagnosisState::TargetMismatch.as_str(), "target_mismatch");
    assert_eq!(DiagnosisState::PolicyBlocked.as_str(), "policy_blocked");
    assert_eq!(FindingSeverity::Blocking.as_str(), "blocking");
    assert_eq!(
        FindingConfidence::UnknownRequiresProbe.as_str(),
        "unknown_requires_probe"
    );
    assert_eq!(ScopeKind::KernelEngine.as_str(), "kernel_engine");
    assert_eq!(
        ReadOnlyPosture::ReadOnlyNoMutation.as_str(),
        "read_only_no_mutation"
    );
}

#[test]
fn canonical_scope_kinds_are_a_bijection_over_lanes() {
    let scopes: BTreeSet<ScopeKind> = DoctorLane::ALL
        .iter()
        .map(|l| l.canonical_scope_kind())
        .collect();
    assert_eq!(
        scopes.len(),
        DoctorLane::ALL.len(),
        "each lane must map to a distinct canonical scope kind"
    );
    assert_eq!(scopes.len(), ScopeKind::ALL.len());
}
