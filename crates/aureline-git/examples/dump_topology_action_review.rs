//! Conformance dump for reviewed topology-remediation action sheets.
//!
//! Prints the canonical export-safe [`TopologyActionReviewPacket`] as
//! deterministic JSON. The optional first argument selects a narrowed fixture
//! variant, one per remediation verb plus the broadened and wrong-root previews:
//!
//! * (no argument) — the canonical action-review packet
//! * `widen` — a single sparse-slice widen sheet (local, no network)
//! * `deepen` — a single shallow-history deepen sheet (reviewed network fetch)
//! * `initialize` — a single uninitialized-submodule initialize sheet
//! * `hydrate` — a single pointer-only Git LFS hydrate sheet
//! * `multi-root` — a hydrate broadened across roots behind an explicit preview
//! * `wrong-root` — a widen targeting a root the caller has not selected
//!
//! The canonical document is the source of the checked-in artifact, and the
//! variants are the source of the protected `widen-deepen-initialize-hydrate`
//! fixtures.

use aureline_git::{
    CheckoutFilterClass, DepthBoundary, HistoryDepthClass, LfsObjectState, LfsState,
    MultiRootPreview, ObjectAvailability, OmittedPathSet, RepoIdentity, RepoIdentityKind,
    RepositoryTopologyClass, TopologyActionReviewPacket, TopologyActionSheet,
    TopologyActionSupportExport, TopologyHonestyLabel, TopologyOperationScope,
    TopologyRootDescriptor, WorktreeKind, WorktreeScope, GIT_TOPOLOGY_ROOT_DESCRIPTOR_RECORD_KIND,
    TOPOLOGY_ACTION_REQUIRED_RECONSTRUCTION_FIELDS, TOPOLOGY_ACTION_REVIEW_PACKET_RECORD_KIND,
    TOPOLOGY_ACTION_REVIEW_SCHEMA_VERSION, TOPOLOGY_ACTION_SUPPORT_EXPORT_RECORD_KIND,
};

const STAMP: &str = "2026-06-17T00:00:00Z";

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

