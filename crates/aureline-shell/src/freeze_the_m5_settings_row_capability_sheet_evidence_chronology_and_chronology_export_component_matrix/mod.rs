//! Frozen M5 settings-row, capability-sheet, evidence-chronology, and
//! chronology-export component matrix.
//!
//! This module locks Aureline's highest-trust reusable UI components into one
//! export-safe packet. Every component family M5 claims that still drifts too
//! easily by feature lane — settings rows, permission/capability sheets,
//! event/history rows, timeline groups, narrative summary cards, and chronology
//! export previews — is named once here, bound to a canonical shell zone,
//! responsive class, and window class, and constrained by the same
//! effective-value, permission-scope, provenance, and export rules regardless of
//! the surface family that renders it.
//!
//! The shell topology this matrix binds against — the eight canonical shell
//! zones, the compact/standard/expanded responsive classes, the window classes,
//! and the ten claimed M5 surface families — is the one already frozen by
//! [`crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix`];
//! this matrix re-exports that vocabulary rather than minting parallel terms.
//! What this matrix adds is the stable vocabulary for the *components*
//! themselves: the component families, the settings-row states and source pills,
//! the capability consequence classes and scope states, the chronology verbs and
//! provenance badges, the chronology detail states, the chronology export fields,
//! the non-visual accessibility routes, and the mandatory labels every component
//! must be able to show.
//!
//! The matrix is the single source of truth for whether a claimed M5 high-trust
//! component may publish a settings, capability, chronology, or export claim.
//! Settings surfaces, permission sheets, activity/evidence timelines, and
//! chronology export previews all consume this packet so one settings-row model
//! carries effective-versus-configured truth with source pills and lock-state
//! explainability, one capability-sheet model groups requests by consequence,
//! shows transitive scope, and preserves reduced-mode and re-consent behavior,
//! and one evidence/chronology model carries stable verbs, provenance badges, and
//! portable detail/export semantics. No M5 lane invents a second row grammar or
//! drops audit/support truth.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5TrustComponentVocabularySet`] rather than minted per surface. Raw URLs,
//! raw local paths, raw usernames, raw hostnames, tokens, raw diagnostics,
//! private endpoints, credentials, and user text bodies stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-trust-chronology-components.schema.json`](../../../../schemas/ui/m5-trust-chronology-components.schema.json)
//! and the contract doc is
//! [`docs/components/m5_trust_chronology_components_contract.md`](../../../../docs/components/m5_trust_chronology_components_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-trust-chronology-components/`](../../../../fixtures/ui/m5-trust-chronology-components/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_trust_chronology_component_matrix,
    seeded_m5_trust_chronology_component_matrix_chronology_export_preview_preview_narrowed,
    seeded_m5_trust_chronology_component_matrix_narrative_summary_card_beta_narrowed,
    M5_TRUST_COMPONENTS_MATRIX_PACKET_ID,
};

// The canonical shell topology — zones, responsive classes, window classes,
// consumer surfaces, and the ten claimed M5 surface families — is frozen once,
// in the shell-zone matrix. This matrix reuses it verbatim so no high-trust
// component invents a parallel slot, layout class, window class, or surface
// family.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellSurfaceFamily, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5TrustComponentMatrixPacket`].
pub const M5_TRUST_COMPONENTS_MATRIX_RECORD_KIND: &str =
    "freeze_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix";

/// Schema version for M5 trust-chronology-component-matrix records.
pub const M5_TRUST_COMPONENTS_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the trust-chronology-components boundary schema.
pub const M5_TRUST_COMPONENTS_SCHEMA_REF: &str =
    "schemas/ui/m5-trust-chronology-components.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_TRUST_COMPONENTS_DOC_REF: &str =
    "docs/components/m5_trust_chronology_components_contract.md";

