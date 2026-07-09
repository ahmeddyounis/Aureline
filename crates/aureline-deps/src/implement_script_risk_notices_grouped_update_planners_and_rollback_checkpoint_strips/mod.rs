//! Script-risk notices, grouped-update planners, and rollback/checkpoint strips
//! that keep package mutation side effects and recovery posture explicit before a
//! grouped or risky change runs — and visible afterwards.
//!
//! This module narrows the last three components frozen in
//! [`crate::freeze_the_m5_package_management_component_matrix`] — the
//! `script_risk_notice`, the `grouped_update_planner`, and the
//! `rollback_checkpoint_strip` — into one implemented, export-safe packet with
//! three co-equal control vectors.
//!
//! A [`ScriptRiskNotice`] always names what may run (an install lifecycle script,
//! a native build step, a post-install binary fetch, or nothing at all), and its
//! risk class is *derived* from the execution source, the policy disposition, and
//! whether the source is trusted rather than asserted — so a package that runs an
//! untrusted post-install hook can never present as benign, and every notice that
//! can execute code offers either a policy-block action or a review action plus
//! explicit support and client notes.
//!
//! A [`GroupedUpdatePlanner`] always names the update reason and the grouped
//! packages, and its plan class (direct bump, security patch, grouped refresh, or
//! broad convergence) is *derived* from the reason, the grouped-package count, and
//! the transitive churn — so a broad convergence plan can never read as a single
//! direct bump and no generic one-click update hides the real breadth.
//!
//! A [`RollbackCheckpointStrip`] keeps recovery posture visible *after* a mutation
//! instead of only when something breaks: it names the checkpoint identity, the
//! remove-blocked state, and the revert / open-diff / export-patch actions on
//! offer, and its recovery posture is *derived* from the remove-blocked state and
//! whether the write regenerated a lockfile — so a remove-blocked revert can never
//! claim a clean automatic rollback.
//!
//! The registry/resolution degradation vocabulary
//! ([`M5PackageComponentDegradationState`]), the rollback posture
//! ([`M5PackageComponentRollbackPosture`]), the downgrade triggers
//! ([`M5PackageComponentDowngradeTrigger`]), and the consumer surfaces
//! ([`M5PackageComponentConsumerSurface`]) are reused directly from the frozen
//! matrix.
//!
//! Raw manifest bodies, raw lockfile bodies, raw script bodies, registry
//! credentials, private registry URLs, and live registry responses stay outside
//! the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-script-risk-grouped-update-rollback-controls.schema.json`](../../../../schemas/ui/m5-script-risk-grouped-update-rollback-controls.schema.json).
//! The contract doc is
//! [`docs/deps/m5/implement_script_risk_notices_grouped_update_planners_and_rollback_checkpoint_strips.md`](../../../../docs/deps/m5/implement_script_risk_notices_grouped_update_planners_and_rollback_checkpoint_strips.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-script-risk-grouped-update-rollback-controls/`](../../../../fixtures/ui/m5-script-risk-grouped-update-rollback-controls/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_management_component_matrix::{
    M5PackageComponent, M5PackageComponentConsumerSurface, M5PackageComponentDegradationState,
    M5PackageComponentDowngradeTrigger, M5PackageComponentRollbackPosture,
    M5_PACKAGE_COMPONENT_MATRIX_DOC_REF, M5_PACKAGE_COMPONENT_MATRIX_GROUPED_UPDATE_CONTRACT_REF,
    M5_PACKAGE_COMPONENT_MATRIX_ROLLBACK_STRIP_CONTRACT_REF,
    M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF, M5_PACKAGE_COMPONENT_MATRIX_SCRIPT_RISK_CONTRACT_REF,
};

/// Stable record-kind tag carried by [`ScriptRiskGroupedUpdateRollbackControlsPacket`].
pub const SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_RECORD_KIND: &str =
    "script_risk_grouped_update_rollback_controls";

/// Schema version for script-risk / grouped-update / rollback control records.
pub const SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-script-risk-grouped-update-rollback-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_DOC_REF: &str =
    "docs/deps/m5/implement_script_risk_notices_grouped_update_planners_and_rollback_checkpoint_strips.md";

/// Repo-relative path of the protected fixture directory.
pub const SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-script-risk-grouped-update-rollback-controls";

/// Repo-relative path of the checked support-export artifact.
pub const SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-script-risk-grouped-update-rollback-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_SUMMARY_REF: &str =
    "artifacts/release/m5-script-risk-grouped-update-rollback-proof/summary.md";

/// Transitive-churn count at or above which a grouped update is at least grouped.
pub const GROUPED_UPDATE_CHURN_THRESHOLD: u32 = 6;

/// Transitive-churn count at or above which a grouped update is a broad convergence.
pub const BROAD_UPDATE_CHURN_THRESHOLD: u32 = 25;

/// Grouped-package count at or above which a grouped update is a broad convergence.
pub const BROAD_UPDATE_GROUP_THRESHOLD: u32 = 8;

// ---------------------------------------------------------------------------
// Script-risk notice
// ---------------------------------------------------------------------------

/// What a script-risk notice reports may run during a package mutation.
///
/// The source stays explicit so a notice never flattens "an install lifecycle
/// script runs" and "a native toolchain builds a binary" into a generic warning,
/// and a no-scripts-declared package can present its reassuring truth honestly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptExecutionSource {
    /// A package-manager install lifecycle script runs (pre/post install).
    InstallLifecycleScript,
    /// A native build step compiles code on the host toolchain.
    NativeBuildStep,
    /// A post-install step downloads and runs a prebuilt binary.
    PostinstallBinaryFetch,
    /// No install scripts or native build are declared; nothing runs.
    NoScriptsDeclared,
}

impl ScriptExecutionSource {
    /// Every execution source, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InstallLifecycleScript,
        Self::NativeBuildStep,
        Self::PostinstallBinaryFetch,
        Self::NoScriptsDeclared,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallLifecycleScript => "install_lifecycle_script",
            Self::NativeBuildStep => "native_build_step",
            Self::PostinstallBinaryFetch => "postinstall_binary_fetch",
            Self::NoScriptsDeclared => "no_scripts_declared",
        }
    }

    /// Whether this source executes host code during the mutation.
    pub const fn executes_code(self) -> bool {
        !matches!(self, Self::NoScriptsDeclared)
    }
}

