//! Shared Start Center, system-open / callback, editor, terminal, settings, help / about, auth /
//! credential, notification, and support / export consumers that keep the B139 platform-fit
//! families — platform conventions, shortcut notation, file / path / reveal terminology, live
//! theme / contrast response, credential-store wording, and input-method behavior — at **one
//! convention** across every claimed M5 desktop surface.
//!
//! This module is the consumer-adoption lane for the six reusable platform-fit families frozen in
//! [`crate::m5_platform_fit_matrix`] and implemented by the shortcut-notation / command-label lane
//! ([`crate::m5_shortcut_notation_and_command_label_registries`]), the file-path-reveal / native
//! window-menu lane ([`crate::m5_file_path_reveal_and_native_window_menu_registries`]), the live
//! theme / contrast lane ([`crate::m5_system_appearance_live_apply_and_source_provenance_registries`]),
//! and the input-method / credential-store-wording lane
//! ([`crate::m5_input_method_and_credential_store_wording_registries`]).
//!
//! It binds each shared platform-fit family to the concrete shell, settings, auth, input, docs /
//! help, onboarding, CLI / export, support-export, and general product consumers that render it, and
//! proves — by fixtures, not screenshots — that the same platform-fit object presents the same
//! platform-fit-role, family, registry-reference, host-platform, surface-context, and
//! command-identity grammar wherever it appears.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Reuse.** Each of the six shared platform-fit families must be adopted by at least two
//!    distinct consumers, so a family is proven to be shared platform-fit infrastructure rather than
//!    a one-surface, feature-local fork of shortcut notation, path wording, appearance response,
//!    credential copy, or input handling.
//! 2. **One convention / no drift.** For a given platform-fit object every consumer surface must
//!    present identical [`PlatformFitStateFacetValues`] — the same platform-fit-role word, the same
//!    family word, the same registry-reference word, the same host-platform word, the same
//!    surface-context word, and the same command-identity word. The platform-fit-role word must be a
//!    token from the frozen [`M5PlatformFitRole`] vocabulary, so no surface rewrites `shortcut`,
//!    `window_menu`, `path_terminology`, `appearance`, `credential_wording`, `input_fidelity`, or
//!    `command_stability` in its own words. A surface may narrow *how much* it shows across desktop,
//!    compact, remote, and exported representations, but it may never reword the underlying grammar
//!    per surface, and a role that carries shortcut, window-menu, input-fidelity, or
//!    command-stability meaning may never let a platform-specific label change command or permission
//!    meaning, hide a primary action only in OS chrome, silently fall back to plaintext secret
//!    storage, corrupt input text or trust fidelity, or mislabel a shortcut or path / reveal verb.
//! 3. **Map back to one family.** Support and CLI/export consumers must point at the canonical
//!    per-domain schema and the frozen matrix by id, so an exported packet can always map a shell /
//!    settings / auth / input / docs platform-fit surface back to one shared contract family.
//!
//! Narrowing is disclosed, never hidden: a compact, remote, or exported representation carries an
//! explicit [`PlatformFitNarrowNote`] naming the reason, the preserved grammar, and the next action,
//! and an exported representation additionally names its export-safe detail boundary rather than
//! collapsing the object out of view.
//!
//! The packet references upstream platform-fit contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/platform/m5-platform-fit-shared-consumers.schema.json`](../../../../schemas/platform/m5-platform-fit-shared-consumers.schema.json).
//! The contract doc is
//! [`docs/platform/m5_platform_fit_shared_consumers_one_convention.md`](../../../../docs/platform/m5_platform_fit_shared_consumers_one_convention.md).
//! The protected fixture directory is
//! [`fixtures/platform/m5-platform-fit-shared-consumers/`](../../../../fixtures/platform/m5-platform-fit-shared-consumers/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_platform_fit_shared_consumers,
    seeded_m5_platform_fit_shared_consumers_compact_remote_narrowed,
    seeded_m5_platform_fit_shared_consumers_exported_redaction_narrowed,
};

