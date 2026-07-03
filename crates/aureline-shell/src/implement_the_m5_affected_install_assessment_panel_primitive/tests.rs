use super::*;

fn assessment(
    install_state: M5AdvisoryInstallState,
    mirror_freshness: M5AdvisoryFreshnessState,
) -> M5AffectedInstallResolutionInput {
    M5AffectedInstallResolutionInput {
        install_profile: M5InstallProfileLane::PerUserInstalled,
        advisory_id: "AURELINE-ADV-2026-0001".to_owned(),
        severity: M5AdvisorySeverityClass::Critical,
        affected_object_repr: "artifact:aureline-desktop".to_owned(),
        build_identity_repr: "build_identity:2026.6.0+stable".to_owned(),
        impacted_components_repr: "impacted_components:renderer-core".to_owned(),
        install_state,
        mirror_freshness,
        delivery_profile: M5AdvisoryDeliveryProfile::LocalOnly,
        fixed_build_or_mitigation_repr: "fixed_build:2026.6.1+stable".to_owned(),
        signer_source_state_repr: "signer_source_state:signed_current".to_owned(),
        action_state: M5AdvisoryActionState::ImmediateRemediation,
        primary_action: M5AdvisoryRequiredAction::RollbackOrRepin,
        help_action: M5AdvisoryRequiredAction::ExportSupportPacket,
        continuity_claim: M5AdvisoryContinuityClaim::NoSafeLocalContinuity,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_derives_exposure_verdict_and_keeps_panel_visible() {
    let resolved = resolve_affected_install(&assessment(
        M5AdvisoryInstallState::InstalledActive,
        M5AdvisoryFreshnessState::UpToDate,
    ))
    .expect("resolves");

    assert_eq!(resolved.exposure_state, M5AdvisoryExposureState::Exposed);
    assert_eq!(
        resolved.assessment_verdict,
        M5InstallAssessmentVerdict::Affected
    );
    assert!(resolved.installed_but_affected);
    assert!(resolved.resolved_from_local_graph);
    assert!(!resolved.requires_external_website_lookup);
    assert!(resolved.mirror_freshness_visible);
    assert!(resolved.install_mode_visible);
    assert!(resolved.actions_attached_to_panel);
    assert!(resolved.remains_visible);
    // The rollback / repin and support-export actions stay attached to the panel.
    assert_eq!(
        resolved.attached_actions,
        vec![
            M5AdvisoryRequiredAction::RollbackOrRepin,
            M5AdvisoryRequiredAction::ExportSupportPacket
        ]
    );
    // Every channel is projected with identical core truth.
    assert_eq!(
        resolved.channel_projections.len(),
        M5AffectedInstallChannel::ALL.len()
    );
    for projection in &resolved.channel_projections {
        assert_eq!(projection.assessment_verdict, resolved.assessment_verdict);
        assert_eq!(projection.mirror_freshness, resolved.mirror_freshness);
        assert_eq!(projection.install_profile, resolved.install_profile);
        assert_eq!(projection.primary_action, resolved.primary_action);
    }
    // The export summary carries every mandatory column with a populated value,
    // including the install mode (delivery profile) and mirror freshness.
    assert_eq!(
        resolved.export_summary.columns.len(),
        MANDATORY_EXPORT_FIELDS.len()
    );
    assert_eq!(resolved.export_summary.advisory_id, resolved.advisory_id);
    assert!(resolved
        .export_summary
        .columns
        .iter()
        .all(|c| !c.value.trim().is_empty()));
    assert!(resolved
        .export_summary
        .columns
        .iter()
        .any(|c| c.field == M5AdvisoryExportField::FreshnessState));
    assert!(resolved
        .export_summary
        .columns
        .iter()
        .any(|c| c.field == M5AdvisoryExportField::DeliveryProfile));
}

#[test]
fn resolver_maps_every_install_state_to_its_exposure() {
    let expected = [
        (
            M5AdvisoryInstallState::InstalledActive,
            M5AdvisoryExposureState::Exposed,
        ),
        (
            M5AdvisoryInstallState::InstalledMitigated,
            M5AdvisoryExposureState::MitigatedInPlace,
        ),
        (
            M5AdvisoryInstallState::InstalledBlocked,
            M5AdvisoryExposureState::ContainedByBlock,
        ),
        (
            M5AdvisoryInstallState::InstalledDisabled,
            M5AdvisoryExposureState::ContainedByDisable,
        ),
        (
            M5AdvisoryInstallState::InstalledAwaitingRollback,
            M5AdvisoryExposureState::AwaitingRollback,
        ),
        (
            M5AdvisoryInstallState::NotInstalled,
            M5AdvisoryExposureState::NotAffected,
        ),
        (
            M5AdvisoryInstallState::Superseded,
            M5AdvisoryExposureState::Resolved,
        ),
    ];
    for (state, exposure) in expected {
        let resolved =
            resolve_affected_install(&assessment(state, M5AdvisoryFreshnessState::UpToDate))
                .expect("resolves");
        assert_eq!(
            resolved.exposure_state,
            exposure,
            "install {}",
            state.as_str()
        );
    }
}

#[test]
fn stale_mirror_auto_narrows_clean_verdict_but_not_an_active_exposure() {
    // A clean exposure (not installed) over a stale mirror narrows to the
    // mirror-refresh-pending verdict rather than staying silently "not affected".
    for freshness in [
        M5AdvisoryFreshnessState::StalePastGrace,
        M5AdvisoryFreshnessState::OfflineExpired,
        M5AdvisoryFreshnessState::Unknown,
    ] {
        let resolved =
            resolve_affected_install(&assessment(M5AdvisoryInstallState::NotInstalled, freshness))
                .expect("resolves");
        assert_eq!(
            resolved.assessment_verdict,
            M5InstallAssessmentVerdict::CleanPendingMirrorRefresh,
            "freshness {}",
            freshness.as_str()
        );
    }
    // An active exposure is never softened by mirror staleness — it stays `affected`.
    let resolved = resolve_affected_install(&assessment(
        M5AdvisoryInstallState::InstalledActive,
        M5AdvisoryFreshnessState::OfflineExpired,
    ))
    .expect("resolves");
    assert_eq!(
        resolved.assessment_verdict,
        M5InstallAssessmentVerdict::Affected
    );
}

#[test]
fn authoritative_mirror_keeps_clean_verdict() {
    for freshness in [
        M5AdvisoryFreshnessState::UpToDate,
        M5AdvisoryFreshnessState::StaleWithinGrace,
    ] {
        assert!(freshness_is_authoritative(freshness));
        let resolved =
            resolve_affected_install(&assessment(M5AdvisoryInstallState::NotInstalled, freshness))
                .expect("resolves");
        assert_eq!(
            resolved.assessment_verdict,
            M5InstallAssessmentVerdict::NotAffected,
            "freshness {}",
            freshness.as_str()
        );
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let mut e = assessment(
        M5AdvisoryInstallState::InstalledActive,
        M5AdvisoryFreshnessState::UpToDate,
    );
    e.advisory_id = "  ".to_owned();
    assert_eq!(
        resolve_affected_install(&e),
        Err(M5AffectedInstallResolutionError::EmptyAdvisoryId)
    );

    let mut e = assessment(
        M5AdvisoryInstallState::InstalledActive,
        M5AdvisoryFreshnessState::UpToDate,
    );
    e.affected_object_repr = "".to_owned();
    assert_eq!(
        resolve_affected_install(&e),
        Err(M5AffectedInstallResolutionError::EmptyAffectedObject)
    );

    let mut e = assessment(
        M5AdvisoryInstallState::InstalledActive,
        M5AdvisoryFreshnessState::UpToDate,
    );
    e.build_identity_repr = "".to_owned();
    assert_eq!(
        resolve_affected_install(&e),
        Err(M5AffectedInstallResolutionError::EmptyBuildIdentity)
    );

    let mut e = assessment(
        M5AdvisoryInstallState::InstalledActive,
        M5AdvisoryFreshnessState::UpToDate,
    );
    e.impacted_components_repr = "  ".to_owned();
    assert_eq!(
        resolve_affected_install(&e),
        Err(M5AffectedInstallResolutionError::EmptyImpactedComponents)
    );

    let mut e = assessment(
        M5AdvisoryInstallState::InstalledActive,
        M5AdvisoryFreshnessState::UpToDate,
    );
    e.fixed_build_or_mitigation_repr = "".to_owned();
    assert_eq!(
        resolve_affected_install(&e),
        Err(M5AffectedInstallResolutionError::EmptyFixedBuildOrMitigation)
    );

    let mut e = assessment(
        M5AdvisoryInstallState::InstalledActive,
        M5AdvisoryFreshnessState::UpToDate,
    );
    e.signer_source_state_repr = "".to_owned();
    assert_eq!(
        resolve_affected_install(&e),
        Err(M5AffectedInstallResolutionError::EmptySignerSourceState)
    );

    let mut e = assessment(
        M5AdvisoryInstallState::InstalledActive,
        M5AdvisoryFreshnessState::UpToDate,
    );
    e.build_identity_repr = "reach https://example.test".to_owned();
    assert_eq!(
        resolve_affected_install(&e),
        Err(M5AffectedInstallResolutionError::ForbiddenMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_affected_install_panel_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_install_profile() {
    let packet = seeded_m5_affected_install_panel_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .install_rows
        .iter()
        .map(|r| r.install_profile)
        .collect();
    for profile in M5InstallProfileLane::ALL {
        assert!(
            present.contains(&profile),
            "missing install-profile lane {}",
            profile.as_str()
        );
    }
    assert_eq!(packet.install_rows.len(), M5InstallProfileLane::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_channels_and_export() {
    let packet = seeded_m5_affected_install_panel_primitive_packet();
    for row in &packet.install_rows {
        for part in M5AffectedInstallAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for channel in M5AffectedInstallChannel::ALL {
            assert!(row.channels.contains(&channel));
        }
        for field in MANDATORY_EXPORT_FIELDS {
            assert!(row.export_fields.contains(&field));
        }
        assert!(!row.delivery_profiles.is_empty());
        assert!(!row.freshness_states.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_assessments.is_empty());
    }
}

#[test]
fn every_verdict_and_severity_is_exercised_by_some_example() {
    let packet = seeded_m5_affected_install_panel_primitive_packet();
    let panels: Vec<&M5ResolvedAffectedInstall> = packet
        .install_rows
        .iter()
        .flat_map(|row| row.example_assessments.iter())
        .map(|case| &case.resolved)
        .collect();

    for verdict in M5InstallAssessmentVerdict::ALL {
        assert!(
            panels.iter().any(|p| p.assessment_verdict == verdict),
            "no worked resolution exercises verdict {}",
            verdict.as_str()
        );
    }
    for severity in M5AdvisorySeverityClass::ALL {
        assert!(
            panels.iter().any(|p| p.severity == severity),
            "no worked resolution exercises severity {}",
            severity.as_str()
        );
    }
}

#[test]
fn some_example_resolves_installed_but_affected_against_local_graph() {
    let packet = seeded_m5_affected_install_panel_primitive_packet();
    let proven = packet
        .install_rows
        .iter()
        .flat_map(|row| row.example_assessments.iter())
        .any(|case| {
            case.resolved.installed_but_affected
                && case.resolved.resolved_from_local_graph
                && !case.resolved.requires_external_website_lookup
        });
    assert!(
        proven,
        "no worked resolution resolves an installed-but-affected build against the local graph"
    );
}

#[test]
fn some_example_proves_stale_mirror_auto_narrows() {
    let packet = seeded_m5_affected_install_panel_primitive_packet();
    let proven = packet
        .install_rows
        .iter()
        .flat_map(|row| row.example_assessments.iter())
        .any(|case| {
            !freshness_is_authoritative(case.resolved.mirror_freshness)
                && case.resolved.assessment_verdict
                    == M5InstallAssessmentVerdict::CleanPendingMirrorRefresh
        });
    assert!(
        proven,
        "no worked resolution proves a stale mirror auto-narrows a clean verdict"
    );
}

#[test]
fn attached_actions_cover_rollback_and_help() {
    let packet = seeded_m5_affected_install_panel_primitive_packet();
    let actions: std::collections::BTreeSet<_> = packet
        .install_rows
        .iter()
        .flat_map(|row| row.example_assessments.iter())
        .flat_map(|case| case.resolved.attached_actions.iter().copied())
        .collect();
    assert!(actions.contains(&M5AdvisoryRequiredAction::RollbackOrRepin));
    assert!(
        actions.contains(&M5AdvisoryRequiredAction::ExportSupportPacket)
            || actions.contains(&M5AdvisoryRequiredAction::ContactAdmin)
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_affected_install_panel_primitive_packet();
    for row in &packet.install_rows {
        for case in &row.example_assessments {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.install_profile.as_str()
            );
        }
    }
}

#[test]
fn missing_install_profile_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet
        .install_rows
        .retain(|row| row.install_profile != M5InstallProfileLane::Portable);
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::RequiredInstallProfileMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.vocabulary_set.channels.pop();
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.install_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5AffectedInstallAnatomyPart::MirrorFreshness);
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::MandatoryAnatomyMissing));
}

#[test]
fn channel_parity_mismatch_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.install_rows[0]
        .channels
        .retain(|c| *c != M5AffectedInstallChannel::AdminReport);
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::ChannelParityMismatch));
}

#[test]
fn delivery_profile_missing_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.install_rows[0].delivery_profiles.clear();
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::DeliveryProfileMissing));
}

