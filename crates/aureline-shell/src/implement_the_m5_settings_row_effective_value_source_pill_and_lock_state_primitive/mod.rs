//! One reusable M5 settings-row primitive: effective value, source pill, and
//! lock state, plus view-diff and source-detail parity across every M5
//! config-bearing surface.
//!
//! Aureline's frozen component matrix
//! ([`crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix`])
//! names the settings row as one governed component family and freezes its
//! controlled state vocabulary and source pills. This module *implements* that
//! settings-row contract as one reusable primitive so effective value, configured
//! value, source scope, lock reason, and diff / open-source-detail behavior stay
//! consistent instead of drifting by screen.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_settings_row`] — that takes the per-source
//!    contributions for one setting and produces one [`M5ResolvedSettingsRow`]
//!    carrying the effective value, the winning source pill, the retained
//!    user-configured value, the typed [`M5SettingsRowState`], the lock source,
//!    and the shadow chain. The resolver never confuses a user-authored value with
//!    the effective value and never hides the user-configured value when a higher
//!    source or policy wins.
//! 2. A parity matrix — [`M5SettingsRowPrimitivePacket`] — that binds one row per
//!    claimed M5 config-bearing surface family (admin, trust, AI, network,
//!    execution, extension, and update/config) to the shared anatomy, the same
//!    settings-row states and source pills, the same lock-disclosure and focus
//!    behaviors, and the same export fields, so the support / export packet can
//!    reconstruct effective-value truth from one shared model on every surface.
//!
//! The controlled state vocabulary ([`M5SettingsRowState`]), the source pills
//! ([`M5SettingSourcePill`]), the non-visual accessibility routes
//! ([`M5TrustAccessibilityRoute`]), the qualification classes
//! ([`M5TrustQualificationClass`]), and the downgrade triggers
//! ([`M5TrustComponentDowngradeTrigger`]) are reused verbatim from the frozen
//! component matrix; the shell topology — zones, responsive classes, window
//! classes, and consumer surfaces — is reused from the frozen shell-zone matrix.
//! This module mints new vocabulary only for what the frozen matrix left implicit
//! about the settings row itself: its anatomy parts, its lock disclosures, its
//! focus behaviors, and its export fields. No M5 surface invents a second settings
//! row grammar.
//!
//! Raw URLs, raw local paths, raw usernames, raw hostnames, tokens, credentials,
//! and user text bodies stay outside the support boundary; managed values are
//! carried as the redaction token [`M5_SETTINGS_REDACTED_VALUE_REPR`] only.
//!
//! The boundary schema is
//! [`schemas/ui/m5-settings-row.schema.json`](../../../../schemas/ui/m5-settings-row.schema.json)
//! and the contract doc is
//! [`docs/components/m5_settings_row_primitive_contract.md`](../../../../docs/components/m5_settings_row_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-settings-row-primitive/`](../../../../fixtures/ui/m5-settings-row-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_settings_row_primitive_admin_enterprise_beta_narrowed,
    seeded_m5_settings_row_primitive_packet,
    seeded_m5_settings_row_primitive_update_channel_preview_narrowed,
    M5_SETTINGS_ROW_PRIMITIVE_PACKET_ID,
};

// The settings-row state vocabulary, source pills, accessibility routes,
// qualification classes, and downgrade triggers are frozen once, in the
// trust-chronology component matrix. This primitive reuses them verbatim so it
// never invents a parallel settings vocabulary.
pub use crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix::{
    M5SettingSourcePill, M5SettingsRowState, M5TrustAccessibilityRoute,
    M5TrustComponentDowngradeTrigger, M5TrustQualificationClass,
};

// The canonical shell topology — zones, responsive classes, window classes, and
// consumer surfaces — is frozen once, in the shell-zone matrix.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SettingsRowPrimitivePacket`].
pub const M5_SETTINGS_ROW_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_settings_row_effective_value_source_pill_and_lock_state_primitive";

/// Schema version for M5 settings-row-primitive records.
pub const M5_SETTINGS_ROW_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the settings-row-primitive boundary schema.
pub const M5_SETTINGS_ROW_SCHEMA_REF: &str = "schemas/ui/m5-settings-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SETTINGS_ROW_DOC_REF: &str = "docs/components/m5_settings_row_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds
/// against.
pub const M5_SETTINGS_ROW_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen component matrix this primitive narrows from.
pub const M5_SETTINGS_ROW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-trust-chronology-components.schema.json";

