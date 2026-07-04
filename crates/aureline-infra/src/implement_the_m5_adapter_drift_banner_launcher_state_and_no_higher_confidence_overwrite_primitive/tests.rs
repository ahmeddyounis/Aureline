//! Tests for the M5 execution-confidence primitive: the resolver, the parity
//! matrix, and the checked-in support export.

use super::*;

// --- resolver: AC1 affordances narrow before launch on a capability drop ---

#[test]
fn resolver_preserves_target_identity_across_surfaces() {
    let input = adapter_dropped_to_heuristic_input();
    let resolved = resolve_execution_confidence(&input).expect("resolves");
    assert_eq!(resolved.target_id, input.target_id);
    assert_eq!(resolved.drift_banner.target_id, input.target_id);
    assert_eq!(resolved.launcher.target_id, input.target_id);
    assert_eq!(resolved.overwrite_guard.target_id, input.target_id);
    assert!(resolved.identity_consistent());
}

#[test]
fn resolver_blocks_lost_and_narrows_downgraded_before_launch() {
    let resolved =
        resolve_execution_confidence(&adapter_dropped_to_heuristic_input()).expect("resolves");
    assert!(resolved.launcher.narrowed_before_launch);
    assert!(resolved.capability_drop_present());
    // Debug was lost -> blocked.
    assert!(resolved.drift_banner.lost_verbs.contains(&M5BuildVerb::Debug));
    assert!(resolved.launcher.blocked_verbs.contains(&M5BuildVerb::Debug));
    let debug = resolved
        .launcher
        .affordances
        .iter()
        .find(|a| a.verb == M5BuildVerb::Debug)
        .expect("debug affordance present");
    assert_eq!(debug.affordance, M5AffordanceState::Blocked);
    assert!(!debug.launchable_before_run);
    // Build was downgraded -> narrowed (not launchable, not blocked).
    let build = resolved
        .launcher
        .affordances
        .iter()
        .find(|a| a.verb == M5BuildVerb::Build)
        .expect("build affordance present");
    assert_eq!(build.affordance, M5AffordanceState::Narrowed);
    assert!(!build.launchable_before_run);
    assert!(resolved.affordances_narrow_when_capability_drops());
}

#[test]
fn resolver_keeps_all_verbs_launchable_without_drift() {
    let resolved = resolve_execution_confidence(&native_no_drift_input()).expect("resolves");
    assert!(!resolved.launcher.narrowed_before_launch);
    assert!(!resolved.capability_drop_present());
    assert!(!resolved.drift_banner.drift_detected);
    assert!(resolved
        .launcher
        .affordances
        .iter()
        .all(|a| a.affordance == M5AffordanceState::Available));
    assert!(resolved.affordances_narrow_when_capability_drops());
}

#[test]
fn resolver_marks_gained_verbs_available_after_recompute() {
    let resolved =
        resolve_execution_confidence(&recompute_recovered_input()).expect("resolves");
    assert!(resolved.drift_banner.gained_verbs.contains(&M5BuildVerb::Build));
    assert!(resolved.drift_banner.gained_verbs.contains(&M5BuildVerb::Run));
    assert!(!resolved.launcher.narrowed_before_launch);
    assert!(resolved
        .launcher
        .affordances
        .iter()
        .all(|a| a.affordance == M5AffordanceState::Available));
}

// --- resolver: AC2 drift and affected targets visible before action ---

#[test]
fn resolver_makes_drift_and_affected_targets_visible() {
    let resolved =
        resolve_execution_confidence(&adapter_dropped_to_heuristic_input()).expect("resolves");
    assert!(resolved.drift_banner.drift_detected);
    assert!(resolved.drift_banner.adapter_changed);
    assert!(resolved.drift_banner.visible_before_action);
    assert_eq!(resolved.drift_banner.affected_targets.len(), 2);
    assert!(resolved.drift_banner.divergence_note.is_some());
    assert!(resolved
        .drift_banner
        .actions
        .iter()
        .any(|a| a.is_recompute()));
    assert!(resolved
        .drift_banner
        .actions
        .iter()
        .any(|a| a.is_diagnostics()));
    assert!(resolved.drift_visible_and_actionable());
}

#[test]
fn resolver_detects_capability_drift_without_adapter_change() {
    let resolved =
        resolve_execution_confidence(&provider_overlay_gated_input()).expect("resolves");
    assert!(!resolved.drift_banner.adapter_changed);
    assert!(resolved.drift_banner.drift_detected);
    assert!(resolved.drift_banner.downgraded_verbs.contains(&M5BuildVerb::Test));
    assert!(resolved.drift_visible_and_actionable());
}

#[test]
fn resolver_rejects_drift_without_affected_targets() {
    let input = M5ExecutionConfidenceInput {
        affected_targets: vec![],
        ..adapter_dropped_to_heuristic_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::DriftWithoutAffectedTargets)
    );
}

