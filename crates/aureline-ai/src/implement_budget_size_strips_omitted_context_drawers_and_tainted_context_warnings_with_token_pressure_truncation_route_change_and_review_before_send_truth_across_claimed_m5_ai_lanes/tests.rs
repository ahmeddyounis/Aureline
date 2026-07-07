use super::*;

fn within_budget() -> M5BudgetSizeStripResolutionInput {
    M5BudgetSizeStripResolutionInput {
        strip_id: "strip.test.within".to_owned(),
        strip_label: "Test budget".to_owned(),
        included_context_classes: M5ContextClass::ALL.to_vec(),
        omitted_entries: vec![],
        unmetered_local: false,
        hard_ceiling_hit: false,
        over_budget: false,
        truncation_pending: false,
        near_limit: false,
        route_before: Some(M5ComposerRouteClass::ManagedRoute),
        route_after: M5ComposerRouteClass::ManagedRoute,
    }
}

fn injection_warning() -> M5TaintedContextWarningResolutionInput {
    M5TaintedContextWarningResolutionInput {
        warning_id: "warn.test.injection".to_owned(),
        context_label: "pasted external block".to_owned(),
        taint_source: M5TaintSource::PastedExternalText,
        taint_severity: M5TaintSeverity::InjectionSuspected,
        treated_as_data: true,
        side_effecting_route: true,
        acknowledged: false,
        quarantine_note: Some("held for review".to_owned()),
    }
}

// ---- budget-or-size strip -----------------------------------------------

#[test]
fn budget_within_reads_nominal_and_sends() {
    let resolved = resolve_budget_size_strip(&within_budget()).expect("resolves");
    assert_eq!(resolved.budget_posture, M5BudgetPosture::WithinBudget);
    assert_eq!(resolved.pressure_band, M5BudgetPressureBand::Nominal);
    assert_eq!(resolved.route_switch, M5RouteSwitchConsequence::Unchanged);
    assert!(!resolved.has_omitted_context);
    assert!(resolved.is_sendable);
    assert!(!resolved.requires_review_before_send);
    assert_eq!(
        resolved.available_actions,
        vec![M5BudgetStripAction::ProceedToSend]
    );
    assert_eq!(resolved.strip_id, "strip.test.within");
}

#[test]
fn budget_posture_ladder_is_blocking_first() {
    let hard = resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
        hard_ceiling_hit: true,
        over_budget: true,
        truncation_pending: true,
        near_limit: true,
        ..within_budget()
    })
    .expect("resolves");
    assert_eq!(hard.budget_posture, M5BudgetPosture::HardBlocked);
    assert_eq!(hard.pressure_band, M5BudgetPressureBand::Exhausted);
    assert!(!hard.is_sendable);
    assert!(hard
        .available_actions
        .contains(&M5BudgetStripAction::AdjustBudgetOrScope));
    assert!(!hard
        .available_actions
        .contains(&M5BudgetStripAction::ProceedToSend));

    let over = resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
        over_budget: true,
        truncation_pending: true,
        near_limit: true,
        ..within_budget()
    })
    .expect("resolves");
    assert_eq!(over.budget_posture, M5BudgetPosture::OverBudget);
    assert_eq!(over.pressure_band, M5BudgetPressureBand::Critical);
    assert!(over.requires_review_before_send);

    let trunc = resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
        truncation_pending: true,
        near_limit: true,
        ..within_budget()
    })
    .expect("resolves");
    assert_eq!(trunc.budget_posture, M5BudgetPosture::TruncationPending);
    assert!(trunc.truncation_active);

    let near = resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
        near_limit: true,
        ..within_budget()
    })
    .expect("resolves");
    assert_eq!(near.budget_posture, M5BudgetPosture::NearLimit);
    assert_eq!(near.pressure_band, M5BudgetPressureBand::Watch);

    let local = resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
        unmetered_local: true,
        ..within_budget()
    })
    .expect("resolves");
    assert_eq!(local.budget_posture, M5BudgetPosture::UnmeteredLocal);
    assert_eq!(local.pressure_band, M5BudgetPressureBand::Unmetered);
}

