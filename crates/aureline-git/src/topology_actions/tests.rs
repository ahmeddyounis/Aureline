//! Inline coverage for reviewed topology-remediation sheets.

use super::*;
use crate::stabilize_repository_topology_truth::{
    RepositoryTopologyClass, SurfaceResultTruth, TopologyActionApproval, TopologyActionClass,
    TopologyHonestyLabel, TopologyOperationScope,
};
use crate::topology::{
    CheckoutFilterClass, DepthBoundary, GeneratedVendorClass, GeneratedVendorOrigin,
    HistoryDepthClass, LfsObjectState, LfsState, ObjectAvailability, OmittedPathSet, RepoIdentity,
    RepoIdentityKind, TopologyRootDescriptor, WorktreeKind, WorktreeScope,
    GIT_TOPOLOGY_ROOT_DESCRIPTOR_RECORD_KIND,
};

fn standalone(root_id: &str) -> RepoIdentity {
    RepoIdentity {
        kind: RepoIdentityKind::Standalone,
        root_id: root_id.to_owned(),
        parent_root_id: None,
        gitlink_path_ref: None,
        pinned_commit_ref: None,
        child_initialized: true,
    }
}

fn primary_worktree(root_id: &str) -> WorktreeScope {
    WorktreeScope {
        kind: WorktreeKind::Primary,
        worktree_root_ref: format!("worktree-ref:{root_id}"),
        common_dir_ref: None,
        locked: false,
    }
}

/// Minimal descriptor builder defaulting every structured field to "complete".
fn descriptor(root_id: &str, class: RepositoryTopologyClass) -> TopologyRootDescriptor {
    TopologyRootDescriptor {
        record_kind: GIT_TOPOLOGY_ROOT_DESCRIPTOR_RECORD_KIND.to_owned(),
        root_id: root_id.to_owned(),
        root_path_ref: format!("path-ref:{root_id}"),
        topology_classes: vec![class],
        repo_identity: standalone(root_id),
        worktree: primary_worktree(root_id),
        filter_class: CheckoutFilterClass::FullCheckout,
        omitted_paths: OmittedPathSet::none(),
        depth_boundary: DepthBoundary::full(),
        object_availability: ObjectAvailability::FullyHydrated,
        lfs: LfsState::not_applicable(),
        generated_vendor: None,
        honesty_labels: vec![],
        safe_operation_scope: TopologyOperationScope::ActiveRootOnly,
    }
}

fn sparse_descriptor() -> TopologyRootDescriptor {
    let mut root = descriptor("sparse", RepositoryTopologyClass::SparseCheckoutRoot);
    root.filter_class = CheckoutFilterClass::SparseCheckoutCone;
    root.omitted_paths = OmittedPathSet {
        path_refs: vec!["omitted-ref:vendor".to_owned()],
        omitted_estimate: Some(120),
    };
    root.honesty_labels = vec![TopologyHonestyLabel::OutsideCurrentSlice];
    root
}

fn shallow_descriptor() -> TopologyRootDescriptor {
    let mut root = descriptor("shallow", RepositoryTopologyClass::ShallowHistoryRoot);
    root.depth_boundary = DepthBoundary {
        depth_class: HistoryDepthClass::ShallowDepth,
        shallow_boundary_refs: vec!["boundary-ref:edge".to_owned()],
        configured_depth: Some(50),
    };
    root.honesty_labels = vec![TopologyHonestyLabel::ShallowBoundary];
    root
}

fn submodule_descriptor() -> TopologyRootDescriptor {
    let mut root = descriptor("submodule", RepositoryTopologyClass::SubmoduleRoot);
    root.repo_identity = RepoIdentity {
        kind: RepoIdentityKind::SubmoduleChild,
        root_id: "submodule".to_owned(),
        parent_root_id: Some("main".to_owned()),
        gitlink_path_ref: Some("gitlink-ref:libs/widget".to_owned()),
        pinned_commit_ref: Some("pin-ref:0".to_owned()),
        child_initialized: false,
    };
    root.honesty_labels = vec![TopologyHonestyLabel::SubmoduleNotInitialized];
    root.safe_operation_scope = TopologyOperationScope::MutationDenied;
    root
}

fn promisor_descriptor() -> TopologyRootDescriptor {
    let mut root = descriptor("partial", RepositoryTopologyClass::PartialClonePromisorRoot);
    root.filter_class = CheckoutFilterClass::PartialCloneBlobless;
    root.object_availability = ObjectAvailability::MissingUnfetched;
    root.honesty_labels = vec![TopologyHonestyLabel::NotFetched];
    root
}

