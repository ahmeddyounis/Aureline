//! Localized prose catalog and locale-stable render for the new M5 shell,
//! command, settings, help, error, and notification surfaces.
//!
//! The [`message_registry`](crate::message_registry) proves which surfaces own
//! which stable message ids and *whether* a locale carries a translation. This
//! module carries the missing half: the **actual translated display strings**
//! for the claimed localized profiles, bound to those same stable ids, plus a
//! render that proves localizing the prose never moves a command id, telemetry
//! key, policy name, diagnostic id, setting id, or docs-pack key.
//!
//! Three release-bearing claims are made testable here rather than reviewed by
//! hand:
//!
//! - **Ids and routing survive localization.** [`M5LocalizedCatalog::render`]
//!   joins the catalog onto the registry and returns one
//!   [`LocalizedRenderRow`] per message. Across every requested locale the row
//!   sequence, the stable message ids, and the [`StableMessageIdentityRefs`] are
//!   identical; only the visible prose, effective locale, text direction, and
//!   source-language fallback flag vary.
//! - **Truncation and zoom never hide scope or severity.**
//!   [`LocalizedRenderRow::truncate`] cuts the visible prose to a grapheme
//!   budget while carrying the surface scope and [`RenderSeverityClass`] through
//!   untouched — severity and scope live in metadata, not in the truncatable
//!   string, so a tighter budget (a zoomed display) can shorten the label but
//!   cannot drop the fact that a row is an error or a notice.
//! - **Coverage is honest.** Every claimed-locale translation preserves the
//!   message's placeholders, and any message a locale does not translate is
//!   marked explicitly and rendered in the source language rather than hidden.
//!
//! Translated bodies live in this catalog (it *is* the first-party locale-pack
//! content) and in the per-locale render. The metadata-only support projections
//! downstream omit them; this module never carries signing keys or credentials.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    seeded_m5_message_registry, LocalePackValidationFinding, M5MessageRegistry, M5MessageSurface,
    MessageRegistryEntry, StableMessageIdentityRefs, GENERATED_AT, SOURCE_LANGUAGE_LOCALE,
    TARGET_BUILD,
};

/// Schema version for the localized catalog, render, and parity records.
pub const M5_LOCALIZED_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`M5LocalizedCatalog`].
pub const M5_LOCALIZED_CATALOG_RECORD_KIND: &str = "m5_localized_catalog_packet";

/// Record kind for [`M5LocalizedRender`].
pub const M5_LOCALIZED_RENDER_RECORD_KIND: &str = "m5_localized_render_packet";

/// Record kind for [`M5LocalizationParityReport`].
pub const M5_LOCALIZATION_PARITY_RECORD_KIND: &str = "m5_localization_parity_report";

/// Stable packet id for the seeded localized catalog.
pub const M5_LOCALIZED_CATALOG_PACKET_ID: &str =
    "i18n:m5-localized-catalog:shell-command-settings-help-error-notification:v1";

/// Stable report id for the seeded localization parity report.
pub const M5_LOCALIZATION_PARITY_REPORT_ID: &str =
    "i18n:m5-localization-parity:claimed-profiles:v1";

/// Fixture path for the seeded localized catalog.
pub const M5_LOCALIZED_CATALOG_FIXTURE_REF: &str =
    "fixtures/i18n/shell-command-help/localized-catalog.json";

/// Fixture path for the seeded localization parity report.
pub const M5_LOCALIZATION_PARITY_FIXTURE_REF: &str =
    "fixtures/i18n/shell-command-help/localization-parity.json";

/// Default visible-grapheme budget the truncation proof renders against.
///
/// A small budget stands in for the tightest realistic chrome slot (a narrow
/// status area or a zoomed display); the proof shows severity and scope survive
/// even here.
pub const DEFAULT_TRUNCATION_BUDGET_GRAPHEMES: usize = 18;

/// Claimed localized M5 profiles this batch governs.
///
/// Source language (`en-US`) is authoritative and never appears here; these are
/// the requested locales whose translated prose the catalog ships.
pub const CLAIMED_LOCALES: [&str; 3] = ["es-MX", "ja-JP", "ar-SA"];

/// Writing direction for a localized surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextDirection {
    /// Left-to-right script.
    LeftToRight,
    /// Right-to-left script (bidi surfaces).
    RightToLeft,
}

