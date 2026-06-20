//! Localization posture for human-facing Project Doctor report prose.
//!
//! This module owns the checked-in truth packet that lets Doctor finding
//! titles, explanations, and recommended-action prose localize while the
//! machine contract a support escalation depends on stays pinned. Every
//! translatable string is bound to a stable, locale-neutral message id, a
//! stable source-language key, and the locale-neutral anchors a parser or
//! reviewer routes by — finding codes, probe ids, canonical exit classes,
//! evidence-ref kinds, scope labels, policy names, and recovery command ids.
//!
//! The packet makes the spec's contract testable rather than reviewed by hand:
//!
//! - **Prose localizes, finding identity does not.**
//!   [`DoctorReportLocalizationPacket::render`] returns the same message ids,
//!   finding codes, exit classes, and evidence-ref kinds for every requested
//!   locale; only the effective locale and the per-message source-language
//!   fallback flag change.
//! - **Locale and fallback stay inspectable on exported artifacts.**
//!   [`DoctorReportLocalizationPacket::support_export`] projects the active
//!   locale, the requested → base → source fallback chain, the fallback origin,
//!   and the degraded state into a metadata-only export that preserves the
//!   stable anchors and source-language keys needed for escalation and omits
//!   raw translated bodies.
//!
//! Support exports, incident packets, and release-truth surfaces ingest this
//! packet instead of cloning localization status text. Raw translated bodies,
//! signing keys, and provider payloads never cross this boundary.

use std::collections::{BTreeMap, BTreeSet};

use aureline_i18n::{
    DegradedLocalizationState, LocaleFallbackOriginClass, MachineOutputLocaleClass,
    MessagePlaceholder, MessageSurfaceFamily, SourceLanguageEscapeHatchClass,
    SOURCE_LANGUAGE_LOCALE,
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version for the Doctor report localization posture packet.
pub const DOCTOR_REPORT_LOCALE_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`DoctorReportLocalizationPacket`].
pub const DOCTOR_REPORT_LOCALIZATION_RECORD_KIND: &str = "doctor_report_localization_packet";

/// Record kind for [`DoctorReportSupportExport`].
pub const DOCTOR_REPORT_SUPPORT_EXPORT_RECORD_KIND: &str = "doctor_report_locale_support_export";

/// Stable packet id for the seeded Doctor report localization posture.
pub const DOCTOR_REPORT_LOCALIZATION_PACKET_ID: &str =
    "i18n:doctor-report-localization:findings-recovery:v1";

/// Fixture path for the seeded Doctor report localization posture.
pub const DOCTOR_REPORT_LOCALIZATION_FIXTURE_REF: &str =
    "fixtures/i18n/cli-doctor-support/doctor-report-localization.json";

/// Schema path for the Doctor report localization posture packet.
pub const DOCTOR_REPORT_LOCALIZATION_SCHEMA_REF: &str =
    "schemas/i18n/doctor-report-locale.schema.json";

/// Deterministic generation timestamp for the seeded packet.
const GENERATED_AT: &str = "2026-06-20T17:30:00Z";

/// Target build identity the seeded packet pins anchors against.
const TARGET_BUILD: &str = "build:aureline:0.0.0-beta.2026.06.20";

/// Human-facing Doctor report surface that owns a translatable message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorMessageSurface {
    /// Finding title shown in the report header row.
    FindingTitle,
    /// Finding explanation describing the diagnosed condition.
    FindingExplanation,
    /// Recommended-action or recovery prose for a finding.
    RecommendedAction,
    /// Explicit unsupported, partial, or target-mismatch state note.
    UnsupportedStateNote,
    /// Section heading in the human-readable report.
    ReportHeading,
    /// Heading rendered on the exported support artifact.
    SupportExportHeading,
}

impl DoctorMessageSurface {
    /// All Doctor report surfaces the packet is required to cover.
    pub const ALL: [DoctorMessageSurface; 6] = [
        DoctorMessageSurface::FindingTitle,
        DoctorMessageSurface::FindingExplanation,
        DoctorMessageSurface::RecommendedAction,
        DoctorMessageSurface::UnsupportedStateNote,
        DoctorMessageSurface::ReportHeading,
        DoctorMessageSurface::SupportExportHeading,
    ];

    /// Returns the shared message-catalog family for this surface.
    pub const fn surface_family(self) -> MessageSurfaceFamily {
        match self {
            Self::FindingTitle
            | Self::FindingExplanation
            | Self::RecommendedAction
            | Self::UnsupportedStateNote => MessageSurfaceFamily::SettingsHelpOrError,
            Self::ReportHeading | Self::SupportExportHeading => {
                MessageSurfaceFamily::ExportOrReportHeading
            }
        }
    }

