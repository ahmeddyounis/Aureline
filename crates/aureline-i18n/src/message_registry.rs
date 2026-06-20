//! Stable message-id registry and source-language fallback truth for the new
//! M5 shell, command, settings, help, error, and notification surfaces.
//!
//! This module materializes a checked-in catalog that binds every translatable
//! string on those surfaces to a stable, locale-neutral message id and a stable
//! source-language key. Command ids, setting ids, diagnostic ids, telemetry
//! keys, and policy names live next to the message id, never behind localized
//! prose, so command routing, analytics, policy, and export tooling keep
//! working when copy changes or a locale pack is missing.
//!
//! The registry is built to prove two continuity claims that the spec treats as
//! release-bearing:
//!
//! - **Across locale changes** — a message id never carries a locale tag, so the
//!   id set rendered for one locale is identical to the id set rendered for any
//!   other. [`M5MessageRegistry::render`] returns the same ids regardless of the
//!   requested locale; only the effective locale and the source-language
//!   fallback flag change.
//! - **Across release builds** — [`M5MessageRegistry::continuity_against`] diffs
//!   the current registry against a frozen [`MessageIdBaselineSnapshot`] from a
//!   prior build. Ids may be added, but a removed id or a drifted source-language
//!   key fails the continuity check.
//!
//! Settings, Help/About, diagnostics, and support-export surfaces ingest this
//! packet (and the shell-side fallback inspector that projects it) instead of
//! cloning localization status prose. Raw translated bodies, signing keys, and
//! credentials never cross this boundary.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    DegradedLocalizationState, LocaleFallbackOriginClass, LocalePackValidationFinding,
    MachineOutputLocaleClass, MessageIdClass, MessagePlaceholder, MessageSurfaceFamily,
    SourceLanguageEscapeHatchClass, StableMessageIdentityRefs, GENERATED_AT,
    SOURCE_LANGUAGE_LOCALE, TARGET_BUILD,
};

/// Schema version for the M5 message-id registry and baseline snapshot.
pub const M5_MESSAGE_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`M5MessageRegistry`].
pub const M5_MESSAGE_REGISTRY_RECORD_KIND: &str = "m5_message_registry_packet";

/// Record kind for [`MessageIdBaselineSnapshot`].
pub const M5_MESSAGE_ID_BASELINE_RECORD_KIND: &str = "m5_message_id_baseline_snapshot";

/// Stable packet id for the seeded M5 message-id registry.
pub const M5_MESSAGE_REGISTRY_PACKET_ID: &str =
    "i18n:m5-message-registry:shell-command-settings-help-error-notification:v1";

/// Stable id for the seeded prior-release baseline snapshot.
pub const M5_MESSAGE_ID_BASELINE_SNAPSHOT_ID: &str = "i18n:m5-message-id-baseline:prior-release:v1";

/// Fixture path for the seeded M5 message-id registry.
pub const M5_MESSAGE_REGISTRY_FIXTURE_REF: &str =
    "fixtures/i18n/message-id-stability/registry.json";

/// Fixture path for the seeded prior-release baseline snapshot.
pub const M5_MESSAGE_ID_BASELINE_FIXTURE_REF: &str =
    "fixtures/i18n/message-id-stability/baseline-ids.json";

/// Prior release build whose message ids the registry must preserve.
const BASELINE_BUILD: &str = "build:aureline:0.0.0-beta.2026.04.20";

/// New M5 surface family that owns a registered message.
///
/// This is the finer-grained surface dimension the spec names directly. Each
/// variant maps to a shared [`MessageSurfaceFamily`] for downstream tooling that
/// already speaks the message-catalog vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MessageSurface {
    /// Shell title bars, status areas, and switcher labels.
    ShellChrome,
    /// Command-palette and menu labels bound to canonical commands.
    CommandPalette,
    /// Settings rows, labels, and descriptions bound to stable setting ids.
    Settings,
    /// Help, About, and docs cards bound to docs-pack keys.
    Help,
    /// Error, denial, and disabled-state explanations bound to diagnostic ids.
    Error,
    /// Toasts, banners, and OS notifications.
    Notification,
}

impl M5MessageSurface {
    /// All M5 surfaces the registry is required to cover.
    pub const ALL: [M5MessageSurface; 6] = [
        M5MessageSurface::ShellChrome,
        M5MessageSurface::CommandPalette,
        M5MessageSurface::Settings,
        M5MessageSurface::Help,
        M5MessageSurface::Error,
        M5MessageSurface::Notification,
    ];

    /// Returns the shared message-catalog family for this surface.
    pub const fn surface_family(self) -> MessageSurfaceFamily {
        match self {
            // Notifications render in shell chrome surfaces, so they share its family.
            Self::ShellChrome | Self::Notification => MessageSurfaceFamily::ShellChrome,
            Self::CommandPalette => MessageSurfaceFamily::CommandLabel,
            Self::Settings | Self::Error => MessageSurfaceFamily::SettingsHelpOrError,
            Self::Help => MessageSurfaceFamily::DocsTourOrAuthText,
        }
    }

