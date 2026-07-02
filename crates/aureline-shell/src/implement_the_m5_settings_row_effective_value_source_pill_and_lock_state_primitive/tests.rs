use super::*;

fn contribution(source: M5SettingSourcePill, value: &str) -> M5SettingsSourceContribution {
    M5SettingsSourceContribution::new(source, value)
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_locked_policy_retains_user_value() {
    let input = M5SettingsRowResolutionInput {
        setting_key: "admin.telemetry_sharing".to_owned(),
        contributions: vec![
            contribution(M5SettingSourcePill::DefaultValue, "disabled"),
            contribution(M5SettingSourcePill::UserConfigured, "enabled"),
            M5SettingsSourceContribution::locked(M5SettingSourcePill::PolicyManaged, "disabled"),
        ],
        pending_reload: false,
        invalid_value_held: false,
        held_value_repr: None,
    };
    let resolved = resolve_settings_row(&input).expect("resolves");
    assert_eq!(resolved.row_state, M5SettingsRowState::LockedByPolicy);
    assert_eq!(resolved.effective_value_repr, "disabled");
    assert_eq!(resolved.winning_source, M5SettingSourcePill::PolicyManaged);
    assert_eq!(
        resolved.lock_source,
        Some(M5SettingSourcePill::PolicyManaged)
    );
    assert!(resolved.is_locked);
    // The user-configured value is retained, not hidden, and the diff is exposed.
    assert_eq!(resolved.configured_value_repr.as_deref(), Some("enabled"));
    assert!(resolved.differs_from_configured);
    // Shadow chain is highest precedence first.
    assert_eq!(
        resolved.shadow_chain,
        vec![
            M5SettingSourcePill::PolicyManaged,
            M5SettingSourcePill::UserConfigured,
            M5SettingSourcePill::DefaultValue,
        ]
    );
}

#[test]
fn resolver_user_value_matches_configured() {
    let input = M5SettingsRowResolutionInput {
        setting_key: "workspace.trust_level".to_owned(),
        contributions: vec![
            contribution(M5SettingSourcePill::DefaultValue, "restricted"),
            contribution(M5SettingSourcePill::UserConfigured, "trusted"),
        ],
        pending_reload: false,
        invalid_value_held: false,
        held_value_repr: None,
    };
    let resolved = resolve_settings_row(&input).expect("resolves");
    assert_eq!(
        resolved.row_state,
        M5SettingsRowState::EffectiveMatchesConfigured
    );
    assert_eq!(resolved.winning_source, M5SettingSourcePill::UserConfigured);
    assert!(!resolved.is_locked);
    assert!(!resolved.differs_from_configured);
}

#[test]
fn resolver_higher_source_overrides() {
    let input = M5SettingsRowResolutionInput {
        setting_key: "network.proxy_mode".to_owned(),
        contributions: vec![
            contribution(M5SettingSourcePill::DefaultValue, "auto"),
            contribution(M5SettingSourcePill::UserConfigured, "auto"),
            contribution(M5SettingSourcePill::EnvironmentOverride, "manual"),
        ],
        pending_reload: false,
        invalid_value_held: false,
        held_value_repr: None,
    };
    let resolved = resolve_settings_row(&input).expect("resolves");
    assert_eq!(
        resolved.row_state,
        M5SettingsRowState::OverriddenByHigherSource
    );
    assert_eq!(resolved.effective_value_repr, "manual");
    assert_eq!(
        resolved.winning_source,
        M5SettingSourcePill::EnvironmentOverride
    );
    assert_eq!(resolved.configured_value_repr.as_deref(), Some("auto"));
    assert!(resolved.differs_from_configured);
}

#[test]
fn resolver_default_is_inherited() {
    let input = M5SettingsRowResolutionInput {
        setting_key: "extension.autoupdate".to_owned(),
        contributions: vec![contribution(M5SettingSourcePill::DefaultValue, "enabled")],
        pending_reload: false,
        invalid_value_held: false,
        held_value_repr: None,
    };
    let resolved = resolve_settings_row(&input).expect("resolves");
    assert_eq!(resolved.row_state, M5SettingsRowState::InheritedFromDefault);
    assert_eq!(resolved.winning_source, M5SettingSourcePill::DefaultValue);
    assert!(resolved.configured_value_repr.is_none());
    assert!(!resolved.differs_from_configured);
}

#[test]
fn resolver_pending_reload_and_invalid_and_redacted() {
    let pending = M5SettingsRowResolutionInput {
        setting_key: "execution.max_parallel_jobs".to_owned(),
        contributions: vec![
            contribution(M5SettingSourcePill::DefaultValue, "four"),
            contribution(M5SettingSourcePill::UserConfigured, "eight"),
        ],
        pending_reload: true,
        invalid_value_held: false,
        held_value_repr: None,
    };
    assert_eq!(
        resolve_settings_row(&pending).unwrap().row_state,
        M5SettingsRowState::PendingReloadToApply
    );

    let invalid = M5SettingsRowResolutionInput {
        setting_key: "update.channel".to_owned(),
        contributions: vec![
            contribution(M5SettingSourcePill::DefaultValue, "stable"),
            contribution(M5SettingSourcePill::UserConfigured, "not-a-channel"),
        ],
        pending_reload: false,
        invalid_value_held: true,
        held_value_repr: Some("stable".to_owned()),
    };
    let resolved = resolve_settings_row(&invalid).unwrap();
    assert_eq!(resolved.row_state, M5SettingsRowState::InvalidValueHeld);
    assert_eq!(resolved.effective_value_repr, "stable");

    let redacted = M5SettingsRowResolutionInput {
        setting_key: "ai.provider_credential_ref".to_owned(),
        contributions: vec![
            contribution(
                M5SettingSourcePill::DefaultValue,
                M5_SETTINGS_REDACTED_VALUE_REPR,
            ),
            M5SettingsSourceContribution::locked(
                M5SettingSourcePill::PolicyManaged,
                M5_SETTINGS_REDACTED_VALUE_REPR,
            ),
        ],
        pending_reload: false,
        invalid_value_held: false,
        held_value_repr: None,
    };
    let resolved = resolve_settings_row(&redacted).unwrap();
    assert_eq!(resolved.row_state, M5SettingsRowState::RedactedManagedValue);
    assert!(resolved.is_locked);
}

#[test]
fn resolver_rejects_malformed_input() {
    let no_default = M5SettingsRowResolutionInput {
        setting_key: "x.y".to_owned(),
        contributions: vec![contribution(M5SettingSourcePill::UserConfigured, "v")],
        pending_reload: false,
        invalid_value_held: false,
        held_value_repr: None,
    };
    assert_eq!(
        resolve_settings_row(&no_default),
        Err(M5SettingsResolutionError::MissingDefaultContribution)
    );

    let dup = M5SettingsRowResolutionInput {
        setting_key: "x.y".to_owned(),
        contributions: vec![
            contribution(M5SettingSourcePill::DefaultValue, "a"),
            contribution(M5SettingSourcePill::DefaultValue, "b"),
        ],
        pending_reload: false,
        invalid_value_held: false,
        held_value_repr: None,
    };
    assert_eq!(
        resolve_settings_row(&dup),
        Err(M5SettingsResolutionError::DuplicateSource(
            M5SettingSourcePill::DefaultValue
        ))
    );

    let bad_lock = M5SettingsRowResolutionInput {
        setting_key: "x.y".to_owned(),
        contributions: vec![
            contribution(M5SettingSourcePill::DefaultValue, "a"),
            M5SettingsSourceContribution::locked(M5SettingSourcePill::UserConfigured, "b"),
        ],
        pending_reload: false,
        invalid_value_held: false,
        held_value_repr: None,
    };
    assert_eq!(
        resolve_settings_row(&bad_lock),
        Err(M5SettingsResolutionError::LockOnUnprivilegedSource(
            M5SettingSourcePill::UserConfigured
        ))
    );

    let forbidden = M5SettingsRowResolutionInput {
        setting_key: "x.y".to_owned(),
        contributions: vec![contribution(
            M5SettingSourcePill::DefaultValue,
            "https://example.test",
        )],
        pending_reload: false,
        invalid_value_held: false,
        held_value_repr: None,
    };
    assert_eq!(
        resolve_settings_row(&forbidden),
        Err(M5SettingsResolutionError::ForbiddenValueMaterial)
    );

    let missing_held = M5SettingsRowResolutionInput {
        setting_key: "x.y".to_owned(),
        contributions: vec![contribution(M5SettingSourcePill::DefaultValue, "a")],
        pending_reload: false,
        invalid_value_held: true,
        held_value_repr: None,
    };
    assert_eq!(
        resolve_settings_row(&missing_held),
        Err(M5SettingsResolutionError::MissingHeldValue)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_settings_row_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SETTINGS_ROW_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_surface_family() {
    let packet = seeded_m5_settings_row_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for family in M5SettingsSurfaceFamily::ALL {
        assert!(
            present.contains(&family),
            "missing surface family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.surface_rows.len(),
        M5SettingsSurfaceFamily::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_lock_and_export() {
    let packet = seeded_m5_settings_row_primitive_packet();
    for row in &packet.surface_rows {
        for part in M5SettingsRowAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for disclosure in M5SettingsLockDisclosure::MANDATORY {
            assert!(row.lock_disclosures.contains(&disclosure));
        }
        for field in M5SettingsRowExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TrustAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_row_state_and_source_pill_is_exercised_by_some_example() {
    let packet = seeded_m5_settings_row_primitive_packet();
    let cases: Vec<&M5SettingsRowResolutionCase> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for state in M5SettingsRowState::ALL {
        assert!(
            cases.iter().any(|case| case.resolved.row_state == state),
            "no worked resolution exercises row state {}",
            state.as_str()
        );
    }
    for pill in M5SettingSourcePill::ALL {
        assert!(
            cases
                .iter()
                .any(|case| case.input.contributions.iter().any(|c| c.source == pill)),
            "no worked resolution exercises source pill {}",
            pill.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_settings_row_primitive_packet();
    for row in &packet.surface_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.surface_family.as_str()
            );
        }
    }
}

#[test]
fn missing_surface_family_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet
        .surface_rows
        .retain(|row| row.surface_family != M5SettingsSurfaceFamily::NetworkProxy);
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.vocabulary_set.anatomy_parts.pop();
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.surface_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5SettingsRowAnatomyPart::SourcePill);
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_lock_disclosure_missing_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.surface_rows[0]
        .lock_disclosures
        .retain(|d| *d != M5SettingsLockDisclosure::UserConfiguredValueRetained);
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::MandatoryLockDisclosureMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5SettingsRowExportField::EffectiveValueRepr);
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    // Corrupt a stored resolution so it no longer matches a fresh resolve.
    packet.surface_rows[1].example_resolutions[0]
        .resolved
        .effective_value_repr = "tampered".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.surface_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn locked_retention_unproven_fails_when_no_locked_example_retains_user_value() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    // Remove the admin locked example (the one proving retention) and confirm the
    // packet-level lint fires.
    packet.surface_rows[0].example_resolutions.clear();
    // The admin row would now also trip ExampleResolutionMissing; restore a
    // non-locked example so we isolate the retention lint.
    packet.surface_rows[0]
        .example_resolutions
        .push(M5SettingsRowResolutionCase::resolved(
            M5SettingsRowResolutionInput {
                setting_key: "admin.some_flag".to_owned(),
                contributions: vec![M5SettingsSourceContribution::new(
                    M5SettingSourcePill::DefaultValue,
                    "off",
                )],
                pending_reload: false,
                invalid_value_held: false,
                held_value_repr: None,
            },
        ));
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::LockedRetentionUnproven));
}

#[test]
fn surface_invariant_violation_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.surface_rows[0].hides_user_configured_when_locked = true;
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::SurfaceInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.surface_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet
        .governance_review
        .locked_value_never_hides_user_configured = false;
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet
        .consumer_projection
        .resolver_reads_single_precedence_ladder = false;
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_settings_row_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SettingsRowPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface_family() {
    let summary = seeded_m5_settings_row_primitive_packet().render_markdown_summary();
    for family in M5SettingsSurfaceFamily::ALL {
        assert!(
            summary.contains(family.label()),
            "summary missing surface {}",
            family.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_surface() {
    let csv = seeded_m5_settings_row_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5SettingsSurfaceFamily::ALL.len());
    assert!(lines[0].starts_with("surface_family,qualification,owner,"));
    for family in M5SettingsSurfaceFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing surface {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_settings_row_primitive_export()
        .expect("checked M5 settings-row primitive export validates");
    assert_eq!(from_disk.packet_id, M5_SETTINGS_ROW_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_settings_row_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_settings_row_primitive_admin_enterprise_beta_narrowed(),
        seeded_m5_settings_row_primitive_update_channel_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.surface_rows.len(),
            M5SettingsSurfaceFamily::ALL.len()
        );
    }

    let admin = seeded_m5_settings_row_primitive_admin_enterprise_beta_narrowed();
    let row = admin
        .surface_rows
        .iter()
        .find(|r| r.surface_family == M5SettingsSurfaceFamily::AdminEnterprise)
        .expect("admin row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Beta);

    let update = seeded_m5_settings_row_primitive_update_channel_preview_narrowed();
    let row = update
        .surface_rows
        .iter()
        .find(|r| r.surface_family == M5SettingsSurfaceFamily::UpdateChannel)
        .expect("update row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let admin: M5SettingsRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-settings-row-primitive/admin_enterprise_beta_narrowed.json"
    )))
    .expect("admin fixture parses");
    assert!(admin.validate().is_empty());
    assert_eq!(
        admin,
        seeded_m5_settings_row_primitive_admin_enterprise_beta_narrowed()
    );

    let update: M5SettingsRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-settings-row-primitive/update_channel_preview_narrowed.json"
    )))
    .expect("update fixture parses");
    assert!(update.validate().is_empty());
    assert_eq!(
        update,
        seeded_m5_settings_row_primitive_update_channel_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_settings_row_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
