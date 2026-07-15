//! Canonical seed builders for the M5 sync-conflict and device-action registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean sync-conflict and device-action entries are built so
//! the one conflict packet landing per conflict, resolutions that never collapse into last-writer-wins, local
//! authoritative state preserved before any protected (policy-locked / machine-only / stale-remote) conflict
//! applies, the canonical / accessible / audit resolution forms, and the complete actor / action-timestamp /
//! transport-state / policy-state / capability-dependency / attribution-reference / last-ledger-revision device-
//! action-record object are proven across the settings-resolver, shell, sync, policy, diagnostics, and support
//! surfaces without any hand-copied per-conflict assumption, collapsed resolution, incomplete packet, hidden
//! ledger, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_PACKET_ID: &str =
    "m5-setting-sync-conflict-and-device-action-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn conflict(input: M5SyncConflictPacketEntryResolutionInput) -> M5ResolvedSyncConflictPacketEntry {
    resolve_sync_conflict_packet_entry(input).expect("seed sync-conflict entry resolves")
}

fn device(input: M5DeviceActionRecordEntryResolutionInput) -> M5ResolvedDeviceActionRecordEntry {
    resolve_device_action_record_entry(input).expect("seed device-action entry resolves")
}

fn all_forms() -> Vec<M5ConfigSyncResolutionForm> {
    M5ConfigSyncResolutionForm::ALL.to_vec()
}

// -- Clean sync-conflict entries (one packet, field-aware, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_conflict_base(
    entry_id: &str,
    conflict_ref: &str,
    token_name: &str,
    semantic_role: M5SettingsGovernanceRole,
    conflict_class: M5SyncConflictClass,
    surface_context: M5ConfigSyncSurfaceContext,
    field_path: &str,
    local_revision: &str,
    remote_revision: &str,
    keep_local_option: &str,
    keep_synced_option: &str,
    compare_reference: &str,
    blocked_state_reason: &str,
) -> M5SyncConflictPacketEntryResolutionInput {
    M5SyncConflictPacketEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        conflict_ref: conflict_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        conflict_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        field_path: field_path.to_owned(),
        local_revision: local_revision.to_owned(),
        remote_revision: remote_revision.to_owned(),
        keep_local_option: keep_local_option.to_owned(),
        keep_synced_option: keep_synced_option.to_owned(),
        compare_reference: compare_reference.to_owned(),
        blocked_state_reason: blocked_state_reason.to_owned(),
        bound_to_registry: true,
        resolution_is_field_aware: true,
        requires_local_authoritative: false,
        local_authority_preserved: true,
        proof_fresh: true,
    }
}

fn conflict_same_key_session_clean() -> M5ResolvedSyncConflictPacketEntry {
    conflict(clean_conflict_base(
        "conflict:session:same-key",
        "settings.acme.editor.font-size@device-42",
        "conflict.editor.font_size",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::SameKeyDivergent,
        M5ConfigSyncSurfaceContext::SyncSessionFlow,
        "editor.font-size",
        "local.rev-0007",
        "remote.rev-0009",
        "keep-local.font-size-14",
        "keep-synced.font-size-16",
        "compare.field-diff-0007",
        "blocked.none-review-and-choose",
    ))
}

fn conflict_policy_locked_import_clean() -> M5ResolvedSyncConflictPacketEntry {
    // A policy-locked conflict preserves local authoritative state before it applies.
    let mut base = clean_conflict_base(
        "conflict:import:policy-locked",
        "settings.acme.security.telemetry-optin@device-42",
        "conflict.security.telemetry_optin",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::PolicyLocked,
        M5ConfigSyncSurfaceContext::ImportApplyFlow,
        "security.telemetry-optin",
        "local.rev-0011",
        "remote.rev-0004",
        "keep-local.policy-locked-off",
        "keep-synced.blocked-by-policy",
        "compare.field-diff-0011",
        "blocked.policy-lock-holds-local",
    );
    base.requires_local_authoritative = true;
    base.local_authority_preserved = true;
    conflict(base)
}

