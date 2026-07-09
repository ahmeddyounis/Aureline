//! Keyboard, screen-reader, CLI, and export parity plus automatic claim narrowing
//! for the eight shared M5 package-management components.
//!
//! This module is the accessibility / headless / export capstone over the
//! package-management components frozen in
//! [`crate::freeze_the_m5_package_management_component_matrix`], implemented by the
//! package-explorer-row, manifest-scope / registry-or-mirror, install-review /
//! lockfile-impact, and script-risk / grouped-update / rollback-checkpoint lanes, and
//! adopted by the shared consumers in
//! [`crate::add_shared_package_explorer_search_detail_help_support_diagnostics_and_export_consumers_so_package_components_keep_scope_auth_and_lockfile_language_aligned`].
//! Where the consumer lane proves scope / auth / lockfile parity across desktop
//! surfaces, this lane proves the harder claim: that manifest scope, registry source /
//! auth posture, script / native-build side-effect class, lockfile churn, and
//! rollback / checkpoint truth is exposed just as honestly in assistive, headless, and
//! exported forms as it is on the desktop — and that a claim-bearing component
//! automatically narrows the moment its manifest scope, registry freshness, auth state,
//! lockfile-impact computation, or rollback / checkpoint truth stops being trustworthy.
//!
//! The honesty axes are two. First, parity across forms: every claimed component must
//! expose a keyboard label, a screen-reader label, a CLI enum token, an export enum
//! token, and a human-readable explanation field, and must render on the desktop, the
//! headless CLI, and the support export alike. No component may be pointer-only,
//! export-opaque, or semantically stronger on the desktop than it is in CLI or support
//! output.
//!
//! Second, automatic narrowing: each component carries a claim about how much
//! reviewable package-management capability it asserts, drawn from
//! [`PackageComponentClaimTier`]. When manifest scope is only partially known, when
//! registry freshness is stale (mirror- or offline-sourced), when auth state is
//! unsatisfied, when lockfile-impact computation is unavailable, or when the
//! rollback / checkpoint truth is unavailable or policy-blocked, the claim must narrow
//! to the ceiling permitted by that condition
//! ([`PackageComponentClaimCondition::permitted_ceiling`]), disclose the narrowing
//! through an explicit trigger and next action, keep the manifest scope and side-effect
//! class explicit, keep mirror / offline continuity explicit, keep the registry auth
//! posture explicit, and keep the rollback / checkpoint truth explicit before any write.
//! A component may never keep asserting full reviewable management while one of those
//! conditions holds.
//!
//! The packet references upstream component and consumer contracts by id rather than
//! embedding their content. Raw manifests, lockfiles, credentials, and live registry
//! responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-package-management-component-accessibility-parity.schema.json`](../../../../schemas/ui/m5-package-management-component-accessibility-parity.schema.json).
//! The contract doc is
//! [`docs/deps/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_manifest_scope_registry_freshness_auth_state_or_lockfile_impact_truth_is_stale_or_partial_across_claimed_m5_package_components.md`](../../../../docs/deps/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_manifest_scope_registry_freshness_auth_state_or_lockfile_impact_truth_is_stale_or_partial_across_claimed_m5_package_components.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-package-management-component-accessibility-parity/`](../../../../fixtures/ui/m5-package-management-component-accessibility-parity/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_management_component_matrix::M5PackageComponent;

/// Stable record-kind tag carried by [`PackageComponentAccessibilityPacket`].
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_RECORD_KIND: &str =
    "package_management_component_accessibility_parity_truth";

/// Schema version for package-management accessibility parity records.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCHEMA_REF: &str =
    "schemas/ui/m5-package-management-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_DOC_REF: &str =
    "docs/deps/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_manifest_scope_registry_freshness_auth_state_or_lockfile_impact_truth_is_stale_or_partial_across_claimed_m5_package_components.md";

/// Repo-relative path of the frozen component matrix these claims exercise.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-package-management-component-matrix.schema.json";

/// Repo-relative path of the shared-consumer parity contract this capstone extends.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_CONSUMER_CONTRACT_REF: &str =
    "schemas/ui/m5-package-management-component-consumer.schema.json";

/// Repo-relative path of the package-explorer-row controls contract.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_EXPLORER_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-package-explorer-row.schema.json";

/// Repo-relative path of the manifest-scope / registry-or-mirror controls contract.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF: &str =
    "schemas/ui/m5-manifest-scope-registry-controls.schema.json";

/// Repo-relative path of the install-review / lockfile-impact controls contract.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF: &str =
    "schemas/ui/m5-install-review-lockfile-controls.schema.json";

/// Repo-relative path of the script-risk / grouped-update / rollback controls contract.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF:
    &str = "schemas/ui/m5-script-risk-grouped-update-rollback-controls.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-package-management-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-package-management-accessibility-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_PACKAGE_COMPONENT_ACCESSIBILITY_SUMMARY_REF: &str =
    "artifacts/release/m5-package-management-accessibility-proof/summary.md";