/// Repo-relative path of the effective-setting record contract this primitive
/// projects from.
pub const M5_SETTINGS_ROW_EFFECTIVE_SETTING_REF: &str =
    "schemas/settings/effective_setting.schema.json";

/// Repo-relative path of the lock-state reason contract this primitive consumes.
pub const M5_SETTINGS_ROW_LOCK_STATE_REF: &str = "schemas/settings/lock_state_reason.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SETTINGS_ROW_FIXTURE_DIR: &str = "fixtures/ui/m5-settings-row-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SETTINGS_ROW_ARTIFACT_REF: &str =
    "artifacts/release/m5-settings-row-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SETTINGS_ROW_CSV_REF: &str = "artifacts/release/m5-settings-row-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_SETTINGS_ROW_REPORT_REF: &str = "artifacts/components/m5-settings-row-primitive.md";

/// Value representation carried when a source enforces a credential-managed value
/// that must not be shown. The bare value never leaves the boundary.
pub const M5_SETTINGS_REDACTED_VALUE_REPR: &str = "redacted_managed_value";

/// One claimed M5 config-bearing surface family that renders the shared settings
/// row. These are the surfaces the goal names — admin, trust, AI, network,
/// execution, extension, and update/config.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsSurfaceFamily {
    /// Admin / enterprise settings surface.
    AdminEnterprise,
    /// Workspace / project trust settings surface.
    WorkspaceTrust,
    /// AI / model settings surface.
    AiModel,
    /// Network / proxy settings surface.
    NetworkProxy,
    /// Execution / runtime settings surface.
    ExecutionRuntime,
    /// Extension settings surface.
    ExtensionSettings,
    /// Update / config-channel settings surface.
    UpdateChannel,
}

impl M5SettingsSurfaceFamily {
    /// Every claimed config-bearing surface family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AdminEnterprise,
        Self::WorkspaceTrust,
        Self::AiModel,
        Self::NetworkProxy,
        Self::ExecutionRuntime,
        Self::ExtensionSettings,
        Self::UpdateChannel,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdminEnterprise => "admin_enterprise",
            Self::WorkspaceTrust => "workspace_trust",
            Self::AiModel => "ai_model",
            Self::NetworkProxy => "network_proxy",
            Self::ExecutionRuntime => "execution_runtime",
            Self::ExtensionSettings => "extension_settings",
            Self::UpdateChannel => "update_channel",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AdminEnterprise => "Admin / Enterprise",
            Self::WorkspaceTrust => "Workspace Trust",
            Self::AiModel => "AI / Model",
            Self::NetworkProxy => "Network / Proxy",
            Self::ExecutionRuntime => "Execution / Runtime",
            Self::ExtensionSettings => "Extension Settings",
            Self::UpdateChannel => "Update / Config Channel",
        }
    }
}

/// One anatomy part the shared settings row surfaces. The first five in
/// [`M5SettingsRowAnatomyPart::MANDATORY`] are required on every row; the last
/// three are the escalation affordances.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsRowAnatomyPart {
    /// The setting's stable label.
    Label,
    /// A plain-language description of what the setting does.
    PlainLanguageDescription,
    /// The value control (toggle, select, field).
    ValueControl,
    /// The source pill naming which source produced the effective value.
    SourcePill,
    /// The reset-to-default (or reset-to-inherited) action.
    ResetAction,
    /// The view-diff affordance comparing effective versus configured.
    ViewDiffAffordance,
    /// The source-detail affordance escalating to an explanation.
    SourceDetailAffordance,
    /// The open-in-JSON affordance jumping to the authoritative file.
    OpenInJsonAffordance,
}

impl M5SettingsRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Label,
        Self::PlainLanguageDescription,
        Self::ValueControl,
        Self::SourcePill,
        Self::ResetAction,
        Self::ViewDiffAffordance,
        Self::SourceDetailAffordance,
        Self::OpenInJsonAffordance,
    ];

    /// The anatomy parts every settings row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::Label,
        Self::PlainLanguageDescription,
        Self::ValueControl,
        Self::SourcePill,
        Self::ResetAction,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Label => "label",
            Self::PlainLanguageDescription => "plain_language_description",
            Self::ValueControl => "value_control",
            Self::SourcePill => "source_pill",
            Self::ResetAction => "reset_action",
            Self::ViewDiffAffordance => "view_diff_affordance",
            Self::SourceDetailAffordance => "source_detail_affordance",
            Self::OpenInJsonAffordance => "open_in_json_affordance",
        }
    }
}