    /// Returns a stable snake_case key for the surface.
    pub const fn as_key(self) -> &'static str {
        match self {
            Self::ShellChrome => "shell_chrome",
            Self::CommandPalette => "command_palette",
            Self::Settings => "settings",
            Self::Help => "help",
            Self::Error => "error",
            Self::Notification => "notification",
        }
    }
}

/// One registered message: a stable id, a source-language key, and the stable
/// non-prose refs that business logic routes by.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageRegistryEntry {
    /// Stable, locale-neutral message id.
    pub message_id: String,
    /// Stable source-language catalog key.
    pub source_language_key: String,
    /// M5 surface that owns the message.
    pub surface: M5MessageSurface,
    /// Shared message-catalog family for the surface.
    pub surface_family: MessageSurfaceFamily,
    /// Message-id class.
    pub message_id_class: MessageIdClass,
    /// Product source-language locale for the source text.
    pub source_language_locale: String,
    /// Short source-language template summary (no localized prose ships here).
    pub source_text: String,
    /// Stable non-prose identity refs bound to this message.
    pub stable_refs: StableMessageIdentityRefs,
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
    /// Build identity in which this message id was introduced.
    pub introduced_in_build_ref: String,
    /// Whether this id existed in the prior-release baseline snapshot.
    pub present_in_baseline_build: bool,
}

impl MessageRegistryEntry {
    /// Returns true when `requested_locale` resolves to a translation for this
    /// message, either exactly or through its language base.
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

/// Requested-locale fallback profile describing the chain and degraded state a
/// surface reaches before any per-message coverage is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleProfileRow {
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
}

/// Continuity verdict for one message id compared across release builds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageIdContinuityState {
    /// Id and source-language key are unchanged since the baseline build.
    StableAcrossBuilds,
    /// Id is new in the current build and absent from the baseline.
    NewlyIntroduced,
    /// Baseline id is missing from the current build without a governed removal.
    RemovedWithoutGovernance,
    /// Baseline id survives but its source-language key drifted.
    KeyDrift,
}

impl MessageIdContinuityState {
    /// Returns true when this state breaks message-id continuity.
    pub const fn is_break(self) -> bool {
        matches!(self, Self::RemovedWithoutGovernance | Self::KeyDrift)
    }
}

/// Continuity row for one message id across the baseline and current builds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageIdContinuityRow {
    /// Message id under comparison.
    pub message_id: String,
    /// Continuity verdict.
    pub state: MessageIdContinuityState,
    /// Source-language key recorded in the baseline, when present there.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline_source_language_key: Option<String>,
    /// Source-language key in the current registry, when present here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_source_language_key: Option<String>,
}

/// Continuity report comparing the registry against a prior-release baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageIdContinuityReport {
    /// Prior-release build identity.
    pub baseline_build_identity_ref: String,
    /// Current build identity.
    pub target_build_identity_ref: String,
    /// Per-id continuity rows, sorted by message id.
    pub rows: Vec<MessageIdContinuityRow>,
    /// Number of ids preserved unchanged.
    pub preserved_count: usize,
    /// Number of ids newly introduced in the current build.
    pub added_count: usize,
    /// Number of baseline ids removed without governance.
    pub removed_count: usize,
    /// Number of baseline ids whose source-language key drifted.
    pub key_drift_count: usize,
}

impl MessageIdContinuityReport {
    /// Returns true when no id was removed or had its key drift.
    pub fn is_stable(&self) -> bool {
        self.removed_count == 0 && self.key_drift_count == 0
    }
}

/// One row in the prior-release baseline snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageIdBaselineRow {
    /// Stable message id frozen at the baseline build.
    pub message_id: String,
    /// Source-language key frozen at the baseline build.
    pub source_language_key: String,
    /// M5 surface that owned the message at the baseline build.
    pub surface: M5MessageSurface,
}

/// Frozen snapshot of message ids and source-language keys from a prior build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageIdBaselineSnapshot {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Build identity this snapshot was frozen at.
    pub build_identity_ref: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Frozen id rows, sorted by message id.
    pub ids: Vec<MessageIdBaselineRow>,
}

impl MessageIdBaselineSnapshot {
    /// Validates the snapshot shape and id uniqueness.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();
        if self.record_kind != M5_MESSAGE_ID_BASELINE_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.snapshot_id.clone(),
                "baseline snapshot record_kind is unsupported",
            ));
        }
        if self.schema_version != M5_MESSAGE_REGISTRY_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.snapshot_id.clone(),
                "baseline snapshot schema_version is unsupported",
            ));
        }
        let mut seen = BTreeSet::new();
        for row in &self.ids {
            if !seen.insert(row.message_id.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    row.message_id.clone(),
                    "baseline snapshot repeats a message id",
                ));
            }
            if id_carries_locale_tag(&row.message_id) {
                findings.push(LocalePackValidationFinding::new(
                    row.message_id.clone(),
                    "baseline message id carries a locale tag",
                ));
            }
        }
        finish(findings)
    }
}

