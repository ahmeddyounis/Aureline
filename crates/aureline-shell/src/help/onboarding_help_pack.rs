//! Onboarding/help-pack alpha manifest and validation.
//!
//! This module loads the checked-in onboarding/help/glossary pack substrate
//! and validates that pack items preserve stable IDs, command metadata,
//! locale fallback, offline posture, citations, exact reopen refs, and
//! portable user-owned progress state before Start Center or help search
//! projects them.

use std::collections::{BTreeMap, BTreeSet};

use aureline_commands::CommandRegistry;
use serde::{Deserialize, Serialize};

/// Stable path to the checked-in onboarding/help-pack alpha manifest.
pub const CURRENT_ONBOARDING_HELP_PACK_ALPHA_PATH: &str =
    "artifacts/docs/onboarding_help_pack_alpha.yaml";

const CURRENT_ONBOARDING_HELP_PACK_ALPHA_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/docs/onboarding_help_pack_alpha.yaml"
));

/// Current schema version for [`OnboardingHelpPackAlphaManifest`].
pub const ONBOARDING_HELP_PACK_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Current record kind for [`OnboardingHelpPackAlphaManifest`].
pub const ONBOARDING_HELP_PACK_ALPHA_RECORD_KIND: &str = "onboarding_help_pack_alpha_record";

/// Governed alpha substrate for onboarding, help, glossary, and future learning items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackAlphaManifest {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Projection timestamp.
    pub generated_at: String,
    /// Source contracts that own the vocabulary used by this manifest.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime/support consumers that read this manifest directly.
    pub runtime_consumer_refs: Vec<String>,
    /// Pack descriptors available to item rows.
    pub packs: Vec<OnboardingHelpPackAlphaPack>,
    /// Stable item rows consumed by Start Center, help search, glossary, and support export.
    pub items: Vec<OnboardingHelpPackAlphaItem>,
    /// Portable progress, dismissal, bookmark, and resume state descriptors.
    pub progress_states: Vec<OnboardingHelpPackProgressState>,
    /// Protected proof fixtures for this manifest.
    pub protected_proofs: Vec<OnboardingHelpPackProof>,
}

impl OnboardingHelpPackAlphaManifest {
    /// Returns the pack descriptor for `pack_id`.
    pub fn pack(&self, pack_id: &str) -> Option<&OnboardingHelpPackAlphaPack> {
        self.packs.iter().find(|pack| pack.pack_id == pack_id)
    }

    /// Returns the item descriptor for `item_id`.
    pub fn item(&self, item_id: &str) -> Option<&OnboardingHelpPackAlphaItem> {
        self.items.iter().find(|item| item.item_id == item_id)
    }

    /// Returns the progress descriptor for `state_item_id`.
    pub fn progress_state(&self, state_item_id: &str) -> Option<&OnboardingHelpPackProgressState> {
        self.progress_states
            .iter()
            .find(|state| state.state_item_id == state_item_id)
    }

    /// Returns items that may render in onboarding or help search.
    pub fn renderable_items(&self) -> Vec<&OnboardingHelpPackAlphaItem> {
        self.items
            .iter()
            .filter(|item| item.content_render_state == OnboardingHelpPackRenderState::Renderable)
            .collect()
    }

    /// Returns export-safe support reconstruction rows for every item.
    pub fn support_export_rows(&self) -> Vec<OnboardingHelpPackSupportExportRow> {
        self.items
            .iter()
            .map(|item| OnboardingHelpPackSupportExportRow {
                item_id: item.item_id.clone(),
                pack_id: item.pack_id.clone(),
                pack_revision_ref: item.pack_revision_ref.clone(),
                command_id: item.command_hint.command_id.clone(),
                help_anchor_id: item.command_hint.help_anchor_id.clone(),
                requested_locale: item.locale.requested_locale.clone(),
                effective_locale: item.locale.effective_locale.clone(),
                fallback_class: item
                    .source_language_fallback
                    .fallback_class
                    .as_str()
                    .to_owned(),
                citation_refs: item.citation.citation_refs.clone(),
                exact_reopen_ref: item.exact_reopen_ref.clone(),
                support_pack_item_id: item.support_export_identity.support_pack_item_id.clone(),
                raw_body_exported: item.support_export_identity.raw_body_exported,
            })
            .collect()
    }