impl TextDirection {
    /// Returns the writing direction for a locale tag from its language base.
    pub fn for_locale(locale: &str) -> Self {
        match locale_base(locale) {
            "ar" | "he" | "fa" | "ur" => Self::RightToLeft,
            _ => Self::LeftToRight,
        }
    }
}

/// Scope-bearing severity a row must keep visible through truncation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderSeverityClass {
    /// Plain informational label (chrome, command, setting, help).
    Neutral,
    /// Notice-level prose (notifications).
    Notice,
    /// Warning-level prose (errors, denials, disabled-state explanations).
    Warning,
}

impl RenderSeverityClass {
    /// Returns the severity carried by a message on a given surface.
    ///
    /// Severity follows the surface: errors and disabled-state explanations are
    /// warnings, notifications are notices, everything else is neutral.
    pub const fn for_surface(surface: M5MessageSurface) -> Self {
        match surface {
            M5MessageSurface::Error => Self::Warning,
            M5MessageSurface::Notification => Self::Notice,
            M5MessageSurface::ShellChrome
            | M5MessageSurface::CommandPalette
            | M5MessageSurface::Settings
            | M5MessageSurface::Help => Self::Neutral,
        }
    }

    /// Returns true when severity must remain visible regardless of truncation.
    pub const fn is_disclosable(self) -> bool {
        !matches!(self, Self::Neutral)
    }
}

/// Whether a rendered row showed the requested locale or fell back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalizationRenderState {
    /// The requested locale (or its language base) supplied the prose.
    Localized,
    /// No translation existed, so the source language was shown.
    SourceLanguageFallback,
}

/// One translated display string bound to a stable message id and locale.
///
/// The catalog holds exactly the `(message_id, locale)` pairs the registry
/// declares translated, so the prose set cannot drift from the coverage truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedStringEntry {
    /// Stable, locale-neutral message id this string renders.
    pub message_id: String,
    /// Locale this translated string is authored for.
    pub locale: String,
    /// Writing direction for the locale.
    pub text_direction: TextDirection,
    /// Translated display string, preserving every source placeholder token.
    pub localized_text: String,
}

/// Summary posture derived from the catalog rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedCatalogSummary {
    /// Total translated strings across all claimed locales.
    pub total_strings: usize,
    /// Translated-string count per claimed locale.
    pub strings_by_locale: BTreeMap<String, usize>,
    /// Distinct message ids that carry at least one translation.
    pub translated_message_ids: usize,
    /// Number of claimed locales.
    pub claimed_locales: usize,
}

/// Localized prose catalog for the new M5 surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalizedCatalog {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable catalog id.
    pub catalog_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Current build identity.
    pub target_build_identity_ref: String,
    /// Stable id of the message registry this catalog renders against.
    pub registry_packet_id_ref: String,
    /// Claimed localized profiles, in stable order.
    pub claimed_locales: Vec<String>,
    /// Translated strings, sorted by message id then locale.
    pub strings: Vec<LocalizedStringEntry>,
    /// Summary posture derived from the rows.
    pub summary: LocalizedCatalogSummary,
}

/// Truncated presentation of a localized label under a grapheme budget.
///
/// Scope (`scope_surface_key`) and `severity` are carried straight through from
/// the row: truncation shortens the visible prose but cannot hide them, which is
/// what `severity_preserved` asserts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruncatedLabel {
    /// Visible-grapheme budget requested.
    pub requested_budget: usize,
    /// Visible text after truncation (or the full text when it fits).
    pub display_text: String,
    /// Whether the text was shortened to fit.
    pub was_truncated: bool,
    /// Surface scope, unchanged by truncation.
    pub scope_surface_key: String,
    /// Severity, unchanged by truncation.
    pub severity: RenderSeverityClass,
    /// Always true: severity and scope live in metadata, not the cut prose.
    pub severity_preserved: bool,
}

