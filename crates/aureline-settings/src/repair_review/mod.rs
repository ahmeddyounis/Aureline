//! Beta-grade settings repair, reset, import, and migration review.
//!
//! The repair-review module is the page-level surface that finishes
//! the settings-safety story by routing every "reset", "repair",
//! "reapply imported fragment", or "revert migration step" action
//! through the same write-intent pipeline as ordinary UI/CLI/sync
//! writes. Surfaces never invent a private "broad reset" or
//! convenience-write path that bypasses the resolver, the policy
//! ceiling, or the checkpoint rules.
//!
//! Each repair flow produces a [`SettingsRepairPlan`] that names:
//!
//! - the typed [`RepairActionClass`] (reset current value, reset
//!   section, repair drift, reapply imported profile fragment,
//!   revert migration step);
//! - the exact [`RepairTargetScopeClass`] the plan would touch
//!   (`user`, `profile`, `workspace`, `policy_owned`, or
//!   `machine_local`), and the underlying [`SettingScope`] token;
//! - the intended artifact ref (settings file, profile fragment, or
//!   migration step) the write would land on;
//! - the explicit list of `setting_id`s the user selected, so
//!   convenience resets cannot quietly widen to adjacent rows;
//! - the [`HiddenResetGuard`] verdict that proves the plan refuses
//!   to broaden scope or touch unselected rows;
//! - the per-setting [`SettingWritePreviewRecord`] produced by the
//!   inspector's `preview_write` flow, so the resolver, the policy
//!   ceiling, capability dependencies, and the redaction posture
//!   stay enforced;
//! - the checkpoint ref the plan preserves before any broad rewrite,
//!   plus the rollback action ref the user can route to after apply.
//!
//! The plan, the per-action review sheets, and the support-export
//! wrapper all read from the same canonical projection so the UI,
//! the headless CLI, support tooling, and the mutation journal
//! describe the same repair attempt — including whether the user
//! accepted or declined the plan.

use serde::{Deserialize, Serialize};

use crate::inspector::{
    preview_write, EffectiveSettingInspectionRecord, SettingWritePreviewRecord,
    SettingWritePreviewRequest, SettingsInspectError, SettingsInspectionContext, WriteActorClass,
    WriteReasonClass,
};
use crate::resolver::EffectiveSettingsResolver;
use crate::schema::{LifecycleLabel, PreviewClass, SettingDefinition, SettingScope, SettingValue};

/// Schema version for the settings repair plan envelope.
pub const SETTINGS_REPAIR_PLAN_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by settings UI, CLI/headless, sync,
/// policy, support/export, and docs/help surfaces.
pub const SETTINGS_REPAIR_PLAN_SHARED_CONTRACT_REF: &str = "settings:repair_plan_beta:v1";

/// Frozen repair-action vocabulary. Surfaces consume these tokens
/// verbatim; never invent a parallel "fix" or "auto-repair" class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairActionClass {
    /// Reset one setting at one scope back to the next-lower scope's
    /// value, or to the built-in default when no lower overlay exists.
    ResetCurrentValue,
    /// Reset every row inside one section prefix at one scope.
    ResetSection,
    /// Repair a drifted value — restore the last-known intended value
    /// for a setting whose written value disagrees with the resolved
    /// intent (commonly seen after a partial import or a hand-edit).
    RepairDrift,
    /// Re-apply a named fragment from a previously imported profile
    /// artifact. Only the rows the user re-selects are touched.
    ReapplyImportedProfileFragment,
    /// Revert a specific migration step on one setting. Rolls back
    /// the value-shape transform recorded by the migration row, using
    /// the checkpoint captured before the migration applied.
    RevertMigrationStep,
}

impl RepairActionClass {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResetCurrentValue => "reset_current_value",
            Self::ResetSection => "reset_section",
            Self::RepairDrift => "repair_drift",
            Self::ReapplyImportedProfileFragment => "reapply_imported_profile_fragment",
            Self::RevertMigrationStep => "revert_migration_step",
        }
    }

    /// Returns true when the action class can rewrite more than one
    /// setting in a single plan. These plans always preserve a
    /// rollback checkpoint before applying.
    pub const fn is_multi_row(self) -> bool {
        matches!(
            self,
            Self::ResetSection | Self::ReapplyImportedProfileFragment
        )
    }
}

/// Frozen target-scope class. The class names which artifact owner
/// the plan would touch; the resolver scope is recorded separately so
/// support surfaces can quote both without re-deriving the mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairTargetScopeClass {
    /// User-global artifact (e.g. user `settings.jsonc`).
    User,
    /// Imported profile artifact (a profile fragment under the
    /// user's portable profile).
    Profile,
    /// Workspace-scoped artifact (`.aureline/settings.jsonc` in the
    /// workspace).
    Workspace,
    /// Policy-owned artifact (admin policy bundle). Repair flows
    /// never write here; settings under this class stay locked and
    /// surface through `locked_classes`.
    PolicyOwned,
    /// Machine-local artifact (per-host topology / GPU / process
    /// tuning). Never carried across machines via optional sync.
    MachineLocal,
}

impl RepairTargetScopeClass {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Profile => "profile",
            Self::Workspace => "workspace",
            Self::PolicyOwned => "policy_owned",
            Self::MachineLocal => "machine_local",
        }
    }

    /// Returns true when the class is allowed to receive a write from
    /// a repair plan. Policy-owned artifacts always return false.
    pub const fn is_writable(self) -> bool {
        !matches!(self, Self::PolicyOwned)
    }

    /// Maps a setting scope to its repair target-scope class.
    pub const fn from_scope(scope: SettingScope) -> Self {
        match scope {
            SettingScope::UserGlobal => Self::User,
            SettingScope::ImportedProfileDefault => Self::Profile,
            SettingScope::Workspace
            | SettingScope::FolderOrModuleOverride
            | SettingScope::LanguageOverride
            | SettingScope::SessionOverride => Self::Workspace,
            SettingScope::MachineSpecific => Self::MachineLocal,
            SettingScope::AdminPolicyNarrowing => Self::PolicyOwned,
            SettingScope::BuiltInDefault | SettingScope::ChannelOrExperimentDefault => Self::User,
        }
    }
}

