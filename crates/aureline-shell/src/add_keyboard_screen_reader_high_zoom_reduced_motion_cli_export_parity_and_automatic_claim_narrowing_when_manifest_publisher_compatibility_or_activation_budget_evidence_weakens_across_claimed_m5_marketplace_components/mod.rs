//! Keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity, and honest automatic
//! claim narrowing for the M5 marketplace-result-row / marketplace-detail-fact-grid /
//! compatibility-label-strip / permission-manifest-summary / activation-budget-band /
//! install-update-disable-rollback-review-sheet / publisher-continuity-row /
//! installed-state-diagnostics-card components.
//!
//! This module is the M05-1106 accessibility-and-auto-narrowing capstone over the frozen M5
//! marketplace-install component matrix
//! ([`crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix`]).
//! Where the freeze matrix defines the reusable marketplace result row, detail fact grid,
//! compatibility label strip, permission-manifest summary, activation-budget band,
//! install/update/disable/rollback review sheet, publisher-continuity row, and installed-state
//! diagnostics card primitives, and the 1101-1105 implementation lanes resolve their per-surface
//! truth, this lane certifies — per component family — that marketplace and install-review claims
//! stay **keyboard-complete, assistive-tech-reachable, high-zoom / reduced-motion-safe,
//! CLI/export-safe, and self-narrowing** rather than presenting a stale compatibility signal, a
//! partial permission manifest, an unverifiable publisher continuity, or a stale activation budget as
//! still a fully ready, source-attributed install:
//!
//! - **Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, reduced-motion-safe, and
//!   CLI/headless-reachable path into the same artifact identity, registry source class, compatibility
//!   range, host/runtime model, permission posture, activation-budget band, publisher continuity,
//!   disable scope, and rollback compatibility the rich component shows — never a hover-only badge that
//!   strands assistive-tech or headless-CLI users. Hierarchy-heavy families (the marketplace detail
//!   fact grid's nested compatibility / host / permission / activation-budget / publisher / source
//!   facts) additionally bind their grid to a flat list / textual path.
//! - **Export parity.** The support / release / CLI export reconstructs each component's meaning from
//!   typed tokens and opaque refs **without a raw payload**, preserving the same artifact identity,
//!   source class, compatibility range, permission posture, activation-budget band, publisher
//!   continuity, disable scope, and rollback compatibility shown in-product so support, help, and
//!   release proof can reconstruct exactly what the user was actually shown without leaking a raw
//!   manifest body, permission token, or activation-budget payload.
//! - **Honest auto-narrowing.** When a compatibility signal is stale, a permission manifest is only
//!   partial, publisher continuity is unverifiable, an activation budget is stale, a rollback's
//!   evidence is unverifiable, or a quarantine history is only partially captured, the component's
//!   claim auto-narrows from `install_ready_result` / `reviewable_listing_result` to a
//!   compatibility-unverified / permission-unverified / publisher-continuity / activation-budget /
//!   rollback-unverified / quarantine-history projection, discloses the narrowing with a precise
//!   trigger and binding dimension, and preserves the canonical artifact identity / registry source
//!   class / install scope. The underlying marketplace / install truth is never dropped opaquely. A
//!   component with every dimension intact must NOT carry a spurious narrowing, and a stale-compat /
//!   partial-permission / unverifiable-publisher / stale-budget / unverifiable-rollback state can never
//!   keep an install-ready claim — a stale compatibility signal never masquerades as a ready install,
//!   and hidden permission widening never reads as cost-free.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the marketplace UI, the
//!   extensions UI, the registry UI, the install-review UI, the settings UI, the help UI, the
//!   AI-context UI, the support export, and the product UI so product, help, and release publication
//!   stay aligned on downgrade behavior rather than drifting in copy — a ready-looking listing can
//!   never outrun the compatibility / permission / publisher / budget evidence it is being viewed away
//!   from.
//!
//! Each [`MarketplaceComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix::M5MarketplaceInstallComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5MarketplaceInstallRequiredLabel`],
//! [`M5MarketplaceInstallDowngradeTrigger`], and shared [`M5MarketplaceInstallConsumerSurface`]
//! consumer surfaces rather than minting parallel synonyms, so the certified labels stay byte-identical
//! to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw manifest bodies, permission tokens, activation-budget payloads,
//! credentials, secrets, and endpoint refs never cross this boundary; the packet carries only typed
//! class tokens, opaque marketplace / install refs, booleans, and controlled labels so support,
//! release, and diagnostics exports can reconstruct exactly what an accessible fallback would have
//! shown without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix::{
    M5MarketplaceInstallComponentFamily, M5MarketplaceInstallConsumerSurface,
    M5MarketplaceInstallDowngradeTrigger, M5MarketplaceInstallRequiredLabel,
    M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1106 marketplace-install component accessibility parity packet.
pub const MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`MarketplaceComponentAccessibilityPacket`].
pub const MARKETPLACE_INSTALL_A11Y_RECORD_KIND: &str =
    "m5_marketplace_install_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`MarketplaceComponentAccessibilityRow`].
pub const MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND: &str =
    "m5_marketplace_install_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const MARKETPLACE_INSTALL_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-marketplace-install-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const MARKETPLACE_INSTALL_A11Y_DOC_REF: &str =
    "docs/marketplace/m5_marketplace_install_component_accessibility_parity.md";

/// Repo-relative path of the frozen marketplace-install component matrix this lane certifies.
pub const MARKETPLACE_INSTALL_A11Y_COMPONENT_MATRIX_REF: &str =
    M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const MARKETPLACE_INSTALL_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-marketplace-install-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const MARKETPLACE_INSTALL_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-marketplace-install-component-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const MARKETPLACE_INSTALL_A11Y_CSV_REF: &str =
    "artifacts/release/m5-marketplace-install-component-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const MARKETPLACE_INSTALL_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-marketplace-install-component-accessibility-parity.md";

