//! Localization posture for human-facing CLI and `--help` prose.
//!
//! This module owns the checked-in truth packet that lets CLI/help copy
//! localize while the contract automation depends on stays pinned. Every
//! translatable usage line, subcommand summary, flag description, and error or
//! hint string is bound to a stable, locale-neutral message id, a stable
//! source-language key, and the locale-neutral anchors a script routes by —
//! subcommand paths, flag tokens, `--format json` output keys, canonical exit
//! classes, command ids, and telemetry keys.
//!
//! The packet exists to make the spec's contract testable rather than reviewed
//! by hand:
//!
//! - **Prose localizes, contracts do not.** [`CliLocalizationPacket::render`]
//!   returns the same message ids, the same flag tokens, and the same JSON
//!   output keys for every requested locale; only the effective locale and the
//!   per-message source-language fallback flag change. A
//!   [`CliMachineOutputContract`] pins that flags, subcommand names, and JSON
//!   keys are never localized, while at most one optional human field may carry
//!   translated prose.
//! - **Fallback is inspectable.** [`CliLocaleProfileRow`]s expose the
//!   requested → base → source fallback chain, the fallback origin, the degraded
//!   state, and the missing-key count for every claimed locale, and
//!   [`CliLocalizationPacket::support_export`] projects that posture into a
//!   metadata-only, copy/escalation-safe export that preserves the stable
//!   anchors and omits raw translated bodies.
//!
//! Help/About, support exports, and release-truth surfaces ingest this packet
//! instead of cloning localization status text. Raw translated bodies, signing
//! keys, and provider payloads never cross this boundary.

use std::collections::{BTreeMap, BTreeSet};

use aureline_i18n::{
    DegradedLocalizationState, LocaleFallbackOriginClass, MachineOutputLocaleClass,
    MessagePlaceholder, MessageSurfaceFamily, SourceLanguageEscapeHatchClass,
    SOURCE_LANGUAGE_LOCALE,
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version for the CLI/help localization posture packet.
pub const CLI_LOCALIZATION_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`CliLocalizationPacket`].
pub const CLI_LOCALIZATION_RECORD_KIND: &str = "cli_help_localization_packet";

/// Record kind for [`CliLocaleSupportExport`].
pub const CLI_LOCALE_SUPPORT_EXPORT_RECORD_KIND: &str = "cli_help_locale_support_export";

/// Stable packet id for the seeded CLI/help localization posture.
pub const CLI_LOCALIZATION_PACKET_ID: &str = "i18n:cli-help-localization:usage-flags-errors:v1";

/// Fixture path for the seeded CLI/help localization posture.
pub const CLI_LOCALIZATION_FIXTURE_REF: &str =
    "fixtures/i18n/cli-doctor-support/cli-help-localization.json";

/// Schema path for the CLI/help localization posture packet.
pub const CLI_LOCALIZATION_SCHEMA_REF: &str = "schemas/i18n/cli-help-locale.schema.json";

/// Deterministic generation timestamp for the seeded packet.
const GENERATED_AT: &str = "2026-06-20T17:30:00Z";

/// Target build identity the seeded packet pins anchors against.
const TARGET_BUILD: &str = "build:aureline:0.0.0-beta.2026.06.20";

/// Human-facing CLI surface that owns a translatable message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CliMessageSurface {
    /// Top-level or subcommand usage synopsis.
    Usage,
    /// One-line subcommand summary in a command list.
    SubcommandSummary,
    /// Flag or option description in `--help`.
    FlagDescription,
    /// Positional argument description in `--help`.
    ArgumentDescription,
    /// Error or denial explanation printed to stderr.
    ErrorProse,
    /// Hint or next-step prose printed alongside output.
    HintProse,
    /// Optional human-readable field inside `--format json` machine output.
    JsonHumanField,
}

impl CliMessageSurface {
    /// All CLI surfaces the packet is required to cover.
    pub const ALL: [CliMessageSurface; 7] = [
        CliMessageSurface::Usage,
        CliMessageSurface::SubcommandSummary,
        CliMessageSurface::FlagDescription,
        CliMessageSurface::ArgumentDescription,
        CliMessageSurface::ErrorProse,
        CliMessageSurface::HintProse,
        CliMessageSurface::JsonHumanField,
    ];

    /// Returns the shared message-catalog family for this surface.
    pub const fn surface_family(self) -> MessageSurfaceFamily {
        match self {
            Self::Usage
            | Self::SubcommandSummary
            | Self::FlagDescription
            | Self::ArgumentDescription
            | Self::HintProse => MessageSurfaceFamily::CliHelpText,
            Self::ErrorProse => MessageSurfaceFamily::SettingsHelpOrError,
            Self::JsonHumanField => MessageSurfaceFamily::ExportOrReportHeading,
        }
    }

    /// Returns a stable snake_case key for the surface.
    pub const fn as_key(self) -> &'static str {
        match self {
            Self::Usage => "usage",
            Self::SubcommandSummary => "subcommand_summary",
            Self::FlagDescription => "flag_description",
            Self::ArgumentDescription => "argument_description",
            Self::ErrorProse => "error_prose",
            Self::HintProse => "hint_prose",
            Self::JsonHumanField => "json_human_field",
        }
    }

    /// Returns true when this surface can appear in machine-readable output.
    pub const fn is_machine_output_bound(self) -> bool {
        matches!(self, Self::JsonHumanField)
    }
}

/// Locale-neutral identifiers a CLI consumer routes or parses by.
///
/// None of these fields ever localize: a script can pin behavior to a flag
/// token, a JSON output key, a subcommand path, or an exit class regardless of
/// the display language.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CliStableRefs {
    /// Space-joined subcommand path, e.g. `doctor diagnose`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcommand_path_ref: Option<String>,
    /// Canonical command id bound to the message, when command-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id_ref: Option<String>,
    /// Literal flag tokens the message describes, e.g. `--format`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flag_token_refs: Vec<String>,
    /// Locale-neutral `--format json` output keys the message annotates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub json_output_key_refs: Vec<String>,
    /// Canonical CLI exit class id, when the message reports a terminal state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_class_ref: Option<String>,
    /// Diagnostic id for error or denial prose.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_id_ref: Option<String>,
    /// Docs-pack key or help anchor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_key_ref: Option<String>,
    /// Locale-neutral telemetry key ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_key_ref: Option<String>,
}

