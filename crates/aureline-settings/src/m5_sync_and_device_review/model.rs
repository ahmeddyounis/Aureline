//! Governed sync-and-device review record for M5-added feature families.
//!
//! This module is the sync-lane contract that brings the new M5 feature
//! families — notebooks, data/API, profiler, extension bundles, and companion —
//! into a field-aware sync and device-participation model instead of an opaque
//! last-writer-wins blob. It does not move bytes; it defines the canonical record
//! those families emit so the settings UI, CLI inspect, docs/help, and support
//! exports all answer the same questions for an M5 sync scope: which bundle is in
//! play (with its schema version, capability dependencies, redaction mode, source
//! device/profile, and local/remote revision sets), which fields diverge and how
//! (same-key divergent, policy-locked, missing-capability, machine-only,
//! delete-versus-modify, or stale-remote), which device actions were taken with a
//! durable audit trail, and how the row behaves when sync transport, encryption,
//! or policy degrades.
//!
//! The gate is fail-closed. Local durable state is always authoritative; a record
//! can never claim a remote payload won silently. A field whose remote value would
//! widen trust, extension permissions, AI egress, or a managed entitlement can
//! never be auto-applied — it must require explicit review and may not carry an
//! applied disposition. A device pause/resume/revoke/forget/rotation action can
//! never be recorded without a durable audit reference and the explicit statement
//! that local durable state remains intact. Every degraded-state drill must keep
//! local authoritative and visibly labeled. All of these are build-time
//! invariants, so a record that hides remote takeover, a silent trust widening, a
//! local-state wipe, or a dishonest fallback behind an opaque sync state cannot be
//! constructed.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for M5 sync-and-device review records.
pub const M5_SYNC_AND_DEVICE_REVIEW_RECORD_KIND: &str = "m5_sync_and_device_review_record";

/// Schema version for [`M5SyncAndDeviceReview`] records.
pub const M5_SYNC_AND_DEVICE_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by settings UI, CLI, docs/help, and support.
pub const M5_SYNC_AND_DEVICE_REVIEW_SHARED_CONTRACT_REF: &str =
    "settings:m5_sync_and_device_review:v1";

const MAX_REF_CHARS: usize = 240;
const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Returns true when `reference` is a non-empty canonical object ref.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    !class.is_empty() && !ident.is_empty()
}

/// A new M5 feature family whose settings/artifacts participate in sync.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncScopeFamily {
    /// Notebook execution, kernel, and trust settings.
    Notebooks,
    /// Data and API connection, egress, and credential settings.
    DataApi,
    /// Profiler sampling, capture, and retention settings.
    Profiler,
    /// Extension/bundle acquisition and auto-install settings.
    ExtensionBundles,
    /// Companion device control and remote-surface settings.
    Companion,
}

impl SyncScopeFamily {
    /// Returns the canonical token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebooks => "notebooks",
            Self::DataApi => "data_api",
            Self::Profiler => "profiler",
            Self::ExtensionBundles => "extension_bundles",
            Self::Companion => "companion",
        }
    }

    /// Every M5 feature family the contract requires a scope bundle to cover.
    pub const REQUIRED: [Self; 5] = [
        Self::Notebooks,
        Self::DataApi,
        Self::Profiler,
        Self::ExtensionBundles,
        Self::Companion,
    ];
}

/// How a scope bundle redacts its payload before it leaves the local device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionMode {
    /// No redaction needed; the bundle carries no sensitive fields.
    None,
    /// Secret-bearing fields are stripped to opaque references before sync.
    RedactSecrets,
    /// Machine-local fields are excluded from sync entirely.
    MachineLocalExcluded,
    /// The whole bundle stays local-only and never syncs.
    FullyLocalOnly,
}

impl RedactionMode {
    /// Returns the canonical token for this redaction mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RedactSecrets => "redact_secrets",
            Self::MachineLocalExcluded => "machine_local_excluded",
            Self::FullyLocalOnly => "fully_local_only",
        }
    }
}

/// Field-aware classification for one divergent key in a scope bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictClass {
    /// Local and remote both set the same key to different values.
    SameKeyDivergent,
    /// An admin policy locks the key locally; the remote value cannot win.
    PolicyLocked,
    /// The remote value needs a capability that is absent on this device.
    MissingCapability,
    /// The key is machine-only and is never reconciled across devices.
    MachineOnly,
    /// One side deleted the key while the other modified it.
    DeleteVersusModify,
    /// The remote revision is older than the local common ancestor.
    StaleRemote,
}