/// The reusable component families that render a non-linear hierarchy (the marketplace detail fact
/// grid's nested compatibility / host / permission / activation-budget / publisher / source facts) and
/// therefore MUST bind their grid to an equivalent flat list / textual path so the hierarchy is
/// navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5MarketplaceInstallComponentFamily) -> bool {
    matches!(
        family,
        M5MarketplaceInstallComponentFamily::MarketplaceDetailFactGrid
    )
}

/// The marketplace / install dimension whose weakening a family primarily discloses. Every row must
/// model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5MarketplaceInstallComponentFamily,
) -> M5MarketplaceComponentClaimDimension {
    match family {
        M5MarketplaceInstallComponentFamily::MarketplaceResultRow => {
            M5MarketplaceComponentClaimDimension::SourceClassClarity
        }
        M5MarketplaceInstallComponentFamily::MarketplaceDetailFactGrid => {
            M5MarketplaceComponentClaimDimension::CombinedFactClarity
        }
        M5MarketplaceInstallComponentFamily::CompatibilityLabelStrip => {
            M5MarketplaceComponentClaimDimension::CompatibilityEvidenceClarity
        }
        M5MarketplaceInstallComponentFamily::PermissionManifestSummary => {
            M5MarketplaceComponentClaimDimension::PermissionEvidenceClarity
        }
        M5MarketplaceInstallComponentFamily::ActivationBudgetBand => {
            M5MarketplaceComponentClaimDimension::ActivationBudgetClarity
        }
        M5MarketplaceInstallComponentFamily::InstallUpdateDisableRollbackReviewSheet => {
            M5MarketplaceComponentClaimDimension::RollbackReviewClarity
        }
        M5MarketplaceInstallComponentFamily::PublisherContinuityRow => {
            M5MarketplaceComponentClaimDimension::PublisherContinuityClarity
        }
        M5MarketplaceInstallComponentFamily::InstalledStateDiagnosticsCard => {
            M5MarketplaceComponentClaimDimension::InstalledHealthClarity
        }
    }
}

/// A rendered fallback modality for a marketplace / install component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceComponentFallbackModality {
    /// A rich, structured (nested compatibility / host / permission / budget / publisher / source
    /// facts) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5MarketplaceComponentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceComponentRenderingSurface {
    /// The full-capability desktop shell surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5MarketplaceComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability baseline
    /// and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless-CLI users
    /// (red).
    ViewOnlyTrap,
}

impl MarketplaceComponentNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceComponentExportSummaryState {
    /// The component meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl MarketplaceComponentExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl MarketplaceComponentNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The marketplace / install claim ceiling a component asserts: how strong a ready / reviewable install
/// posture it lets a surface present. Auto-narrowing lowers this ceiling when a marketplace / install
/// dimension weakens so a stale compatibility signal, a partial permission manifest, an unverifiable
/// publisher continuity, a stale activation budget, an unverifiable rollback, or a partial quarantine
/// history can never keep an old `InstallReadyResult` or `ReviewableListingResult` label — a stale
/// compatibility signal never masquerades as a ready install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceComponentClaim {
    /// Install-ready result: a fully identified, source-attributed, compatible, permission-clear
    /// artifact — the strongest claim, a surface Aureline can present as exactly ready right now.
    InstallReadyResult,
    /// Reviewable listing result: a self-sufficient, reviewable read-only marketplace / install view (a
    /// result a user can review) that is not itself a one-click-ready install path.
    ReviewableListingResult,
    /// Compatibility-unverified projection: the compatibility signal is stale / unavailable; the
    /// surface stays a compatibility-unverified projection with its last-known compatibility range
    /// preserved, never a ready install.
    CompatibilityUnverifiedProjection,
    /// Permission-unverified projection: the permission manifest is only partial; the surface stays a
    /// permission-unverified projection with its last-known permission posture preserved, never a
    /// permission-clear ready install.
    PermissionUnverifiedProjection,
    /// Activation-budget projection: the activation-budget signal is stale; the surface stays an
    /// activation-budget projection with its last-known band preserved, never a cost-free ready
    /// install.
    ActivationBudgetProjection,
    /// Rollback-unverified projection: a rollback's compatibility evidence is unverifiable; the surface
    /// stays a rollback-unverified projection that names the rollback limits, never a clean-revert
    /// result.
    RollbackUnverifiedProjection,
    /// Publisher-continuity projection: publisher continuity is unverifiable / transferred; the surface
    /// stays a publisher-continuity projection that names the transfer, never a continuous-publisher
    /// ready install.
    PublisherContinuityProjection,
    /// Quarantine-history projection: the installed quarantine history is only partially captured; the
    /// surface stays a quarantine-history projection that discloses the partial capture, never a
    /// clean-health result.
    QuarantineHistoryProjection,
}

impl M5MarketplaceComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::InstallReadyResult,
        Self::ReviewableListingResult,
        Self::CompatibilityUnverifiedProjection,
        Self::PermissionUnverifiedProjection,
        Self::ActivationBudgetProjection,
        Self::RollbackUnverifiedProjection,
        Self::PublisherContinuityProjection,
        Self::QuarantineHistoryProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::InstallReadyResult => 7,
            Self::ReviewableListingResult => 6,
            Self::CompatibilityUnverifiedProjection => 5,
            Self::PermissionUnverifiedProjection => 4,
            Self::ActivationBudgetProjection => 3,
            Self::RollbackUnverifiedProjection => 2,
            Self::PublisherContinuityProjection => 1,
            Self::QuarantineHistoryProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully ready, source-attributed install.
    pub const fn asserts_install_ready_result(self) -> bool {
        matches!(self, Self::InstallReadyResult)
    }

    /// Returns true when this claim asserts a fully self-sufficient (install-ready or reviewable)
    /// result.
    pub const fn asserts_self_sufficient_result(self) -> bool {
        matches!(
            self,
            Self::InstallReadyResult | Self::ReviewableListingResult
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallReadyResult => "install_ready_result",
            Self::ReviewableListingResult => "reviewable_listing_result",
            Self::CompatibilityUnverifiedProjection => "compatibility_unverified_projection",
            Self::PermissionUnverifiedProjection => "permission_unverified_projection",
            Self::ActivationBudgetProjection => "activation_budget_projection",
            Self::RollbackUnverifiedProjection => "rollback_unverified_projection",
            Self::PublisherContinuityProjection => "publisher_continuity_projection",
            Self::QuarantineHistoryProjection => "quarantine_history_projection",
        }
    }
}