/// Verdict produced by the hidden-reset guard. The guard refuses any
/// plan that would silently broaden scope (e.g. reach into a higher
/// authority artifact) or touch settings the user did not explicitly
/// select.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenResetGuard {
    /// True when the plan would have touched a broader scope than
    /// the user selected. Any true verdict pairs with at least one
    /// blocked-write reason and refuses apply.
    pub would_broaden_scope: bool,
    /// True when the plan would have touched a setting the user did
    /// not explicitly select.
    pub would_touch_adjacent_settings: bool,
    /// Explicit list of `setting_id`s the user selected. Frozen for
    /// the lifetime of the plan; the projection does not extend it.
    pub selected_setting_ids: Vec<String>,
    /// Settings that fell outside `selected_setting_ids` and were
    /// refused. Empty when no adjacent rows were proposed.
    pub refused_setting_ids: Vec<String>,
    /// Short human-readable summary suitable for support exports.
    pub summary: String,
}

/// Typed blocked-write reason attached to a plan. Surfaces MUST quote
/// the typed token rather than render free-form prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum RepairBlockedWriteReason {
    /// The setting is owned by admin policy and cannot be repaired
    /// from a user-initiated plan.
    PolicyOwnedClass { setting_id: String },
    /// The target scope is not in the setting's `allowed_scopes`.
    NonWritableScope {
        setting_id: String,
        target_scope: String,
    },
    /// The setting is retired and refuses writes.
    RetiredSetting { setting_id: String },
    /// A capability dependency is not satisfied; the resolver locks
    /// the row and refuses the repair until the capability returns.
    CapabilityDependencyUnmet { setting_id: String },
    /// A rollback checkpoint must be created before apply.
    CheckpointMissing { setting_id: String },
    /// An approval ticket must be granted before apply.
    ApprovalMissing { setting_id: String },
    /// The setting can only be written by a managed authority.
    ManagedModeOnly { setting_id: String },
    /// The plan would have broadened the user-selected scope.
    ScopeBroadeningRefused {
        proposed_scope: String,
        selected_scope: String,
    },
    /// The plan would have touched a setting outside the user
    /// selection.
    AdjacentSettingRefused { setting_id: String },
    /// The setting is not registered.
    UnknownSetting { setting_id: String },
}

impl RepairBlockedWriteReason {
    /// Returns the stable code token.
    pub const fn code_token(&self) -> &'static str {
        match self {
            Self::PolicyOwnedClass { .. } => "policy_owned_class",
            Self::NonWritableScope { .. } => "non_writable_scope",
            Self::RetiredSetting { .. } => "retired_setting",
            Self::CapabilityDependencyUnmet { .. } => "capability_dependency_unmet",
            Self::CheckpointMissing { .. } => "checkpoint_missing",
            Self::ApprovalMissing { .. } => "approval_missing",
            Self::ManagedModeOnly { .. } => "managed_mode_only",
            Self::ScopeBroadeningRefused { .. } => "scope_broadening_refused",
            Self::AdjacentSettingRefused { .. } => "adjacent_setting_refused",
            Self::UnknownSetting { .. } => "unknown_setting",
        }
    }
}

/// Overall verdict for a repair plan. Mirrors the inspector
/// write-intent vocabulary so support/export consumers do not have
/// to learn a parallel apply-state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairPlanVerdict {
    /// Plan is safe to apply immediately.
    ReadyToApply,
    /// Plan is allowed but requires the user to acknowledge a preview.
    AwaitingPreview,
    /// Plan is allowed once a rollback checkpoint has been recorded.
    AwaitingCheckpoint,
    /// Plan is allowed once an approval ticket has been granted.
    AwaitingApproval,
    /// Plan is refused. At least one blocked-write reason is present.
    Denied,
}

impl RepairPlanVerdict {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToApply => "ready_to_apply",
            Self::AwaitingPreview => "awaiting_preview",
            Self::AwaitingCheckpoint => "awaiting_checkpoint",
            Self::AwaitingApproval => "awaiting_approval",
            Self::Denied => "denied",
        }
    }
}

/// Lifecycle state of a repair plan attempt. Recorded so the support
/// export can explain whether the user accepted or declined the plan
/// without the surface having to reconstruct the dialog state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairUserDecision {
    /// User has not yet decided.
    Pending,
    /// User accepted the plan; apply proceeded.
    Accepted,
    /// User declined the plan before apply.
    Declined,
    /// User withdrew the plan after preview without applying.
    Withdrawn,
}

impl RepairUserDecision {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Accepted => "accepted",
            Self::Declined => "declined",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// One per-setting write the plan would issue. Mirrors the
/// inspector's [`SettingWritePreviewRecord`] but adds the
/// repair-plan source ref so the projection can be replayed without
/// stitching the records back together by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairWriteIntentRow {
    /// Canonical setting id.
    pub setting_id: String,
    /// True when this row is the selected target of the repair plan
    /// (any other row would be refused by the hidden-reset guard).
    pub selected_by_user: bool,
    /// True when the row is currently blocked by lock state or
    /// adjacent-row guard.
    pub blocked: bool,
    /// Lock state token copied from the underlying inspector record.
    pub lock_state: String,
    /// Lifecycle label token copied from the definition.
    pub lifecycle_label: String,
    /// Write preview produced by the inspector's `preview_write`
    /// flow. Always present, even when the plan is denied — the
    /// preview tells the user *what* would have happened.
    pub write_preview: SettingWritePreviewRecord,
}

/// Reference to an imported profile fragment the plan re-applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedProfileFragmentRef {
    /// Stable profile artifact id.
    pub profile_id: String,
    /// Stable fragment id inside the artifact.
    pub fragment_id: String,
    /// Human-readable label rendered above the diff.
    pub fragment_label: String,
    /// Source label for the profile artifact.
    pub source_label: String,
}