/// One rendered message for a requested locale.
///
/// The `message_id` and `stable_refs` are independent of the locale, which is
/// what proves command-id and contract continuity across locale changes; only
/// `display_text`, `effective_locale`, `text_direction`, and the fallback flag
/// vary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedRenderRow {
    /// Stable message id.
    pub message_id: String,
    /// Owning M5 surface.
    pub surface: M5MessageSurface,
    /// Stable snake_case key for the surface (scope).
    pub surface_key: String,
    /// Stable non-prose identity refs, copied verbatim from the registry.
    pub stable_refs: StableMessageIdentityRefs,
    /// Requested locale.
    pub requested_locale: String,
    /// Locale that supplied the rendered prose.
    pub effective_locale: String,
    /// Whether the row localized or fell back to the source language.
    pub localization_state: LocalizationRenderState,
    /// Writing direction for the effective locale.
    pub text_direction: TextDirection,
    /// Severity carried by the row.
    pub severity: RenderSeverityClass,
    /// Visible prose in the effective locale.
    pub display_text: String,
    /// Source-language template for the same message.
    pub source_text: String,
    /// Visible-grapheme length of the source text.
    pub source_grapheme_len: usize,
    /// Visible-grapheme length of the display text.
    pub display_grapheme_len: usize,
    /// Display length as a percentage of the source length (text expansion).
    pub expansion_ratio_pct: u32,
    /// Whether every source placeholder survived into the display text.
    pub placeholders_preserved: bool,
}

impl LocalizedRenderRow {
    /// Truncates the visible prose to a grapheme budget, preserving severity.
    ///
    /// Scope and severity are copied through untouched, so a tighter budget can
    /// only shorten the label, never demote an error to a plain string.
    pub fn truncate(&self, budget_graphemes: usize) -> TruncatedLabel {
        let (display_text, was_truncated) =
            truncate_graphemes(&self.display_text, budget_graphemes);
        TruncatedLabel {
            requested_budget: budget_graphemes,
            display_text,
            was_truncated,
            scope_surface_key: self.surface_key.clone(),
            severity: self.severity,
            severity_preserved: true,
        }
    }
}

/// Summary posture for one rendered locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedRenderSummary {
    /// Total rendered rows.
    pub total_rows: usize,
    /// Rows shown in the requested locale.
    pub localized_rows: usize,
    /// Rows that fell back to the source language.
    pub source_fallback_rows: usize,
    /// Largest display-to-source expansion ratio across rows.
    pub max_expansion_ratio_pct: u32,
    /// Message ids the requested locale does not translate (marked explicitly).
    pub nonlocalized_message_ids: Vec<String>,
}

/// Locale-stable render of every M5 message for one requested locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalizedRender {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Source registry packet id.
    pub registry_packet_id_ref: String,
    /// Source catalog id.
    pub catalog_id_ref: String,
    /// Current build identity.
    pub target_build_identity_ref: String,
    /// Requested locale.
    pub requested_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Writing direction for the requested locale.
    pub text_direction: TextDirection,
    /// Rendered rows in registry order.
    pub rows: Vec<LocalizedRenderRow>,
    /// Summary posture derived from the rows.
    pub summary: LocalizedRenderSummary,
}

impl M5LocalizedRender {
    /// Returns the ordered stable message ids of the render.
    pub fn message_ids(&self) -> Vec<String> {
        self.rows.iter().map(|row| row.message_id.clone()).collect()
    }

    /// Returns the rendered row for a message id, when present.
    pub fn row(&self, message_id: &str) -> Option<&LocalizedRenderRow> {
        self.rows.iter().find(|row| row.message_id == message_id)
    }
}

impl M5LocalizedCatalog {
    /// Returns the translated string for a message id and locale, if any.
    pub fn translation(&self, message_id: &str, locale: &str) -> Option<&LocalizedStringEntry> {
        self.strings
            .iter()
            .find(|entry| entry.message_id == message_id && entry.locale == locale)
    }

