//! Alpha launch-wedge palette discoverability projections.
//!
//! This module materializes a bounded, inspectable palette session for the
//! launch-critical discovery lane. It combines recent actions, command
//! descriptors, symbol candidates, and file candidates into one result-row
//! contract while reusing the canonical command registry, enablement engine,
//! diagnostics sheet, and invocation-preview sheet.

use std::collections::HashMap;

use aureline_commands::enablement::DisabledReasonCode;
use aureline_commands::invocation::{
    mint_approval_ticket_ref, mint_basis_snapshot_ref, mint_invocation_session_id,
    mint_preview_record_ref, AliasUsedBlock, ApprovalPostureBlock, CommandInvocationSession,
    ContextRefsBlock, EnablementDecisionBlock, InvocationContextSnapshot, NoBypassGuards,
    PreviewPostureBlock,
};
use aureline_commands::{
    automation_display_labels, labels_include, why_not_automatable_reason,
    CommandEnablementContext, CommandRegistry, CommandRegistryEntryRecord,
    ControlledAutomationLabel, PreflightDecision, PreflightDecisionClass,
};
use serde::{Deserialize, Serialize};

use super::preview::cli_skeleton_for;
use crate::commands::diagnostics_sheet::{
    materialize_command_diagnostics_sheet_record, CommandDiagnosticsSheetRecord,
};
use crate::commands::invocation_preview::{
    materialize_command_invocation_preview_sheet_record, CommandInvocationPreviewSheetRecord,
};
use crate::commands::review_enforcement::{
    review_enforcement_row_for_entry, AlphaReviewEnforcementRow,
};
use crate::commands::{argument_provenance_map_for, CommandReviewRuntimeInputs};

/// Maximum rows surfaced per category in the alpha discoverability snapshot.
pub const ALPHA_DISCOVERABILITY_LANE_CAP: usize = 6;

/// Stable row-kind vocabulary for alpha palette rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlphaPaletteRowKind {
    /// Recently used command or navigation target.
    RecentAction,
    /// Command projected from the canonical command registry.
    Command,
    /// Symbol target projected from a symbol or structural fallback provider.
    Symbol,
    /// Workspace file target projected from hot-set or lexical file state.
    File,
}

impl AlphaPaletteRowKind {
    /// Returns the stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecentAction => "recent_action",
            Self::Command => "command",
            Self::Symbol => "symbol",
            Self::File => "file",
        }
    }
}

/// Input candidate for a recent action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaRecentActionCandidate {
    /// Stable recent-entry id from the local recent-action store.
    pub recent_action_id: String,
    /// User-facing label rendered in the row.
    pub label: String,
    /// Stable category or path-like context rendered under the label.
    pub category_or_path: String,
    /// Optional canonical command id when the recent action replays a command.
    pub command_id: Option<String>,
    /// Opaque target refs restored by this recent action.
    pub target_refs: Vec<String>,
}

/// Input candidate for a symbol result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaSymbolCandidate {
    /// Symbol display name.
    pub symbol_name: String,
    /// Symbol kind token such as `function`, `type`, or `route`.
    pub symbol_kind: String,
    /// Workspace-relative path that owns the symbol.
    pub relative_path: String,
    /// Opaque symbol anchor ref used for navigation and support export.
    pub symbol_anchor_ref: String,
    /// Provider/source badge such as `symbol_index` or `structural_fallback`.
    pub origin_source_badge: String,
    /// Readiness or freshness token surfaced in preview detail.
    pub freshness_state: String,
}

/// Input candidate for a file result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaFileCandidate {
    /// Workspace-relative path to the file.
    pub relative_path: String,
    /// Opaque path or VFS identity ref for the file.
    pub path_identity_ref: String,
    /// Provider/source badge such as `file_index`, `recent_file`, or `hot_set`.
    pub origin_source_badge: String,
    /// Readiness or freshness token surfaced in preview detail.
    pub freshness_state: String,
}

/// Runtime inputs for materializing an alpha palette query snapshot.
pub struct AlphaPaletteQueryInputs<'a> {
    /// Canonical command registry used for command rows and command-backed recents.
    pub registry: &'a CommandRegistry,
    /// Current query text from the palette search field.
    pub query: &'a str,
    /// Shortcut resolver output keyed by canonical command id.
    pub shortcuts_by_command_id: &'a HashMap<String, Vec<String>>,
    /// Runtime posture used for command enablement and preflight.
    pub runtime: CommandReviewRuntimeInputs<'a>,
    /// Recent actions supplied by the local shell state.
    pub recent_actions: &'a [AlphaRecentActionCandidate],
    /// Symbol candidates supplied by the current symbol/search provider.
    pub symbols: &'a [AlphaSymbolCandidate],
    /// File candidates supplied by hot-set or lexical file discovery.
    pub files: &'a [AlphaFileCandidate],
}

/// Serializable snapshot for the alpha palette discoverability lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaPaletteDiscoverabilitySnapshot {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version for this projection.
    pub schema_version: u32,
    /// Raw local query text for runtime inspection fixtures.
    pub query: String,
    /// Provider summaries for the surfaced categories.
    pub providers: Vec<AlphaPaletteProviderSummary>,
    /// Ordered rows shown by the alpha lane.
    pub rows: Vec<AlphaPaletteResultRow>,
}