/// Reference to a migration step the plan reverts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationStepRef {
    /// Stable migration step id.
    pub migration_id: String,
    /// Source version the step migrated from.
    pub from_version: String,
    /// Target version the step migrated to.
    pub to_version: String,
    /// Transform class token copied from the migration row.
    pub transform_class: String,
    /// True when the underlying migration row marked itself lossy.
    pub is_lossy: bool,
    /// True when the migration row declared rollback support.
    pub rollback_supported: bool,
}

/// Canonical repair plan record exported to UI, CLI, sync, policy,
/// support, and docs/help consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsRepairPlan {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta repair-plan schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, support, and docs.
    pub shared_contract_ref: String,
    /// Opaque stable id for this plan.
    pub plan_id: String,
    /// Repair-action class token.
    pub action_class: String,
    /// Underlying setting scope the plan would touch.
    pub target_scope: String,
    /// Repair target-scope class token.
    pub target_scope_class: String,
    /// Stable artifact ref that would receive the write (settings
    /// file path, profile fragment ref, or migration step ref).
    pub target_artifact_ref: String,
    /// Optional section id for `reset_section` plans.
    pub section_id: Option<String>,
    /// Optional imported profile fragment ref.
    pub imported_profile_fragment: Option<ImportedProfileFragmentRef>,
    /// Optional migration step ref.
    pub migration_step: Option<MigrationStepRef>,
    /// Actor class token.
    pub actor_class: String,
    /// Reason class token.
    pub reason_class: String,
    /// Free-form short reason quoted on support exports.
    pub reason_note: Option<String>,
    /// Preview class token copied from the most-restrictive affected
    /// setting; the plan inherits the strictest posture in the set.
    pub preview_class: String,
    /// True when a rollback checkpoint must be recorded before apply.
    pub checkpoint_required: bool,
    /// Opaque ref for the rollback checkpoint preserved before apply.
    pub checkpoint_ref: Option<String>,
    /// Opaque ref the user can route to after apply to roll back.
    pub rollback_action_ref: Option<String>,
    /// Opaque ref for an approval ticket the plan requires.
    pub approval_ticket_ref: Option<String>,
    /// True when an approval ticket is required.
    pub approval_required: bool,
    /// Per-row write intents produced by the inspector pipeline.
    pub write_intents: Vec<RepairWriteIntentRow>,
    /// Per-row blocked-write reasons. Empty when the plan is fully
    /// ready to apply.
    pub blocked_write_reasons: Vec<RepairBlockedWriteReason>,
    /// Locked classes (e.g. `policy_owned`, `capability_locked`,
    /// `retired_setting`) that fell outside the writable surface.
    pub locked_classes: Vec<String>,
    /// Hidden-reset guard verdict.
    pub hidden_reset_guard: HiddenResetGuard,
    /// Overall verdict token.
    pub verdict: String,
    /// User-decision state token.
    pub user_decision: String,
    /// Effective records for every affected setting *before* the
    /// proposed apply. Lets support exports compare with the
    /// `effective_after` snapshot on each write preview.
    pub effective_before: Vec<EffectiveSettingInspectionRecord>,
}

/// Request describing the inputs to one repair-plan build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsRepairPlanRequest {
    /// Opaque stable id for this plan.
    pub plan_id: String,
    /// Repair-action class.
    pub action_class: RepairActionClass,
    /// Underlying setting scope to write at.
    pub target_scope: SettingScope,
    /// Optional section id for `reset_section`.
    pub section_id: Option<String>,
    /// Optional imported profile fragment ref.
    pub imported_profile_fragment: Option<ImportedProfileFragmentRef>,
    /// Optional migration step ref.
    pub migration_step: Option<MigrationStepRef>,
    /// Actor class for the mutation attempt.
    pub actor_class: WriteActorClass,
    /// Reason class for the mutation attempt.
    pub reason_class: WriteReasonClass,
    /// Optional short reason note quoted on support exports.
    pub reason_note: Option<String>,
    /// Explicit list of setting ids the user selected. The hidden
    /// reset guard refuses any write that falls outside this set.
    pub selected_setting_ids: Vec<String>,
    /// Per-setting proposed value. Maps `setting_id` to the value
    /// the plan would write.
    pub proposed_values: Vec<(String, SettingValue)>,
    /// Pre-existing rollback checkpoint ref, if one was already
    /// captured by the surface.
    pub checkpoint_ref: Option<String>,
    /// Pre-existing approval ticket ref, if one was already granted.
    pub approval_ticket_ref: Option<String>,
    /// Current user-decision state (defaults to `Pending`).
    pub user_decision: RepairUserDecision,
}

impl SettingsRepairPlanRequest {
    /// Builds a request scaffold with the minimum required fields.
    pub fn new(
        plan_id: impl Into<String>,
        action_class: RepairActionClass,
        target_scope: SettingScope,
    ) -> Self {
        Self {
            plan_id: plan_id.into(),
            action_class,
            target_scope,
            section_id: None,
            imported_profile_fragment: None,
            migration_step: None,
            actor_class: WriteActorClass::UserCommand,
            reason_class: WriteReasonClass::UserEdit,
            reason_note: None,
            selected_setting_ids: Vec::new(),
            proposed_values: Vec::new(),
            checkpoint_ref: None,
            approval_ticket_ref: None,
            user_decision: RepairUserDecision::Pending,
        }
    }
}