    /// Returns a stable snake_case key for the surface.
    pub const fn as_key(self) -> &'static str {
        match self {
            Self::FindingTitle => "finding_title",
            Self::FindingExplanation => "finding_explanation",
            Self::RecommendedAction => "recommended_action",
            Self::UnsupportedStateNote => "unsupported_state_note",
            Self::ReportHeading => "report_heading",
            Self::SupportExportHeading => "support_export_heading",
        }
    }

    /// Returns true when the surface is bound to a specific finding.
    pub const fn is_finding_bound(self) -> bool {
        matches!(
            self,
            Self::FindingTitle
                | Self::FindingExplanation
                | Self::RecommendedAction
                | Self::UnsupportedStateNote
        )
    }
}

/// Locale-neutral identifiers a Doctor-report consumer routes or parses by.
///
/// None of these fields ever localize: a support parser or incident packet can
/// pin behavior to a finding code, an exit class, or an evidence-ref kind
/// regardless of the display language.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DoctorStableRefs {
    /// Canonical Doctor finding code, when finding-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finding_code_ref: Option<String>,
    /// Probe id that produced the finding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probe_id_ref: Option<String>,
    /// Canonical CLI exit class id derived from the diagnosis state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_class_ref: Option<String>,
    /// Evidence-ref kinds preserved on the finding, e.g. `log_excerpt`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_ref_kinds: Vec<String>,
    /// Locale-neutral scope label, e.g. `workspace` or `provider_auth`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_label_ref: Option<String>,
    /// Recovery command id offered by the finding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_command_id_ref: Option<String>,
    /// Locale-neutral policy name ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_name_ref: Option<String>,
    /// Docs-pack key or help anchor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_key_ref: Option<String>,
    /// Locale-neutral telemetry key ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telemetry_key_ref: Option<String>,
}

impl DoctorStableRefs {
    /// Returns true when the message has at least one locale-neutral anchor.
    pub fn has_anchor(&self) -> bool {
        self.finding_code_ref.is_some()
            || self.probe_id_ref.is_some()
            || self.exit_class_ref.is_some()
            || !self.evidence_ref_kinds.is_empty()
            || self.scope_label_ref.is_some()
            || self.recovery_command_id_ref.is_some()
            || self.policy_name_ref.is_some()
            || self.docs_pack_key_ref.is_some()
            || self.telemetry_key_ref.is_some()
    }

    /// Returns every anchor value as a flat, sorted list of locale-neutral refs.
    pub fn anchor_values(&self) -> Vec<String> {
        let mut refs: BTreeSet<String> = BTreeSet::new();
        refs.extend(self.finding_code_ref.clone());
        refs.extend(self.probe_id_ref.clone());
        refs.extend(self.exit_class_ref.clone());
        refs.extend(self.evidence_ref_kinds.iter().cloned());
        refs.extend(self.scope_label_ref.clone());
        refs.extend(self.recovery_command_id_ref.clone());
        refs.extend(self.policy_name_ref.clone());
        refs.extend(self.docs_pack_key_ref.clone());
        refs.extend(self.telemetry_key_ref.clone());
        refs.into_iter().collect()
    }
}

/// One translatable Doctor report message with its stable anchors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorMessageEntry {
    /// Stable, locale-neutral message id.
    pub message_id: String,
    /// Stable source-language catalog key.
    pub source_language_key: String,
    /// Doctor report surface that owns the message.
    pub surface: DoctorMessageSurface,
    /// Shared message-catalog family for the surface.
    pub surface_family: MessageSurfaceFamily,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Short source-language template summary (no localized prose ships here).
    pub source_text: String,
    /// Locale-neutral anchors a Doctor-report consumer routes or parses by.
    pub doctor_refs: DoctorStableRefs,
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

impl DoctorMessageEntry {
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

/// Requested-locale fallback profile for one claimed locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorLocaleProfileRow {
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
pub struct RenderedDoctorMessage {
    /// Stable message id.
    pub message_id: String,
    /// Stable source-language key.
    pub source_language_key: String,
    /// Locale-neutral anchors, byte-identical across locales.
    pub doctor_refs: DoctorStableRefs,
    /// Locale that produced the rendered message.
    pub effective_locale: String,
    /// Whether this message fell back to the source language.
    pub used_source_language_fallback: bool,
}

/// One row in a metadata-only Doctor report locale support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorSupportExportRow {
    /// Stable message id.
    pub message_id: String,
    /// Stable source-language key preserved for escalation.
    pub source_language_key: String,
    /// Doctor report surface key.
    pub surface_key: String,
    /// Finding code preserved on the row, when finding-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finding_code_ref: Option<String>,
    /// Locale-neutral anchors preserved for escalation.
    pub stable_anchor_refs: Vec<String>,
    /// Effective locale after fallback.
    pub effective_locale: String,
    /// Whether this row fell back to the source language.
    pub used_source_language_fallback: bool,
    /// Whether raw translated body text is excluded from the row.
    pub raw_translated_body_omitted: bool,
}

