//! Tests for the M5 build / run confidence primitive: the resolver, the parity
//! matrix, and the checked-in support export.

use super::*;

// --- resolver: AC1 adapter provenance disclosed, structured/fallback distinct ---

#[test]
fn resolver_preserves_target_identity_across_surfaces() {
    let input = native_build_server_input();
    let resolved = resolve_build_confidence(&input).expect("resolves");
    assert_eq!(resolved.target_id, input.target_id);
    assert_eq!(resolved.adapter_badge.target_id, input.target_id);
    assert_eq!(resolved.target_graph_row.target_id, input.target_id);
    assert_eq!(resolved.capability_matrix.target_id, input.target_id);
    assert_eq!(resolved.raw_event_drawer.target_id, input.target_id);
    assert_eq!(resolved.fallback_drawer.target_id, input.target_id);
    assert!(resolved.identity_consistent());
}

#[test]
fn resolver_discloses_native_source_explicitly() {
    let resolved = resolve_build_confidence(&native_build_server_input()).expect("resolves");
    assert!(resolved.adapter_badge.source_kind_explicit);
    assert!(resolved.adapter_badge.is_native);
    assert!(resolved.adapter_badge.confidence_consistent);
    assert_eq!(
        resolved.adapter_badge.adapter_source,
        M5AdapterSourceKind::NativeBuildServer
    );
    assert!(!resolved.fallback_drawer.is_fallback);
    assert!(resolved.provenance_disclosed_consistently());
}

#[test]
fn resolver_discloses_fallback_source_without_claiming_native() {
    let resolved = resolve_build_confidence(&heuristic_fallback_input()).expect("resolves");
    assert!(!resolved.adapter_badge.is_native);
    assert!(resolved.fallback_drawer.is_fallback);
    assert_eq!(
        resolved.fallback_drawer.fallback_reason,
        Some(M5FallbackReason::AdapterUnavailable)
    );
    assert!(resolved.fallback_drawer.fallback_note.is_some());
    assert_eq!(
        resolved.fallback_drawer.downgrade_trigger,
        Some(M5ManifestBuildDowngradeTrigger::AdapterUnavailable)
    );
    assert!(resolved.provenance_disclosed_consistently());
}

#[test]
fn resolver_rejects_fallback_source_claiming_high_confidence() {
    let input = M5BuildConfidenceInput {
        confidence: M5DiscoveryConfidence::High,
        ..heuristic_fallback_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::AdapterConfidenceInconsistent)
    );
}

#[test]
fn resolver_rejects_native_source_claiming_fallback_state() {
    let input = M5BuildConfidenceInput {
        fallback_state: M5FallbackConfidenceState::HeuristicFallback,
        fallback_reason: Some(M5FallbackReason::AdapterUnavailable),
        fallback_note: Some("note".to_owned()),
        ..native_build_server_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::AdapterFallbackMismatch)
    );
}

#[test]
fn resolver_rejects_fallback_without_reason() {
    let input = M5BuildConfidenceInput {
        fallback_reason: None,
        ..heuristic_fallback_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::FallbackWithoutReason)
    );
}

#[test]
fn resolver_rejects_fallback_without_note() {
    let input = M5BuildConfidenceInput {
        fallback_note: None,
        ..heuristic_fallback_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::FallbackWithoutNote)
    );
}

#[test]
fn resolver_rejects_structured_state_with_fallback_reason() {
    let input = M5BuildConfidenceInput {
        fallback_reason: Some(M5FallbackReason::PolicyBlock),
        ..native_build_server_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::StructuredWithFallbackReason)
    );
}

// --- resolver: AC2 identity + confidence inspectable before action ---

#[test]
fn resolver_keeps_target_graph_identity_and_verbs_inspectable() {
    let resolved =
        resolve_build_confidence(&native_build_event_partial_input()).expect("resolves");
    assert!(resolved.target_graph_row.target_context_visible);
    assert!(resolved.target_graph_row.identity.is_stable());
    assert_eq!(
        resolved.target_graph_row.node_kind,
        M5TargetGraphNodeKind::TestTarget
    );
    assert_eq!(
        resolved.target_graph_row.required_environment,
        vec!["env:TEST_SHARD".to_owned()]
    );
    // Supported verbs include partial; downgraded verbs list the rest.
    assert!(resolved
        .target_graph_row
        .supported_verbs
        .contains(&M5BuildVerb::Debug));
    assert!(resolved
        .capability_matrix
        .downgraded_verbs
        .contains(&M5BuildVerb::Coverage));
    assert!(resolved.identity_and_confidence_inspectable());
}

#[test]
fn resolver_marks_downgraded_capability_cells() {
    let resolved =
        resolve_build_confidence(&native_build_event_partial_input()).expect("resolves");
    let debug_cell = resolved
        .capability_matrix
        .cells
        .iter()
        .find(|c| c.verb == M5BuildVerb::Debug)
        .expect("debug cell present");
    assert_eq!(debug_cell.state, M5CapabilityState::Partial);
    assert!(debug_cell.downgraded);
    let build_cell = resolved
        .capability_matrix
        .cells
        .iter()
        .find(|c| c.verb == M5BuildVerb::Build)
        .expect("build cell present");
    assert!(!build_cell.downgraded);
}

