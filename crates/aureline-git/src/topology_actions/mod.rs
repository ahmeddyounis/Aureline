//! Reviewed topology-remediation actions and worktree/root selectors.
//!
//! Repository topology only helps users if they can act on it safely. Where the
//! [`crate::topology`] descriptors record *why* a surface answer is partial, this
//! module turns each partial state into an explicit, reviewed **remediation
//! action**: widen a sparse/workset slice, deepen shallow history, initialize a
//! submodule child, or hydrate pointer-only/unfetched objects. Each action is a
//! [`TopologyActionSheet`] that names exactly one target ([`ActionTargetSelector`]),
//! discloses object scope, network side effects, provider/auth posture,
//! review/export parity, and recovery before it can mutate anything, and carries
//! an explicit approval posture so a network-bearing action can never become a
//! silent background fetch.
//!
//! Three invariants hold across every sheet:
//!
//! * **One target unless explicitly broadened.** A sheet scopes to a single
//!   selected root/worktree. A broad action that touches more than one root must
//!   carry a [`MultiRootPreview`] that names every additional root; otherwise the
//!   safe scope stays [`TopologyOperationScope::ActiveRootOnly`].
//! * **No wrong root.** When a caller's active root is not the root that owns the
//!   targeted object, the [`WrongRootGuard`] blocks the action and demands an
//!   explicit retarget or child-root open instead of flattening two roots.
//! * **Network stays reviewed and attributable.** Deepen, initialize, and hydrate
//!   reach the network; they always carry an approval posture and an egress and
//!   recovery reference, and never auto-execute.
//!
//! The same [`TopologyRootDescriptor`] that drives the read surfaces also drives
//! these actions: [`TopologyActionSheet::for_descriptor`] derives the single
//! remediation a root's structured state calls for, so search, review, blame, and
//! AI lanes recommend the *same* reviewed action rather than inventing ambient
//! bulk operations.
//!
//! The boundary schema is
//! [`schemas/git/topology_action_review.schema.json`](../../../../schemas/git/topology_action_review.schema.json).
//! The protected fixture corpus is
//! [`fixtures/git/m5/widen-deepen-initialize-hydrate/`](../../../../fixtures/git/m5/widen-deepen-initialize-hydrate/).
//! The checked-in canonical packet is
//! [`artifacts/git/m5/git_topology/topology_action_review.json`](../../../../artifacts/git/m5/git_topology/topology_action_review.json).

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_repository_topology_truth::{
    RepositoryTopologyClass, SurfaceResultTruth, TopologyActionApproval, TopologyActionClass,
    TopologyHonestyLabel, TopologyOperationScope,
};
use crate::topology::{
    HistoryDepthClass, LfsObjectState, ObjectAvailability, RepoIdentityKind, TopologyRootDescriptor,
};

/// Schema version for [`TopologyActionReviewPacket`].
pub const TOPOLOGY_ACTION_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`TopologyActionReviewPacket`].
pub const TOPOLOGY_ACTION_REVIEW_PACKET_RECORD_KIND: &str = "topology_action_review_packet";

/// Stable record-kind tag carried by [`TopologyActionSheet`].
pub const TOPOLOGY_ACTION_SHEET_RECORD_KIND: &str = "topology_action_sheet";

/// Stable record-kind tag carried by [`TopologyActionSupportExport`].
pub const TOPOLOGY_ACTION_SUPPORT_EXPORT_RECORD_KIND: &str = "topology_action_support_export";

/// Repo-relative path of the boundary schema.
pub const TOPOLOGY_ACTION_REVIEW_SCHEMA_REF: &str =
    "schemas/git/topology_action_review.schema.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const TOPOLOGY_ACTION_REVIEW_FIXTURE_DIR: &str =
    "fixtures/git/m5/widen-deepen-initialize-hydrate";

/// Repo-relative path of the checked-in canonical action-review packet.
pub const TOPOLOGY_ACTION_REVIEW_ARTIFACT_REF: &str =
    "artifacts/git/m5/git_topology/topology_action_review.json";

/// Reconstruction fields a support export must retain after redaction.
pub const TOPOLOGY_ACTION_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 6] = [
    "action_kind",
    "target_kind",
    "target_root_ref",
    "active_root_ref",
    "approval",
    "network_reaches",
];

/// One of the four distinct reviewed remediation actions.
///
/// These are mutually exclusive verbs, not a single generic "resolve": a user
/// always knows whether they are widening a slice, deepening history,
/// initializing a child checkout, or hydrating object content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyActionKind {
    /// Widen a sparse checkout or workset slice so omitted paths become visible.
    Widen,
    /// Deepen shallow or grafted history so blame and log reach further back.
    Deepen,
    /// Initialize an uninitialized submodule child checkout.
    Initialize,
    /// Hydrate pointer-only Git LFS objects or fetch unfetched promisor objects.
    Hydrate,
}

