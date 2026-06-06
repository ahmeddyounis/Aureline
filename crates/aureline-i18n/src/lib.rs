//! Localization message identity, locale-pack governance, and fallback projections.
//!
//! This crate owns the runtime-facing beta contract for locale packs and
//! localized surfaces. It keeps translated prose behind stable message ids,
//! command ids, diagnostic ids, docs-pack keys, and schema keys, while exposing
//! the active locale, fallback chain, pack signature state, compatibility
//! posture, extension locale declarations, and metadata-only support export.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub mod stable_locale_lifecycle;

pub use stable_locale_lifecycle::{
    seeded_stable_locale_lifecycle_parity_packet, ClaimGateState, FallbackChainTruthRow,
    LocaleLifecycleState, LocalePackLifecycleWindow, LocalePackParitySummary,
    LocalizationClaimClass, ReleaseGateProofRow, StableLocaleLifecycleParityPacket,
    StableMessageIdProofRow, TranslatedSurfaceKind, TranslatedSurfaceParityResult,
    TranslatedSurfaceParityRow, STABLE_LOCALE_LIFECYCLE_PARITY_FIXTURE_REF,
    STABLE_LOCALE_LIFECYCLE_PARITY_PACKET_ID, STABLE_LOCALE_LIFECYCLE_PARITY_RECORD_KIND,
    STABLE_LOCALE_LIFECYCLE_PARITY_SCHEMA_VERSION,
};

/// Schema version shared by beta locale-pack records and projections.
pub const LOCALE_PACK_BETA_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`LocalePackBetaContract`].
pub const LOCALE_PACK_BETA_RECORD_KIND: &str = "locale_pack_beta_contract_record";

/// Record kind for [`LocalePackSurfaceProjection`].
pub const LOCALE_PACK_SURFACE_PROJECTION_RECORD_KIND: &str =
    "locale_pack_surface_projection_record";

/// Record kind for [`LocalePackSupportExport`].
pub const LOCALE_PACK_SUPPORT_EXPORT_RECORD_KIND: &str = "locale_pack_support_export_record";

/// Stable id for the seeded beta localization contract.
pub const LOCALE_PACK_BETA_CONTRACT_ID: &str = "locale-pack:beta:governed-contract:v1";

/// Stable version ref for the seeded beta localization contract.
pub const LOCALE_PACK_BETA_VERSION_REF: &str = "locale-pack-contract-rev:2026.05.18-01";

/// Source-language locale for first-party product strings.
pub const SOURCE_LANGUAGE_LOCALE: &str = "en-US";

const GENERATED_AT: &str = "2026-05-18T17:30:00Z";
const TARGET_BUILD: &str = "build:aureline:0.0.0-beta.2026.05.18";
const POLICY_EPOCH: &str = "policy-epoch:locale-pack-beta:2026.05.18";

/// Localized surface family from the shared message-catalog vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageSurfaceFamily {
    /// Shell title bars, status areas, palette chrome, and switcher labels.
    ShellChrome,
    /// Command-palette, menu, or button labels bound to canonical commands.
    CommandLabel,
    /// Settings labels, descriptions, explain-why text, errors, and denials.
    SettingsHelpOrError,
    /// Docs, tours, onboarding, auth, recovery, and help prose.
    DocsTourOrAuthText,
    /// Extension-owned UI strings inside an extension namespace.
    ExtensionContributedUi,
    /// CLI help, usage, flag descriptions, and terminal human prose.
    CliHelpText,
    /// Human headings in reports and exports.
    ExportOrReportHeading,
    /// Captions or scripts paired with screenshots, recordings, or demos.
    ScreenshotOrDemoCaption,
    /// Glossary terms and definitions.
    GlossaryOrTerminologyTerm,
    /// Policy, legal, trust, and recovery text.
    PolicyLegalOrRecoveryText,
    /// Pseudolocalization stress strings that never ship to production users.
    PseudolocOnlyTestString,
}

/// Stable class for a message id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageIdClass {
    /// First-party canonical message id.
    StableCanonical,
    /// Extension-owned overlay id.
    ExtensionOverlay,
    /// Derived id that preserves an upstream stable id.
    DerivedWithUpstreamId,
    /// Test-only pseudolocalization id.
    PseudolocTestOnly,
}

/// Machine-output localization posture for a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MachineOutputLocaleClass {
    /// Machine output is completely locale-neutral.
    LocaleNeutralCanonical,
    /// Canonical fields stay neutral while optional human fields may localize.
    LocaleNeutralWithTranslatedHumanField,
    /// Human-only output may localize.
    LocaleNativeHumanOnly,
    /// The message must never appear in machine output.
    ForbiddenForMachineOutput,
}

/// Escape hatch that lets a user or reviewer reach source-language truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLanguageEscapeHatchClass {
    /// Inline source-language toggle.
    InlineSourceLanguageToggle,
    /// Command route for opening the source-language version.
    CommandOpenInSourceLanguage,
    /// Docs pane route for viewing the source-language page.
    DocsPaneSourceLanguageRoute,
    /// CLI flag or format mode that emits locale-neutral output.
    CliLocaleNeutralOutputFlag,
    /// Export mode that emits source-language review fields.
    ExportInSourceLanguageForReview,
}

/// Distribution class for a locale-pack artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalePackDistributionClass {
    /// Pack is bundled with the product image.
    BuiltInWithProduct,
    /// Official pack distributed through an approved mirror path.
    MirroredOfficialPack,
    /// Reviewed community pack admitted under the beta contract.
    CommunitySuppliedPack,
    /// Extension-owned overlay pack.
    ExtensionOverlayPack,
    /// Offline pack imported through the air-gapped path.
    AirGappedOfflinePack,
}

/// Signature state for a locale-pack artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalePackSignatureState {
    /// Signature has verified against an admitted signing root.
    SignedVerified,
    /// Signature exists but has not verified in the current environment.
    SignedUnverified,
    /// Unsigned pack was accepted through an explicit decision row.
    UnsignedExplicitAcceptance,
    /// Signature failed and the pack is blocked from rendering messages.
    SignatureFailedBlocked,
    /// Built-in source pack does not carry an external pack signature.
    NotApplicableBuiltIn,
}

impl LocalePackSignatureState {
    /// Returns true when this state may render localized strings.
    pub const fn may_render(self) -> bool {
        matches!(
            self,
            Self::SignedVerified | Self::UnsignedExplicitAcceptance | Self::NotApplicableBuiltIn
        )
    }
}

/// Mirrorability class for a locale-pack artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalePackMirrorabilityClass {
    /// Mirroring is allowed without extra attribution beyond normal metadata.
    MirrorAllowed,
    /// Mirroring is allowed when attribution metadata is preserved.
    MirrorWithAttributionRequired,
    /// Mirroring is forbidden.
    MirrorForbidden,
    /// Pack may be imported only through the air-gapped path.
    AirGappedOnly,
    /// Signed blob cannot be mirrored as a standalone artifact.
    NotMirrorableSignedBlob,
}

/// Compatibility state against the active build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionMatchState {
    /// Pack exactly matches the target build.
    ExactBuildMatch,
    /// Pack is compatible with bounded minor drift.
    CompatibleMinorDrift,
    /// Pack is incompatible with the target build.
    IncompatibleDriftDetected,
    /// Pack targets a pre-release build and has not been verified.
    PreReleaseUnverified,
    /// Target build could not be determined.
    UnknownTargetBuild,
}

impl VersionMatchState {
    /// Returns true when this compatibility state may render localized strings.
    pub const fn may_render(self) -> bool {
        matches!(self, Self::ExactBuildMatch | Self::CompatibleMinorDrift)
    }
}

/// Source class for a locale pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalePackSourceClass {
    /// Built-in first-party source-language strings.
    FirstPartySourceLanguage,
    /// First-party translated strings distributed as an official pack.
    FirstPartyLocalePack,
    /// Reviewed community translation pack.
    ReviewedCommunityPack,
    /// Extension-owned locale overlay.
    ExtensionOwnedPack,
}

/// Fallback origin for a localized presentation event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleFallbackOriginClass {
    /// Requested locale was authoritative for the rendered message.
    RequestedLocaleAuthoritative,
    /// Requested locale was partial and a base locale filled the message.
    RequestedLocalePartialWithBaseFill,
    /// Requested locale was unavailable and the language base filled the message.
    BaseLocaleFallback,
    /// Fallback reached the source language.
    SourceLanguageFallback,
    /// Signature failure forced source-language rendering.
    PackSignatureFailedSourceLanguageOnly,
    /// Missing pack forced source-language rendering.
    PackMissingSourceLanguageOnly,
    /// Policy disabled the locale and forced source-language rendering.
    PolicyDisabledSourceLanguageOnly,
}

impl LocaleFallbackOriginClass {
    /// Returns true when this origin must be visible to the user or reviewer.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::RequestedLocaleAuthoritative)
    }
}

/// Localized rendering state after fallback resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedLocalizationState {
    /// Requested-locale coverage was complete.
    FullyLocalized,
    /// Partial translation is disclosed.
    PartialTranslationDisclosed,
    /// Mixed-locale rendering is kept visibly separated.
    MixedLocaleStrictSeparation,
    /// Only glossary terms were localized.
    GlossaryOnlyLocalized,
    /// Source-language text is being shown in a pseudolocalization lane.
    SourceLanguageWithPseudoloc,
    /// A failed pack forced source-language rendering.
    FailedPackSourceLanguageOnly,
}

/// Whether a command id survived fallback unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandIdPreservationState {
    /// Canonical command id did not change across fallback.
    CommandIdUnchangedAcrossFallback,
    /// A drift attempt was detected and blocked.
    CommandIdDriftedBlocked,
    /// Surface is not command-bound.
    NotApplicable,
}

/// Locale-support posture declared by an extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionLocaleSupportMode {
    /// Extension inherits host locale behavior and host fallback disclosure.
    InheritsHostLocale,
    /// Extension ships its own locale pack.
    ShipsOwnLocalePack,
    /// Extension ships a companion pack governed with the host.
    ShipsCompanionLocalePack,
    /// Extension remains in source language and discloses fallback.
    SourceLanguageOnly,
}

/// Governed operation class for a locale-pack artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalePackOperationClass {
    /// Install a locale pack.
    Install,
    /// Update a locale pack.
    Update,
    /// Roll a locale pack back to a prior admitted revision.
    Rollback,
    /// Import a pack from an approved mirror.
    MirrorImport,
    /// Import a pack from an offline bundle.
    OfflineImport,
}

/// Compatibility outcome for one pack against one build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleCompatibilityOutcome {
    /// Pack is compatible and may render.
    Compatible,
    /// Pack is compatible only with a bounded waiver and disclosed fallback.
    CompatibleWithWaiver,
    /// Pack is blocked by signature failure.
    BlockedSignatureFailure,
    /// Pack is blocked by version drift.
    BlockedVersionDrift,
    /// Pack falls back to source language only.
    SourceLanguageOnlyFallback,
}

/// Policy context paired with locale-pack artifacts and projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContext {
    /// Policy epoch that admitted this record.
    pub policy_epoch: String,
    /// Trust state used when evaluating the record.
    pub trust_state: String,
    /// Optional execution context id for support reconstruction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id: Option<String>,
}

/// Inclusive build range supported by a locale-pack revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityBuildRange {
    /// Minimum supported build identity.
    pub min_build_identity_ref: String,
    /// Maximum supported build identity.
    pub max_build_identity_ref: String,
}

/// Metadata for one locale-pack manifest record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackManifestRecord {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable pack id.
    pub pack_id: String,
    /// Stable pack revision ref.
    pub pack_revision_ref: String,
    /// Primary locale for the pack.
    pub locale: String,
    /// Locales this pack can satisfy directly or as a base-locale proxy.
    pub coverage_locales: Vec<String>,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain.
    pub base_locale_fallback_chain: Vec<String>,
    /// Source class for pack governance and support export.
    pub source_class: LocalePackSourceClass,
    /// Distribution class for installation and mirror policy.
    pub distribution_class: LocalePackDistributionClass,
    /// Signature state used before rendering.
    pub signature_state: LocalePackSignatureState,
    /// Mirrorability posture.
    pub mirrorability_class: LocalePackMirrorabilityClass,
    /// Compatibility state against the active build.
    pub compatibility_class: VersionMatchState,
    /// Inclusive build range for the pack revision.
    pub compatibility_build_range: CompatibilityBuildRange,
    /// Surface families with claimed coverage.
    pub covered_surface_families: Vec<MessageSurfaceFamily>,
    /// Covered surface families that remain partial.
    pub partially_translated_surface_families: Vec<MessageSurfaceFamily>,
    /// Extension overlay pack refs that depend on this pack.
    pub extension_overlay_pack_refs: Vec<String>,
    /// Extension namespaces contributed by this pack.
    pub extension_namespace_refs: Vec<String>,
    /// Deployment profiles where this pack is admissible.
    pub permitted_deployment_profiles: Vec<String>,
    /// Decision row authorizing an unsigned pack, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_acceptance_decision_row_ref: Option<String>,
    /// Opaque source repository or package ref.
    pub source_artifact_ref: String,
    /// Signer identity ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_identity_ref: Option<String>,
    /// Signature artifact ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_artifact_ref: Option<String>,
    /// Mirror receipt refs preserved for offline review.
    pub mirror_receipt_refs: Vec<String>,
    /// Offline import bundle ref, when the pack supports it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_import_ref: Option<String>,
    /// Rollback target ref for this pack.
    pub rollback_ref: String,
    /// Policy context used to admit this pack.
    pub policy_context: PolicyContext,
    /// Redaction class for support exports.
    pub redaction_class: String,
    /// Short label used on product surfaces.
    pub presentation_label: String,
    /// Deterministic mint timestamp.
    pub minted_at: String,
}

/// Stable machine identifiers bound to a translatable message.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StableMessageIdentityRefs {
    /// Canonical command id, when command-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id_ref: Option<String>,
    /// Semantic action id, when action-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_action_id_ref: Option<String>,
    /// Diagnostic or Doctor finding id, when diagnostic-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_id_ref: Option<String>,
    /// Docs-pack key, docs node, or help anchor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_key_ref: Option<String>,
    /// Stable settings id, when settings-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setting_id_ref: Option<String>,
    /// Locale-neutral telemetry key ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_key_ref: Option<String>,
    /// Locale-neutral policy name ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_name_ref: Option<String>,
    /// Stable schema id, when schema-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_id_ref: Option<String>,
}

impl StableMessageIdentityRefs {
    /// Returns true when the message has at least one stable non-prose anchor.
    pub fn has_anchor(&self) -> bool {
        self.command_id_ref.is_some()
            || self.semantic_action_id_ref.is_some()
            || self.diagnostic_id_ref.is_some()
            || self.docs_pack_key_ref.is_some()
            || self.setting_id_ref.is_some()
            || self.telemetry_key_ref.is_some()
            || self.policy_name_ref.is_some()
            || self.schema_id_ref.is_some()
    }
}

/// Placeholder metadata for a translatable message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePlaceholder {
    /// Stable placeholder id inside the message template.
    pub placeholder_id: String,
    /// Placeholder kind from the message-catalog vocabulary.
    pub placeholder_kind: String,
    /// Short translation note preserving token semantics.
    pub translator_note: String,
}

/// First-party or extension message binding with stable non-prose refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageCatalogBindingRecord {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable message id.
    pub message_id: String,
    /// Message-id class.
    pub message_id_class: MessageIdClass,
    /// Surface family where the message renders.
    pub surface_family: MessageSurfaceFamily,
    /// Source-language locale.
    pub source_language_locale: String,
    /// Short source-language template summary.
    pub source_text: String,
    /// Stable non-prose identity refs bound to this message.
    pub stable_refs: StableMessageIdentityRefs,
    /// Placeholder descriptors for localization-safe rendering.
    pub placeholders: Vec<MessagePlaceholder>,
    /// Machine-output localization posture.
    pub machine_output_locale_class: MachineOutputLocaleClass,
    /// Source-language escape hatches supported by the message surface.
    pub source_language_escape_hatches: Vec<SourceLanguageEscapeHatchClass>,
    /// Whether localized human prose may render for this message.
    pub localized_human_prose_allowed: bool,
    /// Whether identifiers and machine keys remain locale-neutral.
    pub machine_identifier_fields_locale_neutral: bool,
    /// Must remain false; behavior cannot route by localized prose.
    pub routed_by_localized_prose: bool,
    /// Optional extension namespace for extension-owned messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_namespace_ref: Option<String>,
    /// Translation review refs that govern safety-sensitive copy.
    pub translation_review_refs: Vec<String>,
    /// Deterministic mint timestamp.
    pub minted_at: String,
}

/// Pack consultation row inside a fallback state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackConsultationDescriptor {
    /// Locale-pack manifest ref consulted.
    pub pack_ref: String,
    /// Signature state observed while consulting the pack.
    pub signature_state: LocalePackSignatureState,
    /// Locale looked up in this pack.
    pub consulted_locale: String,
    /// Whether this pack produced the rendered message.
    pub produced_message: bool,
}

/// Inspectable locale fallback state for one presentation event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleFallbackStateRecord {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable fallback-state id.
    pub state_id: String,
    /// User-requested locale.
    pub requested_locale: String,
    /// Locale that produced the rendered message.
    pub effective_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Why fallback did or did not occur.
    pub fallback_origin_class: LocaleFallbackOriginClass,
    /// Degraded localization state after fallback.
    pub degraded_localization_state: DegradedLocalizationState,
    /// Ordered fallback chain walked for the rendered message.
    pub fallback_chain_walked: Vec<String>,
    /// Packs consulted while resolving the message.
    pub packs_consulted: Vec<PackConsultationDescriptor>,
    /// Message id this fallback applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id_ref: Option<String>,
    /// Command id this fallback preserves, when command-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id_ref: Option<String>,
    /// Whether the fallback is visible to the user or reviewer.
    pub disclosed_to_reviewer: bool,
    /// Source-language routes available under fallback.
    pub source_language_escape_hatches_active: Vec<SourceLanguageEscapeHatchClass>,
    /// Surface family affected by fallback.
    pub surface_family: MessageSurfaceFamily,
    /// Command-id preservation state.
    pub command_id_preservation_state: CommandIdPreservationState,
    /// Machine-output localization posture.
    pub machine_output_locale_class: MachineOutputLocaleClass,
    /// Deployment profiles where this fallback applies.
    pub deployment_profile_refs: Vec<String>,
    /// Denial reason when fallback blocks a pack or drift attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_on_deny: Option<String>,
    /// Policy context used for fallback resolution.
    pub policy_context: PolicyContext,
    /// Redaction class for exports.
    pub redaction_class: String,
    /// Target build identity.
    pub target_build_identity_ref: String,
    /// Short label rendered on inspection surfaces.
    pub presentation_label: String,
    /// Optional subtitle rendered on inspection surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,
    /// Deterministic mint timestamp.
    pub minted_at: String,
}

/// Active locale and fallback state exposed by product surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveLocaleState {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable active-locale state id.
    pub state_id: String,
    /// Requested locale for the active session.
    pub requested_locale: String,
    /// Primary effective locale for fully covered surfaces.
    pub effective_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Full inspectable requested-to-base-to-source fallback chain.
    pub fallback_chain: Vec<String>,
    /// Active pack refs consulted by the session.
    pub active_pack_refs: Vec<String>,
    /// Active fallback state refs for partial or blocked surfaces.
    pub active_fallback_state_refs: Vec<String>,
    /// Whether any active surface reached source-language fallback.
    pub source_language_fallback_active: bool,
    /// Stable settings projection ref.
    pub settings_projection_ref: String,
    /// Stable Help/About projection ref.
    pub help_about_projection_ref: String,
    /// Stable support-export ref.
    pub support_export_ref: String,
    /// Summary of active signature states.
    pub signature_state_summary: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
}

/// Surface kind for active-locale projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleProjectionSurface {
    /// Settings locale and language-pack inspector.
    Settings,
    /// Help/About locale provenance card.
    HelpAbout,
    /// Metadata-only support export.
    SupportExport,
}

impl LocaleProjectionSurface {
    fn as_ref(self) -> &'static str {
        match self {
            Self::Settings => "surface:settings:locale_and_language_packs",
            Self::HelpAbout => "surface:help_about:locale_pack_state",
            Self::SupportExport => "surface:support_export:locale_pack_state",
        }
    }
}