/// Redacted support/export projection for an alpha palette snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaPaletteSupportExport {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version for this projection.
    pub schema_version: u32,
    /// Source record kind consumed by this export.
    pub source_record_kind: String,
    /// Redaction posture for omitted query material.
    pub redaction_class: String,
    /// Number of rows captured from the source snapshot.
    pub row_count: usize,
    /// Row-kind tokens captured from the source snapshot.
    pub row_kind_tokens: Vec<String>,
    /// Command ids captured from command-backed rows.
    pub command_ids: Vec<String>,
    /// Material intentionally omitted from the support export.
    pub omitted_material: Vec<String>,
}

/// Count summary for one source category in a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaPaletteProviderSummary {
    /// Provider/category token.
    pub provider_class: String,
    /// Stable readiness state for the provider.
    pub state_class: String,
    /// Number of visible rows from this provider.
    pub visible_result_count: usize,
}

/// One row in the alpha palette result list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaPaletteResultRow {
    /// Stable row id minted from the row kind and target identity.
    pub row_id: String,
    /// Stable row kind.
    pub row_kind: AlphaPaletteRowKind,
    /// Primary display label.
    pub label: String,
    /// Category, owner path, or relevant scope for the row.
    pub category_or_path: String,
    /// Origin/source badge exposed on the row.
    pub origin_source_badge: String,
    /// Current winning keybinding or `Unassigned`.
    pub winning_keybinding: String,
    /// Dominant side-effect cue surfaced before invocation or insertion.
    pub dominant_side_effect_class: String,
    /// Descriptor-owned automation labels surfaced on command rows.
    #[serde(default)]
    pub automation_labels: Vec<String>,
    /// Human-facing automation cues derived from [`Self::automation_labels`].
    #[serde(default)]
    pub automation_cues: Vec<String>,
    /// Availability class derived from command preflight or target readiness.
    pub availability_class: String,
    /// Structured disabled reason when the row is not directly invocable.
    pub disabled_reason_code: Option<String>,
    /// Canonical command id when this row resolves to a command.
    pub command_id: Option<String>,
    /// Opaque target refs used by preview, navigation, or support export.
    pub target_refs: Vec<String>,
    /// Ranking reason classes that explain why the row appeared.
    pub ranking_reason_classes: Vec<String>,
    /// Preview-pane payload for the selected row.
    pub preview: AlphaPalettePreviewPane,
    /// Action-footer payload for modifier and copy behaviors.
    pub action_footer: AlphaPaletteActionFooter,
}

/// Preview-pane payload for one alpha palette row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaPalettePreviewPane {
    /// Preview family such as `command_descriptor`, `symbol_target`, or `file_target`.
    pub preview_kind: String,
    /// Optional canonical command id for command-backed rows.
    pub descriptor_command_id: Option<String>,
    /// Preflight decision token for command-backed rows.
    pub preflight_decision_class: Option<String>,
    /// Scope summary token rendered before apply/open.
    pub scope_summary: String,
    /// Dominant side-effect class rendered in the preview pane.
    pub side_effect_class: String,
    /// Target refs rendered in the preview pane.
    pub target_refs: Vec<String>,
    /// Rollback or checkpoint posture rendered before apply.
    pub rollback_or_checkpoint_posture: String,
}

/// Action-footer payload for one alpha palette row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaPaletteActionFooter {
    /// Primary action for Enter.
    pub default_action: AlphaPaletteFooterAction,
    /// Split or alternate-open action when relevant.
    pub split_or_alternate_open: AlphaPaletteFooterAction,
    /// Copy canonical command id action.
    pub copy_command_id: AlphaPaletteFooterAction,
    /// Copy CLI/headless skeleton action.
    pub copy_cli_headless_form: AlphaPaletteFooterAction,
    /// Add command to a recipe action.
    pub add_to_recipe: AlphaPaletteFooterAction,
    /// Inspect why the command is not automatable action.
    pub inspect_why_not_automatable: AlphaPaletteFooterAction,
    /// True when a disabled command can open the diagnostics sheet.
    pub command_diagnostics_sheet_available: bool,
    /// True when primary action must route to an invocation preview sheet.
    pub invocation_preview_required: bool,
}

/// One footer action slot in the alpha palette row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaPaletteFooterAction {
    /// Stable action class token.
    pub action_class: String,
    /// Whether the action can be selected.
    pub enabled: bool,
    /// Structured reason ref or code when the action is unavailable.
    pub unavailable_reason: Option<String>,
    /// Copy payload for copy-only actions.
    pub copy_payload: Option<String>,
}

/// Command deep-link projection that proves no deep-link invocation bypasses
/// descriptor, enablement, diagnostics, or preview checks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandDeepLinkReviewRecord {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version for this projection.
    pub schema_version: u32,
    /// Canonical command id requested by the deep link.
    pub command_id: String,
    /// Surface through which the command is being reviewed.
    pub issuing_surface: String,
    /// Preflight decision returned by the canonical command engine.
    pub preflight_decision_class: String,
    /// Outcome chosen for the deep-link route.
    pub route_outcome_class: String,
    /// Strict no-bypass guard set inherited by command result packets.
    pub no_bypass_guards: NoBypassGuards,
    /// Review-enforcement row proving whether the command is reviewed,
    /// directly allowed, or explicitly outside this alpha lane.
    pub review_enforcement: AlphaReviewEnforcementRow,
    /// Diagnostics sheet when the command is disabled or blocked.
    pub diagnostics_sheet: Option<CommandDiagnosticsSheetRecord>,
    /// Invocation preview sheet when preview or approval is required.
    pub invocation_preview_sheet: Option<CommandInvocationPreviewSheetRecord>,
}

