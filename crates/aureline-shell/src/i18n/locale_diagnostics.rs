//! Consolidated localization diagnostics, Help/About, and support-export packet.
//!
//! This module joins the canonical [`aureline_i18n`] locale-pack compatibility
//! report into one inspectable truth packet that Help/About, the diagnostics
//! surface, support exports, and release/shiproom tooling all read instead of
//! cloning localization status text or spelunking raw logs. It owns no
//! localization truth of its own: pack versions, compatibility and signature
//! state, fallback chains, missing-key counts, and degraded-localization reasons
//! are projected from the seeded compatibility report so every audience quotes
//! the same numbers.
//!
//! The packet exposes three audience-shaped views plus a release gate:
//!
//! - **Help/About** — [`LocaleDiagnosticsHelpAboutCard`] answers "what language
//!   am I in, what packs are installed, and is anything degraded?" with one
//!   honesty marker, without raw log access.
//! - **Diagnostics** — the packet's [`InstalledLocalePackRow`] and
//!   [`LocaleDiagnosticsProfileRow`] lists are the diagnostic fields a Problems
//!   or support view renders: active locale, installed pack versions,
//!   compatibility state, fallback chain, missing-key counts, and the
//!   degraded-localization reason.
//! - **Support export** — [`LocaleDiagnosticsSupportExport`] is metadata-only:
//!   it preserves stable pack ids, locale tags, and source-language route refs
//!   while omitting translated bodies, signing keys, and provider payloads. Each
//!   row carries a [`LocaleProblemOrigin`] so an escalation can tell whether a
//!   problem came from the requested locale, a base fallback, a source-language
//!   fallback, pack skew, or missing translations.
//! - **Release / shiproom** — [`LocaleDiagnosticsReleaseGate`] narrows or blocks
//!   a locale-bearing claim when diagnostics show an incompatible or degraded
//!   localization state.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use aureline_i18n::{
    seeded_locale_pack_compatibility_report, DegradedLocalizationState, LocaleFallbackOriginClass,
    LocalePackCompatibilityReport, LocalePackCompatibilityRow, LocalePackSignatureState,
    LocalePackSourceClass, PackApplicationDecision, SkewDegradeReason, VersionMatchState,
};

#[cfg(test)]
mod tests;

/// Schema version for the locale diagnostics packet and its projections.
pub const LOCALE_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`LocaleDiagnosticsPacket`].
pub const LOCALE_DIAGNOSTICS_RECORD_KIND: &str = "locale_diagnostics_packet";

/// Record kind for [`LocaleDiagnosticsHelpAboutCard`].
pub const LOCALE_DIAGNOSTICS_HELP_ABOUT_RECORD_KIND: &str = "locale_diagnostics_help_about_card";

/// Record kind for [`LocaleDiagnosticsSupportExport`].
pub const LOCALE_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND: &str = "locale_diagnostics_support_export";

/// Record kind for [`LocaleDiagnosticsReleaseGate`].
pub const LOCALE_DIAGNOSTICS_RELEASE_GATE_RECORD_KIND: &str = "locale_diagnostics_release_gate";

/// Stable packet id for the seeded locale diagnostics posture.
pub const LOCALE_DIAGNOSTICS_PACKET_ID: &str = "i18n:locale-diagnostics:m5-support-and-release:v1";

/// Fixture path for the seeded locale diagnostics packet.
pub const LOCALE_DIAGNOSTICS_FIXTURE_REF: &str =
    "fixtures/i18n/locale-diagnostics-exports/locale-diagnostics-packet.json";

/// Fixture path for the seeded locale diagnostics support export.
pub const LOCALE_DIAGNOSTICS_SUPPORT_EXPORT_FIXTURE_REF: &str =
    "fixtures/i18n/locale-diagnostics-exports/locale-diagnostics-support-export.json";

/// Schema path for the locale diagnostics packet.
pub const LOCALE_DIAGNOSTICS_SCHEMA_REF: &str = "schemas/i18n/locale-diagnostics.schema.json";

/// Requested locale the seeded session is rendering. `de-DE` ships an
/// incompatible pack, so the headline state demonstrates pack skew and a
/// disclosed source-language fallback.
const SEEDED_ACTIVE_LOCALE: &str = "de-DE";

/// Export-safe classification of where a localization result or problem
/// originates, shared by diagnostics rows, the support export, and the release
/// gate. This is the bucket a support engineer assigns a localization issue to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleProblemOrigin {
    /// The requested locale was authoritative; any issue is in that locale's pack.
    RequestedLocale,
    /// A base-language fill served some keys for the requested locale.
    BaseFallback,
    /// The surface fell back to the source language.
    SourceLanguageFallback,
    /// An incompatible pack version or build range forced a degrade.
    PackSkew,
    /// The pack applied but is missing some translated keys.
    MissingTranslations,
}

impl LocaleProblemOrigin {
    /// All problem-origin buckets in stable order.
    pub const ALL: [LocaleProblemOrigin; 5] = [
        LocaleProblemOrigin::RequestedLocale,
        LocaleProblemOrigin::BaseFallback,
        LocaleProblemOrigin::SourceLanguageFallback,
        LocaleProblemOrigin::PackSkew,
        LocaleProblemOrigin::MissingTranslations,
    ];