/// The marketplace / install dimension whose state governs how far a component may claim to be a fully
/// ready, source-attributed install. The dimensions map 1:1 to the eight frozen component families so
/// every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceComponentClaimDimension {
    /// Source class clarity: is the registry source class (public / mirrored / enterprise) fully
    /// stated, or collapsed?
    SourceClassClarity,
    /// Combined fact clarity: are the compatibility / host / permission / activation-budget / publisher
    /// / source facts fully stated together?
    CombinedFactClarity,
    /// Compatibility evidence clarity: is the compatibility range and host / runtime model fully stated
    /// and current?
    CompatibilityEvidenceClarity,
    /// Permission evidence clarity: is the permission posture and any transitive widening fully stated?
    PermissionEvidenceClarity,
    /// Activation-budget clarity: is the activation-budget band and cost fully stated?
    ActivationBudgetClarity,
    /// Rollback review clarity: is the disable scope and rollback compatibility fully stated before
    /// mutation?
    RollbackReviewClarity,
    /// Publisher continuity clarity: is publisher transfer / deprecation fully stated, or shown as
    /// continuous?
    PublisherContinuityClarity,
    /// Installed health clarity: is the installed quarantine history and activation health fully
    /// stated?
    InstalledHealthClarity,
}

impl M5MarketplaceComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SourceClassClarity,
        Self::CombinedFactClarity,
        Self::CompatibilityEvidenceClarity,
        Self::PermissionEvidenceClarity,
        Self::ActivationBudgetClarity,
        Self::RollbackReviewClarity,
        Self::PublisherContinuityClarity,
        Self::InstalledHealthClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceClassClarity => "source_class_clarity",
            Self::CombinedFactClarity => "combined_fact_clarity",
            Self::CompatibilityEvidenceClarity => "compatibility_evidence_clarity",
            Self::PermissionEvidenceClarity => "permission_evidence_clarity",
            Self::ActivationBudgetClarity => "activation_budget_clarity",
            Self::RollbackReviewClarity => "rollback_review_clarity",
            Self::PublisherContinuityClarity => "publisher_continuity_clarity",
            Self::InstalledHealthClarity => "installed_health_clarity",
        }
    }
}

/// The observed condition of one marketplace / install dimension. Anything weaker than
/// [`Self::FullyQualified`] imposes a narrowing ceiling on the component's claim. The stale / partial /
/// unverifiable states the lane must auto-narrow on as *weakened evidence* — a stale compatibility
/// signal, a partial permission manifest, a stale activation budget, an unverifiable rollback, and an
/// unverifiable publisher continuity — are the states that [`Self::cannot_be_shown_install_ready`]
/// flags. A partial quarantine history is an honest disclosed-absence operation, not a truth
/// overstatement, so it is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceComponentConditionState {
    /// Fully identified, source-attributed, compatible, permission-clear, current — imposes no ceiling.
    FullyQualified,
    /// The compatibility signal is stale / unavailable — claim drops to a compatibility-unverified
    /// projection.
    CompatibilityEvidenceStale,
    /// The permission manifest is only partial — claim drops to a permission-unverified projection.
    PermissionEvidencePartial,
    /// The activation-budget signal is stale — claim drops to an activation-budget projection.
    ActivationBudgetStale,
    /// A rollback's compatibility evidence is unverifiable — claim drops to a rollback-unverified
    /// projection.
    RollbackEvidenceUnverifiable,
    /// Publisher continuity is unverifiable / transferred — claim drops to a publisher-continuity
    /// projection.
    PublisherContinuityUnverifiable,
    /// The installed quarantine history is only partially captured — claim drops to a quarantine-history
    /// projection.
    QuarantineHistoryPartial,
}

