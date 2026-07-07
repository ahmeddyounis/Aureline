use super::*;

fn full_input(
    consumer: M5ComposerComponentConsumer,
    family: M5PromptComposerComponentFamily,
) -> M5ComposerBindingInput {
    M5ComposerBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5ComposerParityDescriptor::ALL.to_vec(),
        parity_health: M5ComposerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_composer_binding(&full_input(
        M5ComposerComponentConsumer::InlinePanel,
        M5PromptComposerComponentFamily::PromptComposerHeader,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert_eq!(
        resolved.claim_parity_state,
        M5ComposerClaimParityState::ClaimsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5PromptComposerComponentFamily::PromptComposerHeader)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5ComposerBindingInput {
        parity_health: M5ComposerParityHealth::ReviewOnlyNarrowed,
        export_caveats: vec![M5ComposerConsumerExportCaveat::SendPathDisabledReviewOnly],
        ..full_input(
            M5ComposerComponentConsumer::PatchReview,
            M5PromptComposerComponentFamily::SendReviewControl,
        )
    };
    let resolved = resolve_composer_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.claim_parity_state,
        M5ComposerClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5ComposerParityNarrowingReason::ReviewOnlyWorkflow
    );
    assert_eq!(
        banner.recovery_action,
        M5ComposerParityRecoveryAction::ReturnToLiveComposerToSend
    );
    // Descriptors stay preserved even under the narrowing.
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5ComposerParityDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    // Not a generic "degraded" note.
    assert!(banner.headline.to_lowercase().contains("review-only"));
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5ComposerParityHealth::ReviewOnlyNarrowed,
            M5ComposerParityNarrowingReason::ReviewOnlyWorkflow,
        ),
        (
            M5ComposerParityHealth::HandoffOnlyNarrowed,
            M5ComposerParityNarrowingReason::HandoffOnlyWorkflow,
        ),
        (
            M5ComposerParityHealth::OfflineMirrorNarrowed,
            M5ComposerParityNarrowingReason::OfflineOrMirrorScope,
        ),
        (
            M5ComposerParityHealth::CompanionScopeNarrowed,
            M5ComposerParityNarrowingReason::CompanionScopeLimited,
        ),
    ] {
        let input = M5ComposerBindingInput {
            parity_health: health,
            ..full_input(
                M5ComposerComponentConsumer::Companion,
                M5PromptComposerComponentFamily::DraftStateRow,
            )
        };
        let resolved = resolve_composer_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5ComposerBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5ComposerComponentConsumer::InlinePanel,
            M5PromptComposerComponentFamily::PromptComposerHeader,
        )
    };
    assert_eq!(
        resolve_composer_binding(&empty),
        Err(M5ComposerBindingError::EmptyDescriptorSet)
    );

    let missing = M5ComposerBindingInput {
        descriptor_families: vec![M5ComposerParityDescriptor::Route],
        ..full_input(
            M5ComposerComponentConsumer::InlinePanel,
            M5PromptComposerComponentFamily::PromptComposerHeader,
        )
    };
    assert_eq!(
        resolve_composer_binding(&missing),
        Err(M5ComposerBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5ComposerBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5ComposerComponentConsumer::InlinePanel,
            M5PromptComposerComponentFamily::PromptComposerHeader,
        )
    };
    assert_eq!(
        resolve_composer_binding(&forbidden),
        Err(M5ComposerBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes::M5_BUDGET_TAINT_SCHEMA_REF;
    use crate::implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces::M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_REF;
    use crate::ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces::M5_DRAFT_SEND_SCHEMA_REF;
    use crate::ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces::M5_MENTION_SLASH_COMMAND_SCHEMA_REF;
    use M5PromptComposerComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::PromptComposerHeader),
        M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ContextAttachmentPill),
        M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::MentionResolver),
        M5_MENTION_SLASH_COMMAND_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SlashCommandRow),
        M5_MENTION_SLASH_COMMAND_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::BudgetSizeStrip),
        M5_BUDGET_TAINT_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::TaintedContextWarning),
        M5_BUDGET_TAINT_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::DraftStateRow),
        M5_DRAFT_SEND_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::AttachmentStaleBanner),
        M5_DRAFT_SEND_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SendReviewControl),
        M5_DRAFT_SEND_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_prompt_composer_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_prompt_composer_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5ComposerComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5ComposerComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_prompt_composer_component_consumer_packet();
    for family in M5PromptComposerComponentFamily::ALL {
        let count = packet
            .consumer_rows
            .iter()
            .filter(|row| {
                row.component_bindings
                    .iter()
                    .any(|b| b.component_family == family)
            })
            .count();
        assert!(
            count >= 2,
            "family {} adopted by only {} consumer(s)",
            family.as_str(),
            count
        );
    }
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_descriptors() {
    let packet = seeded_m5_prompt_composer_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5ComposerConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5ComposerConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5ComposerParityDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_prompt_composer_component_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            assert_eq!(
                b.canonical_schema_ref,
                family_canonical_schema_ref(b.component_family)
            );
            assert_eq!(
                b.canonical_artifact_ref,
                family_canonical_artifact_ref(b.component_family)
            );
            assert!(b.references_canonical_not_local_prose);
        }
    }
}