    /// Returns the stable snake_case token for this origin.
    pub const fn as_token(self) -> &'static str {
        match self {
            Self::RequestedLocale => "requested_locale",
            Self::BaseFallback => "base_fallback",
            Self::SourceLanguageFallback => "source_language_fallback",
            Self::PackSkew => "pack_skew",
            Self::MissingTranslations => "missing_translations",
        }
    }

    /// Returns a short export-safe human label for this origin.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RequestedLocale => "Requested locale served",
            Self::BaseFallback => "Base-language fallback",
            Self::SourceLanguageFallback => "Source-language fallback",
            Self::PackSkew => "Incompatible locale pack (version skew)",
            Self::MissingTranslations => "Missing translations",
        }
    }

    /// Returns true when this origin reflects a degraded localization state.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::RequestedLocale)
    }

    /// Classifies the origin from one evaluated compatibility row's state.
    ///
    /// Version/build incompatibility maps to [`Self::PackSkew`]; signature,
    /// integrity, missing-pack, or policy degrades that drop fully to the source
    /// language map to [`Self::SourceLanguageFallback`]; an applied pack with a
    /// base-language fill maps to [`Self::BaseFallback`]; an applied pack with
    /// disclosed gaps maps to [`Self::MissingTranslations`]; and a fully covered
    /// requested locale maps to [`Self::RequestedLocale`].
    fn classify(
        application_decision: PackApplicationDecision,
        skew_degrade_reason: SkewDegradeReason,
        fallback_origin: LocaleFallbackOriginClass,
        missing_key_count: usize,
    ) -> Self {
        match skew_degrade_reason {
            SkewDegradeReason::IncompatibleVersionDrift
            | SkewDegradeReason::BuildOutsideCompatibilityRange
            | SkewDegradeReason::UnknownTargetBuild => Self::PackSkew,
            SkewDegradeReason::SignatureFailed
            | SkewDegradeReason::SignatureUnverifiedNotAccepted
            | SkewDegradeReason::IntegrityDigestMismatch
            | SkewDegradeReason::PackMissing
            | SkewDegradeReason::PolicyDisabledLocale => Self::SourceLanguageFallback,
            SkewDegradeReason::NotDegraded => {
                if !application_decision.applies() {
                    Self::SourceLanguageFallback
                } else if matches!(fallback_origin, LocaleFallbackOriginClass::BaseLocaleFallback) {
                    Self::BaseFallback
                } else if missing_key_count == 0 {
                    Self::RequestedLocale
                } else {
                    Self::MissingTranslations
                }
            }
        }
    }
}

/// One installed locale-pack row as the diagnostics surface renders it.
///
/// Every field is export-safe: it carries the pack version and state a support
/// engineer or release reviewer needs, never the translated bodies or signing
/// keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstalledLocalePackRow {
    /// Stable pack id.
    pub pack_id: String,
    /// Installed pack version for support and release reporting.
    pub pack_version: String,
    /// Pack revision ref.
    pub pack_revision_ref: String,
    /// Locale the pack localizes.
    pub locale: String,
    /// Pack source class.
    pub source_class: LocalePackSourceClass,
    /// Observed signature state.
    pub signature_state: LocalePackSignatureState,
    /// Observed version-match state against the active build.
    pub version_match_state: VersionMatchState,
    /// Whether the active build is inside the pack's compatibility range.
    pub compatible_with_active_build: bool,
    /// Apply-or-degrade decision.
    pub application_decision: PackApplicationDecision,
    /// Reason a degrade occurred, when applicable.
    pub skew_degrade_reason: SkewDegradeReason,
    /// Total translatable keys.
    pub total_key_count: usize,
    /// Keys falling back to the source language after evaluation.
    pub missing_key_count: usize,
    /// Whether this pack backs a claimed localized profile.
    pub claimed_localized_profile: bool,
    /// Where this pack's result or problem originates.
    pub problem_origin: LocaleProblemOrigin,
    /// Same-surface source-language route preserved for the user and support.
    pub open_in_source_language_route_ref: String,
    /// Always true; translated body text never crosses this boundary.
    pub raw_translated_body_omitted: bool,
}

/// Per-requested-locale diagnostics profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleDiagnosticsProfileRow {
    /// User-requested locale.
    pub requested_locale: String,
    /// Locale that produces rendered text after evaluation.
    pub effective_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain.
    pub fallback_chain: Vec<String>,
    /// Why fallback did or did not occur.
    pub fallback_origin: LocaleFallbackOriginClass,
    /// Degraded localization state after evaluation.
    pub degraded_state: DegradedLocalizationState,
    /// Export-safe classification of the result or problem origin.
    pub problem_origin: LocaleProblemOrigin,
    /// Whether a visible source-language route is active for this locale.
    pub source_language_route_active: bool,
    /// Total translatable keys.
    pub total_key_count: usize,
    /// Keys falling back to the source language.
    pub missing_key_count: usize,
    /// Pack id backing this locale, when one is installed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backing_pack_id_ref: Option<String>,
}

/// Help/About localization card. Answers the language-and-localization question
/// a user opens Help/About to settle, without raw log access.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleDiagnosticsHelpAboutCard {
    /// Boundary record kind.
    pub record_kind: String,
    /// Stable section heading.
    pub heading: String,
    /// User-requested active locale.
    pub requested_locale: String,
    /// Effective locale for the active session.
    pub effective_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain for the active locale.
    pub fallback_chain: Vec<String>,
    /// Degraded localization state for the active locale.
    pub degraded_state: DegradedLocalizationState,
    /// Where the active locale's result or problem originates.
    pub problem_origin: LocaleProblemOrigin,
    /// Number of installed locale packs.
    pub installed_pack_count: usize,
    /// Installed packs that are incompatible with the active build.
    pub incompatible_pack_count: usize,
    /// Missing keys for the active locale.
    pub missing_key_count: usize,
    /// Whether a visible source-language route is active.
    pub source_language_route_active: bool,
    /// Same-surface source-language route for the active locale.
    pub open_in_source_language_route_ref: String,
    /// True when the active locale is degraded or any pack is incompatible; the
    /// chrome MUST surface a visible honesty marker when this is true.
    pub honesty_marker_present: bool,
    /// Always true; translated body text never crosses this boundary.
    pub raw_translated_body_omitted: bool,
}