/// Materializes a full alpha discoverability snapshot from command, symbol,
/// file, and recent-action candidates.
pub fn materialize_alpha_palette_query(
    inputs: AlphaPaletteQueryInputs<'_>,
) -> AlphaPaletteDiscoverabilitySnapshot {
    let normalized = normalize_query(inputs.query);
    let mut rows: Vec<(usize, i32, String, AlphaPaletteResultRow)> = Vec::new();

    for recent in inputs.recent_actions {
        if !matches_recent_action(recent, &normalized, inputs.registry) {
            continue;
        }
        if let Some(row) = recent_action_row(recent, &inputs, &normalized) {
            rows.push((
                0,
                score_text(&recent.label, &normalized),
                recent.label.clone(),
                row,
            ));
        }
    }

    for entry in inputs.registry.entries() {
        if !command_is_palette_visible(
            entry,
            inputs.runtime.client_scope,
            inputs.runtime.labs_enabled,
        ) {
            continue;
        }
        if !matches_command(entry, &normalized) {
            continue;
        }
        let row = command_result_row(entry, &inputs, &normalized, AlphaPaletteRowKind::Command);
        rows.push((
            1,
            score_text(&entry.title, &normalized),
            entry.title.clone(),
            row,
        ));
    }

    for symbol in inputs.symbols {
        if !matches_symbol(symbol, &normalized) {
            continue;
        }
        let row = symbol_result_row(symbol, &normalized);
        rows.push((
            2,
            score_text(&symbol.symbol_name, &normalized),
            symbol.symbol_name.clone(),
            row,
        ));
    }

    for file in inputs.files {
        if !matches_file(file, &normalized) {
            continue;
        }
        let label = file_label(&file.relative_path);
        let row = file_result_row(file, &normalized);
        rows.push((3, score_text(&label, &normalized), label, row));
    }

    rows.sort_by(|a, b| (a.0, a.1, &a.2).cmp(&(b.0, b.1, &b.2)));

    let mut lane_counts: HashMap<&'static str, usize> = HashMap::new();
    let rows: Vec<AlphaPaletteResultRow> = rows
        .into_iter()
        .filter_map(|(_, _, _, row)| {
            let key = row.row_kind.as_str();
            let count = lane_counts.entry(key).or_insert(0);
            if *count >= ALPHA_DISCOVERABILITY_LANE_CAP {
                return None;
            }
            *count += 1;
            Some(row)
        })
        .collect();

    AlphaPaletteDiscoverabilitySnapshot {
        record_kind: "alpha_palette_discoverability_snapshot".to_string(),
        schema_version: 1,
        query: inputs.query.to_string(),
        providers: provider_summaries(&rows),
        rows,
    }
}

/// Materializes a redacted support/export projection for a palette snapshot.
pub fn materialize_alpha_palette_support_export(
    snapshot: &AlphaPaletteDiscoverabilitySnapshot,
) -> AlphaPaletteSupportExport {
    let mut command_ids: Vec<String> = snapshot
        .rows
        .iter()
        .filter_map(|row| row.command_id.clone())
        .collect();
    command_ids.sort();
    command_ids.dedup();

    AlphaPaletteSupportExport {
        record_kind: "alpha_palette_support_export".to_string(),
        schema_version: 1,
        source_record_kind: snapshot.record_kind.clone(),
        redaction_class: "metadata_safe_no_query_text".to_string(),
        row_count: snapshot.rows.len(),
        row_kind_tokens: snapshot
            .rows
            .iter()
            .map(|row| row.row_kind.as_str().to_string())
            .collect(),
        command_ids,
        omitted_material: vec!["raw_query_text".to_string()],
    }
}

/// Materializes the command review route used for a command deep link.
pub fn materialize_command_deep_link_review(
    registry: &CommandRegistry,
    command_id: &str,
    runtime: CommandReviewRuntimeInputs<'_>,
) -> Option<CommandDeepLinkReviewRecord> {
    let entry = registry.get(command_id)?;
    let preflight = preflight_for(entry, runtime);
    let preflight_decision_class = preflight_decision_token(preflight.decision_class).to_string();

    let (route_outcome_class, diagnostics_sheet, invocation_preview_sheet) = match preflight
        .decision_class
    {
        PreflightDecisionClass::BlockedByPolicy | PreflightDecisionClass::DisabledWithReason => (
            "diagnostics_sheet_required".to_string(),
            Some(materialize_command_diagnostics_sheet_record(entry, runtime)),
            None,
        ),
        PreflightDecisionClass::PreviewRequired | PreflightDecisionClass::ApprovalRequired => {
            let session = materialize_invocation_session_for_review(
                entry,
                runtime,
                "command_deep_link",
                "external_command_deeplink_review",
                &preflight,
            );
            (
                "invocation_preview_required".to_string(),
                None,
                Some(materialize_command_invocation_preview_sheet_record(
                    entry, &session, runtime,
                )),
            )
        }
        PreflightDecisionClass::Allowed => {
            ("dispatch_allowed_after_preflight".to_string(), None, None)
        }
    };

    Some(CommandDeepLinkReviewRecord {
        record_kind: "command_deep_link_review_record".to_string(),
        schema_version: 1,
        command_id: command_id.to_string(),
        issuing_surface: "command_deep_link".to_string(),
        preflight_decision_class,
        route_outcome_class,
        no_bypass_guards: NoBypassGuards::strict(),
        review_enforcement: review_enforcement_row_for_entry(entry),
        diagnostics_sheet,
        invocation_preview_sheet,
    })
}

