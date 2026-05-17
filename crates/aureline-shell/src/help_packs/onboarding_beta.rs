//! Beta onboarding/help-pack manifest, projections, and validation.
//!
//! The beta pack is the shell-readable contract for first-run cards,
//! migration-center guidance, Help search results, contextual why-now cards,
//! glossary entries, and recovery-first help. It keeps command identity,
//! citations, freshness, locale fallback, mirror/offline posture, and
//! support-export reconstruction in one typed record.

use std::collections::{BTreeMap, BTreeSet};

use aureline_commands::CommandRegistry;
use serde::{Deserialize, Serialize};

/// Schema version for beta onboarding/help-pack records.
pub const ONBOARDING_HELP_PACK_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`OnboardingHelpPackBetaManifest`].
pub const ONBOARDING_HELP_PACK_BETA_RECORD_KIND: &str = "onboarding_help_pack_beta_manifest_record";

/// Stable record kind for [`OnboardingHelpPackBetaSurfaceProjection`].
pub const ONBOARDING_HELP_PACK_BETA_SURFACE_PROJECTION_RECORD_KIND: &str =
    "onboarding_help_pack_beta_surface_projection_record";

/// Stable record kind for [`OnboardingHelpPackBetaSupportExport`].
pub const ONBOARDING_HELP_PACK_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "onboarding_help_pack_beta_support_export_record";

/// Stable id for the seeded beta onboarding/help pack.
pub const ONBOARDING_HELP_PACK_BETA_MANIFEST_ID: &str = "help-pack:onboarding:beta:v1";

/// Stable version ref for the seeded beta onboarding/help pack.
pub const ONBOARDING_HELP_PACK_BETA_VERSION_REF: &str = "help-pack-rev:onboarding:2026.05.17-01";

/// Repository-relative schema ref for the beta help-pack manifest.
pub const ONBOARDING_HELP_PACK_BETA_SCHEMA_REF: &str =
    "schemas/help/onboarding_help_pack.schema.json";

/// Repository-relative fixture ref for the beta help-pack manifest.
pub const ONBOARDING_HELP_PACK_BETA_FIXTURE_REF: &str =
    "fixtures/help/m3/onboarding_help_packs/manifest.json";

/// Repository-relative support-export fixture ref.
pub const ONBOARDING_HELP_PACK_BETA_SUPPORT_EXPORT_FIXTURE_REF: &str =
    "fixtures/help/m3/onboarding_help_packs/support_export.json";

/// Repository-relative surface-projection fixture ref.
pub const ONBOARDING_HELP_PACK_BETA_SURFACE_FIXTURE_REF: &str =
    "fixtures/help/m3/onboarding_help_packs/surface_projection.json";

/// Repository-relative docs page ref for the beta help-pack contract.
pub const ONBOARDING_HELP_PACK_BETA_DOC_REF: &str = "docs/help/m3/onboarding_help_pack_beta.md";

/// Repository-relative release packet ref for the beta help-pack contract.
pub const ONBOARDING_HELP_PACK_BETA_RELEASE_PACKET_REF: &str =
    "artifacts/help/m3/help_pack_release_packet.md";

const GENERATED_AT: &str = "2026-05-17T20:00:00Z";

const SURFACE_START_CENTER: &str = "surface:start_center:first_run";
const SURFACE_MIGRATION_CENTER: &str = "surface:migration_center:beta";
const SURFACE_HELP_SEARCH: &str = "surface:help_search:onboarding";
const SURFACE_CONTEXTUAL_WHY_NOW: &str = "surface:contextual_why_now:onboarding";
const SURFACE_RECOVERY_FIRST: &str = "surface:recovery_first:onboarding";

const PACK_ROLE_FIRST_RUN: &str = "first_run_starter_pack";
const PACK_ROLE_MIGRATION: &str = "migration_welcome_pack";
const PACK_ROLE_GLOSSARY: &str = "glossary_bundle";
const PACK_ROLE_GUIDED: &str = "guided_content_pack";
const PACK_ROLE_RECOVERY: &str = "recovery_first_pack";
const PACK_ROLE_HELP_OVERLAY: &str = "in_product_help_overlay_pack";

const SOURCE_PROJECT_DOCS: &str = "project_docs";
const SOURCE_MIRRORED_DOCS: &str = "mirrored_official_docs";
const SOURCE_CURATED_PACK: &str = "curated_knowledge_pack";
const SOURCE_SUPPORT_RUNBOOK: &str = "support_runbook";

const FRESHNESS_AUTHORITATIVE: &str = "authoritative_live";
const FRESHNESS_WARM_CACHED: &str = "warm_cached";
const FRESHNESS_DEGRADED: &str = "degraded_cached";

const VERSION_EXACT: &str = "exact_build_match";
const VERSION_COMPATIBLE: &str = "compatible_minor_drift";

const INSTALL_LOCAL: &str = "local_only_starter";
const INSTALL_CACHED: &str = "cached_snapshot_current";
const INSTALL_MIRROR: &str = "mirror_only_verified";
const INSTALL_NOT_INSTALLED: &str = "not_installed";

const OFFLINE_LOCAL: &str = "fully_available_offline_local_build";
const OFFLINE_CACHED: &str = "cached_snapshot_offline";
const OFFLINE_MIRROR: &str = "mirror_verified_offline";
const OFFLINE_NOT_AVAILABLE: &str = "not_available_offline";

const MIRROR_NONE: &str = "no_mirror_required";
const MIRROR_VERIFIED: &str = "verified_mirror_snapshot";

const LOCALE_REVIEWED: &str = "locale_available_reviewed";
const LOCALE_STALE: &str = "locale_available_stale_copy";
const LOCALE_SOURCE_FALLBACK: &str = "locale_missing_fallback_to_source";
const LOCALE_NOT_INSTALLED: &str = "locale_missing_not_installed";

const FALLBACK_NONE: &str = "no_fallback_primary_locale_only";
const FALLBACK_SOURCE: &str = "fallback_to_source_language_disclosed";
const FALLBACK_PACK_MISSING: &str = "fallback_blocked_pack_missing";

const METADATA_COMMAND_REGISTRY: &str = "command_registry";
const METADATA_PACK_DESCRIPTOR: &str = "pack_owned_descriptor";

const RENDERABLE: &str = "renderable";

const STATE_DISMISSED: &str = "dismissed_hint";
const STATE_HELPFUL: &str = "helpful_vote";
const STORAGE_PROFILE: &str = "portable_user_profile_state";
const RESET_PER_PROFILE: &str = "resettable_per_profile";
const EXPORT_SUPPORT_REDACTED: &str = "in_support_bundle_redacted";

/// Versioned manifest consumed by onboarding, Help search, migration, and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaManifest {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Stable version ref for this manifest.
    pub manifest_version_ref: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Channel or lifecycle posture for the manifest.
    pub release_channel: String,
    /// Contract refs that own the model vocabulary.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime consumers that resolve against this manifest.
    pub runtime_consumer_refs: Vec<String>,
    /// Surface classes that must be covered by at least one item.
    pub required_surface_classes: Vec<String>,
    /// Source packs available to item rows.
    pub packs: Vec<OnboardingHelpPackBetaPack>,
    /// Help, onboarding, glossary, and recovery item rows.
    pub items: Vec<OnboardingHelpPackBetaItem>,
    /// Portable dismissal/helpfulness state rows for the items.
    pub state_records: Vec<OnboardingHelpPackBetaStateRecord>,
    /// Bounded diagnostics policy for support/export reconstruction.
    pub diagnostics_policy: OnboardingHelpPackDiagnosticsPolicy,
    /// Protected proof refs covering locale, offline, command, and export cases.
    pub protected_proofs: Vec<OnboardingHelpPackBetaProof>,
}

impl OnboardingHelpPackBetaManifest {
    /// Returns the pack with `pack_id`.
    pub fn pack(&self, pack_id: &str) -> Option<&OnboardingHelpPackBetaPack> {
        self.packs.iter().find(|pack| pack.pack_id == pack_id)
    }

    /// Returns the item with `item_id`.
    pub fn item(&self, item_id: &str) -> Option<&OnboardingHelpPackBetaItem> {
        self.items.iter().find(|item| item.item_id == item_id)
    }

    /// Returns the state record with `state_item_id`.
    pub fn state_record(&self, state_item_id: &str) -> Option<&OnboardingHelpPackBetaStateRecord> {
        self.state_records
            .iter()
            .find(|state| state.state_item_id == state_item_id)
    }

    /// Returns every item that declares `surface_ref`.
    pub fn items_for_surface(&self, surface_ref: &str) -> Vec<&OnboardingHelpPackBetaItem> {
        self.items
            .iter()
            .filter(|item| {
                item.surface_refs
                    .iter()
                    .any(|surface| surface == surface_ref)
            })
            .collect()
    }

    /// Projects a deterministic surface coverage artifact.
    pub fn surface_projection(&self) -> OnboardingHelpPackBetaSurfaceProjection {
        let mut rows = Vec::new();
        for item in &self.items {
            for surface_ref in &item.surface_refs {
                rows.push(OnboardingHelpPackBetaSurfaceRow {
                    row_id: format!("help-pack-surface-row:{}:{}", surface_ref, item.item_id),
                    surface_ref: surface_ref.clone(),
                    item_id: item.item_id.clone(),
                    pack_id: item.pack_id.clone(),
                    pack_version_ref: item.pack_version_ref.clone(),
                    command_id: item.command_hint.command_id.clone(),
                    help_anchor_id: item.command_hint.help_anchor_id.clone(),
                    keyboard_route: item.command_hint.keyboard_route.clone(),
                    requested_locale: item.locale.requested_locale.clone(),
                    effective_locale: item.locale.effective_locale.clone(),
                    locale_availability: item.locale.locale_availability.clone(),
                    source_language_fallback_class: item
                        .source_language_fallback
                        .fallback_class
                        .clone(),
                    offline_posture: item.offline_fallback.content_availability.clone(),
                    mirror_posture: item.offline_fallback.mirror_posture.clone(),
                    freshness_class: item.source_truth.freshness_class.clone(),
                    version_match_state: item.source_truth.version_match_state.clone(),
                    exact_reopen_ref: item.exact_reopen_ref.clone(),
                    support_pack_item_id: item.support_export_identity.support_pack_item_id.clone(),
                });
            }
        }
        OnboardingHelpPackBetaSurfaceProjection {
            record_kind: ONBOARDING_HELP_PACK_BETA_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: ONBOARDING_HELP_PACK_BETA_SCHEMA_VERSION,
            projection_id: "help-pack:onboarding:beta:surface-projection:v1".to_owned(),
            generated_at: self.generated_at.clone(),
            manifest_id: self.manifest_id.clone(),
            manifest_version_ref: self.manifest_version_ref.clone(),
            rows,
            coverage: self.surface_coverage(),
        }
    }

