//! Review enforcement for alpha destructive and external-effect commands.
//!
//! The command registry already carries preview and approval posture on each
//! descriptor. This module adds the alpha enforcement projection that asks the
//! release question directly: every destructive or external-effect command is
//! either routed through preview/apply/revert, covered by an equivalent
//! reviewed lane, or explicitly outside this alpha lane.

use aureline_commands::enablement::DisabledReasonCode;
use aureline_commands::invocation::CommandInvocationSession;
use aureline_commands::{CommandRegistry, CommandRegistryEntryRecord};
use serde::{Deserialize, Serialize};

/// Stable record kind emitted by [`AlphaReviewEnforcementSnapshot`].
pub const ALPHA_REVIEW_ENFORCEMENT_RECORD_KIND: &str = "alpha_review_enforcement_snapshot_record";

/// Schema version for [`AlphaReviewEnforcementSnapshot`].
pub const ALPHA_REVIEW_ENFORCEMENT_SCHEMA_VERSION: u32 = 1;

const NO_PREVIEW_REQUIRED: &str = "no_preview_required";
const NO_APPROVAL_REQUIRED: &str = "no_approval_required";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExternalReviewedCommandClaim {
    command_id: &'static str,
    canonical_verb: &'static str,
    command_revision_ref: &'static str,
    lane_class: &'static str,
    effect_class: &'static str,
    capability_scope_class: &'static str,
    dominant_side_effect_class: &'static str,
    preview_class: &'static str,
    approval_posture_class: &'static str,
    ai_tool_surfacing_class: &'static str,
    result_contract_class: &'static str,
    required_evidence_refs: &'static [&'static str],
    revert_posture_class: &'static str,
    surface_families: &'static [&'static str],
    support_refs: &'static [&'static str],
}