impl CliStableRefs {
    /// Returns true when the message has at least one locale-neutral anchor.
    pub fn has_anchor(&self) -> bool {
        self.subcommand_path_ref.is_some()
            || self.command_id_ref.is_some()
            || !self.flag_token_refs.is_empty()
            || !self.json_output_key_refs.is_empty()
            || self.exit_class_ref.is_some()
            || self.diagnostic_id_ref.is_some()
            || self.docs_pack_key_ref.is_some()
            || self.telemetry_key_ref.is_some()
    }

    /// Returns every anchor value as a flat, sorted list of locale-neutral refs.
    pub fn anchor_values(&self) -> Vec<String> {
        let mut refs: BTreeSet<String> = BTreeSet::new();
        refs.extend(self.subcommand_path_ref.clone());
        refs.extend(self.command_id_ref.clone());
        refs.extend(self.flag_token_refs.iter().cloned());
        refs.extend(self.json_output_key_refs.iter().cloned());
        refs.extend(self.exit_class_ref.clone());
        refs.extend(self.diagnostic_id_ref.clone());
        refs.extend(self.docs_pack_key_ref.clone());
        refs.extend(self.telemetry_key_ref.clone());
        refs.into_iter().collect()
    }
}

/// One translatable CLI/help message with its stable anchors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliMessageEntry {
    /// Stable, locale-neutral message id.
    pub message_id: String,
    /// Stable source-language catalog key.
    pub source_language_key: String,
    /// CLI surface that owns the message.
    pub surface: CliMessageSurface,
    /// Shared message-catalog family for the surface.
    pub surface_family: MessageSurfaceFamily,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Short source-language template summary (no localized prose ships here).
    pub source_text: String,
    /// Locale-neutral anchors a CLI consumer routes or parses by.
    pub cli_refs: CliStableRefs,
    /// Placeholder descriptors for localization-safe rendering.
    pub placeholders: Vec<MessagePlaceholder>,
    /// Machine-output localization posture.
    pub machine_output_locale_class: MachineOutputLocaleClass,
    /// Source-language escape hatches available on this message's surface.
    pub source_language_escape_hatches: Vec<SourceLanguageEscapeHatchClass>,
    /// Locales (or language bases) that carry a translation for this message.
    pub translated_in_locales: Vec<String>,
    /// Whether localized human prose may render for this message.
    pub localized_human_prose_allowed: bool,
    /// Whether identifiers and machine keys stay locale-neutral. Must be true.
    pub machine_identifier_fields_locale_neutral: bool,
    /// Must remain false; behavior cannot route by localized prose.
    pub routed_by_localized_prose: bool,
}

impl CliMessageEntry {
    /// Returns true when `requested_locale` resolves to a translation, either
    /// exactly or through its language base.
    pub fn covered_in_locale(&self, requested_locale: &str, source_language_locale: &str) -> bool {
        requested_locale == source_language_locale
            || self
                .translated_in_locales
                .iter()
                .any(|locale| locale == requested_locale)
            || self
                .translated_in_locales
                .iter()
                .any(|locale| locale == locale_base(requested_locale))
    }
}

/// Machine-output neutrality contract for `--format json` and headless output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliMachineOutputContract {
    /// Locale-neutral JSON output keys that never localize.
    pub json_output_keys_locale_neutral: Vec<String>,
    /// The single optional human field that may carry translated prose.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_translated_human_field: Option<String>,
    /// Must remain false; JSON keys are never localized.
    pub json_keys_localized: bool,
    /// Must remain false; flag tokens are never localized.
    pub flags_localized: bool,
    /// Must remain false; subcommand names are never localized.
    pub subcommand_names_localized: bool,
    /// The flag that forces fully locale-neutral output for automation.
    pub locale_neutral_output_flag: String,
}

/// Requested-locale fallback profile for one claimed locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliLocaleProfileRow {
    /// Requested locale.
    pub requested_locale: String,
    /// Locale that produced covered messages for this profile.
    pub effective_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain.
    pub fallback_chain: Vec<String>,
    /// Why fallback did or did not occur.
    pub fallback_origin: LocaleFallbackOriginClass,
    /// Degraded localization state after fallback.
    pub degraded_state: DegradedLocalizationState,
    /// Whether a visible source-language route is active for this profile.
    pub source_language_route_active: bool,
    /// Messages that fall back to the source language for this locale.
    pub missing_key_count: usize,
}

/// Rendered message identity for one requested locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderedCliMessage {
    /// Stable message id.
    pub message_id: String,
    /// Stable source-language key.
    pub source_language_key: String,
    /// Locale-neutral anchors, byte-identical across locales.
    pub cli_refs: CliStableRefs,
    /// Locale that produced the rendered message.
    pub effective_locale: String,
    /// Whether this message fell back to the source language.
    pub used_source_language_fallback: bool,
}

/// One row in a metadata-only CLI locale support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliSupportExportRow {
    /// Stable message id.
    pub message_id: String,
    /// Stable source-language key preserved for escalation.
    pub source_language_key: String,
    /// CLI surface key.
    pub surface_key: String,
    /// Locale-neutral anchors preserved for escalation.
    pub stable_anchor_refs: Vec<String>,
    /// Effective locale after fallback.
    pub effective_locale: String,
    /// Whether this row fell back to the source language.
    pub used_source_language_fallback: bool,
    /// Whether raw translated body text is excluded from the row.
    pub raw_translated_body_omitted: bool,
}

/// Metadata-only, copy/escalation-safe export of the CLI locale posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliLocaleSupportExport {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Source packet id.
    pub source_packet_id: String,
    /// Requested locale captured by the export.
    pub requested_locale: String,
    /// Effective locale for fully covered surfaces.
    pub effective_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain.
    pub fallback_chain: Vec<String>,
    /// Why fallback did or did not occur.
    pub fallback_origin: LocaleFallbackOriginClass,
    /// Degraded localization state after fallback.
    pub degraded_state: DegradedLocalizationState,
    /// Messages that fall back to the source language for the requested locale.
    pub missing_key_count: usize,
    /// Whether a visible source-language route is active.
    pub source_language_route_active: bool,
    /// Locale-neutral output flag preserved for automation.
    pub locale_neutral_output_flag: String,
    /// Per-message export rows.
    pub rows: Vec<CliSupportExportRow>,
    /// Whether any raw translated body was exported. Must be false.
    pub raw_translated_bodies_exported: bool,
    /// Material classes deliberately omitted from the export.
    pub omitted_material_classes: Vec<String>,
    /// Deterministic generation timestamp.
    pub generated_at: String,
}