/// Repo-relative path of the frozen shell-zone schema this matrix binds against.
pub const M5_TRUST_COMPONENTS_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the settings-row source contract this matrix binds
/// against.
pub const M5_TRUST_COMPONENTS_SETTINGS_CONTRACT_REF: &str =
    "schemas/settings/m5-settings-row.schema.json";

/// Repo-relative path of the capability-sheet source contract this matrix binds
/// against.
pub const M5_TRUST_COMPONENTS_CAPABILITY_CONTRACT_REF: &str =
    "schemas/capabilities/m5-capability-sheet.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_TRUST_COMPONENTS_FIXTURE_DIR: &str = "fixtures/ui/m5-trust-chronology-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TRUST_COMPONENTS_ARTIFACT_REF: &str =
    "artifacts/release/m5-trust-chronology-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_TRUST_COMPONENTS_CSV_REF: &str =
    "artifacts/release/m5-trust-chronology-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_TRUST_COMPONENTS_REPORT_REF: &str =
    "artifacts/components/m5-trust-chronology-components.md";

/// One of the six governed high-trust component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustComponentFamily {
    /// A settings row carrying effective-versus-configured truth.
    SettingsRow,
    /// A permission/capability sheet grouping requests by consequence.
    CapabilitySheet,
    /// A single event/history row in an activity or evidence timeline.
    EventHistoryRow,
    /// A timeline group that collapses related events under one heading.
    TimelineGroup,
    /// A narrative summary card that summarizes a chronology span.
    NarrativeSummaryCard,
    /// A chronology export preview showing what will be exported.
    ChronologyExportPreview,
}

impl M5TrustComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SettingsRow,
        Self::CapabilitySheet,
        Self::EventHistoryRow,
        Self::TimelineGroup,
        Self::NarrativeSummaryCard,
        Self::ChronologyExportPreview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsRow => "settings_row",
            Self::CapabilitySheet => "capability_sheet",
            Self::EventHistoryRow => "event_history_row",
            Self::TimelineGroup => "timeline_group",
            Self::NarrativeSummaryCard => "narrative_summary_card",
            Self::ChronologyExportPreview => "chronology_export_preview",
        }
    }

    /// `true` when this family is a settings row and must therefore declare its
    /// settings-row states and source pills.
    pub const fn is_settings(self) -> bool {
        matches!(self, Self::SettingsRow)
    }

    /// `true` when this family is a capability sheet and must therefore declare
    /// its consequence classes and scope states.
    pub const fn is_capability(self) -> bool {
        matches!(self, Self::CapabilitySheet)
    }

    /// `true` when this family renders chronology events and must therefore
    /// declare its stable verbs and provenance badges.
    pub const fn is_chronology_row(self) -> bool {
        matches!(
            self,
            Self::EventHistoryRow | Self::TimelineGroup | Self::NarrativeSummaryCard
        )
    }

    /// `true` when this family groups chronology and must therefore declare its
    /// detail states.
    pub const fn groups_chronology(self) -> bool {
        matches!(self, Self::TimelineGroup | Self::NarrativeSummaryCard)
    }

    /// `true` when this family previews a chronology export and must therefore
    /// declare its export fields.
    pub const fn is_export(self) -> bool {
        matches!(self, Self::ChronologyExportPreview)
    }
}

/// Controlled settings-row state — how the effective value relates to the
/// configured value and its lock/validity posture. A settings row may not surface
/// a bare value without one of these named states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsRowState {
    /// Effective value equals the configured value.
    EffectiveMatchesConfigured,
    /// A higher-priority source overrode the configured value.
    OverriddenByHigherSource,
    /// The effective value is inherited from a default.
    InheritedFromDefault,
    /// The value is locked by policy and not editable here.
    LockedByPolicy,
    /// A change is staged and pending a restart or reload to apply.
    PendingReloadToApply,
    /// The configured value is invalid and the prior value is held.
    InvalidValueHeld,
    /// A credential-managed value is redacted from display.
    RedactedManagedValue,
}

