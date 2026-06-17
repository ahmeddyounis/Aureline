//! Unit tests for the native-desktop qualification builder, validator, and
//! claim-narrowing derivation.

use super::*;

fn qualified(dimension: QualificationDimension, profile_id: &str) -> QualificationDimensionBinding {
    QualificationDimensionBinding {
        dimension,
        required_drill: dimension.required_drill(),
        status: QualificationStatus::Qualified,
        failure_mode: None,
        drill_ref: Some(format!(
            "drill:{profile_id}:{}",
            dimension.required_drill().as_str()
        )),
        evidence_pack_ref: Some(format!("evidence:{profile_id}:{}", dimension.as_str())),
        narrowing_reason: None,
        note: None,
    }
}

fn full_bindings(profile_id: &str) -> Vec<QualificationDimensionBinding> {
    QualificationDimension::required_dimensions()
        .into_iter()
        .map(|dimension| qualified(dimension, profile_id))
        .collect()
}

fn clean_descriptor(profile_id: &str) -> QualificationProfileDescriptor {
    QualificationProfileDescriptor {
        profile_id: profile_id.to_owned(),
        platform: DesktopPlatform::Macos,
        channel: DeliveryChannel::Stable,
        descriptor_revision_ref: format!("{profile_id}:rev"),
        display_label_ref: format!("{profile_id}:label"),
        channel_build_owner_ref: format!("{profile_id}:owner"),
        ownership_kind: OwnershipKind::ChannelScopedOwner,
        trust_checkpoint_ref: format!("{profile_id}:trust"),
        continuity_note: "preserves context".to_owned(),
        evidence_freshness: EvidenceFreshness::Fresh,
        evidence_captured_at: "2026-06-16T00:00:00Z".to_owned(),
        downgrade_rule_ref: "downgrade:rule".to_owned(),
        marketed: true,
        registered_on_qualification_harness: true,
    }
}

#[test]
fn seeded_report_is_clean_and_validates() {
    let report = seeded_native_desktop_qualification();
    assert!(report.report_clean, "seeded report must be clean");
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    validate_qualification_report(&report).expect("seeded report must validate");
}

#[test]
fn seeded_report_qualifies_every_dimension() {
    let report = seeded_native_desktop_qualification();
    assert!(
        report.every_dimension_qualified(),
        "every required dimension must be qualified by at least one profile"
    );
    assert_eq!(
        report.required_dimensions.len(),
        QualificationDimension::required_dimensions().len()
    );
    assert_eq!(
        report.required_drills.len(),
        QualificationDrill::required_drills().len()
    );
}

#[test]
fn seeded_report_binds_every_dimension_on_every_profile() {
    let report = seeded_native_desktop_qualification();
    for profile in &report.profiles {
        assert_eq!(
            profile.bindings.len(),
            QualificationDimension::required_dimensions().len(),
            "{} must bind every dimension",
            profile.descriptor.profile_id
        );
    }
}

#[test]
fn seeded_claim_scope_publishes_full_profiles_and_narrows_portable() {
    let report = seeded_native_desktop_qualification();
    let portable = report
        .claim_scope
        .iter()
        .find(|claim| claim.profile_id == "profile:linux.portable")
        .expect("portable profile must be present");
    assert_eq!(portable.claim_state, ClaimState::Narrowed);
    assert!(portable
        .narrowed_dimensions
        .contains(&QualificationDimension::ProtocolHandlerOwnership));
    assert!(portable
        .narrowed_dimensions
        .contains(&QualificationDimension::FileAssociationOwnership));

    let macos = report
        .claim_scope
        .iter()
        .find(|claim| claim.profile_id == "profile:macos.stable")
        .expect("macos stable profile must be present");
    assert_eq!(macos.claim_state, ClaimState::Published);

    assert_eq!(report.published_claim_count, 5);
    assert_eq!(report.narrowed_claim_count, 1);
    assert_eq!(report.withheld_claim_count, 0);
    assert!(report.narrowable_marketed_profiles.is_empty());
}

#[test]
fn failed_dimensions_emit_distinct_failure_classes() {
    let profile_id = "profile:test.failures";
    let mut bindings = full_bindings(profile_id);
    for binding in &mut bindings {
        binding.status = QualificationStatus::Failed;
        binding.failure_mode = Some(binding.dimension.canonical_failure_mode());
        binding.evidence_pack_ref = None;
    }
    let row = build_profile_row(clean_descriptor(profile_id), bindings);
    let classes: std::collections::BTreeSet<&str> = row
        .blocking_findings
        .iter()
        .map(|finding| finding.class_token())
        .collect();
    for expected in [
        "ownership_unprovable",
        "protocol_handler_conflict",
        "file_association_conflict",
        "wrong_target_reopen",
        "lock_screen_leak",
        "missing_root_silent_loss",
        "store_lock_dead_end",
    ] {
        assert!(
            classes.contains(expected),
            "missing distinct failure class {expected}"
        );
    }
    // Every dimension failed, so nothing qualifies and the claim is withheld
    // rather than narrowed — the claim is never greener than the proof.
    assert_eq!(row.claim_state, ClaimState::Withheld);
}

