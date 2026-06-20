//! Governed localized vocabulary for durable-attention and lifecycle states.
//!
//! Durable-attention surfaces — the activity center, job-row badges,
//! notification summaries, lifecycle-state copy, and quiet-hours/admin
//! suppression messaging — must say the same thing in every locale. This module
//! owns the single governed terminology glossary those surfaces draw from so no
//! feature family coins its own translated status wording.
//!
//! Each [`AttentionGlossaryTerm`] binds one durable-attention or lifecycle
//! concept to:
//!
//! - a stable, locale-neutral `term_key` (the anchor support, docs, and exported
//!   packets map localized states back to),
//! - a locale-neutral [`AttentionSeverityRank`] and `action_order_index` that
//!   translation can never soften, strengthen, or reorder,
//! - one canonical source-language [`AttentionGlossaryTerm::definition`] that
//!   fixes the meaning once, and
//! - per-locale [`AttentionTermTranslation`] rows that carry only the localized
//!   label plus the reviewer-asserted severity rank and order, so a drifting
//!   translation is a checkable failure rather than a manual catch.
//!
//! [`build_attention_vocabulary_parity_report`] audits the glossary across
//! claimed locales, and [`seeded_attention_vocabulary_drift_scenarios`] proves a
//! softened severity, reordered action, broken lexical consistency, or hidden
//! severity under truncation is caught before release.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::localized_catalog::{LocalizationRenderState, TextDirection, CLAIMED_LOCALES};
use crate::m5_dense_surface_lab::locale_text_direction;
use crate::LocalePackValidationFinding;

/// Schema version exported by attention-vocabulary records.
pub const ATTENTION_VOCABULARY_SCHEMA_VERSION: u32 = 1;

/// Record kind for [`AttentionVocabularyGlossary`].
pub const ATTENTION_VOCABULARY_GLOSSARY_RECORD_KIND: &str = "attention_vocabulary_glossary_record";

/// Record kind for [`AttentionVocabularyParityReport`].
pub const ATTENTION_VOCABULARY_PARITY_RECORD_KIND: &str =
    "attention_vocabulary_parity_report_record";

/// Record kind for [`AttentionVocabularyDriftScenarioSet`].
pub const ATTENTION_VOCABULARY_DRIFT_RECORD_KIND: &str =
    "attention_vocabulary_drift_scenario_set_record";

/// Stable id for the seeded governed glossary.
pub const ATTENTION_VOCABULARY_GLOSSARY_ID: &str = "i18n-glossary:m5-attention-and-lifecycle:v1";

/// Stable id for the seeded parity report.
pub const ATTENTION_VOCABULARY_PARITY_REPORT_ID: &str = "i18n-parity:m5-attention-and-lifecycle:v1";

/// Stable id for the seeded drift scenario set.
pub const ATTENTION_VOCABULARY_DRIFT_SCENARIO_SET_ID: &str =
    "i18n-drift:m5-attention-and-lifecycle:v1";

/// Fixture ref for the seeded glossary packet.
pub const ATTENTION_VOCABULARY_GLOSSARY_FIXTURE_REF: &str =
    "fixtures/i18n/activity-center-and-notifications/glossary.json";

/// Fixture ref for the seeded parity report.
pub const ATTENTION_VOCABULARY_PARITY_FIXTURE_REF: &str =
    "fixtures/i18n/activity-center-and-notifications/parity_report.json";

/// Fixture ref for the seeded drift scenario set.
pub const ATTENTION_VOCABULARY_DRIFT_FIXTURE_REF: &str =
    "fixtures/i18n/activity-center-and-notifications/drift_scenarios.json";

/// Fixture root for the attention-vocabulary fixtures.
pub const ATTENTION_VOCABULARY_FIXTURE_ROOT: &str =
    "fixtures/i18n/activity-center-and-notifications";

/// Source-language locale for governed attention terms.
pub const ATTENTION_VOCABULARY_SOURCE_LANGUAGE_LOCALE: &str = "en-US";

/// Visible-grapheme budget the truncated terms are rendered against.
pub const ATTENTION_TERM_TRUNCATION_BUDGET_GRAPHEMES: usize = 24;

const GENERATED_AT: &str = "2026-06-01T00:00:00Z";
const TARGET_BUILD_IDENTITY_REF: &str = "build-identity:m5-stable-candidate";

/// Durable-attention surface family a governed term belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionTermDomain {
    /// Job-row lifecycle state shared by the activity center and durable rows.
    LifecycleState,
    /// Activity-center partition / grouping header.
    ActivityPartition,
    /// Job-row badge and aggregate badge counts.
    JobRowBadge,
    /// Quiet-hours, focus, do-not-disturb, presentation, and admin mode label.
    QuietHoursMode,
    /// Reason a fanout was held, suppressed, deduped, muted, or released.
    SuppressionReason,
}

impl AttentionTermDomain {
    /// Returns every governed domain in stable order.
    pub fn all() -> Vec<Self> {
        vec![
            Self::LifecycleState,
            Self::ActivityPartition,
            Self::JobRowBadge,
            Self::QuietHoursMode,
            Self::SuppressionReason,
        ]
    }
}

/// Locale-neutral severity rank a translation can never alter.
///
/// The rank is metadata bound to the stable term, not derived from prose, so a
/// translated label cannot make a state read softer or stronger than the
/// canonical durable-attention state it stands for. Higher means more severe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionSeverityRank {
    /// Neutral or terminal-informational state.
    Informational,
    /// Work is actively in progress.
    InProgress,
    /// Work is queued or waiting to start.
    Pending,
    /// Work completed successfully.
    Success,
    /// State requires a person to act.
    NeedsAttention,
    /// State is degraded or only partially complete.
    Degraded,
    /// State failed.
    Failure,
    /// State is security-critical or blocking.
    Critical,
}

impl AttentionSeverityRank {
    /// Returns the monotonic severity level (higher means more severe).
    pub const fn level(self) -> u8 {
        match self {
            Self::Informational => 0,
            Self::InProgress => 1,
            Self::Pending => 2,
            Self::Success => 3,
            Self::NeedsAttention => 4,
            Self::Degraded => 5,
            Self::Failure => 6,
            Self::Critical => 7,
        }
    }
}

/// Per-locale rendering of one governed term.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionTermTranslation {
    /// Locale tag for the translation.
    pub locale: String,
    /// Writing direction for the locale.
    pub text_direction: TextDirection,
    /// Localized label shown on durable-attention surfaces.
    pub localized_term: String,
    /// Whether the locale localized the term or fell back to the source language.
    pub localization_state: LocalizationRenderState,
    /// Visible-grapheme length of the source-language term.
    pub source_grapheme_len: usize,
    /// Visible-grapheme length of the localized term.
    pub display_grapheme_len: usize,
    /// Localized length as a percentage of the source length (text expansion).
    pub expansion_ratio_pct: u32,
    /// Localized term truncated to the view budget.
    pub truncated_term: String,
    /// Whether the localized term was shortened to fit the budget.
    pub was_truncated: bool,
    /// Always true; truncation never hides the severity icon or scope.
    pub severity_preserved_under_truncation: bool,
    /// Reviewer-recorded severity rank for the translated term.
    ///
    /// Governance requires this to equal the term's canonical
    /// [`AttentionGlossaryTerm::severity_rank`]; any difference is drift.
    pub asserted_severity_rank: AttentionSeverityRank,
    /// Reviewer-recorded action-order index for the translated term.
    ///
    /// Must equal the term's canonical [`AttentionGlossaryTerm::action_order_index`].
    pub asserted_action_order_index: u32,
    /// Whether every source placeholder survived into the localized term.
    pub placeholders_preserved: bool,
}