/// One support-export row for an installed pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportPackRow {
    /// Stable pack id.
    pub pack_id: String,
    /// Installed pack version.
    pub pack_version: String,
    /// Locale the pack localizes.
    pub locale: String,
    /// Observed signature state.
    pub signature_state: LocalePackSignatureState,
    /// Observed version-match state.
    pub version_match_state: VersionMatchState,
    /// Keys falling back to the source language.
    pub missing_key_count: usize,
    /// Where this pack's result or problem originates.
    pub problem_origin: LocaleProblemOrigin,
    /// Always true; translated body text never crosses this boundary.
    pub raw_translated_body_omitted: bool,
}

/// One support-export row for a requested-locale profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportProfileRow {
    /// User-requested locale.
    pub requested_locale: String,
    /// Effective locale after evaluation.
    pub effective_locale: String,
    /// Ordered requested-to-base-to-source fallback chain.
    pub fallback_chain: Vec<String>,
    /// Why fallback did or did not occur.
    pub fallback_origin: LocaleFallbackOriginClass,
    /// Degraded localization state after evaluation.
    pub degraded_state: DegradedLocalizationState,
    /// Where this locale's result or problem originates.
    pub problem_origin: LocaleProblemOrigin,
    /// Keys falling back to the source language.
    pub missing_key_count: usize,
    /// Whether this locale fell back to the source language for some keys.
    pub used_source_language_fallback: bool,
    /// Always true; translated body text never crosses this boundary.
    pub raw_translated_body_omitted: bool,
}

/// Metadata-only, escalation-safe export of the locale diagnostics posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleDiagnosticsSupportExport {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Source packet id.
    pub source_packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Active build identity the export was evaluated against.
    pub target_build_identity_ref: String,
    /// User-requested active locale.
    pub requested_locale: String,
    /// Effective locale for the active session.
    pub effective_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain for the active locale.
    pub fallback_chain: Vec<String>,
    /// Why fallback did or did not occur for the active locale.
    pub fallback_origin: LocaleFallbackOriginClass,
    /// Degraded localization state for the active locale.
    pub degraded_state: DegradedLocalizationState,
    /// Where the active locale's result or problem originates.
    pub problem_origin: LocaleProblemOrigin,
    /// Missing keys for the active locale.
    pub missing_key_count: usize,
    /// Whether a visible source-language route is active for the active locale.
    pub source_language_route_active: bool,
    /// Installed-pack rows preserved for escalation.
    pub installed_pack_rows: Vec<SupportExportPackRow>,
    /// Per-locale profile rows preserved for escalation.
    pub profile_rows: Vec<SupportExportProfileRow>,
    /// Stable anchors (pack ids, locale tags, source-language routes) preserved
    /// for escalation.
    pub preserved_stable_anchor_refs: Vec<String>,
    /// Whether any raw translated body was exported. Must be false.
    pub raw_translated_bodies_exported: bool,
    /// Material classes deliberately omitted from the export.
    pub omitted_material_classes: Vec<String>,
}

/// Release/shiproom gate state for one locale-bearing claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocaleClaimGateState {
    /// The localized claim holds; the locale is fully covered.
    ClaimHoldsFullyLocalized,
    /// The claim is narrowed to a disclosed-partial localization.
    ClaimNarrowedPartial,
    /// The claim is narrowed to a source-language-only fallback.
    ClaimNarrowedSourceLanguage,
    /// The claim is blocked by an incompatible locale pack.
    ClaimBlockedIncompatiblePack,
}

impl LocaleClaimGateState {
    /// Returns the stable snake_case token for this gate state.
    pub const fn as_token(self) -> &'static str {
        match self {
            Self::ClaimHoldsFullyLocalized => "claim_holds_fully_localized",
            Self::ClaimNarrowedPartial => "claim_narrowed_partial",
            Self::ClaimNarrowedSourceLanguage => "claim_narrowed_source_language",
            Self::ClaimBlockedIncompatiblePack => "claim_blocked_incompatible_pack",
        }
    }

    /// Returns true when the localized claim was narrowed or blocked.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::ClaimHoldsFullyLocalized)
    }

    /// Returns true when a localized claim may still publish (with disclosure).
    pub const fn publishable(self) -> bool {
        matches!(
            self,
            Self::ClaimHoldsFullyLocalized | Self::ClaimNarrowedPartial
        )
    }

    /// Derives the gate state from a locale's problem origin.
    fn from_problem_origin(origin: LocaleProblemOrigin) -> Self {
        match origin {
            LocaleProblemOrigin::RequestedLocale => Self::ClaimHoldsFullyLocalized,
            LocaleProblemOrigin::BaseFallback | LocaleProblemOrigin::MissingTranslations => {
                Self::ClaimNarrowedPartial
            }
            LocaleProblemOrigin::SourceLanguageFallback => Self::ClaimNarrowedSourceLanguage,
            LocaleProblemOrigin::PackSkew => Self::ClaimBlockedIncompatiblePack,
        }
    }
}

