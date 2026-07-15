//! Implemented M5 sync-conflict-packet and device-action-record registries.
//!
//! The frozen [settings-governance matrix][matrix] names Aureline's five configuration-runtime families and
//! locks their controlled vocabulary. This is the sync / conflict engine implement lane over the `sync_scope`
//! family: it turns the *sync-conflict-packet* grammar (how a sync scope bundle, session, and conflict packet
//! declare which field diverged, the local and remote revisions, the field-level keep-local / keep-synced
//! options, the compare surface, and the blocked-state reason a conflict class carries) and the
//! *device-action-record* grammar (how a device action ledger records the actor, timestamp, transport and policy
//! state, capability dependency, attribution, and last ledger revision for a pause, resume, revoke, forget, or
//! token-rotation action) into registry resolvers that produce export-safe, honest projections. Every claimed M5
//! sync conflict then resolves to one sync-conflict-packet object — the conflict class it classifies (same-key
//! divergent / policy-locked / missing-capability / machine-only / delete-versus-modify / stale-remote), the
//! field path, the local and remote revisions, the keep-local option, the keep-synced option, the compare
//! reference, and the blocked-state reason — and every claimed device action resolves to one device-action-record
//! object — the actor, the action timestamp, the transport state, the policy state, the capability dependency,
//! the attribution reference, and the last ledger revision — that the sync-session, import-apply, outage-recovery,
//! device-review, and support / export flows can inspect at the field level before apply without manual
//! reconstruction, so sync never silently overwrites locked, machine-only, or stale-local authoritative state, a
//! conflict never collapses into last-writer-wins or one generic warning, a device action ledger always names its
//! attribution and stays reconstructable, and a sync flow that cannot explain a conflict or a device action
//! degrades honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one sync-conflict-packet object per conflict.** [`resolve_sync_conflict_packet_entry`] refuses to
//!   read as a clean, registry-bound conflict entry unless it names a canonical registry token, a classified
//!   [conflict class][M5SyncConflictClass], a settings-governance role, covers every
//!   [resolution form][M5ConfigSyncResolutionForm] (the canonical object, the accessible summary, and the audit
//!   record), publishes every packet field (field path, local revision, remote revision, keep-local option,
//!   keep-synced option, compare reference, and blocked-state reason), keeps its resolution field-aware, and
//!   preserves local authoritative state before a protected (policy-locked / machine-only / stale-remote)
//!   conflict applies; otherwise it degrades.
//! * **Keep a conflict from silently overwriting local state or hiding its field-level resolution.**
//!   [`conflict_does_not_silently_overwrite`] rejects a conflict entry whose resolution is not field-aware so it
//!   degrades to
//!   [`M5SyncConflictPacketEntryDegradeReason::ConflictSilentlyOverwritesOrHidesFieldResolution`], and a protected
//!   conflict that has not preserved local authoritative state degrades the same way.
//! * **Keep the device action ledger from hiding its attribution or dropping reconstruction.**
//!   [`resolve_device_action_record_entry`] names a classified [device-action class][M5DeviceActionClass],
//!   requires the full actor / action-timestamp / transport-state / policy-state / capability-dependency /
//!   attribution-reference / last-ledger-revision device-action-record object, covers every resolution form, and
//!   degrades to
//!   [`M5DeviceActionRecordEntryDegradeReason::DeviceActionLedgerHidesAttributionOrDropsReconstruction`] when the
//!   record would hide a revoke / forget cause without disclosing its reason or leave a degraded-transport action
//!   without disclosing that local state stays authoritative, so a device action can never read as trustworthy
//!   when it has quietly dropped the reason it ran or the local-authority posture the user still has.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5SettingsGovernanceRole`] role vocabulary
//! and the [`M5SettingsGovernanceConsumerSurface`] consumer-surface taxonomy — so the settings, shell,
//! diagnostics, admin, sync, policy, capability, docs, CLI, and support surfaces can never fork their own
//! conflict or device-action meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_settings_governance_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_setting_sync_conflict_and_device_action_registries,
    seeded_m5_setting_sync_conflict_and_device_action_registries_device_action_preview_narrowed,
    seeded_m5_setting_sync_conflict_and_device_action_registries_sync_conflict_beta_narrowed,
    M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_PACKET_ID,
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
    M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SettingSyncConflictDeviceActionRegistriesPacket`].
pub const M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_setting_sync_conflict_and_device_action_registries";

/// Schema version for M5 sync-conflict / device-action registry records.
pub const M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_SCHEMA_REF: &str =
    "schemas/config/m5-setting-sync-conflict-and-device-action-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_DOC_REF: &str =
    "docs/settings/m5_setting_sync_conflict_and_device_action_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-setting-sync-conflict-and-device-action-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-setting-sync-conflict-and-device-action-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-setting-sync-conflict-and-device-action-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/config/m5-setting-sync-conflict-and-device-action-registries";

/// Repo-relative path of the already-landed device-record schema the device-action registry binds back to, so a
/// pause / resume / revoke / forget / token-rotation action's actor, transport and policy state, capability
/// dependency, attribution, and revision trace to one canonical device-record contract rather than a lane-local
/// invention.
pub const M5_SYNC_DEVICE_RECORD_LANDED_SCHEMA_REF: &str =
    "schemas/settings/sync_device_record.schema.json";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5SettingSyncConflictDeviceActionRegistriesConsumerSurface =
    M5SettingsGovernanceConsumerSurface;

/// One of the three resolution forms every sync-conflict or device-action entry must hold across so its truth
/// keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or written to
/// the audit / support record. Minted by this lane because the frozen matrix names the sync-scope *family* but
/// not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigSyncResolutionForm {
    /// The canonical resolved sync-conflict / device-action object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved conflict discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved conflict inspectable off-renderer.
    AuditRecord,
}

impl M5ConfigSyncResolutionForm {
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

/// Controlled sync-conflict class a sync-conflict-packet entry declares, so the conflict model shares one
/// registry rather than collapsing every divergence into last-writer-wins or one generic warning. Minted by this
/// lane because the frozen matrix carries the configuration families but not the concrete same-key-divergent /
/// policy-locked / missing-capability / machine-only / delete-versus-modify / stale-remote conflict class a
/// conflict classifies against. Every classified class carries its canonical class mode, and the policy-locked,
/// machine-only, and stale-remote classes carry locked, machine-only, or stale-local authoritative state so they
/// must preserve local durable state before the conflict applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyncConflictClass {
    /// The same key diverged between local and remote and needs a field-level keep-local / keep-synced choice.
    SameKeyDivergent,
    /// The remote change targets a policy-locked setting; local locked state stays authoritative.
    PolicyLocked,
    /// The remote change requires a capability the local device is missing.
    MissingCapability,
    /// The remote change targets machine-only state that must never be treated as portable.
    MachineOnly,
    /// One side deleted a setting the other side modified.
    DeleteVersusModify,
    /// The remote is stale relative to local durable state; local stays authoritative.
    StaleRemote,
    /// The conflict class is unclassified, which is disallowed.
    ConflictClassUnclassified,
}

impl M5SyncConflictClass {
    /// Every conflict class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SameKeyDivergent,
        Self::PolicyLocked,
        Self::MissingCapability,
        Self::MachineOnly,
        Self::DeleteVersusModify,
        Self::StaleRemote,
        Self::ConflictClassUnclassified,
    ];

    /// The six canonical conflict classes every claimed M5 conflict classifies against.
    pub const CANONICAL_CLASSES: [Self; 6] = [
        Self::SameKeyDivergent,
        Self::PolicyLocked,
        Self::MissingCapability,
        Self::MachineOnly,
        Self::DeleteVersusModify,
        Self::StaleRemote,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameKeyDivergent => "same_key_divergent",
            Self::PolicyLocked => "policy_locked",
            Self::MissingCapability => "missing_capability",
            Self::MachineOnly => "machine_only",
            Self::DeleteVersusModify => "delete_versus_modify",
            Self::StaleRemote => "stale_remote",
            Self::ConflictClassUnclassified => "conflict_class_unclassified",
        }
    }

    /// Whether the class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ConflictClassUnclassified)
    }

    /// The canonical class mode for this conflict class.
    pub const fn canonical_class_mode(self) -> &'static str {
        match self {
            Self::SameKeyDivergent => "same_key_divergent_conflict",
            Self::PolicyLocked => "policy_locked_conflict",
            Self::MissingCapability => "missing_capability_conflict",
            Self::MachineOnly => "machine_only_conflict",
            Self::DeleteVersusModify => "delete_versus_modify_conflict",
            Self::StaleRemote => "stale_remote_conflict",
            Self::ConflictClassUnclassified => "",
        }
    }

    /// Whether this class carries locked, machine-only, or stale-local authoritative state and so must preserve
    /// local durable state before the conflict applies.
    pub const fn requires_local_authoritative(self) -> bool {
        matches!(
            self,
            Self::PolicyLocked | Self::MachineOnly | Self::StaleRemote
        )
    }
}

/// Controlled device-action class a device-action-record entry must resolve, so a pause / resume / revoke /
/// forget / token-rotation action shares one registry rather than a hand-copied per-record assumption. Minted by
/// this lane, tracking the device-action ledger dispositions the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeviceActionClass {
    /// Sync was paused on this device.
    PauseSync,
    /// Sync was resumed on this device.
    ResumeSync,
    /// A device's sync grant was revoked.
    RevokeDevice,
    /// A device was forgotten from the sync registry.
    ForgetDevice,
    /// A device's sync token was rotated.
    RotateToken,
    /// The device-action class is unclassified, which is disallowed.
    DeviceActionClassUnclassified,
}

impl M5DeviceActionClass {
    /// Every device-action class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PauseSync,
        Self::ResumeSync,
        Self::RevokeDevice,
        Self::ForgetDevice,
        Self::RotateToken,
        Self::DeviceActionClassUnclassified,
    ];

    /// The five canonical device-action classes every device action ledger must stay distinct across.
    pub const CANONICAL_CLASSES: [Self; 5] = [
        Self::PauseSync,
        Self::ResumeSync,
        Self::RevokeDevice,
        Self::ForgetDevice,
        Self::RotateToken,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PauseSync => "pause_sync",
            Self::ResumeSync => "resume_sync",
            Self::RevokeDevice => "revoke_device",
            Self::ForgetDevice => "forget_device",
            Self::RotateToken => "rotate_token",
            Self::DeviceActionClassUnclassified => "device_action_class_unclassified",
        }
    }

    /// Whether the device-action class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::DeviceActionClassUnclassified)
    }
}

/// Controlled render context — which claimed M5 flow renders the registry entry, so a sync-conflict or
/// device-action token's meaning stays stable whether it appears before apply in a sync-session, import-apply,
/// outage-recovery, or device-review flow, or in a support / export form. Minted by this lane, tracking the
/// first-consumer flows the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigSyncSurfaceContext {
    /// The sync-session flow.
    SyncSessionFlow,
    /// The import-apply flow.
    ImportApplyFlow,
    /// The outage-recovery flow (sync transport, encryption, policy, or provider state degraded).
    OutageRecoveryFlow,
    /// The device-review flow.
    DeviceReviewFlow,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5ConfigSyncSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SyncSessionFlow,
        Self::ImportApplyFlow,
        Self::OutageRecoveryFlow,
        Self::DeviceReviewFlow,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer flows the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::SyncSessionFlow,
        Self::ImportApplyFlow,
        Self::OutageRecoveryFlow,
        Self::DeviceReviewFlow,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncSessionFlow => "sync_session_flow",
            Self::ImportApplyFlow => "import_apply_flow",
            Self::OutageRecoveryFlow => "outage_recovery_flow",
            Self::DeviceReviewFlow => "device_review_flow",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a sync-conflict or device-action entry must be able to show, so no conflict class,
/// field path, keep-local / keep-synced surface, device-action-ledger field, or registry fact is left implicit
/// behind a hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigSyncAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The conflict class the entry classifies (sync-conflict entry).
    ConflictClassLabel,
    /// The field path and local / remote revisions the conflict carries (sync-conflict entry).
    FieldPathAndRevisions,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The field-level keep-local option, keep-synced option, compare reference, and blocked-state reason the
    /// entry publishes (sync-conflict entry).
    KeepLocalKeepSyncedAndCompare,
    /// The device-action-ledger fields (actor, action timestamp, transport state, policy state, capability
    /// dependency, attribution) the entry publishes (device-action entry).
    DeviceActionLedgerFields,
    /// The blocked-state / local-authority hint the entry publishes.
    BlockedStateReasonHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved conflict or device action (both entries).
    PlainLanguageMeaning,
}

impl M5ConfigSyncAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::ConflictClassLabel,
        Self::FieldPathAndRevisions,
        Self::ResolutionFormCoverage,
        Self::KeepLocalKeepSyncedAndCompare,
        Self::DeviceActionLedgerFields,
        Self::BlockedStateReasonHint,
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
            Self::ConflictClassLabel => "conflict_class_label",
            Self::FieldPathAndRevisions => "field_path_and_revisions",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::KeepLocalKeepSyncedAndCompare => "keep_local_keep_synced_and_compare",
            Self::DeviceActionLedgerFields => "device_action_ledger_fields",
            Self::BlockedStateReasonHint => "blocked_state_reason_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// conflict, a device action, or a degraded conflict / device-action entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfigSyncNextAction {
    /// Expand the resolved conflict's or device action's plain-language meaning.
    ExpandConflictMeaning,
    /// Inspect the conflict class or device-action ledger the entry resolves.
    InspectClassOrLedger,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5ConfigSyncNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandConflictMeaning,
        Self::InspectClassOrLedger,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandConflictMeaning => "expand_conflict_meaning",
            Self::InspectClassOrLedger => "inspect_class_or_ledger",
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
pub enum M5ConfigSyncExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The settings-governance families covered.
    SettingsGovernanceFamilies,
    /// The sync-conflict classes carried.
    SyncConflictClasses,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The device-action classes carried.
    DeviceActionClasses,
    /// The render / surface context.
    SurfaceContext,
    /// The class modes carried.
    ConflictClassModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ConfigSyncExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::SettingsGovernanceFamilies,
        Self::SyncConflictClasses,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::DeviceActionClasses,
        Self::SurfaceContext,
        Self::ConflictClassModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::SettingsGovernanceFamilies,
        Self::SyncConflictClasses,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::SettingsGovernanceFamilies => "settings_governance_families",
            Self::SyncConflictClasses => "sync_conflict_classes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::DeviceActionClasses => "device_action_classes",
            Self::SurfaceContext => "surface_context",
            Self::ConflictClassModes => "conflict_class_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a sync-conflict-packet entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, silently-overwriting, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyncConflictPacketEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the conflict means.
    SyncConflictTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The conflict class is unclassified (not in the resolved taxonomy).
    ConflictClassUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    SyncConflictNotBoundToRegistry,
    /// The resolved sync-conflict-packet object is incomplete: the field path, local revision, remote revision,
    /// keep-local option, keep-synced option, compare reference, or blocked-state reason is unstated.
    SyncConflictPacketIncomplete,
    /// The resolution is not field-aware (it would collapse into last-writer-wins), or a protected
    /// (policy-locked / machine-only / stale-remote) conflict silently overwrote local authoritative state.
    ConflictSilentlyOverwritesOrHidesFieldResolution,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A protected conflict did not preserve local authoritative state before it applied.
    LocalAuthorityNotPreservedForProtectedConflict,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SyncConflictPacketEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SyncConflictTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::ConflictClassUnclassified,
        Self::SyncConflictNotBoundToRegistry,
        Self::SyncConflictPacketIncomplete,
        Self::ConflictSilentlyOverwritesOrHidesFieldResolution,
        Self::ResolutionFormCoverageIncomplete,
        Self::LocalAuthorityNotPreservedForProtectedConflict,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncConflictTokenUnstated => "sync_conflict_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ConflictClassUnclassified => "conflict_class_unclassified",
            Self::SyncConflictNotBoundToRegistry => "sync_conflict_not_bound_to_registry",
            Self::SyncConflictPacketIncomplete => "sync_conflict_packet_incomplete",
            Self::ConflictSilentlyOverwritesOrHidesFieldResolution => {
                "conflict_silently_overwrites_or_hides_field_resolution"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::LocalAuthorityNotPreservedForProtectedConflict => {
                "local_authority_not_preserved_for_protected_conflict"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ConfigSyncNextAction {
        match self {
            Self::SyncConflictTokenUnstated | Self::SyncConflictNotBoundToRegistry => {
                M5ConfigSyncNextAction::TraceCanonicalRegistry
            }
            Self::ConflictClassUnclassified
            | Self::SyncConflictPacketIncomplete
            | Self::ConflictSilentlyOverwritesOrHidesFieldResolution => {
                M5ConfigSyncNextAction::InspectClassOrLedger
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5ConfigSyncNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::LocalAuthorityNotPreservedForProtectedConflict
            | Self::ProofStale => M5ConfigSyncNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SettingsGovernanceDowngradeTrigger {
        match self {
            Self::SyncConflictTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::ResolutionFormCoverageIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::ConflictClassUnclassified | Self::SyncConflictPacketIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::SyncConflictRuleUnstated
            }
            Self::SyncConflictNotBoundToRegistry => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::ConflictSilentlyOverwritesOrHidesFieldResolution
            | Self::LocalAuthorityNotPreservedForProtectedConflict => {
                M5SettingsGovernanceDowngradeTrigger::SilentlyOverwroteLockedOrMachineOnlyStateDuringSync
            }
            Self::ProofStale => M5SettingsGovernanceDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a device-action-record entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeviceActionRecordEntryDegradeReason {
    /// The canonical registry token name is unstated.
    DeviceActionTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The device-action class is unclassified (not in the resolved taxonomy).
    DeviceActionClassUnclassified,
    /// The device-action record would hide a revoke / forget cause without disclosing its reason, leave a
    /// degraded-transport action without disclosing that local state stays authoritative, or it dropped one of
    /// the required device-action-ledger fields (actor, action timestamp, transport state, policy state,
    /// capability dependency, attribution reference, last ledger revision).
    DeviceActionLedgerHidesAttributionOrDropsReconstruction,
    /// The canonical / accessible / audit resolution-form coverage of the record is incomplete.
    LedgerFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5DeviceActionRecordEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DeviceActionTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::DeviceActionClassUnclassified,
        Self::DeviceActionLedgerHidesAttributionOrDropsReconstruction,
        Self::LedgerFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeviceActionTokenUnstated => "device_action_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DeviceActionClassUnclassified => "device_action_class_unclassified",
            Self::DeviceActionLedgerHidesAttributionOrDropsReconstruction => {
                "device_action_ledger_hides_attribution_or_drops_reconstruction"
            }
            Self::LedgerFormCoverageIncomplete => "ledger_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ConfigSyncNextAction {
        match self {
            Self::DeviceActionTokenUnstated => M5ConfigSyncNextAction::TraceCanonicalRegistry,
            Self::DeviceActionClassUnclassified
            | Self::DeviceActionLedgerHidesAttributionOrDropsReconstruction => {
                M5ConfigSyncNextAction::InspectClassOrLedger
            }
            Self::LedgerFormCoverageIncomplete => {
                M5ConfigSyncNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ConfigSyncNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5SettingsGovernanceDowngradeTrigger {
        match self {
            Self::DeviceActionTokenUnstated => {
                M5SettingsGovernanceDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::DeviceActionClassUnclassified => {
                M5SettingsGovernanceDowngradeTrigger::LifecycleStateUnstated
            }
            Self::DeviceActionLedgerHidesAttributionOrDropsReconstruction => {
                M5SettingsGovernanceDowngradeTrigger::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy
            }
            Self::LedgerFormCoverageIncomplete => {
                M5SettingsGovernanceDowngradeTrigger::ScopeBoundaryDriftedBySurface
            }
            Self::ProofStale => M5SettingsGovernanceDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_sync_conflict_packet_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SyncConflictPacketEntryResolutionInput {
    /// Stable identity of the sync-conflict-registry entry.
    pub entry_id: String,
    /// The stable conflict-target ID this packet binds to (e.g. `settings.acme.editor.font-size@device-42`);
    /// empty means unstated.
    pub conflict_ref: String,
    /// The canonical registry token name (e.g. `conflict.editor.font_size`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5SettingsGovernanceRole,
    /// The conflict class this entry classifies.
    pub conflict_class: M5SyncConflictClass,
    /// The render / surface context.
    pub surface_context: M5ConfigSyncSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5ConfigSyncResolutionForm>,
    /// The published field path the conflict is on; empty means unstated.
    pub field_path: String,
    /// The published local revision; empty means unstated.
    pub local_revision: String,
    /// The published remote revision; empty means unstated.
    pub remote_revision: String,
    /// The published field-level keep-local option; empty means unstated.
    pub keep_local_option: String,
    /// The published field-level keep-synced option; empty means unstated.
    pub keep_synced_option: String,
    /// The published field-level compare reference; empty means unstated.
    pub compare_reference: String,
    /// The published blocked-state reason; empty means unstated.
    pub blocked_state_reason: String,
    /// True when the behavior traces to the sync-conflict registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the resolution is field-aware (keep-local / keep-synced / compare) rather than collapsing into
    /// last-writer-wins (a hard invariant when `false`).
    pub resolution_is_field_aware: bool,
    /// True when this conflict class carries locked, machine-only, or stale-local authoritative state.
    pub requires_local_authoritative: bool,
    /// True when local authoritative state is preserved before a protected conflict applies.
    pub local_authority_preserved: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe sync-conflict-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSyncConflictPacketEntry {
    /// Stable identity of the sync-conflict-registry entry.
    pub entry_id: String,
    /// The stable conflict-target ID this packet binds to.
    pub conflict_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve evidence and disclose cause before applying.
    pub semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: bool,
    /// The conflict-class token named by the entry.
    pub conflict_class: String,
    /// Whether the conflict class is classified into the resolved taxonomy.
    pub conflict_class_is_classified: bool,
    /// The canonical class mode for the entry's conflict class.
    pub canonical_class_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published field path.
    pub field_path: String,
    /// The published local revision.
    pub local_revision: String,
    /// The published remote revision.
    pub remote_revision: String,
    /// The published keep-local option.
    pub keep_local_option: String,
    /// The published keep-synced option.
    pub keep_synced_option: String,
    /// The published compare reference.
    pub compare_reference: String,
    /// The published blocked-state reason.
    pub blocked_state_reason: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved sync-conflict-packet object publishes every required field.
    pub sync_conflict_packet_complete: bool,
    /// Whether the entry traces to the sync-conflict registry.
    pub bound_to_registry: bool,
    /// Whether the resolution is field-aware (never collapses into last-writer-wins).
    pub resolution_is_field_aware: bool,
    /// Whether this conflict requires local authoritative state to be preserved.
    pub requires_local_authoritative: bool,
    /// Whether local authoritative state is preserved before the conflict applies.
    pub local_authority_preserved: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5SyncConflictPacketEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ConfigSyncNextAction,
    /// Whether the conflict resolves to one object across every claimed route (clean entry naming every fact).
    pub conflict_resolves_across_routes: bool,
}

impl M5ResolvedSyncConflictPacketEntry {
    /// Whether this sync-conflict entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_device_action_record_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DeviceActionRecordEntryResolutionInput {
    /// Stable identity of the device-action entry.
    pub entry_id: String,
    /// The stable device-ref this record binds to; empty means unstated.
    pub device_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5SettingsGovernanceRole,
    /// The device-action class this record must resolve.
    pub device_action_class: M5DeviceActionClass,
    /// The render / surface context.
    pub surface_context: M5ConfigSyncSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5ConfigSyncResolutionForm>,
    /// The published actor who performed the action; empty means missing.
    pub actor: String,
    /// The published action timestamp; empty means missing.
    pub action_timestamp: String,
    /// The published transport state; empty means missing.
    pub transport_state: String,
    /// The published policy state; empty means missing.
    pub policy_state: String,
    /// The published capability dependency; empty means missing.
    pub capability_dependency: String,
    /// The published attribution reference; empty means missing.
    pub attribution_reference: String,
    /// The published last ledger revision; empty means missing.
    pub last_ledger_revision: String,
    /// True when the record keeps the attribution visible.
    pub keeps_attribution_visible: bool,
    /// True when the ledger is truthful (never claims a clean resolution over a hidden attribution).
    pub ledger_is_truthful: bool,
    /// True when the action is a revoke or forget (its cause must be disclosed).
    pub revocation_present: bool,
    /// True when a revoke / forget action discloses its reason (never hides the cause).
    pub revocation_reason_disclosed: bool,
    /// True when the action ran under degraded sync transport / encryption / policy / provider state.
    pub degraded_transport_present: bool,
    /// True when a degraded-transport action discloses that local durable state stays authoritative.
    pub local_authority_preserved_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe device-action projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDeviceActionRecordEntry {
    /// Stable identity of the device-action entry.
    pub entry_id: String,
    /// The stable device-ref this record binds to.
    pub device_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve evidence and disclose cause before applying.
    pub semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: bool,
    /// The device-action-class token named by the entry.
    pub device_action_class: String,
    /// Whether the device-action class is classified into the resolved taxonomy.
    pub device_action_class_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published actor.
    pub actor: String,
    /// The published action timestamp.
    pub action_timestamp: String,
    /// The published transport state.
    pub transport_state: String,
    /// The published policy state.
    pub policy_state: String,
    /// The published capability dependency.
    pub capability_dependency: String,
    /// The published attribution reference.
    pub attribution_reference: String,
    /// The published last ledger revision.
    pub last_ledger_revision: String,
    /// Whether the record keeps the attribution visible.
    pub keeps_attribution_visible: bool,
    /// Whether the ledger is truthful.
    pub ledger_is_truthful: bool,
    /// Whether the action is a revoke or forget.
    pub revocation_present: bool,
    /// Whether a revoke / forget action discloses its reason.
    pub revocation_reason_disclosed: bool,
    /// Whether the action ran under degraded transport.
    pub degraded_transport_present: bool,
    /// Whether a degraded-transport action discloses local-authority preservation.
    pub local_authority_preserved_disclosed: bool,
    /// Whether the record stays reconstructable (attribution visible, revoke cause disclosed, local-authority
    /// posture disclosed).
    pub device_action_ledger_stays_reconstructable: bool,
    /// Whether the entry provides the complete device-action-record object (actor, action timestamp, transport
    /// state, policy state, capability dependency, attribution reference, last ledger revision).
    pub provides_complete_device_action_ledger: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5DeviceActionRecordEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ConfigSyncNextAction,
    /// Whether the device action ledger is safe on every claimed route (clean entry naming every fact).
    pub ledger_safe_on_every_route: bool,
}

impl M5ResolvedDeviceActionRecordEntry {
    /// Whether this device-action entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ConfigSyncResolutionError {
    /// The sync-conflict-entry id was empty.
    EmptySyncConflictEntryId,
    /// The device-action-entry id was empty.
    EmptyDeviceActionEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ConfigSyncResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySyncConflictEntryId => "empty_sync_conflict_entry_id",
            Self::EmptyDeviceActionEntryId => "empty_device_action_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ConfigSyncResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 sync-conflict / device-action registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ConfigSyncResolutionError {}

fn form_tokens(forms: &[M5ConfigSyncResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5ConfigSyncResolutionForm]) -> bool {
    let present: BTreeSet<M5ConfigSyncResolutionForm> = forms.iter().copied().collect();
    M5ConfigSyncResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved sync-conflict-packet object publishes every required field: declared conflict class (via
/// a classified class), field path, local revision, remote revision, keep-local option, keep-synced option,
/// compare reference, and blocked-state reason. An unclassified class or any empty field never resolves to a
/// complete object.
#[allow(clippy::too_many_arguments)]
pub fn sync_conflict_packet_is_complete(
    class: M5SyncConflictClass,
    field_path: &str,
    local_revision: &str,
    remote_revision: &str,
    keep_local_option: &str,
    keep_synced_option: &str,
    compare_reference: &str,
    blocked_state_reason: &str,
) -> bool {
    class.is_classified()
        && !field_path.trim().is_empty()
        && !local_revision.trim().is_empty()
        && !remote_revision.trim().is_empty()
        && !keep_local_option.trim().is_empty()
        && !keep_synced_option.trim().is_empty()
        && !compare_reference.trim().is_empty()
        && !blocked_state_reason.trim().is_empty()
}

/// Whether the conflict keeps its resolution field-aware and local-authoritative: the class must be classified,
/// the resolution must be field-aware (keep-local / keep-synced / compare rather than last-writer-wins), and a
/// protected (policy-locked / machine-only / stale-remote) conflict must preserve local authoritative state
/// before it applies. An unclassified class, a collapsed resolution, or an overwritten protected conflict never
/// matches.
pub fn conflict_does_not_silently_overwrite(
    class: M5SyncConflictClass,
    resolution_is_field_aware: bool,
    requires_local_authoritative: bool,
    local_authority_preserved: bool,
) -> bool {
    class.is_classified()
        && resolution_is_field_aware
        && (!requires_local_authoritative || local_authority_preserved)
}

/// Whether a device action ledger stays reconstructable: the class must be classified, the ledger must be
/// truthful, it must keep the attribution visible, any revoke / forget action must disclose its reason rather
/// than hide it, and any degraded-transport action must disclose that local durable state stays authoritative
/// rather than read as an ambiguous overwrite.
pub fn device_action_ledger_stays_reconstructable(
    class: M5DeviceActionClass,
    ledger_is_truthful: bool,
    keeps_attribution_visible: bool,
    revocation_present: bool,
    revocation_reason_disclosed: bool,
    degraded_transport_present: bool,
    local_authority_preserved_disclosed: bool,
) -> bool {
    class.is_classified()
        && ledger_is_truthful
        && keeps_attribution_visible
        && (!revocation_present || revocation_reason_disclosed)
        && (!degraded_transport_present || local_authority_preserved_disclosed)
}

/// Resolves a sync-conflict-registry entry so it stays bound to the sync-conflict registry: the entry names its
/// canonical token, semantic role, and conflict class, covers all three resolution forms, publishes a complete
/// sync-conflict-packet object (field path, local revision, remote revision, keep-local option, keep-synced
/// option, compare reference, blocked-state reason), keeps its resolution field-aware, and preserves local
/// authoritative state before a protected conflict applies.
pub fn resolve_sync_conflict_packet_entry(
    input: M5SyncConflictPacketEntryResolutionInput,
) -> Result<M5ResolvedSyncConflictPacketEntry, M5ConfigSyncResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ConfigSyncResolutionError::EmptySyncConflictEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.conflict_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.field_path)
        || string_is_forbidden(&input.local_revision)
        || string_is_forbidden(&input.remote_revision)
        || string_is_forbidden(&input.keep_local_option)
        || string_is_forbidden(&input.keep_synced_option)
        || string_is_forbidden(&input.compare_reference)
        || string_is_forbidden(&input.blocked_state_reason)
    {
        return Err(M5ConfigSyncResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = sync_conflict_packet_is_complete(
        input.conflict_class,
        &input.field_path,
        &input.local_revision,
        &input.remote_revision,
        &input.keep_local_option,
        &input.keep_synced_option,
        &input.compare_reference,
        &input.blocked_state_reason,
    );
    let resolution_ok = conflict_does_not_silently_overwrite(
        input.conflict_class,
        input.resolution_is_field_aware,
        input.requires_local_authoritative,
        input.local_authority_preserved,
    );
    let local_authority_unpreserved =
        input.requires_local_authoritative && !input.local_authority_preserved;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5SyncConflictPacketEntryDegradeReason::SyncConflictTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SyncConflictPacketEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.conflict_class.is_classified() {
        Some(M5SyncConflictPacketEntryDegradeReason::ConflictClassUnclassified)
    } else if !input.bound_to_registry {
        Some(M5SyncConflictPacketEntryDegradeReason::SyncConflictNotBoundToRegistry)
    } else if !object_complete {
        Some(M5SyncConflictPacketEntryDegradeReason::SyncConflictPacketIncomplete)
    } else if !resolution_ok {
        Some(M5SyncConflictPacketEntryDegradeReason::ConflictSilentlyOverwritesOrHidesFieldResolution)
    } else if !all_forms {
        Some(M5SyncConflictPacketEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if local_authority_unpreserved {
        Some(M5SyncConflictPacketEntryDegradeReason::LocalAuthorityNotPreservedForProtectedConflict)
    } else if !input.proof_fresh {
        Some(M5SyncConflictPacketEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ConfigSyncNextAction::ExpandConflictMeaning,
    };

    Ok(M5ResolvedSyncConflictPacketEntry {
        entry_id: input.entry_id,
        conflict_ref: input.conflict_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: input
            .semantic_role
            .must_preserve_evidence_and_disclose_cause_before_applying(),
        conflict_class: input.conflict_class.as_str().to_owned(),
        conflict_class_is_classified: input.conflict_class.is_classified(),
        canonical_class_mode: input.conflict_class.canonical_class_mode().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        field_path: input.field_path,
        local_revision: input.local_revision,
        remote_revision: input.remote_revision,
        keep_local_option: input.keep_local_option,
        keep_synced_option: input.keep_synced_option,
        compare_reference: input.compare_reference,
        blocked_state_reason: input.blocked_state_reason,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        sync_conflict_packet_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        resolution_is_field_aware: input.resolution_is_field_aware,
        requires_local_authoritative: input.requires_local_authoritative,
        local_authority_preserved: input.local_authority_preserved,
        degrade_reason,
        next_action,
        conflict_resolves_across_routes: degrade_reason.is_none(),
    })
}

/// Resolves a device-action entry so its resolution stays safe: the entry names its canonical token, semantic
/// role, and device-action class, covers all three resolution forms, provides the complete actor / action-
/// timestamp / transport-state / policy-state / capability-dependency / attribution-reference / last-ledger-
/// revision device-action-record object, and degrades honestly when the record would hide a revoke / forget
/// cause without disclosing its reason or leave a degraded-transport action without disclosing that local state
/// stays authoritative.
pub fn resolve_device_action_record_entry(
    input: M5DeviceActionRecordEntryResolutionInput,
) -> Result<M5ResolvedDeviceActionRecordEntry, M5ConfigSyncResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ConfigSyncResolutionError::EmptyDeviceActionEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.device_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.actor)
        || string_is_forbidden(&input.action_timestamp)
        || string_is_forbidden(&input.transport_state)
        || string_is_forbidden(&input.policy_state)
        || string_is_forbidden(&input.capability_dependency)
        || string_is_forbidden(&input.attribution_reference)
        || string_is_forbidden(&input.last_ledger_revision)
    {
        return Err(M5ConfigSyncResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let record_stays_reconstructable = device_action_ledger_stays_reconstructable(
        input.device_action_class,
        input.ledger_is_truthful,
        input.keeps_attribution_visible,
        input.revocation_present,
        input.revocation_reason_disclosed,
        input.degraded_transport_present,
        input.local_authority_preserved_disclosed,
    );
    let provides_record = input.device_action_class.is_classified()
        && !input.actor.trim().is_empty()
        && !input.action_timestamp.trim().is_empty()
        && !input.transport_state.trim().is_empty()
        && !input.policy_state.trim().is_empty()
        && !input.capability_dependency.trim().is_empty()
        && !input.attribution_reference.trim().is_empty()
        && !input.last_ledger_revision.trim().is_empty()
        && record_stays_reconstructable;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5DeviceActionRecordEntryDegradeReason::DeviceActionTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5DeviceActionRecordEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.device_action_class.is_classified() {
        Some(M5DeviceActionRecordEntryDegradeReason::DeviceActionClassUnclassified)
    } else if !provides_record {
        Some(M5DeviceActionRecordEntryDegradeReason::DeviceActionLedgerHidesAttributionOrDropsReconstruction)
    } else if !all_forms {
        Some(M5DeviceActionRecordEntryDegradeReason::LedgerFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5DeviceActionRecordEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ConfigSyncNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedDeviceActionRecordEntry {
        entry_id: input.entry_id,
        device_ref: input.device_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_preserve_evidence_and_disclose_cause_before_applying: input
            .semantic_role
            .must_preserve_evidence_and_disclose_cause_before_applying(),
        device_action_class: input.device_action_class.as_str().to_owned(),
        device_action_class_is_classified: input.device_action_class.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        actor: input.actor,
        action_timestamp: input.action_timestamp,
        transport_state: input.transport_state,
        policy_state: input.policy_state,
        capability_dependency: input.capability_dependency,
        attribution_reference: input.attribution_reference,
        last_ledger_revision: input.last_ledger_revision,
        keeps_attribution_visible: input.keeps_attribution_visible,
        ledger_is_truthful: input.ledger_is_truthful,
        revocation_present: input.revocation_present,
        revocation_reason_disclosed: input.revocation_reason_disclosed,
        degraded_transport_present: input.degraded_transport_present,
        local_authority_preserved_disclosed: input.local_authority_preserved_disclosed,
        device_action_ledger_stays_reconstructable: record_stays_reconstructable,
        provides_complete_device_action_ledger: provides_record,
        degrade_reason,
        next_action,
        ledger_safe_on_every_route: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved sync-conflict and device-action entries it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSyncConflictDeviceActionRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SettingSyncConflictDeviceActionRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5ConfigSyncAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ConfigSyncExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    /// Resolved sync-conflict-registry examples.
    pub sync_conflict_entries: Vec<M5ResolvedSyncConflictPacketEntry>,
    /// Resolved device-action examples.
    pub device_action_entries: Vec<M5ResolvedDeviceActionRecordEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the sync-conflict-packet domain and the
    /// device-record landed schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never silently overwrites locked or machine-only state during sync. MUST be
    /// `false`.
    pub silently_overwrites_locked_or_machine_only_state_during_sync: bool,
    /// Hard invariant: this row never collapses conflict classes into last-writer-wins. MUST be `false`.
    pub collapses_conflict_classes_into_last_writer_wins: bool,
    /// Hard invariant: this row never resolves a conflict without a field-level keep-local or blocked reason.
    /// MUST be `false`.
    pub resolves_a_conflict_without_a_field_level_keep_local_or_blocked_reason: bool,
    /// Hard invariant: this row never loses device action lineage in diagnostics or support. MUST be `false`.
    pub loses_device_action_lineage_in_diagnostics_or_support: bool,
}

impl M5SettingSyncConflictDeviceActionRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ConfigSyncAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ConfigSyncAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ConfigSyncExportField> =
            self.export_fields.iter().copied().collect();
        M5ConfigSyncExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.silently_overwrites_locked_or_machine_only_state_during_sync
            && !self.collapses_conflict_classes_into_last_writer_wins
            && !self.resolves_a_conflict_without_a_field_level_keep_local_or_blocked_reason
            && !self.loses_device_action_lineage_in_diagnostics_or_support
    }

    /// True when a clean sync-conflict entry preserves registry-bound truth: it traces to the registry, keeps a
    /// classified conflict class, publishes a complete conflict packet, keeps its resolution field-aware, covers
    /// all three resolution forms, and preserves local authority for a protected conflict.
    fn conflict_is_honest(ex: &M5ResolvedSyncConflictPacketEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.conflict_class_is_classified
                && ex.sync_conflict_packet_complete
                && ex.resolution_is_field_aware
                && ex.covers_all_resolution_forms
                && (!ex.requires_local_authoritative || ex.local_authority_preserved))
    }

    /// True when a clean device-action entry preserves a safe record: it keeps a classified class, provides the
    /// complete device-action-record object, stays reconstructable, and covers all three resolution forms.
    fn device_action_is_honest(ex: &M5ResolvedDeviceActionRecordEntry) -> bool {
        !ex.is_clean()
            || (ex.device_action_class_is_classified
                && ex.provides_complete_device_action_ledger
                && ex.device_action_ledger_stays_reconstructable
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.sync_conflict_entries
            .iter()
            .all(Self::conflict_is_honest)
            && self
                .device_action_entries
                .iter()
                .all(Self::device_action_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSyncConflictDeviceActionRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Sync-conflict-class tokens (minted by this lane).
    pub sync_conflict_classes: Vec<String>,
    /// Device-action-class tokens (minted by this lane).
    pub device_action_classes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Sync-conflict-entry degrade-reason tokens.
    pub sync_conflict_degrade_reasons: Vec<String>,
    /// Device-action-entry degrade-reason tokens.
    pub device_action_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SettingSyncConflictDeviceActionRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5SettingsGovernanceRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5ConfigSyncResolutionForm::ALL, |v| v.as_str()),
            sync_conflict_classes: tokens(&M5SyncConflictClass::ALL, |v| v.as_str()),
            device_action_classes: tokens(&M5DeviceActionClass::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ConfigSyncSurfaceContext::ALL, |v| v.as_str()),
            sync_conflict_degrade_reasons: tokens(
                &M5SyncConflictPacketEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            device_action_degrade_reasons: tokens(
                &M5DeviceActionRecordEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5ConfigSyncAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ConfigSyncNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ConfigSyncExportField::ALL, |v| v.as_str()),
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
pub struct M5SettingSyncConflictDeviceActionRegistriesGovernanceReview {
    /// The sync-conflict registry names a canonical token, semantic role, and conflict class for every entry.
    pub sync_conflict_registry_names_token_role_and_class: bool,
    /// Every claimed conflict resolves to one conflict packet from the shared registry, not per-entry
    /// reconstruction.
    pub conflict_resolves_to_one_packet_from_shared_registry: bool,
    /// The field path, local / remote revisions, keep-local option, keep-synced option, compare reference, and
    /// blocked-state reason are published for every resolved conflict.
    pub field_path_revisions_keep_local_keep_synced_and_blocked_reason_published: bool,
    /// Conflicts never collapse into last-writer-wins; a protected conflict never silently overwrites local
    /// authoritative state.
    pub conflicts_never_collapse_into_last_writer_wins: bool,
    /// The device-action record keeps the attribution visible and discloses the revoke / forget cause and
    /// local-authority posture.
    pub device_action_record_keeps_attribution_visible_and_discloses_cause: bool,
    /// Local durable state is preserved before any protected (policy-locked / machine-only / stale-remote)
    /// conflict applies.
    pub local_authority_preserved_before_protected_conflict_applies: bool,
    /// Every sync-conflict and device-action entry covers the canonical / accessible / audit resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Sync-conflict and device-action behavior stay bound to the shared registries rather than hand-copied per
    /// conflict.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Sync-session, import-apply, outage-recovery, and device-review flows read a single configuration source.
    pub sync_import_outage_and_device_review_read_single_source: bool,
    /// A collapsed resolution, an incomplete packet, or a hidden device-action ledger is caught by fixtures
    /// before release evidence turns green.
    pub conflict_or_ledger_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSyncConflictDeviceActionRegistriesConsumerProjection {
    /// Sync-session and import-apply flows consume the shared sync-conflict registry.
    pub sync_and_import_consume_shared_registries: bool,
    /// Outage-recovery and device-review flows consume the shared device-action registry.
    pub outage_and_device_review_consume_shared_registries: bool,
    /// Sync and device services consume the shared registries.
    pub sync_and_device_services_consume_shared_registries: bool,
    /// Docs, admin, and CLI export consume the shared registries.
    pub docs_admin_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical sync-conflict-packet and device-record domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical sync-conflict / device-action registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSyncConflictDeviceActionRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSyncConflictDeviceActionRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting settings-governance audit for the lane.
    pub settings_governance_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SettingSyncConflictDeviceActionRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SettingSyncConflictDeviceActionRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SettingSyncConflictDeviceActionRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingSyncConflictDeviceActionRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingSyncConflictDeviceActionRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingSyncConflictDeviceActionRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingSyncConflictDeviceActionRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingSyncConflictDeviceActionRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 sync-conflict and device-action registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingSyncConflictDeviceActionRegistriesPacket {
    /// Record kind; must equal [`M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SettingSyncConflictDeviceActionRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingSyncConflictDeviceActionRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingSyncConflictDeviceActionRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingSyncConflictDeviceActionRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingSyncConflictDeviceActionRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingSyncConflictDeviceActionRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SettingSyncConflictDeviceActionRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SettingSyncConflictDeviceActionRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5SettingSyncConflictDeviceActionRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_RECORD_KIND {
            violations.push(M5SettingSyncConflictDeviceActionRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_SCHEMA_VERSION {
            violations
                .push(M5SettingSyncConflictDeviceActionRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SettingSyncConflictDeviceActionRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations
                .push(M5SettingSyncConflictDeviceActionRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 sync-conflict / device-action registries packet serializes"),
        ) {
            violations
                .push(M5SettingSyncConflictDeviceActionRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 sync-conflict / device-action registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,sync_conflict_entries,device_action_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .sync_conflict_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.device_action_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.sync_conflict_entries.len(),
                row.device_action_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Setting-Sync-Conflict and Device-Action Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Sync-conflict classes: {}\n",
            self.vocabulary_set.sync_conflict_classes.join(", ")
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
                "  - Sync-conflict entries: {} / device-action entries: {}\n",
                row.sync_conflict_entries.len(),
                row.device_action_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry sync-conflict reference table generated from the registry, so docs and sync
    /// runbooks render the same class-mode / field-path / keep-local / keep-synced / blocked-reason /
    /// compare-reference truth the resolvers produced rather than a hand-copied conflict table. Only clean,
    /// registry-bound sync-conflict entries are listed.
    pub fn render_conflict_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| conflict_ref | class_mode | field_path | keep_local | keep_synced | blocked_reason | compare_reference |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.sync_conflict_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.conflict_ref,
                    ex.canonical_class_mode,
                    ex.field_path,
                    ex.keep_local_option,
                    ex.keep_synced_option,
                    ex.blocked_state_reason,
                    ex.compare_reference
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SettingSyncConflictDeviceActionRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SettingSyncConflictDeviceActionRegistriesViolation>),
}

impl fmt::Display for M5SettingSyncConflictDeviceActionRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 sync-conflict / device-action registries export parse failed: {error}"
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
                    "m5 sync-conflict / device-action registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SettingSyncConflictDeviceActionRegistriesArtifactError {}

/// Validation failures emitted by [`M5SettingSyncConflictDeviceActionRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SettingSyncConflictDeviceActionRegistriesViolation {
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
    /// A registry row does not point at both the sync-conflict-packet domain and the device-record landed
    /// schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, last-writer-wins, field-incomplete,
    /// form-incomplete, or a device-action entry missing the complete record object).
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
    /// Sync-conflict-resolution is not proven: clean conflict entries do not cover the canonical conflict classes
    /// or the first sync-session / import-apply / outage-recovery / device-review / support flows, no
    /// packet-incomplete example degrades, or a clean conflict entry published an incomplete packet.
    SyncConflictResolutionNotProven,
    /// Conflict-overwrite-honesty is not proven: no field-collapse example and no unbound example degrade, no
    /// clean field-aware conflict entry is present, or a clean conflict entry collapsed into last-writer-wins or
    /// is unbound.
    ConflictOverwriteHonestyNotProven,
    /// Device-action-ledger-integrity is not proven: clean device-action entries do not cover the canonical
    /// pause / resume / revoke / forget / rotate classes with full resolution-form coverage while providing the
    /// complete record object, no hidden-ledger or form-incomplete example degrades, or a clean device-action
    /// entry is missing the complete record object.
    DeviceActionLedgerIntegrityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SettingSyncConflictDeviceActionRegistriesViolation {
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
            Self::SyncConflictResolutionNotProven => "sync_conflict_resolution_not_proven",
            Self::ConflictOverwriteHonestyNotProven => "conflict_overwrite_honesty_not_proven",
            Self::DeviceActionLedgerIntegrityNotProven => {
                "device_action_ledger_integrity_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_setting_sync_conflict_and_device_action_registries_export() -> Result<
    M5SettingSyncConflictDeviceActionRegistriesPacket,
    M5SettingSyncConflictDeviceActionRegistriesArtifactError,
> {
    let packet: M5SettingSyncConflictDeviceActionRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-setting-sync-conflict-and-device-action-registries-proof/support_export.json"
        )
    ))
    .map_err(M5SettingSyncConflictDeviceActionRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SettingSyncConflictDeviceActionRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SettingSyncConflictDeviceActionRegistriesPacket,
    violations: &mut Vec<M5SettingSyncConflictDeviceActionRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_SCHEMA_REF,
        M5_SETTING_SYNC_CONFLICT_DEVICE_ACTION_REGISTRIES_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF,
        M5_SYNC_DEVICE_RECORD_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5SettingSyncConflictDeviceActionRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SettingSyncConflictDeviceActionRegistriesPacket,
    violations: &mut Vec<M5SettingSyncConflictDeviceActionRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5SettingSyncConflictDeviceActionRegistriesViolation::NoRegistryRows);
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
            violations
                .push(M5SettingSyncConflictDeviceActionRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5SettingSyncConflictDeviceActionRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5SettingSyncConflictDeviceActionRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_SYNC_DEVICE_RECORD_LANDED_SCHEMA_REF)
        {
            violations
                .push(M5SettingSyncConflictDeviceActionRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.sync_conflict_entries.is_empty() || row.device_action_entries.is_empty() {
            violations.push(M5SettingSyncConflictDeviceActionRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5SettingSyncConflictDeviceActionRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations
                .push(M5SettingSyncConflictDeviceActionRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5SettingSyncConflictDeviceActionRegistriesPacket,
    violations: &mut Vec<M5SettingSyncConflictDeviceActionRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.sync_conflict_registry_names_token_role_and_class,
        review.conflict_resolves_to_one_packet_from_shared_registry,
        review.field_path_revisions_keep_local_keep_synced_and_blocked_reason_published,
        review.conflicts_never_collapse_into_last_writer_wins,
        review.device_action_record_keeps_attribution_visible_and_discloses_cause,
        review.local_authority_preserved_before_protected_conflict_applies,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.sync_import_outage_and_device_review_read_single_source,
        review.conflict_or_ledger_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5SettingSyncConflictDeviceActionRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SettingSyncConflictDeviceActionRegistriesPacket,
    violations: &mut Vec<M5SettingSyncConflictDeviceActionRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.sync_and_import_consume_shared_registries,
        projection.outage_and_device_review_consume_shared_registries,
        projection.sync_and_device_services_consume_shared_registries,
        projection.docs_admin_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5SettingSyncConflictDeviceActionRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SettingSyncConflictDeviceActionRegistriesPacket,
    violations: &mut Vec<M5SettingSyncConflictDeviceActionRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations
            .push(M5SettingSyncConflictDeviceActionRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SettingSyncConflictDeviceActionRegistriesPacket,
    violations: &mut Vec<M5SettingSyncConflictDeviceActionRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.settings_governance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations
            .push(M5SettingSyncConflictDeviceActionRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5SettingSyncConflictDeviceActionRegistriesPacket,
    violations: &mut Vec<M5SettingSyncConflictDeviceActionRegistriesViolation>,
) {
    let conflicts = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.sync_conflict_entries.iter())
    };
    let devices = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.device_action_entries.iter())
    };

    // AC1: users can review and resolve conflicts at the field level. Clean conflict entries cover the canonical
    // conflict classes and the first sync-session / import-apply / outage-recovery / device-review / support
    // flows, a packet-incomplete example degrades, and no clean conflict entry published an incomplete packet.
    let clean_classes: BTreeSet<String> = conflicts()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.conflict_class.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = conflicts()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let classes_covered = M5SyncConflictClass::CANONICAL_CLASSES
        .iter()
        .all(|k| clean_classes.contains(k.as_str()));
    let first_surfaces_covered = M5ConfigSyncSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let packet_incomplete_degrades = conflicts().any(|ex| {
        ex.degrade_reason
            == Some(M5SyncConflictPacketEntryDegradeReason::SyncConflictPacketIncomplete)
    });
    let no_clean_incomplete =
        !conflicts().any(|ex| ex.is_clean() && !ex.sync_conflict_packet_complete);
    if !(classes_covered
        && first_surfaces_covered
        && packet_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5SettingSyncConflictDeviceActionRegistriesViolation::SyncConflictResolutionNotProven,
        );
    }

    // AC2: no sync / import route silently overwrites locked, machine-only, or stale-local state. A field-
    // collapse / overwrite example degrades, an unbound example degrades, at least one clean field-aware conflict
    // entry is present, and no clean conflict entry collapsed into last-writer-wins or is unbound.
    let overwrite_degrades = conflicts().any(|ex| {
        ex.degrade_reason
            == Some(
                M5SyncConflictPacketEntryDegradeReason::ConflictSilentlyOverwritesOrHidesFieldResolution,
            )
    });
    let unbound_degrades = conflicts().any(|ex| {
        ex.degrade_reason
            == Some(M5SyncConflictPacketEntryDegradeReason::SyncConflictNotBoundToRegistry)
    });
    let field_aware_clean_conflict =
        conflicts().any(|ex| ex.is_clean() && ex.resolution_is_field_aware);
    let no_clean_unbound = !conflicts().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_collapsed = !conflicts().any(|ex| ex.is_clean() && !ex.resolution_is_field_aware);
    if !(overwrite_degrades
        && unbound_degrades
        && field_aware_clean_conflict
        && no_clean_unbound
        && no_clean_collapsed)
    {
        violations.push(
            M5SettingSyncConflictDeviceActionRegistriesViolation::ConflictOverwriteHonestyNotProven,
        );
    }

    // AC3: device action ledgers stay reconstructable in diagnostics and support. Clean device-action entries
    // cover every canonical pause / resume / revoke / forget / rotate class with full resolution-form coverage
    // while providing the complete record object, a hidden-ledger example degrades, a form-incomplete example
    // degrades, and no clean device-action entry is missing the complete record object.
    let clean_record_classes: BTreeSet<String> = devices()
        .filter(|ex| {
            ex.is_clean()
                && ex.device_action_class_is_classified
                && ex.provides_complete_device_action_ledger
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.device_action_class.clone())
        .collect();
    let record_classes_covered = M5DeviceActionClass::CANONICAL_CLASSES
        .iter()
        .all(|m| clean_record_classes.contains(m.as_str()));
    let hidden_ledger_degrades = devices().any(|ex| {
        ex.degrade_reason
            == Some(
                M5DeviceActionRecordEntryDegradeReason::DeviceActionLedgerHidesAttributionOrDropsReconstruction,
            )
    });
    let form_incomplete_degrades = devices().any(|ex| {
        ex.degrade_reason
            == Some(M5DeviceActionRecordEntryDegradeReason::LedgerFormCoverageIncomplete)
    });
    let no_clean_missing_record =
        !devices().any(|ex| ex.is_clean() && !ex.provides_complete_device_action_ledger);
    if !(record_classes_covered
        && hidden_ledger_degrades
        && form_incomplete_degrades
        && no_clean_missing_record)
    {
        violations.push(
            M5SettingSyncConflictDeviceActionRegistriesViolation::DeviceActionLedgerIntegrityNotProven,
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
    [M5SettingsGovernanceFamily::SyncScope];
