//! Monorepo package-set inventory, dependency-tree, and manifest/workset scope
//! truth packet for dependency-intelligence surfaces.
//!
//! This module publishes the canonical vocabulary and typed packet that keeps
//! package-set inventories, dependency trees, and the scope selector aligned
//! across desktop, CLI/headless, and support/export surfaces. It is built so
//! the three distinct scopes never collapse into one generic inventory:
//! whole-workspace truth, selected-manifest truth, and workset/slice truth are
//! separate [`ScopeView`] rows with their own loaded/matching/total counts.
//!
//! Each [`PackageInventoryRow`] carries a stable package identity, the
//! manifests that own it, owner/runtime context, converged-versus-diverged
//! state, per-manifest requested-versus-resolved claims, mirror/offline
//! freshness, and export-safe open-raw / open-manifest escapes. Each
//! [`DependencyEdgeRow`] preserves the owning manifest and discloses duplicate
//! or conflicting versions instead of hiding them behind resolver output text.
//!
//! The checked-in packet lives at
//! `artifacts/deps/m5/package-set-inventory-and-scope-truth.json` and is
//! embedded here so Rust consumers, CLI/headless output, support exports, and
//! release evidence all validate against the same source of truth.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported package-set inventory and scope-truth packet schema version.
pub const PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_RECORD_KIND: &str =
    "package_set_inventory_and_scope_truth";

/// Repo-relative path to the checked-in packet.
pub const PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_PATH: &str =
    "artifacts/deps/m5/package-set-inventory-and-scope-truth.json";

/// Embedded checked-in packet JSON.
pub const PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/package-set-inventory-and-scope-truth.json"
));

/// Ecosystem class for monorepo package-set rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EcosystemClass {
    /// Rust Cargo workspace and crate manifests.
    Cargo,
    /// Node package manifests using pnpm workspace semantics.
    NodePnpm,
}

impl EcosystemClass {
    /// Every ecosystem class claimed by this packet.
    pub const ALL: [Self; 2] = [Self::Cargo, Self::NodePnpm];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::NodePnpm => "node_pnpm",
        }
    }
}

/// Distinct scope a package-set view is rendered for.
///
/// These three scopes are deliberately separate so whole-workspace truth,
/// selected-manifest truth, and workset/slice truth never collapse into one
/// generic inventory surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeKind {
    /// The full resolved workspace package set across every manifest.
    FullWorkspace,
    /// A user-selected subset of manifests.
    SelectedManifests,
    /// A workset or slice limited to an active working set of files/manifests.
    WorksetSlice,
}

impl ScopeKind {
    /// Every scope kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::FullWorkspace,
        Self::SelectedManifests,
        Self::WorksetSlice,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullWorkspace => "full_workspace",
            Self::SelectedManifests => "selected_manifests",
            Self::WorksetSlice => "workset_slice",
        }
    }
}

/// Whether a package resolves to one converged version or diverges across the
/// manifests that own it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConvergenceState {
    /// Exactly one manifest owns the package.
    Unique,
    /// Multiple manifests own the package and all resolve to one version.
    Converged,
    /// Multiple resolved versions coexist without a hard conflict.
    Diverged,
    /// Multiple resolved versions coexist and at least one is a conflict.
    Conflicted,
}

impl ConvergenceState {
    /// Every convergence state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Unique,
        Self::Converged,
        Self::Diverged,
        Self::Conflicted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unique => "unique",
            Self::Converged => "converged",
            Self::Diverged => "diverged",
            Self::Conflicted => "conflicted",
        }
    }
}

/// Runtime context a manifest declares a dependency for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeClass {
    /// Normal runtime/build-graph dependency.
    Runtime,
    /// Development-only dependency (tests, tooling, dev server).
    Development,
    /// Build-script or native-build dependency.
    Build,
    /// Optional or feature-gated dependency.
    Optional,
}

impl RuntimeClass {
    /// Every runtime class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Runtime,
        Self::Development,
        Self::Build,
        Self::Optional,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Runtime => "runtime",
            Self::Development => "development",
            Self::Build => "build",
            Self::Optional => "optional",
        }
    }
}

