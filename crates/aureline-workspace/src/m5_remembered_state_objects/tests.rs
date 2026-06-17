use super::*;

const SCHEMA_EVOLUTION_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-remembered-state-objects/schema_evolution_forward_migrate.json"
));

const PARTIAL_HYDRATE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-remembered-state-objects/partial_hydrate.json"
));

const MISSING_DEPENDENCY_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-remembered-state-objects/missing_dependency_substitution.json"
));

fn packet() -> M5RememberedStateObjects {
    current_m5_remembered_state_objects().expect("packet parses")
}

fn snapshot(json: &str) -> WindowTopologySnapshot {
    serde_json::from_str(json).expect("fixture parses")
}

// --- Embedded packet -----------------------------------------------------------------------------

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_REMEMBERED_STATE_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_REMEMBERED_STATE_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn embedded_packet_round_trips_byte_stable_shape() {
    let packet = packet();
    let encoded = serde_json::to_string(&packet).expect("serializes");
    let decoded: M5RememberedStateObjects = serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded, packet);
}

#[test]
fn summary_counts_match_objects() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn object_schema_registry_matches_build() {
    let packet = packet();
    assert_eq!(
        packet.object_schema_versions,
        ObjectSchemaVersions::current()
    );
}

#[test]
fn checkpoint_keeps_authority_safe_and_separate() {
    let packet = packet();
    let ckpt = packet
        .checkpoint("ckpt:primary-session")
        .expect("checkpoint present");
    assert!(ckpt.is_authority_safe());
    assert_eq!(
        ckpt.authority_handle_class,
        AuthorityHandleClass::ReResolvableReference
    );
    assert!(ckpt.excludes_live_authority);
    // Dirty buffers carry identity, not content.
    assert!(ckpt.dirty_buffers.iter().all(|b| !b.buffer_id.is_empty()));
    assert!(ckpt.dirty_buffers.iter().any(|b| b.dirty));
    assert!(ckpt
        .dirty_buffers
        .iter()
        .any(|b| b.draft_journal_ref.is_some()));
}

#[test]
fn bundle_wires_four_objects_by_reference() {
    let packet = packet();
    let bundle = &packet.bundles[0];
    assert!(packet.checkpoint(&bundle.workspace_authority_ref).is_some());
    assert!(bundle
        .window_snapshot_refs
        .iter()
        .all(|r| packet.snapshot(r).is_some()));
    assert!(packet
        .profile(bundle.profile_defaults_ref.as_deref().unwrap())
        .is_some());
    assert!(packet
        .machine_hints(bundle.machine_local_hints_ref.as_deref().unwrap())
        .is_some());
    // The authority ref is a checkpoint, never a snapshot — authority and topology stay separate.
    assert!(packet.snapshot(&bundle.workspace_authority_ref).is_none());
}

#[test]
fn machine_local_hints_never_exportable() {
    let packet = packet();
    for hints in &packet.machine_local_hints {
        assert_eq!(hints.ownership, StateOwnership::MachineLocal);
        assert!(!hints.exportable);
    }
}

#[test]
fn profile_defaults_are_portable_and_anchor_free() {
    let packet = packet();
    for profile in &packet.profile_defaults {
        assert!(profile.ownership.exportable_into_portable_package());
        assert!(profile.excludes_machine_local_anchors);
    }
}

#[test]
fn snapshot_pane_ids_are_stable_and_unique() {
    let packet = packet();
    let snap = packet.snapshot("snap:primary-window").expect("snapshot");
    let ids = snap.pane_tree.pane_ids();
    assert_eq!(ids, vec!["pane:editor-main", "pane:docs", "pane:preview"]);
    assert!(snap.pane_tree.has_unique_pane_ids());
    assert_eq!(snap.pane_tree.placeholder_pane_ids(), vec!["pane:preview"]);
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let packet = packet();
    let export = packet.support_export("export:remembered-state", "2026-06-16");
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id_ref, packet.packet_id);
    let encoded = serde_json::to_string(&export).expect("serializes");
    let decoded: M5RememberedStateSupportExport =
        serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded.packet, packet);
}

// --- Fixtures ------------------------------------------------------------------------------------