#[test]
fn freshness_state_missing_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.install_rows[0].freshness_states.clear();
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::FreshnessStateMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.install_rows[0]
        .export_fields
        .retain(|f| *f != M5AdvisoryExportField::FreshnessState);
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_assessment_drift_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.install_rows[0].example_assessments[0]
        .resolved
        .build_identity_repr = "tampered".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::ExampleAssessmentDrift));
}

#[test]
fn example_assessment_missing_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.install_rows[2].example_assessments.clear();
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::ExampleAssessmentMissing));
}

#[test]
fn local_graph_resolution_unproven_fails_when_no_installed_but_affected_example() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    // Rewrite every example so nothing is installed-but-affected.
    for row in &mut packet.install_rows {
        for case in &mut row.example_assessments {
            case.input.install_state = M5AdvisoryInstallState::NotInstalled;
            case.input.mirror_freshness = M5AdvisoryFreshnessState::UpToDate;
            *case = M5AffectedInstallResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::LocalGraphResolutionUnproven));
}

#[test]
fn mirror_freshness_install_mode_unproven_fails_when_no_stale_narrow() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    // Make every example authoritative so the stale-narrow proof disappears.
    for row in &mut packet.install_rows {
        for case in &mut row.example_assessments {
            case.input.mirror_freshness = M5AdvisoryFreshnessState::UpToDate;
            *case = M5AffectedInstallResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::MirrorFreshnessInstallModeUnproven));
}

