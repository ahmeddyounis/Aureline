use super::*;

fn ready_header() -> M5PromptComposerHeaderResolutionInput {
    M5PromptComposerHeaderResolutionInput {
        composer_mode: M5ComposerMode::ChatAsk,
        composer_scope: M5ComposerScope::ActiveFile,
        route_class: M5ComposerRouteClass::ManagedRoute,
        provider_model_label: "managed:model-x".to_owned(),
        budget_posture: M5BudgetPosture::WithinBudget,
        route_blocked: false,
        review_context_available: true,
    }
}

fn fresh_pill() -> M5ContextAttachmentPillResolutionInput {
    M5ContextAttachmentPillResolutionInput {
        attachment_id: "attach.file.x".to_owned(),
        attachment_label: "x.rs".to_owned(),
        attachment_kind: M5AttachmentKind::File,
        trust_state: M5AttachmentTrustState::TrustedFresh,
        is_stale: false,
        staleness_reason: None,
        source_removed: false,
        in_scope: true,
    }
}

// ---- header resolver ----------------------------------------------------

#[test]
fn header_ready_managed_route_is_sendable_and_not_local() {
    let resolved = resolve_prompt_composer_header(&ready_header()).expect("resolves");
    assert_eq!(
        resolved.header_posture,
        M5ComposerHeaderPosture::ReadyComposing
    );
    assert!(resolved.is_sendable);
    assert!(!resolved.needs_attention);
    assert!(!resolved.route_stays_local);
    assert!(resolved.route_leaves_shell);
    assert!(!resolved.requires_review_before_send);
}

#[test]
fn header_local_model_route_reads_local_only() {
    let resolved = resolve_prompt_composer_header(&M5PromptComposerHeaderResolutionInput {
        route_class: M5ComposerRouteClass::LocalModel,
        ..ready_header()
    })
    .expect("resolves");
    assert_eq!(
        resolved.header_posture,
        M5ComposerHeaderPosture::LocalOnlyComposing
    );
    assert!(resolved.route_stays_local);
    assert!(!resolved.route_leaves_shell);
    assert!(resolved.is_sendable);
}

#[test]
fn header_posture_ladder_is_blocking_first() {
    // A blocked route wins even over a hard budget block.
    let blocked = resolve_prompt_composer_header(&M5PromptComposerHeaderResolutionInput {
        route_blocked: true,
        budget_posture: M5BudgetPosture::HardBlocked,
        ..ready_header()
    })
    .expect("resolves");
    assert_eq!(
        blocked.header_posture,
        M5ComposerHeaderPosture::RouteBlocked
    );
    assert!(!blocked.is_sendable);
    assert!(blocked.needs_attention);

    // A hard budget block blocks next.
    let budget_blocked = resolve_prompt_composer_header(&M5PromptComposerHeaderResolutionInput {
        budget_posture: M5BudgetPosture::HardBlocked,
        ..ready_header()
    })
    .expect("resolves");
    assert_eq!(
        budget_blocked.header_posture,
        M5ComposerHeaderPosture::BudgetBlocked
    );
    assert!(!budget_blocked.is_sendable);

    // Review-first mode requires review before send.
    let review = resolve_prompt_composer_header(&M5PromptComposerHeaderResolutionInput {
        composer_mode: M5ComposerMode::ReviewFirst,
        ..ready_header()
    })
    .expect("resolves");
    assert_eq!(
        review.header_posture,
        M5ComposerHeaderPosture::ReviewBeforeSend
    );
    assert!(review.requires_review_before_send);
    assert!(review.is_sendable);

    // Over budget (not hard) reads as budget-constrained.
    let constrained = resolve_prompt_composer_header(&M5PromptComposerHeaderResolutionInput {
        budget_posture: M5BudgetPosture::OverBudget,
        ..ready_header()
    })
    .expect("resolves");
    assert_eq!(
        constrained.header_posture,
        M5ComposerHeaderPosture::BudgetConstrained
    );
    assert!(constrained.needs_attention);
    assert!(constrained.is_sendable);
}

#[test]
fn header_rejects_malformed_input() {
    assert_eq!(
        resolve_prompt_composer_header(&M5PromptComposerHeaderResolutionInput {
            provider_model_label: "  ".to_owned(),
            ..ready_header()
        }),
        Err(M5PromptComposerHeaderResolutionError::EmptyProviderModelLabel)
    );
    assert_eq!(
        resolve_prompt_composer_header(&M5PromptComposerHeaderResolutionInput {
            provider_model_label: "https://leak.test/model".to_owned(),
            ..ready_header()
        }),
        Err(M5PromptComposerHeaderResolutionError::ForbiddenHeaderMaterial)
    );
}

// ---- pill resolver ------------------------------------------------------

