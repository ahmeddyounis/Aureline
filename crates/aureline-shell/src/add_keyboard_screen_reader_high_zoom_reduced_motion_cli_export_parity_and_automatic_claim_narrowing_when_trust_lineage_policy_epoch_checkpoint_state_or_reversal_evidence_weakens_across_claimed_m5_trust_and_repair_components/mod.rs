//! Keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity, and honest automatic
//! claim narrowing for the M5 workspace-trust-banner / trust-fact-grid / trust-elevation-sheet /
//! restricted-capability-row / root-trust-strip / repair-transaction-preview-card / rollback-class-strip
//! / repair-result-receipt-row components.
//!
//! This module is the M05-1098 accessibility-and-auto-narrowing capstone over the frozen M5
//! workspace-trust-repair component matrix
//! ([`crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix`]).
//! Where the freeze matrix defines the reusable workspace-trust banner, trust-fact grid, trust-elevation
//! sheet, restricted-capability row, root-trust strip, repair-transaction preview card, rollback-class
//! strip, and repair-result receipt row primitives, and the 1093-1097 implementation lanes resolve
//! their per-surface truth, this lane certifies — per component family — that trust and repair claims
//! stay **keyboard-complete, assistive-tech-reachable, high-zoom / reduced-motion-safe,
//! CLI/export-safe, and self-narrowing** rather than presenting a stale trust lineage, an expired
//! policy epoch, a mixed-root trust, a narrowed capability, a missing repair checkpoint, or an unproven
//! reversal as still a fully trusted, reviewed result:
//!
//! - **Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, reduced-motion-safe, and
//!   CLI/headless-reachable path into the same object identity, trust class, grant source, policy
//!   epoch, per-root trust, narrowed capability, checkpoint state, reversal class, and repair outcome
//!   the rich component shows — never a hover-only badge that strands assistive-tech or headless-CLI
//!   users. Hierarchy-heavy families (the trust-fact grid's nested actor / object / scope /
//!   policy-source / capability-delta facts) additionally bind their grid to a flat list / textual
//!   path.
//! - **Export parity.** The support / release / CLI export reconstructs each component's meaning from
//!   typed tokens and opaque refs **without a raw payload**, preserving the same object identity, trust
//!   class, grant source, policy epoch, per-root trust, narrowed capability, checkpoint state, reversal
//!   class, and repair outcome shown in-product so support, docs, and release proof can reconstruct
//!   exactly what the user was actually shown without leaking a raw grant token, policy body, or
//!   checkpoint payload.
//! - **Honest auto-narrowing.** When a trust lineage is stale, a policy epoch is expired, per-root
//!   trust is mixed, a capability is narrowed, a repair checkpoint is missing, or a reversal's evidence
//!   is only partial, the component's claim auto-narrows from `full_trust_reviewed_result` /
//!   `reviewable_result` to a stale-lineage / expired-epoch / mixed-root / narrowed-capability /
//!   missing-checkpoint / unproven-reversal projection, discloses the narrowing with a precise trigger
//!   and binding dimension, and preserves the canonical object identity / grant source / repair scope.
//!   The underlying trust / repair truth is never dropped opaquely. A component with every dimension
//!   intact must NOT carry a spurious narrowing, and a stale-lineage / expired-epoch / mixed-root /
//!   unproven-reversal state can never keep a full-trust reviewed claim — a stale trust lineage never
//!   masquerades as full, blanket trust and a partial reversal never reads as a generic success.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the workspace-trust UI, the
//!   settings UI, the Project Doctor UI, the safe-mode UI, the extensions UI, the remote UI, the
//!   AI-context UI, the support export, and the product UI so product, docs, and release publication
//!   stay aligned on downgrade behavior rather than drifting in copy — a trusted-looking surface can
//!   never outrun the grant / epoch / checkpoint / reversal proof it is being viewed away from.
//!
//! Each [`TrustRepairComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix::M5WorkspaceTrustRepairComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5WorkspaceTrustRepairRequiredLabel`],
//! [`M5WorkspaceTrustRepairDowngradeTrigger`], and shared [`M5WorkspaceTrustRepairConsumerSurface`]
//! consumer surfaces rather than minting parallel synonyms, so the certified labels stay byte-identical
//! to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw grant tokens, policy bodies, checkpoint payloads, credentials,
//! secrets, and endpoint refs never cross this boundary; the packet carries only typed class tokens,
//! opaque trust / repair refs, booleans, and controlled labels so support, release, and diagnostics
//! exports can reconstruct exactly what an accessible fallback would have shown without leaking
//! sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix::{
    M5WorkspaceTrustRepairComponentFamily, M5WorkspaceTrustRepairConsumerSurface,
    M5WorkspaceTrustRepairDowngradeTrigger, M5WorkspaceTrustRepairRequiredLabel,
    M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1098 workspace-trust-repair component accessibility parity packet.
pub const WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`TrustRepairComponentAccessibilityPacket`].
pub const WORKSPACE_TRUST_REPAIR_A11Y_RECORD_KIND: &str =
    "m5_workspace_trust_repair_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`TrustRepairComponentAccessibilityRow`].
pub const WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND: &str =
    "m5_workspace_trust_repair_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-workspace-trust-repair-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const WORKSPACE_TRUST_REPAIR_A11Y_DOC_REF: &str =
    "docs/trust/m5_workspace_trust_repair_component_accessibility_parity.md";

/// Repo-relative path of the frozen workspace-trust-repair component matrix this lane certifies.
pub const WORKSPACE_TRUST_REPAIR_A11Y_COMPONENT_MATRIX_REF: &str =
    M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const WORKSPACE_TRUST_REPAIR_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-workspace-trust-repair-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const WORKSPACE_TRUST_REPAIR_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-workspace-trust-repair-component-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const WORKSPACE_TRUST_REPAIR_A11Y_CSV_REF: &str =
    "artifacts/release/m5-workspace-trust-repair-component-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const WORKSPACE_TRUST_REPAIR_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-workspace-trust-repair-component-accessibility-parity.md";

