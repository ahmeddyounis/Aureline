//! Inline unit tests for the M5 advisory-claim downgrade certification proof.

use super::*;

#[test]
fn seeded_packet_covers_every_profile_and_is_clean() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    validate_m5_advisory_claim_downgrade_certification_packet(&packet)
        .expect("seeded packet must validate clean");

    assert_eq!(packet.rows.len(), REQUIRED_PROFILES.len());
    for profile in REQUIRED_PROFILES {
        assert!(
            packet.row(profile).is_some(),
            "missing row for {}",
            profile.as_str()
        );
    }
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn seeded_packet_has_one_green_and_two_yellow_rows() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    assert_eq!(packet.green_row_count, 1);
    assert_eq!(packet.yellow_row_count, 2);
    assert_eq!(packet.red_row_count, 0);

    assert_eq!(
        packet
            .row(M5AdvisoryClaimProfile::Managed)
            .unwrap()
            .derived_status,
        AdvisoryClaimStatus::Green,
        "managed should stay green"
    );
    for profile in [
        M5AdvisoryClaimProfile::SelfHosted,
        M5AdvisoryClaimProfile::Offline,
    ] {
        assert_eq!(
            packet.row(profile).unwrap().derived_status,
            AdvisoryClaimStatus::Yellow,
            "{} should auto-narrow to yellow",
            profile.as_str()
        );
    }
}

#[test]
fn every_row_status_states_and_causes_are_derived() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.profile.as_str()
        );
        assert_eq!(row.claim_states, row.recompute_claim_states());
        assert_eq!(row.claim_causes, row.recompute_causes());
    }
}

#[test]
fn every_row_evaluates_all_families_and_projects_all_channels() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    for row in &packet.rows {
        assert!(
            row.families_complete(),
            "row {} does not evaluate all six claimed advisory families",
            row.profile.as_str()
        );
        assert_eq!(row.evaluated_families.len(), REQUIRED_FAMILIES.len());
        assert!(
            row.channels_complete(),
            "row {} does not project into all five claim surfaces",
            row.profile.as_str()
        );
        assert_eq!(row.projected_channels.len(), REQUIRED_CHANNELS.len());
    }
}

#[test]
fn narrowed_rows_disclose_a_reason_and_a_distinct_state() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    for row in &packet.rows {
        if !matches!(row.derived_status, AdvisoryClaimStatus::Green) {
            assert!(
                row.narrowing_reason
                    .as_deref()
                    .map(|reason| !reason.trim().is_empty())
                    .unwrap_or(false),
                "narrowed row {} hides its reason",
                row.profile.as_str()
            );
            assert!(
                !row.claim_states.is_empty(),
                "narrowed row {} collapsed into a generic degraded state",
                row.profile.as_str()
            );
        } else {
            assert!(
                row.claim_states.is_empty(),
                "green row {} should carry no downgrade claim state",
                row.profile.as_str()
            );
        }
    }
}

#[test]
fn managed_self_hosted_and_offline_keep_distinct_downgrade_reasons() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let self_hosted = packet.row(M5AdvisoryClaimProfile::SelfHosted).unwrap();
    assert_eq!(
        self_hosted.claim_states,
        vec![
            M5AdvisoryClaimState::MirrorLagged,
            M5AdvisoryClaimState::UnsignedUnverified
        ]
    );
    let offline = packet.row(M5AdvisoryClaimProfile::Offline).unwrap();
    assert_eq!(
        offline.claim_states,
        vec![
            M5AdvisoryClaimState::WarningOnly,
            M5AdvisoryClaimState::AwaitingUserAction
        ]
    );
    // The two narrowed profiles do not share a downgrade reason.
    assert!(self_hosted
        .claim_states
        .iter()
        .all(|state| !offline.claim_states.contains(state)));
}

#[test]
fn every_cause_names_a_restore_action() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    for cause in &packet.claim_causes {
        assert!(
            !matches!(cause.restore_action, M5AdvisoryRestoreAction::NoneRequired),
            "cause {} on {} has no restore action",
            cause.cause_token(),
            cause.profile.as_str()
        );
    }
    // The self-hosted mirror-lag cause is restored by refreshing the mirror; the offline stale
    // notice by awaiting a refresh.
    assert!(packet
        .claim_causes
        .iter()
        .any(|cause| matches!(cause.restore_action, M5AdvisoryRestoreAction::RefreshMirror)));
    assert!(packet.claim_causes.iter().any(|cause| matches!(
        cause.restore_action,
        M5AdvisoryRestoreAction::AwaitNoticeRefresh
    )));
}

