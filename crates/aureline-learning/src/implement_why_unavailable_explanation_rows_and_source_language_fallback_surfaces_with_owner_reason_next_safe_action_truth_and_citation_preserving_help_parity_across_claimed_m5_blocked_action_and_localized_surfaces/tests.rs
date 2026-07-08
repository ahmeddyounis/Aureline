use super::*;

fn policy_blocked_row() -> M5WhyUnavailableRowResolutionInput {
    M5WhyUnavailableRowResolutionInput {
        blocked_action_ref: "command:workspace.rotate-tokens".to_owned(),
        unavailable_reason: M5UnavailableReasonClass::PolicyBlocked,
        blocking_owner: M5BlockedActionOwner::PolicyOwner,
        next_safe_action: M5NextSafeActionClass::RequestAccess,
        next_safe_action_ref: Some("action:request-access.rotate-tokens".to_owned()),
        deeper_docs_ref: "docs:help/blocked-actions/policy".to_owned(),
        screen_reader_announcement: "Rotate tokens is blocked by policy. Request access."
            .to_owned(),
        row_identity_ref: "why:command-help:rotate-tokens-policy".to_owned(),
    }
}

fn source_language_fallback() -> M5SourceLanguageFallbackResolutionInput {
    M5SourceLanguageFallbackResolutionInput {
        source_language_class: M5SourceLanguageClass::FallbackToSource,
        fallback_state: M5FallbackStateClass::SourceLanguageShown,
        display_locale: "fr-FR".to_owned(),
        stable_id_ref: "stable:cmd.open-settings".to_owned(),
        canonical_citation_ref: "citation:docs/settings#open".to_owned(),
        source_language_text_ref: Some("text:source/en/open-settings".to_owned()),
        screen_reader_announcement: "Open settings help shows the English source.".to_owned(),
        row_identity_ref: "loc:command-help:open-settings-fallback".to_owned(),
    }
}

// ---- why-unavailable resolver -------------------------------------------

#[test]
fn policy_blocked_names_owner_reason_and_domain() {
    let resolved =
        resolve_why_unavailable_explanation_row(&policy_blocked_row()).expect("resolves");
    assert_eq!(
        resolved.help_posture,
        M5WhyUnavailablePosture::BlockedByPolicy
    );
    assert_eq!(resolved.failure_domain, M5UnavailableFailureDomain::Policy);
    assert!(resolved.has_next_safe_action);
    assert!(resolved.owner_is_contactable);
    assert!(!resolved.reason_is_transient);
    assert!(resolved.names_blocked_action);
    assert!(resolved.names_exact_reason);
    assert!(resolved.names_owning_boundary);
    assert!(resolved.names_next_safe_action_or_states_none);
    assert!(resolved.links_deeper_docs);
    assert!(resolved.never_generic_disabled);
    assert!(resolved.never_requires_pointer_hover);
    assert!(resolved.provides_screen_reader_announcement);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5WhyUnavailableAction::TakeNextSafeAction,
            M5WhyUnavailableAction::ContactBlockingOwner,
            M5WhyUnavailableAction::OpenDeeperDocs,
            M5WhyUnavailableAction::ExportUnavailableEvidence,
        ]
    );
}

#[test]
fn transient_offline_offers_retry() {
    let resolved = resolve_why_unavailable_explanation_row(&M5WhyUnavailableRowResolutionInput {
        unavailable_reason: M5UnavailableReasonClass::OfflineUnavailable,
        blocking_owner: M5BlockedActionOwner::ProviderService,
        next_safe_action: M5NextSafeActionClass::SwitchContext,
        next_safe_action_ref: Some("action:switch-context.work-offline".to_owned()),
        ..policy_blocked_row()
    })
    .expect("resolves");
    assert_eq!(
        resolved.help_posture,
        M5WhyUnavailablePosture::OfflineUnavailable
    );
    assert_eq!(resolved.failure_domain, M5UnavailableFailureDomain::Runtime);
    assert!(resolved.reason_is_transient);
    assert!(resolved
        .available_actions
        .contains(&M5WhyUnavailableAction::RetryWhenResolved));
}