/// Summary posture derived from the packet rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliLocalizationSummary {
    /// Total registered messages.
    pub total_entries: usize,
    /// Entry count per surface, keyed by [`CliMessageSurface::as_key`].
    pub entries_by_surface: BTreeMap<String, usize>,
    /// Number of supported requested locales.
    pub supported_locales: usize,
    /// Locales served with full requested-locale coverage.
    pub fully_localized_locales: usize,
    /// Locales served by source-language fallback only.
    pub source_language_fallback_locales: usize,
    /// Distinct locale-neutral anchors preserved across every locale.
    pub preserved_anchor_count: usize,
    /// Whether the machine-output contract keeps keys/flags/names neutral.
    pub machine_output_locale_neutral: bool,
    /// Product source-language locale.
    pub source_language_locale: String,
}

/// CLI/help localization posture packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliLocalizationPacket {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Current build identity.
    pub target_build_identity_ref: String,
    /// Source contracts that govern this packet.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime consumers that ingest this packet.
    pub runtime_consumer_refs: Vec<String>,
    /// Registered messages, grouped by surface in registration order.
    pub entries: Vec<CliMessageEntry>,
    /// Machine-output neutrality contract.
    pub machine_output_contract: CliMachineOutputContract,
    /// Requested-locale fallback profiles.
    pub locale_profiles: Vec<CliLocaleProfileRow>,
    /// Metadata-only support export of the locale posture.
    pub support_export: CliLocaleSupportExport,
    /// Summary posture derived from the rows.
    pub summary: CliLocalizationSummary,
}

/// Per-locale parity row proving prose localizes without breaking automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliLocaleParityRow {
    /// Requested locale under comparison.
    pub requested_locale: String,
    /// Whether the rendered id set matches the source-language render.
    pub id_set_matches_source: bool,
    /// Whether every flag token survives the render unchanged.
    pub flag_tokens_preserved: bool,
    /// Whether every `--format json` output key survives the render unchanged.
    pub json_keys_preserved: bool,
    /// Whether every exit class survives the render unchanged.
    pub exit_classes_preserved: bool,
    /// Whether every subcommand path survives the render unchanged.
    pub subcommand_paths_preserved: bool,
    /// Messages that fell back to the source language for this locale.
    pub source_fallback_count: usize,
}

impl CliLocaleParityRow {
    /// Returns true when this locale preserves every automation anchor.
    pub fn is_parity_clean(&self) -> bool {
        self.id_set_matches_source
            && self.flag_tokens_preserved
            && self.json_keys_preserved
            && self.exit_classes_preserved
            && self.subcommand_paths_preserved
    }
}

/// Cross-locale parity report for the CLI/help posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliLocaleParityReport {
    /// Per-locale parity rows.
    pub rows: Vec<CliLocaleParityRow>,
    /// Whether every locale preserves every automation anchor.
    pub parity_clean: bool,
}

/// Validation finding emitted by the CLI localization packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CliLocalizationFinding {
    /// Row or record id that failed validation.
    pub row_ref: String,
    /// Validation message.
    pub message: String,
}

impl CliLocalizationFinding {
    /// Builds a finding for `row_ref` with `message`.
    pub fn new(row_ref: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            row_ref: row_ref.into(),
            message: message.into(),
        }
    }
}

impl CliLocalizationPacket {
    /// Returns an entry by message id.
    pub fn entry(&self, message_id: &str) -> Option<&CliMessageEntry> {
        self.entries
            .iter()
            .find(|entry| entry.message_id == message_id)
    }

    /// Returns the full set of registered message ids.
    pub fn message_ids(&self) -> BTreeSet<String> {
        self.entries
            .iter()
            .map(|entry| entry.message_id.clone())
            .collect()
    }

    /// Returns the locale profile for a requested locale.
    pub fn locale_profile(&self, requested_locale: &str) -> Option<&CliLocaleProfileRow> {
        self.locale_profiles
            .iter()
            .find(|profile| profile.requested_locale == requested_locale)
    }

    /// Returns the missing-key count for a requested locale.
    pub fn missing_key_count(&self, requested_locale: &str) -> usize {
        self.entries
            .iter()
            .filter(|entry| {
                !entry.covered_in_locale(requested_locale, &self.source_language_locale)
            })
            .count()
    }

    /// Renders the messages for a requested locale.
    ///
    /// Ids, keys, and locale-neutral anchors are independent of the locale;
    /// only the effective locale and the source-language fallback flag vary.
    pub fn render(&self, requested_locale: &str) -> Vec<RenderedCliMessage> {
        self.entries
            .iter()
            .map(|entry| {
                let covered =
                    entry.covered_in_locale(requested_locale, &self.source_language_locale);
                RenderedCliMessage {
                    message_id: entry.message_id.clone(),
                    source_language_key: entry.source_language_key.clone(),
                    cli_refs: entry.cli_refs.clone(),
                    effective_locale: if covered {
                        requested_locale.to_owned()
                    } else {
                        self.source_language_locale.clone()
                    },
                    used_source_language_fallback: !covered,
                }
            })
            .collect()
    }

