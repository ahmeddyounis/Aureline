//! Top-incumbent migration corpus and flow scoreboard.
//!
//! The migration corpus is the page-level projection that names every
//! top-incumbent flow Aureline claims a switching promise for during
//! beta and records its current
//! `Exact` / `Translated` / `Partial` / `Shimmed` / `Unsupported`
//! mapping result. The projection is consumed by the live migration
//! center, the in-product wizard summary, the cohort/support export
//! wrapper, and the markdown scoreboard checked in under
//! `artifacts/migration/m3/migration_scoreboard.md`.
//!
//! Four named incumbent ecosystems must be present and each must
//! carry a non-empty set of flow rows so the scoreboard can downgrade
//! an overclaimed path automatically:
//!
//! - VS Code / Code-OSS
//! - JetBrains family (IntelliJ and siblings)
//! - Vim / Neovim
//! - Emacs
//!
//! The corpus quotes the wizard projection in
//! [`crate::migration_wizard`] so the wizard page, the corpus
//! scoreboard, and the docs scoreboard never disagree on shared ids.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/migration/m3/incumbent_flows/`
//! are bit-for-bit equal to the seeded scoreboard produced by
//! [`seeded_migration_scoreboard`].

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::import::diff_review::{ImportMappingClassification, ImportReviewDomain};
use crate::migration_wizard::seeded_migration_wizard_page;

/// Schema version exported with every corpus record.
pub const MIGRATION_CORPUS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every corpus row.
pub const MIGRATION_CORPUS_SHARED_CONTRACT_REF: &str = "shell:migration_corpus_beta:v1";

/// Stable record kind for [`MigrationScoreboard`] payloads.
pub const MIGRATION_SCOREBOARD_RECORD_KIND: &str = "shell_migration_corpus_beta_scoreboard_record";

/// Stable record kind for [`EcosystemScoreboardSection`] payloads.
pub const ECOSYSTEM_SECTION_RECORD_KIND: &str =
    "shell_migration_corpus_beta_ecosystem_section_record";

/// Stable record kind for [`IncumbentFlowRow`] payloads.
pub const INCUMBENT_FLOW_ROW_RECORD_KIND: &str =
    "shell_migration_corpus_beta_incumbent_flow_row_record";

/// Stable record kind for [`MigrationCorpusSupportExport`] payloads.
pub const MIGRATION_CORPUS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_migration_corpus_beta_support_export_record";

/// Stable scoreboard id used by every consumer.
pub const MIGRATION_SCOREBOARD_ID: &str = "shell:migration_corpus_beta:scoreboard:v1";

const GENERATED_AT: &str = "2026-05-15T00:00:00Z";

/// One of the four named beta incumbent ecosystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncumbentEcosystem {
    /// VS Code and Code-OSS profiles.
    VsCodeCodeOss,
    /// IntelliJ and JetBrains family IDEs.
    JetBrainsFamily,
    /// Vim and Neovim profiles.
    VimNeovim,
    /// Emacs profiles.
    Emacs,
}

impl IncumbentEcosystem {
    /// Returns the stable schema token for this ecosystem.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VsCodeCodeOss => "vs_code_code_oss",
            Self::JetBrainsFamily => "jetbrains_family",
            Self::VimNeovim => "vim_neovim",
            Self::Emacs => "emacs",
        }
    }

    /// Returns the reviewer-facing ecosystem label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::VsCodeCodeOss => "VS Code / Code-OSS",
            Self::JetBrainsFamily => "JetBrains IDEs",
            Self::VimNeovim => "Vim / Neovim",
            Self::Emacs => "Emacs",
        }
    }

    /// Returns the source-ecosystem row id consumed by docs and support.
    pub fn source_ecosystem_row_ref(self) -> &'static str {
        match self {
            Self::VsCodeCodeOss => "migration_source:vs_code_code_oss",
            Self::JetBrainsFamily => "migration_source:jetbrains_family",
            Self::VimNeovim => "migration_source:vim_neovim",
            Self::Emacs => "migration_source:emacs",
        }
    }

    /// Returns the four required ecosystems in canonical order.
    pub fn required_ecosystems() -> [Self; 4] {
        [
            Self::VsCodeCodeOss,
            Self::JetBrainsFamily,
            Self::VimNeovim,
            Self::Emacs,
        ]
    }
}

/// One classified flow row inside the corpus scoreboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncumbentFlowRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Stable flow id quoted across surfaces.
    pub flow_id: String,
    /// Ecosystem this flow belongs to.
    pub ecosystem: IncumbentEcosystem,
    /// Reviewer-facing flow label (e.g. "Command palette shortcut").
    pub flow_label: String,
    /// Domain the flow lives in.
    pub domain: ImportReviewDomain,
    /// Required Exact / Translated / Partial / Shimmed / Unsupported class.
    pub classification: ImportMappingClassification,
    /// Source object label retained for support evidence.
    pub source_object_label: String,
    /// Aureline-side target label.
    pub aureline_target_label: String,
    /// Reviewer-facing before/after summary.
    pub before_after_summary: String,
    /// Caveat retained for `Partial`, `Shimmed`, and `Unsupported` rows.
    pub caveat: Option<String>,
    /// Downgrade triggers that automatically narrow the row when the
    /// claimed evidence rots. Required for every non-`Exact` row.
    pub downgrade_triggers: Vec<String>,
    /// Evidence refs that prove the row's current classification.
    pub evidence_refs: Vec<String>,
    /// Docs/help refs that publish the row.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs that retain the row in support evidence.
    pub support_export_refs: Vec<String>,
    /// Wizard mapping report ref the row composes with.
    pub wizard_mapping_report_ref: String,
    /// Rollback checkpoint ref retained with the row.
    pub rollback_checkpoint_ref: String,
}

impl IncumbentFlowRow {
    fn requires_downgrade_trigger(&self) -> bool {
        !matches!(self.classification, ImportMappingClassification::Exact)
    }

