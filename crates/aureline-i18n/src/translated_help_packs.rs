//! Translated docs, tours, glossary cards, auth/recovery copy, and onboarding
//! assets for the claimed localized M5 profiles.
//!
//! The [`message_registry`](crate::message_registry) and
//! [`localized_catalog`](crate::localized_catalog) lanes cover the *short* M5
//! surfaces — chrome, command, settings, help, error, and notification strings.
//! This module carries the *long-form* half the spec treats as release-bearing:
//! the actual translated learnability and recovery assets, each one bound to a
//! stable, locale-neutral asset id and to the stable refs business logic and
//! support flows route by.
//!
//! Three claims are made testable here rather than reviewed by hand:
//!
//! - **Translations stay citation-faithful and command-faithful.** Every
//!   translated asset preserves the source asset's citation anchors, command
//!   ids, keyboard paths, and scope labels byte-for-byte; only the prose and the
//!   visible title change. [`M5TranslatedHelpPack::validate`] rejects a
//!   translation whose preserved refs drift from its source asset, and
//!   [`build_translated_help_parity_report`] proves the property per claimed
//!   locale.
//! - **Source-language truth stays reachable.** Every asset — translated or
//!   fallen back — exposes an `Open in source language` escape hatch that is
//!   keyboard reachable and points at the canonical source body, so support,
//!   troubleshooting, and learning can always reach exact wording.
//! - **Imported or stale translated help never masquerades as live truth.** Each
//!   translation discloses its freshness, the source revision it was translated
//!   from, and its mirror/offline posture. When freshness diverges from the live
//!   source the rendered row is marked `distinct_from_live_source`, and
//!   escalation-critical auth/recovery copy keeps its escalation command routes
//!   even when stale.
//!
//! Translated bodies live as real files under `docs/help/locales/<locale>/`; the
//! packet references them by path and carries the stable refs extracted from
//! them. This boundary never carries credentials, raw provider payloads, or
//! private workspace paths.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::localized_catalog::{TextDirection, CLAIMED_LOCALES};
use crate::{
    LocalePackValidationFinding, SourceLanguageEscapeHatchClass, GENERATED_AT,
    SOURCE_LANGUAGE_LOCALE, TARGET_BUILD,
};

/// Schema version for the translated-help pack, render, and parity records.
pub const M5_TRANSLATED_HELP_PACK_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`M5TranslatedHelpPack`].
pub const M5_TRANSLATED_HELP_PACK_RECORD_KIND: &str = "m5_translated_help_pack_packet";

/// Record kind for [`M5TranslatedHelpRender`].
pub const M5_TRANSLATED_HELP_RENDER_RECORD_KIND: &str = "m5_translated_help_render_packet";

/// Record kind for [`M5TranslatedHelpParityReport`].
pub const M5_TRANSLATED_HELP_PARITY_RECORD_KIND: &str = "m5_translated_help_parity_report";

/// Stable packet id for the seeded translated-help pack.
pub const M5_TRANSLATED_HELP_PACK_ID: &str =
    "i18n:m5-translated-help-pack:docs-tour-glossary-auth-recovery-onboarding:v1";

/// Stable report id for the seeded translated-help parity report.
pub const M5_TRANSLATED_HELP_PARITY_REPORT_ID: &str =
    "i18n:m5-translated-help-parity:claimed-profiles:v1";

/// Fixture path for the seeded translated-help pack.
pub const M5_TRANSLATED_HELP_PACK_FIXTURE_REF: &str =
    "fixtures/i18n/docs-tour-auth-recovery/translated-help-packs.json";

/// Fixture path for the seeded translated-help parity report.
pub const M5_TRANSLATED_HELP_PARITY_FIXTURE_REF: &str =
    "fixtures/i18n/docs-tour-auth-recovery/translated-help-parity.json";

/// Stable user-facing action label for source-language continuity.
///
/// This exact label is the escape-hatch contract: it must appear unchanged on
/// every translated and fallback asset so reviewers can always find the route
/// back to exact source-language wording.
pub const OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL: &str = "Open in source language";

/// Asset family carried by the translated-help pack.
///
/// These are exactly the long-form M5 flows the spec names: docs/help pages,
/// guided tours, glossary cards, auth copy, recovery copy, and onboarding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranslatedAssetFamily {
    /// Docs and help pages.
    Docs,
    /// Guided tours and learning steps.
    GuidedTour,
    /// Glossary and terminology cards.
    GlossaryCard,
    /// Sign-in, account, and authorization copy.
    AuthCopy,
    /// Store-lock, restore, and recovery copy.
    RecoveryCopy,
    /// First-run and onboarding assets.
    Onboarding,
}

impl TranslatedAssetFamily {
    /// All families the pack is required to cover.
    pub const ALL: [TranslatedAssetFamily; 6] = [
        TranslatedAssetFamily::Docs,
        TranslatedAssetFamily::GuidedTour,
        TranslatedAssetFamily::GlossaryCard,
        TranslatedAssetFamily::AuthCopy,
        TranslatedAssetFamily::RecoveryCopy,
        TranslatedAssetFamily::Onboarding,
    ];

    /// Returns a stable snake_case key for the family.
    pub const fn as_key(self) -> &'static str {
        match self {
            Self::Docs => "docs",
            Self::GuidedTour => "guided_tour",
            Self::GlossaryCard => "glossary_card",
            Self::AuthCopy => "auth_copy",
            Self::RecoveryCopy => "recovery_copy",
            Self::Onboarding => "onboarding",
        }
    }

    /// Returns true when the family carries safety-critical escalation routes.
    ///
    /// Auth and recovery copy must never drop escalation command routes under
    /// translation, fallback, or staleness.
    pub const fn is_escalation_critical(self) -> bool {
        matches!(self, Self::AuthCopy | Self::RecoveryCopy)
    }
}

/// Translation coverage represented by one translated asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetCoverageState {
    /// Requested locale has complete reviewed coverage for the current source.
    TranslatedComplete,
    /// Requested locale is reviewed for part of the asset.
    TranslatedPartial,
    /// Requested locale was reviewed against an older source revision.
    TranslatedStale,
}

impl AssetCoverageState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TranslatedComplete => "translated_complete",
            Self::TranslatedPartial => "translated_partial",
            Self::TranslatedStale => "translated_stale",
        }
    }

    /// Returns the badge class required for this coverage state.
    pub const fn required_badge(self) -> TranslationBadgeClass {
        match self {
            Self::TranslatedComplete => TranslationBadgeClass::Translated,
            Self::TranslatedPartial => TranslationBadgeClass::PartialTranslation,
            Self::TranslatedStale => TranslationBadgeClass::StaleTranslation,
        }
    }
}

/// Freshness of the translation basis relative to the live source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranslationFreshnessClass {
    /// Translation matches the current live source revision.
    CurrentWithLiveSource,
    /// Translation is served from a warm cache and may lag the live source.
    WarmCached,
    /// Translation basis is behind the current source revision.
    StaleBehindSource,
    /// Translation is pinned for offline or air-gapped use.
    OfflinePinned,
}