const EXTERNAL_REVIEWED_COMMAND_CLAIMS: &[ExternalReviewedCommandClaim] = &[
    ExternalReviewedCommandClaim {
        command_id: "cmd:git.push_branch",
        canonical_verb: "git.push_branch",
        command_revision_ref: "cmd-rev:git.push_branch:2026.04.21-01",
        lane_class: "git",
        effect_class: "external_effect",
        capability_scope_class: "externally_visible_mutation",
        dominant_side_effect_class: "remote_mutation",
        preview_class: "irreversible_publish_preview",
        approval_posture_class: "second_party_review_required",
        ai_tool_surfacing_class: "ai_callable_externally_visible_mutation_requires_approval",
        result_contract_class: "journal_entry_appended_ref",
        required_evidence_refs: &[
            "preview_record_ref",
            "approval_ticket_ref",
            "mutation_journal_entry_ref",
            "browser_handoff_packet_ref",
        ],
        revert_posture_class: "external_recovery_packet_or_browser_handoff",
        surface_families: &[
            "command_palette",
            "source_control_context_menu",
            "cli_help",
            "ai_tool_surface",
        ],
        support_refs: &[
            "fixtures/commands/command_descriptor_examples/git_push_branch.json",
            "fixtures/commands/invocation_result_cases/ai_push_branch_approval_pending.yaml",
            "fixtures/review/git_activity_alpha/git_review_activity_snapshot.json",
        ],
    },
    ExternalReviewedCommandClaim {
        command_id: "cmd:provider.publish_release_notes",
        canonical_verb: "provider.publish_release_notes",
        command_revision_ref: "cmd-rev:provider.publish_release_notes:alpha",
        lane_class: "provider",
        effect_class: "external_effect",
        capability_scope_class: "externally_visible_mutation",
        dominant_side_effect_class: "provider_visible_mutation",
        preview_class: "externally_mutating_preview",
        approval_posture_class: "explicit_confirmation_required",
        ai_tool_surfacing_class: "not_ai_callable",
        result_contract_class: "audit_event_emitted_ref",
        required_evidence_refs: &[
            "preview_record_ref",
            "approval_ticket_ref",
            "browser_handoff_packet_ref",
            "audit_event_ref",
        ],
        revert_posture_class: "provider_compensating_action_or_handoff",
        surface_families: &["command_palette", "provider_header", "browser_handoff"],
        support_refs: &[
            "fixtures/runtime/route_taxonomy_examples/route_changed_to_browser_handoff_publish.yaml",
            "fixtures/runtime/approval_ticket_examples/external_mutation_provider_publish.json",
        ],
    },
    ExternalReviewedCommandClaim {
        command_id: "cmd:package.install.apply",
        canonical_verb: "package.install.apply",
        command_revision_ref: "cmd-rev:package.install.apply:alpha",
        lane_class: "install",
        effect_class: "destructive_or_durable_mutation",
        capability_scope_class: "recoverable_durable_mutation",
        dominant_side_effect_class: "install_or_update",
        preview_class: "install_or_update_preview",
        approval_posture_class: "explicit_confirmation_required",
        ai_tool_surfacing_class: "not_ai_callable",
        result_contract_class: "package_review_packet_ref",
        required_evidence_refs: &[
            "package_review_packet_ref",
            "rollback_checkpoint_ref",
            "approval_ticket_ref",
        ],
        revert_posture_class: "rollback_via_lockfile_checkpoint",
        surface_families: &["package_review_sheet", "activity_center", "support_export"],
        support_refs: &[
            "fixtures/package/package_action_cases/rollback_preview_lockfile_checkpoint.json",
            "fixtures/ux/control_family_cases/busy_submit.json",
        ],
    },
    ExternalReviewedCommandClaim {
        command_id: "cmd:ai.apply_patch",
        canonical_verb: "ai.apply_patch",
        command_revision_ref: "cmd-rev:ai.apply_patch:alpha",
        lane_class: "ai_mutation",
        effect_class: "destructive_or_durable_mutation",
        capability_scope_class: "recoverable_durable_mutation",
        dominant_side_effect_class: "writes_files",
        preview_class: "structured_diff_preview",
        approval_posture_class: "explicit_confirmation_required",
        ai_tool_surfacing_class: "not_ai_callable",
        result_contract_class: "ai_evidence_packet_ref",
        required_evidence_refs: &[
            "preview_record_ref",
            "approval_ticket_ref",
            "ai_evidence_packet_ref",
            "rollback_checkpoint_ref",
        ],
        revert_posture_class: "restore_from_checkpoint",
        surface_families: &["ai_composer", "diff_review", "support_export"],
        support_refs: &[
            "fixtures/security/audit_stream_cases/ai_apply_action_applied_with_ticket.yaml",
            "fixtures/change/mutation_class_examples/ai_patch_apply.yaml",
        ],
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OutOfScopeAlphaCommand {
    command_id: &'static str,
    reason: &'static str,
}

const OUT_OF_SCOPE_ALPHA_COMMANDS: &[OutOfScopeAlphaCommand] = &[
    OutOfScopeAlphaCommand {
        command_id: "cmd:docs.open_in_browser",
        reason: "read_only_docs_handoff_emits_browser_handoff_packet_but_no_mutating_apply",
    },
    OutOfScopeAlphaCommand {
        command_id: "cmd:terminal.toggle",
        reason: "opens_terminal_surface_only_shell_input_and_paste_have_separate_review_lanes",
    },
    OutOfScopeAlphaCommand {
        command_id: "cmd:task.rerun_last",
        reason: "seed_descriptor_only_task_rerun_router_lands_in_execution_context_lane",
    },
    OutOfScopeAlphaCommand {
        command_id: "cmd:test.rerun_last",
        reason: "seed_descriptor_only_test_rerun_router_lands_in_execution_context_lane",
    },
];

/// One command row in the alpha review-enforcement snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaReviewEnforcementRow {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version for this row.
    pub schema_version: u32,
    /// Canonical command id under review.
    pub command_id: String,
    /// Descriptor revision or external fixture revision the row quotes.
    pub command_revision_ref: String,
    /// Canonical verb used by CLI, recipes, and AI surfaces.
    pub canonical_verb: String,
    /// Source of the row.
    pub source_class: String,
    /// Bounded alpha lane owning this command's review posture.
    pub lane_class: String,
    /// Effect class used by enforcement.
    pub effect_class: String,
    /// Descriptor capability scope.
    pub capability_scope_class: String,
    /// Dominant side-effect class surfaced to rows and review packets.
    pub dominant_side_effect_class: String,
    /// Descriptor preview posture.
    pub preview_class: String,
    /// Descriptor approval posture.
    pub approval_posture_class: String,
    /// AI tool surfacing posture.
    pub ai_tool_surfacing_class: String,
    /// Result contract class expected after apply.
    pub result_contract_class: String,
    /// Evidence references required by the reviewed result path.
    pub required_evidence_refs: Vec<String>,
    /// Enforcement requirement for this row.
    pub review_requirement_class: String,
    /// Current enforcement status.
    pub enforcement_status: String,
    /// Why another surface cannot bypass the reviewed path.
    pub bypass_protection_class: String,
    /// Recovery or revert posture advertised by the reviewed lane.
    pub revert_posture_class: String,
    /// Surface families covered by the descriptor or external claim.
    pub surface_families: Vec<String>,
    /// Reason token when this row is explicitly outside the alpha lane.
    pub explicit_out_of_scope_reason: Option<String>,
    /// Fixture, doc, or artifact refs supporting the row.
    pub support_refs: Vec<String>,
    /// Machine-readable findings. Empty means the row is conforming.
    pub finding_codes: Vec<String>,
}

impl AlphaReviewEnforcementRow {
    /// Returns true when this row is a destructive or external-effect row.
    pub fn is_destructive_or_external_effect(&self) -> bool {
        matches!(
            self.effect_class.as_str(),
            "destructive_or_durable_mutation" | "external_effect"
        )
    }

    /// Returns true when the row has no actionable enforcement gap.
    pub fn is_conforming(&self) -> bool {
        self.finding_codes.is_empty()
    }

    /// Returns true when apply must be preceded by a review path.
    pub fn requires_review(&self) -> bool {
        self.review_requirement_class == "review_required"
    }
}

/// Roll-up for [`AlphaReviewEnforcementSnapshot`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaReviewEnforcementSummary {
    /// Count of rows that require a reviewed path.
    pub review_required_count: usize,
    /// Count of review-required rows that are enforced.
    pub review_enforced_count: usize,
    /// Count of rows explicitly outside the alpha lane.
    pub explicit_out_of_scope_count: usize,
    /// Count of rows allowed to run directly.
    pub direct_allowed_count: usize,
    /// Count of actionable enforcement gaps.
    pub gap_count: usize,
    /// Lane classes present in the snapshot.
    pub covered_lane_classes: Vec<String>,
}

