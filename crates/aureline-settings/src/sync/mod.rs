//! Beta-grade settings sync and device-registry conflict review.
//!
//! The sync module is the page-level surface above the
//! [`crate::inspector::conflict`] alpha records. It pulls the
//! resolver, the alpha conflict packet, and caller-supplied device
//! registry rows into one projection so settings UI, CLI/headless
//! inspectors, and support exports can render the same conflict
//! review without re-deriving sync truth from raw bundles.
//!
//! The projection adds the things a beta-grade conflict review
//! expects on top of the alpha contract:
//!
//! - a [`SyncStateClass`] that distinguishes
//!   `local_authoritative`, `synced`, `imported`, `stale`, and
//!   `disabled_device` rows instead of letting surfaces invent
//!   their own status names;
//! - a [`SyncBetaDeviceRecord`] block that carries device class,
//!   os family, identity mode, participation state, lineage
//!   cursor, and revocation reason without ever quoting hostnames,
//!   serials, or IP addresses;
//! - a [`LastWriterBreadcrumb`] that names which device,
//!   revision, scope, and actor produced the local effective value
//!   so a reviewer can pivot from a row to the winning lineage;
//! - a [`RollbackDecision`] that names when a conflict must route
//!   through a rollback checkpoint or retry path before any apply,
//!   instead of letting the resolution path imply rollback posture;
//! - a support-export wrapper
//!   ([`SyncConflictReviewBetaSupportExport`]) that quotes the
//!   beta page plus the alpha conflict packets it was built from
//!   so support reviewers and the user see the same lineage truth.
//!
//! The same projection feeds the headless inspector
//! (`aureline_settings_inspect sync-beta-review`) and the
//! support-export wrapper
//! (`aureline_settings_inspect sync-beta-support-export`).

use serde::{Deserialize, Serialize};

use crate::inspector::conflict::{
    inspect_sync_conflict, project_sync_conflict_review_surface, SyncConflictClass,
    SyncConflictDevice, SyncConflictPacket, SyncConflictResolutionPath,
    SyncConflictResolutionState, SyncConflictReviewRequest, SyncConflictReviewSurface,
};
use crate::inspector::{inspect_setting, SettingsInspectError, SettingsInspectionContext};
use crate::resolver::EffectiveSettingsResolver;
use crate::schema::{PreviewClass, SettingScope, SettingValue};

/// Schema version for the sync beta projection.
pub const SETTINGS_SYNC_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by settings UI, shell inspectors, CLI, and support export.
pub const SETTINGS_SYNC_BETA_SHARED_CONTRACT_REF: &str = "settings:sync_beta:v1";

/// High-level sync state for one row in a settings sync review.
///
/// The vocabulary mirrors the row classes defined in the M3 sync
/// contract: surfaces consume these tokens verbatim and MUST NOT
/// invent their own status names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncStateClass {
    /// Local value is authoritative; no live sync or import is in effect.
    LocalAuthoritative,
    /// Local value matches a synced peer and the bundle is fresh.
    Synced,
    /// Value was carried from a user-initiated profile export / import.
    Imported,
    /// The arriving bundle is stale (older epoch, lapsed freshness budget).
    Stale,
    /// The producer device is paused, revoked, or forgotten.
    DisabledDevice,
}

impl SyncStateClass {
    /// Returns the stable settings-sync state-class token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAuthoritative => "local_authoritative",
            Self::Synced => "synced",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::DisabledDevice => "disabled_device",
        }
    }
}

/// Frozen device participation states from the M1 device-registry seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceParticipationState {
    /// Normal participating state.
    Active,
    /// User- or admin-initiated freeze; lineage retained, no traffic.
    Paused,
    /// Durable refusal of all sync traffic.
    Revoked,
    /// Terminal state after the revoke retention window.
    Forgotten,
}

impl DeviceParticipationState {
    /// Returns the stable participation-state token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Revoked => "revoked",
            Self::Forgotten => "forgotten",
        }
    }

    /// Returns true when the device cannot currently carry sync traffic.
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Paused | Self::Revoked | Self::Forgotten)
    }
}

/// Identity mode the device reports at the moment a row is produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityModeClass {
    /// No remote identity; local-only configuration.
    AccountFreeLocal,
    /// Self-hosted org identity.
    SelfHostedOrg,
    /// Managed convenience identity.
    ManagedConvenience,
}

impl IdentityModeClass {
    /// Returns the stable identity-mode token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountFreeLocal => "account_free_local",
            Self::SelfHostedOrg => "self_hosted_org",
            Self::ManagedConvenience => "managed_convenience",
        }
    }
}

