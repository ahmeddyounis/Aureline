//! Canonical cross-ecosystem package-state descriptor — the reusable product
//! object that carries one package's requested identity, resolved identity,
//! source provenance, effective policy, and finding/suppression linkage across
//! every M5 package surface.
//!
//! Where
//! [`crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix`]
//! *freezes the vocabulary* — the canonical package-state labels, registry/auth
//! and lockfile-authority control objects, and the surface-binding contract —
//! this module *implements the descriptor that speaks it*. A
//! [`PackageStateDescriptor`] is the one product object the dependency graph,
//! package detail, advisories, license/compliance views, update proposals, the
//! CLI inspect surface, and support/export packets all reuse, so a dependency's
//! provenance and current state survive search, detail, finding, update, and
//! export flows without semantic collapse.
//!
//! Three properties hold by construction and are validated against the frozen
//! matrix:
//!
//! 1. **Requested-versus-resolved stays separate.** Every descriptor carries a
//!    [`RequestedIdentity`] *and* an optional [`ResolvedIdentity`] in distinct
//!    fields; a label the descriptor surfaces is sorted by its frozen
//!    [`IdentitySide`], so a requested constraint and a resolved fact can never
//!    flatten into one badge. Direct, transitive, workspace-local, and path/VCS
//!    are kept as four distinct [`DependencyRelation`] values, never collapsed.
//! 2. **No state overclaims certainty.** A descriptor records its
//!    [`ResolutionConfidence`]; when a package is auth-gated, offline-snapshot
//!    only, or stale/unknown, [`PackageStateDescriptor::can_claim_resolved_exact`]
//!    is `false` and [`PackageStateDescriptor::primary_message_class`] renders
//!    the specific offline/auth/unknown disclosure rather than a generic
//!    "package not found" or "install failed".
//! 3. **Every descriptor binds to one matrix.** A packet pins
//!    [`PackageStateDescriptors::references_matrix_id`] to the frozen matrix's
//!    `packet_id`, and every label a descriptor surfaces resolves to a frozen
//!    state row, so product, CLI, and support/export paths express the same
//!    governed vocabulary mechanically instead of by hand.
//!
//! The packet is checked in at
//! `artifacts/deps/m5/package-state-descriptors.json` and embedded here, so this
//! typed consumer and any CI gate agree on every descriptor without a cargo
//! build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque,
//! redacted ref. It carries no credential bodies, registry tokens, raw provider
//! payloads, or private registry URLs.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::{
    current_m5_package_state_matrix, AuthMode, LockfileAuthority, ManifestScopeClass,
    PackageStateLabel, PackageStateMessageClass, PackageSurface, RegistrySourceAuthority,
    ResolverIdentityClass, RollbackClass, SurfaceWriteAuthority,
};

/// Supported package-state descriptors packet schema version.
pub const PACKAGE_STATE_DESCRIPTORS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PACKAGE_STATE_DESCRIPTORS_RECORD_KIND: &str = "package_state_descriptors";

/// Repo-relative path to the checked-in packet.
pub const PACKAGE_STATE_DESCRIPTORS_PATH: &str = "artifacts/deps/m5/package-state-descriptors.json";

/// Embedded checked-in packet JSON.
pub const PACKAGE_STATE_DESCRIPTORS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/package-state-descriptors.json"
));

/// A marketed M5 package ecosystem a descriptor can belong to.
///
/// The descriptor vocabulary is identical across ecosystems; the ecosystem only
/// names which package manager produced the requested and resolved identity, so
/// cross-ecosystem ambiguity never hides behind a generic label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EcosystemKind {
    /// Cargo / crates.io.
    Cargo,
    /// Node with the pnpm package manager.
    NodePnpm,
    /// Python with pip.
    PythonPip,
    /// Any other qualified ecosystem.
    Other,
}

impl EcosystemKind {
    /// Every ecosystem, in declaration order.
    pub const ALL: [Self; 4] = [Self::Cargo, Self::NodePnpm, Self::PythonPip, Self::Other];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::NodePnpm => "node_pnpm",
            Self::PythonPip => "python_pip",
            Self::Other => "other",
        }
    }
}

/// How a resolved package relates to the target manifest.
///
/// The four relations map one-to-one onto the frozen resolved-identity labels
/// and are kept distinct so direct and transitive truth can never be flattened
/// into a single "installed" badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyRelation {
    /// Direct dependency of the target manifest.
    Direct,
    /// Transitive dependency resolved through another package.
    Transitive,
    /// Workspace-local member dependency.
    WorkspaceLocal,
    /// Filesystem-path or version-control source.
    PathOrVcs,
}

impl DependencyRelation {
    /// Every relation, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Direct,
        Self::Transitive,
        Self::WorkspaceLocal,
        Self::PathOrVcs,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Transitive => "transitive",
            Self::WorkspaceLocal => "workspace_local",
            Self::PathOrVcs => "path_or_vcs",
        }
    }

    /// The frozen package-state label this relation surfaces.
    pub const fn state_label(self) -> PackageStateLabel {
        match self {
            Self::Direct => PackageStateLabel::Direct,
            Self::Transitive => PackageStateLabel::Transitive,
            Self::WorkspaceLocal => PackageStateLabel::WorkspaceLocal,
            Self::PathOrVcs => PackageStateLabel::PathOrVcsSource,
        }
    }

    /// Whether this relation is sourced from a registry rather than a path/VCS or
    /// workspace member.
    pub const fn is_registry_sourced(self) -> bool {
        matches!(self, Self::Direct | Self::Transitive)
    }

    /// Whether this relation is a filesystem-path or version-control source.
    pub const fn is_path_or_vcs(self) -> bool {
        matches!(self, Self::PathOrVcs)
    }
}

/// The kind of source the manifest *requested* for a package.
///
/// This is the requested side of provenance; it is recorded independently of the
/// resolved [`DependencyRelation`] so a request for a registry range that
/// resolved to a workspace override stays legible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestedSourceKind {
    /// A version range against a registry.
    RegistryRange,
    /// A workspace-local member path.
    WorkspaceLocalPath,
    /// A filesystem path outside the workspace.
    FilesystemPath,
    /// A version-control reference (branch, tag, or revision).
    VersionControlRef,
}