impl TranslationFreshnessClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentWithLiveSource => "current_with_live_source",
            Self::WarmCached => "warm_cached",
            Self::StaleBehindSource => "stale_behind_source",
            Self::OfflinePinned => "offline_pinned",
        }
    }

    /// Returns true when this freshness diverges from the current live source.
    ///
    /// Rows that diverge must render visibly distinct from live-source help so
    /// imported or stale translated help cannot masquerade as current truth.
    pub const fn diverges_from_live(self) -> bool {
        !matches!(self, Self::CurrentWithLiveSource)
    }
}

/// Mirror, cache, or offline posture for a translated asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorOfflinePosture {
    /// Asset is bundled with the local product or project pack.
    BuiltInLocal,
    /// Asset resolves through a verified mirror.
    Mirrored,
    /// Asset is pinned for offline or air-gapped use.
    OfflinePack,
    /// Asset resolves from a warm cache.
    Cached,
    /// Asset is online-only and requires an explicit handoff.
    LiveOnline,
    /// Asset is not installed locally; the surface falls back to source.
    NotInstalled,
}

impl MirrorOfflinePosture {
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

/// Revision-skew state between the source asset and the translation basis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlaySkewState {
    /// Translation basis matches the current source revision.
    NoSkew,
    /// The source revision is newer than the translation basis.
    SourceRevisionAhead,
}

impl OverlaySkewState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSkew => "no_skew",
            Self::SourceRevisionAhead => "source_revision_ahead",
        }
    }
}

/// User-visible badge class for a translated asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranslationBadgeClass {
    /// Complete reviewed translation.
    Translated,
    /// Partial translation with source-language continuity.
    PartialTranslation,
    /// Translation is stale relative to the source revision.
    StaleTranslation,
    /// Source-language text is active as fallback.
    SourceLanguageFallback,
}

impl TranslationBadgeClass {
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

/// Whether a rendered asset showed the requested locale or fell back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetLocalizationState {
    /// The requested locale supplied the rendered body.
    TranslatedRequestedLocale,
    /// No translation existed for the requested locale; source was shown.
    SourceLanguageFallback,
}

/// Stable refs a translated asset must preserve from its source asset.
///
/// These never localize: citation anchors, command ids, keyboard paths, and
/// scope labels are the machine-routable truth the translated prose merely
/// describes.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PreservedAssetRefs {
    /// Citation anchors preserved through translation.
    pub citation_anchor_refs: Vec<String>,
    /// Stable command ids preserved through translation.
    pub command_id_refs: Vec<String>,
    /// Keyboard path refs preserved through translation.
    pub keyboard_path_refs: Vec<String>,
    /// Scope label refs preserved through translation.
    pub scope_label_refs: Vec<String>,
}

impl PreservedAssetRefs {
    /// Returns true when citation, command, keyboard, and scope refs are present.
    pub fn is_complete(&self) -> bool {
        !self.citation_anchor_refs.is_empty()
            && !self.command_id_refs.is_empty()
            && !self.keyboard_path_refs.is_empty()
            && !self.scope_label_refs.is_empty()
    }
}

/// `Open in source language` escape hatch bound to one asset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLanguageEscapeHatch {
    /// Escape-hatch route class.
    pub hatch_kind: SourceLanguageEscapeHatchClass,
    /// User-facing action label; always [`OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL`].
    pub action_label: String,
    /// Stable command id that opens the source-language material.
    pub command_id_ref: String,
    /// Source-language target ref for exact reopen.
    pub source_target_ref: String,
    /// Keyboard path that reaches the escape hatch without a pointer.
    pub keyboard_path_ref: String,
    /// Whether the escape hatch is reachable without a pointer.
    pub keyboard_reachable: bool,
}

impl SourceLanguageEscapeHatch {
    /// Returns true when the escape hatch meets the source-language contract.
    pub fn is_valid(&self) -> bool {
        self.action_label == OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL
            && self.command_id_ref.starts_with("cmd:")
            && !self.source_target_ref.trim().is_empty()
            && !self.keyboard_path_ref.trim().is_empty()
            && self.keyboard_reachable
    }
}

/// Canonical source-language asset that translations preserve.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceHelpAsset {
    /// Stable, locale-neutral asset id.
    pub asset_id: String,
    /// Asset family.
    pub asset_family: TranslatedAssetFamily,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Source-language title.
    pub source_title: String,
    /// Current source revision for the asset.
    pub source_revision_ref: String,
    /// Repository-relative path to the source-language body.
    pub source_body_ref: String,
    /// Stable refs translations must preserve.
    pub preserved_refs: PreservedAssetRefs,
    /// Canonical source-language escape hatch for this asset.
    pub escape_hatch: SourceLanguageEscapeHatch,
}

/// One translated asset for a claimed locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslatedHelpAsset {
    /// Stable asset id; matches a [`SourceHelpAsset`].
    pub asset_id: String,
    /// Asset family.
    pub asset_family: TranslatedAssetFamily,
    /// Locale requested by the user.
    pub requested_locale: String,
    /// Locale actually rendered (equals the requested locale).
    pub effective_locale: String,
    /// Coverage state for this translation.
    pub coverage_state: AssetCoverageState,
    /// Freshness of the translation basis.
    pub freshness_class: TranslationFreshnessClass,
    /// Revision skew between source and translation basis.
    pub skew_state: OverlaySkewState,
    /// Mirror, cache, or offline posture.
    pub mirror_offline_posture: MirrorOfflinePosture,
    /// User-visible translation badge.
    pub badge_class: TranslationBadgeClass,
    /// Writing direction for the locale.
    pub text_direction: TextDirection,
    /// Current source revision for the asset.
    pub source_revision_ref: String,
    /// Translation revision rendered to the user.
    pub overlay_revision_ref: String,
    /// Source revision this translation was produced from.
    pub overlay_source_revision_ref: String,
    /// Translated title.
    pub translated_title: String,
    /// Repository-relative path to the translated body.
    pub translated_body_ref: String,
    /// Stable refs preserved from the source asset.
    pub preserved_refs: PreservedAssetRefs,
    /// Source-language escape hatch for this translation.
    pub escape_hatch: SourceLanguageEscapeHatch,
}

/// Summary posture derived from the pack rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslatedHelpPackSummary {
    /// Source assets defined by the pack.
    pub source_asset_count: usize,
    /// Asset families covered by the source assets.
    pub families_covered: usize,
    /// Total translated assets across all claimed locales.
    pub total_translations: usize,
    /// Translated-asset count per claimed locale.
    pub translations_by_locale: BTreeMap<String, usize>,
    /// Translated-asset count per family.
    pub translations_by_family: BTreeMap<String, usize>,
    /// Number of claimed locales.
    pub claimed_locales: usize,
}