/// One governed durable-attention or lifecycle term.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionGlossaryTerm {
    /// Stable, locale-neutral term key support and exports anchor on.
    pub term_key: String,
    /// Durable-attention surface family the term belongs to.
    pub domain: AttentionTermDomain,
    /// Locale-neutral severity rank for the term.
    pub severity_rank: AttentionSeverityRank,
    /// Stable presentation order within the domain.
    pub action_order_index: u32,
    /// Whether the term describes a durable-attention state.
    pub durable: bool,
    /// Source-language label.
    pub source_term: String,
    /// Canonical, locale-neutral definition that fixes the meaning once.
    pub definition: String,
    /// Per-locale translations of the label.
    pub translations: Vec<AttentionTermTranslation>,
}

impl AttentionGlossaryTerm {
    /// Returns the translation for a locale, when present.
    pub fn translation(&self, locale: &str) -> Option<&AttentionTermTranslation> {
        self.translations.iter().find(|t| t.locale == locale)
    }
}

/// Roll-up summary for the governed glossary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionGlossarySummary {
    /// Total governed terms.
    pub total_terms: usize,
    /// Domains covered by the glossary.
    pub domains_covered: Vec<AttentionTermDomain>,
    /// Claimed locales the glossary translates into.
    pub claimed_locales: Vec<String>,
    /// Count of durable-attention terms.
    pub durable_term_count: usize,
    /// Largest text-expansion ratio across all translations.
    pub max_expansion_ratio_pct: u32,
}

/// The single governed terminology glossary for durable-attention surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionVocabularyGlossary {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable glossary id.
    pub glossary_id: String,
    /// Deterministic mint timestamp.
    pub generated_at: String,
    /// Target build identity the glossary defends.
    pub target_build_identity_ref: String,
    /// Source-language locale.
    pub source_language_locale: String,
    /// Claimed locales the glossary translates into.
    pub claimed_locales: Vec<String>,
    /// Visible-grapheme budget truncated terms render against.
    pub truncation_budget_graphemes: usize,
    /// Governed terms.
    pub terms: Vec<AttentionGlossaryTerm>,
    /// Roll-up summary.
    pub summary: AttentionGlossarySummary,
}

impl AttentionVocabularyGlossary {
    /// Returns the term for a key, when present.
    pub fn term(&self, term_key: &str) -> Option<&AttentionGlossaryTerm> {
        self.terms.iter().find(|t| t.term_key == term_key)
    }

    /// Returns true when the glossary governs the given term key.
    pub fn governs(&self, term_key: &str) -> bool {
        self.term(term_key).is_some()
    }

    /// Returns the governed term keys for a domain in action order.
    pub fn term_keys_in_domain(&self, domain: AttentionTermDomain) -> Vec<String> {
        let mut rows: Vec<&AttentionGlossaryTerm> =
            self.terms.iter().filter(|t| t.domain == domain).collect();
        rows.sort_by_key(|t| t.action_order_index);
        rows.into_iter().map(|t| t.term_key.clone()).collect()
    }

    /// Returns the semantic drift findings present in the glossary.
    ///
    /// Structural problems (missing locales, duplicate keys) are reported by
    /// [`AttentionVocabularyGlossary::validate`]; this returns only the
    /// translation-governance drift the release gate forbids.
    pub fn audit_findings(&self) -> Vec<AttentionDriftFinding> {
        let mut findings = Vec::new();

        for term in &self.terms {
            for translation in &term.translations {
                if translation.asserted_severity_rank != term.severity_rank {
                    findings.push(AttentionDriftFinding::new(
                        AttentionDriftClass::SeverityRankAltered,
                        &term.term_key,
                        Some(&translation.locale),
                        format!(
                            "translated severity {:?} differs from canonical {:?}",
                            translation.asserted_severity_rank, term.severity_rank
                        ),
                    ));
                }
                if translation.asserted_action_order_index != term.action_order_index {
                    findings.push(AttentionDriftFinding::new(
                        AttentionDriftClass::ActionOrderAltered,
                        &term.term_key,
                        Some(&translation.locale),
                        format!(
                            "translated action order {} differs from canonical {}",
                            translation.asserted_action_order_index, term.action_order_index
                        ),
                    ));
                }
                if translation.was_truncated && !translation.severity_preserved_under_truncation {
                    findings.push(AttentionDriftFinding::new(
                        AttentionDriftClass::SeverityHiddenUnderTruncation,
                        &term.term_key,
                        Some(&translation.locale),
                        "truncation hides the severity or scope of the term".to_owned(),
                    ));
                }
                if !translation.placeholders_preserved {
                    findings.push(AttentionDriftFinding::new(
                        AttentionDriftClass::PlaceholderDropped,
                        &term.term_key,
                        Some(&translation.locale),
                        "a source placeholder was dropped in translation".to_owned(),
                    ));
                }
            }
        }

        // Lexical consistency and collisions are evaluated per locale: terms that
        // share a source word must share a translation, and distinct source words
        // must not collapse to the same translation.
        for locale in &self.claimed_locales {
            for (i, left) in self.terms.iter().enumerate() {
                let Some(left_tr) = left.translation(locale) else {
                    continue;
                };
                for right in self.terms.iter().skip(i + 1) {
                    let Some(right_tr) = right.translation(locale) else {
                        continue;
                    };
                    let same_source = left.source_term == right.source_term;
                    let same_localized = left_tr.localized_term == right_tr.localized_term;
                    if same_source && !same_localized {
                        findings.push(AttentionDriftFinding::new(
                            AttentionDriftClass::LexicalConsistencyBroken,
                            &left.term_key,
                            Some(locale),
                            format!(
                                "shares source term \"{}\" with {} but translates differently",
                                left.source_term, right.term_key
                            ),
                        ));
                    }
                    if !same_source && same_localized {
                        findings.push(AttentionDriftFinding::new(
                            AttentionDriftClass::TermCollision,
                            &left.term_key,
                            Some(locale),
                            format!(
                                "distinct term {} collides on translation \"{}\"",
                                right.term_key, left_tr.localized_term
                            ),
                        ));
                    }
                }
            }
        }

        findings.sort();
        findings
    }