impl ConflictClass {
    /// Returns the canonical token for this conflict class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameKeyDivergent => "same_key_divergent",
            Self::PolicyLocked => "policy_locked",
            Self::MissingCapability => "missing_capability",
            Self::MachineOnly => "machine_only",
            Self::DeleteVersusModify => "delete_versus_modify",
            Self::StaleRemote => "stale_remote",
        }
    }

    /// Every conflict class the corpus must demonstrate at least once.
    pub const ALL: [Self; 6] = [
        Self::SameKeyDivergent,
        Self::PolicyLocked,
        Self::MissingCapability,
        Self::MachineOnly,
        Self::DeleteVersusModify,
        Self::StaleRemote,
    ];
}

/// How a field conflict is dispositioned. There is deliberately no
/// last-writer-wins variant: a remote value can only land after review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictDisposition {
    /// The local value is kept; the remote value is recorded but not applied.
    LocalAuthoritativeKept,
    /// The field is held for explicit user review before anything is applied.
    AwaitingFieldReview,
    /// The remote apply is blocked (policy lock, missing capability, trust widening).
    RemoteApplyBlocked,
    /// The remote value was applied only after explicit review confirmed it.
    RemoteAppliedAfterReview,
}

impl ConflictDisposition {
    /// Returns the canonical token for this disposition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAuthoritativeKept => "local_authoritative_kept",
            Self::AwaitingFieldReview => "awaiting_field_review",
            Self::RemoteApplyBlocked => "remote_apply_blocked",
            Self::RemoteAppliedAfterReview => "remote_applied_after_review",
        }
    }

    /// Returns true when this disposition would land a remote value locally.
    pub const fn applies_remote(self) -> bool {
        matches!(self, Self::RemoteAppliedAfterReview)
    }
}

/// The kind of trust a remote field value would widen if silently applied.
///
/// A field carrying any of these can never be auto-applied: it must require
/// explicit review and may not be recorded as applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustWideningClass {
    /// Raises a trust boundary (e.g. trusting untrusted content).
    TrustElevation,
    /// Grants additional extension permissions.
    ExtensionPermission,
    /// Opens additional AI or network egress.
    AiEgress,
    /// Widens a managed entitlement or admin-granted capability.
    ManagedEntitlement,
}

impl TrustWideningClass {
    /// Returns the canonical token for this trust-widening class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustElevation => "trust_elevation",
            Self::ExtensionPermission => "extension_permission",
            Self::AiEgress => "ai_egress",
            Self::ManagedEntitlement => "managed_entitlement",
        }
    }

    /// Every trust-widening class the corpus must demonstrate at least once.
    pub const ALL: [Self; 4] = [
        Self::TrustElevation,
        Self::ExtensionPermission,
        Self::AiEgress,
        Self::ManagedEntitlement,
    ];
}

/// A device action that changes sync participation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceAction {
    /// Temporarily stop syncing on a device, keeping its registration.
    Pause,
    /// Resume syncing on a previously paused device.
    Resume,
    /// Revoke a device's sync credentials and participation.
    Revoke,
    /// Forget a device entirely, removing it from the registry.
    Forget,
    /// Rotate a device's sync keys/credentials in place.
    Rotate,
}

impl DeviceAction {
    /// Returns the canonical token for this device action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pause => "pause",
            Self::Resume => "resume",
            Self::Revoke => "revoke",
            Self::Forget => "forget",
            Self::Rotate => "rotate",
        }
    }

    /// Every device action the record must expose as a control.
    pub const REQUIRED: [Self; 5] = [
        Self::Pause,
        Self::Resume,
        Self::Revoke,
        Self::Forget,
        Self::Rotate,
    ];
}

/// Sync participation state of a device after an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceParticipationState {
    /// Actively participating in sync.
    Active,
    /// Registered but paused.
    Paused,
    /// Credentials revoked; no longer participating.
    Revoked,
    /// Removed from the registry entirely.
    Forgotten,
    /// Mid key/credential rotation.
    Rotating,
}

impl DeviceParticipationState {
    /// Returns the canonical token for this participation state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Revoked => "revoked",
            Self::Forgotten => "forgotten",
            Self::Rotating => "rotating",
        }
    }
}

/// Hardware/role class of a participating device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceClass {
    /// Primary desktop install.
    Desktop,
    /// Laptop install.
    Laptop,
    /// Companion device (phone/tablet) with a remote surface.
    Companion,
    /// Headless or CI install.
    Headless,
    /// Centrally managed/enrolled install.
    Managed,
}

impl DeviceClass {
    /// Returns the canonical token for this device class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Laptop => "laptop",
            Self::Companion => "companion",
            Self::Headless => "headless",
            Self::Managed => "managed",
        }
    }
}

/// Degraded-state drill the record must demonstrate for claimed sync rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillKind {
    /// The device is offline; sync is suspended.
    Offline,
    /// The remote payload is stale and must not overwrite local state.
    StaleRemote,
    /// Applying a remote change is blocked by policy or capability.
    BlockedSyncApply,
    /// End-to-end encryption is unavailable; sync must not transmit secrets.
    E2eeUnavailable,
    /// Sync is unavailable and the device falls back to local-only operation.
    LocalOnlyFallback,
}

