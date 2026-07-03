use super::*;

fn item(
    token: &str,
    consequence: M5CapabilityConsequenceClass,
    decision: M5CapabilityDecision,
) -> M5CapabilityRequestItem {
    M5CapabilityRequestItem {
        capability_token: token.to_owned(),
        consequence_class: consequence,
        purpose_repr: "purpose".to_owned(),
        decision,
        policy_predecision: M5CapabilityPolicyPredecision::NoPolicy,
        is_transitive: false,
        transitive_origin_repr: None,
        reduced_mode_available: false,
        re_consent_triggered: false,
        has_prior_grant: false,
    }
}

fn input(
    surface: M5CapabilitySurfaceFamily,
    actor: &str,
    requests: Vec<M5CapabilityRequestItem>,
) -> M5CapabilitySheetResolutionInput {
    M5CapabilitySheetResolutionInput {
        surface_family: surface,
        actor_identity_repr: actor.to_owned(),
        requests,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_and_reduced_grants_are_revocable() {
    let mut modify = item(
        "modify_docs",
        M5CapabilityConsequenceClass::ModifyWorkspace,
        M5CapabilityDecision::ApproveReduced,
    );
    modify.reduced_mode_available = true;
    let resolved = resolve_capability_sheet(&input(
        M5CapabilitySurfaceFamily::ExtensionInstall,
        "extension:pack",
        vec![
            item(
                "read_files",
                M5CapabilityConsequenceClass::ReadLocalContext,
                M5CapabilityDecision::ApproveFull,
            ),
            modify,
        ],
    ))
    .expect("resolves");

    assert_eq!(
        resolved.resolved_requests[0].scope_state,
        M5CapabilityScopeState::GrantedFullScope
    );
    assert!(resolved.resolved_requests[0].revocable);
    assert_eq!(
        resolved.resolved_requests[1].scope_state,
        M5CapabilityScopeState::GrantedReducedScope
    );
    assert!(resolved.resolved_requests[1].reduced_mode_offered);
    assert!(resolved.reduced_mode_offered);
    assert!(resolved.revocable_from_settings);
    assert!(!resolved.widens_effective_scope);
    // Two distinct consequence classes → two groups, in canonical order.
    assert_eq!(resolved.consequence_groups.len(), 2);
    assert_eq!(
        resolved.consequence_groups[0].consequence_class,
        M5CapabilityConsequenceClass::ReadLocalContext
    );
    assert_eq!(
        resolved.consequence_groups[1].consequence_class,
        M5CapabilityConsequenceClass::ModifyWorkspace
    );
}

#[test]
fn resolver_groups_by_consequence_not_request_order() {
    let resolved = resolve_capability_sheet(&input(
        M5CapabilitySurfaceFamily::AiToolRequest,
        "ai-tool:x",
        vec![
            item(
                "net_a",
                M5CapabilityConsequenceClass::NetworkAccess,
                M5CapabilityDecision::RequestedNotGranted,
            ),
            item(
                "read_a",
                M5CapabilityConsequenceClass::ReadLocalContext,
                M5CapabilityDecision::RequestedNotGranted,
            ),
            item(
                "net_b",
                M5CapabilityConsequenceClass::NetworkAccess,
                M5CapabilityDecision::RequestedNotGranted,
            ),
        ],
    ))
    .expect("resolves");
    // ReadLocalContext precedes NetworkAccess in canonical order even though the
    // network requests were listed first; both network tokens share one group.
    assert_eq!(resolved.consequence_groups.len(), 2);
    assert_eq!(
        resolved.consequence_groups[0].consequence_class,
        M5CapabilityConsequenceClass::ReadLocalContext
    );
    assert_eq!(
        resolved.consequence_groups[1].capability_tokens,
        vec!["net_a".to_owned(), "net_b".to_owned()]
    );
}

#[test]
fn resolver_transitive_request_is_disclosed_and_widens_scope() {
    let mut transitive = item(
        "reach_endpoint",
        M5CapabilityConsequenceClass::NetworkAccess,
        M5CapabilityDecision::RequestedNotGranted,
    );
    transitive.is_transitive = true;
    transitive.transitive_origin_repr = Some("dependency:lib".to_owned());
    let resolved = resolve_capability_sheet(&input(
        M5CapabilitySurfaceFamily::AiToolRequest,
        "ai-tool:x",
        vec![transitive],
    ))
    .expect("resolves");
    assert_eq!(
        resolved.resolved_requests[0].scope_state,
        M5CapabilityScopeState::TransitiveScopeDisclosed
    );
    assert!(resolved.widens_effective_scope);
    // Disclosed but not granted → nothing to revoke.
    assert!(!resolved.resolved_requests[0].revocable);
}

#[test]
fn resolver_re_consent_wins_over_standing_grant() {
    let mut req = item(
        "reach_endpoint",
        M5CapabilityConsequenceClass::NetworkAccess,
        M5CapabilityDecision::ApproveFull,
    );
    req.re_consent_triggered = true;
    req.has_prior_grant = true;
    let resolved = resolve_capability_sheet(&input(
        M5CapabilitySurfaceFamily::ProviderRoute,
        "provider:x",
        vec![req],
    ))
    .expect("resolves");
    assert_eq!(
        resolved.resolved_requests[0].scope_state,
        M5CapabilityScopeState::ReConsentRequired
    );
    assert!(resolved.requires_re_consent);
    assert!(resolved.resolved_requests[0].revocable);
}

#[test]
fn resolver_revoke_keeps_history_and_is_not_revocable_again() {
    let mut req = item(
        "read_credential_handle",
        M5CapabilityConsequenceClass::CredentialAccess,
        M5CapabilityDecision::Revoke,
    );
    req.has_prior_grant = true;
    let resolved = resolve_capability_sheet(&input(
        M5CapabilitySurfaceFamily::RemoteConnector,
        "remote:x",
        vec![req],
    ))
    .expect("resolves");
    assert_eq!(
        resolved.resolved_requests[0].scope_state,
        M5CapabilityScopeState::RevokedWithHistory
    );
    assert!(!resolved.resolved_requests[0].revocable);
    assert!(!resolved.revocable_from_settings);
}

#[test]
fn resolver_rejects_policy_denied_approval() {
    let mut req = item(
        "control_host",
        M5CapabilityConsequenceClass::SystemControl,
        M5CapabilityDecision::ApproveFull,
    );
    req.policy_predecision = M5CapabilityPolicyPredecision::PreDenied;
    let err = resolve_capability_sheet(&input(
        M5CapabilitySurfaceFamily::AutomationFlow,
        "automation:x",
        vec![req],
    ))
    .unwrap_err();
    assert_eq!(
        err,
        M5CapabilityResolutionError::PolicyDeniedCapabilityApproved("control_host".to_owned())
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    // Empty actor.
    assert_eq!(
        resolve_capability_sheet(&input(
            M5CapabilitySurfaceFamily::ExtensionInstall,
            "  ",
            vec![item(
                "t",
                M5CapabilityConsequenceClass::ReadLocalContext,
                M5CapabilityDecision::RequestedNotGranted
            )],
        )),
        Err(M5CapabilityResolutionError::EmptyActorIdentity)
    );

    // No requests.
    assert_eq!(
        resolve_capability_sheet(&input(
            M5CapabilitySurfaceFamily::ExtensionInstall,
            "actor",
            vec![]
        )),
        Err(M5CapabilityResolutionError::NoRequests)
    );

    // Duplicate token.
    assert_eq!(
        resolve_capability_sheet(&input(
            M5CapabilitySurfaceFamily::ExtensionInstall,
            "actor",
            vec![
                item(
                    "dup",
                    M5CapabilityConsequenceClass::ReadLocalContext,
                    M5CapabilityDecision::RequestedNotGranted
                ),
                item(
                    "dup",
                    M5CapabilityConsequenceClass::ModifyWorkspace,
                    M5CapabilityDecision::RequestedNotGranted
                ),
            ],
        )),
        Err(M5CapabilityResolutionError::DuplicateCapability(
            "dup".to_owned()
        ))
    );

    // Transitive without an origin.
    let mut transitive = item(
        "t",
        M5CapabilityConsequenceClass::NetworkAccess,
        M5CapabilityDecision::RequestedNotGranted,
    );
    transitive.is_transitive = true;
    assert_eq!(
        resolve_capability_sheet(&input(
            M5CapabilitySurfaceFamily::AiToolRequest,
            "actor",
            vec![transitive]
        )),
        Err(M5CapabilityResolutionError::MissingTransitiveOrigin(
            "t".to_owned()
        ))
    );

    // Reduced selected but unavailable.
    let reduced = item(
        "t",
        M5CapabilityConsequenceClass::ModifyWorkspace,
        M5CapabilityDecision::ApproveReduced,
    );
    assert_eq!(
        resolve_capability_sheet(&input(
            M5CapabilitySurfaceFamily::ExtensionInstall,
            "actor",
            vec![reduced]
        )),
        Err(M5CapabilityResolutionError::ReducedSelectedButUnavailable(
            "t".to_owned()
        ))
    );

    // Forbidden material in a purpose.
    let mut forbidden = item(
        "t",
        M5CapabilityConsequenceClass::NetworkAccess,
        M5CapabilityDecision::RequestedNotGranted,
    );
    forbidden.purpose_repr = "reach https://example.test".to_owned();
    assert_eq!(
        resolve_capability_sheet(&input(
            M5CapabilitySurfaceFamily::AiToolRequest,
            "actor",
            vec![forbidden]
        )),
        Err(M5CapabilityResolutionError::ForbiddenMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_capability_sheet_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CAPABILITY_SHEET_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_surface_family() {
    let packet = seeded_m5_capability_sheet_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for family in M5CapabilitySurfaceFamily::ALL {
        assert!(
            present.contains(&family),
            "missing surface family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.surface_rows.len(),
        M5CapabilitySurfaceFamily::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_consent_and_export() {
    let packet = seeded_m5_capability_sheet_primitive_packet();
    for row in &packet.surface_rows {
        for part in M5CapabilitySheetAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for disclosure in M5CapabilityConsentDisclosure::MANDATORY {
            assert!(row.consent_disclosures.contains(&disclosure));
        }
        for field in M5CapabilitySheetExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TrustAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_sheets.is_empty());
    }
}

#[test]
fn every_scope_state_and_consequence_class_is_exercised_by_some_example() {
    let packet = seeded_m5_capability_sheet_primitive_packet();
    let resolved: Vec<&M5ResolvedCapabilityRequest> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_sheets.iter())
        .flat_map(|case| case.resolved.resolved_requests.iter())
        .collect();

    for state in M5CapabilityScopeState::ALL {
        assert!(
            resolved.iter().any(|r| r.scope_state == state),
            "no worked resolution exercises scope state {}",
            state.as_str()
        );
    }
    for class in M5CapabilityConsequenceClass::ALL {
        assert!(
            resolved.iter().any(|r| r.consequence_class == class),
            "no worked resolution exercises consequence class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_capability_sheet_primitive_packet();
    for row in &packet.surface_rows {
        for case in &row.example_sheets {
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
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet
        .surface_rows
        .retain(|row| row.surface_family != M5CapabilitySurfaceFamily::ProviderRoute);
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.vocabulary_set.anatomy_parts.pop();
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.surface_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5CapabilitySheetAnatomyPart::ConsequenceGroupedRequests);
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_consent_disclosure_missing_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.surface_rows[0]
        .consent_disclosures
        .retain(|d| *d != M5CapabilityConsentDisclosure::RevokePathShown);
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::MandatoryConsentDisclosureMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5CapabilitySheetExportField::ScopeState);
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_sheet_drift_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.surface_rows[0].example_sheets[0]
        .resolved
        .actor_identity_repr = "tampered".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::ExampleSheetDrift));
}

#[test]
fn example_sheet_missing_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.surface_rows[2].example_sheets.clear();
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::ExampleSheetMissing));
}

#[test]
fn transitive_disclosure_unproven_fails_when_no_example_discloses_transitive_scope() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    // Rewrite the AI-tool and privileged-helper examples so no example carries a
    // transitive-disclosed request, then confirm the packet-level lint fires.
    for row in &mut packet.surface_rows {
        for case in &mut row.example_sheets {
            case.input.requests.retain(|r| !r.is_transitive);
            *case = M5CapabilitySheetResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::TransitiveDisclosureUnproven));
}

#[test]
fn reduced_mode_unproven_fails_when_no_example_grants_reduced_scope() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    for row in &mut packet.surface_rows {
        for case in &mut row.example_sheets {
            for req in &mut case.input.requests {
                if req.decision == M5CapabilityDecision::ApproveReduced {
                    req.decision = M5CapabilityDecision::ApproveFull;
                }
            }
            *case = M5CapabilitySheetResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::ReducedModeUnproven));
}

#[test]
fn revocable_grant_unproven_fails_when_no_example_holds_a_revocable_grant() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    // Turn every grant / re-consent into a not-granted or revoked posture so no
    // resolved request is revocable.
    for row in &mut packet.surface_rows {
        for case in &mut row.example_sheets {
            for req in &mut case.input.requests {
                req.decision = M5CapabilityDecision::RequestedNotGranted;
                req.re_consent_triggered = false;
                req.policy_predecision = M5CapabilityPolicyPredecision::NoPolicy;
            }
            *case = M5CapabilitySheetResolutionCase::resolved(case.input.clone());
        }
    }
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::RevocableGrantUnproven));
}

