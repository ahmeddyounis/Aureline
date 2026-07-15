//! Canonical seed builders for the M5 capability-record and kill-switch-record registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean capability and kill-switch entries are built so the
//! one capability record landing per capability, dependency markers that never hide behind unpublished flags, a
//! fallback published before any protected (Labs / Preview / Beta) capability is claimed, the canonical /
//! accessible / audit resolution forms, and the complete disabling-source / disabled-timestamp /
//! preserved-data-reference / explanation-reference / capability-dependency / fallback-reference /
//! last-ledger-revision kill-switch-record object are proven across the settings-resolver, shell, sync, policy,
//! diagnostics, and support surfaces without any hand-copied per-capability assumption, hidden dependency,
//! incomplete record, hidden kill-switch cause, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_PACKET_ID: &str =
    "m5-setting-capability-lifecycle-and-kill-switch-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn capability(input: M5CapabilityRecordEntryResolutionInput) -> M5ResolvedCapabilityRecordEntry {
    resolve_capability_record_entry(input).expect("seed capability entry resolves")
}

fn kill_switch(input: M5KillSwitchRecordEntryResolutionInput) -> M5ResolvedKillSwitchRecordEntry {
    resolve_kill_switch_record_entry(input).expect("seed kill-switch entry resolves")
}

fn all_forms() -> Vec<M5ConfigCapabilityResolutionForm> {
    M5ConfigCapabilityResolutionForm::ALL.to_vec()
}

// -- Clean capability entries (one record, dependency-published, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_capability_base(
    entry_id: &str,
    capability_ref: &str,
    token_name: &str,
    semantic_role: M5SettingsGovernanceRole,
    lifecycle_class: M5CapabilityLifecycleClass,
    surface_context: M5ConfigCapabilitySurfaceContext,
    owner: &str,
    scope: &str,
    review_or_expiry: &str,
    enabled_posture: &str,
    dependency_marker: &str,
    fallback: &str,
    rollback_note: &str,
) -> M5CapabilityRecordEntryResolutionInput {
    M5CapabilityRecordEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        capability_ref: capability_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        lifecycle_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        owner: owner.to_owned(),
        scope: scope.to_owned(),
        review_or_expiry: review_or_expiry.to_owned(),
        enabled_posture: enabled_posture.to_owned(),
        dependency_marker: dependency_marker.to_owned(),
        fallback: fallback.to_owned(),
        rollback_note: rollback_note.to_owned(),
        bound_to_registry: true,
        dependency_marker_published: true,
        requires_dependency_marker_and_fallback: false,
        fallback_published: true,
        proof_fresh: true,
    }
}

fn capability_labs_settings_clean() -> M5ResolvedCapabilityRecordEntry {
    // A Labs capability publishes an explicit dependency marker and fallback before a stable surface depends on it.
    let mut base = clean_capability_base(
        "capability:settings:labs",
        "capability.acme.ai.inline-assist@labs",
        "capability.ai.inline_assist",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::Labs,
        M5ConfigCapabilitySurfaceContext::SettingsSurfaceFlow,
        "owner.ai-platform-team",
        "scope.user",
        "review.2026-10-01-labs-review",
        "posture.opt-in-off-by-default",
        "dependency.marker-ai-runtime-v3",
        "fallback.classic-completion",
        "rollback.disable-restores-classic",
    );
    base.requires_dependency_marker_and_fallback = true;
    base.fallback_published = true;
    capability(base)
}

fn capability_preview_docs_clean() -> M5ResolvedCapabilityRecordEntry {
    // A Preview capability publishes an explicit dependency marker and fallback.
    let mut base = clean_capability_base(
        "capability:docs:preview",
        "capability.acme.editor.live-share@preview",
        "capability.editor.live_share",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::Preview,
        M5ConfigCapabilitySurfaceContext::DocsHelpFlow,
        "owner.collab-team",
        "scope.workspace",
        "review.2026-09-01-preview-review",
        "posture.opt-in-preview",
        "dependency.marker-collab-relay-v2",
        "fallback.single-user-editing",
        "rollback.disable-keeps-local-edits",
    );
    base.requires_dependency_marker_and_fallback = true;
    base.fallback_published = true;
    capability(base)
}