    /// Validates the glossary's structure and governance invariants.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != ATTENTION_VOCABULARY_GLOSSARY_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                &self.glossary_id,
                "record_kind is not the governed glossary kind",
            ));
        }
        if self.schema_version != ATTENTION_VOCABULARY_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                &self.glossary_id,
                "schema_version drifted from the governed version",
            ));
        }
        if self.terms.is_empty() {
            findings.push(LocalePackValidationFinding::new(
                &self.glossary_id,
                "glossary has no governed terms",
            ));
        }

        // Term keys must be unique and sorted action order must be dense per domain.
        let mut seen_keys = BTreeSet::new();
        for term in &self.terms {
            if !seen_keys.insert(term.term_key.clone()) {
                findings.push(LocalePackValidationFinding::new(
                    &term.term_key,
                    "duplicate term key",
                ));
            }
            for locale in &self.claimed_locales {
                if term.translation(locale).is_none() {
                    findings.push(LocalePackValidationFinding::new(
                        &term.term_key,
                        format!("missing translation for claimed locale {locale}"),
                    ));
                }
            }
        }

        for domain in AttentionTermDomain::all() {
            let mut indices: Vec<u32> = self
                .terms
                .iter()
                .filter(|t| t.domain == domain)
                .map(|t| t.action_order_index)
                .collect();
            if indices.is_empty() {
                findings.push(LocalePackValidationFinding::new(
                    format!("{domain:?}"),
                    "domain has no governed terms",
                ));
                continue;
            }
            indices.sort_unstable();
            for (expected, actual) in indices.iter().enumerate() {
                if *actual != expected as u32 {
                    findings.push(LocalePackValidationFinding::new(
                        format!("{domain:?}"),
                        "action order indices are not dense from zero",
                    ));
                    break;
                }
            }
        }

        // Semantic drift must be empty for a governed glossary.
        for drift in self.audit_findings() {
            findings.push(LocalePackValidationFinding::new(
                drift.term_key,
                format!("{:?}: {}", drift.class, drift.detail),
            ));
        }

        // The summary must reflect the terms.
        let recomputed = summarize(&self.terms, &self.claimed_locales);
        if recomputed != self.summary {
            findings.push(LocalePackValidationFinding::new(
                &self.glossary_id,
                "summary does not match the governed terms",
            ));
        }

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

/// Class of localization drift the release gate forbids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionDriftClass {
    /// A translation asserts a softer or stronger severity than the canonical term.
    SeverityRankAltered,
    /// A translation reorders an action relative to the canonical order.
    ActionOrderAltered,
    /// Two terms with the same source word translate inconsistently.
    LexicalConsistencyBroken,
    /// Two distinct terms collapse onto the same translation.
    TermCollision,
    /// Truncation hides the severity or scope of a term.
    SeverityHiddenUnderTruncation,
    /// A source placeholder was dropped in translation.
    PlaceholderDropped,
}

/// One semantic drift finding produced by [`AttentionVocabularyGlossary::audit_findings`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AttentionDriftFinding {
    /// Drift class.
    pub class: AttentionDriftClass,
    /// Term key the finding applies to.
    pub term_key: String,
    /// Locale the finding applies to, when locale-specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Export-safe detail.
    pub detail: String,
}

impl AttentionDriftFinding {
    fn new(
        class: AttentionDriftClass,
        term_key: &str,
        locale: Option<&str>,
        detail: String,
    ) -> Self {
        Self {
            class,
            term_key: term_key.to_owned(),
            locale: locale.map(str::to_owned),
            detail,
        }
    }
}

/// Per-locale parity row in the audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionLocaleParityRow {
    /// Locale tag.
    pub locale: String,
    /// Writing direction.
    pub text_direction: TextDirection,
    /// Total governed terms.
    pub total_terms: usize,
    /// Terms rendered in the requested locale.
    pub localized_terms: usize,
    /// Terms that fell back to the source language.
    pub source_fallback_terms: usize,
    /// Terms truncated to the view budget.
    pub truncated_terms: usize,
    /// Largest text-expansion ratio across the locale's terms.
    pub max_expansion_ratio_pct: u32,
    /// Whether every term keeps its canonical severity rank.
    pub all_severity_ranks_preserved: bool,
    /// Whether action order is stable for every term.
    pub action_order_stable: bool,
    /// Whether terms sharing a source word share a translation.
    pub lexical_consistency_holds: bool,
    /// Whether distinct terms avoid collapsing onto one translation.
    pub no_term_collisions: bool,
    /// Whether severity survives truncation on every term.
    pub all_severities_preserved_under_truncation: bool,
    /// Per-locale claim state: `green`, `narrowed`, or `blocked`.
    pub claim_state: String,
}

impl AttentionLocaleParityRow {
    /// Returns true when the locale holds a fully governed (green) claim.
    pub fn is_governed(&self) -> bool {
        self.claim_state == "green"
    }
}

/// Per-domain parity row in the audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionDomainParityRow {
    /// Domain.
    pub domain: AttentionTermDomain,
    /// Number of governed terms in the domain.
    pub term_count: usize,
    /// Action-order indices in the domain, sorted.
    pub action_order_indices: Vec<u32>,
    /// Whether the action order is dense from zero with no gaps or duplicates.
    pub action_order_dense: bool,
    /// Distinct severity ranks present in the domain.
    pub severity_ranks: Vec<AttentionSeverityRank>,
}

/// Roll-up summary for the parity report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionParitySummary {
    /// Total governed terms.
    pub total_terms: usize,
    /// Total domains.
    pub total_domains: usize,
    /// Total claimed locales.
    pub total_locales: usize,
    /// Locales holding a green claim.
    pub green_locale_count: usize,
    /// Locales narrowed to source-language fallback.
    pub narrowed_locale_count: usize,
    /// Locales blocked by drift.
    pub blocked_locale_count: usize,
    /// Largest text-expansion ratio across every locale.
    pub max_expansion_ratio_pct: u32,
    /// Number of semantic drift findings.
    pub drift_finding_count: usize,
    /// Roll-up parity state: `green`, `narrowed`, or `blocked`.
    pub parity_state: String,
}

/// Audit report proving the governed glossary stays consistent across locales.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionVocabularyParityReport {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Deterministic mint timestamp.
    pub generated_at: String,
    /// Glossary id this report audits.
    pub glossary_id_ref: String,
    /// Target build identity.
    pub target_build_identity_ref: String,
    /// Source-language locale.
    pub source_language_locale: String,
    /// Claimed locales.
    pub claimed_locales: Vec<String>,
    /// Per-locale parity rows.
    pub locale_rows: Vec<AttentionLocaleParityRow>,
    /// Per-domain parity rows.
    pub domain_rows: Vec<AttentionDomainParityRow>,
    /// Semantic drift findings carried by the audited glossary.
    pub drift_findings: Vec<AttentionDriftFinding>,
    /// Roll-up summary.
    pub summary: AttentionParitySummary,
}

impl AttentionVocabularyParityReport {
    /// Returns the parity row for a locale, when present.
    pub fn locale_row(&self, locale: &str) -> Option<&AttentionLocaleParityRow> {
        self.locale_rows.iter().find(|r| r.locale == locale)
    }

    /// Validates the report against a freshly built audit of the glossary.
    pub fn validate(&self) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();
        if self.record_kind != ATTENTION_VOCABULARY_PARITY_RECORD_KIND {
            findings.push(LocalePackValidationFinding::new(
                &self.report_id,
                "record_kind is not the parity report kind",
            ));
        }
        if self.schema_version != ATTENTION_VOCABULARY_SCHEMA_VERSION {
            findings.push(LocalePackValidationFinding::new(
                &self.report_id,
                "schema_version drifted from the governed version",
            ));
        }
        let rebuilt =
            build_attention_vocabulary_parity_report(&seeded_attention_vocabulary_glossary());
        if rebuilt.locale_rows != self.locale_rows {
            findings.push(LocalePackValidationFinding::new(
                &self.report_id,
                "locale parity rows do not match the audited glossary",
            ));
        }
        if rebuilt.domain_rows != self.domain_rows {
            findings.push(LocalePackValidationFinding::new(
                &self.report_id,
                "domain parity rows do not match the audited glossary",
            ));
        }
        if rebuilt.summary != self.summary {
            findings.push(LocalePackValidationFinding::new(
                &self.report_id,
                "summary does not match the audited glossary",
            ));
        }
        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }
}

