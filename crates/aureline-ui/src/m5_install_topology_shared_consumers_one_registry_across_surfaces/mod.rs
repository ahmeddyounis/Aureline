//! Shared installer, package-manager, About, update-center, diagnostics, admin, support / export,
//! and enterprise fleet-rollout consumers that keep the B140 install-topology families — per-user
//! managed, per-machine managed, side-by-side stable-plus-preview, portable mode, and offline /
//! air-gap bundles — at **one canonical registry** across every claimed M5 distribution surface.
//!
//! This module is the consumer-adoption lane for the five reusable delivery-topology families frozen
//! in [`crate::m5_install_topology_matrix`] and implemented by the install-topology / state-root
//! registries lane ([`crate::m5_install_topology_and_state_root_registries`]), the portable-mode
//! state-containment lane ([`crate::m5_portable_mode_state_containment_and_diagnostics`]), the
//! managed-deployment operations lane
//! ([`crate::m5_managed_deployment_operations_and_policy_bootstrap_injection`]), and the side-by-side
//! channel-isolation lane
//! ([`crate::m5_channel_isolation_precedence_review_and_rollback_targets`]).
//!
//! It binds each shared install-topology family to the concrete installer, package-manager, About /
//! shell, update-center / updater, diagnostics, admin, docs / help, CLI / export, support-export, and
//! general product consumers that render it, and proves — by fixtures, not screenshots — that the same
//! delivery profile presents the same install-topology-role, family, registry-reference, channel,
//! surface-context, and ownership-identity grammar wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the five shared install-topology families must be adopted by at least two
//!    distinct consumers, so a family is proven to be shared delivery-topology infrastructure rather
//!    than a one-surface, feature-local fork of install-mode, updater-ownership, state-root, or
//!    rollback copy.
//! 2. **One registry / no drift.** For a given delivery profile every consumer surface must present
//!    identical [`InstallTopologyStateFacetValues`] — the same install-topology-role word, the same
//!    family word, the same registry-reference word, the same channel word, the same surface-context
//!    word, and the same ownership-identity word. The install-topology-role word must be a token from
//!    the frozen [`M5InstallTopologyRole`] vocabulary, so no surface rewrites `install_mode`,
//!    `updater_owner`, `binary_root`, `writable_state_roots`, `policy_roots`, `rollback_target`, or
//!    `rollout_ring` in its own words. A surface may narrow *how much* it shows across desktop,
//!    compact, remote, and exported representations, but it may never reword the underlying grammar
//!    per surface, and a role that carries updater-ownership or state-isolation meaning may never let a
//!    topology change hide who owns the updater, spill durable state into hidden machine-global paths,
//!    reuse a stable state namespace without an explicit handoff, narrow rollback below the full
//!    artifact graph, or publish a deployment claim that outpaces ring or repair / verify evidence.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the canonical
//!    per-domain schema and the frozen matrix by id, so an exported packet can always map an installer /
//!    About / update / diagnostics / admin install-topology surface back to one shared contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an
//! explicit [`InstallTopologyNarrowNote`] naming the reason, the preserved grammar, and the next
//! action, and an exported representation additionally names its export-safe detail boundary rather
//! than collapsing the profile out of view.
//!
//! The packet references upstream install-topology contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/install/m5-install-topology-shared-consumers.schema.json`](../../../../schemas/install/m5-install-topology-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/install/m5_install_topology_shared_consumers_one_registry.md`](../../../../docs/install/m5_install_topology_shared_consumers_one_registry.md).
//! The protected fixture directory is
//! [`fixtures/install/m5-install-topology-shared-consumers/`](../../../../fixtures/install/m5-install-topology-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_install_topology_shared_consumers,
    seeded_m5_install_topology_shared_consumers_compact_remote_narrowed,
    seeded_m5_install_topology_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_install_topology_matrix::{
    M5InstallTopologyConsumerSurface, M5InstallTopologyFamily, M5InstallTopologyRole,
    M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF, M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5InstallTopologySharedConsumersPacket`].
pub const M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_install_topology_shared_consumer_registry_parity";

/// Schema version for install-topology shared-consumer parity records.
pub const M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-install-topology-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/install/m5-install-topology-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/install/m5_install_topology_shared_consumers_one_registry.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-install-topology-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-install-topology-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-install-topology-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/install/m5-install-topology-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Ownership-identity sentinel words an updater-ownership / writable-state-roots / policy-roots /
/// rollback-target role may never fall back to; an ownership-carrying role that changes topology
/// presentation must always keep a real preserved ownership identity, never hiding who owns the
/// updater, spilling durable state into a hidden machine-global path, reusing a stable state namespace
/// without a handoff, or narrowing rollback below the full artifact graph.
const OWNERSHIP_IDENTITY_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "hidden_updater_owner",
    "spilled_machine_global_state",
    "rollback_primary_only",
    "namespace_reused_without_handoff",
];

/// Whether a consumer surface is an export / support path that must map a family back to its
/// canonical contract by id.
pub const fn consumer_must_reference_canonical(consumer: M5InstallTopologyConsumerSurface) -> bool {
    matches!(
        consumer,
        M5InstallTopologyConsumerSurface::SupportExport
            | M5InstallTopologyConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5InstallTopologyRole`] vocabulary.