/// Materialize a repair plan from the live resolver.
pub fn build_repair_plan(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    request: SettingsRepairPlanRequest,
) -> Result<SettingsRepairPlan, SettingsInspectError> {
    let registry = resolver.registry();
    let target_scope_class = RepairTargetScopeClass::from_scope(request.target_scope);
    let target_artifact_ref = target_artifact_ref_for(
        request.action_class,
        request.target_scope,
        request.section_id.as_deref(),
        request.imported_profile_fragment.as_ref(),
        request.migration_step.as_ref(),
    );

    let mut write_intents: Vec<RepairWriteIntentRow> = Vec::new();
    let mut blocked: Vec<RepairBlockedWriteReason> = Vec::new();
    let mut locked_classes: Vec<String> = Vec::new();
    let mut effective_before: Vec<EffectiveSettingInspectionRecord> = Vec::new();
    let mut refused_settings: Vec<String> = Vec::new();
    let mut would_broaden = !target_scope_class.is_writable();
    if !target_scope_class.is_writable() {
        blocked.push(RepairBlockedWriteReason::PolicyOwnedClass {
            setting_id: target_artifact_ref.clone(),
        });
        if !locked_classes
            .iter()
            .any(|class| class == "policy_owned_class")
        {
            locked_classes.push("policy_owned_class".to_owned());
        }
    }

    let mut strictest_preview = PreviewClass::SafeApply;
    let mut underlying_preview = PreviewClass::SafeApply;
    let mut checkpoint_required = false;
    let mut approval_required = false;

    let selected_set: std::collections::BTreeSet<String> = request
        .selected_setting_ids
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();

    for (setting_id, proposed_value) in &request.proposed_values {
        let selected = selected_set.contains(setting_id);
        let definition = registry.resolve_definition(setting_id);
        if !selected {
            // The plan must refuse adjacent rows the user did not
            // select. We do not run the write preview for them.
            refused_settings.push(setting_id.clone());
            blocked.push(RepairBlockedWriteReason::AdjacentSettingRefused {
                setting_id: setting_id.clone(),
            });
            continue;
        }

        let Some(def) = definition else {
            blocked.push(RepairBlockedWriteReason::UnknownSetting {
                setting_id: setting_id.clone(),
            });
            // No write preview when the setting is unknown.
            continue;
        };

        let preview = preview_write(
            resolver,
            SettingWritePreviewRequest {
                setting_id: def.setting_id.clone(),
                target_scope: request.target_scope,
                proposed_value: proposed_value.clone(),
                actor_class: request.actor_class,
                reason_class: request.reason_class,
                checkpoint_ref: request.checkpoint_ref.clone(),
                approval_ticket_ref: request.approval_ticket_ref.clone(),
            },
            context,
        );
        if let Some(before) = preview.effective_before.as_ref() {
            effective_before.push(before.clone());
        }

        // Promote the strictest preview class across all rows.
        strictest_preview = strictest_preview_class(strictest_preview, def.preview_class);
        underlying_preview = strictest_preview_class(underlying_preview, def.preview_class);
        checkpoint_required |= def.preview_class.requires_checkpoint();
        approval_required |= def.preview_class.requires_approval();

        let blocked_row =
            collect_blocked_reasons(def, &preview, target_scope_class, &mut would_broaden);
        for reason in &blocked_row {
            track_locked_class(&mut locked_classes, reason);
        }
        let blocked_row_present = !blocked_row.is_empty();
        blocked.extend(blocked_row);

        write_intents.push(RepairWriteIntentRow {
            setting_id: def.setting_id.clone(),
            selected_by_user: true,
            blocked: blocked_row_present,
            lock_state: preview
                .effective_before
                .as_ref()
                .map(|record| record.lock_state.clone())
                .unwrap_or_else(|| "unknown".to_owned()),
            lifecycle_label: lifecycle_token(def.lifecycle_label).to_owned(),
            write_preview: preview,
        });
    }

    // For multi-row actions, checkpoint creation is mandatory even
    // when no individual row requires it; broad rewrites always
    // preserve a rollback path.
    if request.action_class.is_multi_row() {
        checkpoint_required = true;
        if strictest_preview == PreviewClass::SafeApply {
            strictest_preview = PreviewClass::RollbackCheckpointRequired;
        }
    }
    // The revert-migration-step action always preserves a checkpoint
    // because the underlying transform may be lossy.
    if matches!(request.action_class, RepairActionClass::RevertMigrationStep) {
        checkpoint_required = true;
        if strictest_preview == PreviewClass::SafeApply {
            strictest_preview = PreviewClass::RollbackCheckpointRequired;
        }
    }
    if checkpoint_required && request.checkpoint_ref.is_none() {
        // Surface the missing checkpoint as a blocked-write reason
        // for every selected setting so reviewers can see exactly
        // which rows need a checkpoint preserved.
        for row in &write_intents {
            if !row.blocked {
                blocked.push(RepairBlockedWriteReason::CheckpointMissing {
                    setting_id: row.setting_id.clone(),
                });
            }
        }
    }
    if approval_required && request.approval_ticket_ref.is_none() {
        for row in &write_intents {
            if !row.blocked {
                blocked.push(RepairBlockedWriteReason::ApprovalMissing {
                    setting_id: row.setting_id.clone(),
                });
            }
        }
    }

    let hidden_reset_guard = HiddenResetGuard {
        would_broaden_scope: would_broaden,
        would_touch_adjacent_settings: !refused_settings.is_empty(),
        selected_setting_ids: request.selected_setting_ids.clone(),
        refused_setting_ids: refused_settings.clone(),
        summary: hidden_reset_summary(
            would_broaden,
            !refused_settings.is_empty(),
            target_scope_class,
        ),
    };

    if would_broaden
        && !blocked.iter().any(|reason| {
            matches!(
                reason,
                RepairBlockedWriteReason::ScopeBroadeningRefused { .. }
            )
        })
    {
        blocked.push(RepairBlockedWriteReason::ScopeBroadeningRefused {
            proposed_scope: request.target_scope.as_str().to_owned(),
            selected_scope: request.target_scope.as_str().to_owned(),
        });
    }

    let verdict = verdict_for(
        &blocked,
        checkpoint_required,
        approval_required,
        request.checkpoint_ref.is_some(),
        request.approval_ticket_ref.is_some(),
        underlying_preview,
    );

    let rollback_action_ref = if matches!(verdict, RepairPlanVerdict::Denied) {
        None
    } else {
        Some(format!(
            "settings://repair/rollback/{}",
            request.plan_id.replace(':', "-")
        ))
    };

    Ok(SettingsRepairPlan {
        record_kind: "settings_repair_plan".to_owned(),
        schema_version: SETTINGS_REPAIR_PLAN_SCHEMA_VERSION,
        shared_contract_ref: SETTINGS_REPAIR_PLAN_SHARED_CONTRACT_REF.to_owned(),
        plan_id: request.plan_id,
        action_class: request.action_class.as_str().to_owned(),
        target_scope: request.target_scope.as_str().to_owned(),
        target_scope_class: target_scope_class.as_str().to_owned(),
        target_artifact_ref,
        section_id: request.section_id,
        imported_profile_fragment: request.imported_profile_fragment,
        migration_step: request.migration_step,
        actor_class: request.actor_class.as_str().to_owned(),
        reason_class: request.reason_class.as_str().to_owned(),
        reason_note: request.reason_note,
        preview_class: strictest_preview.as_str().to_owned(),
        checkpoint_required,
        checkpoint_ref: request.checkpoint_ref,
        rollback_action_ref,
        approval_ticket_ref: request.approval_ticket_ref,
        approval_required,
        write_intents,
        blocked_write_reasons: blocked,
        locked_classes,
        hidden_reset_guard,
        verdict: verdict.as_str().to_owned(),
        user_decision: request.user_decision.as_str().to_owned(),
        effective_before,
    })
}