#[test]
fn resolver_rejects_drift_without_divergence_note() {
    let input = M5ExecutionConfidenceInput {
        divergence_note: None,
        ..adapter_dropped_to_heuristic_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::DriftWithoutDivergenceDetail)
    );
}

#[test]
fn resolver_rejects_drift_without_recovery_actions() {
    let input = M5ExecutionConfidenceInput {
        available_actions: vec![
            M5ExecutionActionKind::InspectCapabilities,
            M5ExecutionActionKind::CopyExport,
        ],
        ..adapter_dropped_to_heuristic_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::DriftWithoutRecoveryActions)
    );
}

#[test]
fn resolver_rejects_unstable_affected_target() {
    let mut input = adapter_dropped_to_heuristic_input();
    input.affected_targets[0].owning_module = "  ".to_owned();
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::AffectedTargetNotStable)
    );
}

// --- resolver: AC3 no higher-confidence overwrite / masquerade ---

#[test]
fn resolver_records_explicit_downgrade_preserving_higher_truth() {
    let resolved =
        resolve_execution_confidence(&structured_channel_lost_input()).expect("resolves");
    assert_eq!(
        resolved.overwrite_guard.verdict,
        M5OverwriteVerdict::RecordedExplicitDowngrade
    );
    assert!(resolved.overwrite_guard.explicit_downgrade_recorded);
    assert!(resolved.overwrite_guard.preserves_higher_confidence_truth);
    assert!(resolved.overwrite_guard.downgrade_note.is_some());
    assert_eq!(
        resolved.overwrite_guard.downgrade_trigger,
        Some(M5ManifestBuildDowngradeTrigger::StructuredChannelLost)
    );
    assert!(resolved.no_higher_confidence_masquerade());
}

#[test]
fn resolver_promotes_higher_confidence_without_downgrade() {
    let resolved =
        resolve_execution_confidence(&recompute_recovered_input()).expect("resolves");
    assert_eq!(
        resolved.overwrite_guard.verdict,
        M5OverwriteVerdict::PromotedHigherConfidence
    );
    assert!(!resolved.overwrite_guard.explicit_downgrade_recorded);
    assert!(resolved.overwrite_guard.downgrade_note.is_none());
    assert!(resolved.no_higher_confidence_masquerade());
}

#[test]
fn resolver_matches_existing_confidence_without_downgrade() {
    let resolved =
        resolve_execution_confidence(&provider_overlay_gated_input()).expect("resolves");
    assert_eq!(
        resolved.overwrite_guard.verdict,
        M5OverwriteVerdict::MatchedExistingConfidence
    );
    assert!(!resolved.overwrite_guard.explicit_downgrade_recorded);
}

#[test]
fn resolver_rejects_silent_higher_confidence_overwrite() {
    let input = M5ExecutionConfidenceInput {
        downgrade_acknowledged: false,
        downgrade_note: None,
        ..structured_channel_lost_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::SilentHigherConfidenceOverwrite)
    );
}

#[test]
fn resolver_rejects_silent_native_masquerade() {
    // Incoming fallback at the SAME confidence rank as existing native truth still
    // needs an acknowledged downgrade, or it would mask the native lane.
    let input = M5ExecutionConfidenceInput {
        prior_adapter: M5AdapterSourceKind::HeuristicParse,
        current_adapter: M5AdapterSourceKind::HeuristicParse,
        confidence: M5DiscoveryConfidence::Medium,
        existing_adapter: M5AdapterSourceKind::NativeBuildServer,
        existing_confidence: M5DiscoveryConfidence::Medium,
        downgrade_acknowledged: false,
        downgrade_note: None,
        ..provider_overlay_gated_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::SilentNativeMasquerade)
    );
}

#[test]
fn resolver_rejects_downgrade_without_note() {
    let input = M5ExecutionConfidenceInput {
        downgrade_note: None,
        ..structured_channel_lost_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::DowngradeWithoutNote)
    );
}

#[test]
fn resolver_rejects_downgrade_note_without_downgrade() {
    let input = M5ExecutionConfidenceInput {
        downgrade_note: Some("spurious note".to_owned()),
        ..native_no_drift_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::DowngradeNoteWithoutDowngrade)
    );
}

#[test]
fn resolver_parity_consumers_carry_source_and_confidence() {
    let resolved =
        resolve_execution_confidence(&adapter_dropped_to_heuristic_input()).expect("resolves");
    assert_eq!(resolved.parity_consumers.len(), 4);
    for consumer in &resolved.parity_consumers {
        assert!(consumer.carries_adapter_source);
        assert!(consumer.carries_confidence);
        assert_eq!(consumer.adapter_source, M5AdapterSourceKind::HeuristicParse);
        assert_eq!(consumer.confidence, M5DiscoveryConfidence::Low);
    }
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_empty_target_id() {
    let input = M5ExecutionConfidenceInput {
        target_id: "   ".to_owned(),
        ..native_no_drift_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::EmptyTargetId)
    );
}