impl DrillKind {
    /// Returns the canonical token for this drill kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::StaleRemote => "stale_remote",
            Self::BlockedSyncApply => "blocked_sync_apply",
            Self::E2eeUnavailable => "e2ee_unavailable",
            Self::LocalOnlyFallback => "local_only_fallback",
        }
    }

    /// Every drill the record is required to demonstrate.
    pub const REQUIRED: [Self; 5] = [
        Self::Offline,
        Self::StaleRemote,
        Self::BlockedSyncApply,
        Self::E2eeUnavailable,
        Self::LocalOnlyFallback,
    ];
}

/// Transport/encryption state observed during a drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncTransportState {
    /// Online with end-to-end encryption.
    OnlineEncrypted,
    /// Offline.
    Offline,
    /// Online but the remote payload is stale.
    StaleRemote,
    /// Online but end-to-end encryption is unavailable.
    E2eeUnavailable,
    /// Apply blocked by policy.
    PolicyBlocked,
    /// Operating local-only.
    LocalOnly,
}

impl SyncTransportState {
    /// Returns the canonical token for this transport state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnlineEncrypted => "online_encrypted",
            Self::Offline => "offline",
            Self::StaleRemote => "stale_remote",
            Self::E2eeUnavailable => "e2ee_unavailable",
            Self::PolicyBlocked => "policy_blocked",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Source surface that must render the same sync-and-device truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// Desktop settings UI / sync review pane.
    DesktopSettings,
    /// CLI or headless inspect command.
    CliInspect,
    /// Docs and inline help.
    DocsHelp,
    /// Support bundle / support-center export.
    SupportExport,
}

impl SurfaceClass {
    /// Required surface set for parity.
    pub const REQUIRED: [Self; 4] = [
        Self::DesktopSettings,
        Self::CliInspect,
        Self::DocsHelp,
        Self::SupportExport,
    ];
}

/// Local/remote revision cursors for a scope bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeRevisionSets {
    /// Canonical ref to the local durable revision (always present).
    pub local_revision_ref: String,
    /// Canonical ref to the remote revision, when one is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_revision_ref: Option<String>,
    /// Canonical ref to the last common ancestor, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_common_revision_ref: Option<String>,
}

/// A capability a scope bundle depends on to apply fully.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeCapabilityDependency {
    /// Canonical ref to the capability the bundle depends on.
    pub capability_ref: String,
    /// Whether the capability is present on this device.
    pub present_locally: bool,
    /// Whether the absence narrows how the bundle can apply.
    pub narrows_apply: bool,
}

/// One field-aware conflict within a scope bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldConflict {
    /// Dotted field path within the bundle.
    pub field_path: String,
    /// Field-aware conflict class.
    pub class: ConflictClass,
    /// How the conflict is dispositioned (never last-writer-wins).
    pub disposition: ConflictDisposition,
    /// Canonical ref to the local value snapshot.
    pub local_value_ref: String,
    /// Canonical ref to the remote value snapshot, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_value_ref: Option<String>,
    /// Trust-widening class when the remote value would widen trust.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widens_trust: Option<TrustWideningClass>,
    /// Whether the field requires explicit review before any apply.
    pub requires_explicit_review: bool,
    /// Human-readable explanation of the divergence and disposition.
    pub detail: String,
}

impl FieldConflict {
    /// Returns true when this conflict blocks clean sync for its bundle.
    pub fn is_blocking(&self) -> bool {
        self.widens_trust.is_some()
            || self.disposition == ConflictDisposition::RemoteApplyBlocked
            || self.class == ConflictClass::PolicyLocked
    }
}

/// A schema-backed sync scope bundle for one M5 feature family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncScopeBundle {
    /// Stable bundle id.
    pub bundle_id: String,
    /// Feature family this bundle belongs to.
    pub family: SyncScopeFamily,
    /// Human-readable title.
    pub title: String,
    /// Bundle payload schema version (>= 1).
    pub bundle_schema_version: u32,
    /// Redaction mode applied before the bundle leaves the device.
    pub redaction_mode: RedactionMode,
    /// Canonical ref to the source device that produced the bundle.
    pub source_device_ref: String,
    /// Canonical ref to the source profile that produced the bundle.
    pub source_profile_ref: String,
    /// Local/remote revision cursors.
    pub revisions: ScopeRevisionSets,
    /// Capability dependencies for the bundle.
    pub capability_dependencies: Vec<ScopeCapabilityDependency>,
    /// Whether local durable state is authoritative (always true).
    pub local_authoritative: bool,
    /// Whether the remote payload is currently in clean sync with local.
    pub remote_synced: bool,
    /// Field-aware conflicts for this bundle.
    pub conflicts: Vec<FieldConflict>,
}

