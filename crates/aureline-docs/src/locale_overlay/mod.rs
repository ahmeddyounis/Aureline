//! Locale-overlay contracts for translated docs and learnability packs.
//!
//! This module owns the metadata layer that sits between docs/help pack
//! identity and localized rendering. It records which source revision a
//! translation was based on, which overlay revision rendered, whether the
//! translation is complete, partial, stale, or falling back to source language,
//! and which citation, command, keyboard, and scope anchors survived the
//! translation pass.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::citations::DocsFreshnessClass;

/// Schema version shared by locale-overlay beta records.
pub const LOCALE_OVERLAY_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`LocaleOverlayContract`].
pub const LOCALE_OVERLAY_CONTRACT_RECORD_KIND: &str = "locale_overlay_contract_record";

/// Stable record kind for [`LocaleOverlayRecord`].
pub const LOCALE_OVERLAY_RECORD_KIND: &str = "locale_overlay_record";

/// Stable record kind for [`LocaleOverlaySurfaceProjection`].
pub const LOCALE_OVERLAY_SURFACE_PROJECTION_RECORD_KIND: &str =
    "locale_overlay_surface_projection_record";

/// Stable record kind for [`LocaleOverlaySupportExport`].
pub const LOCALE_OVERLAY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "locale_overlay_support_export_record";

/// Stable id for the seeded translated-pack overlay contract.
pub const TRANSLATED_PACK_LOCALE_OVERLAY_CONTRACT_ID: &str =
    "locale-overlay:translated-learnability:beta:v1";

/// Stable version ref for the seeded translated-pack overlay contract.
pub const TRANSLATED_PACK_LOCALE_OVERLAY_VERSION_REF: &str =
    "locale-overlay-rev:translated-learnability:2026.05.18-01";

/// Repository-relative schema ref for locale-overlay records.
pub const LOCALE_OVERLAY_SCHEMA_REF: &str = "schemas/docs/locale_overlay.schema.json";

/// Repository-relative fixture ref for the seeded locale-overlay manifest.
pub const LOCALE_OVERLAY_FIXTURE_REF: &str =
    "fixtures/docs/m3/translated_pack_parity/manifest.json";

/// Repository-relative surface-projection fixture ref.
pub const LOCALE_OVERLAY_SURFACE_FIXTURE_REF: &str =
    "fixtures/docs/m3/translated_pack_parity/surface_projection.json";

/// Repository-relative support-export fixture ref.
pub const LOCALE_OVERLAY_SUPPORT_EXPORT_FIXTURE_REF: &str =
    "fixtures/docs/m3/translated_pack_parity/support_export.json";

/// Stable user-facing action label for source-language continuity.
pub const OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL: &str = "Open in source language";

const GENERATED_AT: &str = "2026-05-18T19:20:00Z";
const SOURCE_LOCALE: &str = "en-US";
const SOURCE_LANGUAGE_COMMAND_ID: &str = "cmd:docs.open_in_browser";

/// Pack family that may receive a locale overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleOverlayPackKind {
    /// Versioned docs-pack content.
    DocsPack,
    /// Product help or Help/About pack content.
    HelpPack,
    /// Glossary or terminology pack content.
    GlossaryPack,
    /// Onboarding or first-run teaching pack content.
    OnboardingPack,
    /// Guided-tour or learning-mode pack content.
    GuidedTourPack,
    /// Troubleshooting, recovery, or support-runbook help content.
    TroubleshootingHelpPack,
}

impl LocaleOverlayPackKind {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPack => "docs_pack",
            Self::HelpPack => "help_pack",
            Self::GlossaryPack => "glossary_pack",
            Self::OnboardingPack => "onboarding_pack",
            Self::GuidedTourPack => "guided_tour_pack",
            Self::TroubleshootingHelpPack => "troubleshooting_help_pack",
        }
    }
}

/// Translation coverage represented by one locale overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleOverlayCoverageState {
    /// Source-language content rendered without a translation overlay.
    SourceLanguageOriginal,
    /// Requested locale has complete reviewed coverage for the source revision.
    TranslatedComplete,
    /// Requested locale is reviewed for only part of the pack.
    TranslatedPartial,
    /// Requested locale was reviewed against an older source revision.
    TranslatedStale,
    /// Requested locale falls back to source-language content.
    SourceLanguageFallback,
    /// Requested locale overlay is missing or not installed.
    LocaleNotInstalled,
}

impl LocaleOverlayCoverageState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLanguageOriginal => "source_language_original",
            Self::TranslatedComplete => "translated_complete",
            Self::TranslatedPartial => "translated_partial",
            Self::TranslatedStale => "translated_stale",
            Self::SourceLanguageFallback => "source_language_fallback",
            Self::LocaleNotInstalled => "locale_not_installed",
        }
    }

    /// Returns the badge class required for this coverage state.
    pub const fn required_badge(self) -> LocaleOverlayBadgeClass {
        match self {
            Self::SourceLanguageOriginal => LocaleOverlayBadgeClass::SourceLanguageFallback,
            Self::TranslatedComplete => LocaleOverlayBadgeClass::Translated,
            Self::TranslatedPartial => LocaleOverlayBadgeClass::PartialTranslation,
            Self::TranslatedStale => LocaleOverlayBadgeClass::StaleTranslation,
            Self::SourceLanguageFallback | Self::LocaleNotInstalled => {
                LocaleOverlayBadgeClass::SourceLanguageFallback
            }
        }
    }

    /// Returns true when the rendered row must expose source-language continuity.
    pub const fn requires_source_language_action(self) -> bool {
        matches!(
            self,
            Self::TranslatedPartial
                | Self::TranslatedStale
                | Self::SourceLanguageFallback
                | Self::LocaleNotInstalled
        )
    }

    /// Returns true when translated prose should render in the requested locale.
    pub const fn renders_requested_locale(self) -> bool {
        matches!(self, Self::TranslatedComplete | Self::TranslatedPartial)
    }
}

/// Revision-skew state between the source pack and locale overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleOverlaySkewState {
    /// Overlay source basis matches the current source revision.
    NoSkew,
    /// The source revision is newer than the translation basis.
    SourceRevisionAhead,
    /// The overlay was produced for a source revision newer than the local source.
    OverlayRevisionAhead,
    /// Skew could not be determined.
    UnknownSkew,
}

impl LocaleOverlaySkewState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSkew => "no_skew",
            Self::SourceRevisionAhead => "source_revision_ahead",
            Self::OverlayRevisionAhead => "overlay_revision_ahead",
            Self::UnknownSkew => "unknown_skew",
        }
    }
}

