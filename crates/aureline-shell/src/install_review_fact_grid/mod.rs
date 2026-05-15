//! Bounded install-review fact grid wedge on one certified install-like
//! action.
//!
//! ## What the wedge is for
//!
//! Without a forced fact grid, an install / review surface for an
//! extension-bearing or package-bearing action collapses publisher
//! identity, declared permissions, lifecycle posture, install origin,
//! and rollback truth onto a single "Install" button with one bag of
//! marketing copy. The bounded prototype in this module lists those
//! facts side-by-side **before** commit, using the same closed
//! vocabulary as the boundary manifest and the extension
//! manifest-baseline freeze. Chrome reads the
//! resulting [`InstallReviewFactGridRecord`] verbatim; it does not
//! invent local synonyms for any axis.
//!
//! ## Reused vocabularies
//!
//! - [`aureline_extensions::manifest_baseline`] supplies:
//!   - [`PublisherTrustTierClass`] / [`PublisherLifecycleStateClass`]
//!     for the publisher identity row,
//!   - [`ExtensionLifecycleStateClass`] / [`HostContractFamilyClass`]
//!     for the lifecycle / compatibility row,
//!   - [`ManifestOriginSourceClass`] for the origin / mirror row,
//!   - [`PermissionScopeClass`] / [`PermissionScopeEntry`] /
//!     [`EffectivePermissionDiffClass`] for the declared & effective
//!     permission rows, and
//!   - [`InstallDecisionClass`] / [`InstallDecisionReasonClass`] for the
//!     review verdict row.
//! - [`crate::state_cards::DegradedStateToken`] supplies the chrome
//!   chip on rows that cannot proceed at full fidelity.
//!
//! ## What the wedge owns
//!
//! - Two **new** closed vocabularies the upstream manifest baseline does
//!   not own:
//!   - [`ActivationBudgetClass`] — what activation cost the extension
//!     will pay if admitted (eager / lazy / step-up-gated / denied / not
//!     applicable). The install / review surface MUST surface this
//!     before commit so the user is not surprised by a workspace-wide
//!     eager activation.
//!   - [`RollbackPostureClass`] — what removal / revert behaviour the
//!     install ships with (clean uninstall, uninstall with retained
//!     state, quarantine-only, blocked-by-admin, not-applicable). The
//!     install / review surface MUST surface this on every admit
//!     decision so the user knows what removing the extension actually
//!     does.
//! - A closed [`InstallReviewFactGridClaimLimit`] set the chrome quotes
//!   verbatim under every card, so the prototype cannot be mistaken for
//!   a marketplace product.
//! - A closed [`InstallReviewFactGridInvariantViolation`] vocabulary so
//!   a buggy caller that strips publisher identity, drops a permission
//!   rationale, attempts to admit a row with a widening attempt, or
//!   hides the rollback posture under an admit decision lands a typed
//!   failure the chrome MUST surface verbatim.
//!
//! ## Bounded scope (deliberately)
//!
//! - One install-like path only — the certified extension-bearing
//!   prototype path. Marketplace breadth, publisher services, and
//!   compatibility-policy automation stay out of scope.
//! - The wedge is read-only: it projects the upstream manifest-baseline,
//!   effective-permission, and install-decision records into a fact
//!   grid. It does not own the install pipeline, the policy-pack
//!   narrowing engine, or the marketplace discovery surface.

use serde::{Deserialize, Serialize};

use aureline_content_safety::{
    project_content_integrity_warnings, ContentIntegritySurfaceKind, ContentIntegrityWarningRecord,
};
use aureline_extensions::{
    install_review::{ActivationBudget as StructuredActivationBudget, CompatibilityLabel},
    manifest_baseline::{
        validate_manifest_baseline_record, DeclaredVsEffectiveDiffEntry,
        EffectivePermissionBaselineRecord, EffectivePermissionDiffClass,
        ExtensionLifecycleStateClass, ExtensionManifestBaselineRecord, HostContractFamilyClass,
        InstallDecisionClass, InstallDecisionReasonClass, ManifestInstallDecisionRecord,
        ManifestOriginSourceClass, ManifestScopeCompletenessClass, PermissionScopeClass,
        PermissionScopeEntry, PublisherLifecycleStateClass, PublisherTrustTierClass,
    },
};

use crate::state_cards::DegradedStateToken;

/// Stable record-kind tag carried on serialized
/// [`InstallReviewFactGridRecord`] payloads.
pub const INSTALL_REVIEW_FACT_GRID_RECORD_KIND: &str = "install_review_fact_grid_record";

/// Schema version for the [`InstallReviewFactGridRecord`] payload shape.
pub const INSTALL_REVIEW_FACT_GRID_SCHEMA_VERSION: u32 = 1;

/// Prototype label carried on every card. Chrome quotes the token
/// verbatim; the chip MUST NOT be dropped even when the row admits
/// cleanly, because the wedge as a whole is a bounded prototype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrototypeLabel {
    /// Bounded M1 prototype: install-review fact grid on one certified
    /// extension-bearing wedge.
    M1PrototypeInstallReviewFactGrid,
}

impl PrototypeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M1PrototypeInstallReviewFactGrid => "m1_prototype_install_review_fact_grid",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::M1PrototypeInstallReviewFactGrid => {
                "Prototype — install-review fact grid (one certified wedge)"
            }
        }
    }
}

/// Closed activation-budget vocabulary. The install / review surface
/// MUST surface this on every admit decision so the user knows what
/// activation cost the extension will pay if admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationBudgetClass {
    /// Activates eagerly on workspace open; bounded to the current
    /// workspace and never spans beyond it.
    EagerWithinWorkspaceOnly,
    /// Activates only on an explicit user invocation (command, palette,
    /// or recipe).
    LazyOnDemandOnly,
    /// Activates lazily when a subscribed event fires (e.g. a recipe
    /// publishes, a task subscribes).
    LazyOnEventSubscription,
    /// Activation requires a typed step-up under the active policy pack
    /// before the extension can run.
    RestrictedStepUpRequired,
    /// Activation denied entirely by a policy pack; the extension stays
    /// admitted for review but cannot run.
    DeniedByPolicyPack,
    /// Install was denied; activation budget is not applicable.
    NotApplicableInstallDenied,
}