    /// Projects a metadata-only support export for the active manifest.
    pub fn support_export(
        &self,
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> OnboardingHelpPackBetaSupportExport {
        let active_pack_versions = self
            .packs
            .iter()
            .map(|pack| OnboardingHelpPackBetaActivePackVersion {
                pack_id: pack.pack_id.clone(),
                pack_version_ref: pack.pack_version_ref.clone(),
                source_version_ref: pack.source_version_ref.clone(),
                freshness_class: pack.freshness_class.clone(),
                version_match_state: pack.version_match_state.clone(),
                mirror_posture: pack.mirror_posture.clone(),
                client_scope_token: pack.release_truth_badges.client_scope_token.clone(),
            })
            .collect::<Vec<_>>();

        let rows = self
            .items
            .iter()
            .map(|item| OnboardingHelpPackBetaSupportRow {
                support_pack_item_id: item.support_export_identity.support_pack_item_id.clone(),
                item_id: item.item_id.clone(),
                pack_id: item.pack_id.clone(),
                active_pack_version_ref: item
                    .support_export_identity
                    .active_pack_version_ref
                    .clone(),
                command_id: item.command_hint.command_id.clone(),
                help_anchor_id: item.command_hint.help_anchor_id.clone(),
                requested_locale: item.locale.requested_locale.clone(),
                effective_locale: item.locale.effective_locale.clone(),
                locale_availability: item.locale.locale_availability.clone(),
                source_language_fallback_class: item
                    .source_language_fallback
                    .fallback_class
                    .clone(),
                unresolved_locale_fallback: item.support_export_identity.unresolved_locale_fallback,
                unresolved_source_fallback: item.support_export_identity.unresolved_source_fallback,
                mirror_posture: item.offline_fallback.mirror_posture.clone(),
                content_availability: item.offline_fallback.content_availability.clone(),
                source_class: item.source_truth.source_class.clone(),
                source_version_ref: item.source_truth.source_version_ref.clone(),
                freshness_class: item.source_truth.freshness_class.clone(),
                version_match_state: item.source_truth.version_match_state.clone(),
                citation_refs: item.source_truth.citation_refs.clone(),
                exact_reopen_ref: item.exact_reopen_ref.clone(),
                diagnostics_row_ref: item.support_export_identity.diagnostics_row_ref.clone(),
                dismissed_state_ref: item.support_export_identity.dismissed_state_ref.clone(),
                helpful_state_ref: item.support_export_identity.helpful_state_ref.clone(),
                raw_body_exported: item.support_export_identity.raw_body_exported,
            })
            .collect::<Vec<_>>();

        let unresolved_fallback_rows = rows
            .iter()
            .filter(|row| row.unresolved_locale_fallback || row.unresolved_source_fallback)
            .map(|row| row.support_pack_item_id.clone())
            .collect::<Vec<_>>();

        OnboardingHelpPackBetaSupportExport {
            record_kind: ONBOARDING_HELP_PACK_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ONBOARDING_HELP_PACK_BETA_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            generated_at: generated_at.into(),
            source_manifest_id: self.manifest_id.clone(),
            manifest_version_ref: self.manifest_version_ref.clone(),
            diagnostics_policy_id: self.diagnostics_policy.diagnostics_policy_id.clone(),
            active_pack_versions,
            rows,
            unresolved_fallback_rows,
            omitted_material_classes: self.diagnostics_policy.omitted_material_classes.clone(),
            raw_body_exported: false,
        }
    }

    /// Validates pack, item, command, locale, offline, and support-export invariants.
    pub fn validate_against_registry(
        &self,
        registry: &CommandRegistry,
    ) -> Result<(), Vec<OnboardingHelpPackBetaFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != ONBOARDING_HELP_PACK_BETA_RECORD_KIND {
            findings.push(OnboardingHelpPackBetaFinding::new(
                self.manifest_id.clone(),
                "manifest record_kind is unsupported",
            ));
        }
        if self.schema_version != ONBOARDING_HELP_PACK_BETA_SCHEMA_VERSION {
            findings.push(OnboardingHelpPackBetaFinding::new(
                self.manifest_id.clone(),
                "manifest schema version is unsupported",
            ));
        }
        if self.manifest_version_ref.trim().is_empty() {
            findings.push(OnboardingHelpPackBetaFinding::new(
                self.manifest_id.clone(),
                "manifest_version_ref must be non-empty",
            ));
        }
        if self.runtime_consumer_refs.is_empty() {
            findings.push(OnboardingHelpPackBetaFinding::new(
                self.manifest_id.clone(),
                "manifest has no runtime consumers",
            ));
        }
        validate_diagnostics_policy(&self.diagnostics_policy, &mut findings);

        let mut pack_ids = BTreeSet::new();
        for pack in &self.packs {
            validate_pack(pack, &mut findings);
            if !pack_ids.insert(pack.pack_id.as_str()) {
                findings.push(OnboardingHelpPackBetaFinding::new(
                    pack.pack_id.clone(),
                    "duplicate pack_id",
                ));
            }
        }

        let mut item_ids = BTreeSet::new();
        for item in &self.items {
            validate_item(self, registry, item, &mut findings);
            if !item_ids.insert(item.item_id.as_str()) {
                findings.push(OnboardingHelpPackBetaFinding::new(
                    item.item_id.clone(),
                    "duplicate item_id",
                ));
            }
        }

        for state in &self.state_records {
            validate_state(state, &item_ids, &mut findings);
        }

        for proof in &self.protected_proofs {
            if proof.fixture_ref.trim().is_empty() {
                findings.push(OnboardingHelpPackBetaFinding::new(
                    proof.proof_id.clone(),
                    "proof fixture_ref must be non-empty",
                ));
            }
            for item_ref in &proof.exercised_item_refs {
                if !item_ids.contains(item_ref.as_str()) {
                    findings.push(OnboardingHelpPackBetaFinding::new(
                        proof.proof_id.clone(),
                        "proof references an unknown item",
                    ));
                }
            }
        }

        let coverage = self.surface_coverage();
        for required in &self.required_surface_classes {
            if !coverage
                .covered_surface_refs
                .iter()
                .any(|surface| surface == required)
            {
                findings.push(OnboardingHelpPackBetaFinding::new(
                    self.manifest_id.clone(),
                    format!("required surface {required} is not covered"),
                ));
            }
        }

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    fn surface_coverage(&self) -> OnboardingHelpPackBetaCoverage {
        let mut covered_surface_refs = BTreeSet::new();
        let mut pack_ids = BTreeSet::new();
        let mut command_ids = BTreeSet::new();
        let mut unresolved_locale_fallback_count = 0;
        let mut unresolved_source_fallback_count = 0;
        for item in &self.items {
            for surface_ref in &item.surface_refs {
                covered_surface_refs.insert(surface_ref.clone());
            }
            pack_ids.insert(item.pack_id.clone());
            command_ids.insert(item.command_hint.command_id.clone());
            if item.locale.unresolved_locale_fallback {
                unresolved_locale_fallback_count += 1;
            }
            if item.source_language_fallback.unresolved_source_fallback {
                unresolved_source_fallback_count += 1;
            }
        }
        OnboardingHelpPackBetaCoverage {
            item_count: self.items.len(),
            pack_count: pack_ids.len(),
            command_count: command_ids.len(),
            covered_surface_refs: covered_surface_refs.into_iter().collect(),
            unresolved_locale_fallback_count,
            unresolved_source_fallback_count,
        }
    }
}

/// One source pack that owns beta onboarding/help items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaPack {
    /// Stable pack id.
    pub pack_id: String,
    /// Stable version ref for this pack.
    pub pack_version_ref: String,
    /// Pack role across onboarding, migration, glossary, and recovery help.
    pub pack_role: String,
    /// Glossary pack ids carried by this pack.
    pub glossary_pack_ids: Vec<String>,
    /// Source class inherited from docs/help truth vocabulary.
    pub source_class: String,
    /// Source artifact or docs-node ref.
    pub source_ref: String,
    /// Source version or release ref.
    pub source_version_ref: String,
    /// Freshness class visible on consuming surfaces.
    pub freshness_class: String,
    /// Version-match state against the active build.
    pub version_match_state: String,
    /// Optional docs-pack ref that backs this pack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_ref: Option<String>,
    /// Mirror posture for offline and air-gapped rows.
    pub mirror_posture: String,
    /// Install state visible to help/search/onboarding.
    pub install_state: String,
    /// Offline posture visible to help/search/onboarding.
    pub offline_posture: String,
    /// Canonical source-language locale.
    pub source_locale: String,
    /// Locales available in this pack.
    pub available_locales: Vec<String>,
    /// Locale overlays available from this pack.
    pub locale_overlay_refs: Vec<String>,
    /// Citation availability for this pack.
    pub citation_availability: String,
    /// Release-truth badges rendered by consuming surfaces.
    pub release_truth_badges: OnboardingHelpPackReleaseTruthBadges,
    /// Client scope token inherited by support and diagnostics.
    pub client_scope: String,
    /// Support owner ref for escalation and docs review.
    pub support_owner_ref: String,
}