///
/// This is the "one registry" gate: a delivery profile's install-topology-role word must be a
/// controlled role token rather than a per-surface synonym.
pub fn is_known_install_topology_role_token(token: &str) -> bool {
    install_topology_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5InstallTopologyRole`], if it is one.
pub fn install_topology_role_from_token(token: &str) -> Option<M5InstallTopologyRole> {
    M5InstallTopologyRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared install-topology family a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying grammar: a narrowed representation still
/// carries the same install-topology-role, family, registry-reference, channel, surface-context, and
/// ownership-identity words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallTopologyRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl InstallTopologyRepresentation {
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
pub enum InstallTopologyParityFacet {
    /// The frozen install-topology-role word.
    InstallTopologyRoleWord,
    /// The install-topology-family word.
    FamilyWord,
    /// The canonical registry-reference word the family points at.
    RegistryReferenceWord,
    /// The channel word (stable / preview / beta / LTS) the profile ships.
    ChannelWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The ownership-identity word paired with an updater-ownership / state-isolation / rollback role.
    OwnershipIdentityWord,
}

impl InstallTopologyParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InstallTopologyRoleWord,
        Self::FamilyWord,
        Self::RegistryReferenceWord,
        Self::ChannelWord,
        Self::SurfaceContextWord,
        Self::OwnershipIdentityWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallTopologyRoleWord => "install_topology_role_word",
            Self::FamilyWord => "family_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::ChannelWord => "channel_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::OwnershipIdentityWord => "ownership_identity_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared install-topology family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallTopologyNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl InstallTopologyNarrowReason {
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
pub enum InstallTopologyNarrowNextAction {
    /// Expand the family in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl InstallTopologyNarrowNextAction {
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
pub enum InstallTopologyParityState {
    /// All grammar is preserved and shown in full.
    FacetsPreserved,
    /// All grammar is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl InstallTopologyParityState {
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
pub enum InstallTopologySharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Install-topology grammar drifted between surfaces for the same profile.
    InstallTopologyGrammarDriftDetected,
    /// An ownership-carrying role dropped its ownership identity or state-isolation meaning.
    OwnershipIdentityOrStateIsolationDropped,
    /// Portable mode wrote hidden machine-global durable state.
    PortableModeWroteHiddenMachineGlobalDurableState,
    /// A preview channel reused a stable state namespace without an explicit import / handoff.
    PreviewChannelReusedStableStateNamespaceWithoutHandoff,
    /// A rollback targeted only the primary executable while sidecars or metadata drifted.
    RollbackTargetedPrimaryExecutableWhileSidecarsDrifted,
    /// Updater ownership or admin control was hidden in a managed flow.
    UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
    /// A deployment claim outpaced ring or repair / verify evidence.
    DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared install-topology family narrowed.
    UpstreamInstallTopologyNarrowed,
}

impl InstallTopologySharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::InstallTopologyGrammarDriftDetected,
        Self::OwnershipIdentityOrStateIsolationDropped,
        Self::PortableModeWroteHiddenMachineGlobalDurableState,
        Self::PreviewChannelReusedStableStateNamespaceWithoutHandoff,
        Self::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted,
        Self::UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
        Self::DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamInstallTopologyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::InstallTopologyGrammarDriftDetected => "install_topology_grammar_drift_detected",
            Self::OwnershipIdentityOrStateIsolationDropped => {
                "ownership_identity_or_state_isolation_dropped"
            }
            Self::PortableModeWroteHiddenMachineGlobalDurableState => {
                "portable_mode_wrote_hidden_machine_global_durable_state"
            }
            Self::PreviewChannelReusedStableStateNamespaceWithoutHandoff => {
                "preview_channel_reused_stable_state_namespace_without_handoff"
            }
            Self::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted => {
                "rollback_targeted_primary_executable_while_sidecars_drifted"
            }
            Self::UpdaterOwnershipOrAdminControlHiddenInManagedFlow => {
                "updater_ownership_or_admin_control_hidden_in_managed_flow"
            }
            Self::DeploymentClaimOutpacedRingOrRepairVerifyEvidence => {
                "deployment_claim_outpaced_ring_or_repair_verify_evidence"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamInstallTopologyNarrowed => "upstream_install_topology_narrowed",
        }
    }
}

/// The controlled grammar a delivery profile presents.
///
/// These six words must be identical across every consumer surface that shows the same delivery
/// profile. The install-topology-role word must be a frozen role token; the rest are controlled words
/// the profile's family carries. A surface may narrow how much it renders, but it may never reword any
/// of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyStateFacetValues {
    /// Install-topology-role word (must be a frozen [`M5InstallTopologyRole`] token).
    pub install_topology_role_word: String,
    /// Install-topology-family word.
    pub family_word: String,
    /// Canonical registry-reference word the family points at.
    pub registry_reference_word: String,
    /// Channel word (stable / preview / beta / LTS) the profile ships.
    pub channel_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Ownership-identity word paired with an updater-ownership / state-isolation / rollback role.
    pub ownership_identity_word: String,
}

impl InstallTopologyStateFacetValues {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.install_topology_role_word.trim().is_empty()
            && !self.family_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.channel_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.ownership_identity_word.trim().is_empty()
    }

    /// Whether the install-topology-role word is a member of the frozen role vocabulary.
    pub fn install_topology_role_word_in_vocabulary(&self) -> bool {
        is_known_install_topology_role_token(self.install_topology_role_word.trim())
    }

    /// Whether the profile honours the ownership-identity rule: a role that carries updater-ownership,
    /// writable-state-roots, policy-roots, or rollback-target meaning must pair its topology change
    /// with a real preserved ownership identity and never collapse to a hidden-updater-owner,
    /// spilled-machine-global-state, rollback-primary-only, or namespace-reused-without-handoff
    /// sentinel.
    pub fn ownership_identity_satisfied(&self) -> bool {
        match install_topology_role_from_token(self.install_topology_role_word.trim()) {
            Some(role) if role.must_preserve_state_isolation_and_ownership_under_coexistence() => {
                let identity = self.ownership_identity_word.trim().to_lowercase();
                !identity.is_empty()
                    && !OWNERSHIP_IDENTITY_ABSENT_SENTINELS.contains(&identity.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyNarrowNote {
    /// Why the representation narrowed.
    pub reason: InstallTopologyNarrowReason,
    /// Note naming the preserved grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: InstallTopologyNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstallTopologyRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: InstallTopologyParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<InstallTopologyNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<InstallTopologyNarrowNextAction>,
    /// Whether the binding must carry an explicit narrow note.
    pub needs_narrow_note: bool,
    /// Whether the binding must carry an explicit remote-source note.
    pub needs_remote_source_note: bool,
    /// Whether the binding must carry an explicit export-safe-detail note.
    pub needs_export_detail_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its representation.
///
/// The full desktop representation renders at full parity. A compact representation narrows
/// disclosure depth, a remote-projected representation names its remote source, and an exported
/// representation names its export-safe-detail boundary — but all three keep every grammar word and
/// disclose the narrowing through an explicit note.
pub const fn resolve_install_topology_render_disclosure(
    representation: InstallTopologyRepresentation,
) -> InstallTopologyRenderDisclosure {
    match representation {
        InstallTopologyRepresentation::DesktopFull => InstallTopologyRenderDisclosure {
            parity_state: InstallTopologyParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        InstallTopologyRepresentation::CompactNarrowed => InstallTopologyRenderDisclosure {
            parity_state: InstallTopologyParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(InstallTopologyNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(InstallTopologyNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        InstallTopologyRepresentation::RemoteProjected => InstallTopologyRenderDisclosure {
            parity_state: InstallTopologyParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(InstallTopologyNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(InstallTopologyNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        InstallTopologyRepresentation::ExportedRedacted => InstallTopologyRenderDisclosure {
            parity_state: InstallTopologyParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(InstallTopologyNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(InstallTopologyNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared install-topology family rendered on one consumer surface in one
/// representation for one delivery profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable delivery-profile id (shared across surfaces that show the same profile).
    pub delivery_profile_id: String,
    /// Human-readable delivery-profile identity.
    pub delivery_profile_label: String,
    /// Which shared install-topology family this binding renders.
    pub family: M5InstallTopologyFamily,
    /// Which consumer surface renders it.
    pub consumer: M5InstallTopologyConsumerSurface,
    /// Which representation this surface renders.
    pub representation: InstallTopologyRepresentation,
    /// The controlled grammar presented (identical across surfaces for one profile).
    pub state_facets: InstallTopologyStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: InstallTopologyParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<InstallTopologyNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface lets portable mode write hidden machine-global durable state. MUST be
    /// `false`.
    pub portable_mode_writes_hidden_machine_global_durable_state: bool,
    /// Guardrail: this surface lets a preview channel reuse a stable state namespace without an
    /// explicit import / handoff. MUST be `false`.
    pub preview_channel_reuses_stable_state_namespace_without_handoff: bool,
    /// Guardrail: this surface rolls back only the primary executable while sidecars or metadata
    /// drift. MUST be `false`.
    pub rollback_targets_primary_executable_while_sidecars_drift: bool,
    /// Guardrail: this surface hides updater ownership or admin control in a managed flow. MUST be
    /// `false`.
    pub hides_updater_ownership_or_admin_control_in_managed_flow: bool,
    /// Guardrail: this surface publishes a deployment claim that outpaces ring or repair / verify
    /// evidence. MUST be `false`.
    pub publishes_deployment_claim_outpacing_ring_or_repair_verify_evidence: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl InstallTopologyConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> InstallTopologyRenderDisclosure {
        resolve_install_topology_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.portable_mode_writes_hidden_machine_global_durable_state
            && !self.preview_channel_reuses_stable_state_namespace_without_handoff
            && !self.rollback_targets_primary_executable_while_sidecars_drift
            && !self.hides_updater_ownership_or_admin_control_in_managed_flow
            && !self.publishes_deployment_claim_outpacing_ring_or_repair_verify_evidence
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
                .any(|reference| reference == M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologySharedConsumersTrustReview {
    /// Family reuse is proven by fixtures rather than inferred from screenshots.
    pub family_reuse_proven_by_fixtures: bool,
    /// The same delivery profile presents the same grammar across surfaces.
    pub same_profile_same_install_topology_across_surfaces: bool,
    /// Every install-topology-role word is a frozen role token.
    pub install_topology_role_words_stay_in_frozen_vocabulary: bool,
    /// Ownership-carrying roles never hide the updater owner or spill / corrupt isolated state.
    pub ownership_roles_never_hide_owner_or_spill_state: bool,
    /// Portable mode never spills durable state into hidden machine-global paths.
    pub portable_mode_never_spills_hidden_machine_global_durable_state: bool,
    /// Preview channels never reuse a stable state namespace without an explicit handoff.
    pub preview_channel_never_reuses_stable_state_namespace_without_handoff: bool,
    /// Rollback never targets only the primary executable while sidecars drift.
    pub rollback_never_targets_primary_executable_while_sidecars_drift: bool,
    /// Updater ownership or admin control is never hidden in a managed flow.
    pub updater_ownership_or_admin_control_never_hidden_in_managed_flow: bool,
    /// Deployment claims never outpace ring or repair / verify evidence.
    pub deployment_claims_never_outpace_ring_or_repair_verify_evidence: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl InstallTopologySharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.family_reuse_proven_by_fixtures
            && self.same_profile_same_install_topology_across_surfaces
            && self.install_topology_role_words_stay_in_frozen_vocabulary
            && self.ownership_roles_never_hide_owner_or_spill_state
            && self.portable_mode_never_spills_hidden_machine_global_durable_state
            && self.preview_channel_never_reuses_stable_state_namespace_without_handoff
            && self.rollback_never_targets_primary_executable_while_sidecars_drift
            && self.updater_ownership_or_admin_control_never_hidden_in_managed_flow
            && self.deployment_claims_never_outpace_ring_or_repair_verify_evidence
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologySharedConsumersProjection {
    /// The updater service / update center consumes the shared install-topology grammar.
    pub updater_service_consumes_shared_install_topology: bool,
    /// The About / shell surface consumes the shared install-topology grammar.
    pub shell_about_consumes_shared_install_topology: bool,
    /// The diagnostics surface consumes the shared install-topology grammar.
    pub diagnostics_consumes_shared_install_topology: bool,
    /// The admin surface consumes the shared install-topology grammar.
    pub admin_consumes_shared_install_topology: bool,
    /// The installer / package-manager surface consumes the shared install-topology grammar.
    pub installer_consumes_shared_install_topology: bool,
    /// The docs / help surface consumes the shared install-topology grammar.
    pub docs_help_consumes_shared_install_topology: bool,
    /// The CLI / export path consumes the shared install-topology grammar.
    pub cli_export_consumes_shared_install_topology: bool,
    /// The support / export path consumes the shared install-topology grammar.
    pub support_export_consumes_shared_install_topology: bool,
    /// The general product UI / fleet-rollout surface consumes the shared install-topology grammar.
    pub product_ui_consumes_shared_install_topology: bool,
    /// Every family is adopted by two or more consumers.
    pub every_family_adopted_by_two_or_more_consumers: bool,
    /// Grammar is identical for the same delivery profile.
    pub install_topology_identical_for_same_profile: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a family back to one shared contract family.
    pub export_maps_back_to_one_install_topology_family: bool,
}

impl InstallTopologySharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.updater_service_consumes_shared_install_topology
            && self.shell_about_consumes_shared_install_topology
            && self.diagnostics_consumes_shared_install_topology
            && self.admin_consumes_shared_install_topology
            && self.installer_consumes_shared_install_topology
            && self.docs_help_consumes_shared_install_topology
            && self.cli_export_consumes_shared_install_topology
            && self.support_export_consumes_shared_install_topology
            && self.product_ui_consumes_shared_install_topology
            && self.every_family_adopted_by_two_or_more_consumers
            && self.install_topology_identical_for_same_profile
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_install_topology_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologySharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5InstallTopologySharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InstallTopologySharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<InstallTopologyConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<InstallTopologySharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5InstallTopologyConsumerSurface>,
    /// Trust review block.
    pub trust_review: InstallTopologySharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: InstallTopologySharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: InstallTopologySharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe install-topology shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologySharedConsumersPacket {
    /// Record kind; must equal [`M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<InstallTopologyConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<InstallTopologySharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5InstallTopologyConsumerSurface>,
    /// Trust review block.
    pub trust_review: InstallTopologySharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: InstallTopologySharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: InstallTopologySharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5InstallTopologySharedConsumersPacket {
    /// Builds an install-topology shared-consumer packet from stable-lane input.
    pub fn new(input: M5InstallTopologySharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the install-topology shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5InstallTopologySharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5InstallTopologySharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5InstallTopologySharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5InstallTopologySharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5InstallTopologySharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5InstallTopologySharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5InstallTopologySharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(M5InstallTopologySharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5InstallTopologySharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("install-topology shared-consumer packet serializes"),
        ) {
            violations.push(M5InstallTopologySharedConsumersViolation::RawBoundaryMaterialInExport);
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
            .expect("install-topology shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "family,consumer,representation,install_topology_role_word,parity_state\n",
        );
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.install_topology_role_word,
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
        out.push_str("# Shared Install-Topology Consumers: One Registry Across Surfaces\n\n");
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
                binding.delivery_profile_label,
                binding.binding_id,
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.install_topology_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in install-topology shared-consumer export.
#[derive(Debug)]
pub enum M5InstallTopologySharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5InstallTopologySharedConsumersViolation>),
}

impl fmt::Display for M5InstallTopologySharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "install-topology shared-consumer export parse failed: {error}"
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
                    "install-topology shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5InstallTopologySharedConsumersArtifactError {}

/// Validation failures emitted by [`M5InstallTopologySharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5InstallTopologySharedConsumersViolation {
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
    /// A binding's install-topology-role word is not a frozen role token.
    InstallTopologyRoleWordOutsideVocabulary,
    /// A binding's ownership-carrying role dropped its ownership identity.
    OwnershipIdentityMissingForOwnershipRole,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same delivery profile with different grammar.
    InstallTopologyGrammarDriftAcrossSurfaces,
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
    /// A binding lets portable mode write hidden machine-global durable state.
    PortableModeWritesHiddenMachineGlobalDurableState,
    /// A binding lets a preview channel reuse a stable state namespace without a handoff.
    PreviewChannelReusesStableStateNamespaceWithoutHandoff,
    /// A binding rolls back only the primary executable while sidecars drift.
    RollbackTargetsPrimaryExecutableWhileSidecarsDrift,
    /// A binding hides updater ownership or admin control in a managed flow.
    HidesUpdaterOwnershipOrAdminControlInManagedFlow,
    /// A binding publishes a deployment claim that outpaces ring or repair / verify evidence.
    PublishesDeploymentClaimOutpacingRingOrRepairVerifyEvidence,
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

impl M5InstallTopologySharedConsumersViolation {
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
            Self::InstallTopologyRoleWordOutsideVocabulary => {
                "install_topology_role_word_outside_vocabulary"
            }
            Self::OwnershipIdentityMissingForOwnershipRole => {
                "ownership_identity_missing_for_ownership_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::InstallTopologyGrammarDriftAcrossSurfaces => {
                "install_topology_grammar_drift_across_surfaces"
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
            Self::PortableModeWritesHiddenMachineGlobalDurableState => {
                "portable_mode_writes_hidden_machine_global_durable_state"
            }
            Self::PreviewChannelReusesStableStateNamespaceWithoutHandoff => {
                "preview_channel_reuses_stable_state_namespace_without_handoff"
            }
            Self::RollbackTargetsPrimaryExecutableWhileSidecarsDrift => {
                "rollback_targets_primary_executable_while_sidecars_drift"
            }
            Self::HidesUpdaterOwnershipOrAdminControlInManagedFlow => {
                "hides_updater_ownership_or_admin_control_in_managed_flow"
            }
            Self::PublishesDeploymentClaimOutpacingRingOrRepairVerifyEvidence => {
                "publishes_deployment_claim_outpacing_ring_or_repair_verify_evidence"
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

/// Reads and validates the checked-in stable install-topology shared-consumer export.
pub fn current_stable_m5_install_topology_shared_consumers_export(
) -> Result<M5InstallTopologySharedConsumersPacket, M5InstallTopologySharedConsumersArtifactError> {
    let packet: M5InstallTopologySharedConsumersPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-install-topology-shared-consumers-proof/support_export.json"
        )))
        .map_err(M5InstallTopologySharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5InstallTopologySharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5InstallTopologySharedConsumersPacket,
    violations: &mut Vec<M5InstallTopologySharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_SHARED_CONSUMERS_DOC_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
    ];
    // The five families map to two canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5InstallTopologyFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5InstallTopologySharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5InstallTopologySharedConsumersPacket,
    violations: &mut Vec<M5InstallTopologySharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5InstallTopologySharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One registry: the facet values must be identical for every binding that renders the same
    // delivery profile.
    let mut profile_facets: BTreeMap<&str, &InstallTopologyStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each family must be adopted by at least two distinct consumers.
    let mut family_consumers: BTreeMap<
        M5InstallTopologyFamily,
        BTreeSet<M5InstallTopologyConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5InstallTopologyConsumerSurface> = BTreeSet::new();
    let mut seen_families: BTreeSet<M5InstallTopologyFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.delivery_profile_id.trim().is_empty()
            || binding.delivery_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5InstallTopologySharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5InstallTopologySharedConsumersViolation::GrammarFacetIncomplete);
        }
        if !binding
            .state_facets
            .install_topology_role_word_in_vocabulary()
        {
            violations.push(
                M5InstallTopologySharedConsumersViolation::InstallTopologyRoleWordOutsideVocabulary,
            );
        }
        if !binding.state_facets.ownership_identity_satisfied() {
            violations.push(
                M5InstallTopologySharedConsumersViolation::OwnershipIdentityMissingForOwnershipRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5InstallTopologySharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5InstallTopologySharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5InstallTopologySharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5InstallTopologySharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5InstallTopologySharedConsumersViolation::NarrowNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5InstallTopologySharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5InstallTopologySharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5InstallTopologySharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5InstallTopologySharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.portable_mode_writes_hidden_machine_global_durable_state {
            violations.push(
                M5InstallTopologySharedConsumersViolation::PortableModeWritesHiddenMachineGlobalDurableState,
            );
        }
        if binding.preview_channel_reuses_stable_state_namespace_without_handoff {
            violations.push(
                M5InstallTopologySharedConsumersViolation::PreviewChannelReusesStableStateNamespaceWithoutHandoff,
            );
        }
        if binding.rollback_targets_primary_executable_while_sidecars_drift {
            violations.push(
                M5InstallTopologySharedConsumersViolation::RollbackTargetsPrimaryExecutableWhileSidecarsDrift,
            );
        }
        if binding.hides_updater_ownership_or_admin_control_in_managed_flow {
            violations.push(
                M5InstallTopologySharedConsumersViolation::HidesUpdaterOwnershipOrAdminControlInManagedFlow,
            );
        }
        if binding.publishes_deployment_claim_outpacing_ring_or_repair_verify_evidence {
            violations.push(
                M5InstallTopologySharedConsumersViolation::PublishesDeploymentClaimOutpacingRingOrRepairVerifyEvidence,
            );
        }

        // Support / export consumers must map a family back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5InstallTopologySharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_facets.get(binding.delivery_profile_id.as_str()) {
            None => {
                profile_facets.insert(binding.delivery_profile_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5InstallTopologySharedConsumersViolation::InstallTopologyGrammarDriftAcrossSurfaces,
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
    for consumer in M5InstallTopologyConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5InstallTopologySharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for family in M5InstallTopologyFamily::ALL {
        if !seen_families.contains(&family) {
            violations.push(M5InstallTopologySharedConsumersViolation::FamilyCoverageMissing);
            break;
        }
    }

    // Reuse: every present family must be adopted by two or more distinct consumers.
    for consumers in family_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5InstallTopologySharedConsumersViolation::FamilyReuseUnproven);
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
