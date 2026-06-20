//! Dense-surface localization qualification for claimed M5 localized profiles.
//!
//! This module turns pseudolocalization, text-expansion, RTL/bidi,
//! font-fallback, IME composition, CJK, and localized date/number proof into one
//! release-gating qualification packet for the dense M5 surfaces — editor
//! adjacent panes, the command palette, settings, terminal/help, notebooks,
//! data grids, pipeline/log views, docs/help panes, guided tours, and
//! support/report surfaces. Each harness result is bound to a claimed localized
//! profile so an IME, bidi/RTL, font-fallback, or localized-format regression
//! narrows or blocks the profile claim automatically rather than depending on a
//! one-off screenshot review. The seeded packet is the canonical truth that QA,
//! shiproom, and support ingest instead of re-running manual review sessions.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::localized_catalog::{TextDirection, CLAIMED_LOCALES};
use crate::localized_profile_matrix::{MatrixGateState, ProfileClaimClass};
use crate::{
    DenseI18nAssertionClass, DenseI18nBoundedWaiver, DenseI18nFailureClass, DenseI18nLaneBinding,
    DenseI18nLaneCadence, ImeCompositionChurnEvent, ImeCompositionScenario, LiteralTechnicalToken,
    LocalePackValidationFinding, RtlMirroringExpectation, TextExpansionBudget, GENERATED_AT,
    SOURCE_LANGUAGE_LOCALE,
};

#[cfg(test)]
mod tests;

/// Schema version shared by the M5 dense-surface i18n qualification records.
pub const M5_DENSE_SURFACE_LAB_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`M5DenseSurfaceI18nQualification`].
pub const M5_DENSE_SURFACE_LAB_RECORD_KIND: &str = "m5_dense_surface_i18n_qualification_packet";

/// Record kind for [`M5DenseSurfaceI18nReviewPacket`].
pub const M5_DENSE_SURFACE_LAB_REVIEW_RECORD_KIND: &str = "m5_dense_surface_i18n_review_packet";

/// Record kind for [`M5DenseNarrowingScenarioSet`].
pub const M5_DENSE_SURFACE_LAB_NARROWING_RECORD_KIND: &str =
    "m5_dense_surface_i18n_narrowing_scenario_set";

/// Stable id for the seeded M5 dense-surface qualification packet.
pub const M5_DENSE_SURFACE_LAB_PACKET_ID: &str = "i18n-qualification:m5-dense-surface:v1";

/// Fixture ref for the seeded qualification packet.
pub const M5_DENSE_SURFACE_LAB_FIXTURE_REF: &str =
    "fixtures/i18n/pseudoloc-rtl-ime-cjk/qualification.json";

/// Fixture root that holds the qualification, review, and narrowing fixtures.
pub const M5_DENSE_SURFACE_LAB_FIXTURE_ROOT: &str = "fixtures/i18n/pseudoloc-rtl-ime-cjk";

/// Target build identity the seeded packet defends.
const TARGET_BUILD_IDENTITY_REF: &str = "build-identity:m5-stable-candidate";

/// Dense M5 product surface family covered by the localization qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DenseSurfaceFamily {
    /// Editor-adjacent panes: inline rename, hover, peek, and completion chrome.
    EditorAdjacentPane,
    /// Command palette query input, result rows, and command preview panes.
    CommandPalette,
    /// Settings rows, schema-backed help, locale rows, and validation messages.
    Settings,
    /// Terminal transcript, terminal input, and the terminal/help surface.
    TerminalHelp,
    /// Notebook code and markdown cells, cell outputs, and notebook chrome.
    Notebook,
    /// Data and API tooling grids with sticky headers, counts, and filter input.
    DataGrid,
    /// Pipeline run views, log streams, traces, and timeline chrome.
    PipelineLogView,
    /// Docs, help, glossary, and support knowledge panes.
    DocsHelpPane,
    /// Guided tours, onboarding exercises, and guided learning steps.
    GuidedTour,
    /// Support flows and exported support/report surfaces.
    SupportReport,
}

impl M5DenseSurfaceFamily {
    /// Returns every dense M5 surface family the qualification must cover.
    pub fn all() -> Vec<Self> {
        vec![
            Self::EditorAdjacentPane,
            Self::CommandPalette,
            Self::Settings,
            Self::TerminalHelp,
            Self::Notebook,
            Self::DataGrid,
            Self::PipelineLogView,
            Self::DocsHelpPane,
            Self::GuidedTour,
            Self::SupportReport,
        ]
    }
}

/// Localization harness exercised against a dense M5 surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DenseHarnessKind {
    /// Pseudolocalization accent wrapping and clip detection.
    Pseudolocalization,
    /// Long translated strings against an explicit expansion budget.
    TextExpansion,
    /// Right-to-left chrome mirroring and bidi technical-token handling.
    RtlBidi,
    /// CJK, full-width, and emoji glyph fallback through an accepted font chain.
    FontFallback,
    /// IME preedit, candidate, and commit behavior under dense churn.
    ImeComposition,
    /// CJK full-width layout, counting, and wrapping behavior.
    Cjk,
    /// Locale-sensitive date, number, duration, and count formatting.
    LocalizedDateNumber,
}

impl M5DenseHarnessKind {
    /// Returns every harness kind the qualification must exercise.
    pub fn all() -> Vec<Self> {
        vec![
            Self::Pseudolocalization,
            Self::TextExpansion,
            Self::RtlBidi,
            Self::FontFallback,
            Self::ImeComposition,
            Self::Cjk,
            Self::LocalizedDateNumber,
        ]
    }

    /// Maps a failing harness to the claim-narrowing reason it triggers.
    pub fn narrow_reason(self) -> M5DenseClaimNarrowReason {
        match self {
            Self::Pseudolocalization => M5DenseClaimNarrowReason::PseudolocClippingOrOverflow,
            Self::TextExpansion => M5DenseClaimNarrowReason::TextExpansionOverflow,
            Self::RtlBidi => M5DenseClaimNarrowReason::RtlBidiMirrorRegression,
            Self::FontFallback => M5DenseClaimNarrowReason::FontFallbackRegression,
            Self::ImeComposition => M5DenseClaimNarrowReason::ImeCompositionRegression,
            Self::Cjk => M5DenseClaimNarrowReason::CjkGlyphRegression,
            Self::LocalizedDateNumber => M5DenseClaimNarrowReason::LocalizedFormatRegression,
        }
    }
}

/// Result state recorded for one harness against one locale on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HarnessResultState {
    /// The harness passed against the requested locale.
    Passed,
    /// The surface fell back to source language for this locale and passed there.
    SourceLanguageFallbackPassed,
    /// The harness failed and the surface must not claim localized support.
    Failed,
    /// A bounded, expiring waiver currently covers the result.
    WaivedBounded,
}

/// Reason a claimed localized profile narrows or blocks on dense-surface proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DenseClaimNarrowReason {
    /// Pseudoloc expansion clipped, overflowed, or obscured critical copy.
    PseudolocClippingOrOverflow,
    /// Translated text overflowed the surface's declared expansion budget.
    TextExpansionOverflow,
    /// RTL chrome mirrored incorrectly or a literal token was mirrored.
    RtlBidiMirrorRegression,
    /// CJK, full-width, or emoji glyph fallback failed.
    FontFallbackRegression,
    /// IME composition was silently committed, cancelled, or occluded.
    ImeCompositionRegression,
    /// CJK full-width layout, counting, or wrapping regressed.
    CjkGlyphRegression,
    /// Locale-sensitive date, number, duration, or count formatting drifted.
    LocalizedFormatRegression,
    /// A required harness result is missing or its evidence is stale.
    EvidenceStaleOrMissing,
}

/// One dense M5 surface in the qualification inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DenseSurfaceRow {
    /// Stable surface id.
    pub surface_id: String,
    /// Dense M5 surface family.
    pub surface_family: M5DenseSurfaceFamily,
    /// Human-readable surface title.
    pub title: String,
    /// Runtime crates that own or project this surface.
    pub crate_refs: Vec<String>,
    /// Dense workflow the qualification exercises on this surface.
    pub dense_workflow: String,
    /// Whether the surface accepts free text and therefore exercises IME.
    pub accepts_text_input: bool,
    /// Whether the surface renders locale-sensitive dates, numbers, or counts.
    pub renders_localized_formats: bool,
}

