//! Canonical seed builders for the M5 setting-write-intent and policy-constraint registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean write-intent and policy-constraint entries are built
//! so the one write-intent object landing per mutation, writes landing only in the chosen scope and artifact,
//! the preview / checkpoint / rollback recovery evidence materialized before any high-risk write applies, the
//! canonical / accessible / audit resolution forms, and the complete lock-source / allowed-override-classes /
//! expiry-review / validation-status / review-state / docs-pointer / last-review-revision policy-constraint
//! object are proven across the settings-resolver, shell, sync, policy, diagnostics, and support surfaces
//! without any hand-copied per-write assumption, scope rewrite, incomplete object, masked lock, or
//! resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_PACKET_ID: &str =
    "m5-setting-write-intent-and-policy-constraint-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn write_intent(
    input: M5SettingWriteIntentEntryResolutionInput,
) -> M5ResolvedSettingWriteIntentEntry {
    resolve_setting_write_intent_entry(input).expect("seed write-intent entry resolves")
}

fn constraint(input: M5PolicyConstraintEntryResolutionInput) -> M5ResolvedPolicyConstraintEntry {
    resolve_policy_constraint_entry(input).expect("seed policy-constraint entry resolves")
}

fn all_forms() -> Vec<M5ConfigWriteResolutionForm> {
    M5ConfigWriteResolutionForm::ALL.to_vec()
}

// -- Clean write-intent entries (one intent object, scope preserved, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_write_intent_base(
    entry_id: &str,
    write_target_id: &str,
    token_name: &str,
    semantic_role: M5SettingsGovernanceRole,
    preview_class: M5WriteIntentPreviewClass,
    surface_context: M5ConfigWriteSurfaceContext,
    target_scope: &str,
    target_artifact: &str,
    intended_value: &str,
    actor: &str,
    change_reason: &str,
    preview_reference: &str,
    recovery_reference: &str,
) -> M5SettingWriteIntentEntryResolutionInput {
    M5SettingWriteIntentEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        write_target_id: write_target_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        preview_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        target_scope: target_scope.to_owned(),
        target_artifact: target_artifact.to_owned(),
        intended_value: intended_value.to_owned(),
        actor: actor.to_owned(),
        change_reason: change_reason.to_owned(),
        preview_reference: preview_reference.to_owned(),
        recovery_reference: recovery_reference.to_owned(),
        bound_to_registry: true,
        scope_ownership_preserved: true,
        is_high_risk_write: false,
        evidence_materialized: true,
        proof_fresh: true,
    }
}

fn write_intent_no_op_settings_clean() -> M5ResolvedSettingWriteIntentEntry {
    write_intent(clean_write_intent_base(
        "write-intent:settings:no-op",
        "settings.acme.editor.format-on-save@workspace",
        "write.intent.editor.format_on_save",
        M5SettingsGovernanceRole::WriteIntent,
        M5WriteIntentPreviewClass::NoOpReversible,
        M5ConfigWriteSurfaceContext::SettingsSurface,
        "scope.workspace",
        "artifact.workspace-settings-json",
        "value.true",
        "actor.user-edit",
        "reason.enable-format-on-save",
        "preview.none-needed",
        "recovery.checkpoint-and-rollback-0007",
    ))
}

fn write_intent_low_risk_shell_clean() -> M5ResolvedSettingWriteIntentEntry {
    write_intent(clean_write_intent_base(
        "write-intent:shell:low-risk",
        "settings.acme.workbench.theme-mode@user",
        "write.intent.workbench.theme_mode",
        M5SettingsGovernanceRole::WriteIntent,
        M5WriteIntentPreviewClass::LowRiskReversible,
        M5ConfigWriteSurfaceContext::ShellSurface,
        "scope.user",
        "artifact.user-settings-json",
        "value.dark",
        "actor.profile-apply",
        "reason.apply-dark-theme",
        "preview.diff-summary-0007",
        "recovery.checkpoint-and-rollback-0007",
    ))
}