fn capability_beta_bundle_clean() -> M5ResolvedCapabilityRecordEntry {
    // A Beta capability publishes an explicit dependency marker and fallback.
    let mut base = clean_capability_base(
        "capability:bundle:beta",
        "capability.acme.search.semantic-index@beta",
        "capability.search.semantic_index",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::Beta,
        M5ConfigCapabilitySurfaceContext::BundleFlow,
        "owner.search-team",
        "scope.workspace",
        "review.2026-08-15-beta-review",
        "posture.opt-in-beta",
        "dependency.marker-index-service-v4",
        "fallback.lexical-search",
        "rollback.disable-restores-lexical",
    );
    base.requires_dependency_marker_and_fallback = true;
    base.fallback_published = true;
    capability(base)
}

fn capability_generally_available_import_clean() -> M5ResolvedCapabilityRecordEntry {
    capability(clean_capability_base(
        "capability:import:generally-available",
        "capability.acme.git.smart-merge@ga",
        "capability.git.smart_merge",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::GenerallyAvailable,
        M5ConfigCapabilitySurfaceContext::ImportApplyFlow,
        "owner.scm-team",
        "scope.user",
        "review.2027-01-01-annual-review",
        "posture.on-by-default",
        "dependency.marker-merge-engine-v5",
        "fallback.manual-merge",
        "rollback.disable-restores-manual-merge",
    ))
}

fn capability_deprecated_settings_clean() -> M5ResolvedCapabilityRecordEntry {
    capability(clean_capability_base(
        "capability:settings:deprecated",
        "capability.acme.terminal.legacy-shell@deprecated",
        "capability.terminal.legacy_shell",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::Deprecated,
        M5ConfigCapabilitySurfaceContext::SettingsSurfaceFlow,
        "owner.terminal-team",
        "scope.user",
        "review.2026-12-01-removal-review",
        "posture.on-until-removal",
        "dependency.marker-legacy-pty-v1",
        "fallback.modern-shell",
        "rollback.removal-migrates-to-modern-shell",
    ))
}

fn capability_graduated_support_clean() -> M5ResolvedCapabilityRecordEntry {
    capability(clean_capability_base(
        "capability:support:graduated",
        "capability.acme.debug.time-travel@graduated",
        "capability.debug.time_travel",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::Graduated,
        M5ConfigCapabilitySurfaceContext::SupportOrExportForm,
        "owner.debug-team",
        "scope.workspace",
        "review.2027-02-01-annual-review",
        "posture.on-by-default",
        "dependency.marker-trace-store-v3",
        "fallback.forward-only-debugging",
        "rollback.disable-restores-forward-only",
    ))
}

// -- Degraded capability entries ----------------------------------------------------------------

/// Degraded capability entry: the resolved capability record is incomplete — the rollback note is unstated.
fn capability_record_incomplete() -> M5ResolvedCapabilityRecordEntry {
    let mut base = clean_capability_base(
        "capability:settings:incomplete",
        "capability.acme.ai.inline-assist@labs",
        "capability.ai.inline_assist",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::Labs,
        M5ConfigCapabilitySurfaceContext::SettingsSurfaceFlow,
        "owner.ai-platform-team",
        "scope.user",
        "review.2026-10-01-labs-review",
        "posture.opt-in-off-by-default",
        "dependency.marker-ai-runtime-v3",
        "fallback.classic-completion",
        "rollback.disable-restores-classic",
    );
    base.requires_dependency_marker_and_fallback = true;
    base.rollback_note = "   ".to_owned();
    capability(base)
}

