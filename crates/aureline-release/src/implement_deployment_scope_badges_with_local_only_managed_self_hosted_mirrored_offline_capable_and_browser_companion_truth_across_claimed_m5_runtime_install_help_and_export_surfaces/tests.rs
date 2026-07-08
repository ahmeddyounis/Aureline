use super::*;

fn input(scope: M5DeploymentScopeBadgeValue) -> M5DeploymentScopeBadgeInput {
    M5DeploymentScopeBadgeInput {
        subject_label: "aureline capability: sample".to_owned(),
        scope,
        residual_dependency_repr: None,
        last_evaluated_repr: "2026-07-01T00:00:00Z".to_owned(),
    }
}

fn sovereign_input(
    scope: M5DeploymentScopeBadgeValue,
    residual: &str,
) -> M5DeploymentScopeBadgeInput {
    M5DeploymentScopeBadgeInput {
        residual_dependency_repr: Some(residual.to_owned()),
        ..input(scope)
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_managed_is_provider_governed_with_no_note() {
    let resolved = resolve_deployment_scope_badge(&input(M5DeploymentScopeBadgeValue::Managed))
        .expect("resolves");
    assert_eq!(
        resolved.sovereignty_posture,
        M5DeploymentSovereigntyPosture::ProviderGoverned
    );
    assert!(resolved.is_provider_governed);
    assert!(!resolved.is_locally_sovereign);
    assert!(!resolved.is_offline_or_mirror);
    assert!(!resolved.is_browser_companion);
    assert!(resolved.residual_dependency_note.is_none());
    assert_eq!(resolved.scope, M5DeploymentScopeBadgeValue::Managed);
}

#[test]
fn resolver_sovereignty_claims_disclose_residual_dependency_and_preserve_scope() {
    for (scope, posture, class, action, continuity, offline, browser) in [
        (
            M5DeploymentScopeBadgeValue::LocalOnly,
            M5DeploymentSovereigntyPosture::LocallySovereign,
            M5ResidualDependencyClass::SigningAndUpdateChannel,
            M5DeploymentScopeNextAction::ReviewResidualDependency,
            M5LocalSafeContinuity::ContinuesFullyLocal,
            false,
            false,
        ),
        (
            M5DeploymentScopeBadgeValue::SelfHosted,
            M5DeploymentSovereigntyPosture::OperatorGoverned,
            M5ResidualDependencyClass::OperatorInfrastructure,
            M5DeploymentScopeNextAction::ReviewResidualDependency,
            M5LocalSafeContinuity::ContinuesFullyLocal,
            false,
            false,
        ),
        (
            M5DeploymentScopeBadgeValue::Mirrored,
            M5DeploymentSovereigntyPosture::MirrorSynced,
            M5ResidualDependencyClass::UpstreamMirrorSync,
            M5DeploymentScopeNextAction::ConfirmOfflineReadinessWindow,
            M5LocalSafeContinuity::ContinuesWithLastMirroredState,
            true,
            false,
        ),
        (
            M5DeploymentScopeBadgeValue::OfflineCapable,
            M5DeploymentSovereigntyPosture::OfflineResilient,
            M5ResidualDependencyClass::CachedCapabilityWindow,
            M5DeploymentScopeNextAction::ConfirmOfflineReadinessWindow,
            M5LocalSafeContinuity::ContinuesWithCachedWindow,
            true,
            false,
        ),
        (
            M5DeploymentScopeBadgeValue::BrowserCompanion,
            M5DeploymentSovereigntyPosture::HostDelegated,
            M5ResidualDependencyClass::HostBrowserRuntime,
            M5DeploymentScopeNextAction::ConfirmHostCompanionScope,
            M5LocalSafeContinuity::ContinuesWithinHostSession,
            false,
            true,
        ),
    ] {
        let resolved =
            resolve_deployment_scope_badge(&sovereign_input(scope, "residual:token/example"))
                .expect("resolves");
        assert_eq!(resolved.sovereignty_posture, posture, "{}", scope.as_str());
        assert!(resolved.is_locally_sovereign, "{}", scope.as_str());
        assert!(!resolved.is_provider_governed);
        assert_eq!(resolved.is_offline_or_mirror, offline, "{}", scope.as_str());
        assert_eq!(resolved.is_browser_companion, browser, "{}", scope.as_str());
        let note = resolved
            .residual_dependency_note
            .expect("residual dependency note present");
        assert_eq!(note.residual_dependency_class, class);
        assert_eq!(note.next_action, action);
        assert_eq!(note.local_safe_continuity, continuity);
        // Implementation requirement: the badge names what it still depends on.
        assert_eq!(note.residual_dependency, "residual:token/example");
        // The underlying scope context is preserved, not dropped.
        assert_eq!(note.preserved_scope, scope);
        assert!(!note.headline.trim().is_empty());
        assert!(note.headline.contains(scope.as_str()));
    }
}

#[test]
fn resolver_rejects_sovereignty_claim_without_residual_dependency() {
    // Implementation requirement: a local/offline/companion badge must never overstate
    // sovereignty — the resolver refuses to build one without a residual dependency.
    let missing = input(M5DeploymentScopeBadgeValue::BrowserCompanion);
    assert_eq!(
        resolve_deployment_scope_badge(&missing),
        Err(M5DeploymentScopeBadgeError::MissingResidualDependencyDisclosure)
    );

    let blank = sovereign_input(M5DeploymentScopeBadgeValue::OfflineCapable, "   ");
    assert_eq!(
        resolve_deployment_scope_badge(&blank),
        Err(M5DeploymentScopeBadgeError::MissingResidualDependencyDisclosure)
    );
}

#[test]
fn resolver_posture_is_derived_from_scope_alone() {
    // Each scope maps to exactly one posture, and the posture never depends on anything
    // outside the scope axis.
    for scope in M5DeploymentScopeBadgeValue::ALL {
        let residual = if scope == M5DeploymentScopeBadgeValue::Managed {
            None
        } else {
            Some("residual:token/example")
        };
        let resolved = resolve_deployment_scope_badge(&M5DeploymentScopeBadgeInput {
            residual_dependency_repr: residual.map(str::to_owned),
            ..input(scope)
        })
        .expect("resolves");
        assert_eq!(resolved.scope, scope);
        // Provider-governed exactly when Managed; everything else is a local claim.
        assert_eq!(
            resolved.is_provider_governed,
            scope == M5DeploymentScopeBadgeValue::Managed
        );
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_label = M5DeploymentScopeBadgeInput {
        subject_label: "  ".to_owned(),
        ..input(M5DeploymentScopeBadgeValue::Managed)
    };
    assert_eq!(
        resolve_deployment_scope_badge(&empty_label),
        Err(M5DeploymentScopeBadgeError::EmptySubjectLabel)
    );

    let empty_ts = M5DeploymentScopeBadgeInput {
        last_evaluated_repr: "   ".to_owned(),
        ..input(M5DeploymentScopeBadgeValue::Managed)
    };
    assert_eq!(
        resolve_deployment_scope_badge(&empty_ts),
        Err(M5DeploymentScopeBadgeError::EmptyLastEvaluated)
    );

    let forbidden = sovereign_input(
        M5DeploymentScopeBadgeValue::LocalOnly,
        "https://example.test/dependency",
    );
    assert_eq!(
        resolve_deployment_scope_badge(&forbidden),
        Err(M5DeploymentScopeBadgeError::ForbiddenBadgeMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_deployment_scope_badge_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_deployment_scope_badge_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .badge_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5DeploymentScopeConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.badge_rows.len(),
        M5DeploymentScopeConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_explanation() {
    let packet = seeded_m5_deployment_scope_badge_primitive_packet();
    for row in &packet.badge_rows {
        for part in M5DeploymentScopeAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5DeploymentScopeExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for field in M5BadgeExplanationField::MANDATORY {
            assert!(row.explanation_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5BadgeAccessibilityRoute::KeyboardFocusable));
        assert!(row
            .accessibility_routes
            .contains(&M5BadgeAccessibilityRoute::NonColorEncoded));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_deployment_scope_badge_primitive_packet();
    let cases: Vec<&M5DeploymentScopeResolutionCase> = packet
        .badge_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for scope in M5DeploymentScopeBadgeValue::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.scope == scope),
            "no worked resolution exercises scope {}",
            scope.as_str()
        );
    }
    for posture in M5DeploymentSovereigntyPosture::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.sovereignty_posture == posture),
            "no worked resolution exercises posture {}",
            posture.as_str()
        );
    }
    for class in M5ResidualDependencyClass::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .residual_dependency_note
                .as_ref()
                .is_some_and(|n| n.residual_dependency_class == class)),
            "no worked resolution exercises residual-dependency class {}",
            class.as_str()
        );
    }
    for continuity in M5LocalSafeContinuity::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .residual_dependency_note
                .as_ref()
                .is_some_and(|n| n.local_safe_continuity == continuity)),
            "no worked resolution exercises local-safe continuity {}",
            continuity.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_deployment_scope_badge_primitive_packet();
    for row in &packet.badge_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet
        .badge_rows
        .retain(|row| row.consumer_surface != M5DeploymentScopeConsumerSurface::DiagnosticsReport);
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.vocabulary_set.scope_values.pop();
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.badge_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DeploymentScopeAnatomyPart::ResidualDependencyDrawer);
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.badge_rows[0]
        .export_fields
        .retain(|f| *f != M5DeploymentScopeExportField::ResidualDependency);
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn explanation_drawer_incomplete_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.badge_rows[0]
        .explanation_fields
        .retain(|f| *f != M5BadgeExplanationField::WhatItMeans);
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::ExplanationDrawerIncomplete));
}