fn write_intent_material_diagnostics_clean() -> M5ResolvedSettingWriteIntentEntry {
    // A material behavior change is high-risk and materializes preview / checkpoint / rollback evidence.
    let mut base = clean_write_intent_base(
        "write-intent:diagnostics:material",
        "settings.acme.telemetry.sample-rate@machine",
        "write.intent.telemetry.sample_rate",
        M5SettingsGovernanceRole::WriteIntent,
        M5WriteIntentPreviewClass::MaterialBehaviorChange,
        M5ConfigWriteSurfaceContext::DiagnosticsSurface,
        "scope.machine",
        "artifact.machine-policy-json",
        "value.0-point-1",
        "actor.import",
        "reason.raise-sample-rate",
        "preview.behavior-diff-0007",
        "recovery.checkpoint-and-rollback-0007",
    );
    base.is_high_risk_write = true;
    base.evidence_materialized = true;
    write_intent(base)
}

fn write_intent_high_risk_admin_clean() -> M5ResolvedSettingWriteIntentEntry {
    // A high-risk irreversible change is high-risk and materializes preview / checkpoint / rollback evidence.
    let mut base = clean_write_intent_base(
        "write-intent:admin:high-risk",
        "settings.acme.tools.plugin-root@machine",
        "write.intent.tools.plugin_root",
        M5SettingsGovernanceRole::WriteIntent,
        M5WriteIntentPreviewClass::HighRiskIrreversible,
        M5ConfigWriteSurfaceContext::AdminSurface,
        "scope.machine",
        "artifact.machine-policy-json",
        "value.redacted-path",
        "actor.sync",
        "reason.repoint-plugin-root",
        "preview.behavior-diff-0007",
        "recovery.checkpoint-and-rollback-0007",
    );
    base.is_high_risk_write = true;
    base.evidence_materialized = true;
    write_intent(base)
}

fn write_intent_destructive_support_clean() -> M5ResolvedSettingWriteIntentEntry {
    // A destructive reset is high-risk and materializes preview / checkpoint / rollback evidence.
    let mut base = clean_write_intent_base(
        "write-intent:support:destructive",
        "settings.acme.sync.reset-state@machine",
        "write.intent.sync.reset_state",
        M5SettingsGovernanceRole::WriteIntent,
        M5WriteIntentPreviewClass::DestructiveReset,
        M5ConfigWriteSurfaceContext::SupportOrExportForm,
        "scope.machine",
        "artifact.machine-policy-json",
        "value.reset-to-default",
        "actor.automation",
        "reason.reset-sync-state",
        "preview.behavior-diff-0007",
        "recovery.checkpoint-and-rollback-0007",
    );
    base.is_high_risk_write = true;
    base.evidence_materialized = true;
    write_intent(base)
}

// -- Degraded write-intent entries --------------------------------------------------------------

/// Degraded write-intent entry: the resolved write-intent object is incomplete — the preview reference is
/// unstated.
fn write_intent_object_incomplete() -> M5ResolvedSettingWriteIntentEntry {
    let mut base = clean_write_intent_base(
        "write-intent:settings:incomplete",
        "settings.acme.editor.format-on-save@workspace",
        "write.intent.editor.format_on_save",
        M5SettingsGovernanceRole::WriteIntent,
        M5WriteIntentPreviewClass::NoOpReversible,
        M5ConfigWriteSurfaceContext::SettingsSurface,
        "scope.workspace",
        "artifact.workspace-settings-json",
        "value.true",
        "actor.user-edit",
        "reason.enable-format-on-save",
        "preview.none-needed",
        "recovery.checkpoint-and-rollback-0007",
    );
    base.preview_reference = "   ".to_owned();
    write_intent(base)
}

/// Degraded write-intent entry: the chosen scope / artifact ownership was rewritten into a broader scope.
fn write_intent_scope_rewritten() -> M5ResolvedSettingWriteIntentEntry {
    let mut base = clean_write_intent_base(
        "write-intent:sync:scope-rewritten",
        "settings.acme.telemetry.sample-rate@machine",
        "write.intent.telemetry.sample_rate",
        M5SettingsGovernanceRole::WriteIntent,
        M5WriteIntentPreviewClass::MaterialBehaviorChange,
        M5ConfigWriteSurfaceContext::DiagnosticsSurface,
        "scope.machine",
        "artifact.machine-policy-json",
        "value.0-point-1",
        "actor.import",
        "reason.raise-sample-rate",
        "preview.behavior-diff-0007",
        "recovery.checkpoint-and-rollback-0007",
    );
    base.is_high_risk_write = true;
    base.evidence_materialized = true;
    base.scope_ownership_preserved = false;
    write_intent(base)
}