/// Metadata-only, escalation-safe export of the Doctor report locale posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorReportSupportExport {
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
    /// Distinct finding codes preserved across the export for escalation.
    pub preserved_finding_codes: Vec<String>,
    /// Per-message export rows.
    pub rows: Vec<DoctorSupportExportRow>,
    /// Whether any raw translated body was exported. Must be false.
    pub raw_translated_bodies_exported: bool,
    /// Material classes deliberately omitted from the export.
    pub omitted_material_classes: Vec<String>,
    /// Deterministic generation timestamp.
    pub generated_at: String,
}

/// Summary posture derived from the packet rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorReportLocalizationSummary {
    /// Total registered messages.
    pub total_entries: usize,
    /// Entry count per surface, keyed by [`DoctorMessageSurface::as_key`].
    pub entries_by_surface: BTreeMap<String, usize>,
    /// Distinct finding codes covered by the packet.
    pub finding_codes_covered: usize,
    /// Number of supported requested locales.
    pub supported_locales: usize,
    /// Locales served with full requested-locale coverage.
    pub fully_localized_locales: usize,
    /// Locales served by source-language fallback only.
    pub source_language_fallback_locales: usize,
    /// Distinct locale-neutral anchors preserved across every locale.
    pub preserved_anchor_count: usize,
    /// Product source-language locale.
    pub source_language_locale: String,
}

/// Doctor report localization posture packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorReportLocalizationPacket {
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
    pub entries: Vec<DoctorMessageEntry>,
    /// Requested-locale fallback profiles.
    pub locale_profiles: Vec<DoctorLocaleProfileRow>,
    /// Metadata-only support export of the locale posture.
    pub support_export: DoctorReportSupportExport,
    /// Summary posture derived from the rows.
    pub summary: DoctorReportLocalizationSummary,
}

/// Per-locale parity row proving prose localizes without losing finding truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorLocaleParityRow {
    /// Requested locale under comparison.
    pub requested_locale: String,
    /// Whether the rendered id set matches the source-language render.
    pub id_set_matches_source: bool,
    /// Whether every finding code survives the render unchanged.
    pub finding_codes_preserved: bool,
    /// Whether every exit class survives the render unchanged.
    pub exit_classes_preserved: bool,
    /// Whether every evidence-ref kind survives the render unchanged.
    pub evidence_refs_preserved: bool,
    /// Whether every scope label survives the render unchanged.
    pub scope_labels_preserved: bool,
    /// Messages that fell back to the source language for this locale.
    pub source_fallback_count: usize,
}

impl DoctorLocaleParityRow {
    /// Returns true when this locale preserves every finding anchor.
    pub fn is_parity_clean(&self) -> bool {
        self.id_set_matches_source
            && self.finding_codes_preserved
            && self.exit_classes_preserved
            && self.evidence_refs_preserved
            && self.scope_labels_preserved
    }
}

/// Cross-locale parity report for the Doctor report posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorLocaleParityReport {
    /// Per-locale parity rows.
    pub rows: Vec<DoctorLocaleParityRow>,
    /// Whether every locale preserves every finding anchor.
    pub parity_clean: bool,
}

/// Validation finding emitted by the Doctor report localization packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorLocalizationFinding {
    /// Row or record id that failed validation.
    pub row_ref: String,
    /// Validation message.
    pub message: String,
}

impl DoctorLocalizationFinding {
    /// Builds a finding for `row_ref` with `message`.
    pub fn new(row_ref: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            row_ref: row_ref.into(),
            message: message.into(),
        }
    }
}