#[test]
fn disclosed_reduced_continuity_carries_an_active_waiver() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let offline = packet.row(M5AdvisoryClaimProfile::Offline).unwrap();
    assert!(matches!(
        offline.local_continuity,
        LocalContinuityProofState::DisclosedReducedContinuityProof
    ));
    assert!(offline.requires_waiver());
    assert!(offline.has_active_waiver());
    assert_eq!(packet.active_waivers.len(), 1);
    assert!(packet.active_waivers[0].is_active_at(&packet.generated_at));
}

#[test]
fn all_five_distinct_states_are_preserved_across_seed_and_fixtures() {
    // The clean seed exercises four of the five states; forced-disable appears only in a blocked
    // fixture (a red claim). Together the lane preserves every distinct downgrade path.
    let clean = seeded_m5_advisory_claim_downgrade_certification_packet();
    let blocked =
        seeded_m5_advisory_claim_downgrade_certification_packet_managed_continuity_lost_blocked();
    let mut observed: BTreeSet<M5AdvisoryClaimState> = BTreeSet::new();
    for packet in [&clean, &blocked] {
        for row in &packet.rows {
            observed.extend(row.claim_states.iter().copied());
        }
    }
    for state in M5AdvisoryClaimState::ALL {
        assert!(
            observed.contains(&state),
            "distinct claim state {} is never preserved",
            state.as_str()
        );
    }
}

#[test]
fn mirror_lag_blocks_the_self_hosted_profile() {
    let packet =
        seeded_m5_advisory_claim_downgrade_certification_packet_self_hosted_mirror_lag_blocked();
    let row = packet.row(M5AdvisoryClaimProfile::SelfHosted).unwrap();
    assert_eq!(row.derived_status, AdvisoryClaimStatus::Red);
    assert!(row
        .claim_states
        .contains(&M5AdvisoryClaimState::ForcedDisable));
    assert!(!packet.report_clean);
    assert!(!packet.all_rows_publishable);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AdvisoryClaimFinding::MirrorLaggedClaimOverclaimed { .. }
    )));
    assert!(row.claim_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5AdvisoryDowngradeTrigger::MirrorLagUndisclosed
    ) && !cause.disclosed));
    assert!(validate_m5_advisory_claim_downgrade_certification_packet(&packet).is_err());
}

#[test]
fn unsigned_distribution_blocks_the_self_hosted_profile() {
    let packet =
        seeded_m5_advisory_claim_downgrade_certification_packet_self_hosted_unsigned_blocked();
    let row = packet.row(M5AdvisoryClaimProfile::SelfHosted).unwrap();
    assert_eq!(row.derived_status, AdvisoryClaimStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AdvisoryClaimFinding::UnsignedOrUnverifiedDistribution { .. }
    )));
    assert!(row.claim_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5AdvisoryDowngradeTrigger::UnsignedDistributionUndisclosed
    ) && !cause.disclosed));
    assert!(validate_m5_advisory_claim_downgrade_certification_packet(&packet).is_err());
}

#[test]
fn stale_notice_blocks_the_offline_profile() {
    let packet =
        seeded_m5_advisory_claim_downgrade_certification_packet_offline_stale_notice_blocked();
    let row = packet.row(M5AdvisoryClaimProfile::Offline).unwrap();
    assert_eq!(row.derived_status, AdvisoryClaimStatus::Red);
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AdvisoryClaimFinding::AdvisoryStateStaleAndOverclaimed { .. }
    )));
    assert!(row.claim_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5AdvisoryDowngradeTrigger::StaleNoticeStateSilent
    ) && !cause.disclosed));
    assert!(validate_m5_advisory_claim_downgrade_certification_packet(&packet).is_err());
}

#[test]
fn lost_continuity_forces_disable_on_the_managed_profile() {
    let packet =
        seeded_m5_advisory_claim_downgrade_certification_packet_managed_continuity_lost_blocked();
    let row = packet.row(M5AdvisoryClaimProfile::Managed).unwrap();
    assert_eq!(row.derived_status, AdvisoryClaimStatus::Red);
    assert!(row
        .claim_states
        .contains(&M5AdvisoryClaimState::ForcedDisable));
    assert!(!packet.report_clean);
    assert!(packet.blocking_findings.iter().any(|finding| matches!(
        finding,
        AdvisoryClaimFinding::ContinuityProofMissingOrUnsafe { .. }
    )));
    assert!(row.claim_causes.iter().any(|cause| matches!(
        cause.trigger,
        M5AdvisoryDowngradeTrigger::LocalContinuityHidden
    ) && !cause.disclosed));
    assert!(validate_m5_advisory_claim_downgrade_certification_packet(&packet).is_err());
}

