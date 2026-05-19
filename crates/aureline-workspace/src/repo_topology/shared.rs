//! Shared vocabulary for repo-topology beta descriptors.
//!
//! The four descriptors in this module (`RepoRootDescriptor`,
//! `FetchDepthDescriptor`, `SubmoduleLink`, `LfsHydrationDescriptor`) all
//! share a small set of closed enums: client scope, redaction class,
//! freshness class, topology class, and affordance class. Each value
//! mirrors the corresponding `$defs` entry on the four boundary schemas in
//! `schemas/workspace/`.

use serde::{Deserialize, Serialize};

/// Client surface a descriptor or projection is consumed by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientScope {
    DesktopProduct,
    Cli,
    CompanionSurface,
    RemoteAgent,
    SdkOrApi,
    ManagedAdminSurface,
}

impl ClientScope {
    /// Stable token used in schemas, fixtures, and shell logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopProduct => "desktop_product",
            Self::Cli => "cli",
            Self::CompanionSurface => "companion_surface",
            Self::RemoteAgent => "remote_agent",
            Self::SdkOrApi => "sdk_or_api",
            Self::ManagedAdminSurface => "managed_admin_surface",
        }
    }
}

/// Redaction class controlling how a descriptor may be exported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    MetadataSafeDefault,
    OperatorOnlyRestricted,
    InternalSupportRestricted,
    SigningEvidenceOnly,
}

/// Freshness class for a descriptor snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    AuthoritativeLive,
    WarmCached,
    DegradedCached,
    Stale,
    Unverified,
}

/// Topology class an active root participates in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepoTopologyClass {
    CurrentRepoRoot,
    WorksetRoot,
    SparseCheckoutRoot,
    WorktreeRoot,
    PartialClonePromisorRoot,
    ShallowHistoryRoot,
    SubmoduleRoot,
    NestedIndependentRepoRoot,
    LfsHydrationBoundary,
}

/// Topology affordance class a descriptor may offer or a surface may
/// require before claiming broader coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyAffordanceClass {
    WidenWorksetScope,
    FetchMissingObjects,
    DeepenHistory,
    InitSubmodule,
    UpdateSubmoduleToPinnedCommit,
    OpenChildRepoRoot,
    HydrateLfsObjects,
    SwitchTargetRoot,
    OpenSparseCoverageInspector,
    ExportTopologyPacket,
    NoneAvailable,
}

impl TopologyAffordanceClass {
    /// Stable string token used by fixtures and audit packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WidenWorksetScope => "widen_workset_scope",
            Self::FetchMissingObjects => "fetch_missing_objects",
            Self::DeepenHistory => "deepen_history",
            Self::InitSubmodule => "init_submodule",
            Self::UpdateSubmoduleToPinnedCommit => "update_submodule_to_pinned_commit",
            Self::OpenChildRepoRoot => "open_child_repo_root",
            Self::HydrateLfsObjects => "hydrate_lfs_objects",
            Self::SwitchTargetRoot => "switch_target_root",
            Self::OpenSparseCoverageInspector => "open_sparse_coverage_inspector",
            Self::ExportTopologyPacket => "export_topology_packet",
            Self::NoneAvailable => "none_available",
        }
    }
}

/// Optional fixture metadata block (`__fixture__`) carried by every
/// boundary-record fixture. Surface code never reads this block; the
/// integration test suite does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureMetadata {
    pub name: String,
    pub scenario: String,
    pub doc_sections: Vec<String>,
    #[serde(flatten, default)]
    pub extras: serde_json::Map<String, serde_json::Value>,
}