/// One row in a settings, Help/About, or support locale projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleSurfaceProjectionRow {
    /// Stable projection row id.
    pub row_id: String,
    /// Surface ref rendering this row.
    pub surface_ref: String,
    /// Row kind.
    pub row_kind: String,
    /// Message id ref, when row is message-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id_ref: Option<String>,
    /// Fallback state ref, when row is fallback-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_state_ref: Option<String>,
    /// Pack id ref, when row is pack-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_id_ref: Option<String>,
    /// Pack revision ref, when row is pack-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_revision_ref: Option<String>,
    /// Requested locale visible on the row.
    pub requested_locale: String,
    /// Effective locale visible on the row.
    pub effective_locale: String,
    /// Full fallback chain visible on the row.
    pub fallback_chain: Vec<String>,
    /// Signature state for the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_state: Option<LocalePackSignatureState>,
    /// Compatibility state for the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility_class: Option<VersionMatchState>,
    /// Stable command id ref preserved by the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id_ref: Option<String>,
    /// Stable locale-neutral ids visible to copy/support flows.
    pub stable_machine_id_refs: Vec<String>,
    /// Source-language escape hatches visible on the row.
    pub source_language_escape_hatches: Vec<SourceLanguageEscapeHatchClass>,
    /// Short export-safe display summary.
    pub display_summary: String,
    /// Whether raw translated body text is excluded from the row.
    pub raw_translated_body_omitted: bool,
}

/// Settings, Help/About, or support locale projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackSurfaceProjection {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Projection surface.
    pub projection_surface: LocaleProjectionSurface,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source contract id.
    pub source_contract_id: String,
    /// Active locale state projected by this surface.
    pub active_locale_state: ActiveLocaleState,
    /// Projection rows.
    pub rows: Vec<LocaleSurfaceProjectionRow>,
    /// Raw translated body material omitted by this projection.
    pub omitted_material_classes: Vec<String>,
}

/// Locale-support declaration for an extension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionLocaleDeclaration {
    /// Stable extension id.
    pub extension_id: String,
    /// Stable extension namespace ref.
    pub extension_namespace_ref: String,
    /// Declared locale-support mode.
    pub support_mode: ExtensionLocaleSupportMode,
    /// Whether the extension inherits host fallback disclosure.
    pub inherits_host_fallback_disclosure: bool,
    /// Locale pack ref when the extension ships one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale_pack_ref: Option<String>,
    /// Source-language locale for extension-owned strings.
    pub source_language_locale: String,
    /// Fallback locale when an overlay is missing or blocked.
    pub fallback_locale: String,
    /// Must remain false; extensions cannot override host stable ids.
    pub may_override_host_stable_ids: bool,
    /// Compatibility result row for this declaration.
    pub compatibility_result_ref: String,
    /// Whether product UI must disclose the declaration.
    pub visible_disclosure_required: bool,
}

/// Governed install/update/rollback/mirror action for a pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackGovernanceAction {
    /// Stable action id.
    pub action_id: String,
    /// Operation class.
    pub operation_class: LocalePackOperationClass,
    /// Pack ref affected by this action.
    pub pack_ref: String,
    /// Whether the operation requires a preview/review step.
    pub review_required: bool,
    /// Whether signature verification is required.
    pub signature_verification_required: bool,
    /// Whether compatibility evaluation is required.
    pub compatibility_check_required: bool,
    /// Whether mirror/offline metadata must be preserved.
    pub mirror_metadata_preserved: bool,
    /// Rollback target emitted by this action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_ref: Option<String>,
    /// Support-export row emitted by this action.
    pub support_export_ref: String,
}

/// Compatibility result for a pack against the target build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackCompatibilityResult {
    /// Stable result id.
    pub result_id: String,
    /// Pack ref evaluated.
    pub pack_ref: String,
    /// Target build identity.
    pub target_build_identity_ref: String,
    /// Compatibility outcome.
    pub outcome: LocaleCompatibilityOutcome,
    /// Signature state observed during evaluation.
    pub signature_state: LocalePackSignatureState,
    /// Version match state observed during evaluation.
    pub compatibility_class: VersionMatchState,
    /// Surface families checked.
    pub surface_families_checked: Vec<MessageSurfaceFamily>,
    /// Fallback states tied to this result.
    pub fallback_state_refs: Vec<String>,
    /// Optional bounded waiver ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
}

/// Bounded compatibility waiver recorded in a release packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackCompatibilityWaiver {
    /// Stable waiver ref.
    pub waiver_ref: String,
    /// Pack ref under waiver.
    pub pack_ref: String,
    /// Surface families covered by the waiver.
    pub bounded_to_surface_families: Vec<MessageSurfaceFamily>,
    /// Export-safe reason for the waiver.
    pub reason: String,
    /// Expiry timestamp for the waiver.
    pub expires_at: String,
    /// Whether source-language fallback is required while waiver is active.
    pub fallback_required: bool,
    /// Release packet ref that carries this waiver.
    pub release_packet_ref: String,
}

/// Locale-neutral posture for machine-facing fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MachineIdentifierPosture {
    /// Field family being protected.
    pub field_family: String,
    /// Stable identifier refs in this family.
    pub stable_identifier_refs: Vec<String>,
    /// Whether the identifiers remain locale-neutral.
    pub locale_neutral: bool,
    /// Whether translated human prose may appear beside canonical fields.
    pub human_prose_overlay_allowed: bool,
}

/// Protected proof row referenced by the beta contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackProtectedProof {
    /// Stable proof id.
    pub proof_id: String,
    /// Fixture path relative to the repository root.
    pub fixture_ref: String,
    /// Contract axes exercised by this proof.
    pub exercised_axes: Vec<String>,
}

/// Schema version shared by dense i18n conformance corpus records.
pub const DENSE_I18N_CORPUS_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`DenseI18nConformanceCorpus`].
pub const DENSE_I18N_CORPUS_RECORD_KIND: &str = "dense_i18n_conformance_corpus_record";

/// Record kind for [`DenseI18nConformanceReviewPacket`].
pub const DENSE_I18N_REVIEW_PACKET_RECORD_KIND: &str =
    "dense_i18n_conformance_review_packet_record";

/// Stable id for the beta dense-surface i18n corpus.
pub const DENSE_I18N_CORPUS_ID: &str = "i18n-conformance:dense-beta:v1";

/// Dense product surface family covered by the beta i18n corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenseI18nSurfaceFamily {
    /// Editor buffers, rename fields, completion surfaces, and snippet traversal.
    Editor,
    /// Command palette query input, result rows, and command preview panes.
    CommandPalette,
    /// Settings rows, schema-backed help, locale rows, and validation messages.
    Settings,
    /// File, symbol, docs, or workspace trees with dense row chrome.
    TreeView,
    /// Data, settings, search, or evidence tables with sticky headers and counts.
    TableView,
    /// Logs, traces, timelines, and diagnostic streams.
    LogView,
    /// Terminal transcript, terminal input, and raw or escaped copy routes.
    Terminal,
    /// Diff, review, conflict, and hosted-review panes.
    ReviewPane,
    /// Guided tours, onboarding exercises, and learning steps.
    GuidedTour,
    /// Docs, help, glossary, and support knowledge surfaces.
    DocsHelp,
}

/// Stress mode exercised by a dense i18n conformance case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenseI18nStressMode {
    /// Source-language rendering baseline.
    SourceLanguageBaseline,
    /// Pseudolocalization expansion and accent wrapping.
    PseudolocExpansion,
    /// Right-to-left chrome mirroring.
    RtlChrome,
    /// Bidirectional prose with literal technical tokens.
    MixedDirectionTechnicalText,
    /// IME preedit, candidate, and commit behavior.
    ImeComposition,
    /// CJK, full-width glyph, emoji, and font-fallback rendering.
    CjkFontFallback,
    /// Long translated strings and explicit expansion budgets.
    TextExpansion,
    /// Missing, partial, stale, blocked, or failed locale-pack fallback.
    LocaleFallbackReview,
    /// Translated docs, help, command, citation, and policy parity.
    TranslatedSurfaceParity,
    /// Locale-sensitive date, number, and count formatting.
    LocalizedDateNumberFormatting,
}

/// Failure class the dense i18n corpus must catch before beta release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenseI18nFailureClass {
    /// Text clips, overflows, or overlaps nearby UI.
    TextClippedOrOverflow,
    /// Truncation hides scope, severity, count, or action meaning.
    TruncationHidesScopeOrAction,
    /// Placeholder order or literal technical token semantics drift.
    PlaceholderOrderDrift,
    /// Directional chrome mirrors incorrectly or focus order stays stale.
    MirroredChromeOrFocusOrderError,
    /// Literal code, path, hostname, flag, or command id is mirrored.
    LiteralTechnicalStringMirrored,
    /// Raw, rendered, or escaped copy output diverges without disclosure.
    BidiCopyDrift,
    /// IME preedit is lost, committed, cancelled, or moved silently.
    ImePreeditLoss,
    /// Candidate strip or active caret is occluded by dense UI.
    CandidateWindowOccluded,
    /// Focus churn silently commits or cancels composition.
    FocusChurnSilentCommitOrCancel,
    /// Completion preview silently commits or cancels composition.
    CompletionPreviewSilentCommitOrCancel,
    /// Snippet traversal silently commits or cancels composition.
    SnippetTraversalSilentCommitOrCancel,
    /// Command preview silently commits or cancels composition.
    CommandPreviewSilentCommitOrCancel,
    /// CJK, full-width, or emoji glyph fallback fails.
    MissingGlyphOrWrongFontFallback,
    /// Locale-sensitive date, number, or count formatting drifts.
    LocalizedDateNumberDrift,
    /// Stable command, citation, keyboard, schema, or policy id drifts.
    StableIdDrift,
    /// Required source-language escape hatch is missing.
    SourceLanguageEscapeHatchMissing,
    /// Policy, trust, recovery, or legal copy drifts from governed terms.
    PolicyTrustCopyDrift,
    /// Missing or stale locale pack blocks core use instead of falling back.
    LocalePackFallbackBlockedCoreUse,
}

/// Assertion class required by a dense i18n conformance case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenseI18nAssertionClass {
    /// Pseudoloc and text expansion cannot clip or obscure critical meaning.
    NoTruncationOrOverflow,
    /// Directional chrome mirrors while literal technical tokens do not.
    RtlChromeMirrorsOnlyDirectionalUi,
    /// Literal code, paths, hostnames, flags, and ids remain copy-safe.
    LiteralTechnicalTokensPreserved,
    /// IME composition cannot be silently committed, cancelled, or retargeted.
    ImeCompositionNotSilentlyCommittedOrCancelled,
    /// Candidate windows and active carets remain visible.
    CandidateAndCaretRemainVisible,
    /// CJK, emoji, and full-width glyphs render through an accepted fallback chain.
    FontFallbackWorks,
    /// Missing or stale locale packs disclose fallback and continue core use.
    LocaleFallbackDisclosedAndNonBlocking,
    /// Command ids, keyboard paths, citations, scope labels, and policy ids persist.
    StableTranslatedSurfaceRefsPreserved,
    /// Source-language access remains reachable on translated or fallback surfaces.
    SourceLanguageEscapeHatchAvailable,
    /// Locale-sensitive dates, numbers, and counts keep meaning and stable ids.
    LocalizedFormattingKeepsStableSemantics,
}

/// Event that can churn focus or previews during IME composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImeCompositionChurnEvent {
    /// Focus changes to a related dense row or inspector and returns.
    FocusChange,
    /// Completion or inline suggestion preview opens or re-ranks.
    CompletionPreview,
    /// Snippet placeholder traversal advances while composition is active.
    SnippetTraversal,
    /// Command preview opens, updates, or closes while composition is active.
    CommandPreview,
    /// Palette, table, tree, or log filtering re-ranks rows.
    FilterRerank,
    /// Dialog, side sheet, or source-language route opens and returns.
    OverlayTransition,
}

/// Release lane cadence for the dense i18n corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenseI18nLaneCadence {
    /// Runs on the scheduled nightly lane.
    Nightly,
    /// Runs on release-candidate promotion lanes.
    ReleaseCandidate,
    /// Runs on pull requests that touch localization, dense surfaces, or input paths.
    PullRequest,
}

/// Technical token that must remain literal under translation or RTL rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiteralTechnicalToken {
    /// Literal token value as authored.
    pub token: String,
    /// Token class such as `command_id`, `file_path`, `hostname`, `flag`, or `citation`.
    pub token_class: String,
    /// Whether RTL mirroring must leave this token's order unchanged.
    pub must_remain_unmirrored: bool,
    /// Whether raw copy is required for this token.
    pub copy_raw_required: bool,
    /// Whether escaped copy is required when directionality or invisibles matter.
    pub copy_escaped_required: bool,
}

/// IME composition scenario exercised by one dense surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImeCompositionScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Input method, layout, or platform route under test.
    pub input_method: String,
    /// Text shown while composition is active.
    pub preedit_text: String,
    /// Text expected after commit.
    pub expected_commit_text: String,
    /// Churn events that occur while composition is active.
    pub churn_events: Vec<ImeCompositionChurnEvent>,
    /// Whether silent commit is forbidden.
    pub silent_commit_forbidden: bool,
    /// Whether silent cancel is forbidden.
    pub silent_cancel_forbidden: bool,
    /// Whether active caret and candidate location must stay visible.
    pub candidate_and_caret_visibility_required: bool,
}

/// Expansion budget a surface must absorb before truncating critical copy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextExpansionBudget {
    /// Minimum expansion ratio for one-line chrome and state labels.
    pub single_line_min_ratio: f32,
    /// Minimum expansion ratio for multi-line banners, reviews, and guided steps.
    pub multiline_min_ratio: f32,
    /// Whether overflow is forbidden for the case.
    pub overflow_forbidden: bool,
    /// Whether a same-flow full-text route is required before truncation.
    pub full_text_route_required: bool,
}

/// RTL and bidi rendering expectations for one surface case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtlMirroringExpectation {
    /// Whether directional UI chrome mirrors.
    pub directional_chrome_mirrors: bool,
    /// Whether literal technical strings remain unmirrored.
    pub literal_technical_strings_unmirrored: bool,
    /// Whether keyboard and screen-reader order tracks visual order.
    pub focus_order_tracks_visual_order: bool,
    /// Whether raw copy preserves authored logical order.
    pub raw_copy_preserves_author_order: bool,
}

/// Dense i18n conformance case for one beta surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DenseI18nSurfaceCase {
    /// Stable case id.
    pub case_id: String,
    /// Dense surface family under test.
    pub surface_family: DenseI18nSurfaceFamily,
    /// Product surface ref or projection id.
    pub surface_ref: String,
    /// Runtime crates that own or project this surface.
    pub crate_refs: Vec<String>,
    /// Dense workflow represented by the case.
    pub dense_workflow: String,
    /// Locales used by this case.
    pub locale_tags: Vec<String>,
    /// Stress modes exercised by this case.
    pub stress_modes: Vec<DenseI18nStressMode>,
    /// Readiness rows joined from `artifacts/i18n/test_mode_matrix.yaml`.
    pub readiness_row_refs: Vec<String>,
    /// Fixture refs used as input or evidence for this case.
    pub fixture_refs: Vec<String>,
    /// Stable source strings or seed refs used by the case.
    pub source_seed_refs: Vec<String>,
    /// Literal technical tokens that must remain unmirrored and copy-safe.
    pub literal_tokens: Vec<LiteralTechnicalToken>,
    /// IME scenarios for surfaces that exercise composition.
    pub ime_scenarios: Vec<ImeCompositionScenario>,
    /// Text expansion budget applied to this surface.
    pub expansion_budget: TextExpansionBudget,
    /// RTL or bidi expectation when the case exercises directionality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rtl_expectation: Option<RtlMirroringExpectation>,
    /// Assertion classes this case must satisfy.
    pub assertion_refs: Vec<DenseI18nAssertionClass>,
    /// Failure classes this case is expected to catch.
    pub expected_failure_classes: Vec<DenseI18nFailureClass>,
    /// Optional bounded waiver ref when a case is not fully green.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounded_waiver_ref: Option<String>,
}

/// Stable ref that translated surfaces must preserve.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservedTranslatedSurfaceRef {
    /// Ref kind such as `command_id`, `keyboard_path`, `citation_anchor`, or `scope_label`.
    pub ref_kind: String,
    /// Literal ref value.
    pub value: String,
}

/// Assertion that translated or fallback surfaces preserve stable truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslatedSurfaceAssertion {
    /// Stable assertion id.
    pub assertion_id: String,
    /// Surface family the assertion applies to.
    pub surface_family: DenseI18nSurfaceFamily,
    /// Stable refs that must be preserved.
    pub preserved_refs: Vec<PreservedTranslatedSurfaceRef>,
    /// Whether source-language access is required on the same surface.
    pub open_in_source_language_required: bool,
    /// Whether machine output remains locale-neutral.
    pub machine_output_locale_neutral: bool,
    /// Whether governed trust, policy, or recovery terminology is preserved.
    pub governed_terminology_preserved: bool,
}

/// CI or release lane binding for running the dense i18n corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DenseI18nLaneBinding {
    /// Stable lane id.
    pub lane_id: String,
    /// Lane cadence.
    pub cadence: DenseI18nLaneCadence,
    /// Exact command the lane runs.
    pub command: String,
    /// Artifact refs exported by the lane.
    pub artifact_refs: Vec<String>,
    /// Whether this lane is release-blocking for claimed localized beta surfaces.
    pub release_blocking_for_claimed_surfaces: bool,
}

/// Bounded waiver for a non-green dense i18n corpus row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DenseI18nBoundedWaiver {
    /// Stable waiver ref.
    pub waiver_ref: String,
    /// Surface case ref covered by the waiver.
    pub case_ref: String,
    /// Export-safe reason.
    pub reason: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Failure classes covered by the waiver.
    pub bounded_failure_classes: Vec<DenseI18nFailureClass>,
    /// Required fallback, workaround, or claim narrowing while waiver is active.
    pub required_fallback_or_workaround: String,
}

/// Exportable dense-surface corpus used by nightly and release-candidate lanes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DenseI18nConformanceCorpus {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable corpus id.
    pub corpus_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Release channel this corpus defends.
    pub release_channel: String,
    /// Fixture directory relative to the repository root.
    pub fixture_root: String,
    /// Source contracts that govern this corpus.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime crates that must consume the corpus when their surfaces claim beta parity.
    pub runtime_consumer_refs: Vec<String>,
    /// CI and release lane bindings that run this corpus.
    pub lane_bindings: Vec<DenseI18nLaneBinding>,
    /// Surface cases in the corpus.
    pub surface_cases: Vec<DenseI18nSurfaceCase>,
    /// Cross-surface translated parity assertions.
    pub translated_surface_assertions: Vec<TranslatedSurfaceAssertion>,
    /// Explicit bounded waivers, if any.
    pub waivers: Vec<DenseI18nBoundedWaiver>,
    /// Review packet artifact ref.
    pub review_packet_ref: String,
    /// RTL and expansion audit artifact ref.
    pub rtl_and_text_expansion_audit_ref: String,
    /// User-facing contract document ref.
    pub documentation_ref: String,
}

/// Review row exported from one dense i18n surface case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DenseI18nReviewRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface family reviewed by this row.
    pub surface_family: DenseI18nSurfaceFamily,
    /// Surface case ref.
    pub case_ref: String,
    /// Stress modes exercised by the row.
    pub stress_modes: Vec<DenseI18nStressMode>,
    /// Assertion classes covered by the row.
    pub assertion_refs: Vec<DenseI18nAssertionClass>,
    /// Export-safe result state.
    pub result_state: String,
    /// Whether the row has an active bounded waiver.
    pub bounded_waiver_active: bool,
}

/// Exportable review packet derived from [`DenseI18nConformanceCorpus`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DenseI18nConformanceReviewPacket {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable review packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source corpus id.
    pub source_corpus_id: String,
    /// Covered surface families.
    pub covered_surface_families: Vec<DenseI18nSurfaceFamily>,
    /// Covered stress modes.
    pub covered_stress_modes: Vec<DenseI18nStressMode>,
    /// Lane ids that must run the corpus.
    pub lane_refs: Vec<String>,
    /// Review rows.
    pub rows: Vec<DenseI18nReviewRow>,
    /// Number of active bounded waivers.
    pub active_waiver_count: usize,
    /// Exported artifact refs.
    pub artifact_refs: Vec<String>,
}

/// Metadata-only support export for active locale-pack state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackSupportExport {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support export id.
    pub support_export_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source contract id.
    pub source_contract_id: String,
    /// Active locale state.
    pub active_locale_state: ActiveLocaleState,
    /// Active pack versions captured by support.
    pub active_pack_versions: Vec<LocalePackVersionSummary>,
    /// Fallback rows captured by support.
    pub fallback_rows: Vec<LocaleSurfaceProjectionRow>,
    /// Machine identifier posture.
    pub machine_identifier_posture: Vec<MachineIdentifierPosture>,
    /// Extension declarations captured by support.
    pub extension_locale_declarations: Vec<ExtensionLocaleDeclaration>,
    /// Compatibility results captured by support.
    pub compatibility_results: Vec<LocalePackCompatibilityResult>,
    /// Bounded waivers captured by support.
    pub compatibility_waivers: Vec<LocalePackCompatibilityWaiver>,
    /// Material classes omitted from the export.
    pub omitted_material_classes: Vec<String>,
    /// Whether raw translated bodies are exported.
    pub raw_translated_bodies_exported: bool,
}