#[test]
fn incomplete_family_evaluation_blocks() {
    let mut packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile == M5AdvisoryClaimProfile::Managed)
        .unwrap();
    row.evaluated_families.pop();
    assert!(!row.families_complete());
    assert_eq!(row.recompute_status(), AdvisoryClaimStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        AdvisoryClaimFinding::EvaluatedFamiliesIncomplete { .. }
    )));
}

#[test]
fn incomplete_channel_projection_blocks() {
    let mut packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile == M5AdvisoryClaimProfile::Managed)
        .unwrap();
    row.projected_channels.pop();
    assert!(!row.channels_complete());
    assert_eq!(row.recompute_status(), AdvisoryClaimStatus::Red);
    let findings = row.compute_findings(&packet.generated_at);
    assert!(findings.iter().any(|finding| matches!(
        finding,
        AdvisoryClaimFinding::ProjectedChannelsIncomplete { .. }
    )));
}

#[test]
fn dashboard_projects_every_row_badge_and_counts() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let dashboard = packet.dashboard();
    assert_eq!(dashboard.rows.len(), packet.rows.len());
    assert_eq!(dashboard.green_row_count, packet.green_row_count);
    assert_eq!(dashboard.yellow_row_count, packet.yellow_row_count);
    assert_eq!(dashboard.red_row_count, packet.red_row_count);
    assert_eq!(dashboard.all_rows_publishable, packet.all_rows_publishable);
    assert_eq!(dashboard.source_packet_ref, packet.packet_id);
    assert_eq!(dashboard.covered_claim_states, packet.covered_claim_states);
    assert!(!dashboard.claim_automation_refs.is_empty());

    let offline = dashboard
        .rows
        .iter()
        .find(|row| row.profile == M5AdvisoryClaimProfile::Offline)
        .unwrap();
    assert_eq!(offline.status, AdvisoryClaimStatus::Yellow);
    assert_eq!(offline.controlled_badge_token, "advisory_claim_narrowed");
    assert!(offline.has_active_waiver);
    assert!(offline.restore_actions.contains(
        &M5AdvisoryRestoreAction::AcknowledgeOrAct
            .as_str()
            .to_owned()
    ));

    let managed = dashboard
        .rows
        .iter()
        .find(|row| row.profile == M5AdvisoryClaimProfile::Managed)
        .unwrap();
    assert_eq!(managed.controlled_badge_token, "advisory_claim_current");
}

#[test]
fn support_export_quotes_packet_matrix_and_waiver_refs() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let export = AdvisoryClaimSupportExport::from_packet(
        M5_ADVISORY_CLAIM_DOWNGRADE_SUPPORT_EXPORT_ID,
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    assert!(export.case_ids.contains(&packet.build_identity_ref));
    for row in &packet.rows {
        assert!(export.case_ids.contains(&row.profile.as_str().to_owned()));
    }
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
    assert_eq!(export.dashboard, packet.dashboard());
}

#[test]
fn markdown_and_csv_name_every_profile() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let markdown = packet.render_markdown();
    let csv = packet.render_matrix_csv();
    for profile in REQUIRED_PROFILES {
        assert!(
            markdown.contains(profile.label()),
            "markdown omits {}",
            profile.as_str()
        );
        assert!(
            csv.contains(profile.as_str()),
            "csv omits {}",
            profile.as_str()
        );
    }
    assert!(markdown.contains("m5_advisory_claim_downgrade_certification_fixtures"));
    assert!(markdown.contains("waiver:offline-reduced-continuity-proof:0001"));
}

#[test]
fn export_carries_no_forbidden_material() {
    let packet = seeded_m5_advisory_claim_downgrade_certification_packet();
    let json = packet.export_safe_json().to_lowercase();
    assert!(!json.contains("api_key"));
    assert!(!json.contains("password"));
    assert!(!json.contains("bearer "));
    assert!(!json.contains("://"));
}

#[test]
fn expired_waiver_is_flagged() {
    let waiver = AdvisoryClaimWaiver {
        waiver_id: "waiver:expired:0001".to_owned(),
        profile: M5AdvisoryClaimProfile::Offline,
        reason: "test".to_owned(),
        owner_role: "owner".to_owned(),
        expires_at: "2026-01-01T00:00:00Z".to_owned(),
    };
    assert!(!waiver.is_active_at("2026-06-30T00:00:00Z"));
    assert!(waiver.is_active_at("2025-01-01T00:00:00Z"));
}