/// One stable onboarding/help item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaItem {
    /// Stable item id.
    pub item_id: String,
    /// Item family.
    pub item_kind: String,
    /// User-facing title.
    pub title: String,
    /// Export-safe summary.
    pub summary: String,
    /// Owning pack id.
    pub pack_id: String,
    /// Owning pack version ref.
    pub pack_version_ref: String,
    /// Optional glossary pack id for glossary cards.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glossary_pack_id: Option<String>,
    /// Optional glossary item id for glossary cards.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glossary_item_id: Option<String>,
    /// Docs-node identity that owns citations and exact reopen.
    pub docs_node_id: String,
    /// Surface refs that may render this item.
    pub surface_refs: Vec<String>,
    /// Command-aligned hint metadata.
    pub command_hint: OnboardingHelpPackCommandHint,
    /// Locale and fallback state for this item.
    pub locale: OnboardingHelpPackLocaleState,
    /// Source-language fallback disclosure.
    pub source_language_fallback: OnboardingHelpPackSourceLanguageFallback,
    /// Offline, mirror, and cache fallback disclosure.
    pub offline_fallback: OnboardingHelpPackOfflineFallback,
    /// Source, version, freshness, citation, and badge metadata.
    pub source_truth: OnboardingHelpPackSourceTruth,
    /// Whether content renders, renders as placeholder, or is blocked.
    pub content_render_state: String,
    /// Exact reopen ref preserving pack revision and locale.
    pub exact_reopen_ref: String,
    /// Export-safe reconstruction identity.
    pub support_export_identity: OnboardingHelpPackSupportExportIdentity,
    /// Portable state refs owned by this item.
    pub state_refs: OnboardingHelpPackItemStateRefs,
}

/// Command metadata embedded in a beta help-pack item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackCommandHint {
    /// Stable command id.
    pub command_id: String,
    /// Command revision ref when the command is registry-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_revision_ref: Option<String>,
    /// Stable help anchor id.
    pub help_anchor_id: String,
    /// Keyboard route shown to keyboard users.
    pub keyboard_route: String,
    /// Route family used for exact reopen.
    pub route_kind: String,
    /// Source of command metadata.
    pub metadata_source: String,
    /// Canonical action-graph ref used by non-prose surfaces.
    pub canonical_action_graph_ref: String,
}

/// Locale state for a beta help-pack item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackLocaleState {
    /// Canonical source-language locale.
    pub source_locale: String,
    /// Locale the user requested.
    pub requested_locale: String,
    /// Locale actually rendered.
    pub effective_locale: String,
    /// Locale availability class.
    pub locale_availability: String,
    /// Visible locale fallback chain.
    pub fallback_chain: Vec<String>,
    /// Whether the active translation is stale relative to source.
    pub stale_translation_marker: bool,
    /// Whether support export must record unresolved locale fallback.
    pub unresolved_locale_fallback: bool,
    /// Locale overlay or translation pack ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translation_pack_ref: Option<String>,
}

/// Source-language fallback disclosure state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackSourceLanguageFallback {
    /// Fallback class rendered by consumers.
    pub fallback_class: String,
    /// Source-language item ref when fallback is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_language_item_ref: Option<String>,
    /// Command used for source-language or external reopen escape hatch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escape_hatch_command_id: Option<String>,
    /// Whether support export must record unresolved source fallback.
    pub unresolved_source_fallback: bool,
    /// User-visible state text token for fallback disclosure.
    pub user_visible_status: String,
}

/// Offline, mirror, and cache fallback state for one item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackOfflineFallback {
    /// Connectivity posture expected by this item.
    pub connectivity_posture: String,
    /// Mirror posture used by offline and air-gapped rows.
    pub mirror_posture: String,
    /// Fallback behavior token rendered by consumers.
    pub fallback_behavior: String,
    /// Content availability token rendered by consumers.
    pub content_availability: String,
    /// Reason token when content is unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
    /// Whether the row has an explicit user-visible fallback state.
    pub explicit_user_visible_state: bool,
    /// Freshness class for the fallback source.
    pub freshness_class: String,
}

/// Source truth for one beta help-pack item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackSourceTruth {
    /// Source class inherited from docs/help truth vocabulary.
    pub source_class: String,
    /// Source version or release ref.
    pub source_version_ref: String,
    /// Freshness class visible on consuming surfaces.
    pub freshness_class: String,
    /// Version-match state against the active build.
    pub version_match_state: String,
    /// Citation refs preserved for reconstruction.
    pub citation_refs: Vec<String>,
    /// Source strip ref opened from the item.
    pub source_strip_ref: String,
    /// Citation drawer ref opened from the item.
    pub citation_drawer_ref: String,
    /// Release-truth badges rendered by consuming surfaces.
    pub release_truth_badges: OnboardingHelpPackReleaseTruthBadges,
}

/// Release-truth badge metadata shared with docs/help/About surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackReleaseTruthBadges {
    /// Badge family for source class.
    pub source_badge_family: String,
    /// Source token.
    pub source_token: String,
    /// Badge family for version-match state.
    pub version_badge_family: String,
    /// Version token.
    pub version_token: String,
    /// Badge family for freshness class.
    pub freshness_badge_family: String,
    /// Freshness token.
    pub freshness_token: String,
    /// Badge family for client scope.
    pub client_scope_badge_family: String,
    /// Client scope token.
    pub client_scope_token: String,
}

/// Export-safe reconstruction identity for one item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackSupportExportIdentity {
    /// Export-safe support pack item id.
    pub support_pack_item_id: String,
    /// Redaction class for the exported row.
    pub redaction_class: String,
    /// Whether raw item body text may be exported.
    pub raw_body_exported: bool,
    /// Exact reopen ref preserved by support packets.
    pub exact_reopen_ref: String,
    /// Active pack version ref to record in support export.
    pub active_pack_version_ref: String,
    /// Diagnostics row ref that reconstructs fallback state.
    pub diagnostics_row_ref: String,
    /// Whether unresolved locale fallback was active.
    pub unresolved_locale_fallback: bool,
    /// Whether unresolved source fallback was active.
    pub unresolved_source_fallback: bool,
    /// Dismissed state ref captured in bounded diagnostics.
    pub dismissed_state_ref: String,
    /// Helpful state ref captured in bounded diagnostics.
    pub helpful_state_ref: String,
}

/// Portable state refs attached to one item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackItemStateRefs {
    /// Dismissal state row ref.
    pub dismissed_state_ref: String,
    /// Helpfulness state row ref.
    pub helpful_state_ref: String,
}

/// Portable state row for dismissed/helpful item state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaStateRecord {
    /// Stable state item id.
    pub state_item_id: String,
    /// Item this state belongs to.
    pub item_ref: String,
    /// State kind.
    pub state_kind: String,
    /// Storage lane for the state.
    pub storage_lane: String,
    /// Reset class for this state.
    pub reset_class: String,
    /// Export class for this state.
    pub export_class: String,
    /// Diagnostics capture class.
    pub diagnostics_capture_class: String,
    /// Whether the state grants repo mutation.
    pub repo_mutation_allowed: bool,
    /// Whether the state grants default repo read access.
    pub repo_read_access_default: bool,
    /// Whether the state grants default telemetry-style read access.
    pub telemetry_read_default: bool,
}

/// Bounded diagnostics policy for support/export reconstruction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackDiagnosticsPolicy {
    /// Stable diagnostics policy id.
    pub diagnostics_policy_id: String,
    /// Whether active pack versions are recorded.
    pub record_active_pack_version: bool,
    /// Whether unresolved locale fallback is recorded.
    pub record_unresolved_locale_fallback: bool,
    /// Whether unresolved source fallback is recorded.
    pub record_unresolved_source_fallback: bool,
    /// Whether dismissed item state is recorded.
    pub record_dismissed_state: bool,
    /// Whether helpful item state is recorded.
    pub record_helpful_state: bool,
    /// Whether raw article bodies are exported.
    pub raw_body_exported: bool,
    /// Bounded metadata classes allowed in diagnostics.
    pub bounded_material_classes: Vec<String>,
    /// Material classes explicitly omitted from diagnostics.
    pub omitted_material_classes: Vec<String>,
}

/// Protected proof fixture row for the beta manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaProof {
    /// Stable proof id.
    pub proof_id: String,
    /// Fixture path relative to the repository root.
    pub fixture_ref: String,
    /// Item refs exercised by this proof.
    pub exercised_item_refs: Vec<String>,
    /// State names exercised by this proof.
    pub exercised_states: Vec<String>,
}

/// Surface projection for the beta help-pack manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaSurfaceProjection {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source manifest id.
    pub manifest_id: String,
    /// Source manifest version ref.
    pub manifest_version_ref: String,
    /// Surface rows projected from the manifest.
    pub rows: Vec<OnboardingHelpPackBetaSurfaceRow>,
    /// Coverage summary for required surfaces.
    pub coverage: OnboardingHelpPackBetaCoverage,
}

/// One item projected onto one consuming surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaSurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface ref that renders this row.
    pub surface_ref: String,
    /// Stable item id.
    pub item_id: String,
    /// Owning pack id.
    pub pack_id: String,
    /// Owning pack version ref.
    pub pack_version_ref: String,
    /// Stable command id.
    pub command_id: String,
    /// Stable help anchor id.
    pub help_anchor_id: String,
    /// Keyboard route shown to keyboard users.
    pub keyboard_route: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Effective locale.
    pub effective_locale: String,
    /// Locale availability token.
    pub locale_availability: String,
    /// Source-language fallback class token.
    pub source_language_fallback_class: String,
    /// Offline posture token.
    pub offline_posture: String,
    /// Mirror posture token.
    pub mirror_posture: String,
    /// Freshness class token.
    pub freshness_class: String,
    /// Version-match token.
    pub version_match_state: String,
    /// Exact reopen ref preserving pack revision and locale.
    pub exact_reopen_ref: String,
    /// Export-safe support pack item id.
    pub support_pack_item_id: String,
}

/// Coverage summary for beta help-pack surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaCoverage {
    /// Number of item rows.
    pub item_count: usize,
    /// Number of pack ids referenced by items.
    pub pack_count: usize,
    /// Number of command ids referenced by items.
    pub command_count: usize,
    /// Surface refs covered by at least one item.
    pub covered_surface_refs: Vec<String>,
    /// Number of rows with unresolved locale fallback.
    pub unresolved_locale_fallback_count: usize,
    /// Number of rows with unresolved source fallback.
    pub unresolved_source_fallback_count: usize,
}