/// Derived risk class a script-risk notice may present.
///
/// This is the notice honesty axis: the class is derived from the execution
/// source, the policy disposition, and whether the source is trusted, never
/// asserted, so a package that runs an untrusted hook can never present as benign.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptRiskClass {
    /// No host code runs; the notice is reassuring, not a warning.
    NoExecution,
    /// Host code runs from a trusted source; review is recommended before apply.
    ReviewRecommended,
    /// Policy blocks execution; the mutation cannot run these scripts.
    PolicyBlocked,
    /// Host code runs from an unknown or untrusted source.
    UnknownUntrusted,
}

impl ScriptRiskClass {
    /// Every risk class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoExecution,
        Self::ReviewRecommended,
        Self::PolicyBlocked,
        Self::UnknownUntrusted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoExecution => "no_execution",
            Self::ReviewRecommended => "review_recommended",
            Self::PolicyBlocked => "policy_blocked",
            Self::UnknownUntrusted => "unknown_untrusted",
        }
    }

    /// Whether this class must offer an explicit review action.
    pub const fn needs_review_action(self) -> bool {
        matches!(self, Self::ReviewRecommended | Self::UnknownUntrusted)
    }

    /// Whether this class must offer an explicit policy-block action.
    pub const fn needs_policy_block_action(self) -> bool {
        matches!(self, Self::PolicyBlocked)
    }
}

/// Disclosures a script-risk notice must carry, derived from its risk inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScriptRiskDisclosure {
    /// The derived risk class this notice may present.
    pub risk_class: ScriptRiskClass,
    /// Whether the notice must disclose what code executes.
    pub requires_execution_disclosure: bool,
    /// Whether the notice must offer a review action.
    pub needs_review_action: bool,
    /// Whether the notice must offer a policy-block action.
    pub needs_policy_block_action: bool,
}

/// Resolves the risk class a script-risk notice may present.
///
/// A notice is `no_execution` when nothing runs, `policy_blocked` when policy
/// blocks the scripts that would run, `review_recommended` when trusted code runs,
/// and `unknown_untrusted` when untrusted code runs — never asserted directly.
pub fn resolve_script_risk(
    execution_source: ScriptExecutionSource,
    policy_blocks: bool,
    source_trusted: bool,
) -> ScriptRiskDisclosure {
    let executes = execution_source.executes_code();
    let risk_class = if !executes {
        ScriptRiskClass::NoExecution
    } else if policy_blocks {
        ScriptRiskClass::PolicyBlocked
    } else if source_trusted {
        ScriptRiskClass::ReviewRecommended
    } else {
        ScriptRiskClass::UnknownUntrusted
    };

    ScriptRiskDisclosure {
        risk_class,
        requires_execution_disclosure: executes,
        needs_review_action: risk_class.needs_review_action(),
        needs_policy_block_action: risk_class.needs_policy_block_action(),
    }
}

/// A script-risk notice reporting what may run during a package mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptRiskNotice {
    /// Frozen component this control implements; must be `script_risk_notice`.
    pub component: M5PackageComponent,
    /// Stable notice id.
    pub notice_id: String,
    /// Human-readable package (or package-set) label; required and non-empty.
    pub package_label: String,
    /// What may run during the mutation.
    pub execution_source: ScriptExecutionSource,
    /// Whether policy blocks execution of these scripts.
    pub policy_blocks: bool,
    /// Whether the execution source is trusted.
    pub source_trusted: bool,
    /// Risk class; derived and validated against the risk inputs.
    pub risk_class: ScriptRiskClass,
    /// Execution-source note (what runs); required when code executes.
    pub execution_source_note: String,
    /// Review-action label; required when the class warrants a review action.
    pub review_action_label: String,
    /// Policy-block-action label; required when policy blocks execution.
    pub policy_block_action_label: String,
    /// Support-facing note; always required and non-empty.
    pub support_note: String,
    /// Client-facing note; always required and non-empty.
    pub client_note: String,
    /// Registry/resolution degradation state, reused from the frozen matrix.
    pub degradation_state: M5PackageComponentDegradationState,
    /// Degradation note; required when resolution is not exact.
    pub degradation_note: String,
    /// Rollback / write-back posture; a notice never mutates.
    pub rollback_posture: M5PackageComponentRollbackPosture,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this notice.
    pub source_contract_refs: Vec<String>,
}

impl ScriptRiskNotice {
    /// Risk disclosures this notice must carry, derived from its risk inputs.
    pub fn risk_disclosure(&self) -> ScriptRiskDisclosure {
        resolve_script_risk(
            self.execution_source,
            self.policy_blocks,
            self.source_trusted,
        )
    }

    /// Whether the rollback posture is consistent with an informational notice.
    ///
    /// A notice reports risk; it never writes, so it must be read-only or a
    /// staged-review disclosure.
    pub fn rollback_posture_consistent(&self) -> bool {
        matches!(
            self.rollback_posture,
            M5PackageComponentRollbackPosture::ReadOnlyNoMutation
                | M5PackageComponentRollbackPosture::StagedReviewNoWrite
        )
    }
}

// ---------------------------------------------------------------------------
// Grouped-update planner
// ---------------------------------------------------------------------------

/// The reason a grouped-update planner exists.
///
/// The reason stays explicit so a security patch is never flattened into a
/// routine refresh and a convergence plan is never sold as a direct request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateReason {
    /// The user directly requested this bump.
    DirectRequest,
    /// A security advisory motivates this update.
    SecurityAdvisory,
    /// A routine refresh of otherwise up-to-date dependencies.
    RoutineRefresh,
    /// A convergence plan reconciling many dependencies onto compatible versions.
    DependencyConvergence,
}

