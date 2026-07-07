use super::*;

fn signed_in_full_write() -> M5ProviderAccountRowResolutionInput {
    M5ProviderAccountRowResolutionInput {
        identity_class: M5ProviderIdentityClass::OrganizationMember,
        connection_state: M5AccountConnectionState::SignedIn,
        tenant_scope: M5TenantScopeClass::OrgScoped,
        write_scope: M5ProviderWriteScope::FullWrite,
        session_freshness: M5ProviderAccountSessionFreshness::FreshSession,
        has_local_drafts: false,
        account_label: "acme-eng org account".to_owned(),
        account_identity_ref: "account:acme-eng:org-member".to_owned(),
    }
}

// ---- provider-account-row resolver --------------------------------------

#[test]
fn signed_in_full_write_reads_and_writes_with_full_connection() {
    let resolved = resolve_provider_account_row(&signed_in_full_write()).expect("resolves");
    assert_eq!(
        resolved.row_posture,
        M5ProviderAccountRowPosture::SignedInRow
    );
    assert_eq!(
        resolved.access_capability,
        M5ProviderAccountAccessCapability::CanReadAndWrite
    );
    assert!(resolved.can_read_live);
    assert!(resolved.can_write);
    assert!(!resolved.only_inspect_cached);
    assert!(!resolved.needs_attention);
    assert!(!resolved.needs_reauth);
    assert!(resolved.preserves_local_drafts);
    assert!(resolved.preserves_support_export_continuity);
    assert!(!resolved.requires_blind_credential_reentry);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5ProviderAccountRowAction::RevealScope,
            M5ProviderAccountRowAction::RemoveAccount,
            M5ProviderAccountRowAction::ExportRow,
        ]
    );
    assert_eq!(resolved.account_identity_ref, "account:acme-eng:org-member");
}

#[test]
fn posture_maps_one_to_one_from_connection_state() {
    let cases = [
        (
            M5AccountConnectionState::NotConfigured,
            M5ProviderAccountRowPosture::NotConfiguredRow,
        ),
        (
            M5AccountConnectionState::SignedIn,
            M5ProviderAccountRowPosture::SignedInRow,
        ),
        (
            M5AccountConnectionState::LimitedScope,
            M5ProviderAccountRowPosture::LimitedScopeRow,
        ),
        (
            M5AccountConnectionState::StaleSession,
            M5ProviderAccountRowPosture::StaleSessionRow,
        ),
        (
            M5AccountConnectionState::OfflineCachedRead,
            M5ProviderAccountRowPosture::OfflineCachedReadRow,
        ),
        (
            M5AccountConnectionState::PolicyBlocked,
            M5ProviderAccountRowPosture::PolicyBlockedRow,
        ),
    ];
    for (state, expected) in cases {
        let resolved = resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
            connection_state: state,
            ..signed_in_full_write()
        })
        .expect("resolves");
        assert_eq!(
            resolved.row_posture,
            expected,
            "connection state {} collapsed its posture",
            state.as_str()
        );
    }
}

#[test]
fn cached_and_no_access_states_never_read_as_live() {
    // Stale session and offline cached read can only inspect a cached read.
    for state in [
        M5AccountConnectionState::StaleSession,
        M5AccountConnectionState::OfflineCachedRead,
    ] {
        let resolved = resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
            connection_state: state,
            ..signed_in_full_write()
        })
        .expect("resolves");
        assert_eq!(
            resolved.access_capability,
            M5ProviderAccountAccessCapability::CachedInspectOnly,
            "state {} did not read as cached-inspect-only",
            state.as_str()
        );
        assert!(!resolved.can_read_live, "{} read live", state.as_str());
        assert!(!resolved.can_write, "{} claimed write", state.as_str());
        assert!(resolved.only_inspect_cached);
    }

    // Not-configured and policy-blocked have no access at all.
    for state in [
        M5AccountConnectionState::NotConfigured,
        M5AccountConnectionState::PolicyBlocked,
    ] {
        let resolved = resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
            connection_state: state,
            ..signed_in_full_write()
        })
        .expect("resolves");
        assert_eq!(
            resolved.access_capability,
            M5ProviderAccountAccessCapability::NoAccess,
            "state {} was not no-access",
            state.as_str()
        );
        assert!(!resolved.can_read_live);
        assert!(!resolved.can_write);
        assert!(!resolved.only_inspect_cached);
    }
}