/// Metadata-only support export for beta help packs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaSupportExport {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Export generation timestamp.
    pub generated_at: String,
    /// Source manifest id.
    pub source_manifest_id: String,
    /// Source manifest version ref.
    pub manifest_version_ref: String,
    /// Diagnostics policy id.
    pub diagnostics_policy_id: String,
    /// Active pack versions captured by support export.
    pub active_pack_versions: Vec<OnboardingHelpPackBetaActivePackVersion>,
    /// Metadata-safe item rows captured by support export.
    pub rows: Vec<OnboardingHelpPackBetaSupportRow>,
    /// Support item ids whose locale or source fallback was unresolved.
    pub unresolved_fallback_rows: Vec<String>,
    /// Material classes omitted from the export.
    pub omitted_material_classes: Vec<String>,
    /// Whether raw article bodies were exported.
    pub raw_body_exported: bool,
}

impl OnboardingHelpPackBetaSupportExport {
    /// Validates support export reconstruction against `manifest`.
    pub fn validate_against_manifest(
        &self,
        manifest: &OnboardingHelpPackBetaManifest,
    ) -> Result<(), Vec<OnboardingHelpPackBetaFinding>> {
        let mut findings = Vec::new();
        if self.record_kind != ONBOARDING_HELP_PACK_BETA_SUPPORT_EXPORT_RECORD_KIND {
            findings.push(OnboardingHelpPackBetaFinding::new(
                self.support_export_id.clone(),
                "support export record_kind is unsupported",
            ));
        }
        if self.schema_version != ONBOARDING_HELP_PACK_BETA_SCHEMA_VERSION {
            findings.push(OnboardingHelpPackBetaFinding::new(
                self.support_export_id.clone(),
                "support export schema version is unsupported",
            ));
        }
        if self.source_manifest_id != manifest.manifest_id {
            findings.push(OnboardingHelpPackBetaFinding::new(
                self.support_export_id.clone(),
                "support export source manifest id drifted",
            ));
        }
        if self.manifest_version_ref != manifest.manifest_version_ref {
            findings.push(OnboardingHelpPackBetaFinding::new(
                self.support_export_id.clone(),
                "support export manifest version ref drifted",
            ));
        }
        if self.raw_body_exported {
            findings.push(OnboardingHelpPackBetaFinding::new(
                self.support_export_id.clone(),
                "support export must not include raw article bodies",
            ));
        }
        if self.rows.len() != manifest.items.len() {
            findings.push(OnboardingHelpPackBetaFinding::new(
                self.support_export_id.clone(),
                "support export row count does not match manifest items",
            ));
        }
        for row in &self.rows {
            let Some(item) = manifest.item(&row.item_id) else {
                findings.push(OnboardingHelpPackBetaFinding::new(
                    row.support_pack_item_id.clone(),
                    "support row references an unknown item",
                ));
                continue;
            };
            if row.pack_id != item.pack_id
                || row.active_pack_version_ref != item.pack_version_ref
                || row.command_id != item.command_hint.command_id
                || row.help_anchor_id != item.command_hint.help_anchor_id
                || row.exact_reopen_ref != item.exact_reopen_ref
                || row.dismissed_state_ref != item.state_refs.dismissed_state_ref
                || row.helpful_state_ref != item.state_refs.helpful_state_ref
            {
                findings.push(OnboardingHelpPackBetaFinding::new(
                    row.support_pack_item_id.clone(),
                    "support row drifted from manifest item",
                ));
            }
            if row.raw_body_exported {
                findings.push(OnboardingHelpPackBetaFinding::new(
                    row.support_pack_item_id.clone(),
                    "support row must not include raw body",
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

/// Active pack version captured by support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaActivePackVersion {
    /// Stable pack id.
    pub pack_id: String,
    /// Active pack version ref.
    pub pack_version_ref: String,
    /// Source version ref.
    pub source_version_ref: String,
    /// Freshness class token.
    pub freshness_class: String,
    /// Version-match token.
    pub version_match_state: String,
    /// Mirror posture token.
    pub mirror_posture: String,
    /// Client scope token.
    pub client_scope_token: String,
}

/// Metadata-safe support row for one item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaSupportRow {
    /// Export-safe support pack item id.
    pub support_pack_item_id: String,
    /// Stable item id.
    pub item_id: String,
    /// Owning pack id.
    pub pack_id: String,
    /// Active pack version ref.
    pub active_pack_version_ref: String,
    /// Stable command id.
    pub command_id: String,
    /// Stable help anchor id.
    pub help_anchor_id: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Effective locale.
    pub effective_locale: String,
    /// Locale availability token.
    pub locale_availability: String,
    /// Source-language fallback class token.
    pub source_language_fallback_class: String,
    /// Whether unresolved locale fallback was active.
    pub unresolved_locale_fallback: bool,
    /// Whether unresolved source fallback was active.
    pub unresolved_source_fallback: bool,
    /// Mirror posture token.
    pub mirror_posture: String,
    /// Content availability token.
    pub content_availability: String,
    /// Source class token.
    pub source_class: String,
    /// Source version ref.
    pub source_version_ref: String,
    /// Freshness class token.
    pub freshness_class: String,
    /// Version-match token.
    pub version_match_state: String,
    /// Citation refs preserved for reconstruction.
    pub citation_refs: Vec<String>,
    /// Exact reopen ref preserving pack revision and locale.
    pub exact_reopen_ref: String,
    /// Diagnostics row ref that reconstructs fallback state.
    pub diagnostics_row_ref: String,
    /// Dismissed state ref captured in bounded diagnostics.
    pub dismissed_state_ref: String,
    /// Helpful state ref captured in bounded diagnostics.
    pub helpful_state_ref: String,
    /// Whether raw article bodies were exported.
    pub raw_body_exported: bool,
}

/// Validation finding for beta help-pack manifests and exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingHelpPackBetaFinding {
    /// Row or record id that failed validation.
    pub row_ref: String,
    /// Validation message.
    pub message: String,
}

impl OnboardingHelpPackBetaFinding {
    fn new(row_ref: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            row_ref: row_ref.into(),
            message: message.into(),
        }
    }
}

/// Returns the seeded beta onboarding/help-pack manifest.
pub fn seeded_onboarding_help_pack_beta_manifest() -> OnboardingHelpPackBetaManifest {
    let first_run = pack(
        "pack:onboarding-help:first-run-beta",
        "pack-rev:onboarding-help:first-run:2026.05.17-01",
        PACK_ROLE_FIRST_RUN,
        Vec::new(),
        SOURCE_PROJECT_DOCS,
        "docs:help:m3:onboarding:first_run",
        "build:aureline:0.0.0-beta.2026.05.17",
        FRESHNESS_AUTHORITATIVE,
        VERSION_EXACT,
        Some("docs-pack:first-run:onboarding:beta".to_owned()),
        MIRROR_NONE,
        INSTALL_LOCAL,
        OFFLINE_LOCAL,
        vec!["en-US", "es-MX"],
    );
    let migration = pack(
        "pack:onboarding-help:migration-beta",
        "pack-rev:onboarding-help:migration:2026.05.17-01",
        PACK_ROLE_MIGRATION,
        Vec::new(),
        SOURCE_PROJECT_DOCS,
        "docs:help:m3:migration_center_beta",
        "build:aureline:0.0.0-beta.2026.05.17",
        FRESHNESS_AUTHORITATIVE,
        VERSION_EXACT,
        Some("docs-pack:migration:center:beta".to_owned()),
        MIRROR_NONE,
        INSTALL_CACHED,
        OFFLINE_CACHED,
        vec!["en-US"],
    );
    let glossary = pack(
        "pack:onboarding-help:glossary-beta",
        "pack-rev:onboarding-help:glossary:2026.05.17-01",
        PACK_ROLE_GLOSSARY,
        vec![
            "glossary:pack:truth_terms:v1".to_owned(),
            "glossary:pack:migration_terms:v1".to_owned(),
        ],
        SOURCE_CURATED_PACK,
        "docs:help:m3:glossary_beta",
        "glossary:aureline:2026.05.17",
        FRESHNESS_WARM_CACHED,
        VERSION_COMPATIBLE,
        Some("docs-pack:glossary:aureline:beta".to_owned()),
        MIRROR_NONE,
        INSTALL_LOCAL,
        OFFLINE_LOCAL,
        vec!["en-US", "es-MX"],
    );
    let mirrored_docs = pack(
        "pack:onboarding-help:docs-mirror-beta",
        "pack-rev:onboarding-help:docs-mirror:2026.05.17-01",
        PACK_ROLE_HELP_OVERLAY,
        Vec::new(),
        SOURCE_MIRRORED_DOCS,
        "mirror:docs-help:aureline:2026.05.17",
        "mirror-snapshot:docs-help:aureline:2026.05.17",
        FRESHNESS_WARM_CACHED,
        VERSION_EXACT,
        Some("docs-pack:mirrored-help:beta".to_owned()),
        MIRROR_VERIFIED,
        INSTALL_MIRROR,
        OFFLINE_MIRROR,
        vec!["en-US"],
    );
    let recovery = pack(
        "pack:onboarding-help:recovery-beta",
        "pack-rev:onboarding-help:recovery:2026.05.17-01",
        PACK_ROLE_RECOVERY,
        Vec::new(),
        SOURCE_SUPPORT_RUNBOOK,
        "docs:help:m3:recovery_first",
        "support-runbook:recovery-first:2026.05.17",
        FRESHNESS_AUTHORITATIVE,
        VERSION_EXACT,
        Some("docs-pack:recovery-first:beta".to_owned()),
        MIRROR_NONE,
        INSTALL_LOCAL,
        OFFLINE_LOCAL,
        vec!["en-US"],
    );
    let guided_missing = pack(
        "pack:onboarding-help:guided-learning-beta",
        "pack-rev:onboarding-help:guided-learning:2026.05.17-01",
        PACK_ROLE_GUIDED,
        Vec::new(),
        SOURCE_CURATED_PACK,
        "docs:help:m3:guided-learning-placeholder",
        "curated-pack:guided-learning:not-installed",
        FRESHNESS_DEGRADED,
        VERSION_COMPATIBLE,
        Some("docs-pack:guided-learning:placeholder:beta".to_owned()),
        MIRROR_NONE,
        INSTALL_NOT_INSTALLED,
        OFFLINE_NOT_AVAILABLE,
        vec!["en-US"],
    );

    let packs = vec![
        first_run.clone(),
        migration.clone(),
        glossary.clone(),
        mirrored_docs.clone(),
        recovery.clone(),
        guided_missing.clone(),
    ];

    let items = vec![
        beta_item(
            "ohp:item:first_run.open_folder",
            "Open local work",
            "Start Center resolves the local folder entry through the workspace open command.",
            "start_center_card",
            &first_run,
            "docs-node:project-entry.open-folder",
            None,
            None,
            vec![SURFACE_START_CENTER, SURFACE_HELP_SEARCH],
            command_hint(
                "cmd:workspace.open_folder",
                Some("cmd-rev:workspace.open_folder:2026.04.21-01"),
                "docs:anchor:workspace:open_folder_overview",
                "Cmd/Ctrl+O",
                "command",
                METADATA_COMMAND_REGISTRY,
            ),
            locale("en-US", "en-US", "en-US", LOCALE_REVIEWED, false, false),
            fallback(FALLBACK_NONE, None, None, false, "source_language_not_needed"),
            offline("local_only", MIRROR_NONE, "local_build_available", OFFLINE_LOCAL, None),
            vec!["citation:docs-pack:project-entry.open-folder:source"],
            RENDERABLE,
        ),
        beta_item(
            "ohp:item:keymap_bridge.command_palette",
            "Command palette bridge",
            "Migration hints keep the imported shortcut mapped to the canonical command palette command.",
            "keymap_bridge",
            &migration,
            "docs-node:onboarding.keymap-bridge",
            None,
            None,
            vec![SURFACE_MIGRATION_CENTER, SURFACE_HELP_SEARCH, SURFACE_CONTEXTUAL_WHY_NOW],
            command_hint(
                "cmd:command_palette.open",
                Some("cmd-rev:command_palette.open:2026.04.22-01"),
                "docs:anchor:onboarding_beta:keymap_bridge",
                "Cmd/Ctrl+Shift+P",
                "command",
                METADATA_COMMAND_REGISTRY,
            ),
            locale("en-US", "es-MX", "en-US", LOCALE_SOURCE_FALLBACK, true, true),
            fallback(
                FALLBACK_SOURCE,
                Some("ohp:item:keymap_bridge.command_palette#en-US"),
                Some("cmd:docs.open_in_browser"),
                true,
                "source_language_fallback_visible",
            ),
            offline("offline_local_safe", MIRROR_NONE, "cached_snapshot_available", OFFLINE_CACHED, None),
            vec![
                "citation:docs-pack:onboarding.keymap-bridge:source",
                "citation:docs-pack:onboarding.keymap-bridge:fallback",
            ],
            RENDERABLE,
        ),
        beta_item(
            "ohp:item:glossary.release_truth",
            "Release-truth terms",
            "Glossary cards define claim, lifecycle, freshness, source, and client-scope terms.",
            "glossary_pack_item",
            &glossary,
            "docs-node:glossary.release-truth",
            Some("glossary:pack:truth_terms:v1"),
            Some("glossary:item:release_truth_terms"),
            vec![SURFACE_MIGRATION_CENTER, SURFACE_HELP_SEARCH],
            command_hint(
                "cmd:docs.open_in_browser",
                Some("cmd-rev:docs.open_in_browser:2026.04.22-01"),
                "docs:anchor:glossary:release_truth_terms",
                "Command palette > Open docs",
                "help_search",
                METADATA_COMMAND_REGISTRY,
            ),
            locale("en-US", "es-MX", "es-MX", LOCALE_REVIEWED, false, false),
            fallback(FALLBACK_NONE, None, None, false, "source_language_not_needed"),
            offline("local_only", MIRROR_NONE, "local_build_available", OFFLINE_LOCAL, None),
            vec![
                "citation:glossary:release_truth_terms:source",
                "citation:docs-help:truth_source_model:source",
            ],
            RENDERABLE,
        ),
        beta_item(
            "ohp:item:glossary.migration_outcomes",
            "Migration vocabulary",
            "Glossary cards name exact, translated, partial, shimmed, and unsupported migration outcomes.",
            "glossary_pack_item",
            &glossary,
            "docs-node:glossary.migration-outcomes",
            Some("glossary:pack:migration_terms:v1"),
            Some("glossary:item:migration_outcomes"),
            vec![SURFACE_MIGRATION_CENTER, SURFACE_HELP_SEARCH],
            command_hint(
                "cmd:docs.open_in_browser",
                Some("cmd-rev:docs.open_in_browser:2026.04.22-01"),
                "docs:anchor:glossary:migration_outcomes",
                "Command palette > Open docs",
                "help_search",
                METADATA_COMMAND_REGISTRY,
            ),
            locale("en-US", "en-US", "en-US", LOCALE_REVIEWED, false, false),
            fallback(FALLBACK_NONE, None, None, false, "source_language_not_needed"),
            offline("local_only", MIRROR_NONE, "local_build_available", OFFLINE_LOCAL, None),
            vec![
                "citation:glossary:migration_outcomes:source",
                "citation:docs-help:migration_center_beta:source",
            ],
            RENDERABLE,
        ),
        beta_item(
            "ohp:item:offline.docs_browser_mirror",
            "Docs mirror fallback",
            "Help search and why-now cards disclose when docs came from the verified mirror snapshot.",
            "contextual_hint",
            &mirrored_docs,
            "docs-node:help.docs-browser.mirror-fallback",
            None,
            None,
            vec![SURFACE_HELP_SEARCH, SURFACE_CONTEXTUAL_WHY_NOW],
            command_hint(
                "cmd:docs.open_in_browser",
                Some("cmd-rev:docs.open_in_browser:2026.04.22-01"),
                "docs:anchor:docs:open_in_browser_overview",
                "Command palette > Open docs",
                "browser_handoff",
                METADATA_COMMAND_REGISTRY,
            ),
            locale("en-US", "en-US", "en-US", LOCALE_REVIEWED, false, false),
            fallback(FALLBACK_NONE, None, None, false, "source_language_not_needed"),
            offline(
                "offline_mirror_safe",
                MIRROR_VERIFIED,
                "verified_mirror_snapshot_disclosed",
                OFFLINE_MIRROR,
                None,
            ),
            vec![
                "citation:docs-browser:mirror-fallback:source",
                "citation:docs-help:release_truth_surfaces:source",
            ],
            RENDERABLE,
        ),
        beta_item(
            "ohp:item:recovery.restore_checkpoint",
            "Restore from checkpoint",
            "Recovery-first help routes restore guidance through the canonical restore command.",
            "recovery_first_item",
            &recovery,
            "docs-node:recovery.restore-checkpoint",
            None,
            None,
            vec![SURFACE_RECOVERY_FIRST, SURFACE_MIGRATION_CENTER, SURFACE_HELP_SEARCH],
            command_hint(
                "cmd:workspace.restore_from_checkpoint",
                Some("cmd-rev:workspace.restore_from_checkpoint:2026.04.22-01"),
                "docs:anchor:workspace:restore_from_checkpoint",
                "Command palette > Restore from checkpoint",
                "command",
                METADATA_COMMAND_REGISTRY,
            ),
            locale("en-US", "en-US", "en-US", LOCALE_REVIEWED, false, false),
            fallback(FALLBACK_NONE, None, None, false, "source_language_not_needed"),
            offline("local_only", MIRROR_NONE, "local_build_available", OFFLINE_LOCAL, None),
            vec![
                "citation:recovery:restore-checkpoint:source",
                "citation:migration:center:rollback-checkpoint:source",
            ],
            RENDERABLE,
        ),
        beta_item(
            "ohp:item:recovery.support_export",
            "Export support evidence",
            "Recovery-first help records active help-pack versions and fallback state before export.",
            "recovery_first_item",
            &recovery,
            "docs-node:recovery.support-export",
            None,
            None,
            vec![SURFACE_RECOVERY_FIRST, SURFACE_MIGRATION_CENTER, SURFACE_HELP_SEARCH],
            command_hint(
                "cmd:support.export_packet",
                None,
                "docs:anchor:support:export_packet",
                "Command palette > Export support evidence",
                "evidence_card",
                METADATA_PACK_DESCRIPTOR,
            ),
            locale("en-US", "en-US", "en-US", LOCALE_REVIEWED, false, false),
            fallback(FALLBACK_NONE, None, None, false, "source_language_not_needed"),
            offline("local_only", MIRROR_NONE, "local_build_available", OFFLINE_LOCAL, None),
            vec![
                "citation:support:export-packet:source",
                "citation:help-pack:diagnostics-policy:source",
            ],
            RENDERABLE,
        ),
        beta_item(
            "ohp:item:learning_digest.not_installed",
            "Guided learning digest placeholder",
            "Help search discloses that the guided learning digest is not installed and keeps local work available.",
            "future_guided_learning_ref",
            &guided_missing,
            "docs-node:onboarding.deep-dive.not-installed",
            None,
            None,
            vec![SURFACE_HELP_SEARCH, SURFACE_CONTEXTUAL_WHY_NOW],
            command_hint(
                "cmd:help.search",
                None,
                "docs:anchor:onboarding_beta:learning_digest_not_installed",
                "Cmd/Ctrl+Shift+H",
                "help_search",
                METADATA_PACK_DESCRIPTOR,
            ),
            locale("en-US", "en-US", "en-US", LOCALE_NOT_INSTALLED, false, false),
            fallback(
                FALLBACK_PACK_MISSING,
                None,
                None,
                false,
                "pack_missing_placeholder_visible",
            ),
            offline(
                "offline_local_safe",
                MIRROR_NONE,
                "not_installed_placeholder_disclosed",
                OFFLINE_NOT_AVAILABLE,
                Some("guided_learning_pack_not_installed"),
            ),
            vec!["citation:help-pack:guided-learning-placeholder:source"],
            "blocked_not_installed",
        ),
    ];

    OnboardingHelpPackBetaManifest {
        record_kind: ONBOARDING_HELP_PACK_BETA_RECORD_KIND.to_owned(),
        schema_version: ONBOARDING_HELP_PACK_BETA_SCHEMA_VERSION,
        manifest_id: ONBOARDING_HELP_PACK_BETA_MANIFEST_ID.to_owned(),
        manifest_version_ref: ONBOARDING_HELP_PACK_BETA_VERSION_REF.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        release_channel: "beta".to_owned(),
        source_contract_refs: BTreeMap::from([
            (
                "schema".to_owned(),
                ONBOARDING_HELP_PACK_BETA_SCHEMA_REF.to_owned(),
            ),
            (
                "command_registry".to_owned(),
                "artifacts/commands/command_registry_seed.yaml".to_owned(),
            ),
            (
                "truth_source_model".to_owned(),
                "docs/help/truth_source_model.md".to_owned(),
            ),
            (
                "migration_center_contract".to_owned(),
                "docs/help/m3/migration_center_beta.md".to_owned(),
            ),
            (
                "release_packet".to_owned(),
                ONBOARDING_HELP_PACK_BETA_RELEASE_PACKET_REF.to_owned(),
            ),
        ]),
        runtime_consumer_refs: vec![
            SURFACE_START_CENTER.to_owned(),
            SURFACE_MIGRATION_CENTER.to_owned(),
            SURFACE_HELP_SEARCH.to_owned(),
            SURFACE_CONTEXTUAL_WHY_NOW.to_owned(),
            SURFACE_RECOVERY_FIRST.to_owned(),
            "support_export:onboarding_help_pack_beta".to_owned(),
        ],
        required_surface_classes: vec![
            SURFACE_START_CENTER.to_owned(),
            SURFACE_MIGRATION_CENTER.to_owned(),
            SURFACE_HELP_SEARCH.to_owned(),
            SURFACE_CONTEXTUAL_WHY_NOW.to_owned(),
            SURFACE_RECOVERY_FIRST.to_owned(),
        ],
        state_records: state_records_for_items(&items),
        packs,
        items,
        diagnostics_policy: OnboardingHelpPackDiagnosticsPolicy {
            diagnostics_policy_id: "diagnostics-policy:onboarding-help-pack:beta:v1".to_owned(),
            record_active_pack_version: true,
            record_unresolved_locale_fallback: true,
            record_unresolved_source_fallback: true,
            record_dismissed_state: true,
            record_helpful_state: true,
            raw_body_exported: false,
            bounded_material_classes: vec![
                "pack_id".to_owned(),
                "item_id".to_owned(),
                "command_id".to_owned(),
                "locale_fallback_state".to_owned(),
                "source_fallback_state".to_owned(),
                "dismissed_state_token".to_owned(),
                "helpful_state_token".to_owned(),
            ],
            omitted_material_classes: vec![
                "raw_article_body".to_owned(),
                "raw_help_search_query".to_owned(),
                "private_workspace_path".to_owned(),
                "account_identifier".to_owned(),
            ],
        },
        protected_proofs: vec![
            OnboardingHelpPackBetaProof {
                proof_id: "proof:onboarding-help-pack-beta:locale-source-fallback".to_owned(),
                fixture_ref: "fixtures/help/m3/onboarding_help_packs/source_language_fallback.yaml"
                    .to_owned(),
                exercised_item_refs: vec!["ohp:item:keymap_bridge.command_palette".to_owned()],
                exercised_states: vec![
                    LOCALE_SOURCE_FALLBACK.to_owned(),
                    FALLBACK_SOURCE.to_owned(),
                    "command_route_preserved".to_owned(),
                ],
            },
            OnboardingHelpPackBetaProof {
                proof_id: "proof:onboarding-help-pack-beta:offline-mirror-fallback".to_owned(),
                fixture_ref: "fixtures/help/m3/onboarding_help_packs/offline_mirror_fallback.yaml"
                    .to_owned(),
                exercised_item_refs: vec!["ohp:item:offline.docs_browser_mirror".to_owned()],
                exercised_states: vec![
                    MIRROR_VERIFIED.to_owned(),
                    OFFLINE_MIRROR.to_owned(),
                    FRESHNESS_WARM_CACHED.to_owned(),
                ],
            },
            OnboardingHelpPackBetaProof {
                proof_id: "proof:onboarding-help-pack-beta:support-export-reconstruction"
                    .to_owned(),
                fixture_ref: ONBOARDING_HELP_PACK_BETA_SUPPORT_EXPORT_FIXTURE_REF.to_owned(),
                exercised_item_refs: vec![
                    "ohp:item:first_run.open_folder".to_owned(),
                    "ohp:item:keymap_bridge.command_palette".to_owned(),
                    "ohp:item:recovery.support_export".to_owned(),
                ],
                exercised_states: vec![
                    "active_pack_version_recorded".to_owned(),
                    "fallback_state_recorded".to_owned(),
                    "raw_body_not_exported".to_owned(),
                ],
            },
        ],
    }
}

/// Returns the seeded surface projection.
pub fn seeded_onboarding_help_pack_beta_surface_projection(
) -> OnboardingHelpPackBetaSurfaceProjection {
    seeded_onboarding_help_pack_beta_manifest().surface_projection()
}

/// Returns the seeded support export.
pub fn seeded_onboarding_help_pack_beta_support_export() -> OnboardingHelpPackBetaSupportExport {
    seeded_onboarding_help_pack_beta_manifest()
        .support_export("support-export:onboarding-help-pack-beta:001", GENERATED_AT)
}

/// Validates all seeded beta help-pack records against the command registry.
pub fn validate_seeded_onboarding_help_pack_beta(
    registry: &CommandRegistry,
) -> Result<(), Vec<OnboardingHelpPackBetaFinding>> {
    let manifest = seeded_onboarding_help_pack_beta_manifest();
    let export =
        manifest.support_export("support-export:onboarding-help-pack-beta:001", GENERATED_AT);
    let mut findings = Vec::new();
    if let Err(mut manifest_findings) = manifest.validate_against_registry(registry) {
        findings.append(&mut manifest_findings);
    }
    if let Err(mut export_findings) = export.validate_against_manifest(&manifest) {
        findings.append(&mut export_findings);
    }
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn pack(
    pack_id: &str,
    pack_version_ref: &str,
    pack_role: &str,
    glossary_pack_ids: Vec<String>,
    source_class: &str,
    source_ref: &str,
    source_version_ref: &str,
    freshness_class: &str,
    version_match_state: &str,
    docs_pack_ref: Option<String>,
    mirror_posture: &str,
    install_state: &str,
    offline_posture: &str,
    available_locales: Vec<&str>,
) -> OnboardingHelpPackBetaPack {
    OnboardingHelpPackBetaPack {
        pack_id: pack_id.to_owned(),
        pack_version_ref: pack_version_ref.to_owned(),
        pack_role: pack_role.to_owned(),
        glossary_pack_ids,
        source_class: source_class.to_owned(),
        source_ref: source_ref.to_owned(),
        source_version_ref: source_version_ref.to_owned(),
        freshness_class: freshness_class.to_owned(),
        version_match_state: version_match_state.to_owned(),
        docs_pack_ref,
        mirror_posture: mirror_posture.to_owned(),
        install_state: install_state.to_owned(),
        offline_posture: offline_posture.to_owned(),
        source_locale: "en-US".to_owned(),
        available_locales: available_locales.into_iter().map(str::to_owned).collect(),
        locale_overlay_refs: vec!["locale-overlay:onboarding-help:en-US".to_owned()],
        citation_availability: "available".to_owned(),
        release_truth_badges: badges(source_class, version_match_state, freshness_class),
        client_scope: "desktop_product".to_owned(),
        support_owner_ref: "owner:docs-help-learnability".to_owned(),
    }
}

fn badges(
    source_class: &str,
    version_match_state: &str,
    freshness_class: &str,
) -> OnboardingHelpPackReleaseTruthBadges {
    OnboardingHelpPackReleaseTruthBadges {
        source_badge_family: "docs_help_source_class".to_owned(),
        source_token: source_class.to_owned(),
        version_badge_family: "docs_help_version_match_state".to_owned(),
        version_token: version_match_state.to_owned(),
        freshness_badge_family: "docs_help_freshness_class".to_owned(),
        freshness_token: freshness_class.to_owned(),
        client_scope_badge_family: "client_scope_badge_family".to_owned(),
        client_scope_token: "desktop_product".to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn beta_item(
    item_id: &str,
    title: &str,
    summary: &str,
    item_kind: &str,
    pack: &OnboardingHelpPackBetaPack,
    docs_node_id: &str,
    glossary_pack_id: Option<&str>,
    glossary_item_id: Option<&str>,
    surface_refs: Vec<&str>,
    command_hint: OnboardingHelpPackCommandHint,
    locale: OnboardingHelpPackLocaleState,
    source_language_fallback: OnboardingHelpPackSourceLanguageFallback,
    offline_fallback: OnboardingHelpPackOfflineFallback,
    citation_refs: Vec<&str>,
    content_render_state: &str,
) -> OnboardingHelpPackBetaItem {
    let exact_reopen_ref = format!(
        "reopen:{}@{}#{}",
        docs_node_id, pack.pack_version_ref, locale.effective_locale
    );
    let state_refs = OnboardingHelpPackItemStateRefs {
        dismissed_state_ref: state_ref(item_id, "dismissed"),
        helpful_state_ref: state_ref(item_id, "helpful"),
    };
    OnboardingHelpPackBetaItem {
        item_id: item_id.to_owned(),
        item_kind: item_kind.to_owned(),
        title: title.to_owned(),
        summary: summary.to_owned(),
        pack_id: pack.pack_id.clone(),
        pack_version_ref: pack.pack_version_ref.clone(),
        glossary_pack_id: glossary_pack_id.map(str::to_owned),
        glossary_item_id: glossary_item_id.map(str::to_owned),
        docs_node_id: docs_node_id.to_owned(),
        surface_refs: surface_refs.into_iter().map(str::to_owned).collect(),
        command_hint,
        locale,
        source_language_fallback,
        offline_fallback,
        source_truth: OnboardingHelpPackSourceTruth {
            source_class: pack.source_class.clone(),
            source_version_ref: pack.source_version_ref.clone(),
            freshness_class: pack.freshness_class.clone(),
            version_match_state: pack.version_match_state.clone(),
            citation_refs: citation_refs.into_iter().map(str::to_owned).collect(),
            source_strip_ref: format!("source-strip:{}", docs_node_id),
            citation_drawer_ref: format!("citation-drawer:{}", docs_node_id),
            release_truth_badges: pack.release_truth_badges.clone(),
        },
        content_render_state: content_render_state.to_owned(),
        exact_reopen_ref: exact_reopen_ref.clone(),
        support_export_identity: OnboardingHelpPackSupportExportIdentity {
            support_pack_item_id: format!("support:{}", item_id.replace("ohp:item:", "")),
            redaction_class: "metadata_safe_default".to_owned(),
            raw_body_exported: false,
            exact_reopen_ref,
            active_pack_version_ref: pack.pack_version_ref.clone(),
            diagnostics_row_ref: format!("diagnostics:{}", item_id.replace("ohp:item:", "")),
            unresolved_locale_fallback: false,
            unresolved_source_fallback: false,
            dismissed_state_ref: state_refs.dismissed_state_ref.clone(),
            helpful_state_ref: state_refs.helpful_state_ref.clone(),
        },
        state_refs,
    }
    .with_support_fallback_flags()
}

impl OnboardingHelpPackBetaItem {
    fn with_support_fallback_flags(mut self) -> Self {
        self.support_export_identity.unresolved_locale_fallback =
            self.locale.unresolved_locale_fallback;
        self.support_export_identity.unresolved_source_fallback =
            self.source_language_fallback.unresolved_source_fallback;
        self
    }
}

fn command_hint(
    command_id: &str,
    command_revision_ref: Option<&str>,
    help_anchor_id: &str,
    keyboard_route: &str,
    route_kind: &str,
    metadata_source: &str,
) -> OnboardingHelpPackCommandHint {
    OnboardingHelpPackCommandHint {
        command_id: command_id.to_owned(),
        command_revision_ref: command_revision_ref.map(str::to_owned),
        help_anchor_id: help_anchor_id.to_owned(),
        keyboard_route: keyboard_route.to_owned(),
        route_kind: route_kind.to_owned(),
        metadata_source: metadata_source.to_owned(),
        canonical_action_graph_ref: format!("action-graph:{command_id}"),
    }
}

fn locale(
    source_locale: &str,
    requested_locale: &str,
    effective_locale: &str,
    locale_availability: &str,
    stale_translation_marker: bool,
    unresolved_locale_fallback: bool,
) -> OnboardingHelpPackLocaleState {
    OnboardingHelpPackLocaleState {
        source_locale: source_locale.to_owned(),
        requested_locale: requested_locale.to_owned(),
        effective_locale: effective_locale.to_owned(),
        locale_availability: locale_availability.to_owned(),
        fallback_chain: if requested_locale == effective_locale {
            vec![requested_locale.to_owned()]
        } else {
            vec![
                requested_locale.to_owned(),
                "es".to_owned(),
                effective_locale.to_owned(),
            ]
        },
        stale_translation_marker,
        unresolved_locale_fallback,
        translation_pack_ref: if requested_locale == "es-MX" {
            Some("locale-pack:onboarding-help:es-MX:2026.05".to_owned())
        } else {
            None
        },
    }
}

fn fallback(
    fallback_class: &str,
    source_language_item_ref: Option<&str>,
    escape_hatch_command_id: Option<&str>,
    unresolved_source_fallback: bool,
    user_visible_status: &str,
) -> OnboardingHelpPackSourceLanguageFallback {
    OnboardingHelpPackSourceLanguageFallback {
        fallback_class: fallback_class.to_owned(),
        source_language_item_ref: source_language_item_ref.map(str::to_owned),
        escape_hatch_command_id: escape_hatch_command_id.map(str::to_owned),
        unresolved_source_fallback,
        user_visible_status: user_visible_status.to_owned(),
    }
}

fn offline(
    connectivity_posture: &str,
    mirror_posture: &str,
    fallback_behavior: &str,
    content_availability: &str,
    unavailable_reason: Option<&str>,
) -> OnboardingHelpPackOfflineFallback {
    OnboardingHelpPackOfflineFallback {
        connectivity_posture: connectivity_posture.to_owned(),
        mirror_posture: mirror_posture.to_owned(),
        fallback_behavior: fallback_behavior.to_owned(),
        content_availability: content_availability.to_owned(),
        unavailable_reason: unavailable_reason.map(str::to_owned),
        explicit_user_visible_state: true,
        freshness_class: match content_availability {
            OFFLINE_MIRROR | OFFLINE_CACHED => FRESHNESS_WARM_CACHED.to_owned(),
            OFFLINE_NOT_AVAILABLE => FRESHNESS_DEGRADED.to_owned(),
            _ => FRESHNESS_AUTHORITATIVE.to_owned(),
        },
    }
}

fn state_records_for_items(
    items: &[OnboardingHelpPackBetaItem],
) -> Vec<OnboardingHelpPackBetaStateRecord> {
    let mut records = Vec::new();
    for item in items {
        records.push(state_record(
            &item.state_refs.dismissed_state_ref,
            &item.item_id,
            STATE_DISMISSED,
        ));
        records.push(state_record(
            &item.state_refs.helpful_state_ref,
            &item.item_id,
            STATE_HELPFUL,
        ));
    }
    records
}

fn state_record(
    state_item_id: &str,
    item_ref: &str,
    state_kind: &str,
) -> OnboardingHelpPackBetaStateRecord {
    OnboardingHelpPackBetaStateRecord {
        state_item_id: state_item_id.to_owned(),
        item_ref: item_ref.to_owned(),
        state_kind: state_kind.to_owned(),
        storage_lane: STORAGE_PROFILE.to_owned(),
        reset_class: RESET_PER_PROFILE.to_owned(),
        export_class: EXPORT_SUPPORT_REDACTED.to_owned(),
        diagnostics_capture_class: "bounded_boolean_state_only".to_owned(),
        repo_mutation_allowed: false,
        repo_read_access_default: false,
        telemetry_read_default: false,
    }
}

fn state_ref(item_id: &str, suffix: &str) -> String {
    format!(
        "state:onboarding-help:{}.{}",
        item_id.trim_start_matches("ohp:item:"),
        suffix
    )
}

fn validate_pack(
    pack: &OnboardingHelpPackBetaPack,
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    require_token(
        &pack.pack_id,
        "pack_role",
        &pack.pack_role,
        &[
            PACK_ROLE_FIRST_RUN,
            PACK_ROLE_MIGRATION,
            PACK_ROLE_GLOSSARY,
            PACK_ROLE_GUIDED,
            PACK_ROLE_RECOVERY,
            PACK_ROLE_HELP_OVERLAY,
        ],
        findings,
    );
    require_token(
        &pack.pack_id,
        "source_class",
        &pack.source_class,
        &[
            SOURCE_PROJECT_DOCS,
            SOURCE_MIRRORED_DOCS,
            SOURCE_CURATED_PACK,
            SOURCE_SUPPORT_RUNBOOK,
        ],
        findings,
    );
    require_token(
        &pack.pack_id,
        "freshness_class",
        &pack.freshness_class,
        &[
            FRESHNESS_AUTHORITATIVE,
            FRESHNESS_WARM_CACHED,
            FRESHNESS_DEGRADED,
        ],
        findings,
    );
    require_token(
        &pack.pack_id,
        "version_match_state",
        &pack.version_match_state,
        &[VERSION_EXACT, VERSION_COMPATIBLE],
        findings,
    );
    require_token(
        &pack.pack_id,
        "install_state",
        &pack.install_state,
        &[
            INSTALL_LOCAL,
            INSTALL_CACHED,
            INSTALL_MIRROR,
            INSTALL_NOT_INSTALLED,
        ],
        findings,
    );
    require_token(
        &pack.pack_id,
        "offline_posture",
        &pack.offline_posture,
        &[
            OFFLINE_LOCAL,
            OFFLINE_CACHED,
            OFFLINE_MIRROR,
            OFFLINE_NOT_AVAILABLE,
        ],
        findings,
    );
    if !pack.available_locales.contains(&pack.source_locale) {
        findings.push(OnboardingHelpPackBetaFinding::new(
            pack.pack_id.clone(),
            "pack available_locales does not include source_locale",
        ));
    }
    if pack.pack_role == PACK_ROLE_GLOSSARY && pack.glossary_pack_ids.is_empty() {
        findings.push(OnboardingHelpPackBetaFinding::new(
            pack.pack_id.clone(),
            "glossary pack must declare glossary_pack_ids",
        ));
    }
    if pack.install_state == INSTALL_NOT_INSTALLED && pack.offline_posture != OFFLINE_NOT_AVAILABLE
    {
        findings.push(OnboardingHelpPackBetaFinding::new(
            pack.pack_id.clone(),
            "not-installed pack must declare not_available_offline",
        ));
    }
    if pack.install_state == INSTALL_MIRROR && pack.mirror_posture != MIRROR_VERIFIED {
        findings.push(OnboardingHelpPackBetaFinding::new(
            pack.pack_id.clone(),
            "mirror-only pack must declare verified_mirror_snapshot",
        ));
    }
    validate_badges(&pack.pack_id, &pack.release_truth_badges, findings);
}

fn validate_item(
    manifest: &OnboardingHelpPackBetaManifest,
    registry: &CommandRegistry,
    item: &OnboardingHelpPackBetaItem,
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    let Some(pack) = manifest.pack(&item.pack_id) else {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "item references unknown pack",
        ));
        return;
    };
    if item.pack_version_ref != pack.pack_version_ref {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "item pack_version_ref drifted from owning pack",
        ));
    }
    if item.surface_refs.is_empty() {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "item must declare at least one surface_ref",
        ));
    }
    for surface_ref in &item.surface_refs {
        if ![
            SURFACE_START_CENTER,
            SURFACE_MIGRATION_CENTER,
            SURFACE_HELP_SEARCH,
            SURFACE_CONTEXTUAL_WHY_NOW,
            SURFACE_RECOVERY_FIRST,
        ]
        .contains(&surface_ref.as_str())
        {
            findings.push(OnboardingHelpPackBetaFinding::new(
                item.item_id.clone(),
                format!("unsupported surface_ref {surface_ref}"),
            ));
        }
    }

    validate_command_hint(registry, item, findings);
    validate_locale(item, findings);
    validate_offline(item, findings);
    validate_source_truth(item, findings);
    validate_support_identity(manifest, item, findings);
}

