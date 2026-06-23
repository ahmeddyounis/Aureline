//! Versioned, signed, mirrorable locale-pack delivery and skew handling.
//!
//! This module owns the runtime contract for *shipping* a locale pack: a
//! versioned [`LocalePackArtifact`] that carries a compatibility build range,
//! signer identity, mirrorability metadata, and an integrity digest, plus the
//! compatibility evaluation that turns one of those artifacts plus the observed
//! environment into an explicit decision.
//!
//! The central invariant is **skew handling without ambiguity**: an unsigned,
//! tampered, or version-incompatible pack does not partially apply stale
//! translations. It degrades *fully* to source-language behavior, with a
//! recorded reason, so a half-localized shell or help surface can never sit in
//! an undefined state. A pack that *is* renderable but only partially translated
//! still applies, with its per-surface missing-key count disclosed.
//!
//! [`LocalePackCompatibilityReport`] is the inspectable packet that diagnostics,
//! support export, and release tooling ingest. For every evaluated pack it
//! records the exact pack version, the compatibility and signature state, the
//! missing-key count (overall and per surface), the active fallback, and — for
//! degraded packs — the degraded-localization reason. The same packet captures
//! the install, upgrade, mirror, and downgrade operations that produced each
//! row, so support can report what was applied and why.
//!
//! Raw translated bodies, signing keys, and credentials never cross this
//! boundary: artifacts carry digests and refs, not payloads.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    CompatibilityBuildRange, DegradedLocalizationState, LocaleFallbackOriginClass,
    LocalePackDistributionClass, LocalePackMirrorabilityClass, LocalePackOperationClass,
    LocalePackSignatureState, LocalePackSourceClass, LocalePackValidationFinding,
    MessageSurfaceFamily, VersionMatchState, GENERATED_AT, SOURCE_LANGUAGE_LOCALE, TARGET_BUILD,
};

/// Schema version for the locale-pack delivery and compatibility records.
pub const LOCALE_PACK_DELIVERY_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`LocalePackArtifact`].
pub const LOCALE_PACK_ARTIFACT_RECORD_KIND: &str = "locale_pack_artifact_record";

/// Record kind for [`LocalePackCompatibilityReport`].
pub const LOCALE_PACK_COMPATIBILITY_REPORT_RECORD_KIND: &str = "locale_pack_compatibility_report";

/// Stable id for the seeded locale-pack compatibility report.
pub const LOCALE_PACK_COMPATIBILITY_REPORT_ID: &str = "i18n:m5-locale-pack-compatibility:v1";

/// Fixture path for the seeded compatibility report.
pub const LOCALE_PACK_COMPATIBILITY_REPORT_FIXTURE_REF: &str =
    "fixtures/i18n/pack-skew-and-signature/compatibility_report.json";

/// Directory holding the checked-in first-party core locale-pack artifacts.
pub const LOCALE_PACK_CORE_ARTIFACT_ROOT: &str = "locale-packs/core";

/// Same-surface route that always reaches source-language truth under fallback.
const SOURCE_LANGUAGE_ROUTE: &str = "route:i18n:source-language:open";

/// Whether a pack applies its translations or degrades to source language only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackApplicationDecision {
    /// Pack is renderable; translations apply and any missing keys are disclosed.
    ApplyLocalizedWithDisclosedMissingKeys,
    /// Pack is unsupported, unsigned, tampered, or skewed; it degrades fully to
    /// source language rather than partially applying stale translations.
    DegradeToSourceLanguageOnly,
}

impl PackApplicationDecision {
    /// Returns true when the pack applies its translations.
    pub const fn applies(self) -> bool {
        matches!(self, Self::ApplyLocalizedWithDisclosedMissingKeys)
    }
}

/// Why a pack degraded to source-language behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkewDegradeReason {
    /// Pack applied; no degradation occurred.
    NotDegraded,
    /// Pack was not present for the requested locale.
    PackMissing,
    /// Pack signature failed verification and was blocked.
    SignatureFailed,
    /// Pack signature is unverified and was not explicitly accepted.
    SignatureUnverifiedNotAccepted,
    /// Pack content digest did not match its signed integrity digest.
    IntegrityDigestMismatch,
    /// Active build is outside the pack's compatibility build range.
    BuildOutsideCompatibilityRange,
    /// Pack version drift against the active build is incompatible.
    IncompatibleVersionDrift,
    /// Active build could not be determined, so compatibility is unverified.
    UnknownTargetBuild,
    /// Policy disabled the locale, forcing source-language behavior.
    PolicyDisabledLocale,
}

impl SkewDegradeReason {
    /// Returns true when this reason represents an applied, non-degraded pack.
    pub const fn is_applied(self) -> bool {
        matches!(self, Self::NotDegraded)
    }
}

/// Translation coverage for one surface family inside a pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceKeyCoverage {
    /// Surface family this coverage row describes.
    pub surface_family: MessageSurfaceFamily,
    /// Total translatable keys the product defines for this surface.
    pub total_key_count: usize,
    /// Keys this pack carries a translation for.
    pub translated_key_count: usize,
}

impl SurfaceKeyCoverage {
    /// Returns keys that would fall back to source language for this surface.
    pub const fn missing_key_count(&self) -> usize {
        self.total_key_count
            .saturating_sub(self.translated_key_count)
    }
}

/// Versioned, signed, mirrorable locale-pack delivery artifact.
///
/// This is the static, shippable record. Whether the pack *renders* in a given
/// environment is decided by [`LocalePackArtifact::evaluate`] against a
/// [`PackEvaluationInput`]; the artifact itself only declares identity,
/// provenance, compatibility bounds, and integrity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackArtifact {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable pack id.
    pub pack_id: String,
    /// Human-facing artifact version (for support and release reporting).
    pub pack_version: String,
    /// Stable pack revision ref for immutable joins.
    pub pack_revision_ref: String,
    /// Primary locale this pack translates.
    pub locale: String,
    /// Locales this pack can satisfy directly or as a base-locale proxy.
    pub coverage_locales: Vec<String>,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain.
    pub base_locale_fallback_chain: Vec<String>,
    /// Pack source class for governance and support export.
    pub source_class: LocalePackSourceClass,
    /// Distribution class for installation and mirror policy.
    pub distribution_class: LocalePackDistributionClass,
    /// Signer identity ref, when the pack is signed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_identity_ref: Option<String>,
    /// Detached signature artifact ref, when the pack is signed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_artifact_ref: Option<String>,
    /// Content integrity digest over the signed pack blob (lowercase hex sha256).
    pub integrity_digest_sha256: String,
    /// Mirrorability posture.
    pub mirrorability_class: LocalePackMirrorabilityClass,
    /// Mirror receipts that prove the artifact can be mirrored.
    pub mirror_receipt_refs: Vec<String>,
    /// Offline import bundle ref, when air-gapped import is supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_import_ref: Option<String>,
    /// Inclusive build range this pack revision is compatible with.
    pub compatibility_build_range: CompatibilityBuildRange,
    /// Rollback target for downgrades.
    pub rollback_ref: String,
    /// Per-surface translation coverage.
    pub surface_coverage: Vec<SurfaceKeyCoverage>,
    /// Short label rendered on inspection surfaces.
    pub presentation_label: String,
    /// Deterministic mint timestamp.
    pub minted_at: String,
}

impl LocalePackArtifact {
    /// Returns the total translatable key count across all surfaces.
    pub fn total_key_count(&self) -> usize {
        self.surface_coverage
            .iter()
            .map(|coverage| coverage.total_key_count)
            .sum()
    }

    /// Returns the translated key count across all surfaces.
    pub fn translated_key_count(&self) -> usize {
        self.surface_coverage
            .iter()
            .map(|coverage| coverage.translated_key_count)
            .sum()
    }

