//! Governed effective-settings record for M5-added settings families.
//!
//! This module is the settings-lane contract that brings new M5 settings
//! families — notebooks, data/API, profiler, bundle, sync, and companion — into
//! parity with the stable effective-settings resolver. It does not re-implement
//! the precedence engine; it defines the canonical record those families must
//! emit so the settings UI, CLI inspect, Help/About, policy explainers,
//! admin-distribution audit, and support bundles all answer the same questions
//! for an M5 row:
//!
//! 1. Which value is active, which scope won, and which candidates lost.
//! 2. Whether the row is being viewed as source, effective, or live truth.
//! 3. Why a write is allowed, shadowed, constrained, or denied.
//! 4. Which signed bundle or governing scope narrowed the row, when it last
//!    applied, and what still works under the narrower local-safe posture.
//!
//! The gate is fail-closed. A high-impact M5 setting — one that materially
//! changes trust, AI/network egress, extension behavior, remote exposure, or a
//! destructive-automation default — can never be recorded without a
//! scope-explicit write preview and a rollback checkpoint. Likewise, a
//! constrained or locked row cannot hide its governing bundle/scope, active
//! review or expiry posture, last-applied facts, or local-safe continuation
//! path behind generic denial text.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for M5 effective-settings records.
pub const M5_EFFECTIVE_SETTINGS_RECORD_KIND: &str = "m5_effective_settings_certification_record";

/// Schema version for [`M5EffectiveSettingsCertification`] records.
pub const M5_EFFECTIVE_SETTINGS_SCHEMA_VERSION: u32 = 2;

/// Shared contract ref consumed by settings UI, CLI, help, policy, admin audit,
/// and support.
pub const M5_EFFECTIVE_SETTINGS_SHARED_CONTRACT_REF: &str = "settings:m5_effective_settings:v2";

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

/// Canonical settings scope, mirroring the resolver's precedence vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingScope {
    /// Built-in product default.
    BuiltInDefault,
    /// Channel- or experiment-supplied default.
    ChannelOrExperimentDefault,
    /// Default carried in from an imported profile.
    ImportedProfileDefault,
    /// User-global preference.
    UserGlobal,
    /// Machine-specific preference.
    MachineSpecific,
    /// Workspace-scoped value.
    Workspace,
    /// Folder- or module-scoped override.
    FolderOrModuleOverride,
    /// Language-scoped override.
    LanguageOverride,
    /// Session-scoped override.
    SessionOverride,
    /// Admin policy ceiling that narrows or locks the layered winner.
    AdminPolicyNarrowing,
}

impl SettingScope {
    /// Returns the canonical token for this scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuiltInDefault => "built_in_default",
            Self::ChannelOrExperimentDefault => "channel_or_experiment_default",
            Self::ImportedProfileDefault => "imported_profile_default",
            Self::UserGlobal => "user_global",
            Self::MachineSpecific => "machine_specific",
            Self::Workspace => "workspace",
            Self::FolderOrModuleOverride => "folder_or_module_override",
            Self::LanguageOverride => "language_override",
            Self::SessionOverride => "session_override",
            Self::AdminPolicyNarrowing => "admin_policy_narrowing",
        }
    }
}

/// A new M5-added settings family brought into resolver parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingFamily {
    /// Notebook execution, kernel, and trust settings.
    Notebooks,
    /// Data and API connection, egress, and credential settings.
    DataApi,
    /// Profiler sampling, capture, and retention settings.
    Profiler,
    /// Bundle and extension acquisition and auto-install settings.
    Bundle,
    /// Sync participation and device-registry settings.
    Sync,
    /// Companion device control and remote-surface settings.
    Companion,
}

impl M5SettingFamily {
    /// Returns the canonical token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebooks => "notebooks",
            Self::DataApi => "data_api",
            Self::Profiler => "profiler",
            Self::Bundle => "bundle",
            Self::Sync => "sync",
            Self::Companion => "companion",
        }
    }

    /// Every M5 settings family the contract requires a record to cover.
    pub const REQUIRED: [Self; 6] = [
        Self::Notebooks,
        Self::DataApi,
        Self::Profiler,
        Self::Bundle,
        Self::Sync,
        Self::Companion,
    ];
}

/// What a settings consumer must do for a changed value to take effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestartPosture {
    /// Picked up live; no restart required.
    NoRestart,
    /// Reload affected views to apply.
    ReloadView,
    /// Reload the active workspace to apply.
    ReloadWorkspace,
    /// Restart extension/runtime hosts to apply.
    RestartExtensions,
    /// Restart the application process to apply.
    RestartProcess,
}

impl RestartPosture {
    /// Returns the canonical token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRestart => "no_restart",
            Self::ReloadView => "reload_view",
            Self::ReloadWorkspace => "reload_workspace",
            Self::RestartExtensions => "restart_extensions",
            Self::RestartProcess => "restart_process",
        }
    }
}

/// Validation outcome for the value that won the precedence tie-break.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationState {
    /// The winning value validates against the current schema.
    Valid,
    /// The stored value was invalid and fell back to the declared default.
    CoercedToDefault,
    /// The stored value was clamped to an allowed bound.
    OutOfRange,
    /// The value came from a stale schema and cannot be trusted until migrated.
    SchemaStale,
}

impl ValidationState {
    /// Returns the canonical token for this validation state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::CoercedToDefault => "coerced_to_default",
            Self::OutOfRange => "out_of_range",
            Self::SchemaStale => "schema_stale",
        }
    }

    /// Returns true when the value validates cleanly against the live schema.
    pub const fn is_clean(self) -> bool {
        matches!(self, Self::Valid)
    }

    /// Returns true when the resolved value cannot be trusted as-is.
    pub const fn is_untrusted(self) -> bool {
        matches!(self, Self::SchemaStale)
    }
}

/// High-impact class for an M5 setting that materially changes risk posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighImpactClass {
    /// Changes a trust boundary (e.g. running untrusted content).
    TrustBoundary,
    /// Changes AI or network egress behavior.
    AiNetworkEgress,
    /// Changes extension or runtime-host behavior.
    ExtensionBehavior,
    /// Changes remote exposure of the local environment.
    RemoteExposure,
    /// Changes a destructive-automation default.
    DestructiveAutomation,
}