/// Mirror, cache, or offline posture for a locale overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleOverlayMirrorOfflinePosture {
    /// Overlay is bundled with the local product or project pack.
    BuiltInLocal,
    /// Overlay resolves through a verified mirror.
    Mirrored,
    /// Overlay is pinned for offline or air-gapped use.
    OfflinePack,
    /// Overlay resolves from a warm cache.
    Cached,
    /// Overlay is online-only and requires an explicit handoff.
    LiveOnline,
    /// Overlay is not installed locally.
    NotInstalled,
}

impl LocaleOverlayMirrorOfflinePosture {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuiltInLocal => "built_in_local",
            Self::Mirrored => "mirrored",
            Self::OfflinePack => "offline_pack",
            Self::Cached => "cached",
            Self::LiveOnline => "live_online",
            Self::NotInstalled => "not_installed",
        }
    }
}

/// User-visible badge class for translated learnability content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleOverlayBadgeClass {
    /// Complete reviewed translation.
    Translated,
    /// Partial translation with source-language continuity.
    PartialTranslation,
    /// Translation is stale relative to the source revision.
    StaleTranslation,
    /// Source-language text is active or available as fallback.
    SourceLanguageFallback,
}

impl LocaleOverlayBadgeClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Translated => "translated",
            Self::PartialTranslation => "partial_translation",
            Self::StaleTranslation => "stale_translation",
            Self::SourceLanguageFallback => "source_language_fallback",
        }
    }

    /// Returns the user-facing badge label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Translated => "Translated",
            Self::PartialTranslation => "Partial translation",
            Self::StaleTranslation => "Stale translation",
            Self::SourceLanguageFallback => "Source-language fallback",
        }
    }
}

/// Stable source-language action for a locale overlay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlaySourceLanguageAction {
    /// Stable action id used by keyboard, command palette, and support export.
    pub action_id: String,
    /// User-facing action label.
    pub action_label: String,
    /// Stable command used to open the source-language material.
    pub command_id: String,
    /// Source-language target ref for exact reopen.
    pub target_ref: String,
    /// Source locale opened by this action.
    pub source_locale: String,
    /// Whether the action is reachable without a pointer.
    pub keyboard_reachable: bool,
}

/// One locale overlay bound to a docs/help/glossary/onboarding pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlayRecord {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Stable overlay id.
    pub overlay_id: String,
    /// Pack id receiving the overlay.
    pub pack_id: String,
    /// Pack family.
    pub pack_kind: LocaleOverlayPackKind,
    /// Pack owner or reviewer group.
    pub pack_owner_ref: String,
    /// Canonical source-language locale.
    pub source_locale: String,
    /// Locale requested by the user.
    pub requested_locale: String,
    /// Locale actually rendered.
    pub effective_locale: String,
    /// Current source revision for the pack.
    pub source_revision_ref: String,
    /// Locale overlay revision rendered to the user.
    pub overlay_revision_ref: String,
    /// Source revision the overlay was translated from.
    pub overlay_source_revision_ref: String,
    /// Coverage state for this overlay.
    pub coverage_state: LocaleOverlayCoverageState,
    /// Freshness class for the rendered translation basis.
    pub freshness_class: DocsFreshnessClass,
    /// Skew state between source and overlay basis.
    pub skew_state: LocaleOverlaySkewState,
    /// Mirror, cache, or offline posture.
    pub mirror_offline_posture: LocaleOverlayMirrorOfflinePosture,
    /// User-visible translation badge.
    pub badge_class: LocaleOverlayBadgeClass,
    /// Surface refs that render this overlay.
    pub surface_refs: Vec<String>,
    /// Citation anchors preserved through translation.
    pub citation_anchor_refs: Vec<String>,
    /// Stable command ids preserved through translation.
    pub command_id_refs: Vec<String>,
    /// Keyboard path refs preserved through translation.
    pub keyboard_path_refs: Vec<String>,
    /// Scope label refs preserved through translation.
    pub scope_label_refs: Vec<String>,
    /// Source-language continuity action.
    pub source_language_action: LocaleOverlaySourceLanguageAction,
    /// Exact reopen ref preserving source, overlay, and locale identity.
    pub exact_reopen_ref: String,
    /// Support row ref used by support exports.
    pub support_row_ref: String,
}

impl LocaleOverlayRecord {
    /// Returns true when core citation and command-path anchors are present.
    pub fn preserves_identity_refs(&self) -> bool {
        !self.citation_anchor_refs.is_empty()
            && !self.command_id_refs.is_empty()
            && !self.keyboard_path_refs.is_empty()
            && !self.scope_label_refs.is_empty()
    }