#[test]
fn every_parity_health_mode_reason_and_parity_state_is_exercised() {
    let packet = seeded_m5_prompt_composer_component_consumer_packet();
    let cases: Vec<&M5ComposerBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5ComposerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5ComposerParityNarrowingReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked binding exercises narrowing reason {}",
            reason.as_str()
        );
    }
    for state in M5ComposerClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_prompt_composer_component_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                assert!(
                    case.is_self_consistent(),
                    "worked binding for {} drifted from resolver output",
                    row.consumer.as_str()
                );
            }
        }
    }
}

#[test]
fn missing_consumer_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5ComposerComponentConsumer::Companion);
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ai/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5ComposerParityDescriptor::Route);
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5ComposerConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    // Strip every PromptComposerHeader binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5PromptComposerComponentFamily::PromptComposerHeader {
                if seen_first {
                    return false;
                }
                seen_first = true;
            }
            true
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5ComposerBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.consumer_rows[0].inherits_stronger_label_from_healthier_surface = true;
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn docs_help_reference_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5ComposerComponentConsumer::DocsHelp)
        .expect("docs/help row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(violations.contains(&M5ComposerComponentConsumerViolation::DocsHelpReferenceMissing));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet
        .governance_review
        .degraded_workflow_auto_narrows_claim = false;
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.consumer_projection.route_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ComposerComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_prompt_composer_component_consumer_packet().render_markdown_summary();
    for consumer in M5ComposerComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_prompt_composer_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ComposerComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5ComposerComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_prompt_composer_component_consumer_export()
        .expect("checked M5 prompt-composer component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_prompt_composer_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_prompt_composer_component_consumer_branch_agent_beta_narrowed(),
        seeded_m5_prompt_composer_component_consumer_companion_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5ComposerComponentConsumer::ALL.len()
        );
    }

    let branch = seeded_m5_prompt_composer_component_consumer_branch_agent_beta_narrowed();
    let row = branch
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5ComposerComponentConsumer::BranchAgent)
        .expect("branch-agent row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Beta);

    let companion = seeded_m5_prompt_composer_component_consumer_companion_preview_narrowed();
    let row = companion
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5ComposerComponentConsumer::Companion)
        .expect("companion row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let branch: M5ComposerComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/m5-prompt-composer-component-consumers/branch_agent_beta_narrowed.json"
    )))
    .expect("branch-agent fixture parses");
    assert!(branch.validate().is_empty());
    assert_eq!(
        branch,
        seeded_m5_prompt_composer_component_consumer_branch_agent_beta_narrowed()
    );

    let companion: M5ComposerComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/m5-prompt-composer-component-consumers/companion_preview_narrowed.json"
    )))
    .expect("companion fixture parses");
    assert!(companion.validate().is_empty());
    assert_eq!(
        companion,
        seeded_m5_prompt_composer_component_consumer_companion_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_prompt_composer_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