    /// Validates structural and command-registry invariants.
    pub fn validate_against_registry(
        &self,
        registry: &CommandRegistry,
    ) -> Result<(), Vec<OnboardingHelpPackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != ONBOARDING_HELP_PACK_ALPHA_RECORD_KIND {
            findings.push(OnboardingHelpPackValidationFinding::new(
                self.manifest_id.clone(),
                "manifest record_kind is unsupported",
            ));
        }
        if self.schema_version != ONBOARDING_HELP_PACK_ALPHA_SCHEMA_VERSION {
            findings.push(OnboardingHelpPackValidationFinding::new(
                self.manifest_id.clone(),
                "manifest schema version is unsupported",
            ));
        }
        if self.runtime_consumer_refs.is_empty() {
            findings.push(OnboardingHelpPackValidationFinding::new(
                self.manifest_id.clone(),
                "manifest has no runtime consumers",
            ));
        }

        let mut pack_ids = BTreeSet::new();
        let mut pack_revisions = BTreeMap::new();
        for pack in &self.packs {
            if !pack_ids.insert(pack.pack_id.as_str()) {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    pack.pack_id.clone(),
                    "duplicate pack id",
                ));
            }
            pack_revisions.insert(pack.pack_id.as_str(), pack.pack_revision_ref.as_str());
            if !pack.available_locales.contains(&pack.source_locale) {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    pack.pack_id.clone(),
                    "pack available_locales does not include source_locale",
                ));
            }
            if pack.install_state == OnboardingHelpPackInstallState::NotInstalled
                && pack.offline_posture != OnboardingHelpPackOfflinePosture::NotAvailableOffline
            {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    pack.pack_id.clone(),
                    "not-installed pack must declare not_available_offline",
                ));
            }
            if pack.install_state == OnboardingHelpPackInstallState::LocalOnlyStarter
                && pack.offline_posture
                    != OnboardingHelpPackOfflinePosture::FullyAvailableOfflineLocalBuild
            {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    pack.pack_id.clone(),
                    "local-only starter pack must declare fully_available_offline_local_build",
                ));
            }
        }

        let mut item_ids = BTreeSet::new();
        let mut progress_ids = BTreeSet::new();
        let progress_by_id = self
            .progress_states
            .iter()
            .map(|state| (state.state_item_id.as_str(), state))
            .collect::<BTreeMap<_, _>>();

        for state in &self.progress_states {
            if !progress_ids.insert(state.state_item_id.as_str()) {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    state.state_item_id.clone(),
                    "duplicate progress state id",
                ));
            }
            if state.storage_lane != OnboardingHelpPackStorageLane::PortableUserProfileState {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    state.state_item_id.clone(),
                    "progress state must live in portable user profile state",
                ));
            }
            if state.repo_mutation_allowed
                || state.repo_read_access_default
                || state.telemetry_read_default
            {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    state.state_item_id.clone(),
                    "progress state grants hidden repo or telemetry access",
                ));
            }
        }

        for item in &self.items {
            if !item_ids.insert(item.item_id.as_str()) {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "duplicate item id",
                ));
            }

            match pack_revisions.get(item.pack_id.as_str()) {
                Some(pack_revision) if *pack_revision == item.pack_revision_ref => {}
                Some(_) => findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "item pack revision does not match pack descriptor",
                )),
                None => findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "item references an unknown pack",
                )),
            }

            if item.command_hint.keyboard_route.trim().is_empty() {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "item command hint is missing keyboard_route",
                ));
            }
            if item.command_hint.metadata_source
                == OnboardingHelpPackCommandMetadataSource::CommandRegistry
            {
                match registry.get(&item.command_hint.command_id) {
                    Some(entry) => {
                        if let Some(revision_ref) = &item.command_hint.command_revision_ref {
                            if revision_ref != &entry.descriptor.command_revision_ref {
                                findings.push(OnboardingHelpPackValidationFinding::new(
                                    item.item_id.clone(),
                                    "item command revision ref drifted from registry",
                                ));
                            }
                        } else {
                            findings.push(OnboardingHelpPackValidationFinding::new(
                                item.item_id.clone(),
                                "registry-backed item is missing command_revision_ref",
                            ));
                        }
                    }
                    None => findings.push(OnboardingHelpPackValidationFinding::new(
                        item.item_id.clone(),
                        "registry-backed item references unknown command_id",
                    )),
                }
            }

            if item.locale.locale_availability
                == OnboardingHelpPackLocaleAvailability::LocaleMissingFallbackToPrimary
            {
                if item.source_language_fallback.fallback_class
                    != OnboardingHelpPackFallbackClass::FallbackToSourceLanguageDisclosed
                {
                    findings.push(OnboardingHelpPackValidationFinding::new(
                        item.item_id.clone(),
                        "locale fallback item is missing fallback disclosure",
                    ));
                }
                if item
                    .source_language_fallback
                    .source_language_item_ref
                    .is_none()
                {
                    findings.push(OnboardingHelpPackValidationFinding::new(
                        item.item_id.clone(),
                        "locale fallback item is missing source language item ref",
                    ));
                }
                if item.locale.effective_locale != item.locale.source_locale {
                    findings.push(OnboardingHelpPackValidationFinding::new(
                        item.item_id.clone(),
                        "source-language fallback effective_locale must equal source_locale",
                    ));
                }
            }

            if item.citation.availability == OnboardingHelpPackCitationAvailability::Available
                && item.citation.citation_refs.is_empty()
            {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "item declares available citations without citation refs",
                ));
            }
            if item.citation.availability == OnboardingHelpPackCitationAvailability::RequiredMissing
                && !item.citation.citation_refs.is_empty()
            {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "item declares missing citations but still carries citation refs",
                ));
            }
            if item.content_render_state == OnboardingHelpPackRenderState::Renderable
                && item.citation.availability != OnboardingHelpPackCitationAvailability::Available
            {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "renderable item must preserve citation refs",
                ));
            }
            if let Some(pack) = self.pack(&item.pack_id) {
                if pack.install_state == OnboardingHelpPackInstallState::NotInstalled
                    && item.content_render_state
                        != OnboardingHelpPackRenderState::BlockedNotInstalled
                {
                    findings.push(OnboardingHelpPackValidationFinding::new(
                        item.item_id.clone(),
                        "not-installed pack item must render as blocked_not_installed",
                    ));
                }
            }

            if item.exact_reopen_ref != item.support_export_identity.exact_reopen_ref {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "support export exact reopen ref does not match item",
                ));
            }
            if item.support_export_identity.raw_body_exported {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "support export must not include raw item body",
                ));
            }

            match progress_by_id.get(item.progress_state_ref.as_str()) {
                Some(state) if state.item_ref == item.item_id => {}
                Some(_) => findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "item progress_state_ref points at a state for a different item",
                )),
                None => findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "item progress_state_ref cannot be resolved",
                )),
            }

            if item.item_kind == OnboardingHelpPackItemKind::FutureGuidedLearningRef
                && item.future_surface_refs.is_empty()
            {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    item.item_id.clone(),
                    "future guided-learning item is missing future_surface_refs",
                ));
            }
        }

        for state in &self.progress_states {
            if !item_ids.contains(state.item_ref.as_str()) {
                findings.push(OnboardingHelpPackValidationFinding::new(
                    state.state_item_id.clone(),
                    "progress state references an unknown item",
                ));
            }
        }

        for proof in &self.protected_proofs {
            for item_ref in &proof.exercised_item_refs {
                if !item_ids.contains(item_ref.as_str()) {
                    findings.push(OnboardingHelpPackValidationFinding::new(
                        proof.proof_id.clone(),
                        "proof references an unknown item",
                    ));
                }
            }
        }

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