/// Builds the parity report from a governed glossary.
pub fn build_attention_vocabulary_parity_report(
    glossary: &AttentionVocabularyGlossary,
) -> AttentionVocabularyParityReport {
    let drift_findings = glossary.audit_findings();

    let mut locale_rows = Vec::new();
    let mut overall_max_expansion = 0u32;
    let (mut green, mut narrowed, mut blocked) = (0usize, 0usize, 0usize);

    for locale in &glossary.claimed_locales {
        let mut localized = 0usize;
        let mut fallback = 0usize;
        let mut truncated = 0usize;
        let mut max_expansion = 0u32;
        let mut severity_ok = true;
        let mut order_ok = true;
        let mut trunc_severity_ok = true;

        for term in &glossary.terms {
            if let Some(tr) = term.translation(locale) {
                match tr.localization_state {
                    LocalizationRenderState::Localized => localized += 1,
                    LocalizationRenderState::SourceLanguageFallback => fallback += 1,
                }
                if tr.was_truncated {
                    truncated += 1;
                }
                max_expansion = max_expansion.max(tr.expansion_ratio_pct);
                if tr.asserted_severity_rank != term.severity_rank {
                    severity_ok = false;
                }
                if tr.asserted_action_order_index != term.action_order_index {
                    order_ok = false;
                }
                if tr.was_truncated && !tr.severity_preserved_under_truncation {
                    trunc_severity_ok = false;
                }
            }
        }
        overall_max_expansion = overall_max_expansion.max(max_expansion);

        let lexical_ok = !drift_findings.iter().any(|f| {
            f.locale.as_deref() == Some(locale.as_str())
                && f.class == AttentionDriftClass::LexicalConsistencyBroken
        });
        let no_collisions = !drift_findings.iter().any(|f| {
            f.locale.as_deref() == Some(locale.as_str())
                && f.class == AttentionDriftClass::TermCollision
        });

        let governance_ok =
            severity_ok && order_ok && trunc_severity_ok && lexical_ok && no_collisions;
        let claim_state = if !governance_ok {
            blocked += 1;
            "blocked"
        } else if fallback > 0 {
            narrowed += 1;
            "narrowed"
        } else {
            green += 1;
            "green"
        };

        locale_rows.push(AttentionLocaleParityRow {
            locale: locale.clone(),
            text_direction: locale_text_direction(locale),
            total_terms: glossary.terms.len(),
            localized_terms: localized,
            source_fallback_terms: fallback,
            truncated_terms: truncated,
            max_expansion_ratio_pct: max_expansion,
            all_severity_ranks_preserved: severity_ok,
            action_order_stable: order_ok,
            lexical_consistency_holds: lexical_ok,
            no_term_collisions: no_collisions,
            all_severities_preserved_under_truncation: trunc_severity_ok,
            claim_state: claim_state.to_owned(),
        });
    }

    let mut domain_rows = Vec::new();
    for domain in AttentionTermDomain::all() {
        let terms: Vec<&AttentionGlossaryTerm> = glossary
            .terms
            .iter()
            .filter(|t| t.domain == domain)
            .collect();
        let mut indices: Vec<u32> = terms.iter().map(|t| t.action_order_index).collect();
        indices.sort_unstable();
        let dense = indices
            .iter()
            .enumerate()
            .all(|(expected, actual)| *actual == expected as u32);
        let mut ranks: Vec<AttentionSeverityRank> = terms.iter().map(|t| t.severity_rank).collect();
        ranks.sort();
        ranks.dedup();
        domain_rows.push(AttentionDomainParityRow {
            domain,
            term_count: terms.len(),
            action_order_indices: indices,
            action_order_dense: dense,
            severity_ranks: ranks,
        });
    }

    let parity_state = if blocked > 0 {
        "blocked"
    } else if narrowed > 0 {
        "narrowed"
    } else {
        "green"
    };

    let summary = AttentionParitySummary {
        total_terms: glossary.terms.len(),
        total_domains: domain_rows.len(),
        total_locales: glossary.claimed_locales.len(),
        green_locale_count: green,
        narrowed_locale_count: narrowed,
        blocked_locale_count: blocked,
        max_expansion_ratio_pct: overall_max_expansion,
        drift_finding_count: drift_findings.len(),
        parity_state: parity_state.to_owned(),
    };

    AttentionVocabularyParityReport {
        record_kind: ATTENTION_VOCABULARY_PARITY_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_VOCABULARY_SCHEMA_VERSION,
        report_id: ATTENTION_VOCABULARY_PARITY_REPORT_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        glossary_id_ref: glossary.glossary_id.clone(),
        target_build_identity_ref: glossary.target_build_identity_ref.clone(),
        source_language_locale: glossary.source_language_locale.clone(),
        claimed_locales: glossary.claimed_locales.clone(),
        locale_rows,
        domain_rows,
        drift_findings,
        summary,
    }
}

/// A change applied to the glossary to prove a drift class is caught.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AttentionDriftMutation {
    /// Assert a different severity rank for a term's translation.
    SetAssertedSeverity {
        /// Term key to mutate.
        term_key: String,
        /// Locale to mutate.
        locale: String,
        /// Severity rank to assert.
        rank: AttentionSeverityRank,
    },
    /// Assert a different action-order index for a term's translation.
    SetAssertedActionOrder {
        /// Term key to mutate.
        term_key: String,
        /// Locale to mutate.
        locale: String,
        /// Action-order index to assert.
        index: u32,
    },
    /// Replace a term's localized label for a locale.
    SetLocalizedTerm {
        /// Term key to mutate.
        term_key: String,
        /// Locale to mutate.
        locale: String,
        /// Localized label to set.
        text: String,
    },
    /// Mark severity as hidden under truncation for a term's translation.
    HideSeverityUnderTruncation {
        /// Term key to mutate.
        term_key: String,
        /// Locale to mutate.
        locale: String,
    },
    /// Drop a placeholder from a term's translation.
    DropPlaceholder {
        /// Term key to mutate.
        term_key: String,
        /// Locale to mutate.
        locale: String,
    },
}

impl AttentionDriftMutation {
    fn apply(&self, glossary: &mut AttentionVocabularyGlossary) {
        let (key, locale) = match self {
            Self::SetAssertedSeverity {
                term_key, locale, ..
            }
            | Self::SetAssertedActionOrder {
                term_key, locale, ..
            }
            | Self::SetLocalizedTerm {
                term_key, locale, ..
            }
            | Self::HideSeverityUnderTruncation { term_key, locale }
            | Self::DropPlaceholder { term_key, locale } => (term_key.clone(), locale.clone()),
        };
        let Some(term) = glossary.terms.iter_mut().find(|t| t.term_key == key) else {
            return;
        };
        let Some(tr) = term.translations.iter_mut().find(|t| t.locale == locale) else {
            return;
        };
        match self {
            Self::SetAssertedSeverity { rank, .. } => tr.asserted_severity_rank = *rank,
            Self::SetAssertedActionOrder { index, .. } => tr.asserted_action_order_index = *index,
            Self::SetLocalizedTerm { text, .. } => {
                tr.localized_term = text.clone();
                tr.truncated_term = text.clone();
            }
            Self::HideSeverityUnderTruncation { .. } => {
                tr.was_truncated = true;
                tr.severity_preserved_under_truncation = false;
            }
            Self::DropPlaceholder { .. } => tr.placeholders_preserved = false,
        }
    }
}