/// Rendered message id for one requested locale.
///
/// The id and key are independent of the requested locale, which is what proves
/// message-id continuity across locale changes; only the effective locale and
/// the fallback flag vary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderedMessageId {
    /// Stable message id.
    pub message_id: String,
    /// Stable source-language key.
    pub source_language_key: String,
    /// Locale that produced the rendered message.
    pub effective_locale: String,
    /// Whether this message fell back to the source language.
    pub used_source_language_fallback: bool,
}

/// Summary posture derived from the registry rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageRegistrySummary {
    /// Total registered messages.
    pub total_entries: usize,
    /// Entry count per surface, keyed by [`M5MessageSurface::as_key`].
    pub entries_by_surface: BTreeMap<String, usize>,
    /// Messages present in the prior-release baseline.
    pub baseline_entries: usize,
    /// Messages newly introduced in the current build.
    pub newly_introduced_entries: usize,
    /// Number of supported requested locales.
    pub supported_locales: usize,
    /// Locales served with full requested-locale coverage.
    pub fully_localized_locales: usize,
    /// Locales served by source-language fallback only.
    pub source_language_fallback_locales: usize,
    /// Whether the seeded registry preserves the baseline ids.
    pub continuity_stable: bool,
    /// Product source-language locale.
    pub source_language_locale: String,
}

/// Stable message-id registry for the new M5 surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MessageRegistry {
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
    /// Prior-release build identity the registry preserves ids against.
    pub baseline_build_identity_ref: String,
    /// Source contracts that govern this packet.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime consumers that ingest this packet.
    pub runtime_consumer_refs: Vec<String>,
    /// Registered messages, grouped by surface in registration order.
    pub entries: Vec<MessageRegistryEntry>,
    /// Requested-locale fallback profiles.
    pub locale_profiles: Vec<LocaleProfileRow>,
    /// Summary posture derived from the rows.
    pub summary: MessageRegistrySummary,
}

impl M5MessageRegistry {
    /// Returns an entry by message id.
    pub fn entry(&self, message_id: &str) -> Option<&MessageRegistryEntry> {
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
    pub fn locale_profile(&self, requested_locale: &str) -> Option<&LocaleProfileRow> {
        self.locale_profiles
            .iter()
            .find(|profile| profile.requested_locale == requested_locale)
    }

    /// Returns the message ids missing a translation for a requested locale.
    ///
    /// These are the ids that must fall back to the source language; the count
    /// is what the shell fallback inspector reports as the missing-key count.
    pub fn missing_message_ids(&self, requested_locale: &str) -> Vec<String> {
        self.entries
            .iter()
            .filter(|entry| {
                !entry.covered_in_locale(requested_locale, &self.source_language_locale)
            })
            .map(|entry| entry.message_id.clone())
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

    /// Returns the missing-key count for a requested locale on one surface.
    pub fn missing_key_count_for_surface(
        &self,
        requested_locale: &str,
        surface: M5MessageSurface,
    ) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.surface == surface)
            .filter(|entry| {
                !entry.covered_in_locale(requested_locale, &self.source_language_locale)
            })
            .count()
    }