impl RequestedSourceKind {
    /// Every requested source kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RegistryRange,
        Self::WorkspaceLocalPath,
        Self::FilesystemPath,
        Self::VersionControlRef,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegistryRange => "registry_range",
            Self::WorkspaceLocalPath => "workspace_local_path",
            Self::FilesystemPath => "filesystem_path",
            Self::VersionControlRef => "version_control_ref",
        }
    }

    /// Whether the requested source is a filesystem path or version-control ref.
    pub const fn is_path_or_vcs(self) -> bool {
        matches!(self, Self::FilesystemPath | Self::VersionControlRef)
    }
}

/// How confidently the resolved identity reflects current upstream truth.
///
/// This is the non-overclaim axis: a package can be pinned to an exact ref yet
/// still be sourced only from a mirror, an offline snapshot, or be wholly
/// unresolved because auth was never satisfied. Each level decides whether the
/// descriptor may claim an exact resolution and which specific disclosure it
/// must render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionConfidence {
    /// Resolved against a reachable authoritative source; may claim exact.
    ResolvedAuthoritative,
    /// Resolved, but only from a local cache or enterprise mirror; the source
    /// must be disclosed even though the pin is exact.
    ResolvedFromCacheOrMirror,
    /// Only an offline snapshot is available; the pin may be stale and cannot be
    /// claimed as the current resolution.
    OfflineSnapshotOnly,
    /// Registry auth is required and unsatisfied; the package is unresolved.
    AuthGatedUnresolved,
    /// The resolution could not be established or is stale.
    StaleOrUnknown,
}

impl ResolutionConfidence {
    /// Every confidence level, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ResolvedAuthoritative,
        Self::ResolvedFromCacheOrMirror,
        Self::OfflineSnapshotOnly,
        Self::AuthGatedUnresolved,
        Self::StaleOrUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedAuthoritative => "resolved_authoritative",
            Self::ResolvedFromCacheOrMirror => "resolved_from_cache_or_mirror",
            Self::OfflineSnapshotOnly => "offline_snapshot_only",
            Self::AuthGatedUnresolved => "auth_gated_unresolved",
            Self::StaleOrUnknown => "stale_or_unknown",
        }
    }

    /// Whether a [`ResolvedIdentity`] must be present at this confidence.
    ///
    /// Authoritative, cache/mirror, and offline-snapshot resolutions all carry an
    /// exact pinned ref; auth-gated and stale/unknown states are unresolved and
    /// must not assert one.
    pub const fn carries_resolved_identity(self) -> bool {
        matches!(
            self,
            Self::ResolvedAuthoritative
                | Self::ResolvedFromCacheOrMirror
                | Self::OfflineSnapshotOnly
        )
    }

    /// Whether the descriptor may claim an exact, current resolution.
    ///
    /// An offline snapshot has a pinned ref but may not reflect upstream, so it
    /// cannot claim exact; auth-gated and stale states are unresolved. Only an
    /// authoritative or cache/mirror resolution may claim exact.
    pub const fn can_claim_resolved_exact(self) -> bool {
        matches!(
            self,
            Self::ResolvedAuthoritative | Self::ResolvedFromCacheOrMirror
        )
    }

    /// Whether the descriptor must disclose its resolution environment rather
    /// than reading as a clean upstream resolution.
    pub const fn must_disclose_environment(self) -> bool {
        !matches!(self, Self::ResolvedAuthoritative)
    }

    /// The specific environment/indeterminate label this confidence surfaces, if
    /// any. Authoritative and cache/mirror resolutions surface none here; their
    /// source is disclosed through the registry source instead.
    pub const fn environment_label(self) -> Option<PackageStateLabel> {
        match self {
            Self::ResolvedAuthoritative | Self::ResolvedFromCacheOrMirror => None,
            Self::OfflineSnapshotOnly => Some(PackageStateLabel::OfflineSnapshotOnly),
            Self::AuthGatedUnresolved => Some(PackageStateLabel::AuthRequired),
            Self::StaleOrUnknown => Some(PackageStateLabel::UnknownOrStale),
        }
    }
}

/// The kind of finding overlaid on a resolved package.
///
/// Each kind maps one-to-one onto a frozen finding-overlay label and is kept
/// separate from package identity, so an open advisory never silently rewrites
/// the resolved version it sits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// An advisory is open against the package.
    AdvisoryOpen,
    /// An advisory is suppressed until a stated expiry or condition.
    SuppressedUntil,
    /// License review is required before the package may ship.
    LicenseReviewRequired,
}

impl FindingKind {
    /// Every finding kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::AdvisoryOpen,
        Self::SuppressedUntil,
        Self::LicenseReviewRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryOpen => "advisory_open",
            Self::SuppressedUntil => "suppressed_until",
            Self::LicenseReviewRequired => "license_review_required",
        }
    }

    /// The frozen package-state label this finding surfaces.
    pub const fn state_label(self) -> PackageStateLabel {
        match self {
            Self::AdvisoryOpen => PackageStateLabel::AdvisoryOpen,
            Self::SuppressedUntil => PackageStateLabel::SuppressedUntil,
            Self::LicenseReviewRequired => PackageStateLabel::LicenseReviewRequired,
        }
    }

    /// Whether this finding requires suppression linkage (an actor/expiry).
    pub const fn requires_suppression_linkage(self) -> bool {
        matches!(self, Self::SuppressedUntil)
    }
}

/// What the manifest or policy requested for a package, before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestedIdentity {
    /// Ecosystem the request belongs to.
    pub ecosystem: EcosystemKind,
    /// Package name as written in the manifest.
    pub package_name: String,
    /// Requested range, path, or redacted VCS ref; never a raw URL or token.
    pub requested_ref: String,
    /// The kind of source requested.
    pub requested_source: RequestedSourceKind,
    /// Manifest scope the request belongs to.
    pub manifest_scope: ManifestScopeClass,
    /// Whether policy pins this request to an exact version or source.
    pub policy_pinned: bool,
}