fn lfs_descriptor() -> TopologyRootDescriptor {
    let mut root = descriptor("lfs", RepositoryTopologyClass::LfsHydrationBoundary);
    root.lfs = LfsState {
        state: LfsObjectState::PointerOnly,
        pointer_path_refs: vec!["pointer-ref:assets/model.bin".to_owned()],
    };
    root.honesty_labels = vec![TopologyHonestyLabel::PointerOnly];
    root.safe_operation_scope = TopologyOperationScope::MetadataOnly;
    root
}

fn valid_packet(sheets: Vec<TopologyActionSheet>) -> TopologyActionReviewPacket {
    let support_export = TopologyActionSupportExport {
        record_kind: TOPOLOGY_ACTION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: "topology-action-export:test".to_owned(),
        sheet_refs: sheets.iter().map(|sheet| sheet.sheet_id.clone()).collect(),
        action_kinds: sheets.iter().map(|sheet| sheet.action_kind).collect(),
        reconstruction_fields: TOPOLOGY_ACTION_REQUIRED_RECONSTRUCTION_FIELDS
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        raw_paths_redacted: true,
        raw_object_bytes_redacted: true,
    };
    TopologyActionReviewPacket {
        record_kind: TOPOLOGY_ACTION_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: TOPOLOGY_ACTION_REVIEW_SCHEMA_VERSION,
        packet_id: "topology-action-review:test".to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        sheets,
        support_export,
    }
}

#[test]
fn each_partial_state_derives_its_distinct_action() {
    let cases = [
        (sparse_descriptor(), TopologyActionKind::Widen),
        (shallow_descriptor(), TopologyActionKind::Deepen),
        (submodule_descriptor(), TopologyActionKind::Initialize),
        (promisor_descriptor(), TopologyActionKind::Hydrate),
        (lfs_descriptor(), TopologyActionKind::Hydrate),
    ];
    for (root, expected) in cases {
        let sheet = TopologyActionSheet::for_descriptor(&root, &root.root_id, "sheet")
            .expect("partial root yields a remediation");
        assert_eq!(sheet.action_kind, expected);
        let packet = valid_packet(vec![sheet]);
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}

#[test]
fn complete_and_generated_roots_yield_no_action() {
    let complete = descriptor("main", RepositoryTopologyClass::CurrentRepoRoot);
    assert!(TopologyActionSheet::for_descriptor(&complete, "main", "sheet").is_none());

    let mut generated = descriptor("generated", RepositoryTopologyClass::GeneratedVendorRoot);
    generated.generated_vendor = Some(GeneratedVendorOrigin {
        class: GeneratedVendorClass::Generated,
        origin_ref: "origin-ref:codegen".to_owned(),
        editable_truth: false,
    });
    generated.honesty_labels = vec![TopologyHonestyLabel::GeneratedOrExcluded];
    generated.safe_operation_scope = TopologyOperationScope::MutationDenied;
    assert!(TopologyActionSheet::for_descriptor(&generated, "generated", "sheet").is_none());
}

#[test]
fn widen_is_local_and_needs_no_network_approval() {
    let root = sparse_descriptor();
    let sheet = TopologyActionSheet::for_descriptor(&root, &root.root_id, "sheet").unwrap();
    assert!(!sheet.network.reaches_network);
    assert_eq!(sheet.approval, TopologyActionApproval::NotNetworkBearing);
    assert_eq!(sheet.provider_auth, ProviderAuthPosture::NoAuthRequired);
    assert!(sheet.is_executable());
}

#[test]
fn network_actions_stay_reviewed_and_attributable() {
    for root in [
        promisor_descriptor(),
        shallow_descriptor(),
        submodule_descriptor(),
    ] {
        let sheet = TopologyActionSheet::for_descriptor(&root, &root.root_id, "sheet").unwrap();
        assert!(sheet.action_kind.is_network_bearing());
        assert!(sheet.network.reaches_network);
        assert!(sheet.network.egress_ref.is_some());
        assert_eq!(sheet.approval, TopologyActionApproval::ApprovalRequired);
        assert!(!sheet.recovery.recovery_ref.is_empty());
        // Reviewed: not executable until approved.
        assert!(!sheet.is_executable());
    }
}

#[test]
fn hydrate_distinguishes_pointer_from_promisor_targets() {
    let lfs = TopologyActionSheet::for_descriptor(&lfs_descriptor(), "lfs", "sheet").unwrap();
    assert_eq!(lfs.action_class, TopologyActionClass::HydrateLfsObjects);
    assert_eq!(
        lfs.selector.target_kind,
        TopologyTargetKind::PointerBackedAsset
    );

    let promisor =
        TopologyActionSheet::for_descriptor(&promisor_descriptor(), "partial", "sheet").unwrap();
    assert_eq!(
        promisor.action_class,
        TopologyActionClass::FetchMissingObjects
    );
    assert_eq!(
        promisor.selector.target_kind,
        TopologyTargetKind::PromisorRemote
    );
}

#[test]
fn wrong_root_target_is_guarded_and_denied() {
    // A caller active on "main" cannot widen a sparse slice owned by "sparse"
    // without an explicit broadening preview.
    let root = sparse_descriptor();
    let sheet = TopologyActionSheet::for_descriptor(&root, "main", "sheet").unwrap();
    assert_eq!(
        sheet.wrong_root_guard,
        WrongRootGuard::RetargetRequiredWrongRoot
    );
    assert_eq!(
        sheet.safe_operation_scope,
        TopologyOperationScope::MutationDenied
    );
    assert_ne!(sheet.approval, TopologyActionApproval::Approved);
    assert!(!sheet.is_executable());
    assert_eq!(
        sheet.object_scope.post_action_truth,
        SurfaceResultTruth::WrongTargetRoot
    );
    let packet = valid_packet(vec![sheet]);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn nested_wrong_root_blocks_with_child_root_scope() {
    let mut nested = descriptor("nested", RepositoryTopologyClass::NestedIndependentRepoRoot);
    nested.repo_identity = RepoIdentity {
        kind: RepoIdentityKind::NestedIndependent,
        root_id: "nested".to_owned(),
        parent_root_id: Some("main".to_owned()),
        gitlink_path_ref: None,
        pinned_commit_ref: None,
        child_initialized: true,
    };
    // Make the nested root itself partial so it has a remediation to offer.
    nested.object_availability = ObjectAvailability::MissingUnfetched;
    nested.filter_class = CheckoutFilterClass::PartialCloneBlobless;
    nested.honesty_labels = vec![
        TopologyHonestyLabel::NotFetched,
        TopologyHonestyLabel::NestedRepoBoundary,
    ];

    let sheet = TopologyActionSheet::for_descriptor(&nested, "main", "sheet").unwrap();
    assert_eq!(
        sheet.wrong_root_guard,
        WrongRootGuard::BlockedNestedBoundary
    );
    assert_eq!(
        sheet.safe_operation_scope,
        TopologyOperationScope::ChildRootOnly
    );
    assert_eq!(sheet.approval, TopologyActionApproval::PolicyBlocked);
    let packet = valid_packet(vec![sheet]);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn broadened_action_requires_named_roots_and_preview_scope() {
    let root = promisor_descriptor();
    let mut sheet = TopologyActionSheet::for_descriptor(&root, "partial", "sheet").unwrap();
    sheet.multi_root_preview = MultiRootPreview {
        broadened: true,
        additional_root_refs: vec!["sibling".to_owned()],
        preview_ref: "preview-ref:multi".to_owned(),
    };
    sheet.safe_operation_scope = TopologyOperationScope::ExplicitMultiRootPreviewRequired;
    let packet = valid_packet(vec![sheet.clone()]);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    // Broadening without naming roots is rejected.
    let mut bad = sheet.clone();
    bad.multi_root_preview.additional_root_refs.clear();
    let packet = valid_packet(vec![bad]);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        TopologyActionValidationError::BroadenedPreviewMissingRoots { .. }
    )));

    // Broadening without the preview-required scope is rejected.
    let mut bad = sheet;
    bad.safe_operation_scope = TopologyOperationScope::ActiveRootOnly;
    let packet = valid_packet(vec![bad]);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        TopologyActionValidationError::PreviewScopeMismatch { .. }
    )));
}