/// Translated docs/tour/glossary/auth/recovery/onboarding pack for the claimed
/// localized M5 profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TranslatedHelpPack {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable pack id.
    pub pack_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Current build identity.
    pub target_build_identity_ref: String,
    /// Source contracts that govern this pack.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime consumers that ingest this pack.
    pub runtime_consumer_refs: Vec<String>,
    /// Claimed localized profiles, in stable order.
    pub claimed_locales: Vec<String>,
    /// Canonical source-language assets, sorted by asset id.
    pub source_assets: Vec<SourceHelpAsset>,
    /// Translated assets, sorted by asset id then locale.
    pub translations: Vec<TranslatedHelpAsset>,
    /// Summary posture derived from the rows.
    pub summary: TranslatedHelpPackSummary,
}

/// One rendered asset for a requested locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslatedHelpRenderRow {
    /// Stable asset id.
    pub asset_id: String,
    /// Asset family.
    pub asset_family: TranslatedAssetFamily,
    /// Requested locale.
    pub requested_locale: String,
    /// Locale that supplied the rendered body.
    pub effective_locale: String,
    /// Whether the row showed the requested locale or fell back to source.
    pub localization_state: AssetLocalizationState,
    /// User-visible translation badge.
    pub badge_class: TranslationBadgeClass,
    /// Freshness of the rendered body.
    pub freshness_class: TranslationFreshnessClass,
    /// Revision skew between source and rendered basis.
    pub skew_state: OverlaySkewState,
    /// Mirror, cache, or offline posture.
    pub mirror_offline_posture: MirrorOfflinePosture,
    /// Writing direction for the effective locale.
    pub text_direction: TextDirection,
    /// Title shown in the effective locale.
    pub display_title: String,
    /// Repository-relative path to the rendered body.
    pub body_ref: String,
    /// Current source revision for the asset.
    pub source_revision_ref: String,
    /// Source revision the rendered body was produced from.
    pub rendered_source_revision_ref: String,
    /// Stable refs carried by the row (preserved from the source asset).
    pub preserved_refs: PreservedAssetRefs,
    /// Source-language escape hatch for the row.
    pub escape_hatch: SourceLanguageEscapeHatch,
    /// Whether the row is visibly distinct from current live-source help.
    pub distinct_from_live_source: bool,
}

/// Summary posture for one rendered locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslatedHelpRenderSummary {
    /// Total rendered rows.
    pub total_rows: usize,
    /// Rows shown in the requested locale.
    pub translated_rows: usize,
    /// Rows that fell back to the source language.
    pub source_fallback_rows: usize,
    /// Rows visibly distinct from current live-source help.
    pub distinct_from_live_rows: usize,
    /// Asset ids the requested locale does not translate (marked explicitly).
    pub nontranslated_asset_ids: Vec<String>,
}

/// Locale-stable render of every help asset for one requested locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TranslatedHelpRender {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Source pack id.
    pub pack_id_ref: String,
    /// Current build identity.
    pub target_build_identity_ref: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Writing direction for the requested locale.
    pub text_direction: TextDirection,
    /// Rendered rows in source-asset order.
    pub rows: Vec<TranslatedHelpRenderRow>,
    /// Summary posture derived from the rows.
    pub summary: TranslatedHelpRenderSummary,
}

impl M5TranslatedHelpRender {
    /// Returns the ordered stable asset ids of the render.
    pub fn asset_ids(&self) -> Vec<String> {
        self.rows.iter().map(|row| row.asset_id.clone()).collect()
    }

    /// Returns the rendered row for an asset id, when present.
    pub fn row(&self, asset_id: &str) -> Option<&TranslatedHelpRenderRow> {
        self.rows.iter().find(|row| row.asset_id == asset_id)
    }
}

impl M5TranslatedHelpPack {
    /// Returns the source asset with `asset_id`.
    pub fn source_asset(&self, asset_id: &str) -> Option<&SourceHelpAsset> {
        self.source_assets
            .iter()
            .find(|asset| asset.asset_id == asset_id)
    }

    /// Returns the translation for an asset id and locale, if any.
    pub fn translation(&self, asset_id: &str, locale: &str) -> Option<&TranslatedHelpAsset> {
        self.translations
            .iter()
            .find(|asset| asset.asset_id == asset_id && asset.requested_locale == locale)
    }

    /// Renders every source asset for a requested locale.
    ///
    /// An asset is translated when the pack carries a translation for the
    /// requested locale; otherwise the row falls back to the source language and
    /// is counted in the render summary.
    pub fn render(&self, requested_locale: &str) -> M5TranslatedHelpRender {
        let rows: Vec<TranslatedHelpRenderRow> = self
            .source_assets
            .iter()
            .map(|asset| self.render_row(asset, requested_locale))
            .collect();

        let translated_rows = rows
            .iter()
            .filter(|row| {
                row.localization_state == AssetLocalizationState::TranslatedRequestedLocale
            })
            .count();
        let distinct_rows = rows
            .iter()
            .filter(|row| row.distinct_from_live_source)
            .count();
        let nontranslated: Vec<String> = rows
            .iter()
            .filter(|row| row.localization_state == AssetLocalizationState::SourceLanguageFallback)
            .map(|row| row.asset_id.clone())
            .collect();

        M5TranslatedHelpRender {
            record_kind: M5_TRANSLATED_HELP_RENDER_RECORD_KIND.to_owned(),
            schema_version: M5_TRANSLATED_HELP_PACK_SCHEMA_VERSION,
            pack_id_ref: self.pack_id.clone(),
            target_build_identity_ref: self.target_build_identity_ref.clone(),
            requested_locale: requested_locale.to_owned(),
            source_language_locale: self.source_language_locale.clone(),
            text_direction: TextDirection::for_locale(requested_locale),
            summary: TranslatedHelpRenderSummary {
                total_rows: rows.len(),
                translated_rows,
                source_fallback_rows: rows.len() - translated_rows,
                distinct_from_live_rows: distinct_rows,
                nontranslated_asset_ids: nontranslated,
            },
            rows,
        }
    }

