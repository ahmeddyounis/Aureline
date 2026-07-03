use super::*;

fn emg(
    local_work_state: M5EmergencyLocalWorkState,
    dismissal_policy: M5EmergencyDismissalPolicy,
) -> M5EmergencyBannerResolutionInput {
    M5EmergencyBannerResolutionInput {
        reason_class: M5EmergencyReasonClass::CapabilityKillSwitch,
        notice_id: "AURELINE-EMG-2026-0001".to_owned(),
        severity: M5AdvisorySeverityClass::Critical,
        affected_capability_repr: "extension:code-lens:network-capability".to_owned(),
        blast_radius_repr: "blast_radius:single_extension_capability".to_owned(),
        local_work_state,
        deadline_repr: "deadline:acknowledge_within_24h".to_owned(),
        recovery_repr: "recovery:await_signed_replacement".to_owned(),
        signer_source_state_repr: "signer_source_state:signed_current".to_owned(),
        action_state: M5AdvisoryActionState::ImmediateRemediation,
        primary_action: M5AdvisoryRequiredAction::DisableOrRemove,
        recovery_action: M5AdvisoryRequiredAction::WaitForSupersedingAction,
        continuity_claim: M5AdvisoryContinuityClaim::RequiresDisablingAffectedProfile,
        dismissal_policy,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_derives_posture_dismissal_and_keeps_banner_visible() {
    let resolved = resolve_emergency_banner(&emg(
        M5EmergencyLocalWorkState::AffectedCapabilitySuspended,
        M5EmergencyDismissalPolicy::NotDismissableBlocked,
    ))
    .expect("resolves");

    assert_eq!(
        resolved.continuity_posture,
        M5EmergencyContinuityPosture::AffectedCapabilitySuspendedLocalSafe
    );
    assert!(resolved.local_work_safe);
    assert!(!resolved.implies_data_loss);
    assert!(resolved.remains_visible);
    // The blocked-until-remediated policy resolves to a must-acknowledge state and
    // forbids snooze / dismiss — not one generic close button.
    assert_eq!(
        resolved.dismissal_state,
        M5AdvisoryDismissalState::BlockedUntilRemediated
    );
    assert_eq!(
        resolved.allowed_dismissal_actions,
        vec![M5EmergencyDismissalAction::Acknowledge]
    );
    // Every channel is projected with identical core truth.
    assert_eq!(
        resolved.channel_projections.len(),
        M5EmergencyBannerChannel::ALL.len()
    );
    for projection in &resolved.channel_projections {
        assert_eq!(projection.severity, resolved.severity);
        assert_eq!(projection.continuity_posture, resolved.continuity_posture);
        assert_eq!(projection.primary_action, resolved.primary_action);
        assert_eq!(projection.dismissal_state, resolved.dismissal_state);
    }
    // The export summary carries every mandatory column with a populated value.
    assert_eq!(
        resolved.export_summary.columns.len(),
        MANDATORY_EXPORT_FIELDS.len()
    );
    assert_eq!(resolved.export_summary.notice_id, resolved.notice_id);
    assert!(resolved
        .export_summary
        .columns
        .iter()
        .all(|c| !c.value.trim().is_empty()));
}

#[test]
fn resolver_maps_every_local_work_state_to_its_posture() {
    let expected = [
        (
            M5EmergencyLocalWorkState::EditingReviewExportSafe,
            M5EmergencyContinuityPosture::LocalWorkContinuesSafely,
        ),
        (
            M5EmergencyLocalWorkState::DegradedButSafe,
            M5EmergencyContinuityPosture::LocalWorkContinuesDegraded,
        ),
        (
            M5EmergencyLocalWorkState::AffectedCapabilitySuspended,
            M5EmergencyContinuityPosture::AffectedCapabilitySuspendedLocalSafe,
        ),
        (
            M5EmergencyLocalWorkState::BlockedPendingAcknowledgement,
            M5EmergencyContinuityPosture::BlockedPendingAcknowledgement,
        ),
        (
            M5EmergencyLocalWorkState::DataLossConfirmed,
            M5EmergencyContinuityPosture::DataLossProven,
        ),
        (
            M5EmergencyLocalWorkState::ContinuityNotYetDetermined,
            M5EmergencyContinuityPosture::ContinuityAssessmentPending,
        ),
    ];
    for (state, posture) in expected {
        let resolved =
            resolve_emergency_banner(&emg(state, M5EmergencyDismissalPolicy::FullyDismissible))
                .expect("resolves");
        assert_eq!(
            resolved.continuity_posture,
            posture,
            "local work {}",
            state.as_str()
        );
    }
}

#[test]
fn resolver_implies_data_loss_only_when_the_event_proves_it() {
    for state in M5EmergencyLocalWorkState::ALL {
        let resolved =
            resolve_emergency_banner(&emg(state, M5EmergencyDismissalPolicy::AcknowledgeRequired))
                .expect("resolves");
        assert_eq!(
            resolved.implies_data_loss,
            state == M5EmergencyLocalWorkState::DataLossConfirmed,
            "local work {}",
            state.as_str()
        );
        // The banner always stays visible regardless of the local-work state.
        assert!(resolved.remains_visible);
    }
}

#[test]
fn resolver_derives_dismissal_actions_from_policy() {
    let expected = [
        (
            M5EmergencyDismissalPolicy::NotDismissableBlocked,
            vec![M5EmergencyDismissalAction::Acknowledge],
        ),
        (
            M5EmergencyDismissalPolicy::AcknowledgeRequired,
            vec![M5EmergencyDismissalAction::Acknowledge],
        ),
        (
            M5EmergencyDismissalPolicy::AcknowledgeOrSnooze,
            vec![
                M5EmergencyDismissalAction::Acknowledge,
                M5EmergencyDismissalAction::Snooze,
            ],
        ),
        (
            M5EmergencyDismissalPolicy::FullyDismissible,
            vec![
                M5EmergencyDismissalAction::Acknowledge,
                M5EmergencyDismissalAction::Snooze,
                M5EmergencyDismissalAction::Dismiss,
            ],
        ),
        (
            M5EmergencyDismissalPolicy::InformationalDismissible,
            vec![M5EmergencyDismissalAction::Dismiss],
        ),
    ];
    for (policy, actions) in expected {
        let resolved =
            resolve_emergency_banner(&emg(M5EmergencyLocalWorkState::DegradedButSafe, policy))
                .expect("resolves");
        assert_eq!(
            resolved.allowed_dismissal_actions,
            actions,
            "policy {}",
            policy.as_str()
        );
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let mut e = emg(
        M5EmergencyLocalWorkState::EditingReviewExportSafe,
        M5EmergencyDismissalPolicy::FullyDismissible,
    );
    e.notice_id = "  ".to_owned();
    assert_eq!(
        resolve_emergency_banner(&e),
        Err(M5EmergencyBannerResolutionError::EmptyNoticeId)
    );

    let mut e = emg(
        M5EmergencyLocalWorkState::EditingReviewExportSafe,
        M5EmergencyDismissalPolicy::FullyDismissible,
    );
    e.affected_capability_repr = "".to_owned();
    assert_eq!(
        resolve_emergency_banner(&e),
        Err(M5EmergencyBannerResolutionError::EmptyAffectedCapability)
    );

    let mut e = emg(
        M5EmergencyLocalWorkState::EditingReviewExportSafe,
        M5EmergencyDismissalPolicy::FullyDismissible,
    );
    e.blast_radius_repr = "".to_owned();
    assert_eq!(
        resolve_emergency_banner(&e),
        Err(M5EmergencyBannerResolutionError::EmptyBlastRadius)
    );

    let mut e = emg(
        M5EmergencyLocalWorkState::EditingReviewExportSafe,
        M5EmergencyDismissalPolicy::FullyDismissible,
    );
    e.deadline_repr = "  ".to_owned();
    assert_eq!(
        resolve_emergency_banner(&e),
        Err(M5EmergencyBannerResolutionError::EmptyDeadline)
    );

    let mut e = emg(
        M5EmergencyLocalWorkState::EditingReviewExportSafe,
        M5EmergencyDismissalPolicy::FullyDismissible,
    );
    e.recovery_repr = "".to_owned();
    assert_eq!(
        resolve_emergency_banner(&e),
        Err(M5EmergencyBannerResolutionError::EmptyRecovery)
    );

    let mut e = emg(
        M5EmergencyLocalWorkState::EditingReviewExportSafe,
        M5EmergencyDismissalPolicy::FullyDismissible,
    );
    e.signer_source_state_repr = "".to_owned();
    assert_eq!(
        resolve_emergency_banner(&e),
        Err(M5EmergencyBannerResolutionError::EmptySignerSourceState)
    );

    let mut e = emg(
        M5EmergencyLocalWorkState::EditingReviewExportSafe,
        M5EmergencyDismissalPolicy::FullyDismissible,
    );
    e.blast_radius_repr = "reach https://example.test".to_owned();
    assert_eq!(
        resolve_emergency_banner(&e),
        Err(M5EmergencyBannerResolutionError::ForbiddenMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_emergency_notice_banner_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_EMERGENCY_BANNER_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_reason_class() {
    let packet = seeded_m5_emergency_notice_banner_primitive_packet();
    let present: std::collections::BTreeSet<_> =
        packet.reason_rows.iter().map(|r| r.reason_class).collect();
    for reason in M5EmergencyReasonClass::ALL {
        assert!(
            present.contains(&reason),
            "missing reason-class lane {}",
            reason.as_str()
        );
    }
    assert_eq!(packet.reason_rows.len(), M5EmergencyReasonClass::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_channels_and_export() {
    let packet = seeded_m5_emergency_notice_banner_primitive_packet();
    for row in &packet.reason_rows {
        for part in M5EmergencyBannerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for channel in M5EmergencyBannerChannel::ALL {
            assert!(row.channels.contains(&channel));
        }
        for field in MANDATORY_EXPORT_FIELDS {
            assert!(row.export_fields.contains(&field));
        }
        assert!(!row.dismissal_policies.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_notices.is_empty());
    }
}

#[test]
fn every_severity_and_posture_is_exercised_by_some_example() {
    let packet = seeded_m5_emergency_notice_banner_primitive_packet();
    let banners: Vec<&M5ResolvedEmergencyBanner> = packet
        .reason_rows
        .iter()
        .flat_map(|row| row.example_notices.iter())
        .map(|case| &case.resolved)
        .collect();

    for severity in M5AdvisorySeverityClass::ALL {
        assert!(
            banners.iter().any(|b| b.severity == severity),
            "no worked resolution exercises severity {}",
            severity.as_str()
        );
    }
    for posture in M5EmergencyContinuityPosture::ALL {
        assert!(
            banners.iter().any(|b| b.continuity_posture == posture),
            "no worked resolution exercises posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn some_example_keeps_local_work_safe_without_data_loss() {
    let packet = seeded_m5_emergency_notice_banner_primitive_packet();
    let proven = packet
        .reason_rows
        .iter()
        .flat_map(|row| row.example_notices.iter())
        .any(|case| case.resolved.local_work_safe && !case.resolved.implies_data_loss);
    assert!(
        proven,
        "no worked resolution keeps local work safe without implying data loss"
    );
}

#[test]
fn only_the_data_loss_example_implies_loss() {
    let packet = seeded_m5_emergency_notice_banner_primitive_packet();
    let loss_examples: Vec<&M5ResolvedEmergencyBanner> = packet
        .reason_rows
        .iter()
        .flat_map(|row| row.example_notices.iter())
        .map(|case| &case.resolved)
        .filter(|b| b.implies_data_loss)
        .collect();
    assert_eq!(
        loss_examples.len(),
        1,
        "exactly one example proves data loss"
    );
    assert_eq!(
        loss_examples[0].local_work_state,
        M5EmergencyLocalWorkState::DataLossConfirmed
    );
}

#[test]
fn dismissal_rules_are_explicit_and_not_one_generic_close() {
    let packet = seeded_m5_emergency_notice_banner_primitive_packet();
    let banners: Vec<&M5ResolvedEmergencyBanner> = packet
        .reason_rows
        .iter()
        .flat_map(|row| row.example_notices.iter())
        .map(|case| &case.resolved)
        .collect();
    let actions: std::collections::BTreeSet<_> = banners
        .iter()
        .flat_map(|b| b.allowed_dismissal_actions.iter().copied())
        .collect();
    for action in M5EmergencyDismissalAction::ALL {
        assert!(
            actions.contains(&action),
            "no worked resolution offers dismissal action {}",
            action.as_str()
        );
    }
    assert!(
        banners.iter().any(|b| !b
            .allowed_dismissal_actions
            .contains(&M5EmergencyDismissalAction::Dismiss)),
        "no worked resolution forbids an outright dismiss"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_emergency_notice_banner_primitive_packet();
    for row in &packet.reason_rows {
        for case in &row.example_notices {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.reason_class.as_str()
            );
        }
    }
}

#[test]
fn missing_reason_class_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet
        .reason_rows
        .retain(|row| row.reason_class != M5EmergencyReasonClass::ChannelFreeze);
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::RequiredReasonMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.vocabulary_set.channels.pop();
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.reason_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5EmergencyBannerAnatomyPart::BlastRadius);
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn channel_parity_mismatch_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.reason_rows[0]
        .channels
        .retain(|c| *c != M5EmergencyBannerChannel::SupportBundle);
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::ChannelParityMismatch));
}

#[test]
fn dismissal_policy_missing_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.reason_rows[0].dismissal_policies.clear();
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::DismissalPolicyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.reason_rows[0]
        .export_fields
        .retain(|f| *f != M5AdvisoryExportField::ContinuityNote);
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_notice_drift_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.reason_rows[0].example_notices[0]
        .resolved
        .blast_radius_repr = "tampered".to_owned();
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::ExampleNoticeDrift));
}

#[test]
fn example_notice_missing_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.reason_rows[2].example_notices.clear();
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::ExampleNoticeMissing));
}