/// Action-specific review sheet projection. Each sheet wraps the
/// canonical [`SettingsRepairPlan`] with the headline copy callers
/// render at the top of the dialog. The sheet body is the plan
/// itself; surfaces never re-derive plan facts from the headline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsRepairReviewSheet {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta repair-plan schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable ref back to the underlying plan.
    pub source_plan_ref: String,
    /// Headline rendered above the diff affordance.
    pub headline: String,
    /// One-sentence subhead naming the exact scope and artifact.
    pub subhead: String,
    /// Short label describing the scope the plan touches.
    pub scope_label: String,
    /// Short label describing what the plan touches (one value, one
    /// section, one profile fragment, or one migration step).
    pub change_breadth_label: String,
    /// Wrapped repair plan.
    pub plan: SettingsRepairPlan,
}

/// Build a review sheet projection from a plan.
pub fn project_review_sheet(plan: SettingsRepairPlan) -> SettingsRepairReviewSheet {
    let (headline, subhead, change_breadth_label) = headline_strings_for(&plan);
    SettingsRepairReviewSheet {
        record_kind: "settings_repair_review_sheet".to_owned(),
        schema_version: SETTINGS_REPAIR_PLAN_SCHEMA_VERSION,
        shared_contract_ref: SETTINGS_REPAIR_PLAN_SHARED_CONTRACT_REF.to_owned(),
        source_plan_ref: format!("settings-repair-plan:{}", plan.plan_id),
        headline,
        subhead,
        scope_label: scope_label_for(plan.target_scope_class.as_str()).to_owned(),
        change_breadth_label,
        plan,
    }
}

/// Support-export wrapper around a set of repair plans. Carries the
/// same `source_plan_ref` every consumer reads so UI, CLI, and
/// support tooling report the same repair attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsRepairSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta repair-plan schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by support tooling.
    pub shared_contract_ref: String,
    /// Export id supplied by the caller.
    pub export_id: String,
    /// Repair plans included in the export, in deterministic order.
    pub plans: Vec<SettingsRepairPlan>,
    /// Number of plans whose verdict is `denied`.
    pub denied_plan_count: usize,
    /// Number of plans the user accepted before export.
    pub accepted_plan_count: usize,
    /// Number of plans the user declined before export.
    pub declined_plan_count: usize,
}

/// Build a support export from a set of plans.
pub fn project_support_export(
    export_id: impl Into<String>,
    plans: Vec<SettingsRepairPlan>,
) -> SettingsRepairSupportExport {
    let denied_plan_count = plans.iter().filter(|plan| plan.verdict == "denied").count();
    let accepted_plan_count = plans
        .iter()
        .filter(|plan| plan.user_decision == "accepted")
        .count();
    let declined_plan_count = plans
        .iter()
        .filter(|plan| plan.user_decision == "declined")
        .count();
    SettingsRepairSupportExport {
        record_kind: "settings_repair_support_export".to_owned(),
        schema_version: SETTINGS_REPAIR_PLAN_SCHEMA_VERSION,
        shared_contract_ref: SETTINGS_REPAIR_PLAN_SHARED_CONTRACT_REF.to_owned(),
        export_id: export_id.into(),
        plans,
        denied_plan_count,
        accepted_plan_count,
        declined_plan_count,
    }
}

fn collect_blocked_reasons(
    def: &SettingDefinition,
    preview: &SettingWritePreviewRecord,
    target_scope_class: RepairTargetScopeClass,
    would_broaden: &mut bool,
) -> Vec<RepairBlockedWriteReason> {
    let mut reasons: Vec<RepairBlockedWriteReason> = Vec::new();
    if matches!(def.lifecycle_label, LifecycleLabel::Retired) {
        reasons.push(RepairBlockedWriteReason::RetiredSetting {
            setting_id: def.setting_id.clone(),
        });
    }
    if !target_scope_class.is_writable() {
        reasons.push(RepairBlockedWriteReason::PolicyOwnedClass {
            setting_id: def.setting_id.clone(),
        });
        *would_broaden = true;
    }
    if preview.verdict == "denied" {
        match preview.denial_reason.as_deref() {
            Some("scope_not_allowed_for_setting") | Some("scope_not_allowed") => {
                reasons.push(RepairBlockedWriteReason::NonWritableScope {
                    setting_id: def.setting_id.clone(),
                    target_scope: preview.target_scope.clone(),
                });
            }
            Some("policy_locked_value") | Some("policy_locked") => {
                reasons.push(RepairBlockedWriteReason::PolicyOwnedClass {
                    setting_id: def.setting_id.clone(),
                });
                *would_broaden = true;
            }
            Some("policy_constrained_value") => {
                reasons.push(RepairBlockedWriteReason::PolicyOwnedClass {
                    setting_id: def.setting_id.clone(),
                });
            }
            Some("capability_dependency_unmet") => {
                reasons.push(RepairBlockedWriteReason::CapabilityDependencyUnmet {
                    setting_id: def.setting_id.clone(),
                });
            }
            Some("managed_mode_only") => {
                reasons.push(RepairBlockedWriteReason::ManagedModeOnly {
                    setting_id: def.setting_id.clone(),
                });
            }
            Some("setting_retired") => {
                reasons.push(RepairBlockedWriteReason::RetiredSetting {
                    setting_id: def.setting_id.clone(),
                });
            }
            Some("setting_unknown_at_registry") => {
                reasons.push(RepairBlockedWriteReason::UnknownSetting {
                    setting_id: def.setting_id.clone(),
                });
            }
            Some("rollback_checkpoint_not_created") => {
                reasons.push(RepairBlockedWriteReason::CheckpointMissing {
                    setting_id: def.setting_id.clone(),
                });
            }
            Some("approval_ticket_missing") => {
                reasons.push(RepairBlockedWriteReason::ApprovalMissing {
                    setting_id: def.setting_id.clone(),
                });
            }
            _ => {}
        }
    }
    reasons
}