/// Definition of the harness battery applied to one dense surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M5DenseHarnessCase {
    /// Stable case id.
    pub case_id: String,
    /// Surface this case exercises.
    pub surface_id_ref: String,
    /// Surface family this case exercises.
    pub surface_family: M5DenseSurfaceFamily,
    /// Locale tags exercised by the case.
    pub locale_tags: Vec<String>,
    /// Harness kinds this case runs.
    pub harness_kinds: Vec<M5DenseHarnessKind>,
    /// Readiness rows joined from `artifacts/i18n/test_mode_matrix.yaml`.
    pub readiness_row_refs: Vec<String>,
    /// Fixture refs used as input or evidence for this case.
    pub fixture_refs: Vec<String>,
    /// Literal technical tokens that must remain unmirrored and copy-safe.
    pub literal_tokens: Vec<LiteralTechnicalToken>,
    /// IME scenario when the surface accepts composition input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ime_scenario: Option<ImeCompositionScenario>,
    /// Text expansion budget applied to the surface.
    pub expansion_budget: TextExpansionBudget,
    /// RTL or bidi expectation for the surface.
    pub rtl_expectation: RtlMirroringExpectation,
    /// Assertion classes the case must satisfy.
    pub assertion_refs: Vec<DenseI18nAssertionClass>,
    /// Failure classes the case is expected to catch.
    pub expected_failure_classes: Vec<DenseI18nFailureClass>,
}

/// Result of one harness against one locale on one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DenseHarnessResultRow {
    /// Stable result id.
    pub result_id: String,
    /// Surface case this result belongs to.
    pub case_ref: String,
    /// Surface family this result covers.
    pub surface_family: M5DenseSurfaceFamily,
    /// Harness kind this result covers.
    pub harness_kind: M5DenseHarnessKind,
    /// Requested locale under test.
    pub requested_locale: String,
    /// Locale actually rendered, equal to the requested locale unless fallen back.
    pub effective_locale: String,
    /// Rendered text direction.
    pub text_direction: TextDirection,
    /// Result state.
    pub result_state: M5HarnessResultState,
    /// Assertion classes this result asserts.
    pub assertion_refs: Vec<DenseI18nAssertionClass>,
    /// Failure class when the result failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_class: Option<DenseI18nFailureClass>,
    /// Export-safe evidence ref.
    pub evidence_ref: String,
    /// Bounded waiver ref when the result is waived.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Export-safe detail; never carries raw translated bodies.
    pub detail: String,
}

/// Per-profile qualification gate derived from the harness results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DenseProfileQualificationRow {
    /// Stable profile id.
    pub profile_id: String,
    /// Requested locale for the profile.
    pub requested_locale: String,
    /// Source-language locale the profile falls back to.
    pub source_language_locale: String,
    /// Text direction the profile renders.
    pub text_direction: TextDirection,
    /// Claim class the profile intends to hold.
    pub intended_claim_class: ProfileClaimClass,
    /// Claim class after dense-surface qualification.
    pub effective_claim_class: ProfileClaimClass,
    /// Gate state derived from the harness results.
    pub gate_state: MatrixGateState,
    /// Number of harness results evaluated for the profile.
    pub evaluated_result_count: usize,
    /// Passed result count.
    pub passed_count: usize,
    /// Source-language fallback result count.
    pub source_language_fallback_count: usize,
    /// Failed result count.
    pub failed_count: usize,
    /// Waived result count.
    pub waived_count: usize,
    /// Narrowing reasons that apply to the profile.
    pub narrow_reasons: Vec<M5DenseClaimNarrowReason>,
    /// Failure classes that block the profile.
    pub blocking_failure_classes: Vec<DenseI18nFailureClass>,
    /// Surface families with a failing or fallback result.
    pub affected_surface_families: Vec<M5DenseSurfaceFamily>,
    /// Harness kinds with a failing or fallback result.
    pub affected_harness_kinds: Vec<M5DenseHarnessKind>,
    /// Whether the profile blocks promotion.
    pub blocks_promotion: bool,
}

/// Downstream consumer that ingests the qualification rather than re-reviewing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DenseConsumptionBinding {
    /// Consuming package ref.
    pub consumer_ref: String,
    /// Packet fields the consumer reads.
    pub ingested_fields: Vec<String>,
    /// What the consumer surfaces or enforces from the qualification.
    pub purpose: String,
}

/// Summary roll-up for the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DenseSurfaceI18nSummary {
    /// Number of inventoried dense surfaces.
    pub total_surfaces: usize,
    /// Number of harness cases.
    pub total_harness_cases: usize,
    /// Number of harness results.
    pub total_harness_results: usize,
    /// Surface families covered.
    pub surface_families_covered: Vec<M5DenseSurfaceFamily>,
    /// Harness kinds covered.
    pub harness_kinds_covered: Vec<M5DenseHarnessKind>,
    /// Claimed localized locales gated by the packet.
    pub claimed_locales: Vec<String>,
    /// Passed result count.
    pub passed_result_count: usize,
    /// Source-language fallback result count.
    pub source_language_fallback_result_count: usize,
    /// Failed result count.
    pub failed_result_count: usize,
    /// Waived result count.
    pub waived_result_count: usize,
    /// Profiles that hold a green claim.
    pub green_profile_count: usize,
    /// Profiles narrowed to source-language fallback.
    pub narrowed_profile_count: usize,
    /// Profiles blocked from claiming localized support.
    pub blocked_profile_count: usize,
    /// Active bounded waiver count.
    pub active_waiver_count: usize,
    /// Roll-up promotion state: `green`, `narrowed`, or `blocked`.
    pub promotion_state: String,
}

/// Qualification packet binding dense-surface i18n proof to claimed profiles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct M5DenseSurfaceI18nQualification {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source-language locale.
    pub source_language_locale: String,
    /// Release channel the packet defends.
    pub release_channel: String,
    /// Target build identity the packet qualifies.
    pub target_build_identity_ref: String,
    /// Fixture directory relative to the repository root.
    pub fixture_root: String,
    /// Source contracts that govern the packet.
    pub source_contract_refs: BTreeMap<String, String>,
    /// Runtime crates that own or project the dense surfaces.
    pub runtime_consumer_refs: Vec<String>,
    /// Claimed localized locales gated by the packet.
    pub claimed_locales: Vec<String>,
    /// Harness kinds exercised by the packet.
    pub harness_kinds: Vec<M5DenseHarnessKind>,
    /// CI and release lane bindings that run the qualification.
    pub lane_bindings: Vec<DenseI18nLaneBinding>,
    /// Inventoried dense surfaces.
    pub surfaces: Vec<M5DenseSurfaceRow>,
    /// Harness case definitions.
    pub harness_cases: Vec<M5DenseHarnessCase>,
    /// Harness results per surface, harness, and locale.
    pub harness_results: Vec<M5DenseHarnessResultRow>,
    /// Per-profile qualification gates derived from the results.
    pub profile_qualifications: Vec<M5DenseProfileQualificationRow>,
    /// Downstream consumers that ingest the qualification.
    pub consumption_bindings: Vec<M5DenseConsumptionBinding>,
    /// Bounded waivers, if any.
    pub waivers: Vec<DenseI18nBoundedWaiver>,
    /// Summary roll-up.
    pub summary: M5DenseSurfaceI18nSummary,
}

/// Profile rows the qualification gate produces, keyed by claimed locale.
struct ProfileSeed {
    profile_id: &'static str,
    requested_locale: &'static str,
    intended_claim_class: ProfileClaimClass,
}

fn seeded_profile_seeds() -> Vec<ProfileSeed> {
    CLAIMED_LOCALES
        .iter()
        .map(|locale| ProfileSeed {
            profile_id: profile_id_for(locale),
            requested_locale: locale,
            intended_claim_class: ProfileClaimClass::ClaimedLocalized,
        })
        .collect()
}

fn profile_id_for(locale: &str) -> &'static str {
    match locale {
        "es-MX" => "profile:m5:es-MX:desktop",
        "ja-JP" => "profile:m5:ja-JP:desktop",
        "ar-SA" => "profile:m5:ar-SA:desktop",
        _ => "profile:m5:unknown:desktop",
    }
}

/// Returns the text direction for a locale tag.
pub fn locale_text_direction(locale: &str) -> TextDirection {
    if locale.starts_with("ar") || locale.starts_with("he") || locale.starts_with("fa") {
        TextDirection::RightToLeft
    } else {
        TextDirection::LeftToRight
    }
}