#[test]
fn channel_parity_unproven_fails_when_examples_drop_a_channel() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    for row in &mut packet.reason_rows {
        for case in &mut row.example_notices {
            case.resolved
                .channel_projections
                .retain(|p| p.channel == M5EmergencyBannerChannel::UpdateCenter);
        }
    }
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::ChannelParityUnproven));
}

#[test]
fn local_safe_continuity_unproven_fails_when_no_example_is_safe() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    // Rewrite every example so the event proves data loss → no local-safe survives.
    for row in &mut packet.reason_rows {
        for case in &mut row.example_notices {
            case.input.local_work_state = M5EmergencyLocalWorkState::DataLossConfirmed;
            *case = M5EmergencyBannerResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::LocalSafeContinuityUnproven));
}

#[test]
fn dismissal_rule_unproven_fails_when_every_example_is_fully_dismissible() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    // Every example fully dismissible → no non-dismissable event survives.
    for row in &mut packet.reason_rows {
        for case in &mut row.example_notices {
            case.input.dismissal_policy = M5EmergencyDismissalPolicy::FullyDismissible;
            *case = M5EmergencyBannerResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::DismissalRuleUnproven));
}

#[test]
fn severity_coverage_unproven_fails_when_examples_drop_a_severity() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    for row in &mut packet.reason_rows {
        for case in &mut row.example_notices {
            case.input.severity = M5AdvisorySeverityClass::High;
            *case = M5EmergencyBannerResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::SeverityCoverageUnproven));
}