fn conflict_machine_only_outage_clean() -> M5ResolvedSyncConflictPacketEntry {
    // A machine-only conflict preserves local authoritative state during an outage.
    let mut base = clean_conflict_base(
        "conflict:outage:machine-only",
        "settings.acme.runtime.gpu-adapter@device-42",
        "conflict.runtime.gpu_adapter",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::MachineOnly,
        M5ConfigSyncSurfaceContext::OutageRecoveryFlow,
        "runtime.gpu-adapter",
        "local.rev-0013",
        "remote.rev-0002",
        "keep-local.machine-only-adapter",
        "keep-synced.not-portable",
        "compare.field-diff-0013",
        "blocked.machine-only-stays-local",
    );
    base.requires_local_authoritative = true;
    base.local_authority_preserved = true;
    conflict(base)
}

fn conflict_stale_remote_review_clean() -> M5ResolvedSyncConflictPacketEntry {
    // A stale-remote conflict preserves local durable authoritative state.
    let mut base = clean_conflict_base(
        "conflict:review:stale-remote",
        "settings.acme.workbench.layout@device-42",
        "conflict.workbench.layout",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::StaleRemote,
        M5ConfigSyncSurfaceContext::DeviceReviewFlow,
        "workbench.layout",
        "local.rev-0021",
        "remote.rev-0015",
        "keep-local.newer-layout",
        "keep-synced.stale-remote",
        "compare.field-diff-0021",
        "blocked.stale-remote-local-wins",
    );
    base.requires_local_authoritative = true;
    base.local_authority_preserved = true;
    conflict(base)
}

fn conflict_delete_versus_modify_support_clean() -> M5ResolvedSyncConflictPacketEntry {
    conflict(clean_conflict_base(
        "conflict:support:delete-versus-modify",
        "settings.acme.tools.formatter@device-42",
        "conflict.tools.formatter",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::DeleteVersusModify,
        M5ConfigSyncSurfaceContext::SupportOrExportForm,
        "tools.formatter",
        "local.rev-0031",
        "remote.rev-deleted",
        "keep-local.restore-modified",
        "keep-synced.accept-delete",
        "compare.field-diff-0031",
        "blocked.none-review-and-choose",
    ))
}

fn conflict_missing_capability_session_clean() -> M5ResolvedSyncConflictPacketEntry {
    conflict(clean_conflict_base(
        "conflict:session:missing-capability",
        "settings.acme.ai.assist-model@device-42",
        "conflict.ai.assist_model",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::MissingCapability,
        M5ConfigSyncSurfaceContext::SyncSessionFlow,
        "ai.assist-model",
        "local.rev-0041",
        "remote.rev-0044",
        "keep-local.no-capability",
        "keep-synced.needs-capability",
        "compare.field-diff-0041",
        "blocked.missing-capability-review",
    ))
}

// -- Degraded sync-conflict entries -------------------------------------------------------------

/// Degraded conflict entry: the resolved conflict packet is incomplete — the blocked-state reason is unstated.
fn conflict_packet_incomplete() -> M5ResolvedSyncConflictPacketEntry {
    let mut base = clean_conflict_base(
        "conflict:session:incomplete",
        "settings.acme.editor.font-size@device-42",
        "conflict.editor.font_size",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::SameKeyDivergent,
        M5ConfigSyncSurfaceContext::SyncSessionFlow,
        "editor.font-size",
        "local.rev-0007",
        "remote.rev-0009",
        "keep-local.font-size-14",
        "keep-synced.font-size-16",
        "compare.field-diff-0007",
        "blocked.none-review-and-choose",
    );
    base.blocked_state_reason = "   ".to_owned();
    conflict(base)
}

/// Degraded conflict entry: a protected machine-only conflict silently overwrote local authoritative state.
fn conflict_overwrite_fold() -> M5ResolvedSyncConflictPacketEntry {
    let mut base = clean_conflict_base(
        "conflict:outage:overwrite",
        "settings.acme.runtime.gpu-adapter@device-42",
        "conflict.runtime.gpu_adapter",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::MachineOnly,
        M5ConfigSyncSurfaceContext::OutageRecoveryFlow,
        "runtime.gpu-adapter",
        "local.rev-0013",
        "remote.rev-0002",
        "keep-local.machine-only-adapter",
        "keep-synced.not-portable",
        "compare.field-diff-0013",
        "blocked.machine-only-stays-local",
    );
    base.requires_local_authoritative = true;
    base.local_authority_preserved = false;
    conflict(base)
}