impl RequestedIdentity {
    /// The frozen requested-constraint labels this request surfaces.
    ///
    /// A policy pin is the only requested-side label; it is reported here, never
    /// merged into the resolved identity.
    pub fn requested_labels(&self) -> Vec<PackageStateLabel> {
        let mut labels = Vec::new();
        if self.policy_pinned {
            labels.push(PackageStateLabel::PolicyPinned);
        }
        labels
    }
}

/// What the resolver produced for a package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedIdentity {
    /// How the resolved package relates to the target manifest.
    pub relation: DependencyRelation,
    /// Exact resolved version, commit, path, or snapshot id; never a raw URL.
    pub resolved_ref: String,
    /// Registry or mirror source the resolution came from; absent for a
    /// workspace-local member or a path/VCS source, which has no registry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry_source: Option<RegistrySourceAuthority>,
    /// Resolver that produced the resolution.
    pub resolver_identity: ResolverIdentityClass,
    /// Authority governing the resolved set for this manifest scope.
    pub lockfile_authority: LockfileAuthority,
}

impl ResolvedIdentity {
    /// The frozen resolved-identity labels this resolution surfaces.
    ///
    /// Always the relation label; [`PackageStateLabel::ResolvedExact`] is added by
    /// the descriptor only when its confidence permits an exact claim.
    pub fn relation_label(&self) -> PackageStateLabel {
        self.relation.state_label()
    }
}

/// A finding overlaid on a resolved package, with optional suppression linkage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FindingOverlay {
    /// The kind of finding.
    pub kind: FindingKind,
    /// Opaque advisory, license, or finding id; never a raw payload.
    pub finding_ref: String,
    /// Suppression record ref; present only for a suppressed finding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppression_ref: Option<String>,
    /// Human-readable expiry or condition label for a suppressed finding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_label: Option<String>,
}

impl FindingOverlay {
    /// Whether this overlay is internally consistent: a suppressed finding must
    /// carry suppression linkage, and a non-suppressed one must not.
    pub fn is_consistent(&self) -> bool {
        if self.kind.requires_suppression_linkage() {
            self.suppression_ref.is_some() && self.expiry_label.is_some()
        } else {
            self.suppression_ref.is_none() && self.expiry_label.is_none()
        }
    }

    /// The frozen finding-overlay label this overlay surfaces.
    pub fn state_label(&self) -> PackageStateLabel {
        self.kind.state_label()
    }
}