/// Canonical component contract that a row must point at for a given component.
///
/// Each of the eight shared components resolves to the checked-in schema of the
/// implement lane that produced it: the package-explorer-row controls, the
/// manifest-scope / registry-or-mirror controls, the install-review / lockfile-impact
/// controls, and the script-risk / grouped-update / rollback-checkpoint controls.
pub const fn component_canonical_schema_ref(component: M5PackageComponent) -> &'static str {
    match component {
        M5PackageComponent::PackageExplorerRow => {
            M5_PACKAGE_COMPONENT_ACCESSIBILITY_EXPLORER_ROW_CONTRACT_REF
        }
        M5PackageComponent::ManifestScopeSwitcher | M5PackageComponent::RegistryOrMirrorRow => {
            M5_PACKAGE_COMPONENT_ACCESSIBILITY_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF
        }
        M5PackageComponent::InstallReviewSheet | M5PackageComponent::LockfileImpactCard => {
            M5_PACKAGE_COMPONENT_ACCESSIBILITY_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF
        }
        M5PackageComponent::ScriptRiskNotice
        | M5PackageComponent::GroupedUpdatePlanner
        | M5PackageComponent::RollbackCheckpointStrip => {
            M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF
        }
    }
}

/// The condition governing how much reviewable package-management capability a
/// component may claim.
///
/// [`PackageTruthTrusted`](Self::PackageTruthTrusted) is the baseline where the full
/// reviewable-management claim is permitted. The other five are the weakening
/// conditions named by the spec: a partially known manifest scope, stale registry
/// freshness, an unsatisfied auth state, an unavailable lockfile-impact computation,
/// and an unavailable or policy-blocked rollback / checkpoint truth. Each weakening
/// condition pins the claim to a ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentClaimCondition {
    /// Manifest scope, registry freshness, auth state, lockfile impact, and rollback
    /// truth are all trusted.
    PackageTruthTrusted,
    /// The target manifest scope is only partially known; the exact scope is not pinned.
    ManifestScopePartial,
    /// Registry freshness is stale; the answer is mirror- or offline-sourced.
    RegistryFreshnessStale,
    /// Registry access requires authentication that is not satisfied.
    AuthStateUnsatisfied,
    /// The lockfile-impact / churn computation is unavailable or stale.
    LockfileImpactUnavailable,
    /// The rollback / checkpoint truth is unavailable or policy-blocked.
    RollbackCheckpointUnavailable,
}

impl PackageComponentClaimCondition {
    /// Every condition, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PackageTruthTrusted,
        Self::ManifestScopePartial,
        Self::RegistryFreshnessStale,
        Self::AuthStateUnsatisfied,
        Self::LockfileImpactUnavailable,
        Self::RollbackCheckpointUnavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageTruthTrusted => "package_truth_trusted",
            Self::ManifestScopePartial => "manifest_scope_partial",
            Self::RegistryFreshnessStale => "registry_freshness_stale",
            Self::AuthStateUnsatisfied => "auth_state_unsatisfied",
            Self::LockfileImpactUnavailable => "lockfile_impact_unavailable",
            Self::RollbackCheckpointUnavailable => "rollback_checkpoint_unavailable",
        }
    }

    /// Whether this condition weakens the reviewable-management claim (everything but trusted).
    pub const fn is_weakening(self) -> bool {
        !matches!(self, Self::PackageTruthTrusted)
    }

    /// The strongest claim tier this condition still permits.
    pub const fn permitted_ceiling(self) -> PackageComponentClaimTier {
        match self {
            Self::PackageTruthTrusted => PackageComponentClaimTier::FullReviewableManagement,
            Self::ManifestScopePartial => PackageComponentClaimTier::ManifestRangeScoped,
            Self::RegistryFreshnessStale => PackageComponentClaimTier::MirrorOrOfflineSourced,
            Self::AuthStateUnsatisfied => PackageComponentClaimTier::AuthRequiredReadOnly,
            Self::LockfileImpactUnavailable => PackageComponentClaimTier::LockfileImpactUnknown,
            Self::RollbackCheckpointUnavailable => {
                PackageComponentClaimTier::RollbackUnavailableManualRecovery
            }
        }
    }

    /// The downgrade trigger a weakening condition must disclose, if any.
    pub const fn default_trigger(self) -> Option<PackageComponentAccessibilityDowngradeTrigger> {
        match self {
            Self::PackageTruthTrusted => None,
            Self::ManifestScopePartial => {
                Some(PackageComponentAccessibilityDowngradeTrigger::ManifestScopePartial)
            }
            Self::RegistryFreshnessStale => {
                Some(PackageComponentAccessibilityDowngradeTrigger::RegistryFreshnessStale)
            }
            Self::AuthStateUnsatisfied => {
                Some(PackageComponentAccessibilityDowngradeTrigger::AuthStateUnsatisfied)
            }
            Self::LockfileImpactUnavailable => {
                Some(PackageComponentAccessibilityDowngradeTrigger::LockfileImpactUnavailable)
            }
            Self::RollbackCheckpointUnavailable => {
                Some(PackageComponentAccessibilityDowngradeTrigger::RollbackCheckpointUnavailable)
            }
        }
    }

    /// The next action a condition's disclosure must offer.
    pub const fn next_action(self) -> PackageComponentClaimNextAction {
        match self {
            Self::PackageTruthTrusted => {
                PackageComponentClaimNextAction::ContinueReviewedManagement
            }
            Self::ManifestScopePartial => PackageComponentClaimNextAction::ConfirmManifestScope,
            Self::RegistryFreshnessStale => {
                PackageComponentClaimNextAction::ReviewMirrorOrOfflineContinuity
            }
            Self::AuthStateUnsatisfied => PackageComponentClaimNextAction::AuthenticateToRegistry,
            Self::LockfileImpactUnavailable => {
                PackageComponentClaimNextAction::RecomputeLockfileImpact
            }
            Self::RollbackCheckpointUnavailable => {
                PackageComponentClaimNextAction::EstablishRollbackCheckpoint
            }
        }
    }
}