impl SyncScopeBundle {
    /// Derives the sync trust state for this bundle from its inputs.
    pub fn bundle_trust(&self) -> BundleSyncTrust {
        if self.conflicts.iter().any(FieldConflict::is_blocking) {
            return BundleSyncTrust::ReviewBlocked;
        }
        if !self.remote_synced || !self.conflicts.is_empty() {
            return BundleSyncTrust::LocalAuthoritative;
        }
        BundleSyncTrust::Synced
    }
}

/// A durable audit record for a device action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceActionRecord {
    /// Canonical ref to the device the action targets.
    pub device_ref: String,
    /// Device class.
    pub device_class: DeviceClass,
    /// The action taken.
    pub action: DeviceAction,
    /// Canonical ref to the durable audit record.
    pub audit_ref: String,
    /// Canonical ref to the actor that took the action.
    pub actor_ref: String,
    /// Participation state after the action.
    pub participation_after: DeviceParticipationState,
    /// Whether local durable state remains intact (always true).
    pub local_state_intact: bool,
    /// Human-readable statement of what the action did and what stays local.
    pub detail: String,
}

/// A degraded-state drill kept honest about local-only fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncDrill {
    /// Drill kind.
    pub kind: DrillKind,
    /// Transport/encryption state observed during the drill.
    pub transport_state: SyncTransportState,
    /// Whether local durable state stays authoritative (always true).
    pub local_authoritative: bool,
    /// Whether the local-only/degraded state is visibly labeled (always true).
    pub local_state_labeled: bool,
    /// The observable signal the drill expects.
    pub expected_signal: String,
    /// The recovery path back to clean sync.
    pub recovery_path: String,
}

/// Source surface parity row for sync-and-device truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTruthRow {
    /// Surface class.
    pub surface_class: SurfaceClass,
    /// Whether the surface consumes this shared record.
    pub consumes_shared_record: bool,
    /// Whether the surface shows the scope bundles.
    pub shows_scope_bundles: bool,
    /// Whether the surface shows the field-aware conflicts.
    pub shows_field_conflicts: bool,
    /// Whether the surface shows the device actions.
    pub shows_device_actions: bool,
    /// Whether the surface shows the local-only fallback drills.
    pub shows_local_only_fallback: bool,
}

/// Derived sync trust state for one scope bundle.
///
/// Ordered from [`Self::Synced`] (best) to [`Self::ReviewBlocked`] (weakest);
/// the record publishes the weakest bundle trust as its effective ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleSyncTrust {
    /// Local and remote are in clean sync; no conflicts.
    Synced,
    /// Local durable state is authoritative; sync degraded or a field pends review.
    LocalAuthoritative,
    /// A trust-widening, policy-locked, or blocked field needs review before apply.
    ReviewBlocked,
}

impl BundleSyncTrust {
    /// Returns the canonical token for this trust state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Synced => "synced",
            Self::LocalAuthoritative => "local_authoritative",
            Self::ReviewBlocked => "review_blocked",
        }
    }

    /// Trust rank where `0` is synced and higher values are weaker.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Synced => 0,
            Self::LocalAuthoritative => 1,
            Self::ReviewBlocked => 2,
        }
    }

    /// Returns true when this bundle is in clean sync parity.
    pub const fn is_synced(self) -> bool {
        matches!(self, Self::Synced)
    }
}

/// Derived pillar verdicts for the sync-and-device contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncReviewPillars {
    /// Every required M5 family has a scope bundle.
    pub families_complete: bool,
    /// Every bundle is schema-typed with revision sets and a redaction mode.
    pub scope_bundles_typed: bool,
    /// Every conflict is field-aware (classed, pathed, never last-writer-wins).
    pub conflicts_field_aware: bool,
    /// Every device action carries a durable audit record and keeps local intact.
    pub device_actions_audited: bool,
    /// Every drill keeps local authoritative and visibly labeled.
    pub local_fallback_honest: bool,
    /// No trust-widening field is silently applied without review.
    pub trust_widening_gated: bool,
    /// All required surfaces render the same record.
    pub surface_truth_complete: bool,
}

/// Reason a record is narrowed below a fully-synced claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// One or more required M5 families have no scope bundle.
    FamiliesIncomplete,
    /// A scope bundle is not schema-typed with revisions and a redaction mode.
    ScopeBundleUntyped,
    /// A conflict is opaque rather than field-aware.
    ConflictNotFieldAware,
    /// A device action lacks a durable audit record or wipes local state.
    DeviceActionUnaudited,
    /// A drill loses local authority or hides the degraded state.
    LocalFallbackDishonest,
    /// A trust-widening field is applied without explicit review.
    TrustWideningUngated,
    /// At least one bundle resolves below clean sync.
    BundleTrustBelowSynced,
    /// One or more surfaces omit required sync-and-device truth.
    SurfaceTruthIncomplete,
}