/// How a locked value is disclosed. The first three in
/// [`M5SettingsLockDisclosure::MANDATORY`] are required so a locked value always
/// shows its enforced value and lock source together without hiding the
/// user-configured value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsLockDisclosure {
    /// The enforced (effective) value is shown.
    EnforcedValueShown,
    /// The lock source is named.
    LockSourceShown,
    /// The user-configured value is retained and shown, never hidden.
    UserConfiguredValueRetained,
    /// The lock reason is explained in plain language.
    LockReasonExplained,
    /// The path to request an override / exception is shown.
    OverrideRequestPath,
    /// The value is never silently hidden behind the lock.
    NoSilentValueHide,
}

impl M5SettingsLockDisclosure {
    /// Every lock disclosure, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EnforcedValueShown,
        Self::LockSourceShown,
        Self::UserConfiguredValueRetained,
        Self::LockReasonExplained,
        Self::OverrideRequestPath,
        Self::NoSilentValueHide,
    ];

    /// The lock disclosures every settings row must offer when locked.
    pub const MANDATORY: [Self; 3] = [
        Self::EnforcedValueShown,
        Self::LockSourceShown,
        Self::UserConfiguredValueRetained,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnforcedValueShown => "enforced_value_shown",
            Self::LockSourceShown => "lock_source_shown",
            Self::UserConfiguredValueRetained => "user_configured_value_retained",
            Self::LockReasonExplained => "lock_reason_explained",
            Self::OverrideRequestPath => "override_request_path",
            Self::NoSilentValueHide => "no_silent_value_hide",
        }
    }
}

/// A focus / navigation behavior the settings row supports so search landing,
/// highlight-on-open, and source-detail escalation stay consistent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsRowFocusBehavior {
    /// A search result lands focus on the row.
    SearchResultFocusLanding,
    /// The row highlights when opened from a deep link or search.
    HighlightOnOpen,
    /// Source detail escalates to a side sheet when inline explanation is
    /// insufficient.
    SourceDetailSideSheetEscalation,
    /// Inline explanation is preferred before escalating.
    InlineExplanationPreferred,
    /// Focus returns to the row after a side sheet closes.
    ReturnFocusOnClose,
    /// The row carries a stable deep-link anchor.
    DeepLinkAnchor,
}

impl M5SettingsRowFocusBehavior {
    /// Every focus behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SearchResultFocusLanding,
        Self::HighlightOnOpen,
        Self::SourceDetailSideSheetEscalation,
        Self::InlineExplanationPreferred,
        Self::ReturnFocusOnClose,
        Self::DeepLinkAnchor,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchResultFocusLanding => "search_result_focus_landing",
            Self::HighlightOnOpen => "highlight_on_open",
            Self::SourceDetailSideSheetEscalation => "source_detail_side_sheet_escalation",
            Self::InlineExplanationPreferred => "inline_explanation_preferred",
            Self::ReturnFocusOnClose => "return_focus_on_close",
            Self::DeepLinkAnchor => "deep_link_anchor",
        }
    }
}

/// A field the support / export packet carries so effective-value truth is
/// reconstructable from the shared row model. The first four in
/// [`M5SettingsRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsRowExportField {
    /// The stable setting key.
    SettingKey,
    /// The effective value representation.
    EffectiveValueRepr,
    /// The user-configured value representation, retained even when overridden.
    ConfiguredValueRepr,
    /// The winning source pill.
    WinningSourcePill,
    /// The typed settings-row state.
    RowState,
    /// The lock source pill, when locked.
    LockSourcePill,
    /// The shadow chain of contributing sources in precedence order.
    ShadowChain,
}

impl M5SettingsRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SettingKey,
        Self::EffectiveValueRepr,
        Self::ConfiguredValueRepr,
        Self::WinningSourcePill,
        Self::RowState,
        Self::LockSourcePill,
        Self::ShadowChain,
    ];

    /// The export fields every settings-row export must carry.
    pub const MANDATORY: [Self; 4] = [
        Self::SettingKey,
        Self::EffectiveValueRepr,
        Self::WinningSourcePill,
        Self::RowState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingKey => "setting_key",
            Self::EffectiveValueRepr => "effective_value_repr",
            Self::ConfiguredValueRepr => "configured_value_repr",
            Self::WinningSourcePill => "winning_source_pill",
            Self::RowState => "row_state",
            Self::LockSourcePill => "lock_source_pill",
            Self::ShadowChain => "shadow_chain",
        }
    }
}