impl DoctorReportLocalizationPacket {
    /// Returns an entry by message id.
    pub fn entry(&self, message_id: &str) -> Option<&DoctorMessageEntry> {
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
    pub fn locale_profile(&self, requested_locale: &str) -> Option<&DoctorLocaleProfileRow> {
        self.locale_profiles
            .iter()
            .find(|profile| profile.requested_locale == requested_locale)
    }

    /// Returns the distinct finding codes the packet covers.
    pub fn finding_codes(&self) -> BTreeSet<String> {
        self.entries
            .iter()
            .filter_map(|entry| entry.doctor_refs.finding_code_ref.clone())
            .collect()
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
    pub fn render(&self, requested_locale: &str) -> Vec<RenderedDoctorMessage> {
        self.entries
            .iter()
            .map(|entry| {
                let covered =
                    entry.covered_in_locale(requested_locale, &self.source_language_locale);
                RenderedDoctorMessage {
                    message_id: entry.message_id.clone(),
                    source_language_key: entry.source_language_key.clone(),
                    doctor_refs: entry.doctor_refs.clone(),
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
    pub fn build_support_export(&self, requested_locale: &str) -> DoctorReportSupportExport {
        let profile = self.locale_profile(requested_locale);
        let rendered = self.render(requested_locale);
        let rows: Vec<DoctorSupportExportRow> = rendered
            .iter()
            .zip(&self.entries)
            .map(|(rendered, entry)| DoctorSupportExportRow {
                message_id: rendered.message_id.clone(),
                source_language_key: rendered.source_language_key.clone(),
                surface_key: entry.surface.as_key().to_owned(),
                finding_code_ref: entry.doctor_refs.finding_code_ref.clone(),
                stable_anchor_refs: entry.doctor_refs.anchor_values(),
                effective_locale: rendered.effective_locale.clone(),
                used_source_language_fallback: rendered.used_source_language_fallback,
                raw_translated_body_omitted: true,
            })
            .collect();
        let preserved_finding_codes: Vec<String> = self.finding_codes().into_iter().collect();

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

        DoctorReportSupportExport {
            record_kind: DOCTOR_REPORT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCTOR_REPORT_LOCALE_SCHEMA_VERSION,
            export_id: format!(
                "{DOCTOR_REPORT_LOCALIZATION_PACKET_ID}:support-export:{requested_locale}"
            ),
            source_packet_id: self.packet_id.clone(),
            requested_locale: requested_locale.to_owned(),
            effective_locale,
            source_language_locale: self.source_language_locale.clone(),
            fallback_chain,
            fallback_origin,
            degraded_state,
            missing_key_count: self.missing_key_count(requested_locale),
            source_language_route_active: route_active,
            preserved_finding_codes,
            rows,
            raw_translated_bodies_exported: false,
            omitted_material_classes: omitted_material_classes(),
            generated_at: self.generated_at.clone(),
        }
    }

    /// Builds the cross-locale parity report.
    pub fn parity_report(&self) -> DoctorLocaleParityReport {
        let source_render = self.render(&self.source_language_locale);
        let source_ids: Vec<String> = source_render
            .iter()
            .map(|row| row.message_id.clone())
            .collect();
        let source_codes = collect_refs(&source_render, |refs| {
            refs.finding_code_ref.clone().into_iter().collect()
        });
        let source_exits = collect_refs(&source_render, |refs| {
            refs.exit_class_ref.clone().into_iter().collect()
        });
        let source_evidence = collect_refs(&source_render, |refs| refs.evidence_ref_kinds.clone());
        let source_scopes = collect_refs(&source_render, |refs| {
            refs.scope_label_ref.clone().into_iter().collect()
        });

        let mut rows = Vec::new();
        for profile in &self.locale_profiles {
            let render = self.render(&profile.requested_locale);
            let ids: Vec<String> = render.iter().map(|row| row.message_id.clone()).collect();
            let fallback = render
                .iter()
                .filter(|row| row.used_source_language_fallback)
                .count();
            rows.push(DoctorLocaleParityRow {
                requested_locale: profile.requested_locale.clone(),
                id_set_matches_source: ids == source_ids,
                finding_codes_preserved: collect_refs(&render, |r| {
                    r.finding_code_ref.clone().into_iter().collect()
                }) == source_codes,
                exit_classes_preserved: collect_refs(&render, |r| {
                    r.exit_class_ref.clone().into_iter().collect()
                }) == source_exits,
                evidence_refs_preserved: collect_refs(&render, |r| r.evidence_ref_kinds.clone())
                    == source_evidence,
                scope_labels_preserved: collect_refs(&render, |r| {
                    r.scope_label_ref.clone().into_iter().collect()
                }) == source_scopes,
                source_fallback_count: fallback,
            });
        }
        let parity_clean = rows.iter().all(DoctorLocaleParityRow::is_parity_clean);
        DoctorLocaleParityReport { rows, parity_clean }
    }

    /// Validates the packet shape, anchor discipline, and locale profiles.
    pub fn validate(&self) -> Result<(), Vec<DoctorLocalizationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != DOCTOR_REPORT_LOCALIZATION_RECORD_KIND {
            findings.push(DoctorLocalizationFinding::new(
                self.packet_id.clone(),
                "packet record_kind is unsupported",
            ));
        }
        if self.schema_version != DOCTOR_REPORT_LOCALE_SCHEMA_VERSION {
            findings.push(DoctorLocalizationFinding::new(
                self.packet_id.clone(),
                "packet schema_version is unsupported",
            ));
        }
        if self.entries.is_empty() {
            findings.push(DoctorLocalizationFinding::new(
                self.packet_id.clone(),
                "packet has no message entries",
            ));
        }

        validate_entries(self, &mut findings);
        validate_surface_coverage(self, &mut findings);
        validate_locale_profiles(self, &mut findings);
        validate_support_export(self, &mut findings);
        validate_summary(self, &mut findings);

        finish(findings)
    }
}

/// Collects a sorted, deduplicated anchor list from a render under `select`.
fn collect_refs(
    render: &[RenderedDoctorMessage],
    select: impl Fn(&DoctorStableRefs) -> Vec<String>,
) -> BTreeSet<String> {
    render
        .iter()
        .flat_map(|row| select(&row.doctor_refs))
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
        "raw_evidence_payloads".to_owned(),
        "locale_pack_signing_keys".to_owned(),
    ]
}

/// Collapses findings into a `Result`.
fn finish(findings: Vec<DoctorLocalizationFinding>) -> Result<(), Vec<DoctorLocalizationFinding>> {
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn validate_entries(
    packet: &DoctorReportLocalizationPacket,
    findings: &mut Vec<DoctorLocalizationFinding>,
) {
    let mut ids = BTreeSet::new();
    let mut keys = BTreeSet::new();
    for entry in &packet.entries {
        if !ids.insert(entry.message_id.as_str()) {
            findings.push(DoctorLocalizationFinding::new(
                entry.message_id.clone(),
                "duplicate message id",
            ));
        }
        if !keys.insert(entry.source_language_key.as_str()) {
            findings.push(DoctorLocalizationFinding::new(
                entry.message_id.clone(),
                "duplicate source-language key",
            ));
        }
        if entry.message_id.is_empty() || entry.source_language_key.is_empty() {
            findings.push(DoctorLocalizationFinding::new(
                packet.packet_id.clone(),
                "entry has an empty id or key",
            ));
        }
        if id_carries_locale_tag(&entry.message_id)
            || id_carries_locale_tag(&entry.source_language_key)
        {
            findings.push(DoctorLocalizationFinding::new(
                entry.message_id.clone(),
                "message id or key carries a locale tag",
            ));
        }
        if entry.source_language_locale != packet.source_language_locale {
            findings.push(DoctorLocalizationFinding::new(
                entry.message_id.clone(),
                "entry source-language locale differs from the packet",
            ));
        }
        if entry.surface_family != entry.surface.surface_family() {
            findings.push(DoctorLocalizationFinding::new(
                entry.message_id.clone(),
                "entry surface_family does not match its surface",
            ));
        }
        if !entry.doctor_refs.has_anchor() {
            findings.push(DoctorLocalizationFinding::new(
                entry.message_id.clone(),
                "entry has no locale-neutral anchor for routing",
            ));
        }
        // Finding-bound surfaces must carry a finding code for escalation.
        if entry.surface.is_finding_bound() && entry.doctor_refs.finding_code_ref.is_none() {
            findings.push(DoctorLocalizationFinding::new(
                entry.message_id.clone(),
                "finding-bound entry is missing a finding code",
            ));
        }
        if entry.routed_by_localized_prose {
            findings.push(DoctorLocalizationFinding::new(
                entry.message_id.clone(),
                "entry routes behavior by localized prose",
            ));
        }
        if !entry.machine_identifier_fields_locale_neutral {
            findings.push(DoctorLocalizationFinding::new(
                entry.message_id.clone(),
                "entry machine identifiers are not locale-neutral",
            ));
        }
        if entry.source_language_escape_hatches.is_empty() {
            findings.push(DoctorLocalizationFinding::new(
                entry.message_id.clone(),
                "entry must offer a source-language escape hatch",
            ));
        }
    }
}

fn validate_surface_coverage(
    packet: &DoctorReportLocalizationPacket,
    findings: &mut Vec<DoctorLocalizationFinding>,
) {
    let surfaces: BTreeSet<DoctorMessageSurface> =
        packet.entries.iter().map(|entry| entry.surface).collect();
    for required in DoctorMessageSurface::ALL {
        if !surfaces.contains(&required) {
            findings.push(DoctorLocalizationFinding::new(
                packet.packet_id.clone(),
                format!("packet is missing surface {}", required.as_key()),
            ));
        }
    }
}

fn validate_locale_profiles(
    packet: &DoctorReportLocalizationPacket,
    findings: &mut Vec<DoctorLocalizationFinding>,
) {
    let total = packet.entries.len();
    let mut seen = BTreeSet::new();
    let mut has_source = false;
    for profile in &packet.locale_profiles {
        if !seen.insert(profile.requested_locale.as_str()) {
            findings.push(DoctorLocalizationFinding::new(
                profile.requested_locale.clone(),
                "duplicate locale profile",
            ));
        }
        if profile.fallback_chain.first() != Some(&profile.requested_locale) {
            findings.push(DoctorLocalizationFinding::new(
                profile.requested_locale.clone(),
                "fallback chain must start at the requested locale",
            ));
        }
        if profile.fallback_chain.last() != Some(&packet.source_language_locale) {
            findings.push(DoctorLocalizationFinding::new(
                profile.requested_locale.clone(),
                "fallback chain must end at the source language",
            ));
        }
        let missing = packet.missing_key_count(&profile.requested_locale);
        if profile.missing_key_count != missing {
            findings.push(DoctorLocalizationFinding::new(
                profile.requested_locale.clone(),
                "profile missing-key count disagrees with coverage",
            ));
        }
        match profile.fallback_origin {
            LocaleFallbackOriginClass::RequestedLocaleAuthoritative => {
                if missing != 0 {
                    findings.push(DoctorLocalizationFinding::new(
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
                    findings.push(DoctorLocalizationFinding::new(
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
                    findings.push(DoctorLocalizationFinding::new(
                        profile.requested_locale.clone(),
                        "source-language profile must have every key missing",
                    ));
                }
                if profile.effective_locale != packet.source_language_locale {
                    findings.push(DoctorLocalizationFinding::new(
                        profile.requested_locale.clone(),
                        "source-language profile must serve the source locale",
                    ));
                }
            }
        }
    }
    if !has_source {
        findings.push(DoctorLocalizationFinding::new(
            packet.packet_id.clone(),
            "packet must declare an authoritative source-language profile",
        ));
    }
}

fn validate_support_export(
    packet: &DoctorReportLocalizationPacket,
    findings: &mut Vec<DoctorLocalizationFinding>,
) {
    let export = &packet.support_export;
    let expected = packet.build_support_export(&export.requested_locale);
    if *export != expected {
        findings.push(DoctorLocalizationFinding::new(
            export.export_id.clone(),
            "support export does not match the derived projection",
        ));
    }
    if export.raw_translated_bodies_exported {
        findings.push(DoctorLocalizationFinding::new(
            export.export_id.clone(),
            "support export must omit raw translated bodies",
        ));
    }
    if !export
        .rows
        .iter()
        .all(|row| row.raw_translated_body_omitted)
    {
        findings.push(DoctorLocalizationFinding::new(
            export.export_id.clone(),
            "support export row retains a raw translated body",
        ));
    }
    if export.preserved_finding_codes.is_empty() {
        findings.push(DoctorLocalizationFinding::new(
            export.export_id.clone(),
            "support export preserves no finding codes for escalation",
        ));
    }
}

fn validate_summary(
    packet: &DoctorReportLocalizationPacket,
    findings: &mut Vec<DoctorLocalizationFinding>,
) {
    let expected = derive_summary(
        &packet.entries,
        &packet.locale_profiles,
        &packet.source_language_locale,
    );
    if packet.summary != expected {
        findings.push(DoctorLocalizationFinding::new(
            packet.packet_id.clone(),
            "summary does not match the derived rows",
        ));
    }
}

fn derive_summary(
    entries: &[DoctorMessageEntry],
    locale_profiles: &[DoctorLocaleProfileRow],
    source_language_locale: &str,
) -> DoctorReportLocalizationSummary {
    let mut entries_by_surface = BTreeMap::new();
    for entry in entries {
        *entries_by_surface
            .entry(entry.surface.as_key().to_owned())
            .or_insert(0usize) += 1;
    }
    let finding_codes: BTreeSet<String> = entries
        .iter()
        .filter_map(|entry| entry.doctor_refs.finding_code_ref.clone())
        .collect();
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
        .flat_map(|entry| entry.doctor_refs.anchor_values())
        .collect();

    DoctorReportLocalizationSummary {
        total_entries: entries.len(),
        entries_by_surface,
        finding_codes_covered: finding_codes.len(),
        supported_locales: locale_profiles.len(),
        fully_localized_locales: fully_localized,
        source_language_fallback_locales: source_fallback,
        preserved_anchor_count: preserved_anchor_count.len(),
        source_language_locale: source_language_locale.to_owned(),
    }
}

/// Compact spec for a seeded Doctor message, expanded by [`build_entry`].
struct EntrySpec {
    message_id: &'static str,
    source_language_key: &'static str,
    surface: DoctorMessageSurface,
    source_text: &'static str,
    doctor_refs: DoctorStableRefs,
    placeholders: &'static [(&'static str, &'static str, &'static str)],
    machine_output_locale_class: MachineOutputLocaleClass,
    escape_hatches: &'static [SourceLanguageEscapeHatchClass],
    translated_in_locales: &'static [&'static str],
}

fn build_entry(spec: &EntrySpec, source_language_locale: &str) -> DoctorMessageEntry {
    let placeholders = spec
        .placeholders
        .iter()
        .map(|(id, kind, note)| MessagePlaceholder {
            placeholder_id: (*id).to_owned(),
            placeholder_kind: (*kind).to_owned(),
            translator_note: (*note).to_owned(),
        })
        .collect();
    DoctorMessageEntry {
        message_id: spec.message_id.to_owned(),
        source_language_key: spec.source_language_key.to_owned(),
        surface: spec.surface,
        surface_family: spec.surface.surface_family(),
        source_language_locale: source_language_locale.to_owned(),
        source_text: spec.source_text.to_owned(),
        doctor_refs: spec.doctor_refs.clone(),
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

/// Returns the seeded Doctor report message specs.
fn entry_specs() -> Vec<EntrySpec> {
    use DoctorMessageSurface::*;
    use MachineOutputLocaleClass::*;
    use SourceLanguageEscapeHatchClass::*;

    // One finding family runs across title / explanation / action, plus an
    // unsupported-state note and the two headings.
    let provider_auth = || DoctorStableRefs {
        finding_code_ref: Some("doctor.provider_auth.expired".to_owned()),
        probe_id_ref: Some("probe.provider_auth.v2".to_owned()),
        exit_class_ref: Some("doctor.exit.action_required".to_owned()),
        evidence_ref_kinds: vec![
            "auth_token_state".to_owned(),
            "provider_endpoint".to_owned(),
        ],
        scope_label_ref: Some("provider_auth".to_owned()),
        ..DoctorStableRefs::default()
    };

    vec![
        EntrySpec {
            message_id: "msg:doctor:provider-auth-expired:title",
            source_language_key: "doctor.finding.provider_auth.expired.title",
            surface: FindingTitle,
            source_text: "Provider sign-in has expired",
            doctor_refs: provider_auth(),
            placeholders: &[],
            machine_output_locale_class: LocaleNeutralWithTranslatedHumanField,
            escape_hatches: &[ExportInSourceLanguageForReview, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
        },
        EntrySpec {
            message_id: "msg:doctor:provider-auth-expired:explanation",
            source_language_key: "doctor.finding.provider_auth.expired.explanation",
            surface: FindingExplanation,
            source_text: "The saved credential for {provider} can no longer authenticate.",
            doctor_refs: provider_auth(),
            placeholders: &[(
                "provider",
                "literal_identifier",
                "Provider id; never translated.",
            )],
            machine_output_locale_class: LocaleNeutralWithTranslatedHumanField,
            escape_hatches: &[ExportInSourceLanguageForReview, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
        },
        EntrySpec {
            message_id: "msg:doctor:provider-auth-expired:action",
            source_language_key: "doctor.finding.provider_auth.expired.action",
            surface: RecommendedAction,
            source_text: "Sign in again to restore provider access",
            doctor_refs: DoctorStableRefs {
                recovery_command_id_ref: Some("workbench.action.signInProvider".to_owned()),
                docs_pack_key_ref: Some("doctor.recovery.provider_auth".to_owned()),
                ..provider_auth()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNeutralWithTranslatedHumanField,
            escape_hatches: &[ExportInSourceLanguageForReview, CommandOpenInSourceLanguage],
            translated_in_locales: &["es-MX", "ja-JP"],
        },
        EntrySpec {
            message_id: "msg:doctor:container-engine-unavailable:title",
            source_language_key: "doctor.finding.container_engine.unavailable.title",
            surface: FindingTitle,
            source_text: "Container engine is unavailable",
            doctor_refs: DoctorStableRefs {
                finding_code_ref: Some("doctor.container_engine.unavailable".to_owned()),
                probe_id_ref: Some("probe.container_engine.v1".to_owned()),
                exit_class_ref: Some("doctor.exit.unsupported".to_owned()),
                evidence_ref_kinds: vec!["engine_reachability".to_owned()],
                scope_label_ref: Some("container_engine".to_owned()),
                ..DoctorStableRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNeutralWithTranslatedHumanField,
            escape_hatches: &[ExportInSourceLanguageForReview, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX"],
        },
        EntrySpec {
            message_id: "msg:doctor:container-engine-unavailable:unsupported-note",
            source_language_key: "doctor.finding.container_engine.unavailable.unsupported",
            surface: UnsupportedStateNote,
            source_text: "Diagnosis is unsupported until an engine is reachable; staying local.",
            doctor_refs: DoctorStableRefs {
                finding_code_ref: Some("doctor.container_engine.unavailable".to_owned()),
                probe_id_ref: Some("probe.container_engine.v1".to_owned()),
                exit_class_ref: Some("doctor.exit.unsupported".to_owned()),
                evidence_ref_kinds: vec!["engine_reachability".to_owned()],
                scope_label_ref: Some("container_engine".to_owned()),
                policy_name_ref: Some("container.engine_required".to_owned()),
                ..DoctorStableRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNeutralWithTranslatedHumanField,
            escape_hatches: &[ExportInSourceLanguageForReview, InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP"],
        },
        EntrySpec {
            message_id: "msg:doctor:report:summary-heading",
            source_language_key: "doctor.report.summary_heading",
            surface: ReportHeading,
            source_text: "Diagnosis summary",
            doctor_refs: DoctorStableRefs {
                docs_pack_key_ref: Some("doctor.report.summary".to_owned()),
                telemetry_key_ref: Some("doctor.report.shown".to_owned()),
                ..DoctorStableRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[ExportInSourceLanguageForReview, DocsPaneSourceLanguageRoute],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
        },
        EntrySpec {
            message_id: "msg:doctor:support-export:locale-heading",
            source_language_key: "doctor.support_export.locale_heading",
            surface: SupportExportHeading,
            source_text: "Language and fallback state",
            doctor_refs: DoctorStableRefs {
                docs_pack_key_ref: Some("doctor.support_export.locale_state".to_owned()),
                ..DoctorStableRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[ExportInSourceLanguageForReview, DocsPaneSourceLanguageRoute],
            translated_in_locales: &["es-MX", "ja-JP"],
        },
    ]
}

/// Returns the seeded requested-locale fallback profiles.
fn seeded_locale_profiles(
    entries: &[DoctorMessageEntry],
    source: &str,
) -> Vec<DoctorLocaleProfileRow> {
    let missing = |locale: &str| {
        entries
            .iter()
            .filter(|entry| !entry.covered_in_locale(locale, source))
            .count()
    };
    vec![
        DoctorLocaleProfileRow {
            requested_locale: source.to_owned(),
            effective_locale: source.to_owned(),
            source_language_locale: source.to_owned(),
            fallback_chain: vec![source.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
            degraded_state: DegradedLocalizationState::FullyLocalized,
            source_language_route_active: false,
            missing_key_count: missing(source),
        },
        DoctorLocaleProfileRow {
            requested_locale: "es-MX".to_owned(),
            effective_locale: "es-MX".to_owned(),
            source_language_locale: source.to_owned(),
            fallback_chain: vec!["es-MX".to_owned(), "es".to_owned(), source.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
            degraded_state: DegradedLocalizationState::FullyLocalized,
            source_language_route_active: false,
            missing_key_count: missing("es-MX"),
        },
        DoctorLocaleProfileRow {
            requested_locale: "ja-JP".to_owned(),
            effective_locale: "ja-JP".to_owned(),
            source_language_locale: source.to_owned(),
            fallback_chain: vec!["ja-JP".to_owned(), "ja".to_owned(), source.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill,
            degraded_state: DegradedLocalizationState::PartialTranslationDisclosed,
            source_language_route_active: true,
            missing_key_count: missing("ja-JP"),
        },
        DoctorLocaleProfileRow {
            requested_locale: "ar-SA".to_owned(),
            effective_locale: "ar-SA".to_owned(),
            source_language_locale: source.to_owned(),
            fallback_chain: vec!["ar-SA".to_owned(), "ar".to_owned(), source.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill,
            degraded_state: DegradedLocalizationState::PartialTranslationDisclosed,
            source_language_route_active: true,
            missing_key_count: missing("ar-SA"),
        },
        DoctorLocaleProfileRow {
            requested_locale: "de-DE".to_owned(),
            effective_locale: source.to_owned(),
            source_language_locale: source.to_owned(),
            fallback_chain: vec!["de-DE".to_owned(), "de".to_owned(), source.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::PackMissingSourceLanguageOnly,
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
        "crates/aureline-doctor".to_owned(),
        "crates/aureline-support".to_owned(),
        "crates/aureline-incident".to_owned(),
    ]
}

/// Returns the seeded Doctor report localization posture packet.
pub fn seeded_doctor_report_localization_packet() -> DoctorReportLocalizationPacket {
    let entries: Vec<DoctorMessageEntry> = entry_specs()
        .iter()
        .map(|spec| build_entry(spec, SOURCE_LANGUAGE_LOCALE))
        .collect();
    let locale_profiles = seeded_locale_profiles(&entries, SOURCE_LANGUAGE_LOCALE);
    let summary = derive_summary(&entries, &locale_profiles, SOURCE_LANGUAGE_LOCALE);

    let mut packet = DoctorReportLocalizationPacket {
        record_kind: DOCTOR_REPORT_LOCALIZATION_RECORD_KIND.to_owned(),
        schema_version: DOCTOR_REPORT_LOCALE_SCHEMA_VERSION,
        packet_id: DOCTOR_REPORT_LOCALIZATION_PACKET_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        target_build_identity_ref: TARGET_BUILD.to_owned(),
        source_contract_refs: seeded_source_contract_refs(),
        runtime_consumer_refs: seeded_runtime_consumer_refs(),
        entries,
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
fn placeholder_support_export() -> DoctorReportSupportExport {
    DoctorReportSupportExport {
        record_kind: DOCTOR_REPORT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: DOCTOR_REPORT_LOCALE_SCHEMA_VERSION,
        export_id: String::new(),
        source_packet_id: DOCTOR_REPORT_LOCALIZATION_PACKET_ID.to_owned(),
        requested_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        effective_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        fallback_chain: vec![SOURCE_LANGUAGE_LOCALE.to_owned()],
        fallback_origin: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
        degraded_state: DegradedLocalizationState::FullyLocalized,
        missing_key_count: 0,
        source_language_route_active: false,
        preserved_finding_codes: Vec::new(),
        rows: Vec::new(),
        raw_translated_bodies_exported: false,
        omitted_material_classes: omitted_material_classes(),
        generated_at: GENERATED_AT.to_owned(),
    }
}