/// Inspectable snapshot for alpha review enforcement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaReviewEnforcementSnapshot {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version for this snapshot.
    pub schema_version: u32,
    /// Number of command-registry entries inspected.
    pub generated_from_registry_entry_count: usize,
    /// Ordered enforcement rows.
    pub rows: Vec<AlphaReviewEnforcementRow>,
    /// Roll-up counts for release checks and support export.
    pub summary: AlphaReviewEnforcementSummary,
}

impl AlphaReviewEnforcementSnapshot {
    /// Returns true when every destructive or external-effect row is either
    /// reviewed or explicitly out of scope.
    pub fn all_destructive_or_external_effect_commands_reviewed_or_out_of_scope(&self) -> bool {
        self.summary.gap_count == 0
    }

    /// Returns the row for `command_id`, if present.
    pub fn row_for_command(&self, command_id: &str) -> Option<&AlphaReviewEnforcementRow> {
        self.rows.iter().find(|row| row.command_id == command_id)
    }
}

/// Decision produced when checking a concrete invocation session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationReviewEnforcementDecision {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version for this decision.
    pub schema_version: u32,
    /// Command id checked.
    pub command_id: String,
    /// Decision class.
    pub decision_class: String,
    /// Disabled reason to propagate into an invocation result when denied.
    pub disabled_reason_code: Option<DisabledReasonCode>,
    /// Enforcement row used to make the decision.
    pub enforcement_row: AlphaReviewEnforcementRow,
}

impl InvocationReviewEnforcementDecision {
    /// Returns true when the invocation must not apply.
    pub fn is_denied(&self) -> bool {
        self.disabled_reason_code.is_some()
    }
}