/// Export-safe device record carried into every beta row.
///
/// Field selection mirrors `schemas/settings/sync_device_record.schema.json`:
/// the record carries the opaque device id, the user-chosen label
/// (already redacted by the caller when the row crosses an org
/// boundary), the device class, os-family class, identity mode,
/// participation state, lineage cursor, and optional revocation
/// reason. Raw hostnames, MAC addresses, serials, and IP addresses
/// MUST NOT appear here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncBetaDeviceRecord {
    /// Opaque stable device id.
    pub device_id: String,
    /// User-chosen redaction-safe label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_label: Option<String>,
    /// Coarse device class token from the seed.
    pub device_class: String,
    /// Coarse os-family class token from the seed.
    pub os_family_class: String,
    /// Identity-mode class token at the time the row was produced.
    pub identity_mode: IdentityModeClass,
    /// Participation state lifecycle stage.
    pub participation_state: DeviceParticipationState,
    /// Optional revocation-reason token (only populated for non-active rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
    /// Opaque monotonic lineage cursor naming the device's position
    /// in the mutation-journal lineage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage_cursor: Option<String>,
    /// Last time the device was observed by the resolver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    /// Surface that last observed the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_source: Option<String>,
    /// Workspace-trust state observed at last seen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_state: Option<String>,
}

impl SyncBetaDeviceRecord {
    /// Returns a minimal active personal-workstation device record.
    pub fn active_local(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
            device_label: None,
            device_class: "personal_workstation".to_owned(),
            os_family_class: "unknown".to_owned(),
            identity_mode: IdentityModeClass::AccountFreeLocal,
            participation_state: DeviceParticipationState::Active,
            revocation_reason: None,
            lineage_cursor: None,
            last_seen_at: None,
            last_seen_source: None,
            trust_state: None,
        }
    }

    fn to_alpha_device(&self) -> SyncConflictDevice {
        let mut device = SyncConflictDevice::new(self.device_id.clone());
        if let Some(label) = &self.device_label {
            device = device.with_label(label.clone());
        }
        device
    }
}

/// Last-writer breadcrumb naming who produced the local effective value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LastWriterBreadcrumb {
    /// Device id that produced the winning value.
    pub device_id: String,
    /// Redaction-safe device label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_label: Option<String>,
    /// Actor class token from the effective-setting vocabulary.
    pub actor_class: String,
    /// Stable revision ref the resolver applied.
    pub revision_ref: String,
    /// Scope the winning value was written at.
    pub winning_scope: String,
    /// Monotonic stamp the winning write applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    /// Opaque mutation-journal ref for the winning write.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
}

/// Rollback-and-retry decision derived from the resolver and the alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackDecision {
    /// True when applying the recommended resolution requires a rollback checkpoint.
    pub rollback_required: bool,
    /// True when approval is required in addition to the checkpoint.
    pub approval_required: bool,
    /// True when the resolver allows a retry path (e.g. keep_local with reattempt
    /// once the producer device returns to `active`).
    pub retry_allowed: bool,
    /// Retry-state token. Stable; surfaces consume this verbatim.
    pub retry_state: String,
    /// Optional checkpoint ref already created by the caller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Optional approval-ticket ref already granted by the caller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Human-readable explanation for review surfaces.
    pub explanation: String,
}

/// One row in the beta sync conflict review projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictReviewBetaRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Ref back to the inspector record used to build this row.
    pub source_record_ref: String,
    /// Ref back to the alpha conflict packet, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_packet_ref: Option<String>,
    /// Canonical setting id.
    pub setting_id: String,
    /// Scope targeted by the arriving synced value.
    pub conflicting_scope: String,
    /// Local (resolving) device record.
    pub local_device: SyncBetaDeviceRecord,
    /// Remote (producing) device record.
    pub remote_device: SyncBetaDeviceRecord,
    /// High-level sync state class.
    pub sync_state: SyncStateClass,
    /// True when the row arrived through a user-initiated manual continuity import.
    pub import_continuity: bool,
    /// Conflict-class token from the alpha packet; `value_equal_no_op`
    /// when the row is not in conflict.
    pub conflict_class: String,
    /// Resolution lifecycle state.
    pub resolution_state: String,
    /// Resolution paths the row offers (stable tokens).
    pub offered_resolution_paths: Vec<String>,
    /// Recommended resolution path.
    pub recommended_resolution_path: String,
    /// True when the recommended path can be applied without human review.
    pub auto_resolvable: bool,
    /// Last-writer breadcrumb for the local effective value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_writer: Option<LastWriterBreadcrumb>,
    /// Scope that supplied the winning effective value.
    pub winning_scope: String,
    /// Human-readable source label for the winning row.
    pub winning_source_label: String,
    /// Redaction-aware preview of the winning value.
    pub winning_value_preview: String,
    /// Setting redaction class applied to previews.
    pub redaction_class: String,
    /// Lock-state token from the effective resolver.
    pub lock_state: String,
    /// Lock-reason token from the effective resolver.
    pub lock_reason: String,
    /// Optional policy-lock source ref when a policy ceiling is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_lock_ref: Option<String>,
    /// Rollback-and-retry decision attached to this row.
    pub rollback_decision: RollbackDecision,
    /// Alpha conflict-review surface, when a packet exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_review_surface: Option<SyncConflictReviewSurface>,
}