/// How a manifest names a dependency edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyRelationClass {
    /// A directly declared requirement.
    Direct,
    /// A transitive requirement pulled in by another package.
    Transitive,
    /// A workspace-local member resolved from within the workspace.
    WorkspaceLocal,
    /// A filesystem path dependency.
    Path,
    /// A version-control (git) dependency.
    Vcs,
}

impl DependencyRelationClass {
    /// Every dependency relation class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Direct,
        Self::Transitive,
        Self::WorkspaceLocal,
        Self::Path,
        Self::Vcs,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Transitive => "transitive",
            Self::WorkspaceLocal => "workspace_local",
            Self::Path => "path",
            Self::Vcs => "vcs",
        }
    }
}

/// Freshness of the registry or mirror data backing a package row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// Live registry or local data current enough to trust.
    Live,
    /// Mirror data is reachable but stale.
    MirrorStale,
    /// Only an offline snapshot is available.
    OfflineSnapshotOnly,
    /// Freshness is unknown or stale beyond claimable bounds.
    UnknownOrStale,
}

impl FreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Live,
        Self::MirrorStale,
        Self::OfflineSnapshotOnly,
        Self::UnknownOrStale,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::MirrorStale => "mirror_stale",
            Self::OfflineSnapshotOnly => "offline_snapshot_only",
            Self::UnknownOrStale => "unknown_or_stale",
        }
    }

    /// Whether the state is anything other than live.
    pub const fn is_mirror_or_offline(self) -> bool {
        !matches!(self, Self::Live)
    }
}

/// Disclosure class for a dependency edge that may duplicate or conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DuplicateConflictClass {
    /// No duplicate or conflict is known for this edge.
    NoneKnown,
    /// Multiple resolved versions coexist for the same package.
    DuplicateVersions,
    /// Constraints conflict and cannot be unified.
    VersionConflict,
    /// Multiple requests were unified onto a single resolved version.
    FeatureUnification,
}

impl DuplicateConflictClass {
    /// Every duplicate/conflict class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoneKnown,
        Self::DuplicateVersions,
        Self::VersionConflict,
        Self::FeatureUnification,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneKnown => "none_known",
            Self::DuplicateVersions => "duplicate_versions",
            Self::VersionConflict => "version_conflict",
            Self::FeatureUnification => "feature_unification",
        }
    }

    /// Whether the class is anything other than `none_known`.
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::NoneKnown)
    }
}

/// Kind of export-safe escape offered from an inventory or tree row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenEscapeKind {
    /// Opens the raw resolver output backing a row.
    OpenRaw,
    /// Opens the owning manifest in the editor.
    OpenManifest,
}

impl OpenEscapeKind {
    /// Every open-escape kind, in declaration order.
    pub const ALL: [Self; 2] = [Self::OpenRaw, Self::OpenManifest];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRaw => "open_raw",
            Self::OpenManifest => "open_manifest",
        }
    }
}

/// Stable surface contract: which surfaces ingest this packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeSurfaceContract {
    /// Desktop package-set inventory surface.
    pub inventory_surface: String,
    /// Desktop dependency-tree surface.
    pub dependency_tree_surface: String,
    /// Scope selector / scope bar surface.
    pub scope_selector_surface: String,
    /// Package detail surface.
    pub package_detail_surface: String,
    /// CLI/headless inspect surface.
    pub cli_headless_surface: String,
    /// Help page describing the packet.
    pub help_page: String,
    /// Support-export channel.
    pub support_export_surface: String,
}

/// One manifest's requested-versus-resolved claim for a package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestVersionClaim {
    /// Manifest that owns this claim.
    pub manifest_path: String,
    /// Workspace member label that owns the manifest.
    pub workspace_member: String,
    /// Requested range or source as written in the manifest.
    pub requested_range_or_source: String,
    /// Resolved exact version or source identity.
    pub resolved_exact_version_or_source: String,
    /// How the manifest names this dependency.
    pub relation_class: DependencyRelationClass,
    /// Runtime context this manifest declares the dependency for.
    pub runtime_class: RuntimeClass,
}

/// An export-safe escape offered from a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenEscape {
    /// Escape kind.
    pub kind: OpenEscapeKind,
    /// Redaction-safe target reference (manifest path or raw-dump id).
    pub target_ref: String,
}

