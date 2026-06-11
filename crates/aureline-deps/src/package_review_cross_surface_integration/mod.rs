//! Package-review and dependency-health integration across framework-pack
//! health bundles, review workspaces, incident bundles, and companion-safe
//! inspect flows.
//!
//! This module owns the canonical packet that lets a dependency/package card
//! cross from the desktop dependency workspace into higher-order M5 surfaces —
//! framework-pack health bundles, review workspaces, incident packets, and
//! companion-safe inspect views — without losing package identity, manifest
//! scope, support class, advisory freshness, or its local-versus-managed
//! source label, and without smuggling write authority the host surface does
//! not actually have.
//!
//! Every [`DependencyCard`] names the [`SurfaceClass`] it is embedded in and the
//! [`WriteAuthority`] it carries there. The model enforces that only the desktop
//! workspace may carry full mutation authority, review workspaces may stage but
//! not apply, and framework-pack, incident, companion, and browser-adjacent
//! surfaces stay inspect-only — so a companion or browser card can never imply
//! hidden mutation parity it does not have. A card's [`FindingTruth`] separates
//! a `live` finding (computed by current local analysis) from an `imported` one
//! (ingested from a feed snapshot), and a `live` finding may only be backed by a
//! local source with current advisory freshness.
//!
//! [`HandoffContinuityRow`] records each cross-surface transition — desktop
//! reopen, browser handoff, or companion follow-up — and binds it back to an
//! originating card so package identity, update class, and review state stay
//! stable across the move instead of being flattened into a screenshot.
//!
//! The packet is checked in at
//! `artifacts/deps/m5/package-review-cross-surface-integration.json` and
//! embedded here, so this typed consumer and any CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! It carries no credential bodies or raw provider payloads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported package-review cross-surface integration packet schema version.
pub const PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_RECORD_KIND: &str =
    "package_review_cross_surface_integration";

/// Repo-relative path to the checked-in packet.
pub const PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_PATH: &str =
    "artifacts/deps/m5/package-review-cross-surface-integration.json";

/// Embedded checked-in packet JSON.
pub const PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/package-review-cross-surface-integration.json"
));

/// Host surface a dependency/package card is embedded in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// The desktop dependency workspace; origin of package mutation authority.
    DesktopWorkspace,
    /// A framework-pack health bundle.
    FrameworkPackHealth,
    /// A code-review workspace.
    ReviewWorkspace,
    /// An incident bundle / support packet.
    IncidentBundle,
    /// A companion-safe inspect view.
    CompanionInspect,
    /// A browser-adjacent handoff view.
    BrowserHandoff,
}

impl SurfaceClass {
    /// Every surface class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopWorkspace,
        Self::FrameworkPackHealth,
        Self::ReviewWorkspace,
        Self::IncidentBundle,
        Self::CompanionInspect,
        Self::BrowserHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopWorkspace => "desktop_workspace",
            Self::FrameworkPackHealth => "framework_pack_health",
            Self::ReviewWorkspace => "review_workspace",
            Self::IncidentBundle => "incident_bundle",
            Self::CompanionInspect => "companion_inspect",
            Self::BrowserHandoff => "browser_handoff",
        }
    }

    /// Highest write authority this surface may legitimately carry.
    ///
    /// Only the desktop workspace may mutate packages; review workspaces may
    /// stage a review but not apply; every other surface — framework-pack
    /// health, incident, companion, and browser-adjacent — stays inspect-only.
    pub const fn max_write_authority(self) -> WriteAuthority {
        match self {
            Self::DesktopWorkspace => WriteAuthority::FullMutation,
            Self::ReviewWorkspace => WriteAuthority::ReviewStage,
            Self::FrameworkPackHealth
            | Self::IncidentBundle
            | Self::CompanionInspect
            | Self::BrowserHandoff => WriteAuthority::InspectOnly,
        }
    }

    /// Whether the surface is companion or browser-adjacent and therefore must
    /// expose an explicit read-only inspect entry.
    pub const fn is_companion_or_browser(self) -> bool {
        matches!(self, Self::CompanionInspect | Self::BrowserHandoff)
    }
}