    fn requires_caveat(&self) -> bool {
        matches!(
            self.classification,
            ImportMappingClassification::Partial
                | ImportMappingClassification::Shimmed
                | ImportMappingClassification::Unsupported
        )
    }
}

/// Per-classification count summary for the scoreboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreboardClassificationSummary {
    /// Number of `Exact` rows.
    pub exact: usize,
    /// Number of `Translated` rows.
    pub translated: usize,
    /// Number of `Partial` rows.
    pub partial: usize,
    /// Number of `Shimmed` rows.
    pub shimmed: usize,
    /// Number of `Unsupported` rows.
    pub unsupported: usize,
}

impl ScoreboardClassificationSummary {
    fn empty() -> Self {
        Self {
            exact: 0,
            translated: 0,
            partial: 0,
            shimmed: 0,
            unsupported: 0,
        }
    }

    fn record(&mut self, classification: ImportMappingClassification) {
        match classification {
            ImportMappingClassification::Exact => self.exact += 1,
            ImportMappingClassification::Translated => self.translated += 1,
            ImportMappingClassification::Partial => self.partial += 1,
            ImportMappingClassification::Shimmed => self.shimmed += 1,
            ImportMappingClassification::Unsupported => self.unsupported += 1,
        }
    }

    /// Returns the total number of classified rows.
    pub const fn total(&self) -> usize {
        self.exact + self.translated + self.partial + self.shimmed + self.unsupported
    }
}

/// One per-ecosystem scoreboard section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EcosystemScoreboardSection {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the section.
    pub schema_version: u32,
    /// Ecosystem this section covers.
    pub ecosystem: IncumbentEcosystem,
    /// Source-ecosystem row ref consumed by docs and support.
    pub source_ecosystem_row_ref: String,
    /// Classified flow rows for this ecosystem, sorted by `flow_id`.
    pub rows: Vec<IncumbentFlowRow>,
    /// Distinct classifications present in this section.
    pub classifications_present: Vec<ImportMappingClassification>,
    /// Per-classification count for this section.
    pub classification_summary: ScoreboardClassificationSummary,
}

/// Beta migration corpus scoreboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationScoreboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable scoreboard id used to pivot across surfaces.
    pub scoreboard_id: String,
    /// Wizard session id this scoreboard composes with.
    pub wizard_session_ref: String,
    /// Wizard mapping report id this scoreboard composes with.
    pub wizard_mapping_report_ref: String,
    /// Rollback checkpoint ref the wizard minted before apply.
    pub rollback_checkpoint_ref: String,
    /// Per-ecosystem scoreboard sections in canonical order.
    pub sections: Vec<EcosystemScoreboardSection>,
    /// Overall classification summary across every section.
    pub overall_summary: ScoreboardClassificationSummary,
    /// Distinct classifications present in the overall scoreboard.
    pub classifications_present: Vec<ImportMappingClassification>,
    /// Markdown scoreboard artifact that publishes the rows.
    pub published_scoreboard_ref: String,
    /// Markdown incumbent-flow matrix that narrates the rows.
    pub published_flow_matrix_ref: String,
    /// Docs/help refs the scoreboard reopens from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the scoreboard reopens from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the scoreboard was generated.
    pub generated_at: String,
}

impl MigrationScoreboard {
    /// Returns `true` when every required ecosystem is present.
    pub fn covers_every_required_ecosystem(&self) -> bool {
        IncumbentEcosystem::required_ecosystems()
            .iter()
            .all(|required| {
                self.sections
                    .iter()
                    .any(|section| section.ecosystem == *required)
            })
    }

    /// Returns `true` when every required classification is present at
    /// the scoreboard level.
    pub fn covers_every_required_classification(&self) -> bool {
        [
            ImportMappingClassification::Exact,
            ImportMappingClassification::Translated,
            ImportMappingClassification::Partial,
            ImportMappingClassification::Shimmed,
            ImportMappingClassification::Unsupported,
        ]
        .iter()
        .all(|required| self.classifications_present.contains(required))
    }