#[test]
fn budget_route_switch_is_derived() {
    let unchanged = resolve_budget_size_strip(&within_budget()).expect("resolves");
    assert_eq!(unchanged.route_switch, M5RouteSwitchConsequence::Unchanged);
    assert!(!unchanged.route_changed);

    let locality = resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
        route_before: Some(M5ComposerRouteClass::LocalModel),
        route_after: M5ComposerRouteClass::ManagedRoute,
        ..within_budget()
    })
    .expect("resolves");
    assert_eq!(
        locality.route_switch,
        M5RouteSwitchConsequence::LocalityChanged
    );
    assert!(locality
        .available_actions
        .contains(&M5BudgetStripAction::ReviewRouteChange));

    let widened = resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
        route_before: Some(M5ComposerRouteClass::SelfHostedRoute),
        route_after: M5ComposerRouteClass::ByokDirect,
        ..within_budget()
    })
    .expect("resolves");
    assert_eq!(widened.route_switch, M5RouteSwitchConsequence::ReachWidened);

    let narrowed = resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
        route_before: Some(M5ComposerRouteClass::ByokDirect),
        route_after: M5ComposerRouteClass::SelfHostedRoute,
        ..within_budget()
    })
    .expect("resolves");
    assert_eq!(
        narrowed.route_switch,
        M5RouteSwitchConsequence::ReachNarrowed
    );

    let provider = resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
        route_before: Some(M5ComposerRouteClass::ByokDirect),
        route_after: M5ComposerRouteClass::ManagedRoute,
        ..within_budget()
    })
    .expect("resolves");
    assert_eq!(
        provider.route_switch,
        M5RouteSwitchConsequence::ProviderChanged
    );
}

#[test]
fn budget_omission_offers_inspect_path_before_send() {
    let resolved = resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
        near_limit: true,
        omitted_entries: vec![M5OmittedContextEntry {
            context_class: M5ContextClass::RetrievedSnippets,
            reason: M5OmittedContextReason::SizeTruncated,
            detail: "trimmed the lowest-ranked snippets".to_owned(),
        }],
        ..within_budget()
    })
    .expect("resolves");
    assert!(resolved.has_omitted_context);
    assert!(resolved.truncation_active);
    assert!(resolved.requires_review_before_send);
    assert!(resolved.discloses_every_omission);
    assert!(resolved
        .available_actions
        .contains(&M5BudgetStripAction::InspectOmittedContext));
}

#[test]
fn budget_rejects_malformed_input() {
    assert_eq!(
        resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
            strip_id: "  ".to_owned(),
            ..within_budget()
        }),
        Err(M5BudgetSizeStripResolutionError::EmptyStripId)
    );
    assert_eq!(
        resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
            strip_label: "".to_owned(),
            ..within_budget()
        }),
        Err(M5BudgetSizeStripResolutionError::EmptyStripLabel)
    );
    assert_eq!(
        resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
            omitted_entries: vec![M5OmittedContextEntry {
                context_class: M5ContextClass::ToolOutput,
                reason: M5OmittedContextReason::NoneOmitted,
                detail: "nothing".to_owned(),
            }],
            ..within_budget()
        }),
        Err(M5BudgetSizeStripResolutionError::OmittedEntryWithoutReason)
    );
    assert_eq!(
        resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
            omitted_entries: vec![M5OmittedContextEntry {
                context_class: M5ContextClass::ToolOutput,
                reason: M5OmittedContextReason::SizeTruncated,
                detail: "  ".to_owned(),
            }],
            ..within_budget()
        }),
        Err(M5BudgetSizeStripResolutionError::OmittedEntryWithoutDetail)
    );
    assert_eq!(
        resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput {
            strip_label: "budget https://leak.test".to_owned(),
            ..within_budget()
        }),
        Err(M5BudgetSizeStripResolutionError::ForbiddenBudgetMaterial)
    );
}

// ---- tainted-context warning --------------------------------------------