    /// Renders message ids for a requested locale.
    ///
    /// Ids and keys are independent of the locale; only the effective locale and
    /// the source-language fallback flag vary per message.
    pub fn render(&self, requested_locale: &str) -> Vec<RenderedMessageId> {
        self.entries
            .iter()
            .map(|entry| {
                let covered =
                    entry.covered_in_locale(requested_locale, &self.source_language_locale);
                RenderedMessageId {
                    message_id: entry.message_id.clone(),
                    source_language_key: entry.source_language_key.clone(),
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

    /// Diffs the registry against a prior-release baseline snapshot.
    pub fn continuity_against(
        &self,
        baseline: &MessageIdBaselineSnapshot,
    ) -> MessageIdContinuityReport {
        let baseline_by_id: BTreeMap<&str, &MessageIdBaselineRow> = baseline
            .ids
            .iter()
            .map(|row| (row.message_id.as_str(), row))
            .collect();
        let current_by_id: BTreeMap<&str, &MessageRegistryEntry> = self
            .entries
            .iter()
            .map(|entry| (entry.message_id.as_str(), entry))
            .collect();

        let mut rows = Vec::new();
        let (mut preserved, mut added, mut removed, mut key_drift) = (0, 0, 0, 0);

        for row in &baseline.ids {
            match current_by_id.get(row.message_id.as_str()) {
                None => {
                    removed += 1;
                    rows.push(MessageIdContinuityRow {
                        message_id: row.message_id.clone(),
                        state: MessageIdContinuityState::RemovedWithoutGovernance,
                        baseline_source_language_key: Some(row.source_language_key.clone()),
                        current_source_language_key: None,
                    });
                }
                Some(entry) if entry.source_language_key != row.source_language_key => {
                    key_drift += 1;
                    rows.push(MessageIdContinuityRow {
                        message_id: row.message_id.clone(),
                        state: MessageIdContinuityState::KeyDrift,
                        baseline_source_language_key: Some(row.source_language_key.clone()),
                        current_source_language_key: Some(entry.source_language_key.clone()),
                    });
                }
                Some(entry) => {
                    preserved += 1;
                    rows.push(MessageIdContinuityRow {
                        message_id: row.message_id.clone(),
                        state: MessageIdContinuityState::StableAcrossBuilds,
                        baseline_source_language_key: Some(row.source_language_key.clone()),
                        current_source_language_key: Some(entry.source_language_key.clone()),
                    });
                }
            }
        }

        for entry in &self.entries {
            if baseline_by_id.contains_key(entry.message_id.as_str()) {
                continue;
            }
            added += 1;
            rows.push(MessageIdContinuityRow {
                message_id: entry.message_id.clone(),
                state: MessageIdContinuityState::NewlyIntroduced,
                baseline_source_language_key: None,
                current_source_language_key: Some(entry.source_language_key.clone()),
            });
        }

        rows.sort_by(|left, right| left.message_id.cmp(&right.message_id));

        MessageIdContinuityReport {
            baseline_build_identity_ref: baseline.build_identity_ref.clone(),
            target_build_identity_ref: self.target_build_identity_ref.clone(),
            rows,
            preserved_count: preserved,
            added_count: added,
            removed_count: removed,
            key_drift_count: key_drift,
        }
    }

    /// Validates the registry shape, stable-id discipline, and locale profiles.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != M5_MESSAGE_REGISTRY_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "registry record_kind is unsupported",
            ));
        }
        if self.schema_version != M5_MESSAGE_REGISTRY_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "registry schema_version is unsupported",
            ));
        }
        if self.entries.is_empty() {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "registry has no message entries",
            ));
        }

        validate_entries(self, &mut findings);
        validate_surface_coverage(self, &mut findings);
        validate_locale_profiles(self, &mut findings);
        validate_summary(self, &mut findings);

        finish(findings)
    }
}

/// Returns the language-base portion of a locale tag (e.g. `es` for `es-MX`).
fn locale_base(locale: &str) -> &str {
    locale
        .split_once('-')
        .map(|(base, _)| base)
        .unwrap_or(locale)
}

/// Returns true when a message id or key embeds a locale tag.
///
/// Locale tags belong in fallback state, never in stable ids; an id like
/// `msg:shell:title:es-MX` would silently break continuity across locale
/// changes. The check is intentionally narrow: a two-letter language segment
/// optionally followed by a region (e.g. `en`, `es-mx`).
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

fn validate_entries(registry: &M5MessageRegistry, findings: &mut Vec<LocalePackValidationFinding>) {
    let mut ids = BTreeSet::new();
    let mut keys = BTreeSet::new();
    for entry in &registry.entries {
        if entry.message_id.is_empty() {
            findings.push(LocalePackValidationFinding::new(
                registry.packet_id.clone(),
                "entry has an empty message id",
            ));
        }
        if !ids.insert(entry.message_id.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "duplicate message id",
            ));
        }
        if !keys.insert(entry.source_language_key.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "duplicate source-language key",
            ));
        }
        if entry.source_language_key.is_empty() {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "entry is missing a source-language key",
            ));
        }
        if id_carries_locale_tag(&entry.message_id) {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "message id carries a locale tag",
            ));
        }
        if id_carries_locale_tag(&entry.source_language_key) {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "source-language key carries a locale tag",
            ));
        }
        if entry.source_language_locale != registry.source_language_locale {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "entry source-language locale differs from the registry",
            ));
        }
        if entry.surface_family != entry.surface.surface_family() {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "entry surface_family does not match its surface",
            ));
        }
        if !entry.stable_refs.has_anchor() {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "entry has no stable non-prose anchor for routing",
            ));
        }
        if entry.routed_by_localized_prose {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "entry routes behavior by localized prose",
            ));
        }
        if !entry.machine_identifier_fields_locale_neutral {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "entry machine identifiers are not locale-neutral",
            ));
        }
        let expects_baseline = entry.introduced_in_build_ref == BASELINE_BUILD;
        if entry.present_in_baseline_build != expects_baseline {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "entry baseline flag disagrees with its introducing build",
            ));
        }
        if !entry.present_in_baseline_build
            && entry.introduced_in_build_ref != registry.target_build_identity_ref
        {
            findings.push(LocalePackValidationFinding::new(
                entry.message_id.clone(),
                "newly introduced entry must cite the current build",
            ));
        }
    }
}

