//! Shared start-center, OS-open / system-association, CLI / headless, browser / deep-link, import,
//! help / docs, and support / export consumers that keep the B142 repository-bootstrap families —
//! open local, clone remote, open archive, import bundle, and resume snapshot — at **one canonical
//! registry** across every claimed M5 project-entry surface.
//!
//! This module is the consumer-adoption lane for the five governed acquisition families frozen in
//! [`crate::m5_repository_bootstrap_matrix`] and implemented by the source-locator / checkout-plan lane
//! ([`crate::m5_source_locator_and_checkout_plan_registries`]), the credential-posture / fetch-route lane
//! ([`crate::m5_bootstrap_credential_posture_and_fetch_route_registries`]), the staged-trust / post-open
//! queue lane ([`crate::m5_staged_trust_and_post_open_queue_registries`]), and the acquisition-evidence /
//! partial-recovery lane
//! ([`crate::m5_acquisition_evidence_and_partial_recovery_registries`]).
//!
//! It binds each shared repository-bootstrap family to the concrete acquisition-engine, shell, workspace,
//! git-service, trust-service, diagnostics, docs / help, CLI / export, and support-export consumers that
//! render it, and proves — by fixtures, not screenshots — that the same acquisition profile presents the
//! same repository-bootstrap-role, family, registry-reference, entry-context, surface-context, and
//! trust-stage-continuity grammar wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the five shared repository-bootstrap families must be adopted by at least two
//!    distinct consumers, so a family is proven to be shared acquisition-engine infrastructure rather than
//!    a one-surface, feature-local fork of source-locator, checkout-plan, or bootstrap-evidence copy.
//! 2. **One registry / no drift.** For a given acquisition profile every consumer surface must present
//!    identical [`RepositoryBootstrapStateFacetValues`] — the same repository-bootstrap-role word, the same
//!    family word, the same registry-reference word, the same entry-context word, the same surface-context
//!    word, and the same trust-stage-continuity word. The repository-bootstrap-role word must be a token
//!    from the frozen [`M5RepositoryBootstrapRole`] vocabulary, so no surface rewrites `source_locator`,
//!    `checkout_plan`, `credential_posture`, `evidence_packet`, `staged_trust`, `resumable_acquisition`, or
//!    `post_open_queue` in its own words. A surface may narrow *how much* it shows across desktop, compact,
//!    remote, and exported representations, but it may never reword the underlying grammar per surface, and
//!    a role that carries credential-posture, evidence-packet, staged-trust, or post-open-queue meaning may
//!    never rewrite clone into open because a local checkout already exists, run a repo-owned action
//!    implicitly during acquisition, lose signer or mirror provenance across an offline or mirrored fetch,
//!    strand a partial acquisition without Resume / Discard / open-read-only-partial-root choices, or hide
//!    the bootstrap credential posture behind generic connected-state copy.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the canonical per-domain
//!    schema and the frozen matrix by id, so an exported packet can always map a shell / workspace /
//!    git-service / diagnostics entry surface back to one shared contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an explicit
//! [`RepositoryBootstrapNarrowNote`] naming the reason, the preserved grammar, and the next action, and an
//! exported representation additionally names its export-safe detail boundary rather than collapsing the
//! profile out of view.
//!
//! The packet references upstream repository-bootstrap contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/workspaces/m5-repository-bootstrap-shared-consumers.schema.json`](../../../../schemas/workspaces/m5-repository-bootstrap-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/workspaces/m5_repository_bootstrap_shared_consumers_one_registry.md`](../../../../docs/workspaces/m5_repository_bootstrap_shared_consumers_one_registry.md).
//! The protected fixture directory is
//! [`fixtures/workspaces/m5-repository-bootstrap-shared-consumers/`](../../../../fixtures/workspaces/m5-repository-bootstrap-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_repository_bootstrap_shared_consumers,
    seeded_m5_repository_bootstrap_shared_consumers_compact_remote_narrowed,
    seeded_m5_repository_bootstrap_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_repository_bootstrap_matrix::{
    M5RepositoryBootstrapConsumerSurface, M5RepositoryBootstrapFamily, M5RepositoryBootstrapRole,
    M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF, M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5RepositoryBootstrapSharedConsumersPacket`].
pub const M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_repository_bootstrap_shared_consumer_registry_parity";

/// Schema version for repository-bootstrap shared-consumer parity records.
pub const M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-repository-bootstrap-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/workspaces/m5-repository-bootstrap-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/workspaces/m5_repository_bootstrap_shared_consumers_one_registry.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-repository-bootstrap-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-repository-bootstrap-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-repository-bootstrap-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/workspaces/m5-repository-bootstrap-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Trust-stage-continuity sentinel words a credential-posture / evidence-packet / staged-trust /
/// post-open-queue role may never fall back to; a trust-carrying role that changes entry presentation must
/// always keep a real staged-trust-and-provenance-disclosed continuity, never running a repo-owned action
/// implicitly, auto-executing a post-open queue, losing signer or mirror provenance, or hiding the
/// credential posture.
const TRUST_STAGE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "ran_repo_owned_action_implicitly",
    "auto_executed_post_open_queue",
    "lost_signer_or_mirror_provenance",
    "hid_credential_posture",
];

/// Whether a consumer surface is an export / support path that must map a family back to its
/// canonical contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5RepositoryBootstrapConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5RepositoryBootstrapConsumerSurface::SupportExport
            | M5RepositoryBootstrapConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5RepositoryBootstrapRole`] vocabulary.