fn validate_command_hint(
    registry: &CommandRegistry,
    item: &OnboardingHelpPackBetaItem,
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    if item.command_hint.command_id.trim().is_empty()
        || item.command_hint.keyboard_route.trim().is_empty()
        || item.command_hint.help_anchor_id.trim().is_empty()
        || item
            .command_hint
            .canonical_action_graph_ref
            .trim()
            .is_empty()
    {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "command hint is missing command id, keyboard route, help anchor, or action graph ref",
        ));
    }
    match item.command_hint.metadata_source.as_str() {
        METADATA_COMMAND_REGISTRY => match registry.get(&item.command_hint.command_id) {
            Some(entry) => {
                if item.command_hint.command_revision_ref.as_deref()
                    != Some(entry.descriptor.command_revision_ref.as_str())
                {
                    findings.push(OnboardingHelpPackBetaFinding::new(
                        item.item_id.clone(),
                        "registry-backed command revision ref drifted",
                    ));
                }
            }
            None => findings.push(OnboardingHelpPackBetaFinding::new(
                item.item_id.clone(),
                "registry-backed item references unknown command_id",
            )),
        },
        METADATA_PACK_DESCRIPTOR => {
            if !item.command_hint.command_id.starts_with("cmd:") {
                findings.push(OnboardingHelpPackBetaFinding::new(
                    item.item_id.clone(),
                    "pack-owned command must still use a stable cmd: id",
                ));
            }
        }
        _ => findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "unsupported command metadata_source",
        )),
    }
}

