//! Shared settings-resolver, shell, sync-service, policy-service, capability-service, diagnostics,
//! docs / help, CLI / export, and support / export consumers that keep the B143 settings-governance
//! families — resolve setting, write setting, sync scope, migrate schema, and rollout capability — at
//! **one canonical registry** across every claimed M5 configuration-bearing surface.
//!
//! This module is the consumer-adoption capstone for the five governed configuration-runtime families
//! frozen in [`crate::m5_settings_governance_matrix`] and implemented by the setting-definition /
//! effective-setting lane ([`crate::m5_setting_definition_and_effective_setting_registries`]), the
//! write-intent / policy-constraint lane
//! ([`crate::m5_setting_write_intent_and_policy_constraint_registries`]), the sync-conflict /
//! device-action lane ([`crate::m5_setting_sync_conflict_and_device_action_registries`]), the
//! schema-migration / compatibility-window lane
//! ([`crate::m5_setting_schema_migration_and_compatibility_window_registries`]), and the
//! capability-lifecycle / kill-switch lane
//! ([`crate::m5_setting_capability_lifecycle_and_kill_switch_registries`]).
//!
//! It binds each shared settings-governance family to the concrete settings-resolver, shell, sync-service,
//! policy-service, capability-service, diagnostics, docs / help, CLI / export, and support-export consumers
//! that render it, and proves — by fixtures, not screenshots — that the same configuration profile presents
//! the same settings-governance-role, family, registry-reference, resolution-context, surface-context, and
//! evidence-continuity grammar wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the five shared settings-governance families must be adopted by at least two
//!    distinct consumers, so a family is proven to be shared settings-resolver infrastructure rather than a
//!    one-surface, feature-local fork of setting-definition, write-intent, sync-conflict, or
//!    capability-lifecycle copy.
//! 2. **One registry / no drift.** For a given configuration profile every consumer surface must present
//!    identical [`SettingsGovernanceStateFacetValues`] — the same settings-governance-role word, the same
//!    family word, the same registry-reference word, the same resolution-context word, the same
//!    surface-context word, and the same evidence-continuity word. The settings-governance-role word must be
//!    a token from the frozen [`M5SettingsGovernanceRole`] vocabulary, so no surface rewrites
//!    `setting_definition`, `effective_resolution`, `write_intent`, `policy_constraint`, `sync_conflict`,
//!    `schema_migration`, or `capability_lifecycle` in its own words. A surface may narrow *how much* it
//!    shows across desktop, compact, remote, and exported representations, but it may never reword the
//!    underlying grammar per surface, and a role that carries write-intent, policy-constraint,
//!    sync-conflict, or capability-lifecycle meaning may never recycle a retired setting ID, rewrite a
//!    scoped write into a broader scope, silently overwrite locked or machine-only state during sync, hide a
//!    lifecycle or experiment dependency behind unpublished markers, or hide a kill-switch or policy-disable
//!    cause behind generic unavailable copy.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the canonical per-domain
//!    schema and the frozen matrix by id, so an exported packet can always map a settings / shell /
//!    sync-service / diagnostics surface back to one shared contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an explicit
//! [`SettingsGovernanceNarrowNote`] naming the reason, the preserved grammar, and the next action, and an
//! exported representation additionally names its export-safe detail boundary rather than collapsing the
//! profile out of view.
//!
//! The packet references upstream settings-governance contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/config/m5-settings-governance-shared-consumers.schema.json`](../../../../schemas/config/m5-settings-governance-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/settings/m5_settings_governance_shared_consumers_one_registry.md`](../../../../docs/settings/m5_settings_governance_shared_consumers_one_registry.md).
//! The protected fixture directory is
//! [`fixtures/config/m5-settings-governance-shared-consumers/`](../../../../fixtures/config/m5-settings-governance-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_settings_governance_shared_consumers,
    seeded_m5_settings_governance_shared_consumers_compact_remote_narrowed,
    seeded_m5_settings_governance_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_settings_governance_matrix::{
    M5SettingsGovernanceConsumerSurface, M5SettingsGovernanceFamily, M5SettingsGovernanceRole,
    M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF, M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SettingsGovernanceSharedConsumersPacket`].
pub const M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_settings_governance_shared_consumer_registry_parity";

/// Schema version for settings-governance shared-consumer parity records.
pub const M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-settings-governance-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/config/m5-settings-governance-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/settings/m5_settings_governance_shared_consumers_one_registry.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-settings-governance-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-settings-governance-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-settings-governance-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/config/m5-settings-governance-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Evidence-continuity sentinel words a write-intent / policy-constraint / sync-conflict /
/// capability-lifecycle role may never fall back to; a trust-carrying role that changes surface
/// presentation must always keep a real evidence-preserved-and-cause-disclosed continuity, never recycling a
/// retired setting ID, widening a scoped write, silently overwriting locked or machine-only state, hiding a
/// lifecycle dependency behind unpublished markers, or hiding a kill-switch or policy-disable cause.
const EVIDENCE_CONTINUITY_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "widened_scoped_write_into_broader_scope",
    "silently_overwrote_locked_or_machine_only_state",
    "hid_lifecycle_dependency_behind_unpublished_markers",
    "hid_kill_switch_or_policy_disable_cause",
];

/// Whether a consumer surface is an export / support path that must map a family back to its
/// canonical contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5SettingsGovernanceConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5SettingsGovernanceConsumerSurface::SupportExport
            | M5SettingsGovernanceConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5SettingsGovernanceRole`] vocabulary.
