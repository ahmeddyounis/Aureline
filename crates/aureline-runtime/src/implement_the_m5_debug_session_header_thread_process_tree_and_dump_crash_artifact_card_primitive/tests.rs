//! Tests for the M5 debug-session-hierarchy primitive: the resolver, the parity matrix,
//! and the checked-in support export.

use super::*;

// --- resolver: AC1 hierarchy stays understandable, never flattened ---

#[test]
fn resolver_preserves_thread_process_hierarchy() {
    let resolved = resolve_debug_hierarchy(&task_launch_live_input()).expect("resolves");
    assert_eq!(resolved.tree_rows.len(), 3);
    let root = &resolved.tree_rows[0];
    assert_eq!(root.node_kind, M5DebugNodeKind::Process);
    assert_eq!(root.depth, 0);
    assert!(root.parent_ref.is_none());
    for child in &resolved.tree_rows[1..] {
        assert_eq!(child.node_kind, M5DebugNodeKind::Thread);
        assert_eq!(child.depth, 1);
        assert_eq!(child.parent_ref.as_deref(), Some(root.node_ref.as_str()));
        assert!(child.hierarchy_preserved);
    }
    assert!(resolved.hierarchy_understandable_when_narrowed());
}

#[test]
fn resolver_keeps_hierarchy_understandable_when_restored_and_degraded() {
    let resolved =
        resolve_debug_hierarchy(&history_restored_unsymbolicated_input()).expect("resolves");
    assert!(resolved.header.restored);
    assert!(resolved.degraded.is_some());
    assert!(resolved.hierarchy_understandable_when_narrowed());
    assert_eq!(
        resolved.export.node_summaries.len(),
        resolved.tree_rows.len()
    );
}

#[test]
fn resolver_rejects_empty_tree() {
    let mut input = task_launch_live_input();
    input.tree_nodes.clear();
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::EmptyTree)
    );
}

#[test]
fn resolver_rejects_dangling_parent() {
    let mut input = task_launch_live_input();
    input.tree_nodes[1].parent_ref = Some("process:missing".to_owned());
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::TreeParentMissing)
    );
}

#[test]
fn resolver_rejects_tree_without_root() {
    let mut input = task_launch_live_input();
    // Point the root at a child so no node is a root.
    input.tree_nodes[0].parent_ref = Some("thread:build-runner:0001#main".to_owned());
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::TreeRootMissing)
    );
}

#[test]
fn resolver_rejects_duplicate_node() {
    let mut input = task_launch_live_input();
    input.tree_nodes[2].node_ref = input.tree_nodes[1].node_ref.clone();
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::DuplicateNode)
    );
}

#[test]
fn resolver_rejects_running_process_without_threads() {
    let mut input = task_launch_live_input();
    input.tree_nodes[0].thread_count = 0;
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::ProcessThreadCountInvalid)
    );
}

// --- resolver: AC2 live control distinguished from captured analysis ---

#[test]
fn resolver_derives_live_control_posture_from_mode() {
    let resolved = resolve_debug_hierarchy(&task_launch_live_input()).expect("resolves");
    assert_eq!(
        resolved.header.control_posture,
        M5DebugControlPosture::LiveAttachedControl
    );
    assert!(resolved.header.is_live_control);
    assert!(resolved.header.truth_mode.is_live());
    assert!(resolved.distinguishes_control());
}

#[test]
fn resolver_derives_captured_posture_from_core_mode() {
    let resolved = resolve_debug_hierarchy(&publish_core_symbolicated_input()).expect("resolves");
    assert_eq!(
        resolved.header.control_posture,
        M5DebugControlPosture::CapturedAnalysis
    );
    assert!(!resolved.header.is_live_control);
    assert!(!resolved.header.truth_mode.is_live());
    assert!(resolved.distinguishes_control());
}

#[test]
fn resolver_derives_inspect_only_posture() {
    let resolved = resolve_debug_hierarchy(&companion_inspect_only_input()).expect("resolves");
    assert_eq!(
        resolved.header.control_posture,
        M5DebugControlPosture::InspectOnlyView
    );
    assert!(!resolved.header.is_live_control);
    assert!(resolved.distinguishes_control());
}