fn validate_surface_coverage(
    registry: &M5MessageRegistry,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let surfaces: BTreeSet<M5MessageSurface> =
        registry.entries.iter().map(|entry| entry.surface).collect();
    for required in M5MessageSurface::ALL {
        if !surfaces.contains(&required) {
            findings.push(LocalePackValidationFinding::new(
                registry.packet_id.clone(),
                format!("registry is missing surface {}", required.as_key()),
            ));
        }
    }
}

fn validate_locale_profiles(
    registry: &M5MessageRegistry,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let total = registry.entries.len();
    let mut seen = BTreeSet::new();
    let mut has_source = false;
    for profile in &registry.locale_profiles {
        if !seen.insert(profile.requested_locale.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                profile.requested_locale.clone(),
                "duplicate locale profile",
            ));
        }
        if profile.fallback_chain.first() != Some(&profile.requested_locale) {
            findings.push(LocalePackValidationFinding::new(
                profile.requested_locale.clone(),
                "fallback chain must start at the requested locale",
            ));
        }
        if profile.fallback_chain.last() != Some(&registry.source_language_locale) {
            findings.push(LocalePackValidationFinding::new(
                profile.requested_locale.clone(),
                "fallback chain must end at the source language",
            ));
        }
        if profile.source_language_locale != registry.source_language_locale {
            findings.push(LocalePackValidationFinding::new(
                profile.requested_locale.clone(),
                "profile source-language locale differs from the registry",
            ));
        }

        let missing = registry.missing_key_count(&profile.requested_locale);
        match profile.fallback_origin {
            LocaleFallbackOriginClass::RequestedLocaleAuthoritative => {
                if missing != 0 {
                    findings.push(LocalePackValidationFinding::new(
                        profile.requested_locale.clone(),
                        "authoritative profile still has missing keys",
                    ));
                }
                if profile.requested_locale == registry.source_language_locale {
                    has_source = true;
                }
            }
            LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill
            | LocaleFallbackOriginClass::BaseLocaleFallback => {
                if missing == 0 || missing == total {
                    findings.push(LocalePackValidationFinding::new(
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
                    findings.push(LocalePackValidationFinding::new(
                        profile.requested_locale.clone(),
                        "source-language profile must have every key missing",
                    ));
                }
                if profile.effective_locale != registry.source_language_locale {
                    findings.push(LocalePackValidationFinding::new(
                        profile.requested_locale.clone(),
                        "source-language profile must serve the source locale",
                    ));
                }
            }
        }
    }
    if !has_source {
        findings.push(LocalePackValidationFinding::new(
            registry.packet_id.clone(),
            "registry must declare an authoritative source-language profile",
        ));
    }
}

fn validate_summary(registry: &M5MessageRegistry, findings: &mut Vec<LocalePackValidationFinding>) {
    let expected = derive_summary(
        &registry.entries,
        &registry.locale_profiles,
        registry.continuity_stable_against_baseline(),
        &registry.source_language_locale,
    );
    if registry.summary != expected {
        findings.push(LocalePackValidationFinding::new(
            registry.packet_id.clone(),
            "summary does not match the derived rows",
        ));
    }
}

impl M5MessageRegistry {
    /// Returns whether the registry preserves the seeded baseline ids.
    fn continuity_stable_against_baseline(&self) -> bool {
        self.continuity_against(&seeded_m5_message_id_baseline())
            .is_stable()
    }
}

fn derive_summary(
    entries: &[MessageRegistryEntry],
    locale_profiles: &[LocaleProfileRow],
    continuity_stable: bool,
    source_language_locale: &str,
) -> MessageRegistrySummary {
    let mut entries_by_surface = BTreeMap::new();
    for entry in entries {
        *entries_by_surface
            .entry(entry.surface.as_key().to_owned())
            .or_insert(0usize) += 1;
    }
    let baseline_entries = entries
        .iter()
        .filter(|entry| entry.present_in_baseline_build)
        .count();
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

    MessageRegistrySummary {
        total_entries: entries.len(),
        entries_by_surface,
        baseline_entries,
        newly_introduced_entries: entries.len() - baseline_entries,
        supported_locales: locale_profiles.len(),
        fully_localized_locales: fully_localized,
        source_language_fallback_locales: source_fallback,
        continuity_stable,
        source_language_locale: source_language_locale.to_owned(),
    }
}

/// Compact spec for a seeded message, expanded by [`build_entry`].
struct EntrySpec {
    message_id: &'static str,
    source_language_key: &'static str,
    surface: M5MessageSurface,
    message_id_class: MessageIdClass,
    source_text: &'static str,
    stable_refs: StableMessageIdentityRefs,
    placeholders: &'static [(&'static str, &'static str, &'static str)],
    machine_output_locale_class: MachineOutputLocaleClass,
    escape_hatches: &'static [SourceLanguageEscapeHatchClass],
    translated_in_locales: &'static [&'static str],
    localized_human_prose_allowed: bool,
    since_baseline: bool,
}