impl UpdateReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DirectRequest,
        Self::SecurityAdvisory,
        Self::RoutineRefresh,
        Self::DependencyConvergence,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectRequest => "direct_request",
            Self::SecurityAdvisory => "security_advisory",
            Self::RoutineRefresh => "routine_refresh",
            Self::DependencyConvergence => "dependency_convergence",
        }
    }
}

/// Derived plan class a grouped-update planner may present.
///
/// This is the planner honesty axis and the AC's "distinguish direct bumps,
/// security patches, grouped refreshes, and broad convergence plans": the class is
/// derived from the reason, the grouped-package count, and the transitive churn,
/// never asserted, so a broad convergence can never read as a single direct bump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupedUpdatePlanClass {
    /// A single-package direct bump with little churn.
    DirectBump,
    /// A security-motivated patch.
    SecurityPatch,
    /// A grouped refresh across several packages.
    GroupedRefresh,
    /// A broad convergence: many packages or high transitive churn.
    BroadConvergence,
}

impl GroupedUpdatePlanClass {
    /// Every plan class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DirectBump,
        Self::SecurityPatch,
        Self::GroupedRefresh,
        Self::BroadConvergence,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectBump => "direct_bump",
            Self::SecurityPatch => "security_patch",
            Self::GroupedRefresh => "grouped_refresh",
            Self::BroadConvergence => "broad_convergence",
        }
    }
}

/// Disclosures a grouped-update planner must carry, derived from its blast radius.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdatePlanDisclosure {
    /// The derived plan class this planner may present.
    pub plan_class: GroupedUpdatePlanClass,
    /// Whether the planner must carry a convergence note.
    pub needs_convergence_note: bool,
    /// Whether the planner must carry a security note.
    pub needs_security_note: bool,
    /// Whether the planner must carry a transitive-churn note.
    pub needs_transitive_churn_note: bool,
}

/// Resolves the plan class a grouped-update planner may present.
///
/// A plan is `broad_convergence` when the reason is convergence, the grouped set
/// is large, or the transitive churn crosses the broad threshold; a
/// `security_patch` when a security advisory motivates it; a `grouped_refresh`
/// when it touches several packages or crosses the grouped threshold; and a
/// `direct_bump` otherwise.
pub fn resolve_update_plan_class(
    reason: UpdateReason,
    grouped_package_count: u32,
    transitive_churn_count: u32,
) -> UpdatePlanDisclosure {
    let is_broad = matches!(reason, UpdateReason::DependencyConvergence)
        || transitive_churn_count >= BROAD_UPDATE_CHURN_THRESHOLD
        || grouped_package_count >= BROAD_UPDATE_GROUP_THRESHOLD;
    let plan_class = if is_broad {
        GroupedUpdatePlanClass::BroadConvergence
    } else if matches!(reason, UpdateReason::SecurityAdvisory) {
        GroupedUpdatePlanClass::SecurityPatch
    } else if grouped_package_count > 1 || transitive_churn_count >= GROUPED_UPDATE_CHURN_THRESHOLD
    {
        GroupedUpdatePlanClass::GroupedRefresh
    } else {
        GroupedUpdatePlanClass::DirectBump
    };

    UpdatePlanDisclosure {
        plan_class,
        needs_convergence_note: is_broad,
        needs_security_note: matches!(plan_class, GroupedUpdatePlanClass::SecurityPatch),
        needs_transitive_churn_note: transitive_churn_count > 0,
    }
}

/// A grouped-update planner previewing the reason, packages, and churn of a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupedUpdatePlanner {
    /// Frozen component this control implements; must be `grouped_update_planner`.
    pub component: M5PackageComponent,
    /// Stable planner id.
    pub planner_id: String,
    /// Human-readable plan label; required and non-empty.
    pub plan_label: String,
    /// Why this plan exists.
    pub update_reason: UpdateReason,
    /// Reason note; always required and non-empty.
    pub reason_note: String,
    /// Packages grouped into this plan; always required and non-empty.
    pub grouped_packages: Vec<String>,
    /// Transitive-dependency churn count this plan introduces.
    pub transitive_churn_count: u32,
    /// Plan class; derived and validated against the reason and blast radius.
    pub plan_class: GroupedUpdatePlanClass,
    /// Transitive-churn note; required when there is any churn.
    pub transitive_churn_note: String,
    /// Convergence note; required when the plan is a broad convergence.
    pub convergence_note: String,
    /// Security note; required when the plan is a security patch.
    pub security_note: String,
    /// Registry/resolution degradation state, reused from the frozen matrix.
    pub degradation_state: M5PackageComponentDegradationState,
    /// Degradation note; required when resolution is not exact.
    pub degradation_note: String,
    /// Rollback / write-back posture; a planner previews and never writes.
    pub rollback_posture: M5PackageComponentRollbackPosture,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this planner.
    pub source_contract_refs: Vec<String>,
}

impl GroupedUpdatePlanner {
    /// Plan disclosures this planner must carry, derived from its blast radius.
    pub fn plan_disclosure(&self) -> UpdatePlanDisclosure {
        resolve_update_plan_class(
            self.update_reason,
            self.grouped_packages.len() as u32,
            self.transitive_churn_count,
        )
    }

    /// Whether the rollback posture is consistent with a preview-first planner.
    ///
    /// A planner previews a plan and never writes until an explicit apply, so it
    /// must be read-only or staged-review.
    pub fn rollback_posture_consistent(&self) -> bool {
        matches!(
            self.rollback_posture,
            M5PackageComponentRollbackPosture::ReadOnlyNoMutation
                | M5PackageComponentRollbackPosture::StagedReviewNoWrite
        )
    }
}

// ---------------------------------------------------------------------------
// Rollback / checkpoint strip
// ---------------------------------------------------------------------------

/// Whether a mutated package can be cleanly removed on revert.
///
/// The remove-blocked state stays explicit so a revert that cannot cleanly remove
/// a now-required package never claims a clean rollback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoveBlockedState {
    /// The mutation was not a remove; nothing is remove-blocked.
    NotARemove,
    /// The package can be cleanly removed on revert.
    Removable,
    /// Removal is blocked because the package is policy-pinned.
    RemoveBlockedPolicyPinned,
    /// Removal is blocked because another dependency now requires it.
    RemoveBlockedRequiredBy,
}