/// The canonical cross-ecosystem package-state descriptor.
///
/// One descriptor describes one package in one manifest scope: what was
/// requested, what (if anything) resolved, where it came from, how confidently,
/// what policy pins it, and which findings overlay it. The descriptor is the
/// single product object every M5 package surface reuses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageStateDescriptor {
    /// Stable descriptor id.
    pub descriptor_id: String,
    /// What the manifest or policy requested.
    pub requested: RequestedIdentity,
    /// What the resolver produced; absent when the package is unresolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedIdentity>,
    /// Auth mode used (or required) to reach the source.
    pub auth_mode: AuthMode,
    /// How confidently the resolution reflects current upstream truth.
    pub confidence: ResolutionConfidence,
    /// Rollback class for a mutation of this package.
    pub rollback_class: RollbackClass,
    /// Findings overlaid on the package.
    #[serde(default)]
    pub findings: Vec<FindingOverlay>,
    /// Redacted registry/source label safe for support exports; never a URL or
    /// token.
    pub redacted_source_label: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl PackageStateDescriptor {
    /// The ecosystem this descriptor belongs to.
    pub fn ecosystem(&self) -> EcosystemKind {
        self.requested.ecosystem
    }

    /// The resolved relation, if the package resolved.
    pub fn relation(&self) -> Option<DependencyRelation> {
        self.resolved.as_ref().map(|r| r.relation)
    }

    /// Whether the package resolved to a direct dependency.
    pub fn is_direct(&self) -> bool {
        self.relation() == Some(DependencyRelation::Direct)
    }

    /// Whether the package resolved to a transitive dependency.
    pub fn is_transitive(&self) -> bool {
        self.relation() == Some(DependencyRelation::Transitive)
    }

    /// Whether the package resolved to a workspace-local member.
    pub fn is_workspace_local(&self) -> bool {
        self.relation() == Some(DependencyRelation::WorkspaceLocal)
    }

    /// Whether the package is sourced from a filesystem path or VCS, on either
    /// the requested or resolved side.
    pub fn is_path_or_vcs(&self) -> bool {
        self.requested.requested_source.is_path_or_vcs()
            || self.relation().is_some_and(|r| r.is_path_or_vcs())
    }

    /// Whether the package is sourced from a registry rather than a path/VCS or
    /// workspace member.
    pub fn is_registry_sourced(&self) -> bool {
        !self.is_path_or_vcs()
            && self
                .relation()
                .is_some_and(DependencyRelation::is_registry_sourced)
    }

    /// Whether policy pins the requested constraint.
    pub fn is_policy_pinned(&self) -> bool {
        self.requested.policy_pinned
    }

    /// Whether reaching the package is blocked on unsatisfied auth.
    pub fn is_auth_gated(&self) -> bool {
        self.confidence == ResolutionConfidence::AuthGatedUnresolved
            || self.auth_mode.blocks_until_satisfied()
    }

    /// Whether the package state is stale, offline-only, auth-gated, or unknown —
    /// the conditions under which the descriptor must not overclaim certainty.
    pub fn must_disclose_environment(&self) -> bool {
        self.confidence.must_disclose_environment()
    }

    /// Whether the descriptor may claim an exact, current resolution.
    ///
    /// `false` whenever the state is offline-snapshot only, auth-gated, or
    /// stale/unknown, so package operations never overclaim certainty.
    pub fn can_claim_resolved_exact(&self) -> bool {
        self.resolved.is_some() && self.confidence.can_claim_resolved_exact()
    }

    /// Whether the package carries a pinned ref at all, even one that may be a
    /// stale offline snapshot.
    pub fn has_pinned_ref(&self) -> bool {
        self.resolved.is_some() && self.confidence.carries_resolved_identity()
    }

    /// The environment/indeterminate label the confidence surfaces, if any.
    pub fn environment_label(&self) -> Option<PackageStateLabel> {
        self.confidence.environment_label()
    }

    /// Open (non-suppressed) advisory and license findings.
    pub fn open_findings(&self) -> impl Iterator<Item = &FindingOverlay> {
        self.findings.iter().filter(|f| f.suppression_ref.is_none())
    }

    /// The number of open (non-suppressed) findings.
    pub fn open_finding_count(&self) -> usize {
        self.open_findings().count()
    }

    /// The frozen requested-constraint labels this descriptor surfaces.
    pub fn requested_labels(&self) -> BTreeSet<PackageStateLabel> {
        self.requested.requested_labels().into_iter().collect()
    }

    /// The frozen resolved-identity labels this descriptor surfaces.
    pub fn resolved_labels(&self) -> BTreeSet<PackageStateLabel> {
        let mut labels = BTreeSet::new();
        if let Some(resolved) = &self.resolved {
            labels.insert(resolved.relation_label());
            if self.can_claim_resolved_exact() {
                labels.insert(PackageStateLabel::ResolvedExact);
            }
        }
        labels
    }

    /// The frozen finding-overlay labels this descriptor surfaces.
    pub fn finding_labels(&self) -> BTreeSet<PackageStateLabel> {
        self.findings
            .iter()
            .map(FindingOverlay::state_label)
            .collect()
    }

    /// Every frozen package-state label this descriptor surfaces, across the
    /// requested, resolved, finding, and environment sides.
    pub fn applicable_labels(&self) -> BTreeSet<PackageStateLabel> {
        let mut labels = self.requested_labels();
        labels.extend(self.resolved_labels());
        labels.extend(self.finding_labels());
        if let Some(env) = self.environment_label() {
            labels.insert(env);
        }
        labels
    }

    /// The single most salient specific message class the descriptor renders.
    ///
    /// Environment disclosures win first so an auth-gated, offline, or stale
    /// package always shows its specific disclosure; a cache/mirror resolution
    /// discloses its source; otherwise the relation message leads. The result is
    /// never a generic collapse message.
    pub fn primary_message_class(&self) -> PackageStateMessageClass {
        match self.confidence {
            ResolutionConfidence::AuthGatedUnresolved => {
                PackageStateMessageClass::AuthRequiredDisclosure
            }
            ResolutionConfidence::StaleOrUnknown => {
                PackageStateMessageClass::UnknownOrStaleDisclosure
            }
            ResolutionConfidence::OfflineSnapshotOnly => {
                PackageStateMessageClass::OfflineSnapshotDisclosure
            }
            ResolutionConfidence::ResolvedFromCacheOrMirror => self
                .resolved
                .as_ref()
                .and_then(|r| r.registry_source)
                .map(RegistrySourceAuthority::canonical_message_class)
                .unwrap_or(PackageStateMessageClass::CacheOnlySource),
            ResolutionConfidence::ResolvedAuthoritative => self
                .resolved
                .as_ref()
                .map(|r| r.relation_label().canonical_message_class())
                .unwrap_or(PackageStateMessageClass::UnknownOrStaleDisclosure),
        }
    }

    /// Whether the requested and resolved identities are kept separate — they
    /// never share a surfaced label.
    pub fn requested_and_resolved_separate(&self) -> bool {
        self.requested_labels().is_disjoint(&self.resolved_labels())
    }

    /// Projects the descriptor into the canonical per-package view reused by
    /// package detail, the dependency tree, and finding/license cards.
    pub fn view(&self) -> PackageStateView {
        PackageStateView {
            descriptor_id: self.descriptor_id.clone(),
            ecosystem: self.ecosystem().as_str().to_owned(),
            package_name: self.requested.package_name.clone(),
            requested: RequestedView {
                requested_ref: self.requested.requested_ref.clone(),
                requested_source: self.requested.requested_source.as_str().to_owned(),
                manifest_scope: self.requested.manifest_scope.as_str().to_owned(),
                policy_pinned: self.requested.policy_pinned,
            },
            resolved: self.resolved.as_ref().map(|r| ResolvedView {
                relation: r.relation.as_str().to_owned(),
                resolved_ref: r.resolved_ref.clone(),
                registry_source: r.registry_source.map(|s| s.as_str().to_owned()),
                resolver_identity: r.resolver_identity.as_str().to_owned(),
                lockfile_authority: r.lockfile_authority.as_str().to_owned(),
            }),
            is_direct: self.is_direct(),
            is_transitive: self.is_transitive(),
            is_workspace_local: self.is_workspace_local(),
            is_registry_sourced: self.is_registry_sourced(),
            is_path_or_vcs: self.is_path_or_vcs(),
            policy_pinned: self.is_policy_pinned(),
            auth_gated: self.is_auth_gated(),
            confidence: self.confidence.as_str().to_owned(),
            can_claim_resolved_exact: self.can_claim_resolved_exact(),
            has_pinned_ref: self.has_pinned_ref(),
            must_disclose_environment: self.must_disclose_environment(),
            primary_message_class: self.primary_message_class().as_str().to_owned(),
            applicable_labels: self
                .applicable_labels()
                .iter()
                .map(|l| l.as_str().to_owned())
                .collect(),
            open_finding_count: self.open_finding_count(),
            redacted_source_label: self.redacted_source_label.clone(),
        }
    }

    /// Projects each finding into a finding card reused by advisory, license, and
    /// compliance surfaces.
    pub fn finding_cards(&self) -> Vec<FindingCardView> {
        self.findings
            .iter()
            .map(|f| FindingCardView {
                descriptor_id: self.descriptor_id.clone(),
                package_name: self.requested.package_name.clone(),
                kind: f.kind.as_str().to_owned(),
                state_label: f.state_label().as_str().to_owned(),
                finding_ref: f.finding_ref.clone(),
                suppressed: f.suppression_ref.is_some(),
                suppression_ref: f.suppression_ref.clone(),
                expiry_label: f.expiry_label.clone(),
                // A finding card never asserts the package is cleanly resolved
                // when the environment is degraded.
                on_resolved_exact: self.can_claim_resolved_exact(),
            })
            .collect()
    }

    /// The license/compliance row for this descriptor, if license review is
    /// required.
    pub fn license_compliance_row(&self) -> Option<FindingCardView> {
        self.finding_cards()
            .into_iter()
            .find(|c| c.kind == FindingKind::LicenseReviewRequired.as_str())
    }

    /// Projects the descriptor into an update-proposal view that gates apply on
    /// auth, lockfile authority, and resolution confidence rather than
    /// overclaiming that an update is safe.
    pub fn update_proposal(&self) -> UpdateProposalView {
        let blocked_reason = self.update_block_reason();
        UpdateProposalView {
            descriptor_id: self.descriptor_id.clone(),
            package_name: self.requested.package_name.clone(),
            from_resolved_ref: self.resolved.as_ref().map(|r| r.resolved_ref.clone()),
            to_requested_ref: self.requested.requested_ref.clone(),
            relation: self.relation().map(|r| r.as_str().to_owned()),
            lockfile_authority: self
                .resolved
                .as_ref()
                .map(|r| r.lockfile_authority.as_str().to_owned()),
            rollback_class: self.rollback_class.as_str().to_owned(),
            can_apply: blocked_reason.is_none(),
            blocked_reason,
        }
    }

    fn update_block_reason(&self) -> Option<String> {
        if self.is_auth_gated() {
            return Some("registry auth is required and unsatisfied".to_owned());
        }
        match self.confidence {
            ResolutionConfidence::StaleOrUnknown => {
                return Some("package state is stale or unknown".to_owned());
            }
            ResolutionConfidence::OfflineSnapshotOnly => {
                return Some("only an offline snapshot is available".to_owned());
            }
            _ => {}
        }
        if let Some(resolved) = &self.resolved {
            if resolved.lockfile_authority.blocks_until_reconciled() {
                return Some("lockfile and manifest diverge and must be reconciled".to_owned());
            }
        }
        None
    }

    /// Projects the descriptor into a redaction-safe export row reused by
    /// support/export packets and the CLI inspect surface.
    pub fn export_row(&self) -> PackageStateExportRow {
        PackageStateExportRow {
            descriptor_id: self.descriptor_id.clone(),
            ecosystem: self.ecosystem().as_str().to_owned(),
            package_name: self.requested.package_name.clone(),
            requested_source: self.requested.requested_source.as_str().to_owned(),
            relation: self.relation().map(|r| r.as_str().to_owned()),
            confidence: self.confidence.as_str().to_owned(),
            primary_message_class: self.primary_message_class().as_str().to_owned(),
            can_claim_resolved_exact: self.can_claim_resolved_exact(),
            must_disclose_environment: self.must_disclose_environment(),
            policy_pinned: self.is_policy_pinned(),
            auth_gated: self.is_auth_gated(),
            open_finding_count: self.open_finding_count(),
            applicable_labels: self
                .applicable_labels()
                .iter()
                .map(|l| l.as_str().to_owned())
                .collect(),
            redacted_source_label: self.redacted_source_label.clone(),
        }
    }

    /// Projects the descriptor onto a specific marketed surface, pinning the
    /// write authority that surface may carry from the frozen matrix.
    pub fn surface_projection(&self, surface: PackageSurface) -> PackageStateSurfaceProjection {
        PackageStateSurfaceProjection {
            surface: surface.as_str().to_owned(),
            write_authority: surface.canonical_write_authority().as_str().to_owned(),
            can_mutate: surface.canonical_write_authority().can_mutate(),
            redacted: matches!(
                surface.canonical_write_authority(),
                SurfaceWriteAuthority::RedactedExport
            ),
            view: self.view(),
        }
    }

    /// Whether the descriptor is internally consistent against the frozen
    /// contract: resolved presence matches confidence, identities stay separate,
    /// findings carry the right linkage, and the primary message never collapses.
    pub fn is_consistent(&self) -> bool {
        self.resolved.is_some() == self.confidence.carries_resolved_identity()
            && self.requested_and_resolved_separate()
            && self.findings.iter().all(FindingOverlay::is_consistent)
            && self.primary_message_class().is_specific()
    }
}

