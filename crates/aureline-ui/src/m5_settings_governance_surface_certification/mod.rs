//! M05-1203 surface certification over the frozen M5 resolve-setting / write-setting / sync-scope /
//! migrate-schema / rollout-capability settings-governance matrix.
//!
//! Where the freeze matrix ([`crate::m5_settings_governance_matrix`]) defines the five governed
//! configuration-runtime families, the M05-1197..1201 implement lanes resolve each one, and the M05-1202
//! shared-consumer lane
//! ([`crate::m5_settings_governance_shared_consumers_one_registry_across_surfaces`]) aligns their grammar and
//! proves keyboard / screen-reader / high-zoom / high-contrast / localization / CLI-export parity and
//! per-family auto-narrowing across the settings-resolver, shell, sync-service, policy-service,
//! capability-service, diagnostics, docs / help, CLI / export, and support-export consumers, this closing
//! capstone *certifies* that the shared settings-governance truth holds on every claimed M5
//! configuration-bearing profile — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a user, reviewer, admin, or support engineer reads a
//! setting-definition, effective-resolution, write-intent, policy-constraint, sync-conflict,
//! schema-migration, or capability-lifecycle surface through (a live, first-party trusted settings surface; a
//! reviewable settings structure; a disclosed write-intent profile; an unverified sync-conflict profile; and
//! an unverified capability-lifecycle profile), not on the configuration-runtime family or implement lane.
//! Each [`SettingsGovernanceProfileCertificationRow`] certifies one profile across nine truth axes — visual,
//! keyboard, screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! settings-governance-component-truth behavior — and either passes (green), auto-narrows its configuration
//! claim to the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind
//! a fresh trusted claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `TrustedSettingsSurface` / `ReviewableSettingsSurface` claim while one of its truth axes is not current is
//! over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with a bound
//! reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party trusted settings
//! surface profile may certify a `TrustedSettingsSurface` claim — a reviewable, disclosed-write-intent,
//! unverified-sync-conflict, or unverified-capability-lifecycle profile that keeps a trusted claim is
//! over-reaching and blocks. The always-on CLI/export axis must always stay certified so support and
//! automation can reconstruct the canonical setting definition, effective value, write intent, policy
//! constraint, sync conflict packet, schema migration, capability record, and registry reference from the
//! same settings-governance truth the operator saw.
//!
//! The B143 hard invariants are enforced per row: no profile may recycle a retired setting ID, rewrite a
//! scoped (Workspace/Profile) write into a broader (User/Machine) scope, silently overwrite locked or
//! machine-only state during sync, hide a lifecycle or experiment dependency behind unpublished markers, or
//! hide a kill-switch or policy-disable cause behind generic unavailable copy. A profile that breaches any
//! invariant blocks (red).
//!
//! Every row cites exactly one canonical settings-governance proof bundle
//! ([`SETTINGS_GOVERNANCE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen settings-governance matrix proof — rather
//! than cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets, bearer
//! tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/config/m5-settings-governance-surface-certification.schema.json`](../../../../schemas/config/m5-settings-governance-surface-certification.schema.json).
//! The contract doc is
//! [`docs/settings/m5_settings_governance_surface_certification.md`](../../../../docs/settings/m5_settings_governance_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_settings_governance_matrix as matrix;
use crate::m5_settings_governance_shared_consumers_one_registry_across_surfaces as shared_consumers;
use matrix::{M5SettingsGovernanceDowngradeTrigger, M5SettingsGovernanceFamily};

/// Schema version stamped on the M05-1203 certification packet.
pub const SETTINGS_GOVERNANCE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`SettingsGovernanceProfileCertificationPacket`].
pub const SETTINGS_GOVERNANCE_CERT_RECORD_KIND: &str =
    "m5_settings_governance_surface_certification_packet";

/// Stable record-kind tag carried by each [`SettingsGovernanceProfileCertificationRow`].
pub const SETTINGS_GOVERNANCE_CERT_ROW_RECORD_KIND: &str =
    "m5_settings_governance_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const SETTINGS_GOVERNANCE_CERT_SCHEMA_REF: &str =
    "schemas/config/m5-settings-governance-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const SETTINGS_GOVERNANCE_CERT_DOC_REF: &str =
    "docs/settings/m5_settings_governance_surface_certification.md";

/// Repo-relative path of the frozen settings-governance matrix schema the certified profiles render.
pub const SETTINGS_GOVERNANCE_CERT_MATRIX_REF: &str =
    matrix::M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF;

/// The one canonical settings-governance proof bundle every certified profile cites as its first-resolved
/// settings-governance truth. All five profiles point back to it rather than cloning per-profile evidence.
pub const SETTINGS_GOVERNANCE_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_SETTINGS_GOVERNANCE_ARTIFACT_REF;