/// Parses the checked-in onboarding/help-pack alpha manifest.
///
/// # Errors
///
/// Returns a YAML parse error when the checked-in manifest no longer matches
/// the shell-side record model.
pub fn current_onboarding_help_pack_alpha_manifest(
) -> Result<OnboardingHelpPackAlphaManifest, serde_yaml::Error> {
    serde_yaml::from_str(CURRENT_ONBOARDING_HELP_PACK_ALPHA_YAML)
}

/// One source pack that owns alpha onboarding/help items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackAlphaPack {
    /// Stable pack id.
    pub pack_id: String,
    /// Stable pack revision ref.
    pub pack_revision_ref: String,
    /// Pack role across onboarding, glossary, and guided content.
    pub pack_role: OnboardingHelpPackRole,
    /// Source class inherited from docs-pack governance.
    pub source_class: OnboardingHelpPackSourceClass,
    /// Opaque source artifact or docs node ref.
    pub source_ref: String,
    /// Opaque source version or release ref.
    pub source_version_ref: String,
    /// Freshness class visible on consuming surfaces.
    pub freshness_class: OnboardingHelpPackFreshnessClass,
    /// Version-match state against the running build.
    pub version_match_state: OnboardingHelpPackVersionMatchState,
    /// Install state visible to help/search/onboarding.
    pub install_state: OnboardingHelpPackInstallState,
    /// Offline posture visible to help/search/onboarding.
    pub offline_posture: OnboardingHelpPackOfflinePosture,
    /// Canonical source-language locale.
    pub source_locale: String,
    /// Locales available in this pack.
    pub available_locales: Vec<String>,
    /// Citation availability for this pack.
    pub citation_availability: OnboardingHelpPackCitationAvailability,
}