#[test]
fn resolver_rejects_supported_capability_from_unknown_confidence() {
    let input = M5BuildConfidenceInput {
        adapter_source: M5AdapterSourceKind::Unknown,
        confidence: M5DiscoveryConfidence::Unknown,
        fallback_state: M5FallbackConfidenceState::Unknown,
        fallback_reason: Some(M5FallbackReason::PolicyBlock),
        fallback_note: Some("adapter kind not yet established".to_owned()),
        capabilities: vec![cap(M5BuildVerb::Build, M5CapabilityState::Supported)],
        ..heuristic_fallback_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::SupportedCapabilityUnknownConfidence)
    );
}

#[test]
fn resolver_rejects_empty_capabilities() {
    let input = M5BuildConfidenceInput {
        capabilities: vec![],
        ..native_build_server_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::NoCapabilitiesDeclared)
    );
}

// --- resolver: AC3 support / AI reuse ---

#[test]
fn resolver_reconstructs_support_reusable_raw_events() {
    let resolved = resolve_build_confidence(&support_replay_input()).expect("resolves");
    assert!(resolved.raw_event_drawer.redaction_applied);
    assert!(resolved.raw_event_drawer.preserves_event_identity);
    assert_eq!(resolved.raw_event_drawer.adapter_version, "snapshot:2026-06-30");
    assert!(!resolved.raw_event_drawer.payload_lineage.is_empty());
    assert!(resolved
        .raw_event_drawer
        .export_actions
        .contains(&M5BuildActionKind::CopyExport));
    assert!(resolved.support_reuse_ready());
}

#[test]
fn resolver_rejects_missing_export_action() {
    let input = M5BuildConfidenceInput {
        available_actions: vec![
            M5BuildActionKind::InspectCapabilities,
            M5BuildActionKind::ViewRawEvents,
        ],
        ..native_build_server_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::NoExportActionOffered)
    );
}

#[test]
fn resolver_rejects_empty_payload_lineage() {
    let input = M5BuildConfidenceInput {
        payload_lineage: vec![],
        ..native_build_server_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::EmptyPayloadLineage)
    );
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_empty_target_id() {
    let input = M5BuildConfidenceInput {
        target_id: "   ".to_owned(),
        ..native_build_server_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::EmptyTargetId)
    );
}

#[test]
fn resolver_rejects_incomplete_identity() {
    let input = M5BuildConfidenceInput {
        identity: M5TargetIdentity {
            node_kind: M5TargetGraphNodeKind::BuildTarget,
            stable_id: "target:x".to_owned(),
            owning_module: "  ".to_owned(),
            workspace_root: "root:workspace".to_owned(),
        },
        ..native_build_server_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::EmptyTargetIdentity)
    );
}

#[test]
fn resolver_rejects_empty_adapter_version() {
    let input = M5BuildConfidenceInput {
        adapter_version: "".to_owned(),
        ..native_build_server_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::EmptyAdapterVersion)
    );
}

#[test]
fn resolver_rejects_no_actions() {
    let input = M5BuildConfidenceInput {
        available_actions: vec![],
        ..native_build_server_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::NoActionsOffered)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5BuildConfidenceInput {
        target_label: "see https://example.com/secrets".to_owned(),
        ..native_build_server_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5BuildConfidenceInput {
        degraded: Some(DegradedState {
            trigger: M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
            degraded_label: "fallback".to_owned(),
        }),
        ..heuristic_fallback_input()
    };
    assert_eq!(
        resolve_build_confidence(&input),
        Err(M5BuildConfidenceResolutionError::DegradedLabelGeneric)
    );
}

#[test]
fn resolver_carries_degraded_trigger_from_block() {
    let resolved = resolve_build_confidence(&heuristic_fallback_input()).expect("resolves");
    assert!(resolved.degraded.is_some());
    assert_eq!(
        resolved.fallback_drawer.downgrade_trigger,
        Some(M5ManifestBuildDowngradeTrigger::AdapterUnavailable)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_build_confidence_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_build_confidence_packet();
    let present: BTreeSet<M5BuildConfidenceSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5BuildConfidenceSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_build_confidence_packet();
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
    assert!(M5BuildConfidenceVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_build_confidence_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_build_confidence_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5BuildConfidenceViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_build_confidence_packet();
    packet.surface_rows[0].presents_fallback_as_structured = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5BuildConfidenceViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_build_confidence_packet();
    packet.surface_rows[0].example_confidence[0]
        .resolved
        .adapter_badge
        .is_native = !packet.surface_rows[0].example_confidence[0]
        .resolved
        .adapter_badge
        .is_native;
    let violations = packet.validate();
    assert!(violations.contains(&M5BuildConfidenceViolation::ExampleConfidenceDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_build_confidence_packet();
    packet.vocabulary_set.build_verbs.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5BuildConfidenceViolation::VocabularySetDrift));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_build_confidence_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_build_confidence_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_build_confidence_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_build_confidence_packet();
    assert_eq!(packet.record_kind, M5_BUILD_CONFIDENCE_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_BUILD_CONFIDENCE_SCHEMA_VERSION);
}