/// Aggregate counters for a sync conflict review page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SyncStateSummary {
    /// Number of rows whose effective value is local-authoritative.
    pub local_authoritative_count: usize,
    /// Number of rows whose effective value is synced.
    pub synced_count: usize,
    /// Number of rows whose value arrived through a manual import.
    pub imported_count: usize,
    /// Number of rows whose arriving bundle is stale.
    pub stale_count: usize,
    /// Number of rows whose producer device is paused, revoked, or forgotten.
    pub disabled_device_count: usize,
    /// Number of rows where rollback is required before any apply.
    pub rollback_required_count: usize,
    /// Number of rows currently under an admin policy ceiling.
    pub policy_locked_count: usize,
    /// Number of rows whose recommended resolution may apply without review.
    pub auto_resolvable_count: usize,
}

/// Beta page projection grouping conflict-review rows for one review session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictReviewBetaPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Page id supplied by the caller.
    pub page_id: String,
    /// Page label.
    pub page_label: String,
    /// Local resolving device.
    pub local_device: SyncBetaDeviceRecord,
    /// Rows in deterministic order.
    pub rows: Vec<SyncConflictReviewBetaRow>,
    /// Devices that are currently paused / revoked / forgotten and so cannot
    /// emit traffic. Quoted here so a reviewer can see the registry posture
    /// without pivoting to a separate surface.
    pub disabled_devices: Vec<SyncBetaDeviceRecord>,
    /// Aggregate counters.
    pub state_summary: SyncStateSummary,
}

/// Support-export wrapper that carries both the beta page and the
/// alpha conflict packets the rows were derived from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictReviewBetaSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by support tooling.
    pub shared_contract_ref: String,
    /// Export id supplied by the caller.
    pub export_id: String,
    /// Beta page included in the export.
    pub page: SyncConflictReviewBetaPage,
    /// Alpha conflict packets the rows were derived from, keyed by source_packet_ref.
    pub conflict_packets: Vec<SyncConflictPacket>,
    /// Number of rows whose literal value preview is redacted.
    pub redacted_value_count: usize,
    /// Number of rows whose recommended resolution requires rollback before apply.
    pub rollback_required_count: usize,
}

/// Caller-supplied request used to materialize one beta row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictReviewBetaRequest {
    /// Canonical setting id or registered alias.
    pub setting_id: String,
    /// Device currently resolving the local effective value.
    pub local_device: SyncBetaDeviceRecord,
    /// Device that produced the arriving synced value.
    pub remote_device: SyncBetaDeviceRecord,
    /// Scope targeted by the arriving synced value.
    pub conflicting_scope: SettingScope,
    /// Arriving synced value. Credential-bearing settings must use broker handles only.
    pub conflicting_value: SettingValue,
    /// True when the arriving value was carried through a user-initiated import.
    #[serde(default)]
    pub import_continuity: bool,
    /// Producer-side bundle epoch. Treated as stale when strictly less than
    /// `local_bundle_epoch_floor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_bundle_epoch: Option<u64>,
    /// Lowest acceptable bundle epoch on the local side; entries with a strictly
    /// lower remote epoch route to `stale`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_bundle_epoch_floor: Option<u64>,
    /// Last-writer breadcrumb for the local effective value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_writer: Option<LastWriterBreadcrumb>,
    /// Optional rollback checkpoint already created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Optional approval ticket already granted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
}

