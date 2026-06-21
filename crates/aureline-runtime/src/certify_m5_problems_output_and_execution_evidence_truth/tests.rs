//! Tests for the Problems / output / execution-evidence qualification capstone.

use super::*;

fn packet() -> ProblemsOutputEvidenceCertificationPacket {
    seeded_m5_problems_output_evidence_certification_packet()
}

#[test]
fn seeded_packet_validates() {
    let packet = packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
    assert!(packet.is_valid());
}

#[test]
fn seeded_packet_has_stable_header() {
    let packet = packet();
    assert_eq!(
        packet.record_kind,
        PROBLEMS_OUTPUT_EVIDENCE_CERT_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        PROBLEMS_OUTPUT_EVIDENCE_CERT_SCHEMA_VERSION
    );
    assert_eq!(
        packet.taxonomy_version,
        PROBLEMS_OUTPUT_EVIDENCE_CERT_TAXONOMY_VERSION
    );
    assert_eq!(packet.packet_id, PROBLEMS_OUTPUT_EVIDENCE_CERT_PACKET_ID);
}

#[test]
fn every_tooling_profile_is_represented() {
    let packet = packet();
    let represented = packet.represented_profiles();
    for profile in ToolingProfile::ALL {
        assert!(
            represented.contains(&profile),
            "missing profile {}",
            profile.as_str()
        );
    }
    assert_eq!(packet.profiles.len(), ToolingProfile::ALL.len());
}

#[test]
fn every_dimension_is_qualified_on_every_profile() {
    let packet = packet();
    for profile in &packet.profiles {
        assert_eq!(
            profile.dimensions.len(),
            CertificationDimension::ALL.len(),
            "profile {} missing dimensions",
            profile.profile.as_str()
        );
        for dimension in CertificationDimension::ALL {
            assert!(
                profile.dimension(dimension).is_some(),
                "profile {} missing dimension {}",
                profile.profile.as_str(),
                dimension.as_str()
            );
        }
    }
}

#[test]
fn release_evidence_rows_cover_every_axis() {
    let packet = packet();
    let represented = packet.represented_axes();
    for axis in ReleaseEvidenceAxis::ALL {
        assert!(
            represented.contains(&axis),
            "missing release-evidence axis {}",
            axis.as_str()
        );
    }
    assert_eq!(
        packet.release_evidence_rows.len(),
        ReleaseEvidenceAxis::ALL.len()
    );
}

#[test]
fn notebook_profile_narrows_to_retest_pending_on_stale_proof() {
    let packet = packet();
    let notebook = packet
        .profile(ToolingProfile::NotebookOutput)
        .expect("notebook profile present");
    assert_eq!(notebook.claimed_grade, ProfileQualificationGrade::Qualified);
    assert_eq!(
        notebook.effective_grade,
        ProfileQualificationGrade::RetestPending
    );
    assert!(notebook.needs_narrow());
    assert_eq!(
        notebook.narrow_trigger,
        Some(QualificationNarrowTrigger::StaleEvidence)
    );
    assert!(notebook
        .narrowed_label
        .as_ref()
        .is_some_and(|label| !label.trim().is_empty()));
}

#[test]
fn overlay_profile_holds_at_limited_when_current() {
    let packet = packet();
    let overlay = packet
        .profile(ToolingProfile::PipelineOverlay)
        .expect("pipeline overlay present");
    assert!(overlay.overlay_profile);
    assert_eq!(overlay.claimed_grade, ProfileQualificationGrade::Limited);
    assert_eq!(overlay.effective_grade, ProfileQualificationGrade::Limited);
    assert!(!overlay.needs_narrow());
    assert!(overlay.narrow_trigger.is_none());
}

#[test]
fn at_least_one_profile_is_fully_qualified() {
    let packet = packet();
    assert!(packet.profiles.iter().any(|p| {
        p.claimed_grade == ProfileQualificationGrade::Qualified
            && p.effective_grade == ProfileQualificationGrade::Qualified
    }));
}

#[test]
fn narrowed_count_and_overlay_count_match_expectations() {
    let packet = packet();
    // Exactly the notebook profile narrows in the healthy seed.
    assert_eq!(packet.narrowed_profile_count(), 1);
    assert_eq!(packet.overlay_profile_count(), 1);
    assert_eq!(packet.claimed_profile_count(), ToolingProfile::ALL.len());
}

#[test]
fn export_is_round_trip_stable() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: ProblemsOutputEvidenceCertificationPacket =
        serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn grade_ranks_order_a_strict_narrow() {
    use ProfileQualificationGrade::*;
    assert!(Qualified.rank() > Limited.rank());
    assert!(Limited.rank() > RetestPending.rank());
    assert!(RetestPending.rank() > Blocked.rank());
    assert!(Blocked.rank() > LabsNotClaimed.rank());
}

#[test]
fn corpus_cases_rederive_to_expected_outcomes() {
    for case in seeded_m5_problems_output_evidence_certification_corpus() {
        case.check().unwrap_or_else(|err| panic!("{err}"));
    }
}

