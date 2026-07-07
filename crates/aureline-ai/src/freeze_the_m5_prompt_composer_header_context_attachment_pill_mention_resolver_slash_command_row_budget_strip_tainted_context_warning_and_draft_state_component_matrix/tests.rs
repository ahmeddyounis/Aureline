use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_prompt_composer_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_PROMPT_COMPOSER_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_prompt_composer_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5PromptComposerComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5PromptComposerComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_prompt_composer_component_matrix();
    for row in &packet.component_rows {
        for label in M5ComposerRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_prompt_composer_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.composer_modes.is_empty(),
            family.is_composer_header(),
            "composer_modes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.composer_scopes.is_empty(),
            family.is_composer_header(),
            "composer_scopes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.route_classes.is_empty(),
            family.is_composer_header(),
            "route_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.attachment_kinds.is_empty(),
            family.is_attachment_pill(),
            "attachment_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.attachment_trust_states.is_empty(),
            family.is_attachment_pill(),
            "attachment_trust_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.mention_resolutions.is_empty(),
            family.is_mention_resolver(),
            "mention_resolutions presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.slash_command_states.is_empty(),
            family.is_slash_command_row(),
            "slash_command_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.budget_postures.is_empty(),
            family.is_budget_strip(),
            "budget_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.omitted_context_reasons.is_empty(),
            family.is_budget_strip(),
            "omitted_context_reasons presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.taint_sources.is_empty(),
            family.is_tainted_warning(),
            "taint_sources presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.taint_severities.is_empty(),
            family.is_tainted_warning(),
            "taint_severities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.draft_localities.is_empty(),
            family.is_draft_state_row(),
            "draft_localities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.staleness_reasons.is_empty(),
            family.is_attachment_stale_banner(),
            "staleness_reasons presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.send_postures.is_empty(),
            family.is_send_review_control(),
            "send_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.review_requirements.is_empty(),
            family.is_send_review_control(),
            "review_requirements presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_prompt_composer_component_matrix();
    for mode in M5ComposerMode::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.composer_modes.contains(&mode)),
            "no component declares composer mode {}",
            mode.as_str()
        );
    }
    for scope in M5ComposerScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.composer_scopes.contains(&scope)),
            "no component declares composer scope {}",
            scope.as_str()
        );
    }
    for class in M5ComposerRouteClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.route_classes.contains(&class)),
            "no component declares route class {}",
            class.as_str()
        );
    }
    for kind in M5AttachmentKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.attachment_kinds.contains(&kind)),
            "no component declares attachment kind {}",
            kind.as_str()
        );
    }
    for state in M5AttachmentTrustState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.attachment_trust_states.contains(&state)),
            "no component declares attachment trust state {}",
            state.as_str()
        );
    }
    for state in M5MentionResolution::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.mention_resolutions.contains(&state)),
            "no component declares mention resolution {}",
            state.as_str()
        );
    }
    for state in M5SlashCommandState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.slash_command_states.contains(&state)),
            "no component declares slash-command state {}",
            state.as_str()
        );
    }
    for posture in M5BudgetPosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.budget_postures.contains(&posture)),
            "no component declares budget posture {}",
            posture.as_str()
        );
    }
    for reason in M5OmittedContextReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.omitted_context_reasons.contains(&reason)),
            "no component declares omitted-context reason {}",
            reason.as_str()
        );
    }
    for source in M5TaintSource::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.taint_sources.contains(&source)),
            "no component declares taint source {}",
            source.as_str()
        );
    }
    for severity in M5TaintSeverity::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.taint_severities.contains(&severity)),
            "no component declares taint severity {}",
            severity.as_str()
        );
    }
    for locality in M5DraftLocality::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.draft_localities.contains(&locality)),
            "no component declares draft locality {}",
            locality.as_str()
        );
    }
    for reason in M5StalenessReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.staleness_reasons.contains(&reason)),
            "no component declares staleness reason {}",
            reason.as_str()
        );
    }
    for posture in M5SendPosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.send_postures.contains(&posture)),
            "no component declares send posture {}",
            posture.as_str()
        );
    }
    for requirement in M5ReviewRequirement::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.review_requirements.contains(&requirement)),
            "no component declares review requirement {}",
            requirement.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.component_rows.retain(|row| {
        row.component_family != M5PromptComposerComponentFamily::TaintedContextWarning
    });
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.vocabulary_set.composer_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5ComposerRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn composer_header_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_prompt_composer_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5PromptComposerComponentFamily::PromptComposerHeader
            })
            .expect("composer header present");
        let expected = match clear {
            0 => {
                row.composer_modes.clear();
                M5PromptComposerComponentMatrixViolation::ComposerModeMissing
            }
            1 => {
                row.composer_scopes.clear();
                M5PromptComposerComponentMatrixViolation::ComposerScopeMissing
            }
            _ => {
                row.route_classes.clear();
                M5PromptComposerComponentMatrixViolation::RouteClassMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn attachment_pill_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_prompt_composer_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5PromptComposerComponentFamily::ContextAttachmentPill
            })
            .expect("attachment pill present");
        let expected = if clear == 0 {
            row.attachment_kinds.clear();
            M5PromptComposerComponentMatrixViolation::AttachmentKindMissing
        } else {
            row.attachment_trust_states.clear();
            M5PromptComposerComponentMatrixViolation::AttachmentTrustStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn mention_resolver_vocab_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5PromptComposerComponentFamily::MentionResolver)
        .expect("mention resolver present");
    row.mention_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::MentionResolutionMissing));
}