fn track_locked_class(locked_classes: &mut Vec<String>, reason: &RepairBlockedWriteReason) {
    let token = match reason {
        RepairBlockedWriteReason::PolicyOwnedClass { .. } => "policy_owned_class",
        RepairBlockedWriteReason::CapabilityDependencyUnmet { .. } => "capability_locked",
        RepairBlockedWriteReason::RetiredSetting { .. } => "retired_setting",
        RepairBlockedWriteReason::ManagedModeOnly { .. } => "managed_mode_only",
        RepairBlockedWriteReason::NonWritableScope { .. } => "non_writable_scope",
        _ => return,
    };
    if !locked_classes.iter().any(|class| class == token) {
        locked_classes.push(token.to_owned());
    }
}

fn target_artifact_ref_for(
    action_class: RepairActionClass,
    target_scope: SettingScope,
    section_id: Option<&str>,
    fragment: Option<&ImportedProfileFragmentRef>,
    migration_step: Option<&MigrationStepRef>,
) -> String {
    match action_class {
        RepairActionClass::ResetCurrentValue | RepairActionClass::RepairDrift => {
            format!("settings://scope/{}", target_scope.as_str())
        }
        RepairActionClass::ResetSection => format!(
            "settings://scope/{}/section/{}",
            target_scope.as_str(),
            section_id.unwrap_or("all"),
        ),
        RepairActionClass::ReapplyImportedProfileFragment => fragment
            .map(|fragment| {
                format!(
                    "settings://profile/{}/fragment/{}",
                    fragment.profile_id, fragment.fragment_id,
                )
            })
            .unwrap_or_else(|| "settings://profile/unknown".to_owned()),
        RepairActionClass::RevertMigrationStep => migration_step
            .map(|step| format!("settings://migration/step/{}", step.migration_id))
            .unwrap_or_else(|| "settings://migration/unknown".to_owned()),
    }
}

fn strictest_preview_class(current: PreviewClass, candidate: PreviewClass) -> PreviewClass {
    fn rank(class: PreviewClass) -> u32 {
        match class {
            PreviewClass::SafeApply => 0,
            PreviewClass::PreviewRequired => 1,
            PreviewClass::RollbackCheckpointRequired => 2,
            PreviewClass::RollbackCheckpointAndApprovalRequired => 3,
            PreviewClass::ManagedActionOnly => 4,
        }
    }
    if rank(candidate) > rank(current) {
        candidate
    } else {
        current
    }
}

fn verdict_for(
    blocked: &[RepairBlockedWriteReason],
    checkpoint_required: bool,
    approval_required: bool,
    has_checkpoint: bool,
    has_approval: bool,
    strictest_preview: PreviewClass,
) -> RepairPlanVerdict {
    if blocked.iter().any(|reason| {
        !matches!(
            reason,
            RepairBlockedWriteReason::CheckpointMissing { .. }
                | RepairBlockedWriteReason::ApprovalMissing { .. }
        )
    }) {
        return RepairPlanVerdict::Denied;
    }
    if checkpoint_required && !has_checkpoint {
        return RepairPlanVerdict::AwaitingCheckpoint;
    }
    if approval_required && !has_approval {
        return RepairPlanVerdict::AwaitingApproval;
    }
    if matches!(
        strictest_preview,
        PreviewClass::PreviewRequired
            | PreviewClass::RollbackCheckpointRequired
            | PreviewClass::RollbackCheckpointAndApprovalRequired
            | PreviewClass::ManagedActionOnly
    ) {
        return RepairPlanVerdict::AwaitingPreview;
    }
    RepairPlanVerdict::ReadyToApply
}

fn hidden_reset_summary(
    would_broaden: bool,
    would_touch_adjacent: bool,
    target_scope_class: RepairTargetScopeClass,
) -> String {
    if !target_scope_class.is_writable() {
        return "Refused: target scope is policy-owned and cannot be repaired from a user-initiated plan.".to_owned();
    }
    if would_broaden && would_touch_adjacent {
        return "Refused: plan would have widened scope and touched adjacent rows.".to_owned();
    }
    if would_broaden {
        return "Refused: plan would have written to a broader scope than the user selected."
            .to_owned();
    }
    if would_touch_adjacent {
        return "Refused: plan would have touched settings outside the user selection.".to_owned();
    }
    "Confirmed: plan touches only the selected rows at the selected scope.".to_owned()
}

fn scope_label_for(class: &str) -> &'static str {
    match class {
        "user" => "User settings",
        "profile" => "Imported profile",
        "workspace" => "Workspace settings",
        "policy_owned" => "Policy-owned (locked)",
        "machine_local" => "This machine only",
        _ => "Unknown scope",
    }
}