/// Pack version summary used in support exports and projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackVersionSummary {
    /// Stable pack id.
    pub pack_id: String,
    /// Stable pack revision ref.
    pub pack_revision_ref: String,
    /// Locale for the pack.
    pub locale: String,
    /// Pack source class.
    pub source_class: LocalePackSourceClass,
    /// Signature state.
    pub signature_state: LocalePackSignatureState,
    /// Compatibility state.
    pub compatibility_class: VersionMatchState,
    /// Mirrorability posture.
    pub mirrorability_class: LocalePackMirrorabilityClass,
}

/// Seeded beta contract for locale-pack runtime and artifact governance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackBetaContract {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable contract id.
    pub contract_id: String,
    /// Stable contract version ref.
    pub contract_version_ref: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Release channel for this contract.
    pub release_channel: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Requested locale used by the seeded active state.
    pub requested_locale: String,
    /// Active locale state exposed in product surfaces.
    pub active_locale_state: ActiveLocaleState,
    /// Source contracts that own this vocabulary.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime consumers that project this contract.
    pub runtime_consumer_refs: Vec<String>,
    /// Message bindings with stable non-prose refs.
    pub message_bindings: Vec<MessageCatalogBindingRecord>,
    /// Locale-pack manifest records.
    pub locale_packs: Vec<LocalePackManifestRecord>,
    /// Inspectable fallback state records.
    pub fallback_states: Vec<LocaleFallbackStateRecord>,
    /// Extension-facing locale declarations.
    pub extension_locale_declarations: Vec<ExtensionLocaleDeclaration>,
    /// Install/update/rollback/mirror governance actions.
    pub governance_actions: Vec<LocalePackGovernanceAction>,
    /// Compatibility results included in release/support packets.
    pub compatibility_results: Vec<LocalePackCompatibilityResult>,
    /// Bounded waivers included in release/support packets.
    pub compatibility_waivers: Vec<LocalePackCompatibilityWaiver>,
    /// Locale-neutral machine-facing fields.
    pub machine_identifier_posture: Vec<MachineIdentifierPosture>,
    /// Protected fixtures covering the contract.
    pub protected_proofs: Vec<LocalePackProtectedProof>,
}

impl LocalePackBetaContract {
    /// Returns a locale pack by pack id.
    pub fn pack(&self, pack_id: &str) -> Option<&LocalePackManifestRecord> {
        self.locale_packs
            .iter()
            .find(|pack| pack.pack_id == pack_id)
    }

    /// Returns a message binding by message id.
    pub fn message(&self, message_id: &str) -> Option<&MessageCatalogBindingRecord> {
        self.message_bindings
            .iter()
            .find(|message| message.message_id == message_id)
    }

    /// Returns a fallback state by state id.
    pub fn fallback_state(&self, state_id: &str) -> Option<&LocaleFallbackStateRecord> {
        self.fallback_states
            .iter()
            .find(|state| state.state_id == state_id)
    }

    /// Projects active locale state for settings, Help/About, or support export.
    pub fn surface_projection(
        &self,
        projection_surface: LocaleProjectionSurface,
    ) -> LocalePackSurfaceProjection {
        let surface_ref = projection_surface.as_ref();
        let mut rows = Vec::new();

        rows.push(LocaleSurfaceProjectionRow {
            row_id: format!("{surface_ref}:active-locale"),
            surface_ref: surface_ref.to_owned(),
            row_kind: "active_locale_state".to_owned(),
            message_id_ref: None,
            fallback_state_ref: None,
            pack_id_ref: None,
            pack_revision_ref: None,
            requested_locale: self.active_locale_state.requested_locale.clone(),
            effective_locale: self.active_locale_state.effective_locale.clone(),
            fallback_chain: self.active_locale_state.fallback_chain.clone(),
            signature_state: None,
            compatibility_class: None,
            command_id_ref: None,
            stable_machine_id_refs: vec![
                self.active_locale_state.state_id.clone(),
                self.active_locale_state.settings_projection_ref.clone(),
                self.active_locale_state.help_about_projection_ref.clone(),
                self.active_locale_state.support_export_ref.clone(),
            ],
            source_language_escape_hatches: vec![
                SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle,
            ],
            display_summary: format!(
                "requested {} -> effective {} -> source {}",
                self.active_locale_state.requested_locale,
                self.active_locale_state.effective_locale,
                self.active_locale_state.source_language_locale
            ),
            raw_translated_body_omitted: true,
        });

        for pack in &self.locale_packs {
            if !self
                .active_locale_state
                .active_pack_refs
                .iter()
                .any(|active| active == &pack.pack_id)
            {
                continue;
            }
            rows.push(LocaleSurfaceProjectionRow {
                row_id: format!("{surface_ref}:pack:{}", pack.pack_id),
                surface_ref: surface_ref.to_owned(),
                row_kind: "locale_pack_version_signature".to_owned(),
                message_id_ref: None,
                fallback_state_ref: None,
                pack_id_ref: Some(pack.pack_id.clone()),
                pack_revision_ref: Some(pack.pack_revision_ref.clone()),
                requested_locale: self.active_locale_state.requested_locale.clone(),
                effective_locale: pack.locale.clone(),
                fallback_chain: pack.base_locale_fallback_chain.clone(),
                signature_state: Some(pack.signature_state),
                compatibility_class: Some(pack.compatibility_class),
                command_id_ref: None,
                stable_machine_id_refs: vec![
                    pack.pack_id.clone(),
                    pack.pack_revision_ref.clone(),
                    pack.rollback_ref.clone(),
                ],
                source_language_escape_hatches: vec![
                    SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle,
                    SourceLanguageEscapeHatchClass::ExportInSourceLanguageForReview,
                ],
                display_summary: format!(
                    "{}: {:?}, {:?}, {:?}",
                    pack.presentation_label,
                    pack.signature_state,
                    pack.compatibility_class,
                    pack.mirrorability_class
                ),
                raw_translated_body_omitted: true,
            });
        }

        for state in &self.fallback_states {
            if !self
                .active_locale_state
                .active_fallback_state_refs
                .iter()
                .any(|active| active == &state.state_id)
            {
                continue;
            }
            let message = state
                .message_id_ref
                .as_deref()
                .and_then(|message_id| self.message(message_id));
            let stable_machine_id_refs = message
                .map(|message| stable_refs_as_vec(&message.stable_refs))
                .unwrap_or_default();
            rows.push(LocaleSurfaceProjectionRow {
                row_id: format!("{surface_ref}:fallback:{}", state.state_id),
                surface_ref: surface_ref.to_owned(),
                row_kind: "locale_fallback_state".to_owned(),
                message_id_ref: state.message_id_ref.clone(),
                fallback_state_ref: Some(state.state_id.clone()),
                pack_id_ref: state
                    .packs_consulted
                    .first()
                    .map(|pack| pack.pack_ref.clone()),
                pack_revision_ref: state
                    .packs_consulted
                    .first()
                    .and_then(|consultation| self.pack(&consultation.pack_ref))
                    .map(|pack| pack.pack_revision_ref.clone()),
                requested_locale: state.requested_locale.clone(),
                effective_locale: state.effective_locale.clone(),
                fallback_chain: state.fallback_chain_walked.clone(),
                signature_state: state
                    .packs_consulted
                    .first()
                    .map(|pack| pack.signature_state),
                compatibility_class: state
                    .packs_consulted
                    .first()
                    .and_then(|consultation| self.pack(&consultation.pack_ref))
                    .map(|pack| pack.compatibility_class),
                command_id_ref: state.command_id_ref.clone(),
                stable_machine_id_refs,
                source_language_escape_hatches: state.source_language_escape_hatches_active.clone(),
                display_summary: state.presentation_label.clone(),
                raw_translated_body_omitted: true,
            });
        }

        LocalePackSurfaceProjection {
            record_kind: LOCALE_PACK_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: LOCALE_PACK_BETA_SCHEMA_VERSION,
            projection_id: match projection_surface {
                LocaleProjectionSurface::Settings => "locale-pack:projection:settings:v1",
                LocaleProjectionSurface::HelpAbout => "locale-pack:projection:help-about:v1",
                LocaleProjectionSurface::SupportExport => "locale-pack:projection:support:v1",
            }
            .to_owned(),
            projection_surface,
            generated_at: self.generated_at.clone(),
            source_contract_id: self.contract_id.clone(),
            active_locale_state: self.active_locale_state.clone(),
            rows,
            omitted_material_classes: vec![
                "raw_translated_message_body".to_owned(),
                "raw_docs_body".to_owned(),
                "raw_user_locale_input".to_owned(),
            ],
        }
    }

    /// Projects the metadata-only support export.
    pub fn support_export(&self) -> LocalePackSupportExport {
        let support_projection = self.surface_projection(LocaleProjectionSurface::SupportExport);
        let active_pack_versions = self
            .locale_packs
            .iter()
            .filter(|pack| {
                self.active_locale_state
                    .active_pack_refs
                    .iter()
                    .any(|active| active == &pack.pack_id)
            })
            .map(|pack| LocalePackVersionSummary {
                pack_id: pack.pack_id.clone(),
                pack_revision_ref: pack.pack_revision_ref.clone(),
                locale: pack.locale.clone(),
                source_class: pack.source_class,
                signature_state: pack.signature_state,
                compatibility_class: pack.compatibility_class,
                mirrorability_class: pack.mirrorability_class,
            })
            .collect::<Vec<_>>();

        LocalePackSupportExport {
            record_kind: LOCALE_PACK_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: LOCALE_PACK_BETA_SCHEMA_VERSION,
            support_export_id: self.active_locale_state.support_export_ref.clone(),
            generated_at: self.generated_at.clone(),
            source_contract_id: self.contract_id.clone(),
            active_locale_state: self.active_locale_state.clone(),
            active_pack_versions,
            fallback_rows: support_projection
                .rows
                .into_iter()
                .filter(|row| row.row_kind == "locale_fallback_state")
                .collect(),
            machine_identifier_posture: self.machine_identifier_posture.clone(),
            extension_locale_declarations: self.extension_locale_declarations.clone(),
            compatibility_results: self.compatibility_results.clone(),
            compatibility_waivers: self.compatibility_waivers.clone(),
            omitted_material_classes: vec![
                "raw_translated_message_body".to_owned(),
                "raw_docs_body".to_owned(),
                "raw_extension_locale_payload".to_owned(),
            ],
            raw_translated_bodies_exported: false,
        }
    }

    /// Validates pack, message, fallback, extension, governance, and export invariants.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != LOCALE_PACK_BETA_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.contract_id.clone(),
                "contract record_kind is unsupported",
            ));
        }
        if self.schema_version != LOCALE_PACK_BETA_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.contract_id.clone(),
                "contract schema_version is unsupported",
            ));
        }
        if self.source_language_locale != SOURCE_LANGUAGE_LOCALE {
            findings.push(LocalePackValidationFinding::new(
                self.contract_id.clone(),
                "contract source language drifted",
            ));
        }

        let mut pack_ids = BTreeSet::new();
        for pack in &self.locale_packs {
            validate_pack(pack, &mut findings);
            if !pack_ids.insert(pack.pack_id.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    pack.pack_id.clone(),
                    "duplicate pack id",
                ));
            }
        }

        let mut message_ids = BTreeSet::new();
        for message in &self.message_bindings {
            validate_message(message, &mut findings);
            if !message_ids.insert(message.message_id.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    message.message_id.clone(),
                    "duplicate message id",
                ));
            }
        }

        let mut fallback_ids = BTreeSet::new();
        for state in &self.fallback_states {
            validate_fallback_state(state, &message_ids, &pack_ids, &mut findings);
            if !fallback_ids.insert(state.state_id.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    state.state_id.clone(),
                    "duplicate fallback state id",
                ));
            }
        }

        validate_active_locale_state(
            &self.active_locale_state,
            &pack_ids,
            &fallback_ids,
            &mut findings,
        );

        let result_ids = self
            .compatibility_results
            .iter()
            .map(|result| result.result_id.as_str())
            .collect::<BTreeSet<_>>();
        let waiver_ids = self
            .compatibility_waivers
            .iter()
            .map(|waiver| waiver.waiver_ref.as_str())
            .collect::<BTreeSet<_>>();

        for declaration in &self.extension_locale_declarations {
            validate_extension_declaration(declaration, &pack_ids, &result_ids, &mut findings);
        }

        for action in &self.governance_actions {
            validate_governance_action(action, &pack_ids, &mut findings);
        }

        for result in &self.compatibility_results {
            validate_compatibility_result(
                result,
                &pack_ids,
                &fallback_ids,
                &waiver_ids,
                &mut findings,
            );
        }

        for waiver in &self.compatibility_waivers {
            if !pack_ids.contains(waiver.pack_ref.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    waiver.waiver_ref.clone(),
                    "waiver references an unknown pack",
                ));
            }
            if !waiver.fallback_required {
                findings.push(LocalePackValidationFinding::new(
                    waiver.waiver_ref.clone(),
                    "compatibility waiver must require fallback disclosure",
                ));
            }
        }

        for posture in &self.machine_identifier_posture {
            if !posture.locale_neutral {
                findings.push(LocalePackValidationFinding::new(
                    posture.field_family.clone(),
                    "machine identifier posture must remain locale-neutral",
                ));
            }
            if posture.stable_identifier_refs.is_empty() {
                findings.push(LocalePackValidationFinding::new(
                    posture.field_family.clone(),
                    "machine identifier posture has no stable refs",
                ));
            }
        }

        for proof in &self.protected_proofs {
            if proof.fixture_ref.trim().is_empty() || proof.exercised_axes.is_empty() {
                findings.push(LocalePackValidationFinding::new(
                    proof.proof_id.clone(),
                    "protected proof must cite a fixture and exercised axes",
                ));
            }
        }

        let support_export = self.support_export();
        if let Err(mut support_findings) = support_export.validate_against_contract(self) {
            findings.append(&mut support_findings);
        }

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

/// Validation finding emitted by the beta localization contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackValidationFinding {
    /// Row or record id that failed validation.
    pub row_ref: String,
    /// Validation message.
    pub message: String,
}

impl LocalePackValidationFinding {
    fn new(row_ref: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            row_ref: row_ref.into(),
            message: message.into(),
        }
    }
}

impl LocalePackSupportExport {
    /// Validates a support export against its source contract.
    pub fn validate_against_contract(
        &self,
        contract: &LocalePackBetaContract,
    ) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != LOCALE_PACK_SUPPORT_EXPORT_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.support_export_id.clone(),
                "support export record_kind is unsupported",
            ));
        }
        if self.schema_version != LOCALE_PACK_BETA_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.support_export_id.clone(),
                "support export schema_version is unsupported",
            ));
        }
        if self.source_contract_id != contract.contract_id {
            findings.push(LocalePackValidationFinding::new(
                self.support_export_id.clone(),
                "support export source contract drifted",
            ));
        }
        if self.raw_translated_bodies_exported {
            findings.push(LocalePackValidationFinding::new(
                self.support_export_id.clone(),
                "support export must omit raw translated bodies",
            ));
        }
        if self.active_locale_state != contract.active_locale_state {
            findings.push(LocalePackValidationFinding::new(
                self.support_export_id.clone(),
                "support export active locale state drifted",
            ));
        }
        if self.machine_identifier_posture != contract.machine_identifier_posture {
            findings.push(LocalePackValidationFinding::new(
                self.support_export_id.clone(),
                "support export machine identifier posture drifted",
            ));
        }
        for row in &self.fallback_rows {
            let Some(state_ref) = row.fallback_state_ref.as_deref() else {
                findings.push(LocalePackValidationFinding::new(
                    row.row_id.clone(),
                    "support fallback row lacks fallback state ref",
                ));
                continue;
            };
            if contract.fallback_state(state_ref).is_none() {
                findings.push(LocalePackValidationFinding::new(
                    row.row_id.clone(),
                    "support fallback row references an unknown fallback state",
                ));
            }
            if !row.raw_translated_body_omitted {
                findings.push(LocalePackValidationFinding::new(
                    row.row_id.clone(),
                    "support fallback row must omit raw translated body",
                ));
            }
        }

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

impl DenseI18nConformanceCorpus {
    /// Validates dense-surface i18n coverage, lane bindings, and invariants.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != DENSE_I18N_CORPUS_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.corpus_id.clone(),
                "dense i18n corpus record_kind is unsupported",
            ));
        }
        if self.schema_version != DENSE_I18N_CORPUS_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.corpus_id.clone(),
                "dense i18n corpus schema_version is unsupported",
            ));
        }
        if self.corpus_id != DENSE_I18N_CORPUS_ID {
            findings.push(LocalePackValidationFinding::new(
                self.corpus_id.clone(),
                "dense i18n corpus id drifted",
            ));
        }

        validate_dense_i18n_lane_bindings(&self.lane_bindings, &mut findings);

        let case_ids = self
            .surface_cases
            .iter()
            .map(|case| case.case_id.as_str())
            .collect::<BTreeSet<_>>();
        let waiver_ids = self
            .waivers
            .iter()
            .map(|waiver| waiver.waiver_ref.as_str())
            .collect::<BTreeSet<_>>();
        let mut covered_surfaces = BTreeSet::new();
        let mut covered_stress_modes = BTreeSet::new();
        let mut ime_churn_events = BTreeSet::new();

        for case in &self.surface_cases {
            validate_dense_i18n_case(
                case,
                &waiver_ids,
                &mut covered_surfaces,
                &mut covered_stress_modes,
                &mut ime_churn_events,
                &mut findings,
            );
        }

        for surface in dense_i18n_required_surface_families() {
            if !covered_surfaces.contains(&surface) {
                findings.push(LocalePackValidationFinding::new(
                    self.corpus_id.clone(),
                    format!("dense i18n corpus is missing {surface:?} coverage"),
                ));
            }
        }

        for stress_mode in dense_i18n_required_stress_modes() {
            if !covered_stress_modes.contains(&stress_mode) {
                findings.push(LocalePackValidationFinding::new(
                    self.corpus_id.clone(),
                    format!("dense i18n corpus is missing {stress_mode:?} stress coverage"),
                ));
            }
        }

        for event in [
            ImeCompositionChurnEvent::FocusChange,
            ImeCompositionChurnEvent::CompletionPreview,
            ImeCompositionChurnEvent::SnippetTraversal,
            ImeCompositionChurnEvent::CommandPreview,
        ] {
            if !ime_churn_events.contains(&event) {
                findings.push(LocalePackValidationFinding::new(
                    self.corpus_id.clone(),
                    format!("IME corpus is missing {event:?} churn coverage"),
                ));
            }
        }

        validate_translated_surface_assertions(&self.translated_surface_assertions, &mut findings);

        for waiver in &self.waivers {
            if !case_ids.contains(waiver.case_ref.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    waiver.waiver_ref.clone(),
                    "dense i18n waiver references an unknown case",
                ));
            }
            if waiver.bounded_failure_classes.is_empty()
                || waiver.expires_at.trim().is_empty()
                || waiver.required_fallback_or_workaround.trim().is_empty()
            {
                findings.push(LocalePackValidationFinding::new(
                    waiver.waiver_ref.clone(),
                    "dense i18n waiver must be bounded, expiring, and paired with fallback",
                ));
            }
        }

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    /// Projects an exportable review packet from the corpus.
    pub fn review_packet(&self) -> DenseI18nConformanceReviewPacket {
        let mut covered_surface_families = BTreeSet::new();
        let mut covered_stress_modes = BTreeSet::new();
        let waiver_refs = self
            .waivers
            .iter()
            .map(|waiver| waiver.waiver_ref.as_str())
            .collect::<BTreeSet<_>>();
        let rows = self
            .surface_cases
            .iter()
            .map(|case| {
                covered_surface_families.insert(case.surface_family);
                covered_stress_modes.extend(case.stress_modes.iter().copied());
                let bounded_waiver_active = case
                    .bounded_waiver_ref
                    .as_deref()
                    .is_some_and(|waiver_ref| waiver_refs.contains(waiver_ref));
                DenseI18nReviewRow {
                    row_id: format!("review-row:{}", case.case_id),
                    surface_family: case.surface_family,
                    case_ref: case.case_id.clone(),
                    stress_modes: case.stress_modes.clone(),
                    assertion_refs: case.assertion_refs.clone(),
                    result_state: if bounded_waiver_active {
                        "waived_bounded".to_owned()
                    } else {
                        "passed".to_owned()
                    },
                    bounded_waiver_active,
                }
            })
            .collect::<Vec<_>>();

        DenseI18nConformanceReviewPacket {
            record_kind: DENSE_I18N_REVIEW_PACKET_RECORD_KIND.to_owned(),
            schema_version: DENSE_I18N_CORPUS_SCHEMA_VERSION,
            packet_id: "i18n-conformance-review:dense-beta:v1".to_owned(),
            generated_at: self.generated_at.clone(),
            source_corpus_id: self.corpus_id.clone(),
            covered_surface_families: covered_surface_families.into_iter().collect(),
            covered_stress_modes: covered_stress_modes.into_iter().collect(),
            lane_refs: self
                .lane_bindings
                .iter()
                .map(|lane| lane.lane_id.clone())
                .collect(),
            rows,
            active_waiver_count: self.waivers.len(),
            artifact_refs: vec![
                format!("{}/manifest.json", self.fixture_root),
                format!("{}/review_export.json", self.fixture_root),
                self.review_packet_ref.clone(),
                self.rtl_and_text_expansion_audit_ref.clone(),
                self.documentation_ref.clone(),
            ],
        }
    }
}