    /// Returns the declared missing-key count across all surfaces.
    pub fn declared_missing_key_count(&self) -> usize {
        self.total_key_count()
            .saturating_sub(self.translated_key_count())
    }

    /// Evaluates this artifact against observed environment state.
    ///
    /// The decision is binary: a renderable pack applies (disclosing any missing
    /// keys per surface); an unsupported, unsigned, tampered, or skewed pack
    /// degrades fully to source language so no stale partial translation ships.
    pub fn evaluate(
        &self,
        requested_locale: &str,
        input: &PackEvaluationInput,
    ) -> PackApplicationOutcome {
        let (decision, reason) = decide_application(input);
        let total_by_surface = self.surface_total_key_map();

        if decision.applies() {
            let missing_by_surface = self.surface_missing_key_map();
            let missing = self.declared_missing_key_count();
            let (origin, degraded) = if missing == 0 {
                (
                    LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
                    DegradedLocalizationState::FullyLocalized,
                )
            } else {
                (
                    LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill,
                    DegradedLocalizationState::PartialTranslationDisclosed,
                )
            };
            PackApplicationOutcome {
                application_decision: decision,
                skew_degrade_reason: reason,
                effective_locale: requested_locale.to_owned(),
                fallback_origin_class: origin,
                degraded_localization_state: degraded,
                total_key_count: self.total_key_count(),
                translated_key_count: self.translated_key_count(),
                missing_key_count: missing,
                missing_key_count_by_surface: missing_by_surface,
            }
        } else {
            // Full degrade: every key falls back to source language; the pack's
            // declared translations are not applied at all.
            PackApplicationOutcome {
                application_decision: decision,
                skew_degrade_reason: reason,
                effective_locale: self.source_language_locale.clone(),
                fallback_origin_class: degrade_origin(reason),
                degraded_localization_state:
                    DegradedLocalizationState::FailedPackSourceLanguageOnly,
                total_key_count: self.total_key_count(),
                translated_key_count: self.translated_key_count(),
                missing_key_count: self.total_key_count(),
                missing_key_count_by_surface: total_by_surface,
            }
        }
    }

    /// Validates the static delivery shape of this artifact.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();
        validate_artifact(self, &mut findings);
        finish(findings)
    }

    fn surface_total_key_map(&self) -> BTreeMap<String, usize> {
        self.surface_coverage
            .iter()
            .map(|coverage| {
                (
                    surface_family_key(coverage.surface_family).to_owned(),
                    coverage.total_key_count,
                )
            })
            .collect()
    }

    fn surface_missing_key_map(&self) -> BTreeMap<String, usize> {
        self.surface_coverage
            .iter()
            .map(|coverage| {
                (
                    surface_family_key(coverage.surface_family).to_owned(),
                    coverage.missing_key_count(),
                )
            })
            .collect()
    }
}

/// Observed environment state used to evaluate one pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackEvaluationInput {
    /// Active build identity the pack is evaluated against.
    pub target_build_identity_ref: String,
    /// Whether the active build falls inside the pack's compatibility range.
    pub target_build_in_compatibility_range: bool,
    /// Observed signature verification state.
    pub signature_state: LocalePackSignatureState,
    /// Observed version-match state against the active build.
    pub version_match_state: VersionMatchState,
    /// Whether the recomputed content digest matched the signed digest.
    pub integrity_digest_matches: bool,
    /// Whether the pack is present for the requested locale.
    pub pack_present: bool,
    /// Whether policy permits localizing this locale.
    pub policy_locale_enabled: bool,
}

/// Computed outcome of evaluating one pack against the environment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackApplicationOutcome {
    /// Apply-or-degrade decision.
    pub application_decision: PackApplicationDecision,
    /// Reason a degrade occurred, or [`SkewDegradeReason::NotDegraded`].
    pub skew_degrade_reason: SkewDegradeReason,
    /// Locale that produces rendered text after evaluation.
    pub effective_locale: String,
    /// Fallback origin class for the rendered surface.
    pub fallback_origin_class: LocaleFallbackOriginClass,
    /// Degraded localization state after evaluation.
    pub degraded_localization_state: DegradedLocalizationState,
    /// Total translatable keys.
    pub total_key_count: usize,
    /// Keys the pack declares a translation for.
    pub translated_key_count: usize,
    /// Keys falling back to source language after evaluation.
    pub missing_key_count: usize,
    /// Missing-key count per surface family, keyed by snake_case family name.
    pub missing_key_count_by_surface: BTreeMap<String, usize>,
}

/// Decides whether a pack applies or degrades, and why.
///
/// This is the single source of truth for skew handling. It is intentionally
/// conservative: any condition that makes the pack unsupported, unsigned,
/// tampered, or skewed yields a full degrade to source language.
pub fn decide_application(
    input: &PackEvaluationInput,
) -> (PackApplicationDecision, SkewDegradeReason) {
    use PackApplicationDecision::{
        ApplyLocalizedWithDisclosedMissingKeys, DegradeToSourceLanguageOnly,
    };

    if !input.pack_present {
        return (DegradeToSourceLanguageOnly, SkewDegradeReason::PackMissing);
    }
    if !input.policy_locale_enabled {
        return (
            DegradeToSourceLanguageOnly,
            SkewDegradeReason::PolicyDisabledLocale,
        );
    }
    if !input.signature_state.may_render() {
        let reason = match input.signature_state {
            LocalePackSignatureState::SignatureFailedBlocked => SkewDegradeReason::SignatureFailed,
            _ => SkewDegradeReason::SignatureUnverifiedNotAccepted,
        };
        return (DegradeToSourceLanguageOnly, reason);
    }
    if !input.integrity_digest_matches {
        return (
            DegradeToSourceLanguageOnly,
            SkewDegradeReason::IntegrityDigestMismatch,
        );
    }
    if !input.version_match_state.may_render() || !input.target_build_in_compatibility_range {
        let reason = if input.version_match_state == VersionMatchState::UnknownTargetBuild {
            SkewDegradeReason::UnknownTargetBuild
        } else if !input.target_build_in_compatibility_range {
            SkewDegradeReason::BuildOutsideCompatibilityRange
        } else {
            SkewDegradeReason::IncompatibleVersionDrift
        };
        return (DegradeToSourceLanguageOnly, reason);
    }
    (
        ApplyLocalizedWithDisclosedMissingKeys,
        SkewDegradeReason::NotDegraded,
    )
}

fn degrade_origin(reason: SkewDegradeReason) -> LocaleFallbackOriginClass {
    match reason {
        SkewDegradeReason::PackMissing => LocaleFallbackOriginClass::PackMissingSourceLanguageOnly,
        SkewDegradeReason::SignatureFailed
        | SkewDegradeReason::SignatureUnverifiedNotAccepted
        | SkewDegradeReason::IntegrityDigestMismatch => {
            LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly
        }
        SkewDegradeReason::PolicyDisabledLocale => {
            LocaleFallbackOriginClass::PolicyDisabledSourceLanguageOnly
        }
        SkewDegradeReason::BuildOutsideCompatibilityRange
        | SkewDegradeReason::IncompatibleVersionDrift
        | SkewDegradeReason::UnknownTargetBuild => {
            LocaleFallbackOriginClass::SourceLanguageFallback
        }
        SkewDegradeReason::NotDegraded => LocaleFallbackOriginClass::RequestedLocaleAuthoritative,
    }
}

