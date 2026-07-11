//! Canonical seed builders for the M5 marketplace-result-row / detail-fact-grid controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean result
//! rows and detail fact grids that describe the same artifact are built from the same fact values so
//! list and detail never present contradictory facts.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_PACKET_ID: &str =
    "m5-marketplace-result-row-detail-fact-grid-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn row(input: M5MarketplaceResultRowResolutionInput) -> M5ResolvedMarketplaceResultRow {
    resolve_marketplace_result_row(input).expect("seed marketplace result row input resolves")
}

fn grid(input: M5MarketplaceDetailFactGridResolutionInput) -> M5ResolvedMarketplaceDetailFactGrid {
    resolve_marketplace_detail_fact_grid(input).expect("seed marketplace detail fact grid resolves")
}

// -- Clean paired examples (same artifact appears as a row and a grid with identical facts) ------

/// Clean result row for a public, compatible, sandboxed artifact.
fn row_acme_public_clean() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean detail fact grid for the same public artifact — identical facts, richer detail.
fn grid_acme_public_clean() -> M5ResolvedMarketplaceDetailFactGrid {
    grid(M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:acme-linter".to_owned(),
        artifact_identity: "acme-linter".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        version_range: ">=1.2.0, <2.0.0".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::Active,
        docs_linked: true,
        changelog_linked: true,
        open_issues_linked: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        proof_fresh: true,
    })
}

/// Clean result row for a mirrored, near-budget artifact naming its activation cost.
fn row_mirror_clean() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:mirror-fmt".to_owned(),
        artifact_identity: "mirror-fmt".to_owned(),
        registry_source: M5RegistrySourceClass::MirroredRegistry,
        compatibility: M5CompatibilityState::CompatibleWithWarnings,
        host_runtime_model: M5HostRuntimeModel::InProcess,
        permission_posture: M5PermissionPostureState::Standard,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::NearBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Community,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean detail fact grid for the same mirrored artifact — identical facts.
fn grid_mirror_clean() -> M5ResolvedMarketplaceDetailFactGrid {
    grid(M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:mirror-fmt".to_owned(),
        artifact_identity: "mirror-fmt".to_owned(),
        registry_source: M5RegistrySourceClass::MirroredRegistry,
        compatibility: M5CompatibilityState::CompatibleWithWarnings,
        host_runtime_model: M5HostRuntimeModel::InProcess,
        permission_posture: M5PermissionPostureState::Standard,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::NearBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Community,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        version_range: ">=0.9.0, <1.1.0".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::Active,
        docs_linked: true,
        changelog_linked: true,
        open_issues_linked: false,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        proof_fresh: true,
    })
}

/// Clean result row for an enterprise, elevated artifact naming its permission widening.
fn row_corp_clean() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:corp-tool".to_owned(),
        artifact_identity: "corp-tool".to_owned(),
        registry_source: M5RegistrySourceClass::EnterpriseRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::RemoteHost,
        permission_posture: M5PermissionPostureState::Elevated,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::VerifiedPublisher,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Clean detail fact grid for the same enterprise artifact — identical facts.
fn grid_corp_clean() -> M5ResolvedMarketplaceDetailFactGrid {
    grid(M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:corp-tool".to_owned(),
        artifact_identity: "corp-tool".to_owned(),
        registry_source: M5RegistrySourceClass::EnterpriseRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::RemoteHost,
        permission_posture: M5PermissionPostureState::Elevated,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::VerifiedPublisher,
        publisher_continuity: M5PublisherContinuityState::VerifiedPublisher,
        publisher_change_stated: true,
        version_range: ">=3.0.0, <4.0.0".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::Active,
        docs_linked: true,
        changelog_linked: false,
        open_issues_linked: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        proof_fresh: true,
    })
}

// -- Degraded result row examples --------------------------------------------------------------

/// Degraded row: the registry source cannot be resolved.
fn row_source_unknown() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:source-unknown".to_owned(),
        artifact_identity: "pending-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::SourceUnknown,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded row: the source class is collapsed into one ambiguous origin.
fn row_source_collapsed() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:source-collapsed".to_owned(),
        artifact_identity: "collapsed-origin-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: true,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded row: an incompatible artifact reads as ready to install.
fn row_incompatible_ready() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:incompatible-ready".to_owned(),
        artifact_identity: "incompatible-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Incompatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: true,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded row: permission widening is hidden behind compact chrome.
fn row_permission_hidden() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:permission-hidden".to_owned(),
        artifact_identity: "widening-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::WidenedTransitive,
        permission_widening_stated: false,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded row: activation cost is hidden behind compact chrome.