impl ActivationBudgetClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EagerWithinWorkspaceOnly => "eager_within_workspace_only",
            Self::LazyOnDemandOnly => "lazy_on_demand_only",
            Self::LazyOnEventSubscription => "lazy_on_event_subscription",
            Self::RestrictedStepUpRequired => "restricted_step_up_required",
            Self::DeniedByPolicyPack => "denied_by_policy_pack",
            Self::NotApplicableInstallDenied => "not_applicable_install_denied",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::EagerWithinWorkspaceOnly => "Eager (workspace open) — workspace only",
            Self::LazyOnDemandOnly => "Lazy — user invocation only",
            Self::LazyOnEventSubscription => "Lazy — event subscription",
            Self::RestrictedStepUpRequired => "Restricted — step-up required",
            Self::DeniedByPolicyPack => "Denied — policy pack blocks activation",
            Self::NotApplicableInstallDenied => "Not applicable — install denied",
        }
    }

    /// Returns `true` when the class indicates the extension can run
    /// after install. `NotApplicableInstallDenied` and
    /// `DeniedByPolicyPack` return `false`.
    pub const fn permits_activation(self) -> bool {
        matches!(
            self,
            Self::EagerWithinWorkspaceOnly
                | Self::LazyOnDemandOnly
                | Self::LazyOnEventSubscription
                | Self::RestrictedStepUpRequired
        )
    }
}

/// Closed rollback-posture vocabulary. The install / review surface
/// MUST surface this on every admit decision so the user knows what
/// removing the extension actually does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPostureClass {
    /// Uninstall is reversible: removing the extension purges its
    /// state and restores the prior workspace shape.
    CleanUninstallAndStatePurge,
    /// Uninstall removes the extension but retains user-owned local
    /// state (settings, caches, derived files) for re-install.
    UninstallWithUserStateRetained,
    /// Cannot fully remove; the extension is quarantined and disabled
    /// pending publisher review; state remains until publisher resolves.
    QuarantineOnlyPendingPublisherReview,
    /// Admin policy locks the install in place; the user cannot remove
    /// it without an admin action.
    UninstallBlockedPendingAdminReview,
    /// Install not yet admitted; no rollback state to manage.
    NotYetAdmittedNoRollbackNeeded,
}

impl RollbackPostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanUninstallAndStatePurge => "clean_uninstall_and_state_purge",
            Self::UninstallWithUserStateRetained => "uninstall_with_user_state_retained",
            Self::QuarantineOnlyPendingPublisherReview => {
                "quarantine_only_pending_publisher_review"
            }
            Self::UninstallBlockedPendingAdminReview => "uninstall_blocked_pending_admin_review",
            Self::NotYetAdmittedNoRollbackNeeded => "not_yet_admitted_no_rollback_needed",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::CleanUninstallAndStatePurge => "Uninstall — state purged",
            Self::UninstallWithUserStateRetained => "Uninstall — user state retained",
            Self::QuarantineOnlyPendingPublisherReview => {
                "Quarantine only — pending publisher review"
            }
            Self::UninstallBlockedPendingAdminReview => "Uninstall blocked — admin review required",
            Self::NotYetAdmittedNoRollbackNeeded => "Not yet admitted — no rollback",
        }
    }

    /// True when the posture means the extension can be removed by the
    /// user. Quarantine-only / admin-blocked / not-yet-admitted return
    /// `false`.
    pub const fn user_can_remove(self) -> bool {
        matches!(
            self,
            Self::CleanUninstallAndStatePurge | Self::UninstallWithUserStateRetained
        )
    }
}

/// Frozen claim-limit vocabulary the chrome quotes verbatim under every
/// card. The set pins the wedge's M1 scope so chrome cannot imply
/// marketplace or compatibility-policy depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallReviewFactGridClaimLimit {
    /// One bounded install-like wedge only.
    SingleBoundedWedgeOnly,
    /// Not a marketplace; install/uninstall pipeline breadth is out of
    /// scope in M1.
    NoMarketplaceBreadth,
    /// Wedge does not own publisher services (verification, lifecycle
    /// transitions, key management).
    NoPublisherServices,
    /// Wedge does not own broad compatibility-policy automation; it
    /// quotes the host-contract family verbatim.
    NoCompatibilityPolicyAutomation,
}

impl InstallReviewFactGridClaimLimit {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleBoundedWedgeOnly => "single_bounded_wedge_only",
            Self::NoMarketplaceBreadth => "no_marketplace_breadth",
            Self::NoPublisherServices => "no_publisher_services",
            Self::NoCompatibilityPolicyAutomation => "no_compatibility_policy_automation",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::SingleBoundedWedgeOnly => {
                "One bounded install-like wedge only; not a marketplace or installer."
            }
            Self::NoMarketplaceBreadth => {
                "Does not stand up a marketplace, install pipeline, or update channel."
            }
            Self::NoPublisherServices => {
                "Does not own publisher verification, lifecycle transitions, or key management."
            }
            Self::NoCompatibilityPolicyAutomation => {
                "Does not own broad compatibility-policy automation; host-contract family is quoted verbatim."
            }
        }
    }

    /// Canonical M1 claim-limit set. Order is stable; chrome MUST render
    /// in this order.
    pub const fn canonical_set() -> [InstallReviewFactGridClaimLimit; 4] {
        [
            Self::SingleBoundedWedgeOnly,
            Self::NoMarketplaceBreadth,
            Self::NoPublisherServices,
            Self::NoCompatibilityPolicyAutomation,
        ]
    }
}

/// One claim-limit row on the serialized card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewFactGridClaimLimitRow {
    pub token: String,
    pub label: String,
}

impl InstallReviewFactGridClaimLimitRow {
    fn from_limit(limit: InstallReviewFactGridClaimLimit) -> Self {
        Self {
            token: limit.as_str().to_owned(),
            label: limit.label().to_owned(),
        }
    }
}

