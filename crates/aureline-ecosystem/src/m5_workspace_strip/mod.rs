//! Canonical M5 local extension workspace strips — the per-workspace authoring
//! chrome an author reads while building an M5 ecosystem pack without leaving the IDE.
//!
//! Where [`crate::m5_author_and_publish_preview`] is the *publish-control gate* an
//! author drives before a package reaches the public registry, this module freezes the
//! *local workspace strip*: the always-on chrome that names, for each workspace, the
//! package identity, source path, runtime class, target host/ABI, signing state,
//! workspace origin, build freshness (last-built), load state (last-loaded and
//! hot-reload/relaunch posture), and the unsigned/local-only trust badge it renders.
//! Each [`LocalWorkspaceStrip`] reuses the shared [`ArtifactFamily`], [`RuntimeClass`],
//! [`HostAbiClass`], [`SignatureState`], [`TrustPosture`], and [`HotReloadPosture`]
//! vocabulary so the strip and the publish gate describe the same artifact without a
//! parallel synonym set.
//!
//! The strip is a render-truth object, not a publish decision. From the observed states
//! it recomputes:
//!
//! - the **rendered trust posture** the strip may display — capped by *both* the signing
//!   state *and* the workspace origin, so a local-dev or side-loaded workspace renders
//!   [`TrustPosture::UnsignedLocalOnly`] even when the artifact is signed on a trusted
//!   machine, and a revoked or unsigned artifact never inherits a verified-publisher or
//!   enterprise-approved badge just because it was built locally;
//! - whether the workspace **requires a fresh review** before its running instance picks
//!   up a hot reload — a hot reload that would widen the runtime class, add an external
//!   executable, or expand permissions holds the running instance in
//!   [`LoadState::ReloadHeldForReview`] rather than widening authority silently; and
//! - whether the strip is **local-only** versus published or mirror-backed, so authoring
//!   surfaces can distinguish a local artifact from a published or mirror-backed one at a
//!   glance.
//!
//! Runtime class and host/ABI are required fields on every strip, so they are never
//! hidden when they change compatibility or publish readiness. Each strip's
//! [`LocalWorkspaceStrip::rendered_trust_posture`] and
//! [`LocalWorkspaceStrip::load_state`] are validated against the same gate, and
//! [`M5LocalWorkspaceStripBoard::cross_check_matrix`] proves the strip never renders a
//! stronger badge than the publish-preview gate would grant the same family, so the
//! authoring chrome, the publish preview, install/update flows, diagnostics, and support
//! exports all project the same trust truth instead of retyping it.
//!
//! The packet is checked in at
//! `artifacts/ecosystem/m5/m5-local-workspace-strip.json` and embedded here, so this
//! typed consumer and any CI gate agree on every strip without a cargo build in CI. The
//! model is metadata-only: every field is a typed state or an opaque ref. It carries no
//! credential bodies, raw provider payloads, signing secrets, or source bodies — the
//! `source_path_ref` is an opaque workspace ref, never a verbatim filesystem path.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::m5_author_and_publish_preview::{
    ArtifactFamily, HostAbiClass, HotReloadPosture, M5AuthorPublishMatrix, RuntimeClass,
    SignatureState, TrustPosture,
};

/// Supported M5 local-workspace-strip board schema version.
pub const M5_WORKSPACE_STRIP_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_WORKSPACE_STRIP_RECORD_KIND: &str = "m5_local_workspace_strip_board";

/// Repo-relative path to the checked-in packet.
pub const M5_WORKSPACE_STRIP_PATH: &str = "artifacts/ecosystem/m5/m5-local-workspace-strip.json";

/// Embedded checked-in packet JSON.
pub const M5_WORKSPACE_STRIP_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-local-workspace-strip.json"
));

/// Origin a local workspace is anchored to.
///
/// The origin distinguishes a local-only authoring workspace from a published or
/// mirror-backed one and contributes a trust ceiling: a local-dev or side-loaded origin
/// caps the rendered badge at [`TrustPosture::UnsignedLocalOnly`], so a local build can
/// never inherit a trusted-publisher badge regardless of the machine's signing keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceOrigin {
    /// A local development workspace that has not been published.
    LocalDevWorkspace,
    /// A workspace side-loaded from an external artifact.
    SideloadedWorkspace,
    /// A workspace anchored to a published registry release.
    PublishedRegistryBacked,
    /// A workspace anchored to a mirrored/private-registry variant.
    MirrorBacked,
}