impl TopologyActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 4] = [Self::Widen, Self::Deepen, Self::Initialize, Self::Hydrate];

    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Widen => "widen",
            Self::Deepen => "deepen",
            Self::Initialize => "initialize",
            Self::Hydrate => "hydrate",
        }
    }

    /// Whether this action can reach the network or materialize remote content.
    ///
    /// Widening a slice rewrites the local working tree only; deepening,
    /// initializing, and hydrating all reach a remote and must stay reviewed.
    pub const fn is_network_bearing(self) -> bool {
        matches!(self, Self::Deepen | Self::Initialize | Self::Hydrate)
    }

    /// Whether the given [`TopologyActionClass`] is a valid realization of this
    /// action kind. [`TopologyActionKind::Hydrate`] covers both the Git LFS and
    /// the promisor-fetch classes.
    pub fn accepts_class(self, class: TopologyActionClass) -> bool {
        match self {
            Self::Widen => class == TopologyActionClass::WidenWorksetScope,
            Self::Deepen => class == TopologyActionClass::DeepenHistory,
            Self::Initialize => class == TopologyActionClass::InitializeSubmodule,
            Self::Hydrate => matches!(
                class,
                TopologyActionClass::HydrateLfsObjects | TopologyActionClass::FetchMissingObjects
            ),
        }
    }
}

/// Structural object a remediation action targets.
///
/// A user can always tell, before commit, whether an action reaches a parent
/// repo, a child repo, a worktree, a sparse slice, a promisor remote, or a
/// pointer-backed asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyTargetKind {
    /// The parent repository root in a parent/child topology.
    ParentRepo,
    /// A submodule or nested child repository root.
    ChildRepo,
    /// A linked or primary worktree root.
    Worktree,
    /// A sparse-checkout or workset slice.
    SparseSlice,
    /// Shallow or grafted history bounded by clone depth.
    ShallowHistory,
    /// A promisor remote backing unfetched objects.
    PromisorRemote,
    /// A Git LFS pointer-backed asset.
    PointerBackedAsset,
}

impl TopologyTargetKind {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParentRepo => "parent_repo",
            Self::ChildRepo => "child_repo",
            Self::Worktree => "worktree",
            Self::SparseSlice => "sparse_slice",
            Self::ShallowHistory => "shallow_history",
            Self::PromisorRemote => "promisor_remote",
            Self::PointerBackedAsset => "pointer_backed_asset",
        }
    }

    /// Whether this target kind is a coherent target for the given action kind.
    pub fn fits_action(self, action: TopologyActionKind) -> bool {
        match action {
            TopologyActionKind::Widen => {
                matches!(self, Self::SparseSlice | Self::ParentRepo | Self::Worktree)
            }
            TopologyActionKind::Deepen => self == Self::ShallowHistory,
            TopologyActionKind::Initialize => self == Self::ChildRepo,
            TopologyActionKind::Hydrate => {
                matches!(self, Self::PointerBackedAsset | Self::PromisorRemote)
            }
        }
    }
}

/// Whether the active root matches the root that owns the targeted object.
///
/// This is the no-wrong-root safeguard: a remediation action against a root other
/// than the caller's active root is never silently widened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WrongRootGuard {
    /// The active root owns the target; the action is in scope.
    TargetMatchesAuthoritativeRoot,
    /// The active root is not the target's root; the user must retarget first.
    RetargetRequiredWrongRoot,
    /// The target sits in a nested independent root that must be opened explicitly.
    BlockedNestedBoundary,
}

impl WrongRootGuard {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetMatchesAuthoritativeRoot => "target_matches_authoritative_root",
            Self::RetargetRequiredWrongRoot => "retarget_required_wrong_root",
            Self::BlockedNestedBoundary => "blocked_nested_boundary",
        }
    }

    /// Whether this guard blocks the action from executing as targeted.
    pub const fn blocks(self) -> bool {
        !matches!(self, Self::TargetMatchesAuthoritativeRoot)
    }
}

/// Provider and authentication posture a network-bearing action discloses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderAuthPosture {
    /// No network or provider credential is required.
    NoAuthRequired,
    /// An existing authenticated remote session is reused.
    ExistingCredentialReused,
    /// The user must satisfy an authentication challenge first.
    AuthChallengeRequired,
    /// Policy blocks reaching the provider for this object.
    PolicyBlocked,
}

impl ProviderAuthPosture {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAuthRequired => "no_auth_required",
            Self::ExistingCredentialReused => "existing_credential_reused",
            Self::AuthChallengeRequired => "auth_challenge_required",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Recovery class for a reviewed remediation action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyRecoveryClass {
    /// The action changes no committed state; it is reversible by re-narrowing.
    NoMutationReversible,
    /// Fetched objects can be dropped again; the remote stays the source of truth.
    RemoteRefetchable,
    /// An initialized submodule can be deinitialized back to its pin.
    SubmoduleDeinitable,
    /// Hydrated pointer content can be restored to pointer-only.
    PointerRestorable,
}

impl TopologyRecoveryClass {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutationReversible => "no_mutation_reversible",
            Self::RemoteRefetchable => "remote_refetchable",
            Self::SubmoduleDeinitable => "submodule_deinitable",
            Self::PointerRestorable => "pointer_restorable",
        }
    }
}