#[test]
fn pill_fresh_trusted_is_openable_with_open_and_remove() {
    let resolved = resolve_context_attachment_pill(&fresh_pill()).expect("resolves");
    assert_eq!(resolved.pill_posture, M5AttachmentPillPosture::FreshTrusted);
    assert!(resolved.is_openable);
    assert!(!resolved.needs_review_before_send);
    assert!(!resolved.needs_attention);
    assert_eq!(
        resolved.available_actions,
        vec![M5AttachmentPillAction::Open, M5AttachmentPillAction::Remove]
    );
    assert_eq!(resolved.attachment_id, "attach.file.x");
}

#[test]
fn pill_posture_ladder_is_honesty_first() {
    // Tainted wins first, even when in-scope and fresh.
    let tainted = resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
        trust_state: M5AttachmentTrustState::TaintedExternal,
        ..fresh_pill()
    })
    .expect("resolves");
    assert_eq!(tainted.pill_posture, M5AttachmentPillPosture::Tainted);
    assert!(tainted.is_tainted);
    assert!(tainted.needs_review_before_send);
    assert!(tainted
        .available_actions
        .contains(&M5AttachmentPillAction::ReviewTrust));

    // Out-of-scope by trust.
    let oos = resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
        trust_state: M5AttachmentTrustState::OutOfScope,
        ..fresh_pill()
    })
    .expect("resolves");
    assert_eq!(oos.pill_posture, M5AttachmentPillPosture::OutOfScope);
    assert!(oos
        .available_actions
        .contains(&M5AttachmentPillAction::RevealScope));

    // Out-of-scope by !in_scope.
    let not_in_scope = resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
        in_scope: false,
        ..fresh_pill()
    })
    .expect("resolves");
    assert_eq!(
        not_in_scope.pill_posture,
        M5AttachmentPillPosture::OutOfScope
    );

    // Unverified.
    let unverified = resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
        trust_state: M5AttachmentTrustState::UnverifiedSource,
        ..fresh_pill()
    })
    .expect("resolves");
    assert_eq!(unverified.pill_posture, M5AttachmentPillPosture::Unverified);
    assert!(unverified.needs_review_before_send);

    // Stale by flag, with a refresh action.
    let stale = resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
        is_stale: true,
        staleness_reason: Some(M5StalenessReason::SourceEdited),
        ..fresh_pill()
    })
    .expect("resolves");
    assert_eq!(stale.pill_posture, M5AttachmentPillPosture::Stale);
    assert!(stale
        .available_actions
        .contains(&M5AttachmentPillAction::Refresh));

    // Redacted.
    let redacted = resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
        trust_state: M5AttachmentTrustState::RedactedScope,
        ..fresh_pill()
    })
    .expect("resolves");
    assert_eq!(redacted.pill_posture, M5AttachmentPillPosture::Redacted);
    assert!(redacted
        .available_actions
        .contains(&M5AttachmentPillAction::RevealScope));
}

#[test]
fn pill_removed_source_is_not_openable_but_still_removable() {
    let removed = resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
        trust_state: M5AttachmentTrustState::OutOfScope,
        is_stale: true,
        staleness_reason: Some(M5StalenessReason::SourceDeleted),
        source_removed: true,
        in_scope: false,
        ..fresh_pill()
    })
    .expect("resolves");
    assert!(!removed.is_openable);
    assert!(!removed
        .available_actions
        .contains(&M5AttachmentPillAction::Open));
    assert!(removed
        .available_actions
        .contains(&M5AttachmentPillAction::Remove));
}