impl HighImpactClass {
    /// Returns the canonical token for this high-impact class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustBoundary => "trust_boundary",
            Self::AiNetworkEgress => "ai_network_egress",
            Self::ExtensionBehavior => "extension_behavior",
            Self::RemoteExposure => "remote_exposure",
            Self::DestructiveAutomation => "destructive_automation",
        }
    }
}

/// Kind of lifecycle-sensitive dependency that narrows an M5 row's behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleDependencyKind {
    /// A required capability is not present, so the setting cannot fully apply.
    MissingCapability,
    /// The setting depends on a Labs/Preview/Experimental feature lifecycle.
    LabsOrPreviewDependent,
}

impl LifecycleDependencyKind {
    /// Returns the canonical token for this dependency kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingCapability => "missing_capability",
            Self::LabsOrPreviewDependent => "labs_or_preview_dependent",
        }
    }
}

/// Why a candidate value lost the precedence tie-break.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShadowReason {
    /// A higher-precedence scope supplied the winning value.
    LowerPrecedence,
    /// An admin policy ceiling narrowed this candidate away.
    PolicyNarrowed,
    /// This candidate failed validation and was rejected.
    ValidationRejected,
}

impl ShadowReason {
    /// Returns the canonical token for this shadow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LowerPrecedence => "lower_precedence",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::ValidationRejected => "validation_rejected",
        }
    }
}

/// Effect a previewed write would have once applied at its target scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteEffect {
    /// The written value becomes the new winning value.
    BecomesWinningValue,
    /// The write lands but stays shadowed by an admin policy ceiling.
    ShadowedByPolicy,
    /// The write is denied because the setting is policy-locked.
    DeniedByLock,
}

impl WriteEffect {
    /// Returns the canonical token for this write effect.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BecomesWinningValue => "becomes_winning_value",
            Self::ShadowedByPolicy => "shadowed_by_policy",
            Self::DeniedByLock => "denied_by_lock",
        }
    }
}

/// Source surface that must render the same effective-settings truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// Desktop settings UI.
    SettingsUi,
    /// CLI or headless inspect command.
    CliInspect,
    /// Help links and inline help.
    HelpLinks,
    /// Help/About truth rows.
    HelpAbout,
    /// Policy explainer surface.
    PolicyExplainer,
    /// Admin-distribution audit surface.
    AdminDistributionAudit,
    /// Support bundle / support-center export.
    SupportBundle,
}

impl SurfaceClass {
    /// Required surface set for parity.
    pub const REQUIRED: [Self; 7] = [
        Self::SettingsUi,
        Self::CliInspect,
        Self::HelpLinks,
        Self::HelpAbout,
        Self::PolicyExplainer,
        Self::AdminDistributionAudit,
        Self::SupportBundle,
    ];
}

/// Projection mode disclosed on review sheets and audit rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionMode {
    /// Authored/source-layer view.
    Source,
    /// Effective/resolved view.
    Effective,
    /// Live/observed view.
    Live,
}

impl ProjectionMode {
    /// Returns the canonical token for this projection mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Effective => "effective",
            Self::Live => "live",
        }
    }
}

/// Policy constraint posture applied to a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyConstraintState {
    /// No policy narrowing applies.
    Unlocked,
    /// Policy constrains the allowed shape or values, but a local write can
    /// still land in a shadowed form.
    Constrained,
    /// Policy locks the setting against local writes.
    Locked,
}

impl PolicyConstraintState {
    /// Returns the canonical token for this constraint state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unlocked => "unlocked",
            Self::Constrained => "constrained",
            Self::Locked => "locked",
        }
    }

    /// Returns true when this state narrows the row below ordinary local control.
    pub const fn narrows(self) -> bool {
        !matches!(self, Self::Unlocked)
    }

    /// Returns true when this state is a hard local write lock.
    pub const fn is_locked(self) -> bool {
        matches!(self, Self::Locked)
    }
}

/// Distribution source preserved on configuration-policy rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDistributionSource {
    /// Live managed pull from the signed origin.
    ManagedPull,
    /// Mirror publication or mirror-sync distribution.
    MirrorPublication,
    /// Local operator file import.
    FileImport,
    /// Device-management or fleet drop.
    MdmFleetDrop,
    /// Air-gapped transfer path.
    AirGapTransfer,
    /// Last-known-good cache selection.
    LastKnownGoodCache,
}

impl PolicyDistributionSource {
    /// Returns the canonical token for this source.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedPull => "managed_pull",
            Self::MirrorPublication => "mirror_publication",
            Self::FileImport => "file_import",
            Self::MdmFleetDrop => "mdm_fleet_drop",
            Self::AirGapTransfer => "air_gap_transfer",
            Self::LastKnownGoodCache => "last_known_good_cache",
        }
    }
}

/// Freshness state disclosed on admin-distribution audit rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditFreshnessState {
    /// The bundle is current and verified.
    Current,
    /// The row is using a stale-but-reviewable bundle.
    Stale,
    /// The row is beyond freshness and narrowed to local-safe continuation.
    Expired,
}

impl AuditFreshnessState {
    /// Returns the canonical token for this freshness state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Expired => "expired",
        }
    }

    /// Returns true when the audit row is no longer fresh.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Stale | Self::Expired)
    }
}

/// Export posture for an effective-value review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewExportPosture {
    /// Export only keys and their contributing sources.
    KeysAndSourcesOnly,
    /// Export keys, contributing sources, and redacted live references.
    KeysSourcesAndLiveRefs,
    /// Export metadata only.
    MetadataOnly,
}

impl ReviewExportPosture {
    /// Returns the canonical token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeysAndSourcesOnly => "keys_and_sources_only",
            Self::KeysSourcesAndLiveRefs => "keys_sources_and_live_refs",
            Self::MetadataOnly => "metadata_only",
        }
    }
}

/// Explicit action offered from an effective-value review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewAction {
    /// Copy a bounded redacted summary.
    CopyRedactedSummary,
    /// Open the authored/source view.
    OpenSource,
    /// Open the effective/resolved view.
    OpenEffective,
    /// Open the live/observed view.
    OpenLive,
    /// Open the governing policy bundle or scope.
    OpenPolicyBundle,
    /// Retry policy sync or refresh.
    RetryPolicySync,
    /// Continue in the local-safe posture.
    ContinueLocal,
}

