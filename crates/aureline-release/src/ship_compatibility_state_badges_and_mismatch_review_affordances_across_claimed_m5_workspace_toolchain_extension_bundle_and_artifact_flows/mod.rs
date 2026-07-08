//! One reusable M5 compatibility-state badge primitive: the parity a claimed artifact
//! actually has with the target it is about to be installed into, imported into, applied
//! to, or reopened in (Exact match / Compatible / Limited / Mismatch), projected the same
//! way across every claimed M5 workspace, toolchain, extension, workflow-bundle,
//! compare/review, and export consumer — as one distinct, composable cue that never
//! collapses into support class, lifecycle, or channel status and never softens a Limited
//! or Mismatch reading into a generic warning.
//!
//! Aureline's frozen badge-family matrix
//! ([`crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`])
//! names the compatibility-state badge as one of the six governed badge families and
//! freezes the shared badge infrastructure — the surface families, the deployment lines,
//! the accessibility routes, the qualification classes, the explanation-drawer fields, the
//! consumer surfaces, and the downgrade triggers. This module *implements* that family as
//! one render-facing badge so a user can tell — from the badge and its explanation and
//! reconciliation drawers alone — exactly which compatibility posture an artifact carries
//! *before* an install / import / apply / reopen flow proceeds, *and* — whenever the
//! posture is Limited or Mismatch — exactly what repair, compare, support-export, and
//! claim-narrowing detail that reading preserves, without the compatibility state
//! overstating parity, implying a support level, or collapsing into a generic warning.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_compatibility_state_badge`] — that takes one artifact's
//!    subject label, its declared compatibility state, an optional reconciliation-detail
//!    disclosure, and its last-evaluated timestamp, and produces one
//!    [`M5ResolvedCompatibilityStateBadge`] carrying the state as its own typed field, the
//!    derived compatibility posture (full parity / compatible within range / reduced
//!    capability / incompatible as claimed), and — whenever the state is Limited or
//!    Mismatch — a self-contained [`M5CompatibilityReconciliationNote`] that names the exact
//!    gap class, the residual capability, the repair action, the reconciliation detail, and
//!    the *preserved* state context. The resolver never collapses the state into support
//!    class, lifecycle, or channel, never implies the support class from the state, and
//!    never lets a Limited or Mismatch badge drop the reconciliation detail a reviewer needs
//!    to repair, compare, export, or narrow the claim.
//! 2. A parity matrix — [`M5CompatibilityStateBadgePrimitivePacket`] — that binds one row
//!    per claimed M5 badge consumer (the workspace-reopen card, the toolchain-install row,
//!    the extension-import row, the workflow-bundle-apply card, the compare/review panel,
//!    and the support-export row) to the shared badge anatomy, the same state values,
//!    compatibility postures, gap classes, residual capabilities, repair actions,
//!    explanation-drawer fields, export fields, and non-visual accessibility routes, so the
//!    compatibility-state vocabulary stays identical across install, import, apply, reopen,
//!    compare, and export surfaces.
//!
//! The badge surface family ([`M5BadgeSurfaceFamily`]), deployment line
//! ([`M5DeploymentLine`]), accessibility route ([`M5BadgeAccessibilityRoute`]),
//! qualification class ([`M5BadgeQualificationClass`]), explanation-drawer field
//! ([`M5BadgeExplanationField`]), consumer surface ([`M5BadgeConsumerSurface`]), and
//! downgrade trigger ([`M5BadgeDowngradeTrigger`]) are reused verbatim from the frozen
//! badge-family matrix. This module mints new vocabulary only for what that matrix left
//! implicit about the rendered compatibility-state badge itself: its render-facing value
//! set, its badge consumers, its anatomy parts, its compatibility postures, its gap
//! classes, its residual capabilities, its repair actions, and its export fields. No M5
//! badge surface invents a second compatibility-state grammar.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user text
//! bodies stay outside the support boundary; every subject label, reconciliation-detail
//! disclosure, and timestamp is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-compatibility-state-badge.schema.json`](../../../../schemas/ui/m5-compatibility-state-badge.schema.json)
//! and the contract doc is
//! [`docs/release/m5_compatibility_state_badge_contract.md`](../../../../docs/release/m5_compatibility_state_badge_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-compatibility-state-badges/`](../../../../fixtures/ui/m5-compatibility-state-badges/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_compatibility_state_badge_primitive_compare_review_panel_beta_narrowed,
    seeded_m5_compatibility_state_badge_primitive_packet,
    seeded_m5_compatibility_state_badge_primitive_support_export_row_preview_narrowed,
    M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_PACKET_ID,
};

