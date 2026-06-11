use super::*;

const PACKET_ID: &str = "m5-template-certification:certified:0001";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";
const CERTIFICATION_LABEL: &str =
    "M5 Template Registry, Scaffold Planner, Framework Packs, and Archetype Health Certification";

fn proof_freshness() -> M5TemplateCertificationProofFreshness {
    M5TemplateCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> M5TemplateCertificationPacket {
    certify_from_current_exports(
        PACKET_ID.to_owned(),
        CERTIFICATION_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        proof_freshness(),
    )
}

#[test]
fn certification_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn certification_covers_every_claimed_profile() {
    let packet = packet();
    let present: BTreeSet<M5TemplateProfile> = packet
        .certified_profiles
        .iter()
        .map(|profile| profile.profile)
        .collect();
    for profile in M5TemplateProfile::ALL {
        assert!(
            present.contains(&profile),
            "missing claimed profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn first_consumer_certifies_from_current_exports() {
    // Every upstream profile currently has a checked-in, valid export, so none are blocked.
    let packet = packet();
    assert_eq!(packet.compatibility_report.blocked_count, 0);
    assert_eq!(packet.compatibility_report.not_certified_count, 0);
    assert!(packet.compatibility_report.all_profiles_publishable);
    for profile in &packet.certified_profiles {
        assert!(
            profile.verdict.is_publishable(),
            "profile {} not publishable: {:?}",
            profile.profile.as_str(),
            profile.verdict
        );
    }
}

#[test]
fn beta_profiles_are_narrowed() {
    let packet = packet();
    for profile_kind in [
        M5TemplateProfile::RicherFrameworkPacks,
        M5TemplateProfile::ConventionDiagnostics,
    ] {
        let profile = packet
            .certified_profiles
            .iter()
            .find(|profile| profile.profile == profile_kind)
            .expect("beta profile present");
        assert_eq!(
            profile.claimed_qualification,
            M5TemplateCertificationQualificationClass::Beta
        );
        assert_eq!(
            profile.verdict,
            M5TemplateCertificationVerdict::NarrowedCertified
        );
    }
}

#[test]
fn missing_profile_fails_validation() {
    let mut packet = packet();
    packet
        .certified_profiles
        .retain(|profile| profile.profile != M5TemplateProfile::FrameworkPackHeader);
    packet.compatibility_report =
        M5TemplateCertificationCompatibilityReport::from_profiles(&packet.certified_profiles);
    assert!(packet
        .validate()
        .contains(&M5TemplateCertificationViolation::RequiredProfileMissing));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.certified_profiles[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5TemplateCertificationViolation::DowngradeTriggersMissing));
}

#[test]
fn profile_incomplete_fails() {
    let mut packet = packet();
    packet.certified_profiles[0].evidence_artifact_ref.clear();
    let violations = packet.validate();
    assert!(violations.contains(&M5TemplateCertificationViolation::ProfileIncomplete));
}

#[test]
fn compatibility_report_mismatch_fails() {
    let mut packet = packet();
    packet.compatibility_report.certified_count += 1;
    assert!(packet
        .validate()
        .contains(&M5TemplateCertificationViolation::CompatibilityReportMismatch));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TemplateCertificationViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_credential_bodies_in_export = false;
    assert!(packet
        .validate()
        .contains(&M5TemplateCertificationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.diff_review_shows_certification = false;
    assert!(packet
        .validate()
        .contains(&M5TemplateCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TemplateCertificationViolation::ProofFreshnessIncomplete));
}

#[test]
fn downgrade_automation_blocks_on_invalid_evidence() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[M5TemplateCertificationProfileObservation {
        profile: M5TemplateProfile::FrameworkPackHeader,
        evidence_valid: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let header = packet
        .certified_profiles
        .iter()
        .find(|profile| profile.profile == M5TemplateProfile::FrameworkPackHeader)
        .expect("framework pack header profile present");
    assert_eq!(header.verdict, M5TemplateCertificationVerdict::Blocked);
    assert_eq!(packet.compatibility_report.blocked_count, 1);
    assert!(!packet.compatibility_report.all_profiles_publishable);
    // Still serializes and validates structurally after automation.
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn downgrade_automation_narrows_on_stale_proof() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[M5TemplateCertificationProfileObservation {
        profile: M5TemplateProfile::ArchetypeHealthBundle,
        evidence_valid: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let bundle = packet
        .certified_profiles
        .iter()
        .find(|profile| profile.profile == M5TemplateProfile::ArchetypeHealthBundle)
        .expect("archetype health bundle profile present");
    assert_eq!(
        bundle.verdict,
        M5TemplateCertificationVerdict::NarrowedCertified
    );
    assert!(!bundle.proof_freshness.proof_fresh);
}

#[test]
fn markdown_summary_lists_every_profile() {
    let summary = packet().render_markdown_summary();
    for profile in M5TemplateProfile::ALL {
        assert!(
            summary.contains(profile.as_str()),
            "summary missing profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_m5_template_certification_export()
        .expect("checked M5 template certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_first_consumer_certification() {
    let checked = current_m5_template_certification_export()
        .expect("checked M5 template certification export validates");
    let regenerated = packet();
    assert_eq!(
        checked, regenerated,
        "checked export drifted from the first-consumer certification"
    );
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/framework_pack_evidence_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile/archetype_health_proof_stale_narrowed.json"
        )),
    ] {
        let packet: M5TemplateCertificationPacket =
            serde_json::from_str(raw).expect("fixture parses as certification packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
