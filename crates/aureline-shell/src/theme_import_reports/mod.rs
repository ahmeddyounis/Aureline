//! Imported-theme mapping reports, unresolved-slot counts, syntax-coverage
//! notes, and rollback refs for the M5 migration center.
//!
//! Switching users bring themes with them — a VS Code color theme, a JetBrains
//! scheme, a Vim colorscheme, a legacy TextMate `tmTheme`. Importing one is only
//! honest if the product can show, before the user trusts the result, what
//! translated cleanly, what stayed approximate, and what did not map at all.
//! Aureline never implies parity when slots are unresolved, when syntax coverage
//! is partial, or when a fallback path changed semantic meaning.
//!
//! This module projects every imported theme as one [`ThemeImportRow`] that
//! carries the import-report contract the migration center, support/export, and
//! sync/import surfaces all consume rather than each rephrasing import truth:
//!
//! - the **source provenance** ([`SourceTool`]): the source ecosystem, tool
//!   name, tool version, and an opaque source-theme identifier;
//! - the **translated token count** and the explicit **unresolved-slot count**
//!   ([`MappingSummary`]), so a partial mapping can never read as full;
//! - the **syntax-token coverage** ([`SyntaxCoverage`]) as translated /
//!   substituted / unresolved / blocked scope counts plus a coverage percent;
//! - a **parity note** and a controlled [`ParityClaimState`] so a row only
//!   claims full parity when zero slots are unresolved and zero honesty checks
//!   are blocked;
//! - a **rollback ref** ([`RollbackRef`]) for every imported visual
//!   customization, so an import that proves incompatible or semantically
//!   misleading is always reversible; and
//! - the controlled [`ImportOutcomeState`] the migration center routes on, plus
//!   disclosed unresolved slots and known deviations.
//!
//! The records are inspectable, serde-serializable truth packets that carry no
//! raw theme files, raw token values, raw screenshots, raw paths, or raw user
//! content — only opaque refs, closed vocabulary, counts, and short labels. They
//! are consumed by the live migration center, the headless inspector
//! (`aureline_shell_m5_theme_import_reports`), the support-export wrapper, the
//! docs page under `docs/m5/theme-import-and-rollback.md`, the published report
//! under `artifacts/ux/m5/theme-import-reports/`, and the boundary schema
//! `schemas/ux/m5-theme-import-report.schema.json`. The closed appearance
//! vocabulary ([`SourceEcosystem`], [`ThemeMappingState`], [`ParityClaimState`],
//! [`ImportOutcomeState`], [`RollbackPathClass`]) mirrors the already-frozen
//! `schemas/ux/theme_import_report.schema.json`; this lane mints no parallel
//! appearance vocabulary.
//!
//! The seeded projection is deterministic so the checked-in fixtures under
//! `fixtures/ux/m5/theme-import-corpus/` are bit-for-bit equal to the output of
//! [`seeded_theme_import_report`].

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every record.
pub const M5_THEME_IMPORT_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by migration center, CLI, docs, and support.
pub const M5_THEME_IMPORT_SHARED_CONTRACT_REF: &str = "shell:m5_theme_import_report:v1";

/// Stable record kind for [`ThemeImportReport`] payloads.
pub const M5_THEME_IMPORT_REPORT_RECORD_KIND: &str = "shell_m5_theme_import_report_record";

/// Stable record kind for [`ThemeImportRow`] payloads.
pub const M5_THEME_IMPORT_ROW_RECORD_KIND: &str = "shell_m5_theme_import_report_row_record";

/// Stable record kind for [`ThemeImportSupportExport`] payloads.
pub const M5_THEME_IMPORT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_theme_import_report_support_export_record";

/// Stable report id used to pivot across surfaces.
pub const M5_THEME_IMPORT_REPORT_ID: &str = "shell:m5_theme_import_report:v1:default";

/// Repo-relative ref to the boundary schema this report conforms to.
pub const M5_THEME_IMPORT_SOURCE_SCHEMA_REF: &str = "schemas/ux/m5-theme-import-report.schema.json";

/// Published markdown artifact ref reviewers reopen the report from.
pub const M5_THEME_IMPORT_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md";

/// Published companion doc ref.
pub const M5_THEME_IMPORT_PUBLISHED_DOC_REF: &str = "docs/m5/theme-import-and-rollback.md";

/// Deterministic generated-at value carried by the seeded report.
const GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// Source ecosystem an imported theme came from.
///
/// Re-exported from `schemas/ux/theme_import_report.schema.json`
/// (`source_ecosystem_class`) without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceEcosystem {
    /// A VS Code color theme.
    #[serde(rename = "vscode")]
    VsCode,
    /// A JetBrains IDE color scheme.
    #[serde(rename = "jetbrains")]
    JetBrains,
    /// A Vim or Neovim colorscheme.
    Vim,
    /// An Emacs theme.
    Emacs,
    /// A Zed theme.
    Zed,
    /// A Sublime Text color scheme.
    Sublime,
    /// A legacy TextMate `tmTheme`.
    #[serde(rename = "textmate")]
    TextMate,
    /// The source ecosystem could not be determined.
    Unknown,
}

impl SourceEcosystem {
    /// Returns the stable schema token for this ecosystem.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VsCode => "vscode",
            Self::JetBrains => "jetbrains",
            Self::Vim => "vim",
            Self::Emacs => "emacs",
            Self::Zed => "zed",
            Self::Sublime => "sublime",
            Self::TextMate => "textmate",
            Self::Unknown => "unknown",
        }
    }
}

/// Controlled per-slot mapping state for an imported theme.
///
/// Re-exported from `schemas/ux/theme_import_report.schema.json`
/// (`mapping_state`) without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMappingState {
    /// The source slot translated to a native token.
    Translated,
    /// The source slot was substituted with a disclosed fallback token.
    SubstitutedFallback,
    /// The source slot has no Aureline target and is unsupported.
    Unsupported,
    /// The source slot could not be resolved and is disclosed, not defaulted.
    Unresolved,
    /// The mapping was blocked to keep a protected cue honest.
    BlockedHonesty,
    /// The source slot maps onto a deprecated token with a disclosed
    /// replacement.
    DeprecatedReplacement,
}

impl ThemeMappingState {
    /// Returns the stable schema token for this mapping state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Translated => "translated",
            Self::SubstitutedFallback => "substituted_fallback",
            Self::Unsupported => "unsupported",
            Self::Unresolved => "unresolved",
            Self::BlockedHonesty => "blocked_honesty",
            Self::DeprecatedReplacement => "deprecated_replacement",
        }
    }
}

/// Controlled migration-center outcome state for an imported theme.
///
/// Re-exported from `schemas/ux/theme_import_report.schema.json`
/// (`import_outcome_state`) without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportOutcomeState {
    /// The import is previewed and ready to apply.
    PreviewReady,
    /// The import applied cleanly.
    Applied,
    /// The import applied with disclosed warnings.
    AppliedWithWarnings,
    /// The import was blocked before any durable change.
    Blocked,
    /// The import applied and was then rolled back.
    RolledBack,
    /// The user cancelled the import.
    Cancelled,
    /// Policy denied the import.
    PolicyDenied,
    /// The import needs explicit human review before it can apply.
    ReviewRequired,
}

