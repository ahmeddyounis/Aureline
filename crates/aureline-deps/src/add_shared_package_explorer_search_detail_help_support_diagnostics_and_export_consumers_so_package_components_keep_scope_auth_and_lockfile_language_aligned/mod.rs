//! Shared package-explorer / dependency-search-detail / help / support /
//! diagnostics / export consumers that keep the eight reusable M5
//! package-management components at scope, provenance, auth, and lockfile
//! parity across every claimed M5 profile.
//!
//! This module is the closing consumer-adoption lane for the package-management
//! components frozen in
//! [`crate::freeze_the_m5_package_management_component_matrix`] and implemented by
//! the package-explorer-row, manifest-scope-switcher / registry-or-mirror-row,
//! install-review-sheet / lockfile-impact-card, and script-risk-notice /
//! grouped-update-planner / rollback-checkpoint-strip lanes. It binds each shared
//! component to the package explorer, dependency search/detail pane, Help surface,
//! support packet, diagnostics view, and exported view that render it, and proves —
//! by fixtures, not screenshots — that the same package object presents the same
//! manifest-scope, registry-source/auth, script/lockfile-risk, and
//! rollback/checkpoint language wherever it appears.
//!
//! The core honesty axes are two. First, parity: for a given package object, every
//! consumer surface must present identical parity facet values — the same
//! target-manifest and direct/transitive scope label, the same registry-source and
//! auth posture, the same script/native-build and lockfile-churn risk language, and
//! the same grouped-update reason and rollback/checkpoint recovery language. A
//! surface may narrow how much it shows when the registry answer degrades to a
//! manifest range, a mirror, an offline snapshot, an unsatisfied auth state, or an
//! unknown/stale state, but it may never reword the underlying language per surface,
//! flatten manifest scope or auth behind generic manage-package chrome, hide broad
//! lockfile regeneration behind one-click update language, or drop mirror/offline
//! continuity and rollback/checkpoint truth. Second, disclosure: when a surface
//! narrows, it must do so through an explicit narrow banner that names the reason,
//! the preserved facets, and the next action — the mirror/offline continuity note
//! and the registry-auth note stay explicit rather than collapsing the package
//! object out of view.
//!
//! Component reuse is proven rather than inferred: every one of the eight shared
//! components must be adopted by at least two distinct consumers, and Help, support,
//! and exported-view consumers must point at the canonical component contracts by
//! id. The registry/resolution degradation vocabulary is reused directly from the
//! frozen matrix ([`M5PackageComponentDegradationState`]) and the component identity
//! from [`M5PackageComponent`], so degradation narrowing and component identity read
//! the same everywhere.
//!
//! The packet references upstream component contracts by id rather than embedding
//! their content. Raw manifests, raw lockfile bodies, registry credentials, private
//! registry URLs, and live registry responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-package-management-component-consumer.schema.json`](../../../../schemas/ui/m5-package-management-component-consumer.schema.json).
//! The protected fixture directory is
//! [`fixtures/ui/m5-package-management-component-consumers/`](../../../../fixtures/ui/m5-package-management-component-consumers/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_management_component_matrix::{
    M5PackageComponent, M5PackageComponentDegradationState, M5PackageComponentDowngradeTrigger,
};

/// Stable record-kind tag carried by [`PackageComponentConsumerPacket`].
pub const PACKAGE_COMPONENT_CONSUMER_RECORD_KIND: &str = "package_component_consumer_parity_truth";

/// Schema version for package-management-component consumer parity records.
pub const PACKAGE_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PACKAGE_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-package-management-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const PACKAGE_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/deps/m5/add_shared_package_explorer_search_detail_help_support_diagnostics_and_export_consumers.md";

/// Repo-relative path of the frozen component matrix these consumers adopt.
pub const PACKAGE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-package-management-component-matrix.schema.json";

/// Repo-relative path of the package-explorer-row contract.
pub const PACKAGE_COMPONENT_CONSUMER_EXPLORER_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-package-explorer-row.schema.json";