/// The reusable component families that render a non-linear hierarchy (the trust-fact grid's nested
/// actor / object / scope / policy-source / capability-delta facts) and therefore MUST bind their grid
/// to an equivalent flat list / textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5WorkspaceTrustRepairComponentFamily) -> bool {
    matches!(family, M5WorkspaceTrustRepairComponentFamily::TrustFactGrid)
}

/// The trust / repair dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5WorkspaceTrustRepairComponentFamily,
) -> M5TrustRepairComponentClaimDimension {
    match family {
        M5WorkspaceTrustRepairComponentFamily::WorkspaceTrustBanner => {
            M5TrustRepairComponentClaimDimension::TrustGrantLineage
        }
        M5WorkspaceTrustRepairComponentFamily::TrustFactGrid => {
            M5TrustRepairComponentClaimDimension::TrustScopeClarity
        }
        M5WorkspaceTrustRepairComponentFamily::TrustElevationSheet => {
            M5TrustRepairComponentClaimDimension::ElevationEffectClarity
        }
        M5WorkspaceTrustRepairComponentFamily::RestrictedCapabilityRow => {
            M5TrustRepairComponentClaimDimension::CapabilityNarrowClarity
        }
        M5WorkspaceTrustRepairComponentFamily::RootTrustStrip => {
            M5TrustRepairComponentClaimDimension::PerRootTrustClarity
        }
        M5WorkspaceTrustRepairComponentFamily::RepairTransactionPreviewCard => {
            M5TrustRepairComponentClaimDimension::RepairPreviewClarity
        }
        M5WorkspaceTrustRepairComponentFamily::RollbackClassStrip => {
            M5TrustRepairComponentClaimDimension::ReversalClassClarity
        }
        M5WorkspaceTrustRepairComponentFamily::RepairResultReceiptRow => {
            M5TrustRepairComponentClaimDimension::RepairOutcomeClarity
        }
    }
}

/// A rendered fallback modality for a trust / repair component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustRepairComponentFallbackModality {
    /// A rich, structured (nested actor / object / scope / policy-source / capability-delta) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5TrustRepairComponentFallbackModality {
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
pub enum M5TrustRepairComponentRenderingSurface {
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

impl M5TrustRepairComponentRenderingSurface {
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
pub enum TrustRepairComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless-CLI users
    /// (red).
    ViewOnlyTrap,
}

impl TrustRepairComponentNonVisualReachState {
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
pub enum TrustRepairComponentExportSummaryState {
    /// The component meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl TrustRepairComponentExportSummaryState {
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
pub enum TrustRepairComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl TrustRepairComponentNarrowingDisclosureState {
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

/// The trust / repair claim ceiling a component asserts: how strong a trusted / reviewed posture it
/// lets a surface present. Auto-narrowing lowers this ceiling when a trust / repair dimension weakens
/// so a stale trust lineage, an expired policy epoch, a mixed-root trust, a narrowed capability, a
/// missing repair checkpoint, or an unproven reversal can never keep an old `FullTrustReviewedResult`
/// or `ReviewableResult` label — a stale trust lineage never masquerades as full, blanket trust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustRepairComponentClaim {
    /// Full trust reviewed result: a fully identified, trusted, reviewed object / repair — the
    /// strongest claim, a surface Aureline can present as exactly true right now.
    FullTrustReviewedResult,
    /// Reviewable result: a self-sufficient, reviewable read-only trust / repair view (a result a user
    /// can review) that is not itself a certified full-trust path.
    ReviewableResult,
    /// Stale-lineage projection: the trust lineage / grant source is stale; the surface stays a
    /// stale-lineage projection with its last-known grant source preserved, never a fully trusted
    /// result.
    StaleLineageProjection,
    /// Expired-epoch projection: the policy epoch behind a grant is expired / superseded; the surface
    /// stays an expired-epoch projection with its last-known epoch preserved, never a current-epoch
    /// trusted result.
    ExpiredEpochProjection,
    /// Mixed-root projection: per-root trust is mixed; the surface stays a mixed-root projection that
    /// names the per-root trust, never a blanket-trusted result.
    MixedRootProjection,
    /// Narrowed-capability projection: a capability is narrowed / restricted; the surface stays a
    /// narrowed-capability projection with its still-safe actions preserved, never a full-capability
    /// result.
    NarrowedCapabilityProjection,
    /// Missing-checkpoint projection: a repair checkpoint is absent; the surface stays a
    /// missing-checkpoint projection that discloses the reversal limits, never a fully reversible
    /// result.
    MissingCheckpointProjection,
    /// Unproven-reversal projection: a reversal's evidence is only partial; the surface stays an
    /// unproven-reversal projection that names the partial outcome, never a generic-success result.
    UnprovenReversalProjection,
}

impl M5TrustRepairComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::FullTrustReviewedResult,
        Self::ReviewableResult,
        Self::StaleLineageProjection,
        Self::ExpiredEpochProjection,
        Self::MixedRootProjection,
        Self::NarrowedCapabilityProjection,
        Self::MissingCheckpointProjection,
        Self::UnprovenReversalProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::FullTrustReviewedResult => 7,
            Self::ReviewableResult => 6,
            Self::StaleLineageProjection => 5,
            Self::ExpiredEpochProjection => 4,
            Self::MixedRootProjection => 3,
            Self::NarrowedCapabilityProjection => 2,
            Self::MissingCheckpointProjection => 1,
            Self::UnprovenReversalProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, reviewed result.
    pub const fn asserts_full_trust_reviewed_result(self) -> bool {
        matches!(self, Self::FullTrustReviewedResult)
    }