#[test]
fn slash_command_row_vocab_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5PromptComposerComponentFamily::SlashCommandRow)
        .expect("slash-command row present");
    row.slash_command_states.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::SlashCommandStateMissing));
}

#[test]
fn budget_strip_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_prompt_composer_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5PromptComposerComponentFamily::BudgetSizeStrip)
            .expect("budget strip present");
        let expected = if clear == 0 {
            row.budget_postures.clear();
            M5PromptComposerComponentMatrixViolation::BudgetPostureMissing
        } else {
            row.omitted_context_reasons.clear();
            M5PromptComposerComponentMatrixViolation::OmittedContextReasonMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn tainted_warning_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_prompt_composer_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5PromptComposerComponentFamily::TaintedContextWarning
            })
            .expect("tainted-context warning present");
        let expected = if clear == 0 {
            row.taint_sources.clear();
            M5PromptComposerComponentMatrixViolation::TaintSourceMissing
        } else {
            row.taint_severities.clear();
            M5PromptComposerComponentMatrixViolation::TaintSeverityMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn draft_state_row_vocab_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5PromptComposerComponentFamily::DraftStateRow)
        .expect("draft-state row present");
    row.draft_localities.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::DraftLocalityMissing));
}

#[test]
fn attachment_stale_banner_vocab_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5PromptComposerComponentFamily::AttachmentStaleBanner)
        .expect("attachment-stale banner present");
    row.staleness_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::StalenessReasonMissing));
}

#[test]
fn send_review_control_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_prompt_composer_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5PromptComposerComponentFamily::SendReviewControl)
            .expect("send-review control present");
        let expected = if clear == 0 {
            row.send_postures.clear();
            M5PromptComposerComponentMatrixViolation::SendPostureMissing
        } else {
            row.review_requirements.clear();
            M5PromptComposerComponentMatrixViolation::ReviewRequirementMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.component_rows[0].masks_mode_or_route = true;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.component_rows[5].hides_taint_or_trust_state = true;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.component_rows[3].invents_private_composer_grammar = true;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.component_rows[8].bypasses_send_review_gate = true;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5PromptComposerComponentFamily::PromptComposerHeader)
        .expect("composer header present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet
        .governance_review
        .tainted_context_never_shown_as_trusted = false;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet
        .consumer_projection
        .help_and_companion_surfaces_read_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_prompt_composer_component_matrix().render_markdown_summary();
    for family in M5PromptComposerComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_prompt_composer_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5PromptComposerComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5PromptComposerComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_prompt_composer_component_matrix_export()
        .expect("checked M5 prompt composer component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_PROMPT_COMPOSER_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_prompt_composer_component_matrix_export()
        .expect("checked M5 prompt composer component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_prompt_composer_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_prompt_composer_component_matrix_tainted_context_warning_beta_narrowed(),
        seeded_m5_prompt_composer_component_matrix_send_review_control_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5PromptComposerComponentFamily::ALL.len()
        );
    }

    let taint = seeded_m5_prompt_composer_component_matrix_tainted_context_warning_beta_narrowed();
    let row = taint
        .component_rows
        .iter()
        .find(|r| r.component_family == M5PromptComposerComponentFamily::TaintedContextWarning)
        .expect("tainted-context-warning row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Beta);

    let send = seeded_m5_prompt_composer_component_matrix_send_review_control_preview_narrowed();
    let row = send
        .component_rows
        .iter()
        .find(|r| r.component_family == M5PromptComposerComponentFamily::SendReviewControl)
        .expect("send-review-control row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let taint: M5PromptComposerComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/tainted_context_warning_beta_narrowed.json"
    )))
    .expect("tainted-context-warning fixture parses");
    assert!(taint.validate().is_empty());
    assert_eq!(
        taint,
        seeded_m5_prompt_composer_component_matrix_tainted_context_warning_beta_narrowed()
    );

    let send: M5PromptComposerComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix/send_review_control_preview_narrowed.json"
    )))
    .expect("send-review-control fixture parses");
    assert!(send.validate().is_empty());
    assert_eq!(
        send,
        seeded_m5_prompt_composer_component_matrix_send_review_control_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_prompt_composer_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