/// Write authority a card carries on its host surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteAuthority {
    /// Read-only inspection; no mutation and no implied mutation parity.
    InspectOnly,
    /// May stage or request a package change through review, but cannot apply.
    ReviewStage,
    /// Full package mutation authority (desktop only).
    FullMutation,
}

impl WriteAuthority {
    /// Every write authority, in declaration order.
    pub const ALL: [Self; 3] = [Self::InspectOnly, Self::ReviewStage, Self::FullMutation];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::ReviewStage => "review_stage",
            Self::FullMutation => "full_mutation",
        }
    }

    /// Monotonic rank used to compare authorities; higher means broader.
    pub const fn rank(self) -> u8 {
        match self {
            Self::InspectOnly => 0,
            Self::ReviewStage => 1,
            Self::FullMutation => 2,
        }
    }

    /// Whether the authority permits actually applying a package mutation.
    pub const fn permits_mutation(self) -> bool {
        matches!(self, Self::FullMutation)
    }

    /// Whether the authority is strictly read-only.
    pub const fn is_inspect_only(self) -> bool {
        matches!(self, Self::InspectOnly)
    }
}

/// Support class of a package, preserved as a card crosses surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Fully supported on the stable line.
    Supported,
    /// Provider-managed; support follows the managed provider.
    Managed,
    /// Community-maintained; no first-party support guarantee.
    Community,
    /// Out of support / deprecated.
    Unsupported,
}

impl SupportClass {
    /// Every support class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Supported,
        Self::Managed,
        Self::Community,
        Self::Unsupported,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Managed => "managed",
            Self::Community => "community",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Local-versus-managed source label preserved across surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLabel {
    /// Computed by local analysis of the workspace.
    Local,
    /// Served by a provider-managed registry.
    Managed,
    /// Served from an enterprise mirror of an origin feed.
    Mirrored,
    /// Imported from an external scanner or audit feed.
    Imported,
}

impl SourceLabel {
    /// Every source label, in declaration order.
    pub const ALL: [Self; 4] = [Self::Local, Self::Managed, Self::Mirrored, Self::Imported];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Managed => "managed",
            Self::Mirrored => "mirrored",
            Self::Imported => "imported",
        }
    }
}

/// Advisory freshness preserved across surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisoryFreshness {
    /// Advisory data is current and within freshness SLO.
    Current,
    /// Advisory data is present but stale.
    Stale,
    /// Only a point-in-time snapshot is available.
    SnapshotOnly,
    /// Advisory freshness cannot be established.
    Unknown,
}

impl AdvisoryFreshness {
    /// Every freshness class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::Stale,
        Self::SnapshotOnly,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::SnapshotOnly => "snapshot_only",
            Self::Unknown => "unknown",
        }
    }
}

/// Whether a finding is live (locally computed) or imported (ingested).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingTruth {
    /// Computed live by current local analysis of the exact build.
    Live,
    /// Ingested from an imported feed, scanner, or snapshot.
    Imported,
}

impl FindingTruth {
    /// Every finding-truth class, in declaration order.
    pub const ALL: [Self; 2] = [Self::Live, Self::Imported];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Imported => "imported",
        }
    }

    /// Whether the given `source` and `freshness` may back this finding truth.
    ///
    /// A `live` finding may only come from a local source with current advisory
    /// freshness; an `imported` finding accepts any source and freshness.
    pub const fn permitted_for(self, source: SourceLabel, freshness: AdvisoryFreshness) -> bool {
        match self {
            Self::Live => {
                matches!(source, SourceLabel::Local)
                    && matches!(freshness, AdvisoryFreshness::Current)
            }
            Self::Imported => true,
        }
    }
}

/// Update class of a package card, preserved across handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateClass {
    /// No pending update.
    None,
    /// A direct version bump.
    DirectBump,
    /// A security patch.
    SecurityPatch,
    /// A grouped refresh of related packages.
    GroupedRefresh,
    /// A lockfile-only refresh with no manifest bump.
    LockfileOnly,
    /// A piloted major-version upgrade.
    MajorPilot,
    /// A workspace-wide convergence of a shared version.
    WorkspaceConvergence,
}