/// Closed invariant-violation vocabulary surfaced on the card.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "violation")]
pub enum InstallReviewFactGridInvariantViolation {
    /// The card is missing the prototype-label chip.
    MissingPrototypeLabel,
    /// The canonical claim-limit set is missing or out of order.
    ClaimLimitsMissingOrOutOfOrder,
    /// The manifest-baseline record carries the typed
    /// `manifest_baseline.publisher_identity_required` finding (or the
    /// publisher trust tier is `anonymous_publisher_class`).
    PublisherIdentityMissing { extension_identity: String },
    /// The manifest origin source class is `unknown_source_class` — the
    /// fact grid cannot pretend it knows where the row came from.
    OriginSourceMissing { extension_identity: String },
    /// A declared permission has an empty `rationale_label`. The fact
    /// grid would render a row with no explanation; the chrome MUST
    /// refuse to commit.
    DeclaredPermissionRationaleMissing {
        scope_class: String,
        scope_target: String,
    },
    /// The effective-permission summary recorded a widening attempt and
    /// the install decision is anything other than `denied`. Widening
    /// MUST always force a denial.
    WideningAttemptedWithoutDeniedDecision {
        install_decision_class: String,
        widening_count: u32,
    },
    /// The install decision is `admit` / `admit_with_step_up` /
    /// `review_only` but the rollback posture is
    /// `not_yet_admitted_no_rollback_needed`. An admitted row MUST
    /// expose a real rollback posture.
    AdmitWithoutRollbackPosture {
        install_decision_class: String,
        rollback_posture_class: String,
    },
    /// The activation budget class disagrees with the install decision
    /// class (e.g. `eager_within_workspace_only` paired with `denied`).
    ActivationBudgetInconsistentWithDecision {
        activation_budget_class: String,
        install_decision_class: String,
    },
    /// The card carries no effective-permission diff entries but the
    /// manifest declares at least one permission. The chrome would
    /// silently hide what changes between declared and effective.
    EffectivePermissionDiffMissing,
    /// The upstream `validate_manifest_baseline_record` returned typed
    /// findings the fact grid would silently swallow.
    ManifestValidationFindingsPresent { findings_count: u32 },
}

impl InstallReviewFactGridInvariantViolation {
    pub fn token(&self) -> &'static str {
        match self {
            Self::MissingPrototypeLabel => "missing_prototype_label",
            Self::ClaimLimitsMissingOrOutOfOrder => "claim_limits_missing_or_out_of_order",
            Self::PublisherIdentityMissing { .. } => "publisher_identity_missing",
            Self::OriginSourceMissing { .. } => "origin_source_missing",
            Self::DeclaredPermissionRationaleMissing { .. } => {
                "declared_permission_rationale_missing"
            }
            Self::WideningAttemptedWithoutDeniedDecision { .. } => {
                "widening_attempted_without_denied_decision"
            }
            Self::AdmitWithoutRollbackPosture { .. } => "admit_without_rollback_posture",
            Self::ActivationBudgetInconsistentWithDecision { .. } => {
                "activation_budget_inconsistent_with_decision"
            }
            Self::EffectivePermissionDiffMissing => "effective_permission_diff_missing",
            Self::ManifestValidationFindingsPresent { .. } => {
                "manifest_validation_findings_present"
            }
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::MissingPrototypeLabel => "Prototype-label chip is missing.".to_owned(),
            Self::ClaimLimitsMissingOrOutOfOrder => {
                "Canonical claim-limit set is missing or out of order.".to_owned()
            }
            Self::PublisherIdentityMissing { extension_identity } => format!(
                "Publisher identity missing for {extension_identity}; anonymous or ambient \
                 publisher privilege is not acceptable."
            ),
            Self::OriginSourceMissing { extension_identity } => format!(
                "Manifest origin source is unknown for {extension_identity}; install/review \
                 cannot proceed without an attributed origin."
            ),
            Self::DeclaredPermissionRationaleMissing {
                scope_class,
                scope_target,
            } => format!(
                "Declared permission {scope_class}={scope_target} is missing a rationale_label."
            ),
            Self::WideningAttemptedWithoutDeniedDecision {
                install_decision_class,
                widening_count,
            } => format!(
                "Effective-permission widening attempted ({widening_count} scope(s) outside \
                 declared set); install decision is {install_decision_class} but MUST be denied."
            ),
            Self::AdmitWithoutRollbackPosture {
                install_decision_class,
                rollback_posture_class,
            } => format!(
                "Install decision is {install_decision_class} but rollback posture is \
                 {rollback_posture_class}; admitted rows MUST expose a real rollback posture."
            ),
            Self::ActivationBudgetInconsistentWithDecision {
                activation_budget_class,
                install_decision_class,
            } => format!(
                "Activation budget {activation_budget_class} is inconsistent with install \
                 decision {install_decision_class}."
            ),
            Self::EffectivePermissionDiffMissing => "Declared-vs-effective permission diff is \
                                                    missing; chrome would hide what changes."
                .to_owned(),
            Self::ManifestValidationFindingsPresent { findings_count } => format!(
                "Upstream manifest-baseline validation returned {findings_count} typed \
                 finding(s); the fact grid MUST surface them."
            ),
        }
    }
}

/// One invariant row on the serialized card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewFactGridInvariantRow {
    pub violation_token: String,
    pub violation_label: String,
    pub violation: InstallReviewFactGridInvariantViolation,
}

impl InstallReviewFactGridInvariantRow {
    fn from_violation(violation: InstallReviewFactGridInvariantViolation) -> Self {
        Self {
            violation_token: violation.token().to_owned(),
            violation_label: violation.label(),
            violation,
        }
    }
}

/// Publisher facts row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewPublisherFacts {
    pub publisher_identity_ref: String,
    pub publisher_display_label: String,
    pub publisher_trust_tier_class: PublisherTrustTierClass,
    pub publisher_trust_tier_token: String,
    pub publisher_lifecycle_state_class: PublisherLifecycleStateClass,
    pub publisher_lifecycle_state_token: String,
    pub publisher_signing_key_ref: String,
}