fn validate_dense_i18n_lane_bindings(
    lanes: &[DenseI18nLaneBinding],
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let cadences = lanes
        .iter()
        .map(|lane| lane.cadence)
        .collect::<BTreeSet<_>>();
    for cadence in [
        DenseI18nLaneCadence::Nightly,
        DenseI18nLaneCadence::ReleaseCandidate,
    ] {
        if !cadences.contains(&cadence) {
            findings.push(LocalePackValidationFinding::new(
                DENSE_I18N_CORPUS_ID,
                format!("dense i18n corpus is missing {cadence:?} lane binding"),
            ));
        }
    }

    let mut lane_ids = BTreeSet::new();
    for lane in lanes {
        if !lane_ids.insert(lane.lane_id.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                lane.lane_id.clone(),
                "duplicate dense i18n lane id",
            ));
        }
        if lane.command.trim().is_empty() || !lane.command.contains("pseudoloc_rtl_ime_corpus") {
            findings.push(LocalePackValidationFinding::new(
                lane.lane_id.clone(),
                "dense i18n lane must run the pseudoloc RTL IME corpus test",
            ));
        }
        if matches!(
            lane.cadence,
            DenseI18nLaneCadence::Nightly | DenseI18nLaneCadence::ReleaseCandidate
        ) && !lane.release_blocking_for_claimed_surfaces
        {
            findings.push(LocalePackValidationFinding::new(
                lane.lane_id.clone(),
                "nightly and release-candidate dense i18n lanes must be release-blocking",
            ));
        }
        if lane.artifact_refs.is_empty() {
            findings.push(LocalePackValidationFinding::new(
                lane.lane_id.clone(),
                "dense i18n lane must export review artifacts",
            ));
        }
    }
}

fn validate_dense_i18n_case(
    case: &DenseI18nSurfaceCase,
    waiver_ids: &BTreeSet<&str>,
    covered_surfaces: &mut BTreeSet<DenseI18nSurfaceFamily>,
    covered_stress_modes: &mut BTreeSet<DenseI18nStressMode>,
    ime_churn_events: &mut BTreeSet<ImeCompositionChurnEvent>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    covered_surfaces.insert(case.surface_family);
    covered_stress_modes.extend(case.stress_modes.iter().copied());

    if case.crate_refs.is_empty()
        || case.readiness_row_refs.is_empty()
        || case.fixture_refs.is_empty()
        || case.assertion_refs.is_empty()
        || case.expected_failure_classes.is_empty()
    {
        findings.push(LocalePackValidationFinding::new(
            case.case_id.clone(),
            "dense i18n case must cite owners, readiness rows, fixtures, assertions, and failure classes",
        ));
    }

    if case.expansion_budget.single_line_min_ratio < 1.35
        || case.expansion_budget.multiline_min_ratio < 1.60
        || !case.expansion_budget.overflow_forbidden
        || !case.expansion_budget.full_text_route_required
    {
        findings.push(LocalePackValidationFinding::new(
            case.case_id.clone(),
            "dense i18n case expansion budget is below beta conformance minimums",
        ));
    }

    let stress_modes = case.stress_modes.iter().copied().collect::<BTreeSet<_>>();
    let assertions = case.assertion_refs.iter().copied().collect::<BTreeSet<_>>();

    if stress_modes.contains(&DenseI18nStressMode::ImeComposition) {
        if case.ime_scenarios.is_empty() {
            findings.push(LocalePackValidationFinding::new(
                case.case_id.clone(),
                "IME stress case must include composition scenarios",
            ));
        }
        if !assertions
            .contains(&DenseI18nAssertionClass::ImeCompositionNotSilentlyCommittedOrCancelled)
        {
            findings.push(LocalePackValidationFinding::new(
                case.case_id.clone(),
                "IME stress case must assert no silent commit or cancel",
            ));
        }
        for scenario in &case.ime_scenarios {
            ime_churn_events.extend(scenario.churn_events.iter().copied());
            if !scenario.silent_commit_forbidden
                || !scenario.silent_cancel_forbidden
                || !scenario.candidate_and_caret_visibility_required
            {
                findings.push(LocalePackValidationFinding::new(
                    scenario.scenario_id.clone(),
                    "IME scenario must forbid silent commit/cancel and keep caret visible",
                ));
            }
        }
    }

    let exercises_directionality = stress_modes.contains(&DenseI18nStressMode::RtlChrome)
        || stress_modes.contains(&DenseI18nStressMode::MixedDirectionTechnicalText);
    if exercises_directionality {
        match &case.rtl_expectation {
            Some(expectation)
                if expectation.directional_chrome_mirrors
                    && expectation.literal_technical_strings_unmirrored
                    && expectation.focus_order_tracks_visual_order
                    && expectation.raw_copy_preserves_author_order => {}
            Some(_) => findings.push(LocalePackValidationFinding::new(
                case.case_id.clone(),
                "RTL case must mirror directional chrome, preserve literals, track focus, and keep raw copy",
            )),
            None => findings.push(LocalePackValidationFinding::new(
                case.case_id.clone(),
                "directionality stress case must declare RTL expectations",
            )),
        }
        if case.literal_tokens.is_empty() {
            findings.push(LocalePackValidationFinding::new(
                case.case_id.clone(),
                "directionality stress case must include literal technical tokens",
            ));
        }
    }

    for token in &case.literal_tokens {
        if !token.must_remain_unmirrored || !token.copy_raw_required {
            findings.push(LocalePackValidationFinding::new(
                case.case_id.clone(),
                "literal technical tokens must remain unmirrored and raw-copyable",
            ));
        }
        if exercises_directionality && !token.copy_escaped_required {
            findings.push(LocalePackValidationFinding::new(
                case.case_id.clone(),
                "directionality tokens must keep escaped copy available",
            ));
        }
    }

    if stress_modes.contains(&DenseI18nStressMode::CjkFontFallback)
        && !assertions.contains(&DenseI18nAssertionClass::FontFallbackWorks)
    {
        findings.push(LocalePackValidationFinding::new(
            case.case_id.clone(),
            "CJK stress case must assert font fallback",
        ));
    }

    if stress_modes.contains(&DenseI18nStressMode::LocaleFallbackReview)
        && !assertions.contains(&DenseI18nAssertionClass::LocaleFallbackDisclosedAndNonBlocking)
    {
        findings.push(LocalePackValidationFinding::new(
            case.case_id.clone(),
            "locale fallback case must assert disclosed non-blocking fallback",
        ));
    }

    if stress_modes.contains(&DenseI18nStressMode::TranslatedSurfaceParity)
        && (!assertions.contains(&DenseI18nAssertionClass::StableTranslatedSurfaceRefsPreserved)
            || !assertions.contains(&DenseI18nAssertionClass::SourceLanguageEscapeHatchAvailable))
    {
        findings.push(LocalePackValidationFinding::new(
            case.case_id.clone(),
            "translated-surface case must preserve stable refs and source-language access",
        ));
    }

    if let Some(waiver_ref) = case.bounded_waiver_ref.as_deref() {
        if !waiver_ids.contains(waiver_ref) {
            findings.push(LocalePackValidationFinding::new(
                case.case_id.clone(),
                "dense i18n case references an unknown waiver",
            ));
        }
    }
}

fn validate_translated_surface_assertions(
    assertions: &[TranslatedSurfaceAssertion],
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let mut ref_kinds = BTreeSet::new();
    for assertion in assertions {
        if assertion.preserved_refs.is_empty()
            || !assertion.open_in_source_language_required
            || !assertion.machine_output_locale_neutral
            || !assertion.governed_terminology_preserved
        {
            findings.push(LocalePackValidationFinding::new(
                assertion.assertion_id.clone(),
                "translated-surface assertion must preserve refs, machine neutrality, terminology, and source access",
            ));
        }
        ref_kinds.extend(
            assertion
                .preserved_refs
                .iter()
                .map(|item| item.ref_kind.as_str()),
        );
    }

    for required_kind in [
        "command_id",
        "keyboard_path",
        "citation_anchor",
        "scope_label",
    ] {
        if !ref_kinds.contains(required_kind) {
            findings.push(LocalePackValidationFinding::new(
                DENSE_I18N_CORPUS_ID,
                format!("translated-surface assertions are missing {required_kind} preservation"),
            ));
        }
    }
}

fn dense_i18n_required_surface_families() -> Vec<DenseI18nSurfaceFamily> {
    vec![
        DenseI18nSurfaceFamily::Editor,
        DenseI18nSurfaceFamily::CommandPalette,
        DenseI18nSurfaceFamily::Settings,
        DenseI18nSurfaceFamily::TreeView,
        DenseI18nSurfaceFamily::TableView,
        DenseI18nSurfaceFamily::LogView,
        DenseI18nSurfaceFamily::Terminal,
        DenseI18nSurfaceFamily::ReviewPane,
        DenseI18nSurfaceFamily::GuidedTour,
        DenseI18nSurfaceFamily::DocsHelp,
    ]
}

fn dense_i18n_required_stress_modes() -> Vec<DenseI18nStressMode> {
    vec![
        DenseI18nStressMode::SourceLanguageBaseline,
        DenseI18nStressMode::PseudolocExpansion,
        DenseI18nStressMode::RtlChrome,
        DenseI18nStressMode::MixedDirectionTechnicalText,
        DenseI18nStressMode::ImeComposition,
        DenseI18nStressMode::CjkFontFallback,
        DenseI18nStressMode::TextExpansion,
        DenseI18nStressMode::LocaleFallbackReview,
        DenseI18nStressMode::TranslatedSurfaceParity,
        DenseI18nStressMode::LocalizedDateNumberFormatting,
    ]
}