    /// Renders one source asset for a requested locale.
    fn render_row(
        &self,
        asset: &SourceHelpAsset,
        requested_locale: &str,
    ) -> TranslatedHelpRenderRow {
        match self.translation(&asset.asset_id, requested_locale) {
            Some(translation) => TranslatedHelpRenderRow {
                asset_id: asset.asset_id.clone(),
                asset_family: asset.asset_family,
                requested_locale: requested_locale.to_owned(),
                effective_locale: translation.effective_locale.clone(),
                localization_state: AssetLocalizationState::TranslatedRequestedLocale,
                badge_class: translation.badge_class,
                freshness_class: translation.freshness_class,
                skew_state: translation.skew_state,
                mirror_offline_posture: translation.mirror_offline_posture,
                text_direction: translation.text_direction,
                display_title: translation.translated_title.clone(),
                body_ref: translation.translated_body_ref.clone(),
                source_revision_ref: translation.source_revision_ref.clone(),
                rendered_source_revision_ref: translation.overlay_source_revision_ref.clone(),
                preserved_refs: translation.preserved_refs.clone(),
                escape_hatch: translation.escape_hatch.clone(),
                distinct_from_live_source: translation.freshness_class.diverges_from_live(),
            },
            None => TranslatedHelpRenderRow {
                asset_id: asset.asset_id.clone(),
                asset_family: asset.asset_family,
                requested_locale: requested_locale.to_owned(),
                effective_locale: self.source_language_locale.clone(),
                localization_state: AssetLocalizationState::SourceLanguageFallback,
                badge_class: TranslationBadgeClass::SourceLanguageFallback,
                freshness_class: TranslationFreshnessClass::CurrentWithLiveSource,
                skew_state: OverlaySkewState::NoSkew,
                mirror_offline_posture: MirrorOfflinePosture::NotInstalled,
                text_direction: TextDirection::for_locale(&self.source_language_locale),
                display_title: asset.source_title.clone(),
                body_ref: asset.source_body_ref.clone(),
                source_revision_ref: asset.source_revision_ref.clone(),
                rendered_source_revision_ref: asset.source_revision_ref.clone(),
                preserved_refs: asset.preserved_refs.clone(),
                escape_hatch: asset.escape_hatch.clone(),
                // The source language is the live source, so it is not distinct.
                distinct_from_live_source: false,
            },
        }
    }

    /// Validates the pack shape and the translation-faithfulness invariants.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != M5_TRANSLATED_HELP_PACK_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.pack_id.clone(),
                "pack record_kind is unsupported",
            ));
        }
        if self.schema_version != M5_TRANSLATED_HELP_PACK_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.pack_id.clone(),
                "pack schema_version is unsupported",
            ));
        }
        if self.source_language_locale != SOURCE_LANGUAGE_LOCALE {
            findings.push(LocalePackValidationFinding::new(
                self.pack_id.clone(),
                "pack source-language locale is unexpected",
            ));
        }

        validate_source_assets(self, &mut findings);
        validate_translations(self, &mut findings);
        validate_summary(self, &mut findings);

        finish(findings)
    }
}

/// Per-locale parity row across one claimed locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranslatedHelpParityRow {
    /// Claimed locale.
    pub locale: String,
    /// Writing direction for the locale.
    pub text_direction: TextDirection,
    /// Rendered row count.
    pub rendered_row_count: usize,
    /// Whether the rendered asset-id sequence matches the source pack.
    pub asset_id_set_matches_source: bool,
    /// Rows shown in the requested locale.
    pub translated_count: usize,
    /// Rows that fell back to the source language.
    pub source_fallback_count: usize,
    /// Whether every row preserved its citation anchors from the source asset.
    pub citation_faithful: bool,
    /// Whether every row preserved its command ids from the source asset.
    pub command_faithful: bool,
    /// Whether every row preserved citation, command, keyboard, and scope refs.
    pub all_refs_preserved: bool,
    /// Whether every row exposes an `Open in source language` escape hatch.
    pub all_escape_hatches_present: bool,
    /// Whether every freshness-diverging row renders distinct from live source.
    pub stale_or_offline_distinct_from_live: bool,
    /// Whether escalation-critical auth/recovery rows keep their command routes.
    pub escalation_routes_preserved: bool,
    /// Asset ids this locale does not translate (marked explicitly).
    pub nontranslated_asset_ids: Vec<String>,
}

impl TranslatedHelpParityRow {
    /// Returns true when this locale meets every parity claim.
    pub fn is_parity_clean(&self) -> bool {
        self.asset_id_set_matches_source
            && self.all_refs_preserved
            && self.all_escape_hatches_present
            && self.stale_or_offline_distinct_from_live
            && self.escalation_routes_preserved
    }
}

/// Cross-locale parity report for the claimed localized M5 profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TranslatedHelpParityReport {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Current build identity.
    pub target_build_identity_ref: String,
    /// Source pack id.
    pub pack_id_ref: String,
    /// Claimed locales evaluated.
    pub claimed_locales: Vec<String>,
    /// Per-locale parity rows.
    pub rows: Vec<TranslatedHelpParityRow>,
    /// Whether every claimed locale meets every parity claim.
    pub parity_clean: bool,
}

impl M5TranslatedHelpParityReport {
    /// Returns the parity row for a locale, when present.
    pub fn row(&self, locale: &str) -> Option<&TranslatedHelpParityRow> {
        self.rows.iter().find(|row| row.locale == locale)
    }

    /// Validates the report shape and that it actually proves parity.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();
        if self.record_kind != M5_TRANSLATED_HELP_PARITY_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.report_id.clone(),
                "parity report record_kind is unsupported",
            ));
        }
        if self.schema_version != M5_TRANSLATED_HELP_PACK_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.report_id.clone(),
                "parity report schema_version is unsupported",
            ));
        }
        if self.rows.len() != self.claimed_locales.len() {
            findings.push(LocalePackValidationFinding::new(
                self.report_id.clone(),
                "parity report is missing a claimed-locale row",
            ));
        }
        let expected_clean = self
            .rows
            .iter()
            .all(TranslatedHelpParityRow::is_parity_clean);
        if self.parity_clean != expected_clean {
            findings.push(LocalePackValidationFinding::new(
                self.report_id.clone(),
                "parity_clean disagrees with the per-locale rows",
            ));
        }
        for row in &self.rows {
            if !row.is_parity_clean() {
                findings.push(LocalePackValidationFinding::new(
                    row.locale.clone(),
                    "claimed locale fails a parity claim",
                ));
            }
        }
        finish(findings)
    }
}

/// Builds the parity report for a pack across its claimed locales.
pub fn build_translated_help_parity_report(
    pack: &M5TranslatedHelpPack,
) -> M5TranslatedHelpParityReport {
    let source_ids: Vec<String> = pack
        .source_assets
        .iter()
        .map(|asset| asset.asset_id.clone())
        .collect();

    let rows: Vec<TranslatedHelpParityRow> = pack
        .claimed_locales
        .iter()
        .map(|locale| build_parity_row(pack, locale, &source_ids))
        .collect();

    let parity_clean = rows.iter().all(TranslatedHelpParityRow::is_parity_clean);

    M5TranslatedHelpParityReport {
        record_kind: M5_TRANSLATED_HELP_PARITY_RECORD_KIND.to_owned(),
        schema_version: M5_TRANSLATED_HELP_PACK_SCHEMA_VERSION,
        report_id: M5_TRANSLATED_HELP_PARITY_REPORT_ID.to_owned(),
        generated_at: pack.generated_at.clone(),
        source_language_locale: pack.source_language_locale.clone(),
        target_build_identity_ref: pack.target_build_identity_ref.clone(),
        pack_id_ref: pack.pack_id.clone(),
        claimed_locales: pack.claimed_locales.clone(),
        rows,
        parity_clean,
    }
}