impl RemoveBlockedState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NotARemove,
        Self::Removable,
        Self::RemoveBlockedPolicyPinned,
        Self::RemoveBlockedRequiredBy,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotARemove => "not_a_remove",
            Self::Removable => "removable",
            Self::RemoveBlockedPolicyPinned => "remove_blocked_policy_pinned",
            Self::RemoveBlockedRequiredBy => "remove_blocked_required_by",
        }
    }

    /// Whether this state blocks a clean removal on revert.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::RemoveBlockedPolicyPinned | Self::RemoveBlockedRequiredBy
        )
    }
}

/// Derived recovery posture a rollback/checkpoint strip may present.
///
/// This is the strip honesty axis: the posture is derived from the remove-blocked
/// state and whether the write regenerated a lockfile, never asserted, so a
/// remove-blocked revert can never claim a clean automatic rollback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPostureClass {
    /// The mutation reverts fully from a durable checkpoint.
    FullyRevertible,
    /// The mutation reverts, but a lockfile regenerates rather than restoring edits.
    RevertWithRegeneration,
    /// Only a compensating cleanup exists; there is no clean automatic revert.
    CompensatingOnly,
}

impl RecoveryPostureClass {
    /// Every posture class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::FullyRevertible,
        Self::RevertWithRegeneration,
        Self::CompensatingOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyRevertible => "fully_revertible",
            Self::RevertWithRegeneration => "revert_with_regeneration",
            Self::CompensatingOnly => "compensating_only",
        }
    }

    /// The frozen rollback posture this recovery class implies.
    pub const fn expected_rollback_posture(self) -> M5PackageComponentRollbackPosture {
        match self {
            Self::FullyRevertible => M5PackageComponentRollbackPosture::WriteBackCheckpointed,
            Self::RevertWithRegeneration => {
                M5PackageComponentRollbackPosture::RegenerateOnlyNoManualEdit
            }
            Self::CompensatingOnly => {
                M5PackageComponentRollbackPosture::CompensatingOnlyNoCleanRevert
            }
        }
    }
}

/// Disclosures a rollback/checkpoint strip must carry, derived from its recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryDisclosure {
    /// The derived recovery posture this strip may present.
    pub recovery_posture: RecoveryPostureClass,
    /// Whether the strip must carry a remove-blocked note.
    pub needs_remove_blocked_note: bool,
    /// Whether the strip must carry a regeneration note.
    pub needs_regeneration_note: bool,
}

/// Resolves the recovery posture a rollback/checkpoint strip may present.
///
/// A blocked removal can only be compensated for, a regenerating write reverts
/// with regeneration, and everything else reverts fully from the checkpoint.
pub fn resolve_recovery_posture(
    remove_blocked_state: RemoveBlockedState,
    regenerated: bool,
) -> RecoveryDisclosure {
    let blocked = remove_blocked_state.is_blocked();
    let recovery_posture = if blocked {
        RecoveryPostureClass::CompensatingOnly
    } else if regenerated {
        RecoveryPostureClass::RevertWithRegeneration
    } else {
        RecoveryPostureClass::FullyRevertible
    };

    RecoveryDisclosure {
        recovery_posture,
        needs_remove_blocked_note: blocked,
        needs_regeneration_note: regenerated,
    }
}

/// A rollback/checkpoint strip keeping recovery posture visible after a mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackCheckpointStrip {
    /// Frozen component this control implements; must be `rollback_checkpoint_strip`.
    pub component: M5PackageComponent,
    /// Stable strip id.
    pub strip_id: String,
    /// Human-readable checkpoint label; required and non-empty.
    pub checkpoint_label: String,
    /// Durable checkpoint identity; required and non-empty.
    pub checkpoint_id: String,
    /// What the applied mutation did; required and non-empty.
    pub mutation_summary: String,
    /// Whether removed packages can be cleanly removed on revert.
    pub remove_blocked_state: RemoveBlockedState,
    /// Remove-blocked note; required when removal is blocked.
    pub remove_blocked_note: String,
    /// Whether the applied write regenerated a lockfile.
    pub regenerated: bool,
    /// Regeneration note; required when the write regenerated a lockfile.
    pub regeneration_note: String,
    /// Recovery posture; derived and validated against the recovery inputs.
    pub recovery_posture_class: RecoveryPostureClass,
    /// Rollback / write-back posture; derived from the recovery posture class.
    pub rollback_posture: M5PackageComponentRollbackPosture,
    /// Whether recovery is visible now (after mutation), not only on failure.
    pub recovery_visible_now: bool,
    /// Revert action label; always required and non-empty.
    pub revert_action_label: String,
    /// Open-diff action label; always required and non-empty.
    pub open_diff_action_label: String,
    /// Export-patch action label; always required and non-empty.
    pub export_patch_action_label: String,
    /// Registry/resolution degradation state, reused from the frozen matrix.
    pub degradation_state: M5PackageComponentDegradationState,
    /// Degradation note; required when resolution is not exact.
    pub degradation_note: String,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this strip.
    pub source_contract_refs: Vec<String>,
}

impl RollbackCheckpointStrip {
    /// Recovery disclosures this strip must carry, derived from its recovery inputs.
    pub fn recovery_disclosure(&self) -> RecoveryDisclosure {
        resolve_recovery_posture(self.remove_blocked_state, self.regenerated)
    }

    /// Whether the rollback posture is consistent with the derived recovery class.
    ///
    /// A remove-blocked revert can never claim a clean write-back checkpoint.
    pub fn rollback_posture_consistent(&self) -> bool {
        self.rollback_posture == self.recovery_posture_class.expected_rollback_posture()
    }
}