/// Single-target selector naming exactly what a remediation action reaches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionTargetSelector {
    /// Structural kind of the target.
    pub target_kind: TopologyTargetKind,
    /// Stable id of the root that owns the targeted object.
    pub target_root_ref: String,
    /// Worktree ref, when the target is a worktree.
    pub worktree_ref: Option<String>,
    /// Topology class the target belongs to.
    pub topology_class: RepositoryTopologyClass,
    /// Redaction-safe ref to the omitted/unfetched/pointer/shallow/gitlink scope.
    pub scope_object_ref: String,
}

/// Explicit broadening of a single-target action to additional roots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiRootPreview {
    /// Whether the action broadens beyond its single target root.
    pub broadened: bool,
    /// Stable ids of every additional root the broadened action touches.
    pub additional_root_refs: Vec<String>,
    /// Redaction-safe ref to the preview evidence shown before commit.
    pub preview_ref: String,
}

impl MultiRootPreview {
    /// A preview that does not broaden beyond the single target root.
    pub fn single_root() -> Self {
        Self {
            broadened: false,
            additional_root_refs: Vec::new(),
            preview_ref: "preview-ref:single-root".to_owned(),
        }
    }
}

/// Object-scope disclosure: what the action materializes and the truth it repairs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectScopeDisclosure {
    /// Redaction-safe ref summarizing the affected object scope.
    pub scope_summary_ref: String,
    /// Estimated affected object count, for an honest preview.
    pub affected_object_estimate: Option<u64>,
    /// Result truth a surface renders before the action runs (never complete).
    pub pre_action_truth: SurfaceResultTruth,
    /// Result truth a surface would render after the action completes in scope.
    pub post_action_truth: SurfaceResultTruth,
}

/// Network side-effect disclosure for a remediation action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSideEffect {
    /// Whether the action reaches the network.
    pub reaches_network: bool,
    /// Redaction-safe ref to the egress target, when the action reaches it.
    pub egress_ref: Option<String>,
    /// Estimated bytes transferred, when known.
    pub estimated_transfer_bytes: Option<u64>,
}

/// Whether the action and its disclosures appear identically in review and export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewExportParity {
    /// Whether the action is disclosed in review surfaces.
    pub review_visible: bool,
    /// Whether the action is disclosed in redaction-safe support/export.
    pub export_visible: bool,
    /// Whether materialized object body bytes may be embedded in an export.
    pub body_export_allowed: bool,
    /// Redaction-safe ref noting the review/export parity evidence.
    pub parity_note_ref: String,
}

/// Recovery disclosure shown before a remediation action mutates state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryDisclosure {
    /// Whether the action is reversible.
    pub reversible: bool,
    /// Recovery class.
    pub recovery_class: TopologyRecoveryClass,
    /// Redaction-safe ref to the recovery path or checkpoint.
    pub recovery_ref: String,
}

/// One reviewed remediation sheet for a single topology caveat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyActionSheet {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable sheet id.
    pub sheet_id: String,
    /// The reviewed action verb.
    pub action_kind: TopologyActionKind,
    /// The concrete action class realizing the verb.
    pub action_class: TopologyActionClass,
    /// Single-target selector.
    pub selector: ActionTargetSelector,
    /// Root the caller has active when the sheet is offered.
    pub active_root_ref: String,
    /// Object-scope disclosure.
    pub object_scope: ObjectScopeDisclosure,
    /// Network side-effect disclosure.
    pub network: NetworkSideEffect,
    /// Provider/auth posture.
    pub provider_auth: ProviderAuthPosture,
    /// Review/export parity disclosure.
    pub review_export: ReviewExportParity,
    /// Recovery disclosure.
    pub recovery: RecoveryDisclosure,
    /// Explicit broadening preview, when the action touches more than one root.
    pub multi_root_preview: MultiRootPreview,
    /// Approval posture before execution.
    pub approval: TopologyActionApproval,
    /// Safe operation scope this sheet permits.
    pub safe_operation_scope: TopologyOperationScope,
    /// No-wrong-root guard state.
    pub wrong_root_guard: WrongRootGuard,
    /// Honesty labels the sheet carries.
    pub honesty_labels: Vec<TopologyHonestyLabel>,
}