/// Builds an invocation session that can be quoted by invocation-preview sheets.
pub fn materialize_invocation_session_for_review(
    entry: &CommandRegistryEntryRecord,
    runtime: CommandReviewRuntimeInputs<'_>,
    issuing_surface: &str,
    authority_class: &str,
    preflight: &PreflightDecision,
) -> CommandInvocationSession {
    let argument_provenance_map = argument_provenance_map_for(entry);
    let preview_required = matches!(
        preflight.decision_class,
        PreflightDecisionClass::PreviewRequired | PreflightDecisionClass::ApprovalRequired
    );
    let approval_required = entry.descriptor.approval_posture_class != "no_approval_required";
    let basis_snapshot_ref = mint_basis_snapshot_ref(&entry.descriptor.canonical_verb);
    let focused_entity_ref = Some("shell-zone:main_workspace".to_string());
    let execution_context_id = entry.descriptor.policy_context.execution_context_id.clone();

    CommandInvocationSession {
        invocation_session_id: mint_invocation_session_id(&entry.descriptor.canonical_verb),
        canonical_command_id: entry.descriptor.command_id.clone(),
        command_revision_ref: entry.descriptor.command_revision_ref.clone(),
        canonical_verb: entry.descriptor.canonical_verb.clone(),
        issuing_surface: issuing_surface.to_string(),
        authority_class: authority_class.to_string(),
        alias_used: AliasUsedBlock {
            alias_kind: "canonical".to_string(),
            alias_id: None,
            alias_state: "not_applicable".to_string(),
            resolves_to_canonical_command_id: entry.descriptor.command_id.clone(),
            migration_trace_ref: None,
            support_window_ref: None,
        },
        argument_provenance_map: argument_provenance_map.clone(),
        context_snapshot: InvocationContextSnapshot {
            focused_entity_ref: focused_entity_ref.clone(),
            selection_ref: None,
            workspace_trust_state: runtime.workspace_trust_state.to_string(),
            execution_context_id: execution_context_id.clone(),
            scope_filter_class_ref: None,
            basis_snapshot_ref: basis_snapshot_ref.clone(),
        },
        context_refs: ContextRefsBlock {
            focused_entity_ref,
            selection_ref: None,
            workspace_ref: None,
            workspace_trust_state: runtime.workspace_trust_state.to_string(),
            execution_context_id: execution_context_id.clone(),
            scope_filter_class_ref: None,
            basis_snapshot_ref,
            context_object_refs: preview_target_refs(entry),
        },
        enablement_decision: EnablementDecisionBlock {
            decision_class: preflight.enablement_snapshot.decision_class,
            disabled_reason_code: preflight.enablement_snapshot.disabled_reason_code,
            repair_hook_ref: preflight.enablement_snapshot.repair_hook_ref.clone(),
        },
        preview_posture: PreviewPostureBlock {
            preview_class_declared: entry.descriptor.preview_class.clone(),
            preview_shown: preview_required,
            preview_record_ref: preview_required
                .then(|| mint_preview_record_ref(&entry.descriptor.canonical_verb)),
        },
        approval_posture: ApprovalPostureBlock {
            approval_posture_class_declared: entry.descriptor.approval_posture_class.clone(),
            approval_state: if approval_required {
                "approval_pending".to_string()
            } else {
                "not_required".to_string()
            },
            approval_ticket_ref: approval_required
                .then(|| mint_approval_ticket_ref(&entry.descriptor.canonical_verb)),
        },
        execution_intent: if preview_required {
            "apply_after_preview".to_string()
        } else {
            "apply_direct_trusted_path".to_string()
        },
        policy_context: entry.descriptor.policy_context.clone(),
        redaction_class: entry.descriptor.redaction_class.clone(),
    }
}