/// Degraded capability entry: a protected Beta capability hid its dependency behind an unpublished flag by
/// publishing no fallback.
fn capability_dependency_hidden_fold() -> M5ResolvedCapabilityRecordEntry {
    let mut base = clean_capability_base(
        "capability:bundle:dependency-hidden",
        "capability.acme.search.semantic-index@beta",
        "capability.search.semantic_index",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::Beta,
        M5ConfigCapabilitySurfaceContext::BundleFlow,
        "owner.search-team",
        "scope.workspace",
        "review.2026-08-15-beta-review",
        "posture.opt-in-beta",
        "dependency.marker-index-service-v4",
        "fallback.lexical-search",
        "rollback.disable-restores-lexical",
    );
    base.requires_dependency_marker_and_fallback = true;
    base.fallback_published = false;
    capability(base)
}

/// Degraded capability entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn capability_unbound() -> M5ResolvedCapabilityRecordEntry {
    let mut base = clean_capability_base(
        "capability:import:unbound",
        "capability.acme.git.smart-merge@ga",
        "capability.git.smart_merge",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::GenerallyAvailable,
        M5ConfigCapabilitySurfaceContext::ImportApplyFlow,
        "owner.scm-team",
        "scope.user",
        "review.2027-01-01-annual-review",
        "posture.on-by-default",
        "dependency.marker-merge-engine-v5",
        "fallback.manual-merge",
        "rollback.disable-restores-manual-merge",
    );
    base.bound_to_registry = false;
    capability(base)
}

/// Degraded capability entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn capability_form_incomplete() -> M5ResolvedCapabilityRecordEntry {
    let mut base = clean_capability_base(
        "capability:docs:form-incomplete",
        "capability.acme.editor.live-share@preview",
        "capability.editor.live_share",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::Preview,
        M5ConfigCapabilitySurfaceContext::DocsHelpFlow,
        "owner.collab-team",
        "scope.workspace",
        "review.2026-09-01-preview-review",
        "posture.opt-in-preview",
        "dependency.marker-collab-relay-v2",
        "fallback.single-user-editing",
        "rollback.disable-keeps-local-edits",
    );
    base.requires_dependency_marker_and_fallback = true;
    base.fallback_published = true;
    base.resolution_form_coverage = vec![M5ConfigCapabilityResolutionForm::CanonicalObject];
    capability(base)
}

/// Degraded capability entry: the canonical registry token name is unstated.
fn capability_token_unstated() -> M5ResolvedCapabilityRecordEntry {
    let mut base = clean_capability_base(
        "capability:support:token-unstated",
        "capability.acme.debug.time-travel@graduated",
        "  ",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5CapabilityLifecycleClass::Graduated,
        M5ConfigCapabilitySurfaceContext::SupportOrExportForm,
        "owner.debug-team",
        "scope.workspace",
        "review.2027-02-01-annual-review",
        "posture.on-by-default",
        "dependency.marker-trace-store-v3",
        "fallback.forward-only-debugging",
        "rollback.disable-restores-forward-only",
    );
    base.token_name = "  ".to_owned();
    capability(base)
}

// -- Clean kill-switch entries ------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_kill_switch_base(
    entry_id: &str,
    capability_ref: &str,
    token_name: &str,
    semantic_role: M5SettingsGovernanceRole,
    kill_switch_class: M5KillSwitchClass,
    surface_context: M5ConfigCapabilitySurfaceContext,
    disabling_source: &str,
    disabled_timestamp: &str,
    preserved_data_reference: &str,
    explanation_reference: &str,
    capability_dependency: &str,
    fallback_reference: &str,
    last_ledger_revision: &str,
) -> M5KillSwitchRecordEntryResolutionInput {
    M5KillSwitchRecordEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        capability_ref: capability_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        kill_switch_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        disabling_source: disabling_source.to_owned(),
        disabled_timestamp: disabled_timestamp.to_owned(),
        preserved_data_reference: preserved_data_reference.to_owned(),
        explanation_reference: explanation_reference.to_owned(),
        capability_dependency: capability_dependency.to_owned(),
        fallback_reference: fallback_reference.to_owned(),
        last_ledger_revision: last_ledger_revision.to_owned(),
        keeps_disabling_source_visible: true,
        ledger_is_truthful: true,
        policy_disable_present: false,
        disable_cause_disclosed: false,
        user_data_present: false,
        user_data_preservation_disclosed: false,
        proof_fresh: true,
    }
}