#[test]
fn schema_evolution_fixture_forward_migrates() {
    let snap = snapshot(SCHEMA_EVOLUTION_FIXTURE);
    // The fixture is an older pane-tree schema version.
    assert_eq!(snap.pane_tree.schema_version, 0);
    let (migrated, outcome) = migrate_pane_tree(snap.pane_tree.schema_version, snap.pane_tree);
    assert_eq!(outcome, PaneTreeMigrationOutcome::ForwardMigrated);
    assert_eq!(outcome.restore_class(), RestoreClass::CompatibleRestore);
    assert_eq!(migrated.schema_version, PANE_TREE_SCHEMA_VERSION);
    // Migration preserves stable pane ids.
    assert_eq!(
        migrated.pane_ids(),
        vec!["pane:legacy-editor", "pane:legacy-terminal"]
    );
}

#[test]
fn partial_hydrate_fixture_keeps_slots_and_marks_states() {
    let snap = snapshot(PARTIAL_HYDRATE_FIXTURE);
    assert!(snap.pane_tree.has_unique_pane_ids());
    assert_eq!(snap.pane_tree.pane_ids().len(), 3);
    // A partial hydrate keeps one ready, one pending, and one placeholder slot.
    let ready = snap
        .pane_tree
        .find_leaf("pane:ready-editor")
        .expect("ready pane");
    assert_eq!(ready.surface.availability, Availability::Ready);
    let pending = snap
        .pane_tree
        .find_leaf("pane:pending-notebook")
        .expect("pending pane");
    assert_eq!(pending.surface.availability, Availability::NeedsHydration);
    let blocked = snap
        .pane_tree
        .find_leaf("pane:blocked-ai")
        .expect("blocked pane");
    assert!(blocked.surface.is_placeholder());
    assert!(blocked.surface.placeholder.is_some());
}

#[test]
fn missing_dependency_fixture_substitutes_without_deleting_slot() {
    let snap = snapshot(MISSING_DEPENDENCY_FIXTURE);
    let pane = snap
        .pane_tree
        .find_leaf("pane:custom-extension-view")
        .expect("substituted pane");
    // The slot and pane id survive; the placeholder names the original role and preserves the slot.
    assert!(pane.surface.is_placeholder());
    let card = pane.surface.placeholder.as_ref().expect("card present");
    assert_eq!(card.reason, PlaceholderReason::MissingExtension);
    assert_eq!(card.original_role, SurfaceRole::Explorer);
    assert!(card.substitution_behavior.preserves_slot());
    assert_ne!(
        card.substitution_behavior,
        SubstitutionBehavior::SilentDelete
    );
    // The editor sibling is untouched, so the missing dependency narrowed one slot, not the window.
    let editor = snap
        .pane_tree
        .find_leaf("pane:incident-editor")
        .expect("editor pane");
    assert_eq!(editor.surface.availability, Availability::Ready);
}

// --- Pane-tree operations ------------------------------------------------------------------------

fn leaf(pane_id: &str, role: SurfaceRole, class: SurfaceClass) -> PaneLeaf {
    PaneLeaf {
        pane_id: pane_id.to_owned(),
        surface: PaneSurface {
            surface_role: role,
            surface_class: class,
            availability: Availability::Ready,
            placeholder: None,
        },
    }
}

fn sample_tree() -> PaneTree {
    PaneTree {
        schema_version: PANE_TREE_SCHEMA_VERSION,
        tree_revision: 1,
        root: PaneNode::Split(SplitNode {
            split_id: "split:root".to_owned(),
            orientation: SplitOrientation::Horizontal,
            weight_permille: vec![500, 500],
            children: vec![
                PaneNode::Leaf(leaf(
                    "pane:a",
                    SurfaceRole::Editor,
                    SurfaceClass::TextEditor,
                )),
                PaneNode::TabGroup(TabGroupNode {
                    group_id: "group:right".to_owned(),
                    active_tab_id: "tab:b".to_owned(),
                    tabs: vec![
                        TabRecord {
                            tab_id: "tab:b".to_owned(),
                            pinned: false,
                            pane: leaf("pane:b", SurfaceRole::Docs, SurfaceClass::DocsBrowser),
                        },
                        TabRecord {
                            tab_id: "tab:c".to_owned(),
                            pinned: false,
                            pane: leaf("pane:c", SurfaceRole::Terminal, SurfaceClass::TerminalView),
                        },
                    ],
                }),
            ],
        }),
    }
}