impl M5SettingsRowState {
    /// Every settings-row state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::EffectiveMatchesConfigured,
        Self::OverriddenByHigherSource,
        Self::InheritedFromDefault,
        Self::LockedByPolicy,
        Self::PendingReloadToApply,
        Self::InvalidValueHeld,
        Self::RedactedManagedValue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectiveMatchesConfigured => "effective_matches_configured",
            Self::OverriddenByHigherSource => "overridden_by_higher_source",
            Self::InheritedFromDefault => "inherited_from_default",
            Self::LockedByPolicy => "locked_by_policy",
            Self::PendingReloadToApply => "pending_reload_to_apply",
            Self::InvalidValueHeld => "invalid_value_held",
            Self::RedactedManagedValue => "redacted_managed_value",
        }
    }
}

/// Controlled settings source pill — the origin that produced the effective
/// value. A settings row that hides which source won never reads as honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingSourcePill {
    /// The shipped default value.
    DefaultValue,
    /// A value the user configured.
    UserConfigured,
    /// A value configured at the workspace level.
    WorkspaceConfigured,
    /// A value managed by policy / an administrator.
    PolicyManaged,
    /// A value supplied by a remote profile.
    RemoteProfile,
    /// A value from an environment override.
    EnvironmentOverride,
}

impl M5SettingSourcePill {
    /// Every source pill, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DefaultValue,
        Self::UserConfigured,
        Self::WorkspaceConfigured,
        Self::PolicyManaged,
        Self::RemoteProfile,
        Self::EnvironmentOverride,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultValue => "default_value",
            Self::UserConfigured => "user_configured",
            Self::WorkspaceConfigured => "workspace_configured",
            Self::PolicyManaged => "policy_managed",
            Self::RemoteProfile => "remote_profile",
            Self::EnvironmentOverride => "environment_override",
        }
    }
}

/// Controlled capability consequence class — the kind of consequence a
/// permission/capability request carries, so a sheet groups by consequence rather
/// than by an arbitrary permission list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityConsequenceClass {
    /// Reads local context / files.
    ReadLocalContext,
    /// Modifies the workspace.
    ModifyWorkspace,
    /// Executes code / commands.
    ExecuteCode,
    /// Accesses the network.
    NetworkAccess,
    /// Accesses stored credentials.
    CredentialAccess,
    /// Controls the host system / environment.
    SystemControl,
}

impl M5CapabilityConsequenceClass {
    /// Every consequence class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadLocalContext,
        Self::ModifyWorkspace,
        Self::ExecuteCode,
        Self::NetworkAccess,
        Self::CredentialAccess,
        Self::SystemControl,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadLocalContext => "read_local_context",
            Self::ModifyWorkspace => "modify_workspace",
            Self::ExecuteCode => "execute_code",
            Self::NetworkAccess => "network_access",
            Self::CredentialAccess => "credential_access",
            Self::SystemControl => "system_control",
        }
    }
}

/// Controlled capability scope state — the consent / scope posture of a capability
/// request. Transitive scope and re-consent are always explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityScopeState {
    /// Requested but not yet granted.
    RequestedNotGranted,
    /// Granted at full requested scope.
    GrantedFullScope,
    /// Granted at a reduced scope, with the narrowing disclosed.
    GrantedReducedScope,
    /// Transitive / downstream scope is disclosed.
    TransitiveScopeDisclosed,
    /// A previously granted scope requires re-consent.
    ReConsentRequired,
    /// Revoked, with the change kept in history.
    RevokedWithHistory,
}

impl M5CapabilityScopeState {
    /// Every scope state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RequestedNotGranted,
        Self::GrantedFullScope,
        Self::GrantedReducedScope,
        Self::TransitiveScopeDisclosed,
        Self::ReConsentRequired,
        Self::RevokedWithHistory,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestedNotGranted => "requested_not_granted",
            Self::GrantedFullScope => "granted_full_scope",
            Self::GrantedReducedScope => "granted_reduced_scope",
            Self::TransitiveScopeDisclosed => "transitive_scope_disclosed",
            Self::ReConsentRequired => "re_consent_required",
            Self::RevokedWithHistory => "revoked_with_history",
        }
    }
}