fn build_parity_row(
    pack: &M5TranslatedHelpPack,
    locale: &str,
    source_ids: &[String],
) -> TranslatedHelpParityRow {
    let render = pack.render(locale);

    let mut citation_faithful = true;
    let mut command_faithful = true;
    let mut keyboard_faithful = true;
    let mut scope_faithful = true;
    let mut all_escape_hatches_present = true;
    let mut distinct_coherent = true;
    let mut escalation_routes_preserved = true;

    for row in &render.rows {
        let Some(source) = pack.source_asset(&row.asset_id) else {
            citation_faithful = false;
            continue;
        };
        let source_refs = &source.preserved_refs;
        if row.preserved_refs.citation_anchor_refs != source_refs.citation_anchor_refs {
            citation_faithful = false;
        }
        if row.preserved_refs.command_id_refs != source_refs.command_id_refs {
            command_faithful = false;
        }
        if row.preserved_refs.keyboard_path_refs != source_refs.keyboard_path_refs {
            keyboard_faithful = false;
        }
        if row.preserved_refs.scope_label_refs != source_refs.scope_label_refs {
            scope_faithful = false;
        }
        if !row.escape_hatch.is_valid() {
            all_escape_hatches_present = false;
        }
        if row.freshness_class.diverges_from_live() && !row.distinct_from_live_source {
            distinct_coherent = false;
        }
        if row.asset_family.is_escalation_critical()
            && row.preserved_refs.command_id_refs.is_empty()
        {
            escalation_routes_preserved = false;
        }
    }

    let all_refs_preserved =
        citation_faithful && command_faithful && keyboard_faithful && scope_faithful;

    TranslatedHelpParityRow {
        locale: locale.to_owned(),
        text_direction: TextDirection::for_locale(locale),
        rendered_row_count: render.rows.len(),
        asset_id_set_matches_source: render.asset_ids() == source_ids,
        translated_count: render.summary.translated_rows,
        source_fallback_count: render.summary.source_fallback_rows,
        citation_faithful,
        command_faithful,
        all_refs_preserved,
        all_escape_hatches_present,
        stale_or_offline_distinct_from_live: distinct_coherent,
        escalation_routes_preserved,
        nontranslated_asset_ids: render.summary.nontranslated_asset_ids,
    }
}

/// Returns the seeded translated-help pack for the claimed M5 profiles.
pub fn seeded_m5_translated_help_pack() -> M5TranslatedHelpPack {
    let source_assets = seeded_source_assets();
    let translations = seeded_translations();
    let summary = derive_summary(&source_assets, &translations, CLAIMED_LOCALES.len());

    M5TranslatedHelpPack {
        record_kind: M5_TRANSLATED_HELP_PACK_RECORD_KIND.to_owned(),
        schema_version: M5_TRANSLATED_HELP_PACK_SCHEMA_VERSION,
        pack_id: M5_TRANSLATED_HELP_PACK_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        target_build_identity_ref: TARGET_BUILD.to_owned(),
        source_contract_refs: seeded_source_contract_refs(),
        runtime_consumer_refs: seeded_runtime_consumer_refs(),
        claimed_locales: CLAIMED_LOCALES.iter().map(|l| (*l).to_owned()).collect(),
        source_assets,
        translations,
        summary,
    }
}

/// Renders the seeded pack for a requested locale.
pub fn seeded_m5_translated_help_render(requested_locale: &str) -> M5TranslatedHelpRender {
    seeded_m5_translated_help_pack().render(requested_locale)
}

/// Returns the seeded translated-help parity report for the claimed profiles.
pub fn seeded_m5_translated_help_parity_report() -> M5TranslatedHelpParityReport {
    build_translated_help_parity_report(&seeded_m5_translated_help_pack())
}

/// Returns source contracts that govern the pack.
fn seeded_source_contract_refs() -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "architecture_localization".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#23.3.1".to_owned(),
        ),
        (
            "design_localization_governance".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#8.10".to_owned(),
        ),
        (
            "terminology_governance".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md#20.7".to_owned(),
        ),
        (
            "pack_schema".to_owned(),
            "schemas/help/translated-doc-pack.schema.json".to_owned(),
        ),
        (
            "parity_schema".to_owned(),
            "schemas/help/translated-doc-pack-parity.schema.json".to_owned(),
        ),
    ])
}

/// Returns runtime consumers that ingest the pack.
fn seeded_runtime_consumer_refs() -> Vec<String> {
    vec![
        "crates/aureline-i18n".to_owned(),
        "crates/aureline-docs".to_owned(),
        "crates/aureline-auth".to_owned(),
        "crates/aureline-help".to_owned(),
        "crates/aureline-onboarding".to_owned(),
        "crates/aureline-support".to_owned(),
    ]
}

/// Builds the seeded source assets, one per family.
fn seeded_source_assets() -> Vec<SourceHelpAsset> {
    let mut assets: Vec<SourceHelpAsset> = source_specs()
        .iter()
        .map(|spec| SourceHelpAsset {
            asset_id: spec.asset_id.to_owned(),
            asset_family: spec.asset_family,
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            source_title: spec.source_title.to_owned(),
            source_revision_ref: spec.source_revision_ref.to_owned(),
            source_body_ref: source_body_ref(spec.asset_family, spec.body_name),
            preserved_refs: preserved_refs(spec),
            escape_hatch: escape_hatch(spec),
        })
        .collect();
    assets.sort_by(|left, right| left.asset_id.cmp(&right.asset_id));
    assets
}

/// Builds the seeded translations across the claimed locales.
fn seeded_translations() -> Vec<TranslatedHelpAsset> {
    let specs = source_specs();
    let by_id: BTreeMap<&str, &SourceSpec> = specs.iter().map(|s| (s.asset_id, s)).collect();

    let mut translations: Vec<TranslatedHelpAsset> = translation_specs()
        .iter()
        .map(|spec| {
            let source = by_id
                .get(spec.asset_id)
                .expect("translation references a known source asset");
            build_translation(spec, source)
        })
        .collect();

    translations.sort_by(|left, right| {
        left.asset_id
            .cmp(&right.asset_id)
            .then(left.requested_locale.cmp(&right.requested_locale))
    });
    translations
}