use crate::m5_platform_fit_matrix::{
    M5PlatformFitConsumerSurface, M5PlatformFitFamily, M5PlatformFitRole,
    M5_PLATFORM_FIT_MATRIX_DOC_REF, M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5PlatformFitSharedConsumersPacket`].
pub const M5_PLATFORM_FIT_SHARED_CONSUMERS_RECORD_KIND: &str =
    "m5_platform_fit_shared_consumer_convention_parity";

/// Schema version for platform-fit shared-consumer parity records.
pub const M5_PLATFORM_FIT_SHARED_CONSUMERS_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_PLATFORM_FIT_SHARED_CONSUMERS_PACKET_ID: &str =
    "m5-platform-fit-shared-consumers:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_PLATFORM_FIT_SHARED_CONSUMERS_SCHEMA_REF: &str =
    "schemas/platform/m5-platform-fit-shared-consumers.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PLATFORM_FIT_SHARED_CONSUMERS_DOC_REF: &str =
    "docs/platform/m5_platform_fit_shared_consumers_one_convention.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PLATFORM_FIT_SHARED_CONSUMERS_ARTIFACT_REF: &str =
    "artifacts/release/m5-platform-fit-shared-consumers-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_PLATFORM_FIT_SHARED_CONSUMERS_CSV_REF: &str =
    "artifacts/release/m5-platform-fit-shared-consumers-proof/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_PLATFORM_FIT_SHARED_CONSUMERS_REPORT_REF: &str =
    "artifacts/release/m5-platform-fit-shared-consumers-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_PLATFORM_FIT_SHARED_CONSUMERS_FIXTURE_DIR: &str =
    "fixtures/platform/m5-platform-fit-shared-consumers";

/// Proof-freshness SLO in hours for this lane.
pub const M5_PLATFORM_FIT_SHARED_CONSUMERS_PROOF_SLO_HOURS: u32 = 720;

/// Command-identity sentinel words a shortcut / window-menu / input-fidelity / command-stability role
/// may never fall back to; an adaptation-carrying role that changes platform presentation must always
/// keep a real preserved command identity, never renaming the command, changing its permission
/// meaning, corrupting its input text, or hiding it only in OS chrome.
const COMMAND_IDENTITY_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "renamed_command",
    "permission_changed",
    "text_corrupted",
    "hidden_in_os_chrome",
];

/// Whether a consumer surface is an export / support path that must map a family back to its
/// canonical contract by id.
pub const fn consumer_must_reference_canonical(consumer: M5PlatformFitConsumerSurface) -> bool {
    matches!(
        consumer,
        M5PlatformFitConsumerSurface::SupportExport | M5PlatformFitConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5PlatformFitRole`] vocabulary.