fn validate_locale(
    item: &OnboardingHelpPackBetaItem,
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    require_token(
        &item.item_id,
        "locale_availability",
        &item.locale.locale_availability,
        &[
            LOCALE_REVIEWED,
            LOCALE_STALE,
            LOCALE_SOURCE_FALLBACK,
            LOCALE_NOT_INSTALLED,
        ],
        findings,
    );
    require_token(
        &item.item_id,
        "fallback_class",
        &item.source_language_fallback.fallback_class,
        &[FALLBACK_NONE, FALLBACK_SOURCE, FALLBACK_PACK_MISSING],
        findings,
    );
    if item.locale.locale_availability == LOCALE_SOURCE_FALLBACK {
        if item.source_language_fallback.fallback_class != FALLBACK_SOURCE
            || item
                .source_language_fallback
                .source_language_item_ref
                .as_deref()
                .unwrap_or("")
                .is_empty()
            || item.locale.effective_locale != item.locale.source_locale
        {
            findings.push(OnboardingHelpPackBetaFinding::new(
                item.item_id.clone(),
                "source-language fallback must disclose source item and render source locale",
            ));
        }
    }
    if item.locale.unresolved_locale_fallback
        && item.source_language_fallback.fallback_class == FALLBACK_NONE
    {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "unresolved locale fallback must not use no_fallback class",
        ));
    }
}