/// One stable item in the onboarding/help-pack substrate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackAlphaItem {
    /// Stable item id.
    pub item_id: String,
    /// Item family.
    pub item_kind: OnboardingHelpPackItemKind,
    /// User-facing title.
    pub title: String,
    /// Owning pack id.
    pub pack_id: String,
    /// Owning pack revision ref.
    pub pack_revision_ref: String,
    /// Docs-node identity that owns citations and exact reopen.
    pub docs_node_id: String,
    /// Optional glossary item id for glossary cards.
    #[serde(default)]
    pub glossary_item_id: Option<String>,
    /// Command-aligned hint metadata.
    pub command_hint: OnboardingHelpPackCommandHint,
    /// Locale and fallback state for this item.
    pub locale: OnboardingHelpPackLocaleState,
    /// Source-language fallback disclosure.
    pub source_language_fallback: OnboardingHelpPackSourceLanguageFallback,
    /// Citation refs and drawer/source-strip metadata.
    pub citation: OnboardingHelpPackCitationState,
    /// Whether content renders, renders as placeholder, or is blocked.
    pub content_render_state: OnboardingHelpPackRenderState,
    /// Exact reopen ref preserving pack revision and locale.
    pub exact_reopen_ref: String,
    /// Export-safe reconstruction identity.
    pub support_export_identity: OnboardingHelpPackSupportExportIdentity,
    /// Progress state row owned by this item.
    pub progress_state_ref: String,
    /// Future surfaces that may reference this item without changing IDs.
    #[serde(default)]
    pub future_surface_refs: Vec<String>,
}

/// Command metadata embedded in an onboarding/help-pack item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackCommandHint {
    /// Stable command id.
    pub command_id: String,
    /// Command revision ref when the command is registry-backed.
    #[serde(default)]
    pub command_revision_ref: Option<String>,
    /// Stable help anchor id.
    pub help_anchor_id: String,
    /// Keyboard route shown to keyboard users.
    pub keyboard_route: String,
    /// Route family used for exact reopen.
    pub route_kind: OnboardingHelpPackRouteKind,
    /// Source of command metadata.
    pub metadata_source: OnboardingHelpPackCommandMetadataSource,
}

/// Locale state for a pack item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackLocaleState {
    /// Canonical source-language locale.
    pub source_locale: String,
    /// Locale the user asked for.
    pub requested_locale: String,
    /// Locale actually rendered.
    pub effective_locale: String,
    /// Locale availability class.
    pub locale_availability: OnboardingHelpPackLocaleAvailability,
    /// Whether the active translation is stale relative to source.
    pub stale_translation_marker: bool,
}