// The surface families, deployment lines, accessibility routes, qualification classes,
// explanation-drawer fields, consumer surfaces, and downgrade triggers are frozen once,
// in the badge-family matrix. This primitive reuses them verbatim so it never invents a
// parallel badge grammar for the shared badge infrastructure.
pub use crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix::{
    M5BadgeAccessibilityRoute, M5BadgeConsumerSurface, M5BadgeDowngradeTrigger,
    M5BadgeExplanationField, M5BadgeQualificationClass, M5BadgeSurfaceFamily, M5DeploymentLine,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CompatibilityStateBadgePrimitivePacket`].
pub const M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_RECORD_KIND: &str =
    "ship_m5_compatibility_state_badges_and_mismatch_review_affordances_across_claimed_m5_workspace_toolchain_extension_bundle_and_artifact_flows";

/// Schema version for M5 compatibility-state badge records.
pub const M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the compatibility-state badge boundary schema.
pub const M5_COMPATIBILITY_STATE_BADGE_SCHEMA_REF: &str =
    "schemas/ui/m5-compatibility-state-badge.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_COMPATIBILITY_STATE_BADGE_DOC_REF: &str =
    "docs/release/m5_compatibility_state_badge_contract.md";

/// Repo-relative path of the frozen badge-family matrix this primitive narrows from.
pub const M5_COMPATIBILITY_STATE_BADGE_FAMILY_MATRIX_REF: &str =
    "schemas/ui/m5-badge-family-matrix.schema.json";

/// Repo-relative path of the repair-action card this primitive projects its repair
/// entrypoint from.
pub const M5_COMPATIBILITY_STATE_BADGE_REPAIR_REF: &str =
    "schemas/ui/m5-repair-action-card.schema.json";

/// Repo-relative path of the repair / compare preview row this primitive projects its
/// compare entrypoint from.
pub const M5_COMPATIBILITY_STATE_BADGE_COMPARE_REF: &str =
    "schemas/ui/m5-repair-preview-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_COMPATIBILITY_STATE_BADGE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-compatibility-state-badges";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COMPATIBILITY_STATE_BADGE_ARTIFACT_REF: &str =
    "artifacts/release/m5-compatibility-state-badge-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_COMPATIBILITY_STATE_BADGE_CSV_REF: &str =
    "artifacts/release/m5-compatibility-state-badge-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_COMPATIBILITY_STATE_BADGE_REPORT_REF: &str =
    "artifacts/components/m5-compatibility-state-badges.md";

/// One claimed M5 badge consumer that renders the shared compatibility-state badge. These
/// are the surfaces the implementation requirements name — the flows where an artifact is
/// installed, imported, applied, or reopened, plus the compare/review and export surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityStateConsumerSurface {
    /// A workspace / portable-state reopen card.
    WorkspaceReopenCard,
    /// A toolchain install row.
    ToolchainInstallRow,
    /// An extension import row.
    ExtensionImportRow,
    /// A workflow-bundle apply card.
    WorkflowBundleApplyCard,
    /// The compare / review panel.
    CompareReviewPanel,
    /// A support-export row.
    SupportExportRow,
}

impl M5CompatibilityStateConsumerSurface {
    /// Every claimed badge consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkspaceReopenCard,
        Self::ToolchainInstallRow,
        Self::ExtensionImportRow,
        Self::WorkflowBundleApplyCard,
        Self::CompareReviewPanel,
        Self::SupportExportRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceReopenCard => "workspace_reopen_card",
            Self::ToolchainInstallRow => "toolchain_install_row",
            Self::ExtensionImportRow => "extension_import_row",
            Self::WorkflowBundleApplyCard => "workflow_bundle_apply_card",
            Self::CompareReviewPanel => "compare_review_panel",
            Self::SupportExportRow => "support_export_row",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::WorkspaceReopenCard => "Workspace Reopen Card",
            Self::ToolchainInstallRow => "Toolchain Install Row",
            Self::ExtensionImportRow => "Extension Import Row",
            Self::WorkflowBundleApplyCard => "Workflow Bundle Apply Card",
            Self::CompareReviewPanel => "Compare / Review Panel",
            Self::SupportExportRow => "Support Export Row",
        }
    }
}

/// Controlled compatibility-state badge value — which parity an artifact carries with the
/// target it is about to be installed into, imported into, applied to, or reopened in.
/// This is the render-facing compatibility vocabulary the acceptance criteria name: Exact
/// match, Compatible, Limited, Mismatch. A compatibility-state badge never leaves its state
/// implicit and never implies a support level, lifecycle stage, or channel — a Mismatch
/// artifact is not "experimental" and a Limited artifact is not a silent exclusion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityStateBadgeValue {
    /// Exact match: the artifact matches the target exactly.
    ExactMatch,
    /// Compatible: the artifact is compatible within the supported range.
    Compatible,
    /// Limited: the artifact is compatible only across a reduced capability subset.
    Limited,
    /// Mismatch: the artifact does not match the claimed version or schema.
    Mismatch,
}

impl M5CompatibilityStateBadgeValue {
    /// Every compatibility-state value, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExactMatch,
        Self::Compatible,
        Self::Limited,
        Self::Mismatch,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactMatch => "exact_match",
            Self::Compatible => "compatible",
            Self::Limited => "limited",
            Self::Mismatch => "mismatch",
        }
    }

    /// Review-safe label for the badge and note.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExactMatch => "Exact match",
            Self::Compatible => "Compatible",
            Self::Limited => "Limited",
            Self::Mismatch => "Mismatch",
        }
    }
}

/// The derived compatibility posture — the resolver's verdict about how much parity an
/// artifact actually holds, computed from the compatibility state alone so it never implies
/// or is implied by the support class, lifecycle, or channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityPosture {
    /// Full parity: matches the target exactly, safe to proceed.
    FullParity,
    /// Compatible within range: compatible without reconciliation, safe to proceed.
    CompatibleWithinRange,
    /// Reduced capability: compatible only across a reduced capability subset.
    ReducedCapability,
    /// Incompatible as claimed: mismatches and must be reconciled before it applies.
    IncompatibleAsClaimed,
}

impl M5CompatibilityPosture {
    /// Every compatibility posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullParity,
        Self::CompatibleWithinRange,
        Self::ReducedCapability,
        Self::IncompatibleAsClaimed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::CompatibleWithinRange => "compatible_within_range",
            Self::ReducedCapability => "reduced_capability",
            Self::IncompatibleAsClaimed => "incompatible_as_claimed",
        }
    }

    /// True when the posture is full parity — the artifact matches exactly.
    pub const fn is_full_parity(self) -> bool {
        matches!(self, Self::FullParity)
    }

    /// True when the posture is compatible within the supported range without any
    /// reconciliation detail to preserve.
    pub const fn is_compatible_within_range(self) -> bool {
        matches!(self, Self::CompatibleWithinRange)
    }

    /// True when the posture is a clean parity claim (full parity or compatible within
    /// range) that carries no reconciliation detail.
    pub const fn is_parity_clean(self) -> bool {
        matches!(self, Self::FullParity | Self::CompatibleWithinRange)
    }

    /// True when the posture is Limited or Mismatch and must therefore preserve the repair,
    /// compare, support-export, and claim-narrowing detail rather than collapse into a
    /// generic warning.
    pub const fn requires_reconciliation(self) -> bool {
        !self.is_parity_clean()
    }

    /// True when the posture is a reduced-capability (Limited) reading.
    pub const fn is_reduced_capability(self) -> bool {
        matches!(self, Self::ReducedCapability)
    }

    /// True when the posture is a hard mismatch (Mismatch) reading.
    pub const fn is_hard_mismatch(self) -> bool {
        matches!(self, Self::IncompatibleAsClaimed)
    }

    /// The gap class this posture carries, if any. Returns `None` for a parity-clean
    /// posture, which carries no reconciliation detail.
    pub const fn gap_class(self) -> Option<M5CompatibilityGapClass> {
        Some(match self {
            Self::ReducedCapability => M5CompatibilityGapClass::CapabilitySubsetReduced,
            Self::IncompatibleAsClaimed => M5CompatibilityGapClass::VersionOrSchemaMismatch,
            Self::FullParity | Self::CompatibleWithinRange => return None,
        })
    }
}

/// The exact compatibility gap a Limited or Mismatch state carries, so a reconciliation
/// note never reads like an unqualified "safe to proceed" claim and never collapses two
/// different gaps into one generic warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityGapClass {
    /// Limited: compatible but only across a reduced capability subset.
    CapabilitySubsetReduced,
    /// Mismatch: the claimed version or schema does not match and must be reconciled.
    VersionOrSchemaMismatch,
}

impl M5CompatibilityGapClass {
    /// Every gap class, in declaration order.
    pub const ALL: [Self; 2] = [Self::CapabilitySubsetReduced, Self::VersionOrSchemaMismatch];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapabilitySubsetReduced => "capability_subset_reduced",
            Self::VersionOrSchemaMismatch => "version_or_schema_mismatch",
        }
    }

    /// Review-safe phrase naming exactly what the gap is, so the badge preserves enough
    /// detail for repair, compare, support export, and claim narrowing instead of a generic
    /// warning.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::CapabilitySubsetReduced => {
                "is compatible but only across a reduced capability subset"
            }
            Self::VersionOrSchemaMismatch => {
                "mismatches the claimed version or schema and must be reconciled before it applies"
            }
        }
    }

    /// True when this gap is a reduced-capability (Limited) gap rather than a hard
    /// mismatch.
    pub const fn is_reduced_capability_claim(self) -> bool {
        matches!(self, Self::CapabilitySubsetReduced)
    }

    /// True when this gap is a hard version / schema mismatch.
    pub const fn is_hard_mismatch(self) -> bool {
        matches!(self, Self::VersionOrSchemaMismatch)
    }

    /// The residual capability this gap preserves — what still works once the state has
    /// narrowed — so a Limited badge states its residual capability honestly and a Mismatch
    /// badge does not silently exclude the artifact.
    pub const fn residual_capability(self) -> M5CompatibilityResidualCapability {
        match self {
            Self::CapabilitySubsetReduced => {
                M5CompatibilityResidualCapability::ContinuesWithReducedScope
            }
            Self::VersionOrSchemaMismatch => {
                M5CompatibilityResidualCapability::BlockedUntilReconciled
            }
        }
    }

    /// The repair / compare action a reviewer should take to reconcile this gap.
    pub const fn repair_action(self) -> M5CompatibilityRepairAction {
        match self {
            Self::CapabilitySubsetReduced => {
                M5CompatibilityRepairAction::CompareAndReviewReducedScope
            }
            Self::VersionOrSchemaMismatch => M5CompatibilityRepairAction::RepairBeforeApply,
        }
    }
}

/// The residual capability a Limited or Mismatch state preserves — what keeps working (or
/// stays reconstructable) even though the artifact carries a compatibility gap, so a badge
/// states its residual capability honestly instead of overstating parity or silently
/// excluding the artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityResidualCapability {
    /// Continues with a reduced scope (Limited).
    ContinuesWithReducedScope,
    /// Blocked until the mismatch is reconciled (Mismatch) — visibly excluded, never
    /// silently dropped.
    BlockedUntilReconciled,
}

impl M5CompatibilityResidualCapability {
    /// Every residual capability, in declaration order.
    pub const ALL: [Self; 2] = [
        Self::ContinuesWithReducedScope,
        Self::BlockedUntilReconciled,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuesWithReducedScope => "continues_with_reduced_scope",
            Self::BlockedUntilReconciled => "blocked_until_reconciled",
        }
    }
}

/// The repair / compare action named on a reconciliation note, so a Limited or Mismatch
/// badge is actionable from the note itself — it offers a repair and compare entrypoint
/// rather than being an inert warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityRepairAction {
    /// Compare and review the reduced scope before proceeding (Limited).
    CompareAndReviewReducedScope,
    /// Repair the artifact before applying it (Mismatch).
    RepairBeforeApply,
}

impl M5CompatibilityRepairAction {
    /// Every repair action, in declaration order.
    pub const ALL: [Self; 2] = [Self::CompareAndReviewReducedScope, Self::RepairBeforeApply];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompareAndReviewReducedScope => "compare_and_review_reduced_scope",
            Self::RepairBeforeApply => "repair_before_apply",
        }
    }
}

/// One anatomy part the shared compatibility-state badge surfaces. The parts in
/// [`M5CompatibilityStateAnatomyPart::MANDATORY`] are required on every consumer so the
/// compatibility state stays a distinct cue with its own explanation and reconciliation
/// drawers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityStateAnatomyPart {
    /// The compatibility-state badge itself.
    StateBadge,
    /// The state explanation drawer.
    StateExplanationDrawer,
    /// The reconciliation / repair-and-compare detail drawer.
    ReconciliationDrawer,
    /// The separately-filterable filter keys for the compatibility axis.
    FilterKeys,
    /// The derived compatibility-posture note.
    CompatibilityPostureNote,
    /// The compare entrypoint (shown when the state is Limited or Mismatch).
    CompareEntrypoint,
    /// The repair entrypoint (shown when the state is Mismatch).
    RepairEntrypoint,
}

impl M5CompatibilityStateAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::StateBadge,
        Self::StateExplanationDrawer,
        Self::ReconciliationDrawer,
        Self::FilterKeys,
        Self::CompatibilityPostureNote,
        Self::CompareEntrypoint,
        Self::RepairEntrypoint,
    ];

    /// The anatomy parts every badge consumer must render: the badge, both drawers, and the
    /// compatibility-posture note.
    pub const MANDATORY: [Self; 4] = [
        Self::StateBadge,
        Self::StateExplanationDrawer,
        Self::ReconciliationDrawer,
        Self::CompatibilityPostureNote,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateBadge => "state_badge",
            Self::StateExplanationDrawer => "state_explanation_drawer",
            Self::ReconciliationDrawer => "reconciliation_drawer",
            Self::FilterKeys => "filter_keys",
            Self::CompatibilityPostureNote => "compatibility_posture_note",
            Self::CompareEntrypoint => "compare_entrypoint",
            Self::RepairEntrypoint => "repair_entrypoint",
        }
    }
}

/// A field the support / export packet carries so compatibility-state truth is
/// reconstructable from the shared model. The fields in
/// [`M5CompatibilityStateExportField::MANDATORY`] are required, and the state, the
/// reconciliation detail, and the residual capability are always carried as *separate*
/// fields so exported evidence never loses badge meaning or drops the reconciliation
/// detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityStateExportField {
    /// The compatibility-state value.
    State,
    /// The derived compatibility posture.
    CompatibilityPosture,
    /// The gap class (when the state is Limited or Mismatch).
    GapClass,
    /// The reconciliation-detail disclosure (when the state is Limited or Mismatch).
    ReconciliationDetail,
    /// The residual capability the state preserves.
    ResidualCapability,
    /// The state explanation.
    StateExplanation,
    /// The opaque last-evaluated timestamp.
    LastEvaluated,
    /// The repair / compare action.
    RepairAction,
    /// The separately-filterable filter keys.
    FilterKeys,
}

impl M5CompatibilityStateExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::State,
        Self::CompatibilityPosture,
        Self::GapClass,
        Self::ReconciliationDetail,
        Self::ResidualCapability,
        Self::StateExplanation,
        Self::LastEvaluated,
        Self::RepairAction,
        Self::FilterKeys,
    ];

    /// The export fields every badge export must carry: the compatibility axis, the
    /// posture, the reconciliation detail, and the residual capability so a Limited or
    /// Mismatch badge keeps its repair and compare detail in exported evidence.
    pub const MANDATORY: [Self; 4] = [
        Self::State,
        Self::CompatibilityPosture,
        Self::ReconciliationDetail,
        Self::ResidualCapability,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::State => "state",
            Self::CompatibilityPosture => "compatibility_posture",
            Self::GapClass => "gap_class",
            Self::ReconciliationDetail => "reconciliation_detail",
            Self::ResidualCapability => "residual_capability",
            Self::StateExplanation => "state_explanation",
            Self::LastEvaluated => "last_evaluated",
            Self::RepairAction => "repair_action",
            Self::FilterKeys => "filter_keys",
        }
    }
}

/// A self-contained reconciliation note: the exact gap class, the repair action, the
/// reconciliation-detail disclosure, the residual capability, and — the
/// implementation-requirement invariant — the *preserved* compatibility-state context, so a
/// Limited or Mismatch badge names exactly what differs and what a reviewer can do about it
/// instead of collapsing into a generic warning, and the state it was evaluated in is never
/// dropped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityReconciliationNote {
    /// The exact gap class the state carries.
    pub gap_class: M5CompatibilityGapClass,
    /// The repair / compare action a reviewer should take.
    pub repair_action: M5CompatibilityRepairAction,
    /// The opaque, export-safe reconciliation-detail disclosure.
    pub reconciliation_detail: String,
    /// The residual capability this state preserves.
    pub residual_capability: M5CompatibilityResidualCapability,
    /// The compatibility state the artifact was evaluated in, preserved as context. Always
    /// equals the resolved state.
    pub preserved_state: M5CompatibilityStateBadgeValue,
    /// True when this gap is a reduced-capability (Limited) reading.
    pub is_reduced_capability: bool,
    /// True when this gap is a hard version / schema mismatch (Mismatch).
    pub is_hard_mismatch: bool,
    /// A deterministic, self-contained headline naming the gap, the residual capability,
    /// the preserved state, and the repair action — never an unqualified "safe to proceed"
    /// claim and never implying a support class from the state.
    pub headline: String,
}

/// The full input to the compatibility-state badge resolver for one artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityStateBadgeInput {
    /// The opaque, export-safe subject label.
    pub subject_label: String,
    /// The declared compatibility state.
    pub state: M5CompatibilityStateBadgeValue,
    /// The opaque, export-safe reconciliation-detail disclosure. Required (non-empty)
    /// whenever the state is Limited or Mismatch.
    pub reconciliation_detail_repr: Option<String>,
    /// The opaque, export-safe last-evaluated representation.
    pub last_evaluated_repr: String,
}

/// The resolved compatibility-state truth for one artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCompatibilityStateBadge {
    /// The opaque subject label.
    pub subject_label: String,
    /// The compatibility state — carried as its own field, never merged with support class,
    /// lifecycle, or channel.
    pub state: M5CompatibilityStateBadgeValue,
    /// The derived compatibility posture, computed from the state alone.
    pub compatibility_posture: M5CompatibilityPosture,
    /// True when the state is full parity.
    pub is_full_parity: bool,
    /// True when the state is compatible within range.
    pub is_compatible_within_range: bool,
    /// True when the state is Limited or Mismatch and must disclose reconciliation detail.
    pub requires_reconciliation: bool,
    /// True when the state is a reduced-capability (Limited) reading.
    pub is_reduced_capability: bool,
    /// True when the state is a hard mismatch (Mismatch) reading.
    pub is_hard_mismatch: bool,
    /// The opaque last-evaluated representation.
    pub last_evaluated_repr: String,
    /// The reconciliation note, present whenever the state is Limited or Mismatch.
    pub reconciliation_note: Option<M5CompatibilityReconciliationNote>,
}

/// Errors returned by [`resolve_compatibility_state_badge`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5CompatibilityStateBadgeError {
    /// The subject label was empty.
    EmptySubjectLabel,
    /// The last-evaluated representation was empty.
    EmptyLastEvaluated,
    /// The state is Limited or Mismatch but no reconciliation-detail disclosure was
    /// supplied — the badge must never collapse a Limited or Mismatch reading into a
    /// generic warning by hiding its reconciliation detail.
    MissingReconciliationDetail,
    /// A subject label, reconciliation-detail disclosure, or timestamp carried forbidden
    /// material.
    ForbiddenBadgeMaterial,
}

impl M5CompatibilityStateBadgeError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySubjectLabel => "empty_subject_label",
            Self::EmptyLastEvaluated => "empty_last_evaluated",
            Self::MissingReconciliationDetail => "missing_reconciliation_detail",
            Self::ForbiddenBadgeMaterial => "forbidden_badge_material",
        }
    }
}

impl fmt::Display for M5CompatibilityStateBadgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "compatibility-state badge resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5CompatibilityStateBadgeError {}

/// Resolves one compatibility-state badge from its declared parity reading.
///
/// The compatibility state stays a distinct, composable cue. The derived compatibility
/// posture is computed from the state axis alone — an Exact-match artifact is full parity
/// regardless of its support class, lifecycle, or channel, because the state is never
/// derived from another axis and never implies one. When the state is Limited or Mismatch,
/// the resolver requires a reconciliation-detail disclosure and produces a self-contained
/// reconciliation note that *preserves* the state context and states the residual
/// capability and repair action honestly rather than collapsing into a generic warning — a
/// Limited or Mismatch badge is always reviewable, with the repair, compare, support-export,
/// and claim-narrowing detail intact before an install / import / apply / reopen flow
/// proceeds.
pub fn resolve_compatibility_state_badge(
    input: &M5CompatibilityStateBadgeInput,
) -> Result<M5ResolvedCompatibilityStateBadge, M5CompatibilityStateBadgeError> {
    if input.subject_label.trim().is_empty() {
        return Err(M5CompatibilityStateBadgeError::EmptySubjectLabel);
    }
    if input.last_evaluated_repr.trim().is_empty() {
        return Err(M5CompatibilityStateBadgeError::EmptyLastEvaluated);
    }
    let reconciliation_detail = input
        .reconciliation_detail_repr
        .as_deref()
        .map(str::trim)
        .unwrap_or("");
    if value_repr_is_forbidden(&input.subject_label)
        || value_repr_is_forbidden(&input.last_evaluated_repr)
        || value_repr_is_forbidden(reconciliation_detail)
    {
        return Err(M5CompatibilityStateBadgeError::ForbiddenBadgeMaterial);
    }

    let compatibility_posture = derive_compatibility_posture(input.state);
    let is_full_parity = compatibility_posture.is_full_parity();
    let is_compatible_within_range = compatibility_posture.is_compatible_within_range();
    let requires_reconciliation = compatibility_posture.requires_reconciliation();
    let is_reduced_capability = compatibility_posture.is_reduced_capability();
    let is_hard_mismatch = compatibility_posture.is_hard_mismatch();

    let reconciliation_note = match compatibility_posture.gap_class() {
        Some(class) => {
            if reconciliation_detail.is_empty() {
                return Err(M5CompatibilityStateBadgeError::MissingReconciliationDetail);
            }
            let repair_action = class.repair_action();
            let residual_capability = class.residual_capability();
            let headline = format!(
                "Compatibility state '{}': {} — residual capability: {}; reconciliation detail '{}'; state '{}' preserved; repair: {}",
                input.state.label(),
                class.phrase(),
                residual_capability.as_str(),
                reconciliation_detail,
                input.state.as_str(),
                repair_action.as_str()
            );
            Some(M5CompatibilityReconciliationNote {
                gap_class: class,
                repair_action,
                reconciliation_detail: reconciliation_detail.to_owned(),
                residual_capability,
                preserved_state: input.state,
                is_reduced_capability: class.is_reduced_capability_claim(),
                is_hard_mismatch: class.is_hard_mismatch(),
                headline,
            })
        }
        None => None,
    };

    Ok(M5ResolvedCompatibilityStateBadge {
        subject_label: input.subject_label.clone(),
        state: input.state,
        compatibility_posture,
        is_full_parity,
        is_compatible_within_range,
        requires_reconciliation,
        is_reduced_capability,
        is_hard_mismatch,
        last_evaluated_repr: input.last_evaluated_repr.clone(),
        reconciliation_note,
    })
}

/// Derives the compatibility posture from the compatibility state alone, so the state is
/// never derived from another badge axis and never implies one.
fn derive_compatibility_posture(state: M5CompatibilityStateBadgeValue) -> M5CompatibilityPosture {
    match state {
        M5CompatibilityStateBadgeValue::ExactMatch => M5CompatibilityPosture::FullParity,
        M5CompatibilityStateBadgeValue::Compatible => M5CompatibilityPosture::CompatibleWithinRange,
        M5CompatibilityStateBadgeValue::Limited => M5CompatibilityPosture::ReducedCapability,
        M5CompatibilityStateBadgeValue::Mismatch => M5CompatibilityPosture::IncompatibleAsClaimed,
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs compatibility-state truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityStateResolutionCase {
    /// The resolver input.
    pub input: M5CompatibilityStateBadgeInput,
    /// The resolved truth. Must equal `resolve_compatibility_state_badge(&input)`.
    pub resolved: M5ResolvedCompatibilityStateBadge,
}

impl M5CompatibilityStateResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5CompatibilityStateBadgeInput) -> Self {
        let resolved =
            resolve_compatibility_state_badge(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_compatibility_state_badge(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one badge consumer bound to the shared badge anatomy,
/// state values, compatibility postures, gap classes, residual capabilities, repair
/// actions, explanation-drawer fields, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityStateRow {
    /// Badge consumer family.
    pub consumer_surface: M5CompatibilityStateConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5BadgeQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable state summary.
    pub state_summary: String,
    /// Claimed M5 badge surface families that render / consume this badge.
    pub surface_families: Vec<M5BadgeSurfaceFamily>,
    /// Deployment lines this badge keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this consumer renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5CompatibilityStateAnatomyPart>,
    /// State values this consumer names.
    pub state_values: Vec<M5CompatibilityStateBadgeValue>,
    /// Compatibility postures this consumer distinguishes.
    pub compatibility_postures: Vec<M5CompatibilityPosture>,
    /// Gap classes this consumer names.
    pub gap_classes: Vec<M5CompatibilityGapClass>,
    /// Residual capabilities this consumer distinguishes.
    pub residual_capabilities: Vec<M5CompatibilityResidualCapability>,
    /// Repair / compare actions this consumer names.
    pub repair_actions: Vec<M5CompatibilityRepairAction>,
    /// Explanation-drawer fields this consumer opens (must include the mandatory
    /// [`M5BadgeExplanationField::MANDATORY`] fields).
    pub explanation_fields: Vec<M5BadgeExplanationField>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5CompatibilityStateExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5BadgeAccessibilityRoute>,
    /// Badge subsystems that consume this badge's projection.
    pub consumer_surfaces: Vec<M5BadgeConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5BadgeDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5CompatibilityStateResolutionCase>,
    /// Hard invariant: this consumer never collapses the compatibility state into support
    /// class, lifecycle, or channel status. MUST be `false`.
    pub collapses_state_into_support_lifecycle_or_channel: bool,
    /// Hard invariant: this consumer never implies the support class from the compatibility
    /// state. MUST be `false`.
    pub implies_support_class_from_compatibility_state: bool,
    /// Hard invariant: this consumer never drops the reconciliation detail when a state is
    /// Limited or Mismatch. MUST be `false`.
    pub drops_reconciliation_detail_on_mismatch: bool,
    /// Hard invariant: this consumer never lets exported evidence lose badge meaning. MUST
    /// be `false`.
    pub drops_badge_meaning_in_export: bool,
}

impl M5CompatibilityStateRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5CompatibilityStateAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5CompatibilityStateAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5CompatibilityStateExportField> =
            self.export_fields.iter().copied().collect();
        M5CompatibilityStateExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory explanation-drawer field.
    fn declares_mandatory_explanation_fields(&self) -> bool {
        let present: BTreeSet<M5BadgeExplanationField> =
            self.explanation_fields.iter().copied().collect();
        M5BadgeExplanationField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_state_into_support_lifecycle_or_channel
            && !self.implies_support_class_from_compatibility_state
            && !self.drops_reconciliation_detail_on_mismatch
            && !self.drops_badge_meaning_in_export
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityStateVocabularySet {
    /// Badge-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// State-value tokens.
    pub state_values: Vec<String>,
    /// Compatibility-posture tokens.
    pub compatibility_postures: Vec<String>,
    /// Gap-class tokens.
    pub gap_classes: Vec<String>,
    /// Residual-capability tokens.
    pub residual_capabilities: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Repair-action tokens.
    pub repair_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Explanation-field tokens (reused from the frozen matrix).
    pub explanation_fields: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
    /// Badge-consumer-subsystem tokens (reused from the frozen matrix).
    pub badge_consumer_surfaces: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5CompatibilityStateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5CompatibilityStateConsumerSurface::ALL, |v| v.as_str()),
            state_values: tokens(&M5CompatibilityStateBadgeValue::ALL, |v| v.as_str()),
            compatibility_postures: tokens(&M5CompatibilityPosture::ALL, |v| v.as_str()),
            gap_classes: tokens(&M5CompatibilityGapClass::ALL, |v| v.as_str()),
            residual_capabilities: tokens(&M5CompatibilityResidualCapability::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5CompatibilityStateAnatomyPart::ALL, |v| v.as_str()),
            repair_actions: tokens(&M5CompatibilityRepairAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5CompatibilityStateExportField::ALL, |v| v.as_str()),
            explanation_fields: tokens(&M5BadgeExplanationField::ALL, |v| v.as_str()),
            surface_families: tokens(&M5BadgeSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5DeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5BadgeAccessibilityRoute::ALL, |v| v.as_str()),
            badge_consumer_surfaces: tokens(&M5BadgeConsumerSurface::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5BadgeDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityStateGovernanceReview {
    /// The compatibility state is shown as one distinct, composable cue.
    pub compatibility_state_shown_as_distinct_cue: bool,
    /// The state is never collapsed into support class, lifecycle, or channel.
    pub state_never_collapsed_into_support_lifecycle_or_channel: bool,
    /// The compatibility state never implies the support class.
    pub compatibility_state_never_implies_support_class: bool,
    /// The compatibility state never implies the lifecycle.
    pub compatibility_state_never_implies_lifecycle: bool,
    /// The compatibility posture is presented explicitly before install / import / apply /
    /// reopen flows proceed.
    pub posture_presented_before_install_import_apply_reopen: bool,
    /// A Limited or Mismatch state automatically discloses its reconciliation detail.
    pub mismatch_auto_discloses_reconciliation_detail: bool,
    /// The reconciliation note preserves the underlying state context.
    pub reconciliation_note_preserves_state_context: bool,
    /// Limited and Mismatch states preserve repair, compare, support-export, and
    /// claim-narrowing detail rather than collapsing into a generic warning.
    pub limited_and_mismatch_preserve_repair_and_compare_detail: bool,
    /// Downgrade behavior is visible and never a silent exclusion.
    pub downgrade_behavior_is_visible_not_silent: bool,
    /// Every badge can open its explanation drawer.
    pub every_badge_opens_explanation_drawer: bool,
    /// Every badge is separately filterable.
    pub every_badge_is_separately_filterable: bool,
    /// Exported evidence keeps the state's meaning.
    pub exported_evidence_keeps_state_meaning: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityStateConsumerProjection {
    /// Install, import, apply, and reopen surfaces consume the shared badge.
    pub install_import_apply_reopen_surfaces_consume_shared_state_badge: bool,
    /// Compare / review and export surfaces consume the shared badge.
    pub compare_review_and_export_surfaces_consume_shared_state_badge: bool,
    /// The state filter reads a single canonical source.
    pub state_filter_reads_single_source: bool,
    /// The compatibility posture reads a single canonical source.
    pub compatibility_posture_reads_single_source: bool,
    /// Support / export reads a single canonical state-badge source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the compatibility-state badge primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityStateReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting badge audit.
    pub badge_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CompatibilityStateBadgePrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CompatibilityStateBadgePrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Badge rows.
    pub badge_rows: Vec<M5CompatibilityStateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CompatibilityStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CompatibilityStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompatibilityStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompatibilityStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CompatibilityStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 compatibility-state badge primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompatibilityStateBadgePrimitivePacket {
    /// Record kind; must equal [`M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Badge rows.
    pub badge_rows: Vec<M5CompatibilityStateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CompatibilityStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CompatibilityStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompatibilityStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompatibilityStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CompatibilityStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CompatibilityStateBadgePrimitivePacket {
    /// Builds an M5 compatibility-state badge primitive packet from stable-lane input.
    pub fn new(input: M5CompatibilityStateBadgePrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            badge_rows: input.badge_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 compatibility-state badge primitive invariants.
    pub fn validate(&self) -> Vec<M5CompatibilityStateBadgePrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_RECORD_KIND {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COMPATIBILITY_STATE_BADGE_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_badge_rows(self, &mut violations);
        validate_preflight_posture_disclosure_coverage(self, &mut violations);
        validate_repair_compare_detail_preservation_coverage(self, &mut violations);
        validate_limited_and_mismatch_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 compatibility state badge primitive packet serializes"),
        ) {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::RawMaterialInExport);
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
            .expect("m5 compatibility state badge primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per badge consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,state_values,compatibility_postures,gap_classes,residual_capabilities,repair_actions,export_fields,example_count\n",
        );
        for row in &self.badge_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.state_values, |v| v.as_str()),
                join_tokens(&row.compatibility_postures, |v| v.as_str()),
                join_tokens(&row.gap_classes, |v| v.as_str()),
                join_tokens(&row.residual_capabilities, |v| v.as_str()),
                join_tokens(&row.repair_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .badge_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Compatibility State Badge Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Badge consumers: {} ({} stable)\n",
            self.badge_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- State values: {}\n",
            self.vocabulary_set.state_values.join(", ")
        ));
        out.push_str(&format!(
            "- Compatibility postures: {}\n",
            self.vocabulary_set.compatibility_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Gap classes: {}\n",
            self.vocabulary_set.gap_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Badge consumers\n\n");
        for row in &self.badge_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - State: {}\n", row.state_summary));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                let note = match &case.resolved.reconciliation_note {
                    Some(note) => note.gap_class.as_str(),
                    None => "no_reconciliation_gap",
                };
                out.push_str(&format!(
                    "    - state `{}` → posture `{}` (gap `{}`)\n",
                    case.resolved.state.as_str(),
                    case.resolved.compatibility_posture.as_str(),
                    note
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 compatibility-state badge primitive
/// export.
#[derive(Debug)]
pub enum M5CompatibilityStateBadgePrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CompatibilityStateBadgePrimitiveViolation>),
}

impl fmt::Display for M5CompatibilityStateBadgePrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 compatibility state badge primitive export parse failed: {error}"
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
                    "m5 compatibility state badge primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CompatibilityStateBadgePrimitiveArtifactError {}

/// Validation failures emitted by [`M5CompatibilityStateBadgePrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CompatibilityStateBadgePrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required badge consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A badge row is incomplete.
    BadgeRowIncomplete,
    /// A badge row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A badge row declares no state values.
    StateValueMissing,
    /// A badge row declares no compatibility postures.
    CompatibilityPostureMissing,
    /// A badge row declares no gap classes.
    GapClassMissing,
    /// A badge row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A badge row omits one of the mandatory explanation-drawer fields.
    ExplanationDrawerIncomplete,
    /// A badge row declares no accessibility routes (or misses keyboard focus or
    /// non-color encoding).
    AccessibilityRouteMissing,
    /// A badge row declares no badge-consumer subsystems.
    ConsumerSurfacesMissing,
    /// A badge row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A badge row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A badge claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves the compatibility posture presented explicitly across the
    /// range before an install / import / apply / reopen flow proceeds — a parity-clean
    /// example and a Limited/Mismatch example both present.
    PreflightPostureDisclosureUnproven,
    /// No worked resolution proves a Limited or Mismatch state preserving its state context
    /// and disclosing its reconciliation detail, residual capability, and repair action.
    RepairCompareDetailPreservationUnproven,
    /// No worked resolution proves the Limited state and the Mismatch state as distinct,
    /// detail-preserving readings rather than one collapsed generic warning.
    LimitedAndMismatchCoverageUnproven,
    /// A badge row violates a hard invariant.
    BadgeInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CompatibilityStateBadgePrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::BadgeRowIncomplete => "badge_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::StateValueMissing => "state_value_missing",
            Self::CompatibilityPostureMissing => "compatibility_posture_missing",
            Self::GapClassMissing => "gap_class_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ExplanationDrawerIncomplete => "explanation_drawer_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::PreflightPostureDisclosureUnproven => "preflight_posture_disclosure_unproven",
            Self::RepairCompareDetailPreservationUnproven => {
                "repair_compare_detail_preservation_unproven"
            }
            Self::LimitedAndMismatchCoverageUnproven => "limited_and_mismatch_coverage_unproven",
            Self::BadgeInvariantViolated => "badge_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 compatibility-state badge primitive export.
pub fn current_stable_m5_compatibility_state_badge_primitive_export(
) -> Result<M5CompatibilityStateBadgePrimitivePacket, M5CompatibilityStateBadgePrimitiveArtifactError>
{
    let packet: M5CompatibilityStateBadgePrimitivePacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-compatibility-state-badge-proof/support_export.json"
        )))
        .map_err(M5CompatibilityStateBadgePrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CompatibilityStateBadgePrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
    violations: &mut Vec<M5CompatibilityStateBadgePrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COMPATIBILITY_STATE_BADGE_SCHEMA_REF,
        M5_COMPATIBILITY_STATE_BADGE_DOC_REF,
        M5_COMPATIBILITY_STATE_BADGE_FAMILY_MATRIX_REF,
        M5_COMPATIBILITY_STATE_BADGE_REPAIR_REF,
        M5_COMPATIBILITY_STATE_BADGE_COMPARE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
    violations: &mut Vec<M5CompatibilityStateBadgePrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5CompatibilityStateBadgePrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_badge_rows(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
    violations: &mut Vec<M5CompatibilityStateBadgePrimitiveViolation>,
) {
    let present: BTreeSet<M5CompatibilityStateConsumerSurface> = packet
        .badge_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5CompatibilityStateConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.badge_rows {
        if row.owner_role.trim().is_empty()
            || row.state_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.residual_capabilities.is_empty()
            || row.repair_actions.is_empty()
        {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::BadgeRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.state_values.is_empty() {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::StateValueMissing);
        }
        if row.compatibility_postures.is_empty() {
            violations
                .push(M5CompatibilityStateBadgePrimitiveViolation::CompatibilityPostureMissing);
        }
        if row.gap_classes.is_empty() {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::GapClassMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5CompatibilityStateBadgePrimitiveViolation::MandatoryExportFieldMissing);
        }
        if !row.declares_mandatory_explanation_fields() {
            violations
                .push(M5CompatibilityStateBadgePrimitiveViolation::ExplanationDrawerIncomplete);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5BadgeAccessibilityRoute::KeyboardFocusable)
            || !row
                .accessibility_routes
                .contains(&M5BadgeAccessibilityRoute::NonColorEncoded)
        {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5CompatibilityStateBadgePrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5CompatibilityStateBadgePrimitiveViolation::BadgeInvariantViolated);
        }
    }
}