fn row_activation_hidden() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:activation-hidden".to_owned(),
        artifact_identity: "over-budget-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::OverBudget,
        activation_cost_stated: false,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded row: a publisher transfer is hidden.
fn row_publisher_transfer_hidden() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:publisher-transfer-hidden".to_owned(),
        artifact_identity: "transferred-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Transferred,
        publisher_change_stated: false,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded row: the support / trust tier cannot be resolved.
fn row_support_unresolved() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:support-unresolved".to_owned(),
        artifact_identity: "untiered-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::TierUnknown,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

/// Degraded row: no command-backed detail entrypoint is reachable.
fn row_detail_missing() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:detail-missing".to_owned(),
        artifact_identity: "detail-less-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: false,
        proof_fresh: true,
    })
}

/// Degraded row: the artifact identity is unstated.
fn row_identity_unstated() -> M5ResolvedMarketplaceResultRow {
    row(M5MarketplaceResultRowResolutionInput {
        row_id: "result-row:no-identity".to_owned(),
        artifact_identity: "  ".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        detail_command_available: true,
        proof_fresh: true,
    })
}

// -- Degraded detail fact grid examples --------------------------------------------------------

/// Degraded grid: the source class is collapsed into one ambiguous origin.
fn grid_source_collapsed() -> M5ResolvedMarketplaceDetailFactGrid {
    grid(M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:source-collapsed".to_owned(),
        artifact_identity: "collapsed-grid-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::MirroredRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        version_range: ">=1.0.0".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::Active,
        docs_linked: true,
        changelog_linked: true,
        open_issues_linked: true,
        collapses_source_class: true,
        reads_incompatible_or_over_budget_as_ready: false,
        proof_fresh: true,
    })
}

/// Degraded grid: the richer version range is unstated.
fn grid_version_unstated() -> M5ResolvedMarketplaceDetailFactGrid {
    grid(M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:version-unstated".to_owned(),
        artifact_identity: "versionless-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        version_range: "".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::Active,
        docs_linked: true,
        changelog_linked: true,
        open_issues_linked: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        proof_fresh: true,
    })
}

/// Degraded grid: an over-budget artifact reads as ready to install.
fn grid_incompatible_ready() -> M5ResolvedMarketplaceDetailFactGrid {
    grid(M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:incompatible-ready".to_owned(),
        artifact_identity: "over-budget-grid-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::OverBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        version_range: ">=1.0.0".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::Active,
        docs_linked: true,
        changelog_linked: true,
        open_issues_linked: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: true,
        proof_fresh: true,
    })
}

/// Degraded grid: permission widening is hidden.
fn grid_permission_hidden() -> M5ResolvedMarketplaceDetailFactGrid {
    grid(M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:permission-hidden".to_owned(),
        artifact_identity: "widening-grid-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::EnterpriseRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::RemoteHost,
        permission_posture: M5PermissionPostureState::WidenedTransitive,
        permission_widening_stated: false,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        version_range: ">=2.0.0".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::Active,
        docs_linked: true,
        changelog_linked: true,
        open_issues_linked: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        proof_fresh: true,
    })
}

/// Degraded grid: the lifecycle state is unstated.
fn grid_lifecycle_unstated() -> M5ResolvedMarketplaceDetailFactGrid {
    grid(M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:lifecycle-unstated".to_owned(),
        artifact_identity: "lifecycle-less-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::MirroredRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::InProcess,
        permission_posture: M5PermissionPostureState::Standard,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Community,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        version_range: ">=0.5.0".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::LifecycleUnknown,
        docs_linked: true,
        changelog_linked: true,
        open_issues_linked: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        proof_fresh: true,
    })
}

/// Degraded grid: no docs / changelog / open-issues linkage is present.
fn grid_docs_unlinked() -> M5ResolvedMarketplaceDetailFactGrid {
    grid(M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:docs-unlinked".to_owned(),
        artifact_identity: "unlinked-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Continuous,
        publisher_change_stated: true,
        version_range: ">=1.0.0".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::Active,
        docs_linked: false,
        changelog_linked: false,
        open_issues_linked: false,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        proof_fresh: true,
    })
}