    /// Builds the markdown scoreboard artifact text.
    ///
    /// The rendering is deterministic so the checked-in
    /// `artifacts/migration/m3/migration_scoreboard.md` artifact stays
    /// bit-for-bit equal to what the headless inspector emits.
    pub fn render_scoreboard_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Top-incumbent migration scoreboard (beta)\n");
        out.push('\n');
        out.push_str(
            "Generated from the seeded migration corpus in\n\
             [`crate::migration_corpus`](../../../crates/aureline-shell/src/migration_corpus/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- scoreboard-md > \\\n  artifacts/migration/m3/migration_scoreboard.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Scoreboard id: `{}`\n", self.scoreboard_id));
        out.push_str(&format!(
            "- Wizard session: `{}`\n",
            self.wizard_session_ref
        ));
        out.push_str(&format!(
            "- Wizard mapping report: `{}`\n",
            self.wizard_mapping_report_ref
        ));
        out.push_str(&format!(
            "- Rollback checkpoint: `{}`\n",
            self.rollback_checkpoint_ref
        ));
        out.push_str(&format!("- Generated at: `{}`\n", self.generated_at));
        out.push('\n');

        out.push_str("## Overall classification summary\n\n");
        out.push_str("| Exact | Translated | Partial | Shimmed | Unsupported | Total |\n");
        out.push_str("|------:|-----------:|--------:|--------:|------------:|------:|\n");
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n\n",
            self.overall_summary.exact,
            self.overall_summary.translated,
            self.overall_summary.partial,
            self.overall_summary.shimmed,
            self.overall_summary.unsupported,
            self.overall_summary.total(),
        ));

        for section in &self.sections {
            out.push_str(&format!(
                "## {} (`{}`)\n\n",
                section.ecosystem.display_label(),
                section.ecosystem.as_str()
            ));
            out.push_str(&format!(
                "Source ecosystem row: `{}`\n\n",
                section.source_ecosystem_row_ref
            ));
            out.push_str("| Exact | Translated | Partial | Shimmed | Unsupported |\n");
            out.push_str("|------:|-----------:|--------:|--------:|------------:|\n");
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n\n",
                section.classification_summary.exact,
                section.classification_summary.translated,
                section.classification_summary.partial,
                section.classification_summary.shimmed,
                section.classification_summary.unsupported,
            ));

            out.push_str("| Flow | Domain | Classification | Source | Aureline target |\n");
            out.push_str("| ---- | ------ | -------------- | ------ | --------------- |\n");
            for row in &section.rows {
                out.push_str(&format!(
                    "| `{flow}` -- {label} | {domain} | **{class}** | {source} | {target} |\n",
                    flow = row.flow_id,
                    label = row.flow_label,
                    domain = row.domain.display_label(),
                    class = row.classification.display_label(),
                    source = row.source_object_label,
                    target = row.aureline_target_label,
                ));
            }
            out.push('\n');

            for row in &section.rows {
                out.push_str(&format!(
                    "### `{}` -- {} ({})\n\n",
                    row.flow_id,
                    row.flow_label,
                    row.classification.display_label()
                ));
                out.push_str(&format!("- Domain: {}\n", row.domain.display_label()));
                out.push_str(&format!("- Source: {}\n", row.source_object_label));
                out.push_str(&format!(
                    "- Aureline target: {}\n",
                    row.aureline_target_label
                ));
                out.push_str(&format!(
                    "- Before/after: {}\n",
                    row.before_after_summary
                ));
                if let Some(caveat) = &row.caveat {
                    out.push_str(&format!("- Caveat: {caveat}\n"));
                }
                if !row.downgrade_triggers.is_empty() {
                    out.push_str("- Downgrade triggers:\n");
                    for trigger in &row.downgrade_triggers {
                        out.push_str(&format!("  - `{trigger}`\n"));
                    }
                }
                if !row.evidence_refs.is_empty() {
                    out.push_str("- Evidence:\n");
                    for ref_path in &row.evidence_refs {
                        out.push_str(&format!("  - `{ref_path}`\n"));
                    }
                }
                if !row.docs_help_refs.is_empty() {
                    out.push_str("- Docs/help:\n");
                    for ref_path in &row.docs_help_refs {
                        out.push_str(&format!("  - `{ref_path}`\n"));
                    }
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str("cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- validate\n");
        out.push_str("cargo test -p aureline-shell --test migration_corpus_fixtures\n");
        out.push_str("```\n");
        out
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "scoreboard: ecosystems={}, rows={}, exact={}, translated={}, partial={}, shimmed={}, unsupported={}",
            self.sections.len(),
            self.overall_summary.total(),
            self.overall_summary.exact,
            self.overall_summary.translated,
            self.overall_summary.partial,
            self.overall_summary.shimmed,
            self.overall_summary.unsupported,
        ));
        for section in &self.sections {
            lines.push(format!(
                "{}: rows={}, exact={}, translated={}, partial={}, shimmed={}, unsupported={}",
                section.ecosystem.display_label(),
                section.rows.len(),
                section.classification_summary.exact,
                section.classification_summary.translated,
                section.classification_summary.partial,
                section.classification_summary.shimmed,
                section.classification_summary.unsupported,
            ));
        }
        lines
    }
}

/// Support-export wrapper for the corpus scoreboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationCorpusSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Scoreboard quoted in full.
    pub scoreboard: MigrationScoreboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl MigrationCorpusSupportExport {
    /// Builds the support-export wrapper for a scoreboard.
    pub fn from_scoreboard(
        support_export_id: impl Into<String>,
        scoreboard: MigrationScoreboard,
    ) -> Self {
        let mut case_ids = vec![
            scoreboard.scoreboard_id.clone(),
            scoreboard.wizard_session_ref.clone(),
            scoreboard.wizard_mapping_report_ref.clone(),
            scoreboard.rollback_checkpoint_ref.clone(),
        ];
        for section in &scoreboard.sections {
            for row in &section.rows {
                case_ids.push(row.flow_id.clone());
            }
        }
        Self {
            record_kind: MIGRATION_CORPUS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MIGRATION_CORPUS_SCHEMA_VERSION,
            shared_contract_ref: MIGRATION_CORPUS_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            scoreboard,
            case_ids,
        }
    }
}

/// Validation error produced by [`validate_migration_scoreboard`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum MigrationScoreboardValidationError {
    /// A required ecosystem section is missing.
    MissingRequiredEcosystem { ecosystem: String },
    /// An ecosystem section is empty.
    EmptyEcosystemSection { ecosystem: String },
    /// The scoreboard does not cover every required classification.
    MissingRequiredClassification { classification: String },
    /// A non-`Exact` row is missing at least one downgrade trigger.
    MissingDowngradeTrigger { flow_id: String },
    /// A `Partial` / `Shimmed` / `Unsupported` row is missing the caveat.
    MissingCaveat { flow_id: String },
    /// A row is missing evidence refs.
    MissingEvidenceRefs { flow_id: String },
    /// A row is missing docs/help refs.
    MissingDocsHelpRefs { flow_id: String },
    /// A row's wizard mapping report ref is empty.
    MissingWizardMappingReportRef { flow_id: String },
    /// The published markdown scoreboard ref is empty.
    PublishedScoreboardRefMissing,
    /// The published markdown flow matrix ref is empty.
    PublishedFlowMatrixRefMissing,
}