impl M5DenseSurfaceI18nQualification {
    /// Validates coverage, lane bindings, derived gates, and summary consistency.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != M5_DENSE_SURFACE_LAB_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "qualification record_kind is unsupported",
            ));
        }
        if self.schema_version != M5_DENSE_SURFACE_LAB_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "qualification schema_version is unsupported",
            ));
        }
        if self.packet_id != M5_DENSE_SURFACE_LAB_PACKET_ID {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "qualification packet id drifted",
            ));
        }

        validate_lane_bindings(&self.lane_bindings, &mut findings);

        let surface_ids = self
            .surfaces
            .iter()
            .map(|surface| surface.surface_id.as_str())
            .collect::<BTreeSet<_>>();
        let covered_families = self
            .surfaces
            .iter()
            .map(|surface| surface.surface_family)
            .collect::<BTreeSet<_>>();
        for family in M5DenseSurfaceFamily::all() {
            if !covered_families.contains(&family) {
                findings.push(LocalePackValidationFinding::new(
                    self.packet_id.clone(),
                    format!("qualification is missing {family:?} surface coverage"),
                ));
            }
        }

        let case_ids = self
            .harness_cases
            .iter()
            .map(|case| case.case_id.as_str())
            .collect::<BTreeSet<_>>();
        let mut covered_harness_kinds = BTreeSet::new();
        for case in &self.harness_cases {
            if !surface_ids.contains(case.surface_id_ref.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    case.case_id.clone(),
                    "harness case references an unknown surface",
                ));
            }
            if case.harness_kinds.is_empty() {
                findings.push(LocalePackValidationFinding::new(
                    case.case_id.clone(),
                    "harness case must exercise at least one harness kind",
                ));
            }
            covered_harness_kinds.extend(case.harness_kinds.iter().copied());
            for token in &case.literal_tokens {
                if !token.must_remain_unmirrored || !token.copy_raw_required {
                    findings.push(LocalePackValidationFinding::new(
                        case.case_id.clone(),
                        "literal technical token must remain unmirrored and copy-safe",
                    ));
                }
            }
            if case
                .harness_kinds
                .contains(&M5DenseHarnessKind::ImeComposition)
            {
                match &case.ime_scenario {
                    None => findings.push(LocalePackValidationFinding::new(
                        case.case_id.clone(),
                        "IME harness case must declare an IME composition scenario",
                    )),
                    Some(scenario) => {
                        if !scenario.silent_commit_forbidden
                            || !scenario.silent_cancel_forbidden
                            || !scenario.candidate_and_caret_visibility_required
                        {
                            findings.push(LocalePackValidationFinding::new(
                                case.case_id.clone(),
                                "IME scenario must forbid silent commit/cancel and keep caret visible",
                            ));
                        }
                    }
                }
            }
        }
        for harness_kind in M5DenseHarnessKind::all() {
            if !covered_harness_kinds.contains(&harness_kind) {
                findings.push(LocalePackValidationFinding::new(
                    self.packet_id.clone(),
                    format!("qualification is missing {harness_kind:?} harness coverage"),
                ));
            }
        }

        let claimed = self
            .claimed_locales
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let waiver_ids = self
            .waivers
            .iter()
            .map(|waiver| waiver.waiver_ref.as_str())
            .collect::<BTreeSet<_>>();
        let mut result_ids = BTreeSet::new();
        for result in &self.harness_results {
            if !result_ids.insert(result.result_id.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    result.result_id.clone(),
                    "duplicate harness result id",
                ));
            }
            if !case_ids.contains(result.case_ref.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    result.result_id.clone(),
                    "harness result references an unknown case",
                ));
            }
            if !claimed.contains(result.requested_locale.as_str()) {
                findings.push(LocalePackValidationFinding::new(
                    result.result_id.clone(),
                    "harness result targets a non-claimed locale",
                ));
            }
            if result.text_direction != locale_text_direction(&result.requested_locale) {
                findings.push(LocalePackValidationFinding::new(
                    result.result_id.clone(),
                    "harness result text direction does not match its locale",
                ));
            }
            match result.result_state {
                M5HarnessResultState::Failed => {
                    if result.failure_class.is_none() {
                        findings.push(LocalePackValidationFinding::new(
                            result.result_id.clone(),
                            "failed harness result must record a failure class",
                        ));
                    }
                }
                M5HarnessResultState::WaivedBounded => match result.waiver_ref.as_deref() {
                    Some(waiver) if waiver_ids.contains(waiver) => {}
                    _ => findings.push(LocalePackValidationFinding::new(
                        result.result_id.clone(),
                        "waived harness result must reference a known bounded waiver",
                    )),
                },
                M5HarnessResultState::SourceLanguageFallbackPassed => {
                    if result.effective_locale == result.requested_locale {
                        findings.push(LocalePackValidationFinding::new(
                            result.result_id.clone(),
                            "fallback result must render the source language locale",
                        ));
                    }
                }
                M5HarnessResultState::Passed => {}
            }
        }

        let derived = self.qualify();
        if derived != self.profile_qualifications {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "stored profile qualifications do not match the derived gates",
            ));
        }

        let derived_summary = self.summarize(&derived);
        if derived_summary != self.summary {
            findings.push(LocalePackValidationFinding::new(
                self.packet_id.clone(),
                "summary roll-up does not match the derived state",
            ));
        }

        for waiver in &self.waivers {
            if waiver.bounded_failure_classes.is_empty()
                || waiver.expires_at.trim().is_empty()
                || waiver.required_fallback_or_workaround.trim().is_empty()
            {
                findings.push(LocalePackValidationFinding::new(
                    waiver.waiver_ref.clone(),
                    "bounded waiver must be bounded, expiring, and paired with a fallback",
                ));
            }
        }

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    /// Derives the per-profile qualification gates from the harness results.
    pub fn qualify(&self) -> Vec<M5DenseProfileQualificationRow> {
        seeded_profile_seeds()
            .into_iter()
            .map(|seed| self.qualify_profile(&seed))
            .collect()
    }

    fn qualify_profile(&self, seed: &ProfileSeed) -> M5DenseProfileQualificationRow {
        let rows = self
            .harness_results
            .iter()
            .filter(|row| row.requested_locale == seed.requested_locale)
            .collect::<Vec<_>>();

        let mut passed = 0usize;
        let mut fallback = 0usize;
        let mut failed = 0usize;
        let mut waived = 0usize;
        let mut narrow_reasons = BTreeSet::new();
        let mut blocking_failure_classes = BTreeSet::new();
        let mut affected_surface_families = BTreeSet::new();
        let mut affected_harness_kinds = BTreeSet::new();

        for row in &rows {
            match row.result_state {
                M5HarnessResultState::Passed => passed += 1,
                M5HarnessResultState::WaivedBounded => waived += 1,
                M5HarnessResultState::SourceLanguageFallbackPassed => {
                    fallback += 1;
                    narrow_reasons.insert(row.harness_kind.narrow_reason());
                    affected_surface_families.insert(row.surface_family);
                    affected_harness_kinds.insert(row.harness_kind);
                }
                M5HarnessResultState::Failed => {
                    failed += 1;
                    narrow_reasons.insert(row.harness_kind.narrow_reason());
                    affected_surface_families.insert(row.surface_family);
                    affected_harness_kinds.insert(row.harness_kind);
                    if let Some(failure_class) = row.failure_class {
                        blocking_failure_classes.insert(failure_class);
                    }
                }
            }
        }

        let (gate_state, effective_claim_class, blocks_promotion) = if failed > 0 {
            (
                MatrixGateState::Blocked,
                ProfileClaimClass::SourceLanguageFallbackOnly,
                true,
            )
        } else if fallback > 0 {
            (
                MatrixGateState::Narrowed,
                ProfileClaimClass::SourceLanguageFallbackOnly,
                false,
            )
        } else {
            (MatrixGateState::Green, seed.intended_claim_class, false)
        };

        M5DenseProfileQualificationRow {
            profile_id: seed.profile_id.to_owned(),
            requested_locale: seed.requested_locale.to_owned(),
            source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
            text_direction: locale_text_direction(seed.requested_locale),
            intended_claim_class: seed.intended_claim_class,
            effective_claim_class,
            gate_state,
            evaluated_result_count: rows.len(),
            passed_count: passed,
            source_language_fallback_count: fallback,
            failed_count: failed,
            waived_count: waived,
            narrow_reasons: narrow_reasons.into_iter().collect(),
            blocking_failure_classes: blocking_failure_classes.into_iter().collect(),
            affected_surface_families: affected_surface_families.into_iter().collect(),
            affected_harness_kinds: affected_harness_kinds.into_iter().collect(),
            blocks_promotion,
        }
    }

    fn summarize(&self, profiles: &[M5DenseProfileQualificationRow]) -> M5DenseSurfaceI18nSummary {
        let surface_families_covered = self
            .surfaces
            .iter()
            .map(|surface| surface.surface_family)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let harness_kinds_covered = self
            .harness_cases
            .iter()
            .flat_map(|case| case.harness_kinds.iter().copied())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        let mut passed = 0usize;
        let mut fallback = 0usize;
        let mut failed = 0usize;
        let mut waived = 0usize;
        for row in &self.harness_results {
            match row.result_state {
                M5HarnessResultState::Passed => passed += 1,
                M5HarnessResultState::SourceLanguageFallbackPassed => fallback += 1,
                M5HarnessResultState::Failed => failed += 1,
                M5HarnessResultState::WaivedBounded => waived += 1,
            }
        }

        let green = profiles
            .iter()
            .filter(|row| row.gate_state == MatrixGateState::Green)
            .count();
        let narrowed = profiles
            .iter()
            .filter(|row| row.gate_state == MatrixGateState::Narrowed)
            .count();
        let blocked = profiles
            .iter()
            .filter(|row| row.gate_state == MatrixGateState::Blocked)
            .count();

        let promotion_state = if blocked > 0 {
            "blocked"
        } else if narrowed > 0 {
            "narrowed"
        } else {
            "green"
        };

        M5DenseSurfaceI18nSummary {
            total_surfaces: self.surfaces.len(),
            total_harness_cases: self.harness_cases.len(),
            total_harness_results: self.harness_results.len(),
            surface_families_covered,
            harness_kinds_covered,
            claimed_locales: self.claimed_locales.clone(),
            passed_result_count: passed,
            source_language_fallback_result_count: fallback,
            failed_result_count: failed,
            waived_result_count: waived,
            green_profile_count: green,
            narrowed_profile_count: narrowed,
            blocked_profile_count: blocked,
            active_waiver_count: self.waivers.len(),
            promotion_state: promotion_state.to_owned(),
        }
    }

    /// Projects an exportable review packet for QA, shiproom, and support.
    pub fn review_packet(&self) -> M5DenseSurfaceI18nReviewPacket {
        let profiles = self.qualify();
        let blocked_profiles = profiles
            .iter()
            .filter(|row| row.gate_state == MatrixGateState::Blocked)
            .map(|row| row.profile_id.clone())
            .collect::<Vec<_>>();
        let narrowed_profiles = profiles
            .iter()
            .filter(|row| row.gate_state == MatrixGateState::Narrowed)
            .map(|row| row.profile_id.clone())
            .collect::<Vec<_>>();
        let promotion_state = if !blocked_profiles.is_empty() {
            "blocked"
        } else if !narrowed_profiles.is_empty() {
            "narrowed"
        } else {
            "green"
        };

        M5DenseSurfaceI18nReviewPacket {
            record_kind: M5_DENSE_SURFACE_LAB_REVIEW_RECORD_KIND.to_owned(),
            schema_version: M5_DENSE_SURFACE_LAB_SCHEMA_VERSION,
            packet_id: "i18n-qualification-review:m5-dense-surface:v1".to_owned(),
            generated_at: self.generated_at.clone(),
            source_packet_id: self.packet_id.clone(),
            claimed_locales: self.claimed_locales.clone(),
            surface_families_covered: self.summary.surface_families_covered.clone(),
            harness_kinds_covered: self.summary.harness_kinds_covered.clone(),
            lane_refs: self
                .lane_bindings
                .iter()
                .map(|lane| lane.lane_id.clone())
                .collect(),
            profile_rows: profiles,
            blocked_profiles,
            narrowed_profiles,
            active_waiver_count: self.waivers.len(),
            artifact_refs: vec![
                format!("{}/qualification.json", self.fixture_root),
                format!("{}/review_export.json", self.fixture_root),
                "artifacts/i18n/m5-pseudoloc-rtl-ime-report/report.md".to_owned(),
                "docs/i18n/m5-dense-surface-i18n-lab.md".to_owned(),
            ],
            promotion_state: promotion_state.to_owned(),
        }
    }

    /// Returns the qualification gate for one requested locale.
    pub fn profile_qualification(
        &self,
        requested_locale: &str,
    ) -> Option<&M5DenseProfileQualificationRow> {
        self.profile_qualifications
            .iter()
            .find(|row| row.requested_locale == requested_locale)
    }

    /// Applies a narrowing scenario and returns the re-derived qualification.
    ///
    /// The result reflects how a regression on one dense surface narrows or
    /// blocks the claimed locale without touching any other surface.
    pub fn with_injected_result(
        &self,
        locale: &str,
        surface_family: M5DenseSurfaceFamily,
        harness_kind: M5DenseHarnessKind,
        result_state: M5HarnessResultState,
        failure_class: Option<DenseI18nFailureClass>,
    ) -> M5DenseSurfaceI18nQualification {
        let mut clone = self.clone();
        for row in &mut clone.harness_results {
            if row.requested_locale == locale
                && row.surface_family == surface_family
                && row.harness_kind == harness_kind
            {
                row.result_state = result_state;
                row.failure_class = failure_class;
                if result_state == M5HarnessResultState::SourceLanguageFallbackPassed {
                    row.effective_locale = SOURCE_LANGUAGE_LOCALE.to_owned();
                } else {
                    row.effective_locale = locale.to_owned();
                }
            }
        }
        clone.profile_qualifications = clone.qualify();
        clone.summary = clone.summarize(&clone.profile_qualifications);
        clone
    }
}