/// Controlled chronology verb — the stable, closed verb vocabulary every
/// event/history row uses so the same action never reads under two names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologyVerb {
    /// An object was created.
    Created,
    /// An object was updated.
    Updated,
    /// An action was run / executed.
    Ran,
    /// An action was approved.
    Approved,
    /// An action was rejected.
    Rejected,
    /// An action failed.
    Failed,
    /// A degraded / failed state recovered.
    Recovered,
    /// A chronology was exported.
    Exported,
}

impl M5ChronologyVerb {
    /// Every chronology verb, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Created,
        Self::Updated,
        Self::Ran,
        Self::Approved,
        Self::Rejected,
        Self::Failed,
        Self::Recovered,
        Self::Exported,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Ran => "ran",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
            Self::Failed => "failed",
            Self::Recovered => "recovered",
            Self::Exported => "exported",
        }
    }
}

/// Controlled provenance badge — who / what initiated a chronology event. Every
/// row attributes its provenance so a human, AI, automation, or remote action is
/// never conflated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProvenanceBadge {
    /// A human initiated the event.
    HumanInitiated,
    /// An AI action initiated the event.
    AiInitiated,
    /// An automation / scheduled task initiated the event.
    AutomationInitiated,
    /// A remote actor initiated the event.
    RemoteActor,
    /// The system initiated the event.
    SystemInitiated,
    /// The event was replayed from durable history.
    ReplayedFromHistory,
}

impl M5ProvenanceBadge {
    /// Every provenance badge, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HumanInitiated,
        Self::AiInitiated,
        Self::AutomationInitiated,
        Self::RemoteActor,
        Self::SystemInitiated,
        Self::ReplayedFromHistory,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanInitiated => "human_initiated",
            Self::AiInitiated => "ai_initiated",
            Self::AutomationInitiated => "automation_initiated",
            Self::RemoteActor => "remote_actor",
            Self::SystemInitiated => "system_initiated",
            Self::ReplayedFromHistory => "replayed_from_history",
        }
    }
}

/// Controlled chronology detail state — how a grouped chronology view expands,
/// groups, filters, and keeps a reopen path into detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologyDetailState {
    /// The group is collapsed to a summary.
    Collapsed,
    /// The group is expanded to individual events.
    Expanded,
    /// Grouped by the object the events touched.
    GroupedByObject,
    /// Grouped by time bucket.
    GroupedByTime,
    /// Filtered to a subset, with the filter disclosed.
    Filtered,
    /// Detail is reopenable from durable history.
    ReopenableDetail,
}

impl M5ChronologyDetailState {
    /// Every chronology detail state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Collapsed,
        Self::Expanded,
        Self::GroupedByObject,
        Self::GroupedByTime,
        Self::Filtered,
        Self::ReopenableDetail,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Collapsed => "collapsed",
            Self::Expanded => "expanded",
            Self::GroupedByObject => "grouped_by_object",
            Self::GroupedByTime => "grouped_by_time",
            Self::Filtered => "filtered",
            Self::ReopenableDetail => "reopenable_detail",
        }
    }
}

/// Controlled chronology export field — the fields a chronology export preview
/// promises to carry into the exported record, so an export never silently drops
/// a truth-bearing column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologyExportField {
    /// The stable event verb.
    EventVerb,
    /// The provenance badge.
    Provenance,
    /// The event timestamp.
    Timestamp,
    /// The stable reference to the touched object.
    ObjectRef,
    /// The actor role attribution.
    ActorRole,
    /// The controlled outcome code.
    OutcomeCode,
    /// The redaction class applied to the row.
    RedactionClass,
}