/// Origin / source facts row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewOriginFacts {
    pub manifest_origin_source_class: ManifestOriginSourceClass,
    pub manifest_origin_source_token: String,
    pub origin_source_label: String,
}

/// Lifecycle / compatibility facts row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewLifecycleFacts {
    pub extension_lifecycle_state_class: ExtensionLifecycleStateClass,
    pub extension_lifecycle_state_token: String,
    pub host_contract_family_class: HostContractFamilyClass,
    pub host_contract_family_token: String,
    pub manifest_scope_completeness_class: ManifestScopeCompletenessClass,
    pub manifest_scope_completeness_token: String,
}

/// One declared permission row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewDeclaredPermissionRow {
    pub scope_class: PermissionScopeClass,
    pub scope_class_token: String,
    pub scope_target: String,
    pub scope_constraint: Option<String>,
    pub rationale_label: String,
}

impl InstallReviewDeclaredPermissionRow {
    fn from_entry(entry: &PermissionScopeEntry) -> Self {
        Self {
            scope_class: entry.scope_class,
            scope_class_token: scope_class_token(entry.scope_class).to_owned(),
            scope_target: entry.scope_target.clone(),
            scope_constraint: entry.scope_constraint.clone(),
            rationale_label: entry.rationale_label.clone(),
        }
    }
}

/// One declared-vs-effective diff row, mirrored from the upstream
/// effective-permission baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewEffectiveDiffRow {
    pub scope_class: PermissionScopeClass,
    pub scope_class_token: String,
    pub scope_target: String,
    pub diff_class: EffectivePermissionDiffClass,
    pub diff_class_token: String,
    pub narrowing_reason_label: String,
}

impl InstallReviewEffectiveDiffRow {
    fn from_entry(entry: &DeclaredVsEffectiveDiffEntry) -> Self {
        Self {
            scope_class: entry.scope_class,
            scope_class_token: scope_class_token(entry.scope_class).to_owned(),
            scope_target: entry.scope_target.clone(),
            diff_class: entry.diff_class,
            diff_class_token: diff_class_token(entry.diff_class).to_owned(),
            narrowing_reason_label: entry.narrowing_reason_label.clone(),
        }
    }
}

/// Decision row carrying the install / review verdict and the typed
/// install / review reason class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewDecisionFacts {
    pub install_decision_class: InstallDecisionClass,
    pub install_decision_class_token: String,
    pub install_decision_reason_class: InstallDecisionReasonClass,
    pub install_decision_reason_class_token: String,
    pub decision_summary: String,
}

/// Serialized fact-grid record. Chrome quotes verbatim; export and proof
/// flows quote verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallReviewFactGridRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub prototype_label_token: String,
    pub prototype_label_display: String,
    pub wedge_id: String,
    pub manifest_baseline_ref: String,
    pub extension_identity: String,
    pub extension_version: String,
    pub publisher: InstallReviewPublisherFacts,
    pub origin: InstallReviewOriginFacts,
    pub lifecycle: InstallReviewLifecycleFacts,
    /// Compatibility label rendered before commit.
    pub compatibility_label: CompatibilityLabel,
    /// Stable compatibility label token.
    pub compatibility_label_token: String,
    /// Short compatibility label shown in review chrome.
    pub compatibility_label_display: String,
    pub declared_permissions: Vec<InstallReviewDeclaredPermissionRow>,
    pub effective_permission_diff: Vec<InstallReviewEffectiveDiffRow>,
    pub widening_attempted_blocked_count: u32,
    pub activation_budget_class: ActivationBudgetClass,
    pub activation_budget_token: String,
    /// Structured CPU, memory, startup, and feature-gate activation budget.
    pub activation_budget: StructuredActivationBudget,
    pub rollback_posture_class: RollbackPostureClass,
    pub rollback_posture_token: String,
    pub decision: InstallReviewDecisionFacts,
    /// Optional chrome degraded chip, e.g. `Limited` when the row is
    /// review-only or `PolicyBlocked` when the decision is denied for
    /// policy reasons.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    pub claim_limits: Vec<InstallReviewFactGridClaimLimitRow>,
    pub invariants: Vec<InstallReviewFactGridInvariantRow>,
    pub has_invariant_violations: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content_integrity_warnings: Vec<ContentIntegrityWarningRecord>,
    pub summary_line: String,
}