#[test]
fn non_color_encoding_missing_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.badge_rows[0]
        .accessibility_routes
        .retain(|r| *r != M5BadgeAccessibilityRoute::NonColorEncoded);
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::AccessibilityRouteMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.badge_rows[0].example_resolutions[0]
        .resolved
        .is_provider_governed = true;
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn scope_axis_independence_unproven_fails_without_both_ends() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    // Replace every example with a managed (provider-governed) one so no locally-sovereign
    // example remains and neither offline/mirror nor companion coverage is proven.
    for row in &mut packet.badge_rows {
        row.example_resolutions = vec![M5DeploymentScopeResolutionCase::resolved(input(
            M5DeploymentScopeBadgeValue::Managed,
        ))];
    }
    let violations = packet.validate();
    assert!(violations
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::ScopeAxisIndependenceUnproven));
    assert!(violations.contains(
        &M5DeploymentScopeBadgePrimitiveViolation::OfflineMirrorAndBrowserCompanionUnproven
    ));
}

#[test]
fn residual_dependency_preservation_unproven_fails_when_no_sovereignty_example() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    for row in &mut packet.badge_rows {
        row.example_resolutions = vec![M5DeploymentScopeResolutionCase::resolved(input(
            M5DeploymentScopeBadgeValue::Managed,
        ))];
    }
    assert!(packet.validate().contains(
        &M5DeploymentScopeBadgePrimitiveViolation::ResidualDependencyPreservationUnproven
    ));
}