/// Repo-relative path of the manifest-scope-switcher / registry-or-mirror-row contract.
pub const PACKAGE_COMPONENT_CONSUMER_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF: &str =
    "schemas/ui/m5-manifest-scope-registry-controls.schema.json";

/// Repo-relative path of the install-review-sheet / lockfile-impact-card contract.
pub const PACKAGE_COMPONENT_CONSUMER_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF: &str =
    "schemas/ui/m5-install-review-lockfile-controls.schema.json";

/// Repo-relative path of the script-risk / grouped-update / rollback-checkpoint contract.
pub const PACKAGE_COMPONENT_CONSUMER_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF: &str =
    "schemas/ui/m5-script-risk-grouped-update-rollback-controls.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const PACKAGE_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-package-management-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const PACKAGE_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-package-management-component-consumers-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PACKAGE_COMPONENT_CONSUMER_SUMMARY_REF: &str =
    "artifacts/release/m5-package-management-component-consumers-proof/summary.md";

/// Canonical component contract that a consumer must point at for a given component.
///
/// Each of the eight shared components resolves to the checked-in schema of the
/// implement lane that produced it: the package-explorer-row controls, the
/// manifest-scope / registry-or-mirror controls, the install-review / lockfile-impact
/// controls, and the script-risk / grouped-update / rollback-checkpoint controls.
pub const fn component_canonical_schema_ref(component: M5PackageComponent) -> &'static str {
    match component {
        M5PackageComponent::PackageExplorerRow => {
            PACKAGE_COMPONENT_CONSUMER_EXPLORER_ROW_CONTRACT_REF
        }
        M5PackageComponent::ManifestScopeSwitcher | M5PackageComponent::RegistryOrMirrorRow => {
            PACKAGE_COMPONENT_CONSUMER_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF
        }
        M5PackageComponent::InstallReviewSheet | M5PackageComponent::LockfileImpactCard => {
            PACKAGE_COMPONENT_CONSUMER_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF
        }
        M5PackageComponent::ScriptRiskNotice
        | M5PackageComponent::GroupedUpdatePlanner
        | M5PackageComponent::RollbackCheckpointStrip => {
            PACKAGE_COMPONENT_CONSUMER_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF
        }
    }
}

/// Consumer surface that must reuse the shared package-management components at parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentConsumer {
    /// Package explorer / browse surface.
    PackageExplorer,
    /// Dependency search / detail pane.
    DependencySearchDetail,
    /// Help / About surface.
    HelpSurface,
    /// Support packet.
    SupportPacket,
    /// Diagnostics / doctor view.
    Diagnostics,
    /// Exported package-operation evidence view.
    ExportedView,
}

impl PackageComponentConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PackageExplorer,
        Self::DependencySearchDetail,
        Self::HelpSurface,
        Self::SupportPacket,
        Self::Diagnostics,
        Self::ExportedView,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageExplorer => "package_explorer",
            Self::DependencySearchDetail => "dependency_search_detail",
            Self::HelpSurface => "help_surface",
            Self::SupportPacket => "support_packet",
            Self::Diagnostics => "diagnostics",
            Self::ExportedView => "exported_view",
        }
    }

    /// Whether this consumer is a Help, support, or exported-view surface that must
    /// point at the canonical component contracts by id.
    pub const fn is_help_support_or_export(self) -> bool {
        matches!(
            self,
            Self::HelpSurface | Self::SupportPacket | Self::ExportedView
        )
    }
}

/// A parity facet whose value must stay identical across surfaces for one object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentParityFacet {
    /// The target-manifest and direct/transitive scope label.
    ManifestScopeLabel,
    /// The registry-source and auth-posture label.
    RegistrySourceAuthLabel,
    /// The script/native-build and lockfile-churn risk language.
    RiskLanguage,
    /// The grouped-update reason and rollback/checkpoint recovery language.
    RecoveryLanguage,
}

impl PackageComponentParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ManifestScopeLabel,
        Self::RegistrySourceAuthLabel,
        Self::RiskLanguage,
        Self::RecoveryLanguage,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifestScopeLabel => "manifest_scope_label",
            Self::RegistrySourceAuthLabel => "registry_source_auth_label",
            Self::RiskLanguage => "risk_language",
            Self::RecoveryLanguage => "recovery_language",
        }
    }
}