/// Precedence rank for a source pill; a higher rank wins the effective value.
///
/// This is the resolver's precedence ladder. It is derived here rather than
/// stored on the frozen [`M5SettingSourcePill`] so the frozen vocabulary stays a
/// pure token set.
pub const fn source_precedence(pill: M5SettingSourcePill) -> u8 {
    match pill {
        M5SettingSourcePill::PolicyManaged => 5,
        M5SettingSourcePill::EnvironmentOverride => 4,
        M5SettingSourcePill::RemoteProfile => 3,
        M5SettingSourcePill::WorkspaceConfigured => 2,
        M5SettingSourcePill::UserConfigured => 1,
        M5SettingSourcePill::DefaultValue => 0,
    }
}

/// One source's contribution to a single setting, before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsSourceContribution {
    /// The source that produced this value.
    pub source: M5SettingSourcePill,
    /// The opaque, export-safe value representation.
    pub value_repr: String,
    /// True when this source enforces a lock on the row. Only non-user sources
    /// may enforce a lock.
    pub enforces_lock: bool,
}

impl M5SettingsSourceContribution {
    /// Convenience constructor for an unlocked contribution.
    pub fn new(source: M5SettingSourcePill, value_repr: &str) -> Self {
        Self {
            source,
            value_repr: value_repr.to_owned(),
            enforces_lock: false,
        }
    }

    /// Convenience constructor for a lock-enforcing contribution.
    pub fn locked(source: M5SettingSourcePill, value_repr: &str) -> Self {
        Self {
            source,
            value_repr: value_repr.to_owned(),
            enforces_lock: true,
        }
    }

    /// True when the value is the credential-managed redaction token.
    fn is_redacted(&self) -> bool {
        self.value_repr == M5_SETTINGS_REDACTED_VALUE_REPR
    }
}

/// The full input to the settings-row resolver for one setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsRowResolutionInput {
    /// The stable setting key.
    pub setting_key: String,
    /// Every source's contribution. Must contain exactly one `DefaultValue`
    /// contribution and no duplicate sources.
    pub contributions: Vec<M5SettingsSourceContribution>,
    /// True when a change is staged and pending a reload / restart to apply.
    pub pending_reload: bool,
    /// True when the configured value is invalid and the prior value is held.
    pub invalid_value_held: bool,
    /// The prior value held while an invalid value is rejected. Required when
    /// `invalid_value_held` is true.
    pub held_value_repr: Option<String>,
}

/// The resolved effective-value truth for one settings row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSettingsRow {
    /// The stable setting key.
    pub setting_key: String,
    /// The effective value representation.
    pub effective_value_repr: String,
    /// The source pill that produced the effective value.
    pub winning_source: M5SettingSourcePill,
    /// The user-configured value representation, retained even when a higher
    /// source or policy wins. `None` when the user configured nothing.
    pub configured_value_repr: Option<String>,
    /// The typed settings-row state.
    pub row_state: M5SettingsRowState,
    /// True when the row is locked.
    pub is_locked: bool,
    /// The source that enforces the lock, when locked.
    pub lock_source: Option<M5SettingSourcePill>,
    /// True when the effective value differs from the user-configured value. This
    /// drives the view-diff affordance.
    pub differs_from_configured: bool,
    /// Every contributing source, highest precedence first.
    pub shadow_chain: Vec<M5SettingSourcePill>,
}

/// Errors returned by [`resolve_settings_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SettingsResolutionError {
    /// The input carried no contributions.
    NoContributions,
    /// The input carried no `DefaultValue` contribution.
    MissingDefaultContribution,
    /// The input carried the same source more than once.
    DuplicateSource(M5SettingSourcePill),
    /// A user-authored or default source claimed to enforce a lock.
    LockOnUnprivilegedSource(M5SettingSourcePill),
    /// `invalid_value_held` was set without a `held_value_repr`.
    MissingHeldValue,
    /// The setting key was empty.
    EmptySettingKey,
    /// A value representation carried forbidden material.
    ForbiddenValueMaterial,
}