impl WorkspaceOrigin {
    /// Every workspace origin, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalDevWorkspace,
        Self::SideloadedWorkspace,
        Self::PublishedRegistryBacked,
        Self::MirrorBacked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDevWorkspace => "local_dev_workspace",
            Self::SideloadedWorkspace => "sideloaded_workspace",
            Self::PublishedRegistryBacked => "published_registry_backed",
            Self::MirrorBacked => "mirror_backed",
        }
    }

    /// Highest trust posture a workspace with this origin may render.
    ///
    /// A local-dev or side-loaded origin caps at [`TrustPosture::UnsignedLocalOnly`];
    /// a published or mirror-backed origin leaves the signing state to govern the cap.
    pub const fn trust_ceiling(self) -> TrustPosture {
        match self {
            Self::LocalDevWorkspace | Self::SideloadedWorkspace => TrustPosture::UnsignedLocalOnly,
            Self::PublishedRegistryBacked | Self::MirrorBacked => TrustPosture::EnterpriseApproved,
        }
    }

    /// Whether this origin structurally caps the workspace to local-only.
    pub const fn caps_to_local_only(self) -> bool {
        matches!(self, Self::LocalDevWorkspace | Self::SideloadedWorkspace)
    }

    /// Whether this origin is a local authoring workspace (not published or mirrored).
    pub const fn is_local_authored(self) -> bool {
        matches!(self, Self::LocalDevWorkspace | Self::SideloadedWorkspace)
    }
}

/// Build/source freshness of the workspace's last build — the strip's last-built fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildFreshness {
    /// A current build exists from the present source.
    BuiltFromCurrentSource,
    /// A build exists but the source has changed since it was produced.
    BuiltStaleVsSource,
    /// The workspace has never been built locally.
    NeverBuilt,
    /// The most recent local build failed.
    BuildFailed,
}

impl BuildFreshness {
    /// Every build-freshness state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::BuiltFromCurrentSource,
        Self::BuiltStaleVsSource,
        Self::NeverBuilt,
        Self::BuildFailed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuiltFromCurrentSource => "built_from_current_source",
            Self::BuiltStaleVsSource => "built_stale_vs_source",
            Self::NeverBuilt => "never_built",
            Self::BuildFailed => "build_failed",
        }
    }

    /// Whether a build artifact exists that a host could load.
    ///
    /// A workspace that was never built or whose build failed has nothing to load.
    pub const fn is_loadable(self) -> bool {
        matches!(
            self,
            Self::BuiltFromCurrentSource | Self::BuiltStaleVsSource
        )
    }
}

/// Runtime load state of the workspace's host instance — the strip's last-loaded fact.
///
/// The load state carries the hot-reload/relaunch truth: a workspace whose hot reload
/// would widen authority is held in [`LoadState::ReloadHeldForReview`] until a fresh
/// review clears it, so authority can never widen through a hot reload silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadState {
    /// The running instance matches the current build.
    LoadedCurrentBuild,
    /// A relaunch is pending to pick up the latest build.
    ReloadPendingRelaunch,
    /// A hot reload would widen authority; the running instance is held for review.
    ReloadHeldForReview,
    /// The workspace has never been loaded into a host.
    NotLoaded,
    /// The most recent load attempt failed.
    LoadFailed,
}

impl LoadState {
    /// Every load state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LoadedCurrentBuild,
        Self::ReloadPendingRelaunch,
        Self::ReloadHeldForReview,
        Self::NotLoaded,
        Self::LoadFailed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LoadedCurrentBuild => "loaded_current_build",
            Self::ReloadPendingRelaunch => "reload_pending_relaunch",
            Self::ReloadHeldForReview => "reload_held_for_review",
            Self::NotLoaded => "not_loaded",
            Self::LoadFailed => "load_failed",
        }
    }

    /// Whether this load state requires a loadable build to be coherent.
    ///
    /// A loaded or reloading instance must have a build to run; only
    /// [`LoadState::NotLoaded`] and [`LoadState::LoadFailed`] are coherent without one.
    pub const fn needs_loadable_build(self) -> bool {
        matches!(
            self,
            Self::LoadedCurrentBuild | Self::ReloadPendingRelaunch | Self::ReloadHeldForReview
        )
    }
}

