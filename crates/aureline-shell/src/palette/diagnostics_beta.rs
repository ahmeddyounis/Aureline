//! Beta command-palette diagnostics packet.
//!
//! This module joins the descriptor-backed palette row projection with command
//! review sheets and deep-link review summaries. The packet is intentionally
//! deterministic so docs, fixtures, support export, and the headless inspector
//! can all quote the same palette diagnostics truth.

use std::collections::{BTreeSet, HashMap};

use aureline_commands::invocation::NoBypassGuards;
use aureline_commands::registry::seeded_registry;
use serde::{Deserialize, Serialize};

use super::discoverability::{
    materialize_alpha_palette_query, materialize_command_deep_link_review, AlphaFileCandidate,
    AlphaPaletteActionFooter, AlphaPalettePreviewPane, AlphaPaletteQueryInputs,
    AlphaPaletteResultRow, AlphaRecentActionCandidate, AlphaSymbolCandidate,
};
use crate::commands::CommandReviewRuntimeInputs;

/// Schema version for beta palette diagnostics records.
pub const COMMAND_PALETTE_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`BetaCommandPaletteDiagnosticsPack`].
pub const COMMAND_PALETTE_DIAGNOSTICS_RECORD_KIND: &str =
    "command_palette_diagnostics_beta_pack_record";

/// Stable record kind for [`BetaCommandPaletteDiagnosticsSupportExport`].
pub const COMMAND_PALETTE_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "command_palette_diagnostics_beta_support_export_record";

/// Stable record kind for [`BetaPaletteParityExamplesArtifact`].
pub const PALETTE_PARITY_EXAMPLES_RECORD_KIND: &str = "palette_parity_examples_record";

/// Stable packet id for the seeded beta palette diagnostics pack.
pub const COMMAND_PALETTE_DIAGNOSTICS_PACK_ID: &str =
    "shell:command_palette_diagnostics_beta:pack:v1";

/// Stable support-export id for the seeded beta palette diagnostics pack.
pub const COMMAND_PALETTE_DIAGNOSTICS_SUPPORT_EXPORT_ID: &str =
    "support-export:command-palette-diagnostics:beta:001";

/// Stable artifact id for palette parity examples.
pub const PALETTE_PARITY_EXAMPLES_ARTIFACT_ID: &str =
    "artifact:commands:palette_parity_examples:beta:v1";

const GENERATED_AT: &str = "2026-05-17T00:00:00Z";
const SOURCE_REGISTRY_REF: &str = "artifacts/commands/command_registry_seed.yaml";
const SOURCE_DESCRIPTOR_SCHEMA_REF: &str = "schemas/commands/command_descriptor.schema.json";
const SOURCE_PARITY_REPORT_REF: &str = "fixtures/commands/m3/command_parity/report.json";
const PUBLISHED_DOC_REF: &str = "docs/ux/m3/command_palette_diagnostics_beta.md";
const PACK_FIXTURE_REF: &str = "fixtures/ux/m3/command_palette_diagnostics/page.json";
const SUPPORT_EXPORT_REF: &str = "fixtures/ux/m3/command_palette_diagnostics/support_export.json";
const PARITY_EXAMPLES_REF: &str = "artifacts/commands/m3/palette_parity_examples.json";

/// Local-first privacy and retention posture for palette query history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaPaletteHistoryPolicy {
    /// Machine-readable history policy class.
    pub history_policy_class: String,
    /// Whether history remains machine-local by default.
    pub local_first: bool,
    /// Whether raw query text is omitted from support export.
    pub support_export_omits_raw_query_text: bool,
    /// Retention class used for palette query history.
    pub retention_class: String,
    /// Omitted material classes in redacted support export.
    pub omitted_material: Vec<String>,
}

/// Protected warm-open budget advertised by the beta palette diagnostics pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaPaletteWarmOpenBudget {
    /// Budget class shared with protected hot-path checks.
    pub budget_class: String,
    /// Warm open target in milliseconds.
    pub target_ms: u32,
    /// Whether the seeded diagnostic rows stay inside the claimed budget.
    pub claimed_rows_under_budget: bool,
    /// Provider strategy used to preserve the budget.
    pub provider_strategy: String,
}

/// Summary of the seeded beta palette diagnostics pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaCommandPaletteDiagnosticsSummary {
    /// Number of diagnostic rows included.
    pub row_count: usize,
    /// Number of command rows included.
    pub command_row_count: usize,
    /// Number of disabled or blocked rows included.
    pub disabled_or_blocked_row_count: usize,
    /// Number of preview or approval review rows included.
    pub preview_or_approval_row_count: usize,
    /// Number of deep-link review summaries included.
    pub deep_link_review_count: usize,
    /// Covered disabled-cause families.
    pub disabled_cause_families: Vec<String>,
    /// Footer action classes covered by at least one row.
    pub modifier_action_classes: Vec<String>,
    /// Whether every deep link summary preserves strict no-bypass guards.
    pub every_deep_link_preserves_no_bypass_guards: bool,
}