#[test]
fn no_safe_action_row_states_none_honestly() {
    let resolved = resolve_why_unavailable_explanation_row(&M5WhyUnavailableRowResolutionInput {
        blocking_owner: M5BlockedActionOwner::UnknownOwner,
        next_safe_action: M5NextSafeActionClass::NoSafeAction,
        next_safe_action_ref: None,
        ..policy_blocked_row()
    })
    .expect("resolves");
    assert!(!resolved.has_next_safe_action);
    assert!(!resolved.owner_is_contactable);
    assert!(!resolved
        .available_actions
        .contains(&M5WhyUnavailableAction::TakeNextSafeAction));
    assert!(!resolved
        .available_actions
        .contains(&M5WhyUnavailableAction::ContactBlockingOwner));
    // Even with no safe action, docs and evidence export are always reachable.
    assert!(resolved
        .available_actions
        .contains(&M5WhyUnavailableAction::OpenDeeperDocs));
    assert!(resolved
        .available_actions
        .contains(&M5WhyUnavailableAction::ExportUnavailableEvidence));
    assert!(resolved.names_next_safe_action_or_states_none);
}

#[test]
fn posture_and_domain_map_one_to_one_from_reason() {
    for reason in M5UnavailableReasonClass::ALL {
        let posture = M5WhyUnavailablePosture::from_reason(reason);
        let expected = match reason {
            M5UnavailableReasonClass::PolicyBlocked => {
                ("blocked_by_policy", M5UnavailableFailureDomain::Policy)
            }
            M5UnavailableReasonClass::MissingPermission => {
                ("missing_permission", M5UnavailableFailureDomain::Trust)
            }
            M5UnavailableReasonClass::UnmetPrecondition => {
                ("precondition_unmet", M5UnavailableFailureDomain::Context)
            }
            M5UnavailableReasonClass::FeatureFlagOff => {
                ("feature_disabled", M5UnavailableFailureDomain::Context)
            }
            M5UnavailableReasonClass::OfflineUnavailable => {
                ("offline_unavailable", M5UnavailableFailureDomain::Runtime)
            }
            M5UnavailableReasonClass::UnsupportedTarget => {
                ("unsupported_target", M5UnavailableFailureDomain::Runtime)
            }
        };
        assert_eq!(posture.as_str(), expected.0);
        assert_eq!(posture.failure_domain(), expected.1);
    }
}

#[test]
fn actionable_row_without_next_action_ref_is_rejected() {
    assert_eq!(
        resolve_why_unavailable_explanation_row(&M5WhyUnavailableRowResolutionInput {
            next_safe_action_ref: None,
            ..policy_blocked_row()
        }),
        Err(M5WhyUnavailableRowResolutionError::MissingNextActionRefForActionableRow)
    );
    assert_eq!(
        resolve_why_unavailable_explanation_row(&M5WhyUnavailableRowResolutionInput {
            next_safe_action_ref: Some("   ".to_owned()),
            ..policy_blocked_row()
        }),
        Err(M5WhyUnavailableRowResolutionError::MissingNextActionRefForActionableRow)
    );
}

#[test]
fn no_safe_action_row_with_next_action_ref_is_rejected() {
    assert_eq!(
        resolve_why_unavailable_explanation_row(&M5WhyUnavailableRowResolutionInput {
            next_safe_action: M5NextSafeActionClass::NoSafeAction,
            next_safe_action_ref: Some("action:something".to_owned()),
            ..policy_blocked_row()
        }),
        Err(M5WhyUnavailableRowResolutionError::NextActionRefOnNoSafeAction)
    );
}

#[test]
fn why_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_why_unavailable_explanation_row(&M5WhyUnavailableRowResolutionInput {
            blocked_action_ref: " ".to_owned(),
            ..policy_blocked_row()
        }),
        Err(M5WhyUnavailableRowResolutionError::EmptyBlockedActionRef)
    );
    assert_eq!(
        resolve_why_unavailable_explanation_row(&M5WhyUnavailableRowResolutionInput {
            deeper_docs_ref: "".to_owned(),
            ..policy_blocked_row()
        }),
        Err(M5WhyUnavailableRowResolutionError::EmptyDeeperDocsRef)
    );
    assert_eq!(
        resolve_why_unavailable_explanation_row(&M5WhyUnavailableRowResolutionInput {
            screen_reader_announcement: "".to_owned(),
            ..policy_blocked_row()
        }),
        Err(M5WhyUnavailableRowResolutionError::EmptyScreenReaderAnnouncement)
    );
    assert_eq!(
        resolve_why_unavailable_explanation_row(&M5WhyUnavailableRowResolutionInput {
            row_identity_ref: "".to_owned(),
            ..policy_blocked_row()
        }),
        Err(M5WhyUnavailableRowResolutionError::EmptyRowIdentity)
    );
    assert_eq!(
        resolve_why_unavailable_explanation_row(&M5WhyUnavailableRowResolutionInput {
            deeper_docs_ref: "https://evil.example/x".to_owned(),
            ..policy_blocked_row()
        }),
        Err(M5WhyUnavailableRowResolutionError::ForbiddenUnavailableMaterial)
    );
}