fn command_ref(command_id: &str, telemetry_key: Option<&str>) -> StableMessageIdentityRefs {
    StableMessageIdentityRefs {
        command_id_ref: Some(command_id.to_owned()),
        telemetry_key_ref: telemetry_key.map(str::to_owned),
        ..StableMessageIdentityRefs::default()
    }
}

fn build_entry(
    spec: &EntrySpec,
    source_language_locale: &str,
    target_build: &str,
) -> MessageRegistryEntry {
    let placeholders = spec
        .placeholders
        .iter()
        .map(|(id, kind, note)| MessagePlaceholder {
            placeholder_id: (*id).to_owned(),
            placeholder_kind: (*kind).to_owned(),
            translator_note: (*note).to_owned(),
        })
        .collect();
    MessageRegistryEntry {
        message_id: spec.message_id.to_owned(),
        source_language_key: spec.source_language_key.to_owned(),
        surface: spec.surface,
        surface_family: spec.surface.surface_family(),
        message_id_class: spec.message_id_class,
        source_language_locale: source_language_locale.to_owned(),
        source_text: spec.source_text.to_owned(),
        stable_refs: spec.stable_refs.clone(),
        placeholders,
        machine_output_locale_class: spec.machine_output_locale_class,
        source_language_escape_hatches: spec.escape_hatches.to_vec(),
        translated_in_locales: spec
            .translated_in_locales
            .iter()
            .map(|locale| (*locale).to_owned())
            .collect(),
        localized_human_prose_allowed: spec.localized_human_prose_allowed,
        machine_identifier_fields_locale_neutral: true,
        routed_by_localized_prose: false,
        introduced_in_build_ref: if spec.since_baseline {
            BASELINE_BUILD.to_owned()
        } else {
            target_build.to_owned()
        },
        present_in_baseline_build: spec.since_baseline,
    }
}