///
/// This is the "one registry" gate: an acquisition profile's repository-bootstrap-role word must be a
/// controlled role token rather than a per-surface synonym.
pub fn is_known_repository_bootstrap_role_token(token: &str) -> bool {
    repository_bootstrap_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5RepositoryBootstrapRole`], if it is one.
pub fn repository_bootstrap_role_from_token(token: &str) -> Option<M5RepositoryBootstrapRole> {
    M5RepositoryBootstrapRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared repository-bootstrap family a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying grammar: a narrowed representation still
/// carries the same repository-bootstrap-role, family, registry-reference, entry-context, surface-context,
/// and trust-stage-continuity words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryBootstrapRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl RepositoryBootstrapRepresentation {
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
pub enum RepositoryBootstrapParityFacet {
    /// The frozen repository-bootstrap-role word.
    RepositoryBootstrapRoleWord,
    /// The repository-bootstrap-family word.
    FamilyWord,
    /// The canonical registry-reference word the family points at.
    RegistryReferenceWord,
    /// The entry-context word (first run / returning workspace / offline or air-gapped / mirrored
    /// registry / resumed after interrupt) the profile ships.
    EntryContextWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The trust-stage-continuity word paired with a credential-posture / evidence-packet /
    /// staged-trust / post-open-queue role.
    TrustStageContinuityWord,
}

impl RepositoryBootstrapParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RepositoryBootstrapRoleWord,
        Self::FamilyWord,
        Self::RegistryReferenceWord,
        Self::EntryContextWord,
        Self::SurfaceContextWord,
        Self::TrustStageContinuityWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepositoryBootstrapRoleWord => "repository_bootstrap_role_word",
            Self::FamilyWord => "family_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::EntryContextWord => "entry_context_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::TrustStageContinuityWord => "trust_stage_continuity_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared repository-bootstrap family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryBootstrapNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl RepositoryBootstrapNarrowReason {
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
pub enum RepositoryBootstrapNarrowNextAction {
    /// Expand the family in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl RepositoryBootstrapNarrowNextAction {
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
pub enum RepositoryBootstrapParityState {
    /// All grammar is preserved and shown in full.
    FacetsPreserved,
    /// All grammar is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl RepositoryBootstrapParityState {
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
pub enum RepositoryBootstrapSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Repository-bootstrap grammar drifted between surfaces for the same profile.
    RepositoryBootstrapGrammarDriftDetected,
    /// A trust-carrying role dropped its trust-stage or provenance-disclosure meaning.
    TrustStageOrProvenanceDropped,
    /// A surface rewrote clone into open because a local checkout already exists.
    RewritesCloneIntoOpenWhenLocalCheckoutAlreadyExists,
    /// A surface ran repo-owned actions implicitly during acquisition.
    RunsRepoOwnedActionsImplicitlyDuringAcquisition,
    /// A surface lost signer or mirror provenance across an offline or mirrored fetch.
    LosesSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches,
    /// A surface stranded partial acquisition without Resume / Discard / read-only choices.
    StrandsPartialAcquisitionWithoutResumeDiscardOrReadonlyChoices,
    /// A surface hid the bootstrap credential posture behind generic connected-state copy.
    HidesBootstrapCredentialPostureBehindGenericConnectedStateCopy,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared repository-bootstrap family narrowed.
    UpstreamRepositoryBootstrapNarrowed,
}

impl RepositoryBootstrapSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::RepositoryBootstrapGrammarDriftDetected,
        Self::TrustStageOrProvenanceDropped,
        Self::RewritesCloneIntoOpenWhenLocalCheckoutAlreadyExists,
        Self::RunsRepoOwnedActionsImplicitlyDuringAcquisition,
        Self::LosesSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches,
        Self::StrandsPartialAcquisitionWithoutResumeDiscardOrReadonlyChoices,
        Self::HidesBootstrapCredentialPostureBehindGenericConnectedStateCopy,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamRepositoryBootstrapNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::RepositoryBootstrapGrammarDriftDetected => {
                "repository_bootstrap_grammar_drift_detected"
            }
            Self::TrustStageOrProvenanceDropped => "trust_stage_or_provenance_dropped",
            Self::RewritesCloneIntoOpenWhenLocalCheckoutAlreadyExists => {
                "rewrites_clone_into_open_when_local_checkout_already_exists"
            }
            Self::RunsRepoOwnedActionsImplicitlyDuringAcquisition => {
                "runs_repo_owned_actions_implicitly_during_acquisition"
            }
            Self::LosesSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches => {
                "loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches"
            }
            Self::StrandsPartialAcquisitionWithoutResumeDiscardOrReadonlyChoices => {
                "strands_partial_acquisition_without_resume_discard_or_readonly_choices"
            }
            Self::HidesBootstrapCredentialPostureBehindGenericConnectedStateCopy => {
                "hides_bootstrap_credential_posture_behind_generic_connected_state_copy"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamRepositoryBootstrapNarrowed => "upstream_repository_bootstrap_narrowed",
        }
    }
}

/// The controlled grammar an acquisition profile presents.
///
/// These six words must be identical across every consumer surface that shows the same acquisition
/// profile. The repository-bootstrap-role word must be a frozen role token; the rest are controlled words
/// the profile's family carries. A surface may narrow how much it renders, but it may never reword any of
/// these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapStateFacetValues {
    /// Repository-bootstrap-role word (must be a frozen [`M5RepositoryBootstrapRole`] token).
    pub repository_bootstrap_role_word: String,
    /// Repository-bootstrap-family word.
    pub family_word: String,
    /// Canonical registry-reference word the family points at.
    pub registry_reference_word: String,
    /// Entry-context word (first run / returning workspace / offline or air-gapped / mirrored registry /
    /// resumed after interrupt) the profile ships.
    pub entry_context_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Trust-stage-continuity word paired with a credential-posture / evidence-packet / staged-trust /
    /// post-open-queue role.
    pub trust_stage_continuity_word: String,
}

impl RepositoryBootstrapStateFacetValues {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.repository_bootstrap_role_word.trim().is_empty()
            && !self.family_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.entry_context_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.trust_stage_continuity_word.trim().is_empty()
    }

    /// Whether the repository-bootstrap-role word is a member of the frozen role vocabulary.
    pub fn repository_bootstrap_role_word_in_vocabulary(&self) -> bool {
        is_known_repository_bootstrap_role_token(self.repository_bootstrap_role_word.trim())
    }

    /// Whether the profile honours the trust-stage rule: a role that carries credential-posture,
    /// evidence-packet, staged-trust, or post-open-queue meaning must pair its entry change with a real
    /// staged-trust-and-provenance-disclosed continuity and never collapse to a
    /// ran-repo-owned-action-implicitly, auto-executed-post-open-queue, lost-signer-or-mirror-provenance,
    /// or hid-credential-posture sentinel.
    pub fn trust_stage_continuity_satisfied(&self) -> bool {
        match repository_bootstrap_role_from_token(self.repository_bootstrap_role_word.trim()) {
            Some(role) if role.must_stage_trust_and_disclose_provenance_before_bootstrap() => {
                let continuity = self.trust_stage_continuity_word.trim().to_lowercase();
                !continuity.is_empty()
                    && !TRUST_STAGE_ABSENT_SENTINELS.contains(&continuity.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapNarrowNote {
    /// Why the representation narrowed.
    pub reason: RepositoryBootstrapNarrowReason,
    /// Note naming the preserved grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: RepositoryBootstrapNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryBootstrapRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: RepositoryBootstrapParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<RepositoryBootstrapNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<RepositoryBootstrapNarrowNextAction>,
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
pub const fn resolve_repository_bootstrap_render_disclosure(
    representation: RepositoryBootstrapRepresentation,
) -> RepositoryBootstrapRenderDisclosure {
    match representation {
        RepositoryBootstrapRepresentation::DesktopFull => RepositoryBootstrapRenderDisclosure {
            parity_state: RepositoryBootstrapParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        RepositoryBootstrapRepresentation::CompactNarrowed => RepositoryBootstrapRenderDisclosure {
            parity_state: RepositoryBootstrapParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(RepositoryBootstrapNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(RepositoryBootstrapNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        RepositoryBootstrapRepresentation::RemoteProjected => RepositoryBootstrapRenderDisclosure {
            parity_state: RepositoryBootstrapParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(RepositoryBootstrapNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(RepositoryBootstrapNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        RepositoryBootstrapRepresentation::ExportedRedacted => {
            RepositoryBootstrapRenderDisclosure {
                parity_state: RepositoryBootstrapParityState::FacetsDisclosedNarrowed,
                narrow_reason: Some(RepositoryBootstrapNarrowReason::ExportRedactionNarrowed),
                narrow_next_action: Some(RepositoryBootstrapNarrowNextAction::OpenFullDetail),
                needs_narrow_note: true,
                needs_remote_source_note: false,
                needs_export_detail_note: true,
            }
        }
    }
}

/// One consumer binding: a shared repository-bootstrap family rendered on one consumer surface in one
/// representation for one acquisition profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable acquisition-profile id (shared across surfaces that show the same profile).
    pub bootstrap_profile_id: String,
    /// Human-readable acquisition-profile identity.
    pub bootstrap_profile_label: String,
    /// Which shared repository-bootstrap family this binding renders.
    pub family: M5RepositoryBootstrapFamily,
    /// Which consumer surface renders it.
    pub consumer: M5RepositoryBootstrapConsumerSurface,
    /// Which representation this surface renders.
    pub representation: RepositoryBootstrapRepresentation,
    /// The controlled grammar presented (identical across surfaces for one profile).
    pub state_facets: RepositoryBootstrapStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: RepositoryBootstrapParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<RepositoryBootstrapNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface rewrites clone into open because a local checkout already exists. MUST be
    /// `false`.
    pub rewrites_clone_into_open_when_local_checkout_already_exists: bool,
    /// Guardrail: this surface runs repo-owned actions implicitly during acquisition. MUST be `false`.
    pub runs_repo_owned_actions_implicitly_during_acquisition: bool,
    /// Guardrail: this surface loses signer or mirror provenance across an offline or mirrored fetch. MUST
    /// be `false`.
    pub loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches: bool,
    /// Guardrail: this surface strands partial acquisition without Resume / Discard / read-only choices.
    /// MUST be `false`.
    pub strands_partial_acquisition_without_resume_discard_or_readonly_choices: bool,
    /// Guardrail: this surface hides the bootstrap credential posture behind generic connected-state copy.
    /// MUST be `false`.
    pub hides_bootstrap_credential_posture_behind_generic_connected_state_copy: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl RepositoryBootstrapConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> RepositoryBootstrapRenderDisclosure {
        resolve_repository_bootstrap_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.rewrites_clone_into_open_when_local_checkout_already_exists
            && !self.runs_repo_owned_actions_implicitly_during_acquisition
            && !self.loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches
            && !self.strands_partial_acquisition_without_resume_discard_or_readonly_choices
            && !self.hides_bootstrap_credential_posture_behind_generic_connected_state_copy
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
                .any(|reference| reference == M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapSharedConsumersTrustReview {
    /// Family reuse is proven by fixtures rather than inferred from screenshots.
    pub family_reuse_proven_by_fixtures: bool,
    /// The same acquisition profile presents the same grammar across surfaces.
    pub same_profile_same_repository_bootstrap_across_surfaces: bool,
    /// Every repository-bootstrap-role word is a frozen role token.
    pub repository_bootstrap_role_words_stay_in_frozen_vocabulary: bool,
    /// Trust-carrying roles never run a repo-owned action implicitly or lose provenance.
    pub trust_roles_never_run_repo_actions_or_lose_provenance: bool,
    /// A surface never rewrites clone into open when a local checkout already exists.
    pub acquisition_never_rewrites_clone_into_open_over_existing_checkout: bool,
    /// A surface never runs repo-owned actions implicitly during acquisition.
    pub acquisition_never_runs_repo_owned_actions_implicitly: bool,
    /// A surface never loses signer or mirror provenance across an offline or mirrored fetch.
    pub acquisition_never_loses_signer_or_mirror_provenance: bool,
    /// A partial acquisition is never stranded without Resume / Discard / read-only choices.
    pub partial_acquisition_never_stranded_without_resume_discard_or_readonly: bool,
    /// The bootstrap credential posture is never hidden behind generic connected-state copy.
    pub bootstrap_credential_posture_never_hidden_behind_generic_copy: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl RepositoryBootstrapSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.family_reuse_proven_by_fixtures
            && self.same_profile_same_repository_bootstrap_across_surfaces
            && self.repository_bootstrap_role_words_stay_in_frozen_vocabulary
            && self.trust_roles_never_run_repo_actions_or_lose_provenance
            && self.acquisition_never_rewrites_clone_into_open_over_existing_checkout
            && self.acquisition_never_runs_repo_owned_actions_implicitly
            && self.acquisition_never_loses_signer_or_mirror_provenance
            && self.partial_acquisition_never_stranded_without_resume_discard_or_readonly
            && self.bootstrap_credential_posture_never_hidden_behind_generic_copy
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapSharedConsumersProjection {
    /// The acquisition engine consumes the shared repository-bootstrap grammar.
    pub acquisition_engine_consumes_shared_repository_bootstrap: bool,
    /// The shell UI consumes the shared repository-bootstrap grammar.
    pub shell_ui_consumes_shared_repository_bootstrap: bool,
    /// The workspace service consumes the shared repository-bootstrap grammar.
    pub workspace_service_consumes_shared_repository_bootstrap: bool,
    /// The git service consumes the shared repository-bootstrap grammar.
    pub git_service_consumes_shared_repository_bootstrap: bool,
    /// The trust service consumes the shared repository-bootstrap grammar.
    pub trust_service_consumes_shared_repository_bootstrap: bool,
    /// The diagnostics surface consumes the shared repository-bootstrap grammar.
    pub diagnostics_consumes_shared_repository_bootstrap: bool,
    /// The docs / help surface consumes the shared repository-bootstrap grammar.
    pub docs_help_consumes_shared_repository_bootstrap: bool,
    /// The CLI / export path consumes the shared repository-bootstrap grammar.
    pub cli_export_consumes_shared_repository_bootstrap: bool,
    /// The support / export path consumes the shared repository-bootstrap grammar.
    pub support_export_consumes_shared_repository_bootstrap: bool,
    /// Every family is adopted by two or more consumers.
    pub every_family_adopted_by_two_or_more_consumers: bool,
    /// Grammar is identical for the same acquisition profile.
    pub repository_bootstrap_identical_for_same_profile: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a family back to one shared contract family.
    pub export_maps_back_to_one_repository_bootstrap_family: bool,
}

impl RepositoryBootstrapSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.acquisition_engine_consumes_shared_repository_bootstrap
            && self.shell_ui_consumes_shared_repository_bootstrap
            && self.workspace_service_consumes_shared_repository_bootstrap
            && self.git_service_consumes_shared_repository_bootstrap
            && self.trust_service_consumes_shared_repository_bootstrap
            && self.diagnostics_consumes_shared_repository_bootstrap
            && self.docs_help_consumes_shared_repository_bootstrap
            && self.cli_export_consumes_shared_repository_bootstrap
            && self.support_export_consumes_shared_repository_bootstrap
            && self.every_family_adopted_by_two_or_more_consumers
            && self.repository_bootstrap_identical_for_same_profile
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_repository_bootstrap_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5RepositoryBootstrapSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RepositoryBootstrapSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<RepositoryBootstrapConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<RepositoryBootstrapSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5RepositoryBootstrapConsumerSurface>,
    /// Trust review block.
    pub trust_review: RepositoryBootstrapSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RepositoryBootstrapSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: RepositoryBootstrapSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe repository-bootstrap shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepositoryBootstrapSharedConsumersPacket {
    /// Record kind; must equal [`M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<RepositoryBootstrapConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<RepositoryBootstrapSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5RepositoryBootstrapConsumerSurface>,
    /// Trust review block.
    pub trust_review: RepositoryBootstrapSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RepositoryBootstrapSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: RepositoryBootstrapSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RepositoryBootstrapSharedConsumersPacket {
    /// Builds a repository-bootstrap shared-consumer packet from stable-lane input.
    pub fn new(input: M5RepositoryBootstrapSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the repository-bootstrap shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5RepositoryBootstrapSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(M5RepositoryBootstrapSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(M5RepositoryBootstrapSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(M5RepositoryBootstrapSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("repository-bootstrap shared-consumer packet serializes"),
        ) {
            violations
                .push(M5RepositoryBootstrapSharedConsumersViolation::RawBoundaryMaterialInExport);
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
            .expect("repository-bootstrap shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "family,consumer,representation,repository_bootstrap_role_word,parity_state\n",
        );
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.repository_bootstrap_role_word,
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
        out.push_str("# Shared Repository-Bootstrap Consumers: One Registry Across Surfaces\n\n");
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
                binding.bootstrap_profile_label,
                binding.binding_id,
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.repository_bootstrap_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in repository-bootstrap shared-consumer export.
#[derive(Debug)]
pub enum M5RepositoryBootstrapSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RepositoryBootstrapSharedConsumersViolation>),
}

impl fmt::Display for M5RepositoryBootstrapSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "repository-bootstrap shared-consumer export parse failed: {error}"
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
                    "repository-bootstrap shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RepositoryBootstrapSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5RepositoryBootstrapSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RepositoryBootstrapSharedConsumersViolation {
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
    /// A binding's repository-bootstrap-role word is not a frozen role token.
    RepositoryBootstrapRoleWordOutsideVocabulary,
    /// A binding's trust-carrying role dropped its trust-stage continuity.
    TrustStageContinuityMissingForTrustRole,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same acquisition profile with different grammar.
    RepositoryBootstrapGrammarDriftAcrossSurfaces,
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
    /// A binding rewrites clone into open because a local checkout already exists.
    RewritesCloneIntoOpenWhenLocalCheckoutAlreadyExists,
    /// A binding runs repo-owned actions implicitly during acquisition.
    RunsRepoOwnedActionsImplicitlyDuringAcquisition,
    /// A binding loses signer or mirror provenance across an offline or mirrored fetch.
    LosesSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches,
    /// A binding strands partial acquisition without Resume / Discard / read-only choices.
    StrandsPartialAcquisitionWithoutResumeDiscardOrReadonlyChoices,
    /// A binding hides the bootstrap credential posture behind generic connected-state copy.
    HidesBootstrapCredentialPostureBehindGenericConnectedStateCopy,
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

impl M5RepositoryBootstrapSharedConsumersViolation {
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
            Self::RepositoryBootstrapRoleWordOutsideVocabulary => {
                "repository_bootstrap_role_word_outside_vocabulary"
            }
            Self::TrustStageContinuityMissingForTrustRole => {
                "trust_stage_continuity_missing_for_trust_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::RepositoryBootstrapGrammarDriftAcrossSurfaces => {
                "repository_bootstrap_grammar_drift_across_surfaces"
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
            Self::RewritesCloneIntoOpenWhenLocalCheckoutAlreadyExists => {
                "rewrites_clone_into_open_when_local_checkout_already_exists"
            }
            Self::RunsRepoOwnedActionsImplicitlyDuringAcquisition => {
                "runs_repo_owned_actions_implicitly_during_acquisition"
            }
            Self::LosesSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches => {
                "loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches"
            }
            Self::StrandsPartialAcquisitionWithoutResumeDiscardOrReadonlyChoices => {
                "strands_partial_acquisition_without_resume_discard_or_readonly_choices"
            }
            Self::HidesBootstrapCredentialPostureBehindGenericConnectedStateCopy => {
                "hides_bootstrap_credential_posture_behind_generic_connected_state_copy"
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

/// Reads and validates the checked-in stable repository-bootstrap shared-consumer export.
pub fn current_stable_m5_repository_bootstrap_shared_consumers_export() -> Result<
    M5RepositoryBootstrapSharedConsumersPacket,
    M5RepositoryBootstrapSharedConsumersArtifactError,
> {
    let packet: M5RepositoryBootstrapSharedConsumersPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-repository-bootstrap-shared-consumers-proof/support_export.json"
        )))
        .map_err(M5RepositoryBootstrapSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RepositoryBootstrapSharedConsumersArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5RepositoryBootstrapSharedConsumersPacket,
    violations: &mut Vec<M5RepositoryBootstrapSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_SHARED_CONSUMERS_DOC_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
    ];
    // The five families map to three canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5RepositoryBootstrapFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5RepositoryBootstrapSharedConsumersPacket,
    violations: &mut Vec<M5RepositoryBootstrapSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5RepositoryBootstrapSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One registry: the facet values must be identical for every binding that renders the same
    // acquisition profile.
    let mut profile_facets: BTreeMap<&str, &RepositoryBootstrapStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each family must be adopted by at least two distinct consumers.
    let mut family_consumers: BTreeMap<
        M5RepositoryBootstrapFamily,
        BTreeSet<M5RepositoryBootstrapConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5RepositoryBootstrapConsumerSurface> = BTreeSet::new();
    let mut seen_families: BTreeSet<M5RepositoryBootstrapFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.bootstrap_profile_id.trim().is_empty()
            || binding.bootstrap_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::GrammarFacetIncomplete);
        }
        if !binding
            .state_facets
            .repository_bootstrap_role_word_in_vocabulary()
        {
            violations.push(
                M5RepositoryBootstrapSharedConsumersViolation::RepositoryBootstrapRoleWordOutsideVocabulary,
            );
        }
        if !binding.state_facets.trust_stage_continuity_satisfied() {
            violations.push(
                M5RepositoryBootstrapSharedConsumersViolation::TrustStageContinuityMissingForTrustRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations
                        .push(M5RepositoryBootstrapSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations.push(
                            M5RepositoryBootstrapSharedConsumersViolation::NarrowReasonMismatch,
                        );
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5RepositoryBootstrapSharedConsumersViolation::NarrowNextActionMismatch,
                        );
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5RepositoryBootstrapSharedConsumersViolation::NarrowNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5RepositoryBootstrapSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.rewrites_clone_into_open_when_local_checkout_already_exists {
            violations.push(
                M5RepositoryBootstrapSharedConsumersViolation::RewritesCloneIntoOpenWhenLocalCheckoutAlreadyExists,
            );
        }
        if binding.runs_repo_owned_actions_implicitly_during_acquisition {
            violations.push(
                M5RepositoryBootstrapSharedConsumersViolation::RunsRepoOwnedActionsImplicitlyDuringAcquisition,
            );
        }
        if binding.loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches {
            violations.push(
                M5RepositoryBootstrapSharedConsumersViolation::LosesSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches,
            );
        }
        if binding.strands_partial_acquisition_without_resume_discard_or_readonly_choices {
            violations.push(
                M5RepositoryBootstrapSharedConsumersViolation::StrandsPartialAcquisitionWithoutResumeDiscardOrReadonlyChoices,
            );
        }
        if binding.hides_bootstrap_credential_posture_behind_generic_connected_state_copy {
            violations.push(
                M5RepositoryBootstrapSharedConsumersViolation::HidesBootstrapCredentialPostureBehindGenericConnectedStateCopy,
            );
        }

        // Support / export consumers must map a family back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5RepositoryBootstrapSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_facets.get(binding.bootstrap_profile_id.as_str()) {
            None => {
                profile_facets.insert(binding.bootstrap_profile_id.as_str(), &binding.state_facets);
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5RepositoryBootstrapSharedConsumersViolation::RepositoryBootstrapGrammarDriftAcrossSurfaces,
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
    for consumer in M5RepositoryBootstrapConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for family in M5RepositoryBootstrapFamily::ALL {
        if !seen_families.contains(&family) {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::FamilyCoverageMissing);
            break;
        }
    }

    // Reuse: every present family must be adopted by two or more distinct consumers.
    for consumers in family_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5RepositoryBootstrapSharedConsumersViolation::FamilyReuseUnproven);
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