#[test]
fn declared_failure_mode_drift_is_a_blocker() {
    let profile_id = "profile:test.drift";
    let mut bindings = full_bindings(profile_id);
    bindings[0].status = QualificationStatus::Failed;
    bindings[0].failure_mode = Some(QualificationFailureMode::LockScreenLeak);
    bindings[0].evidence_pack_ref = None;
    let row = build_profile_row(clean_descriptor(profile_id), bindings);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "failure_mode_drift"));
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "ownership_unprovable"));
}

#[test]
fn drill_kind_drift_is_a_blocker() {
    let profile_id = "profile:test.drill_drift";
    let mut bindings = full_bindings(profile_id);
    // Reopen fidelity bound to the wrong drill.
    bindings[3].required_drill = QualificationDrill::StoreLock;
    let row = build_profile_row(clean_descriptor(profile_id), bindings);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "drill_kind_drift"));
}

#[test]
fn borrowed_proof_across_profile_is_a_blocker() {
    let profile_id = "profile:test.borrowed";
    let mut bindings = full_bindings(profile_id);
    // Evidence pack names a *different* profile, so the row borrows proof.
    bindings[0].evidence_pack_ref =
        Some("evidence:profile:other.stable:channel_build_ownership".to_owned());
    let row = build_profile_row(clean_descriptor(profile_id), bindings);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "borrowed_proof_across_profile"));
}

#[test]
fn unqualified_marketed_dimension_is_a_blocker_and_narrows_claim() {
    let profile_id = "profile:test.unqualified";
    let mut bindings = full_bindings(profile_id);
    bindings[4].status = QualificationStatus::Unqualified;
    bindings[4].drill_ref = None;
    bindings[4].evidence_pack_ref = None;
    let row = build_profile_row(clean_descriptor(profile_id), bindings);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "unqualified_marketed_dimension"));
    assert_eq!(row.claim_state, ClaimState::Narrowed);
}

#[test]
fn narrowed_dimension_without_reason_is_a_blocker() {
    let profile_id = "profile:test.narrow";
    let mut bindings = full_bindings(profile_id);
    bindings[1].status = QualificationStatus::NotApplicable;
    bindings[1].drill_ref = None;
    bindings[1].evidence_pack_ref = None;
    bindings[1].narrowing_reason = None;
    let row = build_profile_row(clean_descriptor(profile_id), bindings);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "missing_narrowing_reason"));
}

#[test]
fn qualified_dimension_without_evidence_pack_is_a_blocker() {
    let profile_id = "profile:test.evidence";
    let mut bindings = full_bindings(profile_id);
    bindings[0].evidence_pack_ref = None;
    let row = build_profile_row(clean_descriptor(profile_id), bindings);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "missing_evidence_pack"));
}

#[test]
fn missing_required_dimension_is_a_blocker() {
    let profile_id = "profile:test.missing_dimension";
    let mut bindings = full_bindings(profile_id);
    bindings.remove(0);
    let row = build_profile_row(clean_descriptor(profile_id), bindings);
    assert!(row
        .blocking_findings
        .iter()
        .any(|finding| finding.class_token() == "missing_required_dimension"));
}

#[test]
fn profile_off_harness_and_missing_owner_are_blockers() {
    let profile_id = "profile:test.descriptor";
    let mut descriptor = clean_descriptor(profile_id);
    descriptor.registered_on_qualification_harness = false;
    descriptor.channel_build_owner_ref = String::new();
    let row = build_profile_row(descriptor, full_bindings(profile_id));
    let classes: std::collections::BTreeSet<&str> = row
        .blocking_findings
        .iter()
        .map(|finding| finding.class_token())
        .collect();
    assert!(classes.contains("profile_not_on_harness"));
    assert!(classes.contains("missing_channel_build_owner"));
}