fn build_translation(spec: &TranslationSpec, source: &SourceSpec) -> TranslatedHelpAsset {
    let coverage = spec.coverage_state;
    let overlay_source_revision_ref = match coverage {
        AssetCoverageState::TranslatedStale => spec
            .stale_source_revision_ref
            .expect("stale translation must name an older source revision")
            .to_owned(),
        _ => source.source_revision_ref.to_owned(),
    };
    TranslatedHelpAsset {
        asset_id: spec.asset_id.to_owned(),
        asset_family: source.asset_family,
        requested_locale: spec.locale.to_owned(),
        effective_locale: spec.locale.to_owned(),
        coverage_state: coverage,
        freshness_class: spec.freshness_class,
        skew_state: match coverage {
            AssetCoverageState::TranslatedStale => OverlaySkewState::SourceRevisionAhead,
            _ => OverlaySkewState::NoSkew,
        },
        mirror_offline_posture: spec.mirror_offline_posture,
        badge_class: coverage.required_badge(),
        text_direction: TextDirection::for_locale(spec.locale),
        source_revision_ref: source.source_revision_ref.to_owned(),
        overlay_revision_ref: format!(
            "help-overlay-rev:{}:{}:2026.05.18-01",
            source.body_name, spec.locale
        ),
        overlay_source_revision_ref,
        translated_title: spec.translated_title.to_owned(),
        translated_body_ref: translated_body_ref(
            source.asset_family,
            source.body_name,
            spec.locale,
        ),
        preserved_refs: preserved_refs(source),
        escape_hatch: escape_hatch(source),
    }
}

/// Derives the pack summary from its rows.
fn derive_summary(
    source_assets: &[SourceHelpAsset],
    translations: &[TranslatedHelpAsset],
    claimed_locales: usize,
) -> TranslatedHelpPackSummary {
    let families: BTreeSet<&'static str> = source_assets
        .iter()
        .map(|asset| asset.asset_family.as_key())
        .collect();
    let mut translations_by_locale = BTreeMap::new();
    let mut translations_by_family = BTreeMap::new();
    for translation in translations {
        *translations_by_locale
            .entry(translation.requested_locale.clone())
            .or_insert(0usize) += 1;
        *translations_by_family
            .entry(translation.asset_family.as_key().to_owned())
            .or_insert(0usize) += 1;
    }
    TranslatedHelpPackSummary {
        source_asset_count: source_assets.len(),
        families_covered: families.len(),
        total_translations: translations.len(),
        translations_by_locale,
        translations_by_family,
        claimed_locales,
    }
}

/// Compact spec for a seeded source asset.
#[derive(Clone)]
struct SourceSpec {
    asset_id: &'static str,
    asset_family: TranslatedAssetFamily,
    body_name: &'static str,
    source_title: &'static str,
    source_revision_ref: &'static str,
    citation_anchor_refs: &'static [&'static str],
    command_id_refs: &'static [&'static str],
    keyboard_path_refs: &'static [&'static str],
    scope_label_refs: &'static [&'static str],
    escape_hatch_kind: SourceLanguageEscapeHatchClass,
    escape_command_id_ref: &'static str,
    escape_keyboard_path_ref: &'static str,
}

/// Compact spec for a seeded translation.
struct TranslationSpec {
    asset_id: &'static str,
    locale: &'static str,
    translated_title: &'static str,
    coverage_state: AssetCoverageState,
    freshness_class: TranslationFreshnessClass,
    mirror_offline_posture: MirrorOfflinePosture,
    stale_source_revision_ref: Option<&'static str>,
}

fn preserved_refs(spec: &SourceSpec) -> PreservedAssetRefs {
    PreservedAssetRefs {
        citation_anchor_refs: to_owned_vec(spec.citation_anchor_refs),
        command_id_refs: to_owned_vec(spec.command_id_refs),
        keyboard_path_refs: to_owned_vec(spec.keyboard_path_refs),
        scope_label_refs: to_owned_vec(spec.scope_label_refs),
    }
}

fn escape_hatch(spec: &SourceSpec) -> SourceLanguageEscapeHatch {
    SourceLanguageEscapeHatch {
        hatch_kind: spec.escape_hatch_kind,
        action_label: OPEN_IN_SOURCE_LANGUAGE_ACTION_LABEL.to_owned(),
        command_id_ref: spec.escape_command_id_ref.to_owned(),
        source_target_ref: format!(
            "source-language:{}#{}",
            source_body_ref(spec.asset_family, spec.body_name),
            SOURCE_LANGUAGE_LOCALE
        ),
        keyboard_path_ref: spec.escape_keyboard_path_ref.to_owned(),
        keyboard_reachable: true,
    }
}

fn to_owned_vec(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

/// Returns the repository-relative source body path for an asset.
fn source_body_ref(family: TranslatedAssetFamily, body_name: &str) -> String {
    format!(
        "docs/help/locales/{}/{}/{}.md",
        SOURCE_LANGUAGE_LOCALE,
        family.as_key(),
        body_name
    )
}

/// Returns the repository-relative translated body path for an asset.
fn translated_body_ref(family: TranslatedAssetFamily, body_name: &str, locale: &str) -> String {
    format!(
        "docs/help/locales/{}/{}/{}.md",
        locale,
        family.as_key(),
        body_name
    )
}

/// Collapses findings into a `Result`.
fn finish(
    findings: Vec<LocalePackValidationFinding>,
) -> Result<(), Vec<LocalePackValidationFinding>> {
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn validate_source_assets(
    pack: &M5TranslatedHelpPack,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let mut ids = BTreeSet::new();
    let mut families = BTreeSet::new();
    for asset in &pack.source_assets {
        if !ids.insert(asset.asset_id.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                asset.asset_id.clone(),
                "duplicate source asset id",
            ));
        }
        families.insert(asset.asset_family);
        if asset.source_language_locale != pack.source_language_locale {
            findings.push(LocalePackValidationFinding::new(
                asset.asset_id.clone(),
                "source asset locale differs from the pack source language",
            ));
        }
        if !asset.preserved_refs.is_complete() {
            findings.push(LocalePackValidationFinding::new(
                asset.asset_id.clone(),
                "source asset is missing a citation, command, keyboard, or scope ref",
            ));
        }
        if !asset.escape_hatch.is_valid() {
            findings.push(LocalePackValidationFinding::new(
                asset.asset_id.clone(),
                "source asset escape hatch is not a valid open-in-source-language route",
            ));
        }
        if asset.asset_family.is_escalation_critical()
            && asset.preserved_refs.command_id_refs.is_empty()
        {
            findings.push(LocalePackValidationFinding::new(
                asset.asset_id.clone(),
                "escalation-critical source asset must carry command routes",
            ));
        }
    }
    for required in TranslatedAssetFamily::ALL {
        if !families.contains(&required) {
            findings.push(LocalePackValidationFinding::new(
                pack.pack_id.clone(),
                format!("pack is missing family {}", required.as_key()),
            ));
        }
    }
}

