use super::*;

fn adv(install_state: M5AdvisoryInstallState) -> M5AdvisoryRowResolutionInput {
    M5AdvisoryRowResolutionInput {
        affected_surface: M5AffectedSurfaceLane::DesktopApp,
        advisory_id: "AURELINE-ADV-2026-0001".to_owned(),
        severity: M5AdvisorySeverityClass::High,
        affected_object_repr: "desktop-app:core-runtime".to_owned(),
        install_state,
        fixed_version_or_mitigation_repr: "fixed-in-2.4.1".to_owned(),
        signer_source_state_repr: "signer_source_state:signed_current".to_owned(),
        action_state: M5AdvisoryActionState::ActionRequired,
        primary_action: M5AdvisoryRequiredAction::UpdateToFixedVersion,
        continuity_claim: M5AdvisoryContinuityClaim::DegradedLocalMode,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_derives_exposure_and_keeps_row_visible() {
    let resolved =
        resolve_advisory_row(&adv(M5AdvisoryInstallState::InstalledBlocked)).expect("resolves");

    assert_eq!(
        resolved.exposure_state,
        M5AdvisoryExposureState::ContainedByBlock
    );
    assert!(resolved.installed_but_affected);
    assert!(resolved.remains_visible);
    assert!(!resolved.degrades_to_generic_prompt);
    // Every channel is projected with identical core truth.
    assert_eq!(
        resolved.channel_projections.len(),
        M5AdvisoryRowChannel::ALL.len()
    );
    for projection in &resolved.channel_projections {
        assert_eq!(projection.severity, resolved.severity);
        assert_eq!(projection.exposure_state, resolved.exposure_state);
        assert_eq!(projection.primary_action, resolved.primary_action);
    }
    // The export summary carries every mandatory column with a populated value.
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
    for (install, exposure) in expected {
        let resolved = resolve_advisory_row(&adv(install)).expect("resolves");
        assert_eq!(
            resolved.exposure_state,
            exposure,
            "install {}",
            install.as_str()
        );
    }
}

#[test]
fn resolver_marks_only_installed_but_affected_states() {
    for install in M5AdvisoryInstallState::ALL {
        let resolved = resolve_advisory_row(&adv(install)).expect("resolves");
        assert_eq!(
            resolved.installed_but_affected,
            install.is_installed_but_affected(),
            "install {}",
            install.as_str()
        );
        // No install state ever degrades to a generic prompt or hides the row.
        assert!(resolved.remains_visible);
        assert!(!resolved.degrades_to_generic_prompt);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let mut e = adv(M5AdvisoryInstallState::InstalledActive);
    e.advisory_id = "  ".to_owned();
    assert_eq!(
        resolve_advisory_row(&e),
        Err(M5AdvisoryRowResolutionError::EmptyAdvisoryId)
    );

    let mut e = adv(M5AdvisoryInstallState::InstalledActive);
    e.affected_object_repr = "".to_owned();
    assert_eq!(
        resolve_advisory_row(&e),
        Err(M5AdvisoryRowResolutionError::EmptyAffectedObject)
    );

    let mut e = adv(M5AdvisoryInstallState::InstalledActive);
    e.fixed_version_or_mitigation_repr = "".to_owned();
    assert_eq!(
        resolve_advisory_row(&e),
        Err(M5AdvisoryRowResolutionError::EmptyFixedVersionOrMitigation)
    );

    let mut e = adv(M5AdvisoryInstallState::InstalledActive);
    e.signer_source_state_repr = "  ".to_owned();
    assert_eq!(
        resolve_advisory_row(&e),
        Err(M5AdvisoryRowResolutionError::EmptySignerSourceState)
    );

    let mut e = adv(M5AdvisoryInstallState::InstalledActive);
    e.affected_object_repr = "reach https://example.test".to_owned();
    assert_eq!(
        resolve_advisory_row(&e),
        Err(M5AdvisoryRowResolutionError::ForbiddenMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_advisory_card_row_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ADVISORY_ROW_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_affected_surface() {
    let packet = seeded_m5_advisory_card_row_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .surface_rows
        .iter()
        .map(|r| r.affected_surface)
        .collect();
    for lane in M5AffectedSurfaceLane::ALL {
        assert!(
            present.contains(&lane),
            "missing surface lane {}",
            lane.as_str()
        );
    }
    assert_eq!(packet.surface_rows.len(), M5AffectedSurfaceLane::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_channels_and_export() {
    let packet = seeded_m5_advisory_card_row_primitive_packet();
    for row in &packet.surface_rows {
        for part in M5AdvisoryRowAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for channel in M5AdvisoryRowChannel::ALL {
            assert!(row.channels.contains(&channel));
        }
        for field in MANDATORY_EXPORT_FIELDS {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5AdvisoryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_advisories.is_empty());
    }
}

#[test]
fn every_severity_and_exposure_is_exercised_by_some_example() {
    let packet = seeded_m5_advisory_card_row_primitive_packet();
    let rows: Vec<&M5ResolvedAdvisoryRow> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_advisories.iter())
        .map(|case| &case.resolved)
        .collect();

    for severity in M5AdvisorySeverityClass::ALL {
        assert!(
            rows.iter().any(|r| r.severity == severity),
            "no worked resolution exercises severity {}",
            severity.as_str()
        );
    }
    for exposure in M5AdvisoryExposureState::ALL {
        assert!(
            rows.iter().any(|r| r.exposure_state == exposure),
            "no worked resolution exercises exposure {}",
            exposure.as_str()
        );
    }
}

#[test]
fn some_example_keeps_a_contained_item_visible() {
    let packet = seeded_m5_advisory_card_row_primitive_packet();
    let proven = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_advisories.iter())
        .any(|case| {
            case.input.install_state.is_contained()
                && case.resolved.installed_but_affected
                && case.resolved.remains_visible
                && !case.resolved.degrades_to_generic_prompt
        });
    assert!(
        proven,
        "no worked resolution keeps a contained item visible"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_advisory_card_row_primitive_packet();
    for row in &packet.surface_rows {
        for case in &row.example_advisories {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.affected_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_affected_surface_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet
        .surface_rows
        .retain(|row| row.affected_surface != M5AffectedSurfaceLane::RemoteHelper);
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.vocabulary_set.channels.pop();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.surface_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5AdvisoryRowAnatomyPart::CurrentExposure);
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn channel_parity_mismatch_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.surface_rows[0]
        .channels
        .retain(|c| *c != M5AdvisoryRowChannel::SupportBundle);
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::ChannelParityMismatch));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5AdvisoryExportField::ContinuityNote);
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_advisory_drift_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.surface_rows[0].example_advisories[0]
        .resolved
        .affected_object_repr = "tampered".to_owned();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::ExampleAdvisoryDrift));
}

#[test]
fn example_advisory_missing_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.surface_rows[2].example_advisories.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::ExampleAdvisoryMissing));
}