#[test]
fn stale_evidence_on_marketed_profile_is_a_blocker_and_narrowable() {
    let profile_id = "profile:test.stale";
    let mut descriptor = clean_descriptor(profile_id);
    descriptor.evidence_freshness = EvidenceFreshness::Stale;
    let row = build_profile_row(descriptor, full_bindings(profile_id));
    let report = build_qualification_report(vec![row]);
    assert!(report
        .profiles
        .iter()
        .flat_map(|profile| &profile.blocking_findings)
        .any(|finding| finding.class_token() == "stale_evidence_on_marketed_profile"));
    assert!(report
        .narrowable_marketed_profiles
        .iter()
        .any(|narrowable| narrowable.profile_id == profile_id));
    // The stale profile narrows rather than publishing.
    assert!(report
        .claim_scope
        .iter()
        .any(|claim| claim.profile_id == profile_id && claim.claim_state == ClaimState::Narrowed));
}

#[test]
fn claim_state_can_never_be_greener_than_proof() {
    // A row with all dimensions unqualified withholds the claim.
    let profile_id = "profile:test.withheld";
    let mut bindings = full_bindings(profile_id);
    for binding in &mut bindings {
        binding.status = QualificationStatus::Unqualified;
        binding.drill_ref = None;
        binding.evidence_pack_ref = None;
    }
    let row = build_profile_row(clean_descriptor(profile_id), bindings);
    assert_eq!(row.claim_state, ClaimState::Withheld);
}

#[test]
fn validation_flags_missing_cross_link() {
    let mut report = seeded_native_desktop_qualification();
    report.cross_links.native_desktop_matrix_ref = String::new();
    let errors = validate_qualification_report(&report).expect_err("must flag missing cross-link");
    assert!(errors.iter().any(|err| matches!(
        err,
        QualificationValidationError::CrossLinkMissing { field }
            if field == "native_desktop_matrix_ref"
    )));
}

#[test]
fn validation_flags_claim_state_drift() {
    let mut report = seeded_native_desktop_qualification();
    // Force a published row to lie about being more qualified.
    let target = report
        .profiles
        .iter_mut()
        .find(|profile| profile.descriptor.profile_id == "profile:linux.portable")
        .expect("portable profile present");
    target.claim_state = ClaimState::Published;
    let errors = validate_qualification_report(&report).expect_err("must flag claim state drift");
    assert!(errors.iter().any(|err| matches!(
        err,
        QualificationValidationError::ClaimStateDrift { profile_id, .. }
            if profile_id == "profile:linux.portable"
    )));
}

#[test]
fn support_export_quotes_report_and_case_ids() {
    let report = seeded_native_desktop_qualification();
    let export = NativeDesktopQualificationSupportExport::from_report(
        QUALIFICATION_SUPPORT_EXPORT_ID,
        report.clone(),
    );
    assert!(export.case_ids.contains(&report.report_id));
    for profile in &report.profiles {
        assert!(export.case_ids.contains(&profile.descriptor.profile_id));
        assert!(export
            .case_ids
            .contains(&profile.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn claim_packet_partitions_profiles_by_claim_state() {
    let packet = seeded_qualification_claim_packet();
    assert!(packet
        .publishable_profiles
        .contains(&"profile:macos.stable".to_owned()));
    assert!(packet
        .narrowed_profiles
        .contains(&"profile:linux.portable".to_owned()));
    assert!(packet.withheld_profiles.is_empty());
    // A clean report with no withheld profiles is publishable.
    assert!(packet.claim_publishable);
    // Sorted partitions and one downgrade rule per profile.
    let mut sorted = packet.publishable_profiles.clone();
    sorted.sort();
    assert_eq!(packet.publishable_profiles, sorted);
    assert_eq!(packet.downgrade_rules.len(), packet.report.profiles.len());
}

#[test]
fn claim_packet_is_not_publishable_when_a_profile_is_withheld() {
    let profile_id = "profile:withheld.only";
    let mut bindings = full_bindings(profile_id);
    for binding in &mut bindings {
        binding.status = QualificationStatus::Unqualified;
        binding.drill_ref = None;
        binding.evidence_pack_ref = None;
    }
    let row = build_profile_row(clean_descriptor(profile_id), bindings);
    let report = build_qualification_report(vec![row]);
    let packet = NativeDesktopClaimPacket::from_report(QUALIFICATION_CLAIM_PACKET_ID, report);
    assert!(packet.withheld_profiles.contains(&profile_id.to_owned()));
    assert!(!packet.claim_publishable);
}

#[test]
fn compact_and_markdown_render_without_panicking() {
    let report = seeded_native_desktop_qualification();
    assert!(!report.compact_lines().is_empty());
    let markdown = report.render_markdown();
    assert!(markdown.contains("native-desktop qualification matrix"));
    assert!(markdown.contains("Claim scope"));
    let packet = seeded_qualification_claim_packet();
    let packet_md = packet.render_markdown();
    assert!(packet_md.contains("Shiproom claim packet"));
    assert!(packet_md.contains("Sign-off gate"));
}