/// Degraded conflict entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn conflict_unbound() -> M5ResolvedSyncConflictPacketEntry {
    let mut base = clean_conflict_base(
        "conflict:review:unbound",
        "settings.acme.workbench.layout@device-42",
        "conflict.workbench.layout",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::StaleRemote,
        M5ConfigSyncSurfaceContext::DeviceReviewFlow,
        "workbench.layout",
        "local.rev-0021",
        "remote.rev-0015",
        "keep-local.newer-layout",
        "keep-synced.stale-remote",
        "compare.field-diff-0021",
        "blocked.stale-remote-local-wins",
    );
    base.requires_local_authoritative = true;
    base.local_authority_preserved = true;
    base.bound_to_registry = false;
    conflict(base)
}

/// Degraded conflict entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn conflict_form_incomplete() -> M5ResolvedSyncConflictPacketEntry {
    let mut base = clean_conflict_base(
        "conflict:import:form-incomplete",
        "settings.acme.security.telemetry-optin@device-42",
        "conflict.security.telemetry_optin",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::PolicyLocked,
        M5ConfigSyncSurfaceContext::ImportApplyFlow,
        "security.telemetry-optin",
        "local.rev-0011",
        "remote.rev-0004",
        "keep-local.policy-locked-off",
        "keep-synced.blocked-by-policy",
        "compare.field-diff-0011",
        "blocked.policy-lock-holds-local",
    );
    base.requires_local_authoritative = true;
    base.local_authority_preserved = true;
    base.resolution_form_coverage = vec![M5ConfigSyncResolutionForm::CanonicalObject];
    conflict(base)
}

/// Degraded conflict entry: the canonical registry token name is unstated.
fn conflict_token_unstated() -> M5ResolvedSyncConflictPacketEntry {
    let mut base = clean_conflict_base(
        "conflict:support:token-unstated",
        "settings.acme.tools.formatter@device-42",
        "  ",
        M5SettingsGovernanceRole::SyncConflict,
        M5SyncConflictClass::DeleteVersusModify,
        M5ConfigSyncSurfaceContext::SupportOrExportForm,
        "tools.formatter",
        "local.rev-0031",
        "remote.rev-deleted",
        "keep-local.restore-modified",
        "keep-synced.accept-delete",
        "compare.field-diff-0031",
        "blocked.none-review-and-choose",
    );
    base.token_name = "  ".to_owned();
    conflict(base)
}

// -- Clean device-action entries ----------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_device_base(
    entry_id: &str,
    device_ref: &str,
    token_name: &str,
    semantic_role: M5SettingsGovernanceRole,
    device_action_class: M5DeviceActionClass,
    surface_context: M5ConfigSyncSurfaceContext,
    actor: &str,
    action_timestamp: &str,
    transport_state: &str,
    policy_state: &str,
    capability_dependency: &str,
    attribution_reference: &str,
    last_ledger_revision: &str,
) -> M5DeviceActionRecordEntryResolutionInput {
    M5DeviceActionRecordEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        device_ref: device_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        device_action_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        actor: actor.to_owned(),
        action_timestamp: action_timestamp.to_owned(),
        transport_state: transport_state.to_owned(),
        policy_state: policy_state.to_owned(),
        capability_dependency: capability_dependency.to_owned(),
        attribution_reference: attribution_reference.to_owned(),
        last_ledger_revision: last_ledger_revision.to_owned(),
        keeps_attribution_visible: true,
        ledger_is_truthful: true,
        revocation_present: false,
        revocation_reason_disclosed: false,
        degraded_transport_present: false,
        local_authority_preserved_disclosed: false,
        proof_fresh: true,
    }
}

fn device_pause_session_clean() -> M5ResolvedDeviceActionRecordEntry {
    device(clean_device_base(
        "device:session:pause",
        "device-42",
        "device_action.pause",
        M5SettingsGovernanceRole::SyncConflict,
        M5DeviceActionClass::PauseSync,
        M5ConfigSyncSurfaceContext::SyncSessionFlow,
        "actor.owner-user",
        "ts.2026-07-15T00-00-00Z",
        "transport.online-encrypted",
        "policy.allow-sync",
        "capability.sync-core",
        "attribution.ledger-0007",
        "revision.0007",
    ))
}