/// One beta diagnostic row projected from a palette command result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaCommandPaletteDiagnosticRow {
    /// Stable case id for this diagnostic row.
    pub case_id: String,
    /// Cause family exercised by this row.
    pub cause_family: String,
    /// Palette query class used to surface this row.
    pub query_class: String,
    /// Canonical command id for the row.
    pub command_id: String,
    /// Primary display label.
    pub label: String,
    /// Category, path, or scope detail.
    pub category_or_path: String,
    /// Origin/source badge displayed on the row.
    pub origin_source_badge: String,
    /// Current winning shortcut or `Unassigned`.
    pub winning_keybinding: String,
    /// Dominant side-effect cue displayed before run or insertion.
    pub dominant_side_effect_class: String,
    /// Descriptor-owned automation labels surfaced on the row.
    pub automation_labels: Vec<String>,
    /// Human-facing automation cues derived from the labels.
    pub automation_cues: Vec<String>,
    /// Availability class from canonical preflight.
    pub availability_class: String,
    /// Disabled reason code when material.
    pub disabled_reason_code: Option<String>,
    /// Preview pane projected for this command row.
    pub preview: AlphaPalettePreviewPane,
    /// Modifier and footer actions projected for this command row.
    pub action_footer: AlphaPaletteActionFooter,
}

/// Deep-link review summary proving external command references cannot bypass gates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaCommandPaletteDeepLinkReviewSummary {
    /// Stable case id for this deep-link review.
    pub case_id: String,
    /// Cause family exercised by this deep-link review.
    pub cause_family: String,
    /// Canonical command id under review.
    pub command_id: String,
    /// Issuing surface used by the review path.
    pub issuing_surface: String,
    /// Preflight decision returned by the canonical command engine.
    pub preflight_decision_class: String,
    /// Route outcome chosen before dispatch.
    pub route_outcome_class: String,
    /// Diagnostics reason when a diagnostics sheet is required.
    pub diagnostics_reason: Option<String>,
    /// Whether an invocation preview sheet was materialized.
    pub invocation_preview_sheet_present: bool,
    /// Whether the invocation preview path records preview as shown.
    pub preview_shown: Option<bool>,
    /// Approval state carried by the preview sheet, when any.
    pub approval_state: Option<String>,
    /// Strict no-bypass guard set inherited by result packets.
    pub no_bypass_guards: NoBypassGuards,
}

/// Deterministic beta diagnostics pack consumed by docs, fixtures, and support export.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BetaCommandPaletteDiagnosticsPack {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub pack_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source command registry artifact.
    pub source_registry_ref: String,
    /// Source descriptor schema artifact.
    pub source_descriptor_schema_ref: String,
    /// Source command parity report artifact.
    pub source_command_parity_report_ref: String,
    /// Published docs page for this packet.
    pub published_doc_ref: String,
    /// Fixture ref for this packet.
    pub pack_fixture_ref: String,
    /// Support-export fixture ref for this packet.
    pub support_export_ref: String,
    /// Palette parity examples artifact ref.
    pub parity_examples_ref: String,
    /// Query-history privacy posture.
    pub history_policy: BetaPaletteHistoryPolicy,
    /// Protected warm-open budget posture.
    pub warm_open_budget: BetaPaletteWarmOpenBudget,
    /// Summary counts and coverage.
    pub summary: BetaCommandPaletteDiagnosticsSummary,
    /// Command result rows exercised by the packet.
    pub rows: Vec<BetaCommandPaletteDiagnosticRow>,
    /// Deep-link reviews exercised by the packet.
    pub deep_link_reviews: Vec<BetaCommandPaletteDeepLinkReviewSummary>,
}

/// Redacted support export for a beta palette diagnostics pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaCommandPaletteDiagnosticsSupportExport {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Source pack id.
    pub source_pack_id: String,
    /// Redaction posture.
    pub redaction_class: String,
    /// Number of rows in the source pack.
    pub row_count: usize,
    /// Command ids quoted by the support export.
    pub command_ids: Vec<String>,
    /// Disabled reason codes quoted by the support export.
    pub disabled_reason_codes: Vec<String>,
    /// Cause families quoted by the support export.
    pub disabled_cause_families: Vec<String>,
    /// Material omitted from support export.
    pub omitted_material: Vec<String>,
    /// Reopen refs available to support and docs surfaces.
    pub reopen_refs: Vec<String>,
}

/// Shared descriptor truth projected into parity examples.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaPaletteParitySharedTruth {
    /// Canonical command id.
    pub command_id: String,
    /// Origin/source badge shared by palette and support.
    pub origin_source_badge: String,
    /// Dominant side-effect class shared by command surfaces.
    pub dominant_side_effect_class: String,
    /// Availability class shared by palette and review surfaces.
    pub availability_class: String,
    /// Disabled reason code shared by palette, keybinding, and support.
    pub disabled_reason_code: Option<String>,
    /// Automation labels shared by palette, CLI, recipe, and AI surfaces.
    pub automation_labels: Vec<String>,
    /// Preview class shared by palette, CLI, and AI surfaces.
    pub preview_class: Option<String>,
    /// Approval posture shared by palette, CLI, and AI surfaces.
    pub approval_posture_class: Option<String>,
}

/// One parity example showing the same command truth across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaPaletteParityExample {
    /// Stable example id.
    pub example_id: String,
    /// Shared truth every listed surface must quote.
    pub shared_truth: BetaPaletteParitySharedTruth,
    /// Surface families that consume the shared truth.
    pub surface_families: Vec<String>,
    /// Footer action classes projected for this command.
    pub modifier_action_classes: Vec<String>,
    /// Whether this example preserves preview and approval posture.
    pub preview_and_approval_preserved: bool,
    /// Whether this example keeps the command discoverable when unavailable.
    pub disabled_reason_explainable: bool,
}