impl ReviewAction {
    /// Returns the canonical token for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyRedactedSummary => "copy_redacted_summary",
            Self::OpenSource => "open_source",
            Self::OpenEffective => "open_effective",
            Self::OpenLive => "open_live",
            Self::OpenPolicyBundle => "open_policy_bundle",
            Self::RetryPolicySync => "retry_policy_sync",
            Self::ContinueLocal => "continue_local",
        }
    }
}

/// The value that won the precedence tie-break for an M5 setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WinningValue {
    /// Scope that supplied the winning value.
    pub scope: SettingScope,
    /// Canonical ref to the value snapshot (never a raw secret body).
    pub value_ref: String,
    /// Short, redaction-safe display string for the active value.
    pub display: String,
}

/// A candidate value that lost the tie-break, kept visible in the shadow chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShadowedCandidate {
    /// Scope that supplied this shadowed candidate.
    pub scope: SettingScope,
    /// Canonical ref to the shadowed value snapshot.
    pub value_ref: String,
    /// Why this candidate lost.
    pub reason: ShadowReason,
}

/// Write-denial or shadowing explanation for a constrained/locked row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteConstraintExplanation {
    /// Governing signed bundle, when the policy came from a bundle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_bundle_ref: Option<String>,
    /// Governing policy scope when the policy came from a narrower scope rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_scope_ref: Option<String>,
    /// Owner who governs the bundle or policy scope, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_owner_ref: Option<String>,
    /// Review object or policy ticket that explains the current posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_ref: Option<String>,
    /// Timestamp at which the current posture should be reviewed again.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_due_at: Option<String>,
    /// Timestamp at which the posture expires, when bounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Human-readable denial or narrowing explanation.
    pub denied_reason: String,
    /// What still works while the row is narrower than full managed freshness.
    pub local_safe_continuation: Vec<String>,
    /// Safe repair or continuation hint.
    pub repair_hint: String,
}

/// Policy state applied to an M5 setting row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyLockState {
    /// Whether the row is unlocked, constrained, or locked.
    pub constraint_state: PolicyConstraintState,
    /// Canonical ref to the policy object that imposes the posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_ref: Option<String>,
    /// Governing bundle under which the posture was applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_bundle_ref: Option<String>,
    /// Governing scope when the posture came from scope targeting rather than a
    /// bundle identity alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_scope_ref: Option<String>,
    /// Owner of the governing bundle/scope, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_owner_ref: Option<String>,
    /// Distribution source that delivered the governing bundle, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution_source: Option<PolicyDistributionSource>,
    /// Timestamp of the last successful apply for the governing bundle/scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_applied_at: Option<String>,
    /// Review timestamp for the governing posture, when bounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_due_at: Option<String>,
    /// Expiry timestamp for the governing posture, when bounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Human-readable summary of the locked or constrained posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraint_summary: Option<String>,
    /// What still works while this row is narrower than fully active.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub local_safe_continuation: Vec<String>,
}

/// A visible marker that a lifecycle-sensitive dependency narrows this row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleDependencyMarker {
    /// Dependency kind narrowing the row.
    pub kind: LifecycleDependencyKind,
    /// Canonical ref to the capability or feature this row depends on.
    pub depends_on_ref: String,
    /// Human-readable description of how behavior is narrowed.
    pub narrows_behavior: String,
    /// Recovery hint, e.g. enable the feature or install the capability.
    pub recovery_hint: String,
    /// Whether the marker is visible to the user/admin (must be true).
    pub visible: bool,
}

/// A review sheet for the current source/effective/live projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveValueReviewSheet {
    /// The key set under review. For a settings row this must include
    /// `setting_id`, even when the review groups adjacent derived values.
    pub selected_keys: Vec<String>,
    /// Which projection is currently active on the review sheet.
    pub active_projection_mode: ProjectionMode,
    /// Which projections the surface can switch between without leaving the
    /// governed contract.
    pub available_projection_modes: Vec<ProjectionMode>,
    /// Ordered winning layers shown on the review sheet.
    pub winning_layers: Vec<SettingScope>,
    /// Any unresolved keys or values that the review sheet must still label.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unresolved_values: Vec<String>,
    /// Export safety posture for the review sheet.
    pub export_posture: ReviewExportPosture,
    /// Bounded actions offered from the review sheet.
    pub available_actions: Vec<ReviewAction>,
}

/// A scope-explicit, checkpointed write preview for a high-impact M5 setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeExplicitWritePreview {
    /// Scope the write would target.
    pub target_scope: SettingScope,
    /// Canonical ref to the current value snapshot.
    pub current_value_ref: String,
    /// Canonical ref to the proposed value snapshot.
    pub proposed_value_ref: String,
    /// What the write would actually do once applied.
    pub effective_after_write: WriteEffect,
    /// Restart posture that applies after the write.
    pub restart_posture_after: RestartPosture,
    /// Whether explicit confirmation is required before applying.
    pub requires_confirmation: bool,
    /// Canonical ref to the rollback checkpoint created before applying.
    pub rollback_checkpoint_ref: String,
    /// Explainability block for denied or constrained writes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<WriteConstraintExplanation>,
}

/// One effective-settings row for an M5-added setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EffectiveSettingRow {
    /// Stable setting id.
    pub setting_id: String,
    /// M5 family this setting belongs to.
    pub family: M5SettingFamily,
    /// Human-readable title.
    pub title: String,
    /// The value that won, with its scope.
    pub winning_value: WinningValue,
    /// Ordered candidates that were shadowed by the winner.
    pub shadow_chain: Vec<ShadowedCandidate>,
    /// Restart posture declared for this setting.
    pub restart_posture: RestartPosture,
    /// Validation state of the winning value.
    pub validation_state: ValidationState,
    /// Policy constraint/lock state for this row.
    pub policy_lock: PolicyLockState,
    /// Review sheet for the current source/effective/live projection.
    pub effective_value_review: EffectiveValueReviewSheet,
    /// Lifecycle-sensitive dependency marker, when one narrows this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_dependency: Option<LifecycleDependencyMarker>,
    /// High-impact class when this setting materially changes risk posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_impact_class: Option<HighImpactClass>,
    /// Scope-explicit, checkpointed write preview (required when high-impact).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_preview: Option<ScopeExplicitWritePreview>,
}