#[test]
fn channel_parity_unproven_fails_when_examples_drop_a_channel() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    for row in &mut packet.surface_rows {
        for case in &mut row.example_advisories {
            case.resolved
                .channel_projections
                .retain(|p| p.channel == M5AdvisoryRowChannel::UpdateCenter);
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::ChannelParityUnproven));
}

#[test]
fn installed_but_affected_unproven_fails_when_no_example_is_contained() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    // Rewrite every example so it is not installed → no contained state survives.
    for row in &mut packet.surface_rows {
        for case in &mut row.example_advisories {
            case.input.install_state = M5AdvisoryInstallState::NotInstalled;
            *case = M5AdvisoryRowResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::InstalledButAffectedUnproven));
}

#[test]
fn severity_coverage_unproven_fails_when_examples_drop_a_severity() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    for row in &mut packet.surface_rows {
        for case in &mut row.example_advisories {
            case.input.severity = M5AdvisorySeverityClass::High;
            *case = M5AdvisoryRowResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::SeverityCoverageUnproven));
}

#[test]
fn exposure_coverage_unproven_fails_when_examples_drop_an_exposure() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    for row in &mut packet.surface_rows {
        for case in &mut row.example_advisories {
            case.input.install_state = M5AdvisoryInstallState::InstalledActive;
            *case = M5AdvisoryRowResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::ExposureCoverageUnproven));
}

#[test]
fn surface_invariant_violation_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.surface_rows[0].disappears_when_installed_but_affected = true;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::SurfaceInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.surface_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet
        .governance_review
        .installed_but_affected_stays_visible = false;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.consumer_projection.marketplace_renders_shared_row = false;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5AdvisoryRowPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_affected_surface() {
    let summary = seeded_m5_advisory_card_row_primitive_packet().render_markdown_summary();
    for lane in M5AffectedSurfaceLane::ALL {
        assert!(
            summary.contains(lane.label()),
            "summary missing surface lane {}",
            lane.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_affected_surface() {
    let csv = seeded_m5_advisory_card_row_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5AffectedSurfaceLane::ALL.len());
    assert!(lines[0].starts_with("affected_surface,qualification,owner,"));
    for lane in M5AffectedSurfaceLane::ALL {
        assert!(
            csv.contains(lane.as_str()),
            "csv missing surface lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_advisory_card_row_primitive_export()
        .expect("checked M5 advisory-row primitive export validates");
    assert_eq!(from_disk.packet_id, M5_ADVISORY_ROW_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_advisory_card_row_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lanes_visible() {
    for packet in [
        seeded_m5_advisory_card_row_primitive_extension_beta_narrowed(),
        seeded_m5_advisory_card_row_primitive_signing_update_path_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.surface_rows.len(), M5AffectedSurfaceLane::ALL.len());
    }

    let extension = seeded_m5_advisory_card_row_primitive_extension_beta_narrowed();
    let row = extension
        .surface_rows
        .iter()
        .find(|r| r.affected_surface == M5AffectedSurfaceLane::Extension)
        .expect("extension row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Beta);

    let signing = seeded_m5_advisory_card_row_primitive_signing_update_path_preview_narrowed();
    let row = signing
        .surface_rows
        .iter()
        .find(|r| r.affected_surface == M5AffectedSurfaceLane::SigningUpdatePath)
        .expect("signing-update-path row present");
    assert_eq!(row.qualification, M5AdvisoryQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let extension: M5AdvisoryRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5-advisory-card-row-primitive/extension_beta_narrowed.json"
    )))
    .expect("extension fixture parses");
    assert!(extension.validate().is_empty());
    assert_eq!(
        extension,
        seeded_m5_advisory_card_row_primitive_extension_beta_narrowed()
    );

    let signing: M5AdvisoryRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/security/m5-advisory-card-row-primitive/signing_update_path_preview_narrowed.json"
    )))
    .expect("signing fixture parses");
    assert!(signing.validate().is_empty());
    assert_eq!(
        signing,
        seeded_m5_advisory_card_row_primitive_signing_update_path_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_advisory_card_row_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