// ---------------------------------------------------------------------------
// Trust / consumer / freshness blocks
// ---------------------------------------------------------------------------

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptRiskGroupedUpdateRollbackTrustReview {
    /// The script/native-build execution source stays explicit.
    pub script_execution_source_always_explicit: bool,
    /// The script risk class is derived, never asserted.
    pub script_risk_class_derived_not_asserted: bool,
    /// A policy-block or review action is always offered for executing code.
    pub policy_block_or_review_action_always_offered: bool,
    /// The update reason stays explicit.
    pub update_reason_always_explicit: bool,
    /// The grouped packages are always listed.
    pub grouped_packages_always_listed: bool,
    /// Transitive churn is never understated.
    pub transitive_churn_never_understated: bool,
    /// The plan class distinguishes bump / patch / grouped / broad.
    pub plan_class_distinguishes_bump_patch_grouped_broad: bool,
    /// Remove-blocked states stay explicit.
    pub remove_blocked_states_explicit: bool,
    /// Recovery posture stays visible after mutation, not only on failure.
    pub recovery_posture_visible_after_mutation: bool,
    /// Revert / open-diff / export-patch actions are always offered.
    pub revert_open_diff_export_patch_always_offered: bool,
    /// No generic one-click-update language conceals scope, churn, or risk.
    pub no_generic_one_click_update_language: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl ScriptRiskGroupedUpdateRollbackTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.script_execution_source_always_explicit
            && self.script_risk_class_derived_not_asserted
            && self.policy_block_or_review_action_always_offered
            && self.update_reason_always_explicit
            && self.grouped_packages_always_listed
            && self.transitive_churn_never_understated
            && self.plan_class_distinguishes_bump_patch_grouped_broad
            && self.remove_blocked_states_explicit
            && self.recovery_posture_visible_after_mutation
            && self.revert_open_diff_export_patch_always_offered
            && self.no_generic_one_click_update_language
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptRiskGroupedUpdateRollbackConsumerProjection {
    /// The script-risk notice shows the execution source and risk class.
    pub script_risk_notice_shows_source_and_risk: bool,
    /// The grouped-update planner shows the reason and plan class.
    pub grouped_update_planner_shows_reason_and_class: bool,
    /// The transitive churn is shown inline.
    pub transitive_churn_shown_inline: bool,
    /// The rollback strip shows the recovery posture.
    pub rollback_strip_shows_recovery_posture: bool,
    /// Remove-blocked states are shown inline.
    pub remove_blocked_states_shown_inline: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_control_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl ScriptRiskGroupedUpdateRollbackConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.script_risk_notice_shows_source_and_risk
            && self.grouped_update_planner_shows_reason_and_class
            && self.transitive_churn_shown_inline
            && self.rollback_strip_shows_recovery_posture
            && self.remove_blocked_states_shown_inline
            && self.cli_headless_shows_control_truth
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptRiskGroupedUpdateRollbackProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ScriptRiskGroupedUpdateRollbackControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptRiskGroupedUpdateRollbackControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Script-risk notices.
    pub script_risk_notices: Vec<ScriptRiskNotice>,
    /// Grouped-update planners.
    pub grouped_update_planners: Vec<GroupedUpdatePlanner>,
    /// Rollback / checkpoint strips.
    pub rollback_checkpoint_strips: Vec<RollbackCheckpointStrip>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5PackageComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: ScriptRiskGroupedUpdateRollbackTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ScriptRiskGroupedUpdateRollbackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ScriptRiskGroupedUpdateRollbackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe script-risk / grouped-update / rollback controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptRiskGroupedUpdateRollbackControlsPacket {
    /// Record kind; must equal [`SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Script-risk notices.
    pub script_risk_notices: Vec<ScriptRiskNotice>,
    /// Grouped-update planners.
    pub grouped_update_planners: Vec<GroupedUpdatePlanner>,
    /// Rollback / checkpoint strips.
    pub rollback_checkpoint_strips: Vec<RollbackCheckpointStrip>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5PackageComponentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5PackageComponentConsumerSurface>,
    /// Trust review block.
    pub trust_review: ScriptRiskGroupedUpdateRollbackTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ScriptRiskGroupedUpdateRollbackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ScriptRiskGroupedUpdateRollbackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ScriptRiskGroupedUpdateRollbackControlsPacket {
    /// Builds a script-risk / grouped-update / rollback controls packet from input.
    pub fn new(input: ScriptRiskGroupedUpdateRollbackControlsPacketInput) -> Self {
        Self {
            record_kind: SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_RECORD_KIND.to_owned(),
            schema_version: SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            script_risk_notices: input.script_risk_notices,
            grouped_update_planners: input.grouped_update_planners,
            rollback_checkpoint_strips: input.rollback_checkpoint_strips,
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

    /// Validates the script-risk / grouped-update / rollback control invariants.
    pub fn validate(&self) -> Vec<ScriptRiskGroupedUpdateRollbackViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_RECORD_KIND {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::WrongRecordKind);
        }
        if self.schema_version != SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_SCHEMA_VERSION {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_script_risk_notices(self, &mut violations);
        validate_grouped_update_planners(self, &mut violations);
        validate_rollback_strips(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("script risk grouped update rollback packet serializes"),
        ) {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::RawBoundaryMaterialInExport);
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
            .expect("script risk grouped update rollback packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let blocked_notices = self
            .script_risk_notices
            .iter()
            .filter(|notice| notice.risk_class == ScriptRiskClass::PolicyBlocked)
            .count();
        let broad_plans = self
            .grouped_update_planners
            .iter()
            .filter(|planner| planner.plan_class == GroupedUpdatePlanClass::BroadConvergence)
            .count();
        let blocked_reverts = self
            .rollback_checkpoint_strips
            .iter()
            .filter(|strip| strip.remove_blocked_state.is_blocked())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Script-risk notices, grouped-update planners, and rollback/checkpoint strips\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Script-risk notices: {} ({} policy-blocked)\n",
            self.script_risk_notices.len(),
            blocked_notices
        ));
        out.push_str(&format!(
            "- Grouped-update planners: {} ({} broad convergence)\n",
            self.grouped_update_planners.len(),
            broad_plans
        ));
        out.push_str(&format!(
            "- Rollback/checkpoint strips: {} ({} remove-blocked)\n",
            self.rollback_checkpoint_strips.len(),
            blocked_reverts
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Script-risk notices\n\n");
        for notice in &self.script_risk_notices {
            out.push_str(&format!(
                "- **{}** — source `{}`, risk `{}`\n",
                notice.package_label,
                notice.execution_source.as_str(),
                notice.risk_class.as_str()
            ));
        }

        out.push_str("\n## Grouped-update planners\n\n");
        for planner in &self.grouped_update_planners {
            out.push_str(&format!(
                "- **{}** — reason `{}`, class `{}`, {} package(s), {} transitive churn\n",
                planner.plan_label,
                planner.update_reason.as_str(),
                planner.plan_class.as_str(),
                planner.grouped_packages.len(),
                planner.transitive_churn_count
            ));
        }

        out.push_str("\n## Rollback/checkpoint strips\n\n");
        for strip in &self.rollback_checkpoint_strips {
            out.push_str(&format!(
                "- **{}** — recovery `{}`, remove-blocked `{}`\n",
                strip.checkpoint_label,
                strip.recovery_posture_class.as_str(),
                strip.remove_blocked_state.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in controls export.
#[derive(Debug)]
pub enum ScriptRiskGroupedUpdateRollbackArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ScriptRiskGroupedUpdateRollbackViolation>),
}

impl fmt::Display for ScriptRiskGroupedUpdateRollbackArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "script risk grouped update rollback export parse failed: {error}"
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
                    "script risk grouped update rollback export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ScriptRiskGroupedUpdateRollbackArtifactError {}

/// Validation failures emitted by [`ScriptRiskGroupedUpdateRollbackControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptRiskGroupedUpdateRollbackViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No script-risk notices are present.
    ScriptRiskNoticesMissing,
    /// A script-risk notice is incomplete.
    ScriptRiskNoticeIncomplete,
    /// A script-risk notice carries the wrong frozen component class.
    ScriptRiskNoticeWrongComponentClass,
    /// A script-risk notice does not name its package label.
    ScriptPackageLabelMissing,
    /// An executing notice does not disclose what code runs.
    ScriptExecutionSourceNoteMissing,
    /// A script-risk notice misrepresents its risk class.
    ScriptRiskClassMisrepresented,
    /// A notice warranting a review action does not offer one.
    ScriptReviewActionMissing,
    /// A policy-blocked notice does not offer a policy-block action.
    ScriptPolicyBlockActionMissing,
    /// A script-risk notice does not carry its support / client notes.
    ScriptSupportClientNoteMissing,
    /// A degraded script-risk notice does not carry a degradation note.
    ScriptRiskNoticeDegradationNoteMissing,
    /// A notice rollback posture is inconsistent with an informational notice.
    ScriptRiskNoticeRollbackPostureInconsistent,
    /// The notices do not cover the required risk classes.
    ScriptRiskCoverageMissing,
    /// No grouped-update planners are present.
    GroupedUpdatePlannersMissing,
    /// A grouped-update planner is incomplete.
    GroupedUpdatePlannerIncomplete,
    /// A grouped-update planner carries the wrong frozen component class.
    GroupedUpdatePlannerWrongComponentClass,
    /// A grouped-update planner does not carry its reason note.
    UpdateReasonNoteMissing,
    /// A grouped-update planner does not list its grouped packages.
    GroupedPackagesMissing,
    /// A grouped-update planner misrepresents its plan class.
    PlanClassMisrepresented,
    /// A planner with churn does not carry a transitive-churn note.
    TransitiveChurnNoteMissing,
    /// A broad-convergence planner does not carry a convergence note.
    ConvergenceNoteMissing,
    /// A security-patch planner does not carry a security note.
    SecurityNoteMissing,
    /// A degraded planner does not carry a degradation note.
    GroupedUpdatePlannerDegradationNoteMissing,
    /// A planner rollback posture is inconsistent with a preview-first planner.
    GroupedUpdatePlannerRollbackPostureInconsistent,
    /// The planners do not cover direct bump, security patch, grouped, and broad.
    PlanClassCoverageMissing,
    /// No rollback/checkpoint strips are present.
    RollbackStripsMissing,
    /// A rollback/checkpoint strip is incomplete.
    RollbackStripIncomplete,
    /// A rollback/checkpoint strip carries the wrong frozen component class.
    RollbackStripWrongComponentClass,
    /// A rollback/checkpoint strip does not name its checkpoint identity.
    CheckpointIdentityMissing,
    /// A rollback/checkpoint strip does not name what the mutation did.
    MutationSummaryMissing,
    /// A remove-blocked strip does not carry a remove-blocked note.
    RemoveBlockedNoteMissing,
    /// A regenerating strip does not carry a regeneration note.
    RegenerationNoteMissing,
    /// A rollback/checkpoint strip misrepresents its recovery posture.
    RecoveryPostureMisrepresented,
    /// A rollback/checkpoint strip does not keep recovery visible after mutation.
    RecoveryPostureNotVisibleAfterMutation,
    /// A strip does not offer revert / open-diff / export-patch actions.
    RollbackActionsMissing,
    /// A strip rollback posture is inconsistent with its recovery class.
    RollbackStripRollbackPostureInconsistent,
    /// A degraded strip does not carry a degradation note.
    RollbackStripDegradationNoteMissing,
    /// The strips do not cover the required recovery postures.
    RecoveryPostureCoverageMissing,
    /// The strips do not demonstrate a remove-blocked state.
    RemoveBlockedCoverageMissing,
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

impl ScriptRiskGroupedUpdateRollbackViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ScriptRiskNoticesMissing => "script_risk_notices_missing",
            Self::ScriptRiskNoticeIncomplete => "script_risk_notice_incomplete",
            Self::ScriptRiskNoticeWrongComponentClass => "script_risk_notice_wrong_component_class",
            Self::ScriptPackageLabelMissing => "script_package_label_missing",
            Self::ScriptExecutionSourceNoteMissing => "script_execution_source_note_missing",
            Self::ScriptRiskClassMisrepresented => "script_risk_class_misrepresented",
            Self::ScriptReviewActionMissing => "script_review_action_missing",
            Self::ScriptPolicyBlockActionMissing => "script_policy_block_action_missing",
            Self::ScriptSupportClientNoteMissing => "script_support_client_note_missing",
            Self::ScriptRiskNoticeDegradationNoteMissing => {
                "script_risk_notice_degradation_note_missing"
            }
            Self::ScriptRiskNoticeRollbackPostureInconsistent => {
                "script_risk_notice_rollback_posture_inconsistent"
            }
            Self::ScriptRiskCoverageMissing => "script_risk_coverage_missing",
            Self::GroupedUpdatePlannersMissing => "grouped_update_planners_missing",
            Self::GroupedUpdatePlannerIncomplete => "grouped_update_planner_incomplete",
            Self::GroupedUpdatePlannerWrongComponentClass => {
                "grouped_update_planner_wrong_component_class"
            }
            Self::UpdateReasonNoteMissing => "update_reason_note_missing",
            Self::GroupedPackagesMissing => "grouped_packages_missing",
            Self::PlanClassMisrepresented => "plan_class_misrepresented",
            Self::TransitiveChurnNoteMissing => "transitive_churn_note_missing",
            Self::ConvergenceNoteMissing => "convergence_note_missing",
            Self::SecurityNoteMissing => "security_note_missing",
            Self::GroupedUpdatePlannerDegradationNoteMissing => {
                "grouped_update_planner_degradation_note_missing"
            }
            Self::GroupedUpdatePlannerRollbackPostureInconsistent => {
                "grouped_update_planner_rollback_posture_inconsistent"
            }
            Self::PlanClassCoverageMissing => "plan_class_coverage_missing",
            Self::RollbackStripsMissing => "rollback_strips_missing",
            Self::RollbackStripIncomplete => "rollback_strip_incomplete",
            Self::RollbackStripWrongComponentClass => "rollback_strip_wrong_component_class",
            Self::CheckpointIdentityMissing => "checkpoint_identity_missing",
            Self::MutationSummaryMissing => "mutation_summary_missing",
            Self::RemoveBlockedNoteMissing => "remove_blocked_note_missing",
            Self::RegenerationNoteMissing => "regeneration_note_missing",
            Self::RecoveryPostureMisrepresented => "recovery_posture_misrepresented",
            Self::RecoveryPostureNotVisibleAfterMutation => {
                "recovery_posture_not_visible_after_mutation"
            }
            Self::RollbackActionsMissing => "rollback_actions_missing",
            Self::RollbackStripRollbackPostureInconsistent => {
                "rollback_strip_rollback_posture_inconsistent"
            }
            Self::RollbackStripDegradationNoteMissing => "rollback_strip_degradation_note_missing",
            Self::RecoveryPostureCoverageMissing => "recovery_posture_coverage_missing",
            Self::RemoveBlockedCoverageMissing => "remove_blocked_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_script_risk_grouped_update_rollback_export() -> Result<
    ScriptRiskGroupedUpdateRollbackControlsPacket,
    ScriptRiskGroupedUpdateRollbackArtifactError,
> {
    let packet: ScriptRiskGroupedUpdateRollbackControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-script-risk-grouped-update-rollback-proof/support_export.json"
        )))
        .map_err(ScriptRiskGroupedUpdateRollbackArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ScriptRiskGroupedUpdateRollbackArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &ScriptRiskGroupedUpdateRollbackControlsPacket,
    violations: &mut Vec<ScriptRiskGroupedUpdateRollbackViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_SCHEMA_REF,
        SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_DOC_REF,
        M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF,
        M5_PACKAGE_COMPONENT_MATRIX_DOC_REF,
        M5_PACKAGE_COMPONENT_MATRIX_SCRIPT_RISK_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_GROUPED_UPDATE_CONTRACT_REF,
        M5_PACKAGE_COMPONENT_MATRIX_ROLLBACK_STRIP_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_script_risk_notices(
    packet: &ScriptRiskGroupedUpdateRollbackControlsPacket,
    violations: &mut Vec<ScriptRiskGroupedUpdateRollbackViolation>,
) {
    if packet.script_risk_notices.is_empty() {
        violations.push(ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskNoticesMissing);
        return;
    }

    let mut risk_classes: BTreeSet<ScriptRiskClass> = BTreeSet::new();

    for notice in &packet.script_risk_notices {
        risk_classes.insert(notice.risk_class);

        if notice.notice_id.trim().is_empty()
            || notice.fields_shown.is_empty()
            || notice.source_contract_refs.is_empty()
        {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskNoticeIncomplete);
        }
        if notice.component != M5PackageComponent::ScriptRiskNotice {
            violations.push(
                ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskNoticeWrongComponentClass,
            );
        }
        if notice.package_label.trim().is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::ScriptPackageLabelMissing);
        }
        if notice.support_note.trim().is_empty() || notice.client_note.trim().is_empty() {
            violations
                .push(ScriptRiskGroupedUpdateRollbackViolation::ScriptSupportClientNoteMissing);
        }
        if !matches!(
            notice.degradation_state,
            M5PackageComponentDegradationState::ResolvedExact
        ) && notice.degradation_note.trim().is_empty()
        {
            violations.push(
                ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskNoticeDegradationNoteMissing,
            );
        }

        let disclosure = notice.risk_disclosure();

        if notice.risk_class != disclosure.risk_class {
            violations
                .push(ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskClassMisrepresented);
        }
        if disclosure.requires_execution_disclosure
            && notice.execution_source_note.trim().is_empty()
        {
            violations
                .push(ScriptRiskGroupedUpdateRollbackViolation::ScriptExecutionSourceNoteMissing);
        }
        if disclosure.needs_review_action && notice.review_action_label.trim().is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::ScriptReviewActionMissing);
        }
        if disclosure.needs_policy_block_action
            && notice.policy_block_action_label.trim().is_empty()
        {
            violations
                .push(ScriptRiskGroupedUpdateRollbackViolation::ScriptPolicyBlockActionMissing);
        }
        if !notice.rollback_posture_consistent() {
            violations.push(
                ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskNoticeRollbackPostureInconsistent,
            );
        }
    }

    for required in [
        ScriptRiskClass::NoExecution,
        ScriptRiskClass::ReviewRecommended,
        ScriptRiskClass::PolicyBlocked,
        ScriptRiskClass::UnknownUntrusted,
    ] {
        if !risk_classes.contains(&required) {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskCoverageMissing);
            break;
        }
    }
}

fn validate_grouped_update_planners(
    packet: &ScriptRiskGroupedUpdateRollbackControlsPacket,
    violations: &mut Vec<ScriptRiskGroupedUpdateRollbackViolation>,
) {
    if packet.grouped_update_planners.is_empty() {
        violations.push(ScriptRiskGroupedUpdateRollbackViolation::GroupedUpdatePlannersMissing);
        return;
    }

    let mut plan_classes: BTreeSet<GroupedUpdatePlanClass> = BTreeSet::new();

    for planner in &packet.grouped_update_planners {
        plan_classes.insert(planner.plan_class);

        if planner.planner_id.trim().is_empty()
            || planner.plan_label.trim().is_empty()
            || planner.fields_shown.is_empty()
            || planner.source_contract_refs.is_empty()
        {
            violations
                .push(ScriptRiskGroupedUpdateRollbackViolation::GroupedUpdatePlannerIncomplete);
        }
        if planner.component != M5PackageComponent::GroupedUpdatePlanner {
            violations.push(
                ScriptRiskGroupedUpdateRollbackViolation::GroupedUpdatePlannerWrongComponentClass,
            );
        }
        if planner.reason_note.trim().is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::UpdateReasonNoteMissing);
        }
        if planner.grouped_packages.is_empty()
            || planner.grouped_packages.iter().any(|p| p.trim().is_empty())
        {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::GroupedPackagesMissing);
        }
        if !matches!(
            planner.degradation_state,
            M5PackageComponentDegradationState::ResolvedExact
        ) && planner.degradation_note.trim().is_empty()
        {
            violations.push(
                ScriptRiskGroupedUpdateRollbackViolation::GroupedUpdatePlannerDegradationNoteMissing,
            );
        }

        let disclosure = planner.plan_disclosure();

        if planner.plan_class != disclosure.plan_class {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::PlanClassMisrepresented);
        }
        if disclosure.needs_transitive_churn_note && planner.transitive_churn_note.trim().is_empty()
        {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::TransitiveChurnNoteMissing);
        }
        if disclosure.needs_convergence_note && planner.convergence_note.trim().is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::ConvergenceNoteMissing);
        }
        if disclosure.needs_security_note && planner.security_note.trim().is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::SecurityNoteMissing);
        }
        if !planner.rollback_posture_consistent() {
            violations.push(
                ScriptRiskGroupedUpdateRollbackViolation::GroupedUpdatePlannerRollbackPostureInconsistent,
            );
        }
    }

    for required in GroupedUpdatePlanClass::ALL {
        if !plan_classes.contains(&required) {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::PlanClassCoverageMissing);
            break;
        }
    }
}