impl M5EffectiveSettingRow {
    /// Returns true when this row materially changes risk posture.
    pub fn is_high_impact(&self) -> bool {
        self.high_impact_class.is_some()
    }

    /// Derives the trust state for this row from its inputs.
    pub fn row_trust(&self) -> RowTrust {
        if self.validation_state.is_untrusted() {
            return RowTrust::Withheld;
        }
        if !self.validation_state.is_clean()
            || self.policy_lock.constraint_state.narrows()
            || self.lifecycle_dependency.is_some()
        {
            return RowTrust::Narrowed;
        }
        RowTrust::Active
    }
}

/// Per-family admin-distribution audit row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminDistributionAuditRow {
    /// Stable audit row id.
    pub audit_id: String,
    /// Family governed by this row.
    pub family: M5SettingFamily,
    /// Active bundle or bundle-set identity.
    pub bundle_ref: String,
    /// Owner of the bundle/scope currently in force.
    pub bundle_owner_ref: String,
    /// Policy scope at which the bundle applied.
    pub policy_scope_ref: String,
    /// Distribution path that delivered the bundle.
    pub distribution_source: PolicyDistributionSource,
    /// Last successful apply timestamp.
    pub last_applied_at: String,
    /// Last successful validation timestamp.
    pub last_validated_at: String,
    /// Whether the audit row is currently being viewed as source, effective, or
    /// live truth.
    pub active_projection_mode: ProjectionMode,
    /// Freshness state of the governing bundle.
    pub freshness_state: AuditFreshnessState,
    /// What still works when freshness is stale or expired.
    pub local_safe_continuation: Vec<String>,
}

/// Source surface parity row for effective-settings truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTruthRow {
    /// Surface class.
    pub surface_class: SurfaceClass,
    /// Whether the surface consumes this shared record.
    pub consumes_shared_record: bool,
    /// Whether the surface shows the winning scope.
    pub shows_winning_scope: bool,
    /// Whether the surface shows the shadow chain.
    pub shows_shadow_chain: bool,
    /// Whether the surface shows the restart posture.
    pub shows_restart_posture: bool,
    /// Whether the surface shows the active source/effective/live projection.
    pub shows_projection_mode: bool,
    /// Whether the surface shows the lifecycle-dependency marker.
    pub shows_lifecycle_dependency: bool,
    /// Whether the surface shows the scope-explicit write preview.
    pub shows_write_preview: bool,
    /// Whether the surface shows the lock/constrained explanation.
    pub shows_write_explanation: bool,
    /// Whether the surface shows the admin-distribution audit row.
    pub shows_distribution_audit: bool,
    /// Whether the surface shows last-applied timestamps.
    pub shows_last_applied: bool,
    /// Whether the surface shows local-safe continuation facts.
    pub shows_local_safe_continuation: bool,
}

/// Derived trust state for one M5 setting row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowTrust {
    /// The value validates and no dependency or constraint narrows it.
    Active,
    /// The value resolves but a lock, constraint, coercion, or lifecycle
    /// dependency narrows it.
    Narrowed,
    /// The value cannot be trusted as resolved (e.g. stale schema).
    Withheld,
}

impl RowTrust {
    /// Returns the canonical token for this trust state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Narrowed => "narrowed",
            Self::Withheld => "withheld",
        }
    }

    /// Trust rank where `0` is active and higher values are weaker.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Active => 0,
            Self::Narrowed => 1,
            Self::Withheld => 2,
        }
    }

    /// Returns true when this trust state is fully active.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }
}

/// Derived pillar verdicts for the effective-settings contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveSettingsPillars {
    /// Every required M5 family has at least one row.
    pub families_complete: bool,
    /// Every row carries an explicit winning scope and shadow chain.
    pub shadow_chains_explicit: bool,
    /// Every row carries an explicit source/effective/live review sheet.
    pub review_sheets_explicit: bool,
    /// Every high-impact row carries a scope-explicit, checkpointed write preview.
    pub high_impact_checkpointed: bool,
    /// Every constrained/locked row explains its governing bundle/scope.
    pub policy_explanations_complete: bool,
    /// Every lifecycle dependency is a visible marker with a recovery hint.
    pub lifecycle_dependencies_visible: bool,
    /// Every required family has an admin-distribution audit row.
    pub distribution_audit_complete: bool,
    /// No row advertises trust above what validation and locks allow.
    pub validation_honest: bool,
    /// All required surfaces render the same record.
    pub surface_truth_complete: bool,
}

/// Reason a record is narrowed below the fully-active claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// One or more required M5 families have no row.
    FamiliesIncomplete,
    /// A row has an implicit or contradictory shadow chain.
    ShadowChainImplicit,
    /// A row hides or omits its source/effective/live review sheet.
    ReviewSheetImplicit,
    /// A high-impact row lacks a scope-explicit, checkpointed write preview.
    HighImpactUncheckpointed,
    /// A constrained/locked row omits its governing explanation.
    PolicyExplanationIncomplete,
    /// A lifecycle dependency is hidden rather than a visible marker.
    LifecycleDependencyHidden,
    /// The admin-distribution audit omits a required family or source fact.
    DistributionAuditIncomplete,
    /// A row advertises trust above what validation and locks allow.
    ValidationDishonest,
    /// At least one row resolves below fully-active trust.
    RowTrustBelowActive,
    /// One or more surfaces omit required effective-settings truth.
    SurfaceTruthIncomplete,
}

/// Public claim class derived from the effective-settings evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectiveSettingsClaim {
    /// Every row resolves to a fully-active, trusted value and every pillar holds.
    FullyActive,
    /// Resolution is sound but at least one row is narrowed or withheld.
    NarrowedActive,
    /// A structural pillar failed; the record is not safely usable as-is.
    Unsupported,
}