// ---- source-language resolver -------------------------------------------

#[test]
fn source_language_fallback_preserves_source_and_citation() {
    let resolved = resolve_source_language_fallback(&source_language_fallback()).expect("resolves");
    assert_eq!(
        resolved.help_posture,
        M5SourceLanguagePosture::ShowingSourceLanguage
    );
    assert!(resolved.shows_source_language);
    assert!(!resolved.is_fully_localized);
    assert!(resolved.requires_source_text);
    assert!(resolved.preserves_source_language_text);
    assert!(resolved.preserves_stable_id);
    assert!(resolved.preserves_canonical_citation);
    assert!(resolved.aligned_with_canonical_ids);
    assert!(resolved.never_unsourced_paraphrase);
    assert!(resolved
        .available_actions
        .contains(&M5SourceLanguageAction::ViewSourceLanguageText));
    assert!(resolved
        .available_actions
        .contains(&M5SourceLanguageAction::OpenCanonicalCitation));
    assert!(resolved
        .available_actions
        .contains(&M5SourceLanguageAction::ExportLocaleEvidence));
}

#[test]
fn fully_localized_allows_absent_source_text() {
    let resolved = resolve_source_language_fallback(&M5SourceLanguageFallbackResolutionInput {
        source_language_class: M5SourceLanguageClass::AuthoredLocale,
        fallback_state: M5FallbackStateClass::LocalizedCurrent,
        source_language_text_ref: None,
        ..source_language_fallback()
    })
    .expect("resolves");
    assert_eq!(
        resolved.help_posture,
        M5SourceLanguagePosture::FullyLocalized
    );
    assert!(resolved.is_fully_localized);
    assert!(!resolved.requires_source_text);
    assert!(!resolved.shows_source_language);
    // A fully localized surface still keeps its canonical citation reachable.
    assert!(resolved
        .available_actions
        .contains(&M5SourceLanguageAction::OpenCanonicalCitation));
}

#[test]
fn no_localization_offers_request_localization() {
    let resolved = resolve_source_language_fallback(&M5SourceLanguageFallbackResolutionInput {
        source_language_class: M5SourceLanguageClass::UntranslatedSource,
        fallback_state: M5FallbackStateClass::NoLocalization,
        source_language_text_ref: Some("text:source/en/lint-warning".to_owned()),
        ..source_language_fallback()
    })
    .expect("resolves");
    assert_eq!(
        resolved.help_posture,
        M5SourceLanguagePosture::NoLocalization
    );
    assert!(resolved.is_incomplete_or_stale);
    assert!(resolved
        .available_actions
        .contains(&M5SourceLanguageAction::RequestLocalization));
    assert!(resolved
        .available_actions
        .contains(&M5SourceLanguageAction::ReportTranslationGap));
}

#[test]
fn fallback_without_source_text_is_rejected() {
    for state in [
        M5FallbackStateClass::SourceLanguageShown,
        M5FallbackStateClass::PartialTranslation,
        M5FallbackStateClass::StaleTranslation,
        M5FallbackStateClass::CitationPreservedFallback,
        M5FallbackStateClass::NoLocalization,
    ] {
        assert_eq!(
            resolve_source_language_fallback(&M5SourceLanguageFallbackResolutionInput {
                fallback_state: state,
                source_language_text_ref: None,
                ..source_language_fallback()
            }),
            Err(M5SourceLanguageFallbackResolutionError::MissingSourceTextForFallback),
            "{} without source text should be rejected",
            M5SourceLanguagePosture::from_fallback_state(state).as_str()
        );
    }
}