/// Whether a hot-reload posture would widen authority and so force a fresh review.
///
/// Reuses the [`HotReloadPosture`] vocabulary frozen by the publish-preview gate: the
/// three `*_widened_pending_review` postures widen authority; `no_widening` and
/// `relaunch_only` do not.
pub const fn hot_reload_widens_authority(posture: HotReloadPosture) -> bool {
    matches!(
        posture,
        HotReloadPosture::RuntimeClassWidenedPendingReview
            | HotReloadPosture::PermissionsWidenedPendingReview
            | HotReloadPosture::ExternalExecutableAddedPendingReview
    )
}

/// One local extension workspace strip for a marketed M5 artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalWorkspaceStrip {
    /// Stable strip id.
    pub strip_id: String,
    /// Marketed M5 artifact family this strip governs.
    pub artifact_family: ArtifactFamily,
    /// Author-facing package identity (display name/id; no secrets).
    pub package_identity: String,
    /// Opaque ref to the workspace source path (never a verbatim filesystem path).
    pub source_path_ref: String,
    /// Origin the workspace is anchored to.
    pub origin: WorkspaceOrigin,
    /// Runtime class of the authored artifact.
    pub runtime_class: RuntimeClass,
    /// Host/ABI execution locus.
    pub host_abi: HostAbiClass,
    /// Signing/provenance state.
    pub signature_state: SignatureState,
    /// Trust posture the author requests, before the strip caps it.
    pub declared_trust_posture: TrustPosture,
    /// Trust posture the strip actually renders after capping.
    ///
    /// Must equal [`LocalWorkspaceStrip::effective_trust_posture`].
    pub rendered_trust_posture: TrustPosture,
    /// Build freshness / last-built fact.
    pub build_freshness: BuildFreshness,
    /// Load state / last-loaded and hot-reload fact.
    pub load_state: LoadState,
    /// Hot-reload/relaunch posture.
    pub hot_reload_posture: HotReloadPosture,
    /// Ref to the workspace's last-built record.
    pub last_built_ref: String,
    /// Ref to the workspace's last-loaded record.
    pub last_loaded_ref: String,
    /// Ref to the family's publish-preview gate row.
    pub publish_preview_ref: String,
    /// Ref binding this strip into diagnostics, support, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the strip.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl LocalWorkspaceStrip {
    /// The trust posture the strip may render for this workspace.
    ///
    /// Lowers the author's declared posture to the *minimum* of the signing-state
    /// ceiling and the origin ceiling, so a local-dev or side-loaded workspace renders
    /// local-only even when signed, and an unsigned or revoked artifact never inherits a
    /// trusted-publisher badge.
    pub fn effective_trust_posture(&self) -> TrustPosture {
        self.declared_trust_posture
            .min(self.signature_state.trust_ceiling())
            .min(self.origin.trust_ceiling())
    }

    /// Whether the strip renders as a local-only artifact (no inherited trust badge).
    pub fn is_local_only(&self) -> bool {
        self.effective_trust_posture() == TrustPosture::UnsignedLocalOnly
    }

    /// Whether the workspace requires a fresh review before a hot reload takes effect.
    pub fn requires_fresh_review(&self) -> bool {
        hot_reload_widens_authority(self.hot_reload_posture)
    }

    /// Whether the workspace is anchored to a published or mirror-backed origin.
    pub fn is_published_or_mirror_backed(&self) -> bool {
        !self.origin.is_local_authored()
    }

    /// Whether the strip carries its own non-empty author-lane refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.last_built_ref.trim().is_empty()
            && !self.last_loaded_ref.trim().is_empty()
            && !self.publish_preview_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether the stored rendered posture, the load state, and the build/load coherence
    /// all agree with the recomputed strip decision.
    pub fn strip_consistent(&self) -> bool {
        self.rendered_trust_posture == self.effective_trust_posture()
            && self.load_state_consistent()
            && self.build_load_coherent()
    }

    /// Whether the load state agrees with the hot-reload posture.
    ///
    /// A widening hot reload must hold the running instance for review, and an instance
    /// held for review must be backed by a widening hot reload.
    fn load_state_consistent(&self) -> bool {
        let widens = self.requires_fresh_review();
        let held = self.load_state == LoadState::ReloadHeldForReview;
        widens == held
    }

    /// Whether the build freshness and load state are mutually coherent.
    fn build_load_coherent(&self) -> bool {
        if self.load_state.needs_loadable_build() && !self.build_freshness.is_loadable() {
            return false;
        }
        if self.load_state == LoadState::LoadedCurrentBuild
            && self.build_freshness != BuildFreshness::BuiltFromCurrentSource
        {
            return false;
        }
        true
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5WorkspaceStripSummary {
    /// Total strips.
    pub total_strips: usize,
    /// Number of marketed families claimed.
    pub family_count: usize,
    /// Strips rendered as local-only (no inherited trust badge).
    pub local_only_strips: usize,
    /// Strips anchored to a published registry release.
    pub published_registry_backed_strips: usize,
    /// Strips anchored to a mirrored/private-registry variant.
    pub mirror_backed_strips: usize,
    /// Strips in a local-dev workspace.
    pub local_dev_strips: usize,
    /// Strips in a side-loaded workspace.
    pub sideloaded_strips: usize,
    /// Strips whose running instance is held for a fresh review.
    pub reload_held_for_review_strips: usize,
    /// Strips whose build is stale against changed source.
    pub stale_build_strips: usize,
    /// Strips whose most recent build failed.
    pub build_failed_strips: usize,
    /// Strips that have never been built locally.
    pub never_built_strips: usize,
    /// Strips rendering a verified-publisher or enterprise-approved badge.
    pub verified_or_enterprise_rendered_strips: usize,
}

/// A redaction-safe export row projected from a workspace strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceStripExportRow {
    /// Strip id.
    pub strip_id: String,
    /// Artifact-family token.
    pub artifact_family: String,
    /// Author-facing package identity.
    pub package_identity: String,
    /// Opaque source-path ref.
    pub source_path_ref: String,
    /// Origin token.
    pub origin: String,
    /// Runtime-class token.
    pub runtime_class: String,
    /// Host/ABI token.
    pub host_abi: String,
    /// Signing-state token.
    pub signature_state: String,
    /// Declared trust-posture token.
    pub declared_trust_posture: String,
    /// Rendered trust-posture token.
    pub rendered_trust_posture: String,
    /// Build-freshness token.
    pub build_freshness: String,
    /// Load-state token.
    pub load_state: String,
    /// Hot-reload-posture token.
    pub hot_reload_posture: String,
    /// Whether the strip renders as local-only.
    pub local_only: bool,
    /// Whether the workspace requires a fresh review before a hot reload takes effect.
    pub requires_fresh_review: bool,
    /// Whether the workspace is published or mirror-backed.
    pub published_or_mirror_backed: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceStripExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected strips.
    pub strips: Vec<M5WorkspaceStripExportRow>,
    /// Whether every strip's stored decision agrees with the recomputed strip.
    pub all_strips_consistent: bool,
    /// Strips rendered as local-only.
    pub local_only_count: usize,
    /// Strips held for a fresh review.
    pub reload_held_count: usize,
}