/// One evaluated compatibility row for a pack against the active build.
///
/// This is what diagnostics, settings, support export, and release tooling read
/// to learn the exact pack version, compatibility and signature state, active
/// fallback, missing-key counts, and — for degraded packs — the reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackCompatibilityRow {
    /// Stable row id.
    pub row_id: String,
    /// Stable pack id.
    pub pack_id: String,
    /// Pack version for support and release reporting.
    pub pack_version: String,
    /// Pack revision ref.
    pub pack_revision_ref: String,
    /// Pack source class.
    pub source_class: LocalePackSourceClass,
    /// Distribution class.
    pub distribution_class: LocalePackDistributionClass,
    /// Mirrorability posture.
    pub mirrorability_class: LocalePackMirrorabilityClass,
    /// Signer identity ref, when signed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_identity_ref: Option<String>,
    /// User-requested locale.
    pub requested_locale: String,
    /// Locale that produces rendered text after evaluation.
    pub effective_locale: String,
    /// Product source-language locale.
    pub source_language_locale: String,
    /// Ordered requested-to-base-to-source fallback chain.
    pub fallback_chain: Vec<String>,
    /// Active build identity the row was evaluated against.
    pub target_build_identity_ref: String,
    /// Inclusive compatibility build range declared by the pack.
    pub compatibility_build_range: CompatibilityBuildRange,
    /// Whether the active build is inside the compatibility range.
    pub target_build_in_compatibility_range: bool,
    /// Observed signature state.
    pub signature_state: LocalePackSignatureState,
    /// Observed version-match state.
    pub version_match_state: VersionMatchState,
    /// Whether the content integrity digest matched.
    pub integrity_digest_matches: bool,
    /// Whether the pack was present for the requested locale.
    pub pack_present: bool,
    /// Whether policy permitted localizing the locale.
    pub policy_locale_enabled: bool,
    /// Apply-or-degrade decision.
    pub application_decision: PackApplicationDecision,
    /// Reason a degrade occurred.
    pub skew_degrade_reason: SkewDegradeReason,
    /// Fallback origin class for the rendered surface.
    pub fallback_origin_class: LocaleFallbackOriginClass,
    /// Degraded localization state after evaluation.
    pub degraded_localization_state: DegradedLocalizationState,
    /// Total translatable keys.
    pub total_key_count: usize,
    /// Keys the pack declares a translation for.
    pub translated_key_count: usize,
    /// Keys falling back to source language after evaluation.
    pub missing_key_count: usize,
    /// Missing-key count per surface family, keyed by snake_case family name.
    pub missing_key_count_by_surface: BTreeMap<String, usize>,
    /// Whether this row backs a claimed localized profile.
    pub claimed_localized_profile: bool,
    /// Claimed profile ref, when this row backs one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_profile_ref: Option<String>,
    /// Decision row authorizing an unsigned but accepted pack, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_acceptance_decision_row_ref: Option<String>,
    /// Whether Settings exposes this row.
    pub visible_in_settings: bool,
    /// Whether diagnostics exposes this row.
    pub visible_in_diagnostics: bool,
    /// Whether support export exposes this row.
    pub visible_in_support_export: bool,
    /// Same-surface source-language route.
    pub open_in_source_language_route_ref: String,
    /// Whether degraded localization keeps local product use available.
    pub non_blocking_core_use: bool,
    /// Short export-safe label rendered on inspection surfaces.
    pub presentation_label: String,
}

impl LocalePackCompatibilityRow {
    fn evaluation_input(&self) -> PackEvaluationInput {
        PackEvaluationInput {
            target_build_identity_ref: self.target_build_identity_ref.clone(),
            target_build_in_compatibility_range: self.target_build_in_compatibility_range,
            signature_state: self.signature_state,
            version_match_state: self.version_match_state,
            integrity_digest_matches: self.integrity_digest_matches,
            pack_present: self.pack_present,
            policy_locale_enabled: self.policy_locale_enabled,
        }
    }
}

/// Governed install, upgrade, mirror, or downgrade of a locale pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackDeliveryOperation {
    /// Stable operation id.
    pub operation_id: String,
    /// Operation class.
    pub operation_class: LocalePackOperationClass,
    /// Pack id affected by this operation.
    pub pack_id: String,
    /// Pack version this operation moves from, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_pack_version: Option<String>,
    /// Pack version this operation moves to.
    pub to_pack_version: String,
    /// Whether signature verification is required before applying.
    pub signature_verification_required: bool,
    /// Whether compatibility evaluation is required before applying.
    pub compatibility_check_required: bool,
    /// Whether mirror or offline provenance metadata is preserved.
    pub mirror_metadata_preserved: bool,
    /// Compatibility row id this operation resolved to.
    pub resulting_decision_row_ref: String,
    /// Rollback target emitted by this operation, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_ref: Option<String>,
    /// Export-safe summary of the operation result.
    pub support_export_ref: String,
}

/// Summary posture derived from the compatibility rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackCompatibilitySummary {
    /// Number of evaluated packs.
    pub total_packs: usize,
    /// Packs that applied their translations.
    pub renderable_packs: usize,
    /// Packs that degraded fully to source language.
    pub degraded_source_language_packs: usize,
    /// Rows backing a claimed localized profile.
    pub claimed_localized_profiles: usize,
    /// Claimed localized profiles with zero missing keys.
    pub claimed_profiles_fully_localized: usize,
    /// Total missing keys across all evaluated packs.
    pub total_missing_keys: usize,
    /// True when no unsigned or incompatible pack masquerades as a claimed
    /// localized profile.
    pub guardrail_clean: bool,
}

/// Inspectable locale-pack compatibility and skew report.
///
/// Diagnostics, support export, and release tooling ingest this packet instead
/// of cloning localization status prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalePackCompatibilityReport {
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
    /// Active build identity the report was evaluated against.
    pub target_build_identity_ref: String,
    /// Source contracts that govern this report.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime consumers that ingest this report.
    pub runtime_consumer_refs: Vec<String>,
    /// Evaluated pack artifacts.
    pub artifacts: Vec<LocalePackArtifact>,
    /// Governance operations that produced the rows.
    pub operations: Vec<LocalePackDeliveryOperation>,
    /// Evaluated compatibility rows.
    pub rows: Vec<LocalePackCompatibilityRow>,
    /// Summary posture derived from the rows.
    pub summary: LocalePackCompatibilitySummary,
    /// Material classes omitted from this report.
    pub omitted_material_classes: Vec<String>,
}

impl LocalePackCompatibilityReport {
    /// Returns the artifact for a pack id, when present.
    pub fn artifact(&self, pack_id: &str) -> Option<&LocalePackArtifact> {
        self.artifacts
            .iter()
            .find(|artifact| artifact.pack_id == pack_id)
    }

    /// Returns the compatibility row for a pack id, when present.
    pub fn row(&self, pack_id: &str) -> Option<&LocalePackCompatibilityRow> {
        self.rows.iter().find(|row| row.pack_id == pack_id)
    }

    /// Validates artifacts, evaluation consistency, skew guardrails, operations,
    /// and the derived summary.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != LOCALE_PACK_COMPATIBILITY_REPORT_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.report_id.clone(),
                "compatibility report record_kind is unsupported",
            ));
        }
        if self.schema_version != LOCALE_PACK_DELIVERY_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.report_id.clone(),
                "compatibility report schema_version is unsupported",
            ));
        }
        if self.source_language_locale != SOURCE_LANGUAGE_LOCALE {
            findings.push(LocalePackValidationFinding::new(
                self.report_id.clone(),
                "compatibility report source language drifted",
            ));
        }

        let mut artifact_ids = BTreeSet::new();
        for artifact in &self.artifacts {
            validate_artifact(artifact, &mut findings);
            if !artifact_ids.insert(artifact.pack_id.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    artifact.pack_id.clone(),
                    "duplicate pack artifact id",
                ));
            }
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            validate_row(row, self.artifact(&row.pack_id), &mut findings);
            if !row_ids.insert(row.row_id.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    row.row_id.clone(),
                    "duplicate compatibility row id",
                ));
            }
        }

        validate_operations(&self.operations, &artifact_ids, &row_ids, &mut findings);

        let expected = derive_summary(&self.rows);
        if self.summary != expected {
            findings.push(LocalePackValidationFinding::new(
                self.report_id.clone(),
                "compatibility report summary drifted from row state",
            ));
        }
        if !self.summary.guardrail_clean {
            findings.push(LocalePackValidationFinding::new(
                self.report_id.clone(),
                "an unsigned or incompatible pack masquerades as a claimed localized profile",
            ));
        }

        finish(findings)
    }
}