    /// Builds a metadata-only support export for a requested locale.
    pub fn build_support_export(&self, requested_locale: &str) -> CliLocaleSupportExport {
        let profile = self.locale_profile(requested_locale);
        let rendered = self.render(requested_locale);
        let rows = rendered
            .iter()
            .zip(&self.entries)
            .map(|(rendered, entry)| CliSupportExportRow {
                message_id: rendered.message_id.clone(),
                source_language_key: rendered.source_language_key.clone(),
                surface_key: entry.surface.as_key().to_owned(),
                stable_anchor_refs: entry.cli_refs.anchor_values(),
                effective_locale: rendered.effective_locale.clone(),
                used_source_language_fallback: rendered.used_source_language_fallback,
                raw_translated_body_omitted: true,
            })
            .collect();

        let (effective_locale, fallback_chain, fallback_origin, degraded_state, route_active) =
            match profile {
                Some(profile) => (
                    profile.effective_locale.clone(),
                    profile.fallback_chain.clone(),
                    profile.fallback_origin,
                    profile.degraded_state,
                    profile.source_language_route_active,
                ),
                None => (
                    self.source_language_locale.clone(),
                    vec![
                        requested_locale.to_owned(),
                        self.source_language_locale.clone(),
                    ],
                    LocaleFallbackOriginClass::SourceLanguageFallback,
                    DegradedLocalizationState::FailedPackSourceLanguageOnly,
                    true,
                ),
            };

        CliLocaleSupportExport {
            record_kind: CLI_LOCALE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: CLI_LOCALIZATION_SCHEMA_VERSION,
            export_id: format!("{CLI_LOCALIZATION_PACKET_ID}:support-export:{requested_locale}"),
            source_packet_id: self.packet_id.clone(),
            requested_locale: requested_locale.to_owned(),
            effective_locale,
            source_language_locale: self.source_language_locale.clone(),
            fallback_chain,
            fallback_origin,
            degraded_state,
            missing_key_count: self.missing_key_count(requested_locale),
            source_language_route_active: route_active,
            locale_neutral_output_flag: self
                .machine_output_contract
                .locale_neutral_output_flag
                .clone(),
            rows,
            raw_translated_bodies_exported: false,
            omitted_material_classes: omitted_material_classes(),
            generated_at: self.generated_at.clone(),
        }
    }

    /// Builds the cross-locale parity report.
    pub fn parity_report(&self) -> CliLocaleParityReport {
        let source_render = self.render(&self.source_language_locale);
        let source_ids: Vec<String> = source_render
            .iter()
            .map(|row| row.message_id.clone())
            .collect();
        let source_flags = collect_refs(&source_render, |refs| refs.flag_token_refs.clone());
        let source_keys = collect_refs(&source_render, |refs| refs.json_output_key_refs.clone());
        let source_exits = collect_refs(&source_render, |refs| {
            refs.exit_class_ref.clone().into_iter().collect()
        });
        let source_subcommands = collect_refs(&source_render, |refs| {
            refs.subcommand_path_ref.clone().into_iter().collect()
        });

        let mut rows = Vec::new();
        for profile in &self.locale_profiles {
            let render = self.render(&profile.requested_locale);
            let ids: Vec<String> = render.iter().map(|row| row.message_id.clone()).collect();
            let fallback = render
                .iter()
                .filter(|row| row.used_source_language_fallback)
                .count();
            rows.push(CliLocaleParityRow {
                requested_locale: profile.requested_locale.clone(),
                id_set_matches_source: ids == source_ids,
                flag_tokens_preserved: collect_refs(&render, |r| r.flag_token_refs.clone())
                    == source_flags,
                json_keys_preserved: collect_refs(&render, |r| r.json_output_key_refs.clone())
                    == source_keys,
                exit_classes_preserved: collect_refs(&render, |r| {
                    r.exit_class_ref.clone().into_iter().collect()
                }) == source_exits,
                subcommand_paths_preserved: collect_refs(&render, |r| {
                    r.subcommand_path_ref.clone().into_iter().collect()
                }) == source_subcommands,
                source_fallback_count: fallback,
            });
        }
        let parity_clean = rows.iter().all(CliLocaleParityRow::is_parity_clean);
        CliLocaleParityReport { rows, parity_clean }
    }

    /// Validates the packet shape, anchor discipline, and locale profiles.
    pub fn validate(&self) -> Result<(), Vec<CliLocalizationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != CLI_LOCALIZATION_RECORD_KIND {
            findings.push(CliLocalizationFinding::new(
                self.packet_id.clone(),
                "packet record_kind is unsupported",
            ));
        }
        if self.schema_version != CLI_LOCALIZATION_SCHEMA_VERSION {
            findings.push(CliLocalizationFinding::new(
                self.packet_id.clone(),
                "packet schema_version is unsupported",
            ));
        }
        if self.entries.is_empty() {
            findings.push(CliLocalizationFinding::new(
                self.packet_id.clone(),
                "packet has no message entries",
            ));
        }

        validate_entries(self, &mut findings);
        validate_surface_coverage(self, &mut findings);
        validate_machine_output_contract(self, &mut findings);
        validate_locale_profiles(self, &mut findings);
        validate_support_export(self, &mut findings);
        validate_summary(self, &mut findings);

        finish(findings)
    }
}

/// Collects a sorted, deduplicated anchor list from a render under `select`.
fn collect_refs(
    render: &[RenderedCliMessage],
    select: impl Fn(&CliStableRefs) -> Vec<String>,
) -> BTreeSet<String> {
    render
        .iter()
        .flat_map(|row| select(&row.cli_refs))
        .collect()
}

/// Returns the language-base portion of a locale tag (e.g. `es` for `es-MX`).
fn locale_base(locale: &str) -> &str {
    locale
        .split_once('-')
        .map(|(base, _)| base)
        .unwrap_or(locale)
}

/// Returns true when a message id or key embeds a locale tag.
fn id_carries_locale_tag(value: &str) -> bool {
    value.split([':', '.', '/']).any(|segment| {
        let lower = segment.to_ascii_lowercase();
        let (lang, region) = match lower.split_once('-') {
            Some((lang, region)) => (lang, Some(region)),
            None => (lower.as_str(), None),
        };
        let lang_is_locale = lang.len() == 2 && lang.bytes().all(|b| b.is_ascii_lowercase());
        match region {
            Some(region) => {
                lang_is_locale
                    && region.len() == 2
                    && region.bytes().all(|b| b.is_ascii_alphabetic())
            }
            None => false,
        }
    })
}

/// Material classes the metadata-only export deliberately omits.
fn omitted_material_classes() -> Vec<String> {
    vec![
        "raw_translated_message_bodies".to_owned(),
        "locale_pack_signing_keys".to_owned(),
        "provider_payloads".to_owned(),
    ]
}