/// A component's claim about how much reviewable package-management capability it asserts.
///
/// Ordered strongest to weakest.
/// [`FullReviewableManagement`](Self::FullReviewableManagement) is the only tier that
/// asserts fully-scoped, registry-fresh, authed, lockfile-quantified, and
/// rollback-safe management; the rest are the honest fallbacks a weakening condition
/// narrows to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentClaimTier {
    /// Full first-class integrated management: scoped, registry-fresh, authed,
    /// lockfile-quantified, and rollback-safe.
    FullReviewableManagement,
    /// Only the manifest range is known; the exact scope is not pinned.
    ManifestRangeScoped,
    /// The answer is mirror- or offline-sourced rather than upstream-fresh.
    MirrorOrOfflineSourced,
    /// Read-only: registry authentication is required and not satisfied.
    AuthRequiredReadOnly,
    /// Lockfile churn cannot be quantified; no safe reviewable mutation is claimed.
    LockfileImpactUnknown,
    /// No durable rollback checkpoint exists; only manual recovery is available.
    RollbackUnavailableManualRecovery,
}

impl PackageComponentClaimTier {
    /// Every tier, in declaration order (strongest first).
    pub const ALL: [Self; 6] = [
        Self::FullReviewableManagement,
        Self::ManifestRangeScoped,
        Self::MirrorOrOfflineSourced,
        Self::AuthRequiredReadOnly,
        Self::LockfileImpactUnknown,
        Self::RollbackUnavailableManualRecovery,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullReviewableManagement => "full_reviewable_management",
            Self::ManifestRangeScoped => "manifest_range_scoped",
            Self::MirrorOrOfflineSourced => "mirror_or_offline_sourced",
            Self::AuthRequiredReadOnly => "auth_required_read_only",
            Self::LockfileImpactUnknown => "lockfile_impact_unknown",
            Self::RollbackUnavailableManualRecovery => "rollback_unavailable_manual_recovery",
        }
    }

    /// Strength rank, higher is stronger. Used for the ceiling comparison.
    pub const fn rank(self) -> u8 {
        match self {
            Self::FullReviewableManagement => 6,
            Self::ManifestRangeScoped => 5,
            Self::MirrorOrOfflineSourced => 4,
            Self::AuthRequiredReadOnly => 3,
            Self::LockfileImpactUnknown => 2,
            Self::RollbackUnavailableManualRecovery => 1,
        }
    }

    /// Whether this tier asserts full first-class reviewable management.
    pub const fn asserts_full_reviewable_management(self) -> bool {
        matches!(self, Self::FullReviewableManagement)
    }
}

/// A rendering form the claim must reach with identical semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentRenderingSurface {
    /// The full desktop surface.
    DesktopFull,
    /// The headless CLI.
    CliHeadless,
    /// The support export.
    SupportExport,
}

impl PackageComponentRenderingSurface {
    /// Every rendering surface, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopFull, Self::CliHeadless, Self::SupportExport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The next action a narrow disclosure offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentClaimNextAction {
    /// Continue the reviewed package-management flow.
    ContinueReviewedManagement,
    /// Confirm the exact target manifest scope before mutating.
    ConfirmManifestScope,
    /// Review the mirror / offline continuity before relying on the answer.
    ReviewMirrorOrOfflineContinuity,
    /// Authenticate to the registry before completing the operation.
    AuthenticateToRegistry,
    /// Recompute the lockfile impact before claiming a safe mutation.
    RecomputeLockfileImpact,
    /// Establish a durable rollback checkpoint before any write.
    EstablishRollbackCheckpoint,
}

impl PackageComponentClaimNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinueReviewedManagement => "continue_reviewed_management",
            Self::ConfirmManifestScope => "confirm_manifest_scope",
            Self::ReviewMirrorOrOfflineContinuity => "review_mirror_or_offline_continuity",
            Self::AuthenticateToRegistry => "authenticate_to_registry",
            Self::RecomputeLockfileImpact => "recompute_lockfile_impact",
            Self::EstablishRollbackCheckpoint => "establish_rollback_checkpoint",
        }
    }
}

/// Downgrade trigger that can narrow this accessibility lane below its full claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageComponentAccessibilityDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The target manifest scope is only partially known.
    ManifestScopePartial,
    /// Registry freshness is stale; the answer is mirror- or offline-sourced.
    RegistryFreshnessStale,
    /// Registry access requires authentication that is not satisfied.
    AuthStateUnsatisfied,
    /// The lockfile-impact / churn computation is unavailable or stale.
    LockfileImpactUnavailable,
    /// The rollback / checkpoint truth is unavailable or policy-blocked.
    RollbackCheckpointUnavailable,
    /// A claim was overstated relative to its permitted ceiling.
    ClaimOverstated,
    /// Parity across desktop, CLI, or export was dropped.
    ParityDropped,
    /// Consumer trust narrowed.
    TrustNarrowing,
}

impl PackageComponentAccessibilityDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ManifestScopePartial,
        Self::RegistryFreshnessStale,
        Self::AuthStateUnsatisfied,
        Self::LockfileImpactUnavailable,
        Self::RollbackCheckpointUnavailable,
        Self::ClaimOverstated,
        Self::ParityDropped,
        Self::TrustNarrowing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ManifestScopePartial => "manifest_scope_partial",
            Self::RegistryFreshnessStale => "registry_freshness_stale",
            Self::AuthStateUnsatisfied => "auth_state_unsatisfied",
            Self::LockfileImpactUnavailable => "lockfile_impact_unavailable",
            Self::RollbackCheckpointUnavailable => "rollback_checkpoint_unavailable",
            Self::ClaimOverstated => "claim_overstated",
            Self::ParityDropped => "parity_dropped",
            Self::TrustNarrowing => "trust_narrowing",
        }
    }
}

/// The disclosures an accessibility row must carry, derived from its condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageComponentClaimResolution {
    /// The strongest claim tier the condition permits.
    pub permitted_ceiling: PackageComponentClaimTier,
    /// Whether the condition requires an explicit narrow disclosure.
    pub requires_narrowing: bool,
    /// The downgrade trigger the narrow disclosure must name, if any.
    pub expected_trigger: Option<PackageComponentAccessibilityDowngradeTrigger>,
    /// The next action the narrow disclosure must offer.
    pub expected_next_action: PackageComponentClaimNextAction,
    /// Whether the row must keep manifest scope and side-effect class explicit.
    pub needs_scope_disclosure_note: bool,
    /// Whether the row must carry an explicit mirror / offline continuity note.
    pub needs_continuity_note: bool,
    /// Whether the row must carry an explicit registry-auth note.
    pub needs_auth_note: bool,
    /// Whether the row must carry an explicit rollback / checkpoint note before write.
    pub needs_rollback_note: bool,
}

/// Resolves the claim narrowing an accessibility row must carry from its condition.
///
/// Trusted package truth keeps the full reviewable-management claim. Each weakening
/// condition pins the claim to a ceiling, demands an explicit narrow disclosure naming
/// its trigger and next action, and keeps the manifest scope and side-effect class
/// explicit so no generic manage-package or one-click language can hide it. Stale
/// registry freshness additionally demands an explicit mirror / offline continuity
/// note, an unsatisfied auth state demands an explicit registry-auth note rather than
/// silently presenting the operation as completable, and an unavailable rollback /
/// checkpoint truth demands an explicit rollback note kept visible before any write.
pub const fn resolve_package_component_claim_narrowing(
    condition: PackageComponentClaimCondition,
) -> PackageComponentClaimResolution {
    PackageComponentClaimResolution {
        permitted_ceiling: condition.permitted_ceiling(),
        requires_narrowing: condition.is_weakening(),
        expected_trigger: condition.default_trigger(),
        expected_next_action: condition.next_action(),
        needs_scope_disclosure_note: condition.is_weakening(),
        needs_continuity_note: matches!(
            condition,
            PackageComponentClaimCondition::RegistryFreshnessStale
        ),
        needs_auth_note: matches!(
            condition,
            PackageComponentClaimCondition::AuthStateUnsatisfied
        ),
        needs_rollback_note: matches!(
            condition,
            PackageComponentClaimCondition::RollbackCheckpointUnavailable
        ),
    }
}

/// The explicit narrow disclosure a claim-narrowed row shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentClaimNarrowing {
    /// The downgrade trigger the narrowing discloses.
    pub trigger: PackageComponentAccessibilityDowngradeTrigger,
    /// The claim tier the narrowing pins the component to.
    pub narrowed_to: PackageComponentClaimTier,
    /// Note naming the truth preserved through the narrowing (never omitted).
    pub preserved_truth_note: String,
    /// The next action offered.
    pub next_action: PackageComponentClaimNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// One accessibility row: a claimed component under one condition, exposed across