/// One package in the inventory, with stable identity and scope context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageInventoryRow {
    /// Stable package identity reused across every surface.
    pub package_id: String,
    /// Human-readable package name.
    pub package_name: String,
    /// Ecosystem this package belongs to.
    pub ecosystem: EcosystemClass,
    /// Manifests that own (declare) this package.
    pub owner_manifests: Vec<String>,
    /// Convergence state across the owning manifests.
    pub convergence_state: ConvergenceState,
    /// Per-manifest requested-versus-resolved claims.
    pub declared_versions: Vec<ManifestVersionClaim>,
    /// Summary of the resolved identity (converged version or divergence note).
    pub resolved_identity_summary: String,
    /// Mirror/offline freshness of the data behind this row.
    pub freshness_state: FreshnessState,
    /// Redaction-safe registry or mirror source label.
    pub source_label: String,
    /// Export-safe open-raw / open-manifest escapes.
    pub open_escapes: Vec<OpenEscape>,
}

/// One dependency-tree edge, preserving owning manifest and duplicate/conflict
/// disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyEdgeRow {
    /// Stable edge identity.
    pub edge_id: String,
    /// Manifest that declares this edge.
    pub owner_manifest: String,
    /// Depending package or workspace member label.
    pub from_label: String,
    /// Depended-upon package; resolves to a [`PackageInventoryRow`].
    pub to_package_id: String,
    /// How the owning manifest names the edge.
    pub relation_class: DependencyRelationClass,
    /// Runtime context the edge is declared for.
    pub runtime_class: RuntimeClass,
    /// Duplicate/conflict disclosure class.
    pub duplicate_conflict_class: DuplicateConflictClass,
    /// Human-readable disclosure note.
    pub disclosure_note: String,
}

/// One package-set scope view.
///
/// The three [`ScopeKind`]s are kept distinct: a whole-workspace view, a
/// selected-manifest view, and a workset/slice view each carry their own
/// honest loaded/matching/total counts and never collapse together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeView {
    /// Stable scope identity.
    pub scope_id: String,
    /// Which distinct scope this view renders.
    pub scope_kind: ScopeKind,
    /// Human-readable scope label.
    pub scope_label: String,
    /// Manifests that define this scope.
    pub included_manifests: Vec<String>,
    /// Package ids currently loaded into the view; resolve to inventory rows.
    pub member_package_ids: Vec<String>,
    /// Rows currently materialized (the virtualized window).
    pub loaded_count: usize,
    /// Rows matching the active filter within scope.
    pub matching_count: usize,
    /// Total rows in scope regardless of filter.
    pub total_count: usize,
    /// Whether the view is virtualized.
    pub virtualized: bool,
    /// Human-readable active filter label.
    pub filter_label: String,
    /// Whether the scope was widened server-side without local intent.
    ///
    /// Must always be `false`: there is no hidden server-side widening.
    pub server_side_widening: bool,
}

/// Summary counts derived from the rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageSetInventoryAndScopeTruthSummary {
    /// Total scope-view rows.
    pub scope_view_rows: usize,
    /// Full-workspace scope rows.
    pub full_workspace_scope_rows: usize,
    /// Selected-manifest scope rows.
    pub selected_manifest_scope_rows: usize,
    /// Workset/slice scope rows.
    pub workset_slice_scope_rows: usize,
    /// Total package inventory rows.
    pub package_rows: usize,
    /// Unique (single-owner) package rows.
    pub unique_package_rows: usize,
    /// Converged package rows.
    pub converged_package_rows: usize,
    /// Diverged package rows.
    pub diverged_package_rows: usize,
    /// Conflicted package rows.
    pub conflicted_package_rows: usize,
    /// Total dependency-edge rows.
    pub dependency_edge_rows: usize,
    /// Edge rows disclosing a duplicate or conflict.
    pub duplicate_or_conflict_edge_rows: usize,
    /// Package rows on stale mirror or offline data.
    pub mirror_or_offline_package_rows: usize,
}