fn recent_action_row(
    recent: &AlphaRecentActionCandidate,
    inputs: &AlphaPaletteQueryInputs<'_>,
    normalized_query: &str,
) -> Option<AlphaPaletteResultRow> {
    if let Some(command_id) = recent.command_id.as_deref() {
        let entry = inputs.registry.get(command_id)?;
        let mut row = command_result_row(
            entry,
            inputs,
            normalized_query,
            AlphaPaletteRowKind::RecentAction,
        );
        row.row_id = format!(
            "alpha-palette:recent:{}",
            sanitize_id(&recent.recent_action_id)
        );
        row.label = recent.label.clone();
        row.category_or_path = recent.category_or_path.clone();
        row.origin_source_badge = "recent_history".to_string();
        row.target_refs = recent.target_refs.clone();
        row.preview.preview_kind = "recent_command".to_string();
        row.preview.target_refs = recent.target_refs.clone();
        row.ranking_reason_classes = vec![
            "recent_action".to_string(),
            "command_descriptor".to_string(),
        ];
        return Some(row);
    }

    Some(AlphaPaletteResultRow {
        row_id: format!(
            "alpha-palette:recent:{}",
            sanitize_id(&recent.recent_action_id)
        ),
        row_kind: AlphaPaletteRowKind::RecentAction,
        label: recent.label.clone(),
        category_or_path: recent.category_or_path.clone(),
        origin_source_badge: "recent_history".to_string(),
        winning_keybinding: "Unassigned".to_string(),
        dominant_side_effect_class: "restores_existing_context".to_string(),
        automation_labels: Vec::new(),
        automation_cues: Vec::new(),
        availability_class: "enabled".to_string(),
        disabled_reason_code: None,
        command_id: None,
        target_refs: recent.target_refs.clone(),
        ranking_reason_classes: vec!["recent_action".to_string()],
        preview: AlphaPalettePreviewPane {
            preview_kind: "recent_action".to_string(),
            descriptor_command_id: None,
            preflight_decision_class: None,
            scope_summary: recent.category_or_path.clone(),
            side_effect_class: "restores_existing_context".to_string(),
            target_refs: recent.target_refs.clone(),
            rollback_or_checkpoint_posture: "no_checkpoint_needed_existing_context".to_string(),
        },
        action_footer: target_action_footer(true),
    })
}

fn command_result_row(
    entry: &CommandRegistryEntryRecord,
    inputs: &AlphaPaletteQueryInputs<'_>,
    normalized_query: &str,
    row_kind: AlphaPaletteRowKind,
) -> AlphaPaletteResultRow {
    let preflight = preflight_for(entry, inputs.runtime);
    let preflight_class = preflight_decision_token(preflight.decision_class).to_string();
    let availability_class = availability_for_preflight(preflight.decision_class).to_string();
    let disabled_reason_code =
        disabled_reason_token(preflight.enablement_snapshot.disabled_reason_code);
    let command_id = entry.descriptor.command_id.clone();
    let target_refs = preview_target_refs(entry);
    let category_or_path = command_categories(entry).join(" · ");

    AlphaPaletteResultRow {
        row_id: format!("alpha-palette:command:{}", sanitize_id(&command_id)),
        row_kind,
        label: entry.title.clone(),
        category_or_path,
        origin_source_badge: origin_badge(entry),
        winning_keybinding: winning_keybinding(inputs.shortcuts_by_command_id, &command_id),
        dominant_side_effect_class: entry.dominant_side_effect_class.clone(),
        automation_labels: entry.automation_labels.clone(),
        automation_cues: automation_cues(entry),
        availability_class,
        disabled_reason_code,
        command_id: Some(command_id.clone()),
        target_refs: target_refs.clone(),
        ranking_reason_classes: command_ranking_reasons(entry, normalized_query),
        preview: AlphaPalettePreviewPane {
            preview_kind: "command_descriptor".to_string(),
            descriptor_command_id: Some(command_id),
            preflight_decision_class: Some(preflight_class),
            scope_summary: command_scope_summary(entry),
            side_effect_class: entry.dominant_side_effect_class.clone(),
            target_refs,
            rollback_or_checkpoint_posture: rollback_or_checkpoint_posture(entry),
        },
        action_footer: command_action_footer(entry, &preflight),
    }
}

fn symbol_result_row(
    symbol: &AlphaSymbolCandidate,
    normalized_query: &str,
) -> AlphaPaletteResultRow {
    AlphaPaletteResultRow {
        row_id: format!(
            "alpha-palette:symbol:{}",
            sanitize_id(&symbol.symbol_anchor_ref)
        ),
        row_kind: AlphaPaletteRowKind::Symbol,
        label: symbol.symbol_name.clone(),
        category_or_path: format!("{} · {}", symbol.symbol_kind, symbol.relative_path),
        origin_source_badge: symbol.origin_source_badge.clone(),
        winning_keybinding: "Unassigned".to_string(),
        dominant_side_effect_class: "opens_existing_target".to_string(),
        automation_labels: Vec::new(),
        automation_cues: Vec::new(),
        availability_class: "enabled".to_string(),
        disabled_reason_code: None,
        command_id: None,
        target_refs: vec![
            symbol.symbol_anchor_ref.clone(),
            symbol.relative_path.clone(),
        ],
        ranking_reason_classes: symbol_ranking_reasons(symbol, normalized_query),
        preview: AlphaPalettePreviewPane {
            preview_kind: "symbol_target".to_string(),
            descriptor_command_id: None,
            preflight_decision_class: None,
            scope_summary: symbol.freshness_state.clone(),
            side_effect_class: "opens_existing_target".to_string(),
            target_refs: vec![
                symbol.symbol_anchor_ref.clone(),
                symbol.relative_path.clone(),
            ],
            rollback_or_checkpoint_posture: "no_checkpoint_needed_navigation_only".to_string(),
        },
        action_footer: target_action_footer(true),
    }
}