/// Validates a scoreboard against the M3 acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_migration_scoreboard(
    scoreboard: &MigrationScoreboard,
) -> Result<(), Vec<MigrationScoreboardValidationError>> {
    let mut errors = Vec::new();

    for required in IncumbentEcosystem::required_ecosystems() {
        if !scoreboard
            .sections
            .iter()
            .any(|section| section.ecosystem == required)
        {
            errors.push(MigrationScoreboardValidationError::MissingRequiredEcosystem {
                ecosystem: required.as_str().to_owned(),
            });
        }
    }

    for section in &scoreboard.sections {
        if section.rows.is_empty() {
            errors.push(MigrationScoreboardValidationError::EmptyEcosystemSection {
                ecosystem: section.ecosystem.as_str().to_owned(),
            });
        }
        for row in &section.rows {
            if row.requires_downgrade_trigger() && row.downgrade_triggers.is_empty() {
                errors.push(MigrationScoreboardValidationError::MissingDowngradeTrigger {
                    flow_id: row.flow_id.clone(),
                });
            }
            if row.requires_caveat() && row.caveat.as_deref().map(str::trim).unwrap_or("").is_empty()
            {
                errors.push(MigrationScoreboardValidationError::MissingCaveat {
                    flow_id: row.flow_id.clone(),
                });
            }
            if row.evidence_refs.is_empty() {
                errors.push(MigrationScoreboardValidationError::MissingEvidenceRefs {
                    flow_id: row.flow_id.clone(),
                });
            }
            if row.docs_help_refs.is_empty() {
                errors.push(MigrationScoreboardValidationError::MissingDocsHelpRefs {
                    flow_id: row.flow_id.clone(),
                });
            }
            if row.wizard_mapping_report_ref.trim().is_empty() {
                errors.push(
                    MigrationScoreboardValidationError::MissingWizardMappingReportRef {
                        flow_id: row.flow_id.clone(),
                    },
                );
            }
        }
    }

    for required in [
        ImportMappingClassification::Exact,
        ImportMappingClassification::Translated,
        ImportMappingClassification::Partial,
        ImportMappingClassification::Shimmed,
        ImportMappingClassification::Unsupported,
    ] {
        if !scoreboard.classifications_present.contains(&required) {
            errors.push(
                MigrationScoreboardValidationError::MissingRequiredClassification {
                    classification: required.as_str().to_owned(),
                },
            );
        }
    }

    if scoreboard.published_scoreboard_ref.trim().is_empty() {
        errors.push(MigrationScoreboardValidationError::PublishedScoreboardRefMissing);
    }
    if scoreboard.published_flow_matrix_ref.trim().is_empty() {
        errors.push(MigrationScoreboardValidationError::PublishedFlowMatrixRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed entry consumed by [`seeded_migration_scoreboard`].
struct FlowSeed {
    ecosystem: IncumbentEcosystem,
    flow_slug: &'static str,
    flow_label: &'static str,
    domain: ImportReviewDomain,
    classification: ImportMappingClassification,
    source_object_label: &'static str,
    aureline_target_label: &'static str,
    before_after_summary: &'static str,
    caveat: Option<&'static str>,
    downgrade_triggers: &'static [&'static str],
    evidence_refs: &'static [&'static str],
    docs_help_refs: &'static [&'static str],
    support_export_refs: &'static [&'static str],
}