fn headline_strings_for(plan: &SettingsRepairPlan) -> (String, String, String) {
    let row_count = plan.write_intents.len();
    match plan.action_class.as_str() {
        "reset_current_value" => (
            "Reset this value".to_owned(),
            format!(
                "Restores one value at {} to the next inherited source.",
                scope_label_for(&plan.target_scope_class),
            ),
            "Touches 1 value".to_owned(),
        ),
        "reset_section" => (
            "Reset this section".to_owned(),
            format!(
                "Restores every value in section {} at {} to inherited sources.",
                plan.section_id.as_deref().unwrap_or("all"),
                scope_label_for(&plan.target_scope_class),
            ),
            format!("Touches {row_count} values in one section"),
        ),
        "repair_drift" => (
            "Repair drifted value".to_owned(),
            format!(
                "Restores the value at {} back to the last intended setting.",
                scope_label_for(&plan.target_scope_class),
            ),
            "Touches 1 drifted value".to_owned(),
        ),
        "reapply_imported_profile_fragment" => (
            "Re-apply imported profile fragment".to_owned(),
            plan.imported_profile_fragment
                .as_ref()
                .map(|fragment| {
                    format!(
                        "Re-applies fragment '{}' from imported profile {}.",
                        fragment.fragment_label, fragment.profile_id,
                    )
                })
                .unwrap_or_else(|| "Re-applies one imported profile fragment.".to_owned()),
            format!("Touches {row_count} values from one profile fragment"),
        ),
        "revert_migration_step" => (
            "Revert migration step".to_owned(),
            plan.migration_step
                .as_ref()
                .map(|step| {
                    format!(
                        "Reverts migration step {} ({} → {}).",
                        step.migration_id, step.from_version, step.to_version,
                    )
                })
                .unwrap_or_else(|| "Reverts one migration step.".to_owned()),
            "Touches 1 migration step".to_owned(),
        ),
        _ => (
            "Repair plan".to_owned(),
            "Reviews one repair plan.".to_owned(),
            format!("Touches {row_count} value(s)"),
        ),
    }
}