/// Requested-side view of a package, surfaced separately from the resolved side.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestedView {
    /// Requested range, path, or redacted VCS ref.
    pub requested_ref: String,
    /// Requested source kind token.
    pub requested_source: String,
    /// Manifest scope token.
    pub manifest_scope: String,
    /// Whether policy pins the request.
    pub policy_pinned: bool,
}

/// Resolved-side view of a package, surfaced separately from the requested side.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedView {
    /// Dependency relation token.
    pub relation: String,
    /// Exact resolved ref.
    pub resolved_ref: String,
    /// Registry source token, absent for workspace-local or path/VCS sources.
    pub registry_source: Option<String>,
    /// Resolver identity token.
    pub resolver_identity: String,
    /// Lockfile authority token.
    pub lockfile_authority: String,
}

/// The canonical per-package view reused by package detail, the dependency tree,
/// and finding/license cards.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageStateView {
    /// Descriptor id.
    pub descriptor_id: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Package name.
    pub package_name: String,
    /// Requested-side view.
    pub requested: RequestedView,
    /// Resolved-side view, absent when unresolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedView>,
    /// Whether the package is a direct dependency.
    pub is_direct: bool,
    /// Whether the package is a transitive dependency.
    pub is_transitive: bool,
    /// Whether the package is a workspace-local member.
    pub is_workspace_local: bool,
    /// Whether the package is registry-sourced.
    pub is_registry_sourced: bool,
    /// Whether the package is path/VCS-sourced.
    pub is_path_or_vcs: bool,
    /// Whether policy pins the request.
    pub policy_pinned: bool,
    /// Whether the package is auth-gated.
    pub auth_gated: bool,
    /// Resolution confidence token.
    pub confidence: String,
    /// Whether the descriptor may claim an exact, current resolution.
    pub can_claim_resolved_exact: bool,
    /// Whether the descriptor carries any pinned ref (even a stale snapshot).
    pub has_pinned_ref: bool,
    /// Whether the resolution environment must be disclosed.
    pub must_disclose_environment: bool,
    /// Primary specific message-class token.
    pub primary_message_class: String,
    /// Every applicable package-state label token.
    pub applicable_labels: Vec<String>,
    /// Number of open (non-suppressed) findings.
    pub open_finding_count: usize,
    /// Redacted source label.
    pub redacted_source_label: String,
}