fn kill_switch_settings_clean() -> M5ResolvedKillSwitchRecordEntry {
    // A kill switch discloses its cause and that user-authored data stays preserved.
    let mut base = clean_kill_switch_base(
        "kill-switch:settings:kill-switch",
        "capability.acme.ai.inline-assist@labs",
        "kill_switch.ai.inline_assist",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5KillSwitchClass::KillSwitch,
        M5ConfigCapabilitySurfaceContext::SettingsSurfaceFlow,
        "source.remote-kill-switch-signal",
        "ts.2026-07-15T00-00-00Z",
        "preserved.user-prompts-retained",
        "explain.disabled-by-remote-kill-switch",
        "capability.ai.inline_assist",
        "fallback.classic-completion",
        "revision.0007",
    );
    base.policy_disable_present = true;
    base.disable_cause_disclosed = true;
    base.user_data_present = true;
    base.user_data_preservation_disclosed = true;
    kill_switch(base)
}

fn kill_switch_policy_docs_clean() -> M5ResolvedKillSwitchRecordEntry {
    // A policy disable (DisabledByPolicy) discloses its cause and that user data stays preserved.
    let mut base = clean_kill_switch_base(
        "kill-switch:docs:policy-disabled",
        "capability.acme.editor.live-share@preview",
        "kill_switch.editor.live_share",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5KillSwitchClass::PolicyDisabled,
        M5ConfigCapabilitySurfaceContext::DocsHelpFlow,
        "source.org-policy-collab-off",
        "ts.2026-07-15T00-05-00Z",
        "preserved.local-edits-retained",
        "explain.disabled-by-org-policy",
        "capability.editor.live_share",
        "fallback.single-user-editing",
        "revision.0008",
    );
    base.policy_disable_present = true;
    base.disable_cause_disclosed = true;
    base.user_data_present = true;
    base.user_data_preservation_disclosed = true;
    kill_switch(base)
}

fn kill_switch_dependency_bundle_clean() -> M5ResolvedKillSwitchRecordEntry {
    // A dependency-unavailable disable discloses that local durable state stays preserved.
    let mut base = clean_kill_switch_base(
        "kill-switch:bundle:dependency-unavailable",
        "capability.acme.search.semantic-index@beta",
        "kill_switch.search.semantic_index",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5KillSwitchClass::DependencyUnavailable,
        M5ConfigCapabilitySurfaceContext::BundleFlow,
        "source.index-service-unavailable",
        "ts.2026-07-15T00-10-00Z",
        "preserved.saved-queries-retained",
        "explain.disabled-dependency-unavailable",
        "capability.search.semantic_index",
        "fallback.lexical-search",
        "revision.0009",
    );
    base.user_data_present = true;
    base.user_data_preservation_disclosed = true;
    kill_switch(base)
}

fn kill_switch_expired_import_clean() -> M5ResolvedKillSwitchRecordEntry {
    // A review-expired disable discloses that user-authored data stays preserved.
    let mut base = clean_kill_switch_base(
        "kill-switch:import:review-expired",
        "capability.acme.git.smart-merge@ga",
        "kill_switch.git.smart_merge",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5KillSwitchClass::ReviewExpired,
        M5ConfigCapabilitySurfaceContext::ImportApplyFlow,
        "source.review-window-lapsed",
        "ts.2026-07-15T00-15-00Z",
        "preserved.merge-history-retained",
        "explain.disabled-review-expired",
        "capability.git.smart_merge",
        "fallback.manual-merge",
        "revision.0010",
    );
    base.user_data_present = true;
    base.user_data_preservation_disclosed = true;
    kill_switch(base)
}