/// How much of a shared component a consumer renders.
///
/// Narrowing changes how much is shown, never the underlying parity language: a
/// narrowed surface still carries the same manifest-scope, registry-source/auth,
/// risk, and recovery language, and discloses the narrowing through an explicit
/// banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentRenderMode {
    /// Full parity; the resolution is an exact, authoritative pin.
    FullParity,
    /// Only a manifest range governs; the exact resolution is not pinned here.
    ManifestRangeNarrowed,
    /// The answer is mirror-backed or an offline snapshot; continuity stays explicit.
    MirrorOrOfflineNarrowed,
    /// Registry access requires authentication that is not satisfied.
    AuthRequiredNarrowed,
    /// The package state is unknown or stale.
    UnknownOrStaleNarrowed,
}

impl PackageComponentRenderMode {
    /// Every render mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::ManifestRangeNarrowed,
        Self::MirrorOrOfflineNarrowed,
        Self::AuthRequiredNarrowed,
        Self::UnknownOrStaleNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::ManifestRangeNarrowed => "manifest_range_narrowed",
            Self::MirrorOrOfflineNarrowed => "mirror_or_offline_narrowed",
            Self::AuthRequiredNarrowed => "auth_required_narrowed",
            Self::UnknownOrStaleNarrowed => "unknown_or_stale_narrowed",
        }
    }

    /// Whether this mode narrows below full parity.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }
}

/// Why a surface narrowed its rendering of a shared component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentNarrowReason {
    /// Only a manifest range governs; the exact resolution is not pinned here.
    ExactResolutionUnavailableRangeOnly,
    /// The answer came from a mirror or offline snapshot; continuity stays explicit.
    MirrorOrOfflineContinuity,
    /// Registry access requires authentication that is not satisfied.
    RegistryAuthRequired,
    /// The package state could not be established or is stale.
    PackageStateUnknownOrStale,
}

impl PackageComponentNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactResolutionUnavailableRangeOnly => "exact_resolution_unavailable_range_only",
            Self::MirrorOrOfflineContinuity => "mirror_or_offline_continuity",
            Self::RegistryAuthRequired => "registry_auth_required",
            Self::PackageStateUnknownOrStale => "package_state_unknown_or_stale",
        }
    }
}

/// The next action a narrow banner offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentNarrowNextAction {
    /// Review the governing manifest range.
    ReviewManifestRange,
    /// Review the mirror / offline continuity posture.
    ReviewMirrorOfflineContinuity,
    /// Complete registry authentication.
    CompleteRegistryAuth,
    /// Review the package-state freshness.
    ReviewPackageStateFreshness,
}

impl PackageComponentNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewManifestRange => "review_manifest_range",
            Self::ReviewMirrorOfflineContinuity => "review_mirror_offline_continuity",
            Self::CompleteRegistryAuth => "complete_registry_auth",
            Self::ReviewPackageStateFreshness => "review_package_state_freshness",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentParityState {
    /// All parity facets are preserved and shown in full.
    FacetsPreserved,
    /// All parity facets are preserved, and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl PackageComponentParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// The parity facet values a shared component presents for one package object.
///
/// These four values must be identical across every consumer surface that shows the
/// same package object. A surface may narrow how much it renders, but it may never
/// reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentParityFacetValues {
    /// Target-manifest and direct/transitive scope label (never reworded per surface).
    pub manifest_scope_label: String,
    /// Registry-source and auth-posture label (identical across surfaces).
    pub registry_source_auth_label: String,
    /// Script/native-build and lockfile-churn risk language (identical across surfaces).
    pub risk_language: String,
    /// Grouped-update reason and rollback/checkpoint recovery language (identical across surfaces).
    pub recovery_language: String,
}