#[test]
fn attached_actions_unproven_fails_when_no_rollback_action() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    // Strip every rollback/repin primary and help so no rollback action survives.
    for row in &mut packet.install_rows {
        for case in &mut row.example_assessments {
            case.input.primary_action = M5AdvisoryRequiredAction::ReviewNotice;
            case.input.help_action = M5AdvisoryRequiredAction::ReviewNotice;
            *case = M5AffectedInstallResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::AttachedActionsUnproven));
}

#[test]
fn verdict_coverage_unproven_fails_when_examples_drop_a_verdict() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    for row in &mut packet.install_rows {
        for case in &mut row.example_assessments {
            case.input.install_state = M5AdvisoryInstallState::InstalledActive;
            case.input.mirror_freshness = M5AdvisoryFreshnessState::UpToDate;
            *case = M5AffectedInstallResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::VerdictCoverageUnproven));
}

#[test]
fn severity_coverage_unproven_fails_when_examples_drop_a_severity() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    for row in &mut packet.install_rows {
        for case in &mut row.example_assessments {
            case.input.severity = M5AdvisorySeverityClass::High;
            *case = M5AffectedInstallResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::SeverityCoverageUnproven));
}

#[test]
fn install_invariant_violation_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.install_rows[0].stale_mirror_stays_silently_green = true;
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::InstallInvariantViolated));
}