/// Materializes the alpha review-enforcement snapshot for the command
/// registry and bounded external claim rows.
pub fn materialize_alpha_review_enforcement_snapshot(
    registry: &CommandRegistry,
) -> AlphaReviewEnforcementSnapshot {
    let mut rows: Vec<AlphaReviewEnforcementRow> = registry
        .entries()
        .iter()
        .map(review_enforcement_row_for_entry)
        .collect();
    rows.extend(
        EXTERNAL_REVIEWED_COMMAND_CLAIMS
            .iter()
            .map(review_enforcement_row_for_external_claim),
    );
    rows.sort_by(|left, right| left.command_id.cmp(&right.command_id));

    let summary = summarize_rows(&rows);
    AlphaReviewEnforcementSnapshot {
        record_kind: ALPHA_REVIEW_ENFORCEMENT_RECORD_KIND.to_string(),
        schema_version: ALPHA_REVIEW_ENFORCEMENT_SCHEMA_VERSION,
        generated_from_registry_entry_count: registry.entries().len(),
        rows,
        summary,
    }
}

/// Materializes the enforcement row for a registry entry.
pub fn review_enforcement_row_for_entry(
    entry: &CommandRegistryEntryRecord,
) -> AlphaReviewEnforcementRow {
    let command_id = entry.descriptor.command_id.as_str();
    let out_of_scope_reason = explicit_out_of_scope_reason(command_id).map(str::to_string);
    let descriptor_requires_review = descriptor_requires_review(entry);
    let high_effect_entry = entry.destructive_or_external_effect_class().is_some();
    let metadata_backed_gate = entry.preview_gate_metadata.is_some();
    let review_requirement_class = if out_of_scope_reason.is_some() {
        "explicitly_out_of_scope"
    } else if descriptor_requires_review {
        "review_required"
    } else if high_effect_entry && metadata_backed_gate {
        "metadata_backed_review_lane"
    } else if high_effect_entry {
        "review_required"
    } else {
        "direct_allowed"
    };
    let mut finding_codes = Vec::new();
    let enforcement_status = if out_of_scope_reason.is_some() {
        "explicitly_out_of_scope"
    } else if descriptor_requires_review {
        "enforced"
    } else if high_effect_entry && metadata_backed_gate {
        "enforced"
    } else if high_effect_entry {
        finding_codes.push("missing_review_path_for_destructive_or_external_effect".to_string());
        "gap_missing_review"
    } else {
        "direct_allowed"
    };

    AlphaReviewEnforcementRow {
        record_kind: "alpha_review_enforcement_row".to_string(),
        schema_version: 1,
        command_id: entry.descriptor.command_id.clone(),
        command_revision_ref: entry.descriptor.command_revision_ref.clone(),
        canonical_verb: entry.descriptor.canonical_verb.clone(),
        source_class: "command_registry_entry".to_string(),
        lane_class: lane_class_for_command(command_id).to_string(),
        effect_class: effect_class_for_entry(entry).to_string(),
        capability_scope_class: entry.descriptor.capability_scope_class.clone(),
        dominant_side_effect_class: entry.dominant_side_effect_class.clone(),
        preview_class: entry.descriptor.preview_class.clone(),
        approval_posture_class: entry.descriptor.approval_posture_class.clone(),
        ai_tool_surfacing_class: entry.descriptor.ai_tool_surfacing_class.clone(),
        result_contract_class: entry
            .descriptor
            .result_contract
            .result_contract_class
            .clone(),
        required_evidence_refs: entry
            .descriptor
            .result_contract
            .evidence_ref_class_required
            .clone(),
        review_requirement_class: review_requirement_class.to_string(),
        enforcement_status: enforcement_status.to_string(),
        bypass_protection_class: bypass_protection_class_for_entry(entry, enforcement_status)
            .to_string(),
        revert_posture_class: revert_posture_for_entry(entry),
        surface_families: surface_families_for_entry(entry),
        explicit_out_of_scope_reason: out_of_scope_reason,
        support_refs: support_refs_for_entry(entry),
        finding_codes,
    }
}