#[test]
fn resolver_rejects_incomplete_identity() {
    let input = M5ExecutionConfidenceInput {
        identity: M5TargetIdentity {
            node_kind: crate::M5TargetGraphNodeKind::BuildTarget,
            stable_id: "target:x".to_owned(),
            owning_module: "  ".to_owned(),
            workspace_root: "root:workspace".to_owned(),
        },
        ..native_no_drift_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::EmptyTargetIdentity)
    );
}

#[test]
fn resolver_rejects_fallback_source_claiming_high_confidence() {
    let input = M5ExecutionConfidenceInput {
        confidence: M5DiscoveryConfidence::High,
        ..adapter_dropped_to_heuristic_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::AdapterConfidenceInconsistent)
    );
}

#[test]
fn resolver_rejects_supported_verb_from_unknown_confidence() {
    let input = M5ExecutionConfidenceInput {
        prior_adapter: M5AdapterSourceKind::Unknown,
        current_adapter: M5AdapterSourceKind::Unknown,
        confidence: M5DiscoveryConfidence::Unknown,
        existing_confidence: M5DiscoveryConfidence::Unknown,
        existing_adapter: M5AdapterSourceKind::Unknown,
        downgrade_acknowledged: false,
        downgrade_note: None,
        verbs: vec![verb(
            M5BuildVerb::Build,
            M5CapabilityState::Supported,
            M5CapabilityState::Supported,
        )],
        ..native_no_drift_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::SupportedVerbUnknownConfidence)
    );
}

#[test]
fn resolver_rejects_empty_verbs() {
    let input = M5ExecutionConfidenceInput {
        verbs: vec![],
        ..native_no_drift_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::NoVerbsDeclared)
    );
}

#[test]
fn resolver_rejects_missing_export_action() {
    let input = M5ExecutionConfidenceInput {
        available_actions: vec![M5ExecutionActionKind::InspectCapabilities],
        ..native_no_drift_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::NoExportActionOffered)
    );
}

#[test]
fn resolver_rejects_no_parity_consumers() {
    let input = M5ExecutionConfidenceInput {
        parity_consumers: vec![],
        ..native_no_drift_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::NoParityConsumers)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5ExecutionConfidenceInput {
        target_label: "see https://example.com/secrets".to_owned(),
        ..native_no_drift_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5ExecutionConfidenceInput {
        degraded: Some(DegradedState {
            trigger: M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
            degraded_label: "fallback".to_owned(),
        }),
        ..adapter_dropped_to_heuristic_input()
    };
    assert_eq!(
        resolve_execution_confidence(&input),
        Err(M5ExecutionConfidenceResolutionError::DegradedLabelGeneric)
    );
}

#[test]
fn resolver_carries_degraded_trigger_from_block() {
    let resolved =
        resolve_execution_confidence(&adapter_dropped_to_heuristic_input()).expect("resolves");
    assert!(resolved.degraded.is_some());
    assert_eq!(
        resolved.overwrite_guard.downgrade_trigger,
        Some(M5ManifestBuildDowngradeTrigger::AdapterUnavailable)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_execution_confidence_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_execution_confidence_packet();
    let present: BTreeSet<M5ExecutionSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5ExecutionSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_execution_confidence_packet();
    for row in &packet.surface_rows {
        for case in &row.example_confidence {
            assert!(
                case.is_self_consistent(),
                "case drifted on {:?}",
                row.surface_family
            );
        }
    }
}

#[test]
fn vocabulary_set_matches_canonical() {
    assert!(M5ExecutionConfidenceVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_execution_confidence_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_execution_confidence_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5ExecutionConfidenceViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_execution_confidence_packet();
    packet.surface_rows[0].allows_silent_overwrite = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5ExecutionConfidenceViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_execution_confidence_packet();
    packet.surface_rows[0].example_confidence[0]
        .resolved
        .launcher
        .narrowed_before_launch = !packet.surface_rows[0].example_confidence[0]
        .resolved
        .launcher
        .narrowed_before_launch;
    let violations = packet.validate();
    assert!(violations.contains(&M5ExecutionConfidenceViolation::ExampleConfidenceDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_execution_confidence_packet();
    packet.vocabulary_set.overwrite_verdicts.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5ExecutionConfidenceViolation::VocabularySetDrift));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_execution_confidence_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_execution_confidence_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_execution_confidence_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_execution_confidence_packet();
    assert_eq!(packet.record_kind, M5_EXECUTION_CONFIDENCE_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_EXECUTION_CONFIDENCE_SCHEMA_VERSION);
}