/// keyboard, screen-reader, CLI, and export forms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentAccessibilityRow {
    /// Stable row id.
    pub row_id: String,
    /// Which shared component this row claims.
    pub component: M5PackageComponent,
    /// The condition governing the claim.
    pub condition: PackageComponentClaimCondition,
    /// The claim tier the component effectively asserts.
    pub effective_claim: PackageComponentClaimTier,
    /// Keyboard reach / operation label (never empty).
    pub keyboard_label: String,
    /// Screen-reader label (never empty).
    pub screen_reader_label: String,
    /// CLI enum token (never empty).
    pub cli_enum_token: String,
    /// Export enum token (never empty).
    pub export_enum_token: String,
    /// Human-readable explanation field (never empty).
    pub explanation_field: String,
    /// The rendering surfaces this row reaches (must cover all three).
    pub rendering_surfaces: Vec<PackageComponentRenderingSurface>,
    /// The explicit narrow disclosure; required and complete when the claim narrows.
    pub narrowing: Option<PackageComponentClaimNarrowing>,
    /// Manifest-scope / side-effect note; required and non-empty when the claim narrows.
    pub scope_disclosure_note: String,
    /// Mirror / offline continuity note; required and non-empty when registry freshness is stale.
    pub continuity_note: String,
    /// Registry-auth note; required and non-empty when auth state is unsatisfied.
    pub auth_note: String,
    /// Rollback / checkpoint note; required and non-empty when rollback truth is unavailable.
    pub rollback_note: String,
    /// Guardrail: this component is reachable only by pointer.
    pub is_pointer_only: bool,
    /// Guardrail: this component omits itself from the export.
    pub is_export_opaque: bool,
    /// Guardrail: this component claims more on the desktop than in CLI or export.
    pub desktop_stronger_than_cli: bool,
    /// Source contract refs this row points at.
    pub source_contract_refs: Vec<String>,
}

impl PackageComponentAccessibilityRow {
    /// The disclosures this row must carry, derived from its condition.
    pub const fn resolution(&self) -> PackageComponentClaimResolution {
        resolve_package_component_claim_narrowing(self.condition)
    }

    /// Whether this row narrows below the full reviewable-management claim.
    pub const fn is_narrowed(&self) -> bool {
        self.condition.is_weakening()
    }

    /// Whether this row reaches all three rendering surfaces.
    pub fn covers_all_rendering_surfaces(&self) -> bool {
        PackageComponentRenderingSurface::ALL
            .iter()
            .all(|surface| self.rendering_surfaces.contains(surface))
    }

    /// Whether every accessibility field is present.
    pub fn accessibility_fields_present(&self) -> bool {
        !self.keyboard_label.trim().is_empty()
            && !self.screen_reader_label.trim().is_empty()
            && !self.cli_enum_token.trim().is_empty()
            && !self.export_enum_token.trim().is_empty()
            && !self.explanation_field.trim().is_empty()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.is_pointer_only && !self.is_export_opaque && !self.desktop_stronger_than_cli
    }