/// Collapses findings into a `Result`.
fn finish(findings: Vec<CliLocalizationFinding>) -> Result<(), Vec<CliLocalizationFinding>> {
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn validate_entries(packet: &CliLocalizationPacket, findings: &mut Vec<CliLocalizationFinding>) {
    let mut ids = BTreeSet::new();
    let mut keys = BTreeSet::new();
    for entry in &packet.entries {
        if !ids.insert(entry.message_id.as_str()) {
            findings.push(CliLocalizationFinding::new(
                entry.message_id.clone(),
                "duplicate message id",
            ));
        }
        if !keys.insert(entry.source_language_key.as_str()) {
            findings.push(CliLocalizationFinding::new(
                entry.message_id.clone(),
                "duplicate source-language key",
            ));
        }
        if entry.message_id.is_empty() || entry.source_language_key.is_empty() {
            findings.push(CliLocalizationFinding::new(
                packet.packet_id.clone(),
                "entry has an empty id or key",
            ));
        }
        if id_carries_locale_tag(&entry.message_id)
            || id_carries_locale_tag(&entry.source_language_key)
        {
            findings.push(CliLocalizationFinding::new(
                entry.message_id.clone(),
                "message id or key carries a locale tag",
            ));
        }
        if entry.source_language_locale != packet.source_language_locale {
            findings.push(CliLocalizationFinding::new(
                entry.message_id.clone(),
                "entry source-language locale differs from the packet",
            ));
        }
        if entry.surface_family != entry.surface.surface_family() {
            findings.push(CliLocalizationFinding::new(
                entry.message_id.clone(),
                "entry surface_family does not match its surface",
            ));
        }
        if !entry.cli_refs.has_anchor() {
            findings.push(CliLocalizationFinding::new(
                entry.message_id.clone(),
                "entry has no locale-neutral anchor for routing",
            ));
        }
        if entry.routed_by_localized_prose {
            findings.push(CliLocalizationFinding::new(
                entry.message_id.clone(),
                "entry routes behavior by localized prose",
            ));
        }
        if !entry.machine_identifier_fields_locale_neutral {
            findings.push(CliLocalizationFinding::new(
                entry.message_id.clone(),
                "entry machine identifiers are not locale-neutral",
            ));
        }
        if entry.source_language_escape_hatches.is_empty() {
            findings.push(CliLocalizationFinding::new(
                entry.message_id.clone(),
                "entry must offer a source-language escape hatch",
            ));
        }
        if entry.machine_output_locale_class == MachineOutputLocaleClass::ForbiddenForMachineOutput
            && entry.surface.is_machine_output_bound()
        {
            findings.push(CliLocalizationFinding::new(
                entry.message_id.clone(),
                "machine-output surface cannot forbid machine output",
            ));
        }
    }
}

fn validate_surface_coverage(
    packet: &CliLocalizationPacket,
    findings: &mut Vec<CliLocalizationFinding>,
) {
    let surfaces: BTreeSet<CliMessageSurface> =
        packet.entries.iter().map(|entry| entry.surface).collect();
    for required in CliMessageSurface::ALL {
        if !surfaces.contains(&required) {
            findings.push(CliLocalizationFinding::new(
                packet.packet_id.clone(),
                format!("packet is missing surface {}", required.as_key()),
            ));
        }
    }
}

fn validate_machine_output_contract(
    packet: &CliLocalizationPacket,
    findings: &mut Vec<CliLocalizationFinding>,
) {
    let contract = &packet.machine_output_contract;
    if contract.json_keys_localized
        || contract.flags_localized
        || contract.subcommand_names_localized
    {
        findings.push(CliLocalizationFinding::new(
            packet.packet_id.clone(),
            "machine-output contract permits localized keys, flags, or names",
        ));
    }
    if contract.json_output_keys_locale_neutral.is_empty() {
        findings.push(CliLocalizationFinding::new(
            packet.packet_id.clone(),
            "machine-output contract lists no locale-neutral keys",
        ));
    }
    if contract.locale_neutral_output_flag.is_empty() {
        findings.push(CliLocalizationFinding::new(
            packet.packet_id.clone(),
            "machine-output contract is missing the locale-neutral output flag",
        ));
    }
    // The only field that may carry translated prose must be a declared key.
    if let Some(field) = &contract.optional_translated_human_field {
        if !contract.json_output_keys_locale_neutral.contains(field) {
            findings.push(CliLocalizationFinding::new(
                packet.packet_id.clone(),
                "translated human field is not a declared JSON key",
            ));
        }
    }
}

fn validate_locale_profiles(
    packet: &CliLocalizationPacket,
    findings: &mut Vec<CliLocalizationFinding>,
) {
    let total = packet.entries.len();
    let mut seen = BTreeSet::new();
    let mut has_source = false;
    for profile in &packet.locale_profiles {
        if !seen.insert(profile.requested_locale.as_str()) {
            findings.push(CliLocalizationFinding::new(
                profile.requested_locale.clone(),
                "duplicate locale profile",
            ));
        }
        if profile.fallback_chain.first() != Some(&profile.requested_locale) {
            findings.push(CliLocalizationFinding::new(
                profile.requested_locale.clone(),
                "fallback chain must start at the requested locale",
            ));
        }
        if profile.fallback_chain.last() != Some(&packet.source_language_locale) {
            findings.push(CliLocalizationFinding::new(
                profile.requested_locale.clone(),
                "fallback chain must end at the source language",
            ));
        }
        let missing = packet.missing_key_count(&profile.requested_locale);
        if profile.missing_key_count != missing {
            findings.push(CliLocalizationFinding::new(
                profile.requested_locale.clone(),
                "profile missing-key count disagrees with coverage",
            ));
        }
        match profile.fallback_origin {
            LocaleFallbackOriginClass::RequestedLocaleAuthoritative => {
                if missing != 0 {
                    findings.push(CliLocalizationFinding::new(
                        profile.requested_locale.clone(),
                        "authoritative profile still has missing keys",
                    ));
                }
                if profile.requested_locale == packet.source_language_locale {
                    has_source = true;
                }
            }
            LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill
            | LocaleFallbackOriginClass::BaseLocaleFallback => {
                if missing == 0 || missing == total {
                    findings.push(CliLocalizationFinding::new(
                        profile.requested_locale.clone(),
                        "partial profile must have some, but not all, missing keys",
                    ));
                }
            }
            LocaleFallbackOriginClass::SourceLanguageFallback
            | LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly
            | LocaleFallbackOriginClass::PackMissingSourceLanguageOnly
            | LocaleFallbackOriginClass::PolicyDisabledSourceLanguageOnly => {
                if missing != total {
                    findings.push(CliLocalizationFinding::new(
                        profile.requested_locale.clone(),
                        "source-language profile must have every key missing",
                    ));
                }
                if profile.effective_locale != packet.source_language_locale {
                    findings.push(CliLocalizationFinding::new(
                        profile.requested_locale.clone(),
                        "source-language profile must serve the source locale",
                    ));
                }
            }
        }
    }
    if !has_source {
        findings.push(CliLocalizationFinding::new(
            packet.packet_id.clone(),
            "packet must declare an authoritative source-language profile",
        ));
    }
}

fn validate_support_export(
    packet: &CliLocalizationPacket,
    findings: &mut Vec<CliLocalizationFinding>,
) {
    let export = &packet.support_export;
    let expected = packet.build_support_export(&export.requested_locale);
    if *export != expected {
        findings.push(CliLocalizationFinding::new(
            export.export_id.clone(),
            "support export does not match the derived projection",
        ));
    }
    if export.raw_translated_bodies_exported {
        findings.push(CliLocalizationFinding::new(
            export.export_id.clone(),
            "support export must omit raw translated bodies",
        ));
    }
    if !export
        .rows
        .iter()
        .all(|row| row.raw_translated_body_omitted)
    {
        findings.push(CliLocalizationFinding::new(
            export.export_id.clone(),
            "support export row retains a raw translated body",
        ));
    }
}

fn validate_summary(packet: &CliLocalizationPacket, findings: &mut Vec<CliLocalizationFinding>) {
    let expected = derive_summary(
        &packet.entries,
        &packet.locale_profiles,
        &packet.machine_output_contract,
        &packet.source_language_locale,
    );
    if packet.summary != expected {
        findings.push(CliLocalizationFinding::new(
            packet.packet_id.clone(),
            "summary does not match the derived rows",
        ));
    }
}

fn derive_summary(
    entries: &[CliMessageEntry],
    locale_profiles: &[CliLocaleProfileRow],
    contract: &CliMachineOutputContract,
    source_language_locale: &str,
) -> CliLocalizationSummary {
    let mut entries_by_surface = BTreeMap::new();
    for entry in entries {
        *entries_by_surface
            .entry(entry.surface.as_key().to_owned())
            .or_insert(0usize) += 1;
    }
    let fully_localized = locale_profiles
        .iter()
        .filter(|profile| {
            profile.fallback_origin == LocaleFallbackOriginClass::RequestedLocaleAuthoritative
        })
        .count();
    let source_fallback = locale_profiles
        .iter()
        .filter(|profile| {
            matches!(
                profile.fallback_origin,
                LocaleFallbackOriginClass::SourceLanguageFallback
                    | LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly
                    | LocaleFallbackOriginClass::PackMissingSourceLanguageOnly
                    | LocaleFallbackOriginClass::PolicyDisabledSourceLanguageOnly
            )
        })
        .count();
    let preserved_anchor_count: BTreeSet<String> = entries
        .iter()
        .flat_map(|entry| entry.cli_refs.anchor_values())
        .collect();

    CliLocalizationSummary {
        total_entries: entries.len(),
        entries_by_surface,
        supported_locales: locale_profiles.len(),
        fully_localized_locales: fully_localized,
        source_language_fallback_locales: source_fallback,
        preserved_anchor_count: preserved_anchor_count.len(),
        machine_output_locale_neutral: !contract.json_keys_localized
            && !contract.flags_localized
            && !contract.subcommand_names_localized,
        source_language_locale: source_language_locale.to_owned(),
    }
}

/// Compact spec for a seeded CLI message, expanded by [`build_entry`].
struct EntrySpec {
    message_id: &'static str,
    source_language_key: &'static str,
    surface: CliMessageSurface,
    source_text: &'static str,
    cli_refs: CliStableRefs,
    placeholders: &'static [(&'static str, &'static str, &'static str)],
    machine_output_locale_class: MachineOutputLocaleClass,
    escape_hatches: &'static [SourceLanguageEscapeHatchClass],
    translated_in_locales: &'static [&'static str],
}