fn validate_artifact(
    artifact: &LocalePackArtifact,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    if artifact.record_kind != LOCALE_PACK_ARTIFACT_RECORD_KIND {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "pack artifact record_kind is unsupported",
        ));
    }
    if artifact.schema_version != LOCALE_PACK_DELIVERY_SCHEMA_VERSION {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "pack artifact schema_version is unsupported",
        ));
    }
    if artifact.pack_version.trim().is_empty()
        || artifact.pack_revision_ref.trim().is_empty()
        || artifact.rollback_ref.trim().is_empty()
    {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "pack artifact must cite version, revision, and rollback refs",
        ));
    }
    if artifact.source_language_locale != SOURCE_LANGUAGE_LOCALE {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "pack artifact source language drifted",
        ));
    }
    if !is_sha256_hex(&artifact.integrity_digest_sha256) {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "pack artifact integrity digest must be lowercase hex sha256",
        ));
    }
    if artifact
        .compatibility_build_range
        .min_build_identity_ref
        .trim()
        .is_empty()
        || artifact
            .compatibility_build_range
            .max_build_identity_ref
            .trim()
            .is_empty()
    {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "pack artifact must declare a compatibility build range",
        ));
    }
    if artifact.base_locale_fallback_chain.first() != Some(&artifact.locale)
        || artifact.base_locale_fallback_chain.last() != Some(&artifact.source_language_locale)
    {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "pack fallback chain must run from pack locale to source language",
        ));
    }
    if artifact.surface_coverage.is_empty() {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "pack artifact must declare per-surface coverage",
        ));
    }
    let mut surfaces = BTreeSet::new();
    for coverage in &artifact.surface_coverage {
        if !surfaces.insert(coverage.surface_family) {
            findings.push(LocalePackValidationFinding::new(
                artifact.pack_id.clone(),
                "pack artifact repeats a surface family",
            ));
        }
        if coverage.translated_key_count > coverage.total_key_count {
            findings.push(LocalePackValidationFinding::new(
                artifact.pack_id.clone(),
                "pack surface translated keys cannot exceed total keys",
            ));
        }
    }

    // Signature material must be paired, and a built-in source pack carries none.
    let signed =
        artifact.signer_identity_ref.is_some() || artifact.signature_artifact_ref.is_some();
    if signed
        && (artifact.signer_identity_ref.is_none() || artifact.signature_artifact_ref.is_none())
    {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "signed pack must carry both signer identity and signature artifact refs",
        ));
    }
    if artifact.source_class == LocalePackSourceClass::FirstPartySourceLanguage && signed {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "built-in source-language pack must not carry an external signature",
        ));
    }
    // Non-source packs must preserve mirror or offline provenance.
    if artifact.source_class != LocalePackSourceClass::FirstPartySourceLanguage
        && artifact.mirror_receipt_refs.is_empty()
        && artifact.offline_import_ref.is_none()
    {
        findings.push(LocalePackValidationFinding::new(
            artifact.pack_id.clone(),
            "non-source pack must preserve mirror or offline provenance",
        ));
    }
}

fn validate_row(
    row: &LocalePackCompatibilityRow,
    artifact: Option<&LocalePackArtifact>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let Some(artifact) = artifact else {
        findings.push(LocalePackValidationFinding::new(
            row.row_id.clone(),
            "compatibility row references an unknown pack artifact",
        ));
        return;
    };

    // The decision, reason, counts, effective locale, origin, and degraded state
    // must all match a fresh evaluation of the artifact against the row's own
    // observed inputs. A hand-edited row cannot lie about skew handling.
    let outcome = artifact.evaluate(&row.requested_locale, &row.evaluation_input());
    if row.application_decision != outcome.application_decision
        || row.skew_degrade_reason != outcome.skew_degrade_reason
        || row.effective_locale != outcome.effective_locale
        || row.fallback_origin_class != outcome.fallback_origin_class
        || row.degraded_localization_state != outcome.degraded_localization_state
        || row.total_key_count != outcome.total_key_count
        || row.translated_key_count != outcome.translated_key_count
        || row.missing_key_count != outcome.missing_key_count
        || row.missing_key_count_by_surface != outcome.missing_key_count_by_surface
    {
        findings.push(LocalePackValidationFinding::new(
            row.row_id.clone(),
            "compatibility row drifted from a fresh evaluation of its pack",
        ));
    }

    if row.pack_version != artifact.pack_version
        || row.pack_revision_ref != artifact.pack_revision_ref
        || row.source_class != artifact.source_class
        || row.distribution_class != artifact.distribution_class
        || row.mirrorability_class != artifact.mirrorability_class
        || row.compatibility_build_range != artifact.compatibility_build_range
    {
        findings.push(LocalePackValidationFinding::new(
            row.row_id.clone(),
            "compatibility row metadata drifted from its pack artifact",
        ));
    }

    if row.fallback_chain.first() != Some(&row.requested_locale)
        || row.fallback_chain.last() != Some(&row.source_language_locale)
    {
        findings.push(LocalePackValidationFinding::new(
            row.row_id.clone(),
            "compatibility row fallback chain must run requested to source language",
        ));
    }

    if !row.visible_in_settings
        || !row.visible_in_diagnostics
        || !row.visible_in_support_export
        || !row.non_blocking_core_use
        || row.open_in_source_language_route_ref.trim().is_empty()
    {
        findings.push(LocalePackValidationFinding::new(
            row.row_id.clone(),
            "compatibility row must be visible, source-language reachable, and non-blocking",
        ));
    }

    // Skew guardrail: a degraded pack cannot back a claimed localized profile,
    // and a claimed localized profile must fully render.
    if row.application_decision == PackApplicationDecision::DegradeToSourceLanguageOnly
        && row.claimed_localized_profile
    {
        findings.push(LocalePackValidationFinding::new(
            row.row_id.clone(),
            "degraded pack must not back a claimed localized profile",
        ));
    }
    if row.claimed_localized_profile
        && (!row.application_decision.applies()
            || !row.signature_state.may_render()
            || !row.version_match_state.may_render()
            || !row.integrity_digest_matches
            || !row.target_build_in_compatibility_range
            || !row.pack_present
            || !row.policy_locale_enabled)
    {
        findings.push(LocalePackValidationFinding::new(
            row.row_id.clone(),
            "claimed localized profile must render from a signed, compatible, present pack",
        ));
    }
    if row.claimed_localized_profile && row.claimed_profile_ref.is_none() {
        findings.push(LocalePackValidationFinding::new(
            row.row_id.clone(),
            "claimed localized profile must cite a profile ref",
        ));
    }

    // An accepted unsigned pack must cite the decision that admitted it and may
    // never be promoted to a claimed localized profile.
    if row.signature_state == LocalePackSignatureState::UnsignedExplicitAcceptance {
        if row.explicit_acceptance_decision_row_ref.is_none() {
            findings.push(LocalePackValidationFinding::new(
                row.row_id.clone(),
                "unsigned accepted pack must cite an explicit acceptance decision",
            ));
        }
        if row.claimed_localized_profile {
            findings.push(LocalePackValidationFinding::new(
                row.row_id.clone(),
                "unsigned accepted pack must not masquerade as a claimed localized profile",
            ));
        }
    }
}

