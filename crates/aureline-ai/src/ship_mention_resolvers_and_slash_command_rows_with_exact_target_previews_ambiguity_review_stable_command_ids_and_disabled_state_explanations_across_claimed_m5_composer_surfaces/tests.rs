use super::*;

fn unique_mention() -> M5MentionResolverResolutionInput {
    M5MentionResolverResolutionInput {
        mention_token: "@parse_config".to_owned(),
        scope_note: "scope: active file symbols".to_owned(),
        candidate_count: 1,
        has_exact_stable_target: true,
        target_is_pinned: false,
        target_object_id: Some("obj.symbol.parse_config".to_owned()),
        target_preview_label: Some("fn parse_config".to_owned()),
        in_scope: true,
        deferred: false,
    }
}

fn ready_command() -> M5SlashCommandRowResolutionInput {
    M5SlashCommandRowResolutionInput {
        command_id: "cmd.ai.explain".to_owned(),
        command_label: "Explain selection".to_owned(),
        capability_class: M5SlashCommandCapabilityClass::ReadOnlyQuery,
        help_path: "docs/help/commands/ai-explain.md".to_owned(),
        state: M5SlashCommandState::Available,
        requires_approval: false,
        disabled_reason: None,
        alias_of: None,
    }
}

// ---- mention resolver ---------------------------------------------------

#[test]
fn mention_exact_stable_binds_unique_with_preview() {
    let resolved = resolve_mention_resolver(&unique_mention()).expect("resolves");
    assert_eq!(resolved.resolution, M5MentionResolution::ResolvedUnique);
    assert!(resolved.is_bound);
    assert!(!resolved.blocks_send);
    assert!(!resolved.needs_explicit_review);
    assert!(resolved.preserves_exact_target_preview);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5MentionResolverAction::OpenTarget,
            M5MentionResolverAction::RemoveMention
        ]
    );
    assert_eq!(resolved.mention_token, "@parse_config");
    assert_eq!(resolved.scope_note, "scope: active file symbols");
}

#[test]
fn mention_pinned_binds_pinned() {
    let resolved = resolve_mention_resolver(&M5MentionResolverResolutionInput {
        target_is_pinned: true,
        ..unique_mention()
    })
    .expect("resolves");
    assert_eq!(resolved.resolution, M5MentionResolution::ResolvedPinned);
    assert!(resolved.is_bound);
}

#[test]
fn mention_resolution_ladder_prefers_exact_stable_and_never_silently_binds() {
    // Out of scope wins first, even with candidates.
    let oos = resolve_mention_resolver(&M5MentionResolverResolutionInput {
        in_scope: false,
        ..unique_mention()
    })
    .expect("resolves");
    assert_eq!(oos.resolution, M5MentionResolution::OutOfScopeDenied);
    assert!(oos.blocks_send);
    assert!(oos
        .available_actions
        .contains(&M5MentionResolverAction::RevealScope));

    // No candidates reads as unresolved-missing.
    let missing = resolve_mention_resolver(&M5MentionResolverResolutionInput {
        candidate_count: 0,
        has_exact_stable_target: false,
        target_object_id: None,
        target_preview_label: None,
        ..unique_mention()
    })
    .expect("resolves");
    assert_eq!(missing.resolution, M5MentionResolution::UnresolvedMissing);
    assert!(missing.blocks_send);
    assert!(missing.needs_explicit_review);
    assert!(missing
        .available_actions
        .contains(&M5MentionResolverAction::EditMention));

    // Deferred reads as deferred-pending.
    let deferred = resolve_mention_resolver(&M5MentionResolverResolutionInput {
        deferred: true,
        has_exact_stable_target: false,
        target_object_id: None,
        target_preview_label: None,
        ..unique_mention()
    })
    .expect("resolves");
    assert_eq!(deferred.resolution, M5MentionResolution::DeferredPending);
    assert!(deferred.blocks_send);

    // An exact stable target binds even with several candidates (prefers exact stable).
    let preferred = resolve_mention_resolver(&M5MentionResolverResolutionInput {
        candidate_count: 4,
        ..unique_mention()
    })
    .expect("resolves");
    assert_eq!(preferred.resolution, M5MentionResolution::ResolvedUnique);

    // Several candidates with no exact stable target read as ambiguous and block send.
    let ambiguous = resolve_mention_resolver(&M5MentionResolverResolutionInput {
        candidate_count: 3,
        has_exact_stable_target: false,
        target_object_id: None,
        target_preview_label: None,
        ..unique_mention()
    })
    .expect("resolves");
    assert_eq!(
        ambiguous.resolution,
        M5MentionResolution::AmbiguousCandidates
    );
    assert!(ambiguous.blocks_send);
    assert!(ambiguous.needs_explicit_review);
    assert!(ambiguous
        .available_actions
        .contains(&M5MentionResolverAction::ChooseCandidate));
}