/// AC1: at least one worked resolution must prove the compatibility posture presented
/// explicitly across the range before an install / import / apply / reopen flow proceeds — a
/// parity-clean example (Exact match or Compatible) *and* a Limited/Mismatch example both
/// present — so the posture is never hidden and never collapsed into a single
/// support/lifecycle/channel rank.
fn validate_preflight_posture_disclosure_coverage(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
    violations: &mut Vec<M5CompatibilityStateBadgePrimitiveViolation>,
) {
    let has_parity_clean = packet.badge_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_full_parity || case.resolved.is_compatible_within_range)
    });
    let has_reconciliation = packet.badge_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.requires_reconciliation)
    });
    if !(has_parity_clean && has_reconciliation) {
        violations
            .push(M5CompatibilityStateBadgePrimitiveViolation::PreflightPostureDisclosureUnproven);
    }
}

/// AC2: at least one worked resolution must prove a Limited or Mismatch state whose
/// reconciliation note discloses a non-empty reconciliation detail, preserves the underlying
/// state context, and states a repair action — the badge preserves enough detail for repair,
/// compare, support export, and claim narrowing rather than collapsing into a generic
/// warning.
fn validate_repair_compare_detail_preservation_coverage(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
    violations: &mut Vec<M5CompatibilityStateBadgePrimitiveViolation>,
) {
    let proven = packet.badge_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.requires_reconciliation
                && case
                    .resolved
                    .reconciliation_note
                    .as_ref()
                    .is_some_and(|note| {
                        !note.reconciliation_detail.trim().is_empty()
                            && note.preserved_state == case.resolved.state
                            && !note.headline.trim().is_empty()
                    })
        })
    });
    if !proven {
        violations.push(
            M5CompatibilityStateBadgePrimitiveViolation::RepairCompareDetailPreservationUnproven,
        );
    }
}