    /// Renders every registry message for a requested locale.
    ///
    /// A message is localized when the catalog carries a translation for the
    /// requested locale or its language base; otherwise the row falls back to
    /// the source language and is counted in the render summary.
    pub fn render(
        &self,
        registry: &M5MessageRegistry,
        requested_locale: &str,
    ) -> M5LocalizedRender {
        let rows: Vec<LocalizedRenderRow> = registry
            .entries
            .iter()
            .map(|entry| self.render_row(entry, requested_locale))
            .collect();

        let localized_rows = rows
            .iter()
            .filter(|row| row.localization_state == LocalizationRenderState::Localized)
            .count();
        let max_expansion = rows
            .iter()
            .map(|row| row.expansion_ratio_pct)
            .max()
            .unwrap_or(0);
        let nonlocalized: Vec<String> = rows
            .iter()
            .filter(|row| row.localization_state == LocalizationRenderState::SourceLanguageFallback)
            .map(|row| row.message_id.clone())
            .collect();

        M5LocalizedRender {
            record_kind: M5_LOCALIZED_RENDER_RECORD_KIND.to_owned(),
            schema_version: M5_LOCALIZED_CATALOG_SCHEMA_VERSION,
            registry_packet_id_ref: registry.packet_id.clone(),
            catalog_id_ref: self.catalog_id.clone(),
            target_build_identity_ref: self.target_build_identity_ref.clone(),
            requested_locale: requested_locale.to_owned(),
            source_language_locale: self.source_language_locale.clone(),
            text_direction: TextDirection::for_locale(requested_locale),
            summary: LocalizedRenderSummary {
                total_rows: rows.len(),
                localized_rows,
                source_fallback_rows: rows.len() - localized_rows,
                max_expansion_ratio_pct: max_expansion,
                nonlocalized_message_ids: nonlocalized,
            },
            rows,
        }
    }

    /// Renders one registry entry for a requested locale.
    fn render_row(
        &self,
        entry: &MessageRegistryEntry,
        requested_locale: &str,
    ) -> LocalizedRenderRow {
        let translation = self
            .translation(&entry.message_id, requested_locale)
            .or_else(|| self.translation(&entry.message_id, locale_base(requested_locale)));

        let severity = RenderSeverityClass::for_surface(entry.surface);
        let source_len = grapheme_len(&entry.source_text);

        let (display_text, effective_locale, text_direction, state) = match translation {
            Some(entry) => (
                entry.localized_text.clone(),
                entry.locale.clone(),
                entry.text_direction,
                LocalizationRenderState::Localized,
            ),
            None => (
                entry.source_text.clone(),
                self.source_language_locale.clone(),
                TextDirection::for_locale(&self.source_language_locale),
                LocalizationRenderState::SourceLanguageFallback,
            ),
        };
        let display_len = grapheme_len(&display_text);

        LocalizedRenderRow {
            message_id: entry.message_id.clone(),
            surface: entry.surface,
            surface_key: entry.surface.as_key().to_owned(),
            stable_refs: entry.stable_refs.clone(),
            requested_locale: requested_locale.to_owned(),
            effective_locale,
            localization_state: state,
            text_direction,
            severity,
            placeholders_preserved: placeholders_preserved(entry, &display_text),
            display_text,
            source_text: entry.source_text.clone(),
            source_grapheme_len: source_len,
            display_grapheme_len: display_len,
            expansion_ratio_pct: expansion_ratio_pct(source_len, display_len),
        }
    }

    /// Validates the catalog shape and its agreement with the message registry.
    ///
    /// The catalog must carry exactly the translations the registry declares,
    /// every translated string must preserve its message's placeholders, and no
    /// translated string may embed a stable identifier verbatim.
    pub fn validate(
        &self,
        registry: &M5MessageRegistry,
    ) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != M5_LOCALIZED_CATALOG_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.catalog_id.clone(),
                "catalog record_kind is unsupported",
            ));
        }
        if self.schema_version != M5_LOCALIZED_CATALOG_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.catalog_id.clone(),
                "catalog schema_version is unsupported",
            ));
        }
        if self.registry_packet_id_ref != registry.packet_id {
            findings.push(LocalePackValidationFinding::new(
                self.catalog_id.clone(),
                "catalog does not reference the active message registry",
            ));
        }
        if self.source_language_locale != registry.source_language_locale {
            findings.push(LocalePackValidationFinding::new(
                self.catalog_id.clone(),
                "catalog source-language locale differs from the registry",
            ));
        }

        validate_catalog_strings(self, registry, &mut findings);
        validate_catalog_coverage(self, registry, &mut findings);
        validate_catalog_summary(self, &mut findings);

        finish(findings)
    }
}

/// Per-locale parity row across one claimed locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleParityRow {
    /// Claimed locale.
    pub locale: String,
    /// Writing direction for the locale.
    pub text_direction: TextDirection,
    /// Rendered row count.
    pub rendered_row_count: usize,
    /// Whether the rendered id sequence matches the source-language render.
    pub id_set_matches_source: bool,
    /// Rows shown in the requested locale.
    pub localized_count: usize,
    /// Rows that fell back to the source language.
    pub source_fallback_count: usize,
    /// Whether every row preserved its stable non-prose refs.
    pub all_stable_refs_preserved: bool,
    /// Whether every localized string preserved its placeholders.
    pub all_placeholders_preserved: bool,
    /// Whether severity and scope survived truncation on every row.
    pub severity_preserved_under_truncation: bool,
    /// Largest display-to-source expansion ratio across rows.
    pub max_expansion_ratio_pct: u32,
    /// Message ids this locale does not translate (marked explicitly).
    pub nonlocalized_message_ids: Vec<String>,
}