#[test]
fn write_scope_caps_the_access_capability() {
    let cases = [
        (
            M5ProviderWriteScope::FullWrite,
            M5ProviderAccountAccessCapability::CanReadAndWrite,
        ),
        (
            M5ProviderWriteScope::CommentOnly,
            M5ProviderAccountAccessCapability::CanReadWriteLimited,
        ),
        (
            M5ProviderWriteScope::StatusOnly,
            M5ProviderAccountAccessCapability::CanReadWriteLimited,
        ),
        (
            M5ProviderWriteScope::ReadOnly,
            M5ProviderAccountAccessCapability::CanReadOnlyLive,
        ),
        (
            M5ProviderWriteScope::NoWrite,
            M5ProviderAccountAccessCapability::CanReadOnlyLive,
        ),
        (
            M5ProviderWriteScope::ScopeUnknown,
            M5ProviderAccountAccessCapability::CanReadOnlyLive,
        ),
    ];
    for (scope, expected) in cases {
        let resolved = resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
            write_scope: scope,
            ..signed_in_full_write()
        })
        .expect("resolves");
        assert_eq!(
            resolved.access_capability,
            expected,
            "signed-in write scope {} produced the wrong access capability",
            scope.as_str()
        );
    }

    // A limited-scope account with a full write scope is capped to limited.
    let limited = resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
        connection_state: M5AccountConnectionState::LimitedScope,
        write_scope: M5ProviderWriteScope::FullWrite,
        ..signed_in_full_write()
    })
    .expect("resolves");
    assert_eq!(
        limited.access_capability,
        M5ProviderAccountAccessCapability::CanReadWriteLimited
    );
}

#[test]
fn actions_offer_sign_in_retry_and_remove_by_state() {
    // Not configured → sign-in only, no remove.
    let not_configured = resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
        connection_state: M5AccountConnectionState::NotConfigured,
        session_freshness: M5ProviderAccountSessionFreshness::NeverAuthenticated,
        ..signed_in_full_write()
    })
    .expect("resolves");
    assert_eq!(
        not_configured.available_actions,
        vec![
            M5ProviderAccountRowAction::RevealScope,
            M5ProviderAccountRowAction::SignInAccount,
            M5ProviderAccountRowAction::ExportRow,
        ]
    );

    // Stale session → retry + remove, keeps local drafts.
    let stale = resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
        connection_state: M5AccountConnectionState::StaleSession,
        session_freshness: M5ProviderAccountSessionFreshness::ExpiredSession,
        has_local_drafts: true,
        ..signed_in_full_write()
    })
    .expect("resolves");
    assert!(stale
        .available_actions
        .contains(&M5ProviderAccountRowAction::RetryAuth));
    assert!(stale
        .available_actions
        .contains(&M5ProviderAccountRowAction::RemoveAccount));
    assert!(!stale
        .available_actions
        .contains(&M5ProviderAccountRowAction::SignInAccount));
    assert!(stale.needs_reauth);
    assert!(stale.preserves_local_drafts);
    assert!(!stale.requires_blind_credential_reentry);

    // Signed-in but near expiry → retry offered ahead of expiry.
    let near_expiry = resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
        session_freshness: M5ProviderAccountSessionFreshness::NearExpiry,
        ..signed_in_full_write()
    })
    .expect("resolves");
    assert!(near_expiry
        .available_actions
        .contains(&M5ProviderAccountRowAction::RetryAuth));

    // Signed-in and fresh → no retry needed.
    let fresh = resolve_provider_account_row(&signed_in_full_write()).expect("resolves");
    assert!(!fresh
        .available_actions
        .contains(&M5ProviderAccountRowAction::RetryAuth));
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
            account_label: " ".to_owned(),
            ..signed_in_full_write()
        }),
        Err(M5ProviderAccountRowResolutionError::EmptyAccountLabel)
    );
    assert_eq!(
        resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
            account_identity_ref: "".to_owned(),
            ..signed_in_full_write()
        }),
        Err(M5ProviderAccountRowResolutionError::EmptyAccountIdentity)
    );
    assert_eq!(
        resolve_provider_account_row(&M5ProviderAccountRowResolutionInput {
            account_identity_ref: "account:https://provider.example/tok".to_owned(),
            ..signed_in_full_write()
        }),
        Err(M5ProviderAccountRowResolutionError::ForbiddenAccountMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_provider_account_row_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_PROVIDER_ACCOUNT_ROW_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_provider_account_row_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5ProviderAccountConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5ProviderAccountConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_provider_account_row_packet();
    for row in &packet.rows {
        for part in M5ProviderAccountRowAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5ProviderAccountRowExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ProviderAccessibilityRoute::KeyboardFocusable));
        assert!(!row.account_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_provider_account_row_packet();
    let cases: Vec<&M5ProviderAccountRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.account_examples.iter())
        .collect();

    for posture in M5ProviderAccountRowPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.row_posture == posture),
            "no example exercises posture {}",
            posture.as_str()
        );
    }
    for capability in M5ProviderAccountAccessCapability::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.access_capability == capability),
            "no example exercises access capability {}",
            capability.as_str()
        );
    }
    for action in M5ProviderAccountRowAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises action {}",
            action.as_str()
        );
    }
    for state in M5AccountConnectionState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.connection_state == state),
            "no example exercises connection state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity_and_drafts() {
    let packet = seeded_m5_provider_account_row_packet();
    for row in &packet.rows {
        for case in &row.account_examples {
            assert!(
                case.is_self_consistent(),
                "account case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "account case for {} lost identity",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_draft_continuity(),
                "account case for {} lost draft continuity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5ProviderAccountConsumerSurface::ConnectionPicker);
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.vocabulary_set.row_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ProviderAccountRowAnatomyPart::WriteScopeCue);
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5ProviderAccountRowExportField::AccessCapability);
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.rows[0].account_examples[0].resolved.can_write = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::ExampleResolutionDrift));
}