impl ImportOutcomeState {
    /// Returns the stable schema token for this outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewReady => "preview_ready",
            Self::Applied => "applied",
            Self::AppliedWithWarnings => "applied_with_warnings",
            Self::Blocked => "blocked",
            Self::RolledBack => "rolled_back",
            Self::Cancelled => "cancelled",
            Self::PolicyDenied => "policy_denied",
            Self::ReviewRequired => "review_required",
        }
    }

    /// Returns `true` when the outcome implies durable state was written and so
    /// must be backed by a reversible rollback path.
    pub const fn implies_durable_change(self) -> bool {
        matches!(
            self,
            Self::Applied | Self::AppliedWithWarnings | Self::RolledBack
        )
    }

    /// Returns `true` when the outcome escalates to support and offline review
    /// (blocked, rolled back, policy-denied, or review-required).
    pub const fn is_escalation(self) -> bool {
        matches!(
            self,
            Self::Blocked | Self::RolledBack | Self::PolicyDenied | Self::ReviewRequired
        )
    }
}

/// Controlled parity-claim state for an imported theme.
///
/// Re-exported from `schemas/ux/theme_import_report.schema.json`
/// (`parity_claim_state`) without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityClaimState {
    /// No parity is claimed.
    NotClaimed,
    /// Full parity is claimed and backed by this report.
    ClaimedWithReport,
    /// Parity is claimed only partially, with disclosed gaps.
    PartialClaimWithGaps,
    /// Parity is denied because slots are unresolved or honesty checks blocked.
    DeniedUnresolvedOrBlocked,
}

impl ParityClaimState {
    /// Returns the stable schema token for this parity-claim state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotClaimed => "not_claimed",
            Self::ClaimedWithReport => "claimed_with_report",
            Self::PartialClaimWithGaps => "partial_claim_with_gaps",
            Self::DeniedUnresolvedOrBlocked => "denied_unresolved_or_blocked",
        }
    }

    /// Returns `true` when this state asserts full parity.
    pub const fn claims_full_parity(self) -> bool {
        matches!(self, Self::ClaimedWithReport)
    }
}

/// Controlled rollback-path class for an imported theme.
///
/// Re-exported from `schemas/ux/theme_import_report.schema.json`
/// (`rollback_path_class`) without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPathClass {
    /// Restore the appearance checkpoint minted before apply.
    RestoreAppearanceCheckpoint,
    /// Discard the preview without applying anything.
    DiscardPreview,
    /// Reopen the import review to re-decide unresolved slots.
    ReopenImportReview,
    /// A manual repair step is required to recover.
    ManualRepairRequired,
    /// No rollback path is available; the row is denied.
    RollbackUnavailableDenied,
}

impl RollbackPathClass {
    /// Returns the stable schema token for this rollback-path class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreAppearanceCheckpoint => "restore_appearance_checkpoint",
            Self::DiscardPreview => "discard_preview",
            Self::ReopenImportReview => "reopen_import_review",
            Self::ManualRepairRequired => "manual_repair_required",
            Self::RollbackUnavailableDenied => "rollback_unavailable_denied",
        }
    }

    /// Returns `true` when this class leaves the import reversible.
    pub const fn is_reversible(self) -> bool {
        !matches!(self, Self::RollbackUnavailableDenied)
    }
}

/// Source provenance for an imported theme. Carries no raw theme bytes — only
/// the ecosystem, the tool name and version, and an opaque source-theme id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceTool {
    /// The source ecosystem the theme came from.
    pub source_ecosystem: SourceEcosystem,
    /// Reviewer-facing source tool name.
    pub source_tool_name: String,
    /// Reviewer-facing source tool version.
    pub source_tool_version: String,
    /// Opaque, stable source-theme identifier (no path, no content).
    pub source_theme_identifier: String,
}

/// Translated and unresolved slot counts for an imported theme. The sum of the
/// per-state counts must equal [`MappingSummary::total_source_slot_count`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingSummary {
    /// Total source slots discovered in the imported theme.
    pub total_source_slot_count: usize,
    /// Slots that translated to a native token.
    pub translated_slot_count: usize,
    /// Slots substituted with a disclosed fallback token.
    pub substituted_with_fallback_count: usize,
    /// Slots with no Aureline target.
    pub unsupported_slot_count: usize,
    /// Slots that could not be resolved and are disclosed, not defaulted.
    pub unresolved_mapping_count: usize,
    /// Slots blocked to keep a protected cue honest.
    pub blocked_honesty_count: usize,
}

impl MappingSummary {
    /// Returns `true` when the per-state counts sum to the declared total.
    pub fn counts_are_consistent(&self) -> bool {
        self.translated_slot_count
            + self.substituted_with_fallback_count
            + self.unsupported_slot_count
            + self.unresolved_mapping_count
            + self.blocked_honesty_count
            == self.total_source_slot_count
    }

    /// Returns `true` when every slot translated cleanly with no unresolved or
    /// blocked mappings — the only state that can back a full parity claim.
    pub fn is_full_parity_eligible(&self) -> bool {
        self.unresolved_mapping_count == 0
            && self.blocked_honesty_count == 0
            && self.unsupported_slot_count == 0
            && self.substituted_with_fallback_count == 0
            && self.translated_slot_count == self.total_source_slot_count
    }
}

/// Syntax-token coverage for an imported theme, as scope counts plus a derived
/// coverage percent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntaxCoverage {
    /// Total source syntax scopes discovered.
    pub total_source_scope_count: usize,
    /// Scopes that translated to a native syntax token.
    pub translated_scope_count: usize,
    /// Scopes substituted with a disclosed fallback.
    pub substituted_scope_count: usize,
    /// Scopes that could not be resolved.
    pub unresolved_scope_count: usize,
    /// Scopes blocked to keep a protected cue honest.
    pub blocked_scope_count: usize,
    /// Coverage percent, computed as `translated * 100 / total` (integer
    /// division), or `100` when there are no source scopes.
    pub coverage_percent: u8,
}

impl SyntaxCoverage {
    /// Computes the canonical coverage percent for a translated/total pair.
    pub fn expected_percent(total: usize, translated: usize) -> u8 {
        if total == 0 {
            100
        } else {
            ((translated * 100) / total) as u8
        }
    }

    /// Returns `true` when the scope counts fit within the total and the stored
    /// percent equals the canonical computation.
    pub fn is_consistent(&self) -> bool {
        let parts = self.translated_scope_count
            + self.substituted_scope_count
            + self.unresolved_scope_count
            + self.blocked_scope_count;
        parts <= self.total_source_scope_count
            && self.coverage_percent
                == Self::expected_percent(
                    self.total_source_scope_count,
                    self.translated_scope_count,
                )
    }

    /// Returns `true` when every source scope translated with no unresolved or
    /// blocked scopes.
    pub fn is_full_coverage(&self) -> bool {
        self.unresolved_scope_count == 0
            && self.blocked_scope_count == 0
            && self.substituted_scope_count == 0
            && self.translated_scope_count == self.total_source_scope_count
    }
}