/// Enforces preview and approval posture for one concrete invocation session.
pub fn enforce_invocation_review_path(
    entry: &CommandRegistryEntryRecord,
    session: &CommandInvocationSession,
) -> InvocationReviewEnforcementDecision {
    let row = review_enforcement_row_for_entry(entry);
    let (decision_class, disabled_reason_code) = if row.enforcement_status == "gap_missing_review" {
        (
            "denied_preview_required".to_string(),
            Some(DisabledReasonCode::PreviewRequiredNotShown),
        )
    } else if row.review_requirement_class == "explicitly_out_of_scope" {
        ("explicitly_out_of_scope".to_string(), None)
    } else if !row.requires_review() {
        ("direct_trusted_path_allowed".to_string(), None)
    } else if session.execution_intent == "apply_direct_trusted_path" {
        (
            "denied_preview_required".to_string(),
            Some(DisabledReasonCode::PreviewRequiredNotShown),
        )
    } else if !session.preview_posture.preview_shown
        || session.preview_posture.preview_record_ref.is_none()
    {
        (
            "denied_preview_required".to_string(),
            Some(DisabledReasonCode::PreviewRequiredNotShown),
        )
    } else if row.approval_posture_class != NO_APPROVAL_REQUIRED
        && session.approval_posture.approval_state != "approval_granted"
    {
        (
            "denied_approval_required".to_string(),
            Some(DisabledReasonCode::ApprovalDenialNoApprovalPath),
        )
    } else {
        ("admitted_reviewed_apply".to_string(), None)
    };

    InvocationReviewEnforcementDecision {
        record_kind: "invocation_review_enforcement_decision".to_string(),
        schema_version: 1,
        command_id: entry.descriptor.command_id.clone(),
        decision_class,
        disabled_reason_code,
        enforcement_row: row,
    }
}

fn review_enforcement_row_for_external_claim(
    claim: &ExternalReviewedCommandClaim,
) -> AlphaReviewEnforcementRow {
    AlphaReviewEnforcementRow {
        record_kind: "alpha_review_enforcement_row".to_string(),
        schema_version: 1,
        command_id: claim.command_id.to_string(),
        command_revision_ref: claim.command_revision_ref.to_string(),
        canonical_verb: claim.canonical_verb.to_string(),
        source_class: "external_claim_fixture".to_string(),
        lane_class: claim.lane_class.to_string(),
        effect_class: claim.effect_class.to_string(),
        capability_scope_class: claim.capability_scope_class.to_string(),
        dominant_side_effect_class: claim.dominant_side_effect_class.to_string(),
        preview_class: claim.preview_class.to_string(),
        approval_posture_class: claim.approval_posture_class.to_string(),
        ai_tool_surfacing_class: claim.ai_tool_surfacing_class.to_string(),
        result_contract_class: claim.result_contract_class.to_string(),
        required_evidence_refs: claim
            .required_evidence_refs
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        review_requirement_class: "review_required".to_string(),
        enforcement_status: "enforced".to_string(),
        bypass_protection_class: "fixture_backed_equivalent_review_path".to_string(),
        revert_posture_class: claim.revert_posture_class.to_string(),
        surface_families: claim
            .surface_families
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        explicit_out_of_scope_reason: None,
        support_refs: claim
            .support_refs
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        finding_codes: Vec::new(),
    }
}

fn summarize_rows(rows: &[AlphaReviewEnforcementRow]) -> AlphaReviewEnforcementSummary {
    let review_required_count = rows
        .iter()
        .filter(|row| row.review_requirement_class == "review_required")
        .count();
    let review_enforced_count = rows
        .iter()
        .filter(|row| row.enforcement_status == "enforced")
        .count();
    let explicit_out_of_scope_count = rows
        .iter()
        .filter(|row| row.enforcement_status == "explicitly_out_of_scope")
        .count();
    let direct_allowed_count = rows
        .iter()
        .filter(|row| row.enforcement_status == "direct_allowed")
        .count();
    let gap_count = rows
        .iter()
        .filter(|row| row.enforcement_status == "gap_missing_review")
        .count();
    let mut covered_lane_classes: Vec<String> =
        rows.iter().map(|row| row.lane_class.clone()).collect();
    covered_lane_classes.sort();
    covered_lane_classes.dedup();

    AlphaReviewEnforcementSummary {
        review_required_count,
        review_enforced_count,
        explicit_out_of_scope_count,
        direct_allowed_count,
        gap_count,
        covered_lane_classes,
    }
}

fn descriptor_requires_review(entry: &CommandRegistryEntryRecord) -> bool {
    entry.descriptor.preview_class != NO_PREVIEW_REQUIRED
        || entry.descriptor.approval_posture_class != NO_APPROVAL_REQUIRED
}