#[test]
fn resolver_rejects_live_control_with_captured_truth() {
    let mut input = task_launch_live_input();
    input.truth_mode = M5ExecutionTruthMode::Captured;
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::ControlPostureTruthMismatch)
    );
}

#[test]
fn resolver_rejects_captured_shown_as_live() {
    let mut input = publish_core_symbolicated_input();
    input.truth_mode = M5ExecutionTruthMode::Live;
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::ControlPostureTruthMismatch)
    );
}

#[test]
fn resolver_rejects_live_control_without_capable_adapter() {
    let mut input = task_launch_live_input();
    input.adapter_state = M5DebugAdapterState::Unavailable;
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::LiveControlAdapterUnavailable)
    );
}

#[test]
fn resolver_rejects_running_stop_reason_for_captured() {
    let mut input = publish_core_symbolicated_input();
    input.stop_reason = M5DebugStopReason::Running;
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::StopReasonInconsistentWithControl)
    );
}

#[test]
fn resolver_rejects_crash_capture_stop_for_live_control() {
    let mut input = task_launch_live_input();
    input.stop_reason = M5DebugStopReason::CrashCapture;
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::StopReasonInconsistentWithControl)
    );
}

#[test]
fn resolver_rejects_captured_tree_row_with_live_control_action() {
    let mut input = publish_core_symbolicated_input();
    input.tree_nodes[1]
        .available_actions
        .push(M5DebugActionKind::ContinueExecution);
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::CapturedTreeRowImpliesLiveControl)
    );
}

#[test]
fn resolver_rejects_dump_card_with_live_control_action() {
    let mut input = publish_core_symbolicated_input();
    input.dump_cards[0]
        .available_actions
        .push(M5DebugActionKind::DetachSession);
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::DumpCardImpliesLiveControl)
    );
}

// --- resolver: AC3 mapping-quality and provenance preserved ---

#[test]
fn resolver_preserves_dump_provenance_and_symbolication() {
    let resolved = resolve_debug_hierarchy(&publish_core_symbolicated_input()).expect("resolves");
    let card = &resolved.dump_cards[0];
    assert_eq!(card.artifact_kind, M5DumpArtifactKind::FullCore);
    assert_eq!(card.symbolication, M5SymbolicationState::Symbolicated);
    assert!(card.captured_truth);
    assert!(!card.implies_live_control);
    assert!(card.provenance_preserved);
    assert_eq!(card.producing_run_ref, "run:release-bundle:0006");
    assert!(resolved.preserves_mapping_and_provenance());
}

#[test]
fn resolver_rejects_dump_without_lineage() {
    let mut input = publish_core_symbolicated_input();
    input.dump_cards[0].producing_run_ref = "  ".to_owned();
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::DumpLineageBroken)
    );
}

#[test]
fn resolver_rejects_dump_without_provenance() {
    let mut input = publish_core_symbolicated_input();
    input.dump_cards[0].build_provenance_label = String::new();
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::DumpProvenanceMissing)
    );
}

#[test]
fn resolver_rejects_duplicate_dump() {
    let mut input = publish_core_symbolicated_input();
    let dup = input.dump_cards[0].clone();
    input.dump_cards.push(dup);
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::DuplicateDump)
    );
}

// --- resolver: shared identity + selected thread + structural rejections ---

#[test]
fn resolver_identity_consistent_across_projections() {
    let resolved = resolve_debug_hierarchy(&publish_core_symbolicated_input()).expect("resolves");
    assert!(resolved.identity_consistent());
    assert_eq!(resolved.cli_line.session_id, resolved.session_id);
    assert_eq!(resolved.export.target_ref, resolved.target_ref);
    assert_eq!(resolved.dump_cards[0].session_ref, resolved.session_ref);
}

#[test]
fn resolver_rejects_collapsed_session_and_target_identity() {
    let mut input = task_launch_live_input();
    input.target_ref = input.session_ref.clone();
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::SessionTargetIdentityCollapsed)
    );
}