/// Degraded grid: a publisher deprecation is hidden.
fn grid_publisher_transfer_hidden() -> M5ResolvedMarketplaceDetailFactGrid {
    grid(M5MarketplaceDetailFactGridResolutionInput {
        grid_id: "detail-grid:publisher-transfer-hidden".to_owned(),
        artifact_identity: "deprecated-grid-artifact".to_owned(),
        registry_source: M5RegistrySourceClass::PublicRegistry,
        compatibility: M5CompatibilityState::Compatible,
        host_runtime_model: M5HostRuntimeModel::Sandboxed,
        permission_posture: M5PermissionPostureState::Minimal,
        permission_widening_stated: true,
        activation_budget: M5ActivationBudgetBandState::WithinBudget,
        activation_cost_stated: true,
        trust_tier: M5MarketplaceTrustTier::Reviewed,
        publisher_continuity: M5PublisherContinuityState::Deprecated,
        publisher_change_stated: false,
        version_range: ">=1.0.0".to_owned(),
        lifecycle: M5MarketplaceLifecycleState::Deprecated,
        docs_linked: true,
        changelog_linked: true,
        open_issues_linked: true,
        collapses_source_class: false,
        reads_incompatible_or_over_budget_as_ready: false,
        proof_fresh: true,
    })
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5MarketplaceResultDetailConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5MarketplaceInstallDowngradeTrigger>,
    marketplace_result_row_examples: Vec<M5ResolvedMarketplaceResultRow>,
    marketplace_detail_fact_grid_examples: Vec<M5ResolvedMarketplaceDetailFactGrid>,
) -> M5MarketplaceResultDetailControlsRow {
    M5MarketplaceResultDetailControlsRow {
        consumer_surface,
        qualification: M5MarketplaceInstallQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5MarketplaceInstallDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5MarketplaceInstallRequiredLabel::Identity,
            M5MarketplaceInstallRequiredLabel::State,
            M5MarketplaceInstallRequiredLabel::KeyboardRoute,
            M5MarketplaceInstallRequiredLabel::CompatibilityAndHost,
            M5MarketplaceInstallRequiredLabel::PermissionAndBudget,
            M5MarketplaceInstallRequiredLabel::PublisherAndSourceClass,
        ],
        accessibility_routes: M5MarketplaceInstallAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5MarketplaceResultDetailAnatomyPart::ALL.to_vec(),
        export_fields: M5MarketplaceResultDetailExportField::ALL.to_vec(),
        downgrade_triggers,
        marketplace_result_row_examples,
        marketplace_detail_fact_grid_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_SCHEMA_REF,
            M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF,
            M5_MARKETPLACE_DETAIL_FACT_GRID_SCHEMA_REF,
        ]),
        collapses_registry_source_class_across_public_mirrored_enterprise: false,
        hides_permission_widening_or_activation_cost: false,
        hides_publisher_transfer_or_deprecation: false,
        presents_incompatible_or_over_budget_as_ready: false,
    }
}