/// One release/shiproom narrowing row for a locale-bearing claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleClaimNarrowRow {
    /// Locale whose localized claim is evaluated.
    pub claimed_locale: String,
    /// Pack id backing the claim, when one is installed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backing_pack_id_ref: Option<String>,
    /// Gate state for the claim.
    pub gate_state: LocaleClaimGateState,
    /// Where the claim's degradation originates.
    pub problem_origin: LocaleProblemOrigin,
    /// Whether the localized claim was narrowed or blocked.
    pub narrowed: bool,
    /// Whether the localized claim may still publish with disclosure.
    pub publishable_localized_claim: bool,
    /// Short export-safe reason describing the gate decision.
    pub narrow_reason: String,
}

/// Release/shiproom gate over every locale-bearing claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleDiagnosticsReleaseGate {
    /// Boundary record kind.
    pub record_kind: String,
    /// Active build identity the gate was evaluated against.
    pub target_build_identity_ref: String,
    /// Per-claim narrowing rows.
    pub rows: Vec<LocaleClaimNarrowRow>,
    /// Whether any locale-bearing claim was narrowed.
    pub any_claim_narrowed: bool,
    /// Whether any locale-bearing claim was blocked outright.
    pub any_claim_blocked: bool,
}

/// Summary posture derived from the packet rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleDiagnosticsSummary {
    /// Number of installed locale packs.
    pub installed_pack_count: usize,
    /// Packs that applied their translations.
    pub renderable_pack_count: usize,
    /// Packs incompatible with the active build.
    pub incompatible_pack_count: usize,
    /// Number of profiled requested locales.
    pub profiled_locale_count: usize,
    /// Locales served with full requested-locale coverage.
    pub fully_localized_locale_count: usize,
    /// Locales that degraded fully to the source language.
    pub source_language_fallback_locale_count: usize,
    /// Total missing keys across profiled locales.
    pub total_missing_key_count: usize,
    /// Profile count per problem-origin token.
    pub problem_origin_counts: BTreeMap<String, usize>,
    /// Product source-language locale.
    pub source_language_locale: String,
}

/// Consolidated localization diagnostics truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleDiagnosticsPacket {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Active build identity the packet was evaluated against.
    pub target_build_identity_ref: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// User-requested active locale.
    pub requested_locale: String,
    /// Effective locale for the active session.
    pub effective_locale: String,
    /// Ordered requested-to-base-to-source fallback chain for the active locale.
    pub fallback_chain: Vec<String>,
    /// Why fallback did or did not occur for the active locale.
    pub fallback_origin: LocaleFallbackOriginClass,
    /// Degraded localization state for the active locale.
    pub degraded_state: DegradedLocalizationState,
    /// Where the active locale's result or problem originates.
    pub problem_origin: LocaleProblemOrigin,
    /// Whether a visible source-language route is active for the active locale.
    pub source_language_route_active: bool,
    /// Total translatable keys for the active locale.
    pub total_message_count: usize,
    /// Missing keys for the active locale.
    pub missing_key_count: usize,
    /// Source contracts that govern this packet.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime consumers that ingest this packet.
    pub runtime_consumer_refs: Vec<String>,
    /// Installed locale packs with their versions and compatibility state.
    pub installed_packs: Vec<InstalledLocalePackRow>,
    /// Per-requested-locale diagnostics profiles.
    pub locale_profiles: Vec<LocaleDiagnosticsProfileRow>,
    /// Help/About localization card.
    pub help_about_card: LocaleDiagnosticsHelpAboutCard,
    /// Metadata-only support export of the locale diagnostics posture.
    pub support_export: LocaleDiagnosticsSupportExport,
    /// Release/shiproom claim-narrowing gate.
    pub release_gate: LocaleDiagnosticsReleaseGate,
    /// Summary posture derived from the rows.
    pub summary: LocaleDiagnosticsSummary,
}

/// Validation finding emitted by the locale diagnostics packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocaleDiagnosticsFinding {
    /// Row or record id that failed validation.
    pub row_ref: String,
    /// Validation message.
    pub message: String,
}

impl LocaleDiagnosticsFinding {
    /// Builds a finding for `row_ref` with `message`.
    pub fn new(row_ref: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            row_ref: row_ref.into(),
            message: message.into(),
        }
    }
}

impl LocaleDiagnosticsPacket {
    /// Returns the installed-pack row for a pack id, when present.
    pub fn installed_pack(&self, pack_id: &str) -> Option<&InstalledLocalePackRow> {
        self.installed_packs
            .iter()
            .find(|row| row.pack_id == pack_id)
    }

    /// Returns the diagnostics profile for a requested locale, when present.
    pub fn locale_profile(&self, requested_locale: &str) -> Option<&LocaleDiagnosticsProfileRow> {
        self.locale_profiles
            .iter()
            .find(|row| row.requested_locale == requested_locale)
    }

    /// Returns the active-locale diagnostics profile.
    pub fn active_profile(&self) -> Option<&LocaleDiagnosticsProfileRow> {
        self.locale_profile(&self.requested_locale)
    }

    /// Validates the packet shape, derived projections, and gate decisions.
    pub fn validate(&self) -> Result<(), Vec<LocaleDiagnosticsFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != LOCALE_DIAGNOSTICS_RECORD_KIND {
            findings.push(LocaleDiagnosticsFinding::new(
                self.packet_id.clone(),
                "packet record_kind is unsupported",
            ));
        }
        if self.schema_version != LOCALE_DIAGNOSTICS_SCHEMA_VERSION {
            findings.push(LocaleDiagnosticsFinding::new(
                self.packet_id.clone(),
                "packet schema_version is unsupported",
            ));
        }
        if self.installed_packs.is_empty() {
            findings.push(LocaleDiagnosticsFinding::new(
                self.packet_id.clone(),
                "packet has no installed packs",
            ));
        }
        if self.locale_profiles.is_empty() {
            findings.push(LocaleDiagnosticsFinding::new(
                self.packet_id.clone(),
                "packet has no locale profiles",
            ));
        }
        if self.active_profile().is_none() {
            findings.push(LocaleDiagnosticsFinding::new(
                self.packet_id.clone(),
                "active requested locale has no diagnostics profile",
            ));
        }