impl PackageComponentParityFacetValues {
    /// Whether every parity facet value is present.
    pub fn all_present(&self) -> bool {
        !self.manifest_scope_label.trim().is_empty()
            && !self.registry_source_auth_label.trim().is_empty()
            && !self.risk_language.trim().is_empty()
            && !self.recovery_language.trim().is_empty()
    }
}

/// The explicit banner a narrowed surface shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentNarrowBanner {
    /// Why the surface narrowed.
    pub reason: PackageComponentNarrowReason,
    /// Note naming the preserved parity facets (never omitted).
    pub preserved_facets_note: String,
    /// The next action offered.
    pub next_action: PackageComponentNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its registry state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageComponentRenderDisclosure {
    /// The render mode the registry state requires.
    pub expected_mode: PackageComponentRenderMode,
    /// The narrow reason the render mode requires, if any.
    pub narrow_reason: Option<PackageComponentNarrowReason>,
    /// Whether the binding must carry an explicit narrow banner.
    pub needs_narrow_banner: bool,
    /// Whether the binding must carry an explicit mirror/offline continuity note.
    pub needs_continuity_note: bool,
    /// Whether the binding must carry an explicit registry-auth note.
    pub needs_auth_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its registry state.
///
/// An exact, authoritative pin renders at full parity. A manifest-range-only answer
/// narrows to a range disclosure without pretending to be an exact pin. A
/// mirror-backed or offline-snapshot answer narrows through an explicit mirror/offline
/// continuity note so an empty or stale answer never reads as a clean upstream
/// resolution. An unsatisfied-auth answer narrows through an explicit registry-auth
/// note rather than a generic no-results message. An unknown or stale state narrows
/// through a freshness disclosure. In every case the package object stays visible with
/// its manifest scope, registry source, and recovery posture intact.
pub fn resolve_package_component_render_disclosure(
    registry_state: M5PackageComponentDegradationState,
) -> PackageComponentRenderDisclosure {
    let (expected_mode, narrow_reason) = match registry_state {
        M5PackageComponentDegradationState::ResolvedExact => {
            (PackageComponentRenderMode::FullParity, None)
        }
        M5PackageComponentDegradationState::ManifestRangeOnly => (
            PackageComponentRenderMode::ManifestRangeNarrowed,
            Some(PackageComponentNarrowReason::ExactResolutionUnavailableRangeOnly),
        ),
        M5PackageComponentDegradationState::MirrorBacked
        | M5PackageComponentDegradationState::OfflineSnapshotOnly => (
            PackageComponentRenderMode::MirrorOrOfflineNarrowed,
            Some(PackageComponentNarrowReason::MirrorOrOfflineContinuity),
        ),
        M5PackageComponentDegradationState::AuthRequiredUnsatisfied => (
            PackageComponentRenderMode::AuthRequiredNarrowed,
            Some(PackageComponentNarrowReason::RegistryAuthRequired),
        ),
        M5PackageComponentDegradationState::UnknownOrStale => (
            PackageComponentRenderMode::UnknownOrStaleNarrowed,
            Some(PackageComponentNarrowReason::PackageStateUnknownOrStale),
        ),
    };

    // Mirror/offline continuity must stay explicit whenever the answer is not an
    // upstream-authoritative pin (spec guardrail).
    let needs_continuity_note = matches!(
        registry_state,
        M5PackageComponentDegradationState::MirrorBacked
            | M5PackageComponentDegradationState::OfflineSnapshotOnly
    );
    let needs_auth_note = matches!(
        registry_state,
        M5PackageComponentDegradationState::AuthRequiredUnsatisfied
    );

    PackageComponentRenderDisclosure {
        expected_mode,
        narrow_reason,
        needs_narrow_banner: expected_mode.is_narrowed(),
        needs_continuity_note,
        needs_auth_note,
    }
}

/// The parity state a render mode requires.
pub const fn parity_state_for_mode(
    mode: PackageComponentRenderMode,
) -> PackageComponentParityState {
    match mode {
        PackageComponentRenderMode::FullParity => PackageComponentParityState::FacetsPreserved,
        PackageComponentRenderMode::ManifestRangeNarrowed
        | PackageComponentRenderMode::MirrorOrOfflineNarrowed
        | PackageComponentRenderMode::AuthRequiredNarrowed
        | PackageComponentRenderMode::UnknownOrStaleNarrowed => {
            PackageComponentParityState::FacetsDisclosedNarrowed
        }
    }
}

/// One consumer binding: a shared component rendered on one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable package-object id (shared across surfaces that show the same object).
    pub package_object_id: String,
    /// Human-readable package-object identity.
    pub package_object_label: String,
    /// Which shared component this binding renders.
    pub component: M5PackageComponent,
    /// Which consumer surface renders it.
    pub consumer: PackageComponentConsumer,
    /// Registry/resolution state, reused from the frozen component matrix.
    pub registry_state: M5PackageComponentDegradationState,
    /// How much of the component this surface renders.
    pub render_mode: PackageComponentRenderMode,
    /// The parity facet values presented (identical across surfaces for one object).
    pub parity_facets: PackageComponentParityFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: PackageComponentParityState,
    /// The explicit narrow banner; required and complete when the binding narrows.
    pub narrow_banner: Option<PackageComponentNarrowBanner>,
    /// Mirror/offline continuity note; required and non-empty when the disclosure demands it.
    pub continuity_note: String,
    /// Registry-auth note; required and non-empty when the disclosure demands it.
    pub auth_note: String,
    /// Guardrail: this surface uses generic manage-package language that hides manifest scope.
    pub uses_generic_manage_package_language_hiding_scope: bool,
    /// Guardrail: this surface uses one-click update language that hides script/lockfile risk.
    pub uses_one_click_update_language_hiding_risk: bool,
    /// Guardrail: this surface conceals the registry auth posture behind a generic prompt.
    pub conceals_registry_auth_posture: bool,
    /// Guardrail: this surface hides broad lockfile regeneration behind a smaller-looking change.
    pub hides_broad_lockfile_regeneration: bool,
    /// Guardrail: this surface drops mirror/offline continuity or rollback/checkpoint truth.
    pub drops_mirror_offline_or_rollback_truth: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl PackageComponentConsumerBinding {
    /// Disclosures this binding must carry, derived from its registry state.
    pub fn disclosure(&self) -> PackageComponentRenderDisclosure {
        resolve_package_component_render_disclosure(self.registry_state)
    }

    /// Whether this binding renders below full parity.
    pub fn is_narrowed(&self) -> bool {
        self.render_mode.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub fn guardrails_hold(&self) -> bool {
        !self.uses_generic_manage_package_language_hiding_scope
            && !self.uses_one_click_update_language_hiding_risk
            && !self.conceals_registry_auth_posture
            && !self.hides_broad_lockfile_regeneration
            && !self.drops_mirror_offline_or_rollback_truth
    }

    /// Whether this binding points at the canonical component schema and matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = component_canonical_schema_ref(self.component);
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self.source_contract_refs.iter().any(|reference| {
                reference == PACKAGE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF
            })
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentConsumerTrustReview {
    /// Component reuse is proven by fixtures rather than inferred from screenshots.
    pub component_reuse_proven_by_fixtures: bool,
    /// The same package object presents the same language across surfaces.
    pub same_object_same_language_across_surfaces: bool,
    /// Target manifest and direct/transitive scope are identical across surfaces.
    pub manifest_scope_identical_across_surfaces: bool,
    /// Registry source and auth posture are identical across surfaces.
    pub registry_source_and_auth_identical_across_surfaces: bool,
    /// Script / native-build risk is identical across surfaces.
    pub script_native_build_risk_identical_across_surfaces: bool,
    /// Lockfile churn is never understated across surfaces.
    pub lockfile_churn_never_understated_across_surfaces: bool,
    /// Rollback / checkpoint identity stays explicit.
    pub rollback_checkpoint_identity_kept_explicit: bool,
    /// Mirror / offline continuity stays explicit rather than reading as clean.
    pub mirror_offline_continuity_kept_explicit: bool,
    /// Generic one-click / manage-package language never conceals scope, auth, or risk.
    pub generic_one_click_language_never_conceals_scope_or_risk: bool,
    /// Help, support, and export consumers point at the canonical contracts.
    pub help_support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl PackageComponentConsumerTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.component_reuse_proven_by_fixtures
            && self.same_object_same_language_across_surfaces
            && self.manifest_scope_identical_across_surfaces
            && self.registry_source_and_auth_identical_across_surfaces
            && self.script_native_build_risk_identical_across_surfaces
            && self.lockfile_churn_never_understated_across_surfaces
            && self.rollback_checkpoint_identity_kept_explicit
            && self.mirror_offline_continuity_kept_explicit
            && self.generic_one_click_language_never_conceals_scope_or_risk
            && self.help_support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentConsumerProjection {
    /// The package explorer reuses the shared components.
    pub package_explorer_reuses_shared_components: bool,
    /// The dependency search/detail pane reuses the shared components.
    pub dependency_search_detail_reuses_shared_components: bool,
    /// The Help surface reuses the shared components.
    pub help_surface_reuses_shared_components: bool,
    /// The support packet reuses the shared components.
    pub support_packet_reuses_shared_components: bool,
    /// The diagnostics view reuses the shared components.
    pub diagnostics_reuses_shared_components: bool,
    /// The exported view reuses the shared components.
    pub exported_view_reuses_shared_components: bool,
    /// Every component is adopted by two or more consumers.
    pub every_component_adopted_by_two_or_more_consumers: bool,
    /// Parity facets are identical for the same package object.
    pub parity_facets_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export preserves manifest scope, registry auth, and lockfile posture.
    pub export_preserves_scope_auth_and_lockfile_posture: bool,
}

impl PackageComponentConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.package_explorer_reuses_shared_components
            && self.dependency_search_detail_reuses_shared_components
            && self.help_surface_reuses_shared_components
            && self.support_packet_reuses_shared_components
            && self.diagnostics_reuses_shared_components
            && self.exported_view_reuses_shared_components
            && self.every_component_adopted_by_two_or_more_consumers
            && self.parity_facets_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_preserves_scope_auth_and_lockfile_posture
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`PackageComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<PackageComponentConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<PackageComponentConsumer>,
    /// Trust review block.
    pub trust_review: PackageComponentConsumerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PackageComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PackageComponentConsumerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe package-management-component consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentConsumerPacket {
    /// Record kind; must equal [`PACKAGE_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PACKAGE_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<PackageComponentConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<PackageComponentConsumer>,
    /// Trust review block.
    pub trust_review: PackageComponentConsumerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PackageComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PackageComponentConsumerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PackageComponentConsumerPacket {
    /// Builds a package-management-component consumer packet from stable-lane input.
    pub fn new(input: PackageComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: PACKAGE_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: PACKAGE_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the package-management-component consumer parity invariants.
    pub fn validate(&self) -> Vec<PackageComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PACKAGE_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(PackageComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != PACKAGE_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(PackageComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PackageComponentConsumerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(PackageComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(PackageComponentConsumerViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(PackageComponentConsumerViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(PackageComponentConsumerViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(PackageComponentConsumerViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("package-management-component consumer packet serializes"),
        ) {
            violations.push(PackageComponentConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("package-management-component consumer packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Shared Package-Management Component Consumers: Scope, Auth, and Lockfile Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: component `{}` on `{}`, mode `{}`\n",
                binding.package_object_label,
                binding.binding_id,
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.render_mode.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in package-consumer export.
#[derive(Debug)]
pub enum PackageComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PackageComponentConsumerViolation>),
}

impl fmt::Display for PackageComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "package-management-component consumer export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "package-management-component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PackageComponentConsumerArtifactError {}

/// Validation failures emitted by [`PackageComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageComponentConsumerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's parity facet values are incomplete.
    ParityFacetIncomplete,
    /// A binding's render mode does not match its registry state.
    RenderModeMismatch,
    /// A binding's parity state does not match its render mode.
    ParityStateMismatch,
    /// Two surfaces show the same package object with different parity language.
    ParityDriftAcrossSurfaces,
    /// A shared component is not adopted by at least two distinct consumers.
    PackageComponentReuseUnproven,
    /// A Help, support, or export binding does not point at the canonical contracts.
    HelpSupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow banner.
    NarrowBannerMissing,
    /// A narrow banner's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow banner is missing its preserved-facets note.
    NarrowBannerPreservedFacetsMissing,
    /// A narrow banner is missing its next-action copy.
    NarrowNextActionMissing,
    /// A binding that must keep a mirror/offline continuity note is missing it.
    ContinuityNoteMissing,
    /// A binding that needs an explicit registry-auth note is missing it.
    AuthNoteMissing,
    /// A binding uses generic manage-package language that hides manifest scope.
    GenericManagePackageLanguageHidesScope,
    /// A binding uses one-click update language that hides script/lockfile risk.
    OneClickUpdateLanguageHidesRisk,
    /// A binding conceals the registry auth posture behind a generic prompt.
    RegistryAuthPostureConcealed,
    /// A binding hides broad lockfile regeneration behind a smaller-looking change.
    BroadLockfileRegenerationHidden,
    /// A binding drops mirror/offline continuity or rollback/checkpoint truth.
    MirrorOfflineOrRollbackTruthDropped,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared component appears among the bindings.
    ComponentCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PackageComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::ParityFacetIncomplete => "parity_facet_incomplete",
            Self::RenderModeMismatch => "render_mode_mismatch",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::ParityDriftAcrossSurfaces => "parity_drift_across_surfaces",
            Self::PackageComponentReuseUnproven => "package_component_reuse_unproven",
            Self::HelpSupportExportReferenceMissing => "help_support_export_reference_missing",
            Self::NarrowBannerMissing => "narrow_banner_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowBannerPreservedFacetsMissing => "narrow_banner_preserved_facets_missing",
            Self::NarrowNextActionMissing => "narrow_next_action_missing",
            Self::ContinuityNoteMissing => "continuity_note_missing",
            Self::AuthNoteMissing => "auth_note_missing",
            Self::GenericManagePackageLanguageHidesScope => {
                "generic_manage_package_language_hides_scope"
            }
            Self::OneClickUpdateLanguageHidesRisk => "one_click_update_language_hides_risk",
            Self::RegistryAuthPostureConcealed => "registry_auth_posture_concealed",
            Self::BroadLockfileRegenerationHidden => "broad_lockfile_regeneration_hidden",
            Self::MirrorOfflineOrRollbackTruthDropped => "mirror_offline_or_rollback_truth_dropped",
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable package-consumer export.
pub fn current_package_component_consumer_export(
) -> Result<PackageComponentConsumerPacket, PackageComponentConsumerArtifactError> {
    let packet: PackageComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-package-management-component-consumers-proof/support_export.json"
    )))
    .map_err(PackageComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PackageComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &PackageComponentConsumerPacket,
    violations: &mut Vec<PackageComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PACKAGE_COMPONENT_CONSUMER_SCHEMA_REF,
        PACKAGE_COMPONENT_CONSUMER_DOC_REF,
        PACKAGE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF,
        PACKAGE_COMPONENT_CONSUMER_EXPLORER_ROW_CONTRACT_REF,
        PACKAGE_COMPONENT_CONSUMER_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF,
        PACKAGE_COMPONENT_CONSUMER_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF,
        PACKAGE_COMPONENT_CONSUMER_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PackageComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &PackageComponentConsumerPacket,
    violations: &mut Vec<PackageComponentConsumerViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(PackageComponentConsumerViolation::ConsumerBindingsMissing);
        return;
    }

    // Parity: the parity facet values must be identical for every binding that
    // renders the same package object.
    let mut object_facets: BTreeMap<&str, &PackageComponentParityFacetValues> = BTreeMap::new();
    let mut parity_drift_reported = false;

    // Reuse: each component must be adopted by at least two distinct consumers.
    let mut component_consumers: BTreeMap<M5PackageComponent, BTreeSet<PackageComponentConsumer>> =
        BTreeMap::new();
    let mut seen_consumers: BTreeSet<PackageComponentConsumer> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5PackageComponent> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.package_object_id.trim().is_empty()
            || binding.package_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(PackageComponentConsumerViolation::BindingIncomplete);
        }
        if !binding.parity_facets.all_present() {
            violations.push(PackageComponentConsumerViolation::ParityFacetIncomplete);
        }

        let disclosure = binding.disclosure();

        if binding.render_mode != disclosure.expected_mode {
            violations.push(PackageComponentConsumerViolation::RenderModeMismatch);
        }
        if binding.parity_state != parity_state_for_mode(binding.render_mode) {
            violations.push(PackageComponentConsumerViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_banner {
            match &binding.narrow_banner {
                None => {
                    violations.push(PackageComponentConsumerViolation::NarrowBannerMissing);
                }
                Some(banner) => {
                    if Some(banner.reason) != disclosure.narrow_reason {
                        violations.push(PackageComponentConsumerViolation::NarrowReasonMismatch);
                    }
                    if banner.preserved_facets_note.trim().is_empty() {
                        violations.push(
                            PackageComponentConsumerViolation::NarrowBannerPreservedFacetsMissing,
                        );
                    }
                    if banner.next_action_label.trim().is_empty() {
                        violations.push(PackageComponentConsumerViolation::NarrowNextActionMissing);
                    }
                }
            }
        } else if binding.narrow_banner.is_some() {
            // A full-parity binding must not carry a narrow banner.
            violations.push(PackageComponentConsumerViolation::NarrowBannerMissing);
        }

        if disclosure.needs_continuity_note && binding.continuity_note.trim().is_empty() {
            violations.push(PackageComponentConsumerViolation::ContinuityNoteMissing);
        }
        if disclosure.needs_auth_note && binding.auth_note.trim().is_empty() {
            violations.push(PackageComponentConsumerViolation::AuthNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.uses_generic_manage_package_language_hiding_scope {
            violations
                .push(PackageComponentConsumerViolation::GenericManagePackageLanguageHidesScope);
        }
        if binding.uses_one_click_update_language_hiding_risk {
            violations.push(PackageComponentConsumerViolation::OneClickUpdateLanguageHidesRisk);
        }
        if binding.conceals_registry_auth_posture {
            violations.push(PackageComponentConsumerViolation::RegistryAuthPostureConcealed);
        }
        if binding.hides_broad_lockfile_regeneration {
            violations.push(PackageComponentConsumerViolation::BroadLockfileRegenerationHidden);
        }
        if binding.drops_mirror_offline_or_rollback_truth {
            violations.push(PackageComponentConsumerViolation::MirrorOfflineOrRollbackTruthDropped);
        }

        // Help / support / export consumers must point at the canonical contracts.
        if binding.consumer.is_help_support_or_export() && !binding.points_at_canonical_contracts()
        {
            violations.push(PackageComponentConsumerViolation::HelpSupportExportReferenceMissing);
        }

        // Parity drift accumulation.
        match object_facets.get(binding.package_object_id.as_str()) {
            None => {
                object_facets.insert(binding.package_object_id.as_str(), &binding.parity_facets);
            }
            Some(existing) => {
                if **existing != binding.parity_facets && !parity_drift_reported {
                    violations.push(PackageComponentConsumerViolation::ParityDriftAcrossSurfaces);
                    parity_drift_reported = true;
                }
            }
        }

        component_consumers
            .entry(binding.component)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_components.insert(binding.component);
    }

    // Coverage: every consumer and every component must appear.
    for consumer in PackageComponentConsumer::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(PackageComponentConsumerViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for component in M5PackageComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(PackageComponentConsumerViolation::ComponentCoverageMissing);
            break;
        }
    }

    // Reuse: every present component must be adopted by two or more distinct consumers.
    for consumers in component_consumers.values() {
        if consumers.len() < 2 {
            violations.push(PackageComponentConsumerViolation::PackageComponentReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("bearer ")
                || lower.contains("://user:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