fn kill_switch_manual_support_clean() -> M5ResolvedKillSwitchRecordEntry {
    kill_switch(clean_kill_switch_base(
        "kill-switch:support:manual-opt-out",
        "capability.acme.debug.time-travel@graduated",
        "kill_switch.debug.time_travel",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5KillSwitchClass::ManualOptOut,
        M5ConfigCapabilitySurfaceContext::SupportOrExportForm,
        "source.user-opt-out",
        "ts.2026-07-15T00-20-00Z",
        "preserved.trace-snapshots-retained",
        "explain.disabled-by-user-choice",
        "capability.debug.time_travel",
        "fallback.forward-only-debugging",
        "revision.0011",
    ))
}

// -- Degraded kill-switch entries ---------------------------------------------------------------

/// Degraded kill-switch entry: the record would hide a kill-switch cause without disclosing its reason — a
/// disabled capability reads as ambiguously unavailable when it has quietly dropped the cause.
fn kill_switch_hides_cause() -> M5ResolvedKillSwitchRecordEntry {
    let mut base = clean_kill_switch_base(
        "kill-switch:settings:hides-cause",
        "capability.acme.ai.inline-assist@labs",
        "kill_switch.ai.inline_assist",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5KillSwitchClass::KillSwitch,
        M5ConfigCapabilitySurfaceContext::SettingsSurfaceFlow,
        "source.remote-kill-switch-signal",
        "ts.2026-07-15T00-00-00Z",
        "preserved.user-prompts-retained",
        "explain.disabled-by-remote-kill-switch",
        "capability.ai.inline_assist",
        "fallback.classic-completion",
        "revision.0007",
    );
    base.policy_disable_present = true;
    base.disable_cause_disclosed = false;
    kill_switch(base)
}

/// Degraded kill-switch entry: the canonical / accessible / audit resolution-form coverage of the record is
/// incomplete.
fn kill_switch_form_incomplete() -> M5ResolvedKillSwitchRecordEntry {
    let mut base = clean_kill_switch_base(
        "kill-switch:docs:form-incomplete",
        "capability.acme.editor.live-share@preview",
        "kill_switch.editor.live_share",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5KillSwitchClass::PolicyDisabled,
        M5ConfigCapabilitySurfaceContext::DocsHelpFlow,
        "source.org-policy-collab-off",
        "ts.2026-07-15T00-05-00Z",
        "preserved.local-edits-retained",
        "explain.disabled-by-org-policy",
        "capability.editor.live_share",
        "fallback.single-user-editing",
        "revision.0008",
    );
    base.policy_disable_present = true;
    base.disable_cause_disclosed = true;
    base.user_data_present = true;
    base.user_data_preservation_disclosed = true;
    base.resolution_form_coverage = vec![M5ConfigCapabilityResolutionForm::CanonicalObject];
    kill_switch(base)
}

/// Degraded kill-switch entry: the kill-switch class is unclassified.
fn kill_switch_class_unclassified() -> M5ResolvedKillSwitchRecordEntry {
    kill_switch(clean_kill_switch_base(
        "kill-switch:import:class-unclassified",
        "capability.acme.git.smart-merge@ga",
        "kill_switch.unknown",
        M5SettingsGovernanceRole::CapabilityLifecycle,
        M5KillSwitchClass::KillSwitchClassUnclassified,
        M5ConfigCapabilitySurfaceContext::ImportApplyFlow,
        "source.review-window-lapsed",
        "ts.2026-07-15T00-15-00Z",
        "preserved.merge-history-retained",
        "explain.disabled-review-expired",
        "capability.git.smart_merge",
        "fallback.manual-merge",
        "revision.0010",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5SettingCapabilityLifecycleKillSwitchRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    capability_entries: Vec<M5ResolvedCapabilityRecordEntry>,
    kill_switch_entries: Vec<M5ResolvedKillSwitchRecordEntry>,
) -> M5SettingCapabilityLifecycleKillSwitchRegistriesRow {
    M5SettingCapabilityLifecycleKillSwitchRegistriesRow {
        consumer_surface,
        qualification: M5SettingsGovernanceQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5SettingsGovernanceDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5SettingsGovernanceRequiredLabel::Identity,
            M5SettingsGovernanceRequiredLabel::SemanticRole,
            M5SettingsGovernanceRequiredLabel::RegistryReference,
            M5SettingsGovernanceRequiredLabel::WinningScope,
            M5SettingsGovernanceRequiredLabel::LifecycleState,
        ],
        accessibility_routes: M5SettingsGovernanceAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ConfigCapabilityAnatomyPart::ALL.to_vec(),
        export_fields: M5ConfigCapabilityExportField::ALL.to_vec(),
        downgrade_triggers,
        capability_entries,
        kill_switch_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_SCHEMA_REF,
            M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF,
            M5_CAPABILITY_LIFECYCLE_LANDED_SCHEMA_REF,
        ]),
        hides_lifecycle_or_experiment_dependency_behind_unpublished_markers: false,
        hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy: false,
        lets_a_stable_surface_depend_on_a_hidden_labs_or_preview_capability: false,
        loses_user_authored_data_when_a_kill_switch_or_policy_disable_fires: false,
    }
}