#[test]
fn surface_invariant_violation_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.surface_rows[0].hides_transitive_scope = true;
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::SurfaceInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.surface_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.governance_review.transitive_scope_always_disclosed = false;
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet
        .consumer_projection
        .resolver_reads_single_scope_ladder = false;
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_capability_sheet_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CapabilitySheetPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface_family() {
    let summary = seeded_m5_capability_sheet_primitive_packet().render_markdown_summary();
    for family in M5CapabilitySurfaceFamily::ALL {
        assert!(
            summary.contains(family.label()),
            "summary missing surface {}",
            family.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_surface() {
    let csv = seeded_m5_capability_sheet_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5CapabilitySurfaceFamily::ALL.len());
    assert!(lines[0].starts_with("surface_family,qualification,owner,"));
    for family in M5CapabilitySurfaceFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing surface {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_capability_sheet_primitive_export()
        .expect("checked M5 capability-sheet primitive export validates");
    assert_eq!(from_disk.packet_id, M5_CAPABILITY_SHEET_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_capability_sheet_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_capability_sheet_primitive_automation_flow_beta_narrowed(),
        seeded_m5_capability_sheet_primitive_privileged_helper_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.surface_rows.len(),
            M5CapabilitySurfaceFamily::ALL.len()
        );
    }

    let automation = seeded_m5_capability_sheet_primitive_automation_flow_beta_narrowed();
    let row = automation
        .surface_rows
        .iter()
        .find(|r| r.surface_family == M5CapabilitySurfaceFamily::AutomationFlow)
        .expect("automation row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Beta);

    let helper = seeded_m5_capability_sheet_primitive_privileged_helper_preview_narrowed();
    let row = helper
        .surface_rows
        .iter()
        .find(|r| r.surface_family == M5CapabilitySurfaceFamily::PrivilegedHelper)
        .expect("helper row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let automation: M5CapabilitySheetPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-capability-sheet-primitive/automation_flow_beta_narrowed.json"
    )))
    .expect("automation fixture parses");
    assert!(automation.validate().is_empty());
    assert_eq!(
        automation,
        seeded_m5_capability_sheet_primitive_automation_flow_beta_narrowed()
    );

    let helper: M5CapabilitySheetPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-capability-sheet-primitive/privileged_helper_preview_narrowed.json"
    )))
    .expect("helper fixture parses");
    assert!(helper.validate().is_empty());
    assert_eq!(
        helper,
        seeded_m5_capability_sheet_primitive_privileged_helper_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_capability_sheet_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