    /// Whether this row points at the canonical component schema and matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = component_canonical_schema_ref(self.component);
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self.source_contract_refs.iter().any(|reference| {
                reference == M5_PACKAGE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF
            })
    }

    /// Whether the effective claim is honest under the row's condition: it never
    /// exceeds the permitted ceiling, and a weakening condition narrows the claim
    /// down to exactly that ceiling.
    pub fn claim_is_honest(&self) -> bool {
        let resolution = self.resolution();
        let ceiling = resolution.permitted_ceiling;
        if self.effective_claim.rank() > ceiling.rank() {
            return false;
        }
        if resolution.requires_narrowing {
            self.effective_claim == ceiling
                && self
                    .narrowing
                    .as_ref()
                    .is_some_and(|narrowing| narrowing.narrowed_to == ceiling)
        } else {
            self.effective_claim == ceiling && self.narrowing.is_none()
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentAccessibilityTrustReview {
    /// Every claim is keyboard-reachable.
    pub keyboard_reachable_on_every_claim: bool,
    /// Every claim carries a screen-reader label.
    pub screen_reader_labeled_on_every_claim: bool,
    /// Every claim exposes a CLI enum token.
    pub cli_enum_exposed_on_every_claim: bool,
    /// Every claim exposes an export enum token.
    pub export_enum_exposed_on_every_claim: bool,
    /// Every claim carries an explanation field.
    pub explanation_field_present_on_every_claim: bool,
    /// No component is pointer-only.
    pub no_component_pointer_only: bool,
    /// No component is export-opaque.
    pub no_component_export_opaque: bool,
    /// No component claims more on the desktop than in CLI or export.
    pub desktop_never_stronger_than_cli: bool,
    /// The claim narrows whenever package truth weakens.
    pub claim_narrows_when_package_truth_weakens: bool,
    /// Manifest scope and side-effect class are never overstated while a weakening condition holds.
    pub scope_and_side_effect_never_overstated_under_weakening: bool,
    /// Mirror / offline continuity is kept explicit when registry freshness is stale.
    pub mirror_offline_continuity_kept_explicit: bool,
    /// Rollback / checkpoint truth is kept explicit before any write.
    pub rollback_checkpoint_truth_kept_explicit_before_write: bool,
}

impl PackageComponentAccessibilityTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.keyboard_reachable_on_every_claim
            && self.screen_reader_labeled_on_every_claim
            && self.cli_enum_exposed_on_every_claim
            && self.export_enum_exposed_on_every_claim
            && self.explanation_field_present_on_every_claim
            && self.no_component_pointer_only
            && self.no_component_export_opaque
            && self.desktop_never_stronger_than_cli
            && self.claim_narrows_when_package_truth_weakens
            && self.scope_and_side_effect_never_overstated_under_weakening
            && self.mirror_offline_continuity_kept_explicit
            && self.rollback_checkpoint_truth_kept_explicit_before_write
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentAccessibilityProjection {
    /// Keyboard and screen-reader labels are exposed.
    pub exposes_keyboard_and_screen_reader_labels: bool,
    /// CLI and export enums are exposed.
    pub exposes_cli_and_export_enums: bool,
    /// Explanation fields are exposed.
    pub exposes_explanation_fields: bool,
    /// The claim auto-narrows when the manifest scope is only partially known.
    pub auto_narrows_on_partial_manifest_scope: bool,
    /// The claim auto-narrows when registry freshness is stale.
    pub auto_narrows_on_stale_registry_freshness: bool,
    /// The claim auto-narrows when the auth state is unsatisfied.
    pub auto_narrows_on_unsatisfied_auth_state: bool,
    /// The claim auto-narrows when lockfile-impact computation is unavailable.
    pub auto_narrows_on_unavailable_lockfile_impact: bool,
    /// The claim auto-narrows when the rollback / checkpoint truth is unavailable.
    pub auto_narrows_on_unavailable_rollback_checkpoint: bool,
    /// Desktop, CLI, and export semantics are identical.
    pub desktop_cli_export_semantics_identical: bool,
    /// Narrowing prevents overstated package-management scope.
    pub narrowing_prevents_overstated_package_management_scope: bool,
}

impl PackageComponentAccessibilityProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.exposes_keyboard_and_screen_reader_labels
            && self.exposes_cli_and_export_enums
            && self.exposes_explanation_fields
            && self.auto_narrows_on_partial_manifest_scope
            && self.auto_narrows_on_stale_registry_freshness
            && self.auto_narrows_on_unsatisfied_auth_state
            && self.auto_narrows_on_unavailable_lockfile_impact
            && self.auto_narrows_on_unavailable_rollback_checkpoint
            && self.desktop_cli_export_semantics_identical
            && self.narrowing_prevents_overstated_package_management_scope
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentAccessibilityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`PackageComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageComponentAccessibilityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Accessibility rows.
    pub accessibility_rows: Vec<PackageComponentAccessibilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PackageComponentAccessibilityDowngradeTrigger>,
    /// Rendering surfaces this packet covers.
    pub rendering_surfaces: Vec<PackageComponentRenderingSurface>,
    /// Trust review block.
    pub trust_review: PackageComponentAccessibilityTrustReview,
    /// Consumer projection block.
    pub projection: PackageComponentAccessibilityProjection,
    /// Proof freshness block.
    pub proof_freshness: PackageComponentAccessibilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe package-management component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageComponentAccessibilityPacket {
    /// Record kind; must equal [`M5_PACKAGE_COMPONENT_ACCESSIBILITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Accessibility rows.
    pub accessibility_rows: Vec<PackageComponentAccessibilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PackageComponentAccessibilityDowngradeTrigger>,
    /// Rendering surfaces this packet covers.
    pub rendering_surfaces: Vec<PackageComponentRenderingSurface>,
    /// Trust review block.
    pub trust_review: PackageComponentAccessibilityTrustReview,
    /// Consumer projection block.
    pub projection: PackageComponentAccessibilityProjection,
    /// Proof freshness block.
    pub proof_freshness: PackageComponentAccessibilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PackageComponentAccessibilityPacket {
    /// Builds a package-management component accessibility packet from stable-lane input.
    pub fn new(input: PackageComponentAccessibilityPacketInput) -> Self {
        Self {
            record_kind: M5_PACKAGE_COMPONENT_ACCESSIBILITY_RECORD_KIND.to_owned(),
            schema_version: M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            accessibility_rows: input.accessibility_rows,
            downgrade_triggers: input.downgrade_triggers,
            rendering_surfaces: input.rendering_surfaces,
            trust_review: input.trust_review,
            projection: input.projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the package-management component accessibility parity invariants.
    pub fn validate(&self) -> Vec<PackageComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PACKAGE_COMPONENT_ACCESSIBILITY_RECORD_KIND {
            violations.push(PackageComponentAccessibilityViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION {
            violations.push(PackageComponentAccessibilityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PackageComponentAccessibilityViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(PackageComponentAccessibilityViolation::DowngradeTriggersMissing);
        }
        if self.rendering_surfaces.is_empty() {
            violations.push(PackageComponentAccessibilityViolation::RenderingSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(PackageComponentAccessibilityViolation::TrustReviewIncomplete);
        }
        if !self.projection.all_hold() {
            violations.push(PackageComponentAccessibilityViolation::ProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(PackageComponentAccessibilityViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("package-management component accessibility packet serializes"),
        ) {
            violations.push(PackageComponentAccessibilityViolation::RawBoundaryMaterialInExport);
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
            .expect("package-management component accessibility packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .accessibility_rows
            .iter()
            .filter(|row| row.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Package-Management Component Accessibility, Headless, and Export Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Accessibility rows: {} ({} claim-narrowed)\n",
            self.accessibility_rows.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Accessibility rows\n\n");
        for row in &self.accessibility_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: condition `{}`, claim `{}`\n",
                row.component.as_str(),
                row.row_id,
                row.condition.as_str(),
                row.effective_claim.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in package-management accessibility export.
#[derive(Debug)]
pub enum PackageComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PackageComponentAccessibilityViolation>),
}

impl fmt::Display for PackageComponentAccessibilityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "package-management component accessibility export parse failed: {error}"
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
                    "package-management component accessibility export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PackageComponentAccessibilityArtifactError {}

/// Validation failures emitted by [`PackageComponentAccessibilityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageComponentAccessibilityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No accessibility rows are present.
    AccessibilityRowsMissing,
    /// An accessibility row is incomplete.
    RowIncomplete,
    /// A row is missing its keyboard label.
    KeyboardLabelMissing,
    /// A row is missing its screen-reader label.
    ScreenReaderLabelMissing,
    /// A row is missing its CLI enum token.
    CliEnumTokenMissing,
    /// A row is missing its export enum token.
    ExportEnumTokenMissing,
    /// A row is missing its explanation field.
    ExplanationFieldMissing,
    /// A row does not reach all three rendering surfaces.
    RenderingSurfaceCoverageMissing,
    /// A component is reachable only by pointer.
    PointerOnlyComponent,
    /// A component omits itself from the export.
    ExportOpaqueComponent,
    /// A component claims more on the desktop than in CLI or export.
    DesktopStrongerThanCli,
    /// A row's effective claim exceeds the ceiling its condition permits.
    ClaimCeilingExceeded,
    /// A weakening condition is missing its explicit narrow disclosure.
    ClaimNarrowingMissing,
    /// A baseline condition unexpectedly carries a narrow disclosure.
    ClaimNarrowingUnexpected,
    /// A narrow disclosure pins the claim to the wrong tier.
    NarrowedToMismatch,
    /// A narrow disclosure names the wrong trigger.
    NarrowTriggerMismatch,
    /// A narrow disclosure offers the wrong next action.
    NarrowNextActionMismatch,
    /// A narrow disclosure is missing its preserved-truth note.
    NarrowPreservedTruthMissing,
    /// A narrow disclosure is missing its next-action copy.
    NarrowNextActionMissing,
    /// A row that must keep manifest scope and side-effect class explicit is missing its note.
    ScopeDisclosureNoteMissing,
    /// A row that must keep mirror / offline continuity explicit is missing its note.
    ContinuityNoteMissing,
    /// A row that must keep the registry-auth posture explicit is missing its note.
    AuthNoteMissing,
    /// A row that must keep the rollback / checkpoint truth explicit is missing its note.
    RollbackNoteMissing,
    /// A row does not point at the canonical component and matrix contracts.
    CanonicalContractReferenceMissing,
    /// Not every shared component appears among the rows.
    ComponentCoverageMissing,
    /// Not every claim condition appears among the rows.
    ConditionCoverageMissing,
    /// Not every claim tier appears as an effective claim.
    ClaimTierCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No rendering surfaces are present.
    RenderingSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PackageComponentAccessibilityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::AccessibilityRowsMissing => "accessibility_rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::KeyboardLabelMissing => "keyboard_label_missing",
            Self::ScreenReaderLabelMissing => "screen_reader_label_missing",
            Self::CliEnumTokenMissing => "cli_enum_token_missing",
            Self::ExportEnumTokenMissing => "export_enum_token_missing",
            Self::ExplanationFieldMissing => "explanation_field_missing",
            Self::RenderingSurfaceCoverageMissing => "rendering_surface_coverage_missing",
            Self::PointerOnlyComponent => "pointer_only_component",
            Self::ExportOpaqueComponent => "export_opaque_component",
            Self::DesktopStrongerThanCli => "desktop_stronger_than_cli",
            Self::ClaimCeilingExceeded => "claim_ceiling_exceeded",
            Self::ClaimNarrowingMissing => "claim_narrowing_missing",
            Self::ClaimNarrowingUnexpected => "claim_narrowing_unexpected",
            Self::NarrowedToMismatch => "narrowed_to_mismatch",
            Self::NarrowTriggerMismatch => "narrow_trigger_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowPreservedTruthMissing => "narrow_preserved_truth_missing",
            Self::NarrowNextActionMissing => "narrow_next_action_missing",
            Self::ScopeDisclosureNoteMissing => "scope_disclosure_note_missing",
            Self::ContinuityNoteMissing => "continuity_note_missing",
            Self::AuthNoteMissing => "auth_note_missing",
            Self::RollbackNoteMissing => "rollback_note_missing",
            Self::CanonicalContractReferenceMissing => "canonical_contract_reference_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::ConditionCoverageMissing => "condition_coverage_missing",
            Self::ClaimTierCoverageMissing => "claim_tier_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RenderingSurfacesMissing => "rendering_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ProjectionIncomplete => "projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable package-management accessibility export.
pub fn current_package_component_accessibility_export(
) -> Result<PackageComponentAccessibilityPacket, PackageComponentAccessibilityArtifactError> {
    let packet: PackageComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-package-management-accessibility-proof/support_export.json"
    )))
    .map_err(PackageComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PackageComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &PackageComponentAccessibilityPacket,
    violations: &mut Vec<PackageComponentAccessibilityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCHEMA_REF,
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_DOC_REF,
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_CONSUMER_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_EXPLORER_ROW_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_MANIFEST_SCOPE_REGISTRY_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_INSTALL_REVIEW_LOCKFILE_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_ACCESSIBILITY_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PackageComponentAccessibilityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &PackageComponentAccessibilityPacket,
    violations: &mut Vec<PackageComponentAccessibilityViolation>,
) {
    if packet.accessibility_rows.is_empty() {
        violations.push(PackageComponentAccessibilityViolation::AccessibilityRowsMissing);
        return;
    }

    let mut seen_components: BTreeSet<M5PackageComponent> = BTreeSet::new();
    let mut seen_conditions: BTreeSet<PackageComponentClaimCondition> = BTreeSet::new();
    let mut seen_tiers: BTreeSet<PackageComponentClaimTier> = BTreeSet::new();

    for row in &packet.accessibility_rows {
        if row.row_id.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(PackageComponentAccessibilityViolation::RowIncomplete);
        }

        if row.keyboard_label.trim().is_empty() {
            violations.push(PackageComponentAccessibilityViolation::KeyboardLabelMissing);
        }
        if row.screen_reader_label.trim().is_empty() {
            violations.push(PackageComponentAccessibilityViolation::ScreenReaderLabelMissing);
        }
        if row.cli_enum_token.trim().is_empty() {
            violations.push(PackageComponentAccessibilityViolation::CliEnumTokenMissing);
        }
        if row.export_enum_token.trim().is_empty() {
            violations.push(PackageComponentAccessibilityViolation::ExportEnumTokenMissing);
        }
        if row.explanation_field.trim().is_empty() {
            violations.push(PackageComponentAccessibilityViolation::ExplanationFieldMissing);
        }

        if !row.covers_all_rendering_surfaces() {
            violations
                .push(PackageComponentAccessibilityViolation::RenderingSurfaceCoverageMissing);
        }

        // AC1 guardrails: parity across desktop, CLI, and export.
        if row.is_pointer_only {
            violations.push(PackageComponentAccessibilityViolation::PointerOnlyComponent);
        }
        if row.is_export_opaque {
            violations.push(PackageComponentAccessibilityViolation::ExportOpaqueComponent);
        }
        if row.desktop_stronger_than_cli {
            violations.push(PackageComponentAccessibilityViolation::DesktopStrongerThanCli);
        }

        let resolution = row.resolution();
        let ceiling = resolution.permitted_ceiling;

        // AC2 core: a claim may never exceed the ceiling its condition permits.
        if row.effective_claim.rank() > ceiling.rank() {
            violations.push(PackageComponentAccessibilityViolation::ClaimCeilingExceeded);
        }

        // Narrow-disclosure presence and completeness.
        if resolution.requires_narrowing {
            match &row.narrowing {
                None => {
                    violations.push(PackageComponentAccessibilityViolation::ClaimNarrowingMissing);
                }
                Some(narrowing) => {
                    if narrowing.narrowed_to != ceiling {
                        violations.push(PackageComponentAccessibilityViolation::NarrowedToMismatch);
                    }
                    if Some(narrowing.trigger) != resolution.expected_trigger {
                        violations
                            .push(PackageComponentAccessibilityViolation::NarrowTriggerMismatch);
                    }
                    if narrowing.next_action != resolution.expected_next_action {
                        violations
                            .push(PackageComponentAccessibilityViolation::NarrowNextActionMismatch);
                    }
                    if narrowing.preserved_truth_note.trim().is_empty() {
                        violations.push(
                            PackageComponentAccessibilityViolation::NarrowPreservedTruthMissing,
                        );
                    }
                    if narrowing.next_action_label.trim().is_empty() {
                        violations
                            .push(PackageComponentAccessibilityViolation::NarrowNextActionMissing);
                    }
                }
            }
        } else if row.narrowing.is_some() {
            violations.push(PackageComponentAccessibilityViolation::ClaimNarrowingUnexpected);
        }

        if resolution.needs_scope_disclosure_note && row.scope_disclosure_note.trim().is_empty() {
            violations.push(PackageComponentAccessibilityViolation::ScopeDisclosureNoteMissing);
        }
        if resolution.needs_continuity_note && row.continuity_note.trim().is_empty() {
            violations.push(PackageComponentAccessibilityViolation::ContinuityNoteMissing);
        }
        if resolution.needs_auth_note && row.auth_note.trim().is_empty() {
            violations.push(PackageComponentAccessibilityViolation::AuthNoteMissing);
        }
        if resolution.needs_rollback_note && row.rollback_note.trim().is_empty() {
            violations.push(PackageComponentAccessibilityViolation::RollbackNoteMissing);
        }

        if !row.points_at_canonical_contracts() {
            violations
                .push(PackageComponentAccessibilityViolation::CanonicalContractReferenceMissing);
        }

        seen_components.insert(row.component);
        seen_conditions.insert(row.condition);
        seen_tiers.insert(row.effective_claim);
    }

    // Coverage: every component, every condition, and every claim tier must appear.
    for component in M5PackageComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(PackageComponentAccessibilityViolation::ComponentCoverageMissing);
            break;
        }
    }
    for condition in PackageComponentClaimCondition::ALL {
        if !seen_conditions.contains(&condition) {
            violations.push(PackageComponentAccessibilityViolation::ConditionCoverageMissing);
            break;
        }
    }
    for tier in PackageComponentClaimTier::ALL {
        if !seen_tiers.contains(&tier) {
            violations.push(PackageComponentAccessibilityViolation::ClaimTierCoverageMissing);
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
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