/// Degraded write-intent entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn write_intent_unbound() -> M5ResolvedSettingWriteIntentEntry {
    let mut base = clean_write_intent_base(
        "write-intent:policy:unbound",
        "settings.acme.tools.plugin-root@machine",
        "write.intent.tools.plugin_root",
        M5SettingsGovernanceRole::WriteIntent,
        M5WriteIntentPreviewClass::HighRiskIrreversible,
        M5ConfigWriteSurfaceContext::AdminSurface,
        "scope.machine",
        "artifact.machine-policy-json",
        "value.redacted-path",
        "actor.sync",
        "reason.repoint-plugin-root",
        "preview.behavior-diff-0007",
        "recovery.checkpoint-and-rollback-0007",
    );
    base.is_high_risk_write = true;
    base.evidence_materialized = true;
    base.bound_to_registry = false;
    write_intent(base)
}

/// Degraded write-intent entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn write_intent_form_incomplete() -> M5ResolvedSettingWriteIntentEntry {
    let mut base = clean_write_intent_base(
        "write-intent:shell:form-incomplete",
        "settings.acme.workbench.theme-mode@user",
        "write.intent.workbench.theme_mode",
        M5SettingsGovernanceRole::WriteIntent,
        M5WriteIntentPreviewClass::LowRiskReversible,
        M5ConfigWriteSurfaceContext::ShellSurface,
        "scope.user",
        "artifact.user-settings-json",
        "value.dark",
        "actor.profile-apply",
        "reason.apply-dark-theme",
        "preview.diff-summary-0007",
        "recovery.checkpoint-and-rollback-0007",
    );
    base.resolution_form_coverage = vec![M5ConfigWriteResolutionForm::CanonicalObject];
    write_intent(base)
}

/// Degraded write-intent entry: the canonical registry token name is unstated.
fn write_intent_token_unstated() -> M5ResolvedSettingWriteIntentEntry {
    let mut base = clean_write_intent_base(
        "write-intent:support:token-unstated",
        "settings.acme.sync.reset-state@machine",
        "  ",
        M5SettingsGovernanceRole::WriteIntent,
        M5WriteIntentPreviewClass::DestructiveReset,
        M5ConfigWriteSurfaceContext::SupportOrExportForm,
        "scope.machine",
        "artifact.machine-policy-json",
        "value.reset-to-default",
        "actor.automation",
        "reason.reset-sync-state",
        "preview.behavior-diff-0007",
        "recovery.checkpoint-and-rollback-0007",
    );
    base.is_high_risk_write = true;
    base.evidence_materialized = true;
    base.token_name = "  ".to_owned();
    write_intent(base)
}

// -- Clean policy-constraint entries ------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_constraint_base(
    entry_id: &str,
    constraint_ref: &str,
    token_name: &str,
    semantic_role: M5SettingsGovernanceRole,
    lock_class: M5PolicyLockClass,
    surface_context: M5ConfigWriteSurfaceContext,
    lock_source: &str,
    allowed_override_classes: &str,
    expiry_review: &str,
    validation_status: &str,
    review_state: &str,
    docs_pointer: &str,
    last_review_revision: &str,
) -> M5PolicyConstraintEntryResolutionInput {
    M5PolicyConstraintEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        constraint_ref: constraint_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        lock_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        lock_source: lock_source.to_owned(),
        allowed_override_classes: allowed_override_classes.to_owned(),
        expiry_review: expiry_review.to_owned(),
        validation_status: validation_status.to_owned(),
        review_state: review_state.to_owned(),
        docs_pointer: docs_pointer.to_owned(),
        last_review_revision: last_review_revision.to_owned(),
        keeps_lock_source_visible: true,
        constraint_is_truthful: true,
        lock_present: false,
        lock_source_disclosed: false,
        denial_present: false,
        fallback_guidance_disclosed: false,
        proof_fresh: true,
    }
}

fn constraint_policy_locked_settings_clean() -> M5ResolvedPolicyConstraintEntry {
    // A locked value discloses its lock source rather than masking it.
    let mut base = clean_constraint_base(
        "constraint:settings:policy-locked",
        "editor.format_on_save",
        "constraint.editor.format_on_save",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5PolicyLockClass::PolicyLocked,
        M5ConfigWriteSurfaceContext::SettingsSurface,
        "lock.org-policy-bundle",
        "override.none",
        "review.expires-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.settings-policy-locked",
        "revision.0007",
    );
    base.lock_present = true;
    base.lock_source_disclosed = true;
    constraint(base)
}