#[test]
fn split_preserves_old_pane_and_adds_new() {
    let mut tree = sample_tree();
    let rev = tree.tree_revision;
    assert!(tree.split_pane(
        "pane:a",
        "split:a",
        SplitOrientation::Vertical,
        leaf("pane:a2", SurfaceRole::Diff, SurfaceClass::DiffEditor),
    ));
    assert!(tree.pane_ids().contains(&"pane:a".to_owned()));
    assert!(tree.pane_ids().contains(&"pane:a2".to_owned()));
    assert!(tree.has_unique_pane_ids());
    assert!(tree.tree_revision > rev);
}

#[test]
fn pin_sets_tab_state() {
    let mut tree = sample_tree();
    assert!(tree.set_tab_pinned("tab:b", true));
    assert!(!tree.set_tab_pinned("tab:missing", true));
}

#[test]
fn close_collapses_emptied_containers() {
    let mut tree = sample_tree();
    // Closing one tab leaves the group; closing the other collapses the group and then the split.
    assert!(tree.close_pane("pane:c"));
    assert_eq!(tree.pane_ids(), vec!["pane:a", "pane:b"]);
    assert!(tree.close_pane("pane:b"));
    // The split collapses to the surviving leaf; the slot never lingers empty.
    assert_eq!(tree.pane_ids(), vec!["pane:a"]);
    assert!(matches!(tree.root, PaneNode::Leaf(_)));
}

#[test]
fn close_last_pane_is_a_no_op() {
    let mut tree = PaneTree {
        schema_version: PANE_TREE_SCHEMA_VERSION,
        tree_revision: 1,
        root: PaneNode::Leaf(leaf(
            "pane:only",
            SurfaceRole::Editor,
            SurfaceClass::TextEditor,
        )),
    };
    assert!(!tree.close_pane("pane:only"));
    assert_eq!(tree.pane_ids(), vec!["pane:only"]);
}

#[test]
fn detach_returns_leaf_preserving_pane_id() {
    let mut tree = sample_tree();
    let detached = tree.detach_pane("pane:b").expect("detached");
    assert_eq!(detached.pane_id, "pane:b");
    assert!(!tree.pane_ids().contains(&"pane:b".to_owned()));
}

#[test]
fn substitute_preserves_pane_id_and_slot() {
    let mut tree = sample_tree();
    let card = PlaceholderCard {
        reason: PlaceholderReason::MissingRemote,
        original_role: SurfaceRole::Terminal,
        original_surface_class: SurfaceClass::TerminalView,
        substitution_behavior: SubstitutionBehavior::PlaceholderSlotPreserved,
        safe_actions: vec![PlaceholderAction::ReconnectRemote],
        evidence_retained: true,
    };
    assert!(tree.substitute_placeholder("pane:c", card));
    assert!(tree.pane_ids().contains(&"pane:c".to_owned()));
    let pane = tree.find_leaf("pane:c").expect("pane preserved");
    assert!(pane.surface.is_placeholder());
}

#[test]
fn diff_reports_added_removed_and_retained() {
    let before = sample_tree();
    let mut after = sample_tree();
    after.close_pane("pane:c");
    after.split_pane(
        "pane:a",
        "split:a",
        SplitOrientation::Vertical,
        leaf("pane:d", SurfaceRole::AiPanel, SurfaceClass::AiPanel),
    );
    let diff = before.diff(&after);
    assert_eq!(diff.added, vec!["pane:d"]);
    assert_eq!(diff.removed, vec!["pane:c"]);
    assert!(diff.retained.contains(&"pane:a".to_owned()));
    assert!(diff.retained.contains(&"pane:b".to_owned()));
}

// --- Migration -----------------------------------------------------------------------------------