/// Public claim class derived from the sync-and-device evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncReviewClaim {
    /// Every bundle is in clean sync parity and every pillar holds.
    FullySynced,
    /// Resolution is sound but at least one bundle is local-authoritative or blocked.
    NarrowedLocalAuthoritative,
    /// A structural pillar failed; the record is not safely usable as-is.
    Unsupported,
}

/// Derived trust verdict for the whole record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncReviewQualification {
    /// Derived claim class.
    pub claim_class: SyncReviewClaim,
    /// Weakest bundle trust across all bundles.
    pub effective_trust_ceiling: BundleSyncTrust,
    /// Whether the record qualifies for a fully-synced claim.
    pub qualifies_fully_synced: bool,
    /// Named narrowing reasons.
    pub narrowing_reasons: Vec<NarrowingReason>,
}

/// Input used to build a [`M5SyncAndDeviceReview`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SyncAndDeviceReviewInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// Scope bundles for M5 feature families.
    pub scope_bundles: Vec<SyncScopeBundle>,
    /// Device action audit records.
    pub device_actions: Vec<DeviceActionRecord>,
    /// Degraded-state drills.
    pub drills: Vec<SyncDrill>,
    /// Surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
}

/// Canonical sync-and-device review record for M5-added feature families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SyncAndDeviceReview {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// Scope bundles for M5 feature families.
    pub scope_bundles: Vec<SyncScopeBundle>,
    /// Device action audit records.
    pub device_actions: Vec<DeviceActionRecord>,
    /// Degraded-state drills.
    pub drills: Vec<SyncDrill>,
    /// Surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
    /// Families covered by the scope bundles.
    pub family_coverage: Vec<SyncScopeFamily>,
    /// Conflict classes surfaced by the bundles.
    pub conflict_class_coverage: Vec<ConflictClass>,
    /// Redaction modes covered by the bundles.
    pub redaction_mode_coverage: Vec<RedactionMode>,
    /// Trust-widening classes surfaced as gated fields.
    pub trust_widening_coverage: Vec<TrustWideningClass>,
    /// Device actions exposed by the record.
    pub device_action_coverage: Vec<DeviceAction>,
    /// Drill kinds demonstrated by the record.
    pub drill_coverage: Vec<DrillKind>,
    /// Derived pillar verdicts.
    pub pillars: SyncReviewPillars,
    /// Derived trust qualification.
    pub trust_qualification: SyncReviewQualification,
}

/// Reasons a sync-and-device review cannot be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// No scope bundles were supplied.
    NoScopeBundles,
    /// A required M5 family has no scope bundle.
    MissingFamily {
        /// The family with no bundle.
        family: SyncScopeFamily,
    },
    /// A bundle id is used by more than one bundle.
    DuplicateBundle {
        /// The duplicated bundle id.
        bundle_id: String,
    },
    /// A scope bundle has an invalid (zero) payload schema version.
    BundleSchemaVersionZero {
        /// The under-specified bundle id.
        bundle_id: String,
    },
    /// A canonical ref field is invalid.
    NonCanonicalRef {
        /// The field carrying the invalid ref.
        field: &'static str,
        /// The offending value.
        value: String,
    },
    /// A scope bundle claims remote authority over local durable state.
    LocalStateNotAuthoritative {
        /// The dishonest bundle id.
        bundle_id: String,
    },
    /// A field conflict has an empty field path or detail.
    ConflictNotFieldAware {
        /// The bundle carrying the opaque conflict.
        bundle_id: String,
    },
    /// A trust-widening field is applied or does not require review.
    TrustWideningSilentlyApplied {
        /// The bundle id.
        bundle_id: String,
        /// The offending field path.
        field_path: String,
    },
    /// A policy-locked field was applied from the remote side.
    PolicyLockedFieldApplied {
        /// The bundle id.
        bundle_id: String,
        /// The offending field path.
        field_path: String,
    },
    /// A device action lacks a durable audit record.
    DeviceActionWithoutAudit {
        /// The device ref carrying the unaudited action.
        device_ref: String,
    },
    /// A device action claims to wipe local durable state.
    DeviceActionWipesLocalState {
        /// The device ref carrying the dishonest action.
        device_ref: String,
    },
    /// A required device action is missing from the record.
    MissingDeviceAction {
        /// The missing action.
        action: DeviceAction,
    },
    /// A drill loses local authority.
    DrillNotLocalAuthoritative {
        /// The offending drill kind.
        kind: DrillKind,
    },
    /// A drill hides the degraded/local-only state.
    DrillNotLabeled {
        /// The offending drill kind.
        kind: DrillKind,
    },
    /// A required drill is missing from the record.
    MissingDrill {
        /// The missing drill kind.
        kind: DrillKind,
    },
    /// A required surface row is missing.
    MissingSurface {
        /// The missing surface.
        surface: SurfaceClass,
    },
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoScopeBundles => write!(f, "at least one scope bundle is required"),
            Self::MissingFamily { family } => {
                write!(f, "missing M5 family `{}`", family.as_str())
            }
            Self::DuplicateBundle { bundle_id } => {
                write!(f, "duplicated bundle id `{bundle_id}`")
            }
            Self::BundleSchemaVersionZero { bundle_id } => {
                write!(f, "bundle `{bundle_id}` needs a non-zero schema version")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field `{field}` must be a canonical ref, got {value:?}")
            }
            Self::LocalStateNotAuthoritative { bundle_id } => write!(
                f,
                "bundle `{bundle_id}` must keep local durable state authoritative"
            ),
            Self::ConflictNotFieldAware { bundle_id } => write!(
                f,
                "bundle `{bundle_id}` has an opaque conflict without a field path or detail"
            ),
            Self::TrustWideningSilentlyApplied {
                bundle_id,
                field_path,
            } => write!(
                f,
                "bundle `{bundle_id}` field `{field_path}` would widen trust and cannot be applied without review"
            ),
            Self::PolicyLockedFieldApplied {
                bundle_id,
                field_path,
            } => write!(
                f,
                "bundle `{bundle_id}` field `{field_path}` is policy-locked and cannot apply a remote value"
            ),
            Self::DeviceActionWithoutAudit { device_ref } => write!(
                f,
                "device action on `{device_ref}` requires a durable audit record"
            ),
            Self::DeviceActionWipesLocalState { device_ref } => write!(
                f,
                "device action on `{device_ref}` must keep local durable state intact"
            ),
            Self::MissingDeviceAction { action } => {
                write!(f, "missing device action `{}`", action.as_str())
            }
            Self::DrillNotLocalAuthoritative { kind } => write!(
                f,
                "drill `{}` must keep local durable state authoritative",
                kind.as_str()
            ),
            Self::DrillNotLabeled { kind } => write!(
                f,
                "drill `{}` must keep the degraded state visibly labeled",
                kind.as_str()
            ),
            Self::MissingDrill { kind } => write!(f, "missing drill `{}`", kind.as_str()),
            Self::MissingSurface { surface } => write!(f, "missing surface `{surface:?}`"),
        }
    }
}