/// Returns the seeded beta localization contract.
pub fn seeded_locale_pack_beta_contract() -> LocalePackBetaContract {
    let policy_context = policy_context();

    let source_pack = locale_pack(LocalePackSeed {
        pack_id: "locale-pack:core:source:en-us",
        pack_revision_ref: "locale-pack-rev:core:source:en-us:2026.05.18-01",
        locale: SOURCE_LANGUAGE_LOCALE,
        coverage_locales: vec![SOURCE_LANGUAGE_LOCALE],
        fallback_chain: vec![SOURCE_LANGUAGE_LOCALE],
        source_class: LocalePackSourceClass::FirstPartySourceLanguage,
        distribution_class: LocalePackDistributionClass::BuiltInWithProduct,
        signature_state: LocalePackSignatureState::NotApplicableBuiltIn,
        mirrorability_class: LocalePackMirrorabilityClass::MirrorAllowed,
        compatibility_class: VersionMatchState::ExactBuildMatch,
        covered_surface_families: all_surface_families(),
        partially_translated_surface_families: Vec::new(),
        extension_overlay_pack_refs: Vec::new(),
        extension_namespace_refs: Vec::new(),
        permitted_deployment_profiles: all_deployment_profiles(),
        source_artifact_ref: "source:locale:core:en-us:built-in".to_owned(),
        signer_identity_ref: None,
        signature_artifact_ref: None,
        mirror_receipt_refs: vec!["mirror-receipt:core:source:en-us".to_owned()],
        offline_import_ref: Some("offline-import:core:source:en-us".to_owned()),
        rollback_ref: "rollback:locale-pack:core:source:en-us:last-known-good".to_owned(),
        presentation_label: "English source language".to_owned(),
        policy_context: policy_context.clone(),
    });

    let spanish_pack = locale_pack(LocalePackSeed {
        pack_id: "locale-pack:core:es-mx:beta",
        pack_revision_ref: "locale-pack-rev:core:es-mx:2026.05.18-01",
        locale: "es-MX",
        coverage_locales: vec!["es-MX", "es"],
        fallback_chain: vec!["es-MX", "es", SOURCE_LANGUAGE_LOCALE],
        source_class: LocalePackSourceClass::FirstPartyLocalePack,
        distribution_class: LocalePackDistributionClass::MirroredOfficialPack,
        signature_state: LocalePackSignatureState::SignedVerified,
        mirrorability_class: LocalePackMirrorabilityClass::MirrorWithAttributionRequired,
        compatibility_class: VersionMatchState::ExactBuildMatch,
        covered_surface_families: vec![
            MessageSurfaceFamily::ShellChrome,
            MessageSurfaceFamily::CommandLabel,
            MessageSurfaceFamily::SettingsHelpOrError,
            MessageSurfaceFamily::DocsTourOrAuthText,
            MessageSurfaceFamily::CliHelpText,
            MessageSurfaceFamily::PolicyLegalOrRecoveryText,
            MessageSurfaceFamily::ExportOrReportHeading,
        ],
        partially_translated_surface_families: vec![MessageSurfaceFamily::DocsTourOrAuthText],
        extension_overlay_pack_refs: Vec::new(),
        extension_namespace_refs: Vec::new(),
        permitted_deployment_profiles: vec![
            "individual_local",
            "self_hosted",
            "enterprise_online",
            "air_gapped",
            "managed_cloud",
        ],
        source_artifact_ref: "artifact:locale-pack:core:es-mx:2026.05.18-01".to_owned(),
        signer_identity_ref: Some("signer:first-party:locale-pack-release-root".to_owned()),
        signature_artifact_ref: Some("signature:locale-pack:core:es-mx:2026.05.18-01".to_owned()),
        mirror_receipt_refs: vec![
            "mirror-receipt:official:locale-pack:core:es-mx".to_owned(),
            "mirror-receipt:airgap:locale-pack:core:es-mx".to_owned(),
        ],
        offline_import_ref: Some("offline-import:locale-pack:core:es-mx:bundle-01".to_owned()),
        rollback_ref: "rollback:locale-pack:core:es-mx:2026.05.17-01".to_owned(),
        presentation_label: "Spanish (Mexico) official pack".to_owned(),
        policy_context: policy_context.clone(),
    });

    let portuguese_pack = locale_pack(LocalePackSeed {
        pack_id: "locale-pack:community:pt-br:beta",
        pack_revision_ref: "locale-pack-rev:community:pt-br:2026.05.18-01",
        locale: "pt-BR",
        coverage_locales: vec!["pt-BR", "pt"],
        fallback_chain: vec!["pt-BR", "pt", SOURCE_LANGUAGE_LOCALE],
        source_class: LocalePackSourceClass::ReviewedCommunityPack,
        distribution_class: LocalePackDistributionClass::CommunitySuppliedPack,
        signature_state: LocalePackSignatureState::SignedVerified,
        mirrorability_class: LocalePackMirrorabilityClass::MirrorWithAttributionRequired,
        compatibility_class: VersionMatchState::CompatibleMinorDrift,
        covered_surface_families: vec![
            MessageSurfaceFamily::CommandLabel,
            MessageSurfaceFamily::SettingsHelpOrError,
            MessageSurfaceFamily::DocsTourOrAuthText,
            MessageSurfaceFamily::CliHelpText,
            MessageSurfaceFamily::GlossaryOrTerminologyTerm,
        ],
        partially_translated_surface_families: vec![
            MessageSurfaceFamily::GlossaryOrTerminologyTerm,
        ],
        extension_overlay_pack_refs: Vec::new(),
        extension_namespace_refs: Vec::new(),
        permitted_deployment_profiles: vec!["individual_local", "self_hosted", "enterprise_online"],
        source_artifact_ref: "artifact:locale-pack:community:pt-br:2026.05.18-01".to_owned(),
        signer_identity_ref: Some("signer:reviewed-community:locale-pack-pt-br".to_owned()),
        signature_artifact_ref: Some(
            "signature:locale-pack:community:pt-br:2026.05.18-01".to_owned(),
        ),
        mirror_receipt_refs: vec!["mirror-receipt:community-reviewed:pt-br".to_owned()],
        offline_import_ref: Some("offline-import:locale-pack:community:pt-br:bundle-01".to_owned()),
        rollback_ref: "rollback:locale-pack:community:pt-br:2026.05.10-01".to_owned(),
        presentation_label: "Portuguese (Brazil) reviewed community pack".to_owned(),
        policy_context: policy_context.clone(),
    });

    let extension_pack = locale_pack(LocalePackSeed {
        pack_id: "locale-pack:extension:docs-helper:de-de:blocked",
        pack_revision_ref: "locale-pack-rev:extension:docs-helper:de-de:2026.05.18-01",
        locale: "de-DE",
        coverage_locales: vec!["de-DE", "de"],
        fallback_chain: vec!["de-DE", "de", SOURCE_LANGUAGE_LOCALE],
        source_class: LocalePackSourceClass::ExtensionOwnedPack,
        distribution_class: LocalePackDistributionClass::ExtensionOverlayPack,
        signature_state: LocalePackSignatureState::SignatureFailedBlocked,
        mirrorability_class: LocalePackMirrorabilityClass::MirrorForbidden,
        compatibility_class: VersionMatchState::IncompatibleDriftDetected,
        covered_surface_families: vec![MessageSurfaceFamily::ExtensionContributedUi],
        partially_translated_surface_families: vec![MessageSurfaceFamily::ExtensionContributedUi],
        extension_overlay_pack_refs: Vec::new(),
        extension_namespace_refs: vec!["ext:namespace:docs-helper".to_owned()],
        permitted_deployment_profiles: vec!["individual_local", "self_hosted", "enterprise_online"],
        source_artifact_ref: "artifact:locale-pack:extension:docs-helper:de-de:2026.05.18-01"
            .to_owned(),
        signer_identity_ref: Some("signer:extension:docs-helper".to_owned()),
        signature_artifact_ref: Some(
            "signature:locale-pack:extension:docs-helper:de-de:2026.05.18-01".to_owned(),
        ),
        mirror_receipt_refs: Vec::new(),
        offline_import_ref: None,
        rollback_ref: "rollback:locale-pack:extension:docs-helper:source-language".to_owned(),
        presentation_label: "Docs Helper German overlay (blocked)".to_owned(),
        policy_context: policy_context.clone(),
    });

    let message_bindings = vec![
        message_binding(MessageSeed {
            message_id: "msg:shell:palette:open-folder:title",
            surface_family: MessageSurfaceFamily::CommandLabel,
            source_text: "Open folder",
            stable_refs: StableMessageIdentityRefs {
                command_id_ref: Some("cmd:core:open_folder".to_owned()),
                semantic_action_id_ref: Some("action:workspace.open_folder".to_owned()),
                telemetry_key_ref: Some("telemetry:command_palette.command_invoked".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: Vec::new(),
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
            source_language_escape_hatches: vec![
                SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle,
                SourceLanguageEscapeHatchClass::CommandOpenInSourceLanguage,
            ],
            translation_review_refs: vec!["review:locale:command-identity-parity".to_owned()],
            extension_namespace_ref: None,
        }),
        message_binding(MessageSeed {
            message_id: "msg:settings:i18n:active-locale:description",
            surface_family: MessageSurfaceFamily::SettingsHelpOrError,
            source_text: "Shows the active locale, fallback chain, and pack verification state.",
            stable_refs: StableMessageIdentityRefs {
                setting_id_ref: Some("settings.i18n.active_locale".to_owned()),
                schema_id_ref: Some("schemas/i18n/locale_pack_manifest.schema.json".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: vec![MessagePlaceholder {
                placeholder_id: "locale".to_owned(),
                placeholder_kind: "locale_tag_token".to_owned(),
                translator_note: "BCP-47 locale tag; preserve literal spelling.".to_owned(),
            }],
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNeutralWithTranslatedHumanField,
            source_language_escape_hatches: vec![
                SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle,
                SourceLanguageEscapeHatchClass::ExportInSourceLanguageForReview,
            ],
            translation_review_refs: vec!["review:locale:settings-schema-id-parity".to_owned()],
            extension_namespace_ref: None,
        }),
        message_binding(MessageSeed {
            message_id: "msg:docs:onboarding:open-folder:summary",
            surface_family: MessageSurfaceFamily::DocsTourOrAuthText,
            source_text: "Open a local folder and keep citations available.",
            stable_refs: StableMessageIdentityRefs {
                command_id_ref: Some("cmd:core:open_folder".to_owned()),
                docs_pack_key_ref: Some("docs-pack:onboarding:first-useful-work:open-folder".to_owned()),
                semantic_action_id_ref: Some("action:onboarding.open_folder".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: Vec::new(),
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
            source_language_escape_hatches: vec![
                SourceLanguageEscapeHatchClass::DocsPaneSourceLanguageRoute,
                SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle,
            ],
            translation_review_refs: vec!["review:locale:docs-citation-parity".to_owned()],
            extension_namespace_ref: None,
        }),
        message_binding(MessageSeed {
            message_id: "msg:doctor:profile-schema-drift:human",
            surface_family: MessageSurfaceFamily::CliHelpText,
            source_text: "Profile schema drift detected.",
            stable_refs: StableMessageIdentityRefs {
                diagnostic_id_ref: Some("doctor.finding.profile.schema_drift".to_owned()),
                schema_id_ref: Some("schemas/diagnostics/diagnostic_record.schema.json".to_owned()),
                telemetry_key_ref: Some("telemetry:doctor.finding_emitted".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: vec![MessagePlaceholder {
                placeholder_id: "finding_code".to_owned(),
                placeholder_kind: "enumerated_state_token".to_owned(),
                translator_note: "Doctor finding code; never translate in JSON output.".to_owned(),
            }],
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNeutralWithTranslatedHumanField,
            source_language_escape_hatches: vec![
                SourceLanguageEscapeHatchClass::CliLocaleNeutralOutputFlag,
                SourceLanguageEscapeHatchClass::ExportInSourceLanguageForReview,
            ],
            translation_review_refs: vec!["review:locale:doctor-code-neutrality".to_owned()],
            extension_namespace_ref: None,
        }),
        message_binding(MessageSeed {
            message_id: "msg:auth:recovery:source-language-fallback",
            surface_family: MessageSurfaceFamily::PolicyLegalOrRecoveryText,
            source_text: "Recovery instructions are shown in source language because the locale pack is incomplete.",
            stable_refs: StableMessageIdentityRefs {
                semantic_action_id_ref: Some("action:auth.recovery.open_source_language".to_owned()),
                policy_name_ref: Some("policy.locale.source_language_fallback_required".to_owned()),
                docs_pack_key_ref: Some("docs-pack:recovery:source-language-fallback".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: Vec::new(),
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
            source_language_escape_hatches: vec![
                SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle,
                SourceLanguageEscapeHatchClass::DocsPaneSourceLanguageRoute,
            ],
            translation_review_refs: vec![
                "review:locale:policy-recovery-copy".to_owned(),
                "review:locale:source-language-escape-hatch".to_owned(),
            ],
            extension_namespace_ref: None,
        }),
        message_binding(MessageSeed {
            message_id: "msg:extension:docs-helper:open-related-docs:title",
            surface_family: MessageSurfaceFamily::ExtensionContributedUi,
            source_text: "Open related docs",
            stable_refs: StableMessageIdentityRefs {
                command_id_ref: Some("ext:docs-helper:command:open_related_docs".to_owned()),
                semantic_action_id_ref: Some("ext:docs-helper:action.open_related_docs".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: Vec::new(),
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
            source_language_escape_hatches: vec![
                SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle,
                SourceLanguageEscapeHatchClass::CommandOpenInSourceLanguage,
            ],
            translation_review_refs: vec!["review:locale:extension-no-host-override".to_owned()],
            extension_namespace_ref: Some("ext:namespace:docs-helper".to_owned()),
        }),
    ];

    let fallback_states = vec![
        fallback_state(FallbackSeed {
            state_id: "locale-fallback:command:open-folder:es-mx:authoritative",
            requested_locale: "es-MX",
            effective_locale: "es-MX",
            fallback_origin_class: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
            degraded_localization_state: DegradedLocalizationState::FullyLocalized,
            fallback_chain_walked: vec!["es-MX"],
            packs_consulted: vec![PackConsultationDescriptor {
                pack_ref: "locale-pack:core:es-mx:beta".to_owned(),
                signature_state: LocalePackSignatureState::SignedVerified,
                consulted_locale: "es-MX".to_owned(),
                produced_message: true,
            }],
            message_id_ref: Some("msg:shell:palette:open-folder:title".to_owned()),
            command_id_ref: Some("cmd:core:open_folder".to_owned()),
            disclosed_to_reviewer: false,
            source_language_escape_hatches_active: vec![SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle],
            surface_family: MessageSurfaceFamily::CommandLabel,
            command_id_preservation_state: CommandIdPreservationState::CommandIdUnchangedAcrossFallback,
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
            denial_reason_on_deny: None,
            presentation_label: "Command label renders from the requested locale with canonical command id preserved.",
            presentation_subtitle: Some("cmd:core:open_folder remains the route key.".to_owned()),
        }),
        fallback_state(FallbackSeed {
            state_id: "locale-fallback:settings:active-locale:es-mx:base-fill",
            requested_locale: "es-MX",
            effective_locale: "es",
            fallback_origin_class: LocaleFallbackOriginClass::BaseLocaleFallback,
            degraded_localization_state: DegradedLocalizationState::PartialTranslationDisclosed,
            fallback_chain_walked: vec!["es-MX", "es"],
            packs_consulted: vec![PackConsultationDescriptor {
                pack_ref: "locale-pack:core:es-mx:beta".to_owned(),
                signature_state: LocalePackSignatureState::SignedVerified,
                consulted_locale: "es".to_owned(),
                produced_message: true,
            }],
            message_id_ref: Some("msg:settings:i18n:active-locale:description".to_owned()),
            command_id_ref: None,
            disclosed_to_reviewer: true,
            source_language_escape_hatches_active: vec![
                SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle,
                SourceLanguageEscapeHatchClass::ExportInSourceLanguageForReview,
            ],
            surface_family: MessageSurfaceFamily::SettingsHelpOrError,
            command_id_preservation_state: CommandIdPreservationState::NotApplicable,
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNeutralWithTranslatedHumanField,
            denial_reason_on_deny: None,
            presentation_label: "Settings locale row discloses base-locale fill from es.",
            presentation_subtitle: Some("Setting id and schema id remain locale-neutral.".to_owned()),
        }),
        fallback_state(FallbackSeed {
            state_id: "locale-fallback:docs:glossary:pt-br:source-language",
            requested_locale: "pt-BR",
            effective_locale: SOURCE_LANGUAGE_LOCALE,
            fallback_origin_class: LocaleFallbackOriginClass::SourceLanguageFallback,
            degraded_localization_state: DegradedLocalizationState::GlossaryOnlyLocalized,
            fallback_chain_walked: vec!["pt-BR", "pt", SOURCE_LANGUAGE_LOCALE],
            packs_consulted: vec![
                PackConsultationDescriptor {
                    pack_ref: "locale-pack:community:pt-br:beta".to_owned(),
                    signature_state: LocalePackSignatureState::SignedVerified,
                    consulted_locale: "pt-BR".to_owned(),
                    produced_message: false,
                },
                PackConsultationDescriptor {
                    pack_ref: "locale-pack:core:source:en-us".to_owned(),
                    signature_state: LocalePackSignatureState::NotApplicableBuiltIn,
                    consulted_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
                    produced_message: true,
                },
            ],
            message_id_ref: Some("msg:docs:onboarding:open-folder:summary".to_owned()),
            command_id_ref: Some("cmd:core:open_folder".to_owned()),
            disclosed_to_reviewer: true,
            source_language_escape_hatches_active: vec![
                SourceLanguageEscapeHatchClass::DocsPaneSourceLanguageRoute,
                SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle,
            ],
            surface_family: MessageSurfaceFamily::DocsTourOrAuthText,
            command_id_preservation_state: CommandIdPreservationState::CommandIdUnchangedAcrossFallback,
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
            denial_reason_on_deny: None,
            presentation_label: "Docs/tour row falls back through pt to source language with citations preserved.",
            presentation_subtitle: Some("Docs pack key and command id remain stable.".to_owned()),
        }),
        fallback_state(FallbackSeed {
            state_id: "locale-fallback:cli:doctor:ja-jp:missing-pack",
            requested_locale: "ja-JP",
            effective_locale: SOURCE_LANGUAGE_LOCALE,
            fallback_origin_class: LocaleFallbackOriginClass::PackMissingSourceLanguageOnly,
            degraded_localization_state: DegradedLocalizationState::FailedPackSourceLanguageOnly,
            fallback_chain_walked: vec!["ja-JP", "ja", SOURCE_LANGUAGE_LOCALE],
            packs_consulted: vec![PackConsultationDescriptor {
                pack_ref: "locale-pack:core:source:en-us".to_owned(),
                signature_state: LocalePackSignatureState::NotApplicableBuiltIn,
                consulted_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
                produced_message: true,
            }],
            message_id_ref: Some("msg:doctor:profile-schema-drift:human".to_owned()),
            command_id_ref: None,
            disclosed_to_reviewer: true,
            source_language_escape_hatches_active: vec![
                SourceLanguageEscapeHatchClass::CliLocaleNeutralOutputFlag,
                SourceLanguageEscapeHatchClass::ExportInSourceLanguageForReview,
            ],
            surface_family: MessageSurfaceFamily::CliHelpText,
            command_id_preservation_state: CommandIdPreservationState::NotApplicable,
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNeutralWithTranslatedHumanField,
            denial_reason_on_deny: None,
            presentation_label: "CLI/Doctor prose falls back to source language while JSON keys and finding codes stay neutral.",
            presentation_subtitle: Some("doctor.finding.profile.schema_drift remains the finding id.".to_owned()),
        }),
        fallback_state(FallbackSeed {
            state_id: "locale-fallback:extension:docs-helper:de-de:signature-failed",
            requested_locale: "de-DE",
            effective_locale: SOURCE_LANGUAGE_LOCALE,
            fallback_origin_class: LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly,
            degraded_localization_state: DegradedLocalizationState::FailedPackSourceLanguageOnly,
            fallback_chain_walked: vec!["de-DE", "de", SOURCE_LANGUAGE_LOCALE],
            packs_consulted: vec![
                PackConsultationDescriptor {
                    pack_ref: "locale-pack:extension:docs-helper:de-de:blocked".to_owned(),
                    signature_state: LocalePackSignatureState::SignatureFailedBlocked,
                    consulted_locale: "de-DE".to_owned(),
                    produced_message: false,
                },
                PackConsultationDescriptor {
                    pack_ref: "locale-pack:core:source:en-us".to_owned(),
                    signature_state: LocalePackSignatureState::NotApplicableBuiltIn,
                    consulted_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
                    produced_message: true,
                },
            ],
            message_id_ref: Some("msg:extension:docs-helper:open-related-docs:title".to_owned()),
            command_id_ref: Some("ext:docs-helper:command:open_related_docs".to_owned()),
            disclosed_to_reviewer: true,
            source_language_escape_hatches_active: vec![
                SourceLanguageEscapeHatchClass::InlineSourceLanguageToggle,
                SourceLanguageEscapeHatchClass::CommandOpenInSourceLanguage,
            ],
            surface_family: MessageSurfaceFamily::ExtensionContributedUi,
            command_id_preservation_state: CommandIdPreservationState::CommandIdUnchangedAcrossFallback,
            machine_output_locale_class: MachineOutputLocaleClass::LocaleNativeHumanOnly,
            denial_reason_on_deny: Some("localization_locale_pack_signature_failed".to_owned()),
            presentation_label: "Extension overlay signature failed; source-language extension copy is shown and host ids stay protected.",
            presentation_subtitle: Some("Extension command id stays namespaced; host ids cannot be overridden.".to_owned()),
        }),
    ];

    let active_locale_state = ActiveLocaleState {
        record_kind: "active_locale_state_record".to_owned(),
        schema_version: LOCALE_PACK_BETA_SCHEMA_VERSION,
        state_id: "locale-state:active:pt-br:beta".to_owned(),
        requested_locale: "pt-BR".to_owned(),
        effective_locale: "pt-BR".to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        fallback_chain: vec![
            "pt-BR".to_owned(),
            "pt".to_owned(),
            SOURCE_LANGUAGE_LOCALE.to_owned(),
        ],
        active_pack_refs: vec![
            "locale-pack:community:pt-br:beta".to_owned(),
            "locale-pack:core:source:en-us".to_owned(),
        ],
        active_fallback_state_refs: vec![
            "locale-fallback:docs:glossary:pt-br:source-language".to_owned(),
            "locale-fallback:cli:doctor:ja-jp:missing-pack".to_owned(),
            "locale-fallback:extension:docs-helper:de-de:signature-failed".to_owned(),
        ],
        source_language_fallback_active: true,
        settings_projection_ref: "locale-pack:projection:settings:v1".to_owned(),
        help_about_projection_ref: "locale-pack:projection:help-about:v1".to_owned(),
        support_export_ref: "support-export:locale-pack:beta:v1".to_owned(),
        signature_state_summary: "active packs: signed_verified + source built-in; blocked extension overlay recorded separately".to_owned(),
        generated_at: GENERATED_AT.to_owned(),
    };

    let source_contract_refs = BTreeMap::from([
        (
            "localization_contract".to_owned(),
            "docs/ux/localization_and_locale_pack_contract.md".to_owned(),
        ),
        (
            "locale_pack_schema".to_owned(),
            "schemas/i18n/locale_pack_manifest.schema.json".to_owned(),
        ),
        (
            "ux_locale_pack_schema".to_owned(),
            "schemas/ux/locale_pack_manifest.schema.json".to_owned(),
        ),
        (
            "message_catalog_schema".to_owned(),
            "schemas/ux/message_catalog_entry.schema.json".to_owned(),
        ),
        (
            "fallback_schema".to_owned(),
            "schemas/ux/locale_fallback_state.schema.json".to_owned(),
        ),
    ]);

    let extension_locale_declarations = vec![
        ExtensionLocaleDeclaration {
            extension_id: "ext:theme-tools".to_owned(),
            extension_namespace_ref: "ext:namespace:theme-tools".to_owned(),
            support_mode: ExtensionLocaleSupportMode::InheritsHostLocale,
            inherits_host_fallback_disclosure: true,
            locale_pack_ref: None,
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            may_override_host_stable_ids: false,
            compatibility_result_ref: "compat:locale-pack:extension:theme-tools:inherits-host"
                .to_owned(),
            visible_disclosure_required: true,
        },
        ExtensionLocaleDeclaration {
            extension_id: "ext:docs-helper".to_owned(),
            extension_namespace_ref: "ext:namespace:docs-helper".to_owned(),
            support_mode: ExtensionLocaleSupportMode::ShipsOwnLocalePack,
            inherits_host_fallback_disclosure: false,
            locale_pack_ref: Some("locale-pack:extension:docs-helper:de-de:blocked".to_owned()),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            may_override_host_stable_ids: false,
            compatibility_result_ref: "compat:locale-pack:extension:docs-helper:de-de:blocked"
                .to_owned(),
            visible_disclosure_required: true,
        },
        ExtensionLocaleDeclaration {
            extension_id: "ext:legacy-build-runner".to_owned(),
            extension_namespace_ref: "ext:namespace:legacy-build-runner".to_owned(),
            support_mode: ExtensionLocaleSupportMode::SourceLanguageOnly,
            inherits_host_fallback_disclosure: true,
            locale_pack_ref: None,
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            may_override_host_stable_ids: false,
            compatibility_result_ref:
                "compat:locale-pack:extension:legacy-build-runner:source-only".to_owned(),
            visible_disclosure_required: true,
        },
    ];

    let governance_actions = vec![
        governance_action(
            "locale-pack-action:install:community:pt-br",
            LocalePackOperationClass::Install,
            "locale-pack:community:pt-br:beta",
            Some("rollback:locale-pack:community:pt-br:2026.05.10-01"),
        ),
        governance_action(
            "locale-pack-action:update:core:es-mx",
            LocalePackOperationClass::Update,
            "locale-pack:core:es-mx:beta",
            Some("rollback:locale-pack:core:es-mx:2026.05.17-01"),
        ),
        governance_action(
            "locale-pack-action:rollback:community:pt-br",
            LocalePackOperationClass::Rollback,
            "locale-pack:community:pt-br:beta",
            Some("rollback:locale-pack:community:pt-br:2026.05.10-01"),
        ),
        governance_action(
            "locale-pack-action:mirror-import:core:es-mx",
            LocalePackOperationClass::MirrorImport,
            "locale-pack:core:es-mx:beta",
            Some("rollback:locale-pack:core:es-mx:2026.05.17-01"),
        ),
        governance_action(
            "locale-pack-action:offline-import:community:pt-br",
            LocalePackOperationClass::OfflineImport,
            "locale-pack:community:pt-br:beta",
            Some("rollback:locale-pack:community:pt-br:2026.05.10-01"),
        ),
    ];

    let compatibility_waivers = vec![LocalePackCompatibilityWaiver {
        waiver_ref: "waiver:locale-pack:community:pt-br:glossary-partial:2026.05.25".to_owned(),
        pack_ref: "locale-pack:community:pt-br:beta".to_owned(),
        bounded_to_surface_families: vec![MessageSurfaceFamily::GlossaryOrTerminologyTerm],
        reason: "Reviewed community pack is admitted for beta with glossary rows falling back to source language.".to_owned(),
        expires_at: "2026-05-25T00:00:00Z".to_owned(),
        fallback_required: true,
        release_packet_ref: "artifacts/ux/m3/locale_pack_and_fallback_review.md".to_owned(),
    }];

    let compatibility_results = vec![
        compatibility_result(
            "compat:locale-pack:core:es-mx:beta",
            "locale-pack:core:es-mx:beta",
            LocaleCompatibilityOutcome::Compatible,
            LocalePackSignatureState::SignedVerified,
            VersionMatchState::ExactBuildMatch,
            vec!["locale-fallback:settings:active-locale:es-mx:base-fill"],
            None,
        ),
        compatibility_result(
            "compat:locale-pack:community:pt-br:beta",
            "locale-pack:community:pt-br:beta",
            LocaleCompatibilityOutcome::CompatibleWithWaiver,
            LocalePackSignatureState::SignedVerified,
            VersionMatchState::CompatibleMinorDrift,
            vec!["locale-fallback:docs:glossary:pt-br:source-language"],
            Some("waiver:locale-pack:community:pt-br:glossary-partial:2026.05.25"),
        ),
        compatibility_result(
            "compat:locale-pack:extension:docs-helper:de-de:blocked",
            "locale-pack:extension:docs-helper:de-de:blocked",
            LocaleCompatibilityOutcome::BlockedSignatureFailure,
            LocalePackSignatureState::SignatureFailedBlocked,
            VersionMatchState::IncompatibleDriftDetected,
            vec!["locale-fallback:extension:docs-helper:de-de:signature-failed"],
            None,
        ),
    ];

    let machine_identifier_posture = vec![
        MachineIdentifierPosture {
            field_family: "cli_json_keys".to_owned(),
            stable_identifier_refs: vec![
                "json-key:command_id".to_owned(),
                "json-key:finding_code".to_owned(),
                "json-key:locale.fallback_chain".to_owned(),
            ],
            locale_neutral: true,
            human_prose_overlay_allowed: true,
        },
        MachineIdentifierPosture {
            field_family: "doctor_finding_codes".to_owned(),
            stable_identifier_refs: vec!["doctor.finding.profile.schema_drift".to_owned()],
            locale_neutral: true,
            human_prose_overlay_allowed: true,
        },
        MachineIdentifierPosture {
            field_family: "policy_and_telemetry_keys".to_owned(),
            stable_identifier_refs: vec![
                "policy.locale.source_language_fallback_required".to_owned(),
                "telemetry:command_palette.command_invoked".to_owned(),
            ],
            locale_neutral: true,
            human_prose_overlay_allowed: false,
        },
    ];

    LocalePackBetaContract {
        record_kind: LOCALE_PACK_BETA_RECORD_KIND.to_owned(),
        schema_version: LOCALE_PACK_BETA_SCHEMA_VERSION,
        contract_id: LOCALE_PACK_BETA_CONTRACT_ID.to_owned(),
        contract_version_ref: LOCALE_PACK_BETA_VERSION_REF.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        release_channel: "beta".to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        requested_locale: "pt-BR".to_owned(),
        active_locale_state,
        source_contract_refs,
        runtime_consumer_refs: vec![
            "crates/aureline-shell".to_owned(),
            "crates/aureline-settings".to_owned(),
            "crates/aureline-docs".to_owned(),
            "crates/aureline-support".to_owned(),
            "crates/aureline-extensions".to_owned(),
        ],
        message_bindings,
        locale_packs: vec![source_pack, spanish_pack, portuguese_pack, extension_pack],
        fallback_states,
        extension_locale_declarations,
        governance_actions,
        compatibility_results,
        compatibility_waivers,
        machine_identifier_posture,
        protected_proofs: vec![
            LocalePackProtectedProof {
                proof_id: "proof:locale-pack:manifest".to_owned(),
                fixture_ref: "fixtures/i18n/m3/locale_fallback/manifest.json".to_owned(),
                exercised_axes: vec![
                    "stable_message_ids".to_owned(),
                    "pack_signature_state".to_owned(),
                    "mirror_offline_governance".to_owned(),
                ],
            },
            LocalePackProtectedProof {
                proof_id: "proof:locale-pack:support-export".to_owned(),
                fixture_ref: "fixtures/i18n/m3/locale_fallback/support_export.json".to_owned(),
                exercised_axes: vec![
                    "support_export_metadata_only".to_owned(),
                    "machine_identifier_locale_neutrality".to_owned(),
                    "fallback_chain_inspection".to_owned(),
                ],
            },
        ],
    }
}

/// Returns the seeded settings projection.
pub fn seeded_locale_pack_settings_projection() -> LocalePackSurfaceProjection {
    seeded_locale_pack_beta_contract().surface_projection(LocaleProjectionSurface::Settings)
}

/// Returns the seeded Help/About projection.
pub fn seeded_locale_pack_help_about_projection() -> LocalePackSurfaceProjection {
    seeded_locale_pack_beta_contract().surface_projection(LocaleProjectionSurface::HelpAbout)
}

/// Returns the seeded support projection.
pub fn seeded_locale_pack_support_projection() -> LocalePackSurfaceProjection {
    seeded_locale_pack_beta_contract().surface_projection(LocaleProjectionSurface::SupportExport)
}

/// Returns the seeded metadata-only support export.
pub fn seeded_locale_pack_support_export() -> LocalePackSupportExport {
    seeded_locale_pack_beta_contract().support_export()
}

/// Returns the seeded dense-surface i18n conformance corpus.
pub fn seeded_dense_i18n_conformance_corpus() -> DenseI18nConformanceCorpus {
    let fixture_root = "fixtures/i18n/m3/pseudoloc_rtl_ime_corpus";
    let lane_command = "cargo test -p aureline-i18n --test pseudoloc_rtl_ime_corpus --locked";
    let review_artifact = "artifacts/ux/m3/localized_surface_review_packet.md";
    let audit_artifact = "artifacts/ux/m3/rtl_and_text_expansion_audit.md";
    let documentation_ref = "docs/ux/m3/localization_conformance_beta.md";

    DenseI18nConformanceCorpus {
        record_kind: DENSE_I18N_CORPUS_RECORD_KIND.to_owned(),
        schema_version: DENSE_I18N_CORPUS_SCHEMA_VERSION,
        corpus_id: DENSE_I18N_CORPUS_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        release_channel: "beta".to_owned(),
        fixture_root: fixture_root.to_owned(),
        source_contract_refs: BTreeMap::from([
            (
                "locale_input_readiness".to_owned(),
                "docs/i18n/locale_input_readiness.md".to_owned(),
            ),
            (
                "test_mode_matrix".to_owned(),
                "artifacts/i18n/test_mode_matrix.yaml".to_owned(),
            ),
            (
                "localization_and_locale_pack_contract".to_owned(),
                "docs/ux/localization_and_locale_pack_contract.md".to_owned(),
            ),
            (
                "locale_surface_matrix".to_owned(),
                "docs/i18n/locale_surface_matrix.md".to_owned(),
            ),
            (
                "locale_fallback_copy_representation".to_owned(),
                "docs/accessibility/locale_fallback_and_copy_representation_contract.md".to_owned(),
            ),
        ]),
        runtime_consumer_refs: vec![
            "crates/aureline-shell".to_owned(),
            "crates/aureline-editor".to_owned(),
            "crates/aureline-terminal".to_owned(),
            "crates/aureline-docs".to_owned(),
            "crates/aureline-settings".to_owned(),
            "crates/aureline-support".to_owned(),
        ],
        lane_bindings: vec![
            dense_lane(
                "lane:i18n:dense-beta:nightly",
                DenseI18nLaneCadence::Nightly,
                lane_command,
                fixture_root,
                review_artifact,
                audit_artifact,
            ),
            dense_lane(
                "lane:i18n:dense-beta:release-candidate",
                DenseI18nLaneCadence::ReleaseCandidate,
                lane_command,
                fixture_root,
                review_artifact,
                audit_artifact,
            ),
            dense_lane(
                "lane:i18n:dense-beta:pull-request",
                DenseI18nLaneCadence::PullRequest,
                lane_command,
                fixture_root,
                review_artifact,
                audit_artifact,
            ),
        ],
        surface_cases: seeded_dense_i18n_cases(),
        translated_surface_assertions: seeded_translated_surface_assertions(),
        waivers: Vec::new(),
        review_packet_ref: review_artifact.to_owned(),
        rtl_and_text_expansion_audit_ref: audit_artifact.to_owned(),
        documentation_ref: documentation_ref.to_owned(),
    }
}

/// Returns the seeded dense-surface i18n conformance review packet.
pub fn seeded_dense_i18n_conformance_review_packet() -> DenseI18nConformanceReviewPacket {
    seeded_dense_i18n_conformance_corpus().review_packet()
}

fn dense_lane(
    lane_id: &str,
    cadence: DenseI18nLaneCadence,
    command: &str,
    fixture_root: &str,
    review_artifact: &str,
    audit_artifact: &str,
) -> DenseI18nLaneBinding {
    DenseI18nLaneBinding {
        lane_id: lane_id.to_owned(),
        cadence,
        command: command.to_owned(),
        artifact_refs: vec![
            format!("{fixture_root}/manifest.json"),
            format!("{fixture_root}/review_export.json"),
            review_artifact.to_owned(),
            audit_artifact.to_owned(),
        ],
        release_blocking_for_claimed_surfaces: true,
    }
}

fn seeded_dense_i18n_cases() -> Vec<DenseI18nSurfaceCase> {
    vec![
        dense_case(DenseCaseSeed {
            case_id: "case:i18n:editor:ime-bidi-snippet",
            surface_family: DenseI18nSurfaceFamily::Editor,
            surface_ref: "surface:editor:buffer-and-assist",
            crate_refs: vec!["crates/aureline-editor", "crates/aureline-render"],
            dense_workflow: "Editor rename, completion preview, snippet traversal, and bidi inspector.",
            locale_tags: vec!["ja-JP", "ar-IQ", "qps-ploc"],
            stress_modes: vec![
                DenseI18nStressMode::SourceLanguageBaseline,
                DenseI18nStressMode::PseudolocExpansion,
                DenseI18nStressMode::MixedDirectionTechnicalText,
                DenseI18nStressMode::ImeComposition,
                DenseI18nStressMode::CjkFontFallback,
                DenseI18nStressMode::TextExpansion,
            ],
            readiness_row_refs: vec![
                "readiness.input.ime_preedit_and_commit",
                "readiness.shell.rtl_chrome_and_mixed_direction_technical_content",
                "readiness.text.cjk_font_fallback_and_full_width_layout",
            ],
            fixture_refs: vec![
                "fixtures/accessibility/ime_and_text_cases/editor_ime_dead_key_altgr_emoji_commit.yaml",
                "fixtures/accessibility/ime_and_text_cases/mixed_direction_technical_strings.yaml",
                "fixtures/text/renderer_decision_examples/bidi_ime_composition.md",
            ],
            source_seed_refs: vec![
                "string.cjk.commit",
                "string.rtl.path_in_prose",
                "string.state.read_only_degraded",
            ],
            literal_tokens: vec![
                token("cmd:editor.rename_symbol", "command_id"),
                token(
                    "--output ./\u{062a}\u{0642}\u{0627}\u{0631}\u{064a}\u{0631}/build.log",
                    "file_path",
                ),
                token(
                    "src/\u{062a}\u{062d}\u{0642}\u{0642}/rename.rs",
                    "file_path",
                ),
            ],
            ime_scenarios: vec![ime_scenario(
                "ime:editor:completion-snippet-focus",
                "Japanese IME",
                "\u{30c6}\u{30b9}\u{30c8}\u{5165}\u{529b}",
                "\u{30c6}\u{30b9}\u{30c8}\u{5165}\u{529b}",
                vec![
                    ImeCompositionChurnEvent::FocusChange,
                    ImeCompositionChurnEvent::CompletionPreview,
                    ImeCompositionChurnEvent::SnippetTraversal,
                ],
            )],
            assertions: vec![
                DenseI18nAssertionClass::NoTruncationOrOverflow,
                DenseI18nAssertionClass::LiteralTechnicalTokensPreserved,
                DenseI18nAssertionClass::ImeCompositionNotSilentlyCommittedOrCancelled,
                DenseI18nAssertionClass::CandidateAndCaretRemainVisible,
                DenseI18nAssertionClass::FontFallbackWorks,
            ],
            expected_failure_classes: vec![
                DenseI18nFailureClass::ImePreeditLoss,
                DenseI18nFailureClass::CompletionPreviewSilentCommitOrCancel,
                DenseI18nFailureClass::SnippetTraversalSilentCommitOrCancel,
                DenseI18nFailureClass::BidiCopyDrift,
                DenseI18nFailureClass::MissingGlyphOrWrongFontFallback,
            ],
            rtl_expectation: Some(rtl_expectation()),
        }),
        dense_case(DenseCaseSeed {
            case_id: "case:i18n:palette:rtl-ime-command-preview",
            surface_family: DenseI18nSurfaceFamily::CommandPalette,
            surface_ref: "surface:shell:command-palette",
            crate_refs: vec!["crates/aureline-shell", "crates/aureline-commands"],
            dense_workflow: "Palette query, command preview, disabled reason, and alternate actions.",
            locale_tags: vec!["ar-IQ", "he-IL", "qps-ploc", "ja-JP"],
            stress_modes: vec![
                DenseI18nStressMode::SourceLanguageBaseline,
                DenseI18nStressMode::PseudolocExpansion,
                DenseI18nStressMode::RtlChrome,
                DenseI18nStressMode::MixedDirectionTechnicalText,
                DenseI18nStressMode::ImeComposition,
                DenseI18nStressMode::TextExpansion,
                DenseI18nStressMode::LocaleFallbackReview,
                DenseI18nStressMode::TranslatedSurfaceParity,
            ],
            readiness_row_refs: vec![
                "readiness.shell.source_language_and_pseudoloc_chrome",
                "readiness.shell.rtl_chrome_and_mixed_direction_technical_content",
                "readiness.input.ime_preedit_and_commit",
                "readiness.locale.fallback_chain_and_locale_pack_contract",
            ],
            fixture_refs: vec![
                "fixtures/i18n/pseudoloc_rtl_ime_manifest.yaml",
                "fixtures/i18n/locale_surface_examples/shell_commands_and_palette_localized_label_stable_ids.yaml",
            ],
            source_seed_refs: vec![
                "string.primary.open_workspace",
                "string.state.policy_blocked",
                "string.fallback.open_source_language",
            ],
            literal_tokens: vec![
                token("cmd:core:open_folder", "command_id"),
                token("Ctrl+K Ctrl+O", "keyboard_path"),
                token("--reuse-window", "flag"),
            ],
            ime_scenarios: vec![ime_scenario(
                "ime:palette:command-preview-rerank",
                "Japanese IME",
                "\u{691c}\u{7d22}\u{30c6}\u{30b9}\u{30c8}",
                "\u{691c}\u{7d22}\u{30c6}\u{30b9}\u{30c8}",
                vec![
                    ImeCompositionChurnEvent::FilterRerank,
                    ImeCompositionChurnEvent::CommandPreview,
                    ImeCompositionChurnEvent::CompletionPreview,
                ],
            )],
            assertions: vec![
                DenseI18nAssertionClass::NoTruncationOrOverflow,
                DenseI18nAssertionClass::RtlChromeMirrorsOnlyDirectionalUi,
                DenseI18nAssertionClass::LiteralTechnicalTokensPreserved,
                DenseI18nAssertionClass::ImeCompositionNotSilentlyCommittedOrCancelled,
                DenseI18nAssertionClass::CandidateAndCaretRemainVisible,
                DenseI18nAssertionClass::LocaleFallbackDisclosedAndNonBlocking,
                DenseI18nAssertionClass::StableTranslatedSurfaceRefsPreserved,
                DenseI18nAssertionClass::SourceLanguageEscapeHatchAvailable,
            ],
            expected_failure_classes: vec![
                DenseI18nFailureClass::TextClippedOrOverflow,
                DenseI18nFailureClass::MirroredChromeOrFocusOrderError,
                DenseI18nFailureClass::CommandPreviewSilentCommitOrCancel,
                DenseI18nFailureClass::StableIdDrift,
                DenseI18nFailureClass::SourceLanguageEscapeHatchMissing,
            ],
            rtl_expectation: Some(rtl_expectation()),
        }),
        dense_case(DenseCaseSeed {
            case_id: "case:i18n:settings:fallback-date-number",
            surface_family: DenseI18nSurfaceFamily::Settings,
            surface_ref: "surface:settings:locale-and-sync",
            crate_refs: vec!["crates/aureline-settings", "crates/aureline-i18n"],
            dense_workflow: "Settings locale row, schema-backed help, stale pack fallback, and date or count formatting.",
            locale_tags: vec!["pt-BR", "ja-JP", "qps-ploc"],
            stress_modes: vec![
                DenseI18nStressMode::SourceLanguageBaseline,
                DenseI18nStressMode::PseudolocExpansion,
                DenseI18nStressMode::TextExpansion,
                DenseI18nStressMode::LocaleFallbackReview,
                DenseI18nStressMode::TranslatedSurfaceParity,
                DenseI18nStressMode::LocalizedDateNumberFormatting,
            ],
            readiness_row_refs: vec![
                "readiness.shell.source_language_and_pseudoloc_chrome",
                "readiness.locale.fallback_chain_and_locale_pack_contract",
            ],
            fixture_refs: vec![
                "fixtures/i18n/m3/locale_fallback/settings_projection.json",
                "fixtures/i18n/locale_surface_examples/settings_help_localized_prose_stable_setting_and_schema_ids.yaml",
            ],
            source_seed_refs: vec![
                "string.fallback.locale_chain",
                "string.fallback.open_source_language",
            ],
            literal_tokens: vec![
                token("settings.i18n.active_locale", "setting_id"),
                token("schemas/i18n/locale_pack_manifest.schema.json", "schema_id"),
                token("pt-BR -> pt -> en-US", "fallback_chain"),
            ],
            ime_scenarios: Vec::new(),
            assertions: vec![
                DenseI18nAssertionClass::NoTruncationOrOverflow,
                DenseI18nAssertionClass::LiteralTechnicalTokensPreserved,
                DenseI18nAssertionClass::LocaleFallbackDisclosedAndNonBlocking,
                DenseI18nAssertionClass::StableTranslatedSurfaceRefsPreserved,
                DenseI18nAssertionClass::SourceLanguageEscapeHatchAvailable,
                DenseI18nAssertionClass::LocalizedFormattingKeepsStableSemantics,
            ],
            expected_failure_classes: vec![
                DenseI18nFailureClass::TruncationHidesScopeOrAction,
                DenseI18nFailureClass::LocalePackFallbackBlockedCoreUse,
                DenseI18nFailureClass::LocalizedDateNumberDrift,
                DenseI18nFailureClass::StableIdDrift,
            ],
            rtl_expectation: None,
        }),
        dense_case(DenseCaseSeed {
            case_id: "case:i18n:tree:rtl-pseudoloc-paths",
            surface_family: DenseI18nSurfaceFamily::TreeView,
            surface_ref: "surface:shell:workspace-tree",
            crate_refs: vec!["crates/aureline-shell", "crates/aureline-workspace"],
            dense_workflow: "Workspace tree rows, nested disclosure chrome, path labels, and source-language action.",
            locale_tags: vec!["ar-IQ", "qps-ploc"],
            stress_modes: vec![
                DenseI18nStressMode::SourceLanguageBaseline,
                DenseI18nStressMode::PseudolocExpansion,
                DenseI18nStressMode::RtlChrome,
                DenseI18nStressMode::MixedDirectionTechnicalText,
                DenseI18nStressMode::TextExpansion,
            ],
            readiness_row_refs: vec![
                "readiness.shell.source_language_and_pseudoloc_chrome",
                "readiness.shell.rtl_chrome_and_mixed_direction_technical_content",
            ],
            fixture_refs: vec![
                "fixtures/accessibility/ime_and_text_cases/virtualized_selection_scope.yaml",
                "fixtures/i18n/pseudoloc_rtl_ime_manifest.yaml",
            ],
            source_seed_refs: vec!["string.rtl.path_in_prose", "string.state.read_only_degraded"],
            literal_tokens: vec![
                token(
                    "packages/\u{0645}\u{062b}\u{0627}\u{0644}/src/lib.rs",
                    "file_path",
                ),
                token("cmd:workspace.reveal_in_tree", "command_id"),
            ],
            ime_scenarios: Vec::new(),
            assertions: vec![
                DenseI18nAssertionClass::NoTruncationOrOverflow,
                DenseI18nAssertionClass::RtlChromeMirrorsOnlyDirectionalUi,
                DenseI18nAssertionClass::LiteralTechnicalTokensPreserved,
            ],
            expected_failure_classes: vec![
                DenseI18nFailureClass::MirroredChromeOrFocusOrderError,
                DenseI18nFailureClass::LiteralTechnicalStringMirrored,
                DenseI18nFailureClass::TruncationHidesScopeOrAction,
            ],
            rtl_expectation: Some(rtl_expectation()),
        }),
        dense_case(DenseCaseSeed {
            case_id: "case:i18n:table:cjk-expansion-counts",
            surface_family: DenseI18nSurfaceFamily::TableView,
            surface_ref: "surface:shell:dense-table",
            crate_refs: vec!["crates/aureline-shell", "crates/aureline-ui"],
            dense_workflow: "Dense table headers, selected counts, blocked rows, and CJK/full-width glyph fallback.",
            locale_tags: vec!["ja-JP", "zh-CN", "ko-KR", "qps-ploc"],
            stress_modes: vec![
                DenseI18nStressMode::SourceLanguageBaseline,
                DenseI18nStressMode::PseudolocExpansion,
                DenseI18nStressMode::CjkFontFallback,
                DenseI18nStressMode::TextExpansion,
                DenseI18nStressMode::LocalizedDateNumberFormatting,
            ],
            readiness_row_refs: vec![
                "readiness.shell.source_language_and_pseudoloc_chrome",
                "readiness.text.cjk_font_fallback_and_full_width_layout",
            ],
            fixture_refs: vec![
                "fixtures/accessibility/ime_and_text_cases/virtualized_selection_scope.yaml",
                "fixtures/text/renderer_decision_examples/cjk_latin_interleaved.md",
                "fixtures/text/renderer_decision_examples/missing_glyph_fallback_chain.md",
            ],
            source_seed_refs: vec!["string.cjk.full_width", "string.emoji.status"],
            literal_tokens: vec![
                token("json-key:selected_count", "json_key"),
                token("json-key:blocked_count", "json_key"),
            ],
            ime_scenarios: Vec::new(),
            assertions: vec![
                DenseI18nAssertionClass::NoTruncationOrOverflow,
                DenseI18nAssertionClass::FontFallbackWorks,
                DenseI18nAssertionClass::LocalizedFormattingKeepsStableSemantics,
            ],
            expected_failure_classes: vec![
                DenseI18nFailureClass::TextClippedOrOverflow,
                DenseI18nFailureClass::MissingGlyphOrWrongFontFallback,
                DenseI18nFailureClass::LocalizedDateNumberDrift,
            ],
            rtl_expectation: None,
        }),
        dense_case(DenseCaseSeed {
            case_id: "case:i18n:log:mixed-direction-trace",
            surface_family: DenseI18nSurfaceFamily::LogView,
            surface_ref: "surface:runtime:log-trace-view",
            crate_refs: vec!["crates/aureline-runtime", "crates/aureline-terminal"],
            dense_workflow: "Log stream rows with hostnames, flags, timestamps, and locale-fallback headings.",
            locale_tags: vec!["ar-IQ", "en-US", "qps-ploc"],
            stress_modes: vec![
                DenseI18nStressMode::SourceLanguageBaseline,
                DenseI18nStressMode::PseudolocExpansion,
                DenseI18nStressMode::MixedDirectionTechnicalText,
                DenseI18nStressMode::TextExpansion,
                DenseI18nStressMode::LocaleFallbackReview,
                DenseI18nStressMode::LocalizedDateNumberFormatting,
            ],
            readiness_row_refs: vec![
                "readiness.shell.source_language_and_pseudoloc_chrome",
                "readiness.shell.rtl_chrome_and_mixed_direction_technical_content",
                "readiness.locale.fallback_chain_and_locale_pack_contract",
            ],
            fixture_refs: vec![
                "fixtures/accessibility/ime_and_text_cases/copy_representation_parity.yaml",
                "fixtures/accessibility/ime_and_text_cases/mixed_direction_technical_strings.yaml",
            ],
            source_seed_refs: vec!["string.rtl.host_flag", "string.fallback.locale_chain"],
            literal_tokens: vec![
                token(
                    "api.\u{0645}\u{062b}\u{0627}\u{0644}.internal",
                    "hostname",
                ),
                token("--since 2026-05-18T17:30:00Z", "flag"),
                token("json-key:timestamp", "json_key"),
            ],
            ime_scenarios: Vec::new(),
            assertions: vec![
                DenseI18nAssertionClass::NoTruncationOrOverflow,
                DenseI18nAssertionClass::LiteralTechnicalTokensPreserved,
                DenseI18nAssertionClass::LocaleFallbackDisclosedAndNonBlocking,
                DenseI18nAssertionClass::LocalizedFormattingKeepsStableSemantics,
            ],
            expected_failure_classes: vec![
                DenseI18nFailureClass::BidiCopyDrift,
                DenseI18nFailureClass::LocalizedDateNumberDrift,
                DenseI18nFailureClass::LocalePackFallbackBlockedCoreUse,
            ],
            rtl_expectation: Some(rtl_expectation()),
        }),
        dense_case(DenseCaseSeed {
            case_id: "case:i18n:terminal:ime-raw-escaped-copy",
            surface_family: DenseI18nSurfaceFamily::Terminal,
            surface_ref: "surface:terminal:input-and-transcript",
            crate_refs: vec!["crates/aureline-terminal"],
            dense_workflow: "Terminal input, transcript export, target header, and raw or escaped copy.",
            locale_tags: vec!["ja-JP", "ar-IQ"],
            stress_modes: vec![
                DenseI18nStressMode::SourceLanguageBaseline,
                DenseI18nStressMode::MixedDirectionTechnicalText,
                DenseI18nStressMode::ImeComposition,
                DenseI18nStressMode::CjkFontFallback,
                DenseI18nStressMode::TextExpansion,
            ],
            readiness_row_refs: vec![
                "readiness.input.ime_preedit_and_commit",
                "readiness.shell.rtl_chrome_and_mixed_direction_technical_content",
                "readiness.text.cjk_font_fallback_and_full_width_layout",
            ],
            fixture_refs: vec![
                "fixtures/accessibility/ime_and_text_cases/editor_ime_dead_key_altgr_emoji_commit.yaml",
                "fixtures/accessibility/ime_and_text_cases/copy_representation_parity.yaml",
                "fixtures/terminal/protocol_corpus_alpha/manifest.json",
            ],
            source_seed_refs: vec!["string.cjk.commit", "string.rtl.host_flag"],
            literal_tokens: vec![
                token(
                    "ssh dev@\u{0645}\u{062b}\u{0627}\u{0644}.internal --port 2222",
                    "command_line",
                ),
                token("cmd:terminal.copy_escaped", "command_id"),
            ],
            ime_scenarios: vec![ime_scenario(
                "ime:terminal:overlay-focus",
                "Japanese IME",
                "\u{5165}\u{529b}\u{78ba}\u{8a8d}",
                "\u{5165}\u{529b}\u{78ba}\u{8a8d}",
                vec![
                    ImeCompositionChurnEvent::FocusChange,
                    ImeCompositionChurnEvent::OverlayTransition,
                ],
            )],
            assertions: vec![
                DenseI18nAssertionClass::NoTruncationOrOverflow,
                DenseI18nAssertionClass::LiteralTechnicalTokensPreserved,
                DenseI18nAssertionClass::ImeCompositionNotSilentlyCommittedOrCancelled,
                DenseI18nAssertionClass::CandidateAndCaretRemainVisible,
                DenseI18nAssertionClass::FontFallbackWorks,
            ],
            expected_failure_classes: vec![
                DenseI18nFailureClass::ImePreeditLoss,
                DenseI18nFailureClass::FocusChurnSilentCommitOrCancel,
                DenseI18nFailureClass::BidiCopyDrift,
            ],
            rtl_expectation: Some(rtl_expectation()),
        }),
        dense_case(DenseCaseSeed {
            case_id: "case:i18n:review:bidi-policy-copy",
            surface_family: DenseI18nSurfaceFamily::ReviewPane,
            surface_ref: "surface:review:diff-and-thread",
            crate_refs: vec!["crates/aureline-review", "crates/aureline-preview"],
            dense_workflow: "Diff review with suspicious bidi text, policy copy, and source-language export.",
            locale_tags: vec!["ar-IQ", "pt-BR", "qps-ploc"],
            stress_modes: vec![
                DenseI18nStressMode::SourceLanguageBaseline,
                DenseI18nStressMode::PseudolocExpansion,
                DenseI18nStressMode::RtlChrome,
                DenseI18nStressMode::MixedDirectionTechnicalText,
                DenseI18nStressMode::TextExpansion,
                DenseI18nStressMode::TranslatedSurfaceParity,
            ],
            readiness_row_refs: vec![
                "readiness.shell.source_language_and_pseudoloc_chrome",
                "readiness.shell.rtl_chrome_and_mixed_direction_technical_content",
            ],
            fixture_refs: vec![
                "fixtures/accessibility/ime_and_text_cases/mixed_direction_technical_strings.yaml",
                "fixtures/accessibility/ime_and_text_cases/copy_representation_parity.yaml",
                "fixtures/git/diff_view_alpha/rust_suspicious_safe_copy.yaml",
            ],
            source_seed_refs: vec!["string.state.policy_blocked", "string.rtl.path_in_prose"],
            literal_tokens: vec![
                token("cmd:review.apply_patch", "command_id"),
                token("citation:review:suspicious-bidi", "citation_anchor"),
                token("--allow-dirty", "flag"),
            ],
            ime_scenarios: Vec::new(),
            assertions: vec![
                DenseI18nAssertionClass::NoTruncationOrOverflow,
                DenseI18nAssertionClass::RtlChromeMirrorsOnlyDirectionalUi,
                DenseI18nAssertionClass::LiteralTechnicalTokensPreserved,
                DenseI18nAssertionClass::StableTranslatedSurfaceRefsPreserved,
                DenseI18nAssertionClass::SourceLanguageEscapeHatchAvailable,
            ],
            expected_failure_classes: vec![
                DenseI18nFailureClass::PolicyTrustCopyDrift,
                DenseI18nFailureClass::BidiCopyDrift,
                DenseI18nFailureClass::StableIdDrift,
                DenseI18nFailureClass::SourceLanguageEscapeHatchMissing,
            ],
            rtl_expectation: Some(rtl_expectation()),
        }),
        dense_case(DenseCaseSeed {
            case_id: "case:i18n:guided-tour:locale-fallback-ime",
            surface_family: DenseI18nSurfaceFamily::GuidedTour,
            surface_ref: "surface:help:guided-tour",
            crate_refs: vec!["crates/aureline-docs", "crates/aureline-shell"],
            dense_workflow: "Guided exercise with command tips, progress row, source-language route, and localized entry field.",
            locale_tags: vec!["pt-BR", "ja-JP", "qps-ploc"],
            stress_modes: vec![
                DenseI18nStressMode::SourceLanguageBaseline,
                DenseI18nStressMode::PseudolocExpansion,
                DenseI18nStressMode::ImeComposition,
                DenseI18nStressMode::TextExpansion,
                DenseI18nStressMode::LocaleFallbackReview,
                DenseI18nStressMode::TranslatedSurfaceParity,
            ],
            readiness_row_refs: vec![
                "readiness.shell.source_language_and_pseudoloc_chrome",
                "readiness.input.ime_preedit_and_commit",
                "readiness.locale.fallback_chain_and_locale_pack_contract",
            ],
            fixture_refs: vec![
                "fixtures/help/m3/guided_tours/manifest.json",
                "fixtures/help/m3/guided_tours/source_projection.json",
                "fixtures/i18n/m3/locale_fallback/help_about_projection.json",
            ],
            source_seed_refs: vec![
                "string.fallback.open_source_language",
                "string.primary.open_workspace",
            ],
            literal_tokens: vec![
                token("cmd:core:open_folder", "command_id"),
                token("keyboard:palette.open", "keyboard_path"),
                token("tour:open-folder:first-useful-work", "docs_anchor"),
            ],
            ime_scenarios: vec![ime_scenario(
                "ime:guided-tour:exercise-field",
                "Japanese IME",
                "\u{30d7}\u{30ed}\u{30b8}\u{30a7}\u{30af}\u{30c8}",
                "\u{30d7}\u{30ed}\u{30b8}\u{30a7}\u{30af}\u{30c8}",
                vec![
                    ImeCompositionChurnEvent::FocusChange,
                    ImeCompositionChurnEvent::OverlayTransition,
                ],
            )],
            assertions: vec![
                DenseI18nAssertionClass::NoTruncationOrOverflow,
                DenseI18nAssertionClass::ImeCompositionNotSilentlyCommittedOrCancelled,
                DenseI18nAssertionClass::CandidateAndCaretRemainVisible,
                DenseI18nAssertionClass::LocaleFallbackDisclosedAndNonBlocking,
                DenseI18nAssertionClass::StableTranslatedSurfaceRefsPreserved,
                DenseI18nAssertionClass::SourceLanguageEscapeHatchAvailable,
            ],
            expected_failure_classes: vec![
                DenseI18nFailureClass::TextClippedOrOverflow,
                DenseI18nFailureClass::ImePreeditLoss,
                DenseI18nFailureClass::LocalePackFallbackBlockedCoreUse,
                DenseI18nFailureClass::SourceLanguageEscapeHatchMissing,
            ],
            rtl_expectation: None,
        }),
        dense_case(DenseCaseSeed {
            case_id: "case:i18n:docs-help:citation-rtl-parity",
            surface_family: DenseI18nSurfaceFamily::DocsHelp,
            surface_ref: "surface:docs:help-and-glossary",
            crate_refs: vec!["crates/aureline-docs", "crates/aureline-i18n"],
            dense_workflow: "Docs/help page with citations, glossary term, RTL technical examples, and source-language toggle.",
            locale_tags: vec!["ar-IQ", "pt-BR", "qps-ploc"],
            stress_modes: vec![
                DenseI18nStressMode::SourceLanguageBaseline,
                DenseI18nStressMode::PseudolocExpansion,
                DenseI18nStressMode::RtlChrome,
                DenseI18nStressMode::MixedDirectionTechnicalText,
                DenseI18nStressMode::TextExpansion,
                DenseI18nStressMode::LocaleFallbackReview,
                DenseI18nStressMode::TranslatedSurfaceParity,
            ],
            readiness_row_refs: vec![
                "readiness.shell.source_language_and_pseudoloc_chrome",
                "readiness.shell.rtl_chrome_and_mixed_direction_technical_content",
                "readiness.locale.fallback_chain_and_locale_pack_contract",
            ],
            fixture_refs: vec![
                "fixtures/i18n/locale_surface_examples/docs_tour_localized_text_citation_parity.yaml",
                "fixtures/i18n/m3/locale_fallback/help_about_projection.json",
                "fixtures/accessibility/ime_and_text_cases/mixed_direction_technical_strings.yaml",
            ],
            source_seed_refs: vec!["string.fallback.open_source_language", "string.rtl.path_in_prose"],
            literal_tokens: vec![
                token("citation:docs:onboarding:open-folder", "citation_anchor"),
                token("docs-pack:onboarding:first-useful-work:open-folder", "docs_anchor"),
                token("cmd:core:open_folder", "command_id"),
            ],
            ime_scenarios: Vec::new(),
            assertions: vec![
                DenseI18nAssertionClass::NoTruncationOrOverflow,
                DenseI18nAssertionClass::RtlChromeMirrorsOnlyDirectionalUi,
                DenseI18nAssertionClass::LiteralTechnicalTokensPreserved,
                DenseI18nAssertionClass::LocaleFallbackDisclosedAndNonBlocking,
                DenseI18nAssertionClass::StableTranslatedSurfaceRefsPreserved,
                DenseI18nAssertionClass::SourceLanguageEscapeHatchAvailable,
            ],
            expected_failure_classes: vec![
                DenseI18nFailureClass::StableIdDrift,
                DenseI18nFailureClass::SourceLanguageEscapeHatchMissing,
                DenseI18nFailureClass::LiteralTechnicalStringMirrored,
                DenseI18nFailureClass::TextClippedOrOverflow,
            ],
            rtl_expectation: Some(rtl_expectation()),
        }),
    ]
}

fn seeded_translated_surface_assertions() -> Vec<TranslatedSurfaceAssertion> {
    vec![
        translated_assertion(
            "assert:i18n:command-palette:stable-command-keybinding",
            DenseI18nSurfaceFamily::CommandPalette,
            vec![
                preserved("command_id", "cmd:core:open_folder"),
                preserved("keyboard_path", "Ctrl+K Ctrl+O"),
                preserved("scope_label", "workspace"),
            ],
        ),
        translated_assertion(
            "assert:i18n:docs-help:citation-source",
            DenseI18nSurfaceFamily::DocsHelp,
            vec![
                preserved("citation_anchor", "citation:docs:onboarding:open-folder"),
                preserved("command_id", "cmd:core:open_folder"),
                preserved("scope_label", "docs_pack"),
            ],
        ),
        translated_assertion(
            "assert:i18n:settings:schema-policy",
            DenseI18nSurfaceFamily::Settings,
            vec![
                preserved("schema_id", "schemas/i18n/locale_pack_manifest.schema.json"),
                preserved(
                    "policy_id",
                    "policy.locale.source_language_fallback_required",
                ),
                preserved("scope_label", "user_profile"),
            ],
        ),
        translated_assertion(
            "assert:i18n:review:policy-trust-copy",
            DenseI18nSurfaceFamily::ReviewPane,
            vec![
                preserved("command_id", "cmd:review.apply_patch"),
                preserved("citation_anchor", "citation:review:suspicious-bidi"),
                preserved("scope_label", "review_thread"),
            ],
        ),
    ]
}

fn translated_assertion(
    assertion_id: &str,
    surface_family: DenseI18nSurfaceFamily,
    preserved_refs: Vec<PreservedTranslatedSurfaceRef>,
) -> TranslatedSurfaceAssertion {
    TranslatedSurfaceAssertion {
        assertion_id: assertion_id.to_owned(),
        surface_family,
        preserved_refs,
        open_in_source_language_required: true,
        machine_output_locale_neutral: true,
        governed_terminology_preserved: true,
    }
}

fn preserved(ref_kind: &str, value: &str) -> PreservedTranslatedSurfaceRef {
    PreservedTranslatedSurfaceRef {
        ref_kind: ref_kind.to_owned(),
        value: value.to_owned(),
    }
}

fn token(token: &str, token_class: &str) -> LiteralTechnicalToken {
    LiteralTechnicalToken {
        token: token.to_owned(),
        token_class: token_class.to_owned(),
        must_remain_unmirrored: true,
        copy_raw_required: true,
        copy_escaped_required: true,
    }
}

fn ime_scenario(
    scenario_id: &str,
    input_method: &str,
    preedit_text: &str,
    expected_commit_text: &str,
    churn_events: Vec<ImeCompositionChurnEvent>,
) -> ImeCompositionScenario {
    ImeCompositionScenario {
        scenario_id: scenario_id.to_owned(),
        input_method: input_method.to_owned(),
        preedit_text: preedit_text.to_owned(),
        expected_commit_text: expected_commit_text.to_owned(),
        churn_events,
        silent_commit_forbidden: true,
        silent_cancel_forbidden: true,
        candidate_and_caret_visibility_required: true,
    }
}

fn expansion_budget() -> TextExpansionBudget {
    TextExpansionBudget {
        single_line_min_ratio: 1.35,
        multiline_min_ratio: 1.60,
        overflow_forbidden: true,
        full_text_route_required: true,
    }
}

fn rtl_expectation() -> RtlMirroringExpectation {
    RtlMirroringExpectation {
        directional_chrome_mirrors: true,
        literal_technical_strings_unmirrored: true,
        focus_order_tracks_visual_order: true,
        raw_copy_preserves_author_order: true,
    }
}

struct DenseCaseSeed<'a> {
    case_id: &'a str,
    surface_family: DenseI18nSurfaceFamily,
    surface_ref: &'a str,
    crate_refs: Vec<&'a str>,
    dense_workflow: &'a str,
    locale_tags: Vec<&'a str>,
    stress_modes: Vec<DenseI18nStressMode>,
    readiness_row_refs: Vec<&'a str>,
    fixture_refs: Vec<&'a str>,
    source_seed_refs: Vec<&'a str>,
    literal_tokens: Vec<LiteralTechnicalToken>,
    ime_scenarios: Vec<ImeCompositionScenario>,
    assertions: Vec<DenseI18nAssertionClass>,
    expected_failure_classes: Vec<DenseI18nFailureClass>,
    rtl_expectation: Option<RtlMirroringExpectation>,
}

fn dense_case(seed: DenseCaseSeed<'_>) -> DenseI18nSurfaceCase {
    DenseI18nSurfaceCase {
        case_id: seed.case_id.to_owned(),
        surface_family: seed.surface_family,
        surface_ref: seed.surface_ref.to_owned(),
        crate_refs: seed.crate_refs.into_iter().map(str::to_owned).collect(),
        dense_workflow: seed.dense_workflow.to_owned(),
        locale_tags: seed.locale_tags.into_iter().map(str::to_owned).collect(),
        stress_modes: seed.stress_modes,
        readiness_row_refs: seed
            .readiness_row_refs
            .into_iter()
            .map(str::to_owned)
            .collect(),
        fixture_refs: seed.fixture_refs.into_iter().map(str::to_owned).collect(),
        source_seed_refs: seed
            .source_seed_refs
            .into_iter()
            .map(str::to_owned)
            .collect(),
        literal_tokens: seed.literal_tokens,
        ime_scenarios: seed.ime_scenarios,
        expansion_budget: expansion_budget(),
        rtl_expectation: seed.rtl_expectation,
        assertion_refs: seed.assertions,
        expected_failure_classes: seed.expected_failure_classes,
        bounded_waiver_ref: None,
    }
}

fn policy_context() -> PolicyContext {
    PolicyContext {
        policy_epoch: POLICY_EPOCH.to_owned(),
        trust_state: "trusted".to_owned(),
        execution_context_id: Some("execution-context:local-desktop:locale-beta".to_owned()),
    }
}

fn all_surface_families() -> Vec<MessageSurfaceFamily> {
    vec![
        MessageSurfaceFamily::ShellChrome,
        MessageSurfaceFamily::CommandLabel,
        MessageSurfaceFamily::SettingsHelpOrError,
        MessageSurfaceFamily::DocsTourOrAuthText,
        MessageSurfaceFamily::ExtensionContributedUi,
        MessageSurfaceFamily::CliHelpText,
        MessageSurfaceFamily::ExportOrReportHeading,
        MessageSurfaceFamily::ScreenshotOrDemoCaption,
        MessageSurfaceFamily::GlossaryOrTerminologyTerm,
        MessageSurfaceFamily::PolicyLegalOrRecoveryText,
    ]
}

fn all_deployment_profiles() -> Vec<&'static str> {
    vec![
        "individual_local",
        "self_hosted",
        "enterprise_online",
        "air_gapped",
        "managed_cloud",
    ]
}

struct LocalePackSeed<'a> {
    pack_id: &'a str,
    pack_revision_ref: &'a str,
    locale: &'a str,
    coverage_locales: Vec<&'a str>,
    fallback_chain: Vec<&'a str>,
    source_class: LocalePackSourceClass,
    distribution_class: LocalePackDistributionClass,
    signature_state: LocalePackSignatureState,
    mirrorability_class: LocalePackMirrorabilityClass,
    compatibility_class: VersionMatchState,
    covered_surface_families: Vec<MessageSurfaceFamily>,
    partially_translated_surface_families: Vec<MessageSurfaceFamily>,
    extension_overlay_pack_refs: Vec<String>,
    extension_namespace_refs: Vec<String>,
    permitted_deployment_profiles: Vec<&'a str>,
    source_artifact_ref: String,
    signer_identity_ref: Option<String>,
    signature_artifact_ref: Option<String>,
    mirror_receipt_refs: Vec<String>,
    offline_import_ref: Option<String>,
    rollback_ref: String,
    presentation_label: String,
    policy_context: PolicyContext,
}

fn locale_pack(seed: LocalePackSeed<'_>) -> LocalePackManifestRecord {
    LocalePackManifestRecord {
        record_kind: "locale_pack_manifest_record".to_owned(),
        schema_version: LOCALE_PACK_BETA_SCHEMA_VERSION,
        pack_id: seed.pack_id.to_owned(),
        pack_revision_ref: seed.pack_revision_ref.to_owned(),
        locale: seed.locale.to_owned(),
        coverage_locales: seed
            .coverage_locales
            .into_iter()
            .map(str::to_owned)
            .collect(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        base_locale_fallback_chain: seed.fallback_chain.into_iter().map(str::to_owned).collect(),
        source_class: seed.source_class,
        distribution_class: seed.distribution_class,
        signature_state: seed.signature_state,
        mirrorability_class: seed.mirrorability_class,
        compatibility_class: seed.compatibility_class,
        compatibility_build_range: CompatibilityBuildRange {
            min_build_identity_ref: "build:aureline:0.0.0-beta.2026.05.01".to_owned(),
            max_build_identity_ref: TARGET_BUILD.to_owned(),
        },
        covered_surface_families: seed.covered_surface_families,
        partially_translated_surface_families: seed.partially_translated_surface_families,
        extension_overlay_pack_refs: seed.extension_overlay_pack_refs,
        extension_namespace_refs: seed.extension_namespace_refs,
        permitted_deployment_profiles: seed
            .permitted_deployment_profiles
            .into_iter()
            .map(str::to_owned)
            .collect(),
        explicit_acceptance_decision_row_ref: None,
        source_artifact_ref: seed.source_artifact_ref,
        signer_identity_ref: seed.signer_identity_ref,
        signature_artifact_ref: seed.signature_artifact_ref,
        mirror_receipt_refs: seed.mirror_receipt_refs,
        offline_import_ref: seed.offline_import_ref,
        rollback_ref: seed.rollback_ref,
        policy_context: seed.policy_context,
        redaction_class: "metadata_safe_default".to_owned(),
        presentation_label: seed.presentation_label,
        minted_at: GENERATED_AT.to_owned(),
    }
}

struct MessageSeed<'a> {
    message_id: &'a str,
    surface_family: MessageSurfaceFamily,
    source_text: &'a str,
    stable_refs: StableMessageIdentityRefs,
    placeholders: Vec<MessagePlaceholder>,
    machine_output_locale_class: MachineOutputLocaleClass,
    source_language_escape_hatches: Vec<SourceLanguageEscapeHatchClass>,
    translation_review_refs: Vec<String>,
    extension_namespace_ref: Option<String>,
}

fn message_binding(seed: MessageSeed<'_>) -> MessageCatalogBindingRecord {
    MessageCatalogBindingRecord {
        record_kind: "message_catalog_binding_record".to_owned(),
        schema_version: LOCALE_PACK_BETA_SCHEMA_VERSION,
        message_id: seed.message_id.to_owned(),
        message_id_class: if seed.extension_namespace_ref.is_some() {
            MessageIdClass::ExtensionOverlay
        } else {
            MessageIdClass::StableCanonical
        },
        surface_family: seed.surface_family,
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        source_text: seed.source_text.to_owned(),
        stable_refs: seed.stable_refs,
        placeholders: seed.placeholders,
        machine_output_locale_class: seed.machine_output_locale_class,
        source_language_escape_hatches: seed.source_language_escape_hatches,
        localized_human_prose_allowed: true,
        machine_identifier_fields_locale_neutral: true,
        routed_by_localized_prose: false,
        extension_namespace_ref: seed.extension_namespace_ref,
        translation_review_refs: seed.translation_review_refs,
        minted_at: GENERATED_AT.to_owned(),
    }
}

struct FallbackSeed<'a> {
    state_id: &'a str,
    requested_locale: &'a str,
    effective_locale: &'a str,
    fallback_origin_class: LocaleFallbackOriginClass,
    degraded_localization_state: DegradedLocalizationState,
    fallback_chain_walked: Vec<&'a str>,
    packs_consulted: Vec<PackConsultationDescriptor>,
    message_id_ref: Option<String>,
    command_id_ref: Option<String>,
    disclosed_to_reviewer: bool,
    source_language_escape_hatches_active: Vec<SourceLanguageEscapeHatchClass>,
    surface_family: MessageSurfaceFamily,
    command_id_preservation_state: CommandIdPreservationState,
    machine_output_locale_class: MachineOutputLocaleClass,
    denial_reason_on_deny: Option<String>,
    presentation_label: &'a str,
    presentation_subtitle: Option<String>,
}

fn fallback_state(seed: FallbackSeed<'_>) -> LocaleFallbackStateRecord {
    LocaleFallbackStateRecord {
        record_kind: "locale_fallback_state_record".to_owned(),
        schema_version: LOCALE_PACK_BETA_SCHEMA_VERSION,
        state_id: seed.state_id.to_owned(),
        requested_locale: seed.requested_locale.to_owned(),
        effective_locale: seed.effective_locale.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        fallback_origin_class: seed.fallback_origin_class,
        degraded_localization_state: seed.degraded_localization_state,
        fallback_chain_walked: seed
            .fallback_chain_walked
            .into_iter()
            .map(str::to_owned)
            .collect(),
        packs_consulted: seed.packs_consulted,
        message_id_ref: seed.message_id_ref,
        command_id_ref: seed.command_id_ref,
        disclosed_to_reviewer: seed.disclosed_to_reviewer,
        source_language_escape_hatches_active: seed.source_language_escape_hatches_active,
        surface_family: seed.surface_family,
        command_id_preservation_state: seed.command_id_preservation_state,
        machine_output_locale_class: seed.machine_output_locale_class,
        deployment_profile_refs: all_deployment_profiles()
            .into_iter()
            .map(str::to_owned)
            .collect(),
        denial_reason_on_deny: seed.denial_reason_on_deny,
        policy_context: policy_context(),
        redaction_class: "metadata_safe_default".to_owned(),
        target_build_identity_ref: TARGET_BUILD.to_owned(),
        presentation_label: seed.presentation_label.to_owned(),
        presentation_subtitle: seed.presentation_subtitle,
        minted_at: GENERATED_AT.to_owned(),
    }
}

fn governance_action(
    action_id: &str,
    operation_class: LocalePackOperationClass,
    pack_ref: &str,
    rollback_ref: Option<&str>,
) -> LocalePackGovernanceAction {
    LocalePackGovernanceAction {
        action_id: action_id.to_owned(),
        operation_class,
        pack_ref: pack_ref.to_owned(),
        review_required: true,
        signature_verification_required: true,
        compatibility_check_required: true,
        mirror_metadata_preserved: matches!(
            operation_class,
            LocalePackOperationClass::MirrorImport | LocalePackOperationClass::OfflineImport
        ),
        rollback_ref: rollback_ref.map(str::to_owned),
        support_export_ref: format!("support-export:locale-pack:action:{action_id}"),
    }
}

fn compatibility_result(
    result_id: &str,
    pack_ref: &str,
    outcome: LocaleCompatibilityOutcome,
    signature_state: LocalePackSignatureState,
    compatibility_class: VersionMatchState,
    fallback_state_refs: Vec<&str>,
    waiver_ref: Option<&str>,
) -> LocalePackCompatibilityResult {
    LocalePackCompatibilityResult {
        result_id: result_id.to_owned(),
        pack_ref: pack_ref.to_owned(),
        target_build_identity_ref: TARGET_BUILD.to_owned(),
        outcome,
        signature_state,
        compatibility_class,
        surface_families_checked: all_surface_families(),
        fallback_state_refs: fallback_state_refs.into_iter().map(str::to_owned).collect(),
        waiver_ref: waiver_ref.map(str::to_owned),
    }
}

fn stable_refs_as_vec(stable_refs: &StableMessageIdentityRefs) -> Vec<String> {
    let mut refs = Vec::new();
    if let Some(value) = &stable_refs.command_id_ref {
        refs.push(value.clone());
    }
    if let Some(value) = &stable_refs.semantic_action_id_ref {
        refs.push(value.clone());
    }
    if let Some(value) = &stable_refs.diagnostic_id_ref {
        refs.push(value.clone());
    }
    if let Some(value) = &stable_refs.docs_pack_key_ref {
        refs.push(value.clone());
    }
    if let Some(value) = &stable_refs.setting_id_ref {
        refs.push(value.clone());
    }
    if let Some(value) = &stable_refs.telemetry_key_ref {
        refs.push(value.clone());
    }
    if let Some(value) = &stable_refs.policy_name_ref {
        refs.push(value.clone());
    }
    if let Some(value) = &stable_refs.schema_id_ref {
        refs.push(value.clone());
    }
    refs
}

fn validate_pack(pack: &LocalePackManifestRecord, findings: &mut Vec<LocalePackValidationFinding>) {
    if pack.record_kind != "locale_pack_manifest_record" {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "pack record_kind is unsupported",
        ));
    }
    if pack.schema_version != LOCALE_PACK_BETA_SCHEMA_VERSION {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "pack schema_version is unsupported",
        ));
    }
    if !pack
        .coverage_locales
        .iter()
        .any(|locale| locale == &pack.locale)
    {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "coverage_locales must include locale",
        ));
    }
    if pack.base_locale_fallback_chain.first() != Some(&pack.locale) {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "fallback chain must begin at pack locale",
        ));
    }
    if pack.base_locale_fallback_chain.last() != Some(&pack.source_language_locale) {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "fallback chain must end at source language",
        ));
    }
    if pack.covered_surface_families.is_empty() {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "pack must cover at least one surface family",
        ));
    }
    for family in &pack.partially_translated_surface_families {
        if !pack.covered_surface_families.contains(family) {
            findings.push(LocalePackValidationFinding::new(
                pack.pack_id.clone(),
                "partial surface must be covered by the pack",
            ));
        }
    }
    if pack.distribution_class == LocalePackDistributionClass::BuiltInWithProduct
        && pack.signature_state != LocalePackSignatureState::NotApplicableBuiltIn
    {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "built-in pack must use not_applicable_built_in signature state",
        ));
    }
    if pack.distribution_class != LocalePackDistributionClass::BuiltInWithProduct
        && pack.signature_state == LocalePackSignatureState::NotApplicableBuiltIn
    {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "external pack must carry a signature state",
        ));
    }
    if pack.distribution_class == LocalePackDistributionClass::ExtensionOverlayPack
        && pack.extension_namespace_refs.is_empty()
    {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "extension overlay pack must declare extension namespace refs",
        ));
    }
    if pack.distribution_class != LocalePackDistributionClass::ExtensionOverlayPack
        && !pack.extension_namespace_refs.is_empty()
    {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "non-extension pack must not declare extension namespace refs",
        ));
    }
    if pack.signature_state != LocalePackSignatureState::NotApplicableBuiltIn
        && pack.signature_artifact_ref.is_none()
    {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "signed pack must carry signature artifact ref",
        ));
    }
    if matches!(
        pack.distribution_class,
        LocalePackDistributionClass::MirroredOfficialPack
            | LocalePackDistributionClass::CommunitySuppliedPack
            | LocalePackDistributionClass::AirGappedOfflinePack
    ) && pack.mirror_receipt_refs.is_empty()
    {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "mirrorable or offline pack must preserve mirror receipt refs",
        ));
    }
}