fn registry_rows() -> Vec<M5SettingCapabilityLifecycleKillSwitchRegistriesRow> {
    use M5SettingsGovernanceConsumerSurface as C;
    use M5SettingsGovernanceDowngradeTrigger as D;

    vec![
        base_row(
            C::SettingsResolver,
            "Settings-resolver owner",
            "The settings resolver lands the Labs capability record — owner, scope, review / expiry, enabled posture, dependency marker, fallback, and rollback note — from the shared registry and records the kill-switch disable for that capability; a capability record missing its rollback note and a kill-switch record that hides its cause without disclosing its reason degrade honestly instead of reading as a clean pass",
            "evidence:m5-settings-governance-settings-resolver:001",
            vec![
                D::HidLifecycleOrExperimentDependencyBehindUnpublishedMarkers,
                D::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
                D::ProofStale,
            ],
            vec![capability_labs_settings_clean(), capability_record_incomplete()],
            vec![kill_switch_settings_clean(), kill_switch_hides_cause()],
        ),
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell lands the Preview capability record while publishing its dependency marker and fallback and records the policy-disable; a resolution-form gap on a capability entry and on a kill-switch record is caught before a screenshot can reintroduce a false clean-lifecycle reading",
            "evidence:m5-settings-governance-shell-ui:001",
            vec![
                D::RegistryReferenceUnstated,
                D::LifecycleStateUnstated,
                D::ProofStale,
            ],
            vec![
                capability_preview_docs_clean(),
                capability_form_incomplete(),
            ],
            vec![kill_switch_policy_docs_clean(), kill_switch_form_incomplete()],
        ),
        base_row(
            C::SyncService,
            "Sync-service owner",
            "The sync service lands the Beta capability record with its dependency marker and fallback published and records the dependency-unavailable disable with its cause and data-preservation posture disclosed; a Beta capability that would hide its dependency by publishing no fallback is caught before a stable surface can depend on it",
            "evidence:m5-settings-governance-sync-service:001",
            vec![
                D::HidLifecycleOrExperimentDependencyBehindUnpublishedMarkers,
                D::LifecycleStateUnstated,
                D::ProofStale,
            ],
            vec![
                capability_beta_bundle_clean(),
                capability_dependency_hidden_fold(),
            ],
            vec![kill_switch_dependency_bundle_clean()],
        ),
        base_row(
            C::PolicyService,
            "Policy-service owner",
            "The policy service lands the generally-available capability record bound to the registry while recording the review-expired disable; a capability that is a hand-copied per-entry assumption and a kill-switch record on an unclassified class degrade honestly",
            "evidence:m5-settings-governance-policy-service:001",
            vec![
                D::ScopeBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![
                capability_generally_available_import_clean(),
                capability_unbound(),
            ],
            vec![kill_switch_expired_import_clean(), kill_switch_class_unclassified()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved capability and kill-switch truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied capability table",
            "evidence:m5-settings-governance-diagnostics:001",
            vec![
                D::RegistryReferenceUnstated,
                D::LifecycleStateUnstated,
                D::ProofStale,
            ],
            vec![
                capability_deprecated_settings_clean(),
                capability_form_incomplete(),
            ],
            vec![kill_switch_policy_docs_clean(), kill_switch_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export and claim publication carry the same resolved capability and kill-switch truth, so a hand-copied constant, an unstated registry token, a hidden Labs/Preview dependency, or a hidden kill-switch cause is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-settings-governance-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
                D::ProofStale,
            ],
            vec![
                capability_graduated_support_clean(),
                capability_token_unstated(),
            ],
            vec![kill_switch_manual_support_clean()],
        ),
    ]
}

fn governance_review() -> M5SettingCapabilityLifecycleKillSwitchRegistriesGovernanceReview {
    M5SettingCapabilityLifecycleKillSwitchRegistriesGovernanceReview {
        capability_registry_names_token_role_and_class: true,
        capability_resolves_to_one_record_from_shared_registry: true,
        owner_scope_review_dependency_marker_fallback_and_rollback_published: true,
        no_stable_surface_depends_on_a_hidden_labs_or_preview_capability: true,
        kill_switch_record_keeps_source_visible_and_discloses_cause: true,
        user_authored_data_preserved_before_kill_switch_fires: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        settings_docs_bundle_and_import_read_single_source: true,
        capability_or_ledger_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SettingCapabilityLifecycleKillSwitchRegistriesConsumerProjection {
    M5SettingCapabilityLifecycleKillSwitchRegistriesConsumerProjection {
        settings_and_docs_consume_shared_registries: true,
        bundle_and_import_consume_shared_registries: true,
        sync_and_policy_services_consume_shared_registries: true,
        docs_admin_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SettingCapabilityLifecycleKillSwitchRegistriesProofFreshness {
    M5SettingCapabilityLifecycleKillSwitchRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SettingCapabilityLifecycleKillSwitchRegistriesReleasePosture {
    M5SettingCapabilityLifecycleKillSwitchRegistriesReleasePosture {
        proof_packet_ref: M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        settings_governance_audit_ref:
            M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_SCHEMA_REF,
        M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF,
        M5_CAPABILITY_LIFECYCLE_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 capability-record and kill-switch-record registries packet.
pub fn seeded_m5_setting_capability_lifecycle_and_kill_switch_registries(
) -> M5SettingCapabilityLifecycleKillSwitchRegistriesPacket {
    M5SettingCapabilityLifecycleKillSwitchRegistriesPacket::new(
        M5SettingCapabilityLifecycleKillSwitchRegistriesPacketInput {
            packet_id: M5_SETTING_CAPABILITY_LIFECYCLE_KILL_SWITCH_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 capability-record and kill-switch-record registries with one capability record landing per capability, dependency markers that never hide behind unpublished flags, a fallback published before any protected capability is claimed, canonical / accessible / audit resolution-form coverage, and the complete disabling-source / disabled-timestamp / preserved-data-reference / explanation-reference / capability-dependency / fallback-reference / last-ledger-revision kill-switch-record object across settings-resolver, shell, sync, policy, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5SettingCapabilityLifecycleKillSwitchRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the settings-resolver row is held at Beta pending capability-lifecycle parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_setting_capability_lifecycle_and_kill_switch_registries_capability_lifecycle_beta_narrowed(
) -> M5SettingCapabilityLifecycleKillSwitchRegistriesPacket {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.packet_id =
        "m5-setting-capability-lifecycle-and-kill-switch-registries:capability-lifecycle-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SettingsGovernanceConsumerSurface::SettingsResolver)
        .expect("settings-resolver row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the sync-service row is narrowed to Preview pending kill-switch parity on every platform;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_setting_capability_lifecycle_and_kill_switch_registries_kill_switch_preview_narrowed(
) -> M5SettingCapabilityLifecycleKillSwitchRegistriesPacket {
    let mut packet = seeded_m5_setting_capability_lifecycle_and_kill_switch_registries();
    packet.packet_id =
        "m5-setting-capability-lifecycle-and-kill-switch-registries:kill-switch-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SettingsGovernanceConsumerSurface::SyncService)
        .expect("sync-service row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Preview;
    packet
}