/// Source-language fallback disclosure state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackSourceLanguageFallback {
    /// Fallback class rendered by consumers.
    pub fallback_class: OnboardingHelpPackFallbackClass,
    /// Source-language item ref when fallback is active.
    #[serde(default)]
    pub source_language_item_ref: Option<String>,
    /// Command used for source-language or external reopen escape hatch.
    #[serde(default)]
    pub escape_hatch_command_id: Option<String>,
}

/// Citation state for a pack item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackCitationState {
    /// Citation availability class.
    pub availability: OnboardingHelpPackCitationAvailability,
    /// Citation refs preserved for support/export reconstruction.
    pub citation_refs: Vec<String>,
    /// Source strip ref opened from the item.
    #[serde(default)]
    pub source_strip_ref: Option<String>,
    /// Citation drawer ref opened from the item.
    #[serde(default)]
    pub citation_drawer_ref: Option<String>,
}

/// Support-export identity for a pack item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackSupportExportIdentity {
    /// Export-safe support pack item id.
    pub support_pack_item_id: String,
    /// Redaction class for the exported row.
    pub redaction_class: OnboardingHelpPackRedactionClass,
    /// Whether raw item body text may be exported.
    pub raw_body_exported: bool,
    /// Exact reopen ref preserved by support packets.
    pub exact_reopen_ref: String,
}

/// Portable progress state for one item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackProgressState {
    /// Stable state item id.
    pub state_item_id: String,
    /// Item this state belongs to.
    pub item_ref: String,
    /// State kind.
    pub state_kind: OnboardingHelpPackProgressKind,
    /// Storage lane for the state.
    pub storage_lane: OnboardingHelpPackStorageLane,
    /// Reset class for this state.
    pub reset_class: OnboardingHelpPackResetClass,
    /// Export class for this state.
    pub export_class: OnboardingHelpPackProgressExportClass,
    /// Whether the state grants repo mutation.
    pub repo_mutation_allowed: bool,
    /// Whether the state grants default repo read access.
    pub repo_read_access_default: bool,
    /// Whether the state grants default telemetry-style read access.
    pub telemetry_read_default: bool,
}

/// Protected proof fixture row for the alpha manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackProof {
    /// Stable proof id.
    pub proof_id: String,
    /// Fixture path relative to the repo root.
    pub fixture_ref: String,
    /// Item refs exercised by this proof.
    pub exercised_item_refs: Vec<String>,
    /// State names exercised by this proof.
    pub exercised_states: Vec<String>,
}

/// Export-safe reconstruction row for support packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackSupportExportRow {
    /// Stable item id.
    pub item_id: String,
    /// Owning pack id.
    pub pack_id: String,
    /// Owning pack revision ref.
    pub pack_revision_ref: String,
    /// Stable command id.
    pub command_id: String,
    /// Stable help anchor id.
    pub help_anchor_id: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Effective locale.
    pub effective_locale: String,
    /// Source-language fallback class token.
    pub fallback_class: String,
    /// Citation refs preserved for reconstruction.
    pub citation_refs: Vec<String>,
    /// Exact reopen ref preserving pack revision and locale.
    pub exact_reopen_ref: String,
    /// Export-safe support pack item id.
    pub support_pack_item_id: String,
    /// Whether raw item body text was exported.
    pub raw_body_exported: bool,
}

/// Validation finding for onboarding/help-pack manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackValidationFinding {
    /// Row id that failed validation.
    pub row_ref: String,
    /// Validation message.
    pub message: String,
}

impl OnboardingHelpPackValidationFinding {
    fn new(row_ref: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            row_ref: row_ref.into(),
            message: message.into(),
        }
    }
}

/// Pack role vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackRole {
    /// First-run starter content rendered in Start Center.
    FirstRunStarterPack,
    /// Migration or keymap bridge content.
    MigrationWelcomePack,
    /// Glossary card pack.
    GlossaryBundle,
    /// Guided content pack.
    GuidedContentPack,
    /// In-product help overlay pack.
    InProductHelpOverlayPack,
}