fn validate_offline(
    item: &OnboardingHelpPackBetaItem,
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    if !item.offline_fallback.explicit_user_visible_state {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "offline fallback must be user-visible",
        ));
    }
    if item.offline_fallback.content_availability == OFFLINE_NOT_AVAILABLE
        && item.content_render_state == RENDERABLE
    {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "unavailable offline content must not render as full content",
        ));
    }
    if item.offline_fallback.mirror_posture == MIRROR_VERIFIED
        && item.offline_fallback.content_availability != OFFLINE_MIRROR
    {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "verified mirror posture must declare mirror offline availability",
        ));
    }
}

fn validate_source_truth(
    item: &OnboardingHelpPackBetaItem,
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    if item.source_truth.citation_refs.is_empty() && item.content_render_state == RENDERABLE {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "renderable item must preserve citation refs",
        ));
    }
    if item.source_truth.source_version_ref.trim().is_empty()
        || item.source_truth.freshness_class.trim().is_empty()
        || item.source_truth.version_match_state.trim().is_empty()
    {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "item source truth must include version, freshness, and version match",
        ));
    }
    validate_badges(
        &item.item_id,
        &item.source_truth.release_truth_badges,
        findings,
    );
}

fn validate_support_identity(
    manifest: &OnboardingHelpPackBetaManifest,
    item: &OnboardingHelpPackBetaItem,
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    if item.support_export_identity.raw_body_exported {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "support export must not include raw item body",
        ));
    }
    if item.support_export_identity.exact_reopen_ref != item.exact_reopen_ref {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "support exact reopen ref drifted from item",
        ));
    }
    if item.support_export_identity.active_pack_version_ref != item.pack_version_ref {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "support active pack version ref drifted from item",
        ));
    }
    if item.support_export_identity.unresolved_locale_fallback
        != item.locale.unresolved_locale_fallback
        || item.support_export_identity.unresolved_source_fallback
            != item.source_language_fallback.unresolved_source_fallback
    {
        findings.push(OnboardingHelpPackBetaFinding::new(
            item.item_id.clone(),
            "support fallback flags drifted from item fallback state",
        ));
    }
    for state_ref in [
        &item.state_refs.dismissed_state_ref,
        &item.state_refs.helpful_state_ref,
        &item.support_export_identity.dismissed_state_ref,
        &item.support_export_identity.helpful_state_ref,
    ] {
        match manifest.state_record(state_ref) {
            Some(state) if state.item_ref == item.item_id => {}
            Some(_) => findings.push(OnboardingHelpPackBetaFinding::new(
                item.item_id.clone(),
                "item state ref points at another item",
            )),
            None => findings.push(OnboardingHelpPackBetaFinding::new(
                item.item_id.clone(),
                "item state ref cannot be resolved",
            )),
        }
    }
}