/// Derived trust verdict for the whole record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveSettingsQualification {
    /// Derived claim class.
    pub claim_class: EffectiveSettingsClaim,
    /// Weakest row trust across all rows.
    pub effective_trust_ceiling: RowTrust,
    /// Whether the record qualifies for a fully-active claim.
    pub qualifies_fully_active: bool,
    /// Named narrowing reasons.
    pub narrowing_reasons: Vec<NarrowingReason>,
}

/// Input used to build a [`M5EffectiveSettingsCertification`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EffectiveSettingsInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// Effective-settings rows for M5-added settings.
    pub setting_rows: Vec<M5EffectiveSettingRow>,
    /// Admin-distribution audit rows.
    pub distribution_audit: Vec<AdminDistributionAuditRow>,
    /// Surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
}

/// Canonical effective-settings record for M5-added settings families.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EffectiveSettingsCertification {
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
    /// Effective-settings rows for M5-added settings.
    pub setting_rows: Vec<M5EffectiveSettingRow>,
    /// Admin-distribution audit rows.
    pub distribution_audit: Vec<AdminDistributionAuditRow>,
    /// Surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
    /// M5 families covered by the rows.
    pub family_coverage: Vec<M5SettingFamily>,
    /// Restart postures covered by the rows.
    pub restart_posture_coverage: Vec<RestartPosture>,
    /// Validation states covered by the rows.
    pub validation_state_coverage: Vec<ValidationState>,
    /// Lifecycle-dependency kinds surfaced as markers.
    pub lifecycle_dependency_coverage: Vec<LifecycleDependencyKind>,
    /// High-impact classes surfaced by the rows.
    pub high_impact_coverage: Vec<HighImpactClass>,
    /// Source/effective/live projection modes surfaced by rows or audit.
    pub projection_mode_coverage: Vec<ProjectionMode>,
    /// Constraint states surfaced by rows.
    pub constraint_state_coverage: Vec<PolicyConstraintState>,
    /// Distribution sources surfaced by audit rows.
    pub distribution_source_coverage: Vec<PolicyDistributionSource>,
    /// Derived pillar verdicts.
    pub pillars: EffectiveSettingsPillars,
    /// Derived trust qualification.
    pub trust_qualification: EffectiveSettingsQualification,
}