        validate_profiles(self, &mut findings);
        validate_help_about(self, &mut findings);
        validate_support_export(self, &mut findings);
        validate_release_gate(self, &mut findings);
        validate_summary(self, &mut findings);

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

fn validate_profiles(
    packet: &LocaleDiagnosticsPacket,
    findings: &mut Vec<LocaleDiagnosticsFinding>,
) {
    for profile in &packet.locale_profiles {
        if profile.fallback_chain.first() != Some(&profile.requested_locale) {
            findings.push(LocaleDiagnosticsFinding::new(
                profile.requested_locale.clone(),
                "fallback chain must start at the requested locale",
            ));
        }
        if profile.fallback_chain.last() != Some(&packet.source_language_locale) {
            findings.push(LocaleDiagnosticsFinding::new(
                profile.requested_locale.clone(),
                "fallback chain must end at the source language",
            ));
        }
        if profile.missing_key_count > profile.total_key_count {
            findings.push(LocaleDiagnosticsFinding::new(
                profile.requested_locale.clone(),
                "missing-key count exceeds total keys",
            ));
        }
        let route_expected = profile.problem_origin.is_degraded();
        if profile.source_language_route_active != route_expected {
            findings.push(LocaleDiagnosticsFinding::new(
                profile.requested_locale.clone(),
                "source-language route flag disagrees with problem origin",
            ));
        }
    }
}

fn validate_help_about(
    packet: &LocaleDiagnosticsPacket,
    findings: &mut Vec<LocaleDiagnosticsFinding>,
) {
    let expected = derive_help_about_card(packet);
    if packet.help_about_card != expected {
        findings.push(LocaleDiagnosticsFinding::new(
            packet.help_about_card.record_kind.clone(),
            "help/about card does not match the derived projection",
        ));
    }
    if packet.help_about_card.problem_origin.is_degraded()
        && !packet.help_about_card.honesty_marker_present
    {
        findings.push(LocaleDiagnosticsFinding::new(
            packet.help_about_card.record_kind.clone(),
            "degraded active locale must light the honesty marker",
        ));
    }
}

fn validate_support_export(
    packet: &LocaleDiagnosticsPacket,
    findings: &mut Vec<LocaleDiagnosticsFinding>,
) {
    let export = &packet.support_export;
    let expected = derive_support_export(packet);
    if *export != expected {
        findings.push(LocaleDiagnosticsFinding::new(
            export.export_id.clone(),
            "support export does not match the derived projection",
        ));
    }
    if export.raw_translated_bodies_exported {
        findings.push(LocaleDiagnosticsFinding::new(
            export.export_id.clone(),
            "support export must omit raw translated bodies",
        ));
    }
    if !export
        .installed_pack_rows
        .iter()
        .all(|row| row.raw_translated_body_omitted)
        || !export
            .profile_rows
            .iter()
            .all(|row| row.raw_translated_body_omitted)
    {
        findings.push(LocaleDiagnosticsFinding::new(
            export.export_id.clone(),
            "support export row retains a raw translated body",
        ));
    }
    if export.preserved_stable_anchor_refs.is_empty() {
        findings.push(LocaleDiagnosticsFinding::new(
            export.export_id.clone(),
            "support export preserves no stable anchors for escalation",
        ));
    }
}

fn validate_release_gate(
    packet: &LocaleDiagnosticsPacket,
    findings: &mut Vec<LocaleDiagnosticsFinding>,
) {
    let expected = derive_release_gate(packet);
    if packet.release_gate != expected {
        findings.push(LocaleDiagnosticsFinding::new(
            packet.release_gate.record_kind.clone(),
            "release gate does not match the derived projection",
        ));
    }
    for row in &packet.release_gate.rows {
        if row.problem_origin.is_degraded() && !row.narrowed {
            findings.push(LocaleDiagnosticsFinding::new(
                row.claimed_locale.clone(),
                "degraded locale claim was not narrowed",
            ));
        }
        if matches!(
            row.problem_origin,
            LocaleProblemOrigin::PackSkew | LocaleProblemOrigin::SourceLanguageFallback
        ) && row.publishable_localized_claim
        {
            findings.push(LocaleDiagnosticsFinding::new(
                row.claimed_locale.clone(),
                "incompatible or source-language locale claim must not stay publishable",
            ));
        }
    }
}

fn validate_summary(
    packet: &LocaleDiagnosticsPacket,
    findings: &mut Vec<LocaleDiagnosticsFinding>,
) {
    let expected = derive_summary(packet);
    if packet.summary != expected {
        findings.push(LocaleDiagnosticsFinding::new(
            packet.packet_id.clone(),
            "summary does not match the derived rows",
        ));
    }
}

/// Builds one installed-pack row from an evaluated compatibility row.
fn installed_row_from(row: &LocalePackCompatibilityRow) -> InstalledLocalePackRow {
    let problem_origin = LocaleProblemOrigin::classify(
        row.application_decision,
        row.skew_degrade_reason,
        row.fallback_origin_class,
        row.missing_key_count,
    );
    InstalledLocalePackRow {
        pack_id: row.pack_id.clone(),
        pack_version: row.pack_version.clone(),
        pack_revision_ref: row.pack_revision_ref.clone(),
        locale: row.requested_locale.clone(),
        source_class: row.source_class,
        signature_state: row.signature_state,
        version_match_state: row.version_match_state,
        compatible_with_active_build: row.target_build_in_compatibility_range,
        application_decision: row.application_decision,
        skew_degrade_reason: row.skew_degrade_reason,
        total_key_count: row.total_key_count,
        missing_key_count: row.missing_key_count,
        claimed_localized_profile: row.claimed_localized_profile,
        problem_origin,
        open_in_source_language_route_ref: row.open_in_source_language_route_ref.clone(),
        raw_translated_body_omitted: true,
    }
}

/// Builds one diagnostics profile from an evaluated compatibility row.
fn profile_row_from(row: &LocalePackCompatibilityRow) -> LocaleDiagnosticsProfileRow {
    let problem_origin = LocaleProblemOrigin::classify(
        row.application_decision,
        row.skew_degrade_reason,
        row.fallback_origin_class,
        row.missing_key_count,
    );
    LocaleDiagnosticsProfileRow {
        requested_locale: row.requested_locale.clone(),
        effective_locale: row.effective_locale.clone(),
        source_language_locale: row.source_language_locale.clone(),
        fallback_chain: row.fallback_chain.clone(),
        fallback_origin: row.fallback_origin_class,
        degraded_state: row.degraded_localization_state,
        problem_origin,
        source_language_route_active: problem_origin.is_degraded(),
        total_key_count: row.total_key_count,
        missing_key_count: row.missing_key_count,
        backing_pack_id_ref: Some(row.pack_id.clone()),
    }
}

/// Derives the Help/About card from the packet's active locale and packs.
fn derive_help_about_card(packet: &LocaleDiagnosticsPacket) -> LocaleDiagnosticsHelpAboutCard {
    let incompatible_pack_count = packet
        .installed_packs
        .iter()
        .filter(|pack| !pack.version_match_state.may_render())
        .count();
    let active_route = packet
        .active_profile()
        .and_then(|profile| profile.backing_pack_id_ref.clone())
        .and_then(|pack_id| packet.installed_pack(&pack_id))
        .map(|pack| pack.open_in_source_language_route_ref.clone())
        .unwrap_or_default();
    let honesty_marker_present =
        packet.problem_origin.is_degraded() || incompatible_pack_count > 0;
    LocaleDiagnosticsHelpAboutCard {
        record_kind: LOCALE_DIAGNOSTICS_HELP_ABOUT_RECORD_KIND.to_owned(),
        heading: "Language and localization".to_owned(),
        requested_locale: packet.requested_locale.clone(),
        effective_locale: packet.effective_locale.clone(),
        source_language_locale: packet.source_language_locale.clone(),
        fallback_chain: packet.fallback_chain.clone(),
        degraded_state: packet.degraded_state,
        problem_origin: packet.problem_origin,
        installed_pack_count: packet.installed_packs.len(),
        incompatible_pack_count,
        missing_key_count: packet.missing_key_count,
        source_language_route_active: packet.source_language_route_active,
        open_in_source_language_route_ref: active_route,
        honesty_marker_present,
        raw_translated_body_omitted: true,
    }
}

/// Material classes the metadata-only export deliberately omits.
fn omitted_material_classes() -> Vec<String> {
    vec![
        "raw_translated_message_bodies".to_owned(),
        "locale_pack_signing_keys".to_owned(),
        "raw_provider_payloads".to_owned(),
        "raw_diagnostic_logs".to_owned(),
    ]
}

/// Derives the metadata-only support export from the packet rows.
fn derive_support_export(packet: &LocaleDiagnosticsPacket) -> LocaleDiagnosticsSupportExport {
    let installed_pack_rows: Vec<SupportExportPackRow> = packet
        .installed_packs
        .iter()
        .map(|pack| SupportExportPackRow {
            pack_id: pack.pack_id.clone(),
            pack_version: pack.pack_version.clone(),
            locale: pack.locale.clone(),
            signature_state: pack.signature_state,
            version_match_state: pack.version_match_state,
            missing_key_count: pack.missing_key_count,
            problem_origin: pack.problem_origin,
            raw_translated_body_omitted: true,
        })
        .collect();
    let profile_rows: Vec<SupportExportProfileRow> = packet
        .locale_profiles
        .iter()
        .map(|profile| SupportExportProfileRow {
            requested_locale: profile.requested_locale.clone(),
            effective_locale: profile.effective_locale.clone(),
            fallback_chain: profile.fallback_chain.clone(),
            fallback_origin: profile.fallback_origin,
            degraded_state: profile.degraded_state,
            problem_origin: profile.problem_origin,
            missing_key_count: profile.missing_key_count,
            used_source_language_fallback: profile.missing_key_count > 0,
            raw_translated_body_omitted: true,
        })
        .collect();

    // Stable anchors a support engineer pastes back: pack ids, locale tags, and
    // the source-language routes — never translated bodies.
    let mut anchors: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for pack in &packet.installed_packs {
        anchors.insert(pack.pack_id.clone());
        anchors.insert(pack.locale.clone());
        anchors.insert(pack.open_in_source_language_route_ref.clone());
    }
    anchors.insert(packet.source_language_locale.clone());
    let preserved_stable_anchor_refs: Vec<String> = anchors.into_iter().collect();

    LocaleDiagnosticsSupportExport {
        record_kind: LOCALE_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: LOCALE_DIAGNOSTICS_SCHEMA_VERSION,
        export_id: format!("{LOCALE_DIAGNOSTICS_PACKET_ID}:support-export"),
        source_packet_id: packet.packet_id.clone(),
        generated_at: packet.generated_at.clone(),
        target_build_identity_ref: packet.target_build_identity_ref.clone(),
        requested_locale: packet.requested_locale.clone(),
        effective_locale: packet.effective_locale.clone(),
        source_language_locale: packet.source_language_locale.clone(),
        fallback_chain: packet.fallback_chain.clone(),
        fallback_origin: packet.fallback_origin,
        degraded_state: packet.degraded_state,
        problem_origin: packet.problem_origin,
        missing_key_count: packet.missing_key_count,
        source_language_route_active: packet.source_language_route_active,
        installed_pack_rows,
        profile_rows,
        preserved_stable_anchor_refs,
        raw_translated_bodies_exported: false,
        omitted_material_classes: omitted_material_classes(),
    }
}

/// Derives the release/shiproom gate from each non-source locale profile.
fn derive_release_gate(packet: &LocaleDiagnosticsPacket) -> LocaleDiagnosticsReleaseGate {
    let rows: Vec<LocaleClaimNarrowRow> = packet
        .locale_profiles
        .iter()
        .filter(|profile| profile.requested_locale != packet.source_language_locale)
        .map(|profile| {
            let gate_state = LocaleClaimGateState::from_problem_origin(profile.problem_origin);
            LocaleClaimNarrowRow {
                claimed_locale: profile.requested_locale.clone(),
                backing_pack_id_ref: profile.backing_pack_id_ref.clone(),
                gate_state,
                problem_origin: profile.problem_origin,
                narrowed: gate_state.is_narrowed(),
                publishable_localized_claim: gate_state.publishable(),
                narrow_reason: narrow_reason(gate_state, profile.problem_origin),
            }
        })
        .collect();
    let any_claim_narrowed = rows.iter().any(|row| row.narrowed);
    let any_claim_blocked = rows
        .iter()
        .any(|row| row.gate_state == LocaleClaimGateState::ClaimBlockedIncompatiblePack);
    LocaleDiagnosticsReleaseGate {
        record_kind: LOCALE_DIAGNOSTICS_RELEASE_GATE_RECORD_KIND.to_owned(),
        target_build_identity_ref: packet.target_build_identity_ref.clone(),
        rows,
        any_claim_narrowed,
        any_claim_blocked,
    }
}

/// Returns an export-safe reason string for a gate decision.
fn narrow_reason(state: LocaleClaimGateState, origin: LocaleProblemOrigin) -> String {
    match state {
        LocaleClaimGateState::ClaimHoldsFullyLocalized => {
            "Requested locale fully covered; localized claim holds.".to_owned()
        }
        LocaleClaimGateState::ClaimNarrowedPartial => format!(
            "Localized claim narrowed to disclosed-partial: {}.",
            origin.label()
        ),
        LocaleClaimGateState::ClaimNarrowedSourceLanguage => {
            "Localized claim narrowed to source-language fallback.".to_owned()
        }
        LocaleClaimGateState::ClaimBlockedIncompatiblePack => {
            "Localized claim blocked by an incompatible locale pack.".to_owned()
        }
    }
}

/// Derives the summary posture from the packet rows.
fn derive_summary(packet: &LocaleDiagnosticsPacket) -> LocaleDiagnosticsSummary {
    let renderable_pack_count = packet
        .installed_packs
        .iter()
        .filter(|pack| pack.application_decision.applies())
        .count();
    let incompatible_pack_count = packet
        .installed_packs
        .iter()
        .filter(|pack| !pack.version_match_state.may_render())
        .count();
    let fully_localized_locale_count = packet
        .locale_profiles
        .iter()
        .filter(|profile| profile.problem_origin == LocaleProblemOrigin::RequestedLocale)
        .count();
    let source_language_fallback_locale_count = packet
        .locale_profiles
        .iter()
        .filter(|profile| {
            profile.problem_origin == LocaleProblemOrigin::SourceLanguageFallback
                || profile.problem_origin == LocaleProblemOrigin::PackSkew
        })
        .count();
    let total_missing_key_count = packet
        .locale_profiles
        .iter()
        .map(|profile| profile.missing_key_count)
        .sum();
    let mut problem_origin_counts: BTreeMap<String, usize> = BTreeMap::new();
    for origin in LocaleProblemOrigin::ALL {
        problem_origin_counts.insert(origin.as_token().to_owned(), 0);
    }
    for profile in &packet.locale_profiles {
        *problem_origin_counts
            .entry(profile.problem_origin.as_token().to_owned())
            .or_insert(0) += 1;
    }
    LocaleDiagnosticsSummary {
        installed_pack_count: packet.installed_packs.len(),
        renderable_pack_count,
        incompatible_pack_count,
        profiled_locale_count: packet.locale_profiles.len(),
        fully_localized_locale_count,
        source_language_fallback_locale_count,
        total_missing_key_count,
        problem_origin_counts,
        source_language_locale: packet.source_language_locale.clone(),
    }
}

/// Builds the consolidated packet from an evaluated compatibility report.
fn build_packet(report: &LocalePackCompatibilityReport, active_locale: &str) -> LocaleDiagnosticsPacket {
    let installed_packs: Vec<InstalledLocalePackRow> =
        report.rows.iter().map(installed_row_from).collect();
    let locale_profiles: Vec<LocaleDiagnosticsProfileRow> =
        report.rows.iter().map(profile_row_from).collect();

    let active = locale_profiles
        .iter()
        .find(|profile| profile.requested_locale == active_locale)
        .cloned()
        .unwrap_or_else(|| {
            profile_row_from(
                report
                    .rows
                    .first()
                    .expect("seeded compatibility report has at least one row"),
            )
        });

    let mut packet = LocaleDiagnosticsPacket {
        record_kind: LOCALE_DIAGNOSTICS_RECORD_KIND.to_owned(),
        schema_version: LOCALE_DIAGNOSTICS_SCHEMA_VERSION,
        packet_id: LOCALE_DIAGNOSTICS_PACKET_ID.to_owned(),
        generated_at: report.generated_at.clone(),
        target_build_identity_ref: report.target_build_identity_ref.clone(),
        source_language_locale: report.source_language_locale.clone(),
        requested_locale: active.requested_locale.clone(),
        effective_locale: active.effective_locale.clone(),
        fallback_chain: active.fallback_chain.clone(),
        fallback_origin: active.fallback_origin,
        degraded_state: active.degraded_state,
        problem_origin: active.problem_origin,
        source_language_route_active: active.source_language_route_active,
        total_message_count: active.total_key_count,
        missing_key_count: active.missing_key_count,
        source_contract_refs: seeded_source_contract_refs(),
        runtime_consumer_refs: seeded_runtime_consumer_refs(),
        installed_packs,
        locale_profiles,
        // Replaced below once the packet can derive its projections.
        help_about_card: placeholder_help_about_card(),
        support_export: placeholder_support_export(),
        release_gate: placeholder_release_gate(),
        summary: placeholder_summary(),
    };
    packet.help_about_card = derive_help_about_card(&packet);
    packet.support_export = derive_support_export(&packet);
    packet.release_gate = derive_release_gate(&packet);
    packet.summary = derive_summary(&packet);
    packet
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
            "compatibility_report".to_owned(),
            "schemas/i18n/locale-pack-compatibility-report.schema.json".to_owned(),
        ),
        (
            "diagnostics_schema".to_owned(),
            LOCALE_DIAGNOSTICS_SCHEMA_REF.to_owned(),
        ),
    ])
}