fn validate_operations(
    operations: &[LocalePackDeliveryOperation],
    artifact_ids: &BTreeSet<&str>,
    row_ids: &BTreeSet<&str>,
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let mut operation_ids = BTreeSet::new();
    let mut classes = BTreeSet::new();
    for operation in operations {
        classes.insert(operation.operation_class);
        if !operation_ids.insert(operation.operation_id.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                operation.operation_id.clone(),
                "duplicate delivery operation id",
            ));
        }
        if !artifact_ids.contains(operation.pack_id.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                operation.operation_id.clone(),
                "delivery operation references an unknown pack",
            ));
        }
        if !row_ids.contains(operation.resulting_decision_row_ref.as_str()) {
            findings.push(LocalePackValidationFinding::new(
                operation.operation_id.clone(),
                "delivery operation references an unknown compatibility row",
            ));
        }
        if !operation.signature_verification_required || !operation.compatibility_check_required {
            findings.push(LocalePackValidationFinding::new(
                operation.operation_id.clone(),
                "delivery operation must require signature and compatibility checks",
            ));
        }
        if matches!(
            operation.operation_class,
            LocalePackOperationClass::MirrorImport | LocalePackOperationClass::OfflineImport
        ) && !operation.mirror_metadata_preserved
        {
            findings.push(LocalePackValidationFinding::new(
                operation.operation_id.clone(),
                "mirror or offline import must preserve provenance metadata",
            ));
        }
        if operation.operation_class == LocalePackOperationClass::Rollback
            && operation.rollback_ref.is_none()
        {
            findings.push(LocalePackValidationFinding::new(
                operation.operation_id.clone(),
                "rollback operation must cite a rollback ref",
            ));
        }
    }

    for required in [
        LocalePackOperationClass::Install,
        LocalePackOperationClass::Update,
        LocalePackOperationClass::Rollback,
        LocalePackOperationClass::MirrorImport,
        LocalePackOperationClass::OfflineImport,
    ] {
        if !classes.contains(&required) {
            findings.push(LocalePackValidationFinding::new(
                LOCALE_PACK_COMPATIBILITY_REPORT_ID,
                format!("compatibility report is missing a {required:?} operation"),
            ));
        }
    }
}

fn derive_summary(rows: &[LocalePackCompatibilityRow]) -> LocalePackCompatibilitySummary {
    let renderable_packs = rows
        .iter()
        .filter(|row| row.application_decision.applies())
        .count();
    let degraded_source_language_packs = rows.len() - renderable_packs;
    let claimed_localized_profiles = rows
        .iter()
        .filter(|row| row.claimed_localized_profile)
        .count();
    let claimed_profiles_fully_localized = rows
        .iter()
        .filter(|row| row.claimed_localized_profile && row.missing_key_count == 0)
        .count();
    let total_missing_keys = rows.iter().map(|row| row.missing_key_count).sum();
    let guardrail_clean = rows.iter().all(|row| {
        if row.claimed_localized_profile {
            row.application_decision.applies()
                && row.signature_state.may_render()
                && row.version_match_state.may_render()
                && row.integrity_digest_matches
                && row.target_build_in_compatibility_range
        } else {
            true
        }
    });

    LocalePackCompatibilitySummary {
        total_packs: rows.len(),
        renderable_packs,
        degraded_source_language_packs,
        claimed_localized_profiles,
        claimed_profiles_fully_localized,
        total_missing_keys,
        guardrail_clean,
    }
}