impl InstallReviewFactGridRecord {
    /// Deterministic plaintext block for support exports and proof
    /// captures. Stable across hosts; never bakes in wall-clock time.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {}\n",
            self.prototype_label_token, self.prototype_label_display,
        ));
        out.push_str(&format!(
            "wedge={} extension={} version={}\n",
            self.wedge_id, self.extension_identity, self.extension_version,
        ));
        out.push_str(&format!(
            "manifest_baseline_ref={}\n",
            self.manifest_baseline_ref
        ));
        out.push_str("publisher:\n");
        out.push_str(&format!(
            "  identity={} display={} trust_tier={} lifecycle={} signing_key={}\n",
            self.publisher.publisher_identity_ref,
            self.publisher.publisher_display_label,
            self.publisher.publisher_trust_tier_token,
            self.publisher.publisher_lifecycle_state_token,
            self.publisher.publisher_signing_key_ref,
        ));
        out.push_str("origin:\n");
        out.push_str(&format!(
            "  source={} label={}\n",
            self.origin.manifest_origin_source_token, self.origin.origin_source_label,
        ));
        out.push_str("lifecycle:\n");
        out.push_str(&format!(
            "  extension={} host_contract_family={} manifest_scope_completeness={}\n",
            self.lifecycle.extension_lifecycle_state_token,
            self.lifecycle.host_contract_family_token,
            self.lifecycle.manifest_scope_completeness_token,
        ));
        out.push_str(&format!(
            "compatibility_label={} display={}\n",
            self.compatibility_label_token, self.compatibility_label_display,
        ));
        out.push_str("declared_permissions:\n");
        if self.declared_permissions.is_empty() {
            out.push_str("  - (none)\n");
        } else {
            for row in &self.declared_permissions {
                let constraint = row
                    .scope_constraint
                    .as_deref()
                    .map(|c| format!(" constraint={c}"))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "  - {}={}{} rationale={}\n",
                    row.scope_class_token, row.scope_target, constraint, row.rationale_label,
                ));
            }
        }
        out.push_str("effective_permission_diff:\n");
        if self.effective_permission_diff.is_empty() {
            out.push_str("  - (none)\n");
        } else {
            for row in &self.effective_permission_diff {
                out.push_str(&format!(
                    "  - {}={} diff={} narrowing_reason={}\n",
                    row.scope_class_token,
                    row.scope_target,
                    row.diff_class_token,
                    row.narrowing_reason_label,
                ));
            }
        }
        out.push_str(&format!(
            "widening_attempted_blocked_count={}\n",
            self.widening_attempted_blocked_count
        ));
        out.push_str(&format!(
            "activation_budget={}\n",
            self.activation_budget_token
        ));
        out.push_str("activation_budget_details:\n");
        out.push_str(&format!(
            "  cpu={} memory={} startup_cost_ceiling={} opt_in_feature_gates={}\n",
            self.activation_budget.cpu.as_str(),
            self.activation_budget.memory.as_str(),
            self.activation_budget.startup_cost_ceiling.as_str(),
            self.activation_budget
                .opt_in_feature_gates_or_unknown()
                .join(","),
        ));
        out.push_str(&format!(
            "rollback_posture={}\n",
            self.rollback_posture_token
        ));
        out.push_str("decision:\n");
        out.push_str(&format!(
            "  class={} reason={} summary={}\n",
            self.decision.install_decision_class_token,
            self.decision.install_decision_reason_class_token,
            self.decision.decision_summary,
        ));
        if let Some(token) = &self.degraded_token {
            out.push_str(&format!("degraded={}\n", token));
        }
        out.push_str("claim_limits:\n");
        for row in &self.claim_limits {
            out.push_str(&format!("  - {}: {}\n", row.token, row.label));
        }
        out.push_str("invariants:\n");
        if self.invariants.is_empty() {
            out.push_str("  - clean\n");
        } else {
            for row in &self.invariants {
                out.push_str(&format!(
                    "  - {}: {}\n",
                    row.violation_token, row.violation_label
                ));
            }
        }
        if !self.content_integrity_warnings.is_empty() {
            out.push_str("content_integrity_warnings:\n");
            for warning in &self.content_integrity_warnings {
                out.push_str(&format!(
                    "  - {} {} at char {}\n",
                    warning.record_kind, warning.warning_label, warning.char_offset
                ));
            }
        }
        out.push_str(&format!("summary: {}\n", self.summary_line));
        out
    }

    /// True when the card represents a clean protected-walk admit: no
    /// invariant violations and the install decision is `Admit`.
    pub fn is_clean_admit(&self) -> bool {
        !self.has_invariant_violations
            && matches!(
                self.decision.install_decision_class,
                InstallDecisionClass::Admit
            )
    }
}

/// Bounded install-review fact-grid wedge.
///
/// Construct with [`InstallReviewFactGridWedge::new`], supply the
/// extension manifest-baseline, the effective-permission baseline, the
/// install decision, an [`ActivationBudgetClass`], and a
/// [`RollbackPostureClass`], then call [`Self::card`] to materialise the
/// serialized record.
#[derive(Debug, Clone)]
pub struct InstallReviewFactGridWedge {
    manifest: ExtensionManifestBaselineRecord,
    effective: EffectivePermissionBaselineRecord,
    decision: ManifestInstallDecisionRecord,
    activation_budget: ActivationBudgetClass,
    activation_budget_record: StructuredActivationBudget,
    compatibility_label: CompatibilityLabel,
    rollback_posture: RollbackPostureClass,
    wedge_id: Option<String>,
    degraded_token: Option<DegradedStateToken>,
}

impl InstallReviewFactGridWedge {
    pub fn new(
        manifest: ExtensionManifestBaselineRecord,
        effective: EffectivePermissionBaselineRecord,
        decision: ManifestInstallDecisionRecord,
        activation_budget: ActivationBudgetClass,
        rollback_posture: RollbackPostureClass,
    ) -> Self {
        Self {
            manifest,
            effective,
            decision,
            activation_budget,
            activation_budget_record: StructuredActivationBudget::unknown(),
            compatibility_label: CompatibilityLabel::Unknown,
            rollback_posture,
            wedge_id: None,
            degraded_token: None,
        }
    }

    pub fn with_wedge_id(mut self, wedge_id: impl Into<String>) -> Self {
        self.wedge_id = Some(wedge_id.into());
        self
    }

    pub fn with_degraded(mut self, token: DegradedStateToken) -> Self {
        self.degraded_token = Some(token);
        self
    }

    /// Replaces the default explicit-unknown structured activation budget.
    pub fn with_activation_budget(mut self, budget: StructuredActivationBudget) -> Self {
        self.activation_budget_record = budget;
        self
    }

    /// Replaces the default explicit-unknown compatibility label.
    pub fn with_compatibility_label(mut self, label: CompatibilityLabel) -> Self {
        self.compatibility_label = label;
        self
    }