impl M5ChronologyExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::EventVerb,
        Self::Provenance,
        Self::Timestamp,
        Self::ObjectRef,
        Self::ActorRole,
        Self::OutcomeCode,
        Self::RedactionClass,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EventVerb => "event_verb",
            Self::Provenance => "provenance",
            Self::Timestamp => "timestamp",
            Self::ObjectRef => "object_ref",
            Self::ActorRole => "actor_role",
            Self::OutcomeCode => "outcome_code",
            Self::RedactionClass => "redaction_class",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5TrustAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed high-trust component must be able to show. The first
/// three are hard requirements on every component per the guardrails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The provenance / source attribution of the shown content.
    Provenance,
    /// The effective value / decision the component carries.
    EffectiveValue,
    /// The audit / reopen path for the component's truth.
    AuditReopenPath,
}

impl M5TrustRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::Provenance,
        Self::EffectiveValue,
        Self::AuditReopenPath,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::Provenance => "provenance",
            Self::EffectiveValue => "effective_value",
            Self::AuditReopenPath => "audit_reopen_path",
        }
    }
}

/// Qualification class for an M5 trust-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5TrustQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a high-trust component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustComponentDowngradeTrigger {
    /// A settings row conflated effective and configured values.
    EffectiveConfiguredConflated,
    /// A settings row hid which source produced the effective value.
    SourcePillMissing,
    /// A settings row left a lock state unexplained.
    LockStateUnexplained,
    /// A capability sheet dropped consequence grouping.
    ConsequenceGroupingDropped,
    /// A capability sheet hid transitive / downstream scope.
    TransitiveScopeHidden,
    /// A capability change skipped required re-consent.
    ReConsentSkipped,
    /// A chronology row drifted from the stable verb vocabulary.
    VerbVocabularyDrift,
    /// A chronology row omitted its provenance badge.
    ProvenanceBadgeMissing,
    /// A chronology detail was not reopenable from history.
    ChronologyDetailNotReopenable,
    /// A chronology export dropped a truth-bearing field.
    ExportFieldDropped,
    /// Audit / support truth was lost off the primary surface.
    AuditTruthLostOffPrimarySurface,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5TrustComponentDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::EffectiveConfiguredConflated,
        Self::SourcePillMissing,
        Self::LockStateUnexplained,
        Self::ConsequenceGroupingDropped,
        Self::TransitiveScopeHidden,
        Self::ReConsentSkipped,
        Self::VerbVocabularyDrift,
        Self::ProvenanceBadgeMissing,
        Self::ChronologyDetailNotReopenable,
        Self::ExportFieldDropped,
        Self::AuditTruthLostOffPrimarySurface,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectiveConfiguredConflated => "effective_configured_conflated",
            Self::SourcePillMissing => "source_pill_missing",
            Self::LockStateUnexplained => "lock_state_unexplained",
            Self::ConsequenceGroupingDropped => "consequence_grouping_dropped",
            Self::TransitiveScopeHidden => "transitive_scope_hidden",
            Self::ReConsentSkipped => "re_consent_skipped",
            Self::VerbVocabularyDrift => "verb_vocabulary_drift",
            Self::ProvenanceBadgeMissing => "provenance_badge_missing",
            Self::ChronologyDetailNotReopenable => "chronology_detail_not_reopenable",
            Self::ExportFieldDropped => "export_field_dropped",
            Self::AuditTruthLostOffPrimarySurface => "audit_truth_lost_off_primary_surface",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed high-trust component family bound to its