#[test]
fn source_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_source_language_fallback(&M5SourceLanguageFallbackResolutionInput {
            display_locale: " ".to_owned(),
            ..source_language_fallback()
        }),
        Err(M5SourceLanguageFallbackResolutionError::EmptyDisplayLocale)
    );
    assert_eq!(
        resolve_source_language_fallback(&M5SourceLanguageFallbackResolutionInput {
            stable_id_ref: "".to_owned(),
            ..source_language_fallback()
        }),
        Err(M5SourceLanguageFallbackResolutionError::EmptyStableIdRef)
    );
    assert_eq!(
        resolve_source_language_fallback(&M5SourceLanguageFallbackResolutionInput {
            canonical_citation_ref: "".to_owned(),
            ..source_language_fallback()
        }),
        Err(M5SourceLanguageFallbackResolutionError::EmptyCanonicalCitationRef)
    );
    assert_eq!(
        resolve_source_language_fallback(&M5SourceLanguageFallbackResolutionInput {
            row_identity_ref: "".to_owned(),
            ..source_language_fallback()
        }),
        Err(M5SourceLanguageFallbackResolutionError::EmptyRowIdentity)
    );
    assert_eq!(
        resolve_source_language_fallback(&M5SourceLanguageFallbackResolutionInput {
            canonical_citation_ref: "https://evil.example/x".to_owned(),
            ..source_language_fallback()
        }),
        Err(M5SourceLanguageFallbackResolutionError::ForbiddenLocaleMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_blocked_localized_row_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BLOCKED_LOCALIZED_ROW_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_blocked_localized_row_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5BlockedLocalizedConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5BlockedLocalizedConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_blocked_localized_row_packet();
    for row in &packet.rows {
        for part in M5WhyUnavailableAnatomyPart::MANDATORY {
            assert!(row.why_unavailable_anatomy_parts.contains(&part));
        }
        for part in M5SourceLanguageAnatomyPart::MANDATORY {
            assert!(row.source_language_anatomy_parts.contains(&part));
        }
        for field in M5WhyUnavailableExportField::MANDATORY {
            assert!(row.why_unavailable_export_fields.contains(&field));
        }
        for field in M5SourceLanguageExportField::MANDATORY {
            assert!(row.source_language_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable));
        assert!(!row.why_unavailable_examples.is_empty());
        assert!(!row.source_language_examples.is_empty());
    }
}

#[test]
fn every_frozen_class_is_exercised_by_some_example() {
    let packet = seeded_m5_blocked_localized_row_packet();
    let why: Vec<&M5WhyUnavailableRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.why_unavailable_examples.iter())
        .collect();
    let source: Vec<&M5SourceLanguageFallbackResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.source_language_examples.iter())
        .collect();

    for reason in M5UnavailableReasonClass::ALL {
        assert!(
            why.iter().any(|c| c.resolved.unavailable_reason == reason),
            "no example exercises reason {}",
            reason.as_str()
        );
    }
    for domain in M5UnavailableFailureDomain::ALL {
        assert!(
            why.iter().any(|c| c.resolved.failure_domain == domain),
            "no example exercises failure domain {}",
            domain.as_str()
        );
    }
    for class in M5NextSafeActionClass::ALL {
        assert!(
            why.iter().any(|c| c.resolved.next_safe_action == class),
            "no example exercises next-safe-action {}",
            class.as_str()
        );
    }
    for action in M5WhyUnavailableAction::ALL {
        assert!(
            why.iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises why-unavailable action {}",
            action.as_str()
        );
    }
    for class in M5SourceLanguageClass::ALL {
        assert!(
            source
                .iter()
                .any(|c| c.resolved.source_language_class == class),
            "no example exercises source-language class {}",
            class.as_str()
        );
    }
    for state in M5FallbackStateClass::ALL {
        assert!(
            source.iter().any(|c| c.resolved.fallback_state == state),
            "no example exercises fallback state {}",
            state.as_str()
        );
    }
    for action in M5SourceLanguageAction::ALL {
        assert!(
            source
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises source-language action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_parity() {
    let packet = seeded_m5_blocked_localized_row_packet();
    for row in &packet.rows {
        for case in &row.why_unavailable_examples {
            assert!(case.is_self_consistent(), "why case drifted");
            assert!(case.preserves_identity(), "why case lost identity");
            assert!(
                case.preserves_explanation_parity(),
                "why case lost explanation parity"
            );
        }
        for case in &row.source_language_examples {
            assert!(case.is_self_consistent(), "source case drifted");
            assert!(case.preserves_identity(), "source case lost identity");
            assert!(case.preserves_citation(), "source case lost citation");
            assert!(
                case.preserves_localized_parity(),
                "source case lost localized parity"
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5BlockedLocalizedConsumerSurface::InlineStatusRow);
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.vocabulary_set.why_unavailable_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::VocabularySetDrift));
}

#[test]
fn mandatory_why_anatomy_missing_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.rows[0]
        .why_unavailable_anatomy_parts
        .retain(|p| *p != M5WhyUnavailableAnatomyPart::BlockingOwnerCue);
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::WhyUnavailableAnatomyMissing));
}