/// AC2: Limited and Mismatch must stay distinct, detail-preserving readings — never one
/// collapsed generic warning. At least one worked resolution must prove the Limited state
/// with a reconciliation note, and at least one must prove the Mismatch state with one.
fn validate_limited_and_mismatch_coverage(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
    violations: &mut Vec<M5CompatibilityStateBadgePrimitiveViolation>,
) {
    let has_limited = packet.badge_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_reduced_capability
                && case
                    .resolved
                    .reconciliation_note
                    .as_ref()
                    .is_some_and(|note| !note.reconciliation_detail.trim().is_empty())
        })
    });
    let has_mismatch = packet.badge_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_hard_mismatch
                && case
                    .resolved
                    .reconciliation_note
                    .as_ref()
                    .is_some_and(|note| !note.reconciliation_detail.trim().is_empty())
        })
    });
    if !(has_limited && has_mismatch) {
        violations
            .push(M5CompatibilityStateBadgePrimitiveViolation::LimitedAndMismatchCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
    violations: &mut Vec<M5CompatibilityStateBadgePrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.compatibility_state_shown_as_distinct_cue,
        review.state_never_collapsed_into_support_lifecycle_or_channel,
        review.compatibility_state_never_implies_support_class,
        review.compatibility_state_never_implies_lifecycle,
        review.posture_presented_before_install_import_apply_reopen,
        review.mismatch_auto_discloses_reconciliation_detail,
        review.reconciliation_note_preserves_state_context,
        review.limited_and_mismatch_preserve_repair_and_compare_detail,
        review.downgrade_behavior_is_visible_not_silent,
        review.every_badge_opens_explanation_drawer,
        review.every_badge_is_separately_filterable,
        review.exported_evidence_keeps_state_meaning,
        review.every_row_declares_accessibility_route,
    ] {
        if !ok {
            violations
                .push(M5CompatibilityStateBadgePrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
    violations: &mut Vec<M5CompatibilityStateBadgePrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.install_import_apply_reopen_surfaces_consume_shared_state_badge,
        projection.compare_review_and_export_surfaces_consume_shared_state_badge,
        projection.state_filter_reads_single_source,
        projection.compatibility_posture_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5CompatibilityStateBadgePrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
    violations: &mut Vec<M5CompatibilityStateBadgePrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CompatibilityStateBadgePrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CompatibilityStateBadgePrimitivePacket,
    violations: &mut Vec<M5CompatibilityStateBadgePrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.badge_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CompatibilityStateBadgePrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