#[test]
fn stable_install_missing_proof_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.install_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::StableInstallMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet
        .governance_review
        .stale_mirror_auto_narrows_clean_verdict = false;
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.consumer_projection.admin_report_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AffectedInstallPanelViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_install_profile() {
    let summary = seeded_m5_affected_install_panel_primitive_packet().render_markdown_summary();
    for profile in M5InstallProfileLane::ALL {
        assert!(
            summary.contains(profile.label()),
            "summary missing install-profile lane {}",
            profile.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_install_profile() {
    let csv = seeded_m5_affected_install_panel_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5InstallProfileLane::ALL.len());
    assert!(lines[0].starts_with("install_profile,qualification,owner,"));
    for profile in M5InstallProfileLane::ALL {
        assert!(
            csv.contains(profile.as_str()),
            "csv missing install-profile lane {}",
            profile.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_affected_install_panel_primitive_export()
        .expect("checked M5 affected-install panel export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_affected_install_panel_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lanes_visible() {
    for packet in [
        seeded_m5_affected_install_panel_primitive_managed_deployed_beta_narrowed(),
        seeded_m5_affected_install_panel_primitive_offline_bundle_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.install_rows.len(), M5InstallProfileLane::ALL.len());
    }

    let managed = seeded_m5_affected_install_panel_primitive_managed_deployed_beta_narrowed();
    let row = managed
        .install_rows
        .iter()
        .find(|r| r.install_profile == M5InstallProfileLane::ManagedDeployed)
        .expect("managed-deployed row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Beta);

    let offline = seeded_m5_affected_install_panel_primitive_offline_bundle_preview_narrowed();
    let row = offline
        .install_rows
        .iter()
        .find(|r| r.install_profile == M5InstallProfileLane::OfflineBundle)
        .expect("offline-bundle row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let managed: M5AffectedInstallPanelPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5-affected-install-panel-primitive/managed_deployed_beta_narrowed.json"
    )))
    .expect("managed-deployed fixture parses");
    assert!(managed.validate().is_empty());
    assert_eq!(
        managed,
        seeded_m5_affected_install_panel_primitive_managed_deployed_beta_narrowed()
    );

    let offline: M5AffectedInstallPanelPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5-affected-install-panel-primitive/offline_bundle_preview_narrowed.json"
    )))
    .expect("offline-bundle fixture parses");
    assert!(offline.validate().is_empty());
    assert_eq!(
        offline,
        seeded_m5_affected_install_panel_primitive_offline_bundle_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_affected_install_panel_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