    /// Returns true when this claim asserts a fully self-sufficient (full-trust or reviewable) result.
    pub const fn asserts_trustworthy_result(self) -> bool {
        matches!(self, Self::FullTrustReviewedResult | Self::ReviewableResult)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullTrustReviewedResult => "full_trust_reviewed_result",
            Self::ReviewableResult => "reviewable_result",
            Self::StaleLineageProjection => "stale_lineage_projection",
            Self::ExpiredEpochProjection => "expired_epoch_projection",
            Self::MixedRootProjection => "mixed_root_projection",
            Self::NarrowedCapabilityProjection => "narrowed_capability_projection",
            Self::MissingCheckpointProjection => "missing_checkpoint_projection",
            Self::UnprovenReversalProjection => "unproven_reversal_projection",
        }
    }
}

/// The trust / repair dimension whose state governs how far a component may claim to be a fully
/// trusted, reviewed result. The dimensions map 1:1 to the eight frozen component families so every
/// family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustRepairComponentClaimDimension {
    /// Trust grant lineage: is the grant source and policy epoch fully stated and current?
    TrustGrantLineage,
    /// Trust scope clarity: is the actor / object / scope of trust fully stated?
    TrustScopeClarity,
    /// Elevation effect clarity: is the elevation's capability delta and lasting-versus-one-time effect
    /// fully stated?
    ElevationEffectClarity,
    /// Capability narrow clarity: are the blocked and still-safe capabilities fully stated?
    CapabilityNarrowClarity,
    /// Per-root trust clarity: is per-root trust fully stated, or collapsed into blanket trust?
    PerRootTrustClarity,
    /// Repair preview clarity: is the repair's checkpoint state and impact scope fully stated?
    RepairPreviewClarity,
    /// Reversal class clarity: is the reversal class (exact / compensate / regenerate / manual /
    /// audit-only) fully stated?
    ReversalClassClarity,
    /// Repair outcome clarity: is the repair's applied outcome (including partial success) fully
    /// stated?
    RepairOutcomeClarity,
}

impl M5TrustRepairComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TrustGrantLineage,
        Self::TrustScopeClarity,
        Self::ElevationEffectClarity,
        Self::CapabilityNarrowClarity,
        Self::PerRootTrustClarity,
        Self::RepairPreviewClarity,
        Self::ReversalClassClarity,
        Self::RepairOutcomeClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustGrantLineage => "trust_grant_lineage",
            Self::TrustScopeClarity => "trust_scope_clarity",
            Self::ElevationEffectClarity => "elevation_effect_clarity",
            Self::CapabilityNarrowClarity => "capability_narrow_clarity",
            Self::PerRootTrustClarity => "per_root_trust_clarity",
            Self::RepairPreviewClarity => "repair_preview_clarity",
            Self::ReversalClassClarity => "reversal_class_clarity",
            Self::RepairOutcomeClarity => "repair_outcome_clarity",
        }
    }
}

/// The observed condition of one trust / repair dimension. Anything weaker than
/// [`Self::FullTrustReviewed`] imposes a narrowing ceiling on the component's claim. The stale /
/// expired / mixed / partial states the lane must auto-narrow on as *weakened evidence* — a stale
/// trust lineage, an expired policy epoch, a mixed-root trust, and a partial reversal — are the states
/// that [`Self::cannot_be_shown_full_trust`] flags. A narrowed capability and a missing checkpoint are
/// honest restricted-mode / disclosed-absence operations, not truth overstatements, so they are
/// deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustRepairComponentConditionState {
    /// Fully identified, trusted, reviewed, current — imposes no ceiling.
    FullTrustReviewed,
    /// The trust lineage / grant source is stale — claim drops to a stale-lineage projection.
    TrustLineageStale,
    /// The policy epoch behind a grant is expired / superseded — claim drops to an expired-epoch
    /// projection.
    PolicyEpochExpired,
    /// Per-root trust is mixed — claim drops to a mixed-root projection.
    PerRootTrustMixed,
    /// A capability is narrowed / restricted — claim drops to a narrowed-capability projection.
    CapabilityNarrowed,
    /// A repair checkpoint is absent — claim drops to a missing-checkpoint projection.
    CheckpointMissing,
    /// A reversal's evidence is only partial — claim drops to an unproven-reversal projection.
    ReversalEvidencePartial,
}