    /// Returns the user-facing badge label.
    pub fn badge_label(&self) -> &'static str {
        self.badge_class.label()
    }

    fn validate(&self, findings: &mut Vec<LocaleOverlayFinding>) {
        if self.record_kind != LOCALE_OVERLAY_RECORD_KIND {
            findings.push(LocaleOverlayFinding::new(
                &self.overlay_id,
                "locale_overlay.record_kind",
                "overlay record_kind is unsupported",
            ));
        }
        if [
            &self.overlay_id,
            &self.pack_id,
            &self.pack_owner_ref,
            &self.source_locale,
            &self.requested_locale,
            &self.effective_locale,
            &self.source_revision_ref,
            &self.overlay_revision_ref,
            &self.overlay_source_revision_ref,
            &self.exact_reopen_ref,
            &self.support_row_ref,
        ]
        .iter()
        .any(|value| value.trim().is_empty())
        {
            findings.push(LocaleOverlayFinding::new(
                &self.overlay_id,
                "locale_overlay.identity",
                "overlay identity, revisions, locales, reopen ref, and support row ref must be non-empty",
            ));
        }
        if self.coverage_state.required_badge() != self.badge_class {
            findings.push(LocaleOverlayFinding::new(
                &self.overlay_id,
                "locale_overlay.badge",
                "translation badge must match coverage state",
            ));
        }
        if self.surface_refs.is_empty() || !self.preserves_identity_refs() {
            findings.push(LocaleOverlayFinding::new(
                &self.overlay_id,
                "locale_overlay.anchor_integrity",
                "overlay must preserve surface, citation, command, keyboard, and scope refs",
            ));
        }
        for command_id in &self.command_id_refs {
            if !command_id.starts_with("cmd:") {
                findings.push(LocaleOverlayFinding::new(
                    &self.overlay_id,
                    "locale_overlay.command_id",
                    "command refs must use stable cmd: ids",
                ));
            }
        }
        self.validate_source_language_action(findings);
        self.validate_locale_and_skew(findings);
    }

    fn validate_source_language_action(&self, findings: &mut Vec<LocaleOverlayFinding>) {
        let action = &self.source_language_action;
        if action.action_label != OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL
            || !action.command_id.starts_with("cmd:")
            || action.target_ref.trim().is_empty()
            || action.source_locale != self.source_locale
            || !action.keyboard_reachable
        {
            findings.push(LocaleOverlayFinding::new(
                &self.overlay_id,
                "locale_overlay.source_language_action",
                "source-language action must be stable, keyboard reachable, and target the source locale",
            ));
        }
        if self.coverage_state.requires_source_language_action()
            && action.action_id.trim().is_empty()
        {
            findings.push(LocaleOverlayFinding::new(
                &self.overlay_id,
                "locale_overlay.source_language_action_required",
                "partial, stale, missing, and fallback overlays must expose source-language continuity",
            ));
        }
    }

    fn validate_locale_and_skew(&self, findings: &mut Vec<LocaleOverlayFinding>) {
        if self.coverage_state.renders_requested_locale()
            && self.effective_locale != self.requested_locale
        {
            findings.push(LocaleOverlayFinding::new(
                &self.overlay_id,
                "locale_overlay.effective_locale",
                "translated overlays must render the requested locale",
            ));
        }
        if matches!(
            self.coverage_state,
            LocaleOverlayCoverageState::SourceLanguageFallback
                | LocaleOverlayCoverageState::LocaleNotInstalled
                | LocaleOverlayCoverageState::SourceLanguageOriginal
        ) && self.effective_locale != self.source_locale
        {
            findings.push(LocaleOverlayFinding::new(
                &self.overlay_id,
                "locale_overlay.source_fallback_locale",
                "source-language fallback overlays must render the source locale",
            ));
        }
        if self.coverage_state == LocaleOverlayCoverageState::TranslatedStale {
            if self.skew_state == LocaleOverlaySkewState::NoSkew
                || self.freshness_class != DocsFreshnessClass::Stale
                || self.overlay_source_revision_ref == self.source_revision_ref
            {
                findings.push(LocaleOverlayFinding::new(
                    &self.overlay_id,
                    "locale_overlay.stale_skew",
                    "stale translations must name source/overlay skew and stale freshness",
                ));
            }
        }
        if self.coverage_state == LocaleOverlayCoverageState::TranslatedComplete
            && (self.skew_state != LocaleOverlaySkewState::NoSkew
                || self.overlay_source_revision_ref != self.source_revision_ref)
        {
            findings.push(LocaleOverlayFinding::new(
                &self.overlay_id,
                "locale_overlay.complete_skew",
                "complete translations must match the current source revision",
            ));
        }
        if self.coverage_state == LocaleOverlayCoverageState::LocaleNotInstalled
            && self.mirror_offline_posture != LocaleOverlayMirrorOfflinePosture::NotInstalled
        {
            findings.push(LocaleOverlayFinding::new(
                &self.overlay_id,
                "locale_overlay.not_installed_posture",
                "missing locale overlays must declare not-installed posture",
            ));
        }
    }
}

/// Governed set of locale overlays for translated learnability packs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlayContract {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable contract id.
    pub contract_id: String,
    /// Stable contract version ref.
    pub contract_version_ref: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Canonical source-language locale.
    pub source_language_locale: String,
    /// Related schema, docs, fixture, and review artifacts.
    pub contract_refs: BTreeMap<String, String>,
    /// Locale overlays covered by this contract.
    pub overlays: Vec<LocaleOverlayRecord>,
    /// Support export policy for these overlays.
    pub support_export_policy: LocaleOverlaySupportExportPolicy,
}

impl LocaleOverlayContract {
    /// Returns the overlay with `overlay_id`.
    pub fn overlay(&self, overlay_id: &str) -> Option<&LocaleOverlayRecord> {
        self.overlays
            .iter()
            .find(|overlay| overlay.overlay_id == overlay_id)
    }

    /// Returns overlays for a pack family.
    pub fn overlays_for_pack_kind(
        &self,
        pack_kind: LocaleOverlayPackKind,
    ) -> Vec<&LocaleOverlayRecord> {
        self.overlays
            .iter()
            .filter(|overlay| overlay.pack_kind == pack_kind)
            .collect()
    }