    /// Materialise the current card.
    pub fn card(&self) -> InstallReviewFactGridRecord {
        let label = PrototypeLabel::M1PrototypeInstallReviewFactGrid;
        let wedge_id = self.wedge_id.clone().unwrap_or_else(|| {
            format!(
                "install_review_fact_grid_wedge:{}:{}",
                self.manifest.extension_identity, self.manifest.extension_version,
            )
        });

        let publisher = InstallReviewPublisherFacts {
            publisher_identity_ref: self.manifest.publisher_identity_ref.clone(),
            publisher_display_label: self.manifest.publisher_display_label.clone(),
            publisher_trust_tier_class: self.manifest.publisher_trust_tier_class,
            publisher_trust_tier_token: publisher_trust_tier_token(
                self.manifest.publisher_trust_tier_class,
            )
            .to_owned(),
            publisher_lifecycle_state_class: self.manifest.publisher_lifecycle_state_class,
            publisher_lifecycle_state_token: publisher_lifecycle_token(
                self.manifest.publisher_lifecycle_state_class,
            )
            .to_owned(),
            publisher_signing_key_ref: self.manifest.publisher_signing_key_ref.clone(),
        };

        let origin = InstallReviewOriginFacts {
            manifest_origin_source_class: self.manifest.manifest_origin_source_class,
            manifest_origin_source_token: origin_source_token(
                self.manifest.manifest_origin_source_class,
            )
            .to_owned(),
            origin_source_label: self.manifest.origin_source_label.clone(),
        };

        let lifecycle = InstallReviewLifecycleFacts {
            extension_lifecycle_state_class: self.manifest.extension_lifecycle_state_class,
            extension_lifecycle_state_token: extension_lifecycle_token(
                self.manifest.extension_lifecycle_state_class,
            )
            .to_owned(),
            host_contract_family_class: self.manifest.host_contract_family_class,
            host_contract_family_token: host_contract_family_token(
                self.manifest.host_contract_family_class,
            )
            .to_owned(),
            manifest_scope_completeness_class: self.manifest.manifest_scope_completeness_class,
            manifest_scope_completeness_token: manifest_scope_completeness_token(
                self.manifest.manifest_scope_completeness_class,
            )
            .to_owned(),
        };

        let declared_permissions: Vec<InstallReviewDeclaredPermissionRow> = self
            .manifest
            .declared_permissions
            .iter()
            .map(InstallReviewDeclaredPermissionRow::from_entry)
            .collect();

        let effective_permission_diff: Vec<InstallReviewEffectiveDiffRow> = self
            .effective
            .declared_vs_effective_diff
            .iter()
            .map(InstallReviewEffectiveDiffRow::from_entry)
            .collect();

        let decision_facts = InstallReviewDecisionFacts {
            install_decision_class: self.decision.install_decision_class,
            install_decision_class_token: install_decision_class_token(
                self.decision.install_decision_class,
            )
            .to_owned(),
            install_decision_reason_class: self.decision.install_decision_reason_class,
            install_decision_reason_class_token: install_decision_reason_class_token(
                self.decision.install_decision_reason_class,
            )
            .to_owned(),
            decision_summary: self.decision.decision_summary.clone(),
        };

        let claim_limits: Vec<InstallReviewFactGridClaimLimitRow> =
            InstallReviewFactGridClaimLimit::canonical_set()
                .into_iter()
                .map(InstallReviewFactGridClaimLimitRow::from_limit)
                .collect();

        let invariants_raw = self.validate_invariants(&declared_permissions);
        let invariants: Vec<InstallReviewFactGridInvariantRow> = invariants_raw
            .into_iter()
            .map(InstallReviewFactGridInvariantRow::from_violation)
            .collect();
        let has_invariant_violations = !invariants.is_empty();
        let content_integrity_warnings = project_install_review_content_integrity(
            &self.manifest.manifest_baseline_id,
            &wedge_id,
            &install_review_content_for_warnings(&self.manifest, &self.decision),
        );
        let summary_line = self.summary_line(has_invariant_violations);

        InstallReviewFactGridRecord {
            record_kind: INSTALL_REVIEW_FACT_GRID_RECORD_KIND.to_owned(),
            schema_version: INSTALL_REVIEW_FACT_GRID_SCHEMA_VERSION,
            prototype_label_token: label.as_str().to_owned(),
            prototype_label_display: label.label().to_owned(),
            wedge_id,
            manifest_baseline_ref: self.manifest.manifest_baseline_id.clone(),
            extension_identity: self.manifest.extension_identity.clone(),
            extension_version: self.manifest.extension_version.clone(),
            publisher,
            origin,
            lifecycle,
            compatibility_label: self.compatibility_label,
            compatibility_label_token: self.compatibility_label.as_str().to_owned(),
            compatibility_label_display: self.compatibility_label.label().to_owned(),
            declared_permissions,
            effective_permission_diff,
            widening_attempted_blocked_count: self.effective.widening_attempted_blocked_count,
            activation_budget_class: self.activation_budget,
            activation_budget_token: self.activation_budget.as_str().to_owned(),
            activation_budget: self.activation_budget_record.clone(),
            rollback_posture_class: self.rollback_posture,
            rollback_posture_token: self.rollback_posture.as_str().to_owned(),
            decision: decision_facts,
            degraded_token: self.degraded_token.map(|t| t.token().to_owned()),
            claim_limits,
            invariants,
            has_invariant_violations,
            content_integrity_warnings,
            summary_line,
        }
    }

    fn summary_line(&self, has_invariant_violations: bool) -> String {
        let suffix = if has_invariant_violations {
            "INVARIANTS BLOCKED"
        } else {
            match self.decision.install_decision_class {
                InstallDecisionClass::Admit => "admit",
                InstallDecisionClass::AdmitWithStepUp => "admit_with_step_up",
                InstallDecisionClass::ReviewOnly => "review_only",
                InstallDecisionClass::Denied => "denied",
            }
        };
        format!(
            "{ext}@{ver} publisher={pub_tier} origin={origin} compatibility={compatibility} decision={dec} — {suffix}",
            ext = self.manifest.extension_identity,
            ver = self.manifest.extension_version,
            pub_tier = publisher_trust_tier_token(self.manifest.publisher_trust_tier_class),
            origin = origin_source_token(self.manifest.manifest_origin_source_class),
            compatibility = self.compatibility_label.as_str(),
            dec = install_decision_class_token(self.decision.install_decision_class),
            suffix = suffix,
        )
    }