/// The rollback ref that protects an imported visual customization. Every
/// imported theme carries one so an incompatible or misleading import is
/// reversible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackRef {
    /// The controlled rollback-path class.
    pub rollback_path_class: RollbackPathClass,
    /// Appearance checkpoint minted before apply, when one exists.
    pub checkpoint_ref: Option<String>,
    /// Opaque, stable rollback ref quoted across surfaces.
    pub rollback_ref: String,
    /// User-visible action id that performs the rollback.
    pub user_visible_action_id: String,
    /// Reviewer-facing note about the rollback path.
    pub note: Option<String>,
}

impl RollbackRef {
    /// Returns `true` when the rollback ref leaves the import reversible: the
    /// class is reversible and a non-empty rollback ref is present.
    pub fn is_reversible(&self) -> bool {
        self.rollback_path_class.is_reversible() && !self.rollback_ref.trim().is_empty()
    }
}

/// One disclosed unresolved slot. The migration center lists these so an
/// unresolved mapping is never silently defaulted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnresolvedSlot {
    /// Opaque source-slot ref (no raw value).
    pub source_slot_ref: String,
    /// Reviewer-facing summary of what stayed unresolved.
    pub summary: String,
    /// True when a disclosed fallback is offered for this slot in preview.
    pub fallback_disclosed: bool,
}

/// A disclosed deviation from a clean translation for an imported theme.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownDeviation {
    /// Stable deviation id quoted across surfaces.
    pub deviation_id: String,
    /// Reviewer-facing summary of what does not translate cleanly.
    pub summary: String,
    /// True when the deviation is recoverable by a documented follow-up.
    pub recoverable: bool,
}

/// One imported theme projected as a migration-center import-report row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeImportRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable row id quoted across surfaces.
    pub row_id: String,
    /// Reviewer-facing title for the row.
    pub title: String,
    /// Source provenance for the imported theme.
    pub source_tool: SourceTool,
    /// Opaque source artifact ref (no path, no content).
    pub source_artifact_ref: String,
    /// Target theme-package ref, when the import produced or targets one.
    pub target_theme_package_ref: Option<String>,
    /// Controlled migration-center outcome.
    pub import_outcome: ImportOutcomeState,
    /// Headline mapping state describing the overall translation.
    pub primary_mapping_state: ThemeMappingState,
    /// Translated and unresolved slot counts.
    pub mapping_summary: MappingSummary,
    /// Syntax-token coverage.
    pub syntax_coverage: SyntaxCoverage,
    /// Controlled parity-claim state; never widened beyond what is proven.
    pub parity_claim_state: ParityClaimState,
    /// Reviewer-facing parity note shown before the user trusts the result.
    pub parity_note: String,
    /// True when applying the row mutates durable appearance state.
    pub mutates_durable_state: bool,
    /// Rollback ref protecting the imported visual customization.
    pub rollback: RollbackRef,
    /// Reviewer-facing compatibility note. Required whenever full parity is not
    /// claimed.
    pub compatibility_note: Option<String>,
    /// Disclosed unresolved slots. Required whenever the unresolved count is
    /// non-zero.
    pub unresolved_slots: Vec<UnresolvedSlot>,
    /// Known deviations from a clean translation.
    pub known_deviations: Vec<KnownDeviation>,
    /// Docs/help refs that publish the row.
    pub docs_help_refs: Vec<String>,
    /// Reviewer-facing narrative summary.
    pub narrative: String,
}

impl ThemeImportRow {
    /// Returns the translated token (slot) count.
    pub fn translated_token_count(&self) -> usize {
        self.mapping_summary.translated_slot_count
    }

    /// Returns the explicit unresolved-slot count.
    pub fn unresolved_slot_count(&self) -> usize {
        self.mapping_summary.unresolved_mapping_count
    }

    /// Returns `true` when the row claims full parity.
    pub fn claims_full_parity(&self) -> bool {
        self.parity_claim_state.claims_full_parity()
    }

    /// Returns `true` when a full parity claim is actually backed by clean
    /// counts: every slot translated, full syntax coverage, and a translated
    /// headline mapping state.
    pub fn full_parity_is_backed(&self) -> bool {
        self.mapping_summary.is_full_parity_eligible()
            && self.syntax_coverage.is_full_coverage()
            && self.primary_mapping_state == ThemeMappingState::Translated
    }

    /// Returns `true` when the imported customization is reversible: every row
    /// carries a rollback ref, and a row that wrote durable state carries a
    /// reversible one.
    pub fn import_is_reversible(&self) -> bool {
        if self.mutates_durable_state || self.import_outcome.implies_durable_change() {
            self.rollback.is_reversible()
        } else {
            !self.rollback.rollback_ref.trim().is_empty()
        }
    }

    /// Returns deterministic compact lines for text review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "{} [{}/{}]",
                self.title,
                self.source_tool.source_ecosystem.as_str(),
                self.source_tool.source_tool_version
            ),
            format!(
                "  outcome={} mapping={} parity={}",
                self.import_outcome.as_str(),
                self.primary_mapping_state.as_str(),
                self.parity_claim_state.as_str()
            ),
            format!(
                "  translated={}/{} unresolved={} syntax_coverage={}%",
                self.mapping_summary.translated_slot_count,
                self.mapping_summary.total_source_slot_count,
                self.mapping_summary.unresolved_mapping_count,
                self.syntax_coverage.coverage_percent
            ),
            format!(
                "  rollback={} reversible={}",
                self.rollback.rollback_path_class.as_str(),
                self.import_is_reversible()
            ),
        ];
        if let Some(note) = &self.compatibility_note {
            lines.push(format!("  compatibility_note: {note}"));
        }
        for slot in &self.unresolved_slots {
            lines.push(format!("  unresolved: {}", slot.summary));
        }
        lines
    }
}

/// Grouped counts for the eight controlled outcome states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeSummary {
    /// Number of `preview_ready` rows.
    pub preview_ready: usize,
    /// Number of `applied` rows.
    pub applied: usize,
    /// Number of `applied_with_warnings` rows.
    pub applied_with_warnings: usize,
    /// Number of `blocked` rows.
    pub blocked: usize,
    /// Number of `rolled_back` rows.
    pub rolled_back: usize,
    /// Number of `cancelled` rows.
    pub cancelled: usize,
    /// Number of `policy_denied` rows.
    pub policy_denied: usize,
    /// Number of `review_required` rows.
    pub review_required: usize,
    /// Total number of rows.
    pub total_rows: usize,
}

impl OutcomeSummary {
    fn from_rows(rows: &[ThemeImportRow]) -> Self {
        let mut summary = Self {
            preview_ready: 0,
            applied: 0,
            applied_with_warnings: 0,
            blocked: 0,
            rolled_back: 0,
            cancelled: 0,
            policy_denied: 0,
            review_required: 0,
            total_rows: rows.len(),
        };
        for row in rows {
            match row.import_outcome {
                ImportOutcomeState::PreviewReady => summary.preview_ready += 1,
                ImportOutcomeState::Applied => summary.applied += 1,
                ImportOutcomeState::AppliedWithWarnings => summary.applied_with_warnings += 1,
                ImportOutcomeState::Blocked => summary.blocked += 1,
                ImportOutcomeState::RolledBack => summary.rolled_back += 1,
                ImportOutcomeState::Cancelled => summary.cancelled += 1,
                ImportOutcomeState::PolicyDenied => summary.policy_denied += 1,
                ImportOutcomeState::ReviewRequired => summary.review_required += 1,
            }
        }
        summary
    }
}