impl UpdateClass {
    /// Every update class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::None,
        Self::DirectBump,
        Self::SecurityPatch,
        Self::GroupedRefresh,
        Self::LockfileOnly,
        Self::MajorPilot,
        Self::WorkspaceConvergence,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DirectBump => "direct_bump",
            Self::SecurityPatch => "security_patch",
            Self::GroupedRefresh => "grouped_refresh",
            Self::LockfileOnly => "lockfile_only",
            Self::MajorPilot => "major_pilot",
            Self::WorkspaceConvergence => "workspace_convergence",
        }
    }
}

/// Review state of a package card, stable across handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    /// No review has started.
    NotStarted,
    /// Review is in progress.
    InReview,
    /// Changes were requested.
    ChangesRequested,
    /// Review approved the change, but it is not yet applied.
    Approved,
    /// The change was applied after review.
    Applied,
    /// The change was applied and then rolled back.
    RolledBack,
}

impl ReviewState {
    /// Every review state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotStarted,
        Self::InReview,
        Self::ChangesRequested,
        Self::Approved,
        Self::Applied,
        Self::RolledBack,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::InReview => "in_review",
            Self::ChangesRequested => "changes_requested",
            Self::Approved => "approved",
            Self::Applied => "applied",
            Self::RolledBack => "rolled_back",
        }
    }

    /// Whether the state implies a mutation was actually applied to the
    /// workspace, which only a full-mutation authority can produce.
    pub const fn implies_applied_mutation(self) -> bool {
        matches!(self, Self::Applied | Self::RolledBack)
    }
}

/// Cross-surface transition recorded by a handoff row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionKind {
    /// Reopening the dependency workspace on desktop.
    DesktopReopen,
    /// Handing off to a browser-adjacent view.
    BrowserHandoff,
    /// Following up from a companion-safe inspect view.
    CompanionFollowup,
}

impl TransitionKind {
    /// Every transition kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DesktopReopen,
        Self::BrowserHandoff,
        Self::CompanionFollowup,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopReopen => "desktop_reopen",
            Self::BrowserHandoff => "browser_handoff",
            Self::CompanionFollowup => "companion_followup",
        }
    }

    /// The surface a transition of this kind must land on.
    pub const fn expected_target(self) -> SurfaceClass {
        match self {
            Self::DesktopReopen => SurfaceClass::DesktopWorkspace,
            Self::BrowserHandoff => SurfaceClass::BrowserHandoff,
            Self::CompanionFollowup => SurfaceClass::CompanionInspect,
        }
    }
}

/// Package ecosystem a card belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageEcosystem {
    /// Rust Cargo workspace and crate manifests.
    Cargo,
    /// Node package manifests using pnpm workspace semantics.
    NodePnpm,
    /// Python pip / project manifests.
    PythonPip,
    /// Any other ecosystem not separately modeled.
    Other,
}

impl PackageEcosystem {
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

/// Manifest scope a card covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestScopeKind {
    /// The whole repository / workspace.
    FullRepo,
    /// A single manifest.
    SingleManifest,
    /// A narrower slice (e.g., a path subtree).
    Slice,
}

impl ManifestScopeKind {
    /// Every scope kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::FullRepo, Self::SingleManifest, Self::Slice];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullRepo => "full_repo",
            Self::SingleManifest => "single_manifest",
            Self::Slice => "slice",
        }
    }
}

/// Stable package identity carried by a card and every handoff that references
/// it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageIdentity {
    /// Package coordinate (e.g., `crate:serde@1.0.0`).
    pub coordinate: String,
    /// Ecosystem the package belongs to.
    pub ecosystem: PackageEcosystem,
    /// Manifest path that owns the requirement.
    pub manifest_path: String,
    /// Scope kind that names how much of the workspace the card covers.
    pub scope_kind: ManifestScopeKind,
}