/// shell zone, layout classes, and the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustComponentRow {
    /// Governed component family.
    pub component_family: M5TrustComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5TrustQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this component attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this component must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this component keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5ShellSurfaceFamily>,
    /// Mandatory labels this component must be able to show (must include the
    /// three [`M5TrustRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5TrustRequiredLabel>,
    /// Settings-row states this component projects (settings only).
    pub settings_row_states: Vec<M5SettingsRowState>,
    /// Source pills this component shows (settings only).
    pub source_pills: Vec<M5SettingSourcePill>,
    /// Capability consequence classes this component groups by (capability only).
    pub consequence_classes: Vec<M5CapabilityConsequenceClass>,
    /// Capability scope states this component honours (capability only).
    pub capability_scope_states: Vec<M5CapabilityScopeState>,
    /// Stable chronology verbs this component uses (chronology rows only).
    pub chronology_verbs: Vec<M5ChronologyVerb>,
    /// Provenance badges this component attributes (chronology rows only).
    pub provenance_badges: Vec<M5ProvenanceBadge>,
    /// Chronology detail states this component honours (grouping only).
    pub chronology_detail_states: Vec<M5ChronologyDetailState>,
    /// Chronology export fields this component promises (export only).
    pub chronology_export_fields: Vec<M5ChronologyExportField>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5TrustAccessibilityRoute>,
    /// Shell subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5TrustComponentDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never conflates effective and configured
    /// truth. MUST be `false`.
    pub conflates_effective_and_configured: bool,
    /// Hard invariant: this component never hides permission scope. MUST be
    /// `false`.
    pub hides_permission_scope: bool,
    /// Hard invariant: this component never invents a private row grammar. MUST
    /// be `false`.
    pub invents_private_row_grammar: bool,
    /// Hard invariant: this component never drops audit / support truth. MUST be
    /// `false`.
    pub drops_audit_or_support_truth: bool,
}