fn base(root_id: &str, class: RepositoryTopologyClass) -> TopologyRootDescriptor {
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

fn sparse() -> TopologyRootDescriptor {
    let mut root = base("sparse", RepositoryTopologyClass::SparseCheckoutRoot);
    root.filter_class = CheckoutFilterClass::SparseCheckoutCone;
    root.omitted_paths = OmittedPathSet {
        path_refs: vec![
            "omitted-ref:vendor".to_owned(),
            "omitted-ref:docs".to_owned(),
        ],
        omitted_estimate: Some(412),
    };
    root.honesty_labels = vec![TopologyHonestyLabel::OutsideCurrentSlice];
    root
}

fn shallow() -> TopologyRootDescriptor {
    let mut root = base("shallow", RepositoryTopologyClass::ShallowHistoryRoot);
    root.depth_boundary = DepthBoundary {
        depth_class: HistoryDepthClass::ShallowDepth,
        shallow_boundary_refs: vec!["boundary-ref:grafted-head".to_owned()],
        configured_depth: Some(50),
    };
    root.honesty_labels = vec![TopologyHonestyLabel::ShallowBoundary];
    root
}

fn submodule() -> TopologyRootDescriptor {
    let mut root = base("submodule", RepositoryTopologyClass::SubmoduleRoot);
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

fn promisor() -> TopologyRootDescriptor {
    let mut root = base("partial", RepositoryTopologyClass::PartialClonePromisorRoot);
    root.filter_class = CheckoutFilterClass::PartialCloneBlobless;
    root.object_availability = ObjectAvailability::MissingUnfetched;
    root.honesty_labels = vec![TopologyHonestyLabel::NotFetched];
    root
}

fn lfs() -> TopologyRootDescriptor {
    let mut root = base("lfs", RepositoryTopologyClass::LfsHydrationBoundary);
    root.lfs = LfsState {
        state: LfsObjectState::PointerOnly,
        pointer_path_refs: vec!["pointer-ref:assets/model.bin".to_owned()],
    };
    root.honesty_labels = vec![TopologyHonestyLabel::PointerOnly];
    root.safe_operation_scope = TopologyOperationScope::MetadataOnly;
    root
}

fn sheet(root: &TopologyRootDescriptor, active: &str) -> TopologyActionSheet {
    let id = format!(
        "sheet-{}-{}-active-{active}",
        TopologyActionSheet::for_descriptor(root, active, "probe")
            .map(|s| s.action_kind.as_str())
            .unwrap_or("none"),
        root.root_id
    );
    TopologyActionSheet::for_descriptor(root, active, id)
        .expect("partial descriptor yields a remediation sheet")
}

/// A hydrate sheet broadened across roots behind an explicit multi-root preview.
fn broadened_hydrate() -> TopologyActionSheet {
    let mut sheet = sheet(&promisor(), "partial");
    sheet.sheet_id = "sheet-hydrate-partial-multi-root".to_owned();
    sheet.multi_root_preview = MultiRootPreview {
        broadened: true,
        additional_root_refs: vec![
            "partial-sibling-a".to_owned(),
            "partial-sibling-b".to_owned(),
        ],
        preview_ref: "preview-ref:partial-fan-out".to_owned(),
    };
    sheet.safe_operation_scope = TopologyOperationScope::ExplicitMultiRootPreviewRequired;
    sheet
}

/// A widen sheet whose target root is not the caller's active root.
fn wrong_root_widen() -> TopologyActionSheet {
    let mut sheet = sheet(&sparse(), "main");
    sheet.sheet_id = "sheet-widen-sparse-wrong-root".to_owned();
    sheet
}

fn support_export(sheets: &[TopologyActionSheet]) -> TopologyActionSupportExport {
    TopologyActionSupportExport {
        record_kind: TOPOLOGY_ACTION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: "topology-action-review-export:0001".to_owned(),
        sheet_refs: sheets.iter().map(|sheet| sheet.sheet_id.clone()).collect(),
        action_kinds: sheets.iter().map(|sheet| sheet.action_kind).collect(),
        reconstruction_fields: TOPOLOGY_ACTION_REQUIRED_RECONSTRUCTION_FIELDS
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        raw_paths_redacted: true,
        raw_object_bytes_redacted: true,
    }
}

fn packet(packet_id: &str, sheets: Vec<TopologyActionSheet>) -> TopologyActionReviewPacket {
    let support_export = support_export(&sheets);
    TopologyActionReviewPacket {
        record_kind: TOPOLOGY_ACTION_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: TOPOLOGY_ACTION_REVIEW_SCHEMA_VERSION,
        packet_id: packet_id.to_owned(),
        generated_at: STAMP.to_owned(),
        sheets,
        support_export,
    }
}

fn canonical_packet() -> TopologyActionReviewPacket {
    packet(
        "topology-action-review:0001",
        vec![
            sheet(&sparse(), "sparse"),
            sheet(&shallow(), "shallow"),
            sheet(&submodule(), "submodule"),
            sheet(&promisor(), "partial"),
            sheet(&lfs(), "lfs"),
            broadened_hydrate(),
            wrong_root_widen(),
        ],
    )
}

fn single(packet_id: &str, sheet: TopologyActionSheet) -> TopologyActionReviewPacket {
    packet(packet_id, vec![sheet])
}

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_default();
    let packet = match variant.as_str() {
        "widen" => single(
            "topology-action-review:widen:0001",
            sheet(&sparse(), "sparse"),
        ),
        "deepen" => single(
            "topology-action-review:deepen:0001",
            sheet(&shallow(), "shallow"),
        ),
        "initialize" => single(
            "topology-action-review:initialize:0001",
            sheet(&submodule(), "submodule"),
        ),
        "hydrate" => single("topology-action-review:hydrate:0001", sheet(&lfs(), "lfs")),
        "multi-root" => single(
            "topology-action-review:multi-root:0001",
            broadened_hydrate(),
        ),
        "wrong-root" => single("topology-action-review:wrong-root:0001", wrong_root_widen()),
        _ => canonical_packet(),
    };
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "action review packet invalid: {violations:?}"
    );
    if std::env::args().any(|arg| arg == "--markdown") {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