/// Returns runtime consumers that ingest the packet.
fn seeded_runtime_consumer_refs() -> Vec<String> {
    vec![
        "crates/aureline-shell".to_owned(),
        "crates/aureline-support".to_owned(),
        "crates/aureline-release".to_owned(),
    ]
}

/// Returns the seeded locale diagnostics packet, composed from the canonical
/// locale-pack compatibility report so its numbers never drift from the report.
pub fn seeded_locale_diagnostics_packet() -> LocaleDiagnosticsPacket {
    let report = seeded_locale_pack_compatibility_report();
    build_packet(&report, SEEDED_ACTIVE_LOCALE)
}

/// Returns the seeded locale diagnostics support export.
pub fn seeded_locale_diagnostics_support_export() -> LocaleDiagnosticsSupportExport {
    seeded_locale_diagnostics_packet().support_export
}

fn placeholder_help_about_card() -> LocaleDiagnosticsHelpAboutCard {
    LocaleDiagnosticsHelpAboutCard {
        record_kind: LOCALE_DIAGNOSTICS_HELP_ABOUT_RECORD_KIND.to_owned(),
        heading: String::new(),
        requested_locale: String::new(),
        effective_locale: String::new(),
        source_language_locale: String::new(),
        fallback_chain: Vec::new(),
        degraded_state: DegradedLocalizationState::FullyLocalized,
        problem_origin: LocaleProblemOrigin::RequestedLocale,
        installed_pack_count: 0,
        incompatible_pack_count: 0,
        missing_key_count: 0,
        source_language_route_active: false,
        open_in_source_language_route_ref: String::new(),
        honesty_marker_present: false,
        raw_translated_body_omitted: true,
    }
}