#[test]
fn mention_always_offers_remove() {
    for input in [
        unique_mention(),
        M5MentionResolverResolutionInput {
            in_scope: false,
            ..unique_mention()
        },
        M5MentionResolverResolutionInput {
            candidate_count: 0,
            has_exact_stable_target: false,
            target_object_id: None,
            target_preview_label: None,
            ..unique_mention()
        },
    ] {
        let resolved = resolve_mention_resolver(&input).expect("resolves");
        assert!(resolved
            .available_actions
            .contains(&M5MentionResolverAction::RemoveMention));
    }
}

#[test]
fn mention_rejects_malformed_input() {
    assert_eq!(
        resolve_mention_resolver(&M5MentionResolverResolutionInput {
            mention_token: "  ".to_owned(),
            ..unique_mention()
        }),
        Err(M5MentionResolverResolutionError::EmptyMentionToken)
    );
    assert_eq!(
        resolve_mention_resolver(&M5MentionResolverResolutionInput {
            scope_note: "".to_owned(),
            ..unique_mention()
        }),
        Err(M5MentionResolverResolutionError::EmptyScopeNote)
    );
    assert_eq!(
        resolve_mention_resolver(&M5MentionResolverResolutionInput {
            target_preview_label: None,
            ..unique_mention()
        }),
        Err(M5MentionResolverResolutionError::BoundMentionWithoutTarget)
    );
    assert_eq!(
        resolve_mention_resolver(&M5MentionResolverResolutionInput {
            scope_note: "scope: https://leak.test".to_owned(),
            ..unique_mention()
        }),
        Err(M5MentionResolverResolutionError::ForbiddenMentionMaterial)
    );
}

// ---- slash-command-row resolver -----------------------------------------

#[test]
fn slash_available_reads_ready_invocable() {
    let resolved = resolve_slash_command_row(&ready_command()).expect("resolves");
    assert_eq!(
        resolved.row_posture,
        M5SlashCommandRowPosture::ReadyInvocable
    );
    assert!(resolved.is_invocable);
    assert!(!resolved.is_blocked);
    assert!(!resolved.requires_approval_before_run);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5SlashCommandRowAction::Invoke,
            M5SlashCommandRowAction::OpenHelp
        ]
    );
    assert_eq!(resolved.command_id, "cmd.ai.explain");
}

#[test]
fn slash_posture_ladder_is_blocking_first_and_approval_escalates() {
    // Unknown wins first.
    let unknown = resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
        state: M5SlashCommandState::UnknownCommand,
        ..ready_command()
    })
    .expect("resolves");
    assert_eq!(
        unknown.row_posture,
        M5SlashCommandRowPosture::UnknownRejected
    );
    assert!(unknown.is_blocked);
    assert!(unknown
        .available_actions
        .contains(&M5SlashCommandRowAction::ExplainDisabled));

    // Policy-hidden requires a reason.
    let hidden = resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
        state: M5SlashCommandState::PolicyHidden,
        disabled_reason: Some("hidden by policy".to_owned()),
        ..ready_command()
    })
    .expect("resolves");
    assert_eq!(hidden.row_posture, M5SlashCommandRowPosture::PolicyHidden);
    assert!(hidden.is_blocked);

    // Disabled-unmet-precondition reads as disabled-explained.
    let disabled = resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
        state: M5SlashCommandState::DisabledUnmetPrecondition,
        disabled_reason: Some("no symbol under the cursor".to_owned()),
        ..ready_command()
    })
    .expect("resolves");
    assert_eq!(
        disabled.row_posture,
        M5SlashCommandRowPosture::DisabledExplained
    );
    assert!(disabled.explains_disabled_state);

    // Declared approval gates the command.
    let approval = resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
        state: M5SlashCommandState::RequiresApproval,
        ..ready_command()
    })
    .expect("resolves");
    assert_eq!(
        approval.row_posture,
        M5SlashCommandRowPosture::ApprovalGated
    );
    assert!(approval.requires_approval_before_run);
    assert!(approval
        .available_actions
        .contains(&M5SlashCommandRowAction::RequestApproval));

    // An available command that still requires approval is escalated to approval-gated.
    let escalated = resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
        state: M5SlashCommandState::Available,
        requires_approval: true,
        ..ready_command()
    })
    .expect("resolves");
    assert_eq!(
        escalated.row_posture,
        M5SlashCommandRowPosture::ApprovalGated
    );

    // Deprecated redirects and offers view-canonical.
    let deprecated = resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
        state: M5SlashCommandState::DeprecatedAliased,
        alias_of: Some("cmd.help.index".to_owned()),
        ..ready_command()
    })
    .expect("resolves");
    assert_eq!(
        deprecated.row_posture,
        M5SlashCommandRowPosture::DeprecatedRedirect
    );
    assert!(deprecated.is_invocable);
    assert!(deprecated
        .available_actions
        .contains(&M5SlashCommandRowAction::ViewCanonical));
}