#[test]
fn mandatory_source_export_missing_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.rows[0]
        .source_language_export_fields
        .retain(|f| *f != M5SourceLanguageExportField::CanonicalCitationRef);
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::SourceLanguageExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.rows[0].why_unavailable_examples[0]
        .resolved
        .failure_domain = M5UnavailableFailureDomain::Runtime;
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::ExampleResolutionDrift));
}

#[test]
fn why_example_missing_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.rows[1].why_unavailable_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::WhyUnavailableExampleMissing));
}

#[test]
fn source_example_missing_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.rows[1].source_language_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::SourceLanguageExampleMissing));
}

#[test]
fn reason_coverage_unproven_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    for row in &mut packet.rows {
        row.why_unavailable_examples = vec![M5WhyUnavailableRowResolutionCase::resolved(
            policy_blocked_row(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::ReasonCoverageUnproven));
}

#[test]
fn failure_domain_coverage_unproven_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    // Every example blocked-by-policy → only the policy domain is exercised.
    for row in &mut packet.rows {
        row.why_unavailable_examples = vec![M5WhyUnavailableRowResolutionCase::resolved(
            policy_blocked_row(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::FailureDomainCoverageUnproven));
}

#[test]
fn next_safe_action_coverage_unproven_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    for row in &mut packet.rows {
        row.why_unavailable_examples = vec![M5WhyUnavailableRowResolutionCase::resolved(
            policy_blocked_row(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::NextSafeActionCoverageUnproven));
}

#[test]
fn source_language_class_coverage_unproven_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    for row in &mut packet.rows {
        row.source_language_examples = vec![M5SourceLanguageFallbackResolutionCase::resolved(
            source_language_fallback(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::SourceLanguageClassCoverageUnproven));
}

#[test]
fn citation_preservation_unproven_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.rows[0].source_language_examples[0]
        .resolved
        .aligned_with_canonical_ids = false;
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::CitationPreservationUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.rows[0].collapses_into_generic_disabled_state = true;
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet
        .governance_review
        .blocked_actions_never_collapse_into_generic_disabled = false;
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet
        .consumer_projection
        .source_language_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_blocked_localized_row_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5BlockedLocalizedRowViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_blocked_localized_row_packet().render_markdown_summary();
    for surface in M5BlockedLocalizedConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_blocked_localized_row_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5BlockedLocalizedConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5BlockedLocalizedConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_blocked_localized_export()
        .expect("checked M5 blocked localized primitive export validates");
    assert_eq!(from_disk.packet_id, M5_BLOCKED_LOCALIZED_ROW_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_blocked_localized_row_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_blocked_localized_menu_and_action_row_beta_narrowed(),
        seeded_m5_blocked_localized_support_explanation_export_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5BlockedLocalizedConsumerSurface::ALL.len()
        );
    }

    let menu = seeded_m5_blocked_localized_menu_and_action_row_beta_narrowed();
    let row = menu
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5BlockedLocalizedConsumerSurface::MenuAndActionRow)
        .expect("menu-and-action-row row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Beta);

    let support = seeded_m5_blocked_localized_support_explanation_export_preview_narrowed();
    let row = support
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5BlockedLocalizedConsumerSurface::SupportExplanationExport)
        .expect("support-explanation-export row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let menu: M5BlockedLocalizedRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-why-unavailable-source-language-primitive/menu_and_action_row_beta_narrowed.json"
    )))
    .expect("menu-and-action-row fixture parses");
    assert!(menu.validate().is_empty());
    assert_eq!(
        menu,
        seeded_m5_blocked_localized_menu_and_action_row_beta_narrowed()
    );

    let support: M5BlockedLocalizedRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-why-unavailable-source-language-primitive/support_explanation_export_preview_narrowed.json"
    )))
    .expect("support-explanation-export fixture parses");
    assert!(support.validate().is_empty());
    assert_eq!(
        support,
        seeded_m5_blocked_localized_support_explanation_export_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_blocked_localized_row_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