/// Aggregate translated and unresolved token counts across every row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AggregateTokenSummary {
    /// Total source slots across all rows.
    pub total_source_slots: usize,
    /// Total translated slots across all rows.
    pub total_translated_slots: usize,
    /// Total unresolved slots across all rows.
    pub total_unresolved_slots: usize,
    /// Total blocked-honesty slots across all rows.
    pub total_blocked_slots: usize,
}

impl AggregateTokenSummary {
    fn from_rows(rows: &[ThemeImportRow]) -> Self {
        let mut summary = Self {
            total_source_slots: 0,
            total_translated_slots: 0,
            total_unresolved_slots: 0,
            total_blocked_slots: 0,
        };
        for row in rows {
            summary.total_source_slots += row.mapping_summary.total_source_slot_count;
            summary.total_translated_slots += row.mapping_summary.translated_slot_count;
            summary.total_unresolved_slots += row.mapping_summary.unresolved_mapping_count;
            summary.total_blocked_slots += row.mapping_summary.blocked_honesty_count;
        }
        summary
    }
}

fn ecosystem_coverage_from_rows(rows: &[ThemeImportRow]) -> Vec<SourceEcosystem> {
    let mut covered = Vec::new();
    for row in rows {
        let ecosystem = row.source_tool.source_ecosystem;
        if !covered.contains(&ecosystem) {
            covered.push(ecosystem);
        }
    }
    covered
}