#[test]
fn account_example_missing_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.rows[1].account_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::AccountExampleMissing));
}

#[test]
fn connection_state_coverage_unproven_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    // Replace every example with a signed-in one so most connection states go uncovered.
    for row in &mut packet.rows {
        row.account_examples = vec![M5ProviderAccountRowResolutionCase::resolved(
            signed_in_full_write(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::ConnectionStateCoverageUnproven));
}

#[test]
fn access_capability_coverage_unproven_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    // Replace every example with a read-and-write one so the cached-inspect half fires.
    for row in &mut packet.rows {
        row.account_examples = vec![M5ProviderAccountRowResolutionCase::resolved(
            signed_in_full_write(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::AccessCapabilityCoverageUnproven));
}

#[test]
fn action_coverage_unproven_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    // Every example signed-in and fresh → no sign-in and no retry offered anywhere.
    for row in &mut packet.rows {
        row.account_examples = vec![M5ProviderAccountRowResolutionCase::resolved(
            signed_in_full_write(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::ActionCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.rows[0].collapses_states_into_generic_connected = true;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.governance_review.cached_inspect_never_reads_as_live = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet
        .consumer_projection
        .access_capability_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_provider_account_row_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderAccountRowViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_provider_account_row_packet().render_markdown_summary();
    for surface in M5ProviderAccountConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_provider_account_row_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ProviderAccountConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5ProviderAccountConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_provider_account_row_export()
        .expect("checked M5 account row primitive export validates");
    assert_eq!(from_disk.packet_id, M5_PROVIDER_ACCOUNT_ROW_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_provider_account_row_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_provider_account_row_connection_picker_preview_narrowed(),
        seeded_m5_provider_account_row_headless_cli_accounts_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5ProviderAccountConsumerSurface::ALL.len()
        );
    }

    let connection_picker = seeded_m5_provider_account_row_connection_picker_preview_narrowed();
    let row = connection_picker
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5ProviderAccountConsumerSurface::ConnectionPicker)
        .expect("connection-picker row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Preview);

    let headless = seeded_m5_provider_account_row_headless_cli_accounts_beta_narrowed();
    let row = headless
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5ProviderAccountConsumerSurface::HeadlessCliAccounts)
        .expect("headless-cli-accounts row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let connection_picker: M5ProviderAccountRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-provider-account-row-primitive/connection_picker_preview_narrowed.json"
    )))
    .expect("connection-picker fixture parses");
    assert!(connection_picker.validate().is_empty());
    assert_eq!(
        connection_picker,
        seeded_m5_provider_account_row_connection_picker_preview_narrowed()
    );

    let headless: M5ProviderAccountRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-provider-account-row-primitive/headless_cli_accounts_beta_narrowed.json"
    )))
    .expect("headless-cli fixture parses");
    assert!(headless.validate().is_empty());
    assert_eq!(
        headless,
        seeded_m5_provider_account_row_headless_cli_accounts_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_provider_account_row_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