/// One dependency/package card embedded in a host surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyCard {
    /// Stable card id.
    pub card_id: String,
    /// Host surface this card is embedded in.
    pub surface: SurfaceClass,
    /// Write authority this card carries on its host surface.
    pub write_authority: WriteAuthority,
    /// Stable package identity.
    pub package_identity: PackageIdentity,
    /// Support class preserved across surfaces.
    pub support_class: SupportClass,
    /// Local-versus-managed source label preserved across surfaces.
    pub source_label: SourceLabel,
    /// Advisory freshness preserved across surfaces.
    pub advisory_freshness: AdvisoryFreshness,
    /// Whether the card's finding is live or imported.
    pub finding_truth: FindingTruth,
    /// Update class preserved across handoff.
    pub update_class: UpdateClass,
    /// Review state stable across handoff.
    pub review_state: ReviewState,
    /// Read-only inspect entry ref; required for companion/browser surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inspect_ref: Option<String>,
    /// Source refs backing the card.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl DependencyCard {
    /// Whether the card's write authority exceeds what its host surface allows.
    pub fn overreaches_authority(&self) -> bool {
        self.write_authority.rank() > self.surface.max_write_authority().rank()
    }
}

/// One cross-surface handoff that preserves a card's identity and state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffContinuityRow {
    /// Stable handoff id.
    pub handoff_id: String,
    /// Card id this handoff carries.
    pub card_id: String,
    /// Transition kind.
    pub transition: TransitionKind,
    /// Surface the handoff originated from.
    pub from_surface: SurfaceClass,
    /// Surface the handoff lands on.
    pub to_surface: SurfaceClass,
    /// Package identity carried across the transition; must match the card.
    pub package_identity: PackageIdentity,
    /// Update class carried across the transition; must match the card.
    pub update_class: UpdateClass,
    /// Review state carried across the transition; must match the card.
    pub review_state: ReviewState,
    /// Source refs backing the handoff.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageReviewCrossSurfaceIntegrationSummary {
    /// Total cards.
    pub total_cards: usize,
    /// Cards on the desktop workspace.
    pub desktop_workspace_cards: usize,
    /// Cards in framework-pack health bundles.
    pub framework_pack_health_cards: usize,
    /// Cards in review workspaces.
    pub review_workspace_cards: usize,
    /// Cards in incident bundles.
    pub incident_bundle_cards: usize,
    /// Cards in companion-safe inspect views.
    pub companion_inspect_cards: usize,
    /// Cards in browser-adjacent handoff views.
    pub browser_handoff_cards: usize,
    /// Cards that are strictly inspect-only.
    pub inspect_only_cards: usize,
    /// Cards that may stage a review.
    pub review_stage_cards: usize,
    /// Cards that carry full mutation authority.
    pub full_mutation_cards: usize,
    /// Cards whose finding is live.
    pub live_finding_cards: usize,
    /// Cards whose finding is imported.
    pub imported_finding_cards: usize,
    /// Total handoffs.
    pub total_handoffs: usize,
    /// Handoffs that preserve package identity, update class, and review state.
    pub identity_preserving_handoffs: usize,
}

/// A redaction-safe export row projected from a card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageReviewCrossSurfaceIntegrationExportRow {
    /// Card id.
    pub card_id: String,
    /// Surface token.
    pub surface: String,
    /// Write authority token.
    pub write_authority: String,
    /// Package coordinate.
    pub coordinate: String,
    /// Support class token.
    pub support_class: String,
    /// Source label token.
    pub source_label: String,
    /// Advisory freshness token.
    pub advisory_freshness: String,
    /// Finding-truth token.
    pub finding_truth: String,
    /// Whether the card is strictly inspect-only.
    pub inspect_only: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageReviewCrossSurfaceIntegrationExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected card rows.
    pub rows: Vec<PackageReviewCrossSurfaceIntegrationExportRow>,
    /// Whether every card stays within its host surface's authority.
    pub all_authority_within_bounds: bool,
}

