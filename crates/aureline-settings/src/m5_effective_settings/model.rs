//! Governed effective-settings record for M5-added settings families.
//!
//! This module is the settings-lane contract that brings new M5 settings
//! families — notebooks, data/API, profiler, bundle, sync, and companion — into
//! parity with the stable effective-settings resolver. It does not re-implement
//! the precedence engine; it defines the canonical record those families must
//! emit so the settings UI, CLI inspect, help links, policy explainers, and
//! support bundles all answer the same four questions for an M5 row: which value
//! is active, which scope won (with the shadow chain that lost), what restart
//! posture applies, and whether a lifecycle-sensitive dependency narrows the
//! behavior.
//!
//! The gate is fail-closed. A high-impact M5 setting — one that materially
//! changes trust, AI/network egress, extension behavior, remote exposure, or a
//! destructive-automation default — can never be recorded without a
//! scope-explicit write preview and a rollback checkpoint, and a policy-locked
//! row can never advertise a write that would silently win against the lock.
//! Both are build-time invariants, so a record that hides non-stable or
//! policy-gated behavior behind an opaque toggle cannot be constructed.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for M5 effective-settings records.
pub const M5_EFFECTIVE_SETTINGS_RECORD_KIND: &str = "m5_effective_settings_certification_record";

/// Schema version for [`M5EffectiveSettingsCertification`] records.
pub const M5_EFFECTIVE_SETTINGS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by settings UI, CLI, help, policy, and support.
pub const M5_EFFECTIVE_SETTINGS_SHARED_CONTRACT_REF: &str = "settings:m5_effective_settings:v1";

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
///
/// Ordinary scopes contribute layered value candidates; `AdminPolicyNarrowing`
/// is a ceiling that may only narrow, lock, or constrain a layered winner.
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

    /// Returns true for an admin policy ceiling rather than an ordinary scope.
    pub const fn is_policy_ceiling(self) -> bool {
        matches!(self, Self::AdminPolicyNarrowing)
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
///
/// A row carrying any of these requires a scope-explicit write preview and a
/// rollback checkpoint; the gate refuses to build a record otherwise.
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
    /// Policy explainer surface.
    PolicyExplainer,
    /// Support bundle / support-center export.
    SupportBundle,
}

impl SurfaceClass {
    /// Required surface set for parity.
    pub const REQUIRED: [Self; 5] = [
        Self::SettingsUi,
        Self::CliInspect,
        Self::HelpLinks,
        Self::PolicyExplainer,
        Self::SupportBundle,
    ];
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

/// Policy-lock state applied to an M5 setting row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyLockState {
    /// Whether an admin policy locks this setting.
    pub locked: bool,
    /// Canonical ref to the policy that imposes the lock, when locked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_ref: Option<String>,
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
    /// Policy-lock state for this row.
    pub policy_lock: PolicyLockState,
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
            || self.policy_lock.locked
            || self.lifecycle_dependency.is_some()
        {
            return RowTrust::Narrowed;
        }
        RowTrust::Active
    }
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
    /// Whether the surface shows the lifecycle-dependency marker.
    pub shows_lifecycle_dependency: bool,
    /// Whether the surface shows the scope-explicit write preview.
    pub shows_write_preview: bool,
}