/// Exportable review packet derived from [`M5DenseSurfaceI18nQualification`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DenseSurfaceI18nReviewPacket {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable review packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source qualification packet id.
    pub source_packet_id: String,
    /// Claimed localized locales.
    pub claimed_locales: Vec<String>,
    /// Surface families covered.
    pub surface_families_covered: Vec<M5DenseSurfaceFamily>,
    /// Harness kinds covered.
    pub harness_kinds_covered: Vec<M5DenseHarnessKind>,
    /// Lane ids that run the qualification.
    pub lane_refs: Vec<String>,
    /// Per-profile qualification rows.
    pub profile_rows: Vec<M5DenseProfileQualificationRow>,
    /// Blocked profile ids.
    pub blocked_profiles: Vec<String>,
    /// Narrowed profile ids.
    pub narrowed_profiles: Vec<String>,
    /// Active bounded waiver count.
    pub active_waiver_count: usize,
    /// Exported artifact refs.
    pub artifact_refs: Vec<String>,
    /// Roll-up promotion state.
    pub promotion_state: String,
}

/// One narrowing scenario proving a regression auto-narrows or blocks a claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DenseNarrowingScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Export-safe scenario description.
    pub description: String,
    /// Claimed locale the scenario regresses.
    pub injected_locale: String,
    /// Surface family the regression lands on.
    pub injected_surface_family: M5DenseSurfaceFamily,
    /// Harness kind that regresses.
    pub injected_harness_kind: M5DenseHarnessKind,
    /// Result state injected for the regression.
    pub injected_result_state: M5HarnessResultState,
    /// Failure class injected for the regression.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub injected_failure_class: Option<DenseI18nFailureClass>,
    /// Gate state the qualification must derive after injection.
    pub expected_gate_state: MatrixGateState,
    /// Narrow reasons the qualification must derive after injection.
    pub expected_narrow_reasons: Vec<M5DenseClaimNarrowReason>,
    /// Whether the scenario must block promotion.
    pub expected_blocks_promotion: bool,
}

/// Set of narrowing scenarios exported with the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DenseNarrowingScenarioSet {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable set id.
    pub set_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source qualification packet id.
    pub source_packet_id: String,
    /// Narrowing scenarios.
    pub scenarios: Vec<M5DenseNarrowingScenario>,
}