impl M5TrustComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5TrustRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5TrustRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.conflates_effective_and_configured
            && !self.hides_permission_scope
            && !self.invents_private_row_grammar
            && !self.drops_audit_or_support_truth
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Settings-row-state tokens.
    pub settings_row_states: Vec<String>,
    /// Source-pill tokens.
    pub source_pills: Vec<String>,
    /// Consequence-class tokens.
    pub consequence_classes: Vec<String>,
    /// Capability-scope-state tokens.
    pub capability_scope_states: Vec<String>,
    /// Chronology-verb tokens.
    pub chronology_verbs: Vec<String>,
    /// Provenance-badge tokens.
    pub provenance_badges: Vec<String>,
    /// Chronology-detail-state tokens.
    pub chronology_detail_states: Vec<String>,
    /// Chronology-export-field tokens.
    pub chronology_export_fields: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5TrustComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5TrustComponentFamily::ALL, |v| v.as_str()),
            settings_row_states: tokens(&M5SettingsRowState::ALL, |v| v.as_str()),
            source_pills: tokens(&M5SettingSourcePill::ALL, |v| v.as_str()),
            consequence_classes: tokens(&M5CapabilityConsequenceClass::ALL, |v| v.as_str()),
            capability_scope_states: tokens(&M5CapabilityScopeState::ALL, |v| v.as_str()),
            chronology_verbs: tokens(&M5ChronologyVerb::ALL, |v| v.as_str()),
            provenance_badges: tokens(&M5ProvenanceBadge::ALL, |v| v.as_str()),
            chronology_detail_states: tokens(&M5ChronologyDetailState::ALL, |v| v.as_str()),
            chronology_export_fields: tokens(&M5ChronologyExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TrustAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5TrustRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5TrustComponentGovernanceReview {
    /// The settings row carries effective-versus-configured truth.
    pub settings_row_carries_effective_versus_configured: bool,
    /// The settings source pills and lock state are explained.
    pub settings_source_pills_and_lock_state_explained: bool,
    /// The capability sheet groups requests by consequence.
    pub capability_sheet_groups_by_consequence: bool,
    /// Transitive scope and re-consent behavior are preserved.
    pub capability_transitive_scope_and_reconsent_preserved: bool,
    /// Chronology uses stable verbs and provenance badges.
    pub chronology_uses_stable_verbs_and_provenance: bool,
    /// Chronology detail and export stay portable.
    pub chronology_detail_and_export_portable: bool,
    /// No component invents a second row grammar.
    pub no_component_invents_second_row_grammar: bool,
    /// No audit or support truth is dropped off the primary surface.
    pub no_audit_or_support_truth_dropped: bool,
    /// Every component is bound to a canonical shell zone.
    pub every_component_bound_to_shell_zone: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel trust-component vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustComponentConsumerProjection {
    /// Settings surfaces consume the shared settings-row / source-pill vocabulary.
    pub settings_surfaces_consume_matrix: bool,
    /// Capability sheets consume the consequence / scope vocabulary.
    pub capability_sheets_consume_scope_vocabulary: bool,
    /// Activity / evidence surfaces consume the chronology vocabulary.
    pub activity_and_evidence_consume_chronology_vocabulary: bool,
    /// Chronology export reads a single canonical export-field source.
    pub chronology_export_reads_single_source: bool,
    /// Support / export reads a single canonical trust-component source.
    pub support_export_reads_single_source: bool,
    /// The accessibility bridge reads a single canonical trust-component source.
    pub accessibility_bridge_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the trust-component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustComponentReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting trust-component audit for the lane.
    pub trust_component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TrustComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TrustComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5TrustComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TrustComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TrustComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TrustComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TrustComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TrustComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 trust-chronology-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustComponentMatrixPacket {
    /// Record kind; must equal [`M5_TRUST_COMPONENTS_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TRUST_COMPONENTS_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5TrustComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TrustComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TrustComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TrustComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TrustComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TrustComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TrustComponentMatrixPacket {
    /// Builds an M5 trust-chronology-component matrix packet from stable-lane
    /// input.
    pub fn new(input: M5TrustComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_TRUST_COMPONENTS_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_TRUST_COMPONENTS_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 trust-chronology-component matrix invariants.
    pub fn validate(&self) -> Vec<M5TrustComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TRUST_COMPONENTS_MATRIX_RECORD_KIND {
            violations.push(M5TrustComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TRUST_COMPONENTS_MATRIX_SCHEMA_VERSION {
            violations.push(M5TrustComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TrustComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 trust component matrix packet serializes"),
        ) {
            violations.push(M5TrustComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 trust component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed
    /// component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,shell_zone_slot,responsive_classes,window_classes,surface_families,required_labels,consumer_surfaces\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.responsive_classes, |v| v.as_str()),
                join_tokens(&row.window_classes, |v| v.as_str()),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Settings-Row, Capability-Sheet, Evidence-Chronology, and Chronology-Export Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Chronology verbs: {}\n",
            self.vocabulary_set.chronology_verbs.join(", ")
        ));
        out.push_str(&format!(
            "- Chronology export fields: {}\n",
            self.vocabulary_set.chronology_export_fields.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 trust-component matrix export.
#[derive(Debug)]
pub enum M5TrustComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TrustComponentMatrixViolation>),
}

impl fmt::Display for M5TrustComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 trust component matrix export parse failed: {error}"
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
                    "m5 trust component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TrustComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5TrustComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TrustComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A settings component declares no settings-row states.
    SettingsRowStateMissing,
    /// A settings component declares no source pills.
    SourcePillMissing,
    /// A capability component declares no consequence classes.
    ConsequenceClassMissing,
    /// A capability component declares no scope states.
    CapabilityScopeStateMissing,
    /// A chronology-row component declares no stable verbs.
    ChronologyVerbMissing,
    /// A chronology-row component declares no provenance badges.
    ProvenanceBadgeMissing,
    /// A grouping component declares no chronology detail states.
    ChronologyDetailStateMissing,
    /// An export component declares no chronology export fields.
    ExportFieldMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no responsive classes.
    ResponsiveClassMissing,
    /// A component declares no window classes.
    WindowClassMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (effective/configured conflation,
    /// hidden permission scope, private row grammar, or dropped audit truth).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5TrustComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::SettingsRowStateMissing => "settings_row_state_missing",
            Self::SourcePillMissing => "source_pill_missing",
            Self::ConsequenceClassMissing => "consequence_class_missing",
            Self::CapabilityScopeStateMissing => "capability_scope_state_missing",
            Self::ChronologyVerbMissing => "chronology_verb_missing",
            Self::ProvenanceBadgeMissing => "provenance_badge_missing",
            Self::ChronologyDetailStateMissing => "chronology_detail_state_missing",
            Self::ExportFieldMissing => "export_field_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ResponsiveClassMissing => "responsive_class_missing",
            Self::WindowClassMissing => "window_class_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 trust-component matrix export.
pub fn current_stable_m5_trust_chronology_component_matrix_export(
) -> Result<M5TrustComponentMatrixPacket, M5TrustComponentMatrixArtifactError> {
    let packet: M5TrustComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-trust-chronology-proof/support_export.json"
    )))
    .map_err(M5TrustComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TrustComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5TrustComponentMatrixPacket,
    violations: &mut Vec<M5TrustComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TRUST_COMPONENTS_SCHEMA_REF,
        M5_TRUST_COMPONENTS_DOC_REF,
        M5_TRUST_COMPONENTS_SHELL_ZONE_REF,
        M5_TRUST_COMPONENTS_SETTINGS_CONTRACT_REF,
        M5_TRUST_COMPONENTS_CAPABILITY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TrustComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5TrustComponentMatrixPacket,
    violations: &mut Vec<M5TrustComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5TrustComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5TrustComponentMatrixPacket,
    violations: &mut Vec<M5TrustComponentMatrixViolation>,
) {
    let present: BTreeSet<M5TrustComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5TrustComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5TrustComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5TrustComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5TrustComponentMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_settings() && row.settings_row_states.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::SettingsRowStateMissing);
        }
        if family.is_settings() && row.source_pills.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::SourcePillMissing);
        }
        if family.is_capability() && row.consequence_classes.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::ConsequenceClassMissing);
        }
        if family.is_capability() && row.capability_scope_states.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::CapabilityScopeStateMissing);
        }
        if family.is_chronology_row() && row.chronology_verbs.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::ChronologyVerbMissing);
        }
        if family.is_chronology_row() && row.provenance_badges.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::ProvenanceBadgeMissing);
        }
        if family.groups_chronology() && row.chronology_detail_states.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::ChronologyDetailStateMissing);
        }
        if family.is_export() && row.chronology_export_fields.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::ExportFieldMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.responsive_classes.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::ResponsiveClassMissing);
        }
        if row.window_classes.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::WindowClassMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5TrustComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5TrustComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5TrustComponentMatrixPacket,
    violations: &mut Vec<M5TrustComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.settings_row_carries_effective_versus_configured,
        review.settings_source_pills_and_lock_state_explained,
        review.capability_sheet_groups_by_consequence,
        review.capability_transitive_scope_and_reconsent_preserved,
        review.chronology_uses_stable_verbs_and_provenance,
        review.chronology_detail_and_export_portable,
        review.no_component_invents_second_row_grammar,
        review.no_audit_or_support_truth_dropped,
        review.every_component_bound_to_shell_zone,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5TrustComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TrustComponentMatrixPacket,
    violations: &mut Vec<M5TrustComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.settings_surfaces_consume_matrix,
        projection.capability_sheets_consume_scope_vocabulary,
        projection.activity_and_evidence_consume_chronology_vocabulary,
        projection.chronology_export_reads_single_source,
        projection.support_export_reads_single_source,
        projection.accessibility_bridge_reads_single_source,
    ] {
        if !ok {
            violations.push(M5TrustComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TrustComponentMatrixPacket,
    violations: &mut Vec<M5TrustComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TrustComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TrustComponentMatrixPacket,
    violations: &mut Vec<M5TrustComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.trust_component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TrustComponentMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