impl TopologyActionSheet {
    /// Derives the single remediation sheet a root's structured state calls for,
    /// for a caller whose active root is `active_root_ref`.
    ///
    /// Returns [`None`] when the root needs no remediation (a complete root, or a
    /// generated/vendor root intentionally outside editable truth). The priority
    /// order mirrors [`TopologyRootDescriptor::result_truth_for`]: pointer-only,
    /// then unfetched, then uninitialized child, then shallow history, then
    /// omitted slice.
    ///
    /// When `active_root_ref` is not the descriptor's own root, the derived sheet
    /// is wrong-root guarded: it is denied and never approved, so a search,
    /// review, blame, or AI lane cannot widen an action onto a root the caller did
    /// not select.
    pub fn for_descriptor(
        descriptor: &TopologyRootDescriptor,
        active_root_ref: &str,
        sheet_id: impl Into<String>,
    ) -> Option<Self> {
        let (action_kind, action_class, target_kind, pre_truth, label, recovery_class) =
            classify_remediation(descriptor)?;

        let target_root_ref = descriptor.root_id.clone();
        let wrong_root = active_root_ref != target_root_ref;
        let nested = descriptor.repo_identity.kind == RepoIdentityKind::NestedIndependent;

        let wrong_root_guard = if !wrong_root {
            WrongRootGuard::TargetMatchesAuthoritativeRoot
        } else if nested {
            WrongRootGuard::BlockedNestedBoundary
        } else {
            WrongRootGuard::RetargetRequiredWrongRoot
        };

        let reaches_network = action_kind.is_network_bearing();
        let network = NetworkSideEffect {
            reaches_network,
            egress_ref: reaches_network.then(|| format!("egress-ref:{target_root_ref}")),
            estimated_transfer_bytes: None,
        };

        let provider_auth = if !reaches_network {
            ProviderAuthPosture::NoAuthRequired
        } else {
            ProviderAuthPosture::ExistingCredentialReused
        };

        // A blocked wrong-root sheet can never be pre-approved; network sheets stay
        // approval-gated; a local widen needs no network approval.
        let approval = if wrong_root_guard.blocks() {
            if nested {
                TopologyActionApproval::PolicyBlocked
            } else {
                TopologyActionApproval::ApprovalRequired
            }
        } else if reaches_network {
            TopologyActionApproval::ApprovalRequired
        } else {
            TopologyActionApproval::NotNetworkBearing
        };

        let safe_operation_scope = if wrong_root_guard.blocks() {
            if nested {
                TopologyOperationScope::ChildRootOnly
            } else {
                TopologyOperationScope::MutationDenied
            }
        } else {
            TopologyOperationScope::ActiveRootOnly
        };

        let post_action_truth = if wrong_root_guard.blocks() {
            if nested {
                SurfaceResultTruth::NestedRoot
            } else {
                SurfaceResultTruth::WrongTargetRoot
            }
        } else {
            SurfaceResultTruth::Complete
        };

        let mut honesty_labels = vec![label];
        if wrong_root_guard.blocks() {
            let extra = if nested {
                TopologyHonestyLabel::NestedRepoBoundary
            } else {
                TopologyHonestyLabel::WrongTargetRoot
            };
            if !honesty_labels.contains(&extra) {
                honesty_labels.push(extra);
            }
        }

        Some(Self {
            record_kind: TOPOLOGY_ACTION_SHEET_RECORD_KIND.to_owned(),
            sheet_id: sheet_id.into(),
            action_kind,
            action_class,
            selector: ActionTargetSelector {
                target_kind,
                target_root_ref: target_root_ref.clone(),
                worktree_ref: None,
                topology_class: descriptor
                    .topology_classes
                    .first()
                    .copied()
                    .unwrap_or(RepositoryTopologyClass::CurrentRepoRoot),
                scope_object_ref: format!("scope-ref:{}:{}", action_kind.as_str(), target_root_ref),
            },
            active_root_ref: active_root_ref.to_owned(),
            object_scope: ObjectScopeDisclosure {
                scope_summary_ref: format!("object-scope-ref:{target_root_ref}"),
                affected_object_estimate: None,
                pre_action_truth: pre_truth,
                post_action_truth,
            },
            network,
            provider_auth,
            review_export: ReviewExportParity {
                review_visible: true,
                export_visible: true,
                body_export_allowed: false,
                parity_note_ref: format!("parity-ref:{target_root_ref}"),
            },
            recovery: RecoveryDisclosure {
                reversible: true,
                recovery_class,
                recovery_ref: format!("recovery-ref:{}:{}", action_kind.as_str(), target_root_ref),
            },
            multi_root_preview: MultiRootPreview::single_root(),
            approval,
            safe_operation_scope,
            wrong_root_guard,
            honesty_labels,
        })
    }

    /// Whether this sheet may execute its action as currently targeted.
    ///
    /// A sheet executes only when the target is the active root (or an explicit
    /// multi-root preview broadened it), the wrong-root guard does not block, and
    /// any network step is approved or local-only.
    pub fn is_executable(&self) -> bool {
        if self.wrong_root_guard.blocks() {
            return false;
        }
        match self.approval {
            TopologyActionApproval::PolicyBlocked
            | TopologyActionApproval::NoCommandAvailable
            | TopologyActionApproval::ApprovalRequired => false,
            TopologyActionApproval::Approved | TopologyActionApproval::NotNetworkBearing => true,
        }
    }
}

/// Classifies the one remediation a descriptor's structured state calls for.
type RemediationClassification = (
    TopologyActionKind,
    TopologyActionClass,
    TopologyTargetKind,
    SurfaceResultTruth,
    TopologyHonestyLabel,
    TopologyRecoveryClass,
);