fn constraint_override_allowed_shell_clean() -> M5ResolvedPolicyConstraintEntry {
    // A locked value that discloses both its lock source and the allowed override class.
    let mut base = clean_constraint_base(
        "constraint:shell:override-allowed",
        "workbench.theme_mode",
        "constraint.workbench.theme_mode",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5PolicyLockClass::OverrideAllowed,
        M5ConfigWriteSurfaceContext::ShellSurface,
        "lock.org-policy-bundle",
        "override.admin-with-reason",
        "review.expires-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.settings-override-allowed",
        "revision.0007",
    );
    base.lock_present = true;
    base.lock_source_disclosed = true;
    constraint(base)
}

fn constraint_advisory_diagnostics_clean() -> M5ResolvedPolicyConstraintEntry {
    // A denied write discloses its fallback guidance rather than reading as ambiguous failure copy.
    let mut base = clean_constraint_base(
        "constraint:diagnostics:advisory",
        "telemetry.sample_rate",
        "constraint.telemetry.sample_rate",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5PolicyLockClass::AdvisoryConstraint,
        M5ConfigWriteSurfaceContext::DiagnosticsSurface,
        "lock.advisory-guidance",
        "override.self-with-ack",
        "review.expires-2026-12-31",
        "validation.warn",
        "review.current",
        "docs.settings-advisory",
        "revision.0007",
    );
    base.denial_present = true;
    base.fallback_guidance_disclosed = true;
    constraint(base)
}

fn constraint_policy_locked_admin_clean() -> M5ResolvedPolicyConstraintEntry {
    let mut base = clean_constraint_base(
        "constraint:admin:policy-locked",
        "tools.plugin_root",
        "constraint.tools.plugin_root",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5PolicyLockClass::PolicyLocked,
        M5ConfigWriteSurfaceContext::AdminSurface,
        "lock.machine-policy-bundle",
        "override.none",
        "review.expires-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.settings-policy-locked",
        "revision.0007",
    );
    base.lock_present = true;
    base.lock_source_disclosed = true;
    constraint(base)
}

fn constraint_advisory_support_clean() -> M5ResolvedPolicyConstraintEntry {
    constraint(clean_constraint_base(
        "constraint:support:advisory",
        "sync.reset_state",
        "constraint.sync.reset_state",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5PolicyLockClass::AdvisoryConstraint,
        M5ConfigWriteSurfaceContext::SupportOrExportForm,
        "lock.advisory-guidance",
        "override.self-with-ack",
        "review.expires-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.settings-advisory",
        "revision.0007",
    ))
}

// -- Degraded policy-constraint entries ---------------------------------------------------------

/// Degraded constraint entry: the record would mask a locked value without disclosing its lock source — a
/// locked write reads as ambiguously unavailable when it has quietly hidden the cause.
fn constraint_masks_lock() -> M5ResolvedPolicyConstraintEntry {
    let mut base = clean_constraint_base(
        "constraint:settings:masks-lock",
        "editor.format_on_save",
        "constraint.editor.format_on_save",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5PolicyLockClass::PolicyLocked,
        M5ConfigWriteSurfaceContext::SettingsSurface,
        "lock.org-policy-bundle",
        "override.none",
        "review.expires-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.settings-policy-locked",
        "revision.0007",
    );
    base.lock_present = true;
    base.lock_source_disclosed = false;
    constraint(base)
}

/// Degraded constraint entry: the canonical / accessible / audit resolution-form coverage of the record is
/// incomplete.
fn constraint_form_incomplete() -> M5ResolvedPolicyConstraintEntry {
    let mut base = clean_constraint_base(
        "constraint:shell:form-incomplete",
        "workbench.theme_mode",
        "constraint.workbench.theme_mode",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5PolicyLockClass::OverrideAllowed,
        M5ConfigWriteSurfaceContext::ShellSurface,
        "lock.org-policy-bundle",
        "override.admin-with-reason",
        "review.expires-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.settings-override-allowed",
        "revision.0007",
    );
    base.lock_present = true;
    base.lock_source_disclosed = true;
    base.resolution_form_coverage = vec![M5ConfigWriteResolutionForm::CanonicalObject];
    constraint(base)
}