#[test]
fn resolver_rejects_selected_thread_not_in_tree() {
    let mut input = task_launch_live_input();
    input.selected_thread_ref = Some("thread:missing".to_owned());
    // Also clear is_selected so the mismatch is specifically about presence in the tree.
    for node in &mut input.tree_nodes {
        node.is_selected = false;
    }
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::SelectedThreadNotInTree)
    );
}

#[test]
fn resolver_rejects_multiple_selected_threads() {
    let mut input = task_launch_live_input();
    input.tree_nodes[2].is_selected = true;
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::MultipleThreadsSelected)
    );
}

#[test]
fn resolver_rejects_selected_ref_mismatch() {
    let mut input = task_launch_live_input();
    input.selected_thread_ref = Some("thread:build-runner:0001#worker".to_owned());
    // Node #main is still is_selected, but the ref points at #worker.
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::SelectedThreadMismatch)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let mut input = task_launch_live_input();
    input.context_summary = "see https://example.com/session".to_owned();
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let mut input = history_restored_unsymbolicated_input();
    input.degraded = Some(DegradedState {
        trigger: M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
        degraded_label: "degraded".to_owned(),
    });
    assert_eq!(
        resolve_debug_hierarchy(&input),
        Err(M5DebugHierarchyError::DegradedLabelGeneric)
    );
}

#[test]
fn cli_line_renders_mode_posture_and_counts() {
    let resolved = resolve_debug_hierarchy(&publish_core_symbolicated_input()).expect("resolves");
    assert!(resolved.cli_line.line.contains("mode=core"));
    assert!(resolved.cli_line.line.contains("posture=captured_analysis"));
    assert!(resolved.cli_line.line.contains("dumps=1"));
    assert!(resolved.cli_line.line.contains("nodes=2"));
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_debug_hierarchy_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_debug_hierarchy_packet();
    let present: BTreeSet<M5RunAttemptSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5RunAttemptSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_matrix_covers_every_session_mode() {
    let packet = seeded_m5_debug_hierarchy_packet();
    let seen: BTreeSet<M5DebugSessionMode> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_sessions.iter())
        .map(|case| case.resolved.header.session_mode)
        .collect();
    for mode in DEBUG_SESSION_MODE_ALL {
        assert!(seen.contains(&mode), "missing session mode {mode:?}");
    }
}

#[test]
fn seeded_matrix_covers_every_control_posture() {
    let packet = seeded_m5_debug_hierarchy_packet();
    let seen: BTreeSet<M5DebugControlPosture> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_sessions.iter())
        .map(|case| case.resolved.header.control_posture)
        .collect();
    for posture in M5DebugControlPosture::ALL {
        assert!(seen.contains(&posture), "missing posture {posture:?}");
    }
}

#[test]
fn seeded_matrix_covers_every_symbolication_state() {
    let packet = seeded_m5_debug_hierarchy_packet();
    let seen: BTreeSet<M5SymbolicationState> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_sessions.iter())
        .flat_map(|case| case.resolved.dump_cards.iter())
        .map(|card| card.symbolication)
        .collect();
    for state in SYMBOLICATION_ALL {
        assert!(seen.contains(&state), "missing symbolication {state:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_debug_hierarchy_packet();
    for row in &packet.surface_rows {
        for case in &row.example_sessions {
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
    assert!(M5DebugVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_debug_hierarchy_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_debug_hierarchy_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5DebugViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_debug_hierarchy_packet();
    packet.surface_rows[0].flattens_hierarchy = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5DebugViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_debug_hierarchy_packet();
    packet.surface_rows[0].example_sessions[0]
        .resolved
        .header
        .session_mode = M5DebugSessionMode::Core;
    let violations = packet.validate();
    assert!(violations.contains(&M5DebugViolation::ExampleSessionDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_debug_hierarchy_packet();
    packet
        .vocabulary_set
        .control_postures
        .push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5DebugViolation::VocabularySetDrift));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_debug_hierarchy_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_debug_hierarchy_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_debug_hierarchy_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_debug_hierarchy_packet();
    assert_eq!(packet.record_kind, M5_DEBUG_HIERARCHY_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_DEBUG_HIERARCHY_SCHEMA_VERSION);
}