/// Artifact containing palette parity examples for docs and release review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaPaletteParityExamplesArtifact {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable artifact id.
    pub artifact_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source diagnostics pack ref.
    pub source_pack_ref: String,
    /// Source command parity report ref.
    pub source_command_parity_report_ref: String,
    /// Parity examples.
    pub examples: Vec<BetaPaletteParityExample>,
}

/// Returns the deterministic beta command-palette diagnostics pack.
pub fn seeded_beta_command_palette_diagnostics_pack() -> BetaCommandPaletteDiagnosticsPack {
    let shortcuts = seeded_shortcuts();
    let rows = vec![
        diagnostic_row(
            "palette-diagnostics:row:enabled_open_folder",
            "enabled_direct",
            "literal_command_query",
            "cmd:workspace.open_folder",
            "open",
            runtime_desktop_trusted(),
            &shortcuts,
        ),
        diagnostic_row(
            "palette-diagnostics:row:trust_blocked_clone",
            "trust",
            "literal_command_query",
            "cmd:workspace.clone_repository",
            "clone",
            runtime_desktop_trust_restricted(),
            &shortcuts,
        ),
        diagnostic_row(
            "palette-diagnostics:row:policy_blocked_browser_handoff",
            "policy",
            "literal_command_query",
            "cmd:docs.open_in_browser",
            "browser",
            runtime_desktop_policy_blocked(),
            &shortcuts,
        ),
        diagnostic_row(
            "palette-diagnostics:row:missing_execution_context_open_folder",
            "missing_dependency",
            "literal_command_query",
            "cmd:workspace.open_folder",
            "open",
            runtime_desktop_execution_unavailable(),
            &shortcuts,
        ),
        diagnostic_row(
            "palette-diagnostics:row:degraded_provider_clone",
            "degraded_provider",
            "literal_command_query",
            "cmd:workspace.clone_repository",
            "clone",
            runtime_desktop_provider_unlinked(),
            &shortcuts,
        ),
        diagnostic_row(
            "palette-diagnostics:row:wrong_focus_restore_checkpoint",
            "wrong_focus",
            "literal_command_query",
            "cmd:workspace.restore_from_checkpoint",
            "restore",
            runtime_desktop_trusted(),
            &shortcuts,
        ),
        diagnostic_row(
            "palette-diagnostics:row:preview_required_import_profile",
            "preview_or_approval",
            "literal_command_query",
            "cmd:workspace.import_profile",
            "import",
            runtime_desktop_trusted(),
            &shortcuts,
        ),
    ];

    let deep_link_reviews = vec![
        deep_link_review_summary(
            "palette-diagnostics:deeplink:policy_blocked_browser_handoff",
            "policy",
            "cmd:docs.open_in_browser",
            runtime_desktop_policy_blocked(),
        ),
        deep_link_review_summary(
            "palette-diagnostics:deeplink:unsupported_surface_browser_handoff",
            "unsupported_surface",
            "cmd:docs.open_in_browser",
            runtime_cli_trusted(),
        ),
        deep_link_review_summary(
            "palette-diagnostics:deeplink:preview_required_import_profile",
            "preview_or_approval",
            "cmd:workspace.import_profile",
            runtime_desktop_trusted(),
        ),
    ];

    let summary = summarize(&rows, &deep_link_reviews);

    BetaCommandPaletteDiagnosticsPack {
        record_kind: COMMAND_PALETTE_DIAGNOSTICS_RECORD_KIND.to_string(),
        schema_version: COMMAND_PALETTE_DIAGNOSTICS_SCHEMA_VERSION,
        pack_id: COMMAND_PALETTE_DIAGNOSTICS_PACK_ID.to_string(),
        generated_at: GENERATED_AT.to_string(),
        source_registry_ref: SOURCE_REGISTRY_REF.to_string(),
        source_descriptor_schema_ref: SOURCE_DESCRIPTOR_SCHEMA_REF.to_string(),
        source_command_parity_report_ref: SOURCE_PARITY_REPORT_REF.to_string(),
        published_doc_ref: PUBLISHED_DOC_REF.to_string(),
        pack_fixture_ref: PACK_FIXTURE_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        parity_examples_ref: PARITY_EXAMPLES_REF.to_string(),
        history_policy: BetaPaletteHistoryPolicy {
            history_policy_class: "local_first_privacy_scoped".to_string(),
            local_first: true,
            support_export_omits_raw_query_text: true,
            retention_class: "machine_local_ephemeral".to_string(),
            omitted_material: vec![
                "raw_query_text".to_string(),
                "raw_argument_values".to_string(),
                "workspace_private_paths".to_string(),
            ],
        },
        warm_open_budget: BetaPaletteWarmOpenBudget {
            budget_class: "protected_warm_open".to_string(),
            target_ms: 50,
            claimed_rows_under_budget: true,
            provider_strategy: "recent_and_lexical_rows_first_semantic_rows_stream".to_string(),
        },
        summary,
        rows,
        deep_link_reviews,
    }
}

/// Builds the redacted support export for the seeded diagnostics pack.
pub fn seeded_beta_command_palette_diagnostics_support_export(
) -> BetaCommandPaletteDiagnosticsSupportExport {
    BetaCommandPaletteDiagnosticsSupportExport::from_pack(
        COMMAND_PALETTE_DIAGNOSTICS_SUPPORT_EXPORT_ID,
        &seeded_beta_command_palette_diagnostics_pack(),
    )
}