///
/// This is the "one registry" gate: a configuration profile's settings-governance-role word must be a
/// controlled role token rather than a per-surface synonym.
pub fn is_known_settings_governance_role_token(token: &str) -> bool {
    settings_governance_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5SettingsGovernanceRole`], if it is one.
pub fn settings_governance_role_from_token(token: &str) -> Option<M5SettingsGovernanceRole> {
    M5SettingsGovernanceRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared settings-governance family a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying grammar: a narrowed representation still
/// carries the same settings-governance-role, family, registry-reference, resolution-context, surface-context,
/// and evidence-continuity words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsGovernanceRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl SettingsGovernanceRepresentation {
    /// Every representation, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DesktopFull,
        Self::CompactNarrowed,
        Self::RemoteProjected,
        Self::ExportedRedacted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompactNarrowed => "compact_narrowed",
            Self::RemoteProjected => "remote_projected",
            Self::ExportedRedacted => "exported_redacted",
        }
    }

    /// Whether this representation narrows below full desktop disclosure.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }
}

/// A grammar axis whose word must stay identical across surfaces for one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsGovernanceParityFacet {
    /// The frozen settings-governance-role word.
    SettingsGovernanceRoleWord,
    /// The settings-governance-family word.
    FamilyWord,
    /// The canonical registry-reference word the family points at.
    RegistryReferenceWord,
    /// The resolution-context word (fresh install / returning profile / offline or outage / policy-managed
    /// fleet / resumed after sync conflict) the profile ships.
    ResolutionContextWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The evidence-continuity word paired with a write-intent / policy-constraint / sync-conflict /
    /// capability-lifecycle role.
    EvidenceContinuityWord,
}

impl SettingsGovernanceParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SettingsGovernanceRoleWord,
        Self::FamilyWord,
        Self::RegistryReferenceWord,
        Self::ResolutionContextWord,
        Self::SurfaceContextWord,
        Self::EvidenceContinuityWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsGovernanceRoleWord => "settings_governance_role_word",
            Self::FamilyWord => "family_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::ResolutionContextWord => "resolution_context_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::EvidenceContinuityWord => "evidence_continuity_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared settings-governance family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsGovernanceNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl SettingsGovernanceNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactionNarrowed => "compaction_narrowed",
            Self::RemoteProjectionNarrowed => "remote_projection_narrowed",
            Self::ExportRedactionNarrowed => "export_redaction_narrowed",
        }
    }
}

/// The next action a narrow note offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsGovernanceNarrowNextAction {
    /// Expand the family in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl SettingsGovernanceNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandInDesktop => "expand_in_desktop",
            Self::OpenRemoteSource => "open_remote_source",
            Self::OpenFullDetail => "open_full_detail",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsGovernanceParityState {
    /// All grammar is preserved and shown in full.
    FacetsPreserved,
    /// All grammar is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl SettingsGovernanceParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsGovernanceSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Settings-governance grammar drifted between surfaces for the same profile.
    SettingsGovernanceGrammarDriftDetected,
    /// A trust-carrying role dropped its evidence-preservation or cause-disclosure meaning.
    EvidenceOrCauseDisclosureDropped,
    /// A surface recycled a retired setting ID.
    RecyclesARetiredSettingId,
    /// A surface rewrote a scoped write into a broader scope.
    RewritesAScopedWriteIntoABroaderScope,
    /// A surface silently overwrote locked or machine-only state during sync.
    SilentlyOverwritesLockedOrMachineOnlyStateDuringSync,
    /// A surface hid a lifecycle or experiment dependency behind unpublished markers.
    HidesLifecycleOrExperimentDependencyBehindUnpublishedMarkers,
    /// A surface hid a kill-switch or policy-disable cause behind generic unavailable copy.
    HidesKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared settings-governance family narrowed.
    UpstreamSettingsGovernanceNarrowed,
}

impl SettingsGovernanceSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::SettingsGovernanceGrammarDriftDetected,
        Self::EvidenceOrCauseDisclosureDropped,
        Self::RecyclesARetiredSettingId,
        Self::RewritesAScopedWriteIntoABroaderScope,
        Self::SilentlyOverwritesLockedOrMachineOnlyStateDuringSync,
        Self::HidesLifecycleOrExperimentDependencyBehindUnpublishedMarkers,
        Self::HidesKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamSettingsGovernanceNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::SettingsGovernanceGrammarDriftDetected => {
                "settings_governance_grammar_drift_detected"
            }
            Self::EvidenceOrCauseDisclosureDropped => "evidence_or_cause_disclosure_dropped",
            Self::RecyclesARetiredSettingId => "recycles_a_retired_setting_id",
            Self::RewritesAScopedWriteIntoABroaderScope => {
                "rewrites_a_scoped_write_into_a_broader_scope"
            }
            Self::SilentlyOverwritesLockedOrMachineOnlyStateDuringSync => {
                "silently_overwrites_locked_or_machine_only_state_during_sync"
            }
            Self::HidesLifecycleOrExperimentDependencyBehindUnpublishedMarkers => {
                "hides_lifecycle_or_experiment_dependency_behind_unpublished_markers"
            }
            Self::HidesKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy => {
                "hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamSettingsGovernanceNarrowed => "upstream_settings_governance_narrowed",
        }
    }
}

/// The controlled grammar a configuration profile presents.
///
/// These six words must be identical across every consumer surface that shows the same configuration
/// profile. The settings-governance-role word must be a frozen role token; the rest are controlled words the
/// profile's family carries. A surface may narrow how much it renders, but it may never reword any of these
/// values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceStateFacetValues {
    /// Settings-governance-role word (must be a frozen [`M5SettingsGovernanceRole`] token).
    pub settings_governance_role_word: String,
    /// Settings-governance-family word.
    pub family_word: String,
    /// Canonical registry-reference word the family points at.
    pub registry_reference_word: String,
    /// Resolution-context word (fresh install / returning profile / offline or outage / policy-managed
    /// fleet / resumed after sync conflict) the profile ships.
    pub resolution_context_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Evidence-continuity word paired with a write-intent / policy-constraint / sync-conflict /
    /// capability-lifecycle role.
    pub evidence_continuity_word: String,
}

impl SettingsGovernanceStateFacetValues {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.settings_governance_role_word.trim().is_empty()
            && !self.family_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.resolution_context_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.evidence_continuity_word.trim().is_empty()
    }

    /// Whether the settings-governance-role word is a member of the frozen role vocabulary.
    pub fn settings_governance_role_word_in_vocabulary(&self) -> bool {
        is_known_settings_governance_role_token(self.settings_governance_role_word.trim())
    }

    /// Whether the profile honours the evidence rule: a role that carries write-intent, policy-constraint,
    /// sync-conflict, or capability-lifecycle meaning must pair its surface change with a real
    /// evidence-preserved-and-cause-disclosed continuity and never collapse to a
    /// widened-scoped-write, silently-overwrote-locked-or-machine-only-state,
    /// hid-lifecycle-dependency-behind-unpublished-markers, or
    /// hid-kill-switch-or-policy-disable-cause sentinel.
    pub fn evidence_continuity_satisfied(&self) -> bool {
        match settings_governance_role_from_token(self.settings_governance_role_word.trim()) {
            Some(role) if role.must_preserve_evidence_and_disclose_cause_before_applying() => {
                let continuity = self.evidence_continuity_word.trim().to_lowercase();
                !continuity.is_empty()
                    && !EVIDENCE_CONTINUITY_ABSENT_SENTINELS.contains(&continuity.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceNarrowNote {
    /// Why the representation narrowed.
    pub reason: SettingsGovernanceNarrowReason,
    /// Note naming the preserved grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: SettingsGovernanceNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsGovernanceRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: SettingsGovernanceParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<SettingsGovernanceNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<SettingsGovernanceNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit remote-source note.
    pub needs_remote_source_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its representation.
///
/// The full desktop representation renders at full parity. A compact representation narrows disclosure
/// depth, a remote-projected representation names its remote source, and an exported representation names
/// its export-safe-detail boundary — but all three keep every grammar word and disclose the narrowing
/// through an explicit note.
pub const fn resolve_settings_governance_render_disclosure(
    representation: SettingsGovernanceRepresentation,
) -> SettingsGovernanceRenderDisclosure {
    match representation {
        SettingsGovernanceRepresentation::DesktopFull => SettingsGovernanceRenderDisclosure {
            parity_state: SettingsGovernanceParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        SettingsGovernanceRepresentation::CompactNarrowed => SettingsGovernanceRenderDisclosure {
            parity_state: SettingsGovernanceParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(SettingsGovernanceNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(SettingsGovernanceNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        SettingsGovernanceRepresentation::RemoteProjected => SettingsGovernanceRenderDisclosure {
            parity_state: SettingsGovernanceParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(SettingsGovernanceNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(SettingsGovernanceNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        SettingsGovernanceRepresentation::ExportedRedacted => SettingsGovernanceRenderDisclosure {
            parity_state: SettingsGovernanceParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(SettingsGovernanceNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(SettingsGovernanceNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared settings-governance family rendered on one consumer surface in one
/// representation for one configuration profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable configuration-profile id (shared across surfaces that show the same profile).
    pub governance_profile_id: String,
    /// Human-readable configuration-profile identity.
    pub governance_profile_label: String,
    /// Which shared settings-governance family this binding renders.
    pub family: M5SettingsGovernanceFamily,
    /// Which consumer surface renders it.
    pub consumer: M5SettingsGovernanceConsumerSurface,
    /// Which representation this surface renders.
    pub representation: SettingsGovernanceRepresentation,
    /// The controlled grammar presented (identical across surfaces for one profile).
    pub state_facets: SettingsGovernanceStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: SettingsGovernanceParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<SettingsGovernanceNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface recycles a retired setting ID. MUST be `false`.
    pub recycles_a_retired_setting_id: bool,
    /// Guardrail: this surface rewrites a scoped write into a broader scope. MUST be `false`.
    pub rewrites_a_scoped_write_into_a_broader_scope: bool,
    /// Guardrail: this surface silently overwrites locked or machine-only state during sync. MUST be
    /// `false`.
    pub silently_overwrites_locked_or_machine_only_state_during_sync: bool,
    /// Guardrail: this surface hides a lifecycle or experiment dependency behind unpublished markers. MUST
    /// be `false`.
    pub hides_lifecycle_or_experiment_dependency_behind_unpublished_markers: bool,
    /// Guardrail: this surface hides a kill-switch or policy-disable cause behind generic unavailable copy.
    /// MUST be `false`.
    pub hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl SettingsGovernanceConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> SettingsGovernanceRenderDisclosure {
        resolve_settings_governance_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.recycles_a_retired_setting_id
            && !self.rewrites_a_scoped_write_into_a_broader_scope
            && !self.silently_overwrites_locked_or_machine_only_state_during_sync
            && !self.hides_lifecycle_or_experiment_dependency_behind_unpublished_markers
            && !self.hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.family.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceSharedConsumersTrustReview {
    /// Family reuse is proven by fixtures rather than inferred from screenshots.
    pub family_reuse_proven_by_fixtures: bool,
    /// The same configuration profile presents the same grammar across surfaces.
    pub same_profile_same_settings_governance_across_surfaces: bool,
    /// Every settings-governance-role word is a frozen role token.
    pub settings_governance_role_words_stay_in_frozen_vocabulary: bool,
    /// Trust-carrying roles never widen a scope or hide a cause.
    pub trust_roles_never_widen_scope_or_hide_cause: bool,
    /// A surface never recycles a retired setting ID.
    pub setting_id_never_recycled_across_surfaces: bool,
    /// A surface never rewrites a scoped write into a broader scope.
    pub write_never_widens_a_scoped_write_into_a_broader_scope: bool,
    /// A surface never silently overwrites locked or machine-only state during sync.
    pub sync_never_silently_overwrites_locked_or_machine_only_state: bool,
    /// A surface never hides a lifecycle or experiment dependency behind unpublished markers.
    pub lifecycle_dependency_never_hidden_behind_unpublished_markers: bool,
    /// A surface never hides a kill-switch or policy-disable cause behind generic copy.
    pub kill_switch_or_policy_disable_cause_never_hidden_behind_generic_copy: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl SettingsGovernanceSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.family_reuse_proven_by_fixtures
            && self.same_profile_same_settings_governance_across_surfaces
            && self.settings_governance_role_words_stay_in_frozen_vocabulary
            && self.trust_roles_never_widen_scope_or_hide_cause
            && self.setting_id_never_recycled_across_surfaces
            && self.write_never_widens_a_scoped_write_into_a_broader_scope
            && self.sync_never_silently_overwrites_locked_or_machine_only_state
            && self.lifecycle_dependency_never_hidden_behind_unpublished_markers
            && self.kill_switch_or_policy_disable_cause_never_hidden_behind_generic_copy
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceSharedConsumersProjection {
    /// The settings resolver consumes the shared settings-governance grammar.
    pub settings_resolver_consumes_shared_settings_governance: bool,
    /// The shell UI consumes the shared settings-governance grammar.
    pub shell_ui_consumes_shared_settings_governance: bool,
    /// The sync service consumes the shared settings-governance grammar.
    pub sync_service_consumes_shared_settings_governance: bool,
    /// The policy service consumes the shared settings-governance grammar.
    pub policy_service_consumes_shared_settings_governance: bool,
    /// The capability service consumes the shared settings-governance grammar.
    pub capability_service_consumes_shared_settings_governance: bool,
    /// The diagnostics surface consumes the shared settings-governance grammar.
    pub diagnostics_consumes_shared_settings_governance: bool,
    /// The docs / help surface consumes the shared settings-governance grammar.
    pub docs_help_consumes_shared_settings_governance: bool,
    /// The CLI / export path consumes the shared settings-governance grammar.
    pub cli_export_consumes_shared_settings_governance: bool,
    /// The support / export path consumes the shared settings-governance grammar.
    pub support_export_consumes_shared_settings_governance: bool,
    /// Every family is adopted by two or more consumers.
    pub every_family_adopted_by_two_or_more_consumers: bool,
    /// Grammar is identical for the same configuration profile.
    pub settings_governance_identical_for_same_profile: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a family back to one shared contract family.
    pub export_maps_back_to_one_settings_governance_family: bool,
}

impl SettingsGovernanceSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.settings_resolver_consumes_shared_settings_governance
            && self.shell_ui_consumes_shared_settings_governance
            && self.sync_service_consumes_shared_settings_governance
            && self.policy_service_consumes_shared_settings_governance
            && self.capability_service_consumes_shared_settings_governance
            && self.diagnostics_consumes_shared_settings_governance
            && self.docs_help_consumes_shared_settings_governance
            && self.cli_export_consumes_shared_settings_governance
            && self.support_export_consumes_shared_settings_governance
            && self.every_family_adopted_by_two_or_more_consumers
            && self.settings_governance_identical_for_same_profile
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_settings_governance_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsGovernanceSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5SettingsGovernanceSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SettingsGovernanceSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<SettingsGovernanceConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<SettingsGovernanceSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5SettingsGovernanceConsumerSurface>,
    /// Trust review block.
    pub trust_review: SettingsGovernanceSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: SettingsGovernanceSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: SettingsGovernanceSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe settings-governance shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsGovernanceSharedConsumersPacket {
    /// Record kind; must equal [`M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<SettingsGovernanceConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<SettingsGovernanceSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5SettingsGovernanceConsumerSurface>,
    /// Trust review block.
    pub trust_review: SettingsGovernanceSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: SettingsGovernanceSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: SettingsGovernanceSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SettingsGovernanceSharedConsumersPacket {
    /// Builds a settings-governance shared-consumer packet from stable-lane input.
    pub fn new(input: M5SettingsGovernanceSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the settings-governance shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5SettingsGovernanceSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(M5SettingsGovernanceSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("settings-governance shared-consumer packet serializes"),
        ) {
            violations
                .push(M5SettingsGovernanceSharedConsumersViolation::RawBoundaryMaterialInExport);
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
            .expect("settings-governance shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "family,consumer,representation,settings_governance_role_word,parity_state\n",
        );
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.settings_governance_role_word,
                binding.parity_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str("# Shared Settings-Governance Consumers: One Registry Across Surfaces\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: family `{}` on `{}`, representation `{}`, role `{}`\n",
                binding.governance_profile_label,
                binding.binding_id,
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.settings_governance_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in settings-governance shared-consumer export.
#[derive(Debug)]
pub enum M5SettingsGovernanceSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SettingsGovernanceSharedConsumersViolation>),
}

impl fmt::Display for M5SettingsGovernanceSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "settings-governance shared-consumer export parse failed: {error}"
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
                    "settings-governance shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SettingsGovernanceSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5SettingsGovernanceSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SettingsGovernanceSharedConsumersViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's settings-governance-role word is not a frozen role token.
    SettingsGovernanceRoleWordOutsideVocabulary,
    /// A binding's trust-carrying role dropped its evidence continuity.
    EvidenceContinuityMissingForTrustRole,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same configuration profile with different grammar.
    SettingsGovernanceGrammarDriftAcrossSurfaces,
    /// A shared family is not adopted by at least two distinct consumers.
    FamilyReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow note.
    NarrowNoteMissing,
    /// A narrow note's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow note's next action does not match the required next action.
    NarrowNextActionMismatch,
    /// A narrow note is missing its preserved-grammar note.
    NarrowNotePreservedGrammarMissing,
    /// A narrow note is missing its next-action copy.
    NarrowNextActionLabelMissing,
    /// A full-desktop binding carries a narrow note it must not.
    UnexpectedNarrowNote,
    /// A binding that needs an explicit remote-source note is missing it.
    RemoteSourceNoteMissing,
    /// A binding that needs an explicit export-detail note is missing it.
    ExportDetailNoteMissing,
    /// A binding recycles a retired setting ID.
    RecyclesARetiredSettingId,
    /// A binding rewrites a scoped write into a broader scope.
    RewritesAScopedWriteIntoABroaderScope,
    /// A binding silently overwrites locked or machine-only state during sync.
    SilentlyOverwritesLockedOrMachineOnlyStateDuringSync,
    /// A binding hides a lifecycle or experiment dependency behind unpublished markers.
    HidesLifecycleOrExperimentDependencyBehindUnpublishedMarkers,
    /// A binding hides a kill-switch or policy-disable cause behind generic unavailable copy.
    HidesKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared family appears among the bindings.
    FamilyCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5SettingsGovernanceSharedConsumersViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::SettingsGovernanceRoleWordOutsideVocabulary => {
                "settings_governance_role_word_outside_vocabulary"
            }
            Self::EvidenceContinuityMissingForTrustRole => {
                "evidence_continuity_missing_for_trust_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::SettingsGovernanceGrammarDriftAcrossSurfaces => {
                "settings_governance_grammar_drift_across_surfaces"
            }
            Self::FamilyReuseUnproven => "family_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::NarrowNoteMissing => "narrow_note_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowNotePreservedGrammarMissing => "narrow_note_preserved_grammar_missing",
            Self::NarrowNextActionLabelMissing => "narrow_next_action_label_missing",
            Self::UnexpectedNarrowNote => "unexpected_narrow_note",
            Self::RemoteSourceNoteMissing => "remote_source_note_missing",
            Self::ExportDetailNoteMissing => "export_detail_note_missing",
            Self::RecyclesARetiredSettingId => "recycles_a_retired_setting_id",
            Self::RewritesAScopedWriteIntoABroaderScope => {
                "rewrites_a_scoped_write_into_a_broader_scope"
            }
            Self::SilentlyOverwritesLockedOrMachineOnlyStateDuringSync => {
                "silently_overwrites_locked_or_machine_only_state_during_sync"
            }
            Self::HidesLifecycleOrExperimentDependencyBehindUnpublishedMarkers => {
                "hides_lifecycle_or_experiment_dependency_behind_unpublished_markers"
            }
            Self::HidesKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy => {
                "hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::FamilyCoverageMissing => "family_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable settings-governance shared-consumer export.
pub fn current_stable_m5_settings_governance_shared_consumers_export() -> Result<
    M5SettingsGovernanceSharedConsumersPacket,
    M5SettingsGovernanceSharedConsumersArtifactError,
> {
    let packet: M5SettingsGovernanceSharedConsumersPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-settings-governance-shared-consumers-proof/support_export.json"
        )))
        .map_err(M5SettingsGovernanceSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SettingsGovernanceSharedConsumersArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SettingsGovernanceSharedConsumersPacket,
    violations: &mut Vec<M5SettingsGovernanceSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_SHARED_CONSUMERS_DOC_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
    ];
    // The five families map to four canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5SettingsGovernanceFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5SettingsGovernanceSharedConsumersPacket,
    violations: &mut Vec<M5SettingsGovernanceSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5SettingsGovernanceSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One registry: the facet values must be identical for every binding that renders the same
    // configuration profile.
    let mut profile_facets: BTreeMap<&str, &SettingsGovernanceStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each family must be adopted by at least two distinct consumers.
    let mut family_consumers: BTreeMap<
        M5SettingsGovernanceFamily,
        BTreeSet<M5SettingsGovernanceConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5SettingsGovernanceConsumerSurface> = BTreeSet::new();
    let mut seen_families: BTreeSet<M5SettingsGovernanceFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.governance_profile_id.trim().is_empty()
            || binding.governance_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::GrammarFacetIncomplete);
        }
        if !binding
            .state_facets
            .settings_governance_role_word_in_vocabulary()
        {
            violations.push(
                M5SettingsGovernanceSharedConsumersViolation::SettingsGovernanceRoleWordOutsideVocabulary,
            );
        }
        if !binding.state_facets.evidence_continuity_satisfied() {
            violations.push(
                M5SettingsGovernanceSharedConsumersViolation::EvidenceContinuityMissingForTrustRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations
                        .push(M5SettingsGovernanceSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations.push(
                            M5SettingsGovernanceSharedConsumersViolation::NarrowReasonMismatch,
                        );
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5SettingsGovernanceSharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5SettingsGovernanceSharedConsumersViolation::NarrowNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5SettingsGovernanceSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.recycles_a_retired_setting_id {
            violations
                .push(M5SettingsGovernanceSharedConsumersViolation::RecyclesARetiredSettingId);
        }
        if binding.rewrites_a_scoped_write_into_a_broader_scope {
            violations.push(
                M5SettingsGovernanceSharedConsumersViolation::RewritesAScopedWriteIntoABroaderScope,
            );
        }
        if binding.silently_overwrites_locked_or_machine_only_state_during_sync {
            violations.push(
                M5SettingsGovernanceSharedConsumersViolation::SilentlyOverwritesLockedOrMachineOnlyStateDuringSync,
            );
        }
        if binding.hides_lifecycle_or_experiment_dependency_behind_unpublished_markers {
            violations.push(
                M5SettingsGovernanceSharedConsumersViolation::HidesLifecycleOrExperimentDependencyBehindUnpublishedMarkers,
            );
        }
        if binding.hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy {
            violations.push(
                M5SettingsGovernanceSharedConsumersViolation::HidesKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
            );
        }

        // Support / export consumers must map a family back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5SettingsGovernanceSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_facets.get(binding.governance_profile_id.as_str()) {
            None => {
                profile_facets.insert(
                    binding.governance_profile_id.as_str(),
                    &binding.state_facets,
                );
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5SettingsGovernanceSharedConsumersViolation::SettingsGovernanceGrammarDriftAcrossSurfaces,
                    );
                    drift_reported = true;
                }
            }
        }

        family_consumers
            .entry(binding.family)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_families.insert(binding.family);
    }

    // Coverage: every consumer surface and every family must appear.
    for consumer in M5SettingsGovernanceConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for family in M5SettingsGovernanceFamily::ALL {
        if !seen_families.contains(&family) {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::FamilyCoverageMissing);
            break;
        }
    }

    // Reuse: every present family must be adopted by two or more distinct consumers.
    for consumers in family_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5SettingsGovernanceSharedConsumersViolation::FamilyReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
