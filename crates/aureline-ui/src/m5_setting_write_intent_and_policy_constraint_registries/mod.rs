//! Implemented M5 setting-write-intent and policy/constraint registries.
//!
//! The frozen [settings-governance matrix][matrix] names Aureline's five configuration-runtime families and
//! locks their controlled vocabulary. This is the write-pipeline implement lane over the `write_setting`
//! family: it turns the *setting-write-intent* grammar (how a configuration mutation declares the scope,
//! artifact, actor, reason, preview class, and recovery evidence it will land) and the *policy / constraint*
//! grammar (how a locked or denied write explains itself with a lock source, allowed override classes,
//! expiry / review, fallback guidance, and docs pointers) into registry resolvers that produce export-safe,
//! honest projections. Every claimed M5 configuration mutation then resolves to one setting-write-intent
//! object — the preview class it classifies, the target scope and artifact it lands in (never silently widened
//! into a broader scope because it is easier downstream), the intended value, the actor, the change reason, the
//! preview reference, and the checkpoint / rollback recovery reference — and to one policy / constraint object —
//! the lock source, the allowed override classes, the expiry / review window, the validation status, the review
//! state, the docs pointer, and the last review revision — that the settings, shell, diagnostics, admin, and
//! support / export surfaces can inspect without manual reconstruction, so a scoped write is never rewritten
//! into a broader scope or an unintended artifact, a high-risk write always materializes preview / checkpoint /
//! rollback evidence before it applies, a locked or denied write always names its cause and fallback, and a
//! configuration route that cannot explain where a mutation lands or why a write is locked degrades honestly
//! instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one setting-write-intent object per mutation.** [`resolve_setting_write_intent_entry`] refuses to
//!   read as a clean, registry-bound write-intent entry unless it names a canonical registry token, a classified
//!   [preview class][M5WriteIntentPreviewClass], a settings-governance role, covers every
//!   [resolution form][M5ConfigWriteResolutionForm] (the canonical object, the accessible summary, and the audit
//!   record), publishes every write-intent field (target scope, target artifact, intended value, actor, change
//!   reason, preview reference, and checkpoint / rollback recovery reference), lands only in the chosen scope and
//!   artifact, and materializes the recovery evidence before a high-risk write applies; otherwise it degrades.
//! * **Keep the write intent from rewriting scope or hiding recovery evidence.**
//!   [`write_intent_lands_in_chosen_scope`] rejects a write-intent entry whose chosen scope / artifact ownership
//!   was rewritten into a broader scope so it degrades to
//!   [`M5SettingWriteIntentEntryDegradeReason::WriteIntentRewritesScopeOrHidesRecoveryEvidence`], and a high-risk
//!   write that has not materialized its preview / checkpoint / rollback evidence degrades the same way.
//! * **Keep the policy / constraint from masking the lock source or hiding the fallback.**
//!   [`resolve_policy_constraint_entry`] names a classified [lock class][M5PolicyLockClass], requires the full
//!   lock-source / allowed-override-classes / expiry-review / validation-status / review-state / docs-pointer /
//!   last-review-revision policy-constraint object, covers every resolution form, and degrades to
//!   [`M5PolicyConstraintEntryDegradeReason::PolicyConstraintMasksLockSourceOrHidesFallback`] when the record
//!   would mask a locked value without disclosing its lock source or deny a write without disclosing the
//!   fallback guidance, so a locked or denied write can never read as trustworthy when it has quietly dropped
//!   the reason it is locked or the route the user still has.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5SettingsGovernanceRole`] role
//! vocabulary and the [`M5SettingsGovernanceConsumerSurface`] consumer-surface taxonomy — so the settings,
//! shell, diagnostics, admin, sync, policy, capability, docs, CLI, and support surfaces can never fork their
//! own write-intent or policy meaning. Raw secret values and private endpoints stay outside the export
//! boundary.
//!
//! [matrix]: crate::m5_settings_governance_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_setting_write_intent_and_policy_constraint_registries,
    seeded_m5_setting_write_intent_and_policy_constraint_registries_policy_constraint_preview_narrowed,
    seeded_m5_setting_write_intent_and_policy_constraint_registries_write_intent_beta_narrowed,
    M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_settings_governance_matrix::{
    M5SettingsGovernanceAccessibilityRoute, M5SettingsGovernanceConsumerSurface,
    M5SettingsGovernanceDeploymentLine, M5SettingsGovernanceDowngradeTrigger,
    M5SettingsGovernanceFamily, M5SettingsGovernanceQualificationClass,
    M5SettingsGovernanceRequiredLabel, M5SettingsGovernanceRole,
    M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF, M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
    M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SettingWriteIntentPolicyConstraintRegistriesPacket`].
pub const M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_setting_write_intent_and_policy_constraint_registries";

/// Schema version for M5 setting-write-intent / policy-constraint registry records.
pub const M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_SCHEMA_REF: &str =
    "schemas/config/m5-setting-write-intent-and-policy-constraint-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_DOC_REF: &str =
    "docs/settings/m5_setting_write_intent_and_policy_constraint_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-setting-write-intent-and-policy-constraint-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-setting-write-intent-and-policy-constraint-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-setting-write-intent-and-policy-constraint-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/config/m5-setting-write-intent-and-policy-constraint-registries";

/// Repo-relative path of the already-landed policy-decision-explain schema the policy / constraint registry
/// binds back to, so a locked or denied write's structured reason and fallback trace to one canonical policy
/// contract rather than a lane-local invention.
pub const M5_POLICY_CONSTRAINT_LANDED_SCHEMA_REF: &str =
    "schemas/governance/policy_decision_explain.schema.json";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5SettingWriteIntentPolicyConstraintRegistriesConsumerSurface =
    M5SettingsGovernanceConsumerSurface;

/// One of the three resolution forms every setting-write-intent or policy-constraint entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary,
/// or written to the audit / support record. Minted by this lane because the frozen matrix names the
/// write-setting *family* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigWriteResolutionForm {
    /// The canonical resolved setting-write-intent / policy-constraint object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved write intent discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved write intent inspectable off-renderer.
    AuditRecord,
}

impl M5ConfigWriteResolutionForm {
    /// Every resolution form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::CanonicalObject,
        Self::AccessibleSummary,
        Self::AuditRecord,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObject => "canonical_object",
            Self::AccessibleSummary => "accessible_summary",
            Self::AuditRecord => "audit_record",
        }
    }
}

/// Controlled preview / risk class a setting-write-intent entry declares, so the write-intent model shares one
/// registry rather than a hand-copied per-mutation assumption of how risky a change is. Minted by this lane
/// because the frozen matrix carries the configuration families but not the concrete no-op / low-risk /
/// material / high-risk / destructive preview class a write intent classifies against. Every classified class
/// carries its canonical class mode, and the material / high-risk / destructive classes are behavior-changing
/// so they must materialize preview / checkpoint / rollback evidence before the write applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteIntentPreviewClass {
    /// A reversible no-op: the effective behavior does not change materially.
    NoOpReversible,
    /// A reversible low-risk change.
    LowRiskReversible,
    /// A change that materially alters effective behavior (evidence-bearing).
    MaterialBehaviorChange,
    /// A high-risk, hard-to-reverse change (evidence-bearing).
    HighRiskIrreversible,
    /// A destructive reset that clears user state (evidence-bearing).
    DestructiveReset,
    /// The preview class is unclassified, which is disallowed.
    PreviewClassUnclassified,
}

impl M5WriteIntentPreviewClass {
    /// Every preview class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoOpReversible,
        Self::LowRiskReversible,
        Self::MaterialBehaviorChange,
        Self::HighRiskIrreversible,
        Self::DestructiveReset,
        Self::PreviewClassUnclassified,
    ];

    /// The five canonical preview classes every claimed M5 write intent classifies against.
    pub const CANONICAL_CLASSES: [Self; 5] = [
        Self::NoOpReversible,
        Self::LowRiskReversible,
        Self::MaterialBehaviorChange,
        Self::HighRiskIrreversible,
        Self::DestructiveReset,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoOpReversible => "no_op_reversible",
            Self::LowRiskReversible => "low_risk_reversible",
            Self::MaterialBehaviorChange => "material_behavior_change",
            Self::HighRiskIrreversible => "high_risk_irreversible",
            Self::DestructiveReset => "destructive_reset",
            Self::PreviewClassUnclassified => "preview_class_unclassified",
        }
    }

    /// Whether the class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::PreviewClassUnclassified)
    }

    /// The canonical class mode for this preview class.
    pub const fn canonical_class_mode(self) -> &'static str {
        match self {
            Self::NoOpReversible => "no_op_reversible_class",
            Self::LowRiskReversible => "low_risk_reversible_class",
            Self::MaterialBehaviorChange => "material_behavior_change_class",
            Self::HighRiskIrreversible => "high_risk_irreversible_class",
            Self::DestructiveReset => "destructive_reset_class",
            Self::PreviewClassUnclassified => "",
        }
    }

    /// Whether this class is behavior-changing and so must materialize preview / checkpoint / rollback evidence
    /// before the write applies.
    pub const fn is_high_risk_class(self) -> bool {
        matches!(
            self,
            Self::MaterialBehaviorChange | Self::HighRiskIrreversible | Self::DestructiveReset
        )
    }
}

/// Controlled lock class a policy / constraint entry must resolve, so a locked or denied write shares one
/// registry rather than a hand-copied per-record assumption. Minted by this lane, tracking the policy-locked /
/// override-allowed / advisory lock dispositions the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PolicyLockClass {
    /// The write is policy-locked and cannot be overridden.
    PolicyLocked,
    /// The write is locked but may be overridden with a disclosed override class and reason.
    OverrideAllowed,
    /// The write is only advisory-constrained (a soft constraint).
    AdvisoryConstraint,
    /// The lock class is unclassified, which is disallowed.
    LockClassUnclassified,
}

impl M5PolicyLockClass {
    /// Every lock class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PolicyLocked,
        Self::OverrideAllowed,
        Self::AdvisoryConstraint,
        Self::LockClassUnclassified,
    ];

    /// The three canonical lock classes every policy / constraint must stay distinct across.
    pub const CANONICAL_CLASSES: [Self; 3] = [
        Self::PolicyLocked,
        Self::OverrideAllowed,
        Self::AdvisoryConstraint,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyLocked => "policy_locked",
            Self::OverrideAllowed => "override_allowed",
            Self::AdvisoryConstraint => "advisory_constraint",
            Self::LockClassUnclassified => "lock_class_unclassified",
        }
    }

    /// Whether the lock class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::LockClassUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a write-intent or
/// policy-constraint token's meaning stays stable whether it appears in the settings, shell, diagnostics,
/// admin, or a support / export form. Minted by this lane, tracking the first-consumer surfaces the
/// implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigWriteSurfaceContext {
    /// The settings surface.
    SettingsSurface,
    /// The shell surface.
    ShellSurface,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The admin surface.
    AdminSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5ConfigWriteSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SettingsSurface,
        Self::ShellSurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::SettingsSurface,
        Self::ShellSurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsSurface => "settings_surface",
            Self::ShellSurface => "shell_surface",
            Self::DiagnosticsSurface => "diagnostics_surface",
            Self::AdminSurface => "admin_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a setting-write-intent or policy-constraint entry must be able to show, so no
/// preview class, target scope, target artifact, recovery evidence, policy-constraint field, or registry fact
/// is left implicit behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigWriteAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The preview class the entry classifies (write-intent entry).
    WriteIntentPreviewClass,
    /// The target scope and target artifact the write intent lands in (write-intent entry).
    TargetScopeAndArtifact,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The preview reference and checkpoint / rollback recovery reference the entry publishes (write-intent
    /// entry).
    PreviewAndRecoveryEvidence,
    /// The policy-constraint fields (lock source, allowed override classes, expiry / review, validation, review
    /// state, docs pointer) the entry publishes (policy-constraint entry).
    PolicyConstraintFields,
    /// The fallback-guidance hint the entry publishes (policy-constraint entry).
    FallbackGuidanceHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved write intent or policy constraint (both entries).
    PlainLanguageMeaning,
}

impl M5ConfigWriteAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::WriteIntentPreviewClass,
        Self::TargetScopeAndArtifact,
        Self::ResolutionFormCoverage,
        Self::PreviewAndRecoveryEvidence,
        Self::PolicyConstraintFields,
        Self::FallbackGuidanceHint,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::WriteIntentPreviewClass => "write_intent_preview_class",
            Self::TargetScopeAndArtifact => "target_scope_and_artifact",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::PreviewAndRecoveryEvidence => "preview_and_recovery_evidence",
            Self::PolicyConstraintFields => "policy_constraint_fields",
            Self::FallbackGuidanceHint => "fallback_guidance_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// write intent, a policy constraint, or a degraded write-intent / policy-constraint entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigWriteNextAction {
    /// Expand the resolved write intent's or policy constraint's plain-language meaning.
    ExpandWriteIntentMeaning,
    /// Inspect the preview class or lock class the entry resolves.
    InspectClassOrLock,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5ConfigWriteNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandWriteIntentMeaning,
        Self::InspectClassOrLock,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandWriteIntentMeaning => "expand_write_intent_meaning",
            Self::InspectClassOrLock => "inspect_class_or_lock",
            Self::CompleteResolutionFormCoverage => "complete_resolution_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigWriteExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The settings-governance families covered.
    SettingsGovernanceFamilies,
    /// The write-intent preview classes carried.
    WriteIntentPreviewClasses,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The policy lock classes carried.
    PolicyLockClasses,
    /// The render / surface context.
    SurfaceContext,
    /// The class modes carried.
    PreviewClassModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ConfigWriteExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::SettingsGovernanceFamilies,
        Self::WriteIntentPreviewClasses,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::PolicyLockClasses,
        Self::SurfaceContext,
        Self::PreviewClassModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::SettingsGovernanceFamilies,
        Self::WriteIntentPreviewClasses,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::SettingsGovernanceFamilies => "settings_governance_families",
            Self::WriteIntentPreviewClasses => "write_intent_preview_classes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::PolicyLockClasses => "policy_lock_classes",
            Self::SurfaceContext => "surface_context",
            Self::PreviewClassModes => "preview_class_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a setting-write-intent entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, scope-rewriting, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingWriteIntentEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the write intent means.
    WriteIntentTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The preview class is unclassified (not in the resolved taxonomy).
    PreviewClassUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    WriteIntentNotBoundToRegistry,
    /// The resolved write-intent object is incomplete: the target scope, target artifact, intended value, actor,
    /// change reason, preview reference, or checkpoint / rollback recovery reference is unstated.
    WriteIntentObjectIncomplete,
    /// The chosen scope / artifact ownership was rewritten into a broader scope, or a high-risk write hid its
    /// preview / checkpoint / rollback recovery evidence behind generic copy.
    WriteIntentRewritesScopeOrHidesRecoveryEvidence,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A high-risk write did not materialize the preview / checkpoint / rollback recovery evidence before it
    /// applied.
    RecoveryEvidenceNotMaterializedForHighRiskWrite,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SettingWriteIntentEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::WriteIntentTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::PreviewClassUnclassified,
        Self::WriteIntentNotBoundToRegistry,
        Self::WriteIntentObjectIncomplete,
        Self::WriteIntentRewritesScopeOrHidesRecoveryEvidence,
        Self::ResolutionFormCoverageIncomplete,
        Self::RecoveryEvidenceNotMaterializedForHighRiskWrite,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WriteIntentTokenUnstated => "write_intent_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::PreviewClassUnclassified => "preview_class_unclassified",
            Self::WriteIntentNotBoundToRegistry => "write_intent_not_bound_to_registry",
            Self::WriteIntentObjectIncomplete => "write_intent_object_incomplete",
            Self::WriteIntentRewritesScopeOrHidesRecoveryEvidence => {
                "write_intent_rewrites_scope_or_hides_recovery_evidence"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::RecoveryEvidenceNotMaterializedForHighRiskWrite => {
                "recovery_evidence_not_materialized_for_high_risk_write"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ConfigWriteNextAction {
        match self {
            Self::WriteIntentTokenUnstated | Self::WriteIntentNotBoundToRegistry => {
                M5ConfigWriteNextAction::TraceCanonicalRegistry
            }
            Self::PreviewClassUnclassified
            | Self::WriteIntentObjectIncomplete
            | Self::WriteIntentRewritesScopeOrHidesRecoveryEvidence => {
                M5ConfigWriteNextAction::InspectClassOrLock
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5ConfigWriteNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RecoveryEvidenceNotMaterializedForHighRiskWrite
            | Self::ProofStale => M5ConfigWriteNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SettingsGovernanceDowngradeTrigger {
        match self {
            Self::WriteIntentTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::PreviewClassUnclassified | Self::WriteIntentObjectIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::WriteIntentNotBoundToRegistry => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::WriteIntentRewritesScopeOrHidesRecoveryEvidence
            | Self::RecoveryEvidenceNotMaterializedForHighRiskWrite => {
                M5SettingsGovernanceDowngradeTrigger::RewroteAScopedWriteIntoABroaderScope
            }
            Self::ProofStale => M5SettingsGovernanceDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a policy / constraint entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PolicyConstraintEntryDegradeReason {
    /// The canonical registry token name is unstated.
    ConstraintTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The lock class is unclassified (not in the resolved taxonomy).
    LockClassUnclassified,
    /// The policy / constraint would mask a locked value without disclosing its lock source, deny a write
    /// without disclosing the fallback guidance, or it dropped one of the required policy-constraint fields
    /// (lock source, allowed override classes, expiry / review, validation, review state, docs pointer, last
    /// review revision).
    PolicyConstraintMasksLockSourceOrHidesFallback,
    /// The canonical / accessible / audit resolution-form coverage of the record is incomplete.
    ConstraintFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PolicyConstraintEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ConstraintTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::LockClassUnclassified,
        Self::PolicyConstraintMasksLockSourceOrHidesFallback,
        Self::ConstraintFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConstraintTokenUnstated => "constraint_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::LockClassUnclassified => "lock_class_unclassified",
            Self::PolicyConstraintMasksLockSourceOrHidesFallback => {
                "policy_constraint_masks_lock_source_or_hides_fallback"
            }
            Self::ConstraintFormCoverageIncomplete => "constraint_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ConfigWriteNextAction {
        match self {
            Self::ConstraintTokenUnstated => M5ConfigWriteNextAction::TraceCanonicalRegistry,
            Self::LockClassUnclassified | Self::PolicyConstraintMasksLockSourceOrHidesFallback => {
                M5ConfigWriteNextAction::InspectClassOrLock
            }
            Self::ConstraintFormCoverageIncomplete => {
                M5ConfigWriteNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ConfigWriteNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SettingsGovernanceDowngradeTrigger {
        match self {
            Self::ConstraintTokenUnstated => {
                M5SettingsGovernanceDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::LockClassUnclassified => {
                M5SettingsGovernanceDowngradeTrigger::WriteIntentUnstated
            }
            Self::PolicyConstraintMasksLockSourceOrHidesFallback => {
                M5SettingsGovernanceDowngradeTrigger::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy
            }
            Self::ConstraintFormCoverageIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::ProofStale => M5SettingsGovernanceDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_setting_write_intent_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SettingWriteIntentEntryResolutionInput {
    /// Stable identity of the write-intent-registry entry.
    pub entry_id: String,
    /// The stable write-target ID this intent binds to (e.g. `settings.acme.editor.font-size@workspace`); empty
    /// means unstated.
    pub write_target_id: String,
    /// The canonical registry token name (e.g. `write.intent.editor.font_size`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5SettingsGovernanceRole,
    /// The preview class this entry classifies.
    pub preview_class: M5WriteIntentPreviewClass,
    /// The render / surface context.
    pub surface_context: M5ConfigWriteSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5ConfigWriteResolutionForm>,
    /// The published target scope the write lands in; empty means unstated.
    pub target_scope: String,
    /// The published target artifact the write lands in; empty means unstated.
    pub target_artifact: String,
    /// The published intended value (redacted where sensitive); empty means unstated.
    pub intended_value: String,
    /// The published actor / route that raised the write; empty means unstated.
    pub actor: String,
    /// The published change reason; empty means unstated.
    pub change_reason: String,
    /// The published preview reference; empty means unstated.
    pub preview_reference: String,
    /// The published checkpoint / rollback recovery reference; empty means unstated.
    pub recovery_reference: String,
    /// True when the behavior traces to the write-intent registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the write lands only in the chosen scope / artifact and is never rewritten into a broader scope
    /// (a hard invariant when `false`).
    pub scope_ownership_preserved: bool,
    /// True when this write is high-risk / behavior-changing.
    pub is_high_risk_write: bool,
    /// True when the preview / checkpoint / rollback recovery evidence is materialized before a high-risk write
    /// applies.
    pub evidence_materialized: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe write-intent-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSettingWriteIntentEntry {
    /// Stable identity of the write-intent-registry entry.
    pub entry_id: String,
    /// The stable write-target ID this intent binds to.
    pub write_target_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve evidence and disclose cause before applying.
    pub semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: bool,
    /// The preview-class token named by the entry.
    pub preview_class: String,
    /// Whether the preview class is classified into the resolved taxonomy.
    pub preview_class_is_classified: bool,
    /// The canonical class mode for the entry's preview class.
    pub canonical_class_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published target scope.
    pub target_scope: String,
    /// The published target artifact.
    pub target_artifact: String,
    /// The published intended value.
    pub intended_value: String,
    /// The published actor / route.
    pub actor: String,
    /// The published change reason.
    pub change_reason: String,
    /// The published preview reference.
    pub preview_reference: String,
    /// The published checkpoint / rollback recovery reference.
    pub recovery_reference: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved write-intent object publishes every required field.
    pub write_intent_object_complete: bool,
    /// Whether the entry traces to the write-intent registry.
    pub bound_to_registry: bool,
    /// Whether the write lands only in the chosen scope / artifact (never rewritten into a broader scope).
    pub scope_ownership_preserved: bool,
    /// Whether this write is high-risk / behavior-changing.
    pub is_high_risk_write: bool,
    /// Whether the preview / checkpoint / rollback recovery evidence is materialized before the write applies.
    pub evidence_materialized: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5SettingWriteIntentEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ConfigWriteNextAction,
    /// Whether the write intent resolves to one object across every claimed route (clean entry naming every
    /// fact).
    pub write_intent_lands_across_routes: bool,
}

impl M5ResolvedSettingWriteIntentEntry {
    /// Whether this write-intent entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_policy_constraint_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PolicyConstraintEntryResolutionInput {
    /// Stable identity of the policy-constraint entry.
    pub entry_id: String,
    /// The stable constraint-ref this record binds to; empty means unstated.
    pub constraint_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5SettingsGovernanceRole,
    /// The lock class this record must resolve.
    pub lock_class: M5PolicyLockClass,
    /// The render / surface context.
    pub surface_context: M5ConfigWriteSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5ConfigWriteResolutionForm>,
    /// The published lock source; empty means missing.
    pub lock_source: String,
    /// The published allowed override classes; empty means missing.
    pub allowed_override_classes: String,
    /// The published expiry / review window; empty means missing.
    pub expiry_review: String,
    /// The published validation status; empty means missing.
    pub validation_status: String,
    /// The published review state; empty means missing.
    pub review_state: String,
    /// The published docs pointer; empty means missing.
    pub docs_pointer: String,
    /// The published last review revision; empty means missing.
    pub last_review_revision: String,
    /// True when the record keeps the lock source visible.
    pub keeps_lock_source_visible: bool,
    /// True when the constraint is truthful (never claims a clean resolution over a masked lock).
    pub constraint_is_truthful: bool,
    /// True when the write is locked or constrained.
    pub lock_present: bool,
    /// True when a locked value discloses its lock source (never masks the lock).
    pub lock_source_disclosed: bool,
    /// True when a write is denied.
    pub denial_present: bool,
    /// True when a denied write discloses its fallback guidance rather than ambiguous failure copy.
    pub fallback_guidance_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe policy-constraint projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPolicyConstraintEntry {
    /// Stable identity of the policy-constraint entry.
    pub entry_id: String,
    /// The stable constraint-ref this record binds to.
    pub constraint_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve evidence and disclose cause before applying.
    pub semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: bool,
    /// The lock-class token named by the entry.
    pub lock_class: String,
    /// Whether the lock class is classified into the resolved taxonomy.
    pub lock_class_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published lock source.
    pub lock_source: String,
    /// The published allowed override classes.
    pub allowed_override_classes: String,
    /// The published expiry / review window.
    pub expiry_review: String,
    /// The published validation status.
    pub validation_status: String,
    /// The published review state.
    pub review_state: String,
    /// The published docs pointer.
    pub docs_pointer: String,
    /// The published last review revision.
    pub last_review_revision: String,
    /// Whether the record keeps the lock source visible.
    pub keeps_lock_source_visible: bool,
    /// Whether the constraint is truthful.
    pub constraint_is_truthful: bool,
    /// Whether the write is locked or constrained.
    pub lock_present: bool,
    /// Whether a locked value discloses its lock source.
    pub lock_source_disclosed: bool,
    /// Whether a write is denied.
    pub denial_present: bool,
    /// Whether a denied write discloses its fallback guidance.
    pub fallback_guidance_disclosed: bool,
    /// Whether the record stays honest (lock source visible, lock source disclosed, fallback guidance
    /// disclosed).
    pub policy_constraint_stays_honest: bool,
    /// Whether the entry provides the complete policy-constraint object (lock source, allowed override classes,
    /// expiry / review, validation, review state, docs pointer, last review revision).
    pub provides_complete_policy_constraint: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5PolicyConstraintEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ConfigWriteNextAction,
    /// Whether the policy constraint is safe on every claimed route (clean entry naming every fact).
    pub constraint_safe_on_every_route: bool,
}

impl M5ResolvedPolicyConstraintEntry {
    /// Whether this policy-constraint entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ConfigWriteResolutionError {
    /// The write-intent-entry id was empty.
    EmptyWriteIntentEntryId,
    /// The policy-constraint-entry id was empty.
    EmptyPolicyConstraintEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ConfigWriteResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyWriteIntentEntryId => "empty_write_intent_entry_id",
            Self::EmptyPolicyConstraintEntryId => "empty_policy_constraint_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ConfigWriteResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 setting-write-intent / policy-constraint registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ConfigWriteResolutionError {}

fn form_tokens(forms: &[M5ConfigWriteResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5ConfigWriteResolutionForm]) -> bool {
    let present: BTreeSet<M5ConfigWriteResolutionForm> = forms.iter().copied().collect();
    M5ConfigWriteResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved write-intent object publishes every required field: declared preview class (via a
/// classified class), target scope, target artifact, intended value, actor, change reason, preview reference,
/// and checkpoint / rollback recovery reference. An unclassified class or any empty field never resolves to a
/// complete object.
#[allow(clippy::too_many_arguments)]
pub fn write_intent_object_is_complete(
    class: M5WriteIntentPreviewClass,
    target_scope: &str,
    target_artifact: &str,
    intended_value: &str,
    actor: &str,
    change_reason: &str,
    preview_reference: &str,
    recovery_reference: &str,
) -> bool {
    class.is_classified()
        && !target_scope.trim().is_empty()
        && !target_artifact.trim().is_empty()
        && !intended_value.trim().is_empty()
        && !actor.trim().is_empty()
        && !change_reason.trim().is_empty()
        && !preview_reference.trim().is_empty()
        && !recovery_reference.trim().is_empty()
}

/// Whether the write intent lands where it was chosen: the class must be classified, the chosen scope / artifact
/// ownership must be preserved (never rewritten into a broader scope), and a high-risk write must materialize the
/// preview / checkpoint / rollback recovery evidence before it applies. An unclassified class, a rewritten
/// scope, or hidden recovery evidence never matches.
pub fn write_intent_lands_in_chosen_scope(
    class: M5WriteIntentPreviewClass,
    scope_ownership_preserved: bool,
    is_high_risk_write: bool,
    evidence_materialized: bool,
) -> bool {
    class.is_classified()
        && scope_ownership_preserved
        && (!is_high_risk_write || evidence_materialized)
}

/// Whether a policy / constraint stays honest: the lock class must be classified, the constraint must be
/// truthful, it must keep the lock source visible, any locked value must disclose its lock source rather than
/// mask it, and any denied write must disclose its fallback guidance rather than read as ambiguous failure copy.
pub fn policy_constraint_stays_honest(
    class: M5PolicyLockClass,
    constraint_is_truthful: bool,
    keeps_lock_source_visible: bool,
    lock_present: bool,
    lock_source_disclosed: bool,
    denial_present: bool,
    fallback_guidance_disclosed: bool,
) -> bool {
    class.is_classified()
        && constraint_is_truthful
        && keeps_lock_source_visible
        && (!lock_present || lock_source_disclosed)
        && (!denial_present || fallback_guidance_disclosed)
}

/// Resolves a write-intent-registry entry so it stays bound to the write-intent registry: the entry names its
/// canonical token, semantic role, and preview class, covers all three resolution forms, publishes a complete
/// write-intent object (target scope, target artifact, intended value, actor, change reason, preview reference,
/// checkpoint / rollback recovery reference), lands only in the chosen scope / artifact, and materializes the
/// recovery evidence before a high-risk write applies.
pub fn resolve_setting_write_intent_entry(
    input: M5SettingWriteIntentEntryResolutionInput,
) -> Result<M5ResolvedSettingWriteIntentEntry, M5ConfigWriteResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ConfigWriteResolutionError::EmptyWriteIntentEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.write_target_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.target_scope)
        || string_is_forbidden(&input.target_artifact)
        || string_is_forbidden(&input.intended_value)
        || string_is_forbidden(&input.actor)
        || string_is_forbidden(&input.change_reason)
        || string_is_forbidden(&input.preview_reference)
        || string_is_forbidden(&input.recovery_reference)
    {
        return Err(M5ConfigWriteResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = write_intent_object_is_complete(
        input.preview_class,
        &input.target_scope,
        &input.target_artifact,
        &input.intended_value,
        &input.actor,
        &input.change_reason,
        &input.preview_reference,
        &input.recovery_reference,
    );
    let scope_ok = write_intent_lands_in_chosen_scope(
        input.preview_class,
        input.scope_ownership_preserved,
        input.is_high_risk_write,
        input.evidence_materialized,
    );
    let evidence_unmaterialized = input.is_high_risk_write && !input.evidence_materialized;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5SettingWriteIntentEntryDegradeReason::WriteIntentTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SettingWriteIntentEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.preview_class.is_classified() {
        Some(M5SettingWriteIntentEntryDegradeReason::PreviewClassUnclassified)
    } else if !input.bound_to_registry {
        Some(M5SettingWriteIntentEntryDegradeReason::WriteIntentNotBoundToRegistry)
    } else if !object_complete {
        Some(M5SettingWriteIntentEntryDegradeReason::WriteIntentObjectIncomplete)
    } else if !scope_ok {
        Some(
            M5SettingWriteIntentEntryDegradeReason::WriteIntentRewritesScopeOrHidesRecoveryEvidence,
        )
    } else if !all_forms {
        Some(M5SettingWriteIntentEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if evidence_unmaterialized {
        Some(
            M5SettingWriteIntentEntryDegradeReason::RecoveryEvidenceNotMaterializedForHighRiskWrite,
        )
    } else if !input.proof_fresh {
        Some(M5SettingWriteIntentEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ConfigWriteNextAction::ExpandWriteIntentMeaning,
    };

    Ok(M5ResolvedSettingWriteIntentEntry {
        entry_id: input.entry_id,
        write_target_id: input.write_target_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: input
            .semantic_role
            .must_preserve_evidence_and_disclose_cause_before_applying(),
        preview_class: input.preview_class.as_str().to_owned(),
        preview_class_is_classified: input.preview_class.is_classified(),
        canonical_class_mode: input.preview_class.canonical_class_mode().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        target_scope: input.target_scope,
        target_artifact: input.target_artifact,
        intended_value: input.intended_value,
        actor: input.actor,
        change_reason: input.change_reason,
        preview_reference: input.preview_reference,
        recovery_reference: input.recovery_reference,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        write_intent_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        scope_ownership_preserved: input.scope_ownership_preserved,
        is_high_risk_write: input.is_high_risk_write,
        evidence_materialized: input.evidence_materialized,
        degrade_reason,
        next_action,
        write_intent_lands_across_routes: degrade_reason.is_none(),
    })
}

/// Resolves a policy / constraint entry so its resolution stays safe: the entry names its canonical token,
/// semantic role, and lock class, covers all three resolution forms, provides the complete lock-source /
/// allowed-override-classes / expiry-review / validation-status / review-state / docs-pointer /
/// last-review-revision policy-constraint object, and degrades honestly when the record would mask a locked
/// value without disclosing its lock source or deny a write without disclosing the fallback guidance.
pub fn resolve_policy_constraint_entry(
    input: M5PolicyConstraintEntryResolutionInput,
) -> Result<M5ResolvedPolicyConstraintEntry, M5ConfigWriteResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ConfigWriteResolutionError::EmptyPolicyConstraintEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.constraint_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.lock_source)
        || string_is_forbidden(&input.allowed_override_classes)
        || string_is_forbidden(&input.expiry_review)
        || string_is_forbidden(&input.validation_status)
        || string_is_forbidden(&input.review_state)
        || string_is_forbidden(&input.docs_pointer)
        || string_is_forbidden(&input.last_review_revision)
    {
        return Err(M5ConfigWriteResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_honest = policy_constraint_stays_honest(
        input.lock_class,
        input.constraint_is_truthful,
        input.keeps_lock_source_visible,
        input.lock_present,
        input.lock_source_disclosed,
        input.denial_present,
        input.fallback_guidance_disclosed,
    );
    let provides_record = input.lock_class.is_classified()
        && !input.lock_source.trim().is_empty()
        && !input.allowed_override_classes.trim().is_empty()
        && !input.expiry_review.trim().is_empty()
        && !input.validation_status.trim().is_empty()
        && !input.review_state.trim().is_empty()
        && !input.docs_pointer.trim().is_empty()
        && !input.last_review_revision.trim().is_empty()
        && record_stays_honest;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5PolicyConstraintEntryDegradeReason::ConstraintTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5PolicyConstraintEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.lock_class.is_classified() {
        Some(M5PolicyConstraintEntryDegradeReason::LockClassUnclassified)
    } else if !provides_record {
        Some(M5PolicyConstraintEntryDegradeReason::PolicyConstraintMasksLockSourceOrHidesFallback)
    } else if !all_forms {
        Some(M5PolicyConstraintEntryDegradeReason::ConstraintFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5PolicyConstraintEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ConfigWriteNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedPolicyConstraintEntry {
        entry_id: input.entry_id,
        constraint_ref: input.constraint_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: input
            .semantic_role
            .must_preserve_evidence_and_disclose_cause_before_applying(),
        lock_class: input.lock_class.as_str().to_owned(),
        lock_class_is_classified: input.lock_class.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        lock_source: input.lock_source,
        allowed_override_classes: input.allowed_override_classes,
        expiry_review: input.expiry_review,
        validation_status: input.validation_status,
        review_state: input.review_state,
        docs_pointer: input.docs_pointer,
        last_review_revision: input.last_review_revision,
        keeps_lock_source_visible: input.keeps_lock_source_visible,
        constraint_is_truthful: input.constraint_is_truthful,
        lock_present: input.lock_present,
        lock_source_disclosed: input.lock_source_disclosed,
        denial_present: input.denial_present,
        fallback_guidance_disclosed: input.fallback_guidance_disclosed,
        policy_constraint_stays_honest: record_stays_honest,
        provides_complete_policy_constraint: provides_record,
        degrade_reason,
        next_action,
        constraint_safe_on_every_route: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved write-intent and policy-constraint entries it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingWriteIntentPolicyConstraintRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SettingWriteIntentPolicyConstraintRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5SettingsGovernanceQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Configuration contexts this row keeps the same truth across.
    pub deployment_lines: Vec<M5SettingsGovernanceDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5SettingsGovernanceRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5SettingsGovernanceAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5ConfigWriteAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ConfigWriteExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    /// Resolved write-intent-registry examples.
    pub write_intent_entries: Vec<M5ResolvedSettingWriteIntentEntry>,
    /// Resolved policy-constraint examples.
    pub policy_constraint_entries: Vec<M5ResolvedPolicyConstraintEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the write-intent domain and the policy /
    /// constraint landed schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never rewrites a scoped write into a broader scope. MUST be `false`.
    pub rewrites_a_scoped_write_into_a_broader_scope: bool,
    /// Hard invariant: this row never lands a write in an unintended artifact or scope. MUST be `false`.
    pub lands_a_write_in_an_unintended_artifact_or_scope: bool,
    /// Hard invariant: this row never applies a high-risk write without preview / checkpoint / rollback
    /// evidence. MUST be `false`.
    pub applies_a_high_risk_write_without_preview_checkpoint_or_rollback: bool,
    /// Hard invariant: this row never hides a lock or policy-disable cause behind generic unavailable copy. MUST
    /// be `false`.
    pub hides_a_lock_or_policy_disable_cause_behind_generic_unavailable_copy: bool,
}

impl M5SettingWriteIntentPolicyConstraintRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ConfigWriteAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ConfigWriteAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ConfigWriteExportField> =
            self.export_fields.iter().copied().collect();
        M5ConfigWriteExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.rewrites_a_scoped_write_into_a_broader_scope
            && !self.lands_a_write_in_an_unintended_artifact_or_scope
            && !self.applies_a_high_risk_write_without_preview_checkpoint_or_rollback
            && !self.hides_a_lock_or_policy_disable_cause_behind_generic_unavailable_copy
    }

    /// True when a clean write-intent entry preserves registry-bound truth: it traces to the registry, keeps a
    /// classified preview class, publishes a complete write-intent object, preserves the chosen scope / artifact,
    /// covers all three resolution forms, and materializes recovery evidence for a high-risk write.
    fn write_intent_is_honest(ex: &M5ResolvedSettingWriteIntentEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.preview_class_is_classified
                && ex.write_intent_object_complete
                && ex.scope_ownership_preserved
                && ex.covers_all_resolution_forms
                && (!ex.is_high_risk_write || ex.evidence_materialized))
    }

    /// True when a clean policy-constraint entry preserves a safe record: it keeps a classified lock class,
    /// provides the complete policy-constraint object, stays honest, and covers all three resolution forms.
    fn constraint_is_honest(ex: &M5ResolvedPolicyConstraintEntry) -> bool {
        !ex.is_clean()
            || (ex.lock_class_is_classified
                && ex.provides_complete_policy_constraint
                && ex.policy_constraint_stays_honest
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.write_intent_entries
            .iter()
            .all(Self::write_intent_is_honest)
            && self
                .policy_constraint_entries
                .iter()
                .all(Self::constraint_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingWriteIntentPolicyConstraintRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Write-intent preview-class tokens (minted by this lane).
    pub write_intent_preview_classes: Vec<String>,
    /// Policy lock-class tokens (minted by this lane).
    pub policy_lock_classes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Write-intent-entry degrade-reason tokens.
    pub write_intent_degrade_reasons: Vec<String>,
    /// Policy-constraint-entry degrade-reason tokens.
    pub policy_constraint_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SettingWriteIntentPolicyConstraintRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5SettingsGovernanceRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5ConfigWriteResolutionForm::ALL, |v| v.as_str()),
            write_intent_preview_classes: tokens(&M5WriteIntentPreviewClass::ALL, |v| v.as_str()),
            policy_lock_classes: tokens(&M5PolicyLockClass::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ConfigWriteSurfaceContext::ALL, |v| v.as_str()),
            write_intent_degrade_reasons: tokens(
                &M5SettingWriteIntentEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            policy_constraint_degrade_reasons: tokens(
                &M5PolicyConstraintEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5ConfigWriteAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ConfigWriteNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ConfigWriteExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5SettingsGovernanceConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5SettingWriteIntentPolicyConstraintRegistriesGovernanceReview {
    /// The write-intent registry names a canonical token, semantic role, and preview class for every entry.
    pub write_intent_registry_names_token_role_and_class: bool,
    /// Every claimed mutation resolves to one write-intent object from the shared registry, not per-entry
    /// reconstruction.
    pub write_lands_to_one_intent_object_from_shared_registry: bool,
    /// The target scope, target artifact, intended value, actor, change reason, preview reference, and recovery
    /// reference are published for every resolved write intent.
    pub target_scope_artifact_value_actor_reason_and_evidence_published: bool,
    /// Writes land only in the chosen scope and artifact; a scoped write is never rewritten into a broader
    /// scope.
    pub writes_land_only_in_chosen_scope_and_artifact: bool,
    /// The policy / constraint record keeps the lock source visible and discloses the fallback guidance.
    pub policy_constraint_keeps_lock_source_visible_and_discloses_fallback: bool,
    /// The preview / checkpoint / rollback recovery evidence is materialized before any high-risk write applies.
    pub recovery_evidence_materialized_for_high_risk_writes: bool,
    /// Every write-intent and policy-constraint entry covers the canonical / accessible / audit resolution
    /// forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Write-intent and policy-constraint behavior stay bound to the shared registries rather than hand-copied
    /// per write.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Settings, shell, diagnostics, and admin read a single configuration source.
    pub settings_shell_diagnostics_admin_read_single_source: bool,
    /// A rewritten scope, an incomplete object, or a masked lock is caught by fixtures before release evidence
    /// turns green.
    pub write_or_constraint_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingWriteIntentPolicyConstraintRegistriesConsumerProjection {
    /// Settings and shell consume the shared write-intent registry.
    pub settings_and_shell_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared policy-constraint registry.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// Sync and policy services consume the shared registries.
    pub sync_and_policy_services_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical write-intent and policy-constraint domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical write-intent / policy-constraint registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingWriteIntentPolicyConstraintRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingWriteIntentPolicyConstraintRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting settings-governance audit for the lane.
    pub settings_governance_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SettingWriteIntentPolicyConstraintRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SettingWriteIntentPolicyConstraintRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SettingWriteIntentPolicyConstraintRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingWriteIntentPolicyConstraintRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingWriteIntentPolicyConstraintRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingWriteIntentPolicyConstraintRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingWriteIntentPolicyConstraintRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingWriteIntentPolicyConstraintRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 setting-write-intent and policy-constraint registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingWriteIntentPolicyConstraintRegistriesPacket {
    /// Record kind; must equal [`M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SettingWriteIntentPolicyConstraintRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingWriteIntentPolicyConstraintRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingWriteIntentPolicyConstraintRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingWriteIntentPolicyConstraintRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingWriteIntentPolicyConstraintRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingWriteIntentPolicyConstraintRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SettingWriteIntentPolicyConstraintRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SettingWriteIntentPolicyConstraintRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version: M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
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

    /// Validates the registries-packet invariants.
    pub fn validate(&self) -> Vec<M5SettingWriteIntentPolicyConstraintRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_RECORD_KIND {
            violations
                .push(M5SettingWriteIntentPolicyConstraintRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version
            != M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_SCHEMA_VERSION
        {
            violations
                .push(M5SettingWriteIntentPolicyConstraintRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations
                .push(M5SettingWriteIntentPolicyConstraintRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations
                .push(M5SettingWriteIntentPolicyConstraintRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 setting-write-intent / policy-constraint registries packet serializes"),
        ) {
            violations
                .push(M5SettingWriteIntentPolicyConstraintRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 setting-write-intent / policy-constraint registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,write_intent_entries,policy_constraint_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .write_intent_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.policy_constraint_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.write_intent_entries.len(),
                row.policy_constraint_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Setting-Write-Intent and Policy-Constraint Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Write-intent preview classes: {}\n",
            self.vocabulary_set.write_intent_preview_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Resolution forms: {}\n",
            self.vocabulary_set.resolution_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Write-intent entries: {} / policy-constraint entries: {}\n",
                row.write_intent_entries.len(),
                row.policy_constraint_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry write-intent reference table generated from the registry, so docs and admin
    /// runbooks render the same class-mode / target-scope / target-artifact / intended-value / change-reason /
    /// recovery-reference truth the resolvers produced rather than a hand-copied write table. Only clean,
    /// registry-bound write-intent entries are listed.
    pub fn render_write_intent_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| write_target_id | class_mode | target_scope | target_artifact | intended_value | change_reason | recovery_reference |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.write_intent_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.write_target_id,
                    ex.canonical_class_mode,
                    ex.target_scope,
                    ex.target_artifact,
                    ex.intended_value,
                    ex.change_reason,
                    ex.recovery_reference
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SettingWriteIntentPolicyConstraintRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SettingWriteIntentPolicyConstraintRegistriesViolation>),
}

impl fmt::Display for M5SettingWriteIntentPolicyConstraintRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 setting-write-intent / policy-constraint registries export parse failed: {error}"
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
                    "m5 setting-write-intent / policy-constraint registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SettingWriteIntentPolicyConstraintRegistriesArtifactError {}

/// Validation failures emitted by [`M5SettingWriteIntentPolicyConstraintRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SettingWriteIntentPolicyConstraintRegistriesViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at both the write-intent domain and the policy / constraint landed schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, scope-rewriting, field-incomplete,
    /// form-incomplete, or a policy-constraint entry missing the complete record object).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Write-intent-resolution is not proven: clean write-intent entries do not cover the canonical preview
    /// classes or the first settings / shell / diagnostics / admin / support surfaces, no object-incomplete
    /// example degrades, or a clean write-intent entry published an incomplete object.
    WriteIntentResolutionNotProven,
    /// Write-scope-ownership-preservation is not proven: no scope-rewrite example and no unbound example degrade,
    /// no clean scope-preserving write-intent entry is present, or a clean write-intent entry rewrote the scope
    /// or is unbound.
    WriteScopeOwnershipPreservationNotProven,
    /// Policy-constraint-integrity is not proven: clean policy-constraint entries do not cover the canonical
    /// policy-locked / override-allowed / advisory lock classes with full resolution-form coverage while
    /// providing the complete record object, no masked-lock or form-incomplete example degrades, or a clean
    /// policy-constraint entry is missing the complete record object.
    PolicyConstraintIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SettingWriteIntentPolicyConstraintRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::WriteIntentResolutionNotProven => "write_intent_resolution_not_proven",
            Self::WriteScopeOwnershipPreservationNotProven => {
                "write_scope_ownership_preservation_not_proven"
            }
            Self::PolicyConstraintIntegrityNotProven => "policy_constraint_integrity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_setting_write_intent_and_policy_constraint_registries_export() -> Result<
    M5SettingWriteIntentPolicyConstraintRegistriesPacket,
    M5SettingWriteIntentPolicyConstraintRegistriesArtifactError,
> {
    let packet: M5SettingWriteIntentPolicyConstraintRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-setting-write-intent-and-policy-constraint-registries-proof/support_export.json"
        )
    ))
    .map_err(M5SettingWriteIntentPolicyConstraintRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SettingWriteIntentPolicyConstraintRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SettingWriteIntentPolicyConstraintRegistriesPacket,
    violations: &mut Vec<M5SettingWriteIntentPolicyConstraintRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_SCHEMA_REF,
        M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF,
        M5_POLICY_CONSTRAINT_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5SettingWriteIntentPolicyConstraintRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SettingWriteIntentPolicyConstraintRegistriesPacket,
    violations: &mut Vec<M5SettingWriteIntentPolicyConstraintRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5SettingWriteIntentPolicyConstraintRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(
                M5SettingWriteIntentPolicyConstraintRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5SettingWriteIntentPolicyConstraintRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5SettingWriteIntentPolicyConstraintRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_POLICY_CONSTRAINT_LANDED_SCHEMA_REF)
        {
            violations.push(
                M5SettingWriteIntentPolicyConstraintRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.write_intent_entries.is_empty() || row.policy_constraint_entries.is_empty() {
            violations
                .push(M5SettingWriteIntentPolicyConstraintRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations
                .push(M5SettingWriteIntentPolicyConstraintRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(
                M5SettingWriteIntentPolicyConstraintRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5SettingWriteIntentPolicyConstraintRegistriesPacket,
    violations: &mut Vec<M5SettingWriteIntentPolicyConstraintRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.write_intent_registry_names_token_role_and_class,
        review.write_lands_to_one_intent_object_from_shared_registry,
        review.target_scope_artifact_value_actor_reason_and_evidence_published,
        review.writes_land_only_in_chosen_scope_and_artifact,
        review.policy_constraint_keeps_lock_source_visible_and_discloses_fallback,
        review.recovery_evidence_materialized_for_high_risk_writes,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.settings_shell_diagnostics_admin_read_single_source,
        review.write_or_constraint_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5SettingWriteIntentPolicyConstraintRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SettingWriteIntentPolicyConstraintRegistriesPacket,
    violations: &mut Vec<M5SettingWriteIntentPolicyConstraintRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.settings_and_shell_consume_shared_registries,
        projection.diagnostics_and_admin_consume_shared_registries,
        projection.sync_and_policy_services_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5SettingWriteIntentPolicyConstraintRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SettingWriteIntentPolicyConstraintRegistriesPacket,
    violations: &mut Vec<M5SettingWriteIntentPolicyConstraintRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5SettingWriteIntentPolicyConstraintRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5SettingWriteIntentPolicyConstraintRegistriesPacket,
    violations: &mut Vec<M5SettingWriteIntentPolicyConstraintRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.settings_governance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5SettingWriteIntentPolicyConstraintRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5SettingWriteIntentPolicyConstraintRegistriesPacket,
    violations: &mut Vec<M5SettingWriteIntentPolicyConstraintRegistriesViolation>,
) {
    let write_intents = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.write_intent_entries.iter())
    };
    let constraints = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.policy_constraint_entries.iter())
    };

    // AC1: high-risk settings changes produce preview / checkpoint / rollback evidence before apply. Clean
    // write-intent entries cover the canonical preview classes and the first settings / shell / diagnostics /
    // admin / support surfaces, an object-incomplete example degrades, and no clean write-intent entry published
    // an incomplete object (an incomplete object is a missing preview / recovery reference).
    let clean_classes: BTreeSet<String> = write_intents()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.preview_class.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = write_intents()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let classes_covered = M5WriteIntentPreviewClass::CANONICAL_CLASSES
        .iter()
        .all(|k| clean_classes.contains(k.as_str()));
    let first_surfaces_covered = M5ConfigWriteSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = write_intents().any(|ex| {
        ex.degrade_reason
            == Some(M5SettingWriteIntentEntryDegradeReason::WriteIntentObjectIncomplete)
    });
    let no_clean_incomplete =
        !write_intents().any(|ex| ex.is_clean() && !ex.write_intent_object_complete);
    if !(classes_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5SettingWriteIntentPolicyConstraintRegistriesViolation::WriteIntentResolutionNotProven,
        );
    }

    // AC3: writes land only in the chosen artifact and scope. A scope-rewrite example degrades, an unbound
    // example degrades, at least one clean scope-preserving write-intent entry is present, and no clean
    // write-intent entry rewrote the scope or is unbound.
    let rewrite_degrades = write_intents().any(|ex| {
        ex.degrade_reason
            == Some(
                M5SettingWriteIntentEntryDegradeReason::WriteIntentRewritesScopeOrHidesRecoveryEvidence,
            )
    });
    let unbound_degrades = write_intents().any(|ex| {
        ex.degrade_reason
            == Some(M5SettingWriteIntentEntryDegradeReason::WriteIntentNotBoundToRegistry)
    });
    let preserving_clean_write_intent =
        write_intents().any(|ex| ex.is_clean() && ex.scope_ownership_preserved);
    let no_clean_unbound = !write_intents().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_rewritten =
        !write_intents().any(|ex| ex.is_clean() && !ex.scope_ownership_preserved);
    if !(rewrite_degrades
        && unbound_degrades
        && preserving_clean_write_intent
        && no_clean_unbound
        && no_clean_rewritten)
    {
        violations.push(
            M5SettingWriteIntentPolicyConstraintRegistriesViolation::WriteScopeOwnershipPreservationNotProven,
        );
    }

    // AC2: locked or denied writes return structured reasons and fallback guidance. Clean policy-constraint
    // entries cover every canonical policy-locked / override-allowed / advisory lock class with full
    // resolution-form coverage while providing the complete record object, a masked-lock example degrades, a
    // form-incomplete example degrades, and no clean policy-constraint entry is missing the complete record
    // object.
    let clean_record_classes: BTreeSet<String> = constraints()
        .filter(|ex| {
            ex.is_clean()
                && ex.lock_class_is_classified
                && ex.provides_complete_policy_constraint
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.lock_class.clone())
        .collect();
    let record_classes_covered = M5PolicyLockClass::CANONICAL_CLASSES
        .iter()
        .all(|m| clean_record_classes.contains(m.as_str()));
    let masked_lock_degrades = constraints().any(|ex| {
        ex.degrade_reason
            == Some(
                M5PolicyConstraintEntryDegradeReason::PolicyConstraintMasksLockSourceOrHidesFallback,
            )
    });
    let form_incomplete_degrades = constraints().any(|ex| {
        ex.degrade_reason
            == Some(M5PolicyConstraintEntryDegradeReason::ConstraintFormCoverageIncomplete)
    });
    let no_clean_missing_record =
        !constraints().any(|ex| ex.is_clean() && !ex.provides_complete_policy_constraint);
    if !(record_classes_covered
        && masked_lock_degrades
        && form_incomplete_degrades
        && no_clean_missing_record)
    {
        violations.push(
            M5SettingWriteIntentPolicyConstraintRegistriesViolation::PolicyConstraintIntegrityNotProven,
        );
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The settings-governance families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5SettingsGovernanceFamily; 1] =
    [M5SettingsGovernanceFamily::WriteSetting];