/// Reasons an effective-settings certification cannot be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// No setting rows were supplied.
    NoSettingRows,
    /// No distribution-audit rows were supplied.
    NoDistributionAuditRows,
    /// A required M5 family has no row.
    MissingFamily {
        /// The family with no row.
        family: M5SettingFamily,
    },
    /// A required M5 family has no distribution-audit row.
    MissingDistributionAudit {
        /// The family with no audit row.
        family: M5SettingFamily,
    },
    /// A setting id is used by more than one row.
    DuplicateSetting {
        /// The duplicated setting id.
        setting_id: String,
    },
    /// A canonical ref field is invalid.
    NonCanonicalRef {
        /// The field carrying the invalid ref.
        field: &'static str,
        /// The offending value.
        value: String,
    },
    /// A high-impact row is missing its scope-explicit write preview.
    HighImpactWithoutWritePreview {
        /// The under-specified setting id.
        setting_id: String,
    },
    /// A high-impact row's write preview lacks a rollback checkpoint.
    HighImpactWriteWithoutCheckpoint {
        /// The under-specified setting id.
        setting_id: String,
    },
    /// A high-impact row's write preview does not require confirmation.
    HighImpactWriteNotConfirmed {
        /// The under-specified setting id.
        setting_id: String,
    },
    /// A constrained or locked row is missing its policy ref.
    PolicyConstrainedWithoutRef {
        /// The under-specified setting id.
        setting_id: String,
    },
    /// A constrained or locked row does not name the source bundle or scope.
    PolicyConstrainedWithoutSource {
        /// The under-specified setting id.
        setting_id: String,
    },
    /// A constrained or locked row omits its local-safe continuation facts.
    PolicyConstrainedWithoutLocalSafe {
        /// The under-specified setting id.
        setting_id: String,
    },
    /// A policy-locked row advertises a write that would silently win.
    WriteEffectContradictsLock {
        /// The dishonest setting id.
        setting_id: String,
    },
    /// A constrained or denied write omitted its explanation block.
    MissingWriteExplanation {
        /// The under-specified setting id.
        setting_id: String,
    },
    /// A row's review sheet does not include the setting id.
    ReviewSheetMissingSettingId {
        /// The under-specified setting id.
        setting_id: String,
    },
    /// A row's review sheet omits the active projection mode from the switcher.
    ReviewSheetMissingProjectionMode {
        /// The under-specified setting id.
        setting_id: String,
    },
    /// A row's winning scope also appears in its shadow chain.
    WinningScopeShadowed {
        /// The contradictory setting id.
        setting_id: String,
    },
    /// A lifecycle-dependency marker is hidden rather than visible.
    LifecycleMarkerHidden {
        /// The setting id carrying the hidden marker.
        setting_id: String,
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
            Self::NoSettingRows => write!(f, "at least one setting row is required"),
            Self::NoDistributionAuditRows => {
                write!(f, "at least one admin-distribution audit row is required")
            }
            Self::MissingFamily { family } => {
                write!(f, "missing M5 family `{}`", family.as_str())
            }
            Self::MissingDistributionAudit { family } => {
                write!(
                    f,
                    "missing admin-distribution audit row for family `{}`",
                    family.as_str()
                )
            }
            Self::DuplicateSetting { setting_id } => {
                write!(f, "duplicated setting id `{setting_id}`")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field `{field}` must be a canonical ref, got {value:?}")
            }
            Self::HighImpactWithoutWritePreview { setting_id } => write!(
                f,
                "high-impact setting `{setting_id}` requires a scope-explicit write preview"
            ),
            Self::HighImpactWriteWithoutCheckpoint { setting_id } => write!(
                f,
                "high-impact setting `{setting_id}` requires a rollback checkpoint"
            ),
            Self::HighImpactWriteNotConfirmed { setting_id } => write!(
                f,
                "high-impact setting `{setting_id}` write must require confirmation"
            ),
            Self::PolicyConstrainedWithoutRef { setting_id } => write!(
                f,
                "constrained/locked setting `{setting_id}` requires a policy ref"
            ),
            Self::PolicyConstrainedWithoutSource { setting_id } => write!(
                f,
                "constrained/locked setting `{setting_id}` must name a source bundle or scope"
            ),
            Self::PolicyConstrainedWithoutLocalSafe { setting_id } => write!(
                f,
                "constrained/locked setting `{setting_id}` must publish what still works locally"
            ),
            Self::WriteEffectContradictsLock { setting_id } => write!(
                f,
                "policy-locked setting `{setting_id}` cannot preview a winning write"
            ),
            Self::MissingWriteExplanation { setting_id } => write!(
                f,
                "setting `{setting_id}` must explain constrained or denied writes"
            ),
            Self::ReviewSheetMissingSettingId { setting_id } => write!(
                f,
                "review sheet for setting `{setting_id}` must include the setting id"
            ),
            Self::ReviewSheetMissingProjectionMode { setting_id } => write!(
                f,
                "review sheet for setting `{setting_id}` must label the active projection mode"
            ),
            Self::WinningScopeShadowed { setting_id } => write!(
                f,
                "setting `{setting_id}` lists its winning scope in the shadow chain"
            ),
            Self::LifecycleMarkerHidden { setting_id } => write!(
                f,
                "setting `{setting_id}` carries a hidden lifecycle-dependency marker"
            ),
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

fn validate_optional_ref(field: &'static str, value: &Option<String>) -> Result<(), BuildError> {
    if let Some(value) = value {
        require_ref(field, value)?;
    }
    Ok(())
}

fn validate_write_explanation(
    setting_id: &str,
    explanation: &WriteConstraintExplanation,
) -> Result<(), BuildError> {
    validate_optional_ref(
        "setting_rows.write_preview.explanation.source_bundle_ref",
        &explanation.source_bundle_ref,
    )?;
    validate_optional_ref(
        "setting_rows.write_preview.explanation.source_scope_ref",
        &explanation.source_scope_ref,
    )?;
    validate_optional_ref(
        "setting_rows.write_preview.explanation.bundle_owner_ref",
        &explanation.bundle_owner_ref,
    )?;
    validate_optional_ref(
        "setting_rows.write_preview.explanation.review_ref",
        &explanation.review_ref,
    )?;
    if explanation.local_safe_continuation.is_empty() {
        return Err(BuildError::PolicyConstrainedWithoutLocalSafe {
            setting_id: setting_id.to_owned(),
        });
    }
    Ok(())
}

impl M5EffectiveSettingsCertification {
    /// Builds a derived effective-settings certification from raw rows.
    ///
    /// Returns a [`BuildError`] when a structural invariant or a fail-closed
    /// guardrail is violated, so a record that hides a high-impact write behind
    /// an opaque toggle, omits its active projection mode, or buries a policy
    /// denial behind copy-only prose cannot be constructed.
    pub fn build(mut input: M5EffectiveSettingsInput) -> Result<Self, BuildError> {
        if input.setting_rows.is_empty() {
            return Err(BuildError::NoSettingRows);
        }
        if input.distribution_audit.is_empty() {
            return Err(BuildError::NoDistributionAuditRows);
        }

        let mut seen_settings = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &input.setting_rows {
            if !seen_settings.insert(row.setting_id.clone()) {
                return Err(BuildError::DuplicateSetting {
                    setting_id: row.setting_id.clone(),
                });
            }
            seen_families.insert(row.family);

            require_ref(
                "setting_rows.winning_value.value_ref",
                &row.winning_value.value_ref,
            )?;
            for shadow in &row.shadow_chain {
                require_ref("setting_rows.shadow_chain.value_ref", &shadow.value_ref)?;
                if shadow.scope == row.winning_value.scope {
                    return Err(BuildError::WinningScopeShadowed {
                        setting_id: row.setting_id.clone(),
                    });
                }
            }

            if !row
                .effective_value_review
                .selected_keys
                .iter()
                .any(|key| key == &row.setting_id)
            {
                return Err(BuildError::ReviewSheetMissingSettingId {
                    setting_id: row.setting_id.clone(),
                });
            }
            if !row
                .effective_value_review
                .available_projection_modes
                .contains(&row.effective_value_review.active_projection_mode)
            {
                return Err(BuildError::ReviewSheetMissingProjectionMode {
                    setting_id: row.setting_id.clone(),
                });
            }

            if row.policy_lock.constraint_state.narrows() {
                match &row.policy_lock.policy_ref {
                    Some(policy_ref) => {
                        require_ref("setting_rows.policy_lock.policy_ref", policy_ref)?
                    }
                    None => {
                        return Err(BuildError::PolicyConstrainedWithoutRef {
                            setting_id: row.setting_id.clone(),
                        })
                    }
                }

                if row.policy_lock.source_bundle_ref.is_none()
                    && row.policy_lock.source_scope_ref.is_none()
                {
                    return Err(BuildError::PolicyConstrainedWithoutSource {
                        setting_id: row.setting_id.clone(),
                    });
                }
                validate_optional_ref(
                    "setting_rows.policy_lock.source_bundle_ref",
                    &row.policy_lock.source_bundle_ref,
                )?;
                validate_optional_ref(
                    "setting_rows.policy_lock.source_scope_ref",
                    &row.policy_lock.source_scope_ref,
                )?;
                validate_optional_ref(
                    "setting_rows.policy_lock.bundle_owner_ref",
                    &row.policy_lock.bundle_owner_ref,
                )?;
                if row.policy_lock.local_safe_continuation.is_empty() {
                    return Err(BuildError::PolicyConstrainedWithoutLocalSafe {
                        setting_id: row.setting_id.clone(),
                    });
                }
            }

            if let Some(marker) = &row.lifecycle_dependency {
                require_ref(
                    "setting_rows.lifecycle_dependency.depends_on_ref",
                    &marker.depends_on_ref,
                )?;
                if !marker.visible {
                    return Err(BuildError::LifecycleMarkerHidden {
                        setting_id: row.setting_id.clone(),
                    });
                }
            }

            if let Some(preview) = &row.write_preview {
                require_ref(
                    "setting_rows.write_preview.current_value_ref",
                    &preview.current_value_ref,
                )?;
                require_ref(
                    "setting_rows.write_preview.proposed_value_ref",
                    &preview.proposed_value_ref,
                )?;
                if !preview.rollback_checkpoint_ref.is_empty() {
                    require_ref(
                        "setting_rows.write_preview.rollback_checkpoint_ref",
                        &preview.rollback_checkpoint_ref,
                    )?;
                }
                if row.policy_lock.constraint_state.is_locked()
                    && preview.effective_after_write == WriteEffect::BecomesWinningValue
                {
                    return Err(BuildError::WriteEffectContradictsLock {
                        setting_id: row.setting_id.clone(),
                    });
                }
                if preview.effective_after_write != WriteEffect::BecomesWinningValue {
                    let Some(explanation) = &preview.explanation else {
                        return Err(BuildError::MissingWriteExplanation {
                            setting_id: row.setting_id.clone(),
                        });
                    };
                    validate_write_explanation(&row.setting_id, explanation)?;
                }
            }

            if row.is_high_impact() {
                let Some(preview) = &row.write_preview else {
                    return Err(BuildError::HighImpactWithoutWritePreview {
                        setting_id: row.setting_id.clone(),
                    });
                };
                if !is_canonical_object_ref(&preview.rollback_checkpoint_ref) {
                    return Err(BuildError::HighImpactWriteWithoutCheckpoint {
                        setting_id: row.setting_id.clone(),
                    });
                }
                if !preview.requires_confirmation {
                    return Err(BuildError::HighImpactWriteNotConfirmed {
                        setting_id: row.setting_id.clone(),
                    });
                }
            }
        }

        let mut audit_families = BTreeSet::new();
        for audit in &input.distribution_audit {
            audit_families.insert(audit.family);
            require_ref("distribution_audit.bundle_ref", &audit.bundle_ref)?;
            require_ref(
                "distribution_audit.bundle_owner_ref",
                &audit.bundle_owner_ref,
            )?;
            require_ref(
                "distribution_audit.policy_scope_ref",
                &audit.policy_scope_ref,
            )?;
            if audit.local_safe_continuation.is_empty() {
                return Err(BuildError::PolicyConstrainedWithoutLocalSafe {
                    setting_id: audit.audit_id.clone(),
                });
            }
        }

        for family in M5SettingFamily::REQUIRED {
            if !seen_families.contains(&family) {
                return Err(BuildError::MissingFamily { family });
            }
            if !audit_families.contains(&family) {
                return Err(BuildError::MissingDistributionAudit { family });
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
            .setting_rows
            .sort_by(|a, b| a.setting_id.cmp(&b.setting_id));
        input
            .distribution_audit
            .sort_by(|a, b| a.audit_id.cmp(&b.audit_id));
        input.surface_truth.sort_by_key(|row| row.surface_class);

        let family_coverage = collect_sorted(input.setting_rows.iter().map(|row| row.family));
        let restart_posture_coverage =
            collect_sorted(input.setting_rows.iter().map(|row| row.restart_posture));
        let validation_state_coverage =
            collect_sorted(input.setting_rows.iter().map(|row| row.validation_state));
        let lifecycle_dependency_coverage = collect_sorted(
            input
                .setting_rows
                .iter()
                .filter_map(|row| row.lifecycle_dependency.as_ref().map(|m| m.kind)),
        );
        let high_impact_coverage = collect_sorted(
            input
                .setting_rows
                .iter()
                .filter_map(|row| row.high_impact_class),
        );
        let projection_mode_coverage = collect_sorted(
            input
                .setting_rows
                .iter()
                .map(|row| row.effective_value_review.active_projection_mode)
                .chain(
                    input
                        .distribution_audit
                        .iter()
                        .map(|audit| audit.active_projection_mode),
                ),
        );
        let constraint_state_coverage = collect_sorted(
            input
                .setting_rows
                .iter()
                .map(|row| row.policy_lock.constraint_state),
        );
        let distribution_source_coverage = collect_sorted(
            input
                .distribution_audit
                .iter()
                .map(|audit| audit.distribution_source),
        );

        let families_complete = M5SettingFamily::REQUIRED
            .iter()
            .all(|family| seen_families.contains(family));

        let shadow_chains_explicit = input.setting_rows.iter().all(|row| {
            !row.winning_value.display.trim().is_empty()
                && !row.title.trim().is_empty()
                && row
                    .shadow_chain
                    .iter()
                    .all(|shadow| shadow.scope != row.winning_value.scope)
        });

        let review_sheets_explicit = input.setting_rows.iter().all(|row| {
            !row.effective_value_review.selected_keys.is_empty()
                && row
                    .effective_value_review
                    .selected_keys
                    .iter()
                    .any(|key| key == &row.setting_id)
                && !row
                    .effective_value_review
                    .available_projection_modes
                    .is_empty()
                && row
                    .effective_value_review
                    .available_projection_modes
                    .contains(&row.effective_value_review.active_projection_mode)
                && !row.effective_value_review.winning_layers.is_empty()
                && !row.effective_value_review.available_actions.is_empty()
        });

        let high_impact_checkpointed = input.setting_rows.iter().all(|row| {
            if !row.is_high_impact() {
                return true;
            }
            row.write_preview.as_ref().is_some_and(|preview| {
                is_canonical_object_ref(&preview.rollback_checkpoint_ref)
                    && preview.requires_confirmation
            })
        });

        let policy_explanations_complete = input.setting_rows.iter().all(|row| {
            if !row.policy_lock.constraint_state.narrows() {
                return true;
            }

            let source_declared = row.policy_lock.source_bundle_ref.is_some()
                || row.policy_lock.source_scope_ref.is_some();
            let write_explained = row.write_preview.as_ref().is_none_or(|preview| {
                if preview.effective_after_write == WriteEffect::BecomesWinningValue {
                    true
                } else {
                    preview
                        .explanation
                        .as_ref()
                        .is_some_and(|explanation| !explanation.local_safe_continuation.is_empty())
                }
            });

            row.policy_lock.policy_ref.is_some()
                && source_declared
                && !row.policy_lock.local_safe_continuation.is_empty()
                && write_explained
        });

        let lifecycle_dependencies_visible =
            input
                .setting_rows
                .iter()
                .all(|row| match &row.lifecycle_dependency {
                    Some(marker) => marker.visible && !marker.recovery_hint.trim().is_empty(),
                    None => true,
                });

        let distribution_audit_complete = M5SettingFamily::REQUIRED.iter().all(|family| {
            input.distribution_audit.iter().any(|audit| {
                audit.family == *family
                    && !audit.local_safe_continuation.is_empty()
                    && !audit.last_applied_at.trim().is_empty()
                    && !audit.last_validated_at.trim().is_empty()
            })
        });

        let validation_honest = input.setting_rows.iter().all(|row| {
            !(row.policy_lock.constraint_state.is_locked()
                && row
                    .write_preview
                    .as_ref()
                    .is_some_and(|p| p.effective_after_write == WriteEffect::BecomesWinningValue))
        });

        let surface_truth_complete = input.surface_truth.iter().all(|row| {
            row.consumes_shared_record
                && row.shows_winning_scope
                && row.shows_shadow_chain
                && row.shows_restart_posture
                && row.shows_projection_mode
                && row.shows_lifecycle_dependency
                && row.shows_write_preview
                && row.shows_write_explanation
                && row.shows_distribution_audit
                && row.shows_last_applied
                && row.shows_local_safe_continuation
        });

        let effective_trust_ceiling = input
            .setting_rows
            .iter()
            .map(M5EffectiveSettingRow::row_trust)
            .max_by_key(|trust| trust.rank())
            .unwrap_or(RowTrust::Active);

        let pillars = EffectiveSettingsPillars {
            families_complete,
            shadow_chains_explicit,
            review_sheets_explicit,
            high_impact_checkpointed,
            policy_explanations_complete,
            lifecycle_dependencies_visible,
            distribution_audit_complete,
            validation_honest,
            surface_truth_complete,
        };

        let mut narrowing_reasons = Vec::new();
        if !pillars.families_complete {
            narrowing_reasons.push(NarrowingReason::FamiliesIncomplete);
        }
        if !pillars.shadow_chains_explicit {
            narrowing_reasons.push(NarrowingReason::ShadowChainImplicit);
        }
        if !pillars.review_sheets_explicit {
            narrowing_reasons.push(NarrowingReason::ReviewSheetImplicit);
        }
        if !pillars.high_impact_checkpointed {
            narrowing_reasons.push(NarrowingReason::HighImpactUncheckpointed);
        }
        if !pillars.policy_explanations_complete {
            narrowing_reasons.push(NarrowingReason::PolicyExplanationIncomplete);
        }
        if !pillars.lifecycle_dependencies_visible {
            narrowing_reasons.push(NarrowingReason::LifecycleDependencyHidden);
        }
        if !pillars.distribution_audit_complete {
            narrowing_reasons.push(NarrowingReason::DistributionAuditIncomplete);
        }
        if !pillars.validation_honest {
            narrowing_reasons.push(NarrowingReason::ValidationDishonest);
        }
        if !effective_trust_ceiling.is_active() {
            narrowing_reasons.push(NarrowingReason::RowTrustBelowActive);
        }
        if !pillars.surface_truth_complete {
            narrowing_reasons.push(NarrowingReason::SurfaceTruthIncomplete);
        }

        let structural_ok = pillars.families_complete
            && pillars.shadow_chains_explicit
            && pillars.review_sheets_explicit
            && pillars.high_impact_checkpointed
            && pillars.policy_explanations_complete
            && pillars.lifecycle_dependencies_visible
            && pillars.distribution_audit_complete
            && pillars.validation_honest
            && pillars.surface_truth_complete;

        let qualifies_fully_active = structural_ok && effective_trust_ceiling.is_active();

        let claim_class = if !structural_ok {
            EffectiveSettingsClaim::Unsupported
        } else if qualifies_fully_active {
            EffectiveSettingsClaim::FullyActive
        } else {
            EffectiveSettingsClaim::NarrowedActive
        };

        let trust_qualification = EffectiveSettingsQualification {
            claim_class,
            effective_trust_ceiling,
            qualifies_fully_active,
            narrowing_reasons,
        };

        Ok(Self {
            record_kind: M5_EFFECTIVE_SETTINGS_RECORD_KIND.to_owned(),
            schema_version: M5_EFFECTIVE_SETTINGS_SCHEMA_VERSION,
            shared_contract_ref: M5_EFFECTIVE_SETTINGS_SHARED_CONTRACT_REF.to_owned(),
            record_id: input.record_id,
            as_of: input.as_of,
            summary: input.summary,
            setting_rows: input.setting_rows,
            distribution_audit: input.distribution_audit,
            surface_truth: input.surface_truth,
            family_coverage,
            restart_posture_coverage,
            validation_state_coverage,
            lifecycle_dependency_coverage,
            high_impact_coverage,
            projection_mode_coverage,
            constraint_state_coverage,
            distribution_source_coverage,
            pillars,
            trust_qualification,
        })
    }

    /// Renders a compact, export-safe support summary from the shared record.
    pub fn support_export_lines(&self) -> Vec<String> {
        let locked_rows = self
            .setting_rows
            .iter()
            .filter(|row| row.policy_lock.constraint_state == PolicyConstraintState::Locked)
            .count();
        let constrained_rows = self
            .setting_rows
            .iter()
            .filter(|row| row.policy_lock.constraint_state == PolicyConstraintState::Constrained)
            .count();
        let stale_audit_rows = self
            .distribution_audit
            .iter()
            .filter(|row| row.freshness_state.is_stale())
            .count();

        vec![
            format!("record_id: {}", self.record_id),
            format!("claim: {:?}", self.trust_qualification.claim_class),
            format!(
                "trust_ceiling: {}",
                self.trust_qualification.effective_trust_ceiling.as_str()
            ),
            format!("setting_rows: {}", self.setting_rows.len()),
            format!("families: {}", self.family_coverage.len()),
            format!("high_impact_rows: {}", self.high_impact_coverage.len()),
            format!("locked_rows: {locked_rows}"),
            format!("constrained_rows: {constrained_rows}"),
            format!("projection_modes: {}", self.projection_mode_coverage.len()),
            format!("distribution_audit_rows: {}", self.distribution_audit.len()),
            format!("stale_distribution_rows: {stale_audit_rows}"),
        ]
    }
}

fn collect_sorted<T: Ord>(values: impl Iterator<Item = T>) -> Vec<T> {
    values.collect::<BTreeSet<_>>().into_iter().collect()
}