/// The M05-1202 shared-consumer support export the certification builds on. Recorded as a supporting evidence
/// ref on every row.
pub const SETTINGS_GOVERNANCE_CERT_CONSUMERS_BUNDLE_REF: &str =
    shared_consumers::M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const SETTINGS_GOVERNANCE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-settings-governance-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SETTINGS_GOVERNANCE_CERT_CSV_REF: &str =
    "artifacts/release/m5-settings-governance-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const SETTINGS_GOVERNANCE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-settings-governance-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const SETTINGS_GOVERNANCE_CERT_FIXTURE_DIR: &str =
    "fixtures/config/m5-settings-governance-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const SETTINGS_GOVERNANCE_CERT_PACKET_ID: &str =
    "m5-settings-governance-surface-certification:stable:0001";

/// The five claimed M5 configuration-bearing operating profiles this capstone certifies. Keyed on the profile
/// a user, reviewer, admin, or support engineer reads a setting-definition, effective-resolution,
/// write-intent, policy-constraint, sync-conflict, schema-migration, or capability-lifecycle surface through,
/// not on the reusable configuration-runtime family it renders. Only a live, first-party trusted settings
/// surface profile may certify a trusted settings surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceCertifiedProfile {
    /// A live, first-party, fully-current settings surface — a registry-bound, winning-scope-resolved,
    /// shadow-chain-visible, restart-posture-disclosed, lock-source-disclosed resolve-setting surface
    /// rendering the trusted settings claim exactly right now.
    LiveTrustedSettingsSurface,
    /// A reviewable settings structure: a self-sufficient, inspectable settings-governance projection (a
    /// setting definition / effective value / schema-migration record an admin can review), never itself an
    /// authoritative, live-resolving settings surface.
    ReviewableSettingsStructure,
    /// A write-setting surface whose write-intent preview / checkpoint / rollback evidence can only be
    /// partially disclosed; the claim narrows to a write-intent-disclosed projection that discloses the
    /// partial write-intent evidence alongside the chosen artifact and scope, never a scoped write silently
    /// rewritten into a broader scope or shown as applied when its recovery evidence is incomplete.
    DisclosedWriteIntentProfile,
    /// A sync-scope surface whose sync-conflict resolution cannot be confirmed; the claim narrows to a
    /// sync-conflict-unverified projection that keeps the last-known field-level conflict posture explicit,
    /// never a sync shown as merged when it may have silently overwritten locked or machine-only state during
    /// an outage.
    UnverifiedSyncConflictProfile,
    /// A rollout-capability surface whose capability-lifecycle dependency marker or kill-switch cause has aged
    /// out or is policy-blocked; the claim narrows to a capability-lifecycle-unverified projection that keeps
    /// the last-known lifecycle / kill-switch posture explicit, never a Labs / Preview / DisabledByPolicy
    /// state shown as generally available or hidden behind generic unavailable copy.
    UnverifiedCapabilityLifecycleProfile,
}

impl M5SettingsGovernanceCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5SettingsGovernanceCertifiedProfile; 5] = [
        M5SettingsGovernanceCertifiedProfile::LiveTrustedSettingsSurface,
        M5SettingsGovernanceCertifiedProfile::ReviewableSettingsStructure,
        M5SettingsGovernanceCertifiedProfile::DisclosedWriteIntentProfile,
        M5SettingsGovernanceCertifiedProfile::UnverifiedSyncConflictProfile,
        M5SettingsGovernanceCertifiedProfile::UnverifiedCapabilityLifecycleProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrustedSettingsSurface => "live_trusted_settings_surface",
            Self::ReviewableSettingsStructure => "reviewable_settings_structure",
            Self::DisclosedWriteIntentProfile => "disclosed_write_intent_profile",
            Self::UnverifiedSyncConflictProfile => "unverified_sync_conflict_profile",
            Self::UnverifiedCapabilityLifecycleProfile => "unverified_capability_lifecycle_profile",
        }
    }

    /// True only for the live, first-party trusted settings surface profile. A trusted settings surface may be
    /// certified on this profile alone; every other profile is at most a reviewable settings structure or a
    /// narrowed projection.
    pub const fn is_live_trusted_settings_surface(self) -> bool {
        matches!(self, Self::LiveTrustedSettingsSurface)
    }
}