/// Degraded constraint entry: the lock class is unclassified.
fn constraint_class_unclassified() -> M5ResolvedPolicyConstraintEntry {
    constraint(clean_constraint_base(
        "constraint:policy:class-unclassified",
        "tools.plugin_root",
        "constraint.tools.plugin_root",
        M5SettingsGovernanceRole::PolicyConstraint,
        M5PolicyLockClass::LockClassUnclassified,
        M5ConfigWriteSurfaceContext::AdminSurface,
        "lock.machine-policy-bundle",
        "override.none",
        "review.expires-2026-12-31",
        "validation.ok",
        "review.current",
        "docs.settings-policy-locked",
        "revision.0007",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5SettingWriteIntentPolicyConstraintRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    write_intent_entries: Vec<M5ResolvedSettingWriteIntentEntry>,
    policy_constraint_entries: Vec<M5ResolvedPolicyConstraintEntry>,
) -> M5SettingWriteIntentPolicyConstraintRegistriesRow {
    M5SettingWriteIntentPolicyConstraintRegistriesRow {
        consumer_surface,
        qualification: M5SettingsGovernanceQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5SettingsGovernanceDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5SettingsGovernanceRequiredLabel::Identity,
            M5SettingsGovernanceRequiredLabel::SemanticRole,
            M5SettingsGovernanceRequiredLabel::RegistryReference,
            M5SettingsGovernanceRequiredLabel::WriteIntent,
            M5SettingsGovernanceRequiredLabel::LifecycleState,
        ],
        accessibility_routes: M5SettingsGovernanceAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ConfigWriteAnatomyPart::ALL.to_vec(),
        export_fields: M5ConfigWriteExportField::ALL.to_vec(),
        downgrade_triggers,
        write_intent_entries,
        policy_constraint_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_SCHEMA_REF,
            M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF,
            M5_POLICY_CONSTRAINT_LANDED_SCHEMA_REF,
        ]),
        rewrites_a_scoped_write_into_a_broader_scope: false,
        lands_a_write_in_an_unintended_artifact_or_scope: false,
        applies_a_high_risk_write_without_preview_checkpoint_or_rollback: false,
        hides_a_lock_or_policy_disable_cause_behind_generic_unavailable_copy: false,
    }
}