#[test]
fn taint_injection_blocks_side_effecting_send_and_preserves_review() {
    let resolved = resolve_tainted_context_warning(&injection_warning()).expect("resolves");
    assert_eq!(
        resolved.warning_posture,
        M5TaintWarningPosture::InjectionBlocked
    );
    assert!(resolved.blocks_send);
    assert!(resolved.requires_review_before_send);
    assert!(resolved.preserves_review_path);
    assert!(resolved.treats_untrusted_as_data);
    assert!(resolved
        .available_actions
        .contains(&M5TaintWarningAction::ReviewTaintedContent));
    assert!(resolved
        .available_actions
        .contains(&M5TaintWarningAction::QuarantineContent));
    assert!(!resolved
        .available_actions
        .contains(&M5TaintWarningAction::ProceedWithSend));
    assert_eq!(resolved.warning_id, "warn.test.injection");
}

#[test]
fn taint_posture_ladder_is_severity_first() {
    let none = resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
        taint_severity: M5TaintSeverity::None,
        treated_as_data: false,
        side_effecting_route: false,
        quarantine_note: None,
        ..injection_warning()
    })
    .expect("resolves");
    assert_eq!(none.warning_posture, M5TaintWarningPosture::NoTaintTrusted);
    assert!(!none.blocks_send);
    assert!(none
        .available_actions
        .contains(&M5TaintWarningAction::ProceedWithSend));

    let quarantine = resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
        taint_severity: M5TaintSeverity::QuarantineRequired,
        ..injection_warning()
    })
    .expect("resolves");
    assert_eq!(
        quarantine.warning_posture,
        M5TaintWarningPosture::QuarantineHeld
    );
    assert!(quarantine.blocks_send);

    let elevated = resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
        taint_severity: M5TaintSeverity::Elevated,
        side_effecting_route: true,
        quarantine_note: None,
        ..injection_warning()
    })
    .expect("resolves");
    assert_eq!(
        elevated.warning_posture,
        M5TaintWarningPosture::ElevatedReviewRequired
    );
    assert!(elevated.blocks_send, "elevated + side-effecting must block");
    assert!(elevated
        .available_actions
        .contains(&M5TaintWarningAction::AcknowledgeAsData));

    let acknowledged = resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
        taint_severity: M5TaintSeverity::Elevated,
        side_effecting_route: true,
        acknowledged: true,
        quarantine_note: None,
        ..injection_warning()
    })
    .expect("resolves");
    assert_eq!(
        acknowledged.warning_posture,
        M5TaintWarningPosture::AcknowledgedProceedable
    );
    assert!(!acknowledged.blocks_send);

    let flagged = resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
        taint_severity: M5TaintSeverity::Informational,
        side_effecting_route: false,
        quarantine_note: None,
        ..injection_warning()
    })
    .expect("resolves");
    assert_eq!(
        flagged.warning_posture,
        M5TaintWarningPosture::FlaggedAsData
    );
}

#[test]
fn taint_always_offers_review_when_tainted() {
    for severity in [
        M5TaintSeverity::Informational,
        M5TaintSeverity::Elevated,
        M5TaintSeverity::QuarantineRequired,
        M5TaintSeverity::InjectionSuspected,
    ] {
        let resolved = resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
            taint_severity: severity,
            treated_as_data: true,
            quarantine_note: Some("held".to_owned()),
            ..injection_warning()
        })
        .expect("resolves");
        assert!(
            resolved
                .available_actions
                .contains(&M5TaintWarningAction::ReviewTaintedContent),
            "no review action for severity {}",
            severity.as_str()
        );
        assert!(resolved.preserves_review_path);
    }
}