/// Builds one beta conflict-review row from the resolver and a caller-supplied request.
pub fn build_review_row(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    request: SyncConflictReviewBetaRequest,
) -> Result<SyncConflictReviewBetaRow, SettingsInspectError> {
    let inspection = inspect_setting(resolver, &request.setting_id, context)?;
    let alpha_packet = inspect_sync_conflict(
        resolver,
        SyncConflictReviewRequest {
            setting_id: request.setting_id.clone(),
            current_device: request.local_device.to_alpha_device(),
            conflicting_device: request.remote_device.to_alpha_device(),
            conflicting_scope: request.conflicting_scope,
            conflicting_value: request.conflicting_value.clone(),
        },
    )?;

    let alpha_review_surface = alpha_packet
        .as_ref()
        .map(project_sync_conflict_review_surface);

    let stale = matches!(
        (request.remote_bundle_epoch, request.local_bundle_epoch_floor),
        (Some(remote), Some(floor)) if remote < floor,
    );

    let sync_state = classify_sync_state(
        request.local_device.participation_state,
        request.remote_device.participation_state,
        alpha_packet.as_ref(),
        request.import_continuity,
        stale,
    );

    let conflict_class = alpha_packet
        .as_ref()
        .map(|packet| packet.conflict_class.as_str().to_owned())
        .unwrap_or_else(|| {
            if stale {
                "stale_payload".to_owned()
            } else {
                SyncConflictClass::ValueEqualNoOp.as_str().to_owned()
            }
        });

    let resolution_state = alpha_packet
        .as_ref()
        .map(|packet| packet.resolution_state.as_str().to_owned())
        .unwrap_or_else(|| SyncConflictResolutionState::Resolved.as_str().to_owned());

    let (offered_paths, recommended_path, auto_resolvable) =
        offered_paths_and_recommendation(sync_state, alpha_packet.as_ref());

    let policy_lock_ref = alpha_packet
        .as_ref()
        .and_then(|packet| packet.policy_lock_ref.clone());

    let rollback_decision = rollback_decision_for(
        &inspection.definition.preview_class,
        sync_state,
        request.local_device.participation_state,
        request.remote_device.participation_state,
        recommended_path,
        request.rollback_checkpoint_ref.clone(),
        request.approval_ticket_ref.clone(),
    );

    let redaction_class = alpha_packet
        .as_ref()
        .map(|packet| packet.redaction_class.clone())
        .unwrap_or_else(|| inspection.definition.redaction_class.clone());

    Ok(SyncConflictReviewBetaRow {
        record_kind: "sync_conflict_review_beta_row".to_owned(),
        schema_version: SETTINGS_SYNC_BETA_SCHEMA_VERSION,
        shared_contract_ref: SETTINGS_SYNC_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_record_ref: inspection.source_record_ref.clone(),
        source_packet_ref: alpha_packet.as_ref().map(|packet| packet.packet_id.clone()),
        setting_id: inspection.setting_id.clone(),
        conflicting_scope: request.conflicting_scope.as_str().to_owned(),
        local_device: request.local_device.clone(),
        remote_device: request.remote_device.clone(),
        sync_state,
        import_continuity: request.import_continuity,
        conflict_class,
        resolution_state,
        offered_resolution_paths: offered_paths
            .iter()
            .map(|path| path.as_str().to_owned())
            .collect(),
        recommended_resolution_path: recommended_path.as_str().to_owned(),
        auto_resolvable,
        last_writer: request.last_writer,
        winning_scope: inspection.winning_scope.clone(),
        winning_source_label: inspection.source_label.clone(),
        winning_value_preview: inspection.winning_value_summary.clone(),
        redaction_class,
        lock_state: inspection.lock_state.clone(),
        lock_reason: inspection.lock_reason.clone(),
        policy_lock_ref,
        rollback_decision,
        alpha_review_surface,
    })
}

/// Projects a beta review page from one or more rows.
pub fn project_review_page(
    page_id: impl Into<String>,
    page_label: impl Into<String>,
    local_device: SyncBetaDeviceRecord,
    rows: Vec<SyncConflictReviewBetaRow>,
    disabled_devices: Vec<SyncBetaDeviceRecord>,
) -> SyncConflictReviewBetaPage {
    let state_summary = state_summary_from_rows(&rows);
    SyncConflictReviewBetaPage {
        record_kind: "sync_conflict_review_beta_page".to_owned(),
        schema_version: SETTINGS_SYNC_BETA_SCHEMA_VERSION,
        shared_contract_ref: SETTINGS_SYNC_BETA_SHARED_CONTRACT_REF.to_owned(),
        page_id: page_id.into(),
        page_label: page_label.into(),
        local_device,
        rows,
        disabled_devices,
        state_summary,
    }
}

