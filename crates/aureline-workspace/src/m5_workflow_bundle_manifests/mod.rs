//! Canonical M5 workflow-bundle manifests: one versioned, diffable manifest per
//! claimed M5 launch wedge that captures the minimum cohesive experience for a
//! named persona, stack, and archetype without hiding trust, registry, or
//! compatibility posture.
//!
//! Each [`WorkflowBundleManifest`] turns a launch wedge ([`M5Wedge`]) into a real bundle rather
//! than an ad hoc pile of templates, extensions, docs, and setup notes. The manifest carries a
//! version, the persona/stack/archetype it targets, a [`CertificationTarget`], and a single
//! diffable [`BundleComponent`] list spanning every content category the spec names — extension
//! sets, profile and layout presets, task/launch/debug recipes, docs and tour packs, template and
//! scaffold references, and migration mappings ([`BundleComponentKind`]).
//!
//! Lifecycle-sensitive dependencies are declared, never hidden. Each component records a
//! [`LifecycleStage`] (stable, preview, labs, policy-gated, mirror-only, or bounded-platform), and
//! any non-stable component is review-gated ([`BundleComponent::requires_review`]) and rolled up
//! into the manifest's [`WorkflowBundleManifest::dependency_markers`]. A manifest that depends on
//! any non-stable capability MUST set [`WorkflowBundleManifest::discloses_non_stable_dependencies`]
//! so discovery, review, and export surfaces always see the weaker posture; a bundle may still aim
//! at a certified target while disclosing a non-stable dependency, but it can never bury it.
//!
//! Manifests stay diffable, mirrorable, and export-safe. Every manifest and every component holds
//! [`WorkflowBundleManifest::diffable`], [`WorkflowBundleManifest::mirrorable`], and
//! [`WorkflowBundleManifest::export_safe`] true and holds the
//! [`WorkflowBundleManifest::opaque_binary_state`] guardrail false — opaque binary bundle state is
//! forbidden on these claimed paths.
//!
//! One manifest drives every consumer: the same packet feeds start center, migration center,
//! bundle detail pages, and release/help surfaces through opaque surface refs, so discovery,
//! review, diagnostics, and claim publication ingest the same object model instead of cloning
//! status text.
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-workflow-bundle-manifests.json` and
//! embedded here. It is metadata-only: every field is a typed state, a count, or an opaque ref,
//! and it carries no credential bodies, raw provider payloads, raw local paths, or bundle binary
//! contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::m5_admission_and_routing::M5Wedge;

/// Supported M5 workflow-bundle-manifests packet schema version.
pub const M5_WORKFLOW_BUNDLE_MANIFESTS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_WORKFLOW_BUNDLE_MANIFESTS_RECORD_KIND: &str = "m5_workflow_bundle_manifests_packet";

/// Repo-relative path to the checked-in packet.
pub const M5_WORKFLOW_BUNDLE_MANIFESTS_PATH: &str =
    "artifacts/workspace/m5/m5-workflow-bundle-manifests.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_WORKFLOW_BUNDLE_MANIFESTS_SCHEMA_REF: &str =
    "schemas/workspace/m5-workflow-bundle-manifests.schema.json";

/// Repo-relative path to the companion document.
pub const M5_WORKFLOW_BUNDLE_MANIFESTS_DOC_REF: &str =
    "docs/workspace/m5/m5-workflow-bundle-manifests.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_WORKFLOW_BUNDLE_MANIFESTS_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-workflow-bundle-manifests";

/// Embedded checked-in packet JSON.
pub const M5_WORKFLOW_BUNDLE_MANIFESTS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-workflow-bundle-manifests.json"
));

/// The content category a [`BundleComponent`] contributes to a manifest.
///
/// The closed set spans every category a real workflow bundle composes: an extension set, profile
/// and layout and settings presets, task/launch/debug recipes, docs and tour packs, template and
/// scaffold references, and migration mappings. Each category stays distinct so discovery and
/// review never collapse a docs pack into a task recipe or a scaffold into a migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleComponentKind {
    /// A workspace extension the bundle installs.
    Extension,
    /// A profile preset (keymap, theme, settings profile).
    ProfilePreset,
    /// A surface or layout preset.
    LayoutPreset,
    /// A settings or token preset.
    SettingsPreset,
    /// A task recipe.
    TaskRecipe,
    /// A launch recipe.
    LaunchRecipe,
    /// A debug recipe.
    DebugRecipe,
    /// A docs pack.
    DocsPack,
    /// A guided tour pack.
    TourPack,
    /// A reference to a checked-in template.
    TemplateRef,
    /// A reference to a scaffold generator.
    ScaffoldRef,
    /// A migration mapping from a prior tool or version into this bundle.
    MigrationMapping,
}

impl BundleComponentKind {
    /// Every component kind, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Extension,
        Self::ProfilePreset,
        Self::LayoutPreset,
        Self::SettingsPreset,
        Self::TaskRecipe,
        Self::LaunchRecipe,
        Self::DebugRecipe,
        Self::DocsPack,
        Self::TourPack,
        Self::TemplateRef,
        Self::ScaffoldRef,
        Self::MigrationMapping,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Extension => "extension",
            Self::ProfilePreset => "profile_preset",
            Self::LayoutPreset => "layout_preset",
            Self::SettingsPreset => "settings_preset",
            Self::TaskRecipe => "task_recipe",
            Self::LaunchRecipe => "launch_recipe",
            Self::DebugRecipe => "debug_recipe",
            Self::DocsPack => "docs_pack",
            Self::TourPack => "tour_pack",
            Self::TemplateRef => "template_ref",
            Self::ScaffoldRef => "scaffold_ref",
            Self::MigrationMapping => "migration_mapping",
        }
    }

    /// Whether this kind is a task, launch, or debug recipe.
    pub const fn is_recipe(self) -> bool {
        matches!(
            self,
            Self::TaskRecipe | Self::LaunchRecipe | Self::DebugRecipe
        )
    }
}

/// The lifecycle stage a bundle component's underlying capability sits in.
///
/// A bundle may depend on a non-stable capability, but the dependency must be declared rather than
/// hidden. Anything other than [`Self::Stable`] is a lifecycle-sensitive marker that forces review
/// and disclosure across discovery, review, and export surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStage {
    /// A generally available, stable capability.
    Stable,
    /// A Preview capability, not yet stable.
    Preview,
    /// A Labs/experimental capability.
    Labs,
    /// A policy-gated capability subject to org policy.
    PolicyGated,
    /// A mirror-only capability available only through a mirror.
    MirrorOnly,
    /// A capability bounded to specific platforms.
    BoundedPlatform,
}

impl LifecycleStage {
    /// Every lifecycle stage, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Preview,
        Self::Labs,
        Self::PolicyGated,
        Self::MirrorOnly,
        Self::BoundedPlatform,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::Labs => "labs",
            Self::PolicyGated => "policy_gated",
            Self::MirrorOnly => "mirror_only",
            Self::BoundedPlatform => "bounded_platform",
        }
    }

    /// Whether this stage is anything other than stable.
    ///
    /// A non-stable stage is a lifecycle-sensitive dependency marker: it must be review-gated on
    /// the component and disclosed at the manifest level.
    pub const fn is_non_stable(self) -> bool {
        !matches!(self, Self::Stable)
    }
}

/// The certification posture a manifest aims at.
///
/// The target is what the bundle claims, not a free pass: a draft manifest may only claim
/// [`Self::LocalDraft`], and non-certified targets must carry a caveat so the weaker assurance is
/// never silent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationTarget {
    /// A certified launch wedge.
    Certified,
    /// A managed, org-approved bundle.
    ManagedApproved,
    /// A community-reviewed bundle.
    CommunityReviewed,
    /// An imported bundle pending review.
    ImportedPendingReview,
    /// A local draft with no certification claim.
    LocalDraft,
}

impl CertificationTarget {
    /// Every certification target, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Certified,
        Self::ManagedApproved,
        Self::CommunityReviewed,
        Self::ImportedPendingReview,
        Self::LocalDraft,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::ManagedApproved => "managed_approved",
            Self::CommunityReviewed => "community_reviewed",
            Self::ImportedPendingReview => "imported_pending_review",
            Self::LocalDraft => "local_draft",
        }
    }

    /// Whether a bundle with this target may present as certified.
    pub const fn presents_as_certified(self) -> bool {
        matches!(self, Self::Certified)
    }
}

/// The publication lifecycle of a manifest version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestPublicationState {
    /// A draft manifest not yet published.
    Draft,
    /// A published manifest.
    Published,
    /// A deprecated or archived manifest.
    Deprecated,
}

impl ManifestPublicationState {
    /// Every publication state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Draft, Self::Published, Self::Deprecated];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Published => "published",
            Self::Deprecated => "deprecated",
        }
    }
}

/// One declared component of a workflow bundle.
///
/// A component is a diffable reference, never an opaque blob: it names its kind, the lifecycle
/// stage of the capability it depends on, whether it is review-gated, a one-line label, and an
/// opaque ref into the registry or repo. Migration mappings additionally carry a `from`/`to` pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleComponent {
    /// Which content category this component contributes.
    pub component_kind: BundleComponentKind,
    /// Stable component identifier within the manifest.
    pub component_id: String,
    /// The lifecycle stage of the capability this component depends on.
    pub lifecycle_stage: LifecycleStage,
    /// Whether the component is review-gated. Must be `true` for any non-stable lifecycle stage.
    pub requires_review: bool,
    /// Guardrail: the component is diffable, never an opaque blob. Always `true`.
    pub diffable: bool,
    /// A human-readable, one-line component label.
    pub label: String,
    /// Opaque registry/repo ref backing the component.
    pub component_ref: String,
    /// Migration source token. Required and non-empty iff the kind is a migration mapping.
    #[serde(default)]
    pub migration_from: Option<String>,
    /// Migration target token. Required and non-empty iff the kind is a migration mapping.
    #[serde(default)]
    pub migration_to: Option<String>,
}

impl BundleComponent {
    /// Whether the migration `from`/`to` fields are populated exactly when the kind is a mapping.
    pub fn migration_fields_consistent(&self) -> bool {
        let from_present = self
            .migration_from
            .as_deref()
            .is_some_and(|s| !s.trim().is_empty());
        let to_present = self
            .migration_to
            .as_deref()
            .is_some_and(|s| !s.trim().is_empty());
        if self.component_kind == BundleComponentKind::MigrationMapping {
            from_present && to_present
        } else {
            self.migration_from.is_none() && self.migration_to.is_none()
        }
    }

    /// Whether a non-stable lifecycle stage is review-gated as required.
    pub fn lifecycle_gated(&self) -> bool {
        !self.lifecycle_stage.is_non_stable() || self.requires_review
    }

    /// Whether this component is internally consistent and never an opaque blob.
    pub fn is_consistent(&self) -> bool {
        self.diffable
            && self.lifecycle_gated()
            && self.migration_fields_consistent()
            && !self.component_id.trim().is_empty()
            && !self.label.trim().is_empty()
            && !self.component_ref.trim().is_empty()
    }
}

/// One versioned workflow-bundle manifest for a single M5 launch wedge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowBundleManifest {
    /// Stable bundle identifier.
    pub bundle_id: String,
    /// Bundle manifest version (semver-style token).
    pub bundle_version: String,
    /// The M5 launch wedge this bundle composes.
    pub wedge: M5Wedge,
    /// The named persona the bundle targets.
    pub persona: String,
    /// The named stack the bundle targets.
    pub stack: String,
    /// The named archetype the bundle targets.
    pub archetype: String,
    /// The publication lifecycle of this manifest version.
    pub publication_state: ManifestPublicationState,
    /// The certification posture this bundle aims at.
    pub certification_target: CertificationTarget,
    /// The diffable component set spanning every content category.
    pub components: Vec<BundleComponent>,
    /// Distinct non-stable lifecycle stages this bundle depends on, sorted. Must equal the set
    /// computed from [`Self::components`].
    pub dependency_markers: Vec<LifecycleStage>,
    /// Whether the bundle discloses a non-stable dependency. Must equal whether
    /// [`Self::dependency_markers`] is non-empty.
    pub discloses_non_stable_dependencies: bool,
    /// Guardrail: the manifest is diffable. Always `true`.
    pub diffable: bool,
    /// Guardrail: the manifest is mirrorable. Always `true`.
    pub mirrorable: bool,
    /// Guardrail: the manifest is export-safe. Always `true`.
    pub export_safe: bool,
    /// Guardrail: opaque binary bundle state is forbidden. Always `false`.
    pub opaque_binary_state: bool,
    /// Opaque manifest provenance ref.
    pub manifest_provenance_ref: String,
    /// Opaque certification-evidence ref.
    pub certification_evidence_ref: String,
    /// Opaque migration provenance ref.
    pub migration_provenance_ref: String,
    /// Accountable owner.
    pub owner: String,
    /// Caveats shown on the bundle.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Start-center consumer ref.
    pub start_center_ref: String,
    /// Migration-center consumer ref.
    pub migration_center_ref: String,
    /// Bundle-detail-page consumer ref.
    pub bundle_detail_ref: String,
    /// Diagnostics surface ref.
    pub diagnostics_ref: String,
    /// Support-export surface ref.
    pub support_export_ref: String,
    /// Help surface ref.
    pub help_surface_ref: String,
    /// Release-evidence surface ref.
    pub release_evidence_ref: String,
    /// A reviewer note summarizing the bundle.
    pub note: String,
}

impl WorkflowBundleManifest {
    /// Components of the given kind.
    pub fn components_of_kind(
        &self,
        kind: BundleComponentKind,
    ) -> impl Iterator<Item = &BundleComponent> {
        self.components
            .iter()
            .filter(move |c| c.component_kind == kind)
    }

    /// Whether the bundle includes at least one component of the given kind.
    pub fn has_kind(&self, kind: BundleComponentKind) -> bool {
        self.components.iter().any(|c| c.component_kind == kind)
    }

    /// Whether any component depends on a non-stable capability.
    pub fn has_non_stable_components(&self) -> bool {
        self.components
            .iter()
            .any(|c| c.lifecycle_stage.is_non_stable())
    }

    /// Whether any component is review-gated.
    pub fn has_review_required_components(&self) -> bool {
        self.components.iter().any(|c| c.requires_review)
    }

    /// The distinct non-stable lifecycle stages this bundle depends on, sorted.
    pub fn computed_dependency_markers(&self) -> Vec<LifecycleStage> {
        let mut set = BTreeSet::new();
        for component in &self.components {
            if component.lifecycle_stage.is_non_stable() {
                set.insert(component.lifecycle_stage);
            }
        }
        set.into_iter().collect()
    }

    /// Whether the bundle captures a minimum cohesive experience.
    ///
    /// A real workflow bundle composes at least an extension set and at least one runnable recipe
    /// (task, launch, or debug); an empty pile of docs or presets is not a cohesive experience.
    pub fn has_minimum_cohesive_experience(&self) -> bool {
        self.has_kind(BundleComponentKind::Extension)
            && self.components.iter().any(|c| c.component_kind.is_recipe())
    }

    /// Whether the guardrails that keep a manifest diffable and non-opaque are held correctly.
    pub fn guards_correct(&self) -> bool {
        self.diffable && self.mirrorable && self.export_safe && !self.opaque_binary_state
    }

    /// Whether the non-stable disclosure flag matches the computed dependency markers.
    pub fn disclosure_consistent(&self) -> bool {
        self.discloses_non_stable_dependencies == self.has_non_stable_components()
            && self.dependency_markers == self.computed_dependency_markers()
    }

    /// Whether the certification target is permitted under the publication state.
    ///
    /// A draft manifest may only claim a local-draft target; once a bundle claims any stronger
    /// posture it must be published.
    pub fn target_within_publication(&self) -> bool {
        match self.publication_state {
            ManifestPublicationState::Draft => {
                self.certification_target == CertificationTarget::LocalDraft
            }
            ManifestPublicationState::Published | ManifestPublicationState::Deprecated => true,
        }
    }

    /// The certified presentation the gate computes from the certification target.
    pub fn presents_as_certified(&self) -> bool {
        self.certification_target.presents_as_certified()
    }

    /// Whether the bundle carries complete manifest, certification, and migration provenance.
    pub fn provenance_complete(&self) -> bool {
        !self.manifest_provenance_ref.trim().is_empty()
            && !self.certification_evidence_ref.trim().is_empty()
            && !self.migration_provenance_ref.trim().is_empty()
    }

    /// Whether a caveat is required on this bundle.
    ///
    /// Anything not certified, anything depending on a non-stable capability, anything with a
    /// review-gated component, and any deprecated manifest must carry a caveat so the weaker
    /// posture is never silent.
    pub fn caveats_required(&self) -> bool {
        self.certification_target != CertificationTarget::Certified
            || self.has_non_stable_components()
            || self.has_review_required_components()
            || self.publication_state == ManifestPublicationState::Deprecated
    }
}

/// Summary counts rolled up across every workflow-bundle manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5WorkflowBundleManifestsSummary {
    /// Total manifests.
    pub total_manifests: usize,
    /// Manifest count (equals `total_manifests`; kept for export parity).
    pub manifest_count: usize,
    /// Manifests aiming at a certified target.
    pub certified_target_manifests: usize,
    /// Manifests aiming at a managed-approved target.
    pub managed_approved_target_manifests: usize,
    /// Manifests aiming at a community-reviewed target.
    pub community_reviewed_target_manifests: usize,
    /// Manifests aiming at an imported-pending-review target.
    pub imported_pending_review_target_manifests: usize,
    /// Manifests aiming at a local-draft target.
    pub local_draft_target_manifests: usize,
    /// Published manifests.
    pub published_manifests: usize,
    /// Draft manifests.
    pub draft_manifests: usize,
    /// Deprecated manifests.
    pub deprecated_manifests: usize,
    /// Manifests that depend on a non-stable capability.
    pub manifests_with_non_stable_dependencies: usize,
    /// Manifests that disclose a non-stable dependency.
    pub manifests_disclosing_non_stable: usize,
    /// Total components across all manifests.
    pub total_components: usize,
    /// Extension components.
    pub extension_components: usize,
    /// Recipe components (task, launch, or debug).
    pub recipe_components: usize,
    /// Docs and tour-pack components.
    pub docs_or_tour_components: usize,
    /// Template and scaffold-ref components.
    pub template_or_scaffold_components: usize,
    /// Migration-mapping components.
    pub migration_mapping_components: usize,
    /// Components that depend on a non-stable capability.
    pub non_stable_components: usize,
    /// Components that are review-gated.
    pub review_required_components: usize,
}

/// One redaction-safe export row projected from a manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkflowBundleManifestsExportRow {
    /// Bundle id.
    pub bundle_id: String,
    /// Bundle version.
    pub bundle_version: String,
    /// Wedge token.
    pub wedge: M5Wedge,
    /// Certification target token.
    pub certification_target: CertificationTarget,
    /// Publication state token.
    pub publication_state: ManifestPublicationState,
    /// Component kinds composed.
    pub component_kinds: Vec<BundleComponentKind>,
    /// Distinct non-stable dependency markers.
    pub dependency_markers: Vec<LifecycleStage>,
    /// Whether the bundle discloses a non-stable dependency.
    pub discloses_non_stable_dependencies: bool,
    /// Whether the bundle presents as certified.
    pub presents_as_certified: bool,
    /// Manifest provenance ref.
    pub manifest_provenance_ref: String,
}

/// A redaction-safe export projection of the whole packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkflowBundleManifestsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected manifest rows.
    pub manifests: Vec<M5WorkflowBundleManifestsExportRow>,
    /// Whether every manifest is gate-consistent.
    pub all_manifests_consistent: bool,
    /// Manifests that depend on a non-stable capability.
    pub manifests_with_non_stable_dependencies: usize,
    /// Manifests aiming at a certified target.
    pub certified_target_manifests: usize,
}

/// The typed M5 workflow-bundle-manifests packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5WorkflowBundleManifestsPacket {
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
    /// Scheme the packet mints stable bundle identities under.
    pub bundle_identity_scheme: String,
    /// Closed wedge vocabulary.
    pub wedges: Vec<M5Wedge>,
    /// Closed component-kind vocabulary.
    pub component_kinds: Vec<BundleComponentKind>,
    /// Closed lifecycle-stage vocabulary.
    pub lifecycle_stages: Vec<LifecycleStage>,
    /// Closed certification-target vocabulary.
    pub certification_targets: Vec<CertificationTarget>,
    /// Closed publication-state vocabulary.
    pub publication_states: Vec<ManifestPublicationState>,
    /// Workflow-bundle manifests, one or more per M5 launch wedge.
    #[serde(default)]
    pub manifests: Vec<WorkflowBundleManifest>,
    /// Summary counts.
    pub summary: M5WorkflowBundleManifestsSummary,
}

impl M5WorkflowBundleManifestsPacket {
    /// Returns the manifest with the given id.
    pub fn manifest(&self, bundle_id: &str) -> Option<&WorkflowBundleManifest> {
        self.manifests.iter().find(|m| m.bundle_id == bundle_id)
    }

    /// Manifests composing the given wedge.
    pub fn manifests_for_wedge(
        &self,
        wedge: M5Wedge,
    ) -> impl Iterator<Item = &WorkflowBundleManifest> {
        self.manifests.iter().filter(move |m| m.wedge == wedge)
    }

    /// Manifests aiming at the given certification target.
    pub fn manifests_with_target(
        &self,
        target: CertificationTarget,
    ) -> impl Iterator<Item = &WorkflowBundleManifest> {
        self.manifests
            .iter()
            .filter(move |m| m.certification_target == target)
    }

    /// Whether every M5 wedge is composed by at least one manifest.
    pub fn covers_every_wedge(&self) -> bool {
        M5Wedge::ALL
            .iter()
            .all(|wedge| self.manifests_for_wedge(*wedge).next().is_some())
    }

    /// Whether every manifest is internally consistent against the gate.
    pub fn all_manifests_consistent(&self) -> bool {
        self.manifests.iter().all(|m| {
            m.guards_correct()
                && m.disclosure_consistent()
                && m.target_within_publication()
                && m.has_minimum_cohesive_experience()
                && m.provenance_complete()
                && m.components.iter().all(BundleComponent::is_consistent)
                && (!m.caveats_required() || m.caveats.iter().any(|c| !c.trim().is_empty()))
        })
    }

    /// Recomputes the summary from the manifests.
    pub fn computed_summary(&self) -> M5WorkflowBundleManifestsSummary {
        let count_target = |target: CertificationTarget| self.manifests_with_target(target).count();
        let count_state = |state: ManifestPublicationState| {
            self.manifests
                .iter()
                .filter(|m| m.publication_state == state)
                .count()
        };
        let components = || self.manifests.iter().flat_map(|m| m.components.iter());
        M5WorkflowBundleManifestsSummary {
            total_manifests: self.manifests.len(),
            manifest_count: self.manifests.len(),
            certified_target_manifests: count_target(CertificationTarget::Certified),
            managed_approved_target_manifests: count_target(CertificationTarget::ManagedApproved),
            community_reviewed_target_manifests: count_target(
                CertificationTarget::CommunityReviewed,
            ),
            imported_pending_review_target_manifests: count_target(
                CertificationTarget::ImportedPendingReview,
            ),
            local_draft_target_manifests: count_target(CertificationTarget::LocalDraft),
            published_manifests: count_state(ManifestPublicationState::Published),
            draft_manifests: count_state(ManifestPublicationState::Draft),
            deprecated_manifests: count_state(ManifestPublicationState::Deprecated),
            manifests_with_non_stable_dependencies: self
                .manifests
                .iter()
                .filter(|m| m.has_non_stable_components())
                .count(),
            manifests_disclosing_non_stable: self
                .manifests
                .iter()
                .filter(|m| m.discloses_non_stable_dependencies)
                .count(),
            total_components: components().count(),
            extension_components: components()
                .filter(|c| c.component_kind == BundleComponentKind::Extension)
                .count(),
            recipe_components: components()
                .filter(|c| c.component_kind.is_recipe())
                .count(),
            docs_or_tour_components: components()
                .filter(|c| {
                    matches!(
                        c.component_kind,
                        BundleComponentKind::DocsPack | BundleComponentKind::TourPack
                    )
                })
                .count(),
            template_or_scaffold_components: components()
                .filter(|c| {
                    matches!(
                        c.component_kind,
                        BundleComponentKind::TemplateRef | BundleComponentKind::ScaffoldRef
                    )
                })
                .count(),
            migration_mapping_components: components()
                .filter(|c| c.component_kind == BundleComponentKind::MigrationMapping)
                .count(),
            non_stable_components: components()
                .filter(|c| c.lifecycle_stage.is_non_stable())
                .count(),
            review_required_components: components().filter(|c| c.requires_review).count(),
        }
    }

    /// Projects a redaction-safe export view of the packet.
    pub fn export_projection(&self) -> M5WorkflowBundleManifestsExportProjection {
        let manifests = self
            .manifests
            .iter()
            .map(|m| M5WorkflowBundleManifestsExportRow {
                bundle_id: m.bundle_id.clone(),
                bundle_version: m.bundle_version.clone(),
                wedge: m.wedge,
                certification_target: m.certification_target,
                publication_state: m.publication_state,
                component_kinds: m.components.iter().map(|c| c.component_kind).collect(),
                dependency_markers: m.dependency_markers.clone(),
                discloses_non_stable_dependencies: m.discloses_non_stable_dependencies,
                presents_as_certified: m.presents_as_certified(),
                manifest_provenance_ref: m.manifest_provenance_ref.clone(),
            })
            .collect();
        let summary = self.computed_summary();
        M5WorkflowBundleManifestsExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            manifests,
            all_manifests_consistent: self.all_manifests_consistent(),
            manifests_with_non_stable_dependencies: summary.manifests_with_non_stable_dependencies,
            certified_target_manifests: summary.certified_target_manifests,
        }
    }

    /// Validates the packet against its honesty contract, returning every violation.
    pub fn validate(&self) -> Vec<M5WorkflowBundleManifestsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        let mut seen_ids = BTreeSet::new();
        for manifest in &self.manifests {
            if !seen_ids.insert(manifest.bundle_id.clone()) {
                violations.push(M5WorkflowBundleManifestsViolation::DuplicateBundleId {
                    bundle_id: manifest.bundle_id.clone(),
                });
            }
            self.validate_manifest(manifest, &mut violations);
        }
        for wedge in M5Wedge::ALL {
            if self.manifests_for_wedge(wedge).next().is_none() {
                violations.push(M5WorkflowBundleManifestsViolation::MissingWedgeCoverage { wedge });
            }
        }
        if self.summary != self.computed_summary() {
            violations.push(M5WorkflowBundleManifestsViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5WorkflowBundleManifestsViolation>) {
        if self.schema_version != M5_WORKFLOW_BUNDLE_MANIFESTS_SCHEMA_VERSION {
            violations.push(M5WorkflowBundleManifestsViolation::SchemaVersionMismatch {
                found: self.schema_version,
            });
        }
        if self.record_kind != M5_WORKFLOW_BUNDLE_MANIFESTS_RECORD_KIND {
            violations.push(M5WorkflowBundleManifestsViolation::RecordKindMismatch {
                found: self.record_kind.clone(),
            });
        }
        let vocab_ok = self.wedges == M5Wedge::ALL
            && self.component_kinds == BundleComponentKind::ALL
            && self.lifecycle_stages == LifecycleStage::ALL
            && self.certification_targets == CertificationTarget::ALL
            && self.publication_states == ManifestPublicationState::ALL;
        if !vocab_ok {
            violations.push(M5WorkflowBundleManifestsViolation::VocabularyMismatch);
        }
        if self.manifests.is_empty() {
            violations.push(M5WorkflowBundleManifestsViolation::NoManifests);
        }
    }

    fn validate_manifest(
        &self,
        manifest: &WorkflowBundleManifest,
        violations: &mut Vec<M5WorkflowBundleManifestsViolation>,
    ) {
        let id = manifest.bundle_id.clone();
        if id.trim().is_empty() {
            violations.push(M5WorkflowBundleManifestsViolation::EmptyBundleId);
        }
        if manifest.bundle_version.trim().is_empty() {
            violations.push(M5WorkflowBundleManifestsViolation::EmptyBundleVersion {
                bundle_id: id.clone(),
            });
        }
        if !manifest.guards_correct() {
            violations.push(M5WorkflowBundleManifestsViolation::OpaqueOrNonDiffable {
                bundle_id: id.clone(),
            });
        }
        if !manifest.disclosure_consistent() {
            violations.push(
                M5WorkflowBundleManifestsViolation::UndisclosedNonStableDependency {
                    bundle_id: id.clone(),
                },
            );
        }
        if !manifest.target_within_publication() {
            violations.push(
                M5WorkflowBundleManifestsViolation::TargetExceedsPublication {
                    bundle_id: id.clone(),
                    target: manifest.certification_target,
                    state: manifest.publication_state,
                },
            );
        }
        if !manifest.has_minimum_cohesive_experience() {
            violations.push(M5WorkflowBundleManifestsViolation::NotCohesive {
                bundle_id: id.clone(),
            });
        }
        if !manifest.provenance_complete() {
            violations.push(M5WorkflowBundleManifestsViolation::MissingProvenance {
                bundle_id: id.clone(),
            });
        }
        for component in &manifest.components {
            if !component.is_consistent() {
                violations.push(M5WorkflowBundleManifestsViolation::InconsistentComponent {
                    bundle_id: id.clone(),
                    component_id: component.component_id.clone(),
                });
            }
        }
        if manifest.caveats_required() && manifest.caveats.iter().all(|c| c.trim().is_empty()) {
            violations.push(M5WorkflowBundleManifestsViolation::MissingCaveat {
                bundle_id: id.clone(),
            });
        }
        let surface_refs = [
            &manifest.owner,
            &manifest.persona,
            &manifest.stack,
            &manifest.archetype,
            &manifest.start_center_ref,
            &manifest.migration_center_ref,
            &manifest.bundle_detail_ref,
            &manifest.diagnostics_ref,
            &manifest.support_export_ref,
            &manifest.help_surface_ref,
            &manifest.release_evidence_ref,
            &manifest.note,
        ];
        if surface_refs.iter().any(|r| r.trim().is_empty()) {
            violations
                .push(M5WorkflowBundleManifestsViolation::MissingSurfaceRef { bundle_id: id });
        }
    }
}

/// A single way the packet can fail its honesty contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5WorkflowBundleManifestsViolation {
    /// The schema version does not match the supported version.
    SchemaVersionMismatch {
        /// The version found in the packet.
        found: u32,
    },
    /// The record kind does not match the canonical tag.
    RecordKindMismatch {
        /// The record kind found in the packet.
        found: String,
    },
    /// A closed vocabulary array does not match its canonical `ALL`.
    VocabularyMismatch,
    /// The packet carries no manifests.
    NoManifests,
    /// An M5 wedge has no workflow-bundle manifest.
    MissingWedgeCoverage {
        /// The uncovered wedge.
        wedge: M5Wedge,
    },
    /// Two manifests share a bundle id.
    DuplicateBundleId {
        /// The duplicated id.
        bundle_id: String,
    },
    /// A manifest id is empty.
    EmptyBundleId,
    /// A manifest version is empty.
    EmptyBundleVersion {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A manifest is non-diffable, non-mirrorable, non-export-safe, or carries opaque binary state.
    OpaqueOrNonDiffable {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A manifest depends on a non-stable capability it does not disclose.
    UndisclosedNonStableDependency {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A manifest's certification target exceeds what its publication state permits.
    TargetExceedsPublication {
        /// The offending bundle id.
        bundle_id: String,
        /// The recorded certification target.
        target: CertificationTarget,
        /// The recorded publication state.
        state: ManifestPublicationState,
    },
    /// A manifest does not capture a minimum cohesive experience.
    NotCohesive {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A manifest lacks complete manifest, certification, or migration provenance.
    MissingProvenance {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A component is internally inconsistent or opaque.
    InconsistentComponent {
        /// The offending bundle id.
        bundle_id: String,
        /// The offending component id.
        component_id: String,
    },
    /// A manifest that needs a caveat carries none.
    MissingCaveat {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// A manifest is missing a required surface or owner ref.
    MissingSurfaceRef {
        /// The offending bundle id.
        bundle_id: String,
    },
    /// The recorded summary diverges from the recomputed summary.
    SummaryMismatch,
}

impl fmt::Display for M5WorkflowBundleManifestsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersionMismatch { found } => {
                write!(f, "schema_version mismatch: found {found}")
            }
            Self::RecordKindMismatch { found } => write!(f, "record_kind mismatch: found {found}"),
            Self::VocabularyMismatch => {
                write!(
                    f,
                    "a closed vocabulary array diverges from its canonical set"
                )
            }
            Self::NoManifests => write!(f, "packet carries no workflow-bundle manifests"),
            Self::MissingWedgeCoverage { wedge } => {
                write!(
                    f,
                    "wedge {} has no workflow-bundle manifest",
                    wedge.as_str()
                )
            }
            Self::DuplicateBundleId { bundle_id } => {
                write!(f, "duplicate bundle id: {bundle_id}")
            }
            Self::EmptyBundleId => write!(f, "a manifest has an empty id"),
            Self::EmptyBundleVersion { bundle_id } => {
                write!(f, "manifest {bundle_id} has an empty version")
            }
            Self::OpaqueOrNonDiffable { bundle_id } => write!(
                f,
                "manifest {bundle_id} is non-diffable, non-mirrorable, non-export-safe, or opaque"
            ),
            Self::UndisclosedNonStableDependency { bundle_id } => write!(
                f,
                "manifest {bundle_id} depends on a non-stable capability it does not disclose"
            ),
            Self::TargetExceedsPublication {
                bundle_id,
                target,
                state,
            } => write!(
                f,
                "manifest {bundle_id} certification target {} exceeds publication state {}",
                target.as_str(),
                state.as_str()
            ),
            Self::NotCohesive { bundle_id } => write!(
                f,
                "manifest {bundle_id} does not capture a minimum cohesive experience"
            ),
            Self::MissingProvenance { bundle_id } => write!(
                f,
                "manifest {bundle_id} lacks manifest, certification, or migration provenance"
            ),
            Self::InconsistentComponent {
                bundle_id,
                component_id,
            } => write!(
                f,
                "manifest {bundle_id} component {component_id} is inconsistent or opaque"
            ),
            Self::MissingCaveat { bundle_id } => {
                write!(f, "manifest {bundle_id} needs a caveat but carries none")
            }
            Self::MissingSurfaceRef { bundle_id } => {
                write!(
                    f,
                    "manifest {bundle_id} is missing a required surface or owner ref"
                )
            }
            Self::SummaryMismatch => write!(f, "summary diverges from the recomputed summary"),
        }
    }
}

impl Error for M5WorkflowBundleManifestsViolation {}

/// Loads the embedded canonical M5 workflow-bundle-manifests packet.
///
/// # Errors
///
/// Returns a deserialization error if the embedded JSON does not parse into the typed packet.
pub fn current_m5_workflow_bundle_manifests_packet(
) -> Result<M5WorkflowBundleManifestsPacket, serde_json::Error> {
    serde_json::from_str(M5_WORKFLOW_BUNDLE_MANIFESTS_JSON)
}

#[cfg(test)]
mod tests;