fn validate_message(
    message: &MessageCatalogBindingRecord,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    if message.record_kind != "message_catalog_binding_record" {
        findings.push(LocalePackValidationFinding::new(
            message.message_id.clone(),
            "message record_kind is unsupported",
        ));
    }
    if !message.stable_refs.has_anchor() {
        findings.push(LocalePackValidationFinding::new(
            message.message_id.clone(),
            "message must bind to a stable non-prose identity",
        ));
    }
    if message.surface_family == MessageSurfaceFamily::CommandLabel
        && message.stable_refs.command_id_ref.is_none()
    {
        findings.push(LocalePackValidationFinding::new(
            message.message_id.clone(),
            "command-label message must bind to command id",
        ));
    }
    if message.surface_family == MessageSurfaceFamily::CommandLabel
        && message.stable_refs.semantic_action_id_ref.is_none()
    {
        findings.push(LocalePackValidationFinding::new(
            message.message_id.clone(),
            "command-label message must bind to semantic action id",
        ));
    }
    if !message.machine_identifier_fields_locale_neutral {
        findings.push(LocalePackValidationFinding::new(
            message.message_id.clone(),
            "machine identifiers must stay locale-neutral",
        ));
    }
    if message.routed_by_localized_prose {
        findings.push(LocalePackValidationFinding::new(
            message.message_id.clone(),
            "message must not route behavior by localized prose",
        ));
    }
    if matches!(
        message.machine_output_locale_class,
        MachineOutputLocaleClass::LocaleNeutralCanonical
            | MachineOutputLocaleClass::LocaleNeutralWithTranslatedHumanField
    ) && !message.machine_identifier_fields_locale_neutral
    {
        findings.push(LocalePackValidationFinding::new(
            message.message_id.clone(),
            "locale-neutral machine output cannot depend on translated identifiers",
        ));
    }
    if message.message_id_class == MessageIdClass::ExtensionOverlay
        && message.extension_namespace_ref.is_none()
    {
        findings.push(LocalePackValidationFinding::new(
            message.message_id.clone(),
            "extension overlay message must declare extension namespace",
        ));
    }
}