/// Derived trust state for one M5 setting row.
///
/// Ordered from [`Self::Active`] (best) to [`Self::Withheld`] (weakest); the
/// record publishes the weakest row trust as its effective ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowTrust {
    /// The value validates and no dependency or lock narrows it.
    Active,
    /// The value resolves but a lock, coercion, or lifecycle dependency narrows it.
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
    /// Every high-impact row carries a scope-explicit, checkpointed write preview.
    pub high_impact_checkpointed: bool,
    /// Every lifecycle dependency is a visible marker with a recovery hint.
    pub lifecycle_dependencies_visible: bool,
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
    /// A high-impact row lacks a scope-explicit, checkpointed write preview.
    HighImpactUncheckpointed,
    /// A lifecycle dependency is hidden rather than a visible marker.
    LifecycleDependencyHidden,
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
    /// A required M5 family has no row.
    MissingFamily {
        /// The family with no row.
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
    /// A policy-locked row is missing its policy ref.
    PolicyLockedWithoutRef {
        /// The under-specified setting id.
        setting_id: String,
    },
    /// A policy-locked row advertises a write that would silently win.
    WriteEffectContradictsLock {
        /// The dishonest setting id.
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
            Self::MissingFamily { family } => {
                write!(f, "missing M5 family `{}`", family.as_str())
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
            Self::PolicyLockedWithoutRef { setting_id } => write!(
                f,
                "policy-locked setting `{setting_id}` requires a policy ref"
            ),
            Self::WriteEffectContradictsLock { setting_id } => write!(
                f,
                "policy-locked setting `{setting_id}` cannot preview a winning write"
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

impl M5EffectiveSettingsCertification {
    /// Builds a derived effective-settings certification from raw rows.
    ///
    /// Returns a [`BuildError`] when a structural invariant or a fail-closed
    /// guardrail is violated, so a record that hides a high-impact write behind
    /// an opaque toggle or advertises a winning write against a policy lock
    /// cannot be constructed.
    pub fn build(mut input: M5EffectiveSettingsInput) -> Result<Self, BuildError> {
        if input.setting_rows.is_empty() {
            return Err(BuildError::NoSettingRows);
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

            if row.policy_lock.locked {
                match &row.policy_lock.policy_ref {
                    Some(policy_ref) => {
                        require_ref("setting_rows.policy_lock.policy_ref", policy_ref)?
                    }
                    None => {
                        return Err(BuildError::PolicyLockedWithoutRef {
                            setting_id: row.setting_id.clone(),
                        })
                    }
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
                // A policy-locked row can never preview a write that silently wins.
                if row.policy_lock.locked
                    && preview.effective_after_write == WriteEffect::BecomesWinningValue
                {
                    return Err(BuildError::WriteEffectContradictsLock {
                        setting_id: row.setting_id.clone(),
                    });
                }
            }

            // High-impact rows are fail-closed: scope-explicit, checkpointed, confirmed.
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

        for family in M5SettingFamily::REQUIRED {
            if !seen_families.contains(&family) {
                return Err(BuildError::MissingFamily { family });
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

        let high_impact_checkpointed = input.setting_rows.iter().all(|row| {
            if !row.is_high_impact() {
                return true;
            }
            row.write_preview.as_ref().is_some_and(|preview| {
                is_canonical_object_ref(&preview.rollback_checkpoint_ref)
                    && preview.requires_confirmation
            })
        });

        let lifecycle_dependencies_visible =
            input
                .setting_rows
                .iter()
                .all(|row| match &row.lifecycle_dependency {
                    Some(marker) => marker.visible && !marker.recovery_hint.trim().is_empty(),
                    None => true,
                });

        // No row may advertise a winning write while policy-locked; the build
        // above rejects that, so this pillar reflects the same invariant.
        let validation_honest = input.setting_rows.iter().all(|row| {
            !(row.policy_lock.locked
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
                && row.shows_lifecycle_dependency
                && row.shows_write_preview
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
            high_impact_checkpointed,
            lifecycle_dependencies_visible,
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
        if !pillars.high_impact_checkpointed {
            narrowing_reasons.push(NarrowingReason::HighImpactUncheckpointed);
        }
        if !pillars.lifecycle_dependencies_visible {
            narrowing_reasons.push(NarrowingReason::LifecycleDependencyHidden);
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
            && pillars.high_impact_checkpointed
            && pillars.lifecycle_dependencies_visible
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
            surface_truth: input.surface_truth,
            family_coverage,
            restart_posture_coverage,
            validation_state_coverage,
            lifecycle_dependency_coverage,
            high_impact_coverage,
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
            format!("setting_rows: {}", self.setting_rows.len()),
            format!("families: {}", self.family_coverage.len()),
            format!("high_impact_rows: {}", self.high_impact_coverage.len()),
            format!(
                "lifecycle_dependencies: {}",
                self.lifecycle_dependency_coverage.len()
            ),
        ]
    }
}

fn collect_sorted<T: Ord>(values: impl Iterator<Item = T>) -> Vec<T> {
    values.collect::<BTreeSet<_>>().into_iter().collect()
}