/// The claim ladder a certified settings-governance profile asserts and is certified down to. Minted locally
/// for this capstone (B143 has no separate accessibility lane): the strongest claim is a fully trusted
/// settings surface; each weaker tier is a disclosed projection that keeps the last-known write-intent,
/// sync-conflict, or capability-lifecycle posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceClaim {
    /// Trusted settings surface: a fully current, registry-bound, winning-scope-resolved,
    /// shadow-chain-visible, restart-posture-disclosed, lock-source-disclosed settings surface — the
    /// strongest claim, a settings-governance surface Aureline can present as exactly trusted and stable right
    /// now.
    TrustedSettingsSurface,
    /// Reviewable settings surface: a self-sufficient, inspectable read-only settings-governance projection (a
    /// static setting definition / effective value / schema-migration record an operator can inspect) that is
    /// not itself an authoritative, live-resolving surface.
    ReviewableSettingsSurface,
    /// Write-intent-disclosed projection: a write-setting family's preview / checkpoint / rollback evidence
    /// can only be partially disclosed; the family stays a write-intent-disclosed projection that discloses
    /// the partial write-intent evidence alongside the chosen artifact and scope, never a scoped write shown
    /// as applied when its recovery evidence is incomplete or rewritten into a broader scope.
    WriteIntentDisclosedProjection,
    /// Sync-conflict-unverified projection: a sync-scope family's field-level conflict resolution cannot be
    /// confirmed; the family stays a sync-conflict-unverified projection that keeps the last-known
    /// keep-local / keep-synced / blocked posture explicit, never a sync shown as merged when it may have
    /// silently overwritten locked or machine-only state.
    SyncConflictUnverifiedProjection,
    /// Capability-lifecycle-unverified projection: a rollout-capability family's dependency marker or
    /// kill-switch cause has aged out or is policy-blocked; the family stays a capability-lifecycle-unverified
    /// projection that keeps the last-known lifecycle / kill-switch posture explicit, never a Labs / Preview /
    /// DisabledByPolicy state shown as generally available or hidden behind generic unavailable copy.
    CapabilityLifecycleUnverifiedProjection,
}

impl M5SettingsGovernanceClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::TrustedSettingsSurface,
        Self::ReviewableSettingsSurface,
        Self::WriteIntentDisclosedProjection,
        Self::SyncConflictUnverifiedProjection,
        Self::CapabilityLifecycleUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedSettingsSurface => 4,
            Self::ReviewableSettingsSurface => 3,
            Self::WriteIntentDisclosedProjection => 2,
            Self::SyncConflictUnverifiedProjection => 1,
            Self::CapabilityLifecycleUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, stable settings surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedSettingsSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedSettingsSurface | Self::ReviewableSettingsSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedSettingsSurface => "trusted_settings_surface",
            Self::ReviewableSettingsSurface => "reviewable_settings_surface",
            Self::WriteIntentDisclosedProjection => "write_intent_disclosed_projection",
            Self::SyncConflictUnverifiedProjection => "sync_conflict_unverified_projection",
            Self::CapabilityLifecycleUnverifiedProjection => {
                "capability_lifecycle_unverified_projection"
            }
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and settings-governance-component-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsGovernanceCertificationAxis {
    /// Visual parity: canonical setting definition, effective value, write intent, policy constraint, sync
    /// conflict packet, schema migration, capability record, and registry reference are shown on the primary
    /// surface without relying on a shell-chrome-only affordance or a mislabeled screenshot alone.
    Visual,
    /// Keyboard-reach parity: the same settings-governance truth and its bound operations are reachable and
    /// operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled screenshot, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the
    /// setting definition, effective value, write intent, sync conflict packet, or registry reference.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping
    /// the setting definition, effective value, or lock source.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling a
    /// scope name, write-intent risk class, sync-conflict class, or capability-lifecycle class when a locale
    /// is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a partially-disclosed write-intent evidence chain, an unconfirmed sync-conflict
    /// resolution, or an aged-out / policy-blocked capability-lifecycle marker honestly downgrades a
    /// `TrustedSettingsSurface` / `ReviewableSettingsSurface` claim rather than reading as a fresh,
    /// authoritative settings surface.
    DegradedState,
    /// Settings-governance-component-truth parity: canonical setting definition, effective value, write
    /// intent, policy constraint, sync conflict packet, schema migration, capability record, and registry
    /// reference stay explicit and never let a configuration surface recycle a retired setting ID, rewrite a
    /// scoped write into a broader scope, silently overwrite locked or machine-only state during sync, hide a
    /// lifecycle or experiment dependency behind unpublished markers, or hide a kill-switch or policy-disable
    /// cause behind generic unavailable copy.
    SettingsGovernanceComponentTruth,
}

impl SettingsGovernanceCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [SettingsGovernanceCertificationAxis; 9] = [
        SettingsGovernanceCertificationAxis::Visual,
        SettingsGovernanceCertificationAxis::Keyboard,
        SettingsGovernanceCertificationAxis::ScreenReader,
        SettingsGovernanceCertificationAxis::HighZoomReflow,
        SettingsGovernanceCertificationAxis::HighContrast,
        SettingsGovernanceCertificationAxis::Localization,
        SettingsGovernanceCertificationAxis::CliExport,
        SettingsGovernanceCertificationAxis::DegradedState,
        SettingsGovernanceCertificationAxis::SettingsGovernanceComponentTruth,
    ];

    /// The always-on CLI/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::CliExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrast => "high_contrast",
            Self::Localization => "localization",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::SettingsGovernanceComponentTruth => "settings_governance_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsGovernanceAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl SettingsGovernanceAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author — always recomputed