#[test]
fn taint_rejects_malformed_input() {
    assert_eq!(
        resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
            warning_id: " ".to_owned(),
            ..injection_warning()
        }),
        Err(M5TaintedContextWarningResolutionError::EmptyWarningId)
    );
    assert_eq!(
        resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
            context_label: "".to_owned(),
            ..injection_warning()
        }),
        Err(M5TaintedContextWarningResolutionError::EmptyContextLabel)
    );
    assert_eq!(
        resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
            taint_severity: M5TaintSeverity::Elevated,
            treated_as_data: false,
            quarantine_note: None,
            ..injection_warning()
        }),
        Err(M5TaintedContextWarningResolutionError::TaintNotTreatedAsData)
    );
    assert_eq!(
        resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
            taint_severity: M5TaintSeverity::QuarantineRequired,
            quarantine_note: None,
            ..injection_warning()
        }),
        Err(M5TaintedContextWarningResolutionError::QuarantineWithoutNote)
    );
    assert_eq!(
        resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput {
            context_label: "see https://leak.test".to_owned(),
            ..injection_warning()
        }),
        Err(M5TaintedContextWarningResolutionError::ForbiddenTaintMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_budget_taint_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BUDGET_TAINT_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_budget_taint_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5BudgetTaintConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5BudgetTaintConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_budget_taint_packet();
    for row in &packet.rows {
        for part in M5BudgetStripAnatomyPart::MANDATORY {
            assert!(row.budget_anatomy_parts.contains(&part));
        }
        for part in M5TaintWarningAnatomyPart::MANDATORY {
            assert!(row.taint_anatomy_parts.contains(&part));
        }
        for field in M5BudgetStripExportField::MANDATORY {
            assert!(row.budget_export_fields.contains(&field));
        }
        for field in M5TaintWarningExportField::MANDATORY {
            assert!(row.taint_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable));
        assert!(!row.budget_examples.is_empty());
        assert!(!row.taint_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_budget_taint_packet();
    let budgets: Vec<&M5BudgetSizeStripResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.budget_examples.iter())
        .collect();
    let taints: Vec<&M5TaintedContextWarningResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.taint_examples.iter())
        .collect();

    for posture in M5BudgetPosture::ALL {
        assert!(
            budgets.iter().any(|c| c.resolved.budget_posture == posture),
            "no budget example exercises posture {}",
            posture.as_str()
        );
    }
    for band in M5BudgetPressureBand::ALL {
        assert!(
            budgets.iter().any(|c| c.resolved.pressure_band == band),
            "no budget example exercises band {}",
            band.as_str()
        );
    }
    for switch in M5RouteSwitchConsequence::ALL {
        assert!(
            budgets.iter().any(|c| c.resolved.route_switch == switch),
            "no budget example exercises route switch {}",
            switch.as_str()
        );
    }
    for action in M5BudgetStripAction::ALL {
        assert!(
            budgets
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no budget example exercises action {}",
            action.as_str()
        );
    }
    for reason in M5OmittedContextReason::ALL {
        let seen = reason == M5OmittedContextReason::NoneOmitted
            || budgets.iter().any(|c| {
                c.resolved
                    .omitted_entries
                    .iter()
                    .any(|e| e.reason == reason)
            });
        assert!(
            seen,
            "no budget example exercises reason {}",
            reason.as_str()
        );
    }
    for class in M5ContextClass::ALL {
        assert!(
            budgets.iter().any(|c| {
                c.resolved
                    .omitted_entries
                    .iter()
                    .any(|e| e.context_class == class)
            }),
            "no omitted entry exercises context class {}",
            class.as_str()
        );
    }

    for source in M5TaintSource::ALL {
        assert!(
            taints.iter().any(|c| c.resolved.taint_source == source),
            "no taint example exercises source {}",
            source.as_str()
        );
    }
    for severity in M5TaintSeverity::ALL {
        assert!(
            taints.iter().any(|c| c.resolved.taint_severity == severity),
            "no taint example exercises severity {}",
            severity.as_str()
        );
    }
    for posture in M5TaintWarningPosture::ALL {
        assert!(
            taints.iter().any(|c| c.resolved.warning_posture == posture),
            "no taint example exercises posture {}",
            posture.as_str()
        );
    }
    for action in M5TaintWarningAction::ALL {
        assert!(
            taints
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no taint example exercises action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_budget_taint_packet();
    for row in &packet.rows {
        for case in &row.budget_examples {
            assert!(
                case.is_self_consistent(),
                "budget case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "budget case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.taint_examples {
            assert!(
                case.is_self_consistent(),
                "taint case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "taint case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5BudgetTaintConsumerSurface::SidePanel);
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.vocabulary_set.taint_warning_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::VocabularySetDrift));
}

#[test]
fn mandatory_budget_anatomy_missing_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.rows[0]
        .budget_anatomy_parts
        .retain(|p| *p != M5BudgetStripAnatomyPart::OmittedContextDrawerCue);
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::MandatoryBudgetAnatomyMissing));
}

#[test]
fn mandatory_taint_export_missing_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.rows[0]
        .taint_export_fields
        .retain(|f| *f != M5TaintWarningExportField::TaintSeverity);
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::MandatoryTaintExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.rows[0].budget_examples[0].resolved.is_sendable = false;
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::ExampleResolutionDrift));
}