///
/// This is the "one convention" gate: a platform-fit object's platform-fit-role word must be a
/// controlled role token rather than a per-surface synonym.
pub fn is_known_platform_fit_role_token(token: &str) -> bool {
    platform_fit_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5PlatformFitRole`], if it is one.
pub fn platform_fit_role_from_token(token: &str) -> Option<M5PlatformFitRole> {
    M5PlatformFitRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// How much of a shared platform-fit family a consumer renders for one representation.
///
/// Narrowing changes how much is shown, never the underlying grammar: a narrowed representation still
/// carries the same platform-fit-role, family, registry-reference, host-platform, surface-context,
/// and command-identity words, and discloses the narrowing through an explicit note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformFitRepresentation {
    /// The full desktop representation; nothing is narrowed.
    DesktopFull,
    /// A compact representation that narrows disclosure depth.
    CompactNarrowed,
    /// A remote-projected representation backed by a remote source.
    RemoteProjected,
    /// An exported, export-safe-redacted representation.
    ExportedRedacted,
}

impl PlatformFitRepresentation {
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

/// A grammar axis whose word must stay identical across surfaces for one object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformFitParityFacet {
    /// The frozen platform-fit-role word.
    PlatformFitRoleWord,
    /// The platform-fit-family word.
    FamilyWord,
    /// The canonical registry-reference word the family points at.
    RegistryReferenceWord,
    /// The host-platform word (macOS / Windows / Linux adaptation).
    HostPlatformWord,
    /// The surface-context word.
    SurfaceContextWord,
    /// The command-identity word paired with a shortcut / window-menu / input / command-stability role.
    CommandIdentityWord,
}

impl PlatformFitParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PlatformFitRoleWord,
        Self::FamilyWord,
        Self::RegistryReferenceWord,
        Self::HostPlatformWord,
        Self::SurfaceContextWord,
        Self::CommandIdentityWord,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlatformFitRoleWord => "platform_fit_role_word",
            Self::FamilyWord => "family_word",
            Self::RegistryReferenceWord => "registry_reference_word",
            Self::HostPlatformWord => "host_platform_word",
            Self::SurfaceContextWord => "surface_context_word",
            Self::CommandIdentityWord => "command_identity_word",
        }
    }
}

/// Why a surface narrowed its rendering of a shared platform-fit family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformFitNarrowReason {
    /// A compact representation narrowed disclosure depth.
    CompactionNarrowed,
    /// A remote-projected representation narrowed to remote-backed truth.
    RemoteProjectionNarrowed,
    /// An exported representation narrowed to export-safe-redacted truth.
    ExportRedactionNarrowed,
}

impl PlatformFitNarrowReason {
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
pub enum PlatformFitNarrowNextAction {
    /// Expand the family in the full desktop representation.
    ExpandInDesktop,
    /// Open the remote source backing the projection.
    OpenRemoteSource,
    /// Open the full detail behind the redacted export.
    OpenFullDetail,
}

impl PlatformFitNarrowNextAction {
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
pub enum PlatformFitParityState {
    /// All grammar is preserved and shown in full.
    FacetsPreserved,
    /// All grammar is preserved and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl PlatformFitParityState {
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
pub enum PlatformFitSharedConsumersDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Platform-fit grammar drifted between surfaces for the same object.
    PlatformGrammarDriftDetected,
    /// An adaptation role dropped its command identity or permission meaning.
    CommandIdentityOrPermissionMeaningDropped,
    /// Platform-specific wording changed command or permission meaning.
    PlatformWordingChangedCommandOrPermissionMeaning,
    /// A primary action was hidden only in OS chrome (menus / title bars).
    PrimaryActionHiddenOnlyInOsChrome,
    /// The credential store silently fell back to plaintext.
    CredentialStoreFellBackToPlaintextSilently,
    /// An input method corrupted text or trust fidelity.
    InputMethodCorruptedTextOrTrustFidelity,
    /// A screenshot or docs page mislabeled a shortcut or path / reveal verb.
    ScreenshotOrDocsMislabeledShortcutOrPathVerb,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream shared platform-fit family narrowed.
    UpstreamPlatformFitNarrowed,
}

impl PlatformFitSharedConsumersDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::PlatformGrammarDriftDetected,
        Self::CommandIdentityOrPermissionMeaningDropped,
        Self::PlatformWordingChangedCommandOrPermissionMeaning,
        Self::PrimaryActionHiddenOnlyInOsChrome,
        Self::CredentialStoreFellBackToPlaintextSilently,
        Self::InputMethodCorruptedTextOrTrustFidelity,
        Self::ScreenshotOrDocsMislabeledShortcutOrPathVerb,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamPlatformFitNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::PlatformGrammarDriftDetected => "platform_grammar_drift_detected",
            Self::CommandIdentityOrPermissionMeaningDropped => {
                "command_identity_or_permission_meaning_dropped"
            }
            Self::PlatformWordingChangedCommandOrPermissionMeaning => {
                "platform_wording_changed_command_or_permission_meaning"
            }
            Self::PrimaryActionHiddenOnlyInOsChrome => "primary_action_hidden_only_in_os_chrome",
            Self::CredentialStoreFellBackToPlaintextSilently => {
                "credential_store_fell_back_to_plaintext_silently"
            }
            Self::InputMethodCorruptedTextOrTrustFidelity => {
                "input_method_corrupted_text_or_trust_fidelity"
            }
            Self::ScreenshotOrDocsMislabeledShortcutOrPathVerb => {
                "screenshot_or_docs_mislabeled_shortcut_or_path_verb"
            }
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamPlatformFitNarrowed => "upstream_platform_fit_narrowed",
        }
    }
}

/// The controlled grammar a platform-fit object presents.
///
/// These six words must be identical across every consumer surface that shows the same platform-fit
/// object. The platform-fit-role word must be a frozen role token; the rest are controlled words the
/// object's family carries. A surface may narrow how much it renders, but it may never reword any of
/// these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitStateFacetValues {
    /// Platform-fit-role word (must be a frozen [`M5PlatformFitRole`] token).
    pub platform_fit_role_word: String,
    /// Platform-fit-family word.
    pub family_word: String,
    /// Canonical registry-reference word the family points at.
    pub registry_reference_word: String,
    /// Host-platform word (macOS / Windows / Linux adaptation).
    pub host_platform_word: String,
    /// Surface-context word.
    pub surface_context_word: String,
    /// Command-identity word paired with a shortcut / window-menu / input / command-stability role.
    pub command_identity_word: String,
}

impl PlatformFitStateFacetValues {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.platform_fit_role_word.trim().is_empty()
            && !self.family_word.trim().is_empty()
            && !self.registry_reference_word.trim().is_empty()
            && !self.host_platform_word.trim().is_empty()
            && !self.surface_context_word.trim().is_empty()
            && !self.command_identity_word.trim().is_empty()
    }

    /// Whether the platform-fit-role word is a member of the frozen role vocabulary.
    pub fn platform_fit_role_word_in_vocabulary(&self) -> bool {
        is_known_platform_fit_role_token(self.platform_fit_role_word.trim())
    }

    /// Whether the object honours the command-identity rule: a role that carries shortcut,
    /// window-menu, input-fidelity, or command-stability meaning must pair its platform adaptation
    /// with a real preserved command identity and never collapse to a renamed-command,
    /// permission-changed, text-corrupted, or hidden-in-OS-chrome sentinel.
    pub fn command_identity_satisfied(&self) -> bool {
        match platform_fit_role_from_token(self.platform_fit_role_word.trim()) {
            Some(role) if role.must_preserve_command_identity_under_platform_adaptation() => {
                let identity = self.command_identity_word.trim().to_lowercase();
                !identity.is_empty()
                    && !COMMAND_IDENTITY_ABSENT_SENTINELS.contains(&identity.as_str())
            }
            _ => true,
        }
    }
}

/// The explicit note a narrowed representation shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitNarrowNote {
    /// Why the representation narrowed.
    pub reason: PlatformFitNarrowReason,
    /// Note naming the preserved grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: PlatformFitNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformFitRenderDisclosure {
    /// The parity state the representation requires.
    pub parity_state: PlatformFitParityState,
    /// The narrow reason the representation requires, if any.
    pub narrow_reason: Option<PlatformFitNarrowReason>,
    /// The next action the narrow note must offer, if any.
    pub narrow_next_action: Option<PlatformFitNarrowNextAction>,
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
pub const fn resolve_platform_fit_render_disclosure(
    representation: PlatformFitRepresentation,
) -> PlatformFitRenderDisclosure {
    match representation {
        PlatformFitRepresentation::DesktopFull => PlatformFitRenderDisclosure {
            parity_state: PlatformFitParityState::FacetsPreserved,
            narrow_reason: None,
            narrow_next_action: None,
            needs_narrow_note: false,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        PlatformFitRepresentation::CompactNarrowed => PlatformFitRenderDisclosure {
            parity_state: PlatformFitParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(PlatformFitNarrowReason::CompactionNarrowed),
            narrow_next_action: Some(PlatformFitNarrowNextAction::ExpandInDesktop),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: false,
        },
        PlatformFitRepresentation::RemoteProjected => PlatformFitRenderDisclosure {
            parity_state: PlatformFitParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(PlatformFitNarrowReason::RemoteProjectionNarrowed),
            narrow_next_action: Some(PlatformFitNarrowNextAction::OpenRemoteSource),
            needs_narrow_note: true,
            needs_remote_source_note: true,
            needs_export_detail_note: false,
        },
        PlatformFitRepresentation::ExportedRedacted => PlatformFitRenderDisclosure {
            parity_state: PlatformFitParityState::FacetsDisclosedNarrowed,
            narrow_reason: Some(PlatformFitNarrowReason::ExportRedactionNarrowed),
            narrow_next_action: Some(PlatformFitNarrowNextAction::OpenFullDetail),
            needs_narrow_note: true,
            needs_remote_source_note: false,
            needs_export_detail_note: true,
        },
    }
}

/// One consumer binding: a shared platform-fit family rendered on one consumer surface in one
/// representation for one platform-fit object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable platform-fit-object id (shared across surfaces that show the same object).
    pub platform_fit_object_id: String,
    /// Human-readable platform-fit-object identity.
    pub platform_fit_object_label: String,
    /// Which shared platform-fit family this binding renders.
    pub family: M5PlatformFitFamily,
    /// Which consumer surface renders it.
    pub consumer: M5PlatformFitConsumerSurface,
    /// Which representation this surface renders.
    pub representation: PlatformFitRepresentation,
    /// The controlled grammar presented (identical across surfaces for one object).
    pub state_facets: PlatformFitStateFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: PlatformFitParityState,
    /// The explicit narrow note; required and complete when the binding narrows.
    pub narrow_note: Option<PlatformFitNarrowNote>,
    /// Remote-source note; required and non-empty when the disclosure demands it.
    pub remote_source_note: String,
    /// Export-safe-detail note; required and non-empty when the disclosure demands it.
    pub export_detail_note: String,
    /// Guardrail: this surface lets platform-specific wording change command or permission meaning.
    /// MUST be `false`.
    pub platform_wording_changes_command_or_permission_meaning: bool,
    /// Guardrail: this surface hides a primary action only in OS menus / title bars. MUST be `false`.
    pub hides_primary_action_only_in_os_chrome: bool,
    /// Guardrail: this surface silently falls back to plaintext credential storage. MUST be `false`.
    pub falls_back_to_plaintext_credential_storage_silently: bool,
    /// Guardrail: this surface lets an input method corrupt text or trust fidelity. MUST be `false`.
    pub input_method_corrupts_text_or_trust_fidelity: bool,
    /// Guardrail: this surface produces a screenshot or docs page that mislabels a shortcut or path /
    /// reveal verb. MUST be `false`.
    pub screenshot_or_docs_mislabels_shortcut_or_path_verb: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl PlatformFitConsumerBinding {
    /// Disclosures this binding must carry, derived from its representation.
    pub const fn disclosure(&self) -> PlatformFitRenderDisclosure {
        resolve_platform_fit_render_disclosure(self.representation)
    }

    /// Whether this binding renders below full parity.
    pub const fn is_narrowed(&self) -> bool {
        self.representation.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.platform_wording_changes_command_or_permission_meaning
            && !self.hides_primary_action_only_in_os_chrome
            && !self.falls_back_to_plaintext_credential_storage_silently
            && !self.input_method_corrupts_text_or_trust_fidelity
            && !self.screenshot_or_docs_mislabels_shortcut_or_path_verb
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
                .any(|reference| reference == M5_PLATFORM_FIT_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitSharedConsumersTrustReview {
    /// Family reuse is proven by fixtures rather than inferred from screenshots.
    pub family_reuse_proven_by_fixtures: bool,
    /// The same platform-fit object presents the same grammar across surfaces.
    pub same_object_same_platform_fit_across_surfaces: bool,
    /// Every platform-fit-role word is a frozen role token.
    pub platform_fit_role_words_stay_in_frozen_vocabulary: bool,
    /// Adaptation roles never let a platform-specific label change command or permission meaning.
    pub adaptation_roles_never_change_command_or_permission_meaning: bool,
    /// Platform-specific wording never changes command or permission meaning.
    pub platform_wording_never_changes_command_or_permission_meaning: bool,
    /// Primary actions are never hidden only in OS chrome.
    pub primary_actions_never_hidden_only_in_os_chrome: bool,
    /// Credentials never silently fall back to plaintext storage.
    pub credentials_never_fall_back_to_plaintext_silently: bool,
    /// Input methods never corrupt text or trust fidelity.
    pub input_methods_never_corrupt_text_or_trust_fidelity: bool,
    /// Screenshots and docs never mislabel a shortcut or path / reveal verb.
    pub screenshots_and_docs_never_mislabel_shortcut_or_path_verb: bool,
    /// Narrowing is disclosed across desktop, compact, remote, and exported forms.
    pub narrowing_disclosed_across_representations: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl PlatformFitSharedConsumersTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.family_reuse_proven_by_fixtures
            && self.same_object_same_platform_fit_across_surfaces
            && self.platform_fit_role_words_stay_in_frozen_vocabulary
            && self.adaptation_roles_never_change_command_or_permission_meaning
            && self.platform_wording_never_changes_command_or_permission_meaning
            && self.primary_actions_never_hidden_only_in_os_chrome
            && self.credentials_never_fall_back_to_plaintext_silently
            && self.input_methods_never_corrupt_text_or_trust_fidelity
            && self.screenshots_and_docs_never_mislabel_shortcut_or_path_verb
            && self.narrowing_disclosed_across_representations
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitSharedConsumersProjection {
    /// The shell UI consumes the shared platform-fit grammar.
    pub shell_ui_consumes_shared_platform_fit: bool,
    /// The settings UI consumes the shared platform-fit grammar.
    pub settings_ui_consumes_shared_platform_fit: bool,
    /// The auth UI consumes the shared platform-fit grammar.
    pub auth_ui_consumes_shared_platform_fit: bool,
    /// The input UI / handling consumes the shared platform-fit grammar.
    pub input_ui_consumes_shared_platform_fit: bool,
    /// The docs / help surface consumes the shared platform-fit grammar.
    pub docs_help_consumes_shared_platform_fit: bool,
    /// The support / export path consumes the shared platform-fit grammar.
    pub support_export_consumes_shared_platform_fit: bool,
    /// The general product UI consumes the shared platform-fit grammar.
    pub product_ui_consumes_shared_platform_fit: bool,
    /// Every family is adopted by two or more consumers.
    pub every_family_adopted_by_two_or_more_consumers: bool,
    /// Grammar is identical for the same platform-fit object.
    pub platform_fit_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a family back to one shared contract family.
    pub export_maps_back_to_one_platform_fit_family: bool,
}

impl PlatformFitSharedConsumersProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_ui_consumes_shared_platform_fit
            && self.settings_ui_consumes_shared_platform_fit
            && self.auth_ui_consumes_shared_platform_fit
            && self.input_ui_consumes_shared_platform_fit
            && self.docs_help_consumes_shared_platform_fit
            && self.support_export_consumes_shared_platform_fit
            && self.product_ui_consumes_shared_platform_fit
            && self.every_family_adopted_by_two_or_more_consumers
            && self.platform_fit_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_maps_back_to_one_platform_fit_family
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitSharedConsumersProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5PlatformFitSharedConsumersPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PlatformFitSharedConsumersPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<PlatformFitConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PlatformFitSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5PlatformFitConsumerSurface>,
    /// Trust review block.
    pub trust_review: PlatformFitSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PlatformFitSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: PlatformFitSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe platform-fit shared-consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PlatformFitSharedConsumersPacket {
    /// Record kind; must equal [`M5_PLATFORM_FIT_SHARED_CONSUMERS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PLATFORM_FIT_SHARED_CONSUMERS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<PlatformFitConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PlatformFitSharedConsumersDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5PlatformFitConsumerSurface>,
    /// Trust review block.
    pub trust_review: PlatformFitSharedConsumersTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PlatformFitSharedConsumersProjection,
    /// Proof freshness block.
    pub proof_freshness: PlatformFitSharedConsumersProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PlatformFitSharedConsumersPacket {
    /// Builds a platform-fit shared-consumer packet from stable-lane input.
    pub fn new(input: M5PlatformFitSharedConsumersPacketInput) -> Self {
        Self {
            record_kind: M5_PLATFORM_FIT_SHARED_CONSUMERS_RECORD_KIND.to_owned(),
            schema_version: M5_PLATFORM_FIT_SHARED_CONSUMERS_SCHEMA_VERSION,
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

    /// Validates the platform-fit shared-consumer parity invariants.
    pub fn validate(&self) -> Vec<M5PlatformFitSharedConsumersViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PLATFORM_FIT_SHARED_CONSUMERS_RECORD_KIND {
            violations.push(M5PlatformFitSharedConsumersViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PLATFORM_FIT_SHARED_CONSUMERS_SCHEMA_VERSION {
            violations.push(M5PlatformFitSharedConsumersViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PlatformFitSharedConsumersViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5PlatformFitSharedConsumersViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5PlatformFitSharedConsumersViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5PlatformFitSharedConsumersViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5PlatformFitSharedConsumersViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5PlatformFitSharedConsumersViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("platform-fit shared-consumer packet serializes"),
        ) {
            violations.push(M5PlatformFitSharedConsumersViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("platform-fit shared-consumer packet serializes")
    }

    /// Deterministic matrix CSV, one row per consumer binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out =
            String::from("family,consumer,representation,platform_fit_role_word,parity_state\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "{},{},{},{},{}\n",
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.platform_fit_role_word,
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
        out.push_str("# Shared Platform-Fit Consumers: One Convention Across Surfaces\n\n");
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
                binding.platform_fit_object_label,
                binding.binding_id,
                binding.family.as_str(),
                binding.consumer.as_str(),
                binding.representation.as_str(),
                binding.state_facets.platform_fit_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in platform-fit shared-consumer export.
#[derive(Debug)]
pub enum M5PlatformFitSharedConsumersArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PlatformFitSharedConsumersViolation>),
}

impl fmt::Display for M5PlatformFitSharedConsumersArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "platform-fit shared-consumer export parse failed: {error}"
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
                    "platform-fit shared-consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PlatformFitSharedConsumersArtifactError {}

/// Validation failures emitted by [`M5PlatformFitSharedConsumersPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PlatformFitSharedConsumersViolation {
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
    /// A binding's platform-fit-role word is not a frozen role token.
    PlatformFitRoleWordOutsideVocabulary,
    /// A binding's shortcut / window-menu / input / command-stability role dropped its command
    /// identity.
    CommandIdentityMissingForAdaptationRole,
    /// A binding's parity state does not match its representation.
    ParityStateMismatch,
    /// Two surfaces show the same platform-fit object with different grammar.
    PlatformGrammarDriftAcrossSurfaces,
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
    /// A binding lets platform-specific wording change command or permission meaning.
    PlatformWordingChangesCommandOrPermissionMeaning,
    /// A binding hides a primary action only in OS chrome.
    HidesPrimaryActionOnlyInOsChrome,
    /// A binding silently falls back to plaintext credential storage.
    FallsBackToPlaintextCredentialStorageSilently,
    /// A binding lets an input method corrupt text or trust fidelity.
    InputMethodCorruptsTextOrTrustFidelity,
    /// A binding produces a screenshot or docs page that mislabels a shortcut or path / reveal verb.
    ScreenshotOrDocsMislabelsShortcutOrPathVerb,
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

impl M5PlatformFitSharedConsumersViolation {
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
            Self::PlatformFitRoleWordOutsideVocabulary => {
                "platform_fit_role_word_outside_vocabulary"
            }
            Self::CommandIdentityMissingForAdaptationRole => {
                "command_identity_missing_for_adaptation_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::PlatformGrammarDriftAcrossSurfaces => "platform_grammar_drift_across_surfaces",
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
            Self::PlatformWordingChangesCommandOrPermissionMeaning => {
                "platform_wording_changes_command_or_permission_meaning"
            }
            Self::HidesPrimaryActionOnlyInOsChrome => "hides_primary_action_only_in_os_chrome",
            Self::FallsBackToPlaintextCredentialStorageSilently => {
                "falls_back_to_plaintext_credential_storage_silently"
            }
            Self::InputMethodCorruptsTextOrTrustFidelity => {
                "input_method_corrupts_text_or_trust_fidelity"
            }
            Self::ScreenshotOrDocsMislabelsShortcutOrPathVerb => {
                "screenshot_or_docs_mislabels_shortcut_or_path_verb"
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

/// Reads and validates the checked-in stable platform-fit shared-consumer export.
pub fn current_stable_m5_platform_fit_shared_consumers_export(
) -> Result<M5PlatformFitSharedConsumersPacket, M5PlatformFitSharedConsumersArtifactError> {
    let packet: M5PlatformFitSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-platform-fit-shared-consumers-proof/support_export.json"
    )))
    .map_err(M5PlatformFitSharedConsumersArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PlatformFitSharedConsumersArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5PlatformFitSharedConsumersPacket,
    violations: &mut Vec<M5PlatformFitSharedConsumersViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_PLATFORM_FIT_SHARED_CONSUMERS_SCHEMA_REF,
        M5_PLATFORM_FIT_SHARED_CONSUMERS_DOC_REF,
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
    ];
    // The six families map to three canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for family in M5PlatformFitFamily::ALL {
        domains.insert(family.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5PlatformFitSharedConsumersViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5PlatformFitSharedConsumersPacket,
    violations: &mut Vec<M5PlatformFitSharedConsumersViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(M5PlatformFitSharedConsumersViolation::ConsumerBindingsMissing);
        return;
    }

    // One convention: the facet values must be identical for every binding that renders the same
    // platform-fit object.
    let mut object_facets: BTreeMap<&str, &PlatformFitStateFacetValues> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each family must be adopted by at least two distinct consumers.
    let mut family_consumers: BTreeMap<
        M5PlatformFitFamily,
        BTreeSet<M5PlatformFitConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5PlatformFitConsumerSurface> = BTreeSet::new();
    let mut seen_families: BTreeSet<M5PlatformFitFamily> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.platform_fit_object_id.trim().is_empty()
            || binding.platform_fit_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5PlatformFitSharedConsumersViolation::BindingIncomplete);
        }
        if !binding.state_facets.all_present() {
            violations.push(M5PlatformFitSharedConsumersViolation::GrammarFacetIncomplete);
        }
        if !binding.state_facets.platform_fit_role_word_in_vocabulary() {
            violations
                .push(M5PlatformFitSharedConsumersViolation::PlatformFitRoleWordOutsideVocabulary);
        }
        if !binding.state_facets.command_identity_satisfied() {
            violations.push(
                M5PlatformFitSharedConsumersViolation::CommandIdentityMissingForAdaptationRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5PlatformFitSharedConsumersViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_note {
            match &binding.narrow_note {
                None => {
                    violations.push(M5PlatformFitSharedConsumersViolation::NarrowNoteMissing);
                }
                Some(note) => {
                    if Some(note.reason) != disclosure.narrow_reason {
                        violations
                            .push(M5PlatformFitSharedConsumersViolation::NarrowReasonMismatch);
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations
                            .push(M5PlatformFitSharedConsumersViolation::NarrowNextActionMismatch);
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5PlatformFitSharedConsumersViolation::NarrowNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5PlatformFitSharedConsumersViolation::NarrowNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_note.is_some() {
            violations.push(M5PlatformFitSharedConsumersViolation::UnexpectedNarrowNote);
        }

        if disclosure.needs_remote_source_note && binding.remote_source_note.trim().is_empty() {
            violations.push(M5PlatformFitSharedConsumersViolation::RemoteSourceNoteMissing);
        }
        if disclosure.needs_export_detail_note && binding.export_detail_note.trim().is_empty() {
            violations.push(M5PlatformFitSharedConsumersViolation::ExportDetailNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.platform_wording_changes_command_or_permission_meaning {
            violations.push(
                M5PlatformFitSharedConsumersViolation::PlatformWordingChangesCommandOrPermissionMeaning,
            );
        }
        if binding.hides_primary_action_only_in_os_chrome {
            violations
                .push(M5PlatformFitSharedConsumersViolation::HidesPrimaryActionOnlyInOsChrome);
        }
        if binding.falls_back_to_plaintext_credential_storage_silently {
            violations.push(
                M5PlatformFitSharedConsumersViolation::FallsBackToPlaintextCredentialStorageSilently,
            );
        }
        if binding.input_method_corrupts_text_or_trust_fidelity {
            violations.push(
                M5PlatformFitSharedConsumersViolation::InputMethodCorruptsTextOrTrustFidelity,
            );
        }
        if binding.screenshot_or_docs_mislabels_shortcut_or_path_verb {
            violations.push(
                M5PlatformFitSharedConsumersViolation::ScreenshotOrDocsMislabelsShortcutOrPathVerb,
            );
        }

        // Support / export consumers must map a family back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5PlatformFitSharedConsumersViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match object_facets.get(binding.platform_fit_object_id.as_str()) {
            None => {
                object_facets.insert(
                    binding.platform_fit_object_id.as_str(),
                    &binding.state_facets,
                );
            }
            Some(existing) => {
                if **existing != binding.state_facets && !drift_reported {
                    violations.push(
                        M5PlatformFitSharedConsumersViolation::PlatformGrammarDriftAcrossSurfaces,
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
    for consumer in M5PlatformFitConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5PlatformFitSharedConsumersViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for family in M5PlatformFitFamily::ALL {
        if !seen_families.contains(&family) {
            violations.push(M5PlatformFitSharedConsumersViolation::FamilyCoverageMissing);
            break;
        }
    }

    // Reuse: every present family must be adopted by two or more distinct consumers.
    for consumers in family_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5PlatformFitSharedConsumersViolation::FamilyReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
