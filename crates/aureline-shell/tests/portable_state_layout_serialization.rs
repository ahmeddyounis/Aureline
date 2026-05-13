use aureline_shell::layout::serialization::{
    build_portable_state_package_from_split_tree, default_layout_preset_artifact,
    display_topology_adjustment, LayoutPortableStateRequest, LayoutSerializationError,
    PaneSurfaceSerialization, RememberedStateInspectorPanel, REMEMBERED_STATE_INSPECTOR_COMMAND_ID,
};
use aureline_shell::layout::split_tree::{SplitAxis, SplitTree};
use aureline_workspace::{
    DisplayAdjustmentClass, NoRerunGuardrail, PlaceholderAction, PlaceholderCard,
    PlaceholderReason, RestoreCandidateClass, SerializedStateClass, SurfaceRestorePosture,
};

fn four_pane_tree() -> SplitTree {
    let mut tree = SplitTree::single();
    let root = tree.root_leaf();
    let second = tree.split_leaf(root, SplitAxis::Vertical).unwrap();
    let third = tree.split_leaf(second, SplitAxis::Vertical).unwrap();
    tree.split_leaf(third, SplitAxis::Vertical).unwrap();
    tree
}

fn request_for(tree: &SplitTree) -> LayoutPortableStateRequest {
    let mut request = LayoutPortableStateRequest::new(
        "portable-state-alpha-package:shell-test",
        "portable-state-manifest:shell-test",
        "workspace:shell-test",
        "mono:9000:00:00:00.0000",
        "producer:aureline-shell-test",
        "mono:8999:23:59:59.0000",
        "window-topology-snapshot:shell-test",
        "machine-display-hints:shell-test",
    );
    request.linked_profile_artifacts = vec![default_layout_preset_artifact("workspace:shell-test")];

    let leaves = tree.leaf_ids_in_order();
    request.pane_surfaces = vec![
        PaneSurfaceSerialization::new(
            leaves[0],
            "editor",
            "text_editor",
            SurfaceRestorePosture::Live,
        ),
        PaneSurfaceSerialization::new(
            leaves[1],
            "terminal",
            "terminal_view",
            SurfaceRestorePosture::ContextOnly,
        )
        .with_placeholder(PlaceholderCard {
            reason: PlaceholderReason::NonReentrantLiveSurface,
            safe_actions: vec![
                PlaceholderAction::OpenContextOnly,
                PlaceholderAction::RerunExplicitly,
                PlaceholderAction::ExportEvidence,
            ],
            evidence_retained: true,
            last_known_label: Some("shell test terminal context".to_string()),
        })
        .with_guardrails(vec![
            NoRerunGuardrail::NoCommandRerun,
            NoRerunGuardrail::ExplicitUserActionRequired,
            NoRerunGuardrail::PlaceholderPreserved,
        ]),
        PaneSurfaceSerialization::new(
            leaves[2],
            "preview",
            "extension_view",
            SurfaceRestorePosture::PlaceholderOnly,
        )
        .with_placeholder(PlaceholderCard {
            reason: PlaceholderReason::MissingExtension,
            safe_actions: vec![
                PlaceholderAction::InstallExtension,
                PlaceholderAction::ExportEvidence,
                PlaceholderAction::RemovePane,
            ],
            evidence_retained: true,
            last_known_label: Some("shell test preview".to_string()),
        })
        .with_guardrails(vec![
            NoRerunGuardrail::NoPreviewServerRestart,
            NoRerunGuardrail::ExplicitUserActionRequired,
            NoRerunGuardrail::PlaceholderPreserved,
        ]),
        PaneSurfaceSerialization::new(
            leaves[3],
            "notebook",
            "notebook_view",
            SurfaceRestorePosture::PlaceholderOnly,
        )
        .with_placeholder(PlaceholderCard {
            reason: PlaceholderReason::MissingRemoteTarget,
            safe_actions: vec![
                PlaceholderAction::ReconnectRemote,
                PlaceholderAction::Reauthenticate,
                PlaceholderAction::ExportEvidence,
            ],
            evidence_retained: true,
            last_known_label: Some("shell test notebook".to_string()),
        })
        .with_guardrails(vec![
            NoRerunGuardrail::NoNotebookKernelRestart,
            NoRerunGuardrail::NoRemoteSessionResume,
            NoRerunGuardrail::ExplicitUserActionRequired,
            NoRerunGuardrail::PlaceholderPreserved,
        ]),
    ];
    request.topology_adjustments = vec![display_topology_adjustment(
        "display-adjustment:shell-test",
        DisplayAdjustmentClass::SnappedToSafeBounds,
        vec![
            "pane-0001".to_string(),
            "pane-0002".to_string(),
            "pane-0003".to_string(),
            "pane-0004".to_string(),
        ],
        RestoreCandidateClass::LayoutOnly,
        "The shell remapped the restored window into visible bounds.",
    )];
    request.restore_provenance_refs = vec!["layout-restore-provenance:shell-test".to_string()];
    request
}

#[test]
fn split_tree_serializes_to_valid_portable_state_package_and_inspector() {
    let tree = four_pane_tree();
    let package = build_portable_state_package_from_split_tree(&tree, request_for(&tree))
        .expect("shell serialization must build a valid package");

    let topology = package
        .state_classes
        .iter()
        .find(|row| row.class_kind == SerializedStateClass::WindowTopology)
        .expect("window topology row");
    assert_eq!(topology.pane_restore_postures.len(), 4);
    assert_eq!(
        topology.pane_restore_postures[0].stable_pane_id,
        "pane-0001"
    );
    assert_eq!(
        topology.pane_restore_postures[3].stable_pane_id,
        "pane-0004"
    );

    let panel = RememberedStateInspectorPanel::from_package(&package).expect("panel must build");
    let rendered = panel.render_lines().join("\n");
    assert_eq!(
        REMEMBERED_STATE_INSPECTOR_COMMAND_ID,
        "cmd:workspace.open_remembered_state_inspector"
    );
    assert!(rendered.contains("classification=portable"));
    assert!(rendered.contains("classification=machine_local"));
    assert!(rendered.contains("pane=pane-0002 posture=context_only"));
    assert!(rendered.contains("pane=pane-0003 posture=placeholder_only"));
    assert!(rendered.contains("actions=inspect,export,clear,compare"));
}

#[test]
fn split_tree_serialization_rejects_missing_surface_for_leaf() {
    let tree = four_pane_tree();
    let mut request = request_for(&tree);
    request.pane_surfaces.pop();

    let err = build_portable_state_package_from_split_tree(&tree, request)
        .expect_err("every split-tree leaf needs surface metadata");
    assert!(matches!(
        err,
        LayoutSerializationError::MissingPaneSurface { pane_id: 4 }
    ));
}