#[test]
fn corpus_covers_every_narrow_trigger_and_blocking_dimension() {
    let cases = seeded_m5_problems_output_evidence_certification_corpus();
    let triggers: BTreeSet<&str> = cases
        .iter()
        .filter_map(|c| c.expected.narrow_trigger.map(|t| t.as_str()))
        .collect();
    for required in [
        QualificationNarrowTrigger::ProblemsCorrelationLost,
        QualificationNarrowTrigger::OutputChannelIdentityFlattened,
        QualificationNarrowTrigger::ProjectionLineageFlattened,
        QualificationNarrowTrigger::CausalLinkBroken,
        QualificationNarrowTrigger::ConfidenceOverclaimed,
        QualificationNarrowTrigger::StaleEvidence,
        QualificationNarrowTrigger::SupersededStateHidden,
        QualificationNarrowTrigger::ReopenPathLost,
        QualificationNarrowTrigger::MissingDimensionProof,
        QualificationNarrowTrigger::ImportedOverlayClaimsLive,
    ] {
        assert!(
            triggers.contains(required.as_str()),
            "corpus missing trigger {}",
            required.as_str()
        );
    }
}

#[test]
fn corpus_index_matches_corpus() {
    let index = seeded_m5_problems_output_evidence_certification_corpus_index();
    let cases = seeded_m5_problems_output_evidence_certification_corpus();
    assert_eq!(index.cases.len(), cases.len());
    for (entry, case) in index.cases.iter().zip(cases.iter()) {
        assert_eq!(entry, &format!("{}.json", case.case_id));
    }
    assert_eq!(
        index.source_set_ref,
        PROBLEMS_OUTPUT_EVIDENCE_CERT_SUPPORT_EXPORT_REF
    );
}

#[test]
fn broken_invariant_that_stays_green_is_rejected() {
    let mut packet = packet();
    // Flatten an output channel's identity but leave its grade qualified.
    let target = packet
        .profiles
        .iter_mut()
        .find(|p| p.profile == ToolingProfile::OutputChannel)
        .expect("output channel present");
    if let Some(d) = target
        .dimensions
        .iter_mut()
        .find(|d| d.dimension == CertificationDimension::OutputChannelIdentity)
    {
        d.invariant_holds = false;
    }
    // Stored grade still claims qualified -> drift + not-narrowed violations.
    let violations = packet.validate();
    assert!(violations.contains(&CertificationViolation::EffectiveGradeDrift));
    assert!(violations.contains(&CertificationViolation::IdentityFlattenedButNotNarrowed));
}

#[test]
fn imported_proof_on_first_party_profile_is_rejected_when_green() {
    let mut packet = packet();
    let target = packet
        .profiles
        .iter_mut()
        .find(|p| p.profile == ToolingProfile::ProblemsPanel)
        .expect("problems panel present");
    assert!(!target.overlay_profile);
    if let Some(d) = target
        .dimensions
        .iter_mut()
        .find(|d| d.dimension == CertificationDimension::CausalLinkIntegrity)
    {
        d.proof_currency = ProofCurrency::ImportedCurrent;
    }
    let violations = packet.validate();
    assert!(violations.contains(&CertificationViolation::EffectiveGradeDrift));
}

#[test]
fn missing_upstream_lane_ref_is_rejected() {
    let mut packet = packet();
    packet.upstream_lane_refs.pop();
    let violations = packet.validate();
    assert!(violations.contains(&CertificationViolation::MissingUpstreamLaneRefs));
}

#[test]
fn missing_release_evidence_row_is_rejected() {
    let mut packet = packet();
    packet.release_evidence_rows.pop();
    let violations = packet.validate();
    assert!(
        violations.contains(&CertificationViolation::ReleaseEvidenceRowDrift)
            || violations.contains(&CertificationViolation::ReleaseEvidenceAxisMissing)
    );
}

#[test]
fn guardrail_breach_is_rejected() {
    let mut packet = packet();
    packet.guardrails.imported_overlay_never_claims_live = false;
    assert!(packet
        .validate()
        .contains(&CertificationViolation::GuardrailsIncomplete));
}

#[test]
fn consumer_surface_gap_is_rejected() {
    let mut packet = packet();
    packet.consumer_surfaces.about_surface_ingests = false;
    assert!(packet
        .validate()
        .contains(&CertificationViolation::ConsumerSurfacesIncomplete));
}

#[test]
fn stale_freshness_block_is_rejected() {
    let mut packet = packet();
    packet.evidence_freshness.auto_narrow_on_stale = false;
    assert!(packet
        .validate()
        .contains(&CertificationViolation::EvidenceFreshnessIncomplete));
}

#[test]
fn forbidden_boundary_material_is_rejected() {
    let mut packet = packet();
    packet.profiles[0].claim_summary = "leaked api_key=hunter2".to_owned();
    // Re-derive the release rows so only the boundary-material check trips.
    assert!(packet
        .validate()
        .contains(&CertificationViolation::RawBoundaryMaterialInExport));
}

#[test]
fn waiver_log_lists_only_narrowed_profiles() {
    let packet = packet();
    let log = packet.render_waiver_and_downgrade_log();
    assert!(log.contains("Auto-downgraded profiles (1)"));
    assert!(log.contains(ToolingProfile::NotebookOutput.as_str()));
    // A clean profile must not appear as auto-downgraded.
    assert!(!log.contains("**problems_panel**"));
}

#[test]
fn report_renders_every_profile_and_axis() {
    let packet = packet();
    let report = packet.render_markdown_report();
    for profile in ToolingProfile::ALL {
        assert!(
            report.contains(profile.as_str()),
            "report missing {}",
            profile.as_str()
        );
    }
    for axis in ReleaseEvidenceAxis::ALL {
        assert!(
            report.contains(axis.as_str()),
            "report missing {}",
            axis.as_str()
        );
    }
}

#[test]
fn checked_in_export_matches_seed() {
    // The checked-in artifact must be byte-aligned with the in-crate builder.
    let from_disk = current_m5_problems_output_evidence_certification_export()
        .expect("checked-in export parses and validates");
    assert_eq!(from_disk, packet());
}