impl M5MarketplaceComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullyQualified,
        Self::CompatibilityEvidenceStale,
        Self::PermissionEvidencePartial,
        Self::ActivationBudgetStale,
        Self::RollbackEvidenceUnverifiable,
        Self::PublisherContinuityUnverifiable,
        Self::QuarantineHistoryPartial,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully ready,
    /// source-attributed install and must never be shown as such. A partial quarantine history is an
    /// honest disclosed-absence operation, not a truth overstatement, so it is deliberately excluded
    /// here.
    pub const fn cannot_be_shown_install_ready(self) -> bool {
        matches!(
            self,
            Self::CompatibilityEvidenceStale
                | Self::PermissionEvidencePartial
                | Self::ActivationBudgetStale
                | Self::RollbackEvidenceUnverifiable
                | Self::PublisherContinuityUnverifiable
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5MarketplaceComponentClaim {
        match self {
            Self::FullyQualified => M5MarketplaceComponentClaim::InstallReadyResult,
            Self::CompatibilityEvidenceStale => {
                M5MarketplaceComponentClaim::CompatibilityUnverifiedProjection
            }
            Self::PermissionEvidencePartial => {
                M5MarketplaceComponentClaim::PermissionUnverifiedProjection
            }
            Self::ActivationBudgetStale => M5MarketplaceComponentClaim::ActivationBudgetProjection,
            Self::RollbackEvidenceUnverifiable => {
                M5MarketplaceComponentClaim::RollbackUnverifiedProjection
            }
            Self::PublisherContinuityUnverifiable => {
                M5MarketplaceComponentClaim::PublisherContinuityProjection
            }
            Self::QuarantineHistoryPartial => {
                M5MarketplaceComponentClaim::QuarantineHistoryProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each
    /// state maps to the on-topic frozen trigger the freeze matrix already governs, so the certified
    /// reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5MarketplaceInstallDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5MarketplaceInstallDowngradeTrigger::ProofStale,
            Self::CompatibilityEvidenceStale => {
                M5MarketplaceInstallDowngradeTrigger::CompatibilityRangeUnstated
            }
            Self::PermissionEvidencePartial => {
                M5MarketplaceInstallDowngradeTrigger::PermissionWideningHidden
            }
            Self::ActivationBudgetStale => {
                M5MarketplaceInstallDowngradeTrigger::ActivationCostHidden
            }
            Self::RollbackEvidenceUnverifiable => {
                M5MarketplaceInstallDowngradeTrigger::RollbackIncompatibilityHidden
            }
            Self::PublisherContinuityUnverifiable => {
                M5MarketplaceInstallDowngradeTrigger::PublisherTransferHidden
            }
            Self::QuarantineHistoryPartial => {
                M5MarketplaceInstallDowngradeTrigger::QuarantineHistoryHidden
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::CompatibilityEvidenceStale => "compatibility_evidence_stale",
            Self::PermissionEvidencePartial => "permission_evidence_partial",
            Self::ActivationBudgetStale => "activation_budget_stale",
            Self::RollbackEvidenceUnverifiable => "rollback_evidence_unverifiable",
            Self::PublisherContinuityUnverifiable => "publisher_continuity_unverifiable",
            Self::QuarantineHistoryPartial => "quarantine_history_partial",
        }
    }
}

/// One marketplace / install dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5MarketplaceComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5MarketplaceComponentConditionState,
}

/// An honest claim auto-narrow block. When a marketplace / install dimension weakens, the component's
/// claim lowers to the permitted ceiling, names the binding dimension and frozen trigger, and preserves
/// the canonical artifact identity / registry source class / install scope rather than silently
/// dropping it — the underlying marketplace / install truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceComponentClaimAutoNarrow {
    /// The claim the component is narrowed to.
    pub narrowed_to: M5MarketplaceComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5MarketplaceComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5MarketplaceInstallDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical artifact identity, registry source class, install scope, and export scope are
    /// preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying marketplace / install truth is preserved (never dropped) across the narrowing;
    /// must hold so compatibility-unverified, permission-unverified, activation-budget,
    /// rollback-unverified, publisher-continuity, and quarantine-history states never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl MarketplaceComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and marketplace /
    /// install truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl MarketplaceComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least
    /// one export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5MarketplaceComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: MarketplaceComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a marketplace / install-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity with no
    /// narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims install-ready, or drops state silently
    /// (red).
    Stranded,
}

impl MarketplaceComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one marketplace / install-component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceComponentAccessibilityRow {
    /// Record kind; must equal [`MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5MarketplaceInstallComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the marketplace / install artifact this component represents; stays visible on
    /// every surface, so this is never empty.
    pub component_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual (list /
    /// textual / CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5MarketplaceComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical artifact identity, source class,
    /// compatibility range, host model, permission posture, activation-budget band, publisher
    /// continuity, disable scope, and rollback compatibility as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: MarketplaceComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: MarketplaceComponentNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: MarketplaceComponentNonVisualReachState,
    /// Reduced-motion behavior of the non-visual path.
    pub reduced_motion_reach: MarketplaceComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: MarketplaceComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: MarketplaceComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: MarketplaceComponentCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5MarketplaceComponentClaim,
    /// The observed condition of each modeled marketplace / install dimension.
    #[serde(default)]
    pub claim_conditions: Vec<MarketplaceComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full
    /// claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<MarketplaceComponentClaimAutoNarrow>,
    /// Whether the underlying marketplace / install truth is preserved on this component regardless of
    /// narrowing; must hold so compatibility-unverified, permission-unverified, activation-budget,
    /// rollback-unverified, publisher-continuity, and quarantine-history states never fail opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5MarketplaceComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<MarketplaceComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5MarketplaceInstallRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5MarketplaceInstallConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl MarketplaceComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat non-visual
    /// path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model
    /// that dimension.
    pub fn condition_for(
        &self,
        dimension: M5MarketplaceComponentClaimDimension,
    ) -> M5MarketplaceComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5MarketplaceComponentConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// family's full claim.
    pub fn permitted_claim(&self) -> M5MarketplaceComponentClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows
    /// below the family's full claim.
    pub fn binding_condition(&self) -> Option<&MarketplaceComponentClaimConditionEntry> {
        let mut binding: Option<(&MarketplaceComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5MarketplaceComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5MarketplaceComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale compatibility signal, a partial permission manifest, a
    /// stale activation budget, an unverifiable rollback, an unverifiable publisher continuity, or a
    /// partial quarantine history can no longer keep an old `InstallReadyResult` /
    /// `ReviewableListingResult` label. The effective claim never exceeds the permitted ceiling; when a
    /// dimension narrows below the full claim, an honest narrow block is present, narrows to exactly the
    /// permitted ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and
    /// preserves canonical identity and truth. When nothing narrows, no spurious narrow block is
    /// present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / install-ready honesty: a stale-compat / partial-permission / stale-budget /
    /// unverifiable-rollback / unverifiable-publisher state never keeps an install-ready claim — a
    /// stale compatibility signal never masquerades as a ready install. When such a state is modeled,
    /// the effective claim must not assert `InstallReadyResult`.
    pub fn install_ready_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_install_ready());
        !(has_unprovable_state && self.effective_claim().asserts_install_ready_result())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / reduced-motion / CLI trap, a hierarchy-heavy family
    /// offers a non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.component_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.reduced_motion_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: compatibility-unverified, permission-unverified, activation-budget,
    /// rollback-unverified, publisher-continuity, and quarantine-history states preserve the underlying
    /// marketplace / install truth. The row must assert `truth_preserved`, and any narrow block must
    /// preserve truth continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an honest
    /// claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.reduced_motion_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / help / release publication stay aligned on the
    /// same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5MarketplaceInstallRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> MarketplaceComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.install_ready_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return MarketplaceComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            MarketplaceComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            MarketplaceComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND
            && self.schema_version == MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.component_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} reduced_motion={reduced_motion} cli={cli} export={export} \
full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            reduced_motion = self.reduced_motion_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1106 marketplace / install-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_install_ready_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`MarketplaceComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketplaceComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<MarketplaceComponentAccessibilityRow>,
}

/// Checked-in M05-1106 marketplace / install-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<MarketplaceComponentAccessibilityRow>,
    pub summary: MarketplaceComponentAccessibilitySummary,
}

impl MarketplaceComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: MarketplaceComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION,
            record_kind: MARKETPLACE_INSTALL_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: MarketplaceComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_install_ready_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5MarketplaceInstallComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5MarketplaceComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5MarketplaceComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5MarketplaceComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5MarketplaceInstallConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> MarketplaceComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5MarketplaceInstallConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&MarketplaceComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                MarketplaceComponentAccessibilityStatus::Parity => green += 1,
                MarketplaceComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                MarketplaceComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        MarketplaceComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(MarketplaceComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(MarketplaceComponentAccessibilityRow::claim_is_honest),
            all_install_ready_honesty_holds: self
                .rows
                .iter()
                .all(MarketplaceComponentAccessibilityRow::install_ready_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(MarketplaceComponentAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(MarketplaceComponentAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(MarketplaceComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<MarketplaceComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION {
            violations.push(MarketplaceComponentAccessibilityViolation::SchemaVersion {
                expected: MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != MARKETPLACE_INSTALL_A11Y_RECORD_KIND {
            violations.push(MarketplaceComponentAccessibilityViolation::RecordKind {
                expected: MARKETPLACE_INSTALL_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(MarketplaceComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(MarketplaceComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_install_ready())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(MarketplaceComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory marketplace / install label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured grid *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5MarketplaceComponentFallbackModality::Structured)
            {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts an install-ready / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / install-ready honesty: a stale-compat / partial-permission / stale-budget /
            // unverifiable-rollback / unverifiable-publisher state never keeps an install-ready claim.
            if !row.install_ready_honesty_holds() {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::WeakStateShownAsInstallReady {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve marketplace / install truth.
            if !row.preserves_truth_continuity() {
                violations.push(MarketplaceComponentAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == MarketplaceComponentAccessibilityStatus::Stranded {
                violations.push(MarketplaceComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5MarketplaceInstallComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5MarketplaceComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis)
        // is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5MarketplaceComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (install-ready → … → quarantine-history) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5MarketplaceComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Install-ready honesty must be proven with at least one stale-compat / partial-permission /
        // stale-budget / unverifiable-rollback / unverifiable-publisher row in the packet, so the
        // "cannot-prove never shown as install-ready" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations
                .push(MarketplaceComponentAccessibilityViolation::InstallReadyHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the marketplace, extensions, registry,
        // install-review, settings, help, AI-context, support-export, and product surfaces — so every
        // consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5MarketplaceInstallConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    MarketplaceComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(MarketplaceComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("marketplace / install-component accessibility parity packet serializes"),
        ) {
            violations
                .push(MarketplaceComponentAccessibilityViolation::RawMarketplaceMaterialInExport);
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
            .expect("marketplace / install-component accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,high_zoom_reach,reduced_motion_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{reduced_motion},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                reduced_motion = row.reduced_motion_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Marketplace / Install-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5MarketplaceInstallComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in marketplace / install-component accessibility parity export.
pub fn current_m5_marketplace_install_component_a11y_export(
) -> Result<MarketplaceComponentAccessibilityPacket, MarketplaceComponentAccessibilityArtifactError>
{
    let packet: MarketplaceComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-marketplace-install-component-accessibility-parity/support_export.json"
    )))
    .map_err(MarketplaceComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(MarketplaceComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in marketplace / install-component accessibility parity
/// export.
#[derive(Debug)]
pub enum MarketplaceComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<MarketplaceComponentAccessibilityViolation>),
}

impl fmt::Display for MarketplaceComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "marketplace / install-component accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "marketplace / install-component accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for MarketplaceComponentAccessibilityArtifactError {}

/// Validation failure for M05-1106 marketplace / install-component accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketplaceComponentAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5MarketplaceComponentClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsInstallReady {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5MarketplaceInstallComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5MarketplaceComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5MarketplaceComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5MarketplaceComponentClaim,
    },
    InstallReadyHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5MarketplaceInstallConsumerSurface,
    },
    SummaryMismatch,
    RawMarketplaceMaterialInExport,
}

impl MarketplaceComponentAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::HierarchyHeavyMissingStructured { .. } => "hierarchy_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsInstallReady { .. } => "weak_state_shown_as_install_ready",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::InstallReadyHonestyUnproven => "install_ready_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawMarketplaceMaterialInExport => "raw_marketplace_material_in_export",
        }
    }
}

impl fmt::Display for MarketplaceComponentAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory marketplace / install label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts an install-ready / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsInstallReady { id } => {
                write!(
                    f,
                    "row {id} shows a stale-compat / partial-permission / stale-budget / unverifiable-rollback / unverifiable-publisher state as an install-ready result"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / reduced-motion / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve marketplace / install truth across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::InstallReadyHonestyUnproven => {
                write!(
                    f,
                    "no stale-compat / partial-permission / stale-budget / unverifiable-rollback / unverifiable-publisher row is present to prove the install-ready-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawMarketplaceMaterialInExport => {
                write!(f, "export contains raw marketplace / install material")
            }
        }
    }
}

impl Error for MarketplaceComponentAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "incompatible"
            | "not compatible"
            | "mixed"
            | "expired"
            | "no budget"
            | "ready"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The canonical packet id for the checked-in stable export.
pub const MARKETPLACE_INSTALL_A11Y_PACKET_ID: &str =
    "m5-marketplace-install-component-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in marketplace / install-component accessibility parity packet. This
/// is the one source of truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_marketplace_install_component_a11y_packet(
) -> MarketplaceComponentAccessibilityPacket {
    MarketplaceComponentAccessibilityPacket::new(MarketplaceComponentAccessibilityPacketInput {
        packet_id: MARKETPLACE_INSTALL_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-11T00:00:00Z".to_owned(),
        matrix_ref: MARKETPLACE_INSTALL_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:marketplace-install-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5MarketplaceInstallRequiredLabel> {
    M5MarketplaceInstallRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> MarketplaceComponentCopyExportParity {
    MarketplaceComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5MarketplaceComponentClaimDimension,
    state: M5MarketplaceComponentConditionState,
) -> MarketplaceComponentClaimConditionEntry {
    MarketplaceComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general
/// product UI — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5MarketplaceInstallConsumerSurface],
) -> Vec<M5MarketplaceInstallConsumerSurface> {
    let mut out = vec![
        M5MarketplaceInstallConsumerSurface::SupportExport,
        M5MarketplaceInstallConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full
/// label and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions
/// it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: MarketplaceComponentNarrowingDisclosureState,
) -> Vec<MarketplaceComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        MarketplaceComponentRenderingNarrowingDisclosure {
            rendering_surface: M5MarketplaceComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        MarketplaceComponentRenderingNarrowingDisclosure {
            rendering_surface: M5MarketplaceComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_overlay".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary
/// parity.
fn parity_surfaces(labels: &[&str]) -> Vec<MarketplaceComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        MarketplaceComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions
/// while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<MarketplaceComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        MarketplaceComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5MarketplaceComponentRenderingSurface> {
    vec![
        M5MarketplaceComponentRenderingSurface::DesktopFull,
        M5MarketplaceComponentRenderingSurface::CliHeadless,
        M5MarketplaceComponentRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5MarketplaceComponentFallbackModality> {
    vec![
        M5MarketplaceComponentFallbackModality::List,
        M5MarketplaceComponentFallbackModality::Textual,
        M5MarketplaceComponentFallbackModality::Cli,
    ]
}

fn seeded_rows() -> Vec<MarketplaceComponentAccessibilityRow> {
    vec![
        // Marketplace result row (fully source-attributed) — the registry source class, compatibility,
        // and publisher continuity are fully stated, so it is an install-ready result reachable on
        // every surface with no narrowing (green).
        MarketplaceComponentAccessibilityRow {
            record_kind: MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:marketplace-result-row-source-attributed".to_owned(),
            component_family: M5MarketplaceInstallComponentFamily::MarketplaceResultRow,
            source_family_schema_ref: MARKETPLACE_INSTALL_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "marketplace:result-row:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                MarketplaceComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:marketplace-result-row-source-attributed:a11y".to_owned(),
            copy_export: copy_export(&[
                "artifact_identity",
                "registry_source_class",
                "compatibility_range",
                "publisher_continuity",
            ]),
            full_ready_claim: M5MarketplaceComponentClaim::InstallReadyResult,
            claim_conditions: vec![condition(
                M5MarketplaceComponentClaimDimension::SourceClassClarity,
                M5MarketplaceComponentConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "artifact_identity",
                "registry_source_class",
                "compatibility_range",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5MarketplaceInstallConsumerSurface::MarketplaceUi,
                M5MarketplaceInstallConsumerSurface::RegistryUi,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 marketplace result row".to_owned(),
                MARKETPLACE_INSTALL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("marketplace-result-row-source-attributed"),
        },
        // Marketplace detail fact grid (fully stated) — hierarchy-heavy (nested compatibility / host /
        // permission / activation-budget / publisher / source facts); the combined facts are fully
        // stated, so it is a reviewable listing result that binds its nested fact grid to a flat list /
        // textual path, but its dense grid narrows the screen-reader traversal to a disclosed linear
        // walk (yellow).
        MarketplaceComponentAccessibilityRow {
            record_kind: MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:marketplace-detail-fact-grid-stated".to_owned(),
            component_family: M5MarketplaceInstallComponentFamily::MarketplaceDetailFactGrid,
            source_family_schema_ref: MARKETPLACE_INSTALL_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "marketplace:detail-fact-grid:0002".to_owned(),
            fallback_modalities: vec![
                M5MarketplaceComponentFallbackModality::Structured,
                M5MarketplaceComponentFallbackModality::List,
                M5MarketplaceComponentFallbackModality::Textual,
                M5MarketplaceComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach:
                MarketplaceComponentNonVisualReachState::DisclosedReducedButReachable,
            high_zoom_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                MarketplaceComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:marketplace-detail-fact-grid-stated:a11y".to_owned(),
            copy_export: copy_export(&[
                "grid_identity",
                "compatibility_and_host",
                "permission_and_budget",
                "publisher_and_source_class",
            ]),
            full_ready_claim: M5MarketplaceComponentClaim::ReviewableListingResult,
            claim_conditions: vec![condition(
                M5MarketplaceComponentClaimDimension::CombinedFactClarity,
                M5MarketplaceComponentConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "grid_identity",
                "compatibility_and_host",
                "permission_and_budget",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5MarketplaceInstallConsumerSurface::MarketplaceUi,
                M5MarketplaceInstallConsumerSurface::AiContextUi,
            ]),
            source_refs: vec![
                "UX Design System marketplace detail fact grid".to_owned(),
                MARKETPLACE_INSTALL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("marketplace-detail-fact-grid-stated"),
        },
        // Compatibility label strip (compatibility evidence stale) — the compatibility signal is stale
        // / unavailable, so the strip auto-narrows to a compatibility-unverified projection that keeps
        // the last-known compatibility range visible, never a ready install (yellow).
        MarketplaceComponentAccessibilityRow {
            record_kind: MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:compatibility-label-strip-stale-compat".to_owned(),
            component_family: M5MarketplaceInstallComponentFamily::CompatibilityLabelStrip,
            source_family_schema_ref: MARKETPLACE_INSTALL_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "marketplace:compatibility-label-strip:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: MarketplaceComponentNonVisualReachState::DisclosedReducedButReachable,
            reduced_motion_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                MarketplaceComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:compatibility-label-strip-stale-compat:a11y".to_owned(),
            copy_export: copy_export(&[
                "strip_identity",
                "compatibility_range",
                "host_runtime_model",
                "last_known_compatibility",
            ]),
            full_ready_claim: M5MarketplaceComponentClaim::InstallReadyResult,
            claim_conditions: vec![condition(
                M5MarketplaceComponentClaimDimension::CompatibilityEvidenceClarity,
                M5MarketplaceComponentConditionState::CompatibilityEvidenceStale,
            )],
            claim_narrow: Some(MarketplaceComponentClaimAutoNarrow {
                narrowed_to: M5MarketplaceComponentClaim::CompatibilityUnverifiedProjection,
                binding_dimension: M5MarketplaceComponentClaimDimension::CompatibilityEvidenceClarity,
                trigger: M5MarketplaceInstallDowngradeTrigger::CompatibilityRangeUnstated,
                narrowed_label:
                    "This artifact's compatibility signal is stale — shown as a compatibility-unverified projection that keeps the last-known compatibility range and host / runtime model visible, never as a freshly verified, install-ready artifact"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "strip_identity",
                "compatibility_range",
                "host_runtime_model",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5MarketplaceInstallConsumerSurface::ExtensionsUi,
                M5MarketplaceInstallConsumerSurface::InstallReviewUi,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 compatibility labels".to_owned(),
                MARKETPLACE_INSTALL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("compatibility-label-strip-stale-compat"),
        },
        // Permission manifest summary (permission evidence partial) — the permission manifest is only
        // partial, so the summary auto-narrows to a permission-unverified projection that keeps the
        // last-known permission posture visible and never hides widening, never a permission-clear
        // ready install (yellow).
        MarketplaceComponentAccessibilityRow {
            record_kind: MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:permission-manifest-summary-partial".to_owned(),
            component_family: M5MarketplaceInstallComponentFamily::PermissionManifestSummary,
            source_family_schema_ref: MARKETPLACE_INSTALL_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "marketplace:permission-manifest-summary:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                MarketplaceComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:permission-manifest-summary-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "summary_identity",
                "permission_posture",
                "transitive_widening",
                "last_known_permission",
            ]),
            full_ready_claim: M5MarketplaceComponentClaim::InstallReadyResult,
            claim_conditions: vec![condition(
                M5MarketplaceComponentClaimDimension::PermissionEvidenceClarity,
                M5MarketplaceComponentConditionState::PermissionEvidencePartial,
            )],
            claim_narrow: Some(MarketplaceComponentClaimAutoNarrow {
                narrowed_to: M5MarketplaceComponentClaim::PermissionUnverifiedProjection,
                binding_dimension: M5MarketplaceComponentClaimDimension::PermissionEvidenceClarity,
                trigger: M5MarketplaceInstallDowngradeTrigger::PermissionWideningHidden,
                narrowed_label:
                    "This artifact's permission manifest is only partial — shown as a permission-unverified projection that keeps the last-known permission posture and any unresolved transitive widening visible, never as a permission-clear, install-ready artifact"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "summary_identity",
                "permission_posture",
                "transitive_widening",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5MarketplaceInstallConsumerSurface::InstallReviewUi,
                M5MarketplaceInstallConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "TDD marketplace integrity / permission manifest".to_owned(),
                MARKETPLACE_INSTALL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("permission-manifest-summary-partial"),
        },
        // Activation-budget band (activation budget stale) — the activation-budget signal is stale, so
        // the band auto-narrows to an activation-budget projection that keeps the last-known band
        // visible, never a cost-free ready install (yellow).
        MarketplaceComponentAccessibilityRow {
            record_kind: MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:activation-budget-band-stale-budget".to_owned(),
            component_family: M5MarketplaceInstallComponentFamily::ActivationBudgetBand,
            source_family_schema_ref: MARKETPLACE_INSTALL_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "marketplace:activation-budget-band:0005".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach:
                MarketplaceComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                MarketplaceComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:activation-budget-band-stale-budget:a11y".to_owned(),
            copy_export: copy_export(&[
                "band_identity",
                "activation_budget_band",
                "activation_cost",
                "last_known_band",
            ]),
            full_ready_claim: M5MarketplaceComponentClaim::InstallReadyResult,
            claim_conditions: vec![condition(
                M5MarketplaceComponentClaimDimension::ActivationBudgetClarity,
                M5MarketplaceComponentConditionState::ActivationBudgetStale,
            )],
            claim_narrow: Some(MarketplaceComponentClaimAutoNarrow {
                narrowed_to: M5MarketplaceComponentClaim::ActivationBudgetProjection,
                binding_dimension: M5MarketplaceComponentClaimDimension::ActivationBudgetClarity,
                trigger: M5MarketplaceInstallDowngradeTrigger::ActivationCostHidden,
                narrowed_label:
                    "This artifact's activation-budget signal is stale — shown as an activation-budget projection that keeps the last-known budget band and activation cost visible, never as a cost-free, install-ready artifact"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "band_identity",
                "activation_budget_band",
                "activation_cost",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5MarketplaceInstallConsumerSurface::ExtensionsUi,
                M5MarketplaceInstallConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "TAD extension-runtime budget / quarantine".to_owned(),
                MARKETPLACE_INSTALL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("activation-budget-band-stale-budget"),
        },
        // Install/update/disable/rollback review sheet (rollback evidence unverifiable) — the rollback's
        // compatibility evidence is unverifiable, so the sheet auto-narrows to a rollback-unverified
        // projection that names the rollback limits and disable scope, never a clean-revert result
        // (yellow).
        MarketplaceComponentAccessibilityRow {
            record_kind: MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:install-review-sheet-unverifiable-rollback".to_owned(),
            component_family:
                M5MarketplaceInstallComponentFamily::InstallUpdateDisableRollbackReviewSheet,
            source_family_schema_ref: MARKETPLACE_INSTALL_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "marketplace:install-review-sheet:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                MarketplaceComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:install-review-sheet-unverifiable-rollback:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "sheet_identity",
                "disable_scope",
                "rollback_compatibility",
                "rollback_limits_note",
            ]),
            full_ready_claim: M5MarketplaceComponentClaim::InstallReadyResult,
            claim_conditions: vec![condition(
                M5MarketplaceComponentClaimDimension::RollbackReviewClarity,
                M5MarketplaceComponentConditionState::RollbackEvidenceUnverifiable,
            )],
            claim_narrow: Some(MarketplaceComponentClaimAutoNarrow {
                narrowed_to: M5MarketplaceComponentClaim::RollbackUnverifiedProjection,
                binding_dimension: M5MarketplaceComponentClaimDimension::RollbackReviewClarity,
                trigger: M5MarketplaceInstallDowngradeTrigger::RollbackIncompatibilityHidden,
                narrowed_label:
                    "This transaction's rollback compatibility is unverifiable — shown as a rollback-unverified projection that names the disable scope and the rollback limits before mutation, never as a clean, one-click reversible install"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "sheet_identity",
                "disable_scope",
                "rollback_compatibility",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5MarketplaceInstallConsumerSurface::InstallReviewUi,
                M5MarketplaceInstallConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 install / update / disable / rollback review".to_owned(),
                MARKETPLACE_INSTALL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("install-review-sheet-unverifiable-rollback"),
        },
        // Publisher-continuity row (publisher continuity unverifiable) — publisher continuity is
        // unverifiable / transferred, so the row auto-narrows to a publisher-continuity projection that
        // names the transfer and source class, never a continuous-publisher ready install (yellow).
        MarketplaceComponentAccessibilityRow {
            record_kind: MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:publisher-continuity-row-unverifiable".to_owned(),
            component_family: M5MarketplaceInstallComponentFamily::PublisherContinuityRow,
            source_family_schema_ref: MARKETPLACE_INSTALL_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "marketplace:publisher-continuity-row:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                MarketplaceComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:publisher-continuity-row-unverifiable:a11y".to_owned(),
            copy_export: copy_export(&[
                "row_identity",
                "publisher_continuity",
                "transfer_history",
                "registry_source_class",
            ]),
            full_ready_claim: M5MarketplaceComponentClaim::InstallReadyResult,
            claim_conditions: vec![condition(
                M5MarketplaceComponentClaimDimension::PublisherContinuityClarity,
                M5MarketplaceComponentConditionState::PublisherContinuityUnverifiable,
            )],
            claim_narrow: Some(MarketplaceComponentClaimAutoNarrow {
                narrowed_to: M5MarketplaceComponentClaim::PublisherContinuityProjection,
                binding_dimension: M5MarketplaceComponentClaimDimension::PublisherContinuityClarity,
                trigger: M5MarketplaceInstallDowngradeTrigger::PublisherTransferHidden,
                narrowed_label:
                    "This artifact's publisher continuity is unverifiable — shown as a publisher-continuity projection that names the transfer history and registry source class, never collapsing a transferred or unverifiable publisher into a continuous, install-ready publisher"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "row_identity",
                "publisher_continuity",
                "transfer_history",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5MarketplaceInstallConsumerSurface::MarketplaceUi,
                M5MarketplaceInstallConsumerSurface::RegistryUi,
            ]),
            source_refs: vec![
                "TDD publisher continuity / SDK publication".to_owned(),
                MARKETPLACE_INSTALL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("publisher-continuity-row-unverifiable"),
        },
        // Installed-state diagnostics card (quarantine history partial) — the installed quarantine
        // history is only partially captured, so the card auto-narrows to a quarantine-history
        // projection that discloses the partial capture and activation health, never a clean-health
        // result (yellow). A partial quarantine history is an honest disclosed-absence operation, not an
        // install-ready overstatement.
        MarketplaceComponentAccessibilityRow {
            record_kind: MARKETPLACE_INSTALL_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: MARKETPLACE_INSTALL_A11Y_SCHEMA_VERSION,
            row_id: "a11y:installed-state-diagnostics-card-partial-quarantine".to_owned(),
            component_family: M5MarketplaceInstallComponentFamily::InstalledStateDiagnosticsCard,
            source_family_schema_ref: MARKETPLACE_INSTALL_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            component_context_ref: "marketplace:installed-state-diagnostics-card:0008".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: MarketplaceComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                MarketplaceComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:installed-state-diagnostics-card-partial-quarantine:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "card_identity",
                "quarantine_history",
                "activation_health",
                "partial_capture_note",
            ]),
            full_ready_claim: M5MarketplaceComponentClaim::InstallReadyResult,
            claim_conditions: vec![condition(
                M5MarketplaceComponentClaimDimension::InstalledHealthClarity,
                M5MarketplaceComponentConditionState::QuarantineHistoryPartial,
            )],
            claim_narrow: Some(MarketplaceComponentClaimAutoNarrow {
                narrowed_to: M5MarketplaceComponentClaim::QuarantineHistoryProjection,
                binding_dimension: M5MarketplaceComponentClaimDimension::InstalledHealthClarity,
                trigger: M5MarketplaceInstallDowngradeTrigger::QuarantineHistoryHidden,
                narrowed_label:
                    "This installed artifact's quarantine history is only partially captured — shown as a quarantine-history projection that discloses the partial capture and the activation health, never as a clean, fully-diagnosed installed health"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "card_identity",
                "quarantine_history",
                "activation_health",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5MarketplaceInstallConsumerSurface::ExtensionsUi,
                M5MarketplaceInstallConsumerSurface::HelpUi,
            ]),
            source_refs: vec![
                "TAD extension-runtime budget / quarantine architecture".to_owned(),
                MARKETPLACE_INSTALL_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("installed-state-diagnostics-card-partial-quarantine"),
        },
    ]
}