fn validate_translations(
    pack: &M5TranslatedHelpPack,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let mut seen = BTreeSet::new();
    for translation in &pack.translations {
        let pair = (
            translation.asset_id.as_str(),
            translation.requested_locale.as_str(),
        );
        if !seen.insert(pair) {
            findings.push(LocalePackValidationFinding::new(
                translation.asset_id.clone(),
                "pack repeats a (asset_id, locale) translation",
            ));
        }
        if !pack.claimed_locales.contains(&translation.requested_locale) {
            findings.push(LocalePackValidationFinding::new(
                translation.asset_id.clone(),
                "translation targets a locale outside the claimed set",
            ));
        }
        if translation.effective_locale != translation.requested_locale {
            findings.push(LocalePackValidationFinding::new(
                translation.asset_id.clone(),
                "translation effective locale must equal the requested locale",
            ));
        }
        if translation.text_direction != TextDirection::for_locale(&translation.requested_locale) {
            findings.push(LocalePackValidationFinding::new(
                translation.asset_id.clone(),
                "translation text_direction does not match its locale",
            ));
        }
        if translation.badge_class != translation.coverage_state.required_badge() {
            findings.push(LocalePackValidationFinding::new(
                translation.asset_id.clone(),
                "translation badge does not match coverage state",
            ));
        }
        if !translation.escape_hatch.is_valid() {
            findings.push(LocalePackValidationFinding::new(
                translation.asset_id.clone(),
                "translation escape hatch is not a valid open-in-source-language route",
            ));
        }
        match pack.source_asset(&translation.asset_id) {
            None => findings.push(LocalePackValidationFinding::new(
                translation.asset_id.clone(),
                "translation references an unknown source asset",
            )),
            Some(source) => {
                if translation.asset_family != source.asset_family {
                    findings.push(LocalePackValidationFinding::new(
                        translation.asset_id.clone(),
                        "translation family differs from its source asset",
                    ));
                }
                if translation.preserved_refs != source.preserved_refs {
                    findings.push(LocalePackValidationFinding::new(
                        translation.asset_id.clone(),
                        "translation drops or rewrites a citation, command, keyboard, or scope ref",
                    ));
                }
                if translation.escape_hatch != source.escape_hatch {
                    findings.push(LocalePackValidationFinding::new(
                        translation.asset_id.clone(),
                        "translation escape hatch drifted from the source asset",
                    ));
                }
                if translation.source_revision_ref != source.source_revision_ref {
                    findings.push(LocalePackValidationFinding::new(
                        translation.asset_id.clone(),
                        "translation cites a stale current-source revision",
                    ));
                }
            }
        }
        validate_translation_freshness(translation, findings);
    }
}

fn validate_translation_freshness(
    translation: &TranslatedHelpAsset,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    match translation.coverage_state {
        AssetCoverageState::TranslatedComplete => {
            if translation.skew_state != OverlaySkewState::NoSkew
                || translation.overlay_source_revision_ref != translation.source_revision_ref
                || translation.freshness_class.diverges_from_live()
            {
                findings.push(LocalePackValidationFinding::new(
                    translation.asset_id.clone(),
                    "complete translation must match the current live source",
                ));
            }
        }
        AssetCoverageState::TranslatedPartial => {
            if translation.skew_state != OverlaySkewState::NoSkew
                || translation.overlay_source_revision_ref != translation.source_revision_ref
            {
                findings.push(LocalePackValidationFinding::new(
                    translation.asset_id.clone(),
                    "partial translation must track the current source revision",
                ));
            }
        }
        AssetCoverageState::TranslatedStale => {
            if translation.skew_state != OverlaySkewState::SourceRevisionAhead
                || translation.freshness_class != TranslationFreshnessClass::StaleBehindSource
                || translation.overlay_source_revision_ref == translation.source_revision_ref
            {
                findings.push(LocalePackValidationFinding::new(
                    translation.asset_id.clone(),
                    "stale translation must name source/overlay skew and stale freshness",
                ));
            }
        }
    }
}

fn validate_summary(pack: &M5TranslatedHelpPack, findings: &mut Vec<LocalePackValidationFinding>) {
    let expected = derive_summary(
        &pack.source_assets,
        &pack.translations,
        pack.claimed_locales.len(),
    );
    if pack.summary != expected {
        findings.push(LocalePackValidationFinding::new(
            pack.pack_id.clone(),
            "pack summary does not match the derived rows",
        ));
    }
}

/// Returns the seeded source-asset specs, one per family.
fn source_specs() -> Vec<SourceSpec> {
    use SourceLanguageEscapeHatchClass::*;
    use TranslatedAssetFamily::*;

    vec![
        SourceSpec {
            asset_id: "asset:docs:getting-started",
            asset_family: Docs,
            body_name: "getting-started",
            source_title: "Getting started",
            source_revision_ref: "help-source-rev:getting-started:2026.05.18-01",
            citation_anchor_refs: &[
                "citation:docs-pack:getting-started:overview",
                "citation:docs-pack:getting-started:open-folder",
            ],
            command_id_refs: &["cmd:workspace.open_folder", "cmd:command_palette.open"],
            keyboard_path_refs: &[
                "keyboard:path:workspace.open_folder",
                "keyboard:path:command_palette.open",
            ],
            scope_label_refs: &["scope:docs-help"],
            escape_hatch_kind: DocsPaneSourceLanguageRoute,
            escape_command_id_ref: "cmd:help.open_in_source_language",
            escape_keyboard_path_ref: "keyboard:path:help.open_in_source_language",
        },
        SourceSpec {
            asset_id: "asset:tour:first-build",
            asset_family: GuidedTour,
            body_name: "first-build",
            source_title: "Run your first build",
            source_revision_ref: "help-source-rev:first-build:2026.05.18-01",
            citation_anchor_refs: &["citation:tour:first-build:run-build"],
            command_id_refs: &["cmd:workbench.action.tasks.runBuild"],
            keyboard_path_refs: &["keyboard:path:tasks.runBuild"],
            scope_label_refs: &["scope:guided-tour"],
            escape_hatch_kind: DocsPaneSourceLanguageRoute,
            escape_command_id_ref: "cmd:help.open_in_source_language",
            escape_keyboard_path_ref: "keyboard:path:help.open_in_source_language",
        },
        SourceSpec {
            asset_id: "asset:glossary:truth-source",
            asset_family: GlossaryCard,
            body_name: "truth-source",
            source_title: "Truth source",
            source_revision_ref: "help-source-rev:truth-source:2026.05.18-01",
            citation_anchor_refs: &[
                "citation:glossary:truth-source:definition",
                "citation:docs-help:truth_source_model:source",
            ],
            command_id_refs: &["cmd:docs.open_in_browser"],
            keyboard_path_refs: &["keyboard:path:glossary.open_related"],
            scope_label_refs: &["scope:glossary"],
            escape_hatch_kind: InlineSourceLanguageToggle,
            escape_command_id_ref: "cmd:help.open_in_source_language",
            escape_keyboard_path_ref: "keyboard:path:help.open_in_source_language",
        },
        SourceSpec {
            asset_id: "asset:auth:sign-in",
            asset_family: AuthCopy,
            body_name: "sign-in",
            source_title: "Sign in to your account",
            source_revision_ref: "help-source-rev:sign-in:2026.05.18-01",
            citation_anchor_refs: &[
                "citation:auth:sign-in:overview",
                "citation:auth:sign-in:escalation",
            ],
            command_id_refs: &["cmd:auth.sign_in", "cmd:auth.contact_support"],
            keyboard_path_refs: &["keyboard:path:auth.sign_in"],
            scope_label_refs: &["scope:auth"],
            escape_hatch_kind: InlineSourceLanguageToggle,
            escape_command_id_ref: "cmd:help.open_in_source_language",
            escape_keyboard_path_ref: "keyboard:path:help.open_in_source_language",
        },
        SourceSpec {
            asset_id: "asset:recovery:restore-checkpoint",
            asset_family: RecoveryCopy,
            body_name: "restore-checkpoint",
            source_title: "Restore from a checkpoint",
            source_revision_ref: "help-source-rev:restore-checkpoint:2026.05.18-01",
            citation_anchor_refs: &[
                "citation:recovery:restore-checkpoint:source",
                "citation:recovery:restore-checkpoint:escalation",
            ],
            command_id_refs: &[
                "cmd:workspace.restore_from_checkpoint",
                "cmd:support.open_recovery_runbook",
            ],
            keyboard_path_refs: &["keyboard:path:workspace.restore_from_checkpoint"],
            scope_label_refs: &["scope:recovery"],
            escape_hatch_kind: ExportInSourceLanguageForReview,
            escape_command_id_ref: "cmd:help.open_in_source_language",
            escape_keyboard_path_ref: "keyboard:path:help.open_in_source_language",
        },
        SourceSpec {
            asset_id: "asset:onboarding:keymap-bridge",
            asset_family: Onboarding,
            body_name: "keymap-bridge",
            source_title: "Bring your keymap",
            source_revision_ref: "help-source-rev:keymap-bridge:2026.05.18-01",
            citation_anchor_refs: &["citation:onboarding:keymap-bridge:source"],
            command_id_refs: &["cmd:command_palette.open"],
            keyboard_path_refs: &["keyboard:path:command_palette.open"],
            scope_label_refs: &["scope:onboarding"],
            escape_hatch_kind: DocsPaneSourceLanguageRoute,
            escape_command_id_ref: "cmd:help.open_in_source_language",
            escape_keyboard_path_ref: "keyboard:path:help.open_in_source_language",
        },
    ]
}