/// One row of the redaction-safe export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageSetInventoryAndScopeTruthExportRow {
    /// Row id (scope id or package id).
    pub row_id: String,
    /// Row kind discriminator.
    pub row_kind: String,
    /// Ecosystem token, or a scope marker for scope rows.
    pub ecosystem: String,
    /// Scope or owner label.
    pub scope_label: String,
    /// Effective state token.
    pub effective_state: String,
    /// Human-readable summary.
    pub summary: String,
}

/// Redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageSetInventoryAndScopeTruthExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<PackageSetInventoryAndScopeTruthExportRow>,
}

/// Typed monorepo package-set inventory and scope-truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageSetInventoryAndScopeTruth {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Stable surface contract.
    pub surface_contract: ScopeSurfaceContract,
    /// Closed ecosystem vocabulary.
    pub ecosystem_classes: Vec<EcosystemClass>,
    /// Closed scope-kind vocabulary.
    pub scope_kinds: Vec<ScopeKind>,
    /// Closed convergence-state vocabulary.
    pub convergence_states: Vec<ConvergenceState>,
    /// Closed runtime-class vocabulary.
    pub runtime_classes: Vec<RuntimeClass>,
    /// Closed dependency-relation vocabulary.
    pub dependency_relation_classes: Vec<DependencyRelationClass>,
    /// Closed freshness-state vocabulary.
    pub freshness_states: Vec<FreshnessState>,
    /// Closed duplicate/conflict vocabulary.
    pub duplicate_conflict_classes: Vec<DuplicateConflictClass>,
    /// Closed open-escape vocabulary.
    pub open_escape_kinds: Vec<OpenEscapeKind>,
    /// Stable ecosystems claimed by this packet.
    pub claimed_stable_ecosystems: Vec<EcosystemClass>,
    /// Scope-view rows.
    #[serde(default)]
    pub scope_views: Vec<ScopeView>,
    /// Package inventory rows.
    #[serde(default)]
    pub packages: Vec<PackageInventoryRow>,
    /// Dependency-edge rows.
    #[serde(default)]
    pub dependency_edges: Vec<DependencyEdgeRow>,
    /// Summary counts.
    pub summary: PackageSetInventoryAndScopeTruthSummary,
}

impl PackageSetInventoryAndScopeTruth {
    /// Returns the scope view for `scope_id`.
    pub fn scope_view(&self, scope_id: &str) -> Option<&ScopeView> {
        self.scope_views.iter().find(|row| row.scope_id == scope_id)
    }