fn device_resume_import_clean() -> M5ResolvedDeviceActionRecordEntry {
    device(clean_device_base(
        "device:import:resume",
        "device-42",
        "device_action.resume",
        M5SettingsGovernanceRole::SyncConflict,
        M5DeviceActionClass::ResumeSync,
        M5ConfigSyncSurfaceContext::ImportApplyFlow,
        "actor.owner-user",
        "ts.2026-07-15T00-05-00Z",
        "transport.online-encrypted",
        "policy.allow-sync",
        "capability.sync-core",
        "attribution.ledger-0008",
        "revision.0008",
    ))
}

fn device_revoke_outage_clean() -> M5ResolvedDeviceActionRecordEntry {
    // A revoke action discloses its cause rather than hiding it.
    let mut base = clean_device_base(
        "device:outage:revoke",
        "device-77",
        "device_action.revoke",
        M5SettingsGovernanceRole::SyncConflict,
        M5DeviceActionClass::RevokeDevice,
        M5ConfigSyncSurfaceContext::OutageRecoveryFlow,
        "actor.admin-user",
        "ts.2026-07-15T00-10-00Z",
        "transport.offline-degraded",
        "policy.revoke-grant",
        "capability.sync-core",
        "attribution.ledger-0009",
        "revision.0009",
    );
    base.revocation_present = true;
    base.revocation_reason_disclosed = true;
    base.degraded_transport_present = true;
    base.local_authority_preserved_disclosed = true;
    device(base)
}

fn device_forget_review_clean() -> M5ResolvedDeviceActionRecordEntry {
    // A forget action discloses its cause rather than hiding it.
    let mut base = clean_device_base(
        "device:review:forget",
        "device-88",
        "device_action.forget",
        M5SettingsGovernanceRole::SyncConflict,
        M5DeviceActionClass::ForgetDevice,
        M5ConfigSyncSurfaceContext::DeviceReviewFlow,
        "actor.owner-user",
        "ts.2026-07-15T00-15-00Z",
        "transport.online-encrypted",
        "policy.forget-device",
        "capability.sync-core",
        "attribution.ledger-0010",
        "revision.0010",
    );
    base.revocation_present = true;
    base.revocation_reason_disclosed = true;
    device(base)
}

fn device_rotate_support_clean() -> M5ResolvedDeviceActionRecordEntry {
    device(clean_device_base(
        "device:support:rotate",
        "device-42",
        "device_action.rotate",
        M5SettingsGovernanceRole::SyncConflict,
        M5DeviceActionClass::RotateToken,
        M5ConfigSyncSurfaceContext::SupportOrExportForm,
        "actor.owner-user",
        "ts.2026-07-15T00-20-00Z",
        "transport.online-encrypted",
        "policy.allow-sync",
        "capability.sync-core",
        "attribution.ledger-0011",
        "revision.0011",
    ))
}

// -- Degraded device-action entries -------------------------------------------------------------

/// Degraded device entry: the record would hide a revoke cause without disclosing its reason — a revoked device
/// reads as ambiguously unavailable when it has quietly dropped the cause.
fn device_ledger_hides() -> M5ResolvedDeviceActionRecordEntry {
    let mut base = clean_device_base(
        "device:session:ledger-hides",
        "device-77",
        "device_action.revoke",
        M5SettingsGovernanceRole::SyncConflict,
        M5DeviceActionClass::RevokeDevice,
        M5ConfigSyncSurfaceContext::SyncSessionFlow,
        "actor.admin-user",
        "ts.2026-07-15T00-10-00Z",
        "transport.online-encrypted",
        "policy.revoke-grant",
        "capability.sync-core",
        "attribution.ledger-0009",
        "revision.0009",
    );
    base.revocation_present = true;
    base.revocation_reason_disclosed = false;
    device(base)
}

/// Degraded device entry: the canonical / accessible / audit resolution-form coverage of the record is
/// incomplete.
fn device_ledger_form_incomplete() -> M5ResolvedDeviceActionRecordEntry {
    let mut base = clean_device_base(
        "device:import:form-incomplete",
        "device-42",
        "device_action.resume",
        M5SettingsGovernanceRole::SyncConflict,
        M5DeviceActionClass::ResumeSync,
        M5ConfigSyncSurfaceContext::ImportApplyFlow,
        "actor.owner-user",
        "ts.2026-07-15T00-05-00Z",
        "transport.online-encrypted",
        "policy.allow-sync",
        "capability.sync-core",
        "attribution.ledger-0008",
        "revision.0008",
    );
    base.resolution_form_coverage = vec![M5ConfigSyncResolutionForm::CanonicalObject];
    device(base)
}