    /// Projects render-ready overlay rows for docs/help/onboarding surfaces.
    pub fn surface_projection(&self) -> LocaleOverlaySurfaceProjection {
        let rows = self
            .overlays
            .iter()
            .flat_map(|overlay| {
                overlay
                    .surface_refs
                    .iter()
                    .map(|surface_ref| LocaleOverlaySurfaceRow {
                        row_id: format!("locale-overlay-surface-row:{surface_ref}:{}", overlay.overlay_id),
                        surface_ref: surface_ref.clone(),
                        overlay_id: overlay.overlay_id.clone(),
                        pack_id: overlay.pack_id.clone(),
                        pack_kind: overlay.pack_kind,
                        requested_locale: overlay.requested_locale.clone(),
                        effective_locale: overlay.effective_locale.clone(),
                        source_revision_ref: overlay.source_revision_ref.clone(),
                        overlay_revision_ref: overlay.overlay_revision_ref.clone(),
                        overlay_source_revision_ref: overlay.overlay_source_revision_ref.clone(),
                        coverage_state: overlay.coverage_state,
                        freshness_class: overlay.freshness_class,
                        skew_state: overlay.skew_state,
                        mirror_offline_posture: overlay.mirror_offline_posture,
                        badge_class: overlay.badge_class,
                        badge_label: overlay.badge_label().to_owned(),
                        source_language_action_id: overlay.source_language_action.action_id.clone(),
                        source_language_action_label: overlay.source_language_action.action_label.clone(),
                        citation_anchor_refs: overlay.citation_anchor_refs.clone(),
                        command_id_refs: overlay.command_id_refs.clone(),
                        keyboard_path_refs: overlay.keyboard_path_refs.clone(),
                        scope_label_refs: overlay.scope_label_refs.clone(),
                        support_row_ref: overlay.support_row_ref.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        LocaleOverlaySurfaceProjection {
            record_kind: LOCALE_OVERLAY_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: LOCALE_OVERLAY_SCHEMA_VERSION,
            projection_id: "locale-overlay:translated-learnability:surface-projection:v1"
                .to_owned(),
            generated_at: self.generated_at.clone(),
            contract_id: self.contract_id.clone(),
            contract_version_ref: self.contract_version_ref.clone(),
            rows,
            coverage: self.coverage(),
        }
    }

    /// Projects a metadata-only support export.
    pub fn support_export(
        &self,
        support_export_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> LocaleOverlaySupportExport {
        let rows = self
            .overlays
            .iter()
            .map(|overlay| LocaleOverlaySupportRow {
                support_row_ref: overlay.support_row_ref.clone(),
                overlay_id: overlay.overlay_id.clone(),
                pack_id: overlay.pack_id.clone(),
                pack_kind: overlay.pack_kind,
                pack_owner_ref: overlay.pack_owner_ref.clone(),
                source_locale: overlay.source_locale.clone(),
                requested_locale: overlay.requested_locale.clone(),
                effective_locale: overlay.effective_locale.clone(),
                source_revision_ref: overlay.source_revision_ref.clone(),
                overlay_revision_ref: overlay.overlay_revision_ref.clone(),
                overlay_source_revision_ref: overlay.overlay_source_revision_ref.clone(),
                coverage_state: overlay.coverage_state,
                freshness_class: overlay.freshness_class,
                skew_state: overlay.skew_state,
                mirror_offline_posture: overlay.mirror_offline_posture,
                badge_class: overlay.badge_class,
                source_language_action_id: overlay.source_language_action.action_id.clone(),
                source_language_action_label: overlay.source_language_action.action_label.clone(),
                citation_anchor_refs: overlay.citation_anchor_refs.clone(),
                command_id_refs: overlay.command_id_refs.clone(),
                keyboard_path_refs: overlay.keyboard_path_refs.clone(),
                scope_label_refs: overlay.scope_label_refs.clone(),
                exact_reopen_ref: overlay.exact_reopen_ref.clone(),
                raw_translated_body_exported: false,
            })
            .collect::<Vec<_>>();

        LocaleOverlaySupportExport {
            record_kind: LOCALE_OVERLAY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: LOCALE_OVERLAY_SCHEMA_VERSION,
            support_export_id: support_export_id.into(),
            generated_at: generated_at.into(),
            source_contract_id: self.contract_id.clone(),
            contract_version_ref: self.contract_version_ref.clone(),
            rows,
            omitted_material_classes: self
                .support_export_policy
                .omitted_material_classes
                .clone(),
            raw_translated_bodies_exported: false,
        }
    }

    /// Validates the contract without resolving raw translated bodies.
    pub fn validate(&self) -> Vec<LocaleOverlayFinding> {
        let mut findings = Vec::new();
        if self.record_kind != LOCALE_OVERLAY_CONTRACT_RECORD_KIND {
            findings.push(LocaleOverlayFinding::new(
                &self.contract_id,
                "locale_overlay.contract.record_kind",
                "contract record_kind is unsupported",
            ));
        }
        if self.schema_version != LOCALE_OVERLAY_SCHEMA_VERSION {
            findings.push(LocaleOverlayFinding::new(
                &self.contract_id,
                "locale_overlay.contract.schema_version",
                "contract schema version is unsupported",
            ));
        }
        if self.contract_id.trim().is_empty()
            || self.contract_version_ref.trim().is_empty()
            || self.source_language_locale.trim().is_empty()
        {
            findings.push(LocaleOverlayFinding::new(
                &self.contract_id,
                "locale_overlay.contract.identity",
                "contract id, version, and source language must be non-empty",
            ));
        }
        if self.overlays.is_empty() {
            findings.push(LocaleOverlayFinding::new(
                &self.contract_id,
                "locale_overlay.contract.overlays",
                "contract must include at least one locale overlay",
            ));
        }
        let mut overlay_ids = BTreeSet::new();
        let mut pack_kinds = BTreeSet::new();
        for overlay in &self.overlays {
            if !overlay_ids.insert(overlay.overlay_id.as_str()) {
                findings.push(LocaleOverlayFinding::new(
                    &overlay.overlay_id,
                    "locale_overlay.duplicate",
                    "duplicate overlay id",
                ));
            }
            pack_kinds.insert(overlay.pack_kind);
            if overlay.source_locale != self.source_language_locale {
                findings.push(LocaleOverlayFinding::new(
                    &overlay.overlay_id,
                    "locale_overlay.source_locale",
                    "overlay source locale must match contract source language",
                ));
            }
            overlay.validate(&mut findings);
        }
        for required in [
            LocaleOverlayPackKind::DocsPack,
            LocaleOverlayPackKind::HelpPack,
            LocaleOverlayPackKind::GlossaryPack,
            LocaleOverlayPackKind::OnboardingPack,
            LocaleOverlayPackKind::GuidedTourPack,
            LocaleOverlayPackKind::TroubleshootingHelpPack,
        ] {
            if !pack_kinds.contains(&required) {
                findings.push(LocaleOverlayFinding::new(
                    &self.contract_id,
                    "locale_overlay.pack_kind_coverage",
                    format!("contract must cover {}", required.as_str()),
                ));
            }
        }
        self.support_export_policy.validate(&mut findings);
        findings
    }

    fn coverage(&self) -> LocaleOverlayCoverage {
        let mut coverage_counts = BTreeMap::new();
        let mut badge_counts = BTreeMap::new();
        let mut pack_kinds = BTreeSet::new();
        let mut source_language_action_count = 0usize;
        for overlay in &self.overlays {
            *coverage_counts
                .entry(overlay.coverage_state.as_str().to_owned())
                .or_insert(0) += 1;
            *badge_counts
                .entry(overlay.badge_class.as_str().to_owned())
                .or_insert(0) += 1;
            pack_kinds.insert(overlay.pack_kind.as_str().to_owned());
            if overlay.source_language_action.action_label == OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL {
                source_language_action_count += 1;
            }
        }
        LocaleOverlayCoverage {
            overlay_count: self.overlays.len(),
            pack_kind_count: pack_kinds.len(),
            pack_kinds: pack_kinds.into_iter().collect(),
            coverage_counts,
            badge_counts,
            source_language_action_count,
        }
    }
}

/// Support-export policy for locale overlays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlaySupportExportPolicy {
    /// Stable policy id.
    pub policy_id: String,
    /// Whether support export records source and overlay revisions.
    pub record_source_and_overlay_revisions: bool,
    /// Whether support export records citation, command, keyboard, and scope refs.
    pub record_integrity_refs: bool,
    /// Whether support export records source-language action identity.
    pub record_source_language_action: bool,
    /// Whether raw translated bodies may be exported.
    pub raw_translated_bodies_exported: bool,
    /// Bounded material classes included in support export.
    pub bounded_material_classes: Vec<String>,
    /// Material classes explicitly omitted from support export.
    pub omitted_material_classes: Vec<String>,
}

impl LocaleOverlaySupportExportPolicy {
    fn validate(&self, findings: &mut Vec<LocaleOverlayFinding>) {
        if !self.record_source_and_overlay_revisions
            || !self.record_integrity_refs
            || !self.record_source_language_action
        {
            findings.push(LocaleOverlayFinding::new(
                &self.policy_id,
                "locale_overlay.support_policy.fields",
                "support export must record revisions, integrity refs, and source-language action identity",
            ));
        }
        if self.raw_translated_bodies_exported || self.omitted_material_classes.is_empty() {
            findings.push(LocaleOverlayFinding::new(
                &self.policy_id,
                "locale_overlay.support_policy.raw_bodies",
                "support export must omit raw translated bodies and disclose omitted classes",
            ));
        }
    }
}

/// Surface projection for locale overlays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlaySurfaceProjection {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source contract id.
    pub contract_id: String,
    /// Source contract version ref.
    pub contract_version_ref: String,
    /// Render-ready rows for docs/help/onboarding surfaces.
    pub rows: Vec<LocaleOverlaySurfaceRow>,
    /// Coverage summary for review and release packets.
    pub coverage: LocaleOverlayCoverage,
}

/// One render-ready surface row for a locale overlay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlaySurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface ref that renders the row.
    pub surface_ref: String,
    /// Locale overlay id.
    pub overlay_id: String,
    /// Pack id receiving the overlay.
    pub pack_id: String,
    /// Pack family.
    pub pack_kind: LocaleOverlayPackKind,
    /// Locale requested by the user.
    pub requested_locale: String,
    /// Locale actually rendered.
    pub effective_locale: String,
    /// Current source revision for the pack.
    pub source_revision_ref: String,
    /// Locale overlay revision rendered to the user.
    pub overlay_revision_ref: String,
    /// Source revision the overlay was translated from.
    pub overlay_source_revision_ref: String,
    /// Coverage state for this overlay.
    pub coverage_state: LocaleOverlayCoverageState,
    /// Freshness class for the rendered translation basis.
    pub freshness_class: DocsFreshnessClass,
    /// Skew state between source and overlay basis.
    pub skew_state: LocaleOverlaySkewState,
    /// Mirror, cache, or offline posture.
    pub mirror_offline_posture: LocaleOverlayMirrorOfflinePosture,
    /// User-visible translation badge.
    pub badge_class: LocaleOverlayBadgeClass,
    /// User-facing badge label.
    pub badge_label: String,
    /// Source-language action id.
    pub source_language_action_id: String,
    /// Source-language action label.
    pub source_language_action_label: String,
    /// Citation anchors preserved through translation.
    pub citation_anchor_refs: Vec<String>,
    /// Stable command ids preserved through translation.
    pub command_id_refs: Vec<String>,
    /// Keyboard path refs preserved through translation.
    pub keyboard_path_refs: Vec<String>,
    /// Scope label refs preserved through translation.
    pub scope_label_refs: Vec<String>,
    /// Support row ref used by support exports.
    pub support_row_ref: String,
}

/// Coverage summary for locale-overlay projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlayCoverage {
    /// Number of overlay records.
    pub overlay_count: usize,
    /// Number of pack families covered.
    pub pack_kind_count: usize,
    /// Pack-family tokens covered by this projection.
    pub pack_kinds: Vec<String>,
    /// Count by coverage-state token.
    pub coverage_counts: BTreeMap<String, usize>,
    /// Count by badge-class token.
    pub badge_counts: BTreeMap<String, usize>,
    /// Number of overlays exposing the source-language action label.
    pub source_language_action_count: usize,
}

/// Metadata-only support export for locale overlays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlaySupportExport {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Export generation timestamp.
    pub generated_at: String,
    /// Source contract id.
    pub source_contract_id: String,
    /// Source contract version ref.
    pub contract_version_ref: String,
    /// Metadata-safe rows captured by support export.
    pub rows: Vec<LocaleOverlaySupportRow>,
    /// Material classes omitted from the export.
    pub omitted_material_classes: Vec<String>,
    /// Whether raw translated bodies were exported.
    pub raw_translated_bodies_exported: bool,
}