/// Returns the seeded entry specs for the new M5 surfaces.
fn entry_specs() -> Vec<EntrySpec> {
    use M5MessageSurface::*;
    use MachineOutputLocaleClass::*;
    use MessageIdClass::StableCanonical;
    use SourceLanguageEscapeHatchClass::*;

    vec![
        // Shell chrome.
        EntrySpec {
            message_id: "msg:shell:title-bar:workspace-name",
            source_language_key: "shell.title_bar.workspace_name",
            surface: ShellChrome,
            message_id_class: StableCanonical,
            source_text: "{workspace_name} — Aureline",
            stable_refs: StableMessageIdentityRefs {
                telemetry_key_ref: Some("shell.title_bar.shown".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: &[(
                "workspace_name",
                "literal_identifier",
                "Workspace name; never translated or reordered.",
            )],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        EntrySpec {
            message_id: "msg:shell:status-bar:background-work",
            source_language_key: "shell.status_bar.background_work",
            surface: ShellChrome,
            message_id_class: StableCanonical,
            source_text: "{count} background tasks running",
            stable_refs: command_ref(
                "workbench.action.showBackgroundWork",
                Some("shell.status_bar.background_work"),
            ),
            placeholders: &[(
                "count",
                "plural_count",
                "Running task count; pluralization is locale-sensitive.",
            )],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle, CommandOpenInSourceLanguage],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        EntrySpec {
            message_id: "msg:shell:switcher:open-window",
            source_language_key: "shell.switcher.open_window",
            surface: ShellChrome,
            message_id_class: StableCanonical,
            source_text: "Switch window",
            stable_refs: command_ref("workbench.action.switchWindow", None),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        // Command palette.
        EntrySpec {
            message_id: "msg:command:run-build",
            source_language_key: "command.tasks.run_build.label",
            surface: CommandPalette,
            message_id_class: StableCanonical,
            source_text: "Run Build Task",
            stable_refs: command_ref(
                "workbench.action.tasks.runBuild",
                Some("command.run_build.invoked"),
            ),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle, CommandOpenInSourceLanguage],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        EntrySpec {
            message_id: "msg:command:open-settings",
            source_language_key: "command.preferences.open_settings.label",
            surface: CommandPalette,
            message_id_class: StableCanonical,
            source_text: "Open Settings",
            stable_refs: command_ref("workbench.action.openSettings", None),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle, CommandOpenInSourceLanguage],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        EntrySpec {
            message_id: "msg:command:open-notebook",
            source_language_key: "command.notebook.open.label",
            surface: CommandPalette,
            message_id_class: StableCanonical,
            source_text: "Open Notebook",
            stable_refs: command_ref("notebook.action.open", None),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle, CommandOpenInSourceLanguage],
            translated_in_locales: &["es-MX"],
            localized_human_prose_allowed: true,
            since_baseline: false,
        },
        // Settings.
        EntrySpec {
            message_id: "msg:settings:locale:active-language",
            source_language_key: "settings.i18n.active_language.label",
            surface: Settings,
            message_id_class: StableCanonical,
            source_text: "Display language",
            stable_refs: StableMessageIdentityRefs {
                setting_id_ref: Some("i18n.activeLocale".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        EntrySpec {
            message_id: "msg:settings:locale:fallback-disclosure",
            source_language_key: "settings.i18n.fallback_disclosure.label",
            surface: Settings,
            message_id_class: StableCanonical,
            source_text: "Show source-language fallback notices",
            stable_refs: StableMessageIdentityRefs {
                setting_id_ref: Some("i18n.fallbackDisclosure".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        EntrySpec {
            message_id: "msg:settings:editor:font-size",
            source_language_key: "settings.editor.font_size.label",
            surface: Settings,
            message_id_class: StableCanonical,
            source_text: "Editor font size",
            stable_refs: StableMessageIdentityRefs {
                setting_id_ref: Some("editor.fontSize".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        // Help.
        EntrySpec {
            message_id: "msg:help:about:locale-provenance",
            source_language_key: "help.about.locale_provenance.title",
            surface: Help,
            message_id_class: StableCanonical,
            source_text: "Language and locale-pack provenance",
            stable_refs: StableMessageIdentityRefs {
                docs_pack_key_ref: Some("help.about.locale_provenance".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle, DocsPaneSourceLanguageRoute],
            translated_in_locales: &["es-MX"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        EntrySpec {
            message_id: "msg:help:docs:getting-started",
            source_language_key: "docs.getting_started.title",
            surface: Help,
            message_id_class: StableCanonical,
            source_text: "Getting started",
            stable_refs: StableMessageIdentityRefs {
                docs_pack_key_ref: Some("docs.getting_started".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle, DocsPaneSourceLanguageRoute],
            translated_in_locales: &["es-MX", "ja-JP"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        // Error.
        EntrySpec {
            message_id: "msg:error:locale-pack:signature-failed",
            source_language_key: "error.i18n.locale_pack.signature_failed",
            surface: Error,
            message_id_class: StableCanonical,
            source_text: "Locale pack signature could not be verified; showing source language.",
            stable_refs: StableMessageIdentityRefs {
                diagnostic_id_ref: Some("i18n.locale_pack.signature_failed".to_owned()),
                policy_name_ref: Some("locale_pack.signature_required".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNeutralWithTranslatedHumanField,
            escape_hatches: &[InlineSourceLanguageToggle, ExportInSourceLanguageForReview],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        EntrySpec {
            message_id: "msg:error:command:disabled-reason",
            source_language_key: "error.command.disabled_reason",
            surface: Error,
            message_id_class: StableCanonical,
            source_text: "{command} is unavailable: {reason}",
            stable_refs: StableMessageIdentityRefs {
                diagnostic_id_ref: Some("command.disabled.reason".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: &[
                (
                    "command",
                    "literal_identifier",
                    "Canonical command id; never translated.",
                ),
                (
                    "reason",
                    "stable_reason_code",
                    "Stable disabled-reason code rendered with its localized gloss.",
                ),
            ],
            machine_output_locale_class: LocaleNeutralWithTranslatedHumanField,
            escape_hatches: &[InlineSourceLanguageToggle, ExportInSourceLanguageForReview],
            translated_in_locales: &["es-MX", "ja-JP"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        // Notification.
        EntrySpec {
            message_id: "msg:notification:update:ready",
            source_language_key: "notification.update.ready",
            surface: Notification,
            message_id_class: StableCanonical,
            source_text: "An update is ready to install",
            stable_refs: command_ref(
                "workbench.action.applyUpdate",
                Some("notification.update.shown"),
            ),
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX", "ja-JP", "ar-SA"],
            localized_human_prose_allowed: true,
            since_baseline: true,
        },
        EntrySpec {
            message_id: "msg:notification:locale-pack:fallback-active",
            source_language_key: "notification.i18n.fallback_active",
            surface: Notification,
            message_id_class: StableCanonical,
            source_text: "Some surfaces are showing the source language",
            stable_refs: StableMessageIdentityRefs {
                diagnostic_id_ref: Some("i18n.fallback.active".to_owned()),
                telemetry_key_ref: Some("notification.locale_fallback.shown".to_owned()),
                ..StableMessageIdentityRefs::default()
            },
            placeholders: &[],
            machine_output_locale_class: LocaleNativeHumanOnly,
            escape_hatches: &[InlineSourceLanguageToggle],
            translated_in_locales: &["es-MX"],
            localized_human_prose_allowed: true,
            since_baseline: false,
        },
    ]
}

/// Returns the seeded requested-locale fallback profiles.
fn seeded_locale_profiles() -> Vec<LocaleProfileRow> {
    vec![
        LocaleProfileRow {
            requested_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            effective_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_chain: vec![SOURCE_LANGUAGE_LOCALE.to_owned()],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
            degraded_state: DegradedLocalizationState::FullyLocalized,
            source_language_route_active: false,
        },
        LocaleProfileRow {
            requested_locale: "es-MX".to_owned(),
            effective_locale: "es-MX".to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_chain: vec![
                "es-MX".to_owned(),
                "es".to_owned(),
                SOURCE_LANGUAGE_LOCALE.to_owned(),
            ],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
            degraded_state: DegradedLocalizationState::FullyLocalized,
            source_language_route_active: false,
        },
        LocaleProfileRow {
            requested_locale: "ja-JP".to_owned(),
            effective_locale: "ja-JP".to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_chain: vec![
                "ja-JP".to_owned(),
                "ja".to_owned(),
                SOURCE_LANGUAGE_LOCALE.to_owned(),
            ],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill,
            degraded_state: DegradedLocalizationState::PartialTranslationDisclosed,
            source_language_route_active: true,
        },
        LocaleProfileRow {
            requested_locale: "ar-SA".to_owned(),
            effective_locale: "ar-SA".to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_chain: vec![
                "ar-SA".to_owned(),
                "ar".to_owned(),
                SOURCE_LANGUAGE_LOCALE.to_owned(),
            ],
            fallback_origin: LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill,
            degraded_state: DegradedLocalizationState::PartialTranslationDisclosed,
            source_language_route_active: true,
        },
        LocaleProfileRow {
            requested_locale: "de-DE".to_owned(),
            effective_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            fallback_chain: vec![
                "de-DE".to_owned(),
                "de".to_owned(),
                SOURCE_LANGUAGE_LOCALE.to_owned(),
            ],
            fallback_origin: LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly,
            degraded_state: DegradedLocalizationState::FailedPackSourceLanguageOnly,
            source_language_route_active: true,
        },
    ]
}

/// Returns source contracts that govern the registry.
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
            "localized_profile_matrix".to_owned(),
            "fixtures/i18n/m5-surface-inventory/manifest.json".to_owned(),
        ),
        (
            "terminology_governance".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md#20.7".to_owned(),
        ),
    ])
}

/// Returns runtime consumers that ingest the registry.
fn seeded_runtime_consumer_refs() -> Vec<String> {
    vec![
        "crates/aureline-i18n".to_owned(),
        "crates/aureline-shell".to_owned(),
        "crates/aureline-settings".to_owned(),
        "crates/aureline-help".to_owned(),
        "crates/aureline-support".to_owned(),
    ]
}

/// Returns the seeded M5 message-id registry.
pub fn seeded_m5_message_registry() -> M5MessageRegistry {
    let entries: Vec<MessageRegistryEntry> = entry_specs()
        .iter()
        .map(|spec| build_entry(spec, SOURCE_LANGUAGE_LOCALE, TARGET_BUILD))
        .collect();
    let locale_profiles = seeded_locale_profiles();
    let summary = derive_summary(&entries, &locale_profiles, true, SOURCE_LANGUAGE_LOCALE);

    M5MessageRegistry {
        record_kind: M5_MESSAGE_REGISTRY_RECORD_KIND.to_owned(),
        schema_version: M5_MESSAGE_REGISTRY_SCHEMA_VERSION,
        packet_id: M5_MESSAGE_REGISTRY_PACKET_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        target_build_identity_ref: TARGET_BUILD.to_owned(),
        baseline_build_identity_ref: BASELINE_BUILD.to_owned(),
        source_contract_refs: seeded_source_contract_refs(),
        runtime_consumer_refs: seeded_runtime_consumer_refs(),
        entries,
        locale_profiles,
        summary,
    }
}

/// Returns the seeded prior-release baseline snapshot.
///
/// The snapshot contains exactly the ids that existed at [`BASELINE_BUILD`], so
/// the continuity check proves the current registry preserves them.
pub fn seeded_m5_message_id_baseline() -> MessageIdBaselineSnapshot {
    let mut ids: Vec<MessageIdBaselineRow> = entry_specs()
        .iter()
        .filter(|spec| spec.since_baseline)
        .map(|spec| MessageIdBaselineRow {
            message_id: spec.message_id.to_owned(),
            source_language_key: spec.source_language_key.to_owned(),
            surface: spec.surface,
        })
        .collect();
    ids.sort_by(|left, right| left.message_id.cmp(&right.message_id));

    MessageIdBaselineSnapshot {
        record_kind: M5_MESSAGE_ID_BASELINE_RECORD_KIND.to_owned(),
        schema_version: M5_MESSAGE_REGISTRY_SCHEMA_VERSION,
        snapshot_id: M5_MESSAGE_ID_BASELINE_SNAPSHOT_ID.to_owned(),
        build_identity_ref: BASELINE_BUILD.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        ids,
    }
}