/// Builds a support-export wrapper from a beta page and the alpha
/// packets the rows were built from.
pub fn project_support_export(
    export_id: impl Into<String>,
    page: SyncConflictReviewBetaPage,
    conflict_packets: Vec<SyncConflictPacket>,
) -> SyncConflictReviewBetaSupportExport {
    let redacted_value_count = page
        .rows
        .iter()
        .filter(|row| !matches!(row.redaction_class.as_str(), "none" | "ui_string_only"))
        .count();
    let rollback_required_count = page
        .rows
        .iter()
        .filter(|row| row.rollback_decision.rollback_required)
        .count();
    SyncConflictReviewBetaSupportExport {
        record_kind: "sync_conflict_review_beta_support_export".to_owned(),
        schema_version: SETTINGS_SYNC_BETA_SCHEMA_VERSION,
        shared_contract_ref: SETTINGS_SYNC_BETA_SHARED_CONTRACT_REF.to_owned(),
        export_id: export_id.into(),
        page,
        conflict_packets,
        redacted_value_count,
        rollback_required_count,
    }
}

fn classify_sync_state(
    local: DeviceParticipationState,
    remote: DeviceParticipationState,
    packet: Option<&SyncConflictPacket>,
    import_continuity: bool,
    stale: bool,
) -> SyncStateClass {
    if remote.is_disabled() || local.is_disabled() {
        return SyncStateClass::DisabledDevice;
    }
    if stale {
        return SyncStateClass::Stale;
    }
    if import_continuity {
        return SyncStateClass::Imported;
    }
    match packet {
        None => SyncStateClass::Synced,
        Some(packet) if packet.conflict_class == SyncConflictClass::ValueEqualNoOp => {
            SyncStateClass::Synced
        }
        Some(_) => SyncStateClass::LocalAuthoritative,
    }
}

fn offered_paths_and_recommendation(
    sync_state: SyncStateClass,
    packet: Option<&SyncConflictPacket>,
) -> (
    Vec<SyncConflictResolutionPath>,
    SyncConflictResolutionPath,
    bool,
) {
    // `stale` and `disabled_device` rows always route to keep_local: an
    // older bundle or a non-participating peer must never silently
    // overwrite the local effective value, regardless of the alpha
    // packet's normal recommendation.
    match sync_state {
        SyncStateClass::Stale | SyncStateClass::DisabledDevice => {
            return (
                vec![
                    SyncConflictResolutionPath::KeepLocal,
                    SyncConflictResolutionPath::Decline,
                ],
                SyncConflictResolutionPath::KeepLocal,
                false,
            );
        }
        _ => {}
    }
    if let Some(packet) = packet {
        return (
            packet.offered_resolution_paths.clone(),
            packet.recommended_resolution_path,
            packet.auto_resolvable,
        );
    }
    match sync_state {
        SyncStateClass::Imported => (
            vec![
                SyncConflictResolutionPath::KeepLocal,
                SyncConflictResolutionPath::KeepSynced,
            ],
            SyncConflictResolutionPath::KeepSynced,
            true,
        ),
        SyncStateClass::Synced | SyncStateClass::LocalAuthoritative => (
            vec![SyncConflictResolutionPath::KeepLocal],
            SyncConflictResolutionPath::KeepLocal,
            true,
        ),
        SyncStateClass::Stale | SyncStateClass::DisabledDevice => unreachable!(),
    }
}

fn rollback_decision_for(
    preview_class_token: &str,
    sync_state: SyncStateClass,
    local: DeviceParticipationState,
    remote: DeviceParticipationState,
    recommended: SyncConflictResolutionPath,
    rollback_checkpoint_ref: Option<String>,
    approval_ticket_ref: Option<String>,
) -> RollbackDecision {
    let preview_class = preview_class_from_token(preview_class_token);
    let rollback_required = match recommended {
        SyncConflictResolutionPath::KeepLocal | SyncConflictResolutionPath::Decline => false,
        _ => preview_class
            .map(PreviewClass::requires_checkpoint)
            .unwrap_or(false),
    };
    let approval_required = match recommended {
        SyncConflictResolutionPath::KeepLocal | SyncConflictResolutionPath::Decline => false,
        _ => preview_class
            .map(PreviewClass::requires_approval)
            .unwrap_or(false),
    };

    let retry_allowed = matches!(
        sync_state,
        SyncStateClass::Stale | SyncStateClass::DisabledDevice
    );
    let retry_state = if retry_allowed {
        match sync_state {
            SyncStateClass::Stale => "retry_when_fresh_bundle_arrives",
            SyncStateClass::DisabledDevice if remote.is_disabled() => {
                "retry_when_remote_device_active"
            }
            SyncStateClass::DisabledDevice if local.is_disabled() => {
                "retry_when_local_device_active"
            }
            _ => "retry_available",
        }
    } else if rollback_required {
        "checkpoint_required_before_apply"
    } else {
        "no_retry_required"
    };

    let explanation = build_rollback_explanation(
        rollback_required,
        approval_required,
        retry_allowed,
        sync_state,
        recommended,
    );

    RollbackDecision {
        rollback_required,
        approval_required,
        retry_allowed,
        retry_state: retry_state.to_owned(),
        rollback_checkpoint_ref,
        approval_ticket_ref,
        explanation,
    }
}