/// Returns the seeded translation specs across the claimed locales.
///
/// `es-MX` is fully translated; `ja-JP` and `ar-SA` are partial, leaving some
/// assets to source-language fallback. `ar-SA` carries a stale recovery card to
/// exercise the freshness-divergence and escalation-preservation proofs.
fn translation_specs() -> Vec<TranslationSpec> {
    use AssetCoverageState::*;
    use MirrorOfflinePosture::*;
    use TranslationFreshnessClass::*;

    vec![
        // es-MX: every asset, complete, current.
        TranslationSpec {
            asset_id: "asset:docs:getting-started",
            locale: "es-MX",
            translated_title: "Primeros pasos",
            coverage_state: TranslatedComplete,
            freshness_class: CurrentWithLiveSource,
            mirror_offline_posture: BuiltInLocal,
            stale_source_revision_ref: None,
        },
        TranslationSpec {
            asset_id: "asset:tour:first-build",
            locale: "es-MX",
            translated_title: "Ejecuta tu primera compilación",
            coverage_state: TranslatedComplete,
            freshness_class: CurrentWithLiveSource,
            mirror_offline_posture: Mirrored,
            stale_source_revision_ref: None,
        },
        TranslationSpec {
            asset_id: "asset:glossary:truth-source",
            locale: "es-MX",
            translated_title: "Fuente de verdad",
            coverage_state: TranslatedComplete,
            freshness_class: CurrentWithLiveSource,
            mirror_offline_posture: OfflinePack,
            stale_source_revision_ref: None,
        },
        TranslationSpec {
            asset_id: "asset:auth:sign-in",
            locale: "es-MX",
            translated_title: "Inicia sesión en tu cuenta",
            coverage_state: TranslatedComplete,
            freshness_class: CurrentWithLiveSource,
            mirror_offline_posture: BuiltInLocal,
            stale_source_revision_ref: None,
        },
        TranslationSpec {
            asset_id: "asset:recovery:restore-checkpoint",
            locale: "es-MX",
            translated_title: "Restaurar desde un punto de control",
            coverage_state: TranslatedComplete,
            freshness_class: CurrentWithLiveSource,
            mirror_offline_posture: BuiltInLocal,
            stale_source_revision_ref: None,
        },
        TranslationSpec {
            asset_id: "asset:onboarding:keymap-bridge",
            locale: "es-MX",
            translated_title: "Trae tu mapa de teclado",
            coverage_state: TranslatedComplete,
            freshness_class: CurrentWithLiveSource,
            mirror_offline_posture: Mirrored,
            stale_source_revision_ref: None,
        },
        // ja-JP: docs, glossary, onboarding; tour/auth/recovery fall back.
        TranslationSpec {
            asset_id: "asset:docs:getting-started",
            locale: "ja-JP",
            translated_title: "はじめに",
            coverage_state: TranslatedComplete,
            freshness_class: CurrentWithLiveSource,
            mirror_offline_posture: Mirrored,
            stale_source_revision_ref: None,
        },
        TranslationSpec {
            asset_id: "asset:glossary:truth-source",
            locale: "ja-JP",
            translated_title: "真実のソース",
            coverage_state: TranslatedPartial,
            freshness_class: WarmCached,
            mirror_offline_posture: Cached,
            stale_source_revision_ref: None,
        },
        TranslationSpec {
            asset_id: "asset:onboarding:keymap-bridge",
            locale: "ja-JP",
            translated_title: "キーマップを引き継ぐ",
            coverage_state: TranslatedComplete,
            freshness_class: CurrentWithLiveSource,
            mirror_offline_posture: OfflinePack,
            stale_source_revision_ref: None,
        },
        // ar-SA: docs and auth complete, recovery stale; the rest fall back.
        TranslationSpec {
            asset_id: "asset:docs:getting-started",
            locale: "ar-SA",
            translated_title: "البدء",
            coverage_state: TranslatedComplete,
            freshness_class: CurrentWithLiveSource,
            mirror_offline_posture: Mirrored,
            stale_source_revision_ref: None,
        },
        TranslationSpec {
            asset_id: "asset:auth:sign-in",
            locale: "ar-SA",
            translated_title: "تسجيل الدخول إلى حسابك",
            coverage_state: TranslatedComplete,
            freshness_class: CurrentWithLiveSource,
            mirror_offline_posture: BuiltInLocal,
            stale_source_revision_ref: None,
        },
        TranslationSpec {
            asset_id: "asset:recovery:restore-checkpoint",
            locale: "ar-SA",
            translated_title: "الاستعادة من نقطة تحقّق",
            coverage_state: TranslatedStale,
            freshness_class: StaleBehindSource,
            mirror_offline_posture: OfflinePack,
            stale_source_revision_ref: Some("help-source-rev:restore-checkpoint:2026.05.10-01"),
        },
    ]
}

#[cfg(test)]
mod tests;