/// Degraded device entry: the device-action class is unclassified.
fn device_class_unclassified() -> M5ResolvedDeviceActionRecordEntry {
    device(clean_device_base(
        "device:review:class-unclassified",
        "device-88",
        "device_action.unknown",
        M5SettingsGovernanceRole::SyncConflict,
        M5DeviceActionClass::DeviceActionClassUnclassified,
        M5ConfigSyncSurfaceContext::DeviceReviewFlow,
        "actor.owner-user",
        "ts.2026-07-15T00-15-00Z",
        "transport.online-encrypted",
        "policy.forget-device",
        "capability.sync-core",
        "attribution.ledger-0010",
        "revision.0010",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5SettingSyncConflictDeviceActionRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    sync_conflict_entries: Vec<M5ResolvedSyncConflictPacketEntry>,
    device_action_entries: Vec<M5ResolvedDeviceActionRecordEntry>,
) -> M5SettingSyncConflictDeviceActionRegistriesRow {
    M5SettingSyncConflictDeviceActionRegistriesRow {
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
        anatomy_parts: M5ConfigSyncAnatomyPart::ALL.to_vec(),
        export_fields: M5ConfigSyncExportField::ALL.to_vec(),
        downgrade_triggers,
        sync_conflict_entries,
        device_action_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_SCHEMA_REF,
            M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF,
            M5_SYNC_DEVICE_RECORD_LANDED_SCHEMA_REF,
        ]),
        silently_overwrites_locked_or_machine_only_state_during_sync: false,
        collapses_conflict_classes_into_last_writer_wins: false,
        resolves_a_conflict_without_a_field_level_keep_local_or_blocked_reason: false,
        loses_device_action_lineage_in_diagnostics_or_support: false,
    }
}

fn registry_rows() -> Vec<M5SettingSyncConflictDeviceActionRegistriesRow> {
    use M5SettingsGovernanceConsumerSurface as C;
    use M5SettingsGovernanceDowngradeTrigger as D;

    vec![
        base_row(
            C::SettingsResolver,
            "Settings-resolver owner",
            "The settings resolver lands the same-key-divergent sync-conflict packet — field path, local and remote revisions, keep-local option, keep-synced option, compare reference, and blocked-state reason — from the shared registry and records the pause device action for that device; a conflict packet missing its blocked-state reason and a device action that hides a revoke cause without disclosing its reason degrade honestly instead of reading as a clean pass",
            "evidence:m5-settings-governance-settings-resolver:001",
            vec![
                D::SilentlyOverwroteLockedOrMachineOnlyStateDuringSync,
                D::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
                D::ProofStale,
            ],
            vec![conflict_same_key_session_clean(), conflict_packet_incomplete()],
            vec![device_pause_session_clean(), device_ledger_hides()],
        ),
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell lands the policy-locked import conflict while preserving local authoritative state and records the resume device action; a resolution-form gap on a conflict entry and on a device action is caught before a screenshot can reintroduce a false clean-sync reading",
            "evidence:m5-settings-governance-shell-ui:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SyncConflictRuleUnstated,
                D::ProofStale,
            ],
            vec![
                conflict_policy_locked_import_clean(),
                conflict_form_incomplete(),
            ],
            vec![device_resume_import_clean(), device_ledger_form_incomplete()],
        ),
        base_row(
            C::SyncService,
            "Sync-service owner",
            "The sync service lands the machine-only outage conflict with local authoritative state preserved and records the revoke device action with its cause and local-authority posture disclosed; a machine-only conflict that would silently overwrite local state is caught before it can collapse into last-writer-wins",
            "evidence:m5-settings-governance-sync-service:001",
            vec![
                D::SilentlyOverwroteLockedOrMachineOnlyStateDuringSync,
                D::SyncConflictRuleUnstated,
                D::ProofStale,
            ],
            vec![
                conflict_machine_only_outage_clean(),
                conflict_overwrite_fold(),
            ],
            vec![device_revoke_outage_clean()],
        ),
        base_row(
            C::PolicyService,
            "Policy-service owner",
            "The policy service lands the stale-remote review conflict with local durable state authoritative and bound to the registry while recording the forget device action; a conflict that is a hand-copied per-entry assumption and a device action on an unclassified class degrade honestly",
            "evidence:m5-settings-governance-policy-service:001",
            vec![
                D::ScopeBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![
                conflict_stale_remote_review_clean(),
                conflict_unbound(),
            ],
            vec![device_forget_review_clean(), device_class_unclassified()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved sync-conflict and device-action truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied conflict table",
            "evidence:m5-settings-governance-diagnostics:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SyncConflictRuleUnstated,
                D::ProofStale,
            ],
            vec![
                conflict_missing_capability_session_clean(),
                conflict_form_incomplete(),
            ],
            vec![device_resume_import_clean(), device_ledger_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved sync-conflict and device-action truth, so a hand-copied constant, an unstated registry token, a collapsed last-writer-wins resolution, or a hidden device-action ledger is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-settings-governance-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::LifecycleStateUnstated,
                D::ProofStale,
            ],
            vec![
                conflict_delete_versus_modify_support_clean(),
                conflict_token_unstated(),
            ],
            vec![device_rotate_support_clean()],
        ),
    ]
}