fn validate_fallback_state(
    state: &LocaleFallbackStateRecord,
    message_ids: &BTreeSet<&str>,
    pack_ids: &BTreeSet<&str>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    if state.record_kind != "locale_fallback_state_record" {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "fallback state record_kind is unsupported",
        ));
    }
    if state.fallback_chain_walked.first() != Some(&state.requested_locale) {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "fallback chain must begin at requested locale",
        ));
    }
    if state.fallback_chain_walked.last() != Some(&state.effective_locale) {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "fallback chain must end at effective locale",
        ));
    }
    if state.fallback_origin_class.requires_disclosure() && !state.disclosed_to_reviewer {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "non-authoritative fallback must disclose to reviewer",
        ));
    }
    if state.fallback_origin_class.requires_disclosure()
        && state.source_language_escape_hatches_active.is_empty()
    {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "fallback must keep a source-language escape hatch",
        ));
    }
    if state.surface_family == MessageSurfaceFamily::CommandLabel
        && state.command_id_preservation_state
            != CommandIdPreservationState::CommandIdUnchangedAcrossFallback
    {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "command fallback must preserve command id",
        ));
    }
    if state.surface_family == MessageSurfaceFamily::CommandLabel && state.command_id_ref.is_none()
    {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "command fallback must carry command id ref",
        ));
    }
    if state.surface_family == MessageSurfaceFamily::CliHelpText
        && !state
            .source_language_escape_hatches_active
            .contains(&SourceLanguageEscapeHatchClass::CliLocaleNeutralOutputFlag)
    {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "CLI fallback must advertise locale-neutral output flag",
        ));
    }
    if let Some(message_id) = state.message_id_ref.as_deref() {
        if !message_ids.contains(message_id) {
            findings.push(LocalePackValidationFinding::new(
                state.state_id.clone(),
                "fallback references an unknown message",
            ));
        }
    }
    for pack in &state.packs_consulted {
        if !pack_ids.contains(pack.pack_ref.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                state.state_id.clone(),
                "fallback references an unknown pack",
            ));
        }
        if pack.signature_state == LocalePackSignatureState::SignatureFailedBlocked
            && pack.produced_message
        {
            findings.push(LocalePackValidationFinding::new(
                state.state_id.clone(),
                "signature-failed pack must not produce a message",
            ));
        }
    }
    if state.fallback_origin_class
        == LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly
        && state.denial_reason_on_deny.as_deref()
            != Some("localization_locale_pack_signature_failed")
    {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "signature failure fallback must carry denial reason",
        ));
    }
}