fn classify_remediation(descriptor: &TopologyRootDescriptor) -> Option<RemediationClassification> {
    if descriptor.lfs.state == LfsObjectState::PointerOnly {
        return Some((
            TopologyActionKind::Hydrate,
            TopologyActionClass::HydrateLfsObjects,
            TopologyTargetKind::PointerBackedAsset,
            SurfaceResultTruth::PointerOnly,
            TopologyHonestyLabel::PointerOnly,
            TopologyRecoveryClass::PointerRestorable,
        ));
    }
    if descriptor.object_availability == ObjectAvailability::MissingUnfetched {
        return Some((
            TopologyActionKind::Hydrate,
            TopologyActionClass::FetchMissingObjects,
            TopologyTargetKind::PromisorRemote,
            SurfaceResultTruth::NotFetched,
            TopologyHonestyLabel::NotFetched,
            TopologyRecoveryClass::RemoteRefetchable,
        ));
    }
    if descriptor.repo_identity.kind == RepoIdentityKind::SubmoduleChild
        && !descriptor.repo_identity.child_initialized
    {
        return Some((
            TopologyActionKind::Initialize,
            TopologyActionClass::InitializeSubmodule,
            TopologyTargetKind::ChildRepo,
            SurfaceResultTruth::Uninitialized,
            TopologyHonestyLabel::SubmoduleNotInitialized,
            TopologyRecoveryClass::SubmoduleDeinitable,
        ));
    }
    if descriptor.depth_boundary.depth_class != HistoryDepthClass::FullHistory {
        return Some((
            TopologyActionKind::Deepen,
            TopologyActionClass::DeepenHistory,
            TopologyTargetKind::ShallowHistory,
            SurfaceResultTruth::ShallowBoundary,
            TopologyHonestyLabel::ShallowBoundary,
            TopologyRecoveryClass::RemoteRefetchable,
        ));
    }
    if descriptor.filter_class.omits_paths() {
        return Some((
            TopologyActionKind::Widen,
            TopologyActionClass::WidenWorksetScope,
            TopologyTargetKind::SparseSlice,
            SurfaceResultTruth::OutsideCurrentSlice,
            TopologyHonestyLabel::OutsideCurrentSlice,
            TopologyRecoveryClass::NoMutationReversible,
        ));
    }
    None
}

/// Redaction-safe support-export projection for an action-review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyActionSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable export id.
    pub export_id: String,
    /// Sheet ids included in the export.
    pub sheet_refs: Vec<String>,
    /// Action kinds chosen, offered, or denied during the captured flow.
    pub action_kinds: Vec<TopologyActionKind>,
    /// Structured fields retained after redaction.
    pub reconstruction_fields: Vec<String>,
    /// True when no raw paths are embedded.
    pub raw_paths_redacted: bool,
    /// True when no raw object bytes are embedded.
    pub raw_object_bytes_redacted: bool,
}

/// Top-level packet binding reviewed remediation sheets to their support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyActionReviewPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp (RFC 3339).
    pub generated_at: String,
    /// Reviewed remediation sheets.
    pub sheets: Vec<TopologyActionSheet>,
    /// Redaction-safe support-export projection.
    pub support_export: TopologyActionSupportExport,
}

impl TopologyActionReviewPacket {
    /// Parses a packet from JSON and validates its cross-sheet invariants.
    ///
    /// # Errors
    ///
    /// Returns [`TopologyActionError`] when the JSON is invalid or the parsed
    /// packet violates the action-review contract.
    pub fn parse_json(input: &str) -> Result<Self, TopologyActionError> {
        let packet: Self = serde_json::from_str(input).map_err(TopologyActionError::Json)?;
        let violations = packet.validate();
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(TopologyActionError::Validation(violations))
        }
    }

    /// Validates every sheet and support-export invariant, returning the full set.
    pub fn validate(&self) -> Vec<TopologyActionValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != TOPOLOGY_ACTION_REVIEW_PACKET_RECORD_KIND {
            errors.push(TopologyActionValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != TOPOLOGY_ACTION_REVIEW_SCHEMA_VERSION {
            errors.push(TopologyActionValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            errors.push(TopologyActionValidationError::MissingIdentity);
        }

        let mut sheet_ids: HashSet<&str> = HashSet::new();
        for sheet in &self.sheets {
            if !sheet_ids.insert(sheet.sheet_id.as_str()) {
                errors.push(TopologyActionValidationError::DuplicateSheetId {
                    sheet_id: sheet.sheet_id.clone(),
                });
            }
            validate_sheet(sheet, &mut errors);
        }

        validate_support_export(self, &sheet_ids, &mut errors);
        errors
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("topology action review packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Topology Action Review Sheets\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Sheets: {}\n\n", self.sheets.len()));
        out.push_str("## Sheets\n\n");
        for sheet in &self.sheets {
            out.push_str(&format!(
                "- **{}** → `{}` ({}): network {}, approval `{}`, guard `{}`, scope `{}`\n",
                sheet.action_kind.as_str(),
                sheet.selector.target_root_ref,
                sheet.selector.target_kind.as_str(),
                sheet.network.reaches_network,
                approval_token(sheet.approval),
                sheet.wrong_root_guard.as_str(),
                operation_scope_token(sheet.safe_operation_scope),
            ));
        }
        out
    }
}

/// Reads and validates the checked-in canonical action-review packet.
///
/// # Errors
///
/// Returns [`TopologyActionError`] when the checked-in packet fails to parse or
/// violates the action-review contract.
pub fn current_topology_action_review_packet(
) -> Result<TopologyActionReviewPacket, TopologyActionError> {
    TopologyActionReviewPacket::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/git_topology/topology_action_review.json"
    )))
}