fn preview_class_from_token(token: &str) -> Option<PreviewClass> {
    match token {
        "safe_apply" => Some(PreviewClass::SafeApply),
        "preview_required" => Some(PreviewClass::PreviewRequired),
        "rollback_checkpoint_required" => Some(PreviewClass::RollbackCheckpointRequired),
        "rollback_checkpoint_and_approval_required" => {
            Some(PreviewClass::RollbackCheckpointAndApprovalRequired)
        }
        "managed_action_only" => Some(PreviewClass::ManagedActionOnly),
        _ => None,
    }
}

fn build_rollback_explanation(
    rollback_required: bool,
    approval_required: bool,
    retry_allowed: bool,
    sync_state: SyncStateClass,
    recommended: SyncConflictResolutionPath,
) -> String {
    if rollback_required && approval_required {
        return "Apply requires a rollback checkpoint and an approval ticket before the synced value lands.".to_owned();
    }
    if rollback_required {
        return "Apply requires a rollback checkpoint before the synced value lands.".to_owned();
    }
    if matches!(recommended, SyncConflictResolutionPath::KeepLocal) {
        if matches!(sync_state, SyncStateClass::DisabledDevice) {
            return "Producer device is paused, revoked, or forgotten; local value remains authoritative.".to_owned();
        }
        if matches!(sync_state, SyncStateClass::Stale) {
            return "Arriving bundle is older than the local lineage; local value remains authoritative.".to_owned();
        }
        return "Local value remains authoritative; no apply is required.".to_owned();
    }
    if retry_allowed {
        return "Retry is available once the upstream condition clears.".to_owned();
    }
    "Recommended path applies without a checkpoint.".to_owned()
}