#[test]
fn slash_always_offers_open_help() {
    for state in M5SlashCommandState::ALL {
        let input = M5SlashCommandRowResolutionInput {
            state,
            requires_approval: false,
            disabled_reason: Some("reason".to_owned()),
            alias_of: Some("cmd.canonical".to_owned()),
            ..ready_command()
        };
        let resolved = resolve_slash_command_row(&input).expect("resolves");
        assert!(resolved
            .available_actions
            .contains(&M5SlashCommandRowAction::OpenHelp));
    }
}

#[test]
fn slash_rejects_malformed_input() {
    assert_eq!(
        resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
            command_id: " ".to_owned(),
            ..ready_command()
        }),
        Err(M5SlashCommandRowResolutionError::EmptyCommandId)
    );
    assert_eq!(
        resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
            command_label: "".to_owned(),
            ..ready_command()
        }),
        Err(M5SlashCommandRowResolutionError::EmptyCommandLabel)
    );
    assert_eq!(
        resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
            help_path: "".to_owned(),
            ..ready_command()
        }),
        Err(M5SlashCommandRowResolutionError::EmptyHelpPath)
    );
    assert_eq!(
        resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
            state: M5SlashCommandState::DisabledUnmetPrecondition,
            disabled_reason: None,
            ..ready_command()
        }),
        Err(M5SlashCommandRowResolutionError::DisabledWithoutExplanation)
    );
    assert_eq!(
        resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
            state: M5SlashCommandState::DeprecatedAliased,
            alias_of: None,
            ..ready_command()
        }),
        Err(M5SlashCommandRowResolutionError::DeprecatedWithoutCanonicalTarget)
    );
    assert_eq!(
        resolve_slash_command_row(&M5SlashCommandRowResolutionInput {
            help_path: "help://commands/x".to_owned(),
            ..ready_command()
        }),
        Err(M5SlashCommandRowResolutionError::ForbiddenCommandMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_mention_slash_command_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_MENTION_SLASH_COMMAND_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_mention_slash_command_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5MentionSlashCommandConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5MentionSlashCommandConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_mention_slash_command_packet();
    for row in &packet.rows {
        for part in M5MentionResolverAnatomyPart::MANDATORY {
            assert!(row.mention_anatomy_parts.contains(&part));
        }
        for part in M5SlashCommandRowAnatomyPart::MANDATORY {
            assert!(row.slash_anatomy_parts.contains(&part));
        }
        for field in M5MentionResolverExportField::MANDATORY {
            assert!(row.mention_export_fields.contains(&field));
        }
        for field in M5SlashCommandRowExportField::MANDATORY {
            assert!(row.slash_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable));
        assert!(!row.mention_examples.is_empty());
        assert!(!row.slash_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_mention_slash_command_packet();
    let mentions: Vec<&M5MentionResolverResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.mention_examples.iter())
        .collect();
    let commands: Vec<&M5SlashCommandRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.slash_examples.iter())
        .collect();

    for resolution in M5MentionResolution::ALL {
        assert!(
            mentions.iter().any(|c| c.resolved.resolution == resolution),
            "no mention example exercises resolution {}",
            resolution.as_str()
        );
    }
    for action in M5MentionResolverAction::ALL {
        assert!(
            mentions
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no mention example exercises action {}",
            action.as_str()
        );
    }
    for posture in M5SlashCommandRowPosture::ALL {
        assert!(
            commands.iter().any(|c| c.resolved.row_posture == posture),
            "no command example exercises posture {}",
            posture.as_str()
        );
    }
    for action in M5SlashCommandRowAction::ALL {
        assert!(
            commands
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no command example exercises action {}",
            action.as_str()
        );
    }
    for capability in M5SlashCommandCapabilityClass::ALL {
        assert!(
            commands
                .iter()
                .any(|c| c.resolved.capability_class == capability),
            "no command example exercises capability {}",
            capability.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_mention_slash_command_packet();
    for row in &packet.rows {
        for case in &row.mention_examples {
            assert!(
                case.is_self_consistent(),
                "mention case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "mention case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.slash_examples {
            assert!(
                case.is_self_consistent(),
                "command case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_command_id(),
                "command case for {} lost id",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5MentionSlashCommandConsumerSurface::CommandPalette);
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.vocabulary_set.slash_row_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::VocabularySetDrift));
}

#[test]
fn mandatory_mention_anatomy_missing_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.rows[0]
        .mention_anatomy_parts
        .retain(|p| *p != M5MentionResolverAnatomyPart::TargetPreviewCue);
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::MandatoryMentionAnatomyMissing));
}

#[test]
fn mandatory_slash_export_missing_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.rows[0]
        .slash_export_fields
        .retain(|f| *f != M5SlashCommandRowExportField::CapabilityClass);
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::MandatorySlashExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.rows[0].mention_examples[0].resolved.is_bound = false;
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::ExampleResolutionDrift));
}