/// The typed package-review cross-surface integration packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackageReviewCrossSurfaceIntegration {
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
    /// Closed surface-class vocabulary.
    pub surface_classes: Vec<SurfaceClass>,
    /// Closed write-authority vocabulary.
    pub write_authorities: Vec<WriteAuthority>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed source-label vocabulary.
    pub source_labels: Vec<SourceLabel>,
    /// Closed advisory-freshness vocabulary.
    pub advisory_freshness_classes: Vec<AdvisoryFreshness>,
    /// Closed finding-truth vocabulary.
    pub finding_truths: Vec<FindingTruth>,
    /// Closed update-class vocabulary.
    pub update_classes: Vec<UpdateClass>,
    /// Closed review-state vocabulary.
    pub review_states: Vec<ReviewState>,
    /// Closed transition-kind vocabulary.
    pub transition_kinds: Vec<TransitionKind>,
    /// Closed ecosystem vocabulary.
    pub ecosystems: Vec<PackageEcosystem>,
    /// Closed manifest-scope-kind vocabulary.
    pub manifest_scope_kinds: Vec<ManifestScopeKind>,
    /// Dependency cards embedded across surfaces.
    #[serde(default)]
    pub cards: Vec<DependencyCard>,
    /// Cross-surface handoff continuity rows.
    #[serde(default)]
    pub handoffs: Vec<HandoffContinuityRow>,
    /// Summary counts.
    pub summary: PackageReviewCrossSurfaceIntegrationSummary,
}

impl PackageReviewCrossSurfaceIntegration {
    /// Returns the card for `card_id`.
    pub fn card(&self, card_id: &str) -> Option<&DependencyCard> {
        self.cards.iter().find(|c| c.card_id == card_id)
    }

    /// Whether every card stays within its host surface's authority bounds.
    pub fn all_authority_within_bounds(&self) -> bool {
        self.cards.iter().all(|c| !c.overreaches_authority())
    }

    /// Whether `handoff` preserves the referenced card's identity, update class,
    /// and review state.
    ///
    /// Returns `false` when the card is unknown or any field diverges.
    pub fn handoff_preserves_truth(&self, handoff: &HandoffContinuityRow) -> bool {
        match self.card(&handoff.card_id) {
            Some(card) => {
                card.package_identity == handoff.package_identity
                    && card.update_class == handoff.update_class
                    && card.review_state == handoff.review_state
            }
            None => false,
        }
    }