/// A finding card reused by advisory, license, and compliance surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingCardView {
    /// Descriptor id.
    pub descriptor_id: String,
    /// Package name.
    pub package_name: String,
    /// Finding-kind token.
    pub kind: String,
    /// Frozen state-label token.
    pub state_label: String,
    /// Opaque finding ref.
    pub finding_ref: String,
    /// Whether the finding is suppressed.
    pub suppressed: bool,
    /// Suppression record ref, if suppressed.
    pub suppression_ref: Option<String>,
    /// Expiry/condition label, if suppressed.
    pub expiry_label: Option<String>,
    /// Whether the package the finding sits on is cleanly resolved exact.
    pub on_resolved_exact: bool,
}

/// An update-proposal view that gates apply on auth, lockfile authority, and
/// confidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateProposalView {
    /// Descriptor id.
    pub descriptor_id: String,
    /// Package name.
    pub package_name: String,
    /// The current resolved ref, if any.
    pub from_resolved_ref: Option<String>,
    /// The requested target ref.
    pub to_requested_ref: String,
    /// Dependency relation token, if resolved.
    pub relation: Option<String>,
    /// Lockfile authority token, if resolved.
    pub lockfile_authority: Option<String>,
    /// Rollback class token.
    pub rollback_class: String,
    /// Whether the update may be applied.
    pub can_apply: bool,
    /// Why the update is blocked, if it is.
    pub blocked_reason: Option<String>,
}

/// A redaction-safe export row reused by support/export packets and CLI inspect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageStateExportRow {
    /// Descriptor id.
    pub descriptor_id: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Package name.
    pub package_name: String,
    /// Requested source kind token.
    pub requested_source: String,
    /// Dependency relation token, if resolved.
    pub relation: Option<String>,
    /// Resolution confidence token.
    pub confidence: String,
    /// Primary specific message-class token.
    pub primary_message_class: String,
    /// Whether the descriptor may claim an exact, current resolution.
    pub can_claim_resolved_exact: bool,
    /// Whether the resolution environment must be disclosed.
    pub must_disclose_environment: bool,
    /// Whether policy pins the request.
    pub policy_pinned: bool,
    /// Whether the package is auth-gated.
    pub auth_gated: bool,
    /// Number of open findings.
    pub open_finding_count: usize,
    /// Every applicable package-state label token.
    pub applicable_labels: Vec<String>,
    /// Redacted source label.
    pub redacted_source_label: String,
}

/// A descriptor projected onto a specific marketed surface with its pinned write
/// authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageStateSurfaceProjection {
    /// Package surface token.
    pub surface: String,
    /// Write authority token pinned by the frozen matrix.
    pub write_authority: String,
    /// Whether the surface may mutate the package.
    pub can_mutate: bool,
    /// Whether the surface produces a redacted export.
    pub redacted: bool,
    /// The canonical per-package view.
    pub view: PackageStateView,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageStateDescriptorsSummary {
    /// Total descriptors.
    pub total_descriptors: usize,
    /// Distinct ecosystems represented.
    pub ecosystems_represented: usize,
    /// Descriptors that resolved to a direct dependency.
    pub direct_descriptors: usize,
    /// Descriptors that resolved to a transitive dependency.
    pub transitive_descriptors: usize,
    /// Descriptors that resolved to a workspace-local member.
    pub workspace_local_descriptors: usize,
    /// Descriptors sourced from a path or VCS.
    pub path_or_vcs_descriptors: usize,
    /// Descriptors that carry a resolved identity.
    pub resolved_descriptors: usize,
    /// Descriptors that may claim an exact, current resolution.
    pub exact_claimable_descriptors: usize,
    /// Descriptors that must disclose their resolution environment.
    pub environment_disclosed_descriptors: usize,
    /// Descriptors pinned by policy.
    pub policy_pinned_descriptors: usize,
    /// Descriptors blocked on unsatisfied auth.
    pub auth_gated_descriptors: usize,
    /// Descriptors with at least one open finding.
    pub descriptors_with_open_findings: usize,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageStateDescriptorsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Matrix id every descriptor binds to.
    pub references_matrix_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected export rows.
    pub rows: Vec<PackageStateExportRow>,
    /// Whether every descriptor is consistent with the contract.
    pub all_consistent: bool,
    /// Whether requested and resolved identities stay separate everywhere.
    pub requested_resolved_separate: bool,
    /// Whether no descriptor renders a generic collapse message.
    pub no_generic_collapse: bool,
    /// Whether every descriptor binds to the frozen matrix.
    pub all_bind_matrix: bool,
}

/// The typed package-state descriptors packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageStateDescriptors {
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
    /// The frozen matrix packet id every descriptor binds to.
    pub references_matrix_id: String,
    /// Closed ecosystem vocabulary represented by this packet.
    pub ecosystems: Vec<EcosystemKind>,
    /// The package-state descriptors.
    #[serde(default)]
    pub descriptors: Vec<PackageStateDescriptor>,
    /// Summary counts.
    pub summary: PackageStateDescriptorsSummary,
}

impl PackageStateDescriptors {
    /// Returns the descriptor with the given id.
    pub fn descriptor(&self, descriptor_id: &str) -> Option<&PackageStateDescriptor> {
        self.descriptors
            .iter()
            .find(|d| d.descriptor_id == descriptor_id)
    }

    /// Whether every descriptor is consistent with the frozen contract.
    pub fn all_consistent(&self) -> bool {
        self.descriptors
            .iter()
            .all(PackageStateDescriptor::is_consistent)
    }

    /// Whether requested and resolved identities stay separate in every
    /// descriptor.
    pub fn requested_resolved_separate(&self) -> bool {
        self.descriptors
            .iter()
            .all(PackageStateDescriptor::requested_and_resolved_separate)
    }

    /// Whether no descriptor renders a generic collapse message.
    pub fn no_generic_collapse(&self) -> bool {
        self.descriptors
            .iter()
            .all(|d| d.primary_message_class().is_specific())
    }