fn validate_sheet(sheet: &TopologyActionSheet, errors: &mut Vec<TopologyActionValidationError>) {
    if sheet.record_kind != TOPOLOGY_ACTION_SHEET_RECORD_KIND {
        errors.push(TopologyActionValidationError::WrongRecordKind {
            observed: sheet.record_kind.clone(),
        });
    }

    // The action class must realize the action verb, and the target kind must be a
    // coherent target for it: widen/deepen/initialize/hydrate stay distinct.
    if !sheet.action_kind.accepts_class(sheet.action_class) {
        errors.push(TopologyActionValidationError::ActionClassMismatch {
            sheet_id: sheet.sheet_id.clone(),
            action: sheet.action_kind,
        });
    }
    if !sheet.selector.target_kind.fits_action(sheet.action_kind) {
        errors.push(TopologyActionValidationError::TargetKindMismatch {
            sheet_id: sheet.sheet_id.clone(),
            action: sheet.action_kind,
            target: sheet.selector.target_kind,
        });
    }

    // A sheet only exists to repair a non-complete state.
    if sheet.object_scope.pre_action_truth == SurfaceResultTruth::Complete {
        errors.push(TopologyActionValidationError::SheetRepairsCompleteState {
            sheet_id: sheet.sheet_id.clone(),
        });
    }

    let blocks = sheet.wrong_root_guard.blocks();
    let broadened = sheet.multi_root_preview.broadened;

    // No-wrong-root safeguard: a non-broadened cross-root target must be guarded,
    // and a guarded sheet must be denied and never pre-approved.
    let cross_root = sheet.active_root_ref != sheet.selector.target_root_ref;
    if cross_root && !broadened && !blocks {
        errors.push(TopologyActionValidationError::WrongRootNotGuarded {
            sheet_id: sheet.sheet_id.clone(),
        });
    }
    if !cross_root && blocks {
        errors.push(TopologyActionValidationError::GuardWithoutWrongRoot {
            sheet_id: sheet.sheet_id.clone(),
        });
    }
    if blocks {
        if matches!(
            sheet.safe_operation_scope,
            TopologyOperationScope::ActiveRootOnly
        ) {
            errors.push(
                TopologyActionValidationError::BlockedSheetAllowsActiveScope {
                    sheet_id: sheet.sheet_id.clone(),
                },
            );
        }
        if matches!(sheet.approval, TopologyActionApproval::Approved) {
            errors.push(TopologyActionValidationError::BlockedSheetApproved {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    }

    // Explicit broadening and the multi-root preview scope are two sides of one
    // coin: a broadened action requires the preview scope and named roots, and a
    // preview-required scope must actually broaden.
    let preview_scope = matches!(
        sheet.safe_operation_scope,
        TopologyOperationScope::ExplicitMultiRootPreviewRequired
    );
    if broadened && sheet.multi_root_preview.additional_root_refs.is_empty() {
        errors.push(
            TopologyActionValidationError::BroadenedPreviewMissingRoots {
                sheet_id: sheet.sheet_id.clone(),
            },
        );
    }
    if broadened != preview_scope {
        errors.push(TopologyActionValidationError::PreviewScopeMismatch {
            sheet_id: sheet.sheet_id.clone(),
        });
    }

    // Network stays reviewed and attributable: a network-bearing action carries an
    // approval posture and an egress ref, and a local action never claims to.
    let network_action = sheet.action_kind.is_network_bearing();
    if network_action != sheet.network.reaches_network {
        errors.push(TopologyActionValidationError::NetworkFlagMismatch {
            sheet_id: sheet.sheet_id.clone(),
        });
    }
    if sheet.network.reaches_network {
        if sheet.network.egress_ref.is_none() {
            errors.push(TopologyActionValidationError::NetworkMissingEgress {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
        if !matches!(
            sheet.approval,
            TopologyActionApproval::ApprovalRequired
                | TopologyActionApproval::Approved
                | TopologyActionApproval::PolicyBlocked
        ) {
            errors.push(TopologyActionValidationError::NetworkMissingApproval {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
        if sheet.recovery.recovery_ref.trim().is_empty() {
            errors.push(TopologyActionValidationError::NetworkMissingRecovery {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    } else if matches!(sheet.approval, TopologyActionApproval::Approved) {
        // A local widen needs no network approval; pre-approving it is meaningless.
        errors.push(TopologyActionValidationError::LocalActionPreApproved {
            sheet_id: sheet.sheet_id.clone(),
        });
    }

    // Review/export parity: the action is disclosed in both review and export.
    if !sheet.review_export.review_visible || !sheet.review_export.export_visible {
        errors.push(TopologyActionValidationError::ParityNotDisclosed {
            sheet_id: sheet.sheet_id.clone(),
        });
    }
    // A sheet is a pre-execution preview; it never embeds materialized body bytes.
    if sheet.review_export.body_export_allowed {
        errors.push(TopologyActionValidationError::SheetEmbedsBodyExport {
            sheet_id: sheet.sheet_id.clone(),
        });
    }

    // The honesty label for the repaired state must be present.
    let required_label = match sheet.object_scope.pre_action_truth {
        SurfaceResultTruth::PointerOnly => Some(TopologyHonestyLabel::PointerOnly),
        SurfaceResultTruth::NotFetched => Some(TopologyHonestyLabel::NotFetched),
        SurfaceResultTruth::Uninitialized => Some(TopologyHonestyLabel::SubmoduleNotInitialized),
        SurfaceResultTruth::ShallowBoundary => Some(TopologyHonestyLabel::ShallowBoundary),
        SurfaceResultTruth::OutsideCurrentSlice => Some(TopologyHonestyLabel::OutsideCurrentSlice),
        _ => None,
    };
    if let Some(label) = required_label {
        if !sheet.honesty_labels.contains(&label) {
            errors.push(TopologyActionValidationError::SheetMissingHonestyLabel {
                sheet_id: sheet.sheet_id.clone(),
                label,
            });
        }
    }
}

fn validate_support_export(
    packet: &TopologyActionReviewPacket,
    sheet_ids: &HashSet<&str>,
    errors: &mut Vec<TopologyActionValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != TOPOLOGY_ACTION_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(TopologyActionValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for sheet_ref in &export.sheet_refs {
        if !sheet_ids.contains(sheet_ref.as_str()) {
            errors.push(TopologyActionValidationError::UnknownSupportSheetRef {
                sheet_ref: sheet_ref.clone(),
            });
        }
    }
    for required in TOPOLOGY_ACTION_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(TopologyActionValidationError::SupportExportMissingField {
                field: required.to_string(),
            });
        }
    }
    if !export.raw_paths_redacted || !export.raw_object_bytes_redacted {
        errors.push(TopologyActionValidationError::SupportExportEmbedsRawMaterial);
    }
}

/// Stable token for a [`TopologyActionApproval`].
fn approval_token(approval: TopologyActionApproval) -> &'static str {
    match approval {
        TopologyActionApproval::NotNetworkBearing => "not_network_bearing",
        TopologyActionApproval::ApprovalRequired => "approval_required",
        TopologyActionApproval::Approved => "approved",
        TopologyActionApproval::PolicyBlocked => "policy_blocked",
        TopologyActionApproval::NoCommandAvailable => "no_command_available",
    }
}

/// Stable token for a [`TopologyOperationScope`].
fn operation_scope_token(scope: TopologyOperationScope) -> &'static str {
    match scope {
        TopologyOperationScope::ActiveRootOnly => "active_root_only",
        TopologyOperationScope::ChildRootOnly => "child_root_only",
        TopologyOperationScope::ExplicitMultiRootPreviewRequired => {
            "explicit_multi_root_preview_required"
        }
        TopologyOperationScope::MetadataOnly => "metadata_only",
        TopologyOperationScope::MutationDenied => "mutation_denied",
    }
}

/// Error returned while parsing an action-review packet.
#[derive(Debug)]
pub enum TopologyActionError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-sheet validation failed.
    Validation(Vec<TopologyActionValidationError>),
}

impl fmt::Display for TopologyActionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(
                    formatter,
                    "failed to parse topology action packet JSON: {error}"
                )
            }
            Self::Validation(errors) => {
                write!(
                    formatter,
                    "topology action review packet has validation errors: "
                )?;
                for (index, error) in errors.iter().enumerate() {
                    if index > 0 {
                        write!(formatter, "; ")?;
                    }
                    write!(formatter, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for TopologyActionError {}

/// Cross-sheet validation error for an action-review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyActionValidationError {
    /// A record-kind tag does not match the stable contract.
    WrongRecordKind {
        /// Observed record-kind tag.
        observed: String,
    },
    /// The packet schema version is unsupported.
    WrongSchemaVersion {
        /// Observed schema version.
        observed: u32,
    },
    /// A required identity field is missing.
    MissingIdentity,
    /// A sheet id is declared more than once.
    DuplicateSheetId {
        /// Duplicated sheet id.
        sheet_id: String,
    },
    /// A sheet's action class does not realize its action verb.
    ActionClassMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Action verb.
        action: TopologyActionKind,
    },
    /// A sheet's target kind is not a coherent target for its action verb.
    TargetKindMismatch {
        /// Sheet id.
        sheet_id: String,
        /// Action verb.
        action: TopologyActionKind,
        /// Target kind.
        target: TopologyTargetKind,
    },
    /// A sheet claims to repair an already-complete state.
    SheetRepairsCompleteState {
        /// Sheet id.
        sheet_id: String,
    },
    /// A cross-root, non-broadened sheet is not wrong-root guarded.
    WrongRootNotGuarded {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet is guarded although the target is the active root.
    GuardWithoutWrongRoot {
        /// Sheet id.
        sheet_id: String,
    },
    /// A blocked sheet still advertises active-root scope.
    BlockedSheetAllowsActiveScope {
        /// Sheet id.
        sheet_id: String,
    },
    /// A blocked sheet was pre-approved.
    BlockedSheetApproved {
        /// Sheet id.
        sheet_id: String,
    },
    /// A broadened preview does not name any additional root.
    BroadenedPreviewMissingRoots {
        /// Sheet id.
        sheet_id: String,
    },
    /// The broadened flag and the preview-required scope disagree.
    PreviewScopeMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// The network flag disagrees with the action verb.
    NetworkFlagMismatch {
        /// Sheet id.
        sheet_id: String,
    },
    /// A network-bearing sheet does not name its egress.
    NetworkMissingEgress {
        /// Sheet id.
        sheet_id: String,
    },
    /// A network-bearing sheet lacks an approval posture.
    NetworkMissingApproval {
        /// Sheet id.
        sheet_id: String,
    },
    /// A network-bearing sheet discloses no recovery path.
    NetworkMissingRecovery {
        /// Sheet id.
        sheet_id: String,
    },
    /// A local action was needlessly pre-approved.
    LocalActionPreApproved {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet is not disclosed in both review and export.
    ParityNotDisclosed {
        /// Sheet id.
        sheet_id: String,
    },
    /// A pre-execution sheet embeds materialized body bytes.
    SheetEmbedsBodyExport {
        /// Sheet id.
        sheet_id: String,
    },
    /// A sheet omits the honesty label its repaired state requires.
    SheetMissingHonestyLabel {
        /// Sheet id.
        sheet_id: String,
        /// Required honesty label.
        label: TopologyHonestyLabel,
    },
    /// A support-export sheet ref is unknown.
    UnknownSupportSheetRef {
        /// Unknown sheet ref.
        sheet_ref: String,
    },
    /// The support export omits a required reconstruction field.
    SupportExportMissingField {
        /// Missing reconstruction field.
        field: String,
    },
    /// The support export embeds raw paths or raw object bytes.
    SupportExportEmbedsRawMaterial,
}

impl fmt::Display for TopologyActionValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind { observed } => {
                write!(formatter, "unexpected record kind {observed}")
            }
            Self::WrongSchemaVersion { observed } => {
                write!(formatter, "unsupported schema version {observed}")
            }
            Self::MissingIdentity => write!(formatter, "packet is missing identity fields"),
            Self::DuplicateSheetId { sheet_id } => {
                write!(formatter, "sheet id {sheet_id} is declared more than once")
            }
            Self::ActionClassMismatch { sheet_id, action } => write!(
                formatter,
                "sheet {sheet_id} action class does not realize verb {}",
                action.as_str()
            ),
            Self::TargetKindMismatch {
                sheet_id,
                action,
                target,
            } => write!(
                formatter,
                "sheet {sheet_id} target {} does not fit action {}",
                target.as_str(),
                action.as_str()
            ),
            Self::SheetRepairsCompleteState { sheet_id } => {
                write!(
                    formatter,
                    "sheet {sheet_id} repairs an already-complete state"
                )
            }
            Self::WrongRootNotGuarded { sheet_id } => {
                write!(
                    formatter,
                    "cross-root sheet {sheet_id} is not wrong-root guarded"
                )
            }
            Self::GuardWithoutWrongRoot { sheet_id } => {
                write!(
                    formatter,
                    "sheet {sheet_id} is guarded without a wrong root"
                )
            }
            Self::BlockedSheetAllowsActiveScope { sheet_id } => {
                write!(
                    formatter,
                    "blocked sheet {sheet_id} still allows active-root scope"
                )
            }
            Self::BlockedSheetApproved { sheet_id } => {
                write!(formatter, "blocked sheet {sheet_id} was pre-approved")
            }
            Self::BroadenedPreviewMissingRoots { sheet_id } => {
                write!(
                    formatter,
                    "broadened sheet {sheet_id} names no additional root"
                )
            }
            Self::PreviewScopeMismatch { sheet_id } => write!(
                formatter,
                "sheet {sheet_id} broadened flag disagrees with its preview scope"
            ),
            Self::NetworkFlagMismatch { sheet_id } => {
                write!(
                    formatter,
                    "sheet {sheet_id} network flag disagrees with its verb"
                )
            }
            Self::NetworkMissingEgress { sheet_id } => {
                write!(formatter, "network sheet {sheet_id} names no egress")
            }
            Self::NetworkMissingApproval { sheet_id } => {
                write!(
                    formatter,
                    "network sheet {sheet_id} lacks an approval posture"
                )
            }
            Self::NetworkMissingRecovery { sheet_id } => {
                write!(
                    formatter,
                    "network sheet {sheet_id} discloses no recovery path"
                )
            }
            Self::LocalActionPreApproved { sheet_id } => {
                write!(
                    formatter,
                    "local sheet {sheet_id} was needlessly pre-approved"
                )
            }
            Self::ParityNotDisclosed { sheet_id } => {
                write!(
                    formatter,
                    "sheet {sheet_id} is not disclosed in review and export"
                )
            }
            Self::SheetEmbedsBodyExport { sheet_id } => {
                write!(
                    formatter,
                    "pre-execution sheet {sheet_id} embeds body bytes"
                )
            }
            Self::SheetMissingHonestyLabel { sheet_id, label } => write!(
                formatter,
                "sheet {sheet_id} is missing honesty label {}",
                label.as_str()
            ),
            Self::UnknownSupportSheetRef { sheet_ref } => {
                write!(
                    formatter,
                    "support export references unknown sheet {sheet_ref}"
                )
            }
            Self::SupportExportMissingField { field } => {
                write!(
                    formatter,
                    "support export missing reconstruction field {field}"
                )
            }
            Self::SupportExportEmbedsRawMaterial => {
                write!(
                    formatter,
                    "support export embeds raw paths or raw object bytes"
                )
            }
        }
    }
}