    fn validate_invariants(
        &self,
        declared_rows: &[InstallReviewDeclaredPermissionRow],
    ) -> Vec<InstallReviewFactGridInvariantViolation> {
        let mut out = Vec::new();

        let manifest_findings = validate_manifest_baseline_record(&self.manifest);
        if !manifest_findings.is_empty() {
            out.push(
                InstallReviewFactGridInvariantViolation::ManifestValidationFindingsPresent {
                    findings_count: manifest_findings.len() as u32,
                },
            );
        }

        // Publisher identity must be attributable.
        let publisher_missing = self.manifest.publisher_identity_ref.trim().is_empty()
            || matches!(
                self.manifest.publisher_trust_tier_class,
                PublisherTrustTierClass::AnonymousPublisherClass
            );
        if publisher_missing {
            out.push(
                InstallReviewFactGridInvariantViolation::PublisherIdentityMissing {
                    extension_identity: self.manifest.extension_identity.clone(),
                },
            );
        }

        // Manifest origin must be attributable.
        if matches!(
            self.manifest.manifest_origin_source_class,
            ManifestOriginSourceClass::UnknownSourceClass
        ) {
            out.push(
                InstallReviewFactGridInvariantViolation::OriginSourceMissing {
                    extension_identity: self.manifest.extension_identity.clone(),
                },
            );
        }

        // Every declared permission must carry a rationale_label.
        for row in declared_rows {
            if row.rationale_label.trim().is_empty() {
                out.push(
                    InstallReviewFactGridInvariantViolation::DeclaredPermissionRationaleMissing {
                        scope_class: row.scope_class_token.clone(),
                        scope_target: row.scope_target.clone(),
                    },
                );
            }
        }

        // Widening must always force a denial.
        if self.effective.widening_attempted_blocked_count > 0
            && !matches!(
                self.decision.install_decision_class,
                InstallDecisionClass::Denied
            )
        {
            out.push(
                InstallReviewFactGridInvariantViolation::WideningAttemptedWithoutDeniedDecision {
                    install_decision_class: install_decision_class_token(
                        self.decision.install_decision_class,
                    )
                    .to_owned(),
                    widening_count: self.effective.widening_attempted_blocked_count,
                },
            );
        }

        // Admit / review-only / step-up decisions must carry a real
        // rollback posture.
        let admitted = matches!(
            self.decision.install_decision_class,
            InstallDecisionClass::Admit
                | InstallDecisionClass::AdmitWithStepUp
                | InstallDecisionClass::ReviewOnly
        );
        if admitted
            && matches!(
                self.rollback_posture,
                RollbackPostureClass::NotYetAdmittedNoRollbackNeeded
            )
        {
            out.push(
                InstallReviewFactGridInvariantViolation::AdmitWithoutRollbackPosture {
                    install_decision_class: install_decision_class_token(
                        self.decision.install_decision_class,
                    )
                    .to_owned(),
                    rollback_posture_class: self.rollback_posture.as_str().to_owned(),
                },
            );
        }

        // Activation budget must agree with the install decision.
        let decision_class = self.decision.install_decision_class;
        let budget_class = self.activation_budget;
        let inconsistent = match (decision_class, budget_class) {
            (InstallDecisionClass::Denied, ActivationBudgetClass::NotApplicableInstallDenied) => {
                false
            }
            (InstallDecisionClass::Denied, _) => true,
            (
                InstallDecisionClass::AdmitWithStepUp,
                ActivationBudgetClass::RestrictedStepUpRequired,
            ) => false,
            (InstallDecisionClass::AdmitWithStepUp, _) => true,
            (_, ActivationBudgetClass::NotApplicableInstallDenied) => true,
            _ => false,
        };
        if inconsistent {
            out.push(
                InstallReviewFactGridInvariantViolation::ActivationBudgetInconsistentWithDecision {
                    activation_budget_class: budget_class.as_str().to_owned(),
                    install_decision_class: install_decision_class_token(decision_class).to_owned(),
                },
            );
        }

        // The chrome MUST surface a declared-vs-effective diff whenever
        // the manifest declares at least one permission.
        if !self.manifest.declared_permissions.is_empty()
            && self.effective.declared_vs_effective_diff.is_empty()
        {
            out.push(InstallReviewFactGridInvariantViolation::EffectivePermissionDiffMissing);
        }

        out
    }
}

/// Projects shared content-integrity warnings for install-review text fields.
pub fn project_install_review_content_integrity(
    case_id: &str,
    subject_ref: &str,
    review_text: &str,
) -> Vec<ContentIntegrityWarningRecord> {
    project_content_integrity_warnings(
        case_id,
        ContentIntegritySurfaceKind::Package,
        subject_ref,
        review_text,
    )
}

fn install_review_content_for_warnings(
    manifest: &ExtensionManifestBaselineRecord,
    decision: &ManifestInstallDecisionRecord,
) -> String {
    let mut parts = vec![
        manifest.extension_identity.clone(),
        manifest.extension_version.clone(),
        manifest.publisher_identity_ref.clone(),
        manifest.publisher_display_label.clone(),
        manifest.origin_source_label.clone(),
        decision.decision_summary.clone(),
    ];
    for permission in &manifest.declared_permissions {
        parts.push(permission.scope_target.clone());
        parts.push(permission.rationale_label.clone());
    }
    parts.join("\n")
}

// ---------------------------------------------------------------------------
// Token helpers: these stringify the upstream manifest-baseline enums.
// The wedge does not own the vocabulary — these helpers are a thin
// projection to keep the fact grid free of `format!("{:?}")` calls and
// stable across upstream refactors.
// ---------------------------------------------------------------------------

const fn publisher_trust_tier_token(class: PublisherTrustTierClass) -> &'static str {
    match class {
        PublisherTrustTierClass::VerifiedPublisher => "verified_publisher",
        PublisherTrustTierClass::CommunityPublisher => "community_publisher",
        PublisherTrustTierClass::OrganisationalPublisher => "organisational_publisher",
        PublisherTrustTierClass::UnverifiedPublisher => "unverified_publisher",
        PublisherTrustTierClass::QuarantinedPublisher => "quarantined_publisher",
        PublisherTrustTierClass::AnonymousPublisherClass => "anonymous_publisher_class",
    }
}