fn file_result_row(file: &AlphaFileCandidate, normalized_query: &str) -> AlphaPaletteResultRow {
    let label = file_label(&file.relative_path);
    AlphaPaletteResultRow {
        row_id: format!(
            "alpha-palette:file:{}",
            sanitize_id(&file.path_identity_ref)
        ),
        row_kind: AlphaPaletteRowKind::File,
        label,
        category_or_path: file.relative_path.clone(),
        origin_source_badge: file.origin_source_badge.clone(),
        winning_keybinding: "Unassigned".to_string(),
        dominant_side_effect_class: "opens_existing_target".to_string(),
        automation_labels: Vec::new(),
        automation_cues: Vec::new(),
        availability_class: "enabled".to_string(),
        disabled_reason_code: None,
        command_id: None,
        target_refs: vec![file.path_identity_ref.clone(), file.relative_path.clone()],
        ranking_reason_classes: file_ranking_reasons(file, normalized_query),
        preview: AlphaPalettePreviewPane {
            preview_kind: "file_target".to_string(),
            descriptor_command_id: None,
            preflight_decision_class: None,
            scope_summary: file.freshness_state.clone(),
            side_effect_class: "opens_existing_target".to_string(),
            target_refs: vec![file.path_identity_ref.clone(), file.relative_path.clone()],
            rollback_or_checkpoint_posture: "no_checkpoint_needed_navigation_only".to_string(),
        },
        action_footer: target_action_footer(true),
    }
}

fn command_action_footer(
    entry: &CommandRegistryEntryRecord,
    preflight: &PreflightDecision,
) -> AlphaPaletteActionFooter {
    let disabled_reason = disabled_reason_token(preflight.enablement_snapshot.disabled_reason_code);
    let preflight_blocks_primary = matches!(
        preflight.decision_class,
        PreflightDecisionClass::BlockedByPolicy | PreflightDecisionClass::DisabledWithReason
    );
    let preview_required = matches!(
        preflight.decision_class,
        PreflightDecisionClass::PreviewRequired | PreflightDecisionClass::ApprovalRequired
    );
    let cli_skeleton = cli_skeleton_for(entry);
    let recipe_safe = labels_include(
        &entry.automation_labels,
        ControlledAutomationLabel::RecipeSafe,
    );
    let why_not_automatable = why_not_automatable_reason(
        &entry.automation_labels,
        &entry.descriptor.approval_posture_class,
    );

    AlphaPaletteActionFooter {
        default_action: AlphaPaletteFooterAction {
            action_class: if preview_required {
                "open_invocation_preview".to_string()
            } else {
                "primary_run_or_open".to_string()
            },
            enabled: !preflight_blocks_primary,
            unavailable_reason: preflight_blocks_primary.then(|| {
                disabled_reason
                    .clone()
                    .unwrap_or_else(|| "disabled_with_reason".to_string())
            }),
            copy_payload: None,
        },
        split_or_alternate_open: AlphaPaletteFooterAction {
            action_class: "split_or_alternate_open".to_string(),
            enabled: supports_alternate_open(entry) && !preflight_blocks_primary,
            unavailable_reason: (!supports_alternate_open(entry) || preflight_blocks_primary).then(
                || {
                    disabled_reason
                        .clone()
                        .unwrap_or_else(|| "no_alternate_open".to_string())
                },
            ),
            copy_payload: None,
        },
        copy_command_id: AlphaPaletteFooterAction {
            action_class: "copy_command_id".to_string(),
            enabled: true,
            unavailable_reason: None,
            copy_payload: Some(entry.descriptor.command_id.clone()),
        },
        copy_cli_headless_form: AlphaPaletteFooterAction {
            action_class: "copy_cli_headless_form".to_string(),
            enabled: cli_skeleton.is_some(),
            unavailable_reason: cli_skeleton
                .is_none()
                .then(|| "no_cli_headless_form".to_string()),
            copy_payload: cli_skeleton,
        },
        add_to_recipe: AlphaPaletteFooterAction {
            action_class: "add_to_recipe".to_string(),
            enabled: recipe_safe,
            unavailable_reason: (!recipe_safe).then(|| {
                why_not_automatable_reason(
                    &entry.automation_labels,
                    &entry.descriptor.approval_posture_class,
                )
                .unwrap_or_else(|| "not_recipe_safe".to_string())
            }),
            copy_payload: None,
        },
        inspect_why_not_automatable: AlphaPaletteFooterAction {
            action_class: "inspect_why_not_automatable".to_string(),
            enabled: why_not_automatable.is_some(),
            unavailable_reason: why_not_automatable
                .is_none()
                .then(|| "command_is_automation_safe".to_string()),
            copy_payload: None,
        },
        command_diagnostics_sheet_available: preflight_blocks_primary,
        invocation_preview_required: preview_required,
    }
}

fn target_action_footer(alternate_open_enabled: bool) -> AlphaPaletteActionFooter {
    AlphaPaletteActionFooter {
        default_action: AlphaPaletteFooterAction {
            action_class: "primary_open".to_string(),
            enabled: true,
            unavailable_reason: None,
            copy_payload: None,
        },
        split_or_alternate_open: AlphaPaletteFooterAction {
            action_class: "split_or_alternate_open".to_string(),
            enabled: alternate_open_enabled,
            unavailable_reason: (!alternate_open_enabled).then(|| "no_alternate_open".to_string()),
            copy_payload: None,
        },
        copy_command_id: AlphaPaletteFooterAction {
            action_class: "copy_command_id".to_string(),
            enabled: false,
            unavailable_reason: Some("not_a_command".to_string()),
            copy_payload: None,
        },
        copy_cli_headless_form: AlphaPaletteFooterAction {
            action_class: "copy_cli_headless_form".to_string(),
            enabled: false,
            unavailable_reason: Some("not_a_command".to_string()),
            copy_payload: None,
        },
        add_to_recipe: AlphaPaletteFooterAction {
            action_class: "add_to_recipe".to_string(),
            enabled: false,
            unavailable_reason: Some("not_a_command".to_string()),
            copy_payload: None,
        },
        inspect_why_not_automatable: AlphaPaletteFooterAction {
            action_class: "inspect_why_not_automatable".to_string(),
            enabled: false,
            unavailable_reason: Some("not_a_command".to_string()),
            copy_payload: None,
        },
        command_diagnostics_sheet_available: false,
        invocation_preview_required: false,
    }
}