#[test]
fn silent_background_fetch_is_rejected() {
    // A network-bearing sheet that drops its approval posture is a silent fetch.
    let root = promisor_descriptor();
    let mut sheet = TopologyActionSheet::for_descriptor(&root, "partial", "sheet").unwrap();
    sheet.approval = TopologyActionApproval::NotNetworkBearing;
    let packet = valid_packet(vec![sheet]);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        TopologyActionValidationError::NetworkMissingApproval { .. }
    )));
}

#[test]
fn action_class_must_match_verb() {
    let root = sparse_descriptor();
    let mut sheet = TopologyActionSheet::for_descriptor(&root, "sparse", "sheet").unwrap();
    sheet.action_class = TopologyActionClass::DeepenHistory;
    let packet = valid_packet(vec![sheet]);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        TopologyActionValidationError::ActionClassMismatch { .. }
    )));
}

#[test]
fn packet_round_trips_through_json() {
    let sheets: Vec<_> = [
        sparse_descriptor(),
        shallow_descriptor(),
        submodule_descriptor(),
        promisor_descriptor(),
        lfs_descriptor(),
    ]
    .into_iter()
    .map(|root| {
        TopologyActionSheet::for_descriptor(&root, &root.root_id, format!("sheet-{}", root.root_id))
            .unwrap()
    })
    .collect();
    let packet = valid_packet(sheets);
    let json = packet.export_safe_json();
    let parsed = TopologyActionReviewPacket::parse_json(&json).expect("round-trips");
    assert_eq!(parsed, packet);
}