const fn publisher_lifecycle_token(class: PublisherLifecycleStateClass) -> &'static str {
    match class {
        PublisherLifecycleStateClass::Active => "active",
        PublisherLifecycleStateClass::Preview => "preview",
        PublisherLifecycleStateClass::Deprecated => "deprecated",
        PublisherLifecycleStateClass::Retired => "retired",
        PublisherLifecycleStateClass::Quarantined => "quarantined",
    }
}

const fn extension_lifecycle_token(class: ExtensionLifecycleStateClass) -> &'static str {
    match class {
        ExtensionLifecycleStateClass::Published => "published",
        ExtensionLifecycleStateClass::Preview => "preview",
        ExtensionLifecycleStateClass::Deprecated => "deprecated",
        ExtensionLifecycleStateClass::Retired => "retired",
        ExtensionLifecycleStateClass::Quarantined => "quarantined",
    }
}

const fn origin_source_token(class: ManifestOriginSourceClass) -> &'static str {
    match class {
        ManifestOriginSourceClass::PublicRegistry => "public_registry",
        ManifestOriginSourceClass::PrivateRegistry => "private_registry",
        ManifestOriginSourceClass::Mirror => "mirror",
        ManifestOriginSourceClass::OfflineBundle => "offline_bundle",
        ManifestOriginSourceClass::VendoredLocal => "vendored_local",
        ManifestOriginSourceClass::UnknownSourceClass => "unknown_source_class",
    }
}

const fn host_contract_family_token(class: HostContractFamilyClass) -> &'static str {
    match class {
        HostContractFamilyClass::WasmComponentModel => "wasm_component_model",
        HostContractFamilyClass::WasmCoreModule => "wasm_core_module",
        HostContractFamilyClass::ExternalHostProcess => "external_host_process",
        HostContractFamilyClass::HelperBinary => "helper_binary",
        HostContractFamilyClass::RemoteSideComponent => "remote_side_component",
        HostContractFamilyClass::CompatibilityBridge => "compatibility_bridge",
    }
}

const fn scope_class_token(class: PermissionScopeClass) -> &'static str {
    match class {
        PermissionScopeClass::FilesystemRead => "filesystem_read",
        PermissionScopeClass::FilesystemWrite => "filesystem_write",
        PermissionScopeClass::ShellExecute => "shell_execute",
        PermissionScopeClass::NetworkEgress => "network_egress",
        PermissionScopeClass::AiProviderAccess => "ai_provider_access",
        PermissionScopeClass::ConnectedProviderAccess => "connected_provider_access",
        PermissionScopeClass::SecretHandleUse => "secret_handle_use",
        PermissionScopeClass::WorkspaceSettingsRead => "workspace_settings_read",
        PermissionScopeClass::WorkspaceSettingsWrite => "workspace_settings_write",
        PermissionScopeClass::ExecutionContextBind => "execution_context_bind",
        PermissionScopeClass::SubscriptionSubscribe => "subscription_subscribe",
        PermissionScopeClass::UiCommandContribute => "ui_command_contribute",
        PermissionScopeClass::CapabilityInherit => "capability_inherit",
    }
}

const fn diff_class_token(class: EffectivePermissionDiffClass) -> &'static str {
    match class {
        EffectivePermissionDiffClass::Unchanged => "unchanged",
        EffectivePermissionDiffClass::Narrowed => "narrowed",
        EffectivePermissionDiffClass::Denied => "denied",
        EffectivePermissionDiffClass::StepUpRequired => "step_up_required",
        EffectivePermissionDiffClass::WideningAttemptedBlocked => "widening_attempted_blocked",
    }
}

const fn manifest_scope_completeness_token(class: ManifestScopeCompletenessClass) -> &'static str {
    match class {
        ManifestScopeCompletenessClass::Complete => "complete",
        ManifestScopeCompletenessClass::IncompletePublisherMissing => {
            "incomplete_publisher_missing"
        }
        ManifestScopeCompletenessClass::IncompleteOriginMissing => "incomplete_origin_missing",
        ManifestScopeCompletenessClass::IncompletePermissionRationaleMissing => {
            "incomplete_permission_rationale_missing"
        }
        ManifestScopeCompletenessClass::IncompleteLifecycleUnknown => {
            "incomplete_lifecycle_unknown"
        }
    }
}

const fn install_decision_class_token(class: InstallDecisionClass) -> &'static str {
    match class {
        InstallDecisionClass::Admit => "admit",
        InstallDecisionClass::AdmitWithStepUp => "admit_with_step_up",
        InstallDecisionClass::ReviewOnly => "review_only",
        InstallDecisionClass::Denied => "denied",
    }
}

const fn install_decision_reason_class_token(class: InstallDecisionReasonClass) -> &'static str {
    match class {
        InstallDecisionReasonClass::AdmittedNoViolation => "admitted_no_violation",
        InstallDecisionReasonClass::StepUpRequiredByPolicyPack => "step_up_required_by_policy_pack",
        InstallDecisionReasonClass::ReviewOnlyUnverifiedPublisher => {
            "review_only_unverified_publisher"
        }
        InstallDecisionReasonClass::PublisherIdentityRequired => "publisher_identity_required",
        InstallDecisionReasonClass::PublisherAnonymous => "publisher_anonymous",
        InstallDecisionReasonClass::PublisherQuarantined => "publisher_quarantined",
        InstallDecisionReasonClass::PublisherLifecycleRetired => "publisher_lifecycle_retired",
        InstallDecisionReasonClass::ExtensionLifecycleRetired => "extension_lifecycle_retired",
        InstallDecisionReasonClass::ManifestScopeIncomplete => "manifest_scope_incomplete",
        InstallDecisionReasonClass::ManifestOriginUnknown => "manifest_origin_unknown",
        InstallDecisionReasonClass::DeclaredPermissionRationaleRequired => {
            "declared_permission_rationale_required"
        }
        InstallDecisionReasonClass::EffectivePermissionWideningAttempted => {
            "effective_permission_widening_attempted"
        }
        InstallDecisionReasonClass::LifecycleStateUnknownClass => "lifecycle_state_unknown_class",
    }
}

#[cfg(test)]
mod tests;