impl LocaleParityRow {
    /// Returns true when this locale meets every parity claim.
    pub fn is_parity_clean(&self) -> bool {
        self.id_set_matches_source
            && self.all_stable_refs_preserved
            && self.all_placeholders_preserved
            && self.severity_preserved_under_truncation
    }
}

/// Cross-locale parity report for the claimed localized M5 profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LocalizationParityReport {
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
    /// Source registry packet id.
    pub registry_packet_id_ref: String,
    /// Source catalog id.
    pub catalog_id_ref: String,
    /// Grapheme budget the truncation proof rendered against.
    pub truncation_budget_graphemes: usize,
    /// Claimed locales evaluated.
    pub claimed_locales: Vec<String>,
    /// Per-locale parity rows.
    pub rows: Vec<LocaleParityRow>,
    /// Whether every claimed locale meets every parity claim.
    pub parity_clean: bool,
}

impl M5LocalizationParityReport {
    /// Returns the parity row for a locale, when present.
    pub fn row(&self, locale: &str) -> Option<&LocaleParityRow> {
        self.rows.iter().find(|row| row.locale == locale)
    }

    /// Validates the report shape and that it actually proves parity.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();
        if self.record_kind != M5_LOCALIZATION_PARITY_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.report_id.clone(),
                "parity report record_kind is unsupported",
            ));
        }
        if self.schema_version != M5_LOCALIZED_CATALOG_SCHEMA_VERSION {
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
        let expected_clean = self.rows.iter().all(LocaleParityRow::is_parity_clean);
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

/// Builds the parity report for a catalog against a registry and budget.
pub fn build_localization_parity_report(
    catalog: &M5LocalizedCatalog,
    registry: &M5MessageRegistry,
    truncation_budget_graphemes: usize,
) -> M5LocalizationParityReport {
    let source_ids: Vec<String> = registry
        .render(&registry.source_language_locale)
        .into_iter()
        .map(|rendered| rendered.message_id)
        .collect();

    let rows: Vec<LocaleParityRow> = catalog
        .claimed_locales
        .iter()
        .map(|locale| {
            let render = catalog.render(registry, locale);
            let id_set_matches_source = render.message_ids() == source_ids;
            let all_stable_refs_preserved = render.rows.iter().all(|row| {
                registry
                    .entry(&row.message_id)
                    .is_some_and(|entry| entry.stable_refs == row.stable_refs)
            });
            let all_placeholders_preserved =
                render.rows.iter().all(|row| row.placeholders_preserved);
            let severity_preserved = render.rows.iter().all(|row| {
                let truncated = row.truncate(truncation_budget_graphemes);
                truncated.severity == row.severity
                    && truncated.scope_surface_key == row.surface_key
                    && truncated.severity_preserved
            });

            LocaleParityRow {
                locale: locale.clone(),
                text_direction: TextDirection::for_locale(locale),
                rendered_row_count: render.rows.len(),
                id_set_matches_source,
                localized_count: render.summary.localized_rows,
                source_fallback_count: render.summary.source_fallback_rows,
                all_stable_refs_preserved,
                all_placeholders_preserved,
                severity_preserved_under_truncation: severity_preserved,
                max_expansion_ratio_pct: render.summary.max_expansion_ratio_pct,
                nonlocalized_message_ids: render.summary.nonlocalized_message_ids,
            }
        })
        .collect();

    let parity_clean = rows.iter().all(LocaleParityRow::is_parity_clean);

    M5LocalizationParityReport {
        record_kind: M5_LOCALIZATION_PARITY_RECORD_KIND.to_owned(),
        schema_version: M5_LOCALIZED_CATALOG_SCHEMA_VERSION,
        report_id: M5_LOCALIZATION_PARITY_REPORT_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_language_locale: registry.source_language_locale.clone(),
        target_build_identity_ref: registry.target_build_identity_ref.clone(),
        registry_packet_id_ref: registry.packet_id.clone(),
        catalog_id_ref: catalog.catalog_id.clone(),
        truncation_budget_graphemes,
        claimed_locales: catalog.claimed_locales.clone(),
        rows,
        parity_clean,
    }
}

/// Returns the seeded localized catalog for the new M5 surfaces.
pub fn seeded_m5_localized_catalog() -> M5LocalizedCatalog {
    let registry = seeded_m5_message_registry();
    build_seeded_catalog(&registry)
}

/// Renders the seeded catalog against the seeded registry for a locale.
pub fn seeded_m5_localized_render(requested_locale: &str) -> M5LocalizedRender {
    let registry = seeded_m5_message_registry();
    seeded_m5_localized_catalog().render(&registry, requested_locale)
}

/// Returns the seeded localization parity report for the claimed profiles.
pub fn seeded_m5_localization_parity_report() -> M5LocalizationParityReport {
    let registry = seeded_m5_message_registry();
    build_localization_parity_report(
        &seeded_m5_localized_catalog(),
        &registry,
        DEFAULT_TRUNCATION_BUDGET_GRAPHEMES,
    )
}

/// Builds the seeded catalog from the registry and the seeded translation table.
fn build_seeded_catalog(registry: &M5MessageRegistry) -> M5LocalizedCatalog {
    let table = seeded_translations();
    let mut strings: Vec<LocalizedStringEntry> = Vec::new();

    for entry in &registry.entries {
        for locale in &entry.translated_in_locales {
            if let Some((_, _, text)) = table
                .iter()
                .find(|(id, loc, _)| *id == entry.message_id && loc == locale)
            {
                strings.push(LocalizedStringEntry {
                    message_id: entry.message_id.clone(),
                    locale: locale.clone(),
                    text_direction: TextDirection::for_locale(locale),
                    localized_text: (*text).to_owned(),
                });
            }
        }
    }

    strings.sort_by(|left, right| {
        left.message_id
            .cmp(&right.message_id)
            .then(left.locale.cmp(&right.locale))
    });

    let summary = derive_catalog_summary(&strings, CLAIMED_LOCALES.len());

    M5LocalizedCatalog {
        record_kind: M5_LOCALIZED_CATALOG_RECORD_KIND.to_owned(),
        schema_version: M5_LOCALIZED_CATALOG_SCHEMA_VERSION,
        catalog_id: M5_LOCALIZED_CATALOG_PACKET_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        target_build_identity_ref: TARGET_BUILD.to_owned(),
        registry_packet_id_ref: registry.packet_id.clone(),
        claimed_locales: CLAIMED_LOCALES.iter().map(|l| (*l).to_owned()).collect(),
        strings,
        summary,
    }
}

/// Returns the seeded translated strings keyed by `(message_id, locale)`.
///
/// Every placeholder token from the source template is preserved verbatim; the
/// product name and any literal identifiers stay untranslated.
fn seeded_translations() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        // Shell chrome.
        (
            "msg:shell:title-bar:workspace-name",
            "es-MX",
            "{workspace_name} — Aureline",
        ),
        (
            "msg:shell:title-bar:workspace-name",
            "ja-JP",
            "{workspace_name} — Aureline",
        ),
        (
            "msg:shell:title-bar:workspace-name",
            "ar-SA",
            "{workspace_name} — Aureline",
        ),
        (
            "msg:shell:status-bar:background-work",
            "es-MX",
            "{count} tareas en segundo plano en ejecución",
        ),
        (
            "msg:shell:status-bar:background-work",
            "ja-JP",
            "{count} 件のバックグラウンドタスクを実行中",
        ),
        (
            "msg:shell:status-bar:background-work",
            "ar-SA",
            "{count} مهمة قيد التشغيل في الخلفية",
        ),
        (
            "msg:shell:switcher:open-window",
            "es-MX",
            "Cambiar de ventana",
        ),
        (
            "msg:shell:switcher:open-window",
            "ja-JP",
            "ウィンドウを切り替え",
        ),
        // Command palette.
        (
            "msg:command:run-build",
            "es-MX",
            "Ejecutar tarea de compilación",
        ),
        ("msg:command:run-build", "ja-JP", "ビルドタスクを実行"),
        ("msg:command:run-build", "ar-SA", "تشغيل مهمة البناء"),
        ("msg:command:open-settings", "es-MX", "Abrir configuración"),
        ("msg:command:open-settings", "ja-JP", "設定を開く"),
        ("msg:command:open-settings", "ar-SA", "فتح الإعدادات"),
        ("msg:command:open-notebook", "es-MX", "Abrir cuaderno"),
        // Settings.
        (
            "msg:settings:locale:active-language",
            "es-MX",
            "Idioma de la interfaz",
        ),
        ("msg:settings:locale:active-language", "ja-JP", "表示言語"),
        ("msg:settings:locale:active-language", "ar-SA", "لغة العرض"),
        (
            "msg:settings:locale:fallback-disclosure",
            "es-MX",
            "Mostrar avisos de respaldo en el idioma de origen",
        ),
        (
            "msg:settings:locale:fallback-disclosure",
            "ja-JP",
            "ソース言語へのフォールバック通知を表示",
        ),
        (
            "msg:settings:editor:font-size",
            "es-MX",
            "Tamaño de fuente del editor",
        ),
        (
            "msg:settings:editor:font-size",
            "ja-JP",
            "エディターのフォント サイズ",
        ),
        ("msg:settings:editor:font-size", "ar-SA", "حجم خط المحرر"),
        // Help.
        (
            "msg:help:about:locale-provenance",
            "es-MX",
            "Procedencia del idioma y del paquete de idioma",
        ),
        ("msg:help:docs:getting-started", "es-MX", "Primeros pasos"),
        ("msg:help:docs:getting-started", "ja-JP", "はじめに"),
        // Error.
        (
            "msg:error:locale-pack:signature-failed",
            "es-MX",
            "No se pudo verificar la firma del paquete de idioma; se muestra el idioma de origen.",
        ),
        (
            "msg:error:locale-pack:signature-failed",
            "ja-JP",
            "ロケールパックの署名を検証できませんでした。ソース言語を表示しています。",
        ),
        (
            "msg:error:locale-pack:signature-failed",
            "ar-SA",
            "تعذّر التحقق من توقيع حزمة اللغة؛ يتم عرض لغة المصدر.",
        ),
        (
            "msg:error:command:disabled-reason",
            "es-MX",
            "{command} no está disponible: {reason}",
        ),
        (
            "msg:error:command:disabled-reason",
            "ja-JP",
            "{command} は利用できません: {reason}",
        ),
        // Notification.
        (
            "msg:notification:update:ready",
            "es-MX",
            "Hay una actualización lista para instalar",
        ),
        (
            "msg:notification:update:ready",
            "ja-JP",
            "更新をインストールする準備ができました",
        ),
        (
            "msg:notification:update:ready",
            "ar-SA",
            "يوجد تحديث جاهز للتثبيت",
        ),
        (
            "msg:notification:locale-pack:fallback-active",
            "es-MX",
            "Algunas superficies muestran el idioma de origen",
        ),
    ]
}