/// Seed table for the four named incumbent ecosystems.
///
/// The order of rows within each ecosystem is preserved verbatim into
/// the fixture so the docs scoreboard can quote the seed without
/// reordering.
const FLOW_SEEDS: &[FlowSeed] = &[
    // VS Code / Code-OSS
    FlowSeed {
        ecosystem: IncumbentEcosystem::VsCodeCodeOss,
        flow_slug: "settings",
        flow_label: "User and workspace settings",
        domain: ImportReviewDomain::Settings,
        classification: ImportMappingClassification::Exact,
        source_object_label: ".vscode/settings.json",
        aureline_target_label: "Aureline user and workspace setting records",
        before_after_summary: "settings.json keys map to stable Aureline setting ids without semantic loss.",
        caveat: None,
        downgrade_triggers: &[],
        evidence_refs: &[
            "fixtures/migration/equivalence_cases/vscode_setting_exact.yaml",
            "fixtures/migration/m3/migration_wizard/mapping_report.json",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#vscode",
            "docs/migration/source_ecosystem_coverage_matrix.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::VsCodeCodeOss,
        flow_slug: "command-palette-shortcut",
        flow_label: "Command palette shortcut",
        domain: ImportReviewDomain::Shortcuts,
        classification: ImportMappingClassification::Translated,
        source_object_label: ".vscode/keybindings.json -- workbench.action.showCommands",
        aureline_target_label: "aureline:command.palette.open",
        before_after_summary: "VS Code command palette gesture maps to the Aureline palette command id.",
        caveat: Some(
            "Translation depends on the Aureline command id remaining stable across the keybinding resolver.",
        ),
        downgrade_triggers: &[
            "aureline_command_id_renamed",
            "vscode_command_id_renamed",
            "keybinding_resolver_layer_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/equivalence_cases/vscode_shortcut_translated.yaml",
            "fixtures/migration/m3/migration_wizard/mapping_report.json",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#vscode",
            "docs/migration/keymap_presets.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::VsCodeCodeOss,
        flow_slug: "high-frequency-keymap-chord",
        flow_label: "High-frequency keymap chord remap",
        domain: ImportReviewDomain::Keymaps,
        classification: ImportMappingClassification::Partial,
        source_object_label: ".vscode/keybindings.json -- multi-key chord",
        aureline_target_label: "aureline:keymaps.shortcut_delta_digest",
        before_after_summary: "Most chord targets map, but the destination chord capacity remaps a small set of high-frequency shortcuts.",
        caveat: Some(
            "Muscle-memory risk for the remapped chord stays visible in the shortcut delta digest until the user accepts the change.",
        ),
        downgrade_triggers: &[
            "shortcut_delta_digest_changed",
            "platform_reserved_chord_changed",
            "keybinding_resolver_layer_changed",
        ],
        evidence_refs: &[
            "fixtures/commands/keybinding_conflict_examples/high_frequency_shortcut_diff_after_import.json",
            "fixtures/migration/m3/migration_wizard/mapping_report.json",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#vscode",
            "docs/migration/migration_restore_and_shortcut_delta_packet.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::VsCodeCodeOss,
        flow_slug: "eslint-native-replacement",
        flow_label: "ESLint extension as native replacement",
        domain: ImportReviewDomain::ExtensionsAndProviders,
        classification: ImportMappingClassification::Shimmed,
        source_object_label: "vscode:extension:dbaeumer.vscode-eslint",
        aureline_target_label: "aureline:package:eslint-native-lint",
        before_after_summary: "Source extension is recorded and the native lint package is recommended; runtime is not bridged silently.",
        caveat: Some(
            "Native replacement does not import source extension authority, storage, or webview state.",
        ),
        downgrade_triggers: &[
            "native_package_or_command_changed",
            "permission_or_policy_vocab_changed",
            "extension_recommendation_evidence_expired",
        ],
        evidence_refs: &[
            "fixtures/migration/compatibility_scorecards/native_alternative_recommendation.json",
            "artifacts/migration/top_imported_workflow_rows.yaml",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#vscode",
            "docs/migration/compatibility_scorecard_contract.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::VsCodeCodeOss,
        flow_slug: "webview-extension-runtime",
        flow_label: "Webview-heavy extension runtime",
        domain: ImportReviewDomain::ExtensionsAndProviders,
        classification: ImportMappingClassification::Unsupported,
        source_object_label: "vscode:extension:sample.webview-tool",
        aureline_target_label: "(no safe target)",
        before_after_summary: "Source webview runtime state has no governed Aureline target and apply is denied.",
        caveat: Some(
            "Extension runtime parity, arbitrary webview state, and source extension storage are not imported.",
        ),
        downgrade_triggers: &[
            "webview_governance_contract_changed",
            "extension_runtime_policy_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/compatibility_scorecards/unsupported_webview_extension.json",
            "fixtures/migration/m3/migration_wizard/unsupported_gaps.json",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#vscode",
            "docs/migration/source_ecosystem_coverage_matrix.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    // JetBrains family
    FlowSeed {
        ecosystem: IncumbentEcosystem::JetBrainsFamily,
        flow_slug: "common-keymap-preset",
        flow_label: "Common IDE keymap preset",
        domain: ImportReviewDomain::Keymaps,
        classification: ImportMappingClassification::Translated,
        source_object_label: "JetBrains keymap export (common preset)",
        aureline_target_label: "aureline:keymaps.jetbrains_preset",
        before_after_summary: "Common navigation and editing chords translate to Aureline command ids through the keymap preset.",
        caveat: Some(
            "Preset translation excludes plugin-specific actions and source-only IDE concepts.",
        ),
        downgrade_triggers: &[
            "aureline_command_id_renamed",
            "jetbrains_action_id_renamed",
            "keybinding_resolver_layer_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/jetbrains_family_profile.yaml",
            "fixtures/migration/equivalence_cases/jetbrains_run_debug_needs_manual_review.yaml",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#jetbrains",
            "docs/migration/keymap_presets.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::JetBrainsFamily,
        flow_slug: "run-debug-config",
        flow_label: "Application run/debug configuration",
        domain: ImportReviewDomain::LaunchDebug,
        classification: ImportMappingClassification::Partial,
        source_object_label: "JetBrains run configuration: app-server",
        aureline_target_label: "aureline:task-candidate:app-server",
        before_after_summary: "Common fields map to a candidate execution-context record; full runnable parity needs review.",
        caveat: Some(
            "Run/debug import is limited to configurations with a clear Aureline execution-context equivalent.",
        ),
        downgrade_triggers: &[
            "source_profile_fixture_changed",
            "native_package_or_command_changed",
            "post_import_validation_state_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/equivalence_cases/jetbrains_run_debug_needs_manual_review.yaml",
            "fixtures/migration/compatibility_scorecards/partial_run_debug_translation.json",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#jetbrains",
            "docs/migration/post_import_validation_contract.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::JetBrainsFamily,
        flow_slug: "code-style-hints",
        flow_label: "Code style and formatter hints",
        domain: ImportReviewDomain::Settings,
        classification: ImportMappingClassification::Exact,
        source_object_label: "JetBrains formatter profile (common keys)",
        aureline_target_label: "Aureline settings and formatter hint records",
        before_after_summary: "Common formatter and indentation keys map directly to Aureline settings.",
        caveat: None,
        downgrade_triggers: &[],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/jetbrains_family_profile.yaml",
        ],
        docs_help_refs: &["docs/migration/m3/incumbent_flow_matrix.md#jetbrains"],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::JetBrainsFamily,
        flow_slug: "plugin-runtime",
        flow_label: "Source IDE plugin runtime",
        domain: ImportReviewDomain::ExtensionsAndProviders,
        classification: ImportMappingClassification::Unsupported,
        source_object_label: "JetBrains plugin runtime (arbitrary plugin)",
        aureline_target_label: "(no safe target)",
        before_after_summary: "Source IDE plugin runtime has no Aureline native or bridge path; apply is denied for the runtime.",
        caveat: Some(
            "Source IDE indexes, generated project models, plugin runtime state, and run history are not imported as native truth.",
        ),
        downgrade_triggers: &[
            "extension_runtime_policy_changed",
            "compat_row_extension_host_changed",
        ],
        evidence_refs: &[
            "artifacts/migration/source_ecosystem_rows.yaml",
            "fixtures/migration/source_profile_examples/jetbrains_family_profile.yaml",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#jetbrains",
            "docs/migration/source_ecosystem_coverage_matrix.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::JetBrainsFamily,
        flow_slug: "project-root-handoff",
        flow_label: "Project root and module content roots",
        domain: ImportReviewDomain::WorkspaceProfile,
        classification: ImportMappingClassification::Shimmed,
        source_object_label: "JetBrains project root and module content roots",
        aureline_target_label: "aureline:workspace.manifest.roots",
        before_after_summary: "Module content roots map through a workspace-manifest shim that preserves provenance without claiming index parity.",
        caveat: Some(
            "Shimmed continuity does not preserve source IDE indexing semantics or generated project models.",
        ),
        downgrade_triggers: &[
            "workspace_manifest_schema_changed",
            "compat_row_workspace_profile_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/jetbrains_family_profile.yaml",
        ],
        docs_help_refs: &["docs/migration/m3/incumbent_flow_matrix.md#jetbrains"],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    // Vim / Neovim
    FlowSeed {
        ecosystem: IncumbentEcosystem::VimNeovim,
        flow_slug: "modal-editing-profile",
        flow_label: "Modal editing profile (normal, visual, operator)",
        domain: ImportReviewDomain::Keymaps,
        classification: ImportMappingClassification::Exact,
        source_object_label: "Curated vim/neovim modal profile",
        aureline_target_label: "aureline:modal_editing.profile",
        before_after_summary: "Normal, visual, and operator mappings map directly to the Aureline modal-editing profile record.",
        caveat: None,
        downgrade_triggers: &[],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/vim_neovim_profile.yaml",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#vim",
            "docs/migration/keymap_presets.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::VimNeovim,
        flow_slug: "leader-key-mappings",
        flow_label: "Leader-key mappings",
        domain: ImportReviewDomain::Shortcuts,
        classification: ImportMappingClassification::Translated,
        source_object_label: "Vim leader-key mappings",
        aureline_target_label: "aureline:keymaps.leader_overlay",
        before_after_summary: "Leader-key chains map to a leader overlay that names every translated Aureline command id.",
        caveat: Some(
            "Translation excludes mappings that call into Vimscript or Lua plugin runtimes.",
        ),
        downgrade_triggers: &[
            "leader_overlay_schema_changed",
            "aureline_command_id_renamed",
        ],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/vim_neovim_profile.yaml",
        ],
        docs_help_refs: &["docs/migration/m3/incumbent_flow_matrix.md#vim"],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::VimNeovim,
        flow_slug: "snippet-directories",
        flow_label: "Selected snippet directories",
        domain: ImportReviewDomain::SnippetsAndTemplates,
        classification: ImportMappingClassification::Partial,
        source_object_label: "Selected vim/neovim snippet directories",
        aureline_target_label: "Aureline snippet/template records",
        before_after_summary: "Standard snippet bodies import; trigger metadata that depends on plugin runtime is held for manual review.",
        caveat: Some(
            "Snippets that depend on a runtime plugin engine require manual review before they are recommended.",
        ),
        downgrade_triggers: &[
            "snippet_engine_compat_changed",
            "source_profile_fixture_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/vim_neovim_profile.yaml",
        ],
        docs_help_refs: &["docs/migration/m3/incumbent_flow_matrix.md#vim"],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::VimNeovim,
        flow_slug: "clipboard-search-defaults-shim",
        flow_label: "Clipboard and search defaults",
        domain: ImportReviewDomain::Settings,
        classification: ImportMappingClassification::Shimmed,
        source_object_label: "Vim/neovim clipboard and search option defaults",
        aureline_target_label: "aureline:settings.modal_profile_shim",
        before_after_summary: "Clipboard and search options are preserved through a modal-profile shim that names the source semantics explicitly.",
        caveat: Some(
            "Shim preserves source intent; source register history and macro history are not imported.",
        ),
        downgrade_triggers: &[
            "modal_profile_shim_changed",
            "clipboard_capability_layer_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/vim_neovim_profile.yaml",
        ],
        docs_help_refs: &["docs/migration/m3/incumbent_flow_matrix.md#vim"],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::VimNeovim,
        flow_slug: "lua-plugin-runtime",
        flow_label: "Lua plugin runtime",
        domain: ImportReviewDomain::ExtensionsAndProviders,
        classification: ImportMappingClassification::Unsupported,
        source_object_label: "Arbitrary Lua plugin runtime",
        aureline_target_label: "(no safe target)",
        before_after_summary: "Plugin runtime execution has no governed Aureline path; apply is denied for the runtime.",
        caveat: Some(
            "Arbitrary Vimscript/Lua plugin execution, register history, and macro history are outside the migration claim.",
        ),
        downgrade_triggers: &[
            "lua_runtime_policy_changed",
            "compat_row_extension_host_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/equivalence_cases/vim_plugin_unsupported.yaml",
            "fixtures/migration/compatibility_scorecards/blocked_lua_plugin_runtime.json",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#vim",
            "docs/migration/source_ecosystem_coverage_matrix.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    // Emacs
    FlowSeed {
        ecosystem: IncumbentEcosystem::Emacs,
        flow_slug: "global-keymap",
        flow_label: "Global keymap and selected presets",
        domain: ImportReviewDomain::Keymaps,
        classification: ImportMappingClassification::Translated,
        source_object_label: "Emacs global keymap",
        aureline_target_label: "aureline:keymaps.emacs_preset",
        before_after_summary: "Common global chords translate to Aureline command ids through the keymap preset.",
        caveat: Some(
            "Preset translation excludes Elisp-driven and mode-specific dynamic bindings.",
        ),
        downgrade_triggers: &[
            "aureline_command_id_renamed",
            "emacs_command_alias_changed",
            "keybinding_resolver_layer_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/emacs_profile.yaml",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#emacs",
            "docs/migration/keymap_presets.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::Emacs,
        flow_slug: "command-aliases",
        flow_label: "Interactive command aliases",
        domain: ImportReviewDomain::Shortcuts,
        classification: ImportMappingClassification::Exact,
        source_object_label: "Emacs M-x command aliases",
        aureline_target_label: "Aureline command aliases and palette metadata",
        before_after_summary: "Common interactive command aliases map directly to Aureline command palette metadata.",
        caveat: None,
        downgrade_triggers: &[],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/emacs_profile.yaml",
        ],
        docs_help_refs: &["docs/migration/m3/incumbent_flow_matrix.md#emacs"],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::Emacs,
        flow_slug: "project-defaults",
        flow_label: "Project root and file-exclude defaults",
        domain: ImportReviewDomain::WorkspaceProfile,
        classification: ImportMappingClassification::Partial,
        source_object_label: "Emacs project.el roots and excludes",
        aureline_target_label: "aureline:workspace.manifest_and_settings",
        before_after_summary: "Project roots and common excludes import; Elisp-driven overrides require manual review.",
        caveat: Some(
            "Project semantics derived from Elisp init files are not evaluated and may need manual review.",
        ),
        downgrade_triggers: &[
            "workspace_manifest_schema_changed",
            "post_import_validation_state_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/emacs_profile.yaml",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#emacs",
            "docs/migration/post_import_validation_contract.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::Emacs,
        flow_slug: "theme-token-shim",
        flow_label: "Selected color theme",
        domain: ImportReviewDomain::ThemesAndVisuals,
        classification: ImportMappingClassification::Shimmed,
        source_object_label: "Emacs selected color theme",
        aureline_target_label: "aureline:themes.token_mapping",
        before_after_summary: "Theme palette imports through a token-mapping shim that names every translated face explicitly.",
        caveat: Some(
            "Faces that depend on Elisp evaluation are surfaced as unsupported sub-rows rather than silently dropped.",
        ),
        downgrade_triggers: &[
            "theme_token_schema_changed",
            "design_token_vocabulary_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/source_profile_examples/emacs_profile.yaml",
        ],
        docs_help_refs: &["docs/migration/m3/incumbent_flow_matrix.md#emacs"],
        support_export_refs: &["support:migration-corpus:beta"],
    },
    FlowSeed {
        ecosystem: IncumbentEcosystem::Emacs,
        flow_slug: "elisp-runtime",
        flow_label: "Elisp package runtime",
        domain: ImportReviewDomain::ExtensionsAndProviders,
        classification: ImportMappingClassification::Unsupported,
        source_object_label: "Arbitrary Elisp package runtime",
        aureline_target_label: "(no safe target)",
        before_after_summary: "Elisp execution has no governed Aureline path; apply is denied for the runtime.",
        caveat: Some(
            "Elisp execution, package runtime parity, live buffers, and org-mode runtime parity are outside the claim.",
        ),
        downgrade_triggers: &[
            "elisp_runtime_policy_changed",
            "compat_row_extension_host_changed",
        ],
        evidence_refs: &[
            "fixtures/migration/compatibility_scorecards/blocked_elisp_package_runtime.json",
            "fixtures/migration/source_profile_examples/emacs_profile.yaml",
        ],
        docs_help_refs: &[
            "docs/migration/m3/incumbent_flow_matrix.md#emacs",
            "docs/migration/source_ecosystem_coverage_matrix.md",
        ],
        support_export_refs: &["support:migration-corpus:beta"],
    },
];

fn flow_id(ecosystem: IncumbentEcosystem, slug: &str) -> String {
    format!("migration-corpus-flow:{}:{}", ecosystem.as_str(), slug)
}

fn build_row(seed: &FlowSeed, wizard_mapping_report_ref: &str, rollback_checkpoint_ref: &str) -> IncumbentFlowRow {
    IncumbentFlowRow {
        record_kind: INCUMBENT_FLOW_ROW_RECORD_KIND.to_owned(),
        schema_version: MIGRATION_CORPUS_SCHEMA_VERSION,
        shared_contract_ref: MIGRATION_CORPUS_SHARED_CONTRACT_REF.to_owned(),
        flow_id: flow_id(seed.ecosystem, seed.flow_slug),
        ecosystem: seed.ecosystem,
        flow_label: seed.flow_label.to_owned(),
        domain: seed.domain,
        classification: seed.classification,
        source_object_label: seed.source_object_label.to_owned(),
        aureline_target_label: seed.aureline_target_label.to_owned(),
        before_after_summary: seed.before_after_summary.to_owned(),
        caveat: seed.caveat.map(str::to_owned),
        downgrade_triggers: seed.downgrade_triggers.iter().map(|s| (*s).to_owned()).collect(),
        evidence_refs: seed.evidence_refs.iter().map(|s| (*s).to_owned()).collect(),
        docs_help_refs: seed.docs_help_refs.iter().map(|s| (*s).to_owned()).collect(),
        support_export_refs: seed.support_export_refs.iter().map(|s| (*s).to_owned()).collect(),
        wizard_mapping_report_ref: wizard_mapping_report_ref.to_owned(),
        rollback_checkpoint_ref: rollback_checkpoint_ref.to_owned(),
    }
}

fn build_section(
    ecosystem: IncumbentEcosystem,
    wizard_mapping_report_ref: &str,
    rollback_checkpoint_ref: &str,
) -> EcosystemScoreboardSection {
    let mut rows: Vec<IncumbentFlowRow> = FLOW_SEEDS
        .iter()
        .filter(|seed| seed.ecosystem == ecosystem)
        .map(|seed| build_row(seed, wizard_mapping_report_ref, rollback_checkpoint_ref))
        .collect();
    rows.sort_by(|left, right| left.flow_id.cmp(&right.flow_id));

    let mut classifications_present: BTreeSet<ImportMappingClassification> = BTreeSet::new();
    let mut summary = ScoreboardClassificationSummary::empty();
    for row in &rows {
        classifications_present.insert(row.classification);
        summary.record(row.classification);
    }
    EcosystemScoreboardSection {
        record_kind: ECOSYSTEM_SECTION_RECORD_KIND.to_owned(),
        schema_version: MIGRATION_CORPUS_SCHEMA_VERSION,
        ecosystem,
        source_ecosystem_row_ref: ecosystem.source_ecosystem_row_ref().to_owned(),
        rows,
        classifications_present: classifications_present.into_iter().collect(),
        classification_summary: summary,
    }
}

/// Seeded fixture builder used by the headless inspector and the
/// integration test. The seed mirrors the JSON checked in under
/// `fixtures/migration/m3/incumbent_flows/`.
pub fn seeded_migration_scoreboard() -> MigrationScoreboard {
    let wizard = seeded_migration_wizard_page();
    let wizard_session_ref = wizard.wizard_session_id.clone();
    let wizard_mapping_report_ref = wizard.mapping_report.mapping_report_id.clone();
    let rollback_checkpoint_ref = wizard.rollback_checkpoint.checkpoint_ref.clone();

    let sections: Vec<EcosystemScoreboardSection> = IncumbentEcosystem::required_ecosystems()
        .iter()
        .map(|ecosystem| {
            build_section(*ecosystem, &wizard_mapping_report_ref, &rollback_checkpoint_ref)
        })
        .collect();

    let mut overall_summary = ScoreboardClassificationSummary::empty();
    let mut classifications_present: BTreeSet<ImportMappingClassification> = BTreeSet::new();
    for section in &sections {
        for row in &section.rows {
            overall_summary.record(row.classification);
            classifications_present.insert(row.classification);
        }
    }

    MigrationScoreboard {
        record_kind: MIGRATION_SCOREBOARD_RECORD_KIND.to_owned(),
        schema_version: MIGRATION_CORPUS_SCHEMA_VERSION,
        shared_contract_ref: MIGRATION_CORPUS_SHARED_CONTRACT_REF.to_owned(),
        scoreboard_id: MIGRATION_SCOREBOARD_ID.to_owned(),
        wizard_session_ref,
        wizard_mapping_report_ref,
        rollback_checkpoint_ref,
        sections,
        overall_summary,
        classifications_present: classifications_present.into_iter().collect(),
        published_scoreboard_ref: "artifacts/migration/m3/migration_scoreboard.md".to_owned(),
        published_flow_matrix_ref: "docs/migration/m3/incumbent_flow_matrix.md".to_owned(),
        docs_help_refs: vec![
            "docs/migration/m3/incumbent_flow_matrix.md".to_owned(),
            "docs/migration/m3/migration_wizard_beta.md".to_owned(),
            "docs/migration/source_ecosystem_coverage_matrix.md".to_owned(),
        ],
        support_export_refs: vec!["support:migration-corpus:beta".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_scoreboard_passes_validation() {
        let scoreboard = seeded_migration_scoreboard();
        validate_migration_scoreboard(&scoreboard).expect("seeded scoreboard must validate");
    }

    #[test]
    fn seeded_scoreboard_covers_every_required_ecosystem() {
        let scoreboard = seeded_migration_scoreboard();
        assert!(scoreboard.covers_every_required_ecosystem());
        for ecosystem in IncumbentEcosystem::required_ecosystems() {
            let section = scoreboard
                .sections
                .iter()
                .find(|section| section.ecosystem == ecosystem)
                .expect("section must be present");
            assert!(
                !section.rows.is_empty(),
                "section {} must have at least one row",
                ecosystem.as_str()
            );
        }
    }

    #[test]
    fn seeded_scoreboard_covers_every_required_classification() {
        let scoreboard = seeded_migration_scoreboard();
        assert!(scoreboard.covers_every_required_classification());
        assert!(scoreboard.overall_summary.exact >= 1);
        assert!(scoreboard.overall_summary.translated >= 1);
        assert!(scoreboard.overall_summary.partial >= 1);
        assert!(scoreboard.overall_summary.shimmed >= 1);
        assert!(scoreboard.overall_summary.unsupported >= 1);
    }

    #[test]
    fn non_exact_rows_have_downgrade_triggers() {
        let scoreboard = seeded_migration_scoreboard();
        for section in &scoreboard.sections {
            for row in &section.rows {
                if !matches!(row.classification, ImportMappingClassification::Exact) {
                    assert!(
                        !row.downgrade_triggers.is_empty(),
                        "flow {} must declare at least one downgrade trigger",
                        row.flow_id
                    );
                }
            }
        }
    }

    #[test]
    fn rows_quote_the_wizard_mapping_report() {
        let scoreboard = seeded_migration_scoreboard();
        let report_ref = &scoreboard.wizard_mapping_report_ref;
        for section in &scoreboard.sections {
            for row in &section.rows {
                assert_eq!(&row.wizard_mapping_report_ref, report_ref);
            }
        }
    }

    #[test]
    fn validation_flags_missing_ecosystem() {
        let mut scoreboard = seeded_migration_scoreboard();
        scoreboard
            .sections
            .retain(|section| section.ecosystem != IncumbentEcosystem::Emacs);
        let errors =
            validate_migration_scoreboard(&scoreboard).expect_err("must flag missing ecosystem");
        assert!(errors.iter().any(|err| matches!(
            err,
            MigrationScoreboardValidationError::MissingRequiredEcosystem { .. }
        )));
    }

    #[test]
    fn validation_flags_missing_downgrade_trigger() {
        let mut scoreboard = seeded_migration_scoreboard();
        for section in scoreboard.sections.iter_mut() {
            for row in section.rows.iter_mut() {
                if !matches!(row.classification, ImportMappingClassification::Exact) {
                    row.downgrade_triggers.clear();
                    break;
                }
            }
        }
        let errors = validate_migration_scoreboard(&scoreboard)
            .expect_err("must flag missing downgrade trigger");
        assert!(errors.iter().any(|err| matches!(
            err,
            MigrationScoreboardValidationError::MissingDowngradeTrigger { .. }
        )));
    }

    #[test]
    fn support_export_quotes_every_case_id() {
        let scoreboard = seeded_migration_scoreboard();
        let export = MigrationCorpusSupportExport::from_scoreboard(
            "support-export:migration-corpus:001",
            scoreboard.clone(),
        );
        assert_eq!(export.shared_contract_ref, MIGRATION_CORPUS_SHARED_CONTRACT_REF);
        assert!(export.case_ids.contains(&scoreboard.scoreboard_id));
        for section in &scoreboard.sections {
            for row in &section.rows {
                assert!(
                    export.case_ids.contains(&row.flow_id),
                    "case ids must quote flow {}",
                    row.flow_id
                );
            }
        }
    }
}