#[test]
fn budget_example_missing_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.rows[1].budget_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::BudgetExampleMissing));
}

#[test]
fn budget_omission_disclosure_unproven_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    // Replace every budget example with a plainly within-budget one so no omission disclosure
    // survives.
    for row in &mut packet.rows {
        row.budget_examples = vec![M5BudgetSizeStripResolutionCase::resolved(within_budget())];
    }
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::BudgetOmissionDisclosureUnproven));
}

#[test]
fn budget_route_change_coverage_unproven_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    // Every budget example keeps an unchanged route, so the route-change proof disappears.
    for row in &mut packet.rows {
        row.budget_examples = vec![M5BudgetSizeStripResolutionCase::resolved(
            M5BudgetSizeStripResolutionInput {
                near_limit: true,
                omitted_entries: vec![M5OmittedContextEntry {
                    context_class: M5ContextClass::RetrievedSnippets,
                    reason: M5OmittedContextReason::SizeTruncated,
                    detail: "trimmed snippets".to_owned(),
                }],
                ..within_budget()
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::BudgetRouteChangeCoverageUnproven));
}

#[test]
fn taint_input_class_coverage_unproven_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    // Replace every taint example with a fetched-url one so the required classes disappear.
    for row in &mut packet.rows {
        row.taint_examples = vec![M5TaintedContextWarningResolutionCase::resolved(
            M5TaintedContextWarningResolutionInput {
                taint_source: M5TaintSource::FetchedUrlContent,
                ..injection_warning()
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::TaintInputClassCoverageUnproven));
}

#[test]
fn taint_review_path_unproven_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    // No side-effecting blocking warning survives: keep only non-side-effecting informational
    // warnings across the required input classes.
    for (i, row) in packet.rows.iter_mut().enumerate() {
        let source = match i % 3 {
            0 => M5TaintSource::PastedExternalText,
            1 => M5TaintSource::ToolOutput,
            _ => M5TaintSource::PriorModelOutput,
        };
        row.taint_examples = vec![M5TaintedContextWarningResolutionCase::resolved(
            M5TaintedContextWarningResolutionInput {
                taint_source: source,
                taint_severity: M5TaintSeverity::Informational,
                side_effecting_route: false,
                quarantine_note: None,
                ..injection_warning()
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::TaintReviewPathUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.rows[0].downplays_taint_source_or_severity = true;
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.governance_review.untrusted_content_treated_as_data = false;
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.consumer_projection.taint_state_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_budget_taint_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5BudgetTaintViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_budget_taint_packet().render_markdown_summary();
    for surface in M5BudgetTaintConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_budget_taint_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5BudgetTaintConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5BudgetTaintConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_budget_taint_export()
        .expect("checked M5 budget/taint primitive export validates");
    assert_eq!(from_disk.packet_id, M5_BUDGET_TAINT_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_budget_taint_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_budget_taint_patch_draft_preview_narrowed(),
        seeded_m5_budget_taint_cli_headless_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5BudgetTaintConsumerSurface::ALL.len());
    }

    let patch = seeded_m5_budget_taint_patch_draft_preview_narrowed();
    let row = patch
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5BudgetTaintConsumerSurface::PatchDraft)
        .expect("patch-draft row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Preview);

    let cli = seeded_m5_budget_taint_cli_headless_beta_narrowed();
    let row = cli
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5BudgetTaintConsumerSurface::CliHeadless)
        .expect("cli-headless row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let patch: M5BudgetTaintPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes/patch_draft_preview_narrowed.json"
    )))
    .expect("patch-draft fixture parses");
    assert!(patch.validate().is_empty());
    assert_eq!(patch, seeded_m5_budget_taint_patch_draft_preview_narrowed());

    let cli: M5BudgetTaintPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes/cli_headless_beta_narrowed.json"
    )))
    .expect("cli-headless fixture parses");
    assert!(cli.validate().is_empty());
    assert_eq!(cli, seeded_m5_budget_taint_cli_headless_beta_narrowed());
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_budget_taint_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