/// Source class vocabulary for onboarding/help packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackSourceClass {
    /// Project-owned docs.
    ProjectDocs,
    /// Generated reference docs.
    GeneratedReference,
    /// Signed mirror of official docs.
    MirroredOfficialDocs,
    /// Curated knowledge pack.
    CuratedKnowledgePack,
    /// Support runbook pack.
    SupportRunbook,
}

/// Freshness vocabulary for onboarding/help packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackFreshnessClass {
    /// Source was authoritative and live when minted.
    AuthoritativeLive,
    /// Cached source is inside its freshness window.
    WarmCached,
    /// Cached source is usable only with degraded disclosure.
    DegradedCached,
    /// Source is stale.
    Stale,
    /// Freshness is unverified.
    Unverified,
}

/// Version-match vocabulary for onboarding/help packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackVersionMatchState {
    /// Pack exactly matches the running build.
    ExactBuildMatch,
    /// Pack is inside compatible minor drift.
    CompatibleMinorDrift,
    /// Pack is incompatible with the running build.
    IncompatibleDriftDetected,
    /// Pre-release pack is not verified.
    PreReleaseUnverified,
    /// Target build is unknown.
    UnknownTargetBuild,
}

/// Install-state vocabulary for onboarding/help packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackInstallState {
    /// Pack ships with the local product build.
    LocalOnlyStarter,
    /// Pack is cached and current.
    CachedSnapshotCurrent,
    /// Pack is cached but stale.
    CachedSnapshotStale,
    /// Pack came from a verified mirror.
    MirrorOnlyVerified,
    /// Pack is referenced but not installed.
    NotInstalled,
}

impl OnboardingHelpPackInstallState {
    /// Returns the stable token for this install state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyStarter => "local_only_starter",
            Self::CachedSnapshotCurrent => "cached_snapshot_current",
            Self::CachedSnapshotStale => "cached_snapshot_stale",
            Self::MirrorOnlyVerified => "mirror_only_verified",
            Self::NotInstalled => "not_installed",
        }
    }
}

/// Offline-posture vocabulary for onboarding/help packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackOfflinePosture {
    /// Pack is fully available from the local build.
    FullyAvailableOfflineLocalBuild,
    /// Pack is available from a cached snapshot.
    CachedSnapshotOffline,
    /// Pack is available from a verified mirror.
    MirrorVerifiedOffline,
    /// Pack is not available offline.
    NotAvailableOffline,
}

impl OnboardingHelpPackOfflinePosture {
    /// Returns the stable token for this offline posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyAvailableOfflineLocalBuild => "fully_available_offline_local_build",
            Self::CachedSnapshotOffline => "cached_snapshot_offline",
            Self::MirrorVerifiedOffline => "mirror_verified_offline",
            Self::NotAvailableOffline => "not_available_offline",
        }
    }
}

/// Item-kind vocabulary for onboarding/help packs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackItemKind {
    /// Start Center card item.
    StartCenterCard,
    /// Contextual hint item.
    ContextualHint,
    /// Keymap bridge item.
    KeymapBridge,
    /// Migration guidance item.
    MigrationGuidance,
    /// Glossary card item.
    GlossaryPackItem,
    /// Future guided-learning reference item.
    FutureGuidedLearningRef,
}

/// Route-kind vocabulary for command-aligned hints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackRouteKind {
    /// Command invocation route.
    Command,
    /// Help search route.
    HelpSearch,
    /// Browser handoff route.
    BrowserHandoff,
    /// Evidence card route.
    EvidenceCard,
}

/// Command metadata source vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackCommandMetadataSource {
    /// Command metadata is owned by the canonical command registry.
    CommandRegistry,
    /// Command metadata is bounded to this alpha descriptor.
    AlphaOwnedDescriptor,
}