#[test]
fn offline_mirror_and_browser_companion_unproven_fails_without_companion() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    // Only local-only + managed examples: locally sovereign is proven, but neither
    // browser companion nor offline/mirror is present.
    for row in &mut packet.badge_rows {
        row.example_resolutions = vec![
            M5DeploymentScopeResolutionCase::resolved(sovereign_input(
                M5DeploymentScopeBadgeValue::LocalOnly,
                "signing:release-keyring/desktop",
            )),
            M5DeploymentScopeResolutionCase::resolved(input(M5DeploymentScopeBadgeValue::Managed)),
        ];
    }
    let violations = packet.validate();
    assert!(violations.contains(
        &M5DeploymentScopeBadgePrimitiveViolation::OfflineMirrorAndBrowserCompanionUnproven
    ));
    // Axis independence and residual preservation still hold.
    assert!(!violations
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::ScopeAxisIndependenceUnproven));
    assert!(!violations.contains(
        &M5DeploymentScopeBadgePrimitiveViolation::ResidualDependencyPreservationUnproven
    ));
}

#[test]
fn badge_invariant_violation_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.badge_rows[0].collapses_scope_into_support_lifecycle_or_channel = true;
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::BadgeInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.badge_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet
        .governance_review
        .sovereignty_claim_auto_discloses_residual_dependency = false;
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet
        .consumer_projection
        .sovereignty_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_deployment_scope_badge_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DeploymentScopeBadgePrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_deployment_scope_badge_primitive_packet().render_markdown_summary();
    for surface in M5DeploymentScopeConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_deployment_scope_badge_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5DeploymentScopeConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5DeploymentScopeConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_deployment_scope_badge_primitive_export()
        .expect("checked M5 deployment scope badge primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_DEPLOYMENT_SCOPE_BADGE_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_deployment_scope_badge_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_deployment_scope_badge_primitive_companion_mode_card_beta_narrowed(),
        seeded_m5_deployment_scope_badge_primitive_diagnostics_report_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.badge_rows.len(),
            M5DeploymentScopeConsumerSurface::ALL.len()
        );
    }

    let companion = seeded_m5_deployment_scope_badge_primitive_companion_mode_card_beta_narrowed();
    let row = companion
        .badge_rows
        .iter()
        .find(|r| r.consumer_surface == M5DeploymentScopeConsumerSurface::CompanionModeCard)
        .expect("companion row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Beta);

    let diagnostics =
        seeded_m5_deployment_scope_badge_primitive_diagnostics_report_preview_narrowed();
    let row = diagnostics
        .badge_rows
        .iter()
        .find(|r| r.consumer_surface == M5DeploymentScopeConsumerSurface::DiagnosticsReport)
        .expect("diagnostics row present");
    assert_eq!(row.qualification, M5BadgeQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let companion: M5DeploymentScopeBadgePrimitivePacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-deployment-scope-badges/companion_mode_card_beta_narrowed.json"
        )))
        .expect("companion fixture parses");
    assert!(companion.validate().is_empty());
    assert_eq!(
        companion,
        seeded_m5_deployment_scope_badge_primitive_companion_mode_card_beta_narrowed()
    );

    let diagnostics: M5DeploymentScopeBadgePrimitivePacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-deployment-scope-badges/diagnostics_report_preview_narrowed.json"
    )))
        .expect("diagnostics fixture parses");
    assert!(diagnostics.validate().is_empty());
    assert_eq!(
        diagnostics,
        seeded_m5_deployment_scope_badge_primitive_diagnostics_report_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_deployment_scope_badge_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