/// from the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsGovernanceProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-live profile claims a trusted settings surface, or the narrowing is inconsistent.
    Red,
}

impl SettingsGovernanceProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red profiles block the
    /// release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The five B143 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile
/// carries all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceCertGuardrails {
    /// True if the profile recycles a retired setting ID. Must be false.
    pub recycles_a_retired_setting_id: bool,
    /// True if the profile rewrites a scoped (Workspace/Profile) write into a broader (User/Machine) scope.
    /// Must be false.
    pub rewrites_a_scoped_write_into_a_broader_scope: bool,
    /// True if the profile silently overwrites locked or machine-only state during sync. Must be false.
    pub silently_overwrites_locked_or_machine_only_state_during_sync: bool,
    /// True if the profile hides a lifecycle or experiment dependency behind unpublished markers. Must be
    /// false.
    pub hides_lifecycle_or_experiment_dependency_behind_unpublished_markers: bool,
    /// True if the profile hides a kill-switch or policy-disable cause behind generic unavailable copy. Must
    /// be false.
    pub hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy: bool,
}

impl SettingsGovernanceCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        recycles_a_retired_setting_id: false,
        rewrites_a_scoped_write_into_a_broader_scope: false,
        silently_overwrites_locked_or_machine_only_state_during_sync: false,
        hides_lifecycle_or_experiment_dependency_behind_unpublished_markers: false,
        hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.recycles_a_retired_setting_id
            && !self.rewrites_a_scoped_write_into_a_broader_scope
            && !self.silently_overwrites_locked_or_machine_only_state_during_sync
            && !self.hides_lifecycle_or_experiment_dependency_behind_unpublished_markers
            && !self.hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The setting-definition / effective-value / write-intent / policy-constraint / sync-conflict /
    /// schema-migration / capability-record / registry-reference fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl SettingsGovernanceCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a raw-payload-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.raw_payload_only_prohibited
    }
}

/// One axis outcome on one certified profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: SettingsGovernanceCertificationAxis,
    /// The certification state of the axis.
    pub state: SettingsGovernanceAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5SettingsGovernanceDowngradeTrigger>,
}