#[test]
fn migration_outcomes_map_to_restore_classes() {
    let tree = sample_tree();
    let (_, exact) = migrate_pane_tree(PANE_TREE_SCHEMA_VERSION, tree.clone());
    assert_eq!(exact, PaneTreeMigrationOutcome::Exact);
    assert_eq!(exact.restore_class(), RestoreClass::ExactRestore);

    let (_, forward) = migrate_pane_tree(0, tree.clone());
    assert_eq!(forward, PaneTreeMigrationOutcome::ForwardMigrated);

    let (kept, future) = migrate_pane_tree(PANE_TREE_SCHEMA_VERSION + 1, tree);
    assert_eq!(future, PaneTreeMigrationOutcome::Unmigratable);
    assert_eq!(future.restore_class(), RestoreClass::ManualReview);
    // An unreadable future version is left untouched rather than guessed at.
    assert_eq!(kept.schema_version, PANE_TREE_SCHEMA_VERSION);
}

// --- Fail-closed gate drills ---------------------------------------------------------------------

#[test]
fn live_authority_ticket_is_rejected() {
    let mut broken = packet();
    broken.workspace_authority_checkpoints[0].authority_handle_class =
        AuthorityHandleClass::LiveTicket;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateViolation::LiveAuthoritySerialized { .. }
    )));
}

#[test]
fn missing_live_authority_attestation_is_rejected() {
    let mut broken = packet();
    broken.workspace_authority_checkpoints[0].excludes_live_authority = false;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateViolation::LiveAuthoritySerialized { .. }
    )));
}

#[test]
fn machine_local_hints_marked_exportable_is_rejected() {
    let mut broken = packet();
    broken.machine_local_hints[0].exportable = true;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RememberedStateViolation::NonPortableExport { .. })));
}

#[test]
fn portable_profile_carrying_machine_local_state_is_rejected() {
    let mut broken = packet();
    broken.profile_defaults[0].ownership = StateOwnership::MachineLocal;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateViolation::MachineLocalAnchorInPortable { .. }
    )));
}

#[test]
fn silent_layout_delete_is_rejected() {
    let mut broken = packet();
    // Force the preview pane's placeholder to silently delete the slot.
    if let PaneNode::Split(split) = &mut broken.window_topology_snapshots[0].pane_tree.root {
        if let PaneNode::TabGroup(group) = &mut split.children[1] {
            let card = group.tabs[1]
                .pane
                .surface
                .placeholder
                .as_mut()
                .expect("placeholder");
            card.substitution_behavior = SubstitutionBehavior::SilentDelete;
        }
    }
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RememberedStateViolation::SilentLayoutDelete { .. })));
}

#[test]
fn flattened_authority_topology_is_rejected() {
    let mut broken = packet();
    // Point the bundle's authority ref at a window snapshot instead of a checkpoint.
    broken.bundles[0].workspace_authority_ref = "snap:primary-window".to_owned();
    let violations = broken.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5RememberedStateViolation::FlattenedAuthorityTopology { .. }
    )));
}

#[test]
fn dangling_snapshot_ref_is_rejected() {
    let mut broken = packet();
    broken.bundles[0]
        .window_snapshot_refs
        .push("snap:nonexistent".to_owned());
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RememberedStateViolation::DanglingRef { .. })));
}

#[test]
fn duplicate_pane_id_is_rejected() {
    let mut broken = packet();
    if let PaneNode::Split(split) = &mut broken.window_topology_snapshots[0].pane_tree.root {
        if let PaneNode::Leaf(l) = &mut split.children[0] {
            l.pane_id = "pane:docs".to_owned();
        }
    }
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RememberedStateViolation::DuplicatePaneId { .. })));
}

#[test]
fn bundle_overstating_fidelity_over_a_placeholder_is_rejected() {
    let mut broken = packet();
    broken.bundles[0].restore_class = RestoreClass::ExactRestore;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateViolation::BundleOverstatesFidelity { .. }
    )));
}

#[test]
fn placeholder_without_card_is_rejected() {
    let mut broken = packet();
    if let PaneNode::Split(split) = &mut broken.window_topology_snapshots[0].pane_tree.root {
        if let PaneNode::TabGroup(group) = &mut split.children[1] {
            group.tabs[1].pane.surface.placeholder = None;
        }
    }
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RememberedStateViolation::PlaceholderWithoutCard { .. })));
}

#[test]
fn schema_version_registry_mismatch_is_rejected() {
    let mut broken = packet();
    broken.object_schema_versions.pane_tree = 99;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RememberedStateViolation::SchemaVersionRegistryMismatch)));
}