/// Migration-center imported-theme report. Shared by the migration center, the
/// support-export wrapper, compatibility packets, and sync/import flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeImportReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the report.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable report id used to pivot across surfaces.
    pub report_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// True when the migration center previews every import before apply.
    pub preview_before_apply: bool,
    /// Imported-theme rows in canonical order.
    pub rows: Vec<ThemeImportRow>,
    /// Grouped outcome counts.
    pub outcome_summary: OutcomeSummary,
    /// Aggregate translated and unresolved token counts.
    pub aggregate_tokens: AggregateTokenSummary,
    /// Source ecosystems covered, in first-seen order.
    pub ecosystem_coverage: Vec<SourceEcosystem>,
    /// True when every imported customization is reversible.
    pub every_import_reversible: bool,
    /// True when no row claims full parity without backed counts.
    pub no_overclaimed_parity: bool,
    /// True when every non-zero unresolved count is disclosed with slots.
    pub unresolved_counts_disclosed: bool,
    /// True when no record carries raw theme content or raw token values.
    pub no_raw_theme_content: bool,
    /// Migration-center refs that route the report.
    pub migration_center_refs: Vec<String>,
    /// Compatibility-report refs published downstream.
    pub compatibility_report_refs: Vec<String>,
    /// Release / public-truth pack refs that consume the report.
    pub release_truth_refs: Vec<String>,
    /// Sync / import refs that preserve provenance and unresolved counts.
    pub sync_refs: Vec<String>,
    /// Readiness review refs that consume the report.
    pub readiness_review_refs: Vec<String>,
    /// Docs/help refs the report reopens from.
    pub docs_help_refs: Vec<String>,
    /// Stable export refs carrying the report into support and offline review.
    pub export_refs: Vec<String>,
    /// Support packet refs that preserve the report.
    pub support_packet_refs: Vec<String>,
    /// Published markdown artifact ref.
    pub published_report_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ThemeImportReport {
    /// Returns the row count for the report.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the total unresolved-slot count across every row.
    pub fn total_unresolved_slot_count(&self) -> usize {
        self.aggregate_tokens.total_unresolved_slots
    }

    /// Returns `true` when the report is clean: every invariant flag holds.
    pub fn is_clean(&self) -> bool {
        self.every_import_reversible
            && self.no_overclaimed_parity
            && self.unresolved_counts_disclosed
            && self.no_raw_theme_content
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "report: id={}, rows={}, ecosystems={}",
            self.report_id,
            self.rows.len(),
            self.ecosystem_coverage.len(),
        ));
        lines.push(format!(
            "translated={}/{} unresolved={} blocked={}",
            self.aggregate_tokens.total_translated_slots,
            self.aggregate_tokens.total_source_slots,
            self.aggregate_tokens.total_unresolved_slots,
            self.aggregate_tokens.total_blocked_slots,
        ));
        lines.push(format!(
            "every_import_reversible={} no_overclaimed_parity={} unresolved_counts_disclosed={}",
            self.every_import_reversible,
            self.no_overclaimed_parity,
            self.unresolved_counts_disclosed,
        ));
        for row in &self.rows {
            lines.extend(row.compact_lines());
        }
        lines
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 imported-theme mapping & rollback report\n\n");
        out.push_str(
            "Generated from the seeded report in\n\
             [`crate::theme_import_reports`](../../../../crates/aureline-shell/src/theme_import_reports/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- markdown > \\\n  artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!("- Rows: {}\n", self.rows.len()));
        out.push_str(&format!(
            "- Ecosystems covered: {}\n",
            self.ecosystem_coverage
                .iter()
                .map(|e| e.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Translated slots: {}/{}\n",
            self.aggregate_tokens.total_translated_slots, self.aggregate_tokens.total_source_slots
        ));
        out.push_str(&format!(
            "- Unresolved slots: {}\n",
            self.aggregate_tokens.total_unresolved_slots
        ));
        out.push_str(&format!(
            "- Preview before apply: {}\n",
            self.preview_before_apply
        ));
        out.push_str(&format!(
            "- Every import reversible: {}\n",
            self.every_import_reversible
        ));
        out.push_str(&format!(
            "- No overclaimed parity: {}\n",
            self.no_overclaimed_parity
        ));
        out.push_str(&format!(
            "- Unresolved counts disclosed: {}\n",
            self.unresolved_counts_disclosed
        ));
        out.push_str(&format!(
            "- No raw theme content: {}\n",
            self.no_raw_theme_content
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Outcome summary\n\n");
        out.push_str("| Outcome | Count |\n|---|---:|\n");
        out.push_str(&format!(
            "| preview_ready | {} |\n",
            self.outcome_summary.preview_ready
        ));
        out.push_str(&format!("| applied | {} |\n", self.outcome_summary.applied));
        out.push_str(&format!(
            "| applied_with_warnings | {} |\n",
            self.outcome_summary.applied_with_warnings
        ));
        out.push_str(&format!("| blocked | {} |\n", self.outcome_summary.blocked));
        out.push_str(&format!(
            "| rolled_back | {} |\n",
            self.outcome_summary.rolled_back
        ));
        out.push_str(&format!(
            "| cancelled | {} |\n",
            self.outcome_summary.cancelled
        ));
        out.push_str(&format!(
            "| policy_denied | {} |\n",
            self.outcome_summary.policy_denied
        ));
        out.push_str(&format!(
            "| review_required | {} |\n",
            self.outcome_summary.review_required
        ));
        out.push_str(&format!(
            "| **total** | **{}** |\n\n",
            self.outcome_summary.total_rows
        ));

        out.push_str("## Imported themes\n\n");
        out.push_str(
            "| Theme | Source | Outcome | Translated | Unresolved | Syntax | Parity | Reversible |\n",
        );
        out.push_str("|---|---|---|---:|---:|---:|---|:---:|\n");
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` {} | `{}` | {}/{} | {} | {}% | `{}` | {} |\n",
                row.title,
                row.source_tool.source_ecosystem.as_str(),
                row.source_tool.source_tool_version,
                row.import_outcome.as_str(),
                row.mapping_summary.translated_slot_count,
                row.mapping_summary.total_source_slot_count,
                row.mapping_summary.unresolved_mapping_count,
                row.syntax_coverage.coverage_percent,
                row.parity_claim_state.as_str(),
                if row.import_is_reversible() {
                    "yes"
                } else {
                    "NO"
                },
            ));
        }
        out.push('\n');

        for row in &self.rows {
            out.push_str(&format!(
                "## {} (`{}`)\n\n",
                row.title,
                row.source_tool.source_ecosystem.as_str()
            ));
            out.push_str(&format!("{}\n\n", row.narrative));
            out.push_str(&format!(
                "- Source: {} {} (`{}`)\n",
                row.source_tool.source_tool_name,
                row.source_tool.source_tool_version,
                row.source_tool.source_theme_identifier
            ));
            out.push_str(&format!("- Outcome: `{}`\n", row.import_outcome.as_str()));
            out.push_str(&format!(
                "- Parity: `{}` — {}\n",
                row.parity_claim_state.as_str(),
                row.parity_note
            ));
            out.push_str(&format!(
                "- Translated slots: {}/{}\n",
                row.mapping_summary.translated_slot_count,
                row.mapping_summary.total_source_slot_count
            ));
            out.push_str(&format!(
                "- Unresolved slots: {}\n",
                row.mapping_summary.unresolved_mapping_count
            ));
            out.push_str(&format!(
                "- Syntax coverage: {}% ({}/{} scopes)\n",
                row.syntax_coverage.coverage_percent,
                row.syntax_coverage.translated_scope_count,
                row.syntax_coverage.total_source_scope_count
            ));
            out.push_str(&format!(
                "- Rollback: `{}` (`{}`)\n",
                row.rollback.rollback_path_class.as_str(),
                row.rollback.rollback_ref
            ));
            if let Some(note) = &row.compatibility_note {
                out.push_str(&format!("- Compatibility note: {note}\n"));
            }
            if !row.unresolved_slots.is_empty() {
                out.push_str("- Unresolved slots:\n");
                for slot in &row.unresolved_slots {
                    out.push_str(&format!(
                        "  - `{}` — {} (fallback disclosed: {})\n",
                        slot.source_slot_ref, slot.summary, slot.fallback_disclosed
                    ));
                }
            }
            if !row.known_deviations.is_empty() {
                out.push_str("- Known deviations:\n");
                for deviation in &row.known_deviations {
                    out.push_str(&format!(
                        "  - `{}` — {} (recoverable: {})\n",
                        deviation.deviation_id, deviation.summary, deviation.recoverable
                    ));
                }
            }
            out.push('\n');
        }

        out
    }
}

/// Support-export wrapper that quotes the report plus every stable id reviewers
/// need to pivot across surfaces — including source provenance and rollback
/// refs so support and sync/import never lose them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeImportSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the wrapper.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Report quoted in full.
    pub report: ThemeImportReport,
    /// Stable report id, row ids, source-theme identifiers, checkpoint refs, and
    /// rollback refs in deterministic order.
    pub case_ids: Vec<String>,
}

impl ThemeImportSupportExport {
    /// Builds the support-export wrapper for a report. The case ids quote the
    /// report id, every row id, every source-theme identifier (provenance), and
    /// every checkpoint and rollback ref, so support and sync/import preserve
    /// provenance and unresolved-slot counts.
    pub fn from_report(support_export_id: impl Into<String>, report: ThemeImportReport) -> Self {
        let mut case_ids = Vec::new();
        case_ids.push(report.report_id.clone());
        for row in &report.rows {
            case_ids.push(row.row_id.clone());
        }
        for row in &report.rows {
            case_ids.push(row.source_tool.source_theme_identifier.clone());
        }
        for row in &report.rows {
            if let Some(checkpoint) = &row.rollback.checkpoint_ref {
                case_ids.push(checkpoint.clone());
            }
        }
        for row in &report.rows {
            case_ids.push(row.rollback.rollback_ref.clone());
        }
        Self {
            record_kind: M5_THEME_IMPORT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_THEME_IMPORT_SCHEMA_VERSION,
            shared_contract_ref: M5_THEME_IMPORT_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Validation error produced by [`validate_theme_import_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum ThemeImportValidationError {
    /// The outcome summary does not match the rows.
    OutcomeSummaryStale,
    /// The aggregate token summary does not match the rows.
    AggregateTokensStale,
    /// The ecosystem coverage does not match the rows.
    EcosystemCoverageStale,
    /// The report does not preview imports before apply.
    PreviewBeforeApplyMissing,
    /// A row's mapping-summary counts do not sum to the declared total.
    MappingSummaryInconsistent {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A row's syntax-coverage counts or percent are inconsistent.
    SyntaxCoverageInconsistent {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A row that wrote durable state carries no reversible rollback ref.
    RollbackPathMissing {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A row claims full parity without backed counts.
    ParityOverclaimed {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A row with a non-zero unresolved count discloses no unresolved slots.
    UnresolvedCountHidden {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A row carries an empty parity note.
    ParityNoteMissing {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A non-full-parity row carries no compatibility note.
    CompatibilityNoteMissing {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A row carries no docs/help ref.
    DocsHelpRefMissing {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A report with escalation rows omits export or support refs.
    EscalationRefsMissing,
    /// The report does not declare a migration-center ref.
    MigrationCenterRefMissing,
    /// The report does not declare a compatibility-report ref.
    CompatibilityReportRefMissing,
    /// The report does not declare a release / public-truth pack ref.
    ReleaseTruthRefMissing,
}

/// Validates a report against the imported-theme acceptance invariants.
///
/// The checks encode the track invariant and acceptance criteria: imported
/// themes surface explicit mapping quality and unresolved-slot counts before
/// users trust the result, no row claims parity it cannot back, every imported
/// customization is reversible, and migration-center, support/export, and
/// release/public-truth surfaces share the same report object.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_theme_import_report(
    report: &ThemeImportReport,
) -> Result<(), Vec<ThemeImportValidationError>> {
    let mut errors = Vec::new();

    if OutcomeSummary::from_rows(&report.rows) != report.outcome_summary {
        errors.push(ThemeImportValidationError::OutcomeSummaryStale);
    }
    if AggregateTokenSummary::from_rows(&report.rows) != report.aggregate_tokens {
        errors.push(ThemeImportValidationError::AggregateTokensStale);
    }
    if ecosystem_coverage_from_rows(&report.rows) != report.ecosystem_coverage {
        errors.push(ThemeImportValidationError::EcosystemCoverageStale);
    }
    if !report.preview_before_apply {
        errors.push(ThemeImportValidationError::PreviewBeforeApplyMissing);
    }

    let mut has_escalation_row = false;
    for row in &report.rows {
        if !row.mapping_summary.counts_are_consistent() {
            errors.push(ThemeImportValidationError::MappingSummaryInconsistent {
                row_id: row.row_id.clone(),
            });
        }
        if !row.syntax_coverage.is_consistent() {
            errors.push(ThemeImportValidationError::SyntaxCoverageInconsistent {
                row_id: row.row_id.clone(),
            });
        }

        // Every imported visual customization must be reversible. A row that
        // wrote durable state must carry a reversible rollback ref; every row
        // must at least carry a non-empty rollback ref.
        if !row.import_is_reversible() {
            errors.push(ThemeImportValidationError::RollbackPathMissing {
                row_id: row.row_id.clone(),
            });
        }

        // A full parity claim must be backed by clean counts and coverage.
        if row.claims_full_parity() && !row.full_parity_is_backed() {
            errors.push(ThemeImportValidationError::ParityOverclaimed {
                row_id: row.row_id.clone(),
            });
        }

        // A non-zero unresolved count must be disclosed with listed slots.
        if row.mapping_summary.unresolved_mapping_count > 0 && row.unresolved_slots.is_empty() {
            errors.push(ThemeImportValidationError::UnresolvedCountHidden {
                row_id: row.row_id.clone(),
            });
        }

        if row.parity_note.trim().is_empty() {
            errors.push(ThemeImportValidationError::ParityNoteMissing {
                row_id: row.row_id.clone(),
            });
        }

        // Any row that does not claim full parity must disclose a compatibility
        // note explaining the gap.
        if !row.claims_full_parity() && row.compatibility_note.is_none() {
            errors.push(ThemeImportValidationError::CompatibilityNoteMissing {
                row_id: row.row_id.clone(),
            });
        }

        if row.docs_help_refs.is_empty() {
            errors.push(ThemeImportValidationError::DocsHelpRefMissing {
                row_id: row.row_id.clone(),
            });
        }

        if row.import_outcome.is_escalation() {
            has_escalation_row = true;
        }
    }

    if has_escalation_row
        && (report.export_refs.is_empty() || report.support_packet_refs.is_empty())
    {
        errors.push(ThemeImportValidationError::EscalationRefsMissing);
    }

    if report.migration_center_refs.is_empty() {
        errors.push(ThemeImportValidationError::MigrationCenterRefMissing);
    }
    if report.compatibility_report_refs.is_empty() {
        errors.push(ThemeImportValidationError::CompatibilityReportRefMissing);
    }
    if report.release_truth_refs.is_empty() {
        errors.push(ThemeImportValidationError::ReleaseTruthRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Builds the seeded imported-theme migration report.
pub fn seeded_theme_import_report() -> ThemeImportReport {
    let rows = seeded_rows();
    let outcome_summary = OutcomeSummary::from_rows(&rows);
    let aggregate_tokens = AggregateTokenSummary::from_rows(&rows);
    let ecosystem_coverage = ecosystem_coverage_from_rows(&rows);
    let every_import_reversible = rows.iter().all(ThemeImportRow::import_is_reversible);
    let no_overclaimed_parity = rows
        .iter()
        .all(|row| !row.claims_full_parity() || row.full_parity_is_backed());
    let unresolved_counts_disclosed = rows.iter().all(|row| {
        row.mapping_summary.unresolved_mapping_count == 0 || !row.unresolved_slots.is_empty()
    });

    ThemeImportReport {
        record_kind: M5_THEME_IMPORT_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_THEME_IMPORT_SCHEMA_VERSION,
        shared_contract_ref: M5_THEME_IMPORT_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_THEME_IMPORT_REPORT_ID.to_owned(),
        source_schema_ref: M5_THEME_IMPORT_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Imported-theme mapping quality, unresolved-slot counts, syntax coverage, and \
             rollback refs for the M5 migration center."
            .to_owned(),
        preview_before_apply: true,
        rows,
        outcome_summary,
        aggregate_tokens,
        ecosystem_coverage,
        every_import_reversible,
        no_overclaimed_parity,
        unresolved_counts_disclosed,
        no_raw_theme_content: true,
        migration_center_refs: vec![
            "migration_center.imported_theme_report".to_owned(),
            "docs/migration/migration_center_object_model.md#imported-theme-report".to_owned(),
        ],
        compatibility_report_refs: vec![
            "artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md".to_owned(),
            "docs/release/compatibility_report_template.md#ux.m5_theme_import".to_owned(),
        ],
        release_truth_refs: vec![
            "release_center.public_truth.m5_theme_import".to_owned(),
            "readiness-review:m5:imported_theme_parity".to_owned(),
        ],
        sync_refs: vec![
            "sync.appearance.imported_theme_report".to_owned(),
            "import.appearance.imported_theme_report".to_owned(),
        ],
        readiness_review_refs: vec![
            "readiness-review:m5:imported_theme_parity".to_owned(),
            "readiness-review:m5:appearance_rollback".to_owned(),
        ],
        docs_help_refs: vec![
            "docs/m5/theme-import-and-rollback.md".to_owned(),
            "docs/m5/theme-package-and-appearance-objects.md#imported-theme-parity".to_owned(),
        ],
        export_refs: vec![
            "support_bundle.m5_theme_import_report".to_owned(),
            "migration_center.machine_readable_theme_import_report".to_owned(),
        ],
        support_packet_refs: vec!["support_packet.m5_theme_import_review.default".to_owned()],
        published_report_ref: M5_THEME_IMPORT_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_THEME_IMPORT_PUBLISHED_DOC_REF.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
    }
}

struct UnresolvedSeed {
    source_slot_ref: &'static str,
    summary: &'static str,
    fallback_disclosed: bool,
}

struct RowSeed {
    row_id: &'static str,
    title: &'static str,
    source_ecosystem: SourceEcosystem,
    source_tool_name: &'static str,
    source_tool_version: &'static str,
    source_theme_identifier: &'static str,
    source_artifact_ref: &'static str,
    target_theme_package_ref: Option<&'static str>,
    import_outcome: ImportOutcomeState,
    primary_mapping_state: ThemeMappingState,
    mapping_summary: (usize, usize, usize, usize, usize, usize),
    syntax_coverage: (usize, usize, usize, usize, usize),
    parity_claim_state: ParityClaimState,
    parity_note: &'static str,
    mutates_durable_state: bool,
    rollback: (
        RollbackPathClass,
        Option<&'static str>,
        &'static str,
        &'static str,
        Option<&'static str>,
    ),
    compatibility_note: Option<&'static str>,
    unresolved_slots: &'static [UnresolvedSeed],
    known_deviations: &'static [(&'static str, &'static str, bool)],
    docs_help_refs: &'static [&'static str],
    narrative: &'static str,
}

fn build_row(seed: &RowSeed) -> ThemeImportRow {
    let (total, translated, substituted, unsupported, unresolved, blocked) = seed.mapping_summary;
    let (s_total, s_translated, s_substituted, s_unresolved, s_blocked) = seed.syntax_coverage;
    let (rollback_class, checkpoint, rollback_ref, action_id, rollback_note) = seed.rollback;

    ThemeImportRow {
        record_kind: M5_THEME_IMPORT_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_THEME_IMPORT_SCHEMA_VERSION,
        shared_contract_ref: M5_THEME_IMPORT_SHARED_CONTRACT_REF.to_owned(),
        row_id: seed.row_id.to_owned(),
        title: seed.title.to_owned(),
        source_tool: SourceTool {
            source_ecosystem: seed.source_ecosystem,
            source_tool_name: seed.source_tool_name.to_owned(),
            source_tool_version: seed.source_tool_version.to_owned(),
            source_theme_identifier: seed.source_theme_identifier.to_owned(),
        },
        source_artifact_ref: seed.source_artifact_ref.to_owned(),
        target_theme_package_ref: seed.target_theme_package_ref.map(str::to_owned),
        import_outcome: seed.import_outcome,
        primary_mapping_state: seed.primary_mapping_state,
        mapping_summary: MappingSummary {
            total_source_slot_count: total,
            translated_slot_count: translated,
            substituted_with_fallback_count: substituted,
            unsupported_slot_count: unsupported,
            unresolved_mapping_count: unresolved,
            blocked_honesty_count: blocked,
        },
        syntax_coverage: SyntaxCoverage {
            total_source_scope_count: s_total,
            translated_scope_count: s_translated,
            substituted_scope_count: s_substituted,
            unresolved_scope_count: s_unresolved,
            blocked_scope_count: s_blocked,
            coverage_percent: SyntaxCoverage::expected_percent(s_total, s_translated),
        },
        parity_claim_state: seed.parity_claim_state,
        parity_note: seed.parity_note.to_owned(),
        mutates_durable_state: seed.mutates_durable_state,
        rollback: RollbackRef {
            rollback_path_class: rollback_class,
            checkpoint_ref: checkpoint.map(str::to_owned),
            rollback_ref: rollback_ref.to_owned(),
            user_visible_action_id: action_id.to_owned(),
            note: rollback_note.map(str::to_owned),
        },
        compatibility_note: seed.compatibility_note.map(str::to_owned),
        unresolved_slots: seed
            .unresolved_slots
            .iter()
            .map(|slot| UnresolvedSlot {
                source_slot_ref: slot.source_slot_ref.to_owned(),
                summary: slot.summary.to_owned(),
                fallback_disclosed: slot.fallback_disclosed,
            })
            .collect(),
        known_deviations: seed
            .known_deviations
            .iter()
            .map(|(id, summary, recoverable)| KnownDeviation {
                deviation_id: (*id).to_owned(),
                summary: (*summary).to_owned(),
                recoverable: *recoverable,
            })
            .collect(),
        docs_help_refs: seed
            .docs_help_refs
            .iter()
            .map(|r| (*r).to_owned())
            .collect(),
        narrative: seed.narrative.to_owned(),
    }
}

fn seeded_rows() -> Vec<ThemeImportRow> {
    const ROW_SEEDS: &[RowSeed] = &[
        RowSeed {
            row_id: "m5-theme-import-row:vscode-github-dark",
            title: "VS Code dark theme import",
            source_ecosystem: SourceEcosystem::VsCode,
            source_tool_name: "Visual Studio Code",
            source_tool_version: "1.97.0",
            source_theme_identifier: "source:vscode:github_dark",
            source_artifact_ref: "source:vscode:theme_bundle:github_dark",
            target_theme_package_ref: Some("aureline:theme_package:imported_github_dark"),
            import_outcome: ImportOutcomeState::Applied,
            primary_mapping_state: ThemeMappingState::Translated,
            mapping_summary: (48, 48, 0, 0, 0, 0),
            syntax_coverage: (60, 60, 0, 0, 0),
            parity_claim_state: ParityClaimState::ClaimedWithReport,
            parity_note:
                "Every semantic, component, and syntax slot translated to a native token with no \
                 unresolved slots; full parity is claimed and backed by this report.",
            mutates_durable_state: true,
            rollback: (
                RollbackPathClass::RestoreAppearanceCheckpoint,
                Some("checkpoint:theme-import:github-dark"),
                "rollback:theme-import:github-dark",
                "appearance.restore_checkpoint",
                Some("Restore the pre-apply appearance checkpoint to revert the imported theme."),
            ),
            compatibility_note: None,
            unresolved_slots: &[],
            known_deviations: &[],
            docs_help_refs: &[
                "docs/m5/theme-import-and-rollback.md#vscode-clean-translate",
                "docs/m5/theme-package-and-appearance-objects.md#imported-theme-parity",
            ],
            narrative:
                "A widely used VS Code dark theme maps one-to-one onto Aureline's semantic and \
                 syntax tokens. The apply is checkpointed, so the user can restore the prior \
                 appearance if they change their mind.",
        },
        RowSeed {
            row_id: "m5-theme-import-row:jetbrains-darcula",
            title: "JetBrains Darcula scheme import",
            source_ecosystem: SourceEcosystem::JetBrains,
            source_tool_name: "IntelliJ IDEA",
            source_tool_version: "2024.3",
            source_theme_identifier: "source:jetbrains:darcula",
            source_artifact_ref: "source:jetbrains:scheme_bundle:darcula",
            target_theme_package_ref: Some("aureline:theme_package:imported_darcula"),
            import_outcome: ImportOutcomeState::AppliedWithWarnings,
            primary_mapping_state: ThemeMappingState::SubstitutedFallback,
            mapping_summary: (52, 40, 8, 2, 2, 0),
            syntax_coverage: (64, 50, 9, 5, 0),
            parity_claim_state: ParityClaimState::PartialClaimWithGaps,
            parity_note:
                "Most slots translated; eight fell back to disclosed neutral defaults and two \
                 remained unresolved. Parity is claimed only partially, with the gaps listed.",
            mutates_durable_state: true,
            rollback: (
                RollbackPathClass::RestoreAppearanceCheckpoint,
                Some("checkpoint:theme-import:darcula"),
                "rollback:theme-import:darcula",
                "appearance.restore_checkpoint",
                Some("Restore the pre-apply appearance checkpoint to revert the imported theme."),
            ),
            compatibility_note: Some(
                "Editor scheme colors translated, but two IDE-specific scopes and the gutter \
                 accent fell back to disclosed neutral defaults; review before relying on them.",
            ),
            unresolved_slots: &[
                UnresolvedSeed {
                    source_slot_ref: "source:jetbrains:darcula:slot:inline_hint",
                    summary: "Inline parameter-hint background has no semantic target and is left \
                              unresolved rather than guessed.",
                    fallback_disclosed: true,
                },
                UnresolvedSeed {
                    source_slot_ref: "source:jetbrains:darcula:slot:breadcrumb_bg",
                    summary:
                        "Breadcrumb background tint is unresolved; the shell keeps its native \
                              chrome rather than approximate it.",
                    fallback_disclosed: true,
                },
            ],
            known_deviations: &[(
                "deviation:jetbrains.gutter_accent",
                "The gutter change-accent is substituted with the native diff token, not the \
                 source accent.",
                true,
            )],
            docs_help_refs: &[
                "docs/m5/theme-import-and-rollback.md#jetbrains-partial",
                "docs/m5/theme-package-and-appearance-objects.md#imported-theme-parity",
            ],
            narrative:
                "A JetBrains Darcula scheme translates its editor colors cleanly, but a handful of \
                 IDE-specific slots have no native target. Those are substituted with disclosed \
                 fallbacks or left unresolved, and the warnings ride along with the apply.",
        },
        RowSeed {
            row_id: "m5-theme-import-row:zed-one-dark-remix",
            title: "Zed theme import (rolled back)",
            source_ecosystem: SourceEcosystem::Zed,
            source_tool_name: "Zed",
            source_tool_version: "0.160.0",
            source_theme_identifier: "source:zed:one_dark_remix",
            source_artifact_ref: "source:zed:theme_bundle:one_dark_remix",
            target_theme_package_ref: Some("aureline:theme_package:imported_one_dark_remix"),
            import_outcome: ImportOutcomeState::RolledBack,
            primary_mapping_state: ThemeMappingState::BlockedHonesty,
            mapping_summary: (44, 36, 4, 1, 1, 2),
            syntax_coverage: (58, 52, 4, 2, 0),
            parity_claim_state: ParityClaimState::DeniedUnresolvedOrBlocked,
            parity_note:
                "The applied import recolored a trust/severity cue using color alone; the honesty \
                 check blocked it and the import was rolled back. Parity is denied.",
            mutates_durable_state: false,
            rollback: (
                RollbackPathClass::RestoreAppearanceCheckpoint,
                Some("checkpoint:theme-import:one-dark-remix"),
                "rollback:theme-import:one-dark-remix",
                "appearance.restore_checkpoint",
                Some(
                    "Restored the pre-apply appearance checkpoint after the honesty check failed.",
                ),
            ),
            compatibility_note: Some(
                "This theme used color alone to signal a trust state, which would have hidden a \
                 protected cue; Aureline rolled the import back rather than ship a misleading \
                 appearance.",
            ),
            unresolved_slots: &[UnresolvedSeed {
                source_slot_ref: "source:zed:one_dark_remix:slot:status_accent",
                summary: "The status accent slot could not be resolved without overriding a \
                          protected trust cue, so it is left unresolved.",
                fallback_disclosed: false,
            }],
            known_deviations: &[(
                "deviation:zed.trust_cue_color_only",
                "The source theme expressed a trust state with color only; Aureline keeps the \
                 non-color cue and refuses the override.",
                false,
            )],
            docs_help_refs: &[
                "docs/m5/theme-import-and-rollback.md#zed-rolled-back",
                "docs/m5/theme-package-and-appearance-objects.md#imported-theme-parity",
            ],
            narrative:
                "An applied Zed theme proved semantically misleading: it recolored a protected \
                 trust cue with color alone. The honesty check blocked the mapping and the import \
                 was rolled back to the checkpoint, demonstrating that imported visual \
                 customizations stay reversible.",
        },
        RowSeed {
            row_id: "m5-theme-import-row:vim-gruvbox",
            title: "Vim colorscheme import (review required)",
            source_ecosystem: SourceEcosystem::Vim,
            source_tool_name: "Neovim",
            source_tool_version: "0.10.2",
            source_theme_identifier: "source:vim:gruvbox",
            source_artifact_ref: "source:vim:colorscheme_bundle:gruvbox",
            target_theme_package_ref: None,
            import_outcome: ImportOutcomeState::ReviewRequired,
            primary_mapping_state: ThemeMappingState::Unresolved,
            mapping_summary: (40, 22, 6, 4, 8, 0),
            syntax_coverage: (50, 28, 6, 16, 0),
            parity_claim_state: ParityClaimState::PartialClaimWithGaps,
            parity_note:
                "The terminal-oriented colorscheme maps cleanly for syntax but leaves eight UI \
                 slots unresolved; review is required before applying.",
            mutates_durable_state: false,
            rollback: (
                RollbackPathClass::ReopenImportReview,
                None,
                "rollback:theme-import:gruvbox:reopen",
                "appearance.reopen_import_review",
                Some("Reopen the import review to re-decide the unresolved chrome slots."),
            ),
            compatibility_note: Some(
                "A Vim colorscheme covers syntax scopes but not the full IDE chrome; the \
                 unresolved chrome slots are listed so they are not silently defaulted.",
            ),
            unresolved_slots: &[
                UnresolvedSeed {
                    source_slot_ref: "source:vim:gruvbox:slot:statusline",
                    summary: "The statusline palette has no direct chrome target and is left for \
                              review.",
                    fallback_disclosed: true,
                },
                UnresolvedSeed {
                    source_slot_ref: "source:vim:gruvbox:slot:tabline",
                    summary: "The tabline palette is unresolved; the shell keeps its native tab \
                              chrome pending review.",
                    fallback_disclosed: true,
                },
            ],
            known_deviations: &[(
                "deviation:vim.chrome_scope",
                "Vim colorschemes target terminal syntax, not IDE chrome, so chrome slots need \
                 explicit review.",
                true,
            )],
            docs_help_refs: &[
                "docs/m5/theme-import-and-rollback.md#vim-review-required",
                "docs/m5/theme-package-and-appearance-objects.md#imported-theme-parity",
            ],
            narrative:
                "A Vim colorscheme maps its syntax scopes but leaves the IDE chrome unresolved. \
                 The import is held for review with the unresolved slots listed, so the user \
                 decides them before anything applies.",
        },
        RowSeed {
            row_id: "m5-theme-import-row:textmate-monokai-classic",
            title: "TextMate tmTheme import (blocked)",
            source_ecosystem: SourceEcosystem::TextMate,
            source_tool_name: "TextMate",
            source_tool_version: "2.0",
            source_theme_identifier: "source:textmate:monokai_classic",
            source_artifact_ref: "source:textmate:tmtheme_bundle:monokai_classic",
            target_theme_package_ref: None,
            import_outcome: ImportOutcomeState::Blocked,
            primary_mapping_state: ThemeMappingState::Unsupported,
            mapping_summary: (30, 6, 2, 18, 4, 0),
            syntax_coverage: (36, 10, 2, 24, 0),
            parity_claim_state: ParityClaimState::DeniedUnresolvedOrBlocked,
            parity_note:
                "The legacy tmTheme format exposes almost no semantic slots; the import is blocked \
                 rather than shipped as a plausible-looking but unmapped theme.",
            mutates_durable_state: false,
            rollback: (
                RollbackPathClass::DiscardPreview,
                None,
                "rollback:theme-import:monokai_classic:discard",
                "appearance.discard_preview",
                Some("Discard the preview; nothing was applied."),
            ),
            compatibility_note: Some(
                "Only raw editor foreground and background could be read; the rest of the design \
                 system has no source slots, so a parity claim would be misleading.",
            ),
            unresolved_slots: &[
                UnresolvedSeed {
                    source_slot_ref: "source:textmate:monokai_classic:slot:semantic_tokens",
                    summary: "The format carries no semantic token slots, so they cannot be \
                              resolved.",
                    fallback_disclosed: false,
                },
                UnresolvedSeed {
                    source_slot_ref: "source:textmate:monokai_classic:slot:diff_tokens",
                    summary: "Diff tokens are absent from the source and are left unresolved \
                              rather than defaulted silently.",
                    fallback_disclosed: false,
                },
            ],
            known_deviations: &[(
                "deviation:textmate.format_coverage",
                "The tmTheme format predates semantic theming; most of the design system has no \
                 source to map from.",
                false,
            )],
            docs_help_refs: &[
                "docs/m5/theme-import-and-rollback.md#textmate-blocked",
                "docs/m5/theme-package-and-appearance-objects.md#imported-theme-parity",
            ],
            narrative:
                "A legacy TextMate tmTheme exposes only editor foreground and background. Rather \
                 than render a plausible-looking but mostly unmapped theme and imply parity, the \
                 migration center blocks the import and the preview is discarded.",
        },
    ];

    ROW_SEEDS.iter().map(build_row).collect()
}