impl SettingsGovernanceAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger (that is
    ///   exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            SettingsGovernanceAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            SettingsGovernanceAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            SettingsGovernanceAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present iff the certified
/// claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: SettingsGovernanceCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5SettingsGovernanceClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5SettingsGovernanceClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 configuration-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceProfileCertificationRow {
    /// Record kind; must equal [`SETTINGS_GOVERNANCE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SETTINGS_GOVERNANCE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5SettingsGovernanceCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5SettingsGovernanceClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5SettingsGovernanceClaim,
    /// The frozen configuration-runtime families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5SettingsGovernanceFamily>,
    /// One outcome per [`SettingsGovernanceCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<SettingsGovernanceAxisOutcome>,
    /// The B143 hard invariants; all must hold.
    pub guardrails: SettingsGovernanceCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<SettingsGovernanceClaimAutoNarrow>,
    /// The one canonical settings-governance proof bundle this profile cites. Must equal
    /// [`SETTINGS_GOVERNANCE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: SettingsGovernanceProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: SettingsGovernanceCertExportParity,
    /// The compatibility notes captured for this profile.
    #[serde(default)]
    pub compatibility_notes: Vec<String>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl SettingsGovernanceProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: SettingsGovernanceCertificationAxis,
    ) -> Option<&SettingsGovernanceAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<SettingsGovernanceCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && SettingsGovernanceCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(SettingsGovernanceAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<SettingsGovernanceCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == SettingsGovernanceAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a live first-party profile may
    /// certify a trusted settings surface, every hard invariant must hold, CLI/export parity must always
    /// certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> SettingsGovernanceProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != SETTINGS_GOVERNANCE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return SettingsGovernanceProfileClaimStatus::Red;
        }

        // Every B143 hard invariant must hold.
        if !self.guardrails.all_held() {
            return SettingsGovernanceProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return SettingsGovernanceProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a trusted settings surface.
        if self.certified_claim.asserts_trusted_surface()
            && !self.profile.is_live_trusted_settings_surface()
        {
            return SettingsGovernanceProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(SettingsGovernanceCertificationAxis::CliExport) {
            Some(o) if o.state == SettingsGovernanceAxisCertificationState::Certified => {}
            _ => return SettingsGovernanceProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == SettingsGovernanceAxisCertificationState::UndisclosedDrift)
        {
            return SettingsGovernanceProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return SettingsGovernanceProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return SettingsGovernanceProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return SettingsGovernanceProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return SettingsGovernanceProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return SettingsGovernanceProfileClaimStatus::Red;
        }

        SettingsGovernanceProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == SETTINGS_GOVERNANCE_CERT_ROW_RECORD_KIND
            && self.schema_version == SETTINGS_GOVERNANCE_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_bundle_ref.trim().is_empty()
            && !self.consumed_families.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.compatibility_notes.is_empty()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "profile={profile} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed}",
            profile = self.profile.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-1203 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceProfileCertificationSummary {
    pub row_count: usize,
    pub profile_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_profiles_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub all_guardrails_held: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_profile_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`SettingsGovernanceProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsGovernanceProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<SettingsGovernanceProfileCertificationRow>,
}

/// Checked-in M05-1203 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<SettingsGovernanceProfileCertificationRow>,
    pub summary: SettingsGovernanceProfileCertificationSummary,
}

impl SettingsGovernanceProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: SettingsGovernanceProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: SETTINGS_GOVERNANCE_CERT_SCHEMA_VERSION,
            record_kind: SETTINGS_GOVERNANCE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: SettingsGovernanceProfileCertificationSummary {
                row_count: 0,
                profile_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_profiles_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                all_guardrails_held: false,
                every_axis_covered_on_every_row: false,
                narrowed_profile_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Profiles represented by some row in this packet.
    pub fn represented_profiles(&self) -> BTreeSet<M5SettingsGovernanceCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Configuration-runtime families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5SettingsGovernanceFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5SettingsGovernanceCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen configuration-runtime family is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5SettingsGovernanceFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(SettingsGovernanceCertificationAxis::CliExport)
                .is_some_and(|o| o.state == SettingsGovernanceAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> SettingsGovernanceProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SettingsGovernanceProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SettingsGovernanceProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SettingsGovernanceProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(SettingsGovernanceProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        SettingsGovernanceProfileCertificationSummary {
            row_count: self.rows.len(),
            profile_count: profiles.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_profiles_present: all_profiles,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == SETTINGS_GOVERNANCE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(SettingsGovernanceProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<SettingsGovernanceCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SETTINGS_GOVERNANCE_CERT_SCHEMA_VERSION {
            violations.push(SettingsGovernanceCertificationViolation::SchemaVersion {
                expected: SETTINGS_GOVERNANCE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != SETTINGS_GOVERNANCE_CERT_RECORD_KIND {
            violations.push(SettingsGovernanceCertificationViolation::RecordKind {
                expected: SETTINGS_GOVERNANCE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(SettingsGovernanceCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != SETTINGS_GOVERNANCE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(SettingsGovernanceCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(SettingsGovernanceCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(SettingsGovernanceCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    SettingsGovernanceCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    SettingsGovernanceCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != SETTINGS_GOVERNANCE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    SettingsGovernanceCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B143 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(
                    SettingsGovernanceCertificationViolation::GuardrailViolated {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Only a live first-party profile may certify a trusted settings surface.
            if row.certified_claim.asserts_trusted_surface()
                && !row.profile.is_live_trusted_settings_surface()
            {
                violations.push(
                    SettingsGovernanceCertificationViolation::NonLiveProfileClaimsTrustedSurface {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(SettingsGovernanceCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    SettingsGovernanceCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    SettingsGovernanceCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    SettingsGovernanceCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == SettingsGovernanceProfileClaimStatus::Red {
                violations.push(SettingsGovernanceCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(SettingsGovernanceCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen configuration-runtime family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(SettingsGovernanceCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(SettingsGovernanceCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(
                SettingsGovernanceCertificationViolation::RawSettingsGovernanceMaterialInExport,
            );
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification packet serializes")
    }

    /// Deterministic CSV of the certification rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,profile,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{profile},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                profile = row.profile.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Settings-Governance Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5SettingsGovernanceCertifiedProfile::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Invariants held: {}\n",
            self.summary.all_guardrails_held
        ));
        out.push_str(&format!(
            "- Auto-narrowed profiles: {}\n",
            self.summary.narrowed_profile_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Profiles\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_settings_governance_surface_certification_export() -> Result<
    SettingsGovernanceProfileCertificationPacket,
    SettingsGovernanceCertificationArtifactError,
> {
    let packet: SettingsGovernanceProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-settings-governance-surface-certification/support_export.json"
        )))
        .map_err(SettingsGovernanceCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SettingsGovernanceCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum SettingsGovernanceCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SettingsGovernanceCertificationViolation>),
}

impl fmt::Display for SettingsGovernanceCertificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for SettingsGovernanceCertificationArtifactError {}

/// Validation failure for M05-1203 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsGovernanceCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    GuardrailViolated { id: String },
    NonLiveProfileClaimsTrustedSurface { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawSettingsGovernanceMaterialInExport,
}

impl fmt::Display for SettingsGovernanceCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::WrongCanonicalBundle => {
                write!(
                    f,
                    "packet does not cite the canonical settings-governance proof bundle"
                )
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::AxisCoverageIncomplete { id } => {
                write!(
                    f,
                    "row {id} does not score every certification axis exactly once"
                )
            }
            Self::MalformedAxisOutcome { id } => {
                write!(
                    f,
                    "row {id} has an axis outcome whose disclosure fields disagree with its state"
                )
            }
            Self::RowMissingCanonicalBundle { id } => {
                write!(
                    f,
                    "row {id} does not cite the one canonical settings-governance proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B143 hard invariant: recycling a retired setting ID; rewriting a scoped \
write into a broader scope; silently overwriting locked or machine-only state during sync; hiding a lifecycle \
or experiment dependency behind unpublished markers; or hiding a kill-switch or policy-disable cause behind \
generic unavailable copy"
                )
            }
            Self::NonLiveProfileClaimsTrustedSurface { id } => {
                write!(
                    f,
                    "row {id} certifies a trusted settings surface on a non-live first-party profile"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CertifiedClaimExceedsClaim { id } => {
                write!(
                    f,
                    "row {id} certifies a claim stronger than the claimed one"
                )
            }
            Self::StatusDerivationStale { id } => {
                write!(
                    f,
                    "row {id} stored status disagrees with a fresh derivation"
                )
            }
            Self::ProfileBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh trusted claim, a hard \
invariant broke, CLI/export parity dropped, a non-live profile claimed a trusted settings surface, or the \
narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 configuration-bearing profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen configuration-runtime family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawSettingsGovernanceMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for SettingsGovernanceCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&SettingsGovernanceAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != SettingsGovernanceAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the settings-governance
/// generics the spec forbids collapsing distinct setting-definition, effective-value, write-intent,
/// policy-constraint, sync-conflict, schema-migration, and capability-lifecycle truth into (whole-label
/// matches so a full sentence naming a concrete setting, scope, or registry reference is not flagged).
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "something went wrong"
            | "degraded"
            | "narrowed"
            | "reduced"
            | "stale"
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "pending"
            | "loading"
            | "partial"
            | "cached"
            | "trusted"
            | "reviewable"
            | "settings"
            | "setting"
            | "configuration"
            | "config"
            | "governance"
            | "definition"
            | "effective"
            | "value"
            | "scope"
            | "winning scope"
            | "write"
            | "intent"
            | "write intent"
            | "policy"
            | "constraint"
            | "policy constraint"
            | "lock"
            | "lock source"
            | "sync"
            | "conflict"
            | "sync conflict"
            | "migration"
            | "schema"
            | "schema migration"
            | "capability"
            | "lifecycle"
            | "capability lifecycle"
            | "kill switch"
            | "kill-switch"
            | "rollout"
            | "resolver"
            | "resolve"
            | "device"
            | "registry reference"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the settings-governance
/// matrix and M05-1202 heuristic so the reused [`M5SettingsGovernanceDowngradeTrigger`] narrowings serialize
/// cleanly — the settings-governance grammar carries only typed class tokens and opaque refs, never raw
/// secret values or endpoints.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// --------------------------------------------------------------------------
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-1203 certification packet. Certifies all five claimed M5
/// configuration-bearing profiles: two deliver their claim (green) and three auto-narrow a not-current truth
/// axis to a weaker configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_settings_governance_surface_certification_packet(
) -> SettingsGovernanceProfileCertificationPacket {
    SettingsGovernanceProfileCertificationPacket::new(
        SettingsGovernanceProfileCertificationPacketInput {
            packet_id: SETTINGS_GOVERNANCE_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-15T00:00:00Z".to_owned(),
            matrix_ref: SETTINGS_GOVERNANCE_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: SETTINGS_GOVERNANCE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:settings-governance-surface-certification:{id}"),
        SETTINGS_GOVERNANCE_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> SettingsGovernanceCertExportParity {
    SettingsGovernanceCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: SettingsGovernanceCertificationAxis) -> &'static str {
    match axis {
        SettingsGovernanceCertificationAxis::Visual => {
            "canonical setting definition, effective value, write intent, policy constraint, sync conflict packet, schema migration, capability record, and registry reference shown on-surface without a shell-chrome-only affordance or a mislabeled screenshot alone"
        }
        SettingsGovernanceCertificationAxis::Keyboard => {
            "the same settings-governance role, registry reference, and bound operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        SettingsGovernanceCertificationAxis::ScreenReader => {
            "the same settings-governance truth is announced non-visually, never a shell-chrome-only / mislabeled-screenshot / unlabeled-control-only cue"
        }
        SettingsGovernanceCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the setting definition, effective value, write intent, sync conflict packet, or registry reference"
        }
        SettingsGovernanceCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the setting definition, effective value, or lock source"
        }
        SettingsGovernanceCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling a scope name, write-intent risk class, sync-conflict class, or capability-lifecycle class"
        }
        SettingsGovernanceCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        SettingsGovernanceCertificationAxis::DegradedState => {
            "a partially-disclosed write-intent evidence chain, an unconfirmed sync-conflict resolution, or an aged-out / policy-blocked capability-lifecycle marker honestly downgrades the TrustedSettingsSurface/ReviewableSettingsSurface claim rather than reading as a fresh authoritative settings surface"
        }
        SettingsGovernanceCertificationAxis::SettingsGovernanceComponentTruth => {
            "canonical setting definition, effective value, write intent, policy constraint, sync conflict packet, schema migration, capability record, and registry reference stay explicit and never let a configuration surface recycle a retired setting ID, rewrite a scoped write into a broader scope, silently overwrite locked or machine-only state during sync, hide a lifecycle or experiment dependency behind unpublished markers, or hide a kill-switch or policy-disable cause behind generic unavailable copy"
        }
    }
}

fn seed_certified(axis: SettingsGovernanceCertificationAxis) -> SettingsGovernanceAxisOutcome {
    SettingsGovernanceAxisOutcome {
        axis,
        state: SettingsGovernanceAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: SettingsGovernanceCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5SettingsGovernanceDowngradeTrigger,
) -> SettingsGovernanceAxisOutcome {
    SettingsGovernanceAxisOutcome {
        axis,
        state: SettingsGovernanceAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<SettingsGovernanceAxisOutcome> {
    SettingsGovernanceCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: SettingsGovernanceCertificationAxis,
    outcome: SettingsGovernanceAxisOutcome,
) -> Vec<SettingsGovernanceAxisOutcome> {
    SettingsGovernanceCertificationAxis::ALL
        .iter()
        .copied()
        .map(|a| {
            if a == axis {
                outcome.clone()
            } else {
                seed_certified(a)
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seed_row(
    row_id: &str,
    profile: M5SettingsGovernanceCertifiedProfile,
    claimed_claim: M5SettingsGovernanceClaim,
    certified_claim: M5SettingsGovernanceClaim,
    consumed_families: &[M5SettingsGovernanceFamily],
    axis_outcomes: Vec<SettingsGovernanceAxisOutcome>,
    claim_auto_narrow: Option<SettingsGovernanceClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> SettingsGovernanceProfileCertificationRow {
    let mut row = SettingsGovernanceProfileCertificationRow {
        record_kind: SETTINGS_GOVERNANCE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: SETTINGS_GOVERNANCE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: SettingsGovernanceCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: SETTINGS_GOVERNANCE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: SettingsGovernanceProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            SETTINGS_GOVERNANCE_CERT_MATRIX_REF.to_owned(),
            SETTINGS_GOVERNANCE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-15T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: SettingsGovernanceCertificationAxis,
    from_claim: M5SettingsGovernanceClaim,
    to_claim: M5SettingsGovernanceClaim,
    label: &str,
) -> SettingsGovernanceClaimAutoNarrow {
    SettingsGovernanceClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<SettingsGovernanceProfileCertificationRow> {
    use M5SettingsGovernanceCertifiedProfile as P;
    use M5SettingsGovernanceClaim::*;
    use M5SettingsGovernanceDowngradeTrigger as Trig;
    use M5SettingsGovernanceFamily::*;
    use SettingsGovernanceCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-trusted-settings-surface",
            P::LiveTrustedSettingsSurface,
            TrustedSettingsSurface,
            TrustedSettingsSurface,
            &[ResolveSetting],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "setting_definition",
            ],
            &[
                "resolve-setting profile: the effective value is resolved from the winning scope with the shadow chain, restart posture, and lock source kept visible rather than merged into an opaque connected-state blob, and the stable setting ID is preserved rather than recycled",
                "the trusted settings surface keeps stable operation IDs while the setting definition, effective value, and lock source bind to the one settings-governance registry across settings-resolver / shell / diagnostics / support",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered settings surface",
                "settings-governance-component-truth: a live first-party settings surface is the only profile that certifies a trusted settings surface",
            ],
        ),
        seed_row(
            "cert:reviewable-settings-structure",
            P::ReviewableSettingsStructure,
            ReviewableSettingsSurface,
            ReviewableSettingsSurface,
            &[MigrateSchema],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "schema_migration",
            ],
            &[
                "migrate-schema profile: the schema-migration record keeps its old-key alias, new key, transform, and lossy-fidelity label bound to the single settings-governance registry and shown before the migration rather than a per-surface description copied by hand, and setting-ID continuity is preserved with a checkpoint",
                "the reviewable settings structure keeps its setting-definition, effective-value, migration-record, and registry labels inspectable rather than a shell-chrome-only or mislabeled-screenshot cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable settings structure",
                "settings-governance-component-truth: a reviewable settings structure never certifies a live trusted, authoritative settings claim and never recycles a retired setting ID",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:disclosed-write-intent-profile",
            P::DisclosedWriteIntentProfile,
            ReviewableSettingsSurface,
            WriteIntentDisclosedProjection,
            &[WriteSetting],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the write-setting preview / checkpoint / rollback evidence can only be partially disclosed for this profile so a fully applied write cannot be certified as proven",
                    "The write-setting preview / checkpoint / rollback evidence can only be partially disclosed, so the ReviewableSettingsSurface claim narrows to a write-intent-disclosed projection and the write discloses the partial recovery evidence alongside its chosen artifact and scope rather than presenting a scoped write as applied when its recovery evidence is incomplete or rewriting it into a broader scope",
                    Trig::RewroteAScopedWriteIntoABroaderScope,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableSettingsSurface,
                WriteIntentDisclosedProjection,
                "Write intent disclosed partial: the write-setting recovery evidence is only partially proven so it is disclosed alongside the chosen artifact and scope and no scoped write is rewritten into a broader scope or shown as applied",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "write-setting profile: the write intent names its chosen artifact and scope and marks the preview / checkpoint / rollback evidence as disclosed-partial rather than landing a scoped Workspace/Profile write in a broader User/Machine scope when the evidence is incomplete",
                "the write-setting surface keeps its chosen artifact, scope, and recovery evidence legible while the write-intent evidence is disclosed as partial",
                "localization: ReviewableSettingsSurface narrows to a write-intent-disclosed projection (auto-narrowed)",
                "settings-governance-component-truth: a partial recovery evidence chain never rewrites a scoped write into a broader scope — the chosen artifact and scope are preserved",
            ],
        ),
        seed_row(
            "cert:unverified-sync-conflict-profile",
            P::UnverifiedSyncConflictProfile,
            ReviewableSettingsSurface,
            SyncConflictUnverifiedProjection,
            &[SyncScope],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the sync-scope field-level conflict resolution cannot be confirmed so a fully merged scope bundle cannot be certified",
                    "The sync-scope field-level conflict resolution cannot be confirmed, so the ReviewableSettingsSurface claim narrows to a sync-conflict-unverified projection and the sync keeps the last-known keep-local / keep-synced / blocked posture explicit rather than presenting a scope bundle as merged when it may have silently overwritten locked or machine-only state during an outage",
                    Trig::SilentlyOverwroteLockedOrMachineOnlyStateDuringSync,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableSettingsSurface,
                SyncConflictUnverifiedProjection,
                "Sync conflict unverified: the field-level resolution cannot be confirmed so the last-known keep-local / keep-synced / blocked posture stays explicit and no scope bundle silently overwrites locked or machine-only state",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "sync-scope profile: the sync keeps its field-level conflict posture explicit and marks the resolution as unverified rather than silently overwriting locked or machine-only state during an outage, and never collapses a conflict into last-writer-wins",
                "the sync-scope surface keeps its conflict packet and device-action lineage legible while the field-level resolution is disclosed as unverified",
                "degraded-state: ReviewableSettingsSurface narrows to a sync-conflict-unverified projection (auto-narrowed)",
                "settings-governance-component-truth: a sync never silently overwrites locked or machine-only state and never overclaims a merge when only a blocked conflict posture was computed",
            ],
        ),
        seed_row(
            "cert:unverified-capability-lifecycle-profile",
            P::UnverifiedCapabilityLifecycleProfile,
            ReviewableSettingsSurface,
            CapabilityLifecycleUnverifiedProjection,
            &[RolloutCapability],
            seed_certified_except(
                Ax::SettingsGovernanceComponentTruth,
                seed_narrowed(
                    Ax::SettingsGovernanceComponentTruth,
                    "the capability-lifecycle dependency marker or kill-switch cause has aged out or is policy-blocked so a generally-available capability cannot be certified",
                    "The capability-lifecycle dependency marker or kill-switch cause has aged out or is policy-blocked, so the ReviewableSettingsSurface claim narrows to a capability-lifecycle-unverified projection and the rollout keeps the last-known lifecycle / kill-switch posture explicit rather than presenting a Labs / Preview / DisabledByPolicy state as generally available or hiding a kill-switch or policy-disable cause behind generic unavailable copy",
                    Trig::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
                ),
            ),
            Some(seed_narrow(
                Ax::SettingsGovernanceComponentTruth,
                ReviewableSettingsSurface,
                CapabilityLifecycleUnverifiedProjection,
                "Capability lifecycle unverified: the dependency marker or kill-switch cause has aged out or is policy-blocked so the last-known lifecycle / kill-switch posture stays explicit and no kill-switch or policy-disable cause is hidden behind generic unavailable copy",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "rollout-capability profile: the rollout keeps its capability record, dependency marker, and kill-switch / DisabledByPolicy cause explicit and marks the lifecycle as unverified rather than hiding a lifecycle or experiment dependency behind unpublished markers or a kill-switch cause behind generic unavailable copy",
                "the rollout-capability surface keeps its capability record and kill-switch cause legible while the lifecycle dependency marker is disclosed as unverified",
                "settings-governance-component-truth: ReviewableSettingsSurface narrows to a capability-lifecycle-unverified projection (auto-narrowed)",
                "settings-governance-component-truth: a kill-switch or policy-disable state preserves user data and explains its cause, and no capability claim outpaces the published dependency marker",
            ],
        ),
    ]
}