    /// Whether every label every descriptor surfaces resolves to a frozen state
    /// row, proving the packet binds to the shared matrix.
    pub fn all_bind_matrix(&self) -> bool {
        let Ok(matrix) = current_m5_package_state_matrix() else {
            return false;
        };
        if self.references_matrix_id != matrix.packet_id {
            return false;
        }
        self.descriptors.iter().all(|d| {
            d.applicable_labels()
                .iter()
                .all(|label| matrix.state(*label).is_some())
        })
    }

    /// Recomputes the summary block from the descriptors.
    pub fn computed_summary(&self) -> PackageStateDescriptorsSummary {
        let ecosystems: BTreeSet<EcosystemKind> =
            self.descriptors.iter().map(|d| d.ecosystem()).collect();
        let count = |pred: &dyn Fn(&PackageStateDescriptor) -> bool| {
            self.descriptors.iter().filter(|d| pred(d)).count()
        };
        PackageStateDescriptorsSummary {
            total_descriptors: self.descriptors.len(),
            ecosystems_represented: ecosystems.len(),
            direct_descriptors: count(&PackageStateDescriptor::is_direct),
            transitive_descriptors: count(&PackageStateDescriptor::is_transitive),
            workspace_local_descriptors: count(&PackageStateDescriptor::is_workspace_local),
            path_or_vcs_descriptors: count(&PackageStateDescriptor::is_path_or_vcs),
            resolved_descriptors: count(&|d| d.resolved.is_some()),
            exact_claimable_descriptors: count(&PackageStateDescriptor::can_claim_resolved_exact),
            environment_disclosed_descriptors: count(
                &PackageStateDescriptor::must_disclose_environment,
            ),
            policy_pinned_descriptors: count(&PackageStateDescriptor::is_policy_pinned),
            auth_gated_descriptors: count(&PackageStateDescriptor::is_auth_gated),
            descriptors_with_open_findings: count(&|d| d.open_finding_count() > 0),
        }
    }