/// Derives the catalog summary from its rows.
fn derive_catalog_summary(
    strings: &[LocalizedStringEntry],
    claimed_locales: usize,
) -> LocalizedCatalogSummary {
    let mut strings_by_locale = BTreeMap::new();
    let mut translated_ids = BTreeSet::new();
    for entry in strings {
        *strings_by_locale
            .entry(entry.locale.clone())
            .or_insert(0usize) += 1;
        translated_ids.insert(entry.message_id.as_str());
    }
    LocalizedCatalogSummary {
        total_strings: strings.len(),
        strings_by_locale,
        translated_message_ids: translated_ids.len(),
        claimed_locales,
    }
}

/// Returns the language-base portion of a locale tag (e.g. `es` for `es-MX`).
fn locale_base(locale: &str) -> &str {
    locale
        .split_once('-')
        .map(|(base, _)| base)
        .unwrap_or(locale)
}

/// Returns the visible-grapheme length, approximated by Unicode scalar values.
fn grapheme_len(text: &str) -> usize {
    text.chars().count()
}

/// Returns the display length as a rounded percentage of the source length.
fn expansion_ratio_pct(source_len: usize, display_len: usize) -> u32 {
    if source_len == 0 {
        return 100;
    }
    let scaled = display_len * 100 + source_len / 2;
    (scaled / source_len) as u32
}