impl M5TrustRepairComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullTrustReviewed,
        Self::TrustLineageStale,
        Self::PolicyEpochExpired,
        Self::PerRootTrustMixed,
        Self::CapabilityNarrowed,
        Self::CheckpointMissing,
        Self::ReversalEvidencePartial,
    ];

    /// Returns true when the dimension is weaker than full trust and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullTrustReviewed)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully
    /// trusted, reviewed result and must never be shown as such. A narrowed capability and a missing
    /// checkpoint are honest restricted-mode / disclosed-absence operations, not truth overstatements,
    /// so they are deliberately excluded here.
    pub const fn cannot_be_shown_full_trust(self) -> bool {
        matches!(
            self,
            Self::TrustLineageStale
                | Self::PolicyEpochExpired
                | Self::PerRootTrustMixed
                | Self::ReversalEvidencePartial
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5TrustRepairComponentClaim {
        match self {
            Self::FullTrustReviewed => M5TrustRepairComponentClaim::FullTrustReviewedResult,
            Self::TrustLineageStale => M5TrustRepairComponentClaim::StaleLineageProjection,
            Self::PolicyEpochExpired => M5TrustRepairComponentClaim::ExpiredEpochProjection,
            Self::PerRootTrustMixed => M5TrustRepairComponentClaim::MixedRootProjection,
            Self::CapabilityNarrowed => M5TrustRepairComponentClaim::NarrowedCapabilityProjection,
            Self::CheckpointMissing => M5TrustRepairComponentClaim::MissingCheckpointProjection,
            Self::ReversalEvidencePartial => {
                M5TrustRepairComponentClaim::UnprovenReversalProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each
    /// state maps to the on-topic frozen trigger the freeze matrix already governs, so the certified
    /// reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            // The full-trust baseline never narrows; kept for exhaustiveness.
            Self::FullTrustReviewed => M5WorkspaceTrustRepairDowngradeTrigger::ProofStale,
            Self::TrustLineageStale => M5WorkspaceTrustRepairDowngradeTrigger::GrantSourceUnstated,
            Self::PolicyEpochExpired => M5WorkspaceTrustRepairDowngradeTrigger::PolicyEpochUnstated,
            Self::PerRootTrustMixed => {
                M5WorkspaceTrustRepairDowngradeTrigger::MixedRootShownAsUniformTrust
            }
            Self::CapabilityNarrowed => {
                M5WorkspaceTrustRepairDowngradeTrigger::NarrowedCapabilityUnstated
            }
            Self::CheckpointMissing => {
                M5WorkspaceTrustRepairDowngradeTrigger::CheckpointAbsenceHidden
            }
            Self::ReversalEvidencePartial => {
                M5WorkspaceTrustRepairDowngradeTrigger::ReversalLimitHidden
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullTrustReviewed => "full_trust_reviewed",
            Self::TrustLineageStale => "trust_lineage_stale",
            Self::PolicyEpochExpired => "policy_epoch_expired",
            Self::PerRootTrustMixed => "per_root_trust_mixed",
            Self::CapabilityNarrowed => "capability_narrowed",
            Self::CheckpointMissing => "checkpoint_missing",
            Self::ReversalEvidencePartial => "reversal_evidence_partial",
        }
    }
}

/// One trust / repair dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5TrustRepairComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5TrustRepairComponentConditionState,
}

/// An honest claim auto-narrow block. When a trust / repair dimension weakens, the component's claim
/// lowers to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the
/// canonical object identity / grant source / repair scope rather than silently dropping it — the
/// underlying trust / repair truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairComponentClaimAutoNarrow {
    /// The claim the component is narrowed to.
    pub narrowed_to: M5TrustRepairComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5TrustRepairComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5WorkspaceTrustRepairDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical object identity, grant source, repair scope, and export scope are preserved rather
    /// than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying trust / repair truth is preserved (never dropped) across the narrowing; must hold
    /// so stale-lineage, expired-epoch, mixed-root, narrowed-capability, missing-checkpoint, and
    /// unproven-reversal states never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl TrustRepairComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and trust / repair
    /// truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl TrustRepairComponentCopyExportParity {
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
pub struct TrustRepairComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5TrustRepairComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: TrustRepairComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a trust / repair-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustRepairComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity with no
    /// narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims full trust, or drops state silently
    /// (red).
    Stranded,
}

impl TrustRepairComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one trust / repair-component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairComponentAccessibilityRow {
    /// Record kind; must equal [`WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5WorkspaceTrustRepairComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the trust / repair object this component represents; stays visible on every
    /// surface, so this is never empty.
    pub trust_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual (list /
    /// textual / CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5TrustRepairComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical object identity, trust class, grant source,
    /// policy epoch, per-root trust, narrowed capability, checkpoint state, reversal class, and repair
    /// outcome as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: TrustRepairComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: TrustRepairComponentNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: TrustRepairComponentNonVisualReachState,
    /// Reduced-motion behavior of the non-visual path.
    pub reduced_motion_reach: TrustRepairComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: TrustRepairComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: TrustRepairComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: TrustRepairComponentCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_trust_claim: M5TrustRepairComponentClaim,
    /// The observed condition of each modeled trust / repair dimension.
    #[serde(default)]
    pub claim_conditions: Vec<TrustRepairComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full
    /// claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<TrustRepairComponentClaimAutoNarrow>,
    /// Whether the underlying trust / repair truth is preserved on this component regardless of
    /// narrowing; must hold so stale-lineage, expired-epoch, mixed-root, narrowed-capability,
    /// missing-checkpoint, and unproven-reversal states never fail opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5TrustRepairComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<TrustRepairComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5WorkspaceTrustRepairRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5WorkspaceTrustRepairConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl TrustRepairComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat non-visual
    /// path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullTrustReviewed` when the row does not
    /// model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5TrustRepairComponentClaimDimension,
    ) -> M5TrustRepairComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5TrustRepairComponentConditionState::FullTrustReviewed)
    }

    /// Whether any modeled dimension is weaker than full trust.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// family's full claim.
    pub fn permitted_claim(&self) -> M5TrustRepairComponentClaim {
        let mut permitted = self.full_trust_claim;
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
    pub fn binding_condition(&self) -> Option<&TrustRepairComponentClaimConditionEntry> {
        let mut binding: Option<(&TrustRepairComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_trust_claim.capability_rank() {
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
    pub fn binding_dimension(&self) -> Option<M5TrustRepairComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5TrustRepairComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_trust_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale trust lineage, an expired policy epoch, a mixed-root trust,
    /// a narrowed capability, a missing repair checkpoint, or an unproven reversal can no longer keep an
    /// old `FullTrustReviewedResult` / `ReviewableResult` label. The effective claim never exceeds the
    /// permitted ceiling; when a dimension narrows below the full claim, an honest narrow block is
    /// present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing dimension with
    /// its frozen trigger, and preserves canonical identity and truth. When nothing narrows, no
    /// spurious narrow block is present.
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

    /// AC / full-trust honesty: a stale-lineage / expired-epoch / mixed-root / unproven-reversal state
    /// never keeps a full-trust reviewed claim — a stale trust lineage never masquerades as full,
    /// blanket trust. When such a state is modeled, the effective claim must not assert
    /// `FullTrustReviewedResult`.
    pub fn full_trust_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_full_trust());
        !(has_unprovable_state && self.effective_claim().asserts_full_trust_reviewed_result())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / reduced-motion / CLI trap, a hierarchy-heavy family
    /// offers a non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.trust_context_ref.trim().is_empty()
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

    /// AC / no-loss: stale-lineage, expired-epoch, mixed-root, narrowed-capability, missing-checkpoint,
    /// and unproven-reversal states preserve the underlying trust / repair truth. The row must assert
    /// `truth_preserved`, and any narrow block must preserve truth continuity too.
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
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned on the
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
        M5WorkspaceTrustRepairRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> TrustRepairComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.full_trust_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return TrustRepairComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            TrustRepairComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            TrustRepairComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND
            && self.schema_version == WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.trust_context_ref.trim().is_empty()
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
            full = self.full_trust_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1098 trust / repair-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_full_trust_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`TrustRepairComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustRepairComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<TrustRepairComponentAccessibilityRow>,
}

/// Checked-in M05-1098 trust / repair-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRepairComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<TrustRepairComponentAccessibilityRow>,
    pub summary: TrustRepairComponentAccessibilitySummary,
}

impl TrustRepairComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: TrustRepairComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION,
            record_kind: WORKSPACE_TRUST_REPAIR_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: TrustRepairComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_full_trust_honesty_holds: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5WorkspaceTrustRepairComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5TrustRepairComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5TrustRepairComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5TrustRepairComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5WorkspaceTrustRepairConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> TrustRepairComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5WorkspaceTrustRepairConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&TrustRepairComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                TrustRepairComponentAccessibilityStatus::Parity => green += 1,
                TrustRepairComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                TrustRepairComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        TrustRepairComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(TrustRepairComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(TrustRepairComponentAccessibilityRow::claim_is_honest),
            all_full_trust_honesty_holds: self
                .rows
                .iter()
                .all(TrustRepairComponentAccessibilityRow::full_trust_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(TrustRepairComponentAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(TrustRepairComponentAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(TrustRepairComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<TrustRepairComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION {
            violations.push(TrustRepairComponentAccessibilityViolation::SchemaVersion {
                expected: WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != WORKSPACE_TRUST_REPAIR_A11Y_RECORD_KIND {
            violations.push(TrustRepairComponentAccessibilityViolation::RecordKind {
                expected: WORKSPACE_TRUST_REPAIR_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(TrustRepairComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(TrustRepairComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_full_trust())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(TrustRepairComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory trust / repair label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured grid *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5TrustRepairComponentFallbackModality::Structured)
            {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a full-trust / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / full-trust honesty: a stale-lineage / expired-epoch / mixed-root / unproven-reversal
            // state never keeps a full-trust reviewed claim.
            if !row.full_trust_honesty_holds() {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::WeakStateShownAsFullTrust {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve trust / repair truth.
            if !row.preserves_truth_continuity() {
                violations.push(TrustRepairComponentAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == TrustRepairComponentAccessibilityStatus::Stranded {
                violations.push(TrustRepairComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5WorkspaceTrustRepairComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5TrustRepairComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the full-trust baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5TrustRepairComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (full-trust → … → unproven-reversal) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5TrustRepairComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Full-trust honesty must be proven with at least one stale-lineage / expired-epoch /
        // mixed-root / unproven-reversal row in the packet, so the "cannot-prove never shown as full
        // trust" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(TrustRepairComponentAccessibilityViolation::FullTrustHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the workspace-trust, settings, Doctor,
        // safe-mode, extensions, remote, AI-context, support-export, and product surfaces — so every
        // consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5WorkspaceTrustRepairConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    TrustRepairComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(TrustRepairComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("trust / repair-component accessibility parity packet serializes"),
        ) {
            violations.push(TrustRepairComponentAccessibilityViolation::RawTrustMaterialInExport);
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
            .expect("trust / repair-component accessibility parity packet serializes")
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
                full = row.full_trust_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Workspace-Trust / Repair-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5WorkspaceTrustRepairComponentFamily::ALL.len(),
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
                    row.full_trust_claim.as_str(),
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

/// Reads and validates the checked-in trust / repair-component accessibility parity export.
pub fn current_m5_workspace_trust_repair_component_a11y_export(
) -> Result<TrustRepairComponentAccessibilityPacket, TrustRepairComponentAccessibilityArtifactError>
{
    let packet: TrustRepairComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workspace-trust-repair-component-accessibility-parity/support_export.json"
    )))
    .map_err(TrustRepairComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TrustRepairComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in trust / repair-component accessibility parity export.
#[derive(Debug)]
pub enum TrustRepairComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TrustRepairComponentAccessibilityViolation>),
}

impl fmt::Display for TrustRepairComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "trust / repair-component accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "trust / repair-component accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for TrustRepairComponentAccessibilityArtifactError {}

/// Validation failure for M05-1098 trust / repair-component accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrustRepairComponentAccessibilityViolation {
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
        dimension: M5TrustRepairComponentClaimDimension,
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
    WeakStateShownAsFullTrust {
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
        family: M5WorkspaceTrustRepairComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5TrustRepairComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5TrustRepairComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5TrustRepairComponentClaim,
    },
    FullTrustHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5WorkspaceTrustRepairConsumerSurface,
    },
    SummaryMismatch,
    RawTrustMaterialInExport,
}

impl TrustRepairComponentAccessibilityViolation {
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
            Self::WeakStateShownAsFullTrust { .. } => "weak_state_shown_as_full_trust",
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
            Self::FullTrustHonestyUnproven => "full_trust_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawTrustMaterialInExport => "raw_trust_material_in_export",
        }
    }
}

impl fmt::Display for TrustRepairComponentAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory trust / repair label")
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
                    "row {id} over-asserts a full-trust / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsFullTrust { id } => {
                write!(
                    f,
                    "row {id} shows a stale-lineage / expired-epoch / mixed-root / unproven-reversal state as a full-trust reviewed result"
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
                    "row {id} does not preserve trust / repair truth across narrowing"
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
            Self::FullTrustHonestyUnproven => {
                write!(
                    f,
                    "no stale-lineage / expired-epoch / mixed-root / unproven-reversal row is present to prove the full-trust-honesty guarantee"
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
            Self::RawTrustMaterialInExport => {
                write!(f, "export contains raw trust / repair material")
            }
        }
    }
}

impl Error for TrustRepairComponentAccessibilityViolation {}

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
            | "untrusted"
            | "not trusted"
            | "mixed"
            | "expired"
            | "no checkpoint"
            | "success"
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
pub const WORKSPACE_TRUST_REPAIR_A11Y_PACKET_ID: &str =
    "m5-workspace-trust-repair-component-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in trust / repair-component accessibility parity packet. This is the
/// one source of truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_workspace_trust_repair_component_a11y_packet(
) -> TrustRepairComponentAccessibilityPacket {
    TrustRepairComponentAccessibilityPacket::new(TrustRepairComponentAccessibilityPacketInput {
        packet_id: WORKSPACE_TRUST_REPAIR_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-11T00:00:00Z".to_owned(),
        matrix_ref: WORKSPACE_TRUST_REPAIR_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:trust-repair-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5WorkspaceTrustRepairRequiredLabel> {
    M5WorkspaceTrustRepairRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> TrustRepairComponentCopyExportParity {
    TrustRepairComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5TrustRepairComponentClaimDimension,
    state: M5TrustRepairComponentConditionState,
) -> TrustRepairComponentClaimConditionEntry {
    TrustRepairComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general
/// product UI — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5WorkspaceTrustRepairConsumerSurface],
) -> Vec<M5WorkspaceTrustRepairConsumerSurface> {
    let mut out = vec![
        M5WorkspaceTrustRepairConsumerSurface::SupportExport,
        M5WorkspaceTrustRepairConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full
/// label and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions
/// it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: TrustRepairComponentNarrowingDisclosureState,
) -> Vec<TrustRepairComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        TrustRepairComponentRenderingNarrowingDisclosure {
            rendering_surface: M5TrustRepairComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        TrustRepairComponentRenderingNarrowingDisclosure {
            rendering_surface: M5TrustRepairComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_overlay".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary
/// parity.
fn parity_surfaces(labels: &[&str]) -> Vec<TrustRepairComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        TrustRepairComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions
/// while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<TrustRepairComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        TrustRepairComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5TrustRepairComponentRenderingSurface> {
    vec![
        M5TrustRepairComponentRenderingSurface::DesktopFull,
        M5TrustRepairComponentRenderingSurface::CliHeadless,
        M5TrustRepairComponentRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5TrustRepairComponentFallbackModality> {
    vec![
        M5TrustRepairComponentFallbackModality::List,
        M5TrustRepairComponentFallbackModality::Textual,
        M5TrustRepairComponentFallbackModality::Cli,
    ]
}

fn seeded_rows() -> Vec<TrustRepairComponentAccessibilityRow> {
    vec![
        // Workspace-trust banner (trust lineage stale) — the grant source / trust lineage behind the
        // banner is stale, so it auto-narrows to a stale-lineage projection that keeps the last-known
        // grant source visible, never a fully trusted result (yellow).
        TrustRepairComponentAccessibilityRow {
            record_kind: WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION,
            row_id: "a11y:workspace-trust-banner-stale-lineage".to_owned(),
            component_family: M5WorkspaceTrustRepairComponentFamily::WorkspaceTrustBanner,
            source_family_schema_ref: WORKSPACE_TRUST_REPAIR_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            trust_context_ref: "trust:workspace-trust-banner:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TrustRepairComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:workspace-trust-banner-stale-lineage:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "trust_class",
                "grant_source",
                "last_known_grant_source",
            ]),
            full_trust_claim: M5TrustRepairComponentClaim::FullTrustReviewedResult,
            claim_conditions: vec![condition(
                M5TrustRepairComponentClaimDimension::TrustGrantLineage,
                M5TrustRepairComponentConditionState::TrustLineageStale,
            )],
            claim_narrow: Some(TrustRepairComponentClaimAutoNarrow {
                narrowed_to: M5TrustRepairComponentClaim::StaleLineageProjection,
                binding_dimension: M5TrustRepairComponentClaimDimension::TrustGrantLineage,
                trigger: M5WorkspaceTrustRepairDowngradeTrigger::GrantSourceUnstated,
                narrowed_label:
                    "This workspace's trust lineage is stale — shown as a stale-lineage projection that keeps the last-known grant source and trust class visible, never as a freshly verified, fully trusted workspace"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "trust_class",
                "grant_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkspaceTrustRepairConsumerSurface::WorkspaceTrustUi,
                M5WorkspaceTrustRepairConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 workspace trust banner".to_owned(),
                WORKSPACE_TRUST_REPAIR_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("workspace-trust-banner-stale-lineage"),
        },
        // Trust-fact grid (fully scoped) — hierarchy-heavy (nested actor / object / scope /
        // policy-source / capability-delta facts); the trust scope is fully stated, so it is a
        // reviewable result that binds its nested fact grid to a flat list / textual path, but its
        // dense grid narrows the screen-reader traversal to a disclosed linear walk (yellow).
        TrustRepairComponentAccessibilityRow {
            record_kind: WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION,
            row_id: "a11y:trust-fact-grid-scoped".to_owned(),
            component_family: M5WorkspaceTrustRepairComponentFamily::TrustFactGrid,
            source_family_schema_ref: WORKSPACE_TRUST_REPAIR_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            trust_context_ref: "trust:trust-fact-grid:0002".to_owned(),
            fallback_modalities: vec![
                M5TrustRepairComponentFallbackModality::Structured,
                M5TrustRepairComponentFallbackModality::List,
                M5TrustRepairComponentFallbackModality::Textual,
                M5TrustRepairComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach:
                TrustRepairComponentNonVisualReachState::DisclosedReducedButReachable,
            high_zoom_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TrustRepairComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:trust-fact-grid-scoped:a11y".to_owned(),
            copy_export: copy_export(&[
                "grid_identity",
                "actor_object_scope",
                "policy_source",
                "capability_delta",
            ]),
            full_trust_claim: M5TrustRepairComponentClaim::ReviewableResult,
            claim_conditions: vec![condition(
                M5TrustRepairComponentClaimDimension::TrustScopeClarity,
                M5TrustRepairComponentConditionState::FullTrustReviewed,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "grid_identity",
                "actor_object_scope",
                "policy_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkspaceTrustRepairConsumerSurface::WorkspaceTrustUi,
                M5WorkspaceTrustRepairConsumerSurface::AiContextUi,
            ]),
            source_refs: vec![
                "UI/UX Design System trust fact grid".to_owned(),
                WORKSPACE_TRUST_REPAIR_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("trust-fact-grid-scoped"),
        },
        // Trust-elevation sheet (policy epoch expired) — the policy epoch behind the elevation is
        // expired / superseded, so the sheet auto-narrows to an expired-epoch projection that keeps the
        // last-known epoch visible, never a current-epoch trusted elevation (yellow).
        TrustRepairComponentAccessibilityRow {
            record_kind: WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION,
            row_id: "a11y:trust-elevation-sheet-expired-epoch".to_owned(),
            component_family: M5WorkspaceTrustRepairComponentFamily::TrustElevationSheet,
            source_family_schema_ref: WORKSPACE_TRUST_REPAIR_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            trust_context_ref: "trust:trust-elevation-sheet:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: TrustRepairComponentNonVisualReachState::DisclosedReducedButReachable,
            reduced_motion_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TrustRepairComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:trust-elevation-sheet-expired-epoch:a11y".to_owned(),
            copy_export: copy_export(&[
                "sheet_identity",
                "capability_delta",
                "policy_epoch",
                "last_known_epoch",
            ]),
            full_trust_claim: M5TrustRepairComponentClaim::FullTrustReviewedResult,
            claim_conditions: vec![condition(
                M5TrustRepairComponentClaimDimension::ElevationEffectClarity,
                M5TrustRepairComponentConditionState::PolicyEpochExpired,
            )],
            claim_narrow: Some(TrustRepairComponentClaimAutoNarrow {
                narrowed_to: M5TrustRepairComponentClaim::ExpiredEpochProjection,
                binding_dimension: M5TrustRepairComponentClaimDimension::ElevationEffectClarity,
                trigger: M5WorkspaceTrustRepairDowngradeTrigger::PolicyEpochUnstated,
                narrowed_label:
                    "The policy epoch behind this elevation is superseded — shown as an expired-epoch projection that keeps the last-known epoch and capability delta visible, never as a current-epoch trusted elevation"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "sheet_identity",
                "capability_delta",
                "policy_epoch",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkspaceTrustRepairConsumerSurface::WorkspaceTrustUi,
                M5WorkspaceTrustRepairConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "TDD trust / authority elevation".to_owned(),
                WORKSPACE_TRUST_REPAIR_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("trust-elevation-sheet-expired-epoch"),
        },
        // Restricted-capability row (capability narrowed) — a capability is narrowed / restricted, so
        // the row auto-narrows to a narrowed-capability projection that keeps its still-safe actions
        // visible, never a full-capability result (yellow). A narrowed capability is an honest
        // restricted-mode operation, not a full-trust overstatement.
        TrustRepairComponentAccessibilityRow {
            record_kind: WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION,
            row_id: "a11y:restricted-capability-row-narrowed".to_owned(),
            component_family: M5WorkspaceTrustRepairComponentFamily::RestrictedCapabilityRow,
            source_family_schema_ref: WORKSPACE_TRUST_REPAIR_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            trust_context_ref: "trust:restricted-capability-row:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TrustRepairComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:restricted-capability-row-narrowed:a11y".to_owned(),
            copy_export: copy_export(&[
                "row_identity",
                "blocked_capability",
                "still_safe_actions",
                "restriction_reason",
            ]),
            full_trust_claim: M5TrustRepairComponentClaim::FullTrustReviewedResult,
            claim_conditions: vec![condition(
                M5TrustRepairComponentClaimDimension::CapabilityNarrowClarity,
                M5TrustRepairComponentConditionState::CapabilityNarrowed,
            )],
            claim_narrow: Some(TrustRepairComponentClaimAutoNarrow {
                narrowed_to: M5TrustRepairComponentClaim::NarrowedCapabilityProjection,
                binding_dimension: M5TrustRepairComponentClaimDimension::CapabilityNarrowClarity,
                trigger: M5WorkspaceTrustRepairDowngradeTrigger::NarrowedCapabilityUnstated,
                narrowed_label:
                    "This capability is narrowed under restricted mode — shown as a narrowed-capability projection that names the blocked action family, the still-safe actions, and the restriction reason, never as a full-capability result"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "row_identity",
                "blocked_capability",
                "still_safe_actions",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkspaceTrustRepairConsumerSurface::SafeModeUi,
                M5WorkspaceTrustRepairConsumerSurface::ExtensionsUi,
            ]),
            source_refs: vec![
                "UX Design System restricted capability guidance".to_owned(),
                WORKSPACE_TRUST_REPAIR_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("restricted-capability-row-narrowed"),
        },
        // Root-trust strip (per-root trust mixed) — per-root trust is mixed, so the strip auto-narrows
        // to a mixed-root projection that names the per-root trust rather than collapsing it into
        // uniform blanket trust (yellow).
        TrustRepairComponentAccessibilityRow {
            record_kind: WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION,
            row_id: "a11y:root-trust-strip-mixed-root".to_owned(),
            component_family: M5WorkspaceTrustRepairComponentFamily::RootTrustStrip,
            source_family_schema_ref: WORKSPACE_TRUST_REPAIR_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            trust_context_ref: "trust:root-trust-strip:0005".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TrustRepairComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:root-trust-strip-mixed-root:a11y".to_owned(),
            copy_export: copy_export(&[
                "strip_identity",
                "per_root_trust",
                "grant_source",
                "narrowed_capability",
            ]),
            full_trust_claim: M5TrustRepairComponentClaim::FullTrustReviewedResult,
            claim_conditions: vec![condition(
                M5TrustRepairComponentClaimDimension::PerRootTrustClarity,
                M5TrustRepairComponentConditionState::PerRootTrustMixed,
            )],
            claim_narrow: Some(TrustRepairComponentClaimAutoNarrow {
                narrowed_to: M5TrustRepairComponentClaim::MixedRootProjection,
                binding_dimension: M5TrustRepairComponentClaimDimension::PerRootTrustClarity,
                trigger: M5WorkspaceTrustRepairDowngradeTrigger::MixedRootShownAsUniformTrust,
                narrowed_label:
                    "This workspace spans roots with differing trust — shown as a mixed-root projection that names each root's trust class and grant source, never collapsing mixed-root trust into uniform blanket trust"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "strip_identity",
                "per_root_trust",
                "grant_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkspaceTrustRepairConsumerSurface::WorkspaceTrustUi,
                M5WorkspaceTrustRepairConsumerSurface::RemoteUi,
            ]),
            source_refs: vec![
                "TAD workspace trust / root scope".to_owned(),
                WORKSPACE_TRUST_REPAIR_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("root-trust-strip-mixed-root"),
        },
        // Repair-transaction preview card (checkpoint missing) — a repair checkpoint is absent, so the
        // card auto-narrows to a missing-checkpoint projection that discloses the reversal limits
        // before apply, never a fully reversible result (yellow). A missing checkpoint is an honest
        // disclosed-absence operation, not a full-trust overstatement.
        TrustRepairComponentAccessibilityRow {
            record_kind: WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION,
            row_id: "a11y:repair-transaction-preview-card-missing-checkpoint".to_owned(),
            component_family:
                M5WorkspaceTrustRepairComponentFamily::RepairTransactionPreviewCard,
            source_family_schema_ref: WORKSPACE_TRUST_REPAIR_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            trust_context_ref: "trust:repair-transaction-preview-card:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TrustRepairComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:repair-transaction-preview-card-missing-checkpoint:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "card_identity",
                "repair_target_ids",
                "checkpoint_state",
                "reversal_limits_note",
            ]),
            full_trust_claim: M5TrustRepairComponentClaim::FullTrustReviewedResult,
            claim_conditions: vec![condition(
                M5TrustRepairComponentClaimDimension::RepairPreviewClarity,
                M5TrustRepairComponentConditionState::CheckpointMissing,
            )],
            claim_narrow: Some(TrustRepairComponentClaimAutoNarrow {
                narrowed_to: M5TrustRepairComponentClaim::MissingCheckpointProjection,
                binding_dimension: M5TrustRepairComponentClaimDimension::RepairPreviewClarity,
                trigger: M5WorkspaceTrustRepairDowngradeTrigger::CheckpointAbsenceHidden,
                narrowed_label:
                    "No checkpoint is available for this repair — shown as a missing-checkpoint projection that discloses the reversal limits and repair target ids before apply, never as a fully reversible transaction"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "card_identity",
                "repair_target_ids",
                "checkpoint_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkspaceTrustRepairConsumerSurface::DoctorUi,
                M5WorkspaceTrustRepairConsumerSurface::RemoteUi,
            ]),
            source_refs: vec![
                "TDD repair preview / rollback contract".to_owned(),
                WORKSPACE_TRUST_REPAIR_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("repair-transaction-preview-card-missing-checkpoint"),
        },
        // Rollback-class strip (reversal evidence partial) — the reversal evidence is only partial, so
        // the strip auto-narrows to an unproven-reversal projection that names the partial outcome and
        // reversal class, never collapsing it into a generic success (yellow).
        TrustRepairComponentAccessibilityRow {
            record_kind: WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION,
            row_id: "a11y:rollback-class-strip-unproven-reversal".to_owned(),
            component_family: M5WorkspaceTrustRepairComponentFamily::RollbackClassStrip,
            source_family_schema_ref: WORKSPACE_TRUST_REPAIR_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            trust_context_ref: "trust:rollback-class-strip:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach:
                TrustRepairComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TrustRepairComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:rollback-class-strip-unproven-reversal:a11y".to_owned(),
            copy_export: copy_export(&[
                "strip_identity",
                "reversal_class",
                "checkpoint_state",
                "partial_outcome_note",
            ]),
            full_trust_claim: M5TrustRepairComponentClaim::FullTrustReviewedResult,
            claim_conditions: vec![condition(
                M5TrustRepairComponentClaimDimension::ReversalClassClarity,
                M5TrustRepairComponentConditionState::ReversalEvidencePartial,
            )],
            claim_narrow: Some(TrustRepairComponentClaimAutoNarrow {
                narrowed_to: M5TrustRepairComponentClaim::UnprovenReversalProjection,
                binding_dimension: M5TrustRepairComponentClaimDimension::ReversalClassClarity,
                trigger: M5WorkspaceTrustRepairDowngradeTrigger::ReversalLimitHidden,
                narrowed_label:
                    "This reversal's evidence is only partial — shown as an unproven-reversal projection that names the reversal class and the partial outcome, never collapsing distinct exact / compensate / regenerate / manual / audit-only outcomes into a generic success"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "strip_identity",
                "reversal_class",
                "checkpoint_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkspaceTrustRepairConsumerSurface::DoctorUi,
                M5WorkspaceTrustRepairConsumerSurface::SafeModeUi,
            ]),
            source_refs: vec![
                "TAD repair-transaction architecture".to_owned(),
                WORKSPACE_TRUST_REPAIR_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("rollback-class-strip-unproven-reversal"),
        },
        // Repair-result receipt row (fully attributed) — the applied outcome, reversal class, and any
        // manual follow-up are fully stated, so it is a full-trust reviewed result reachable on every
        // surface with no narrowing (green).
        TrustRepairComponentAccessibilityRow {
            record_kind: WORKSPACE_TRUST_REPAIR_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_REPAIR_A11Y_SCHEMA_VERSION,
            row_id: "a11y:repair-result-receipt-row-attributed".to_owned(),
            component_family: M5WorkspaceTrustRepairComponentFamily::RepairResultReceiptRow,
            source_family_schema_ref: WORKSPACE_TRUST_REPAIR_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            trust_context_ref: "trust:repair-result-receipt-row:0008".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: TrustRepairComponentNonVisualReachState::ReachableAndLabeled,
            export_summary:
                TrustRepairComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:repair-result-receipt-row-attributed:a11y".to_owned(),
            copy_export: copy_export(&[
                "receipt_identity",
                "applied_outcome",
                "reversal_class",
                "manual_follow_up",
            ]),
            full_trust_claim: M5TrustRepairComponentClaim::FullTrustReviewedResult,
            claim_conditions: vec![condition(
                M5TrustRepairComponentClaimDimension::RepairOutcomeClarity,
                M5TrustRepairComponentConditionState::FullTrustReviewed,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "receipt_identity",
                "applied_outcome",
                "reversal_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5WorkspaceTrustRepairConsumerSurface::DoctorUi,
                M5WorkspaceTrustRepairConsumerSurface::RemoteUi,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 repair result receipt row".to_owned(),
                WORKSPACE_TRUST_REPAIR_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("repair-result-receipt-row-attributed"),
        },
    ]
}