    /// Produces a redaction-safe export projection that downstream surfaces —
    /// support exports, the CLI inspect surface, and release/public-truth — render
    /// instead of restating package state by hand.
    pub fn export_projection(&self) -> PackageStateDescriptorsExportProjection {
        PackageStateDescriptorsExportProjection {
            packet_id: self.packet_id.clone(),
            references_matrix_id: self.references_matrix_id.clone(),
            as_of: self.as_of.clone(),
            rows: self
                .descriptors
                .iter()
                .map(PackageStateDescriptor::export_row)
                .collect(),
            all_consistent: self.all_consistent(),
            requested_resolved_separate: self.requested_resolved_separate(),
            no_generic_collapse: self.no_generic_collapse(),
            all_bind_matrix: self.all_bind_matrix(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<PackageStateDescriptorsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_descriptors(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(PackageStateDescriptorsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<PackageStateDescriptorsViolation>) {
        if self.schema_version != PACKAGE_STATE_DESCRIPTORS_SCHEMA_VERSION {
            violations.push(PackageStateDescriptorsViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != PACKAGE_STATE_DESCRIPTORS_RECORD_KIND {
            violations.push(PackageStateDescriptorsViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("references_matrix_id", &self.references_matrix_id),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageStateDescriptorsViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.ecosystems != EcosystemKind::ALL.to_vec() {
            violations.push(PackageStateDescriptorsViolation::ClosedVocabularyMismatch {
                field: "ecosystems",
            });
        }
        match current_m5_package_state_matrix() {
            Ok(matrix) => {
                if self.references_matrix_id != matrix.packet_id {
                    violations.push(PackageStateDescriptorsViolation::MatrixBindingMismatch {
                        referenced: self.references_matrix_id.clone(),
                        expected: matrix.packet_id,
                    });
                }
            }
            Err(_) => violations.push(PackageStateDescriptorsViolation::MatrixUnavailable),
        }
    }

    fn validate_descriptors(&self, violations: &mut Vec<PackageStateDescriptorsViolation>) {
        let matrix = current_m5_package_state_matrix().ok();
        let mut seen = BTreeSet::new();
        for descriptor in &self.descriptors {
            let id = descriptor.descriptor_id.clone();
            if !seen.insert(id.clone()) {
                violations.push(PackageStateDescriptorsViolation::DuplicateDescriptorId {
                    descriptor_id: id.clone(),
                });
            }
            for (field, value) in [
                ("descriptor_id", &descriptor.descriptor_id),
                ("requested.package_name", &descriptor.requested.package_name),
                (
                    "requested.requested_ref",
                    &descriptor.requested.requested_ref,
                ),
                ("redacted_source_label", &descriptor.redacted_source_label),
                ("note", &descriptor.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(PackageStateDescriptorsViolation::EmptyField {
                        id: id.clone(),
                        field_name: field,
                    });
                }
            }

            // Resolved presence must match the confidence's claim.
            if descriptor.resolved.is_some() != descriptor.confidence.carries_resolved_identity() {
                violations.push(
                    PackageStateDescriptorsViolation::ResolvedConfidenceMismatch {
                        descriptor_id: id.clone(),
                        confidence: descriptor.confidence.as_str(),
                        resolved: descriptor.resolved.is_some(),
                    },
                );
            }

            if let Some(resolved) = &descriptor.resolved {
                if resolved.resolved_ref.trim().is_empty() {
                    violations.push(PackageStateDescriptorsViolation::EmptyField {
                        id: id.clone(),
                        field_name: "resolved.resolved_ref",
                    });
                }
                if leaks_raw_url(&resolved.resolved_ref) {
                    violations.push(PackageStateDescriptorsViolation::RawUrlLeak {
                        id: id.clone(),
                        field_name: "resolved.resolved_ref",
                    });
                }
                // A registry-sourced relation must name its registry source; a
                // workspace member or path/VCS source legitimately has none.
                if resolved.relation.is_registry_sourced() && resolved.registry_source.is_none() {
                    violations.push(PackageStateDescriptorsViolation::MissingRegistrySource {
                        descriptor_id: id.clone(),
                        relation: resolved.relation.as_str(),
                    });
                }
            }

            // Requested and resolved identity must not be flattened together.
            if !descriptor.requested_and_resolved_separate() {
                violations.push(
                    PackageStateDescriptorsViolation::RequestedResolvedConflated {
                        descriptor_id: id.clone(),
                    },
                );
            }

            // Findings must carry the right suppression linkage.
            for finding in &descriptor.findings {
                if !finding.is_consistent() {
                    violations.push(PackageStateDescriptorsViolation::FindingLinkageInvalid {
                        descriptor_id: id.clone(),
                        kind: finding.kind.as_str(),
                    });
                }
                if finding.finding_ref.trim().is_empty() {
                    violations.push(PackageStateDescriptorsViolation::EmptyField {
                        id: id.clone(),
                        field_name: "findings.finding_ref",
                    });
                }
            }

            // Redacted labels and requested refs must never leak a raw URL.
            for (field, value) in [
                ("redacted_source_label", &descriptor.redacted_source_label),
                (
                    "requested.requested_ref",
                    &descriptor.requested.requested_ref,
                ),
            ] {
                if leaks_raw_url(value) {
                    violations.push(PackageStateDescriptorsViolation::RawUrlLeak {
                        id: id.clone(),
                        field_name: field,
                    });
                }
            }

            // The primary message must never collapse into a generic message.
            if descriptor.primary_message_class().is_generic_collapse() {
                violations.push(PackageStateDescriptorsViolation::GenericCollapseMessage {
                    descriptor_id: id.clone(),
                    message: descriptor.primary_message_class().as_str(),
                });
            }

            // Every surfaced label must bind to a frozen state row.
            if let Some(matrix) = &matrix {
                for label in descriptor.applicable_labels() {
                    if matrix.state(label).is_none() {
                        violations.push(PackageStateDescriptorsViolation::UnboundLabel {
                            descriptor_id: id.clone(),
                            label: label.as_str(),
                        });
                    }
                }
            }
        }
    }
}

/// Whether a string leaks a raw URL or scheme that must be redacted.
fn leaks_raw_url(value: &str) -> bool {
    value.contains("://")
}

/// A validation violation for the package-state descriptors packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageStateDescriptorsViolation {
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
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Descriptor or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A descriptor id appears more than once.
    DuplicateDescriptorId {
        /// Duplicate id.
        descriptor_id: String,
    },
    /// The frozen matrix could not be loaded for binding validation.
    MatrixUnavailable,
    /// The packet references a matrix id other than the frozen matrix.
    MatrixBindingMismatch {
        /// Referenced matrix id.
        referenced: String,
        /// Expected (frozen) matrix id.
        expected: String,
    },
    /// A descriptor's resolved presence disagrees with its confidence.
    ResolvedConfidenceMismatch {
        /// Descriptor id.
        descriptor_id: String,
        /// Confidence token.
        confidence: &'static str,
        /// Whether a resolved identity was present.
        resolved: bool,
    },
    /// A descriptor flattens requested and resolved identity into one label.
    RequestedResolvedConflated {
        /// Descriptor id.
        descriptor_id: String,
    },
    /// A registry-sourced resolution does not name its registry source.
    MissingRegistrySource {
        /// Descriptor id.
        descriptor_id: String,
        /// Dependency relation token.
        relation: &'static str,
    },
    /// A finding's suppression linkage is invalid for its kind.
    FindingLinkageInvalid {
        /// Descriptor id.
        descriptor_id: String,
        /// Finding-kind token.
        kind: &'static str,
    },
    /// A field leaks a raw URL that must be redacted.
    RawUrlLeak {
        /// Descriptor id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A descriptor renders a forbidden generic collapse message.
    GenericCollapseMessage {
        /// Descriptor id.
        descriptor_id: String,
        /// Generic message-class token.
        message: &'static str,
    },
    /// A surfaced label does not bind to a frozen state row.
    UnboundLabel {
        /// Descriptor id.
        descriptor_id: String,
        /// Label token.
        label: &'static str,
    },
    /// The summary counts disagree with the descriptors.
    SummaryMismatch,
}

impl fmt::Display for PackageStateDescriptorsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical vocabulary")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateDescriptorId { descriptor_id } => {
                write!(f, "duplicate descriptor id {descriptor_id}")
            }
            Self::MatrixUnavailable => {
                write!(f, "the frozen package-state matrix could not be loaded")
            }
            Self::MatrixBindingMismatch {
                referenced,
                expected,
            } => write!(
                f,
                "packet references matrix id {referenced} instead of the frozen {expected}"
            ),
            Self::ResolvedConfidenceMismatch {
                descriptor_id,
                confidence,
                resolved,
            } => write!(
                f,
                "descriptor {descriptor_id} confidence {confidence} disagrees with resolved presence {resolved}"
            ),
            Self::RequestedResolvedConflated { descriptor_id } => write!(
                f,
                "descriptor {descriptor_id} flattens requested and resolved identity"
            ),
            Self::MissingRegistrySource {
                descriptor_id,
                relation,
            } => write!(
                f,
                "descriptor {descriptor_id} relation {relation} is registry-sourced but names no registry source"
            ),
            Self::FindingLinkageInvalid {
                descriptor_id,
                kind,
            } => write!(
                f,
                "descriptor {descriptor_id} finding {kind} has invalid suppression linkage"
            ),
            Self::RawUrlLeak { id, field_name } => {
                write!(f, "{id} field {field_name} leaks a raw URL")
            }
            Self::GenericCollapseMessage {
                descriptor_id,
                message,
            } => write!(
                f,
                "descriptor {descriptor_id} renders forbidden generic message {message}"
            ),
            Self::UnboundLabel {
                descriptor_id,
                label,
            } => write!(
                f,
                "descriptor {descriptor_id} surfaces label {label} with no frozen state row"
            ),
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the descriptors")
            }
        }
    }
}

impl Error for PackageStateDescriptorsViolation {}

/// Loads the embedded package-state descriptors packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`PackageStateDescriptors`].
pub fn current_package_state_descriptors() -> Result<PackageStateDescriptors, serde_json::Error> {
    serde_json::from_str(PACKAGE_STATE_DESCRIPTORS_JSON)
}

#[cfg(test)]
mod tests;