fn controls_rows() -> Vec<M5MarketplaceResultDetailControlsRow> {
    use M5MarketplaceInstallConsumerSurface as C;
    use M5MarketplaceInstallDowngradeTrigger as D;

    vec![
        base_row(
            C::MarketplaceUi,
            "Marketplace catalog owner",
            "The marketplace catalog renders one compact result row per artifact naming source class, compatibility, runtime model, permission posture, activation budget, support class, and publisher continuity, and the detail fact grid adds richer version ranges and lifecycle so a compare decision needs no disconnected page",
            "evidence:m5-marketplace-result-detail-marketplace-ui:001",
            vec![
                D::CompatibilityRangeUnstated,
                D::RegistrySourceClassCollapsed,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![row_acme_public_clean(), row_mirror_clean()],
            vec![grid_acme_public_clean(), grid_version_unstated()],
        ),
        base_row(
            C::ExtensionsUi,
            "Extensions manager owner",
            "The extensions manager reuses the same fact grammar, names permission widening a widened artifact requests, and degrades honestly when permission widening is hidden behind compact chrome",
            "evidence:m5-marketplace-result-detail-extensions-ui:001",
            vec![
                D::PermissionWideningHidden,
                D::TransitivePermissionHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![row_corp_clean(), row_permission_hidden()],
            vec![grid_corp_clean(), grid_permission_hidden()],
        ),
        base_row(
            C::RegistryUi,
            "Registry admin owner",
            "The registry admin surface degrades honestly when the registry source or lifecycle cannot be resolved or activation cost is hidden, keeping public versus mirrored versus enterprise source class explicit before mutation",
            "evidence:m5-marketplace-result-detail-registry-ui:001",
            vec![
                D::RegistrySourceClassCollapsed,
                D::ActivationCostHidden,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![row_source_unknown(), row_activation_hidden()],
            vec![grid_mirror_clean(), grid_lifecycle_unstated()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved row and grid truth, so a collapsed source class, an incompatible-shown-ready artifact, a hidden publisher transfer, or missing docs linkage is visible in evidence rather than hidden behind compact chrome",
            "evidence:m5-marketplace-result-detail-support-export:001",
            vec![
                D::RegistrySourceClassCollapsed,
                D::PublisherTransferHidden,
                D::CompatibilityRangeUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                row_source_collapsed(),
                row_incompatible_ready(),
                row_publisher_transfer_hidden(),
            ],
            vec![
                grid_source_collapsed(),
                grid_incompatible_ready(),
                grid_docs_unlinked(),
                grid_publisher_transfer_hidden(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product marketplace owner",
            "In-product surfaces reuse the same fact grammar a user sees in the marketplace catalog, always offering the command-backed detail path and degrading honestly when the artifact identity, support class, or detail path is missing",
            "evidence:m5-marketplace-result-detail-product-ui:001",
            vec![
                D::CompatibilityRangeUnstated,
                D::GenericChromeWordingUsed,
                D::RegistrySourceClassCollapsed,
                D::ProofStale,
            ],
            vec![
                row_acme_public_clean(),
                row_detail_missing(),
                row_identity_unstated(),
                row_support_unresolved(),
            ],
            vec![grid_acme_public_clean(), grid_publisher_transfer_hidden()],
        ),
    ]
}

fn governance_review() -> M5MarketplaceResultDetailGovernanceReview {
    M5MarketplaceResultDetailGovernanceReview {
        row_names_source_compatibility_and_runtime: true,
        row_names_permission_and_budget: true,
        grid_names_version_lifecycle_and_tier: true,
        grid_names_docs_changelog_and_issues: true,
        source_class_always_explicit_never_collapsed: true,
        permission_and_cost_always_named: true,
        incompatible_or_over_budget_never_ready: true,
        list_and_detail_share_one_fact_grammar: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5MarketplaceResultDetailConsumerProjection {
    M5MarketplaceResultDetailConsumerProjection {
        marketplace_surfaces_consume_source_and_compatibility_vocabulary: true,
        registry_surfaces_consume_permission_budget_publisher_vocabulary: true,
        marketplace_facts_trace_to_single_component_contract: true,
        support_export_reads_single_marketplace_source: true,
    }
}

fn proof_freshness() -> M5MarketplaceResultDetailProofFreshness {
    M5MarketplaceResultDetailProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5MarketplaceResultDetailReleasePosture {
    M5MarketplaceResultDetailReleasePosture {
        proof_packet_ref: M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_SCHEMA_REF,
        M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_DOC_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
        M5_MARKETPLACE_INSTALL_COMPONENT_DOC_REF,
        M5_MARKETPLACE_RESULT_ROW_SCHEMA_REF,
        M5_MARKETPLACE_DETAIL_FACT_GRID_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 marketplace-result-row / detail-fact-grid controls packet.
pub fn seeded_m5_marketplace_result_detail_controls() -> M5MarketplaceResultDetailControlsPacket {
    M5MarketplaceResultDetailControlsPacket::new(M5MarketplaceResultDetailControlsPacketInput {
        packet_id: M5_MARKETPLACE_RESULT_DETAIL_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 marketplace-result-row and marketplace-detail-fact-grid controls with compatibility, runtime model, permission posture, support class, performance evidence, publisher continuity, and registry source truth aligned across list and detail"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5MarketplaceResultDetailVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the marketplace-UI row is held at Beta pending list/detail parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_marketplace_result_detail_controls_marketplace_ui_beta_narrowed(
) -> M5MarketplaceResultDetailControlsPacket {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.packet_id =
        "m5-marketplace-result-row-detail-fact-grid-controls:marketplace-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MarketplaceInstallConsumerSurface::MarketplaceUi)
        .expect("marketplace-ui row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Beta;
    packet
}

/// Narrowed variant: the registry-UI row is narrowed to Preview pending detail-fact-grid parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_marketplace_result_detail_controls_registry_ui_preview_narrowed(
) -> M5MarketplaceResultDetailControlsPacket {
    let mut packet = seeded_m5_marketplace_result_detail_controls();
    packet.packet_id =
        "m5-marketplace-result-row-detail-fact-grid-controls:registry-ui-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5MarketplaceInstallConsumerSurface::RegistryUi)
        .expect("registry-ui row present");
    row.qualification = M5MarketplaceInstallQualificationClass::Preview;
    packet
}