fn effect_class_for_entry(entry: &CommandRegistryEntryRecord) -> &'static str {
    entry
        .destructive_or_external_effect_class()
        .unwrap_or("direct_local_or_read")
}

fn lane_class_for_command(command_id: &str) -> &'static str {
    if command_id.starts_with("cmd:workspace.") {
        "workspace"
    } else if command_id.starts_with("cmd:git.") {
        "git"
    } else if command_id.starts_with("cmd:provider.")
        || command_id.starts_with("cmd:source_control.")
        || command_id.starts_with("cmd:deploy.")
    {
        "provider"
    } else if command_id.starts_with("cmd:package.")
        || command_id.starts_with("cmd:packages.")
        || command_id.starts_with("cmd:extension.")
    {
        "install"
    } else if command_id.starts_with("cmd:ai.") {
        "ai_mutation"
    } else if command_id.starts_with("cmd:editor.") {
        "editor"
    } else if command_id.starts_with("cmd:terminal.")
        || command_id.starts_with("cmd:task.")
        || command_id.starts_with("cmd:test.")
    {
        "terminal_task"
    } else if command_id.starts_with("cmd:docs.") {
        "docs"
    } else {
        "shell"
    }
}

fn explicit_out_of_scope_reason(command_id: &str) -> Option<&'static str> {
    OUT_OF_SCOPE_ALPHA_COMMANDS
        .iter()
        .find(|row| row.command_id == command_id)
        .map(|row| row.reason)
}

fn bypass_protection_class_for_entry(
    entry: &CommandRegistryEntryRecord,
    enforcement_status: &str,
) -> &'static str {
    match enforcement_status {
        "enforced"
            if entry.preview_gate_metadata.is_some() && !descriptor_requires_review(entry) =>
        {
            "preview_gate_metadata_declared"
        }
        "enforced" => "descriptor_preflight_all_declared_surfaces",
        "explicitly_out_of_scope" => "explicit_scope_exclusion_listed",
        "gap_missing_review" => "not_protected_gap_blocks_alpha",
        _ if entry.descriptor.capability_scope_class == "reversible_local_mutation" => {
            "same_command_registry_direct_path"
        }
        _ => "direct_path_no_high_risk_effect",
    }
}

fn revert_posture_for_entry(entry: &CommandRegistryEntryRecord) -> String {
    if let Some(metadata) = entry.preview_gate_metadata.as_ref() {
        return metadata.revert_posture_class.clone();
    }
    match entry.descriptor.command_id.as_str() {
        "cmd:workspace.import_profile" | "cmd:workspace.restore_from_checkpoint" => {
            "restore_from_checkpoint"
        }
        "cmd:workspace.clone_repository" => "reviewed_materialization_no_in_place_mutation",
        "cmd:editor.save" => "save_review_or_editor_history",
        "cmd:editor.cut" | "cmd:editor.paste" | "cmd:editor.undo" | "cmd:editor.redo" => {
            "editor_undo"
        }
        _ if descriptor_requires_review(entry) => "review_packet_declares_recovery",
        _ => "not_applicable_no_reviewed_mutation",
    }
    .to_string()
}

fn surface_families_for_entry(entry: &CommandRegistryEntryRecord) -> Vec<String> {
    let mut surfaces: Vec<String> = entry
        .descriptor
        .ui_slot_hints
        .iter()
        .map(|hint| hint.ui_slot_class.clone())
        .collect();
    surfaces.extend(
        entry
            .preferred_surface_exposures
            .iter()
            .filter_map(|value| {
                value
                    .get("surface_class")
                    .and_then(|surface| surface.as_str())
                    .map(str::to_string)
            }),
    );
    surfaces.sort();
    surfaces.dedup();
    surfaces
}

fn support_refs_for_entry(entry: &CommandRegistryEntryRecord) -> Vec<String> {
    let mut refs = vec![
        "artifacts/commands/command_registry_seed.yaml".to_string(),
        format!("command-registry-entry:{}", entry.registry_entry_id),
    ];
    if let Some(metadata) = entry.preview_gate_metadata.as_ref() {
        refs.push(format!("preview-gate:{}", metadata.gate_class));
        refs.push(metadata.apply_guard_ref.clone());
    }
    refs
}