fn validate_rollback_strips(
    packet: &ScriptRiskGroupedUpdateRollbackControlsPacket,
    violations: &mut Vec<ScriptRiskGroupedUpdateRollbackViolation>,
) {
    if packet.rollback_checkpoint_strips.is_empty() {
        violations.push(ScriptRiskGroupedUpdateRollbackViolation::RollbackStripsMissing);
        return;
    }

    let mut recovery_postures: BTreeSet<RecoveryPostureClass> = BTreeSet::new();
    let mut saw_remove_blocked = false;

    for strip in &packet.rollback_checkpoint_strips {
        recovery_postures.insert(strip.recovery_posture_class);
        if strip.remove_blocked_state.is_blocked() {
            saw_remove_blocked = true;
        }

        if strip.strip_id.trim().is_empty()
            || strip.fields_shown.is_empty()
            || strip.source_contract_refs.is_empty()
        {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::RollbackStripIncomplete);
        }
        if strip.component != M5PackageComponent::RollbackCheckpointStrip {
            violations
                .push(ScriptRiskGroupedUpdateRollbackViolation::RollbackStripWrongComponentClass);
        }
        if strip.checkpoint_label.trim().is_empty() || strip.checkpoint_id.trim().is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::CheckpointIdentityMissing);
        }
        if strip.mutation_summary.trim().is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::MutationSummaryMissing);
        }
        if strip.revert_action_label.trim().is_empty()
            || strip.open_diff_action_label.trim().is_empty()
            || strip.export_patch_action_label.trim().is_empty()
        {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::RollbackActionsMissing);
        }
        if !strip.recovery_visible_now {
            violations.push(
                ScriptRiskGroupedUpdateRollbackViolation::RecoveryPostureNotVisibleAfterMutation,
            );
        }
        if !matches!(
            strip.degradation_state,
            M5PackageComponentDegradationState::ResolvedExact
        ) && strip.degradation_note.trim().is_empty()
        {
            violations.push(
                ScriptRiskGroupedUpdateRollbackViolation::RollbackStripDegradationNoteMissing,
            );
        }

        let disclosure = strip.recovery_disclosure();

        if strip.recovery_posture_class != disclosure.recovery_posture {
            violations
                .push(ScriptRiskGroupedUpdateRollbackViolation::RecoveryPostureMisrepresented);
        }
        if disclosure.needs_remove_blocked_note && strip.remove_blocked_note.trim().is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::RemoveBlockedNoteMissing);
        }
        if disclosure.needs_regeneration_note && strip.regeneration_note.trim().is_empty() {
            violations.push(ScriptRiskGroupedUpdateRollbackViolation::RegenerationNoteMissing);
        }
        if !strip.rollback_posture_consistent() {
            violations.push(
                ScriptRiskGroupedUpdateRollbackViolation::RollbackStripRollbackPostureInconsistent,
            );
        }
    }

    for required in RecoveryPostureClass::ALL {
        if !recovery_postures.contains(&required) {
            violations
                .push(ScriptRiskGroupedUpdateRollbackViolation::RecoveryPostureCoverageMissing);
            break;
        }
    }
    if !saw_remove_blocked {
        violations.push(ScriptRiskGroupedUpdateRollbackViolation::RemoveBlockedCoverageMissing);
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