#[test]
fn continuity_posture_coverage_unproven_fails_when_examples_drop_a_posture() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    for row in &mut packet.reason_rows {
        for case in &mut row.example_notices {
            case.input.local_work_state = M5EmergencyLocalWorkState::DegradedButSafe;
            *case = M5EmergencyBannerResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::ContinuityPostureCoverageUnproven));
}

#[test]
fn reason_invariant_violation_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.reason_rows[0].implies_data_loss_without_proof = true;
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::ReasonInvariantViolated));
}

#[test]
fn stable_reason_missing_proof_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.reason_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::StableReasonMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet
        .governance_review
        .never_implies_data_loss_without_proof = false;
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet
        .consumer_projection
        .native_notification_renders_shared_banner = false;
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5EmergencyBannerPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_reason_class() {
    let summary = seeded_m5_emergency_notice_banner_primitive_packet().render_markdown_summary();
    for reason in M5EmergencyReasonClass::ALL {
        assert!(
            summary.contains(reason.label()),
            "summary missing reason-class lane {}",
            reason.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_reason_class() {
    let csv = seeded_m5_emergency_notice_banner_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5EmergencyReasonClass::ALL.len());
    assert!(lines[0].starts_with("reason_class,qualification,owner,"));
    for reason in M5EmergencyReasonClass::ALL {
        assert!(
            csv.contains(reason.as_str()),
            "csv missing reason-class lane {}",
            reason.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_emergency_notice_banner_primitive_export()
        .expect("checked M5 emergency-banner primitive export validates");
    assert_eq!(from_disk.packet_id, M5_EMERGENCY_BANNER_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_emergency_notice_banner_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lanes_visible() {
    for packet in [
        seeded_m5_emergency_notice_banner_primitive_forced_disable_beta_narrowed(),
        seeded_m5_emergency_notice_banner_primitive_signed_emergency_bundle_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.reason_rows.len(), M5EmergencyReasonClass::ALL.len());
    }

    let forced_disable = seeded_m5_emergency_notice_banner_primitive_forced_disable_beta_narrowed();
    let row = forced_disable
        .reason_rows
        .iter()
        .find(|r| r.reason_class == M5EmergencyReasonClass::ForcedDisable)
        .expect("forced-disable row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Beta);

    let signed =
        seeded_m5_emergency_notice_banner_primitive_signed_emergency_bundle_preview_narrowed();
    let row = signed
        .reason_rows
        .iter()
        .find(|r| r.reason_class == M5EmergencyReasonClass::SignedEmergencyBundle)
        .expect("signed-emergency-bundle row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let forced_disable: M5EmergencyBannerPrimitivePacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/security/m5-emergency-notice-banner-primitive/forced_disable_beta_narrowed.json"
        )
    ))
    .expect("forced-disable fixture parses");
    assert!(forced_disable.validate().is_empty());
    assert_eq!(
        forced_disable,
        seeded_m5_emergency_notice_banner_primitive_forced_disable_beta_narrowed()
    );

    let signed: M5EmergencyBannerPrimitivePacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/security/m5-emergency-notice-banner-primitive/signed_emergency_bundle_preview_narrowed.json"
        )
    ))
    .expect("signed-emergency-bundle fixture parses");
    assert!(signed.validate().is_empty());
    assert_eq!(
        signed,
        seeded_m5_emergency_notice_banner_primitive_signed_emergency_bundle_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_emergency_notice_banner_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