#[test]
fn pill_rejects_malformed_input() {
    assert_eq!(
        resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
            attachment_id: " ".to_owned(),
            ..fresh_pill()
        }),
        Err(M5ContextAttachmentPillResolutionError::EmptyAttachmentId)
    );
    assert_eq!(
        resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
            attachment_label: "".to_owned(),
            ..fresh_pill()
        }),
        Err(M5ContextAttachmentPillResolutionError::EmptyAttachmentLabel)
    );
    assert_eq!(
        resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
            is_stale: true,
            staleness_reason: None,
            ..fresh_pill()
        }),
        Err(M5ContextAttachmentPillResolutionError::StaleAttachmentWithoutReason)
    );
    assert_eq!(
        resolve_context_attachment_pill(&M5ContextAttachmentPillResolutionInput {
            attachment_label: "s3://bucket/file".to_owned(),
            ..fresh_pill()
        }),
        Err(M5ContextAttachmentPillResolutionError::ForbiddenAttachmentMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_prompt_composer_header_pill_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_PROMPT_COMPOSER_HEADER_PILL_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_prompt_composer_header_pill_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5PromptComposerHeaderPillConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5PromptComposerHeaderPillConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_prompt_composer_header_pill_packet();
    for row in &packet.rows {
        for part in M5ComposerHeaderAnatomyPart::MANDATORY {
            assert!(row.header_anatomy_parts.contains(&part));
        }
        for part in M5AttachmentPillAnatomyPart::MANDATORY {
            assert!(row.pill_anatomy_parts.contains(&part));
        }
        for field in M5ComposerHeaderExportField::MANDATORY {
            assert!(row.header_export_fields.contains(&field));
        }
        for field in M5AttachmentPillExportField::MANDATORY {
            assert!(row.pill_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable));
        assert!(!row.header_examples.is_empty());
        assert!(!row.pill_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_prompt_composer_header_pill_packet();
    let headers: Vec<&M5PromptComposerHeaderResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.header_examples.iter())
        .collect();
    let pills: Vec<&M5ContextAttachmentPillResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.pill_examples.iter())
        .collect();

    for posture in M5ComposerHeaderPosture::ALL {
        assert!(
            headers.iter().any(|c| c.resolved.header_posture == posture),
            "no header example exercises posture {}",
            posture.as_str()
        );
    }
    for posture in M5AttachmentPillPosture::ALL {
        assert!(
            pills.iter().any(|c| c.resolved.pill_posture == posture),
            "no pill example exercises posture {}",
            posture.as_str()
        );
    }
    for action in M5AttachmentPillAction::ALL {
        assert!(
            pills
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no pill example exercises action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_prompt_composer_header_pill_packet();
    for row in &packet.rows {
        for case in &row.header_examples {
            assert!(
                case.is_self_consistent(),
                "header case for {} drifted",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.pill_examples {
            assert!(
                case.is_self_consistent(),
                "pill case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "pill case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.rows.retain(|row| {
        row.consumer_surface != M5PromptComposerHeaderPillConsumerSurface::PatchDraft
    });
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.vocabulary_set.header_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::VocabularySetDrift));
}

#[test]
fn mandatory_header_anatomy_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.rows[0]
        .header_anatomy_parts
        .retain(|p| *p != M5ComposerHeaderAnatomyPart::RouteProviderModelCue);
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::MandatoryHeaderAnatomyMissing));
}

#[test]
fn mandatory_pill_export_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.rows[0]
        .pill_export_fields
        .retain(|f| *f != M5AttachmentPillExportField::TrustState);
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::MandatoryPillExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.rows[0].pill_examples[0].resolved.is_openable = false;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::ExampleResolutionDrift));
}

#[test]
fn header_example_missing_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.rows[1].header_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::HeaderExampleMissing));
}

#[test]
fn header_sendability_coverage_unproven_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    // Replace every header example with a plainly sendable one so the non-sendable half
    // of the coverage lint fires.
    for row in &mut packet.rows {
        row.header_examples = vec![M5PromptComposerHeaderResolutionCase::resolved(
            ready_header(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::HeaderSendabilityCoverageUnproven));
}

#[test]
fn attachment_trust_coverage_unproven_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    // Replace every pill example with a fresh-trusted one so the needs-attention half of
    // the coverage lint fires.
    for row in &mut packet.rows {
        row.pill_examples = vec![M5ContextAttachmentPillResolutionCase::resolved(fresh_pill())];
    }
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::AttachmentTrustCoverageUnproven));
}

#[test]
fn attachment_open_remove_coverage_unproven_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    // Replace every pill example with an openable one so the removed-source half of the
    // coverage lint fires.
    for row in &mut packet.rows {
        row.pill_examples = vec![M5ContextAttachmentPillResolutionCase::resolved(fresh_pill())];
    }
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::AttachmentOpenRemoveCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.rows[0].hides_attachment_freshness_or_trust = true;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.governance_review.header_posture_never_masks_blocked = false;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.consumer_projection.pill_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_prompt_composer_header_pill_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5PromptComposerHeaderPillViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_prompt_composer_header_pill_packet().render_markdown_summary();
    for surface in M5PromptComposerHeaderPillConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_prompt_composer_header_pill_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5PromptComposerHeaderPillConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5PromptComposerHeaderPillConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_prompt_composer_header_pill_export()
        .expect("checked M5 header/pill primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PROMPT_COMPOSER_HEADER_PILL_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_prompt_composer_header_pill_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_prompt_composer_header_pill_patch_draft_preview_narrowed(),
        seeded_m5_prompt_composer_header_pill_handoff_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5PromptComposerHeaderPillConsumerSurface::ALL.len()
        );
    }

    let patch = seeded_m5_prompt_composer_header_pill_patch_draft_preview_narrowed();
    let row = patch
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5PromptComposerHeaderPillConsumerSurface::PatchDraft)
        .expect("patch-draft row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Preview);

    let handoff = seeded_m5_prompt_composer_header_pill_handoff_beta_narrowed();
    let row = handoff
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5PromptComposerHeaderPillConsumerSurface::HandoffSurface)
        .expect("handoff-surface row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let patch: M5PromptComposerHeaderPillPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces/patch_draft_preview_narrowed.json"
    )))
    .expect("patch-draft fixture parses");
    assert!(patch.validate().is_empty());
    assert_eq!(
        patch,
        seeded_m5_prompt_composer_header_pill_patch_draft_preview_narrowed()
    );

    let handoff: M5PromptComposerHeaderPillPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces/handoff_beta_narrowed.json"
    )))
    .expect("handoff fixture parses");
    assert!(handoff.validate().is_empty());
    assert_eq!(
        handoff,
        seeded_m5_prompt_composer_header_pill_handoff_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_prompt_composer_header_pill_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