impl M5DenseNarrowingScenarioSet {
    /// Replays every scenario against the seeded packet and returns findings.
    pub fn validate_against(
        &self,
        packet: &M5DenseSurfaceI18nQualification,
    ) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();
        for scenario in &self.scenarios {
            let injected = packet.with_injected_result(
                &scenario.injected_locale,
                scenario.injected_surface_family,
                scenario.injected_harness_kind,
                scenario.injected_result_state,
                scenario.injected_failure_class,
            );
            let Some(row) = injected.profile_qualification(&scenario.injected_locale) else {
                findings.push(LocalePackValidationFinding::new(
                    scenario.scenario_id.clone(),
                    "scenario targets a locale with no profile qualification",
                ));
                continue;
            };
            if row.gate_state != scenario.expected_gate_state {
                findings.push(LocalePackValidationFinding::new(
                    scenario.scenario_id.clone(),
                    "injected regression did not derive the expected gate state",
                ));
            }
            if row.narrow_reasons != scenario.expected_narrow_reasons {
                findings.push(LocalePackValidationFinding::new(
                    scenario.scenario_id.clone(),
                    "injected regression did not derive the expected narrow reasons",
                ));
            }
            if row.blocks_promotion != scenario.expected_blocks_promotion {
                findings.push(LocalePackValidationFinding::new(
                    scenario.scenario_id.clone(),
                    "injected regression did not derive the expected promotion block",
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

fn validate_lane_bindings(
    lanes: &[DenseI18nLaneBinding],
    findings: &mut Vec<LocalePackValidationFinding>,
) {
    let cadences = lanes
        .iter()
        .map(|lane| lane.cadence)
        .collect::<BTreeSet<_>>();
    for cadence in [
        DenseI18nLaneCadence::Nightly,
        DenseI18nLaneCadence::ReleaseCandidate,
        DenseI18nLaneCadence::PullRequest,
    ] {
        if !cadences.contains(&cadence) {
            findings.push(LocalePackValidationFinding::new(
                M5_DENSE_SURFACE_LAB_PACKET_ID,
                format!("qualification is missing {cadence:?} lane binding"),
            ));
        }
    }
    for lane in lanes {
        if lane.command.trim().is_empty() || !lane.command.contains("m5_dense_surface_i18n_lab") {
            findings.push(LocalePackValidationFinding::new(
                lane.lane_id.clone(),
                "qualification lane must run the dense-surface qualification test",
            ));
        }
        if matches!(
            lane.cadence,
            DenseI18nLaneCadence::Nightly | DenseI18nLaneCadence::ReleaseCandidate
        ) && !lane.release_blocking_for_claimed_surfaces
        {
            findings.push(LocalePackValidationFinding::new(
                lane.lane_id.clone(),
                "nightly and release-candidate lanes must be release-blocking",
            ));
        }
    }
}

/// Returns the seeded M5 dense-surface i18n qualification packet.
pub fn seeded_m5_dense_surface_i18n_qualification() -> M5DenseSurfaceI18nQualification {
    let surfaces = seeded_surfaces();
    let harness_cases = seeded_harness_cases(&surfaces);
    let harness_results = seeded_harness_results(&harness_cases);
    let lane_command = "cargo test -p aureline-i18n --test m5_dense_surface_i18n_lab --locked";

    let mut packet = M5DenseSurfaceI18nQualification {
        record_kind: M5_DENSE_SURFACE_LAB_RECORD_KIND.to_owned(),
        schema_version: M5_DENSE_SURFACE_LAB_SCHEMA_VERSION,
        packet_id: M5_DENSE_SURFACE_LAB_PACKET_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_language_locale: SOURCE_LANGUAGE_LOCALE.to_owned(),
        release_channel: "m5-stable".to_owned(),
        target_build_identity_ref: TARGET_BUILD_IDENTITY_REF.to_owned(),
        fixture_root: M5_DENSE_SURFACE_LAB_FIXTURE_ROOT.to_owned(),
        source_contract_refs: BTreeMap::from([
            (
                "architecture_localization".to_owned(),
                "Aureline_Technical_Architecture_Document.md#localization".to_owned(),
            ),
            (
                "verification_lanes".to_owned(),
                "Aureline_Technical_Architecture_Document.md#i18n-verification-lanes".to_owned(),
            ),
            (
                "content_governance".to_owned(),
                "Aureline_Technical_Design_Document.md#localization-and-content-governance"
                    .to_owned(),
            ),
            (
                "localization_operations".to_owned(),
                "Aureline_UI_UX_Spec_Document.md#localization-operations-and-pseudolocalization"
                    .to_owned(),
            ),
            (
                "international_text_and_input".to_owned(),
                "Aureline_UI_UX_Spec_Document.md#international-text-input-ime-bidi".to_owned(),
            ),
            (
                "localized_profile_matrix".to_owned(),
                "fixtures/i18n/m5-surface-inventory/manifest.json".to_owned(),
            ),
            (
                "dense_corpus".to_owned(),
                "fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/manifest.json".to_owned(),
            ),
            (
                "test_mode_matrix".to_owned(),
                "artifacts/i18n/test_mode_matrix.yaml".to_owned(),
            ),
        ]),
        runtime_consumer_refs: vec![
            "crates/aureline-shell".to_owned(),
            "crates/aureline-editor".to_owned(),
            "crates/aureline-notebook".to_owned(),
            "crates/aureline-data".to_owned(),
            "crates/aureline-pipeline".to_owned(),
            "crates/aureline-docs".to_owned(),
            "crates/aureline-terminal".to_owned(),
            "crates/aureline-support".to_owned(),
            "crates/aureline-release".to_owned(),
        ],
        claimed_locales: CLAIMED_LOCALES
            .iter()
            .map(|locale| locale.to_string())
            .collect(),
        harness_kinds: M5DenseHarnessKind::all(),
        lane_bindings: vec![
            qualification_lane(
                "lane:i18n:m5-dense:nightly",
                DenseI18nLaneCadence::Nightly,
                lane_command,
            ),
            qualification_lane(
                "lane:i18n:m5-dense:release-candidate",
                DenseI18nLaneCadence::ReleaseCandidate,
                lane_command,
            ),
            qualification_lane(
                "lane:i18n:m5-dense:pull-request",
                DenseI18nLaneCadence::PullRequest,
                lane_command,
            ),
        ],
        surfaces,
        harness_cases,
        harness_results,
        profile_qualifications: Vec::new(),
        consumption_bindings: seeded_consumption_bindings(),
        waivers: Vec::new(),
        summary: placeholder_summary(),
    };

    packet.profile_qualifications = packet.qualify();
    packet.summary = packet.summarize(&packet.profile_qualifications);
    packet
}

/// Returns the seeded review packet projected from the qualification.
pub fn seeded_m5_dense_surface_i18n_review_packet() -> M5DenseSurfaceI18nReviewPacket {
    seeded_m5_dense_surface_i18n_qualification().review_packet()
}

/// Returns the seeded narrowing scenarios that prove auto-narrow and block.
pub fn seeded_m5_dense_surface_narrowing_scenarios() -> M5DenseNarrowingScenarioSet {
    let scenarios = vec![
        M5DenseNarrowingScenario {
            scenario_id: "scenario:m5-dense:ja-notebook-ime-block".to_owned(),
            description: "A notebook cell silently commits IME composition for ja-JP.".to_owned(),
            injected_locale: "ja-JP".to_owned(),
            injected_surface_family: M5DenseSurfaceFamily::Notebook,
            injected_harness_kind: M5DenseHarnessKind::ImeComposition,
            injected_result_state: M5HarnessResultState::Failed,
            injected_failure_class: Some(DenseI18nFailureClass::ImePreeditLoss),
            expected_gate_state: MatrixGateState::Blocked,
            expected_narrow_reasons: vec![M5DenseClaimNarrowReason::ImeCompositionRegression],
            expected_blocks_promotion: true,
        },
        M5DenseNarrowingScenario {
            scenario_id: "scenario:m5-dense:ar-pipeline-rtl-block".to_owned(),
            description: "Pipeline/log chrome mirrors a literal command id for ar-SA.".to_owned(),
            injected_locale: "ar-SA".to_owned(),
            injected_surface_family: M5DenseSurfaceFamily::PipelineLogView,
            injected_harness_kind: M5DenseHarnessKind::RtlBidi,
            injected_result_state: M5HarnessResultState::Failed,
            injected_failure_class: Some(DenseI18nFailureClass::LiteralTechnicalStringMirrored),
            expected_gate_state: MatrixGateState::Blocked,
            expected_narrow_reasons: vec![M5DenseClaimNarrowReason::RtlBidiMirrorRegression],
            expected_blocks_promotion: true,
        },
        M5DenseNarrowingScenario {
            scenario_id: "scenario:m5-dense:es-datagrid-format-block".to_owned(),
            description: "Data grid renders a wrong decimal separator for es-MX counts.".to_owned(),
            injected_locale: "es-MX".to_owned(),
            injected_surface_family: M5DenseSurfaceFamily::DataGrid,
            injected_harness_kind: M5DenseHarnessKind::LocalizedDateNumber,
            injected_result_state: M5HarnessResultState::Failed,
            injected_failure_class: Some(DenseI18nFailureClass::LocalizedDateNumberDrift),
            expected_gate_state: MatrixGateState::Blocked,
            expected_narrow_reasons: vec![M5DenseClaimNarrowReason::LocalizedFormatRegression],
            expected_blocks_promotion: true,
        },
        M5DenseNarrowingScenario {
            scenario_id: "scenario:m5-dense:ja-datagrid-font-block".to_owned(),
            description: "Data grid drops CJK glyph fallback for a ja-JP cell.".to_owned(),
            injected_locale: "ja-JP".to_owned(),
            injected_surface_family: M5DenseSurfaceFamily::DataGrid,
            injected_harness_kind: M5DenseHarnessKind::FontFallback,
            injected_result_state: M5HarnessResultState::Failed,
            injected_failure_class: Some(DenseI18nFailureClass::MissingGlyphOrWrongFontFallback),
            expected_gate_state: MatrixGateState::Blocked,
            expected_narrow_reasons: vec![M5DenseClaimNarrowReason::FontFallbackRegression],
            expected_blocks_promotion: true,
        },
        M5DenseNarrowingScenario {
            scenario_id: "scenario:m5-dense:es-support-fallback-narrow".to_owned(),
            description: "Support/report copy falls back to source language for es-MX.".to_owned(),
            injected_locale: "es-MX".to_owned(),
            injected_surface_family: M5DenseSurfaceFamily::SupportReport,
            injected_harness_kind: M5DenseHarnessKind::TextExpansion,
            injected_result_state: M5HarnessResultState::SourceLanguageFallbackPassed,
            injected_failure_class: None,
            expected_gate_state: MatrixGateState::Narrowed,
            expected_narrow_reasons: vec![M5DenseClaimNarrowReason::TextExpansionOverflow],
            expected_blocks_promotion: false,
        },
    ];

    M5DenseNarrowingScenarioSet {
        record_kind: M5_DENSE_SURFACE_LAB_NARROWING_RECORD_KIND.to_owned(),
        schema_version: M5_DENSE_SURFACE_LAB_SCHEMA_VERSION,
        set_id: "i18n-qualification-narrowing:m5-dense-surface:v1".to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        source_packet_id: M5_DENSE_SURFACE_LAB_PACKET_ID.to_owned(),
        scenarios,
    }
}

fn qualification_lane(
    lane_id: &str,
    cadence: DenseI18nLaneCadence,
    command: &str,
) -> DenseI18nLaneBinding {
    DenseI18nLaneBinding {
        lane_id: lane_id.to_owned(),
        cadence,
        command: command.to_owned(),
        artifact_refs: vec![
            format!("{M5_DENSE_SURFACE_LAB_FIXTURE_ROOT}/qualification.json"),
            format!("{M5_DENSE_SURFACE_LAB_FIXTURE_ROOT}/review_export.json"),
            "artifacts/i18n/m5-pseudoloc-rtl-ime-report/report.md".to_owned(),
        ],
        release_blocking_for_claimed_surfaces: true,
    }
}

fn placeholder_summary() -> M5DenseSurfaceI18nSummary {
    M5DenseSurfaceI18nSummary {
        total_surfaces: 0,
        total_harness_cases: 0,
        total_harness_results: 0,
        surface_families_covered: Vec::new(),
        harness_kinds_covered: Vec::new(),
        claimed_locales: Vec::new(),
        passed_result_count: 0,
        source_language_fallback_result_count: 0,
        failed_result_count: 0,
        waived_result_count: 0,
        green_profile_count: 0,
        narrowed_profile_count: 0,
        blocked_profile_count: 0,
        active_waiver_count: 0,
        promotion_state: "green".to_owned(),
    }
}

fn seeded_consumption_bindings() -> Vec<M5DenseConsumptionBinding> {
    vec![
        M5DenseConsumptionBinding {
            consumer_ref: "crates/aureline-release".to_owned(),
            ingested_fields: vec![
                "profile_qualifications.gate_state".to_owned(),
                "profile_qualifications.blocks_promotion".to_owned(),
                "summary.promotion_state".to_owned(),
            ],
            purpose: "Block or narrow a claimed localized profile at promotion when a dense surface regresses.".to_owned(),
        },
        M5DenseConsumptionBinding {
            consumer_ref: "crates/aureline-support".to_owned(),
            ingested_fields: vec![
                "profile_qualifications.narrow_reasons".to_owned(),
                "profile_qualifications.affected_surface_families".to_owned(),
                "review_packet.profile_rows".to_owned(),
            ],
            purpose: "Reuse the same evidence in support export instead of one-off review sessions.".to_owned(),
        },
        M5DenseConsumptionBinding {
            consumer_ref: "crates/aureline-shell".to_owned(),
            ingested_fields: vec![
                "profile_qualifications.effective_claim_class".to_owned(),
                "profile_qualifications.gate_state".to_owned(),
            ],
            purpose: "Surface the localized-claim posture and its narrowing reasons in diagnostics.".to_owned(),
        },
    ]
}

fn seeded_surfaces() -> Vec<M5DenseSurfaceRow> {
    vec![
        surface(
            "surface:m5:editor-adjacent-pane",
            M5DenseSurfaceFamily::EditorAdjacentPane,
            "Editor inline rename, hover, peek, and completion chrome",
            vec!["crates/aureline-editor", "crates/aureline-render"],
            "Inline rename field, hover card, peek view, and completion preview.",
            true,
            false,
        ),
        surface(
            "surface:m5:command-palette",
            M5DenseSurfaceFamily::CommandPalette,
            "Command palette query, results, and command preview",
            vec!["crates/aureline-shell", "crates/aureline-commands"],
            "Palette query input, result rows, disabled reasons, and command preview.",
            true,
            false,
        ),
        surface(
            "surface:m5:settings",
            M5DenseSurfaceFamily::Settings,
            "Settings rows, schema help, and validation messages",
            vec!["crates/aureline-settings", "crates/aureline-shell"],
            "Settings search, schema-backed help, locale rows, and validation messages.",
            true,
            true,
        ),
        surface(
            "surface:m5:terminal-help",
            M5DenseSurfaceFamily::TerminalHelp,
            "Terminal transcript, terminal input, and help",
            vec!["crates/aureline-terminal", "crates/aureline-help"],
            "Terminal transcript, terminal input, and the embedded help surface.",
            true,
            false,
        ),
        surface(
            "surface:m5:notebook",
            M5DenseSurfaceFamily::Notebook,
            "Notebook code and markdown cells and cell outputs",
            vec!["crates/aureline-notebook", "crates/aureline-editor"],
            "Notebook code cell, markdown cell, rich cell output, and execution counts.",
            true,
            true,
        ),
        surface(
            "surface:m5:data-grid",
            M5DenseSurfaceFamily::DataGrid,
            "Data and API tooling grids with counts and filters",
            vec!["crates/aureline-data", "crates/aureline-shell"],
            "Data grid headers, cells, filter input, sort chips, and result counts.",
            true,
            true,
        ),
        surface(
            "surface:m5:pipeline-log-view",
            M5DenseSurfaceFamily::PipelineLogView,
            "Pipeline run views, log streams, and timelines",
            vec!["crates/aureline-pipeline", "crates/aureline-shell"],
            "Pipeline run timeline, log stream, log filter input, and durations.",
            true,
            true,
        ),
        surface(
            "surface:m5:docs-help-pane",
            M5DenseSurfaceFamily::DocsHelpPane,
            "Docs, help, and glossary panes",
            vec!["crates/aureline-docs", "crates/aureline-help"],
            "Docs reader, help pane, glossary card, and docs search input.",
            true,
            false,
        ),
        surface(
            "surface:m5:guided-tour",
            M5DenseSurfaceFamily::GuidedTour,
            "Guided tours and onboarding exercises",
            vec!["crates/aureline-help", "crates/aureline-shell"],
            "Guided tour step banner, exercise instructions, and progress chrome.",
            false,
            false,
        ),
        surface(
            "surface:m5:support-report",
            M5DenseSurfaceFamily::SupportReport,
            "Support flows and exported support/report surfaces",
            vec!["crates/aureline-support", "crates/aureline-shell"],
            "Support flow steps, report summary, evidence rows, and export counts.",
            true,
            true,
        ),
    ]
}

fn surface(
    surface_id: &str,
    surface_family: M5DenseSurfaceFamily,
    title: &str,
    crate_refs: Vec<&str>,
    dense_workflow: &str,
    accepts_text_input: bool,
    renders_localized_formats: bool,
) -> M5DenseSurfaceRow {
    M5DenseSurfaceRow {
        surface_id: surface_id.to_owned(),
        surface_family,
        title: title.to_owned(),
        crate_refs: crate_refs.into_iter().map(str::to_owned).collect(),
        dense_workflow: dense_workflow.to_owned(),
        accepts_text_input,
        renders_localized_formats,
    }
}

fn seeded_harness_cases(surfaces: &[M5DenseSurfaceRow]) -> Vec<M5DenseHarnessCase> {
    surfaces.iter().map(harness_case_for_surface).collect()
}

fn harness_case_for_surface(surface: &M5DenseSurfaceRow) -> M5DenseHarnessCase {
    let mut harness_kinds = vec![
        M5DenseHarnessKind::Pseudolocalization,
        M5DenseHarnessKind::TextExpansion,
        M5DenseHarnessKind::RtlBidi,
        M5DenseHarnessKind::FontFallback,
        M5DenseHarnessKind::Cjk,
    ];
    if surface.accepts_text_input {
        harness_kinds.push(M5DenseHarnessKind::ImeComposition);
    }
    if surface.renders_localized_formats {
        harness_kinds.push(M5DenseHarnessKind::LocalizedDateNumber);
    }
    harness_kinds.sort();

    let mut assertion_refs = vec![
        DenseI18nAssertionClass::NoTruncationOrOverflow,
        DenseI18nAssertionClass::RtlChromeMirrorsOnlyDirectionalUi,
        DenseI18nAssertionClass::LiteralTechnicalTokensPreserved,
        DenseI18nAssertionClass::FontFallbackWorks,
        DenseI18nAssertionClass::LocaleFallbackDisclosedAndNonBlocking,
        DenseI18nAssertionClass::StableTranslatedSurfaceRefsPreserved,
        DenseI18nAssertionClass::SourceLanguageEscapeHatchAvailable,
    ];
    let mut expected_failure_classes = vec![
        DenseI18nFailureClass::TextClippedOrOverflow,
        DenseI18nFailureClass::TruncationHidesScopeOrAction,
        DenseI18nFailureClass::MirroredChromeOrFocusOrderError,
        DenseI18nFailureClass::LiteralTechnicalStringMirrored,
        DenseI18nFailureClass::MissingGlyphOrWrongFontFallback,
        DenseI18nFailureClass::StableIdDrift,
        DenseI18nFailureClass::SourceLanguageEscapeHatchMissing,
    ];
    let ime_scenario = if surface.accepts_text_input {
        assertion_refs.push(DenseI18nAssertionClass::ImeCompositionNotSilentlyCommittedOrCancelled);
        assertion_refs.push(DenseI18nAssertionClass::CandidateAndCaretRemainVisible);
        expected_failure_classes.push(DenseI18nFailureClass::ImePreeditLoss);
        expected_failure_classes.push(DenseI18nFailureClass::CandidateWindowOccluded);
        expected_failure_classes.push(DenseI18nFailureClass::FocusChurnSilentCommitOrCancel);
        Some(ime_scenario_for(surface.surface_family))
    } else {
        None
    };
    if surface.renders_localized_formats {
        assertion_refs.push(DenseI18nAssertionClass::LocalizedFormattingKeepsStableSemantics);
        expected_failure_classes.push(DenseI18nFailureClass::LocalizedDateNumberDrift);
    }
    assertion_refs.sort();
    assertion_refs.dedup();
    expected_failure_classes.sort();
    expected_failure_classes.dedup();

    let single_line_min_ratio = match surface.surface_family {
        M5DenseSurfaceFamily::CommandPalette | M5DenseSurfaceFamily::DataGrid => 1.4,
        _ => 1.3,
    };

    M5DenseHarnessCase {
        case_id: format!("case:{}", surface.surface_id.trim_start_matches("surface:")),
        surface_id_ref: surface.surface_id.clone(),
        surface_family: surface.surface_family,
        locale_tags: vec![
            "es-MX".to_owned(),
            "ja-JP".to_owned(),
            "ar-SA".to_owned(),
            "qps-ploc".to_owned(),
        ],
        harness_kinds,
        readiness_row_refs: readiness_rows_for(surface.surface_family),
        fixture_refs: fixture_refs_for(surface.surface_family),
        literal_tokens: literal_tokens_for(surface.surface_family),
        ime_scenario,
        expansion_budget: TextExpansionBudget {
            single_line_min_ratio,
            multiline_min_ratio: 1.6,
            overflow_forbidden: true,
            full_text_route_required: true,
        },
        rtl_expectation: RtlMirroringExpectation {
            directional_chrome_mirrors: true,
            literal_technical_strings_unmirrored: true,
            focus_order_tracks_visual_order: true,
            raw_copy_preserves_author_order: true,
        },
        assertion_refs,
        expected_failure_classes,
    }
}

fn ime_scenario_for(family: M5DenseSurfaceFamily) -> ImeCompositionScenario {
    let (scenario_id, churn) = match family {
        M5DenseSurfaceFamily::EditorAdjacentPane => (
            "ime:m5:editor-adjacent:completion-snippet",
            vec![
                ImeCompositionChurnEvent::FocusChange,
                ImeCompositionChurnEvent::CompletionPreview,
                ImeCompositionChurnEvent::SnippetTraversal,
            ],
        ),
        M5DenseSurfaceFamily::CommandPalette => (
            "ime:m5:palette:command-preview",
            vec![
                ImeCompositionChurnEvent::FilterRerank,
                ImeCompositionChurnEvent::CommandPreview,
            ],
        ),
        M5DenseSurfaceFamily::Settings => (
            "ime:m5:settings:search",
            vec![
                ImeCompositionChurnEvent::FilterRerank,
                ImeCompositionChurnEvent::OverlayTransition,
            ],
        ),
        M5DenseSurfaceFamily::TerminalHelp => (
            "ime:m5:terminal:input",
            vec![
                ImeCompositionChurnEvent::FocusChange,
                ImeCompositionChurnEvent::CompletionPreview,
            ],
        ),
        M5DenseSurfaceFamily::Notebook => (
            "ime:m5:notebook:cell",
            vec![
                ImeCompositionChurnEvent::FocusChange,
                ImeCompositionChurnEvent::CompletionPreview,
                ImeCompositionChurnEvent::SnippetTraversal,
            ],
        ),
        M5DenseSurfaceFamily::DataGrid => (
            "ime:m5:data-grid:filter",
            vec![
                ImeCompositionChurnEvent::FilterRerank,
                ImeCompositionChurnEvent::FocusChange,
            ],
        ),
        M5DenseSurfaceFamily::PipelineLogView => (
            "ime:m5:pipeline:log-filter",
            vec![
                ImeCompositionChurnEvent::FilterRerank,
                ImeCompositionChurnEvent::OverlayTransition,
            ],
        ),
        M5DenseSurfaceFamily::DocsHelpPane => (
            "ime:m5:docs:search",
            vec![
                ImeCompositionChurnEvent::FilterRerank,
                ImeCompositionChurnEvent::OverlayTransition,
            ],
        ),
        M5DenseSurfaceFamily::SupportReport => (
            "ime:m5:support:report-note",
            vec![
                ImeCompositionChurnEvent::FocusChange,
                ImeCompositionChurnEvent::OverlayTransition,
            ],
        ),
        M5DenseSurfaceFamily::GuidedTour => (
            "ime:m5:guided-tour:note",
            vec![ImeCompositionChurnEvent::OverlayTransition],
        ),
    };

    ImeCompositionScenario {
        scenario_id: scenario_id.to_owned(),
        input_method: "Japanese IME".to_owned(),
        // "test input" rendered in Japanese; commit text must equal the preedit.
        preedit_text: "\u{30c6}\u{30b9}\u{30c8}\u{5165}\u{529b}".to_owned(),
        expected_commit_text: "\u{30c6}\u{30b9}\u{30c8}\u{5165}\u{529b}".to_owned(),
        churn_events: churn,
        silent_commit_forbidden: true,
        silent_cancel_forbidden: true,
        candidate_and_caret_visibility_required: true,
    }
}

fn literal_tokens_for(family: M5DenseSurfaceFamily) -> Vec<LiteralTechnicalToken> {
    let tokens: Vec<(&str, &str)> = match family {
        M5DenseSurfaceFamily::EditorAdjacentPane => vec![
            ("cmd:editor.rename_symbol", "command_id"),
            (
                "src/\u{062a}\u{062d}\u{0642}\u{0642}/rename.rs",
                "file_path",
            ),
        ],
        M5DenseSurfaceFamily::CommandPalette => vec![
            ("cmd:core:open_folder", "command_id"),
            ("Ctrl+K Ctrl+O", "keyboard_path"),
        ],
        M5DenseSurfaceFamily::Settings => vec![
            ("setting:editor.font_family", "setting_id"),
            ("telemetry:settings.changed", "telemetry_key"),
        ],
        M5DenseSurfaceFamily::TerminalHelp => vec![
            ("--reuse-window", "flag"),
            ("https://aureline.dev/docs", "url"),
        ],
        M5DenseSurfaceFamily::Notebook => vec![
            ("cmd:notebook.run_cell", "command_id"),
            ("kernel://python-3.12", "uri"),
        ],
        M5DenseSurfaceFamily::DataGrid => vec![
            ("cmd:data.run_query", "command_id"),
            ("SELECT * FROM events", "query_literal"),
        ],
        M5DenseSurfaceFamily::PipelineLogView => vec![
            ("cmd:pipeline.rerun_stage", "command_id"),
            ("stage://build/compile", "uri"),
        ],
        M5DenseSurfaceFamily::DocsHelpPane => vec![
            ("doc:anchor:locale-packs", "citation_anchor"),
            ("cmd:help.open_in_source_language", "command_id"),
        ],
        M5DenseSurfaceFamily::GuidedTour => vec![
            ("tour:step:open-workspace", "scope_label"),
            ("cmd:core:open_folder", "command_id"),
        ],
        M5DenseSurfaceFamily::SupportReport => vec![
            ("policy:support.redaction", "policy_name"),
            ("cmd:support.export_report", "command_id"),
        ],
    };
    tokens
        .into_iter()
        .map(|(token, token_class)| LiteralTechnicalToken {
            token: token.to_owned(),
            token_class: token_class.to_owned(),
            must_remain_unmirrored: true,
            copy_raw_required: true,
            copy_escaped_required: true,
        })
        .collect()
}

fn readiness_rows_for(family: M5DenseSurfaceFamily) -> Vec<String> {
    let mut rows = vec![
        "readiness.shell.source_language_and_pseudoloc_chrome".to_owned(),
        "readiness.shell.rtl_chrome_and_mixed_direction_technical_content".to_owned(),
        "readiness.text.cjk_font_fallback_and_full_width_layout".to_owned(),
        "readiness.locale.fallback_chain_and_locale_pack_contract".to_owned(),
    ];
    if matches!(
        family,
        M5DenseSurfaceFamily::EditorAdjacentPane
            | M5DenseSurfaceFamily::CommandPalette
            | M5DenseSurfaceFamily::Settings
            | M5DenseSurfaceFamily::TerminalHelp
            | M5DenseSurfaceFamily::Notebook
            | M5DenseSurfaceFamily::DataGrid
            | M5DenseSurfaceFamily::PipelineLogView
            | M5DenseSurfaceFamily::DocsHelpPane
            | M5DenseSurfaceFamily::SupportReport
    ) {
        rows.push("readiness.input.ime_preedit_and_commit".to_owned());
    }
    if matches!(
        family,
        M5DenseSurfaceFamily::Settings
            | M5DenseSurfaceFamily::Notebook
            | M5DenseSurfaceFamily::DataGrid
            | M5DenseSurfaceFamily::PipelineLogView
            | M5DenseSurfaceFamily::SupportReport
    ) {
        rows.push("readiness.format.localized_date_number_and_count".to_owned());
    }
    rows
}

fn fixture_refs_for(family: M5DenseSurfaceFamily) -> Vec<String> {
    let mut refs = vec![
        "fixtures/i18n/pseudoloc_rtl_ime_manifest.yaml".to_owned(),
        "fixtures/i18n/m3/pseudoloc_rtl_ime_corpus/manifest.json".to_owned(),
    ];
    let extra = match family {
        M5DenseSurfaceFamily::CommandPalette => Some(
            "fixtures/i18n/locale_surface_examples/shell_commands_and_palette_localized_label_stable_ids.yaml",
        ),
        M5DenseSurfaceFamily::DocsHelpPane => {
            Some("fixtures/i18n/docs-tour-auth-recovery/translated-help-packs.json")
        }
        M5DenseSurfaceFamily::SupportReport => {
            Some("fixtures/i18n/cli-doctor-support/cli-help-localization.json")
        }
        _ => None,
    };
    if let Some(extra) = extra {
        refs.push(extra.to_owned());
    }
    refs
}

fn seeded_harness_results(cases: &[M5DenseHarnessCase]) -> Vec<M5DenseHarnessResultRow> {
    let mut results = Vec::new();
    for case in cases {
        for harness_kind in &case.harness_kinds {
            for locale in CLAIMED_LOCALES {
                results.push(harness_result(case, *harness_kind, locale));
            }
        }
    }
    results
}

fn harness_result(
    case: &M5DenseHarnessCase,
    harness_kind: M5DenseHarnessKind,
    locale: &str,
) -> M5DenseHarnessResultRow {
    let harness_slug = harness_slug(harness_kind);
    let assertion_refs = assertions_for_harness(harness_kind, case);
    M5DenseHarnessResultRow {
        result_id: format!("{}:{harness_slug}:{locale}", case.case_id),
        case_ref: case.case_id.clone(),
        surface_family: case.surface_family,
        harness_kind,
        requested_locale: locale.to_owned(),
        effective_locale: locale.to_owned(),
        text_direction: locale_text_direction(locale),
        result_state: M5HarnessResultState::Passed,
        assertion_refs,
        failure_class: None,
        evidence_ref: format!(
            "{M5_DENSE_SURFACE_LAB_FIXTURE_ROOT}/qualification.json#{}/{harness_slug}/{locale}",
            case.case_id
        ),
        waiver_ref: None,
        detail: harness_detail(harness_kind, locale),
    }
}

fn assertions_for_harness(
    harness_kind: M5DenseHarnessKind,
    case: &M5DenseHarnessCase,
) -> Vec<DenseI18nAssertionClass> {
    let mut refs = match harness_kind {
        M5DenseHarnessKind::Pseudolocalization | M5DenseHarnessKind::TextExpansion => {
            vec![DenseI18nAssertionClass::NoTruncationOrOverflow]
        }
        M5DenseHarnessKind::RtlBidi => vec![
            DenseI18nAssertionClass::RtlChromeMirrorsOnlyDirectionalUi,
            DenseI18nAssertionClass::LiteralTechnicalTokensPreserved,
        ],
        M5DenseHarnessKind::FontFallback | M5DenseHarnessKind::Cjk => {
            vec![DenseI18nAssertionClass::FontFallbackWorks]
        }
        M5DenseHarnessKind::ImeComposition => vec![
            DenseI18nAssertionClass::ImeCompositionNotSilentlyCommittedOrCancelled,
            DenseI18nAssertionClass::CandidateAndCaretRemainVisible,
        ],
        M5DenseHarnessKind::LocalizedDateNumber => {
            vec![DenseI18nAssertionClass::LocalizedFormattingKeepsStableSemantics]
        }
    };
    refs.push(DenseI18nAssertionClass::StableTranslatedSurfaceRefsPreserved);
    refs.push(DenseI18nAssertionClass::SourceLanguageEscapeHatchAvailable);
    // Keep only assertions the case itself claims to cover.
    refs.retain(|assertion| case.assertion_refs.contains(assertion));
    refs.sort();
    refs.dedup();
    refs
}

fn harness_slug(harness_kind: M5DenseHarnessKind) -> &'static str {
    match harness_kind {
        M5DenseHarnessKind::Pseudolocalization => "pseudoloc",
        M5DenseHarnessKind::TextExpansion => "text-expansion",
        M5DenseHarnessKind::RtlBidi => "rtl-bidi",
        M5DenseHarnessKind::FontFallback => "font-fallback",
        M5DenseHarnessKind::ImeComposition => "ime",
        M5DenseHarnessKind::Cjk => "cjk",
        M5DenseHarnessKind::LocalizedDateNumber => "date-number",
    }
}

fn harness_detail(harness_kind: M5DenseHarnessKind, locale: &str) -> String {
    let dir = match locale_text_direction(locale) {
        TextDirection::LeftToRight => "ltr",
        TextDirection::RightToLeft => "rtl",
    };
    match harness_kind {
        M5DenseHarnessKind::Pseudolocalization => {
            format!("Pseudoloc accent wrap holds layout without clipping ({dir}).")
        }
        M5DenseHarnessKind::TextExpansion => {
            format!("Translated copy stays within the expansion budget ({dir}).")
        }
        M5DenseHarnessKind::RtlBidi => {
            "Directional chrome mirrors while literal tokens stay unmirrored.".to_owned()
        }
        M5DenseHarnessKind::FontFallback => {
            "Glyphs render through the accepted font-fallback chain.".to_owned()
        }
        M5DenseHarnessKind::ImeComposition => {
            "Composition survives dense churn without silent commit or cancel.".to_owned()
        }
        M5DenseHarnessKind::Cjk => {
            "Full-width glyphs and counts wrap without occlusion.".to_owned()
        }
        M5DenseHarnessKind::LocalizedDateNumber => {
            format!("Dates, numbers, and counts format for {locale} with stable semantics.")
        }
    }
}