fn governance_review() -> M5SettingSyncConflictDeviceActionRegistriesGovernanceReview {
    M5SettingSyncConflictDeviceActionRegistriesGovernanceReview {
        sync_conflict_registry_names_token_role_and_class: true,
        conflict_resolves_to_one_packet_from_shared_registry: true,
        field_path_revisions_keep_local_keep_synced_and_blocked_reason_published: true,
        conflicts_never_collapse_into_last_writer_wins: true,
        device_action_record_keeps_attribution_visible_and_discloses_cause: true,
        local_authority_preserved_before_protected_conflict_applies: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        sync_import_outage_and_device_review_read_single_source: true,
        conflict_or_ledger_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SettingSyncConflictDeviceActionRegistriesConsumerProjection {
    M5SettingSyncConflictDeviceActionRegistriesConsumerProjection {
        sync_and_import_consume_shared_registries: true,
        outage_and_device_review_consume_shared_registries: true,
        sync_and_device_services_consume_shared_registries: true,
        docs_admin_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SettingSyncConflictDeviceActionRegistriesProofFreshness {
    M5SettingSyncConflictDeviceActionRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SettingSyncConflictDeviceActionRegistriesReleasePosture {
    M5SettingSyncConflictDeviceActionRegistriesReleasePosture {
        proof_packet_ref: M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_ARTIFACT_REF.to_owned(),
        settings_governance_audit_ref: M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_SCHEMA_REF,
        M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF,
        M5_SYNC_DEVICE_RECORD_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 sync-conflict and device-action registries packet.
pub fn seeded_m5_setting_sync_conflict_and_device_action_registries(
) -> M5SettingSyncConflictDeviceActionRegistriesPacket {
    M5SettingSyncConflictDeviceActionRegistriesPacket::new(
        M5SettingSyncConflictDeviceActionRegistriesPacketInput {
            packet_id: M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 sync-conflict and device-action registries with one conflict packet landing per conflict, resolutions that never collapse into last-writer-wins, local authoritative state preserved before any protected conflict applies, canonical / accessible / audit resolution-form coverage, and the complete actor / action-timestamp / transport-state / policy-state / capability-dependency / attribution-reference / last-ledger-revision device-action-record object across settings-resolver, shell, sync, policy, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5SettingSyncConflictDeviceActionRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the settings-resolver row is held at Beta pending sync-conflict parity on every platform;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_setting_sync_conflict_and_device_action_registries_sync_conflict_beta_narrowed(
) -> M5SettingSyncConflictDeviceActionRegistriesPacket {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.packet_id =
        "m5-setting-sync-conflict-and-device-action-registries:sync-conflict-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SettingsGovernanceConsumerSurface::SettingsResolver)
        .expect("settings-resolver row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the sync-service row is narrowed to Preview pending device-action parity on every platform;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_setting_sync_conflict_and_device_action_registries_device_action_preview_narrowed(
) -> M5SettingSyncConflictDeviceActionRegistriesPacket {
    let mut packet = seeded_m5_setting_sync_conflict_and_device_action_registries();
    packet.packet_id =
        "m5-setting-sync-conflict-and-device-action-registries:device-action-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5SettingsGovernanceConsumerSurface::SyncService)
        .expect("sync-service row present");
    row.qualification = M5SettingsGovernanceQualificationClass::Preview;
    packet
}