/// The typed M5 local-workspace-strip board packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5LocalWorkspaceStripBoard {
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
    /// Marketed families the packet claims; one strip per family.
    pub artifact_families: Vec<ArtifactFamily>,
    /// Closed runtime-class vocabulary.
    pub runtime_classes: Vec<RuntimeClass>,
    /// Closed host/ABI vocabulary.
    pub host_abi_classes: Vec<HostAbiClass>,
    /// Closed workspace-origin vocabulary.
    pub workspace_origins: Vec<WorkspaceOrigin>,
    /// Closed signing-state vocabulary.
    pub signature_states: Vec<SignatureState>,
    /// Closed trust-posture vocabulary.
    pub trust_postures: Vec<TrustPosture>,
    /// Closed build-freshness vocabulary.
    pub build_freshness_states: Vec<BuildFreshness>,
    /// Closed load-state vocabulary.
    pub load_states: Vec<LoadState>,
    /// Closed hot-reload-posture vocabulary.
    pub hot_reload_postures: Vec<HotReloadPosture>,
    /// Strips, one per marketed family.
    #[serde(default)]
    pub strips: Vec<LocalWorkspaceStrip>,
    /// Summary counts.
    pub summary: M5WorkspaceStripSummary,
}

impl M5LocalWorkspaceStripBoard {
    /// Returns the strip for a marketed family.
    pub fn strip(&self, family: ArtifactFamily) -> Option<&LocalWorkspaceStrip> {
        self.strips.iter().find(|s| s.artifact_family == family)
    }