fn lifecycle_token(label: LifecycleLabel) -> &'static str {
    match label {
        LifecycleLabel::Stable => "stable",
        LifecycleLabel::Preview => "beta",
        LifecycleLabel::Experimental => "experimental",
        LifecycleLabel::Deprecated => "deprecated",
        LifecycleLabel::Retired => "retired",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::{EffectiveSettingsResolver, PolicyConstraint, ScopeOverlay};
    use crate::schema::SchemaRegistry;

    fn seeded_resolver() -> EffectiveSettingsResolver {
        let mut resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value("editor.tab_size", SettingValue::Integer(8));
        user.set_value("editor.format_on_save", SettingValue::Boolean(true));
        user.set_value(
            "security.ai.egress_policy",
            SettingValue::String("any_hosted_provider".to_owned()),
        );
        resolver.set_overlay(user).unwrap();
        let mut workspace = ScopeOverlay::new(SettingScope::Workspace, "Workspace settings");
        workspace.set_value("editor.tab_size", SettingValue::Integer(2));
        resolver.set_overlay(workspace).unwrap();
        let mut policy =
            ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle v3");
        policy.set_policy_constraint(
            "security.ai.egress_policy",
            PolicyConstraint::SingleValue {
                value: SettingValue::String("approved_hosted_providers_only".to_owned()),
            },
        );
        resolver.set_overlay(policy).unwrap();
        resolver
    }

    fn empty_context() -> SettingsInspectionContext {
        SettingsInspectionContext::new()
    }

    #[test]
    fn reset_current_value_plan_is_scope_explicit_and_ready_to_apply() {
        let resolver = seeded_resolver();
        let request = SettingsRepairPlanRequest {
            plan_id: "plan:reset-tab-size-001".to_owned(),
            action_class: RepairActionClass::ResetCurrentValue,
            target_scope: SettingScope::Workspace,
            section_id: None,
            imported_profile_fragment: None,
            migration_step: None,
            actor_class: WriteActorClass::UserCommand,
            reason_class: WriteReasonClass::UserEdit,
            reason_note: Some("Reset workspace tab size back to user default".to_owned()),
            selected_setting_ids: vec!["editor.tab_size".to_owned()],
            proposed_values: vec![("editor.tab_size".to_owned(), SettingValue::Integer(4))],
            checkpoint_ref: None,
            approval_ticket_ref: None,
            user_decision: RepairUserDecision::Pending,
        };
        let plan = build_repair_plan(&resolver, &empty_context(), request).unwrap();

        assert_eq!(plan.action_class, "reset_current_value");
        assert_eq!(plan.target_scope, "workspace");
        assert_eq!(plan.target_scope_class, "workspace");
        assert_eq!(plan.write_intents.len(), 1);
        assert!(plan.blocked_write_reasons.is_empty());
        assert!(!plan.hidden_reset_guard.would_broaden_scope);
        assert!(!plan.hidden_reset_guard.would_touch_adjacent_settings);
        assert_eq!(plan.verdict, "ready_to_apply");
        assert!(plan.rollback_action_ref.is_some());
    }

    #[test]
    fn adjacent_setting_is_refused_to_prevent_hidden_resets() {
        let resolver = seeded_resolver();
        let request = SettingsRepairPlanRequest {
            plan_id: "plan:reset-adjacent-001".to_owned(),
            action_class: RepairActionClass::ResetCurrentValue,
            target_scope: SettingScope::Workspace,
            section_id: None,
            imported_profile_fragment: None,
            migration_step: None,
            actor_class: WriteActorClass::UserCommand,
            reason_class: WriteReasonClass::UserEdit,
            reason_note: None,
            // Only editor.tab_size selected, but proposed_values
            // includes an adjacent row (format_on_save). The plan
            // MUST refuse the adjacent row.
            selected_setting_ids: vec!["editor.tab_size".to_owned()],
            proposed_values: vec![
                ("editor.tab_size".to_owned(), SettingValue::Integer(4)),
                (
                    "editor.format_on_save".to_owned(),
                    SettingValue::Boolean(false),
                ),
            ],
            checkpoint_ref: None,
            approval_ticket_ref: None,
            user_decision: RepairUserDecision::Pending,
        };
        let plan = build_repair_plan(&resolver, &empty_context(), request).unwrap();

        assert!(plan.hidden_reset_guard.would_touch_adjacent_settings);
        assert!(plan
            .hidden_reset_guard
            .refused_setting_ids
            .iter()
            .any(|id| id == "editor.format_on_save"));
        assert!(plan.blocked_write_reasons.iter().any(|reason| matches!(
            reason,
            RepairBlockedWriteReason::AdjacentSettingRefused { setting_id }
                if setting_id == "editor.format_on_save"
        )));
        assert_eq!(plan.verdict, "denied");
        assert!(plan.rollback_action_ref.is_none());
    }

    #[test]
    fn policy_owned_target_scope_is_refused() {
        let resolver = seeded_resolver();
        let request = SettingsRepairPlanRequest {
            plan_id: "plan:reset-policy-001".to_owned(),
            action_class: RepairActionClass::ResetCurrentValue,
            target_scope: SettingScope::AdminPolicyNarrowing,
            section_id: None,
            imported_profile_fragment: None,
            migration_step: None,
            actor_class: WriteActorClass::UserCommand,
            reason_class: WriteReasonClass::UserEdit,
            reason_note: None,
            selected_setting_ids: vec!["security.ai.egress_policy".to_owned()],
            proposed_values: vec![(
                "security.ai.egress_policy".to_owned(),
                SettingValue::String("any_hosted_provider".to_owned()),
            )],
            checkpoint_ref: None,
            approval_ticket_ref: None,
            user_decision: RepairUserDecision::Pending,
        };
        let plan = build_repair_plan(&resolver, &empty_context(), request).unwrap();

        assert!(plan.hidden_reset_guard.would_broaden_scope);
        assert!(plan
            .locked_classes
            .iter()
            .any(|class| class == "policy_owned_class"));
        assert!(plan
            .blocked_write_reasons
            .iter()
            .any(|reason| matches!(reason, RepairBlockedWriteReason::PolicyOwnedClass { .. })));
        assert_eq!(plan.verdict, "denied");
    }

    #[test]
    fn reset_section_demands_checkpoint_before_apply() {
        let resolver = seeded_resolver();
        let request = SettingsRepairPlanRequest {
            plan_id: "plan:reset-editor-section-001".to_owned(),
            action_class: RepairActionClass::ResetSection,
            target_scope: SettingScope::UserGlobal,
            section_id: Some("editor".to_owned()),
            imported_profile_fragment: None,
            migration_step: None,
            actor_class: WriteActorClass::UserCommand,
            reason_class: WriteReasonClass::UserEdit,
            reason_note: None,
            selected_setting_ids: vec![
                "editor.tab_size".to_owned(),
                "editor.format_on_save".to_owned(),
            ],
            proposed_values: vec![
                ("editor.tab_size".to_owned(), SettingValue::Integer(4)),
                (
                    "editor.format_on_save".to_owned(),
                    SettingValue::Boolean(false),
                ),
            ],
            checkpoint_ref: None,
            approval_ticket_ref: None,
            user_decision: RepairUserDecision::Pending,
        };
        let plan = build_repair_plan(&resolver, &empty_context(), request).unwrap();

        assert!(plan.checkpoint_required);
        assert_eq!(plan.verdict, "awaiting_checkpoint");
        assert!(plan
            .blocked_write_reasons
            .iter()
            .any(|reason| matches!(reason, RepairBlockedWriteReason::CheckpointMissing { .. })));

        let request_with_checkpoint = SettingsRepairPlanRequest {
            plan_id: "plan:reset-editor-section-002".to_owned(),
            action_class: RepairActionClass::ResetSection,
            target_scope: SettingScope::UserGlobal,
            section_id: Some("editor".to_owned()),
            imported_profile_fragment: None,
            migration_step: None,
            actor_class: WriteActorClass::UserCommand,
            reason_class: WriteReasonClass::UserEdit,
            reason_note: None,
            selected_setting_ids: vec![
                "editor.tab_size".to_owned(),
                "editor.format_on_save".to_owned(),
            ],
            proposed_values: vec![
                ("editor.tab_size".to_owned(), SettingValue::Integer(4)),
                (
                    "editor.format_on_save".to_owned(),
                    SettingValue::Boolean(false),
                ),
            ],
            checkpoint_ref: Some("checkpoint:settings:user_global:editor:001".to_owned()),
            approval_ticket_ref: None,
            user_decision: RepairUserDecision::Pending,
        };
        let plan_ok =
            build_repair_plan(&resolver, &empty_context(), request_with_checkpoint).unwrap();
        assert_eq!(plan_ok.verdict, "ready_to_apply");
    }

    #[test]
    fn review_sheet_and_support_export_carry_plan_truth() {
        let resolver = seeded_resolver();
        let request = SettingsRepairPlanRequest {
            plan_id: "plan:reset-tab-size-002".to_owned(),
            action_class: RepairActionClass::ResetCurrentValue,
            target_scope: SettingScope::Workspace,
            section_id: None,
            imported_profile_fragment: None,
            migration_step: None,
            actor_class: WriteActorClass::UserCommand,
            reason_class: WriteReasonClass::UserEdit,
            reason_note: Some("Reset workspace tab size".to_owned()),
            selected_setting_ids: vec!["editor.tab_size".to_owned()],
            proposed_values: vec![("editor.tab_size".to_owned(), SettingValue::Integer(4))],
            checkpoint_ref: None,
            approval_ticket_ref: None,
            user_decision: RepairUserDecision::Accepted,
        };
        let plan = build_repair_plan(&resolver, &empty_context(), request).unwrap();
        let sheet = project_review_sheet(plan.clone());
        assert_eq!(sheet.shared_contract_ref, plan.shared_contract_ref);
        assert!(sheet.source_plan_ref.contains(&plan.plan_id));
        assert_eq!(sheet.plan.verdict, "ready_to_apply");
        assert_eq!(sheet.plan.user_decision, "accepted");
        assert!(sheet.headline.contains("Reset"));
        assert!(sheet.change_breadth_label.contains("1 value"));

        let export = project_support_export("support:repair:001", vec![plan]);
        assert_eq!(export.plans.len(), 1);
        assert_eq!(export.accepted_plan_count, 1);
        assert_eq!(export.denied_plan_count, 0);
    }
}