#[test]
fn mention_example_missing_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.rows[1].mention_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::MentionExampleMissing));
}

#[test]
fn mention_bind_coverage_unproven_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    // Replace every mention example with a plainly bound one so the blocked half fires.
    for row in &mut packet.rows {
        row.mention_examples = vec![M5MentionResolverResolutionCase::resolved(unique_mention())];
    }
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::MentionBindCoverageUnproven));
}

#[test]
fn mention_ambiguity_review_unproven_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    // No ambiguous example anywhere: keep only a bound and an unresolved example.
    for row in &mut packet.rows {
        row.mention_examples = vec![
            M5MentionResolverResolutionCase::resolved(unique_mention()),
            M5MentionResolverResolutionCase::resolved(M5MentionResolverResolutionInput {
                candidate_count: 0,
                has_exact_stable_target: false,
                target_object_id: None,
                target_preview_label: None,
                ..unique_mention()
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::MentionAmbiguityReviewUnproven));
}

#[test]
fn slash_disabled_explanation_unproven_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    // Replace every command example with a plainly ready one so no disabled-explained proof
    // survives.
    for row in &mut packet.rows {
        row.slash_examples = vec![
            M5SlashCommandRowResolutionCase::resolved(ready_command()),
            M5SlashCommandRowResolutionCase::resolved(M5SlashCommandRowResolutionInput {
                state: M5SlashCommandState::RequiresApproval,
                ..ready_command()
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::SlashDisabledExplanationUnproven));
}

#[test]
fn slash_approval_availability_coverage_unproven_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    // Replace every command example with a ready one so the approval half fires.
    for row in &mut packet.rows {
        row.slash_examples = vec![M5SlashCommandRowResolutionCase::resolved(ready_command())];
    }
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::SlashApprovalAvailabilityCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.rows[0].hides_mention_resolution_or_ambiguity = true;
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.governance_review.disabled_state_always_explained = false;
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet
        .consumer_projection
        .slash_command_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_mention_slash_command_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5MentionSlashCommandViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_mention_slash_command_packet().render_markdown_summary();
    for surface in M5MentionSlashCommandConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_mention_slash_command_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5MentionSlashCommandConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5MentionSlashCommandConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_mention_slash_command_export()
        .expect("checked M5 mention/command primitive export validates");
    assert_eq!(from_disk.packet_id, M5_MENTION_SLASH_COMMAND_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_mention_slash_command_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_mention_slash_command_automation_recipe_preview_narrowed(),
        seeded_m5_mention_slash_command_cli_headless_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5MentionSlashCommandConsumerSurface::ALL.len()
        );
    }

    let automation = seeded_m5_mention_slash_command_automation_recipe_preview_narrowed();
    let row = automation
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5MentionSlashCommandConsumerSurface::AutomationRecipe)
        .expect("automation-recipe row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Preview);

    let cli = seeded_m5_mention_slash_command_cli_headless_beta_narrowed();
    let row = cli
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5MentionSlashCommandConsumerSurface::CliHeadless)
        .expect("cli-headless row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let automation: M5MentionSlashCommandPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces/automation_recipe_preview_narrowed.json"
    )))
    .expect("automation-recipe fixture parses");
    assert!(automation.validate().is_empty());
    assert_eq!(
        automation,
        seeded_m5_mention_slash_command_automation_recipe_preview_narrowed()
    );

    let cli: M5MentionSlashCommandPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces/cli_headless_beta_narrowed.json"
    )))
    .expect("cli-headless fixture parses");
    assert!(cli.validate().is_empty());
    assert_eq!(
        cli,
        seeded_m5_mention_slash_command_cli_headless_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_mention_slash_command_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