/// Locale availability vocabulary for pack items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackLocaleAvailability {
    /// Requested locale is reviewed.
    LocaleAvailableReviewed,
    /// Requested locale is available but stale.
    LocaleAvailableStaleCopy,
    /// Requested locale falls back to the source language.
    LocaleMissingFallbackToPrimary,
    /// Requested locale pack is not installed.
    LocaleMissingNotInstalled,
}

impl OnboardingHelpPackLocaleAvailability {
    /// Returns the stable token for this locale state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocaleAvailableReviewed => "locale_available_reviewed",
            Self::LocaleAvailableStaleCopy => "locale_available_stale_copy",
            Self::LocaleMissingFallbackToPrimary => "locale_missing_fallback_to_primary",
            Self::LocaleMissingNotInstalled => "locale_missing_not_installed",
        }
    }
}

/// Source-language fallback vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackFallbackClass {
    /// No fallback is needed.
    NoFallbackPrimaryLocaleOnly,
    /// Fallback to source language is disclosed.
    FallbackToSourceLanguageDisclosed,
    /// Fallback is blocked because the pack is missing.
    FallbackBlockedPackMissing,
}

impl OnboardingHelpPackFallbackClass {
    /// Returns the stable token for this fallback state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoFallbackPrimaryLocaleOnly => "no_fallback_primary_locale_only",
            Self::FallbackToSourceLanguageDisclosed => "fallback_to_source_language_disclosed",
            Self::FallbackBlockedPackMissing => "fallback_blocked_pack_missing",
        }
    }
}

/// Citation availability vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackCitationAvailability {
    /// Citation refs are available.
    Available,
    /// Citation refs are required but missing.
    RequiredMissing,
    /// Citations are not required.
    NotRequired,
}

/// Render state for a pack item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackRenderState {
    /// Item body is renderable.
    Renderable,
    /// Item can render only as a placeholder.
    PlaceholderOnly,
    /// Item is blocked because the owning pack is not installed.
    BlockedNotInstalled,
}

/// Support export redaction vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackRedactionClass {
    /// Metadata is safe by default.
    MetadataSafeDefault,
    /// Metadata is restricted to operators.
    OperatorOnlyRestricted,
}

/// Portable progress state kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackProgressKind {
    /// Hint was dismissed.
    DismissedHint,
    /// Item was completed.
    CompletedItem,
    /// Item is a resume point.
    ResumePoint,
    /// Glossary item was bookmarked.
    BookmarkedGlossaryItem,
    /// Pack install was deferred.
    DeferredPackInstall,
}

/// Storage lane for progress state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackStorageLane {
    /// State lives in portable user profile state.
    PortableUserProfileState,
}

/// Reset class for progress state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackResetClass {
    /// State resets per profile.
    ResettablePerProfile,
    /// State resets when the pack refreshes.
    ResettableWithPackRefresh,
    /// State resets when the pack is reinstalled or imported.
    ResettableWithReinstallOrImport,
}