    /// Returns the package inventory row for `package_id`.
    pub fn package(&self, package_id: &str) -> Option<&PackageInventoryRow> {
        self.packages
            .iter()
            .find(|row| row.package_id == package_id)
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> PackageSetInventoryAndScopeTruthSummary {
        let scope_kind_count = |kind: ScopeKind| {
            self.scope_views
                .iter()
                .filter(|row| row.scope_kind == kind)
                .count()
        };
        let convergence_count = |state: ConvergenceState| {
            self.packages
                .iter()
                .filter(|row| row.convergence_state == state)
                .count()
        };
        PackageSetInventoryAndScopeTruthSummary {
            scope_view_rows: self.scope_views.len(),
            full_workspace_scope_rows: scope_kind_count(ScopeKind::FullWorkspace),
            selected_manifest_scope_rows: scope_kind_count(ScopeKind::SelectedManifests),
            workset_slice_scope_rows: scope_kind_count(ScopeKind::WorksetSlice),
            package_rows: self.packages.len(),
            unique_package_rows: convergence_count(ConvergenceState::Unique),
            converged_package_rows: convergence_count(ConvergenceState::Converged),
            diverged_package_rows: convergence_count(ConvergenceState::Diverged),
            conflicted_package_rows: convergence_count(ConvergenceState::Conflicted),
            dependency_edge_rows: self.dependency_edges.len(),
            duplicate_or_conflict_edge_rows: self
                .dependency_edges
                .iter()
                .filter(|row| row.duplicate_conflict_class.is_disclosed())
                .count(),
            mirror_or_offline_package_rows: self
                .packages
                .iter()
                .filter(|row| row.freshness_state.is_mirror_or_offline())
                .count(),
        }
    }

    /// Produces a redaction-safe export projection for UI, CLI, support, docs,
    /// release, and public proof consumers.
    pub fn export_projection(&self) -> PackageSetInventoryAndScopeTruthExportProjection {
        let mut rows = Vec::new();
        for scope in &self.scope_views {
            rows.push(PackageSetInventoryAndScopeTruthExportRow {
                row_id: scope.scope_id.clone(),
                row_kind: "scope_view".to_owned(),
                ecosystem: "workspace".to_owned(),
                scope_label: scope.scope_label.clone(),
                effective_state: scope.scope_kind.as_str().to_owned(),
                summary: format!(
                    "{} loaded {} matching {} total {} virtualized {}",
                    scope.scope_label,
                    scope.loaded_count,
                    scope.matching_count,
                    scope.total_count,
                    scope.virtualized
                ),
            });
        }
        for package in &self.packages {
            rows.push(PackageSetInventoryAndScopeTruthExportRow {
                row_id: package.package_id.clone(),
                row_kind: "package_inventory".to_owned(),
                ecosystem: package.ecosystem.as_str().to_owned(),
                scope_label: package.owner_manifests.join(", "),
                effective_state: package.convergence_state.as_str().to_owned(),
                summary: format!(
                    "{} {} owners {} freshness {}",
                    package.package_name,
                    package.resolved_identity_summary,
                    package.owner_manifests.len(),
                    package.freshness_state.as_str()
                ),
            });
        }
        PackageSetInventoryAndScopeTruthExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<PackageSetInventoryAndScopeTruthViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_scope_views(&mut violations);
        self.validate_packages(&mut violations);
        self.validate_dependency_edges(&mut violations);
        if self.summary != self.computed_summary() {
            violations.push(PackageSetInventoryAndScopeTruthViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<PackageSetInventoryAndScopeTruthViolation>) {
        if self.schema_version != PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_SCHEMA_VERSION {
            violations.push(
                PackageSetInventoryAndScopeTruthViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_RECORD_KIND {
            violations.push(
                PackageSetInventoryAndScopeTruthViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageSetInventoryAndScopeTruthViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, value) in [
            (
                "inventory_surface",
                &self.surface_contract.inventory_surface,
            ),
            (
                "dependency_tree_surface",
                &self.surface_contract.dependency_tree_surface,
            ),
            (
                "scope_selector_surface",
                &self.surface_contract.scope_selector_surface,
            ),
            (
                "package_detail_surface",
                &self.surface_contract.package_detail_surface,
            ),
            (
                "cli_headless_surface",
                &self.surface_contract.cli_headless_surface,
            ),
            ("help_page", &self.surface_contract.help_page),
            (
                "support_export_surface",
                &self.surface_contract.support_export_surface,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageSetInventoryAndScopeTruthViolation::EmptyField {
                    id: "<surface_contract>".to_owned(),
                    field_name: field,
                });
            }
        }
        let vocab_checks: [(&'static str, bool); 9] = [
            (
                "ecosystem_classes",
                self.ecosystem_classes == EcosystemClass::ALL.to_vec(),
            ),
            ("scope_kinds", self.scope_kinds == ScopeKind::ALL.to_vec()),
            (
                "convergence_states",
                self.convergence_states == ConvergenceState::ALL.to_vec(),
            ),
            (
                "runtime_classes",
                self.runtime_classes == RuntimeClass::ALL.to_vec(),
            ),
            (
                "dependency_relation_classes",
                self.dependency_relation_classes == DependencyRelationClass::ALL.to_vec(),
            ),
            (
                "freshness_states",
                self.freshness_states == FreshnessState::ALL.to_vec(),
            ),
            (
                "duplicate_conflict_classes",
                self.duplicate_conflict_classes == DuplicateConflictClass::ALL.to_vec(),
            ),
            (
                "open_escape_kinds",
                self.open_escape_kinds == OpenEscapeKind::ALL.to_vec(),
            ),
            (
                "claimed_stable_ecosystems",
                self.claimed_stable_ecosystems == EcosystemClass::ALL.to_vec(),
            ),
        ];
        for (field, ok) in vocab_checks {
            if !ok {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::ClosedVocabularyMismatch { field },
                );
            }
        }
    }

    fn validate_scope_views(
        &self,
        violations: &mut Vec<PackageSetInventoryAndScopeTruthViolation>,
    ) {
        let mut seen = BTreeSet::new();
        let mut kinds_seen = BTreeSet::new();
        for row in &self.scope_views {
            if !seen.insert(row.scope_id.clone()) {
                violations.push(PackageSetInventoryAndScopeTruthViolation::DuplicateRowId {
                    row_id: row.scope_id.clone(),
                    row_kind: "scope_view",
                });
            }
            kinds_seen.insert(row.scope_kind);
            for (field, value) in [
                ("scope_id", &row.scope_id),
                ("scope_label", &row.scope_label),
                ("filter_label", &row.filter_label),
            ] {
                if value.trim().is_empty() {
                    violations.push(PackageSetInventoryAndScopeTruthViolation::EmptyField {
                        id: row.scope_id.clone(),
                        field_name: field,
                    });
                }
            }
            if row.included_manifests.is_empty() {
                violations.push(PackageSetInventoryAndScopeTruthViolation::EmptyField {
                    id: row.scope_id.clone(),
                    field_name: "included_manifests",
                });
            }
            if row.server_side_widening {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::ServerSideWidening {
                        scope_id: row.scope_id.clone(),
                    },
                );
            }
            if row.loaded_count != row.member_package_ids.len() {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::LoadedCountMismatch {
                        scope_id: row.scope_id.clone(),
                    },
                );
            }
            if row.loaded_count > row.matching_count || row.matching_count > row.total_count {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::ScopeCountInversion {
                        scope_id: row.scope_id.clone(),
                    },
                );
            }
            for package_ref in &row.member_package_ids {
                if self.package(package_ref).is_none() {
                    violations.push(
                        PackageSetInventoryAndScopeTruthViolation::DanglingPackageRef {
                            row_id: row.scope_id.clone(),
                            package_ref: package_ref.clone(),
                        },
                    );
                }
            }
        }
        for required in ScopeKind::ALL {
            if !kinds_seen.contains(&required) {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::MissingCorpusState {
                        field: "scope_kind",
                        state: required.as_str(),
                    },
                );
            }
        }
    }

    fn validate_packages(&self, violations: &mut Vec<PackageSetInventoryAndScopeTruthViolation>) {
        let mut seen = BTreeSet::new();
        let mut convergence_seen = BTreeSet::new();
        let mut freshness_seen = BTreeSet::new();
        for row in &self.packages {
            if !seen.insert(row.package_id.clone()) {
                violations.push(PackageSetInventoryAndScopeTruthViolation::DuplicateRowId {
                    row_id: row.package_id.clone(),
                    row_kind: "package_inventory",
                });
            }
            convergence_seen.insert(row.convergence_state);
            freshness_seen.insert(row.freshness_state);
            self.validate_package(row, violations);
        }
        for required in ConvergenceState::ALL {
            if !convergence_seen.contains(&required) {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::MissingCorpusState {
                        field: "convergence_state",
                        state: required.as_str(),
                    },
                );
            }
        }
        for required in FreshnessState::ALL {
            if !freshness_seen.contains(&required) {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::MissingCorpusState {
                        field: "freshness_state",
                        state: required.as_str(),
                    },
                );
            }
        }
    }

    fn validate_package(
        &self,
        row: &PackageInventoryRow,
        violations: &mut Vec<PackageSetInventoryAndScopeTruthViolation>,
    ) {
        for (field, value) in [
            ("package_id", &row.package_id),
            ("package_name", &row.package_name),
            ("resolved_identity_summary", &row.resolved_identity_summary),
            ("source_label", &row.source_label),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageSetInventoryAndScopeTruthViolation::EmptyField {
                    id: row.package_id.clone(),
                    field_name: field,
                });
            }
        }
        if row.owner_manifests.is_empty() {
            violations.push(
                PackageSetInventoryAndScopeTruthViolation::MissingOwnerManifest {
                    package_id: row.package_id.clone(),
                },
            );
        }
        if row.declared_versions.is_empty() {
            violations.push(
                PackageSetInventoryAndScopeTruthViolation::MissingOwnerManifest {
                    package_id: row.package_id.clone(),
                },
            );
        }
        for claim in &row.declared_versions {
            for (field, value) in [
                ("manifest_path", &claim.manifest_path),
                ("workspace_member", &claim.workspace_member),
                (
                    "requested_range_or_source",
                    &claim.requested_range_or_source,
                ),
                (
                    "resolved_exact_version_or_source",
                    &claim.resolved_exact_version_or_source,
                ),
            ] {
                if value.trim().is_empty() {
                    violations.push(PackageSetInventoryAndScopeTruthViolation::EmptyField {
                        id: row.package_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
        // The declared owner manifests must line up with the per-manifest claims.
        let claim_manifests: BTreeSet<&str> = row
            .declared_versions
            .iter()
            .map(|claim| claim.manifest_path.as_str())
            .collect();
        let owner_manifests: BTreeSet<&str> =
            row.owner_manifests.iter().map(String::as_str).collect();
        if !row.declared_versions.is_empty() && claim_manifests != owner_manifests {
            violations.push(
                PackageSetInventoryAndScopeTruthViolation::OwnerManifestClaimMismatch {
                    package_id: row.package_id.clone(),
                },
            );
        }
        // Convergence state must agree with the resolved versions.
        if !row.declared_versions.is_empty() {
            let resolved: BTreeSet<&str> = row
                .declared_versions
                .iter()
                .map(|claim| claim.resolved_exact_version_or_source.as_str())
                .collect();
            let owners = row.declared_versions.len();
            let distinct = resolved.len();
            let consistent = match row.convergence_state {
                ConvergenceState::Unique => owners == 1,
                ConvergenceState::Converged => owners > 1 && distinct == 1,
                ConvergenceState::Diverged | ConvergenceState::Conflicted => {
                    owners > 1 && distinct > 1
                }
            };
            if !consistent {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::ConvergenceMismatch {
                        package_id: row.package_id.clone(),
                    },
                );
            }
        }
        // Export-safe escapes must offer both open-raw and open-manifest.
        let escape_kinds: BTreeSet<OpenEscapeKind> =
            row.open_escapes.iter().map(|escape| escape.kind).collect();
        for required in OpenEscapeKind::ALL {
            if !escape_kinds.contains(&required) {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::MissingOpenEscape {
                        package_id: row.package_id.clone(),
                        kind: required.as_str(),
                    },
                );
            }
        }
        for escape in &row.open_escapes {
            if escape.target_ref.trim().is_empty() {
                violations.push(PackageSetInventoryAndScopeTruthViolation::EmptyField {
                    id: row.package_id.clone(),
                    field_name: "open_escape_target_ref",
                });
            }
        }
    }

    fn validate_dependency_edges(
        &self,
        violations: &mut Vec<PackageSetInventoryAndScopeTruthViolation>,
    ) {
        let mut seen = BTreeSet::new();
        let mut duplicate_classes_seen = BTreeSet::new();
        for row in &self.dependency_edges {
            if !seen.insert(row.edge_id.clone()) {
                violations.push(PackageSetInventoryAndScopeTruthViolation::DuplicateRowId {
                    row_id: row.edge_id.clone(),
                    row_kind: "dependency_edge",
                });
            }
            duplicate_classes_seen.insert(row.duplicate_conflict_class);
            for (field, value) in [
                ("edge_id", &row.edge_id),
                ("owner_manifest", &row.owner_manifest),
                ("from_label", &row.from_label),
                ("to_package_id", &row.to_package_id),
                ("disclosure_note", &row.disclosure_note),
            ] {
                if value.trim().is_empty() {
                    violations.push(PackageSetInventoryAndScopeTruthViolation::EmptyField {
                        id: row.edge_id.clone(),
                        field_name: field,
                    });
                }
            }
            if self.package(&row.to_package_id).is_none() {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::DanglingPackageRef {
                        row_id: row.edge_id.clone(),
                        package_ref: row.to_package_id.clone(),
                    },
                );
            }
            if row.duplicate_conflict_class.is_disclosed() && row.disclosure_note.trim().is_empty()
            {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::DuplicateConflictUndisclosed {
                        edge_id: row.edge_id.clone(),
                    },
                );
            }
        }
        for required in DuplicateConflictClass::ALL {
            if !duplicate_classes_seen.contains(&required) {
                violations.push(
                    PackageSetInventoryAndScopeTruthViolation::MissingCorpusState {
                        field: "duplicate_conflict_class",
                        state: required.as_str(),
                    },
                );
            }
        }
    }
}