/// Returns a stable snake_case key for a surface family.
fn surface_family_key(surface_family: MessageSurfaceFamily) -> &'static str {
    match surface_family {
        MessageSurfaceFamily::ShellChrome => "shell_chrome",
        MessageSurfaceFamily::CommandLabel => "command_label",
        MessageSurfaceFamily::SettingsHelpOrError => "settings_help_or_error",
        MessageSurfaceFamily::DocsTourOrAuthText => "docs_tour_or_auth_text",
        MessageSurfaceFamily::ExtensionContributedUi => "extension_contributed_ui",
        MessageSurfaceFamily::CliHelpText => "cli_help_text",
        MessageSurfaceFamily::ExportOrReportHeading => "export_or_report_heading",
        MessageSurfaceFamily::ScreenshotOrDemoCaption => "screenshot_or_demo_caption",
        MessageSurfaceFamily::GlossaryOrTerminologyTerm => "glossary_or_terminology_term",
        MessageSurfaceFamily::PolicyLegalOrRecoveryText => "policy_legal_or_recovery_text",
        MessageSurfaceFamily::PseudolocOnlyTestString => "pseudoloc_only_test_string",
    }
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn finish(
    findings: Vec<LocalePackValidationFinding>,
) -> Result<(), Vec<LocalePackValidationFinding>> {
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

// ---------------------------------------------------------------------------
// Seeded data
// ---------------------------------------------------------------------------

const REPORT_BUILD: &str = TARGET_BUILD;
const COMPAT_MIN_BUILD: &str = "build:aureline:0.0.0-stable.2026.05.01";

/// Returns the checked-in first-party core locale-pack artifacts.
///
/// These are the artifacts persisted under [`LOCALE_PACK_CORE_ARTIFACT_ROOT`].
/// They are all well-formed, shippable packs; whether each one *renders* in a
/// given environment is decided by evaluation, not by the artifact.
pub fn seeded_core_locale_pack_artifacts() -> Vec<LocalePackArtifact> {
    vec![
        source_artifact(),
        es_mx_artifact(),
        fr_fr_artifact(),
        ja_jp_artifact(),
        de_de_artifact(),
    ]
}

/// Returns the seeded locale-pack compatibility and skew report.
pub fn seeded_locale_pack_compatibility_report() -> LocalePackCompatibilityReport {
    let artifacts = {
        let mut artifacts = seeded_core_locale_pack_artifacts();
        artifacts.push(pt_br_artifact());
        artifacts
    };

    let rows = vec![
        row_for(
            &source_artifact(),
            "row:locale-pack:source:en-us",
            "en-US",
            apply_input(LocalePackSignatureState::NotApplicableBuiltIn),
            ClaimGov::claimed("profile:locale:en-us:source"),
        ),
        row_for(
            &es_mx_artifact(),
            "row:locale-pack:core:es-mx",
            "es-MX",
            apply_input(LocalePackSignatureState::SignedVerified),
            ClaimGov::claimed("profile:locale:es-mx:stable"),
        ),
        row_for(
            &fr_fr_artifact(),
            "row:locale-pack:core:fr-fr",
            "fr-FR",
            PackEvaluationInput {
                version_match_state: VersionMatchState::CompatibleMinorDrift,
                ..apply_input(LocalePackSignatureState::SignedVerified)
            },
            ClaimGov::claimed("profile:locale:fr-fr:stable"),
        ),
        row_for(
            &ja_jp_artifact(),
            "row:locale-pack:core:ja-jp",
            "ja-JP",
            PackEvaluationInput {
                signature_state: LocalePackSignatureState::SignatureFailedBlocked,
                version_match_state: VersionMatchState::ExactBuildMatch,
                ..apply_input(LocalePackSignatureState::SignatureFailedBlocked)
            },
            ClaimGov::narrowed(),
        ),
        row_for(
            &de_de_artifact(),
            "row:locale-pack:core:de-de",
            "de-DE",
            PackEvaluationInput {
                signature_state: LocalePackSignatureState::SignedVerified,
                version_match_state: VersionMatchState::IncompatibleDriftDetected,
                target_build_in_compatibility_range: false,
                ..apply_input(LocalePackSignatureState::SignedVerified)
            },
            ClaimGov::narrowed(),
        ),
        row_for(
            &pt_br_artifact(),
            "row:locale-pack:community:pt-br",
            "pt-BR",
            PackEvaluationInput {
                signature_state: LocalePackSignatureState::UnsignedExplicitAcceptance,
                version_match_state: VersionMatchState::CompatibleMinorDrift,
                ..apply_input(LocalePackSignatureState::UnsignedExplicitAcceptance)
            },
            ClaimGov::accepted_unsigned(
                "decision:locale-pack:community:pt-br:unsigned-acceptance:2026.05.18",
            ),
        ),
    ];

    let operations = seeded_operations();
    let summary = derive_summary(&rows);

    LocalePackCompatibilityReport {
        record_kind: LOCALE_PACK_COMPATIBILITY_REPORT_RECORD_KIND.to_owned(),
        schema_version: LOCALE_PACK_DELIVERY_SCHEMA_VERSION,
        report_id: LOCALE_PACK_COMPATIBILITY_REPORT_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        target_build_identity_ref: REPORT_BUILD.to_owned(),
        source_contract_refs: BTreeMap::from([
            (
                "architecture_locale_governance".to_owned(),
                ".t2/docs/Aureline_Technical_Architecture_Document.md#23.3.1".to_owned(),
            ),
            (
                "locale_pack_lifecycle".to_owned(),
                ".t2/docs/Aureline_Technical_Architecture_Document.md#appendix-df".to_owned(),
            ),
            (
                "pack_artifact_schema".to_owned(),
                "schemas/i18n/locale-pack.schema.json".to_owned(),
            ),
            (
                "compatibility_report_schema".to_owned(),
                "schemas/i18n/locale-pack-compatibility-report.schema.json".to_owned(),
            ),
            (
                "message_registry".to_owned(),
                "fixtures/i18n/message-id-stability/registry.json".to_owned(),
            ),
        ]),
        runtime_consumer_refs: vec![
            "crates/aureline-i18n".to_owned(),
            "crates/aureline-shell".to_owned(),
            "crates/aureline-help".to_owned(),
            "crates/aureline-release".to_owned(),
            "crates/aureline-support".to_owned(),
        ],
        artifacts,
        operations,
        rows,
        summary,
        omitted_material_classes: vec![
            "raw_translated_message_body".to_owned(),
            "signing_private_key_material".to_owned(),
            "raw_provider_payload".to_owned(),
        ],
    }
}

fn seeded_operations() -> Vec<LocalePackDeliveryOperation> {
    vec![
        LocalePackDeliveryOperation {
            operation_id: "op:install:locale-pack:core:es-mx".to_owned(),
            operation_class: LocalePackOperationClass::Install,
            pack_id: "locale-pack:core:es-mx".to_owned(),
            from_pack_version: None,
            to_pack_version: "2026.05.18+1".to_owned(),
            signature_verification_required: true,
            compatibility_check_required: true,
            mirror_metadata_preserved: true,
            resulting_decision_row_ref: "row:locale-pack:core:es-mx".to_owned(),
            rollback_ref: Some("rollback:locale-pack:core:es-mx:2026.05.17+1".to_owned()),
            support_export_ref: "support:locale-pack:core:es-mx:install".to_owned(),
        },
        LocalePackDeliveryOperation {
            operation_id: "op:update:locale-pack:core:fr-fr".to_owned(),
            operation_class: LocalePackOperationClass::Update,
            pack_id: "locale-pack:core:fr-fr".to_owned(),
            from_pack_version: Some("2026.05.10+1".to_owned()),
            to_pack_version: "2026.05.18+1".to_owned(),
            signature_verification_required: true,
            compatibility_check_required: true,
            mirror_metadata_preserved: true,
            resulting_decision_row_ref: "row:locale-pack:core:fr-fr".to_owned(),
            rollback_ref: Some("rollback:locale-pack:core:fr-fr:2026.05.10+1".to_owned()),
            support_export_ref: "support:locale-pack:core:fr-fr:update".to_owned(),
        },
        LocalePackDeliveryOperation {
            operation_id: "op:mirror:locale-pack:core:es-mx".to_owned(),
            operation_class: LocalePackOperationClass::MirrorImport,
            pack_id: "locale-pack:core:es-mx".to_owned(),
            from_pack_version: None,
            to_pack_version: "2026.05.18+1".to_owned(),
            signature_verification_required: true,
            compatibility_check_required: true,
            mirror_metadata_preserved: true,
            resulting_decision_row_ref: "row:locale-pack:core:es-mx".to_owned(),
            rollback_ref: None,
            support_export_ref: "support:locale-pack:core:es-mx:mirror".to_owned(),
        },
        LocalePackDeliveryOperation {
            operation_id: "op:rollback:locale-pack:core:de-de".to_owned(),
            operation_class: LocalePackOperationClass::Rollback,
            pack_id: "locale-pack:core:de-de".to_owned(),
            from_pack_version: Some("2026.05.18+1".to_owned()),
            to_pack_version: "2026.05.01+1".to_owned(),
            signature_verification_required: true,
            compatibility_check_required: true,
            mirror_metadata_preserved: true,
            resulting_decision_row_ref: "row:locale-pack:core:de-de".to_owned(),
            rollback_ref: Some("rollback:locale-pack:core:de-de:2026.05.01+1".to_owned()),
            support_export_ref: "support:locale-pack:core:de-de:rollback".to_owned(),
        },
        LocalePackDeliveryOperation {
            operation_id: "op:offline:locale-pack:community:pt-br".to_owned(),
            operation_class: LocalePackOperationClass::OfflineImport,
            pack_id: "locale-pack:community:pt-br".to_owned(),
            from_pack_version: None,
            to_pack_version: "2026.05.18+1".to_owned(),
            signature_verification_required: true,
            compatibility_check_required: true,
            mirror_metadata_preserved: true,
            resulting_decision_row_ref: "row:locale-pack:community:pt-br".to_owned(),
            rollback_ref: None,
            support_export_ref: "support:locale-pack:community:pt-br:offline".to_owned(),
        },
    ]
}

/// Governance posture applied when assembling one compatibility row.
struct ClaimGov {
    claimed_localized_profile: bool,
    claimed_profile_ref: Option<String>,
    explicit_acceptance_decision_row_ref: Option<String>,
}

impl ClaimGov {
    fn claimed(profile_ref: &str) -> Self {
        Self {
            claimed_localized_profile: true,
            claimed_profile_ref: Some(profile_ref.to_owned()),
            explicit_acceptance_decision_row_ref: None,
        }
    }

    fn narrowed() -> Self {
        Self {
            claimed_localized_profile: false,
            claimed_profile_ref: None,
            explicit_acceptance_decision_row_ref: None,
        }
    }

    fn accepted_unsigned(decision_ref: &str) -> Self {
        Self {
            claimed_localized_profile: false,
            claimed_profile_ref: None,
            explicit_acceptance_decision_row_ref: Some(decision_ref.to_owned()),
        }
    }
}

fn apply_input(signature_state: LocalePackSignatureState) -> PackEvaluationInput {
    PackEvaluationInput {
        target_build_identity_ref: REPORT_BUILD.to_owned(),
        target_build_in_compatibility_range: true,
        signature_state,
        version_match_state: VersionMatchState::ExactBuildMatch,
        integrity_digest_matches: true,
        pack_present: true,
        policy_locale_enabled: true,
    }
}

fn row_for(
    artifact: &LocalePackArtifact,
    row_id: &str,
    requested_locale: &str,
    input: PackEvaluationInput,
    governance: ClaimGov,
) -> LocalePackCompatibilityRow {
    let outcome = artifact.evaluate(requested_locale, &input);
    let presentation_label = match outcome.application_decision {
        PackApplicationDecision::ApplyLocalizedWithDisclosedMissingKeys => {
            if outcome.missing_key_count == 0 {
                format!(
                    "{} {}: fully localized",
                    artifact.presentation_label, artifact.pack_version
                )
            } else {
                format!(
                    "{} {}: localized, {} keys fall back to source",
                    artifact.presentation_label, artifact.pack_version, outcome.missing_key_count
                )
            }
        }
        PackApplicationDecision::DegradeToSourceLanguageOnly => format!(
            "{} {}: source language only ({:?})",
            artifact.presentation_label, artifact.pack_version, outcome.skew_degrade_reason
        ),
    };

    LocalePackCompatibilityRow {
        row_id: row_id.to_owned(),
        pack_id: artifact.pack_id.clone(),
        pack_version: artifact.pack_version.clone(),
        pack_revision_ref: artifact.pack_revision_ref.clone(),
        source_class: artifact.source_class,
        distribution_class: artifact.distribution_class,
        mirrorability_class: artifact.mirrorability_class,
        signer_identity_ref: artifact.signer_identity_ref.clone(),
        requested_locale: requested_locale.to_owned(),
        effective_locale: outcome.effective_locale,
        source_language_locale: artifact.source_language_locale.clone(),
        fallback_chain: artifact.base_locale_fallback_chain.clone(),
        target_build_identity_ref: input.target_build_identity_ref.clone(),
        compatibility_build_range: artifact.compatibility_build_range.clone(),
        target_build_in_compatibility_range: input.target_build_in_compatibility_range,
        signature_state: input.signature_state,
        version_match_state: input.version_match_state,
        integrity_digest_matches: input.integrity_digest_matches,
        pack_present: input.pack_present,
        policy_locale_enabled: input.policy_locale_enabled,
        application_decision: outcome.application_decision,
        skew_degrade_reason: outcome.skew_degrade_reason,
        fallback_origin_class: outcome.fallback_origin_class,
        degraded_localization_state: outcome.degraded_localization_state,
        total_key_count: outcome.total_key_count,
        translated_key_count: outcome.translated_key_count,
        missing_key_count: outcome.missing_key_count,
        missing_key_count_by_surface: outcome.missing_key_count_by_surface,
        claimed_localized_profile: governance.claimed_localized_profile,
        claimed_profile_ref: governance.claimed_profile_ref,
        explicit_acceptance_decision_row_ref: governance.explicit_acceptance_decision_row_ref,
        visible_in_settings: true,
        visible_in_diagnostics: true,
        visible_in_support_export: true,
        open_in_source_language_route_ref: SOURCE_LANGUAGE_ROUTE.to_owned(),
        non_blocking_core_use: true,
        presentation_label,
    }
}

fn full_coverage() -> Vec<SurfaceKeyCoverage> {
    surface_coverage(&[(6, 6), (10, 10), (12, 12), (8, 8)])
}

fn surface_coverage(values: &[(usize, usize)]) -> Vec<SurfaceKeyCoverage> {
    let families = [
        MessageSurfaceFamily::ShellChrome,
        MessageSurfaceFamily::CommandLabel,
        MessageSurfaceFamily::SettingsHelpOrError,
        MessageSurfaceFamily::DocsTourOrAuthText,
    ];
    families
        .iter()
        .zip(values)
        .map(|(surface_family, (total, translated))| SurfaceKeyCoverage {
            surface_family: *surface_family,
            total_key_count: *total,
            translated_key_count: *translated,
        })
        .collect()
}

fn compatibility_range() -> CompatibilityBuildRange {
    CompatibilityBuildRange {
        min_build_identity_ref: COMPAT_MIN_BUILD.to_owned(),
        max_build_identity_ref: REPORT_BUILD.to_owned(),
    }
}

fn source_artifact() -> LocalePackArtifact {
    LocalePackArtifact {
        record_kind: LOCALE_PACK_ARTIFACT_RECORD_KIND.to_owned(),
        schema_version: LOCALE_PACK_DELIVERY_SCHEMA_VERSION,
        pack_id: "locale-pack:core:source:en-us".to_owned(),
        pack_version: "2026.05.18+1".to_owned(),
        pack_revision_ref: "locale-pack-rev:core:source:en-us:2026.05.18-01".to_owned(),
        locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        coverage_locales: vec![SOURCE_LANGUAGE_LOCALE.to_owned()],
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        base_locale_fallback_chain: vec![SOURCE_LANGUAGE_LOCALE.to_owned()],
        source_class: LocalePackSourceClass::FirstPartySourceLanguage,
        distribution_class: LocalePackDistributionClass::BuiltInWithProduct,
        signer_identity_ref: None,
        signature_artifact_ref: None,
        integrity_digest_sha256: "d65d050d50ea37f29a3e28116cd5a8672aee1e3177d6f9f27a1f1bae6ee128f2"
            .to_owned(),
        mirrorability_class: LocalePackMirrorabilityClass::MirrorAllowed,
        mirror_receipt_refs: vec!["mirror-receipt:core:source:en-us".to_owned()],
        offline_import_ref: Some("offline-import:core:source:en-us".to_owned()),
        compatibility_build_range: compatibility_range(),
        rollback_ref: "rollback:locale-pack:core:source:en-us:last-known-good".to_owned(),
        surface_coverage: full_coverage(),
        presentation_label: "English source language".to_owned(),
        minted_at: GENERATED_AT.to_owned(),
    }
}

fn es_mx_artifact() -> LocalePackArtifact {
    LocalePackArtifact {
        record_kind: LOCALE_PACK_ARTIFACT_RECORD_KIND.to_owned(),
        schema_version: LOCALE_PACK_DELIVERY_SCHEMA_VERSION,
        pack_id: "locale-pack:core:es-mx".to_owned(),
        pack_version: "2026.05.18+1".to_owned(),
        pack_revision_ref: "locale-pack-rev:core:es-mx:2026.05.18-01".to_owned(),
        locale: "es-MX".to_owned(),
        coverage_locales: vec!["es-MX".to_owned(), "es".to_owned()],
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        base_locale_fallback_chain: vec![
            "es-MX".to_owned(),
            "es".to_owned(),
            SOURCE_LANGUAGE_LOCALE.to_owned(),
        ],
        source_class: LocalePackSourceClass::FirstPartyLocalePack,
        distribution_class: LocalePackDistributionClass::MirroredOfficialPack,
        signer_identity_ref: Some("signer:first-party:locale-pack-release-root".to_owned()),
        signature_artifact_ref: Some("signature:locale-pack:core:es-mx:2026.05.18-01".to_owned()),
        integrity_digest_sha256: "c1c51dec3981f8ce56570c36e15cf699c6aea490ae51510a9e99aaf8dea854c5"
            .to_owned(),
        mirrorability_class: LocalePackMirrorabilityClass::MirrorWithAttributionRequired,
        mirror_receipt_refs: vec![
            "mirror-receipt:official:locale-pack:core:es-mx".to_owned(),
            "mirror-receipt:airgap:locale-pack:core:es-mx".to_owned(),
        ],
        offline_import_ref: Some("offline-import:locale-pack:core:es-mx:bundle-01".to_owned()),
        compatibility_build_range: compatibility_range(),
        rollback_ref: "rollback:locale-pack:core:es-mx:2026.05.17+1".to_owned(),
        surface_coverage: full_coverage(),
        presentation_label: "Spanish (Mexico) official pack".to_owned(),
        minted_at: GENERATED_AT.to_owned(),
    }
}

fn fr_fr_artifact() -> LocalePackArtifact {
    LocalePackArtifact {
        record_kind: LOCALE_PACK_ARTIFACT_RECORD_KIND.to_owned(),
        schema_version: LOCALE_PACK_DELIVERY_SCHEMA_VERSION,
        pack_id: "locale-pack:core:fr-fr".to_owned(),
        pack_version: "2026.05.18+1".to_owned(),
        pack_revision_ref: "locale-pack-rev:core:fr-fr:2026.05.18-01".to_owned(),
        locale: "fr-FR".to_owned(),
        coverage_locales: vec!["fr-FR".to_owned(), "fr".to_owned()],
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        base_locale_fallback_chain: vec![
            "fr-FR".to_owned(),
            "fr".to_owned(),
            SOURCE_LANGUAGE_LOCALE.to_owned(),
        ],
        source_class: LocalePackSourceClass::FirstPartyLocalePack,
        distribution_class: LocalePackDistributionClass::MirroredOfficialPack,
        signer_identity_ref: Some("signer:first-party:locale-pack-release-root".to_owned()),
        signature_artifact_ref: Some("signature:locale-pack:core:fr-fr:2026.05.18-01".to_owned()),
        integrity_digest_sha256: "bd8523ae43daab35bea125e6d113a2cfbd14b63f5cec0fc38a50aae41c8ed286"
            .to_owned(),
        mirrorability_class: LocalePackMirrorabilityClass::MirrorWithAttributionRequired,
        mirror_receipt_refs: vec!["mirror-receipt:official:locale-pack:core:fr-fr".to_owned()],
        offline_import_ref: Some("offline-import:locale-pack:core:fr-fr:bundle-01".to_owned()),
        compatibility_build_range: compatibility_range(),
        rollback_ref: "rollback:locale-pack:core:fr-fr:2026.05.10+1".to_owned(),
        // French pack ships partial docs coverage; missing keys fall back per key.
        surface_coverage: surface_coverage(&[(6, 6), (10, 10), (12, 12), (8, 5)]),
        presentation_label: "French (France) official pack".to_owned(),
        minted_at: GENERATED_AT.to_owned(),
    }
}

fn ja_jp_artifact() -> LocalePackArtifact {
    LocalePackArtifact {
        record_kind: LOCALE_PACK_ARTIFACT_RECORD_KIND.to_owned(),
        schema_version: LOCALE_PACK_DELIVERY_SCHEMA_VERSION,
        pack_id: "locale-pack:core:ja-jp".to_owned(),
        pack_version: "2026.05.18+1".to_owned(),
        pack_revision_ref: "locale-pack-rev:core:ja-jp:2026.05.18-01".to_owned(),
        locale: "ja-JP".to_owned(),
        coverage_locales: vec!["ja-JP".to_owned(), "ja".to_owned()],
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        base_locale_fallback_chain: vec![
            "ja-JP".to_owned(),
            "ja".to_owned(),
            SOURCE_LANGUAGE_LOCALE.to_owned(),
        ],
        source_class: LocalePackSourceClass::FirstPartyLocalePack,
        distribution_class: LocalePackDistributionClass::MirroredOfficialPack,
        signer_identity_ref: Some("signer:first-party:locale-pack-release-root".to_owned()),
        signature_artifact_ref: Some("signature:locale-pack:core:ja-jp:2026.05.18-01".to_owned()),
        integrity_digest_sha256: "67f666f2144c5f9bc152131619be81eba6f26c02550c4abe46eb884660082cf7"
            .to_owned(),
        mirrorability_class: LocalePackMirrorabilityClass::MirrorWithAttributionRequired,
        mirror_receipt_refs: vec!["mirror-receipt:official:locale-pack:core:ja-jp".to_owned()],
        offline_import_ref: Some("offline-import:locale-pack:core:ja-jp:bundle-01".to_owned()),
        compatibility_build_range: compatibility_range(),
        rollback_ref: "rollback:locale-pack:core:ja-jp:2026.05.10+1".to_owned(),
        // Fully translated on disk; signature failure means none of it applies.
        surface_coverage: full_coverage(),
        presentation_label: "Japanese (Japan) official pack".to_owned(),
        minted_at: GENERATED_AT.to_owned(),
    }
}

fn de_de_artifact() -> LocalePackArtifact {
    LocalePackArtifact {
        record_kind: LOCALE_PACK_ARTIFACT_RECORD_KIND.to_owned(),
        schema_version: LOCALE_PACK_DELIVERY_SCHEMA_VERSION,
        pack_id: "locale-pack:core:de-de".to_owned(),
        pack_version: "2026.05.18+1".to_owned(),
        pack_revision_ref: "locale-pack-rev:core:de-de:2026.05.18-01".to_owned(),
        locale: "de-DE".to_owned(),
        coverage_locales: vec!["de-DE".to_owned(), "de".to_owned()],
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        base_locale_fallback_chain: vec![
            "de-DE".to_owned(),
            "de".to_owned(),
            SOURCE_LANGUAGE_LOCALE.to_owned(),
        ],
        source_class: LocalePackSourceClass::FirstPartyLocalePack,
        distribution_class: LocalePackDistributionClass::MirroredOfficialPack,
        signer_identity_ref: Some("signer:first-party:locale-pack-release-root".to_owned()),
        signature_artifact_ref: Some("signature:locale-pack:core:de-de:2026.05.18-01".to_owned()),
        integrity_digest_sha256: "5c732f8b8509e0d7d92847f2d16284c91f2874feb9fe463ab2bfb1b382595474"
            .to_owned(),
        mirrorability_class: LocalePackMirrorabilityClass::MirrorWithAttributionRequired,
        mirror_receipt_refs: vec!["mirror-receipt:official:locale-pack:core:de-de".to_owned()],
        offline_import_ref: Some("offline-import:locale-pack:core:de-de:bundle-01".to_owned()),
        // This revision predates the active build window, so it is out of range.
        compatibility_build_range: CompatibilityBuildRange {
            min_build_identity_ref: "build:aureline:0.0.0-stable.2026.03.01".to_owned(),
            max_build_identity_ref: "build:aureline:0.0.0-stable.2026.04.15".to_owned(),
        },
        rollback_ref: "rollback:locale-pack:core:de-de:2026.05.01+1".to_owned(),
        surface_coverage: full_coverage(),
        presentation_label: "German (Germany) official pack".to_owned(),
        minted_at: GENERATED_AT.to_owned(),
    }
}

fn pt_br_artifact() -> LocalePackArtifact {
    LocalePackArtifact {
        record_kind: LOCALE_PACK_ARTIFACT_RECORD_KIND.to_owned(),
        schema_version: LOCALE_PACK_DELIVERY_SCHEMA_VERSION,
        pack_id: "locale-pack:community:pt-br".to_owned(),
        pack_version: "2026.05.18+1".to_owned(),
        pack_revision_ref: "locale-pack-rev:community:pt-br:2026.05.18-01".to_owned(),
        locale: "pt-BR".to_owned(),
        coverage_locales: vec!["pt-BR".to_owned(), "pt".to_owned()],
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        base_locale_fallback_chain: vec![
            "pt-BR".to_owned(),
            "pt".to_owned(),
            SOURCE_LANGUAGE_LOCALE.to_owned(),
        ],
        source_class: LocalePackSourceClass::ReviewedCommunityPack,
        distribution_class: LocalePackDistributionClass::CommunitySuppliedPack,
        // Unsigned community pack admitted only through an explicit decision row.
        signer_identity_ref: None,
        signature_artifact_ref: None,
        integrity_digest_sha256: "fc5b6f087901c80867507150628b37d6fa9be1e9e39ca3a28634c271aa035032"
            .to_owned(),
        mirrorability_class: LocalePackMirrorabilityClass::MirrorWithAttributionRequired,
        mirror_receipt_refs: vec!["mirror-receipt:community:locale-pack:pt-br".to_owned()],
        offline_import_ref: Some("offline-import:locale-pack:community:pt-br:bundle-01".to_owned()),
        compatibility_build_range: compatibility_range(),
        rollback_ref: "rollback:locale-pack:community:pt-br:2026.05.10+1".to_owned(),
        surface_coverage: surface_coverage(&[(6, 6), (10, 9), (12, 10), (8, 4)]),
        presentation_label: "Portuguese (Brazil) community pack".to_owned(),
        minted_at: GENERATED_AT.to_owned(),
    }
}