/// Builds the palette parity examples artifact for the seeded diagnostics pack.
pub fn seeded_beta_palette_parity_examples_artifact() -> BetaPaletteParityExamplesArtifact {
    BetaPaletteParityExamplesArtifact::from_pack(&seeded_beta_command_palette_diagnostics_pack())
}

/// Validates a beta command-palette diagnostics pack against the row contract.
pub fn validate_beta_command_palette_diagnostics_pack(
    pack: &BetaCommandPaletteDiagnosticsPack,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    if pack.record_kind != COMMAND_PALETTE_DIAGNOSTICS_RECORD_KIND {
        errors.push(format!("unexpected record_kind {}", pack.record_kind));
    }
    if pack.schema_version != COMMAND_PALETTE_DIAGNOSTICS_SCHEMA_VERSION {
        errors.push(format!("unexpected schema_version {}", pack.schema_version));
    }
    if pack.pack_id != COMMAND_PALETTE_DIAGNOSTICS_PACK_ID {
        errors.push(format!("unexpected pack_id {}", pack.pack_id));
    }
    if pack.rows.is_empty() {
        errors.push("pack must include at least one row".to_string());
    }
    if pack.summary != summarize(&pack.rows, &pack.deep_link_reviews) {
        errors.push("summary does not match row and deep-link contents".to_string());
    }
    if !pack.history_policy.local_first {
        errors.push("history policy must remain local-first".to_string());
    }
    if !pack.history_policy.support_export_omits_raw_query_text {
        errors.push("support export must omit raw query text".to_string());
    }
    if !pack
        .history_policy
        .omitted_material
        .iter()
        .any(|item| item == "raw_query_text")
    {
        errors.push("history policy must name raw_query_text as omitted".to_string());
    }
    if pack.warm_open_budget.target_ms > 50 || !pack.warm_open_budget.claimed_rows_under_budget {
        errors.push("warm-open budget must stay within 50ms for claimed rows".to_string());
    }

    let cause_families = pack
        .summary
        .disabled_cause_families
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "trust",
        "policy",
        "missing_dependency",
        "wrong_focus",
        "degraded_provider",
        "preview_or_approval",
        "unsupported_surface",
    ] {
        if !cause_families.contains(required) {
            errors.push(format!("missing required cause family {required}"));
        }
    }

    let modifier_actions = pack
        .summary
        .modifier_action_classes
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        "split_or_alternate_open",
        "copy_command_id",
        "copy_cli_headless_form",
        "add_to_recipe",
        "inspect_why_not_automatable",
    ] {
        if !modifier_actions.contains(required) {
            errors.push(format!("missing modifier action {required}"));
        }
    }
    if !modifier_actions.contains("primary_run_or_open")
        || !modifier_actions.contains("open_invocation_preview")
    {
        errors.push("modifier actions must cover direct run and invocation preview".to_string());
    }

    let mut case_ids = BTreeSet::new();
    for row in &pack.rows {
        if !case_ids.insert(row.case_id.as_str()) {
            errors.push(format!("duplicate row case_id {}", row.case_id));
        }
        validate_row(row, &mut errors);
    }

    let mut review_case_ids = BTreeSet::new();
    for review in &pack.deep_link_reviews {
        if !review_case_ids.insert(review.case_id.as_str()) {
            errors.push(format!("duplicate deep-link case_id {}", review.case_id));
        }
        if review.no_bypass_guards != NoBypassGuards::strict() {
            errors.push(format!(
                "{} does not preserve strict no-bypass guards",
                review.case_id
            ));
        }
        if review.route_outcome_class == "dispatch_allowed_without_preflight" {
            errors.push(format!(
                "{} attempts dispatch without preflight",
                review.case_id
            ));
        }
        if review.route_outcome_class == "diagnostics_sheet_required"
            && review.diagnostics_reason.is_none()
        {
            errors.push(format!("{} is missing diagnostics reason", review.case_id));
        }
        if review.route_outcome_class == "invocation_preview_required" {
            if !review.invocation_preview_sheet_present {
                errors.push(format!("{} is missing invocation preview", review.case_id));
            }
            if review.preview_shown != Some(true) {
                errors.push(format!("{} did not record preview shown", review.case_id));
            }
            if review.approval_state.as_deref() != Some("approval_pending") {
                errors.push(format!(
                    "{} did not preserve approval state",
                    review.case_id
                ));
            }
        }
    }

    if pack.summary.every_deep_link_preserves_no_bypass_guards
        != pack
            .deep_link_reviews
            .iter()
            .all(|review| review.no_bypass_guards == NoBypassGuards::strict())
    {
        errors.push("deep-link no-bypass summary is inconsistent".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates a redacted support export for command-palette diagnostics.
pub fn validate_beta_command_palette_diagnostics_support_export(
    export: &BetaCommandPaletteDiagnosticsSupportExport,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if export.record_kind != COMMAND_PALETTE_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(format!("unexpected record_kind {}", export.record_kind));
    }
    if export.schema_version != COMMAND_PALETTE_DIAGNOSTICS_SCHEMA_VERSION {
        errors.push(format!(
            "unexpected schema_version {}",
            export.schema_version
        ));
    }
    if export.source_pack_id != COMMAND_PALETTE_DIAGNOSTICS_PACK_ID {
        errors.push(format!(
            "unexpected source_pack_id {}",
            export.source_pack_id
        ));
    }
    if export.row_count == 0 {
        errors.push("support export must quote at least one row".to_string());
    }
    if export.redaction_class != "metadata_safe_no_query_text" {
        errors.push(format!(
            "unexpected redaction_class {}",
            export.redaction_class
        ));
    }
    if !export
        .omitted_material
        .iter()
        .any(|item| item == "raw_query_text")
    {
        errors.push("support export must omit raw_query_text".to_string());
    }
    for required_ref in [PACK_FIXTURE_REF, PUBLISHED_DOC_REF, PARITY_EXAMPLES_REF] {
        if !export.reopen_refs.iter().any(|item| item == required_ref) {
            errors.push(format!("support export missing reopen ref {required_ref}"));
        }
    }
    if export.command_ids.is_empty() {
        errors.push("support export must include command ids".to_string());
    }
    if export.disabled_reason_codes.is_empty() {
        errors.push("support export must include disabled reason codes".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates palette parity examples derived from the diagnostics pack.
pub fn validate_beta_palette_parity_examples_artifact(
    artifact: &BetaPaletteParityExamplesArtifact,
) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if artifact.record_kind != PALETTE_PARITY_EXAMPLES_RECORD_KIND {
        errors.push(format!("unexpected record_kind {}", artifact.record_kind));
    }
    if artifact.schema_version != COMMAND_PALETTE_DIAGNOSTICS_SCHEMA_VERSION {
        errors.push(format!(
            "unexpected schema_version {}",
            artifact.schema_version
        ));
    }
    if artifact.artifact_id != PALETTE_PARITY_EXAMPLES_ARTIFACT_ID {
        errors.push(format!("unexpected artifact_id {}", artifact.artifact_id));
    }
    if artifact.examples.is_empty() {
        errors.push("parity artifact must include examples".to_string());
    }

    let mut example_ids = BTreeSet::new();
    for example in &artifact.examples {
        if !example_ids.insert(example.example_id.as_str()) {
            errors.push(format!("duplicate example_id {}", example.example_id));
        }
        for surface in [
            "command_palette",
            "menu_or_button",
            "keybinding_help",
            "onboarding_hint",
            "cli_headless",
            "ai_tool_surface",
            "support_export",
        ] {
            if !example.surface_families.iter().any(|item| item == surface) {
                errors.push(format!("{} missing surface {surface}", example.example_id));
            }
        }
        if !example.preview_and_approval_preserved {
            errors.push(format!(
                "{} does not preserve preview and approval posture",
                example.example_id
            ));
        }
        if !example.disabled_reason_explainable {
            errors.push(format!(
                "{} does not keep disabled reason explainable",
                example.example_id
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

impl BetaCommandPaletteDiagnosticsSupportExport {
    /// Builds a redacted support export from a diagnostics pack.
    pub fn from_pack(support_export_id: &str, pack: &BetaCommandPaletteDiagnosticsPack) -> Self {
        let mut command_ids = pack
            .rows
            .iter()
            .map(|row| row.command_id.clone())
            .chain(
                pack.deep_link_reviews
                    .iter()
                    .map(|review| review.command_id.clone()),
            )
            .collect::<Vec<_>>();
        command_ids.sort();
        command_ids.dedup();

        let mut disabled_reason_codes = pack
            .rows
            .iter()
            .filter_map(|row| row.disabled_reason_code.clone())
            .chain(
                pack.deep_link_reviews
                    .iter()
                    .filter_map(|review| review.diagnostics_reason.clone()),
            )
            .collect::<Vec<_>>();
        disabled_reason_codes.sort();
        disabled_reason_codes.dedup();

        Self {
            record_kind: COMMAND_PALETTE_DIAGNOSTICS_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: COMMAND_PALETTE_DIAGNOSTICS_SCHEMA_VERSION,
            support_export_id: support_export_id.to_string(),
            source_pack_id: pack.pack_id.clone(),
            redaction_class: "metadata_safe_no_query_text".to_string(),
            row_count: pack.rows.len(),
            command_ids,
            disabled_reason_codes,
            disabled_cause_families: pack.summary.disabled_cause_families.clone(),
            omitted_material: pack.history_policy.omitted_material.clone(),
            reopen_refs: vec![
                pack.pack_fixture_ref.clone(),
                pack.published_doc_ref.clone(),
                pack.parity_examples_ref.clone(),
            ],
        }
    }
}

impl BetaPaletteParityExamplesArtifact {
    /// Builds parity examples from a diagnostics pack.
    pub fn from_pack(pack: &BetaCommandPaletteDiagnosticsPack) -> Self {
        Self {
            record_kind: PALETTE_PARITY_EXAMPLES_RECORD_KIND.to_string(),
            schema_version: COMMAND_PALETTE_DIAGNOSTICS_SCHEMA_VERSION,
            artifact_id: PALETTE_PARITY_EXAMPLES_ARTIFACT_ID.to_string(),
            generated_at: pack.generated_at.clone(),
            source_pack_ref: pack.pack_fixture_ref.clone(),
            source_command_parity_report_ref: pack.source_command_parity_report_ref.clone(),
            examples: pack
                .rows
                .iter()
                .map(BetaPaletteParityExample::from_row)
                .collect(),
        }
    }
}

impl BetaPaletteParityExample {
    fn from_row(row: &BetaCommandPaletteDiagnosticRow) -> Self {
        Self {
            example_id: format!(
                "palette-parity-example:{}:{}",
                sanitize_id(&row.command_id),
                sanitize_id(&row.cause_family)
            ),
            shared_truth: BetaPaletteParitySharedTruth {
                command_id: row.command_id.clone(),
                origin_source_badge: row.origin_source_badge.clone(),
                dominant_side_effect_class: row.dominant_side_effect_class.clone(),
                availability_class: row.availability_class.clone(),
                disabled_reason_code: row.disabled_reason_code.clone(),
                automation_labels: row.automation_labels.clone(),
                preview_class: Some(preview_class_for_row(row)),
                approval_posture_class: Some(approval_posture_for_row(row)),
            },
            surface_families: vec![
                "command_palette".to_string(),
                "menu_or_button".to_string(),
                "keybinding_help".to_string(),
                "onboarding_hint".to_string(),
                "cli_headless".to_string(),
                "ai_tool_surface".to_string(),
                "support_export".to_string(),
            ],
            modifier_action_classes: modifier_action_classes(&row.action_footer),
            preview_and_approval_preserved: row.action_footer.invocation_preview_required
                || row.availability_class == "enabled"
                || row.availability_class == "disabled_with_reason"
                || row.availability_class == "blocked_by_policy",
            disabled_reason_explainable: row.disabled_reason_code.is_some()
                == row.action_footer.command_diagnostics_sheet_available
                || row.disabled_reason_code.is_none(),
        }
    }
}

fn validate_row(row: &BetaCommandPaletteDiagnosticRow, errors: &mut Vec<String>) {
    if row.command_id.trim().is_empty() {
        errors.push(format!("{} is missing command_id", row.case_id));
    }
    if row.label.trim().is_empty() {
        errors.push(format!("{} is missing label", row.case_id));
    }
    if row.category_or_path.trim().is_empty() {
        errors.push(format!("{} is missing category_or_path", row.case_id));
    }
    if row.origin_source_badge.trim().is_empty() {
        errors.push(format!("{} is missing origin badge", row.case_id));
    }
    if row.winning_keybinding.trim().is_empty() {
        errors.push(format!("{} is missing keybinding display", row.case_id));
    }
    if row.dominant_side_effect_class.trim().is_empty() {
        errors.push(format!("{} is missing side-effect cue", row.case_id));
    }
    if row.automation_labels.is_empty() {
        errors.push(format!("{} is missing automation labels", row.case_id));
    }
    if row.automation_cues.is_empty() {
        errors.push(format!("{} is missing automation cues", row.case_id));
    }
    if row.preview.descriptor_command_id.as_deref() != Some(row.command_id.as_str()) {
        errors.push(format!(
            "{} preview does not quote canonical command id",
            row.case_id
        ));
    }
    if row.preview.side_effect_class != row.dominant_side_effect_class {
        errors.push(format!("{} preview side-effect drift", row.case_id));
    }
    if row.action_footer.copy_command_id.copy_payload.as_deref() != Some(row.command_id.as_str()) {
        errors.push(format!("{} copy command id payload drift", row.case_id));
    }
    if !row.action_footer.copy_command_id.enabled {
        errors.push(format!("{} copy command id must be enabled", row.case_id));
    }
    if row.disabled_reason_code.is_some() && !row.action_footer.command_diagnostics_sheet_available
    {
        errors.push(format!(
            "{} disabled row must offer diagnostics sheet",
            row.case_id
        ));
    }
    if row.action_footer.invocation_preview_required
        && row.action_footer.default_action.action_class != "open_invocation_preview"
    {
        errors.push(format!(
            "{} preview-required row must open invocation preview",
            row.case_id
        ));
    }
    match row.cause_family.as_str() {
        "trust" if row.disabled_reason_code.as_deref() != Some("workspace_trust_restricted") => {
            errors.push(format!("{} trust row has wrong reason", row.case_id));
        }
        "policy" if row.disabled_reason_code.as_deref() != Some("policy_blocked_in_context") => {
            errors.push(format!("{} policy row has wrong reason", row.case_id));
        }
        "missing_dependency"
            if row.disabled_reason_code.as_deref() != Some("execution_context_unavailable") =>
        {
            errors.push(format!(
                "{} missing-dependency row has wrong reason",
                row.case_id
            ));
        }
        "degraded_provider"
            if row.disabled_reason_code.as_deref() != Some("required_provider_unlinked") =>
        {
            errors.push(format!(
                "{} degraded-provider row has wrong reason",
                row.case_id
            ));
        }
        "wrong_focus"
            if row.disabled_reason_code.as_deref() != Some("required_argument_unresolved") =>
        {
            errors.push(format!("{} wrong-focus row has wrong reason", row.case_id));
        }
        "preview_or_approval" if !row.action_footer.invocation_preview_required => {
            errors.push(format!(
                "{} preview row did not require invocation preview",
                row.case_id
            ));
        }
        _ => {}
    }
}

fn diagnostic_row(
    case_id: &str,
    cause_family: &str,
    query_class: &str,
    command_id: &str,
    query: &str,
    runtime: CommandReviewRuntimeInputs<'static>,
    shortcuts: &HashMap<String, Vec<String>>,
) -> BetaCommandPaletteDiagnosticRow {
    let registry = seeded_registry();
    let empty_recent: [AlphaRecentActionCandidate; 0] = [];
    let empty_symbols: [AlphaSymbolCandidate; 0] = [];
    let empty_files: [AlphaFileCandidate; 0] = [];
    let snapshot = materialize_alpha_palette_query(AlphaPaletteQueryInputs {
        registry,
        query,
        shortcuts_by_command_id: shortcuts,
        runtime,
        recent_actions: &empty_recent,
        symbols: &empty_symbols,
        files: &empty_files,
    });
    let row = snapshot
        .rows
        .iter()
        .find(|row| row.command_id.as_deref() == Some(command_id))
        .unwrap_or_else(|| panic!("seeded palette diagnostics row missing {command_id}"))
        .clone();
    BetaCommandPaletteDiagnosticRow::from_palette_row(case_id, cause_family, query_class, row)
}

impl BetaCommandPaletteDiagnosticRow {
    fn from_palette_row(
        case_id: &str,
        cause_family: &str,
        query_class: &str,
        row: AlphaPaletteResultRow,
    ) -> Self {
        let command_id = row
            .command_id
            .clone()
            .expect("beta diagnostics rows must be command rows");
        Self {
            case_id: case_id.to_string(),
            cause_family: cause_family.to_string(),
            query_class: query_class.to_string(),
            command_id,
            label: row.label,
            category_or_path: row.category_or_path,
            origin_source_badge: row.origin_source_badge,
            winning_keybinding: row.winning_keybinding,
            dominant_side_effect_class: row.dominant_side_effect_class,
            automation_labels: row.automation_labels,
            automation_cues: row.automation_cues,
            availability_class: row.availability_class,
            disabled_reason_code: row.disabled_reason_code,
            preview: row.preview,
            action_footer: row.action_footer,
        }
    }
}

fn deep_link_review_summary(
    case_id: &str,
    cause_family: &str,
    command_id: &str,
    runtime: CommandReviewRuntimeInputs<'static>,
) -> BetaCommandPaletteDeepLinkReviewSummary {
    let review = materialize_command_deep_link_review(seeded_registry(), command_id, runtime)
        .unwrap_or_else(|| panic!("seeded deep-link review missing {command_id}"));
    let diagnostics_reason = review.diagnostics_sheet.as_ref().and_then(|sheet| {
        sheet
            .disabled_reason
            .as_ref()
            .map(|reason| reason.disabled_reason_code.clone())
    });
    let preview_shown = review
        .invocation_preview_sheet
        .as_ref()
        .map(|sheet| sheet.invocation_session.preview_posture.preview_shown);
    let approval_state = review.invocation_preview_sheet.as_ref().map(|sheet| {
        sheet
            .invocation_session
            .approval_posture
            .approval_state
            .clone()
    });

    BetaCommandPaletteDeepLinkReviewSummary {
        case_id: case_id.to_string(),
        cause_family: cause_family.to_string(),
        command_id: command_id.to_string(),
        issuing_surface: review.issuing_surface,
        preflight_decision_class: review.preflight_decision_class,
        route_outcome_class: review.route_outcome_class,
        diagnostics_reason,
        invocation_preview_sheet_present: review.invocation_preview_sheet.is_some(),
        preview_shown,
        approval_state,
        no_bypass_guards: review.no_bypass_guards,
    }
}

fn summarize(
    rows: &[BetaCommandPaletteDiagnosticRow],
    reviews: &[BetaCommandPaletteDeepLinkReviewSummary],
) -> BetaCommandPaletteDiagnosticsSummary {
    let disabled_or_blocked_row_count = rows
        .iter()
        .filter(|row| row.disabled_reason_code.is_some())
        .count();
    let preview_or_approval_row_count = rows
        .iter()
        .filter(|row| row.action_footer.invocation_preview_required)
        .count();

    let mut disabled_cause_families = rows
        .iter()
        .map(|row| row.cause_family.clone())
        .chain(reviews.iter().map(|review| review.cause_family.clone()))
        .collect::<Vec<_>>();
    disabled_cause_families.sort();
    disabled_cause_families.dedup();

    let mut modifier_action_classes = rows
        .iter()
        .flat_map(|row| modifier_action_classes(&row.action_footer))
        .collect::<Vec<_>>();
    modifier_action_classes.sort();
    modifier_action_classes.dedup();

    BetaCommandPaletteDiagnosticsSummary {
        row_count: rows.len(),
        command_row_count: rows.len(),
        disabled_or_blocked_row_count,
        preview_or_approval_row_count,
        deep_link_review_count: reviews.len(),
        disabled_cause_families,
        modifier_action_classes,
        every_deep_link_preserves_no_bypass_guards: reviews
            .iter()
            .all(|review| review.no_bypass_guards == NoBypassGuards::strict()),
    }
}

fn modifier_action_classes(footer: &AlphaPaletteActionFooter) -> Vec<String> {
    vec![
        footer.default_action.action_class.clone(),
        footer.split_or_alternate_open.action_class.clone(),
        footer.copy_command_id.action_class.clone(),
        footer.copy_cli_headless_form.action_class.clone(),
        footer.add_to_recipe.action_class.clone(),
        footer.inspect_why_not_automatable.action_class.clone(),
    ]
}

fn preview_class_for_row(row: &BetaCommandPaletteDiagnosticRow) -> String {
    row.preview
        .target_refs
        .iter()
        .find_map(|value| value.strip_prefix("preview-class:").map(str::to_string))
        .unwrap_or_else(|| "no_preview_required".to_string())
}

fn approval_posture_for_row(row: &BetaCommandPaletteDiagnosticRow) -> String {
    row.preview
        .target_refs
        .iter()
        .find_map(|value| value.strip_prefix("approval-posture:").map(str::to_string))
        .unwrap_or_else(|| "no_approval_required".to_string())
}

fn seeded_shortcuts() -> HashMap<String, Vec<String>> {
    HashMap::from([
        (
            "cmd:workspace.open_folder".to_string(),
            vec!["Cmd+O".to_string()],
        ),
        (
            "cmd:command_palette.open".to_string(),
            vec!["Cmd+Shift+P".to_string()],
        ),
        (
            "cmd:quick_open.toggle".to_string(),
            vec!["Cmd+P".to_string()],
        ),
    ])
}

fn runtime_desktop_trusted() -> CommandReviewRuntimeInputs<'static> {
    CommandReviewRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: "trusted",
        execution_context_available: true,
        provider_linked: Some(true),
        credential_available: Some(true),
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
    }
}

fn runtime_desktop_trust_restricted() -> CommandReviewRuntimeInputs<'static> {
    CommandReviewRuntimeInputs {
        workspace_trust_state: "restricted",
        ..runtime_desktop_trusted()
    }
}

fn runtime_desktop_policy_blocked() -> CommandReviewRuntimeInputs<'static> {
    CommandReviewRuntimeInputs {
        policy_blocked_in_context: true,
        ..runtime_desktop_trusted()
    }
}

fn runtime_desktop_execution_unavailable() -> CommandReviewRuntimeInputs<'static> {
    CommandReviewRuntimeInputs {
        execution_context_available: false,
        ..runtime_desktop_trusted()
    }
}

fn runtime_desktop_provider_unlinked() -> CommandReviewRuntimeInputs<'static> {
    CommandReviewRuntimeInputs {
        provider_linked: Some(false),
        ..runtime_desktop_trusted()
    }
}

fn runtime_cli_trusted() -> CommandReviewRuntimeInputs<'static> {
    CommandReviewRuntimeInputs {
        client_scope: "cli",
        ..runtime_desktop_trusted()
    }
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
            other => other,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn seeded_pack_covers_required_diagnostics() {
        let pack = seeded_beta_command_palette_diagnostics_pack();
        validate_beta_command_palette_diagnostics_pack(&pack).expect("seeded pack must validate");
        assert_eq!(pack.record_kind, COMMAND_PALETTE_DIAGNOSTICS_RECORD_KIND);
        assert_eq!(pack.rows.len(), 7);
        assert!(pack.history_policy.local_first);
        assert!(pack.history_policy.support_export_omits_raw_query_text);
        assert!(pack.warm_open_budget.claimed_rows_under_budget);
        assert_eq!(pack.warm_open_budget.target_ms, 50);

        let cause_families = pack
            .summary
            .disabled_cause_families
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for required in [
            "trust",
            "policy",
            "missing_dependency",
            "wrong_focus",
            "degraded_provider",
            "preview_or_approval",
            "unsupported_surface",
        ] {
            assert!(cause_families.contains(required), "missing {required}");
        }
    }

    #[test]
    fn command_rows_expose_footer_and_automation_truth() {
        let pack = seeded_beta_command_palette_diagnostics_pack();
        for row in &pack.rows {
            assert!(!row.origin_source_badge.trim().is_empty());
            assert!(!row.winning_keybinding.trim().is_empty());
            assert!(!row.dominant_side_effect_class.trim().is_empty());
            assert!(!row.automation_labels.is_empty(), "{}", row.command_id);
            assert!(!row.automation_cues.is_empty(), "{}", row.command_id);
            assert!(row.action_footer.copy_command_id.enabled);
            assert_eq!(
                row.action_footer.copy_command_id.copy_payload.as_deref(),
                Some(row.command_id.as_str())
            );
        }

        let open = pack
            .rows
            .iter()
            .find(|row| row.command_id == "cmd:workspace.open_folder")
            .expect("open folder row");
        assert_eq!(open.winning_keybinding, "Cmd+O");
        assert!(!open.action_footer.inspect_why_not_automatable.enabled);

        let import = pack
            .rows
            .iter()
            .find(|row| row.command_id == "cmd:workspace.import_profile")
            .expect("import profile row");
        assert!(import.action_footer.invocation_preview_required);
        assert!(import.action_footer.inspect_why_not_automatable.enabled);
    }

    #[test]
    fn deep_links_preserve_strict_no_bypass_guards() {
        let pack = seeded_beta_command_palette_diagnostics_pack();
        assert!(pack.summary.every_deep_link_preserves_no_bypass_guards);
        for review in &pack.deep_link_reviews {
            assert_eq!(review.no_bypass_guards, NoBypassGuards::strict());
            assert_ne!(
                review.route_outcome_class,
                "dispatch_allowed_without_preflight"
            );
        }
    }

    #[test]
    fn support_export_and_parity_examples_validate() {
        let support_export = seeded_beta_command_palette_diagnostics_support_export();
        validate_beta_command_palette_diagnostics_support_export(&support_export)
            .expect("support export must validate");

        let parity_artifact = seeded_beta_palette_parity_examples_artifact();
        validate_beta_palette_parity_examples_artifact(&parity_artifact)
            .expect("parity examples must validate");
    }
}