fn preflight_for(
    entry: &CommandRegistryEntryRecord,
    runtime: CommandReviewRuntimeInputs<'_>,
) -> PreflightDecision {
    let context = CommandEnablementContext {
        client_scope: runtime.client_scope.to_string(),
        workspace_trust_state: runtime.workspace_trust_state.to_string(),
        execution_context_available: runtime.execution_context_available,
        provider_linked: runtime.provider_linked,
        credential_available: runtime.credential_available,
        policy_disabled: runtime.policy_disabled,
        policy_blocked_in_context: runtime.policy_blocked_in_context,
        labs_enabled: runtime.labs_enabled,
        argument_provenance_map: argument_provenance_map_for(entry),
    };
    entry.preflight(&context)
}

fn provider_summaries(rows: &[AlphaPaletteResultRow]) -> Vec<AlphaPaletteProviderSummary> {
    [
        (AlphaPaletteRowKind::RecentAction, "recent_history"),
        (AlphaPaletteRowKind::Command, "command_registry"),
        (AlphaPaletteRowKind::Symbol, "symbol_index"),
        (AlphaPaletteRowKind::File, "file_index"),
    ]
    .into_iter()
    .map(|(kind, provider_class)| AlphaPaletteProviderSummary {
        provider_class: provider_class.to_string(),
        state_class: "ready".to_string(),
        visible_result_count: rows.iter().filter(|row| row.row_kind == kind).count(),
    })
    .collect()
}

fn command_is_palette_visible(
    entry: &CommandRegistryEntryRecord,
    client_scope: &str,
    labs_enabled: bool,
) -> bool {
    entry
        .descriptor
        .client_scopes
        .iter()
        .any(|scope| scope == client_scope)
        && entry.descriptor.palette_visibility != "hidden_palette_callable_only"
        && (entry.descriptor.palette_visibility != "developer_only" || labs_enabled)
}

fn matches_recent_action(
    recent: &AlphaRecentActionCandidate,
    normalized_query: &str,
    registry: &CommandRegistry,
) -> bool {
    if normalized_query.is_empty() {
        return true;
    }
    contains_case_insensitive(&recent.label, normalized_query)
        || contains_case_insensitive(&recent.category_or_path, normalized_query)
        || recent.command_id.as_deref().is_some_and(|command_id| {
            registry
                .get(command_id)
                .is_some_and(|entry| matches_command(entry, normalized_query))
        })
}

fn matches_command(entry: &CommandRegistryEntryRecord, normalized_query: &str) -> bool {
    if normalized_query.is_empty() {
        return true;
    }
    contains_case_insensitive(&entry.title, normalized_query)
        || contains_case_insensitive(&entry.summary, normalized_query)
        || contains_case_insensitive(entry.command_id(), normalized_query)
        || command_categories(entry)
            .iter()
            .any(|category| contains_case_insensitive(category, normalized_query))
}

fn matches_symbol(symbol: &AlphaSymbolCandidate, normalized_query: &str) -> bool {
    normalized_query.is_empty()
        || contains_case_insensitive(&symbol.symbol_name, normalized_query)
        || contains_case_insensitive(&symbol.relative_path, normalized_query)
        || contains_case_insensitive(&symbol.symbol_kind, normalized_query)
}

fn matches_file(file: &AlphaFileCandidate, normalized_query: &str) -> bool {
    normalized_query.is_empty() || contains_case_insensitive(&file.relative_path, normalized_query)
}

fn command_ranking_reasons(
    entry: &CommandRegistryEntryRecord,
    normalized_query: &str,
) -> Vec<String> {
    let mut reasons = Vec::new();
    if normalized_query.is_empty() {
        reasons.push("default_command_order".to_string());
    } else {
        if contains_case_insensitive(entry.command_id(), normalized_query) {
            reasons.push("stable_command_id".to_string());
        }
        if contains_case_insensitive(&entry.title, normalized_query) {
            reasons.push("title_match".to_string());
        }
        if contains_case_insensitive(&entry.summary, normalized_query) {
            reasons.push("summary_match".to_string());
        }
    }
    if reasons.is_empty() {
        reasons.push("discoverability_record".to_string());
    }
    reasons
}

fn symbol_ranking_reasons(symbol: &AlphaSymbolCandidate, normalized_query: &str) -> Vec<String> {
    let mut reasons = vec!["symbol_provider".to_string()];
    if contains_case_insensitive(&symbol.symbol_name, normalized_query) {
        reasons.push("symbol_name_match".to_string());
    }
    if symbol.origin_source_badge == "structural_fallback" {
        reasons.push("structural_fallback".to_string());
    }
    reasons
}