impl M5SettingsResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoContributions => "no_contributions",
            Self::MissingDefaultContribution => "missing_default_contribution",
            Self::DuplicateSource(_) => "duplicate_source",
            Self::LockOnUnprivilegedSource(_) => "lock_on_unprivileged_source",
            Self::MissingHeldValue => "missing_held_value",
            Self::EmptySettingKey => "empty_setting_key",
            Self::ForbiddenValueMaterial => "forbidden_value_material",
        }
    }
}

impl fmt::Display for M5SettingsResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "settings-row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SettingsResolutionError {}

/// True when a source may enforce a lock. User-authored and default sources
/// never lock the row.
const fn source_may_lock(pill: M5SettingSourcePill) -> bool {
    matches!(
        pill,
        M5SettingSourcePill::PolicyManaged
            | M5SettingSourcePill::RemoteProfile
            | M5SettingSourcePill::EnvironmentOverride
            | M5SettingSourcePill::WorkspaceConfigured
    )
}

/// Resolves one settings row from its per-source contributions.
///
/// The winning source is the highest-precedence contribution; its value is the
/// effective value (unless an invalid value is held). The user-configured value
/// is the `UserConfigured` contribution, retained on the resolved row even when a
/// higher source or policy wins, so a locked or overridden row never hides what
/// the user set.
pub fn resolve_settings_row(
    input: &M5SettingsRowResolutionInput,
) -> Result<M5ResolvedSettingsRow, M5SettingsResolutionError> {
    if input.setting_key.trim().is_empty() {
        return Err(M5SettingsResolutionError::EmptySettingKey);
    }
    if input.contributions.is_empty() {
        return Err(M5SettingsResolutionError::NoContributions);
    }

    let mut seen: BTreeSet<M5SettingSourcePill> = BTreeSet::new();
    for contribution in &input.contributions {
        if !seen.insert(contribution.source) {
            return Err(M5SettingsResolutionError::DuplicateSource(
                contribution.source,
            ));
        }
        if contribution.enforces_lock && !source_may_lock(contribution.source) {
            return Err(M5SettingsResolutionError::LockOnUnprivilegedSource(
                contribution.source,
            ));
        }
        if value_repr_is_forbidden(&contribution.value_repr) {
            return Err(M5SettingsResolutionError::ForbiddenValueMaterial);
        }
    }
    if !seen.contains(&M5SettingSourcePill::DefaultValue) {
        return Err(M5SettingsResolutionError::MissingDefaultContribution);
    }
    if input.invalid_value_held {
        match &input.held_value_repr {
            Some(held) if value_repr_is_forbidden(held) => {
                return Err(M5SettingsResolutionError::ForbiddenValueMaterial);
            }
            Some(_) => {}
            None => return Err(M5SettingsResolutionError::MissingHeldValue),
        }
    }

    // The winning contribution is the highest-precedence source; ties cannot
    // happen because sources are unique.
    let winner = input
        .contributions
        .iter()
        .max_by_key(|contribution| source_precedence(contribution.source))
        .expect("contributions are non-empty");

    let configured_value_repr = input
        .contributions
        .iter()
        .find(|contribution| contribution.source == M5SettingSourcePill::UserConfigured)
        .map(|contribution| contribution.value_repr.clone());

    let is_locked = winner.enforces_lock;
    let lock_source = is_locked.then_some(winner.source);

    let effective_value_repr = if input.invalid_value_held {
        input
            .held_value_repr
            .clone()
            .expect("held value present when invalid_value_held")
    } else {
        winner.value_repr.clone()
    };

    let row_state = if input.invalid_value_held {
        M5SettingsRowState::InvalidValueHeld
    } else if input.pending_reload {
        M5SettingsRowState::PendingReloadToApply
    } else if winner.is_redacted() {
        M5SettingsRowState::RedactedManagedValue
    } else if is_locked {
        M5SettingsRowState::LockedByPolicy
    } else if winner.source == M5SettingSourcePill::DefaultValue {
        M5SettingsRowState::InheritedFromDefault
    } else if winner.source == M5SettingSourcePill::UserConfigured {
        M5SettingsRowState::EffectiveMatchesConfigured
    } else {
        M5SettingsRowState::OverriddenByHigherSource
    };

    let differs_from_configured = configured_value_repr
        .as_ref()
        .is_some_and(|configured| configured != &effective_value_repr);

    let mut shadow_chain: Vec<M5SettingSourcePill> =
        input.contributions.iter().map(|c| c.source).collect();
    shadow_chain.sort_by(|a, b| source_precedence(*b).cmp(&source_precedence(*a)));

    Ok(M5ResolvedSettingsRow {
        setting_key: input.setting_key.clone(),
        effective_value_repr,
        winning_source: winner.source,
        configured_value_repr,
        row_state,
        is_locked,
        lock_source,
        differs_from_configured,
        shadow_chain,
    })
}