fn cli_refs(refs: CliStableRefs) -> CliStableRefs {
    refs
}

fn build_entry(spec: &EntrySpec, source_language_locale: &str) -> CliMessageEntry {
    let placeholders = spec
        .placeholders
        .iter()
        .map(|(id, kind, note)| MessagePlaceholder {
            placeholder_id: (*id).to_owned(),
            placeholder_kind: (*kind).to_owned(),
            translator_note: (*note).to_owned(),
        })
        .collect();
    CliMessageEntry {
        message_id: spec.message_id.to_owned(),
        source_language_key: spec.source_language_key.to_owned(),
        surface: spec.surface,
        surface_family: spec.surface.surface_family(),
        source_language_locale: source_language_locale.to_owned(),
        source_text: spec.source_text.to_owned(),
        cli_refs: spec.cli_refs.clone(),
        placeholders,
        machine_output_locale_class: spec.machine_output_locale_class,
        source_language_escape_hatches: spec.escape_hatches.to_vec(),
        translated_in_locales: spec
            .translated_in_locales
            .iter()
            .map(|locale| (*locale).to_owned())
            .collect(),
        localized_human_prose_allowed: true,
        machine_identifier_fields_locale_neutral: true,
        routed_by_localized_prose: false,
    }
}

/// Returns the seeded CLI message specs.
fn entry_specs() -> Vec<EntrySpec> {
    use CliMessageSurface::*;
    use MachineOutputLocaleClass::*;
    use SourceLanguageEscapeHatchClass::*;

    vec![
        EntrySpec {
            message_id: "msg:cli:root:usage",
            source_language_key: "cli.root.usage",
            surface: Usage,
            source_text: "Usage: aureline [OPTIONS] <COMMAND>",
            cli_refs: cli_refs(CliStableRefs {
                subcommand_path_ref: Some(String::new()),
                docs_pack_key_ref: Some("cli.help.root".to_owned()),
                ..CliStableRefs::default()
            }),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[CliLocaleNeutralOutputFlag, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
        },
        EntrySpec {
            message_id: "msg:cli:doctor:summary",
            source_language_key: "cli.doctor.summary",
            surface: SubcommandSummary,
            source_text: "Diagnose and repair the workspace",
            cli_refs: cli_refs(CliStableRefs {
                subcommand_path_ref: Some("doctor".to_owned()),
                command_id_ref: Some("workbench.action.runDoctor".to_owned()),
                ..CliStableRefs::default()
            }),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[CliLocaleNeutralOutputFlag, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
        },
        EntrySpec {
            message_id: "msg:cli:support-bundle:summary",
            source_language_key: "cli.support.bundle.summary",
            surface: SubcommandSummary,
            source_text: "Create a redacted support bundle",
            cli_refs: cli_refs(CliStableRefs {
                subcommand_path_ref: Some("support bundle".to_owned()),
                command_id_ref: Some("workbench.action.createSupportBundle".to_owned()),
                ..CliStableRefs::default()
            }),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[CliLocaleNeutralOutputFlag, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP"],
        },
        EntrySpec {
            message_id: "msg:cli:flag:format",
            source_language_key: "cli.flag.format.description",
            surface: FlagDescription,
            source_text: "Output format: text or json",
            cli_refs: cli_refs(CliStableRefs {
                flag_token_refs: vec!["--format".to_owned()],
                json_output_key_refs: vec!["format".to_owned()],
                ..CliStableRefs::default()
            }),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[CliLocaleNeutralOutputFlag, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
        },
        EntrySpec {
            message_id: "msg:cli:flag:locale-neutral",
            source_language_key: "cli.flag.locale_neutral.description",
            surface: FlagDescription,
            source_text: "Emit locale-neutral output for automation",
            cli_refs: cli_refs(CliStableRefs {
                flag_token_refs: vec!["--locale-neutral".to_owned()],
                ..CliStableRefs::default()
            }),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[CliLocaleNeutralOutputFlag],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
        },
        EntrySpec {
            message_id: "msg:cli:flag:locale",
            source_language_key: "cli.flag.locale.description",
            surface: FlagDescription,
            source_text: "Display language for human-readable output",
            cli_refs: cli_refs(CliStableRefs {
                flag_token_refs: vec!["--locale".to_owned()],
                ..CliStableRefs::default()
            }),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[CliLocaleNeutralOutputFlag, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX"],
        },
        EntrySpec {
            message_id: "msg:cli:arg:workspace-path",
            source_language_key: "cli.arg.workspace_path.description",
            surface: ArgumentDescription,
            source_text: "Path to the workspace folder",
            cli_refs: cli_refs(CliStableRefs {
                docs_pack_key_ref: Some("cli.help.workspace_path".to_owned()),
                ..CliStableRefs::default()
            }),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[CliLocaleNeutralOutputFlag, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP"],
        },
        EntrySpec {
            message_id: "msg:cli:error:unknown-subcommand",
            source_language_key: "cli.error.unknown_subcommand",
            surface: ErrorProse,
            source_text: "Unknown command {command}",
            cli_refs: cli_refs(CliStableRefs {
                diagnostic_id_ref: Some("cli.error.unknown_subcommand".to_owned()),
                exit_class_ref: Some("cli.exit.usage_error".to_owned()),
                ..CliStableRefs::default()
            }),
            placeholders: &[(
                "command",
                "literal_identifier",
                "Subcommand the user typed; never translated.",
            )],
            machine_output_locale_class: LocaleNeutralWithTranslatedHumanField,
            escape_hatches: &[CliLocaleNeutralOutputFlag, ExportInSourceLanguageForReview],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
        },
        EntrySpec {
            message_id: "msg:cli:error:locale-pack-signature",
            source_language_key: "cli.error.locale_pack.signature_failed",
            surface: ErrorProse,
            source_text: "Locale pack signature could not be verified; showing source language.",
            cli_refs: cli_refs(CliStableRefs {
                diagnostic_id_ref: Some("i18n.locale_pack.signature_failed".to_owned()),
                exit_class_ref: Some("cli.exit.ok".to_owned()),
                telemetry_key_ref: Some("cli.locale_fallback.shown".to_owned()),
                ..CliStableRefs::default()
            }),
            placeholders: &[],
            machine_output_locale_class: LocaleNeutralWithTranslatedHumanField,
            escape_hatches: &[CliLocaleNeutralOutputFlag, ExportInSourceLanguageForReview],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
        },
        EntrySpec {
            message_id: "msg:cli:hint:run-help",
            source_language_key: "cli.hint.run_help",
            surface: HintProse,
            source_text: "Run `aureline {command} --help` for details",
            cli_refs: cli_refs(CliStableRefs {
                docs_pack_key_ref: Some("cli.help.hint".to_owned()),
                ..CliStableRefs::default()
            }),
            placeholders: &[(
                "command",
                "literal_identifier",
                "Subcommand name; never translated or reordered.",
            )],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[CliLocaleNeutralOutputFlag, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP"],
        },
        EntrySpec {
            message_id: "msg:cli:json:message-field",
            source_language_key: "cli.json.message_field",
            surface: JsonHumanField,
            source_text: "{summary}",
            cli_refs: cli_refs(CliStableRefs {
                json_output_key_refs: vec!["message".to_owned()],
                exit_class_ref: Some("cli.exit.ok".to_owned()),
                ..CliStableRefs::default()
            }),
            placeholders: &[(
                "summary",
                "translated_human_summary",
                "Optional translated human field beside locale-neutral keys.",
            )],
            machine_output_locale_class: LocaleNeutralWithTranslatedHumanField,
            escape_hatches: &[CliLocaleNeutralOutputFlag, ExportInSourceLanguageForReview],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
        },
    ]
}

/// Returns the seeded machine-output neutrality contract.
fn seeded_machine_output_contract() -> CliMachineOutputContract {
    CliMachineOutputContract {
        json_output_keys_locale_neutral: vec![
            "schema_version".to_owned(),
            "record_kind".to_owned(),
            "status".to_owned(),
            "exit_class".to_owned(),
            "finding_code".to_owned(),
            "subcommand".to_owned(),
            "format".to_owned(),
            "data".to_owned(),
            "message".to_owned(),
        ],
        optional_translated_human_field: Some("message".to_owned()),
        json_keys_localized: false,
        flags_localized: false,
        subcommand_names_localized: false,
        locale_neutral_output_flag: "--locale-neutral".to_owned(),
    }
}

/// Returns the seeded requested-locale fallback profiles.
fn seeded_locale_profiles(entries: &[CliMessageEntry], source: &str) -> Vec<CliLocaleProfileRow> {
    let missing = |locale: &str| {
        entries
            .iter()
            .filter(|entry| !entry.covered_in_locale(locale, source))
            .count()
    };
    vec![
        CliLocaleProfileRow {
            requested_locale: source.to_owned(),
            effective_locale: source.to_owned(),
            source_language_locale: source.to_owned(),
            fallback_chain: vec![source.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
            degraded_state: DegradedLocalizationState::FullyLocalized,
            source_language_route_active: false,
            missing_key_count: missing(source),
        },
        CliLocaleProfileRow {
            requested_locale: "es-MX".to_owned(),
            effective_locale: "es-MX".to_owned(),
            source_language_locale: source.to_owned(),
            fallback_chain: vec!["es-MX".to_owned(), "es".to_owned(), source.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
            degraded_state: DegradedLocalizationState::FullyLocalized,
            source_language_route_active: false,
            missing_key_count: missing("es-MX"),
        },
        CliLocaleProfileRow {
            requested_locale: "ja-JP".to_owned(),
            effective_locale: "ja-JP".to_owned(),
            source_language_locale: source.to_owned(),
            fallback_chain: vec!["ja-JP".to_owned(), "ja".to_owned(), source.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill,
            degraded_state: DegradedLocalizationState::PartialTranslationDisclosed,
            source_language_route_active: true,
            missing_key_count: missing("ja-JP"),
        },
        CliLocaleProfileRow {
            requested_locale: "ar-SA".to_owned(),
            effective_locale: "ar-SA".to_owned(),
            source_language_locale: source.to_owned(),
            fallback_chain: vec!["ar-SA".to_owned(), "ar".to_owned(), source.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill,
            degraded_state: DegradedLocalizationState::PartialTranslationDisclosed,
            source_language_route_active: true,
            missing_key_count: missing("ar-SA"),
        },
        CliLocaleProfileRow {
            requested_locale: "de-DE".to_owned(),
            effective_locale: source.to_owned(),
            source_language_locale: source.to_owned(),
            fallback_chain: vec!["de-DE".to_owned(), "de".to_owned(), source.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly,
            degraded_state: DegradedLocalizationState::FailedPackSourceLanguageOnly,
            source_language_route_active: true,
            missing_key_count: missing("de-DE"),
        },
    ]
}

/// Returns source contracts that govern the packet.
fn seeded_source_contract_refs() -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "architecture_localization".to_owned(),
            ".t2/docs/Aureline_Technical_Architecture_Document.md#23.3.1".to_owned(),
        ),
        (
            "design_content_governance".to_owned(),
            ".t2/docs/Aureline_Technical_Design_Document.md#8.10".to_owned(),
        ),
        (
            "message_registry".to_owned(),
            "fixtures/i18n/message-id-stability/registry.json".to_owned(),
        ),
    ])
}

/// Returns runtime consumers that ingest the packet.
fn seeded_runtime_consumer_refs() -> Vec<String> {
    vec![
        "crates/aureline-cli".to_owned(),
        "crates/aureline-help".to_owned(),
        "crates/aureline-support".to_owned(),
    ]
}

/// Returns the seeded CLI/help localization posture packet.
pub fn seeded_cli_localization_packet() -> CliLocalizationPacket {
    let entries: Vec<CliMessageEntry> = entry_specs()
        .iter()
        .map(|spec| build_entry(spec, SOURCE_LANGUAGE_LOCALE))
        .collect();
    let machine_output_contract = seeded_machine_output_contract();
    let locale_profiles = seeded_locale_profiles(&entries, SOURCE_LANGUAGE_LOCALE);
    let summary = derive_summary(
        &entries,
        &locale_profiles,
        &machine_output_contract,
        SOURCE_LANGUAGE_LOCALE,
    );

    let mut packet = CliLocalizationPacket {
        record_kind: CLI_LOCALIZATION_RECORD_KIND.to_owned(),
        schema_version: CLI_LOCALIZATION_SCHEMA_VERSION,
        packet_id: CLI_LOCALIZATION_PACKET_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        target_build_identity_ref: TARGET_BUILD.to_owned(),
        source_contract_refs: seeded_source_contract_refs(),
        runtime_consumer_refs: seeded_runtime_consumer_refs(),
        entries,
        machine_output_contract,
        locale_profiles,
        // Replaced below once the packet can derive the export.
        support_export: placeholder_support_export(),
        summary,
    };
    // The support export captures a claimed localized profile under partial
    // fallback, proving locale state stays inspectable on exported artifacts.
    packet.support_export = packet.build_support_export("ja-JP");
    packet
}

/// Returns an empty placeholder export, overwritten during seeding.
fn placeholder_support_export() -> CliLocaleSupportExport {
    CliLocaleSupportExport {
        record_kind: CLI_LOCALE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: CLI_LOCALIZATION_SCHEMA_VERSION,
        export_id: String::new(),
        source_packet_id: CLI_LOCALIZATION_PACKET_ID.to_owned(),
        requested_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        effective_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        fallback_chain: vec![SOURCE_LANGUAGE_LOCALE.to_owned()],
        fallback_origin: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
        degraded_state: DegradedLocalizationState::FullyLocalized,
        missing_key_count: 0,
        source_language_route_active: false,
        locale_neutral_output_flag: "--locale-neutral".to_owned(),
        rows: Vec::new(),
        raw_translated_bodies_exported: false,
        omitted_material_classes: omitted_material_classes(),
        generated_at: GENERATED_AT.to_owned(),
    }
}