/// One drift scenario: a mutation plus the drift class it must surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionDriftScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Export-safe description.
    pub description: String,
    /// Mutation applied to the governed glossary.
    pub mutation: AttentionDriftMutation,
    /// Drift class the audit must report.
    pub expected_class: AttentionDriftClass,
}

/// Set of drift scenarios proving governance failures are caught.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttentionVocabularyDriftScenarioSet {
    /// Boundary record kind.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable scenario-set id.
    pub scenario_set_id: String,
    /// Glossary id the scenarios mutate.
    pub glossary_id_ref: String,
    /// Drift scenarios.
    pub scenarios: Vec<AttentionDriftScenario>,
}

impl AttentionVocabularyDriftScenarioSet {
    /// Confirms a clean glossary is drift-free and every scenario surfaces its class.
    pub fn validate_against(
        &self,
        glossary: &AttentionVocabularyGlossary,
    ) -> Result<(), Vec<LocalePackValidationFinding>> {
        let mut findings = Vec::new();

        if !glossary.audit_findings().is_empty() {
            findings.push(LocalePackValidationFinding::new(
                &self.scenario_set_id,
                "baseline glossary already carries drift findings",
            ));
        }

        for scenario in &self.scenarios {
            let mut mutated = glossary.clone();
            scenario.mutation.apply(&mut mutated);
            let surfaced = mutated
                .audit_findings()
                .iter()
                .any(|f| f.class == scenario.expected_class);
            if !surfaced {
                findings.push(LocalePackValidationFinding::new(
                    &scenario.scenario_id,
                    format!(
                        "mutation did not surface expected drift {:?}",
                        scenario.expected_class
                    ),
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

/// Returns the single governed attention/lifecycle vocabulary glossary.
pub fn seeded_attention_vocabulary_glossary() -> AttentionVocabularyGlossary {
    let claimed_locales: Vec<String> = CLAIMED_LOCALES.iter().map(|s| (*s).to_owned()).collect();
    let terms: Vec<AttentionGlossaryTerm> = governed_term_seeds()
        .into_iter()
        .map(|seed| seed.build(&claimed_locales))
        .collect();
    let summary = summarize(&terms, &claimed_locales);

    AttentionVocabularyGlossary {
        record_kind: ATTENTION_VOCABULARY_GLOSSARY_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_VOCABULARY_SCHEMA_VERSION,
        glossary_id: ATTENTION_VOCABULARY_GLOSSARY_ID.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        target_build_identity_ref: TARGET_BUILD_IDENTITY_REF.to_owned(),
        source_language_locale: ATTENTION_VOCABULARY_SOURCE_LANGUAGE_LOCALE.to_owned(),
        claimed_locales,
        truncation_budget_graphemes: ATTENTION_TERM_TRUNCATION_BUDGET_GRAPHEMES,
        terms,
        summary,
    }
}

/// Returns the seeded parity report for the governed glossary.
pub fn seeded_attention_vocabulary_parity_report() -> AttentionVocabularyParityReport {
    build_attention_vocabulary_parity_report(&seeded_attention_vocabulary_glossary())
}

/// Returns the seeded drift scenario set.
pub fn seeded_attention_vocabulary_drift_scenarios() -> AttentionVocabularyDriftScenarioSet {
    AttentionVocabularyDriftScenarioSet {
        record_kind: ATTENTION_VOCABULARY_DRIFT_RECORD_KIND.to_owned(),
        schema_version: ATTENTION_VOCABULARY_SCHEMA_VERSION,
        scenario_set_id: ATTENTION_VOCABULARY_DRIFT_SCENARIO_SET_ID.to_owned(),
        glossary_id_ref: ATTENTION_VOCABULARY_GLOSSARY_ID.to_owned(),
        scenarios: vec![
            AttentionDriftScenario {
                scenario_id: "drift:severity-softened:failed:es-MX".to_owned(),
                description: "A translation softens a failed job to merely informational."
                    .to_owned(),
                mutation: AttentionDriftMutation::SetAssertedSeverity {
                    term_key: "attention.lifecycle.failed".to_owned(),
                    locale: "es-MX".to_owned(),
                    rank: AttentionSeverityRank::Informational,
                },
                expected_class: AttentionDriftClass::SeverityRankAltered,
            },
            AttentionDriftScenario {
                scenario_id: "drift:severity-strengthened:completed:ja-JP".to_owned(),
                description: "A translation strengthens a successful completion into a failure."
                    .to_owned(),
                mutation: AttentionDriftMutation::SetAssertedSeverity {
                    term_key: "attention.lifecycle.completed".to_owned(),
                    locale: "ja-JP".to_owned(),
                    rank: AttentionSeverityRank::Failure,
                },
                expected_class: AttentionDriftClass::SeverityRankAltered,
            },
            AttentionDriftScenario {
                scenario_id: "drift:action-reordered:needs_approval:ar-SA".to_owned(),
                description: "A translation reorders a lifecycle state in the row sequence."
                    .to_owned(),
                mutation: AttentionDriftMutation::SetAssertedActionOrder {
                    term_key: "attention.lifecycle.needs_approval".to_owned(),
                    locale: "ar-SA".to_owned(),
                    index: 99,
                },
                expected_class: AttentionDriftClass::ActionOrderAltered,
            },
            AttentionDriftScenario {
                scenario_id: "drift:lexical-inconsistent:running:es-MX".to_owned(),
                description:
                    "The running lifecycle state and running badge translate the same word differently."
                        .to_owned(),
                mutation: AttentionDriftMutation::SetLocalizedTerm {
                    term_key: "attention.lifecycle.running".to_owned(),
                    locale: "es-MX".to_owned(),
                    text: "Ejecutándose ahora".to_owned(),
                },
                expected_class: AttentionDriftClass::LexicalConsistencyBroken,
            },
            AttentionDriftScenario {
                scenario_id: "drift:collision:failed-vs-suppressed:ja-JP".to_owned(),
                description: "A failed state collides onto the policy-suppressed translation."
                    .to_owned(),
                mutation: AttentionDriftMutation::SetLocalizedTerm {
                    term_key: "attention.suppression.suppressed_by_policy".to_owned(),
                    locale: "ja-JP".to_owned(),
                    text: "失敗".to_owned(),
                },
                expected_class: AttentionDriftClass::TermCollision,
            },
            AttentionDriftScenario {
                scenario_id: "drift:severity-hidden-truncation:admin:ar-SA".to_owned(),
                description: "Truncation hides the severity of admin suppression messaging."
                    .to_owned(),
                mutation: AttentionDriftMutation::HideSeverityUnderTruncation {
                    term_key: "attention.suppression.admin_suppression".to_owned(),
                    locale: "ar-SA".to_owned(),
                },
                expected_class: AttentionDriftClass::SeverityHiddenUnderTruncation,
            },
        ],
    }
}

/// Computes the glossary summary from terms.
fn summarize(
    terms: &[AttentionGlossaryTerm],
    claimed_locales: &[String],
) -> AttentionGlossarySummary {
    let mut domains: Vec<AttentionTermDomain> = terms.iter().map(|t| t.domain).collect();
    domains.sort();
    domains.dedup();
    let durable = terms.iter().filter(|t| t.durable).count();
    let max_expansion = terms
        .iter()
        .flat_map(|t| t.translations.iter())
        .map(|t| t.expansion_ratio_pct)
        .max()
        .unwrap_or(0);
    AttentionGlossarySummary {
        total_terms: terms.len(),
        domains_covered: domains,
        claimed_locales: claimed_locales.to_vec(),
        durable_term_count: durable,
        max_expansion_ratio_pct: max_expansion,
    }
}

/// Returns the visible-grapheme length, approximated by Unicode scalar values.
fn grapheme_len(text: &str) -> usize {
    text.chars().count()
}

/// Truncates `text` to `budget` graphemes, appending an ellipsis when cut.
fn truncate_graphemes(text: &str, budget: usize) -> (String, bool) {
    if grapheme_len(text) <= budget {
        return (text.to_owned(), false);
    }
    let kept: String = text.chars().take(budget.saturating_sub(1)).collect();
    (format!("{kept}…"), true)
}

/// Seed describing one governed term and its three claimed-locale translations.
struct TermSeed {
    term_key: &'static str,
    domain: AttentionTermDomain,
    severity_rank: AttentionSeverityRank,
    action_order_index: u32,
    durable: bool,
    source_term: &'static str,
    definition: &'static str,
    /// Translations as `(es-MX, ja-JP, ar-SA)`.
    es_mx: &'static str,
    ja_jp: &'static str,
    ar_sa: &'static str,
}

impl TermSeed {
    fn build(&self, claimed_locales: &[String]) -> AttentionGlossaryTerm {
        let source_len = grapheme_len(self.source_term);
        let translations = claimed_locales
            .iter()
            .map(|locale| {
                let localized = match locale.as_str() {
                    "es-MX" => self.es_mx,
                    "ja-JP" => self.ja_jp,
                    "ar-SA" => self.ar_sa,
                    _ => self.source_term,
                };
                let display_len = grapheme_len(localized);
                let (truncated, was_truncated) =
                    truncate_graphemes(localized, ATTENTION_TERM_TRUNCATION_BUDGET_GRAPHEMES);
                let expansion = if source_len == 0 {
                    100
                } else {
                    ((display_len as u64 * 100) / source_len as u64) as u32
                };
                AttentionTermTranslation {
                    locale: locale.clone(),
                    text_direction: locale_text_direction(locale),
                    localized_term: localized.to_owned(),
                    localization_state: LocalizationRenderState::Localized,
                    source_grapheme_len: source_len,
                    display_grapheme_len: display_len,
                    expansion_ratio_pct: expansion,
                    truncated_term: truncated,
                    was_truncated,
                    severity_preserved_under_truncation: true,
                    asserted_severity_rank: self.severity_rank,
                    asserted_action_order_index: self.action_order_index,
                    placeholders_preserved: true,
                }
            })
            .collect();

        AttentionGlossaryTerm {
            term_key: self.term_key.to_owned(),
            domain: self.domain,
            severity_rank: self.severity_rank,
            action_order_index: self.action_order_index,
            durable: self.durable,
            source_term: self.source_term.to_owned(),
            definition: self.definition.to_owned(),
            translations,
        }
    }
}

/// The governed term table — one row per durable-attention/lifecycle concept.
///
/// Surfaces never coin their own wording: the shell projection binds every
/// canonical activity-center, badge, notification, lifecycle, quiet-hours, and
/// suppression state to one of these keys.
fn governed_term_seeds() -> Vec<TermSeed> {
    use AttentionSeverityRank::*;
    use AttentionTermDomain::*;
    vec![
        // ---- Lifecycle states (shared by the activity center and durable rows) ----
        TermSeed {
            term_key: "attention.lifecycle.running",
            domain: LifecycleState,
            severity_rank: InProgress,
            action_order_index: 0,
            durable: true,
            source_term: "Running",
            definition: "Work is actively executing.",
            es_mx: "En ejecución",
            ja_jp: "実行中",
            ar_sa: "قيد التشغيل",
        },
        TermSeed {
            term_key: "attention.lifecycle.queued",
            domain: LifecycleState,
            severity_rank: Pending,
            action_order_index: 1,
            durable: true,
            source_term: "Queued",
            definition: "Work is waiting in the queue to start.",
            es_mx: "En cola",
            ja_jp: "待機中",
            ar_sa: "في قائمة الانتظار",
        },
        TermSeed {
            term_key: "attention.lifecycle.preparing",
            domain: LifecycleState,
            severity_rank: InProgress,
            action_order_index: 2,
            durable: true,
            source_term: "Preparing",
            definition: "Work is preparing before execution begins.",
            es_mx: "Preparando",
            ja_jp: "準備中",
            ar_sa: "قيد التحضير",
        },
        TermSeed {
            term_key: "attention.lifecycle.needs_approval",
            domain: LifecycleState,
            severity_rank: NeedsAttention,
            action_order_index: 3,
            durable: true,
            source_term: "Needs approval",
            definition: "Work paused awaiting a person's approval.",
            es_mx: "Requiere aprobación",
            ja_jp: "承認が必要",
            ar_sa: "بحاجة إلى موافقة",
        },
        TermSeed {
            term_key: "attention.lifecycle.completed",
            domain: LifecycleState,
            severity_rank: Success,
            action_order_index: 4,
            durable: true,
            source_term: "Completed",
            definition: "Work finished successfully.",
            es_mx: "Completado",
            ja_jp: "完了",
            ar_sa: "اكتمل",
        },
        TermSeed {
            term_key: "attention.lifecycle.partially_completed",
            domain: LifecycleState,
            severity_rank: Degraded,
            action_order_index: 5,
            durable: true,
            source_term: "Partially completed",
            definition: "Work finished with some parts incomplete or skipped.",
            es_mx: "Completado parcialmente",
            ja_jp: "一部完了",
            ar_sa: "اكتمل جزئيًا",
        },
        TermSeed {
            term_key: "attention.lifecycle.failed",
            domain: LifecycleState,
            severity_rank: Failure,
            action_order_index: 6,
            durable: true,
            source_term: "Failed",
            definition: "Work ended in failure.",
            es_mx: "Falló",
            ja_jp: "失敗",
            ar_sa: "فشل",
        },
        TermSeed {
            term_key: "attention.lifecycle.cancelled",
            domain: LifecycleState,
            severity_rank: Informational,
            action_order_index: 7,
            durable: true,
            source_term: "Cancelled",
            definition: "Work was cancelled before completing.",
            es_mx: "Cancelado",
            ja_jp: "キャンセル済み",
            ar_sa: "أُلغي",
        },
        TermSeed {
            term_key: "attention.lifecycle.superseded",
            domain: LifecycleState,
            severity_rank: Informational,
            action_order_index: 8,
            durable: true,
            source_term: "Superseded",
            definition: "Work was replaced by a newer run.",
            es_mx: "Reemplazado",
            ja_jp: "置き換え済み",
            ar_sa: "تم استبداله",
        },
        TermSeed {
            term_key: "attention.lifecycle.history_only",
            domain: LifecycleState,
            severity_rank: Informational,
            action_order_index: 9,
            durable: true,
            source_term: "History only",
            definition: "A terminal row kept for history with no live work.",
            es_mx: "Solo historial",
            ja_jp: "履歴のみ",
            ar_sa: "السجل فقط",
        },
        // ---- Activity-center partitions ----
        TermSeed {
            term_key: "attention.partition.current_work",
            domain: ActivityPartition,
            severity_rank: InProgress,
            action_order_index: 0,
            durable: true,
            source_term: "Current work",
            definition: "Grouping for work that is in progress now.",
            es_mx: "Trabajo actual",
            ja_jp: "現在の作業",
            ar_sa: "العمل الحالي",
        },
        TermSeed {
            term_key: "attention.partition.needs_attention",
            domain: ActivityPartition,
            severity_rank: NeedsAttention,
            action_order_index: 1,
            durable: true,
            source_term: "Needs attention",
            definition: "Grouping for work that requires a person to act.",
            es_mx: "Requiere atención",
            ja_jp: "要対応",
            ar_sa: "يتطلب انتباهًا",
        },
        TermSeed {
            term_key: "attention.partition.completed",
            domain: ActivityPartition,
            severity_rank: Success,
            action_order_index: 2,
            durable: true,
            source_term: "Completed",
            definition: "Grouping for finished work.",
            es_mx: "Completado",
            ja_jp: "完了",
            ar_sa: "اكتمل",
        },
        TermSeed {
            term_key: "attention.partition.suppressed_held",
            domain: ActivityPartition,
            severity_rank: Informational,
            action_order_index: 3,
            durable: true,
            source_term: "Suppressed or held",
            definition: "Grouping for work held or suppressed from active surfaces.",
            es_mx: "Silenciado o retenido",
            ja_jp: "抑制または保留",
            ar_sa: "مكتوم أو محجوز",
        },
        // ---- Job-row badges ----
        TermSeed {
            term_key: "attention.badge.needs_review",
            domain: JobRowBadge,
            severity_rank: NeedsAttention,
            action_order_index: 0,
            durable: true,
            source_term: "Needs review",
            definition: "Badge for items awaiting review.",
            es_mx: "Requiere revisión",
            ja_jp: "要レビュー",
            ar_sa: "بحاجة إلى مراجعة",
        },
        TermSeed {
            term_key: "attention.badge.failed_runs",
            domain: JobRowBadge,
            severity_rank: Failure,
            action_order_index: 1,
            durable: true,
            source_term: "Failed runs",
            definition: "Badge counting failed runs.",
            es_mx: "Ejecuciones fallidas",
            ja_jp: "失敗した実行",
            ar_sa: "عمليات فاشلة",
        },
        TermSeed {
            term_key: "attention.badge.mentions",
            domain: JobRowBadge,
            severity_rank: Informational,
            action_order_index: 2,
            durable: true,
            source_term: "Mentions",
            definition: "Badge counting mentions.",
            es_mx: "Menciones",
            ja_jp: "メンション",
            ar_sa: "إشارات",
        },
        TermSeed {
            term_key: "attention.badge.security_notices",
            domain: JobRowBadge,
            severity_rank: Critical,
            action_order_index: 3,
            durable: true,
            source_term: "Security notices",
            definition: "Badge counting security-critical notices.",
            es_mx: "Avisos de seguridad",
            ja_jp: "セキュリティ通知",
            ar_sa: "إشعارات أمنية",
        },
        TermSeed {
            term_key: "attention.badge.session_requests",
            domain: JobRowBadge,
            severity_rank: NeedsAttention,
            action_order_index: 4,
            durable: true,
            source_term: "Session requests",
            definition: "Badge counting pending session requests.",
            es_mx: "Solicitudes de sesión",
            ja_jp: "セッション要求",
            ar_sa: "طلبات الجلسة",
        },
        TermSeed {
            term_key: "attention.badge.offline_publish_pending",
            domain: JobRowBadge,
            severity_rank: Pending,
            action_order_index: 5,
            durable: true,
            source_term: "Offline publish pending",
            definition: "Badge for work pending an offline publish.",
            es_mx: "Publicación sin conexión pendiente",
            ja_jp: "オフライン公開の保留",
            ar_sa: "نشر دون اتصال معلّق",
        },
        TermSeed {
            term_key: "attention.badge.durable_running_count",
            domain: JobRowBadge,
            severity_rank: InProgress,
            action_order_index: 6,
            durable: true,
            source_term: "Running",
            definition: "Badge counting durable jobs currently running.",
            es_mx: "En ejecución",
            ja_jp: "実行中",
            ar_sa: "قيد التشغيل",
        },
        TermSeed {
            term_key: "attention.badge.held_or_suppressed_count",
            domain: JobRowBadge,
            severity_rank: Informational,
            action_order_index: 7,
            durable: true,
            source_term: "Held or suppressed",
            definition: "Badge counting held or suppressed items.",
            es_mx: "Retenido o silenciado",
            ja_jp: "保留または抑制",
            ar_sa: "محجوز أو مكتوم",
        },
        TermSeed {
            term_key: "attention.badge.completion_unread",
            domain: JobRowBadge,
            severity_rank: Success,
            action_order_index: 8,
            durable: true,
            source_term: "Completed (unread)",
            definition: "Badge counting unread completions.",
            es_mx: "Completado (sin leer)",
            ja_jp: "完了（未読）",
            ar_sa: "اكتمل (غير مقروء)",
        },
        // ---- Quiet-hours / focus / admin modes ----
        TermSeed {
            term_key: "attention.quiet_hours.none",
            domain: QuietHoursMode,
            severity_rank: Informational,
            action_order_index: 0,
            durable: false,
            source_term: "Notifications on",
            definition: "No quiet-hours or focus mode is active.",
            es_mx: "Notificaciones activadas",
            ja_jp: "通知オン",
            ar_sa: "الإشعارات مفعّلة",
        },
        TermSeed {
            term_key: "attention.quiet_hours.quiet_hours",
            domain: QuietHoursMode,
            severity_rank: Informational,
            action_order_index: 1,
            durable: false,
            source_term: "Quiet hours",
            definition: "User quiet-hours mode is active.",
            es_mx: "Horario de silencio",
            ja_jp: "サイレント時間",
            ar_sa: "ساعات الهدوء",
        },
        TermSeed {
            term_key: "attention.quiet_hours.do_not_disturb",
            domain: QuietHoursMode,
            severity_rank: Informational,
            action_order_index: 2,
            durable: false,
            source_term: "Do not disturb",
            definition: "Do-not-disturb mode is active.",
            es_mx: "No molestar",
            ja_jp: "応答不可",
            ar_sa: "عدم الإزعاج",
        },
        TermSeed {
            term_key: "attention.quiet_hours.focus_mode",
            domain: QuietHoursMode,
            severity_rank: Informational,
            action_order_index: 3,
            durable: false,
            source_term: "Focus mode",
            definition: "Focus mode is active.",
            es_mx: "Modo de concentración",
            ja_jp: "フォーカスモード",
            ar_sa: "وضع التركيز",
        },
        TermSeed {
            term_key: "attention.quiet_hours.presentation",
            domain: QuietHoursMode,
            severity_rank: Informational,
            action_order_index: 4,
            durable: false,
            source_term: "Presentation mode",
            definition: "Presentation mode is active.",
            es_mx: "Modo de presentación",
            ja_jp: "プレゼンテーションモード",
            ar_sa: "وضع العرض",
        },
        TermSeed {
            term_key: "attention.quiet_hours.screen_share",
            domain: QuietHoursMode,
            severity_rank: Informational,
            action_order_index: 5,
            durable: false,
            source_term: "Screen sharing",
            definition: "Screen sharing is active.",
            es_mx: "Uso compartido de pantalla",
            ja_jp: "画面共有",
            ar_sa: "مشاركة الشاشة",
        },
        TermSeed {
            term_key: "attention.quiet_hours.privacy_mode",
            domain: QuietHoursMode,
            severity_rank: Informational,
            action_order_index: 6,
            durable: false,
            source_term: "Privacy mode",
            definition: "Privacy mode is active.",
            es_mx: "Modo de privacidad",
            ja_jp: "プライバシーモード",
            ar_sa: "وضع الخصوصية",
        },
        TermSeed {
            term_key: "attention.quiet_hours.reduced_attention_policy",
            domain: QuietHoursMode,
            severity_rank: Informational,
            action_order_index: 7,
            durable: false,
            source_term: "Reduced attention (policy)",
            definition: "A policy reduced attention posture is active.",
            es_mx: "Atención reducida (política)",
            ja_jp: "注意軽減（ポリシー）",
            ar_sa: "انتباه مخفّض (سياسة)",
        },
        TermSeed {
            term_key: "attention.quiet_hours.power_saver",
            domain: QuietHoursMode,
            severity_rank: Informational,
            action_order_index: 8,
            durable: false,
            source_term: "Power saver",
            definition: "A runtime power-saver posture is active.",
            es_mx: "Ahorro de energía",
            ja_jp: "省電力",
            ar_sa: "توفير الطاقة",
        },
        TermSeed {
            term_key: "attention.quiet_hours.admin_suppression",
            domain: QuietHoursMode,
            severity_rank: NeedsAttention,
            action_order_index: 9,
            durable: false,
            source_term: "Administrator suppression",
            definition: "An administrator suppression mode is active.",
            es_mx: "Supresión del administrador",
            ja_jp: "管理者による抑制",
            ar_sa: "كتم بواسطة المسؤول",
        },
        // ---- Suppression reasons ----
        TermSeed {
            term_key: "attention.suppression.not_suppressed",
            domain: SuppressionReason,
            severity_rank: Informational,
            action_order_index: 0,
            durable: true,
            source_term: "Delivered",
            definition: "The notification was delivered, not suppressed.",
            es_mx: "Entregado",
            ja_jp: "配信済み",
            ar_sa: "تم التسليم",
        },
        TermSeed {
            term_key: "attention.suppression.held_quiet_hours",
            domain: SuppressionReason,
            severity_rank: Informational,
            action_order_index: 1,
            durable: true,
            source_term: "Held during quiet hours",
            definition: "Held because quiet hours are active; released afterward.",
            es_mx: "Retenido en horario de silencio",
            ja_jp: "サイレント時間中に保留",
            ar_sa: "محجوز خلال ساعات الهدوء",
        },
        TermSeed {
            term_key: "attention.suppression.suppressed_by_policy",
            domain: SuppressionReason,
            severity_rank: NeedsAttention,
            action_order_index: 2,
            durable: true,
            source_term: "Suppressed by policy",
            definition: "Suppressed by an active policy posture.",
            es_mx: "Suprimido por política",
            ja_jp: "ポリシーにより抑制",
            ar_sa: "مكتوم بموجب السياسة",
        },
        TermSeed {
            term_key: "attention.suppression.admin_suppression",
            domain: SuppressionReason,
            severity_rank: NeedsAttention,
            action_order_index: 3,
            durable: true,
            source_term: "Suppressed by administrator",
            definition: "Suppressed by an administrator policy.",
            es_mx: "Suprimido por el administrador",
            ja_jp: "管理者により抑制",
            ar_sa: "مكتوم بواسطة المسؤول",
        },
        TermSeed {
            term_key: "attention.suppression.deduped",
            domain: SuppressionReason,
            severity_rank: Informational,
            action_order_index: 4,
            durable: true,
            source_term: "Combined with a repeat",
            definition: "Collapsed into a repeated or grouped event.",
            es_mx: "Combinado con una repetición",
            ja_jp: "重複と統合",
            ar_sa: "مدمج مع تكرار",
        },
        TermSeed {
            term_key: "attention.suppression.muted_by_user",
            domain: SuppressionReason,
            severity_rank: Informational,
            action_order_index: 5,
            durable: true,
            source_term: "Muted",
            definition: "Muted by the user for this class.",
            es_mx: "Silenciado",
            ja_jp: "ミュート済み",
            ar_sa: "مكتوم",
        },
        TermSeed {
            term_key: "attention.suppression.snoozed_by_user",
            domain: SuppressionReason,
            severity_rank: Informational,
            action_order_index: 6,
            durable: true,
            source_term: "Snoozed",
            definition: "Snoozed by the user.",
            es_mx: "Pospuesto",
            ja_jp: "スヌーズ済み",
            ar_sa: "مؤجّل",
        },
        TermSeed {
            term_key: "attention.suppression.reduced_attention",
            domain: SuppressionReason,
            severity_rank: Informational,
            action_order_index: 7,
            durable: true,
            source_term: "Reduced attention",
            definition: "Held under a reduced-attention posture.",
            es_mx: "Atención reducida",
            ja_jp: "注意軽減",
            ar_sa: "انتباه مخفّض",
        },
        TermSeed {
            term_key: "attention.suppression.power_saver_paused",
            domain: SuppressionReason,
            severity_rank: Informational,
            action_order_index: 8,
            durable: true,
            source_term: "Paused to save power",
            definition: "Paused by a runtime power-saver posture.",
            es_mx: "En pausa para ahorrar energía",
            ja_jp: "省電力のため一時停止",
            ar_sa: "موقوف مؤقتًا لتوفير الطاقة",
        },
        TermSeed {
            term_key: "attention.suppression.released_from_hold",
            domain: SuppressionReason,
            severity_rank: Informational,
            action_order_index: 9,
            durable: true,
            source_term: "Released from hold",
            definition: "Released from a hold to the next unsuppressed surface.",
            es_mx: "Liberado de la retención",
            ja_jp: "保留から解放",
            ar_sa: "أُفرج عنه من الحجز",
        },
        TermSeed {
            term_key: "attention.suppression.critical_bypass",
            domain: SuppressionReason,
            severity_rank: Critical,
            action_order_index: 10,
            durable: true,
            source_term: "Delivered (critical override)",
            definition: "Delivered despite suppression because it is critical.",
            es_mx: "Entregado (anulación crítica)",
            ja_jp: "配信（重大な優先）",
            ar_sa: "تم التسليم (تجاوز حرج)",
        },
    ]
}

#[cfg(test)]
mod tests;