/// Truncates `text` to `budget` graphemes, appending an ellipsis when cut.
fn truncate_graphemes(text: &str, budget: usize) -> (String, bool) {
    let len = grapheme_len(text);
    if len <= budget {
        return (text.to_owned(), false);
    }
    if budget == 0 {
        return (String::new(), true);
    }
    let kept: String = text.chars().take(budget - 1).collect();
    (format!("{kept}…"), true)
}

/// Returns true when every source placeholder token survives into `display`.
fn placeholders_preserved(entry: &MessageRegistryEntry, display: &str) -> bool {
    entry.placeholders.iter().all(|placeholder| {
        let token = format!("{{{}}}", placeholder.placeholder_id);
        display.contains(&token)
    })
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

fn validate_catalog_strings(
    catalog: &M5LocalizedCatalog,
    registry: &M5MessageRegistry,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let mut seen = BTreeSet::new();
    for string in &catalog.strings {
        let pair = (string.message_id.as_str(), string.locale.as_str());
        if !seen.insert(pair) {
            findings.push(LocalePackValidationFinding::new(
                string.message_id.clone(),
                "catalog repeats a (message_id, locale) translation",
            ));
        }
        if !catalog.claimed_locales.contains(&string.locale) {
            findings.push(LocalePackValidationFinding::new(
                string.message_id.clone(),
                "translation targets a locale outside the claimed set",
            ));
        }
        if string.text_direction != TextDirection::for_locale(&string.locale) {
            findings.push(LocalePackValidationFinding::new(
                string.message_id.clone(),
                "translation text_direction does not match its locale",
            ));
        }
        match registry.entry(&string.message_id) {
            None => findings.push(LocalePackValidationFinding::new(
                string.message_id.clone(),
                "translation references an unknown message id",
            )),
            Some(entry) => {
                if !entry.translated_in_locales.contains(&string.locale) {
                    findings.push(LocalePackValidationFinding::new(
                        string.message_id.clone(),
                        "translation exists for a locale the registry marks untranslated",
                    ));
                }
                for placeholder in &entry.placeholders {
                    let token = format!("{{{}}}", placeholder.placeholder_id);
                    if !string.localized_text.contains(&token) {
                        findings.push(LocalePackValidationFinding::new(
                            string.message_id.clone(),
                            format!("translation drops placeholder {token}"),
                        ));
                    }
                }
                if translation_leaks_stable_ref(&entry.stable_refs, &string.localized_text) {
                    findings.push(LocalePackValidationFinding::new(
                        string.message_id.clone(),
                        "translation embeds a stable identifier as prose",
                    ));
                }
            }
        }
    }
}

fn validate_catalog_coverage(
    catalog: &M5LocalizedCatalog,
    registry: &M5MessageRegistry,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    // Every declared translation in the registry has matching catalog prose.
    for entry in &registry.entries {
        for locale in &entry.translated_in_locales {
            if !catalog.claimed_locales.contains(locale) {
                continue;
            }
            if catalog.translation(&entry.message_id, locale).is_none() {
                findings.push(LocalePackValidationFinding::new(
                    entry.message_id.clone(),
                    format!("missing translated prose for claimed locale {locale}"),
                ));
            }
        }
    }
}

fn validate_catalog_summary(
    catalog: &M5LocalizedCatalog,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let expected = derive_catalog_summary(&catalog.strings, catalog.claimed_locales.len());
    if catalog.summary != expected {
        findings.push(LocalePackValidationFinding::new(
            catalog.catalog_id.clone(),
            "catalog summary does not match the derived rows",
        ));
    }
}

/// Returns true when a translated string embeds a stable id verbatim.
///
/// Stable command ids, setting ids, diagnostic ids, telemetry keys, policy
/// names, and docs-pack keys must route behavior; they must never be retyped
/// into translated prose where they could fork into a locale-only alias.
fn translation_leaks_stable_ref(refs: &StableMessageIdentityRefs, text: &str) -> bool {
    [
        refs.command_id_ref.as_deref(),
        refs.semantic_action_id_ref.as_deref(),
        refs.diagnostic_id_ref.as_deref(),
        refs.docs_pack_key_ref.as_deref(),
        refs.setting_id_ref.as_deref(),
        refs.telemetry_key_ref.as_deref(),
        refs.policy_name_ref.as_deref(),
        refs.schema_id_ref.as_deref(),
    ]
    .into_iter()
    .flatten()
    .any(|stable| text.contains(stable))
}

#[cfg(test)]
mod tests;