/// One worked resolution case carried in the packet so the support / export
/// packet reconstructs effective-value truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsRowResolutionCase {
    /// The resolver input.
    pub input: M5SettingsRowResolutionInput,
    /// The resolved effective-value truth. Must equal
    /// `resolve_settings_row(&input)`.
    pub resolved: M5ResolvedSettingsRow,
}

impl M5SettingsRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SettingsRowResolutionInput) -> Self {
        let resolved = resolve_settings_row(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_settings_row(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one config-bearing surface family bound to the
/// shared settings-row anatomy, states, source pills, lock disclosures, focus
/// behaviors, and export fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsRowSurfaceRow {
    /// Config-bearing surface family.
    pub surface_family: M5SettingsSurfaceFamily,
    /// Qualification class earned by this surface.
    pub qualification: M5TrustQualificationClass,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this settings row attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this row must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this row keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Anatomy parts this row renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5SettingsRowAnatomyPart>,
    /// Settings-row states this row projects.
    pub row_states: Vec<M5SettingsRowState>,
    /// Source pills this row shows.
    pub source_pills: Vec<M5SettingSourcePill>,
    /// Lock disclosures this row offers (must include the mandatory disclosures).
    pub lock_disclosures: Vec<M5SettingsLockDisclosure>,
    /// Focus behaviors this row supports.
    pub focus_behaviors: Vec<M5SettingsRowFocusBehavior>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5SettingsRowExportField>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5TrustAccessibilityRoute>,
    /// Shell subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5TrustComponentDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface.
    pub example_resolutions: Vec<M5SettingsRowResolutionCase>,
    /// Hard invariant: this row never conflates effective and configured truth.
    /// MUST be `false`.
    pub conflates_effective_and_configured: bool,
    /// Hard invariant: this row never hides the user-configured value when locked.
    /// MUST be `false`.
    pub hides_user_configured_when_locked: bool,
    /// Hard invariant: this row never invents a private row grammar. MUST be
    /// `false`.
    pub invents_private_row_grammar: bool,
    /// Hard invariant: this row never drops export / audit truth. MUST be `false`.
    pub drops_export_or_audit_truth: bool,
}

impl M5SettingsRowSurfaceRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5SettingsRowAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5SettingsRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory lock disclosure.
    fn declares_mandatory_lock_disclosures(&self) -> bool {
        let present: BTreeSet<M5SettingsLockDisclosure> =
            self.lock_disclosures.iter().copied().collect();
        M5SettingsLockDisclosure::MANDATORY
            .iter()
            .all(|disclosure| present.contains(disclosure))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5SettingsRowExportField> =
            self.export_fields.iter().copied().collect();
        M5SettingsRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.conflates_effective_and_configured
            && !self.hides_user_configured_when_locked
            && !self.invents_private_row_grammar
            && !self.drops_export_or_audit_truth
    }
}

/// Self-describing controlled-vocabulary set minted by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsRowVocabularySet {
    /// Config-bearing surface-family tokens.
    pub surface_families: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Settings-row-state tokens (reused from the frozen matrix).
    pub row_states: Vec<String>,
    /// Source-pill tokens (reused from the frozen matrix).
    pub source_pills: Vec<String>,
    /// Lock-disclosure tokens.
    pub lock_disclosures: Vec<String>,
    /// Focus-behavior tokens.
    pub focus_behaviors: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5SettingsRowVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5SettingsSurfaceFamily::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5SettingsRowAnatomyPart::ALL, |v| v.as_str()),
            row_states: tokens(&M5SettingsRowState::ALL, |v| v.as_str()),
            source_pills: tokens(&M5SettingSourcePill::ALL, |v| v.as_str()),
            lock_disclosures: tokens(&M5SettingsLockDisclosure::ALL, |v| v.as_str()),
            focus_behaviors: tokens(&M5SettingsRowFocusBehavior::ALL, |v| v.as_str()),
            export_fields: tokens(&M5SettingsRowExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TrustAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5SettingsRowGovernanceReview {
    /// One settings-row primitive carries effective-versus-configured truth on
    /// every surface.
    pub one_primitive_carries_effective_versus_configured: bool,
    /// The source pill and lock state are always explained.
    pub source_pill_and_lock_state_always_explained: bool,
    /// A locked value never hides the user-configured value.
    pub locked_value_never_hides_user_configured: bool,
    /// View-diff and source-detail escalation behave the same on every surface.
    pub view_diff_and_source_detail_consistent: bool,
    /// Search landing and highlight-on-open behave the same on every surface.
    pub search_landing_and_highlight_consistent: bool,
    /// The support / export packet reconstructs effective-value truth.
    pub support_export_reconstructs_effective_value: bool,
    /// No surface invents a second settings-row grammar.
    pub no_surface_invents_second_row_grammar: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel settings-row vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsRowConsumerProjection {
    /// Admin / trust / AI / network / execution / extension / update surfaces all
    /// consume the shared primitive.
    pub config_surfaces_consume_shared_primitive: bool,
    /// The effective-value resolver reads a single canonical precedence ladder.
    pub resolver_reads_single_precedence_ladder: bool,
    /// The lock explainer reads a single canonical lock-disclosure source.
    pub lock_explainer_reads_single_source: bool,
    /// Search / deep-link reads a single canonical focus-behavior source.
    pub search_and_deep_link_read_single_source: bool,
    /// Support / export reads a single canonical settings-row source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the settings-row primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsRowReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting settings-row audit.
    pub settings_row_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SettingsRowPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SettingsRowPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5SettingsRowSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingsRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingsRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingsRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingsRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingsRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 settings-row-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsRowPrimitivePacket {
    /// Record kind; must equal [`M5_SETTINGS_ROW_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SETTINGS_ROW_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5SettingsRowSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingsRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingsRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingsRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingsRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingsRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SettingsRowPrimitivePacket {
    /// Builds an M5 settings-row-primitive packet from stable-lane input.
    pub fn new(input: M5SettingsRowPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_SETTINGS_ROW_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_SETTINGS_ROW_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
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

    /// Validates the M5 settings-row-primitive invariants.
    pub fn validate(&self) -> Vec<M5SettingsRowPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SETTINGS_ROW_PRIMITIVE_RECORD_KIND {
            violations.push(M5SettingsRowPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SETTINGS_ROW_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5SettingsRowPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SettingsRowPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_locked_retention_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 settings-row primitive packet serializes"),
        ) {
            violations.push(M5SettingsRowPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 settings-row primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,qualification,owner,shell_zone_slot,anatomy_parts,row_states,source_pills,lock_disclosures,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.row_states, |v| v.as_str()),
                join_tokens(&row.source_pills, |v| v.as_str()),
                join_tokens(&row.lock_disclosures, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .surface_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Settings-Row Primitive: Effective Value, Source Pill, and Lock State\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Config-bearing surfaces: {} ({} stable)\n",
            self.surface_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Anatomy parts: {}\n",
            self.vocabulary_set.anatomy_parts.join(", ")
        ));
        out.push_str(&format!(
            "- Settings-row states: {}\n",
            self.vocabulary_set.row_states.join(", ")
        ));
        out.push_str(&format!(
            "- Source pills: {}\n",
            self.vocabulary_set.source_pills.join(", ")
        ));
        out.push_str(&format!(
            "- Export fields: {}\n",
            self.vocabulary_set.export_fields.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Config-bearing surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.surface_family.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                out.push_str(&format!(
                    "    - `{}` → effective `{}` via `{}` ({})\n",
                    case.resolved.setting_key,
                    case.resolved.effective_value_repr,
                    case.resolved.winning_source.as_str(),
                    case.resolved.row_state.as_str()
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 settings-row-primitive export.
#[derive(Debug)]
pub enum M5SettingsRowPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SettingsRowPrimitiveViolation>),
}

impl fmt::Display for M5SettingsRowPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 settings-row primitive export parse failed: {error}"
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
                    "m5 settings-row primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SettingsRowPrimitiveArtifactError {}

/// Validation failures emitted by [`M5SettingsRowPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SettingsRowPrimitiveViolation {
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
    /// A required config-bearing surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A surface row declares no settings-row states.
    RowStateMissing,
    /// A surface row declares no source pills.
    SourcePillMissing,
    /// A surface row omits one of the mandatory lock disclosures.
    MandatoryLockDisclosureMissing,
    /// A surface row declares no focus behaviors.
    FocusBehaviorMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A surface claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// No worked resolution across the matrix proves a locked value retaining the
    /// user-configured value.
    LockedRetentionUnproven,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SettingsRowPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RowStateMissing => "row_state_missing",
            Self::SourcePillMissing => "source_pill_missing",
            Self::MandatoryLockDisclosureMissing => "mandatory_lock_disclosure_missing",
            Self::FocusBehaviorMissing => "focus_behavior_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::LockedRetentionUnproven => "locked_retention_unproven",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 settings-row-primitive export.
pub fn current_stable_m5_settings_row_primitive_export(
) -> Result<M5SettingsRowPrimitivePacket, M5SettingsRowPrimitiveArtifactError> {
    let packet: M5SettingsRowPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-settings-row-proof/support_export.json"
    )))
    .map_err(M5SettingsRowPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SettingsRowPrimitiveArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SettingsRowPrimitivePacket,
    violations: &mut Vec<M5SettingsRowPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SETTINGS_ROW_SCHEMA_REF,
        M5_SETTINGS_ROW_DOC_REF,
        M5_SETTINGS_ROW_SHELL_ZONE_REF,
        M5_SETTINGS_ROW_COMPONENT_MATRIX_REF,
        M5_SETTINGS_ROW_EFFECTIVE_SETTING_REF,
        M5_SETTINGS_ROW_LOCK_STATE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SettingsRowPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SettingsRowPrimitivePacket,
    violations: &mut Vec<M5SettingsRowPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SettingsRowPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5SettingsRowPrimitivePacket,
    violations: &mut Vec<M5SettingsRowPrimitiveViolation>,
) {
    let present: BTreeSet<M5SettingsSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5SettingsSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5SettingsRowPrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
        {
            violations.push(M5SettingsRowPrimitiveViolation::SurfaceRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5SettingsRowPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.row_states.is_empty() {
            violations.push(M5SettingsRowPrimitiveViolation::RowStateMissing);
        }
        if row.source_pills.is_empty() {
            violations.push(M5SettingsRowPrimitiveViolation::SourcePillMissing);
        }
        if !row.declares_mandatory_lock_disclosures() {
            violations.push(M5SettingsRowPrimitiveViolation::MandatoryLockDisclosureMissing);
        }
        if row.focus_behaviors.is_empty() {
            violations.push(M5SettingsRowPrimitiveViolation::FocusBehaviorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5SettingsRowPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TrustAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5SettingsRowPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SettingsRowPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SettingsRowPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5SettingsRowPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5SettingsRowPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5SettingsRowPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5SettingsRowPrimitiveViolation::SurfaceInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must prove a `LockedByPolicy`
/// row that retains a user-configured value the effective value differs from —
/// the acceptance-criterion example that a locked value never hides what the user
/// set.
fn validate_locked_retention_covered(
    packet: &M5SettingsRowPrimitivePacket,
    violations: &mut Vec<M5SettingsRowPrimitiveViolation>,
) {
    let proven = packet.surface_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.row_state == M5SettingsRowState::LockedByPolicy
                && case.resolved.configured_value_repr.is_some()
                && case.resolved.differs_from_configured
        })
    });
    if !proven {
        violations.push(M5SettingsRowPrimitiveViolation::LockedRetentionUnproven);
    }
}

fn validate_governance_review(
    packet: &M5SettingsRowPrimitivePacket,
    violations: &mut Vec<M5SettingsRowPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_effective_versus_configured,
        review.source_pill_and_lock_state_always_explained,
        review.locked_value_never_hides_user_configured,
        review.view_diff_and_source_detail_consistent,
        review.search_landing_and_highlight_consistent,
        review.support_export_reconstructs_effective_value,
        review.no_surface_invents_second_row_grammar,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5SettingsRowPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SettingsRowPrimitivePacket,
    violations: &mut Vec<M5SettingsRowPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.config_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_precedence_ladder,
        projection.lock_explainer_reads_single_source,
        projection.search_and_deep_link_read_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5SettingsRowPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SettingsRowPrimitivePacket,
    violations: &mut Vec<M5SettingsRowPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SettingsRowPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SettingsRowPrimitivePacket,
    violations: &mut Vec<M5SettingsRowPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.settings_row_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SettingsRowPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single value representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