fn file_ranking_reasons(file: &AlphaFileCandidate, normalized_query: &str) -> Vec<String> {
    let mut reasons = vec!["file_provider".to_string()];
    let label = file_label(&file.relative_path);
    if contains_case_insensitive(&label, normalized_query) {
        reasons.push("filename_match".to_string());
    } else if contains_case_insensitive(&file.relative_path, normalized_query) {
        reasons.push("path_match".to_string());
    }
    reasons
}

fn availability_for_preflight(decision: PreflightDecisionClass) -> &'static str {
    match decision {
        PreflightDecisionClass::Allowed => "enabled",
        PreflightDecisionClass::BlockedByPolicy => "blocked_by_policy",
        PreflightDecisionClass::DisabledWithReason => "disabled_with_reason",
        PreflightDecisionClass::PreviewRequired => "preview_required",
        PreflightDecisionClass::ApprovalRequired => "approval_required",
    }
}

fn preflight_decision_token(decision: PreflightDecisionClass) -> &'static str {
    match decision {
        PreflightDecisionClass::Allowed => "allowed",
        PreflightDecisionClass::BlockedByPolicy => "blocked_by_policy",
        PreflightDecisionClass::DisabledWithReason => "disabled_with_reason",
        PreflightDecisionClass::PreviewRequired => "preview_required",
        PreflightDecisionClass::ApprovalRequired => "approval_required",
    }
}

fn disabled_reason_token(reason: Option<DisabledReasonCode>) -> Option<String> {
    reason.map(|code| code.as_str().to_string())
}

fn command_categories(entry: &CommandRegistryEntryRecord) -> Vec<String> {
    entry
        .discoverability_record
        .get("category_refs")
        .and_then(|v| v.as_array())
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str())
                .map(|value| value.to_string())
                .collect()
        })
        .filter(|values: &Vec<String>| !values.is_empty())
        .unwrap_or_else(|| vec![entry.namespace_class.clone()])
}

fn origin_badge(entry: &CommandRegistryEntryRecord) -> String {
    entry
        .origin_badge
        .get("badge_class")
        .and_then(|v| v.as_str())
        .unwrap_or(&entry.namespace_class)
        .to_string()
}

fn winning_keybinding(
    shortcuts_by_command_id: &HashMap<String, Vec<String>>,
    command_id: &str,
) -> String {
    shortcuts_by_command_id
        .get(command_id)
        .and_then(|shortcuts| shortcuts.first())
        .cloned()
        .unwrap_or_else(|| "Unassigned".to_string())
}

fn command_scope_summary(entry: &CommandRegistryEntryRecord) -> String {
    entry
        .descriptor
        .policy_context
        .execution_context_id
        .clone()
        .unwrap_or_else(|| "execution_context:unbound".to_string())
}

fn preview_target_refs(entry: &CommandRegistryEntryRecord) -> Vec<String> {
    let mut refs = vec![entry.descriptor.command_revision_ref.clone()];
    if let Some(exec) = entry.descriptor.policy_context.execution_context_id.clone() {
        refs.push(exec);
    }
    if entry.descriptor.preview_class != "no_preview_required" {
        refs.push(format!("preview-class:{}", entry.descriptor.preview_class));
    }
    if entry.descriptor.approval_posture_class != "no_approval_required" {
        refs.push(format!(
            "approval-posture:{}",
            entry.descriptor.approval_posture_class
        ));
    }
    refs
}

fn rollback_or_checkpoint_posture(entry: &CommandRegistryEntryRecord) -> String {
    if entry
        .descriptor
        .result_contract
        .evidence_ref_class_required
        .iter()
        .any(|class| class.contains("rollback"))
    {
        return "rollback_handle_required".to_string();
    }
    if entry
        .descriptor
        .typed_arguments
        .iter()
        .any(|arg| arg.argument_name.contains("checkpoint"))
    {
        return "checkpoint_argument_declared".to_string();
    }
    if entry.descriptor.preview_class != "no_preview_required" {
        return "preview_record_required".to_string();
    }
    "no_checkpoint_required".to_string()
}

fn supports_alternate_open(entry: &CommandRegistryEntryRecord) -> bool {
    matches!(
        entry.descriptor.canonical_verb.as_str(),
        "workspace.open_folder" | "workspace.open_workspace" | "quick_open.toggle"
    )
}

fn automation_cues(entry: &CommandRegistryEntryRecord) -> Vec<String> {
    automation_display_labels(&entry.automation_labels)
}

fn normalize_query(query: &str) -> String {
    query.trim().to_ascii_lowercase()
}

fn contains_case_insensitive(haystack: &str, needle: &str) -> bool {
    needle.is_empty() || haystack.to_ascii_lowercase().contains(needle)
}

fn score_text(label: &str, normalized_query: &str) -> i32 {
    if normalized_query.is_empty() {
        return 100;
    }
    let lower = label.to_ascii_lowercase();
    if lower == normalized_query {
        0
    } else if lower.starts_with(normalized_query) {
        10
    } else if lower.contains(normalized_query) {
        20
    } else {
        90
    }
}

fn file_label(relative_path: &str) -> String {
    relative_path
        .rsplit_once('/')
        .map(|(_, name)| name.to_string())
        .unwrap_or_else(|| relative_path.to_string())
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