fn state_summary_from_rows(rows: &[SyncConflictReviewBetaRow]) -> SyncStateSummary {
    let mut summary = SyncStateSummary::default();
    for row in rows {
        match row.sync_state {
            SyncStateClass::LocalAuthoritative => summary.local_authoritative_count += 1,
            SyncStateClass::Synced => summary.synced_count += 1,
            SyncStateClass::Imported => summary.imported_count += 1,
            SyncStateClass::Stale => summary.stale_count += 1,
            SyncStateClass::DisabledDevice => summary.disabled_device_count += 1,
        }
        if row.rollback_decision.rollback_required {
            summary.rollback_required_count += 1;
        }
        if row.policy_lock_ref.is_some() {
            summary.policy_locked_count += 1;
        }
        if row.auto_resolvable {
            summary.auto_resolvable_count += 1;
        }
    }
    summary
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::{PolicyConstraint, ScopeOverlay};
    use crate::schema::SchemaRegistry;

    fn seeded_resolver() -> EffectiveSettingsResolver {
        EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog())
    }

    fn local_device() -> SyncBetaDeviceRecord {
        SyncBetaDeviceRecord {
            device_id: "dev-laptop-primary-0001".to_owned(),
            device_label: Some("Dev laptop".to_owned()),
            device_class: "personal_laptop".to_owned(),
            os_family_class: "macos".to_owned(),
            identity_mode: IdentityModeClass::AccountFreeLocal,
            participation_state: DeviceParticipationState::Active,
            revocation_reason: None,
            lineage_cursor: Some("lc-0001-000000000517".to_owned()),
            last_seen_at: Some("2026-04-20T09:00:00Z".to_owned()),
            last_seen_source: Some("local_heartbeat".to_owned()),
            trust_state: Some("trusted".to_owned()),
        }
    }

    fn remote_device(participation_state: DeviceParticipationState) -> SyncBetaDeviceRecord {
        SyncBetaDeviceRecord {
            device_id: "dev-desktop-home-0002".to_owned(),
            device_label: Some("Home desktop".to_owned()),
            device_class: "personal_workstation".to_owned(),
            os_family_class: "macos".to_owned(),
            identity_mode: IdentityModeClass::AccountFreeLocal,
            participation_state,
            revocation_reason: if participation_state.is_disabled() {
                Some("user_paused".to_owned())
            } else {
                None
            },
            lineage_cursor: Some("lc-0002-000000000142".to_owned()),
            last_seen_at: Some("2026-04-19T22:11:00Z".to_owned()),
            last_seen_source: Some("push".to_owned()),
            trust_state: Some("trusted".to_owned()),
        }
    }

    #[test]
    fn no_conflict_row_classifies_as_synced() {
        let mut resolver = seeded_resolver();
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value("editor.format_on_save", SettingValue::Boolean(true));
        resolver.set_overlay(user).unwrap();
        let context = SettingsInspectionContext::new();
        let row = build_review_row(
            &resolver,
            &context,
            SyncConflictReviewBetaRequest {
                setting_id: "editor.format_on_save".to_owned(),
                local_device: local_device(),
                remote_device: remote_device(DeviceParticipationState::Active),
                conflicting_scope: SettingScope::UserGlobal,
                conflicting_value: SettingValue::Boolean(true),
                import_continuity: false,
                remote_bundle_epoch: Some(142),
                local_bundle_epoch_floor: Some(140),
                last_writer: None,
                rollback_checkpoint_ref: None,
                approval_ticket_ref: None,
            },
        )
        .unwrap();
        assert_eq!(row.sync_state, SyncStateClass::Synced);
        assert!(row.source_packet_ref.is_none());
        assert!(!row.rollback_decision.rollback_required);
        assert_eq!(row.rollback_decision.retry_state, "no_retry_required");
        assert_eq!(row.conflict_class, "value_equal_no_op");
    }

    #[test]
    fn stale_bundle_routes_to_keep_local_with_retry() {
        let resolver = seeded_resolver();
        let context = SettingsInspectionContext::new();
        let row = build_review_row(
            &resolver,
            &context,
            SyncConflictReviewBetaRequest {
                setting_id: "editor.tab_size".to_owned(),
                local_device: local_device(),
                remote_device: remote_device(DeviceParticipationState::Active),
                conflicting_scope: SettingScope::UserGlobal,
                conflicting_value: SettingValue::Integer(8),
                import_continuity: false,
                remote_bundle_epoch: Some(100),
                local_bundle_epoch_floor: Some(200),
                last_writer: Some(LastWriterBreadcrumb {
                    device_id: "dev-laptop-primary-0001".to_owned(),
                    device_label: Some("Dev laptop".to_owned()),
                    actor_class: "user_keystroke".to_owned(),
                    revision_ref: "settings-rev:00517".to_owned(),
                    winning_scope: "user_global".to_owned(),
                    at: Some("2026-04-18T14:05:31Z".to_owned()),
                    mutation_journal_ref: Some("mjr-laptop-primary-0001-000000000517".to_owned()),
                }),
                rollback_checkpoint_ref: None,
                approval_ticket_ref: None,
            },
        )
        .unwrap();
        assert_eq!(row.sync_state, SyncStateClass::Stale);
        assert_eq!(row.recommended_resolution_path, "keep_local");
        assert!(row.rollback_decision.retry_allowed);
        assert_eq!(
            row.rollback_decision.retry_state,
            "retry_when_fresh_bundle_arrives"
        );
        assert!(row.last_writer.is_some());
    }

    #[test]
    fn paused_remote_classifies_as_disabled_device() {
        let resolver = seeded_resolver();
        let context = SettingsInspectionContext::new();
        let row = build_review_row(
            &resolver,
            &context,
            SyncConflictReviewBetaRequest {
                setting_id: "editor.tab_size".to_owned(),
                local_device: local_device(),
                remote_device: remote_device(DeviceParticipationState::Paused),
                conflicting_scope: SettingScope::UserGlobal,
                conflicting_value: SettingValue::Integer(8),
                import_continuity: false,
                remote_bundle_epoch: Some(142),
                local_bundle_epoch_floor: Some(140),
                last_writer: None,
                rollback_checkpoint_ref: None,
                approval_ticket_ref: None,
            },
        )
        .unwrap();
        assert_eq!(row.sync_state, SyncStateClass::DisabledDevice);
        assert!(row.rollback_decision.retry_allowed);
        assert_eq!(
            row.rollback_decision.retry_state,
            "retry_when_remote_device_active"
        );
    }

    #[test]
    fn import_continuity_routes_to_imported_state() {
        let resolver = seeded_resolver();
        let context = SettingsInspectionContext::new();
        let row = build_review_row(
            &resolver,
            &context,
            SyncConflictReviewBetaRequest {
                setting_id: "editor.format_on_save".to_owned(),
                local_device: local_device(),
                remote_device: remote_device(DeviceParticipationState::Active),
                conflicting_scope: SettingScope::UserGlobal,
                conflicting_value: SettingValue::Boolean(false),
                import_continuity: true,
                remote_bundle_epoch: None,
                local_bundle_epoch_floor: None,
                last_writer: None,
                rollback_checkpoint_ref: None,
                approval_ticket_ref: None,
            },
        )
        .unwrap();
        assert_eq!(row.sync_state, SyncStateClass::Imported);
    }

    #[test]
    fn policy_locked_high_risk_row_requires_rollback() {
        let mut resolver = seeded_resolver();
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value(
            "security.ai.egress_policy",
            SettingValue::String("any_hosted_provider".to_owned()),
        );
        resolver.set_overlay(user).unwrap();
        let mut policy =
            ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle v3");
        policy.set_policy_constraint(
            "security.ai.egress_policy",
            PolicyConstraint::SingleValue {
                value: SettingValue::String("approved_hosted_providers_only".to_owned()),
            },
        );
        resolver.set_overlay(policy).unwrap();

        let context = SettingsInspectionContext::new();
        let row = build_review_row(
            &resolver,
            &context,
            SyncConflictReviewBetaRequest {
                setting_id: "security.ai.egress_policy".to_owned(),
                local_device: local_device(),
                remote_device: remote_device(DeviceParticipationState::Active),
                conflicting_scope: SettingScope::UserGlobal,
                conflicting_value: SettingValue::String("any_hosted_provider".to_owned()),
                import_continuity: false,
                remote_bundle_epoch: Some(142),
                local_bundle_epoch_floor: Some(140),
                last_writer: None,
                rollback_checkpoint_ref: None,
                approval_ticket_ref: None,
            },
        )
        .unwrap();
        assert_eq!(row.lock_state, "policy_locked");
        assert!(row.policy_lock_ref.is_some());
        assert_eq!(row.recommended_resolution_path, "merge_preview");
        assert!(row.rollback_decision.rollback_required);
        assert!(row.rollback_decision.approval_required);
    }

    #[test]
    fn support_export_counts_redacted_and_rollback_rows() {
        let mut resolver = seeded_resolver();
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value(
            "security.ai.egress_policy",
            SettingValue::String("any_hosted_provider".to_owned()),
        );
        resolver.set_overlay(user).unwrap();
        let mut policy =
            ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle v3");
        policy.set_policy_constraint(
            "security.ai.egress_policy",
            PolicyConstraint::SingleValue {
                value: SettingValue::String("approved_hosted_providers_only".to_owned()),
            },
        );
        resolver.set_overlay(policy).unwrap();
        let context = SettingsInspectionContext::new();
        let row = build_review_row(
            &resolver,
            &context,
            SyncConflictReviewBetaRequest {
                setting_id: "security.ai.egress_policy".to_owned(),
                local_device: local_device(),
                remote_device: remote_device(DeviceParticipationState::Active),
                conflicting_scope: SettingScope::UserGlobal,
                conflicting_value: SettingValue::String("any_hosted_provider".to_owned()),
                import_continuity: false,
                remote_bundle_epoch: Some(142),
                local_bundle_epoch_floor: Some(140),
                last_writer: None,
                rollback_checkpoint_ref: None,
                approval_ticket_ref: None,
            },
        )
        .unwrap();
        let packets = row
            .alpha_review_surface
            .as_ref()
            .map(|_| {
                inspect_sync_conflict(
                    &resolver,
                    SyncConflictReviewRequest {
                        setting_id: "security.ai.egress_policy".to_owned(),
                        current_device: SyncConflictDevice::new("dev-laptop-primary-0001"),
                        conflicting_device: SyncConflictDevice::new("dev-desktop-home-0002"),
                        conflicting_scope: SettingScope::UserGlobal,
                        conflicting_value: SettingValue::String("any_hosted_provider".to_owned()),
                    },
                )
                .unwrap()
                .unwrap()
            })
            .into_iter()
            .collect::<Vec<_>>();

        let page = project_review_page(
            "sync-review-001",
            "Sync conflict review",
            local_device(),
            vec![row],
            Vec::new(),
        );
        let export = project_support_export("support-export:settings-sync-beta:001", page, packets);
        assert_eq!(export.rollback_required_count, 1);
        assert_eq!(export.shared_contract_ref, "settings:sync_beta:v1");
        assert!(export
            .page
            .rows
            .iter()
            .all(|row| !row.winning_value_preview.contains("raw")));
    }
}