fn placeholder_support_export() -> LocaleDiagnosticsSupportExport {
    LocaleDiagnosticsSupportExport {
        record_kind: LOCALE_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: LOCALE_DIAGNOSTICS_SCHEMA_VERSION,
        export_id: String::new(),
        source_packet_id: String::new(),
        generated_at: String::new(),
        target_build_identity_ref: String::new(),
        requested_locale: String::new(),
        effective_locale: String::new(),
        source_language_locale: String::new(),
        fallback_chain: Vec::new(),
        fallback_origin: LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
        degraded_state: DegradedLocalizationState::FullyLocalized,
        problem_origin: LocaleProblemOrigin::RequestedLocale,
        missing_key_count: 0,
        source_language_route_active: false,
        installed_pack_rows: Vec::new(),
        profile_rows: Vec::new(),
        preserved_stable_anchor_refs: Vec::new(),
        raw_translated_bodies_exported: false,
        omitted_material_classes: omitted_material_classes(),
    }
}

fn placeholder_release_gate() -> LocaleDiagnosticsReleaseGate {
    LocaleDiagnosticsReleaseGate {
        record_kind: LOCALE_DIAGNOSTICS_RELEASE_GATE_RECORD_KIND.to_owned(),
        target_build_identity_ref: String::new(),
        rows: Vec::new(),
        any_claim_narrowed: false,
        any_claim_blocked: false,
    }
}

fn placeholder_summary() -> LocaleDiagnosticsSummary {
    LocaleDiagnosticsSummary {
        installed_pack_count: 0,
        renderable_pack_count: 0,
        incompatible_pack_count: 0,
        profiled_locale_count: 0,
        fully_localized_locale_count: 0,
        source_language_fallback_locale_count: 0,
        total_missing_key_count: 0,
        problem_origin_counts: BTreeMap::new(),
        source_language_locale: String::new(),
    }
}