/// A validation violation for the package-set inventory and scope-truth packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageSetInventoryAndScopeTruthViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row, section, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once within its kind.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
        /// Row kind discriminator.
        row_kind: &'static str,
    },
    /// A scope member or edge endpoint references a missing package.
    DanglingPackageRef {
        /// Row id carrying the ref.
        row_id: String,
        /// Unresolvable package ref.
        package_ref: String,
    },
    /// A scope view's loaded count disagrees with its member list length.
    LoadedCountMismatch {
        /// Scope id.
        scope_id: String,
    },
    /// A scope view's loaded/matching/total counts are not monotonic.
    ScopeCountInversion {
        /// Scope id.
        scope_id: String,
    },
    /// A scope view claims hidden server-side widening.
    ServerSideWidening {
        /// Scope id.
        scope_id: String,
    },
    /// A package lacks owner-manifest truth.
    MissingOwnerManifest {
        /// Package id.
        package_id: String,
    },
    /// A package's owner manifests disagree with its per-manifest claims.
    OwnerManifestClaimMismatch {
        /// Package id.
        package_id: String,
    },
    /// A package's convergence state disagrees with its resolved versions.
    ConvergenceMismatch {
        /// Package id.
        package_id: String,
    },
    /// A package lacks an export-safe open escape.
    MissingOpenEscape {
        /// Package id.
        package_id: String,
        /// Missing escape kind token.
        kind: &'static str,
    },
    /// A duplicate/conflict edge lacks a disclosure note.
    DuplicateConflictUndisclosed {
        /// Edge id.
        edge_id: String,
    },
    /// A required corpus state is missing.
    MissingCorpusState {
        /// Field that must exercise the state.
        field: &'static str,
        /// Missing state token.
        state: &'static str,
    },
    /// Summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for PackageSetInventoryAndScopeTruthViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id, row_kind } => {
                write!(f, "duplicate {row_kind} row id {row_id}")
            }
            Self::DanglingPackageRef {
                row_id,
                package_ref,
            } => write!(f, "row {row_id} references missing package {package_ref}"),
            Self::LoadedCountMismatch { scope_id } => write!(
                f,
                "scope {scope_id} loaded_count disagrees with member_package_ids length"
            ),
            Self::ScopeCountInversion { scope_id } => write!(
                f,
                "scope {scope_id} loaded/matching/total counts are not monotonic"
            ),
            Self::ServerSideWidening { scope_id } => {
                write!(f, "scope {scope_id} claims hidden server-side widening")
            }
            Self::MissingOwnerManifest { package_id } => {
                write!(f, "package {package_id} lacks owner-manifest truth")
            }
            Self::OwnerManifestClaimMismatch { package_id } => write!(
                f,
                "package {package_id} owner_manifests disagree with declared_versions"
            ),
            Self::ConvergenceMismatch { package_id } => write!(
                f,
                "package {package_id} convergence_state disagrees with resolved versions"
            ),
            Self::MissingOpenEscape { package_id, kind } => {
                write!(f, "package {package_id} lacks {kind} open escape")
            }
            Self::DuplicateConflictUndisclosed { edge_id } => {
                write!(
                    f,
                    "edge {edge_id} discloses a duplicate/conflict without a note"
                )
            }
            Self::MissingCorpusState { field, state } => {
                write!(f, "packet corpus does not exercise {field} state {state}")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for PackageSetInventoryAndScopeTruthViolation {}

/// Loads the embedded package-set inventory and scope-truth packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`PackageSetInventoryAndScopeTruth`].
pub fn current_package_set_inventory_and_scope_truth(
) -> Result<PackageSetInventoryAndScopeTruth, serde_json::Error> {
    serde_json::from_str(PACKAGE_SET_INVENTORY_AND_SCOPE_TRUTH_JSON)
}

#[cfg(test)]
mod tests;