    /// Recomputes the summary block from the cards and handoffs.
    pub fn computed_summary(&self) -> PackageReviewCrossSurfaceIntegrationSummary {
        let count_surface =
            |surface: SurfaceClass| self.cards.iter().filter(|c| c.surface == surface).count();
        let count_authority = |authority: WriteAuthority| {
            self.cards
                .iter()
                .filter(|c| c.write_authority == authority)
                .count()
        };
        let count_truth = |truth: FindingTruth| {
            self.cards
                .iter()
                .filter(|c| c.finding_truth == truth)
                .count()
        };
        PackageReviewCrossSurfaceIntegrationSummary {
            total_cards: self.cards.len(),
            desktop_workspace_cards: count_surface(SurfaceClass::DesktopWorkspace),
            framework_pack_health_cards: count_surface(SurfaceClass::FrameworkPackHealth),
            review_workspace_cards: count_surface(SurfaceClass::ReviewWorkspace),
            incident_bundle_cards: count_surface(SurfaceClass::IncidentBundle),
            companion_inspect_cards: count_surface(SurfaceClass::CompanionInspect),
            browser_handoff_cards: count_surface(SurfaceClass::BrowserHandoff),
            inspect_only_cards: count_authority(WriteAuthority::InspectOnly),
            review_stage_cards: count_authority(WriteAuthority::ReviewStage),
            full_mutation_cards: count_authority(WriteAuthority::FullMutation),
            live_finding_cards: count_truth(FindingTruth::Live),
            imported_finding_cards: count_truth(FindingTruth::Imported),
            total_handoffs: self.handoffs.len(),
            identity_preserving_handoffs: self
                .handoffs
                .iter()
                .filter(|h| self.handoff_preserves_truth(h))
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces render instead of
    /// cloning status text.
    pub fn export_projection(&self) -> PackageReviewCrossSurfaceIntegrationExportProjection {
        let rows = self
            .cards
            .iter()
            .map(|card| PackageReviewCrossSurfaceIntegrationExportRow {
                card_id: card.card_id.clone(),
                surface: card.surface.as_str().to_owned(),
                write_authority: card.write_authority.as_str().to_owned(),
                coordinate: card.package_identity.coordinate.clone(),
                support_class: card.support_class.as_str().to_owned(),
                source_label: card.source_label.as_str().to_owned(),
                advisory_freshness: card.advisory_freshness.as_str().to_owned(),
                finding_truth: card.finding_truth.as_str().to_owned(),
                inspect_only: card.write_authority.is_inspect_only(),
                summary: format!(
                    "{} on {} ({}, {} finding via {}, advisory {})",
                    card.package_identity.coordinate,
                    card.surface.as_str(),
                    card.write_authority.as_str(),
                    card.finding_truth.as_str(),
                    card.source_label.as_str(),
                    card.advisory_freshness.as_str()
                ),
            })
            .collect();
        PackageReviewCrossSurfaceIntegrationExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_authority_within_bounds: self.all_authority_within_bounds(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<PackageReviewCrossSurfaceIntegrationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_cards = BTreeSet::new();
        for card in &self.cards {
            if !seen_cards.insert(card.card_id.clone()) {
                violations.push(
                    PackageReviewCrossSurfaceIntegrationViolation::DuplicateCardId {
                        card_id: card.card_id.clone(),
                    },
                );
            }
            self.validate_card(card, &mut violations);
        }

        let mut seen_handoffs = BTreeSet::new();
        for handoff in &self.handoffs {
            if !seen_handoffs.insert(handoff.handoff_id.clone()) {
                violations.push(
                    PackageReviewCrossSurfaceIntegrationViolation::DuplicateHandoffId {
                        handoff_id: handoff.handoff_id.clone(),
                    },
                );
            }
            self.validate_handoff(handoff, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(PackageReviewCrossSurfaceIntegrationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(
        &self,
        violations: &mut Vec<PackageReviewCrossSurfaceIntegrationViolation>,
    ) {
        if self.schema_version != PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_SCHEMA_VERSION {
            violations.push(
                PackageReviewCrossSurfaceIntegrationViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_RECORD_KIND {
            violations.push(
                PackageReviewCrossSurfaceIntegrationViolation::UnsupportedRecordKind {
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
                violations.push(PackageReviewCrossSurfaceIntegrationViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "surface_classes",
                self.surface_classes == SurfaceClass::ALL.to_vec(),
            ),
            (
                "write_authorities",
                self.write_authorities == WriteAuthority::ALL.to_vec(),
            ),
            (
                "support_classes",
                self.support_classes == SupportClass::ALL.to_vec(),
            ),
            (
                "source_labels",
                self.source_labels == SourceLabel::ALL.to_vec(),
            ),
            (
                "advisory_freshness_classes",
                self.advisory_freshness_classes == AdvisoryFreshness::ALL.to_vec(),
            ),
            (
                "finding_truths",
                self.finding_truths == FindingTruth::ALL.to_vec(),
            ),
            (
                "update_classes",
                self.update_classes == UpdateClass::ALL.to_vec(),
            ),
            (
                "review_states",
                self.review_states == ReviewState::ALL.to_vec(),
            ),
            (
                "transition_kinds",
                self.transition_kinds == TransitionKind::ALL.to_vec(),
            ),
            (
                "ecosystems",
                self.ecosystems == PackageEcosystem::ALL.to_vec(),
            ),
            (
                "manifest_scope_kinds",
                self.manifest_scope_kinds == ManifestScopeKind::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    PackageReviewCrossSurfaceIntegrationViolation::ClosedVocabularyMismatch {
                        field,
                    },
                );
            }
        }
    }

    fn validate_identity(
        &self,
        id: &str,
        identity: &PackageIdentity,
        violations: &mut Vec<PackageReviewCrossSurfaceIntegrationViolation>,
    ) {
        for (field, value) in [
            ("coordinate", &identity.coordinate),
            ("manifest_path", &identity.manifest_path),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageReviewCrossSurfaceIntegrationViolation::EmptyField {
                    id: id.to_owned(),
                    field_name: field,
                });
            }
        }
    }

    fn validate_card(
        &self,
        card: &DependencyCard,
        violations: &mut Vec<PackageReviewCrossSurfaceIntegrationViolation>,
    ) {
        for (field, value) in [("card_id", &card.card_id), ("note", &card.note)] {
            if value.trim().is_empty() {
                violations.push(PackageReviewCrossSurfaceIntegrationViolation::EmptyField {
                    id: card.card_id.clone(),
                    field_name: field,
                });
            }
        }
        self.validate_identity(&card.card_id, &card.package_identity, violations);

        // Companion, browser, incident, and framework surfaces must never carry
        // more authority than the host allows; only desktop may mutate.
        if card.overreaches_authority() {
            violations.push(
                PackageReviewCrossSurfaceIntegrationViolation::WriteAuthorityOverreach {
                    card_id: card.card_id.clone(),
                    surface: card.surface.as_str(),
                    authority: card.write_authority.as_str(),
                },
            );
        }

        // A live finding may only be backed by current local analysis.
        if !card
            .finding_truth
            .permitted_for(card.source_label, card.advisory_freshness)
        {
            violations.push(
                PackageReviewCrossSurfaceIntegrationViolation::OverstatedFindingTruth {
                    card_id: card.card_id.clone(),
                    finding: card.finding_truth.as_str(),
                    source: card.source_label.as_str(),
                    freshness: card.advisory_freshness.as_str(),
                },
            );
        }

        // An applied/rolled-back review state implies a real mutation, which only
        // a full-mutation authority can produce.
        if card.review_state.implies_applied_mutation() && !card.write_authority.permits_mutation()
        {
            violations.push(
                PackageReviewCrossSurfaceIntegrationViolation::MutationStateWithoutAuthority {
                    card_id: card.card_id.clone(),
                    review_state: card.review_state.as_str(),
                    authority: card.write_authority.as_str(),
                },
            );
        }

        // Companion/browser cards must expose an explicit read-only inspect entry.
        if card.surface.is_companion_or_browser()
            && card
                .inspect_ref
                .as_deref()
                .map(str::trim)
                .map(str::is_empty)
                .unwrap_or(true)
        {
            violations.push(
                PackageReviewCrossSurfaceIntegrationViolation::MissingInspectRef {
                    card_id: card.card_id.clone(),
                    surface: card.surface.as_str(),
                },
            );
        }
    }

    fn validate_handoff(
        &self,
        handoff: &HandoffContinuityRow,
        violations: &mut Vec<PackageReviewCrossSurfaceIntegrationViolation>,
    ) {
        for (field, value) in [
            ("handoff_id", &handoff.handoff_id),
            ("card_id", &handoff.card_id),
            ("note", &handoff.note),
        ] {
            if value.trim().is_empty() {
                violations.push(PackageReviewCrossSurfaceIntegrationViolation::EmptyField {
                    id: handoff.handoff_id.clone(),
                    field_name: field,
                });
            }
        }
        self.validate_identity(&handoff.handoff_id, &handoff.package_identity, violations);

        // The transition kind and its landing surface must agree, so a browser or
        // companion handoff cannot quietly land on a mutation-capable surface.
        if handoff.to_surface != handoff.transition.expected_target() {
            violations.push(
                PackageReviewCrossSurfaceIntegrationViolation::TransitionSurfaceMismatch {
                    handoff_id: handoff.handoff_id.clone(),
                    transition: handoff.transition.as_str(),
                    to_surface: handoff.to_surface.as_str(),
                },
            );
        }

        match self.card(&handoff.card_id) {
            None => violations.push(
                PackageReviewCrossSurfaceIntegrationViolation::HandoffUnknownCard {
                    handoff_id: handoff.handoff_id.clone(),
                    card_id: handoff.card_id.clone(),
                },
            ),
            Some(card) => {
                if card.package_identity != handoff.package_identity
                    || card.update_class != handoff.update_class
                    || card.review_state != handoff.review_state
                {
                    violations.push(
                        PackageReviewCrossSurfaceIntegrationViolation::HandoffDropsTruth {
                            handoff_id: handoff.handoff_id.clone(),
                            card_id: handoff.card_id.clone(),
                        },
                    );
                }
            }
        }
    }
}

/// A validation violation for the package-review cross-surface integration
/// packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageReviewCrossSurfaceIntegrationViolation {
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
        /// Row, handoff, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A card id appears more than once.
    DuplicateCardId {
        /// Duplicate card id.
        card_id: String,
    },
    /// A handoff id appears more than once.
    DuplicateHandoffId {
        /// Duplicate handoff id.
        handoff_id: String,
    },
    /// A card carries more write authority than its host surface allows.
    WriteAuthorityOverreach {
        /// Card id.
        card_id: String,
        /// Surface token.
        surface: &'static str,
        /// Authority token.
        authority: &'static str,
    },
    /// A live finding is backed by a non-local or non-current source.
    OverstatedFindingTruth {
        /// Card id.
        card_id: String,
        /// Finding-truth token.
        finding: &'static str,
        /// Source token.
        source: &'static str,
        /// Freshness token.
        freshness: &'static str,
    },
    /// A card claims an applied/rolled-back state without mutation authority.
    MutationStateWithoutAuthority {
        /// Card id.
        card_id: String,
        /// Review-state token.
        review_state: &'static str,
        /// Authority token.
        authority: &'static str,
    },
    /// A companion/browser card omits a read-only inspect entry.
    MissingInspectRef {
        /// Card id.
        card_id: String,
        /// Surface token.
        surface: &'static str,
    },
    /// A handoff's transition kind disagrees with its landing surface.
    TransitionSurfaceMismatch {
        /// Handoff id.
        handoff_id: String,
        /// Transition token.
        transition: &'static str,
        /// Landing-surface token.
        to_surface: &'static str,
    },
    /// A handoff references a card the packet does not declare.
    HandoffUnknownCard {
        /// Handoff id.
        handoff_id: String,
        /// Referenced card id.
        card_id: String,
    },
    /// A handoff drops package identity, update class, or review state relative
    /// to the card it carries.
    HandoffDropsTruth {
        /// Handoff id.
        handoff_id: String,
        /// Referenced card id.
        card_id: String,
    },
    /// The summary counts disagree with the cards and handoffs.
    SummaryMismatch,
}

impl fmt::Display for PackageReviewCrossSurfaceIntegrationViolation {
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
            Self::DuplicateCardId { card_id } => {
                write!(f, "duplicate card id {card_id}")
            }
            Self::DuplicateHandoffId { handoff_id } => {
                write!(f, "duplicate handoff id {handoff_id}")
            }
            Self::WriteAuthorityOverreach {
                card_id,
                surface,
                authority,
            } => {
                write!(
                    f,
                    "card {card_id} on surface {surface} carries authority {authority} beyond what the surface allows"
                )
            }
            Self::OverstatedFindingTruth {
                card_id,
                finding,
                source,
                freshness,
            } => {
                write!(
                    f,
                    "card {card_id} claims {finding} finding but its source is {source} and advisory freshness is {freshness}"
                )
            }
            Self::MutationStateWithoutAuthority {
                card_id,
                review_state,
                authority,
            } => {
                write!(
                    f,
                    "card {card_id} claims review state {review_state} but only carries authority {authority}"
                )
            }
            Self::MissingInspectRef { card_id, surface } => {
                write!(
                    f,
                    "card {card_id} on surface {surface} must expose a read-only inspect_ref"
                )
            }
            Self::TransitionSurfaceMismatch {
                handoff_id,
                transition,
                to_surface,
            } => {
                write!(
                    f,
                    "handoff {handoff_id} transition {transition} cannot land on surface {to_surface}"
                )
            }
            Self::HandoffUnknownCard {
                handoff_id,
                card_id,
            } => {
                write!(f, "handoff {handoff_id} references unknown card {card_id}")
            }
            Self::HandoffDropsTruth {
                handoff_id,
                card_id,
            } => {
                write!(
                    f,
                    "handoff {handoff_id} drops identity/update-class/review-state relative to card {card_id}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the cards/handoffs")
            }
        }
    }
}

impl Error for PackageReviewCrossSurfaceIntegrationViolation {}

/// Loads the embedded package-review cross-surface integration packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`PackageReviewCrossSurfaceIntegration`].
pub fn current_package_review_cross_surface_integration(
) -> Result<PackageReviewCrossSurfaceIntegration, serde_json::Error> {
    serde_json::from_str(PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_JSON)
}

#[cfg(test)]
mod tests;