fn registry_rows() -> Vec<M5SettingWriteIntentPolicyConstraintRegistriesRow> {
    use M5SettingsGovernanceConsumerSurface as C;
    use M5SettingsGovernanceDowngradeTrigger as D;

    vec![
        base_row(
            C::SettingsResolver,
            "Settings-resolver owner",
            "The settings resolver lands the no-op write intent in its chosen workspace scope and artifact — target scope, target artifact, intended value, actor, change reason, preview reference, and checkpoint / rollback recovery reference — from the shared registry and resolves the policy-locked constraint for that setting; a write-intent object missing its preview reference and a policy constraint that masks a locked value without disclosing its lock source degrade honestly instead of reading as a clean pass",
            "evidence:m5-settings-governance-settings-resolver:001",
            vec![
                D::RewroteAScopedWriteIntoABroaderScope,
                D::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
                D::ProofStale,
            ],
            vec![
                write_intent_no_op_settings_clean(),
                write_intent_object_incomplete(),
            ],
            vec![
                constraint_policy_locked_settings_clean(),
                constraint_masks_lock(),
            ],
        ),
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell lands the low-risk write intent in its chosen user scope while disclosing the override-allowed constraint and its lock source; a resolution-form gap on a write-intent entry and on a policy constraint is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-settings-governance-shell-ui:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ScopeBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                write_intent_low_risk_shell_clean(),
                write_intent_form_incomplete(),
            ],
            vec![
                constraint_override_allowed_shell_clean(),
                constraint_form_incomplete(),
            ],
        ),
        base_row(
            C::SyncService,
            "Sync-service owner",
            "The sync service lands the material behavior change in its chosen machine scope with preview / checkpoint / rollback evidence and reports the advisory constraint with fallback guidance; a scoped write rewritten into a broader scope is caught before it can land in an unintended artifact",
            "evidence:m5-settings-governance-sync-service:001",
            vec![
                D::RewroteAScopedWriteIntoABroaderScope,
                D::ScopeBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                write_intent_material_diagnostics_clean(),
                write_intent_scope_rewritten(),
            ],
            vec![constraint_advisory_diagnostics_clean()],
        ),
        base_row(
            C::PolicyService,
            "Policy-service owner",
            "The policy service lands the high-risk irreversible write with materialized recovery evidence and bound to the registry while resolving the policy-locked constraint; a write intent that is a hand-copied per-entry assumption and a policy constraint on an unclassified lock class degrade honestly",
            "evidence:m5-settings-governance-policy-service:001",
            vec![
                D::ScopeBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![write_intent_high_risk_admin_clean(), write_intent_unbound()],
            vec![
                constraint_policy_locked_admin_clean(),
                constraint_class_unclassified(),
            ],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved write-intent and policy-constraint truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied write table",
            "evidence:m5-settings-governance-diagnostics:001",
            vec![
                D::RegistryReferenceUnstated,
                D::ScopeBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![
                write_intent_material_diagnostics_clean(),
                write_intent_form_incomplete(),
            ],
            vec![
                constraint_override_allowed_shell_clean(),
                constraint_form_incomplete(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved write-intent and policy-constraint truth, so a hand-copied constant, an unstated registry token, a rewritten scope, or a masked lock is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-settings-governance-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::WriteIntentUnstated,
                D::ProofStale,
            ],
            vec![
                write_intent_destructive_support_clean(),
                write_intent_token_unstated(),
            ],
            vec![constraint_advisory_support_clean()],
        ),
    ]
}

fn governance_review() -> M5SettingWriteIntentPolicyConstraintRegistriesGovernanceReview {
    M5SettingWriteIntentPolicyConstraintRegistriesGovernanceReview {
        write_intent_registry_names_token_role_and_class: true,
        write_lands_to_one_intent_object_from_shared_registry: true,
        target_scope_artifact_value_actor_reason_and_evidence_published: true,
        writes_land_only_in_chosen_scope_and_artifact: true,
        policy_constraint_keeps_lock_source_visible_and_discloses_fallback: true,
        recovery_evidence_materialized_for_high_risk_writes: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        settings_shell_diagnostics_admin_read_single_source: true,
        write_or_constraint_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SettingWriteIntentPolicyConstraintRegistriesConsumerProjection {
    M5SettingWriteIntentPolicyConstraintRegistriesConsumerProjection {
        settings_and_shell_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        sync_and_policy_services_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SettingWriteIntentPolicyConstraintRegistriesProofFreshness {
    M5SettingWriteIntentPolicyConstraintRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SettingWriteIntentPolicyConstraintRegistriesReleasePosture {
    M5SettingWriteIntentPolicyConstraintRegistriesReleasePosture {
        proof_packet_ref: M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        settings_governance_audit_ref:
            M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_SCHEMA_REF,
        M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF,
        M5_POLICY_CONSTRAINT_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 setting-write-intent and policy-constraint registries packet.
pub fn seeded_m5_setting_write_intent_and_policy_constraint_registries(
) -> M5SettingWriteIntentPolicyConstraintRegistriesPacket {
    M5SettingWriteIntentPolicyConstraintRegistriesPacket::new(
        M5SettingWriteIntentPolicyConstraintRegistriesPacketInput {
            packet_id: M5_SETTING_WRITE_INTENT_POLICY_CONSTRAINT_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 setting-write-intent and policy-constraint registries with one write-intent object landing per mutation, writes landing only in the chosen scope and artifact, preview / checkpoint / rollback recovery evidence materialized before any high-risk write applies, canonical / accessible / audit resolution-form coverage, and the complete lock-source / allowed-override-classes / expiry-review / validation-status / review-state / docs-pointer / last-review-revision policy-constraint object across settings-resolver, shell, sync, policy, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5SettingWriteIntentPolicyConstraintRegistriesVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the settings-resolver row is held at Beta pending write-intent parity on every platform;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_setting_write_intent_and_policy_constraint_registries_write_intent_beta_narrowed(
) -> M5SettingWriteIntentPolicyConstraintRegistriesPacket {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.packet_id =
        "m5-setting-write-intent-and-policy-constraint-registries:write-intent-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SettingsGovernanceConsumerSurface::SettingsResolver)
        .expect("settings-resolver row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the sync-service row is narrowed to Preview pending policy-constraint parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_setting_write_intent_and_policy_constraint_registries_policy_constraint_preview_narrowed(
) -> M5SettingWriteIntentPolicyConstraintRegistriesPacket {
    let mut packet = seeded_m5_setting_write_intent_and_policy_constraint_registries();
    packet.packet_id =
        "m5-setting-write-intent-and-policy-constraint-registries:policy-constraint-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SettingsGovernanceConsumerSurface::SyncService)
        .expect("sync-service row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Preview;
    packet
}