fn validate_state(
    state: &OnboardingHelpPackBetaStateRecord,
    item_ids: &BTreeSet<&str>,
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    if !item_ids.contains(state.item_ref.as_str()) {
        findings.push(OnboardingHelpPackBetaFinding::new(
            state.state_item_id.clone(),
            "state record references unknown item",
        ));
    }
    if state.storage_lane != STORAGE_PROFILE {
        findings.push(OnboardingHelpPackBetaFinding::new(
            state.state_item_id.clone(),
            "state must live in portable user profile state",
        ));
    }
    if state.repo_mutation_allowed || state.repo_read_access_default || state.telemetry_read_default
    {
        findings.push(OnboardingHelpPackBetaFinding::new(
            state.state_item_id.clone(),
            "state grants hidden repo or telemetry access",
        ));
    }
}

fn validate_diagnostics_policy(
    policy: &OnboardingHelpPackDiagnosticsPolicy,
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    if !policy.record_active_pack_version
        || !policy.record_unresolved_locale_fallback
        || !policy.record_unresolved_source_fallback
        || !policy.record_dismissed_state
        || !policy.record_helpful_state
    {
        findings.push(OnboardingHelpPackBetaFinding::new(
            policy.diagnostics_policy_id.clone(),
            "diagnostics policy must record active version, fallback, dismissed, and helpful state",
        ));
    }
    if policy.raw_body_exported {
        findings.push(OnboardingHelpPackBetaFinding::new(
            policy.diagnostics_policy_id.clone(),
            "diagnostics policy must omit raw article bodies",
        ));
    }
}

fn validate_badges(
    row_ref: &str,
    badges: &OnboardingHelpPackReleaseTruthBadges,
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    if badges.source_badge_family != "docs_help_source_class"
        || badges.version_badge_family != "docs_help_version_match_state"
        || badges.freshness_badge_family != "docs_help_freshness_class"
        || badges.client_scope_badge_family != "client_scope_badge_family"
        || badges.source_token.trim().is_empty()
        || badges.version_token.trim().is_empty()
        || badges.freshness_token.trim().is_empty()
        || badges.client_scope_token.trim().is_empty()
    {
        findings.push(OnboardingHelpPackBetaFinding::new(
            row_ref.to_owned(),
            "release-truth badges must reuse docs/help badge families",
        ));
    }
}

fn require_token(
    row_ref: &str,
    field: &str,
    token: &str,
    allowed: &[&str],
    findings: &mut Vec<OnboardingHelpPackBetaFinding>,
) {
    if !allowed.contains(&token) {
        findings.push(OnboardingHelpPackBetaFinding::new(
            row_ref.to_owned(),
            format!("{field} has unsupported token {token}"),
        ));
    }
}

#[cfg(test)]
mod tests {
    use aureline_commands::registry::seeded_registry;

    use super::*;

    #[test]
    fn seeded_manifest_validates() {
        let manifest = seeded_onboarding_help_pack_beta_manifest();
        manifest
            .validate_against_registry(seeded_registry())
            .expect("seeded beta help pack validates");
    }

    #[test]
    fn seeded_manifest_covers_required_surfaces() {
        let manifest = seeded_onboarding_help_pack_beta_manifest();
        let coverage = manifest.surface_coverage();
        for surface in &manifest.required_surface_classes {
            assert!(coverage.covered_surface_refs.contains(surface));
        }
        assert!(manifest
            .items_for_surface(SURFACE_CONTEXTUAL_WHY_NOW)
            .iter()
            .any(|item| item.item_id == "ohp:item:keymap_bridge.command_palette"));
        assert!(manifest
            .items_for_surface(SURFACE_RECOVERY_FIRST)
            .iter()
            .any(|item| item.item_id == "ohp:item:recovery.support_export"));
    }

    #[test]
    fn support_export_reconstructs_fallback_and_state() {
        let manifest = seeded_onboarding_help_pack_beta_manifest();
        let export =
            manifest.support_export("support-export:onboarding-help-pack-beta:001", GENERATED_AT);
        export
            .validate_against_manifest(&manifest)
            .expect("support export validates");
        assert!(!export.raw_body_exported);
        assert!(export
            .unresolved_fallback_rows
            .contains(&"support:keymap_bridge.command_palette".to_owned()));
        assert!(export.rows.iter().all(|row| {
            !row.dismissed_state_ref.is_empty()
                && !row.helpful_state_ref.is_empty()
                && !row.active_pack_version_ref.is_empty()
        }));
    }

    #[test]
    fn locale_fallback_keeps_command_and_citations() {
        let manifest = seeded_onboarding_help_pack_beta_manifest();
        let item = manifest
            .item("ohp:item:keymap_bridge.command_palette")
            .expect("keymap item exists");
        assert_eq!(item.command_hint.command_id, "cmd:command_palette.open");
        assert_eq!(item.locale.requested_locale, "es-MX");
        assert_eq!(item.locale.effective_locale, "en-US");
        assert_eq!(
            item.source_language_fallback.fallback_class,
            FALLBACK_SOURCE
        );
        assert!(!item.source_truth.citation_refs.is_empty());
    }
}