fn validate_active_locale_state(
    state: &ActiveLocaleState,
    pack_ids: &BTreeSet<&str>,
    fallback_ids: &BTreeSet<&str>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    if state.fallback_chain.first() != Some(&state.requested_locale) {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "active fallback chain must begin at requested locale",
        ));
    }
    if state.fallback_chain.last() != Some(&state.source_language_locale) {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "active fallback chain must end at source language",
        ));
    }
    for pack_ref in &state.active_pack_refs {
        if !pack_ids.contains(pack_ref.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                state.state_id.clone(),
                "active locale state references an unknown pack",
            ));
        }
    }
    for fallback_ref in &state.active_fallback_state_refs {
        if !fallback_ids.contains(fallback_ref.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                state.state_id.clone(),
                "active locale state references an unknown fallback state",
            ));
        }
    }
    if state.settings_projection_ref.trim().is_empty()
        || state.help_about_projection_ref.trim().is_empty()
        || state.support_export_ref.trim().is_empty()
    {
        findings.push(LocalePackValidationFinding::new(
            state.state_id.clone(),
            "active locale state must expose settings, Help/About, and support refs",
        ));
    }
}

fn validate_extension_declaration(
    declaration: &ExtensionLocaleDeclaration,
    pack_ids: &BTreeSet<&str>,
    result_ids: &BTreeSet<&str>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    if declaration.may_override_host_stable_ids {
        findings.push(LocalePackValidationFinding::new(
            declaration.extension_id.clone(),
            "extension locale declaration must not override host stable ids",
        ));
    }
    if matches!(
        declaration.support_mode,
        ExtensionLocaleSupportMode::ShipsOwnLocalePack
            | ExtensionLocaleSupportMode::ShipsCompanionLocalePack
    ) && declaration.locale_pack_ref.is_none()
    {
        findings.push(LocalePackValidationFinding::new(
            declaration.extension_id.clone(),
            "extension locale declaration must name its locale pack",
        ));
    }
    if let Some(pack_ref) = declaration.locale_pack_ref.as_deref() {
        if !pack_ids.contains(pack_ref) {
            findings.push(LocalePackValidationFinding::new(
                declaration.extension_id.clone(),
                "extension locale declaration references an unknown pack",
            ));
        }
    }
    if !result_ids.contains(declaration.compatibility_result_ref.as_str()) {
        let source_only_ok = matches!(
            declaration.support_mode,
            ExtensionLocaleSupportMode::InheritsHostLocale
                | ExtensionLocaleSupportMode::SourceLanguageOnly
        );
        if !source_only_ok {
            findings.push(LocalePackValidationFinding::new(
                declaration.extension_id.clone(),
                "extension locale declaration references an unknown compatibility result",
            ));
        }
    }
    if !declaration.visible_disclosure_required {
        findings.push(LocalePackValidationFinding::new(
            declaration.extension_id.clone(),
            "extension locale declaration must be visible",
        ));
    }
}

fn validate_governance_action(
    action: &LocalePackGovernanceAction,
    pack_ids: &BTreeSet<&str>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    if !pack_ids.contains(action.pack_ref.as_str()) {
        findings.push(LocalePackValidationFinding::new(
            action.action_id.clone(),
            "governance action references an unknown pack",
        ));
    }
    if !action.review_required
        || !action.signature_verification_required
        || !action.compatibility_check_required
    {
        findings.push(LocalePackValidationFinding::new(
            action.action_id.clone(),
            "governance action must require review, signature check, and compatibility check",
        ));
    }
    if matches!(
        action.operation_class,
        LocalePackOperationClass::MirrorImport | LocalePackOperationClass::OfflineImport
    ) && !action.mirror_metadata_preserved
    {
        findings.push(LocalePackValidationFinding::new(
            action.action_id.clone(),
            "mirror/offline import must preserve mirror metadata",
        ));
    }
    if matches!(
        action.operation_class,
        LocalePackOperationClass::Update | LocalePackOperationClass::Rollback
    ) && action.rollback_ref.is_none()
    {
        findings.push(LocalePackValidationFinding::new(
            action.action_id.clone(),
            "update and rollback actions must cite rollback ref",
        ));
    }
}

fn validate_compatibility_result(
    result: &LocalePackCompatibilityResult,
    pack_ids: &BTreeSet<&str>,
    fallback_ids: &BTreeSet<&str>,
    waiver_ids: &BTreeSet<&str>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    if !pack_ids.contains(result.pack_ref.as_str()) {
        findings.push(LocalePackValidationFinding::new(
            result.result_id.clone(),
            "compatibility result references an unknown pack",
        ));
    }
    for fallback_ref in &result.fallback_state_refs {
        if !fallback_ids.contains(fallback_ref.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                result.result_id.clone(),
                "compatibility result references an unknown fallback state",
            ));
        }
    }
    if result.outcome == LocaleCompatibilityOutcome::CompatibleWithWaiver {
        let Some(waiver_ref) = result.waiver_ref.as_deref() else {
            findings.push(LocalePackValidationFinding::new(
                result.result_id.clone(),
                "waived compatibility result must cite waiver ref",
            ));
            return;
        };
        if !waiver_ids.contains(waiver_ref) {
            findings.push(LocalePackValidationFinding::new(
                result.result_id.clone(),
                "compatibility result references an unknown waiver",
            ));
        }
    }
    if matches!(
        result.outcome,
        LocaleCompatibilityOutcome::BlockedSignatureFailure
            | LocaleCompatibilityOutcome::BlockedVersionDrift
            | LocaleCompatibilityOutcome::SourceLanguageOnlyFallback
    ) && result.fallback_state_refs.is_empty()
    {
        findings.push(LocalePackValidationFinding::new(
            result.result_id.clone(),
            "blocked compatibility result must cite fallback state",
        ));
    }
}
