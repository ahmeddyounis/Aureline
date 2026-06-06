//! Stable repository-topology truth shared by Git-adjacent surfaces.
//!
//! This module freezes the packet that search, Git graph, review, blame,
//! code-action, AI-context, run/debug, and support-export rows read when
//! a repository is sparse, partial, shallow, submodule-backed,
//! nested-independent, pointer-backed, or intentionally generated/vendor
//! excluded. It does not fetch, deepen, initialize, or hydrate content;
//! it only preserves the typed truth and approval posture required before
//! a surface may claim complete coverage or target a root for mutation.

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for [`RepositoryTopologyTruthPacket`].
pub const REPOSITORY_TOPOLOGY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RepositoryTopologyTruthPacket`].
pub const REPOSITORY_TOPOLOGY_TRUTH_PACKET_RECORD_KIND: &str = "repository_topology_truth_packet";

/// Stable record-kind tag for [`RepositoryTopologyDescriptor`].
pub const REPOSITORY_TOPOLOGY_DESCRIPTOR_RECORD_KIND: &str = "repository_topology_descriptor";

/// Stable record-kind tag for [`SurfaceTopologyTruthRow`].
pub const SURFACE_TOPOLOGY_TRUTH_ROW_RECORD_KIND: &str = "surface_topology_truth_row";

/// Stable record-kind tag for [`RepositoryTopologySupportExport`].
pub const REPOSITORY_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "repository_topology_support_export";

/// Surfaces that must keep topology caveats visible before making a
/// complete-coverage or mutation-target claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologySurface {
    /// Search result and zero-result rows.
    Search,
    /// Git graph, history graph, or branch graph rows.
    GitGraph,
    /// Review diff, review summary, and publish rows.
    Review,
    /// Blame and file-history rows.
    Blame,
    /// Code actions, quick fixes, refactors, and apply previews.
    CodeActions,
    /// AI-context assembly and evidence inspectors.
    AiContext,
    /// Run/debug launch target and task rows.
    RunDebug,
    /// Redaction-safe support export rows.
    SupportExport,
}

impl TopologySurface {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::GitGraph => "git_graph",
            Self::Review => "review",
            Self::Blame => "blame",
            Self::CodeActions => "code_actions",
            Self::AiContext => "ai_context",
            Self::RunDebug => "run_debug",
            Self::SupportExport => "support_export",
        }
    }
}

/// Repository-topology class that materially changes product truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryTopologyClass {
    /// The active single repository root.
    CurrentRepoRoot,
    /// The active view is constrained by a named workset.
    WorksetRoot,
    /// Git sparse checkout or sparse IDE slice is active.
    SparseCheckoutRoot,
    /// A linked worktree is the explicit operation target.
    WorktreeRoot,
    /// Object completeness depends on a promisor remote.
    PartialClonePromisorRoot,
    /// History is limited by clone depth.
    ShallowHistoryRoot,
    /// A child repository is pinned by a parent gitlink.
    SubmoduleRoot,
    /// A nested `.git` root is independent of its parent.
    NestedIndependentRepoRoot,
    /// Visible content is governed by Git LFS pointer/hydration state.
    LfsHydrationBoundary,
    /// Generated or vendor content is intentionally outside editable truth.
    GeneratedVendorRoot,
}

impl RepositoryTopologyClass {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentRepoRoot => "current_repo_root",
            Self::WorksetRoot => "workset_root",
            Self::SparseCheckoutRoot => "sparse_checkout_root",
            Self::WorktreeRoot => "worktree_root",
            Self::PartialClonePromisorRoot => "partial_clone_promisor_root",
            Self::ShallowHistoryRoot => "shallow_history_root",
            Self::SubmoduleRoot => "submodule_root",
            Self::NestedIndependentRepoRoot => "nested_independent_repo_root",
            Self::LfsHydrationBoundary => "lfs_hydration_boundary",
            Self::GeneratedVendorRoot => "generated_vendor_root",
        }
    }
}

/// Closed honesty labels that explain why a surface answer is partial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyHonestyLabel {
    /// The row is omitted by the active workset or sparse slice.
    OutsideCurrentSlice,
    /// A known object is not materialized locally.
    NotFetched,
    /// History or blame stopped at a shallow boundary.
    ShallowBoundary,
    /// A parent gitlink is known but the child checkout is unavailable.
    SubmoduleNotInitialized,
    /// The object belongs to a nested independent root.
    NestedRepoBoundary,
    /// Only Git LFS pointer metadata is available.
    PointerOnly,
    /// The requested action targeted the wrong root.
    WrongTargetRoot,
    /// Cleanliness was not verified across the relevant scope.
    DirtyStateUnknown,
    /// A root, remote, index, or object cannot currently be reached.
    Unavailable,
    /// Policy excludes the root or content.
    PolicyExcluded,
    /// Generated or vendor content is intentionally outside source truth.
    GeneratedOrExcluded,
}

impl TopologyHonestyLabel {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutsideCurrentSlice => "outside_current_slice",
            Self::NotFetched => "not_fetched",
            Self::ShallowBoundary => "shallow_boundary",
            Self::SubmoduleNotInitialized => "submodule_not_initialized",
            Self::NestedRepoBoundary => "nested_repo_boundary",
            Self::PointerOnly => "pointer_only",
            Self::WrongTargetRoot => "wrong_target_root",
            Self::DirtyStateUnknown => "dirty_state_unknown",
            Self::Unavailable => "unavailable",
            Self::PolicyExcluded => "policy_excluded",
            Self::GeneratedOrExcluded => "generated_or_excluded",
        }
    }
}

/// User-visible action class that may repair or inspect a topology caveat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyActionClass {
    /// Expand a workset or sparse slice after review.
    WidenWorksetScope,
    /// Fetch missing promisor or partial-clone objects.
    FetchMissingObjects,
    /// Deepen shallow history.
    DeepenHistory,
    /// Initialize a submodule checkout.
    InitializeSubmodule,
    /// Hydrate Git LFS objects.
    HydrateLfsObjects,
    /// Switch the action target to the authoritative root.
    SwitchTargetRoot,
    /// Open an explicit child repository root.
    OpenChildRepoRoot,
    /// Inspect generated/vendor lineage.
    InspectGeneratedLineage,
    /// Export a topology packet.
    ExportTopologyPacket,
    /// No direct action is available.
    NoneAvailable,
}

impl TopologyActionClass {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WidenWorksetScope => "widen_workset_scope",
            Self::FetchMissingObjects => "fetch_missing_objects",
            Self::DeepenHistory => "deepen_history",
            Self::InitializeSubmodule => "initialize_submodule",
            Self::HydrateLfsObjects => "hydrate_lfs_objects",
            Self::SwitchTargetRoot => "switch_target_root",
            Self::OpenChildRepoRoot => "open_child_repo_root",
            Self::InspectGeneratedLineage => "inspect_generated_lineage",
            Self::ExportTopologyPacket => "export_topology_packet",
            Self::NoneAvailable => "none_available",
        }
    }

    /// Returns true when the action may reach the network or materialize
    /// remote content.
    pub const fn is_network_bearing(self) -> bool {
        matches!(
            self,
            Self::FetchMissingObjects
                | Self::DeepenHistory
                | Self::InitializeSubmodule
                | Self::HydrateLfsObjects
        )
    }
}

/// Approval and policy posture for an offered topology action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyActionApproval {
    /// The action is local metadata-only and needs no network approval.
    NotNetworkBearing,
    /// The action is network-bearing and awaits explicit approval.
    ApprovalRequired,
    /// The action was explicitly approved before execution.
    Approved,
    /// Policy blocks the action.
    PolicyBlocked,
    /// No command is available.
    NoCommandAvailable,
}

/// Whether a surface may present complete truth for its current answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageClaimPosture {
    /// The descriptor permits a complete coverage claim.
    FullCoverageAllowed,
    /// The descriptor narrows the surface claim.
    NarrowedByTopology,
    /// The operation is denied because the selected root is wrong.
    DeniedWrongRoot,
}

/// Exact result-truth class a surface renders for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceResultTruth {
    /// The row is complete for the selected root and scope.
    Complete,
    /// A result may exist outside the active slice.
    OutsideCurrentSlice,
    /// A known object is not fetched.
    NotFetched,
    /// History or blame is bounded by shallow depth.
    ShallowBoundary,
    /// A submodule child is not initialized.
    Uninitialized,
    /// The row belongs to a nested independent root.
    NestedRoot,
    /// Only pointer metadata is available.
    PointerOnly,
    /// Generated/vendor content is excluded from editable truth.
    GeneratedOrExcluded,
    /// The requested target root is wrong.
    WrongTargetRoot,
    /// The row is currently unavailable.
    Unavailable,
}

/// Mutation or export scope permitted by a descriptor or row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyOperationScope {
    /// Only the active authoritative root may be targeted.
    ActiveRootOnly,
    /// A child root must be targeted explicitly.
    ChildRootOnly,
    /// Parent-plus-child or multi-root operation needs preview.
    ExplicitMultiRootPreviewRequired,
    /// Only metadata may be exported or inspected.
    MetadataOnly,
    /// Mutation is denied.
    MutationDenied,
}

/// Action offered by a topology row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyAction {
    /// Action class.
    pub action_class: TopologyActionClass,
    /// Optional command identifier for surfaces that can route the action.
    pub command_ref: Option<String>,
    /// Approval and policy posture.
    pub approval: TopologyActionApproval,
    /// Redaction-safe evidence reference for preview, approval, or denial.
    pub evidence_ref: String,
}

impl TopologyAction {
    /// Returns true when this action is blocked by policy.
    pub const fn is_policy_blocked(&self) -> bool {
        matches!(self.approval, TopologyActionApproval::PolicyBlocked)
    }
}

/// Root-level topology descriptor shared by every surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryTopologyDescriptor {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable descriptor id.
    pub descriptor_id: String,
    /// Authoritative repository root ref.
    pub repo_root_ref: String,
    /// Worktree ref, when the root is a worktree.
    pub worktree_ref: Option<String>,
    /// Parent repo root ref for child roots.
    pub parent_repo_root_ref: Option<String>,
    /// Child repo root ref for parent-owned boundary rows.
    pub child_repo_root_ref: Option<String>,
    /// Closed topology classes detected for this root.
    pub topology_classes: Vec<RepositoryTopologyClass>,
    /// Opaque ref to omitted sparse/workset scope, if any.
    pub omitted_scope_ref: Option<String>,
    /// Opaque ref to promisor/missing-object scope, if any.
    pub unfetched_scope_ref: Option<String>,
    /// Opaque ref to the shallow boundary, if any.
    pub shallow_boundary_ref: Option<String>,
    /// Opaque ref to the submodule gitlink pin, if any.
    pub submodule_pin_ref: Option<String>,
    /// Opaque ref to Git LFS pointer-only objects, if any.
    pub lfs_pointer_scope_ref: Option<String>,
    /// Opaque ref to generated/vendor exclusion scope, if any.
    pub generated_vendor_scope_ref: Option<String>,
    /// Safe mutation or export scope for this descriptor.
    pub safe_operation_scope: TopologyOperationScope,
    /// Honesty labels downstream rows must preserve.
    pub honesty_labels: Vec<TopologyHonestyLabel>,
    /// Actions a surface may offer without changing root implicitly.
    pub allowed_actions: Vec<TopologyAction>,
}

/// One consumer-surface row produced from a topology descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTopologyTruthRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Surface that renders this row.
    pub surface: TopologySurface,
    /// Referenced [`RepositoryTopologyDescriptor::descriptor_id`].
    pub descriptor_ref: String,
    /// Root selected by the caller.
    pub active_root_ref: String,
    /// Root that owns the object, result, or operation.
    pub authoritative_root_ref: String,
    /// Exact result truth rendered by the surface.
    pub result_truth: SurfaceResultTruth,
    /// Whether a complete-coverage claim is allowed.
    pub coverage_claim: CoverageClaimPosture,
    /// Safe target scope for a mutation, export, or execution action.
    pub safe_operation_scope: TopologyOperationScope,
    /// Labels rendered or carried by the row.
    pub honesty_labels: Vec<TopologyHonestyLabel>,
    /// Actions offered by the row.
    pub offered_actions: Vec<TopologyAction>,
    /// True when this row can mutate content.
    pub mutation_allowed: bool,
    /// True when the surface may embed body bytes in an export.
    pub body_export_allowed: bool,
}

/// Support-export projection that preserves topology reconstruction data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryTopologySupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable export id.
    pub export_id: String,
    /// Descriptor ids included in the export.
    pub descriptor_refs: Vec<String>,
    /// Surface row ids included in the export.
    pub surface_row_refs: Vec<String>,
    /// Reconstruction fields retained after redaction.
    pub reconstruction_fields: Vec<String>,
    /// Actions chosen, denied, or offered during the captured flow.
    pub chosen_actions: Vec<TopologyActionClass>,
    /// True when no raw paths are embedded.
    pub raw_paths_redacted: bool,
    /// True when no raw object bytes are embedded.
    pub raw_object_bytes_redacted: bool,
}

/// Top-level stable packet for repository-topology truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryTopologyTruthPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Root-level topology descriptors.
    pub descriptors: Vec<RepositoryTopologyDescriptor>,
    /// Per-surface truth rows.
    pub surface_rows: Vec<SurfaceTopologyTruthRow>,
    /// Redaction-safe support export projection.
    pub support_export: RepositoryTopologySupportExport,
}

impl RepositoryTopologyTruthPacket {
    /// Parses a packet from JSON and validates its cross-row invariants.
    ///
    /// # Errors
    ///
    /// Returns [`RepositoryTopologyPacketError`] when the JSON is invalid
    /// or the parsed packet violates the stable topology contract.
    pub fn parse_json(input: &str) -> Result<Self, RepositoryTopologyPacketError> {
        let packet: Self =
            serde_json::from_str(input).map_err(RepositoryTopologyPacketError::Json)?;
        packet.validate()?;
        Ok(packet)
    }

    /// Validates descriptor, surface, action, and support-export
    /// invariants that schemas alone cannot express.
    ///
    /// # Errors
    ///
    /// Returns [`RepositoryTopologyPacketError::Validation`] when a
    /// cross-row invariant is violated.
    pub fn validate(&self) -> Result<(), RepositoryTopologyPacketError> {
        let mut errors = Vec::new();

        if self.record_kind != REPOSITORY_TOPOLOGY_TRUTH_PACKET_RECORD_KIND {
            errors.push(RepositoryTopologyValidationError::UnexpectedRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != REPOSITORY_TOPOLOGY_TRUTH_SCHEMA_VERSION {
            errors.push(
                RepositoryTopologyValidationError::UnsupportedSchemaVersion {
                    observed: self.schema_version,
                },
            );
        }

        let descriptor_ids: HashSet<&str> = self
            .descriptors
            .iter()
            .map(|descriptor| descriptor.descriptor_id.as_str())
            .collect();
        let row_ids: HashSet<&str> = self
            .surface_rows
            .iter()
            .map(|row| row.row_id.as_str())
            .collect();

        for descriptor in &self.descriptors {
            if descriptor.record_kind != REPOSITORY_TOPOLOGY_DESCRIPTOR_RECORD_KIND {
                errors.push(RepositoryTopologyValidationError::UnexpectedRecordKind {
                    observed: descriptor.record_kind.clone(),
                });
            }
            if descriptor.topology_classes.is_empty() {
                errors.push(
                    RepositoryTopologyValidationError::DescriptorMissingTopologyClass {
                        descriptor_id: descriptor.descriptor_id.clone(),
                    },
                );
            }
            if descriptor.honesty_labels.is_empty()
                && !descriptor
                    .topology_classes
                    .contains(&RepositoryTopologyClass::CurrentRepoRoot)
            {
                errors.push(
                    RepositoryTopologyValidationError::DescriptorMissingHonestyLabel {
                        descriptor_id: descriptor.descriptor_id.clone(),
                    },
                );
            }
            for action in &descriptor.allowed_actions {
                validate_action(
                    action,
                    &descriptor.descriptor_id,
                    &mut errors,
                    TopologyValidationSubject::Descriptor,
                );
            }
        }

        for row in &self.surface_rows {
            if row.record_kind != SURFACE_TOPOLOGY_TRUTH_ROW_RECORD_KIND {
                errors.push(RepositoryTopologyValidationError::UnexpectedRecordKind {
                    observed: row.record_kind.clone(),
                });
            }
            if !descriptor_ids.contains(row.descriptor_ref.as_str()) {
                errors.push(RepositoryTopologyValidationError::UnknownDescriptorRef {
                    row_id: row.row_id.clone(),
                    descriptor_ref: row.descriptor_ref.clone(),
                });
            }
            if !row.honesty_labels.is_empty()
                && matches!(
                    row.coverage_claim,
                    CoverageClaimPosture::FullCoverageAllowed
                )
            {
                errors.push(
                    RepositoryTopologyValidationError::PartialRowClaimsFullCoverage {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.active_root_ref != row.authoritative_root_ref
                && !matches!(
                    row.coverage_claim,
                    CoverageClaimPosture::DeniedWrongRoot
                        | CoverageClaimPosture::NarrowedByTopology
                )
            {
                errors.push(
                    RepositoryTopologyValidationError::WrongRootNotDeniedOrNarrowed {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            if row.mutation_allowed
                && matches!(
                    row.safe_operation_scope,
                    TopologyOperationScope::MetadataOnly | TopologyOperationScope::MutationDenied
                )
            {
                errors.push(
                    RepositoryTopologyValidationError::MutationAllowedForReadOnlyScope {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            for action in &row.offered_actions {
                validate_action(
                    action,
                    &row.row_id,
                    &mut errors,
                    TopologyValidationSubject::SurfaceRow,
                );
            }
        }

        if self.support_export.record_kind != REPOSITORY_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND {
            errors.push(RepositoryTopologyValidationError::UnexpectedRecordKind {
                observed: self.support_export.record_kind.clone(),
            });
        }
        for descriptor_ref in &self.support_export.descriptor_refs {
            if !descriptor_ids.contains(descriptor_ref.as_str()) {
                errors.push(
                    RepositoryTopologyValidationError::UnknownSupportDescriptorRef {
                        descriptor_ref: descriptor_ref.clone(),
                    },
                );
            }
        }
        for row_ref in &self.support_export.surface_row_refs {
            if !row_ids.contains(row_ref.as_str()) {
                errors.push(
                    RepositoryTopologyValidationError::UnknownSupportSurfaceRowRef {
                        row_ref: row_ref.clone(),
                    },
                );
            }
        }
        for required in [
            "topology_class",
            "omitted_or_unfetched_scope",
            "chosen_action",
            "active_root_ref",
            "authoritative_root_ref",
        ] {
            if !self
                .support_export
                .reconstruction_fields
                .iter()
                .any(|field| field == required)
            {
                errors.push(
                    RepositoryTopologyValidationError::SupportExportMissingField {
                        field: required.to_string(),
                    },
                );
            }
        }
        if !self.support_export.raw_paths_redacted || !self.support_export.raw_object_bytes_redacted
        {
            errors.push(RepositoryTopologyValidationError::SupportExportEmbedsRawMaterial);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(RepositoryTopologyPacketError::Validation(errors))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopologyValidationSubject {
    Descriptor,
    SurfaceRow,
}

fn validate_action(
    action: &TopologyAction,
    subject_id: &str,
    errors: &mut Vec<RepositoryTopologyValidationError>,
    subject: TopologyValidationSubject,
) {
    if action.action_class.is_network_bearing()
        && !matches!(
            action.approval,
            TopologyActionApproval::ApprovalRequired
                | TopologyActionApproval::Approved
                | TopologyActionApproval::PolicyBlocked
        )
    {
        match subject {
            TopologyValidationSubject::Descriptor => {
                errors.push(
                    RepositoryTopologyValidationError::NetworkActionMissingApprovalPosture {
                        subject_id: subject_id.to_string(),
                        action: action.action_class,
                    },
                );
            }
            TopologyValidationSubject::SurfaceRow => {
                errors.push(
                    RepositoryTopologyValidationError::NetworkActionMissingApprovalPosture {
                        subject_id: subject_id.to_string(),
                        action: action.action_class,
                    },
                );
            }
        }
    }
    if matches!(action.action_class, TopologyActionClass::NoneAvailable)
        && !matches!(action.approval, TopologyActionApproval::NoCommandAvailable)
    {
        errors.push(
            RepositoryTopologyValidationError::NoneActionHasCommandPosture {
                subject_id: subject_id.to_string(),
            },
        );
    }
}

/// Error returned while parsing or validating a topology packet.
#[derive(Debug)]
pub enum RepositoryTopologyPacketError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<RepositoryTopologyValidationError>),
}

impl fmt::Display for RepositoryTopologyPacketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => write!(f, "failed to parse topology packet JSON: {error}"),
            Self::Validation(errors) => {
                write!(f, "repository topology packet has validation errors: ")?;
                for (index, error) in errors.iter().enumerate() {
                    if index > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for RepositoryTopologyPacketError {}

/// Cross-row validation error for a topology packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryTopologyValidationError {
    /// A record-kind tag does not match the stable contract.
    UnexpectedRecordKind {
        /// Observed record-kind tag.
        observed: String,
    },
    /// The packet schema version is unsupported.
    UnsupportedSchemaVersion {
        /// Observed schema version.
        observed: u32,
    },
    /// A descriptor carries no topology class.
    DescriptorMissingTopologyClass {
        /// Descriptor id.
        descriptor_id: String,
    },
    /// A non-plain descriptor carries no honesty label.
    DescriptorMissingHonestyLabel {
        /// Descriptor id.
        descriptor_id: String,
    },
    /// A surface row references an unknown descriptor.
    UnknownDescriptorRef {
        /// Row id.
        row_id: String,
        /// Unknown descriptor ref.
        descriptor_ref: String,
    },
    /// A partial row tried to claim full coverage.
    PartialRowClaimsFullCoverage {
        /// Row id.
        row_id: String,
    },
    /// A wrong-root row was not denied or explicitly narrowed.
    WrongRootNotDeniedOrNarrowed {
        /// Row id.
        row_id: String,
    },
    /// A read-only row allowed mutation.
    MutationAllowedForReadOnlyScope {
        /// Row id.
        row_id: String,
    },
    /// A network-bearing action lacks approval or policy posture.
    NetworkActionMissingApprovalPosture {
        /// Descriptor or row id.
        subject_id: String,
        /// Network-bearing action class.
        action: TopologyActionClass,
    },
    /// A `none_available` action used the wrong approval posture.
    NoneActionHasCommandPosture {
        /// Descriptor or row id.
        subject_id: String,
    },
    /// A support-export descriptor ref is unknown.
    UnknownSupportDescriptorRef {
        /// Unknown descriptor ref.
        descriptor_ref: String,
    },
    /// A support-export row ref is unknown.
    UnknownSupportSurfaceRowRef {
        /// Unknown surface row ref.
        row_ref: String,
    },
    /// The support export omits a required reconstruction field.
    SupportExportMissingField {
        /// Missing reconstruction field.
        field: String,
    },
    /// The support export embeds raw paths or raw object bytes.
    SupportExportEmbedsRawMaterial,
}

impl fmt::Display for RepositoryTopologyValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedRecordKind { observed } => {
                write!(f, "unexpected record kind {observed}")
            }
            Self::UnsupportedSchemaVersion { observed } => {
                write!(f, "unsupported schema version {observed}")
            }
            Self::DescriptorMissingTopologyClass { descriptor_id } => {
                write!(f, "descriptor {descriptor_id} has no topology class")
            }
            Self::DescriptorMissingHonestyLabel { descriptor_id } => {
                write!(f, "descriptor {descriptor_id} has no honesty label")
            }
            Self::UnknownDescriptorRef {
                row_id,
                descriptor_ref,
            } => write!(
                f,
                "row {row_id} references unknown descriptor {descriptor_ref}"
            ),
            Self::PartialRowClaimsFullCoverage { row_id } => {
                write!(f, "partial row {row_id} claims full coverage")
            }
            Self::WrongRootNotDeniedOrNarrowed { row_id } => {
                write!(f, "wrong-root row {row_id} is not denied or narrowed")
            }
            Self::MutationAllowedForReadOnlyScope { row_id } => {
                write!(f, "row {row_id} allows mutation for a read-only scope")
            }
            Self::NetworkActionMissingApprovalPosture { subject_id, action } => write!(
                f,
                "{subject_id} offers network-bearing action {} without approval posture",
                action.as_str()
            ),
            Self::NoneActionHasCommandPosture { subject_id } => {
                write!(
                    f,
                    "{subject_id} uses none_available without no-command posture"
                )
            }
            Self::UnknownSupportDescriptorRef { descriptor_ref } => {
                write!(
                    f,
                    "support export references unknown descriptor {descriptor_ref}"
                )
            }
            Self::UnknownSupportSurfaceRowRef { row_ref } => {
                write!(f, "support export references unknown surface row {row_ref}")
            }
            Self::SupportExportMissingField { field } => {
                write!(f, "support export missing reconstruction field {field}")
            }
            Self::SupportExportEmbedsRawMaterial => {
                write!(f, "support export embeds raw paths or raw object bytes")
            }
        }
    }
}