impl LocaleOverlaySupportExport {
    /// Validates support export reconstruction against `contract`.
    pub fn validate_against_contract(
        &self,
        contract: &LocaleOverlayContract,
    ) -> Result<(), Vec<LocaleOverlayFinding>> {
        let mut findings = Vec::new();
        if self.record_kind != LOCALE_OVERLAY_SUPPORT_EXPORT_RECORD_KIND {
            findings.push(LocaleOverlayFinding::new(
                &self.support_export_id,
                "locale_overlay.support.record_kind",
                "support export record_kind is unsupported",
            ));
        }
        if self.schema_version != LOCALE_OVERLAY_SCHEMA_VERSION {
            findings.push(LocaleOverlayFinding::new(
                &self.support_export_id,
                "locale_overlay.support.schema_version",
                "support export schema version is unsupported",
            ));
        }
        if self.source_contract_id != contract.contract_id
            || self.contract_version_ref != contract.contract_version_ref
        {
            findings.push(LocaleOverlayFinding::new(
                &self.support_export_id,
                "locale_overlay.support.contract_ref",
                "support export contract refs drifted",
            ));
        }
        if self.raw_translated_bodies_exported || self.omitted_material_classes.is_empty() {
            findings.push(LocaleOverlayFinding::new(
                &self.support_export_id,
                "locale_overlay.support.raw_bodies",
                "support export must omit raw translated bodies and disclose omitted classes",
            ));
        }
        if self.rows.len() != contract.overlays.len() {
            findings.push(LocaleOverlayFinding::new(
                &self.support_export_id,
                "locale_overlay.support.row_count",
                "support export row count must match overlay count",
            ));
        }
        for row in &self.rows {
            let Some(overlay) = contract.overlay(&row.overlay_id) else {
                findings.push(LocaleOverlayFinding::new(
                    &row.support_row_ref,
                    "locale_overlay.support.unknown_overlay",
                    "support row references an unknown overlay",
                ));
                continue;
            };
            if row.pack_id != overlay.pack_id
                || row.pack_kind != overlay.pack_kind
                || row.pack_owner_ref != overlay.pack_owner_ref
                || row.source_locale != overlay.source_locale
                || row.requested_locale != overlay.requested_locale
                || row.effective_locale != overlay.effective_locale
                || row.source_revision_ref != overlay.source_revision_ref
                || row.overlay_revision_ref != overlay.overlay_revision_ref
                || row.overlay_source_revision_ref != overlay.overlay_source_revision_ref
                || row.coverage_state != overlay.coverage_state
                || row.freshness_class != overlay.freshness_class
                || row.skew_state != overlay.skew_state
                || row.mirror_offline_posture != overlay.mirror_offline_posture
                || row.badge_class != overlay.badge_class
                || row.source_language_action_label != OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL
                || row.citation_anchor_refs != overlay.citation_anchor_refs
                || row.command_id_refs != overlay.command_id_refs
                || row.keyboard_path_refs != overlay.keyboard_path_refs
                || row.scope_label_refs != overlay.scope_label_refs
                || row.exact_reopen_ref != overlay.exact_reopen_ref
            {
                findings.push(LocaleOverlayFinding::new(
                    &row.support_row_ref,
                    "locale_overlay.support.row_drift",
                    "support row drifted from overlay metadata",
                ));
            }
            if row.raw_translated_body_exported {
                findings.push(LocaleOverlayFinding::new(
                    &row.support_row_ref,
                    "locale_overlay.support.row_raw_body",
                    "support row must not export raw translated body",
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

/// Metadata-safe support row for one locale overlay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlaySupportRow {
    /// Support row ref used by support exports.
    pub support_row_ref: String,
    /// Locale overlay id.
    pub overlay_id: String,
    /// Pack id receiving the overlay.
    pub pack_id: String,
    /// Pack family.
    pub pack_kind: LocaleOverlayPackKind,
    /// Pack owner or reviewer group.
    pub pack_owner_ref: String,
    /// Canonical source-language locale.
    pub source_locale: String,
    /// Locale requested by the user.
    pub requested_locale: String,
    /// Locale actually rendered.
    pub effective_locale: String,
    /// Current source revision for the pack.
    pub source_revision_ref: String,
    /// Locale overlay revision rendered to the user.
    pub overlay_revision_ref: String,
    /// Source revision the overlay was translated from.
    pub overlay_source_revision_ref: String,
    /// Coverage state for this overlay.
    pub coverage_state: LocaleOverlayCoverageState,
    /// Freshness class for the rendered translation basis.
    pub freshness_class: DocsFreshnessClass,
    /// Skew state between source and overlay basis.
    pub skew_state: LocaleOverlaySkewState,
    /// Mirror, cache, or offline posture.
    pub mirror_offline_posture: LocaleOverlayMirrorOfflinePosture,
    /// User-visible translation badge.
    pub badge_class: LocaleOverlayBadgeClass,
    /// Source-language action id.
    pub source_language_action_id: String,
    /// Source-language action label.
    pub source_language_action_label: String,
    /// Citation anchors preserved through translation.
    pub citation_anchor_refs: Vec<String>,
    /// Stable command ids preserved through translation.
    pub command_id_refs: Vec<String>,
    /// Keyboard path refs preserved through translation.
    pub keyboard_path_refs: Vec<String>,
    /// Scope label refs preserved through translation.
    pub scope_label_refs: Vec<String>,
    /// Exact reopen ref preserving source, overlay, and locale identity.
    pub exact_reopen_ref: String,
    /// Whether raw translated body content was exported.
    pub raw_translated_body_exported: bool,
}

/// Validation finding for locale-overlay contracts and exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleOverlayFinding {
    /// Row or object that failed validation.
    pub row_ref: String,
    /// Stable validation check id.
    pub check_id: String,
    /// Reviewable validation message.
    pub message: String,
}

impl LocaleOverlayFinding {
    fn new(
        row_ref: impl Into<String>,
        check_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            row_ref: row_ref.into(),
            check_id: check_id.into(),
            message: message.into(),
        }
    }
}

/// Returns the seeded translated-pack locale-overlay contract.
pub fn seeded_translated_pack_locale_overlay_contract() -> LocaleOverlayContract {
    let overlays = vec![
        overlay(LocaleOverlaySeed {
            overlay_id: "locale-overlay:docs-pack:tsjs-launch:es-MX:2026.05.18",
            pack_id: "docs-pack:tsjs-launch-bundle",
            pack_kind: LocaleOverlayPackKind::DocsPack,
            requested_locale: "es-MX",
            effective_locale: "es-MX",
            source_revision_ref: "docs-source-rev:tsjs-launch:2026.05.18-01",
            overlay_revision_ref: "locale-overlay-rev:tsjs-launch:es-MX:2026.05.18-01",
            overlay_source_revision_ref: "docs-source-rev:tsjs-launch:2026.05.18-01",
            coverage_state: LocaleOverlayCoverageState::TranslatedComplete,
            freshness_class: DocsFreshnessClass::AuthoritativeLive,
            skew_state: LocaleOverlaySkewState::NoSkew,
            mirror_offline_posture: LocaleOverlayMirrorOfflinePosture::OfflinePack,
            surface_refs: vec!["surface:docs_browser:pack_row"],
            citation_anchor_refs: vec![
                "docs-anchor:tsjs-launch:setup",
                "docs-anchor:tsjs-launch:commands",
            ],
            command_id_refs: vec!["cmd:docs.open_in_browser"],
            keyboard_path_refs: vec!["keyboard:path:docs.open_in_browser"],
            scope_label_refs: vec!["scope:docs-help"],
        }),
        overlay(LocaleOverlaySeed {
            overlay_id: "locale-overlay:help-pack:onboarding:first-run:es-MX:2026.05.18",
            pack_id: "pack:onboarding-help:first-run-beta",
            pack_kind: LocaleOverlayPackKind::HelpPack,
            requested_locale: "es-MX",
            effective_locale: "es-MX",
            source_revision_ref: "help-pack-source-rev:first-run:2026.05.18-01",
            overlay_revision_ref: "locale-overlay-rev:first-run:es-MX:2026.05.18-01",
            overlay_source_revision_ref: "help-pack-source-rev:first-run:2026.05.18-01",
            coverage_state: LocaleOverlayCoverageState::TranslatedComplete,
            freshness_class: DocsFreshnessClass::AuthoritativeLive,
            skew_state: LocaleOverlaySkewState::NoSkew,
            mirror_offline_posture: LocaleOverlayMirrorOfflinePosture::BuiltInLocal,
            surface_refs: vec![
                "surface:start_center:first_run",
                "surface:help_search:onboarding",
            ],
            citation_anchor_refs: vec!["citation:docs-pack:project-entry.open-folder:source"],
            command_id_refs: vec!["cmd:workspace.open_folder"],
            keyboard_path_refs: vec!["keyboard:path:workspace.open_folder"],
            scope_label_refs: vec!["scope:onboarding"],
        }),
        overlay(LocaleOverlaySeed {
            overlay_id: "locale-overlay:glossary:truth-terms:es-MX:2026.05.18",
            pack_id: "pack:onboarding-help:glossary-beta",
            pack_kind: LocaleOverlayPackKind::GlossaryPack,
            requested_locale: "es-MX",
            effective_locale: "es-MX",
            source_revision_ref: "glossary-source-rev:truth-terms:2026.05.18-01",
            overlay_revision_ref: "locale-overlay-rev:truth-terms:es-MX:2026.05.18-01",
            overlay_source_revision_ref: "glossary-source-rev:truth-terms:2026.05.18-01",
            coverage_state: LocaleOverlayCoverageState::TranslatedPartial,
            freshness_class: DocsFreshnessClass::WarmCached,
            skew_state: LocaleOverlaySkewState::NoSkew,
            mirror_offline_posture: LocaleOverlayMirrorOfflinePosture::OfflinePack,
            surface_refs: vec![
                "surface:migration_center:beta",
                "surface:help_search:glossary",
            ],
            citation_anchor_refs: vec![
                "citation:glossary:release_truth_terms:source",
                "citation:docs-help:truth_source_model:source",
            ],
            command_id_refs: vec!["cmd:docs.open_in_browser"],
            keyboard_path_refs: vec!["keyboard:path:glossary.open_related"],
            scope_label_refs: vec!["scope:glossary", "scope:migration-center"],
        }),
        overlay(LocaleOverlaySeed {
            overlay_id: "locale-overlay:onboarding:keymap-bridge:es-MX:2026.05.18",
            pack_id: "pack:onboarding-help:migration-beta",
            pack_kind: LocaleOverlayPackKind::OnboardingPack,
            requested_locale: "es-MX",
            effective_locale: "es-MX",
            source_revision_ref: "help-pack-source-rev:migration:2026.05.18-01",
            overlay_revision_ref: "locale-overlay-rev:keymap-bridge:es-MX:2026.05.17-01",
            overlay_source_revision_ref: "help-pack-source-rev:migration:2026.05.17-01",
            coverage_state: LocaleOverlayCoverageState::TranslatedStale,
            freshness_class: DocsFreshnessClass::Stale,
            skew_state: LocaleOverlaySkewState::SourceRevisionAhead,
            mirror_offline_posture: LocaleOverlayMirrorOfflinePosture::Cached,
            surface_refs: vec![
                "surface:migration_center:beta",
                "surface:contextual_why_now:onboarding",
            ],
            citation_anchor_refs: vec![
                "citation:docs-pack:onboarding.keymap-bridge:source",
                "citation:docs-pack:onboarding.keymap-bridge:fallback",
            ],
            command_id_refs: vec!["cmd:command_palette.open"],
            keyboard_path_refs: vec!["keyboard:path:command_palette.open"],
            scope_label_refs: vec!["scope:onboarding", "scope:migration-center"],
        }),
        overlay(LocaleOverlaySeed {
            overlay_id: "locale-overlay:guided-tour:cached-docs:fr-FR:2026.05.18",
            pack_id: "tour-pack:aureline.cached-docs.preview",
            pack_kind: LocaleOverlayPackKind::GuidedTourPack,
            requested_locale: "fr-FR",
            effective_locale: "en-US",
            source_revision_ref: "tour-source-rev:cached-docs:2026.05.18-01",
            overlay_revision_ref: "locale-overlay-rev:cached-docs:fr-FR:missing",
            overlay_source_revision_ref: "tour-source-rev:cached-docs:2026.05.10-01",
            coverage_state: LocaleOverlayCoverageState::SourceLanguageFallback,
            freshness_class: DocsFreshnessClass::DegradedCached,
            skew_state: LocaleOverlaySkewState::SourceRevisionAhead,
            mirror_offline_posture: LocaleOverlayMirrorOfflinePosture::Mirrored,
            surface_refs: vec![
                "surface:learning_mode.left_rail",
                "surface:learning_mode.digest",
            ],
            citation_anchor_refs: vec!["citation:mirror:guided-learning.cached-docs"],
            command_id_refs: vec!["cmd:docs.open_in_browser"],
            keyboard_path_refs: vec!["keyboard:path:learning_mode.open_docs"],
            scope_label_refs: vec!["scope:learning-mode", "scope:guided-tour"],
        }),
        overlay(LocaleOverlaySeed {
            overlay_id: "locale-overlay:troubleshooting:recovery:de-DE:not-installed",
            pack_id: "pack:onboarding-help:recovery-beta",
            pack_kind: LocaleOverlayPackKind::TroubleshootingHelpPack,
            requested_locale: "de-DE",
            effective_locale: "en-US",
            source_revision_ref: "support-runbook-source-rev:recovery-first:2026.05.18-01",
            overlay_revision_ref: "locale-overlay-rev:recovery:de-DE:not-installed",
            overlay_source_revision_ref: "support-runbook-source-rev:recovery-first:2026.05.18-01",
            coverage_state: LocaleOverlayCoverageState::LocaleNotInstalled,
            freshness_class: DocsFreshnessClass::DegradedCached,
            skew_state: LocaleOverlaySkewState::UnknownSkew,
            mirror_offline_posture: LocaleOverlayMirrorOfflinePosture::NotInstalled,
            surface_refs: vec![
                "surface:recovery_first:onboarding",
                "surface:help_search:onboarding",
            ],
            citation_anchor_refs: vec![
                "citation:recovery:restore-checkpoint:source",
                "citation:migration:center:rollback-checkpoint:source",
            ],
            command_id_refs: vec!["cmd:workspace.restore_from_checkpoint"],
            keyboard_path_refs: vec!["keyboard:path:workspace.restore_from_checkpoint"],
            scope_label_refs: vec!["scope:troubleshooting", "scope:recovery-first"],
        }),
    ];

    LocaleOverlayContract {
        record_kind: LOCALE_OVERLAY_CONTRACT_RECORD_KIND.to_owned(),
        schema_version: LOCALE_OVERLAY_SCHEMA_VERSION,
        contract_id: TRANSLATED_PACK_LOCALE_OVERLAY_CONTRACT_ID.to_owned(),
        contract_version_ref: TRANSLATED_PACK_LOCALE_OVERLAY_VERSION_REF.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_language_locale: SOURCE_LOCALE.to_owned(),
        contract_refs: BTreeMap::from([
            ("schema".to_owned(), LOCALE_OVERLAY_SCHEMA_REF.to_owned()),
            (
                "docs_contract".to_owned(),
                "docs/ux/m3/translated_help_and_onboarding_pack_contract.md".to_owned(),
            ),
            (
                "review_packet".to_owned(),
                "artifacts/docs/m3/translated_pack_review.md".to_owned(),
            ),
            (
                "locale_pack_contract".to_owned(),
                "docs/ux/m3/locale_pack_beta_contract.md".to_owned(),
            ),
        ]),
        overlays,
        support_export_policy: LocaleOverlaySupportExportPolicy {
            policy_id: "support-policy:locale-overlay:translated-learnability:v1".to_owned(),
            record_source_and_overlay_revisions: true,
            record_integrity_refs: true,
            record_source_language_action: true,
            raw_translated_bodies_exported: false,
            bounded_material_classes: vec![
                "overlay_id".to_owned(),
                "pack_id".to_owned(),
                "source_revision_ref".to_owned(),
                "overlay_revision_ref".to_owned(),
                "coverage_state".to_owned(),
                "freshness_class".to_owned(),
                "citation_anchor_refs".to_owned(),
                "command_id_refs".to_owned(),
                "keyboard_path_refs".to_owned(),
                "scope_label_refs".to_owned(),
                "source_language_action_id".to_owned(),
            ],
            omitted_material_classes: vec![
                "raw_translated_body".to_owned(),
                "raw_source_body".to_owned(),
                "raw_docs_url".to_owned(),
                "private_workspace_path".to_owned(),
                "account_identifier".to_owned(),
            ],
        },
    }
}

/// Returns the seeded translated-pack locale-overlay surface projection.
pub fn seeded_translated_pack_locale_overlay_surface_projection() -> LocaleOverlaySurfaceProjection
{
    seeded_translated_pack_locale_overlay_contract().surface_projection()
}

/// Returns the seeded translated-pack locale-overlay support export.
pub fn seeded_translated_pack_locale_overlay_support_export() -> LocaleOverlaySupportExport {
    seeded_translated_pack_locale_overlay_contract().support_export(
        "support-export:locale-overlay:translated-learnability:001",
        GENERATED_AT,
    )
}

/// Validates all seeded translated-pack locale-overlay records.
pub fn validate_seeded_translated_pack_locale_overlay() -> Result<(), Vec<LocaleOverlayFinding>> {
    let contract = seeded_translated_pack_locale_overlay_contract();
    let export = contract.support_export(
        "support-export:locale-overlay:translated-learnability:001",
        GENERATED_AT,
    );
    let mut findings = contract.validate();
    if let Err(mut export_findings) = export.validate_against_contract(&contract) {
        findings.append(&mut export_findings);
    }
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

struct LocaleOverlaySeed {
    overlay_id: &'static str,
    pack_id: &'static str,
    pack_kind: LocaleOverlayPackKind,
    requested_locale: &'static str,
    effective_locale: &'static str,
    source_revision_ref: &'static str,
    overlay_revision_ref: &'static str,
    overlay_source_revision_ref: &'static str,
    coverage_state: LocaleOverlayCoverageState,
    freshness_class: DocsFreshnessClass,
    skew_state: LocaleOverlaySkewState,
    mirror_offline_posture: LocaleOverlayMirrorOfflinePosture,
    surface_refs: Vec<&'static str>,
    citation_anchor_refs: Vec<&'static str>,
    command_id_refs: Vec<&'static str>,
    keyboard_path_refs: Vec<&'static str>,
    scope_label_refs: Vec<&'static str>,
}

fn overlay(seed: LocaleOverlaySeed) -> LocaleOverlayRecord {
    LocaleOverlayRecord {
        record_kind: LOCALE_OVERLAY_RECORD_KIND.to_owned(),
        overlay_id: seed.overlay_id.to_owned(),
        pack_id: seed.pack_id.to_owned(),
        pack_kind: seed.pack_kind,
        pack_owner_ref: "owner:docs-help-learnability".to_owned(),
        source_locale: SOURCE_LOCALE.to_owned(),
        requested_locale: seed.requested_locale.to_owned(),
        effective_locale: seed.effective_locale.to_owned(),
        source_revision_ref: seed.source_revision_ref.to_owned(),
        overlay_revision_ref: seed.overlay_revision_ref.to_owned(),
        overlay_source_revision_ref: seed.overlay_source_revision_ref.to_owned(),
        coverage_state: seed.coverage_state,
        freshness_class: seed.freshness_class,
        skew_state: seed.skew_state,
        mirror_offline_posture: seed.mirror_offline_posture,
        badge_class: seed.coverage_state.required_badge(),
        surface_refs: seed.surface_refs.into_iter().map(str::to_owned).collect(),
        citation_anchor_refs: seed
            .citation_anchor_refs
            .into_iter()
            .map(str::to_owned)
            .collect(),
        command_id_refs: seed.command_id_refs.into_iter().map(str::to_owned).collect(),
        keyboard_path_refs: seed
            .keyboard_path_refs
            .into_iter()
            .map(str::to_owned)
            .collect(),
        scope_label_refs: seed.scope_label_refs.into_iter().map(str::to_owned).collect(),
        source_language_action: LocaleOverlaySourceLanguageAction {
            action_id: format!("action:{}:open-source-language", seed.overlay_id),
            action_label: OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL.to_owned(),
            command_id: SOURCE_LANGUAGE_COMMAND_ID.to_owned(),
            target_ref: format!("source-language:{}#{}", seed.pack_id, SOURCE_LOCALE),
            source_locale: SOURCE_LOCALE.to_owned(),
            keyboard_reachable: true,
        },
        exact_reopen_ref: format!(
            "reopen:{}@{}#{}",
            seed.pack_id, seed.overlay_revision_ref, seed.effective_locale
        ),
        support_row_ref: format!("support-row:{}", seed.overlay_id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_contract_validates() {
        validate_seeded_translated_pack_locale_overlay()
            .expect("seeded translated-pack locale overlays validate");
    }

    #[test]
    fn badges_cover_translated_partial_stale_and_fallback() {
        let projection = seeded_translated_pack_locale_overlay_surface_projection();
        for badge in [
            LocaleOverlayBadgeClass::Translated,
            LocaleOverlayBadgeClass::PartialTranslation,
            LocaleOverlayBadgeClass::StaleTranslation,
            LocaleOverlayBadgeClass::SourceLanguageFallback,
        ] {
            assert!(projection
                .rows
                .iter()
                .any(|row| row.badge_class == badge && !row.badge_label.is_empty()));
        }
        assert!(projection
            .rows
            .iter()
            .all(|row| row.source_language_action_label == OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL));
    }

    #[test]
    fn support_export_is_metadata_only() {
        let contract = seeded_translated_pack_locale_overlay_contract();
        let export = seeded_translated_pack_locale_overlay_support_export();
        export
            .validate_against_contract(&contract)
            .expect("support export validates");
        assert!(!export.raw_translated_bodies_exported);
        assert!(export.rows.iter().all(|row| {
            !row.raw_translated_body_exported
                && !row.citation_anchor_refs.is_empty()
                && !row.command_id_refs.is_empty()
                && !row.keyboard_path_refs.is_empty()
                && !row.scope_label_refs.is_empty()
        }));
    }
}