/// Export class for progress state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OnboardingHelpPackProgressExportClass {
    /// State is exported in portable profile packages.
    InPortableProfilePackage,
    /// State is exported only in redacted support bundles.
    InSupportBundleRedacted,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use aureline_commands::registry::seeded_registry;
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize)]
    struct FixtureCase {
        expected: serde_yaml::Value,
    }

    fn fixture(name: &str) -> FixtureCase {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/docs/onboarding_help_pack_alpha")
            .join(format!("{name}.yaml"));
        let payload = std::fs::read_to_string(&path).expect("fixture reads");
        serde_yaml::from_str(&payload).expect("fixture parses")
    }

    #[test]
    fn current_manifest_validates_against_command_registry() {
        let manifest = current_onboarding_help_pack_alpha_manifest().expect("manifest parses");
        manifest
            .validate_against_registry(seeded_registry())
            .expect("manifest validates");
        assert!(manifest
            .runtime_consumer_refs
            .contains(&"surface:onboarding_alpha:first_run_start_center".to_owned()));
    }

    #[test]
    fn source_language_fallback_fixture_matches_manifest() {
        let manifest = current_onboarding_help_pack_alpha_manifest().expect("manifest parses");
        let fixture = fixture("source_language_fallback");
        let expected = &fixture.expected;
        let item_ref = expected["item_ref"].as_str().expect("item ref");
        let item = manifest.item(item_ref).expect("item exists");

        assert_eq!(
            item.locale.requested_locale,
            expected["requested_locale"]
                .as_str()
                .expect("requested locale")
        );
        assert_eq!(
            item.locale.effective_locale,
            expected["effective_locale"]
                .as_str()
                .expect("effective locale")
        );
        assert_eq!(
            item.source_language_fallback.fallback_class.as_str(),
            expected["fallback_class"].as_str().expect("fallback class")
        );
        assert_eq!(
            item.command_hint.command_id,
            expected["command_id"].as_str().expect("command id")
        );
        assert_eq!(
            item.command_hint.keyboard_route,
            expected["keyboard_route"].as_str().expect("keyboard route")
        );
        assert_eq!(
            item.citation.citation_refs,
            expected["citation_refs"]
                .as_sequence()
                .expect("citation refs")
                .iter()
                .map(|value| value.as_str().expect("citation ref").to_owned())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            item.support_export_identity.support_pack_item_id,
            expected["support_pack_item_id"]
                .as_str()
                .expect("support id")
        );
    }

    #[test]
    fn offline_posture_fixture_matches_manifest() {
        let manifest = current_onboarding_help_pack_alpha_manifest().expect("manifest parses");
        let fixture = fixture("offline_posture_matrix");
        let rows = fixture.expected["pack_states"]
            .as_sequence()
            .expect("pack states");
        for row in rows {
            let pack_id = row["pack_id"].as_str().expect("pack id");
            let pack = manifest.pack(pack_id).expect("pack exists");
            assert_eq!(
                pack.install_state.as_str(),
                row["install_state"].as_str().expect("install state")
            );
            assert_eq!(
                pack.offline_posture.as_str(),
                row["offline_posture"].as_str().expect("offline posture")
            );
        }

        let missing = manifest
            .item("ohp:item:learning_digest.not_installed")
            .expect("not-installed item");
        assert_eq!(
            missing.content_render_state,
            OnboardingHelpPackRenderState::BlockedNotInstalled
        );
        assert_eq!(
            missing.source_language_fallback.fallback_class,
            OnboardingHelpPackFallbackClass::FallbackBlockedPackMissing
        );
    }

    #[test]
    fn glossary_and_future_guided_learning_items_keep_portable_progress() {
        let manifest = current_onboarding_help_pack_alpha_manifest().expect("manifest parses");
        let fixture = fixture("glossary_guided_learning_reference");
        let glossary_expected = &fixture.expected["glossary_item"];
        let item = manifest
            .item(glossary_expected["item_ref"].as_str().expect("item ref"))
            .expect("glossary item");
        assert_eq!(
            item.glossary_item_id.as_deref(),
            glossary_expected["glossary_item_id"].as_str()
        );
        assert!(!item.future_surface_refs.is_empty());

        let state_expected = &fixture.expected["progress_state"];
        let state = manifest
            .progress_state(
                state_expected["state_item_id"]
                    .as_str()
                    .expect("state item id"),
            )
            .expect("progress state");
        assert_eq!(
            state.storage_lane,
            OnboardingHelpPackStorageLane::PortableUserProfileState
        );
        assert!(!state.repo_mutation_allowed);
        assert!(!state.repo_read_access_default);
        assert!(!state.telemetry_read_default);
    }

    #[test]
    fn support_export_rows_reconstruct_without_raw_bodies() {
        let manifest = current_onboarding_help_pack_alpha_manifest().expect("manifest parses");
        let rows = manifest.support_export_rows();
        assert!(rows.iter().all(|row| !row.raw_body_exported));
        assert!(rows.iter().any(
            |row| row.item_id == "ohp:item:keymap_bridge.command_palette"
                && row.command_id == "cmd:command_palette.open"
                && row.exact_reopen_ref.contains("#es-MX")
        ));
        assert!(rows
            .iter()
            .any(|row| row.support_pack_item_id == "support:onboarding-help:glossary.workset"));
    }
}