    /// Strips rendered as local-only.
    pub fn local_only_strips(&self) -> impl Iterator<Item = &LocalWorkspaceStrip> {
        self.strips.iter().filter(|s| s.is_local_only())
    }

    /// Strips whose running instance is held for a fresh review.
    pub fn reload_held_strips(&self) -> impl Iterator<Item = &LocalWorkspaceStrip> {
        self.strips
            .iter()
            .filter(|s| s.load_state == LoadState::ReloadHeldForReview)
    }

    /// Strips anchored to a published or mirror-backed origin.
    pub fn published_or_mirror_strips(&self) -> impl Iterator<Item = &LocalWorkspaceStrip> {
        self.strips
            .iter()
            .filter(|s| s.is_published_or_mirror_backed())
    }

    /// Whether every strip's stored decision agrees with the recomputed strip.
    pub fn all_strips_consistent(&self) -> bool {
        self.strips.iter().all(|s| s.strip_consistent())
    }

    /// Recomputes the summary block from the strips.
    pub fn computed_summary(&self) -> M5WorkspaceStripSummary {
        let count_origin =
            |origin: WorkspaceOrigin| self.strips.iter().filter(|s| s.origin == origin).count();
        M5WorkspaceStripSummary {
            total_strips: self.strips.len(),
            family_count: self.artifact_families.len(),
            local_only_strips: self.local_only_strips().count(),
            published_registry_backed_strips: count_origin(
                WorkspaceOrigin::PublishedRegistryBacked,
            ),
            mirror_backed_strips: count_origin(WorkspaceOrigin::MirrorBacked),
            local_dev_strips: count_origin(WorkspaceOrigin::LocalDevWorkspace),
            sideloaded_strips: count_origin(WorkspaceOrigin::SideloadedWorkspace),
            reload_held_for_review_strips: self.reload_held_strips().count(),
            stale_build_strips: self
                .strips
                .iter()
                .filter(|s| s.build_freshness == BuildFreshness::BuiltStaleVsSource)
                .count(),
            build_failed_strips: self
                .strips
                .iter()
                .filter(|s| s.build_freshness == BuildFreshness::BuildFailed)
                .count(),
            never_built_strips: self
                .strips
                .iter()
                .filter(|s| s.build_freshness == BuildFreshness::NeverBuilt)
                .count(),
            verified_or_enterprise_rendered_strips: self
                .strips
                .iter()
                .filter(|s| s.rendered_trust_posture.is_trusted_badge())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — local authoring
    /// surfaces, the publish preview, install/update flows, diagnostics, and support
    /// exports — render instead of restating workspace-strip status text by hand.
    pub fn export_projection(&self) -> M5WorkspaceStripExportProjection {
        let strips = self
            .strips
            .iter()
            .map(|s| M5WorkspaceStripExportRow {
                strip_id: s.strip_id.clone(),
                artifact_family: s.artifact_family.as_str().to_owned(),
                package_identity: s.package_identity.clone(),
                source_path_ref: s.source_path_ref.clone(),
                origin: s.origin.as_str().to_owned(),
                runtime_class: s.runtime_class.as_str().to_owned(),
                host_abi: s.host_abi.as_str().to_owned(),
                signature_state: s.signature_state.as_str().to_owned(),
                declared_trust_posture: s.declared_trust_posture.as_str().to_owned(),
                rendered_trust_posture: s.rendered_trust_posture.as_str().to_owned(),
                build_freshness: s.build_freshness.as_str().to_owned(),
                load_state: s.load_state.as_str().to_owned(),
                hot_reload_posture: s.hot_reload_posture.as_str().to_owned(),
                local_only: s.is_local_only(),
                requires_fresh_review: s.requires_fresh_review(),
                published_or_mirror_backed: s.is_published_or_mirror_backed(),
                summary: format!(
                    "{}: origin {}, runtime {}, host {}, signing {}, declared {}, rendered {}{}, build {}, load {}, hot-reload {}",
                    s.artifact_family.as_str(),
                    s.origin.as_str(),
                    s.runtime_class.as_str(),
                    s.host_abi.as_str(),
                    s.signature_state.as_str(),
                    s.declared_trust_posture.as_str(),
                    s.rendered_trust_posture.as_str(),
                    if s.is_local_only() { " (local-only)" } else { "" },
                    s.build_freshness.as_str(),
                    s.load_state.as_str(),
                    s.hot_reload_posture.as_str(),
                ),
            })
            .collect();
        M5WorkspaceStripExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            strips,
            all_strips_consistent: self.all_strips_consistent(),
            local_only_count: self.local_only_strips().count(),
            reload_held_count: self.reload_held_strips().count(),
        }
    }

    /// Cross-checks the strips against the publish-preview gate.
    ///
    /// Proves the strip never renders a *stronger* trust badge than the publish-preview
    /// gate would grant the same family. The local render may be more conservative — a
    /// signed package in a local-dev workspace renders local-only though it would publish
    /// verified — but it can never exceed the gate, so the authoring chrome and the
    /// publish preview project one trust truth.
    pub fn cross_check_matrix(
        &self,
        matrix: &M5AuthorPublishMatrix,
    ) -> Vec<M5WorkspaceStripViolation> {
        let mut violations = Vec::new();
        for strip in &self.strips {
            match matrix.family(strip.artifact_family) {
                None => violations.push(M5WorkspaceStripViolation::MissingMatrixRow {
                    strip_id: strip.strip_id.clone(),
                    family: strip.artifact_family.as_str(),
                }),
                Some(row) => {
                    if strip.rendered_trust_posture.rank() > row.published_trust_posture.rank() {
                        violations.push(M5WorkspaceStripViolation::StripExceedsPublishGate {
                            strip_id: strip.strip_id.clone(),
                            rendered: strip.rendered_trust_posture.as_str(),
                            published: row.published_trust_posture.as_str(),
                        });
                    }
                }
            }
        }
        violations
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5WorkspaceStripViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<ArtifactFamily> = self.artifact_families.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for strip in &self.strips {
            if !seen_ids.insert(strip.strip_id.clone()) {
                violations.push(M5WorkspaceStripViolation::DuplicateStripId {
                    strip_id: strip.strip_id.clone(),
                });
            }
            if !seen_families.insert(strip.artifact_family) {
                violations.push(M5WorkspaceStripViolation::DuplicateFamilyStrip {
                    family: strip.artifact_family.as_str(),
                });
            }
            if !claimed.contains(&strip.artifact_family) {
                violations.push(M5WorkspaceStripViolation::UnclaimedFamilyStrip {
                    strip_id: strip.strip_id.clone(),
                    family: strip.artifact_family.as_str(),
                });
            }
            self.validate_strip(strip, &mut violations);
        }

        // Every claimed family must carry its own strip, so a family never inherits an
        // authoring-chrome posture from an adjacent one.
        for &family in &self.artifact_families {
            if !seen_families.contains(&family) {
                violations.push(M5WorkspaceStripViolation::MissingFamilyStrip {
                    family: family.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5WorkspaceStripViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5WorkspaceStripViolation>) {
        if self.schema_version != M5_WORKSPACE_STRIP_SCHEMA_VERSION {
            violations.push(M5WorkspaceStripViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_WORKSPACE_STRIP_RECORD_KIND {
            violations.push(M5WorkspaceStripViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5WorkspaceStripViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "artifact_families",
                self.artifact_families == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "runtime_classes",
                self.runtime_classes == RuntimeClass::ALL.to_vec(),
            ),
            (
                "host_abi_classes",
                self.host_abi_classes == HostAbiClass::ALL.to_vec(),
            ),
            (
                "workspace_origins",
                self.workspace_origins == WorkspaceOrigin::ALL.to_vec(),
            ),
            (
                "signature_states",
                self.signature_states == SignatureState::ALL.to_vec(),
            ),
            (
                "trust_postures",
                self.trust_postures == TrustPosture::ALL.to_vec(),
            ),
            (
                "build_freshness_states",
                self.build_freshness_states == BuildFreshness::ALL.to_vec(),
            ),
            ("load_states", self.load_states == LoadState::ALL.to_vec()),
            (
                "hot_reload_postures",
                self.hot_reload_postures == HotReloadPosture::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5WorkspaceStripViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_strip(
        &self,
        strip: &LocalWorkspaceStrip,
        violations: &mut Vec<M5WorkspaceStripViolation>,
    ) {
        for (field, value) in [
            ("strip_id", &strip.strip_id),
            ("package_identity", &strip.package_identity),
            ("source_path_ref", &strip.source_path_ref),
            ("last_built_ref", &strip.last_built_ref),
            ("last_loaded_ref", &strip.last_loaded_ref),
            ("publish_preview_ref", &strip.publish_preview_ref),
            ("support_export_ref", &strip.support_export_ref),
            ("note", &strip.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5WorkspaceStripViolation::EmptyField {
                    id: strip.strip_id.clone(),
                    field_name: field,
                });
            }
        }

        // The rendered trust posture must equal the strip's recomputed posture, so a
        // strip can never render a stronger badge than its signing state and origin
        // support.
        let effective = strip.effective_trust_posture();
        if strip.rendered_trust_posture != effective {
            violations.push(M5WorkspaceStripViolation::RenderedTrustOverstated {
                strip_id: strip.strip_id.clone(),
                rendered: strip.rendered_trust_posture.as_str(),
                computed: effective.as_str(),
            });
        }

        // Non-inheritance: a local-dev or side-loaded workspace, or an unsigned/revoked
        // artifact, must render local-only and never inherit a trusted badge — even when
        // signed on a trusted machine.
        if (strip.signature_state.is_local_or_untrusted() || strip.origin.caps_to_local_only())
            && strip.rendered_trust_posture != TrustPosture::UnsignedLocalOnly
        {
            violations.push(M5WorkspaceStripViolation::LocalWorkspaceInheritedTrust {
                strip_id: strip.strip_id.clone(),
                origin: strip.origin.as_str(),
                signature_state: strip.signature_state.as_str(),
                rendered: strip.rendered_trust_posture.as_str(),
            });
        }

        // A hot reload that would widen authority must hold the running instance for a
        // fresh review, so authority can never widen through a hot reload silently.
        if strip.requires_fresh_review() && strip.load_state != LoadState::ReloadHeldForReview {
            violations.push(M5WorkspaceStripViolation::HotReloadWideningNotHeld {
                strip_id: strip.strip_id.clone(),
                hot_reload_posture: strip.hot_reload_posture.as_str(),
                load_state: strip.load_state.as_str(),
            });
        }

        // Conversely, an instance held for review must be backed by a widening hot reload,
        // so a strip can never assert a held instance without a widening cause.
        if strip.load_state == LoadState::ReloadHeldForReview && !strip.requires_fresh_review() {
            violations.push(M5WorkspaceStripViolation::ReloadHeldWithoutWidening {
                strip_id: strip.strip_id.clone(),
                hot_reload_posture: strip.hot_reload_posture.as_str(),
            });
        }

        // A loaded or reloading instance must be backed by a loadable build.
        if strip.load_state.needs_loadable_build() && !strip.build_freshness.is_loadable() {
            violations.push(M5WorkspaceStripViolation::LoadedWithoutBuild {
                strip_id: strip.strip_id.clone(),
                build_freshness: strip.build_freshness.as_str(),
                load_state: strip.load_state.as_str(),
            });
        }

        // A current-build load must be backed by a build from the present source.
        if strip.load_state == LoadState::LoadedCurrentBuild
            && strip.build_freshness != BuildFreshness::BuiltFromCurrentSource
        {
            violations.push(M5WorkspaceStripViolation::LoadedStaleBuild {
                strip_id: strip.strip_id.clone(),
                build_freshness: strip.build_freshness.as_str(),
            });
        }
    }
}

/// A validation violation for the M5 local-workspace-strip board packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5WorkspaceStripViolation {
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
        /// Strip or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A strip id appears more than once.
    DuplicateStripId {
        /// Duplicate strip id.
        strip_id: String,
    },
    /// A marketed family carries more than one strip.
    DuplicateFamilyStrip {
        /// Family token.
        family: &'static str,
    },
    /// A claimed marketed family has no strip.
    MissingFamilyStrip {
        /// Family token.
        family: &'static str,
    },
    /// A strip covers a family the packet does not claim.
    UnclaimedFamilyStrip {
        /// Strip id.
        strip_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A strip renders a trust posture beyond what its signing state and origin support.
    RenderedTrustOverstated {
        /// Strip id.
        strip_id: String,
        /// Rendered trust-posture token.
        rendered: &'static str,
        /// Computed effective trust-posture token.
        computed: &'static str,
    },
    /// A local/side-loaded workspace or unsigned/revoked artifact rendered a trusted badge.
    LocalWorkspaceInheritedTrust {
        /// Strip id.
        strip_id: String,
        /// Origin token.
        origin: &'static str,
        /// Signing-state token.
        signature_state: &'static str,
        /// Rendered trust-posture token.
        rendered: &'static str,
    },
    /// A widening hot reload did not hold the running instance for review.
    HotReloadWideningNotHeld {
        /// Strip id.
        strip_id: String,
        /// Hot-reload-posture token.
        hot_reload_posture: &'static str,
        /// Load-state token.
        load_state: &'static str,
    },
    /// An instance held for review is not backed by a widening hot reload.
    ReloadHeldWithoutWidening {
        /// Strip id.
        strip_id: String,
        /// Hot-reload-posture token.
        hot_reload_posture: &'static str,
    },
    /// A loaded or reloading instance is not backed by a loadable build.
    LoadedWithoutBuild {
        /// Strip id.
        strip_id: String,
        /// Build-freshness token.
        build_freshness: &'static str,
        /// Load-state token.
        load_state: &'static str,
    },
    /// A current-build load is not backed by a build from the present source.
    LoadedStaleBuild {
        /// Strip id.
        strip_id: String,
        /// Build-freshness token.
        build_freshness: &'static str,
    },
    /// A strip covers a family the publish-preview gate does not.
    MissingMatrixRow {
        /// Strip id.
        strip_id: String,
        /// Family token.
        family: &'static str,
    },
    /// A strip renders a stronger badge than the publish-preview gate would grant.
    StripExceedsPublishGate {
        /// Strip id.
        strip_id: String,
        /// Rendered trust-posture token.
        rendered: &'static str,
        /// Published trust-posture token from the gate.
        published: &'static str,
    },
    /// The summary counts disagree with the strips.
    SummaryMismatch,
}

impl fmt::Display for M5WorkspaceStripViolation {
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
            Self::DuplicateStripId { strip_id } => {
                write!(f, "duplicate strip id {strip_id}")
            }
            Self::DuplicateFamilyStrip { family } => {
                write!(f, "duplicate strip for family {family}")
            }
            Self::MissingFamilyStrip { family } => {
                write!(f, "missing strip for claimed family {family}")
            }
            Self::UnclaimedFamilyStrip { strip_id, family } => {
                write!(f, "strip {strip_id} covers unclaimed family {family}")
            }
            Self::RenderedTrustOverstated {
                strip_id,
                rendered,
                computed,
            } => {
                write!(
                    f,
                    "strip {strip_id} renders trust posture {rendered} but the strip computes {computed}"
                )
            }
            Self::LocalWorkspaceInheritedTrust {
                strip_id,
                origin,
                signature_state,
                rendered,
            } => {
                write!(
                    f,
                    "strip {strip_id} is {origin}/{signature_state} but renders {rendered}; local workspaces must render unsigned_local_only"
                )
            }
            Self::HotReloadWideningNotHeld {
                strip_id,
                hot_reload_posture,
                load_state,
            } => {
                write!(
                    f,
                    "strip {strip_id} hot reload {hot_reload_posture} would widen authority but the instance is {load_state} rather than held for review"
                )
            }
            Self::ReloadHeldWithoutWidening {
                strip_id,
                hot_reload_posture,
            } => {
                write!(
                    f,
                    "strip {strip_id} holds the instance for review but its hot reload {hot_reload_posture} does not widen authority"
                )
            }
            Self::LoadedWithoutBuild {
                strip_id,
                build_freshness,
                load_state,
            } => {
                write!(
                    f,
                    "strip {strip_id} is {load_state} but its build is {build_freshness}; a loaded instance needs a loadable build"
                )
            }
            Self::LoadedStaleBuild {
                strip_id,
                build_freshness,
            } => {
                write!(
                    f,
                    "strip {strip_id} reports loaded_current_build but its build is {build_freshness}"
                )
            }
            Self::MissingMatrixRow { strip_id, family } => {
                write!(
                    f,
                    "strip {strip_id} covers family {family} but the publish-preview gate has no row for it"
                )
            }
            Self::StripExceedsPublishGate {
                strip_id,
                rendered,
                published,
            } => {
                write!(
                    f,
                    "strip {strip_id} renders {rendered} but the publish-preview gate grants only {published}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the strips")
            }
        }
    }
}

impl Error for M5WorkspaceStripViolation {}

/// Loads the embedded M5 local-workspace-strip board packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5LocalWorkspaceStripBoard`].
pub fn current_m5_workspace_strip_board() -> Result<M5LocalWorkspaceStripBoard, serde_json::Error> {
    serde_json::from_str(M5_WORKSPACE_STRIP_JSON)
}

#[cfg(test)]
mod tests;