impl std::error::Error for BuildError {}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_owned(),
        })
    }
}

impl M5SyncAndDeviceReview {
    /// Builds a derived sync-and-device review from raw bundles, actions, and drills.
    ///
    /// Returns a [`BuildError`] when a structural invariant or a fail-closed
    /// guardrail is violated, so a record that hides remote takeover, a silent
    /// trust widening, a local-state wipe, or a dishonest local-only fallback
    /// behind an opaque sync state cannot be constructed.
    pub fn build(mut input: M5SyncAndDeviceReviewInput) -> Result<Self, BuildError> {
        if input.scope_bundles.is_empty() {
            return Err(BuildError::NoScopeBundles);
        }

        let mut seen_bundles = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for bundle in &input.scope_bundles {
            if !seen_bundles.insert(bundle.bundle_id.clone()) {
                return Err(BuildError::DuplicateBundle {
                    bundle_id: bundle.bundle_id.clone(),
                });
            }
            seen_families.insert(bundle.family);

            if bundle.bundle_schema_version == 0 {
                return Err(BuildError::BundleSchemaVersionZero {
                    bundle_id: bundle.bundle_id.clone(),
                });
            }
            // Local durable state is always authoritative; a bundle can never
            // claim a remote payload outranks the local source of truth.
            if !bundle.local_authoritative {
                return Err(BuildError::LocalStateNotAuthoritative {
                    bundle_id: bundle.bundle_id.clone(),
                });
            }

            require_ref("scope_bundles.source_device_ref", &bundle.source_device_ref)?;
            require_ref(
                "scope_bundles.source_profile_ref",
                &bundle.source_profile_ref,
            )?;
            require_ref(
                "scope_bundles.revisions.local_revision_ref",
                &bundle.revisions.local_revision_ref,
            )?;
            if let Some(remote) = &bundle.revisions.remote_revision_ref {
                require_ref("scope_bundles.revisions.remote_revision_ref", remote)?;
            }
            if let Some(ancestor) = &bundle.revisions.last_common_revision_ref {
                require_ref("scope_bundles.revisions.last_common_revision_ref", ancestor)?;
            }
            for dependency in &bundle.capability_dependencies {
                require_ref(
                    "scope_bundles.capability_dependencies.capability_ref",
                    &dependency.capability_ref,
                )?;
            }

            for conflict in &bundle.conflicts {
                if conflict.field_path.trim().is_empty() || conflict.detail.trim().is_empty() {
                    return Err(BuildError::ConflictNotFieldAware {
                        bundle_id: bundle.bundle_id.clone(),
                    });
                }
                require_ref(
                    "scope_bundles.conflicts.local_value_ref",
                    &conflict.local_value_ref,
                )?;
                if let Some(remote) = &conflict.remote_value_ref {
                    require_ref("scope_bundles.conflicts.remote_value_ref", remote)?;
                }
                // A trust-widening field can never be applied silently: it must
                // require explicit review and may not carry an applied disposition.
                if conflict.widens_trust.is_some()
                    && (!conflict.requires_explicit_review || conflict.disposition.applies_remote())
                {
                    return Err(BuildError::TrustWideningSilentlyApplied {
                        bundle_id: bundle.bundle_id.clone(),
                        field_path: conflict.field_path.clone(),
                    });
                }
                // A policy-locked field can never apply a remote value.
                if conflict.class == ConflictClass::PolicyLocked
                    && conflict.disposition.applies_remote()
                {
                    return Err(BuildError::PolicyLockedFieldApplied {
                        bundle_id: bundle.bundle_id.clone(),
                        field_path: conflict.field_path.clone(),
                    });
                }
            }
        }

        for family in SyncScopeFamily::REQUIRED {
            if !seen_families.contains(&family) {
                return Err(BuildError::MissingFamily { family });
            }
        }

        // Device actions: each is audited and keeps local state intact; every
        // required action class is exposed.
        let mut seen_actions = BTreeSet::new();
        for record in &input.device_actions {
            require_ref("device_actions.device_ref", &record.device_ref)?;
            require_ref("device_actions.actor_ref", &record.actor_ref)?;
            if !is_canonical_object_ref(&record.audit_ref) {
                return Err(BuildError::DeviceActionWithoutAudit {
                    device_ref: record.device_ref.clone(),
                });
            }
            if !record.local_state_intact {
                return Err(BuildError::DeviceActionWipesLocalState {
                    device_ref: record.device_ref.clone(),
                });
            }
            seen_actions.insert(record.action);
        }
        for action in DeviceAction::REQUIRED {
            if !seen_actions.contains(&action) {
                return Err(BuildError::MissingDeviceAction { action });
            }
        }

        // Drills: each keeps local authoritative and labeled; every required
        // drill kind is demonstrated.
        let mut seen_drills = BTreeSet::new();
        for drill in &input.drills {
            if !drill.local_authoritative {
                return Err(BuildError::DrillNotLocalAuthoritative { kind: drill.kind });
            }
            if !drill.local_state_labeled {
                return Err(BuildError::DrillNotLabeled { kind: drill.kind });
            }
            seen_drills.insert(drill.kind);
        }
        for kind in DrillKind::REQUIRED {
            if !seen_drills.contains(&kind) {
                return Err(BuildError::MissingDrill { kind });
            }
        }

        let present_surfaces: BTreeSet<SurfaceClass> = input
            .surface_truth
            .iter()
            .map(|row| row.surface_class)
            .collect();
        for surface in SurfaceClass::REQUIRED {
            if !present_surfaces.contains(&surface) {
                return Err(BuildError::MissingSurface { surface });
            }
        }

        input
            .scope_bundles
            .sort_by(|a, b| a.bundle_id.cmp(&b.bundle_id));
        input.device_actions.sort_by_key(|record| record.action);
        input.drills.sort_by_key(|drill| drill.kind);
        input.surface_truth.sort_by_key(|row| row.surface_class);

        let family_coverage = collect_sorted(input.scope_bundles.iter().map(|b| b.family));
        let conflict_class_coverage = collect_sorted(
            input
                .scope_bundles
                .iter()
                .flat_map(|b| b.conflicts.iter().map(|c| c.class)),
        );
        let redaction_mode_coverage =
            collect_sorted(input.scope_bundles.iter().map(|b| b.redaction_mode));
        let trust_widening_coverage = collect_sorted(
            input
                .scope_bundles
                .iter()
                .flat_map(|b| b.conflicts.iter().filter_map(|c| c.widens_trust)),
        );
        let device_action_coverage = collect_sorted(input.device_actions.iter().map(|r| r.action));
        let drill_coverage = collect_sorted(input.drills.iter().map(|d| d.kind));

        let families_complete = SyncScopeFamily::REQUIRED
            .iter()
            .all(|family| seen_families.contains(family));

        let scope_bundles_typed = input.scope_bundles.iter().all(|bundle| {
            bundle.bundle_schema_version >= 1
                && !bundle.title.trim().is_empty()
                && is_canonical_object_ref(&bundle.revisions.local_revision_ref)
        });

        let conflicts_field_aware = input.scope_bundles.iter().all(|bundle| {
            bundle.conflicts.iter().all(|conflict| {
                !conflict.field_path.trim().is_empty() && !conflict.detail.trim().is_empty()
            })
        });

        let device_actions_audited = input
            .device_actions
            .iter()
            .all(|record| is_canonical_object_ref(&record.audit_ref) && record.local_state_intact);

        let local_fallback_honest = input
            .drills
            .iter()
            .all(|drill| drill.local_authoritative && drill.local_state_labeled)
            && input
                .scope_bundles
                .iter()
                .all(|bundle| bundle.local_authoritative);

        let trust_widening_gated = input.scope_bundles.iter().all(|bundle| {
            bundle.conflicts.iter().all(|conflict| {
                conflict.widens_trust.is_none()
                    || (conflict.requires_explicit_review && !conflict.disposition.applies_remote())
            })
        });

        let surface_truth_complete = input.surface_truth.iter().all(|row| {
            row.consumes_shared_record
                && row.shows_scope_bundles
                && row.shows_field_conflicts
                && row.shows_device_actions
                && row.shows_local_only_fallback
        });

        let effective_trust_ceiling = input
            .scope_bundles
            .iter()
            .map(SyncScopeBundle::bundle_trust)
            .max_by_key(|trust| trust.rank())
            .unwrap_or(BundleSyncTrust::Synced);

        let pillars = SyncReviewPillars {
            families_complete,
            scope_bundles_typed,
            conflicts_field_aware,
            device_actions_audited,
            local_fallback_honest,
            trust_widening_gated,
            surface_truth_complete,
        };

        let mut narrowing_reasons = Vec::new();
        if !pillars.families_complete {
            narrowing_reasons.push(NarrowingReason::FamiliesIncomplete);
        }
        if !pillars.scope_bundles_typed {
            narrowing_reasons.push(NarrowingReason::ScopeBundleUntyped);
        }
        if !pillars.conflicts_field_aware {
            narrowing_reasons.push(NarrowingReason::ConflictNotFieldAware);
        }
        if !pillars.device_actions_audited {
            narrowing_reasons.push(NarrowingReason::DeviceActionUnaudited);
        }
        if !pillars.local_fallback_honest {
            narrowing_reasons.push(NarrowingReason::LocalFallbackDishonest);
        }
        if !pillars.trust_widening_gated {
            narrowing_reasons.push(NarrowingReason::TrustWideningUngated);
        }
        if !effective_trust_ceiling.is_synced() {
            narrowing_reasons.push(NarrowingReason::BundleTrustBelowSynced);
        }
        if !pillars.surface_truth_complete {
            narrowing_reasons.push(NarrowingReason::SurfaceTruthIncomplete);
        }

        let structural_ok = pillars.families_complete
            && pillars.scope_bundles_typed
            && pillars.conflicts_field_aware
            && pillars.device_actions_audited
            && pillars.local_fallback_honest
            && pillars.trust_widening_gated
            && pillars.surface_truth_complete;

        let qualifies_fully_synced = structural_ok && effective_trust_ceiling.is_synced();

        let claim_class = if !structural_ok {
            SyncReviewClaim::Unsupported
        } else if qualifies_fully_synced {
            SyncReviewClaim::FullySynced
        } else {
            SyncReviewClaim::NarrowedLocalAuthoritative
        };

        let trust_qualification = SyncReviewQualification {
            claim_class,
            effective_trust_ceiling,
            qualifies_fully_synced,
            narrowing_reasons,
        };

        Ok(Self {
            record_kind: M5_SYNC_AND_DEVICE_REVIEW_RECORD_KIND.to_owned(),
            schema_version: M5_SYNC_AND_DEVICE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: M5_SYNC_AND_DEVICE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            record_id: input.record_id,
            as_of: input.as_of,
            summary: input.summary,
            scope_bundles: input.scope_bundles,
            device_actions: input.device_actions,
            drills: input.drills,
            surface_truth: input.surface_truth,
            family_coverage,
            conflict_class_coverage,
            redaction_mode_coverage,
            trust_widening_coverage,
            device_action_coverage,
            drill_coverage,
            pillars,
            trust_qualification,
        })
    }

    /// Renders a compact, export-safe support summary from the shared record.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("record_id: {}", self.record_id),
            format!("claim: {:?}", self.trust_qualification.claim_class),
            format!(
                "trust_ceiling: {}",
                self.trust_qualification.effective_trust_ceiling.as_str()
            ),
            format!("scope_bundles: {}", self.scope_bundles.len()),
            format!("families: {}", self.family_coverage.len()),
            format!("conflict_classes: {}", self.conflict_class_coverage.len()),
            format!(
                "trust_widening_fields: {}",
                self.trust_widening_coverage.len()
            ),
            format!("device_actions: {}", self.device_action_coverage.len()),
            format!("drills: {}", self.drill_coverage.len()),
        ]
    }
}

fn collect_sorted<T: Ord>(values: impl Iterator<Item = T>) -> Vec<T> {
    values.collect::<BTreeSet<_>>().into_iter().collect()
}
