//! Alpha keybinding truth projection for shell, settings, and help surfaces.
//!
//! This module does not define a second shortcut language. It joins the
//! `aureline-input` resolver output, built-in preset rows, settings inspection
//! records, and the alpha command registry so product surfaces can inspect the
//! active binding winner, preset fidelity, conflicts, and command parity from
//! one packet family.

use std::collections::{BTreeMap, BTreeSet};

use aureline_commands::alpha::{alpha_command_registry, AlphaCommandClaimRecord};
use aureline_commands::registry::{seeded_registry, CommandRegistry};
use aureline_input::keybindings::{
    InspectionScope, KeySequence, KeybindingConflictReviewPacketRecord, PlatformClass,
    ResolutionReasonCode, ResolverLayerClass, SequenceResolutionState, SurfaceSupportClass,
    WinningResolutionKind,
};
use aureline_input::presets::{
    preset_binding_rows, preset_conflicts, resolver_with_preset, KeymapPresetId, PresetBindingRow,
};
use aureline_settings::keybindings::{
    KeybindingNarrowingRecord, KeybindingSettingInspectionRecord, KeybindingSettingSourceLayer,
    KeybindingSettingSourceRecord, KeybindingSettingsConflictRecord,
};
use serde::{Deserialize, Serialize};

/// Schema version for the shell alpha keybinding projection.
pub const ALPHA_KEYBINDING_REPORT_SCHEMA_VERSION: u32 = 1;

const RETAINED_TRANSLATION_REPORT_REF: &str =
    "artifacts/migration/keymap_translation_report_sample.json";
const CONFLICT_INSPECTOR_REF: &str = "surface:help.keybinding_inspector.conflicts";
const SETTINGS_SCHEMA_REF: &str = "schemas/settings/keybindings.schema.json";
const PARITY_REPORT_REF: &str = "artifacts/commands/alpha_keybinding_parity_report.json";

/// Controlled shortcut translation outcome used by preset and migration rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeybindingBridgeOutcomeClass {
    /// Source command, gesture, scope, mode, and command posture are preserved.
    Exact,
    /// Command identity is preserved, but gesture or sequence shape changed.
    Translated,
    /// Command identity is preserved for a narrower alpha scope.
    Partial,
    /// A shim or bridge dependency is required for the behavior to feel familiar.
    Shimmed,
    /// No bounded alpha equivalent is currently claimed.
    Unsupported,
}

impl KeybindingBridgeOutcomeClass {
    /// Returns the stable token used by fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Translated => "translated",
            Self::Partial => "partial",
            Self::Shimmed => "shimmed",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Translation row for one command under one built-in preset profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaPresetTranslationRow {
    /// Preset ref such as `preset:keymap:vs_code`.
    pub preset_ref: String,
    /// Canonical command id.
    pub command_id: String,
    /// Human-facing command title from the registry.
    pub command_title: String,
    /// Literal resolved shortcut or `unassigned`.
    pub literal_sequence: String,
    /// Controlled fidelity class.
    pub bridge_outcome_class: KeybindingBridgeOutcomeClass,
    /// Resolver layer that supplies the row.
    pub resolver_layer: String,
    /// Stable source provenance ref surfaced in help/settings.
    pub source_provenance_ref: Option<String>,
    /// Resolver packet ref for winning-source inspection.
    pub resolver_packet_ref: Option<String>,
    /// Conflict review ref when this mapping is contested.
    pub conflict_review_ref: Option<String>,
    /// Behavior changes that prevent an exact claim.
    pub behavior_change_axes: Vec<String>,
    /// Short export-safe note.
    pub note: String,
}

/// Coverage summary for one built-in preset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaPresetProfileReport {
    /// Preset ref such as `preset:keymap:vim`.
    pub preset_ref: String,
    /// User-facing preset name.
    pub display_name: String,
    /// Number of claimed command ids with a binding in the preset.
    pub bound_claimed_command_count: usize,
    /// Number of alpha command ids claimed by the command registry.
    pub expected_claimed_command_count: usize,
    /// Number of non-exact rows disclosed by this profile.
    pub non_exact_row_count: usize,
    /// Translation rows for the claimed alpha command set.
    pub translations: Vec<AlphaPresetTranslationRow>,
}

/// Winning-binding row for the active preset and platform.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaWinningBindingInspection {
    /// Canonical command id.
    pub command_id: String,
    /// Human-facing command title.
    pub command_title: String,
    /// Literal active binding or `unassigned`.
    pub literal_sequence: String,
    /// Resolver sequence state token.
    pub sequence_state: String,
    /// Winning resolver layer token.
    pub winning_layer: Option<String>,
    /// Stable source ref for the active winner.
    pub winning_source_ref: Option<String>,
    /// Resolver reason code.
    pub reason_code: String,
    /// Resolver packet ref backing the row.
    pub resolver_packet_ref: Option<String>,
    /// Conflict review ref when the row is contested.
    pub conflict_review_ref: Option<String>,
    /// Authority class inherited from the command registry publication.
    pub authority_class: String,
    /// Preview class inherited from the command descriptor.
    pub preview_class: String,
    /// Approval posture inherited from the command descriptor.
    pub approval_posture_class: String,
    /// Platform or policy narrowing, if present.
    pub narrowing: Option<KeybindingNarrowingRecord>,
}

/// Conflict inspector row projected from resolver conflict packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaKeybindingConflictInspection {
    /// Preset ref that produced the conflict.
    pub preset_ref: String,
    /// Resolver conflict-review packet ref.
    pub conflict_review_id: String,
    /// Literal sequence under conflict.
    pub literal_sequence: String,
    /// Winning command id when a winner exists.
    pub winning_command_id: Option<String>,
    /// Losing command ids surfaced for review.
    pub losing_command_ids: Vec<String>,
    /// Retained migration or shortcut delta report.
    pub linked_migration_report_ref: String,
    /// Product surface that can reopen the conflict.
    pub conflict_inspector_ref: String,
}

/// Cross-surface parity row for a command's keybinding projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaKeybindingParityRow {
    /// Canonical command id.
    pub command_id: String,
    /// Palette surface projects this command id.
    pub palette_projected_command_id: Option<String>,
    /// Menu or button surface projects this command id.
    pub menu_projected_command_id: Option<String>,
    /// Keybinding-help surface projects this command id.
    pub keybinding_projected_command_id: Option<String>,
    /// True when palette, menu, and keybinding rows agree on the command id.
    pub stable_command_id_preserved: bool,
    /// Authority class projected by the command registry.
    pub authority_class: String,
    /// Preview class projected by the command registry.
    pub preview_class: String,
    /// Approval posture projected by the command registry.
    pub approval_posture_class: String,
    /// Result contract class projected by the command registry.
    pub result_contract_class: String,
    /// Parity status token.
    pub parity_status: String,
}

/// Top-level alpha keybinding truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaKeybindingTruthReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version for this projection.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Active platform used for shortcut projection.
    pub platform_class: String,
    /// Active preset used for winning-binding inspection.
    pub active_preset_ref: String,
    /// Source artifacts and schemas consumed by this packet.
    pub source_refs: Vec<String>,
    /// Preset coverage and fidelity rows for the claimed alpha command set.
    pub preset_profiles: Vec<AlphaPresetProfileReport>,
    /// Winning-binding truth for the active preset.
    pub winning_bindings: Vec<AlphaWinningBindingInspection>,
    /// Settings/detail rows that can be rendered without rereading raw logs.
    pub settings_inspection_rows: Vec<KeybindingSettingInspectionRecord>,
    /// Conflict rows reopenable from help/settings/migration surfaces.
    pub conflict_inspections: Vec<AlphaKeybindingConflictInspection>,
    /// Command parity rows for palette, menus, and keybindings.
    pub parity_rows: Vec<AlphaKeybindingParityRow>,
    /// Summary fields used by tests and support exports.
    pub summary: AlphaKeybindingTruthSummary,
}

/// Compact summary for the top-level keybinding truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaKeybindingTruthSummary {
    /// Number of alpha command claims consumed.
    pub claimed_command_count: usize,
    /// Number of preset profiles checked.
    pub preset_profile_count: usize,
    /// Number of commands with active winning bindings.
    pub winning_binding_count: usize,
    /// Number of conflict rows surfaced for review.
    pub conflict_count: usize,
    /// Number of parity rows with blocking drift.
    pub parity_blocking_findings: usize,
    /// Overall status token.
    pub status: String,
}

/// Materializes the alpha keybinding truth packet for the active preset.
pub fn materialize_alpha_keybinding_truth(
    registry: &CommandRegistry,
    active_preset: KeymapPresetId,
    platform: PlatformClass,
) -> AlphaKeybindingTruthReport {
    let claims = alpha_command_registry().claimed_commands.clone();
    let active_resolver = resolver_with_preset(active_preset, platform)
        .unwrap_or_else(|_| aureline_input::keybindings::seeded_keybinding_resolver().clone());
    let active_rows = binding_rows_by_command(preset_binding_rows(active_preset, platform));

    let preset_profiles = KeymapPresetId::all()
        .into_iter()
        .map(|preset| materialize_preset_profile(registry, preset, platform, &claims))
        .collect::<Vec<_>>();
    let winning_bindings = claims
        .iter()
        .map(|claim| {
            materialize_winning_binding(registry, &active_resolver, &active_rows, platform, claim)
        })
        .collect::<Vec<_>>();
    let conflict_inspections = materialize_conflict_inspections(platform);
    let settings_inspection_rows = winning_bindings
        .iter()
        .map(|row| settings_row_from_winner(row, &conflict_inspections))
        .collect::<Vec<_>>();
    let parity_rows = claims
        .iter()
        .map(materialize_parity_row)
        .collect::<Vec<_>>();

    let parity_blocking_findings = parity_rows
        .iter()
        .filter(|row| row.parity_status != "pass")
        .count();
    let winning_binding_count = winning_bindings
        .iter()
        .filter(|row| row.literal_sequence != "unassigned")
        .count();
    let conflict_count = conflict_inspections.len();
    let status = if parity_blocking_findings == 0
        && winning_binding_count == claims.len()
        && preset_profiles
            .iter()
            .all(|profile| profile.bound_claimed_command_count == claims.len())
    {
        "pass"
    } else {
        "needs_review"
    };

    AlphaKeybindingTruthReport {
        record_kind: "alpha_keybinding_truth_report".to_string(),
        schema_version: ALPHA_KEYBINDING_REPORT_SCHEMA_VERSION,
        report_id: "keybinding-truth:alpha:launch-wedge".to_string(),
        platform_class: platform_token(platform).to_string(),
        active_preset_ref: active_preset.preset_ref().to_string(),
        source_refs: vec![
            "crates/aureline-input/src/keybindings/mod.rs".to_string(),
            "crates/aureline-input/src/presets/mod.rs".to_string(),
            "crates/aureline-settings/src/keybindings/mod.rs".to_string(),
            "artifacts/commands/alpha_command_registry.yaml".to_string(),
            SETTINGS_SCHEMA_REF.to_string(),
            PARITY_REPORT_REF.to_string(),
            RETAINED_TRANSLATION_REPORT_REF.to_string(),
        ],
        preset_profiles,
        winning_bindings,
        settings_inspection_rows,
        conflict_inspections,
        parity_rows,
        summary: AlphaKeybindingTruthSummary {
            claimed_command_count: claims.len(),
            preset_profile_count: KeymapPresetId::all().len(),
            winning_binding_count,
            conflict_count,
            parity_blocking_findings,
            status: status.to_string(),
        },
    }
}

/// Builds compact lines for the keybinding help inspector.
pub fn build_alpha_keybinding_truth_lines(
    registry: &CommandRegistry,
    active_preset: KeymapPresetId,
    platform: PlatformClass,
) -> Vec<String> {
    let report = materialize_alpha_keybinding_truth(registry, active_preset, platform);
    let mut lines = vec![
        "".to_string(),
        format!(
            "Alpha keybinding truth - preset: {} - status: {}",
            report.active_preset_ref, report.summary.status
        ),
        format!(
            "Parity: {} rows, {} blocking findings",
            report.parity_rows.len(),
            report.summary.parity_blocking_findings
        ),
    ];

    for winner in report.winning_bindings.iter().take(6) {
        lines.push(format!(
            "- {} => {}  -  source={}  -  reason={}",
            winner.command_id,
            winner.literal_sequence,
            winner
                .winning_source_ref
                .as_deref()
                .unwrap_or("source:not_bound"),
            winner.reason_code
        ));
    }

    if report.conflict_inspections.is_empty() {
        lines.push("Conflicts: none".to_string());
    } else {
        lines.push("Conflicts linked to migration report".to_string());
        for conflict in report.conflict_inspections.iter().take(4) {
            lines.push(format!(
                "- {}  -  {}  -  losing={}  -  report={}",
                conflict.preset_ref,
                conflict.literal_sequence,
                conflict.losing_command_ids.join(","),
                conflict.linked_migration_report_ref
            ));
        }
    }

    lines
}

fn materialize_preset_profile(
    registry: &CommandRegistry,
    preset: KeymapPresetId,
    platform: PlatformClass,
    claims: &[AlphaCommandClaimRecord],
) -> AlphaPresetProfileReport {
    let rows_by_command = binding_rows_by_command(preset_binding_rows(preset, platform));
    let resolver = resolver_with_preset(preset, platform)
        .unwrap_or_else(|_| aureline_input::keybindings::seeded_keybinding_resolver().clone());
    let translations = claims
        .iter()
        .map(|claim| {
            materialize_translation_row(
                registry,
                &resolver,
                &rows_by_command,
                preset,
                platform,
                claim,
            )
        })
        .collect::<Vec<_>>();
    let bound_claimed_command_count = translations
        .iter()
        .filter(|row| row.literal_sequence != "unassigned")
        .count();
    let non_exact_row_count = translations
        .iter()
        .filter(|row| row.bridge_outcome_class != KeybindingBridgeOutcomeClass::Exact)
        .count();

    AlphaPresetProfileReport {
        preset_ref: preset.preset_ref().to_string(),
        display_name: preset.display_name().to_string(),
        bound_claimed_command_count,
        expected_claimed_command_count: claims.len(),
        non_exact_row_count,
        translations,
    }
}

fn materialize_translation_row(
    registry: &CommandRegistry,
    resolver: &aureline_input::keybindings::KeybindingResolver,
    rows_by_command: &BTreeMap<String, Vec<PresetBindingRow>>,
    preset: KeymapPresetId,
    platform: PlatformClass,
    claim: &AlphaCommandClaimRecord,
) -> AlphaPresetTranslationRow {
    let title = registry
        .get(&claim.command_id)
        .map(|entry| entry.title.clone())
        .unwrap_or_else(|| "<unknown command>".to_string());
    let Some(binding) = rows_by_command
        .get(&claim.command_id)
        .and_then(|rows| rows.first())
    else {
        return AlphaPresetTranslationRow {
            preset_ref: preset.preset_ref().to_string(),
            command_id: claim.command_id.clone(),
            command_title: title,
            literal_sequence: "unassigned".to_string(),
            bridge_outcome_class: KeybindingBridgeOutcomeClass::Unsupported,
            resolver_layer: "not_bound".to_string(),
            source_provenance_ref: None,
            resolver_packet_ref: None,
            conflict_review_ref: None,
            behavior_change_axes: vec!["no_alpha_binding_claimed".to_string()],
            note: "No bounded alpha shortcut is assigned in this preset.".to_string(),
        };
    };

    let sequence = KeySequence::parse_literal_sequence(&binding.literal_sequence).ok();
    let packet = sequence
        .as_ref()
        .map(|sequence| resolver.resolve(sequence, &default_inspection_scope(platform)));
    let conflict_review_ref = packet.as_ref().and_then(|packet| {
        packet.conflict_review_ref.clone().or_else(|| {
            if packet.winning_resolution.reason_code
                == aureline_input::keybindings::ResolutionReasonCode::SameLayerCollisionRequiresReview
            {
                Some(format!("keybinding-conflict-review:{}", packet.resolution_id))
            } else {
                None
            }
        })
    });
    let outcome = classify_bridge_outcome(preset, claim, &binding.literal_sequence);

    AlphaPresetTranslationRow {
        preset_ref: preset.preset_ref().to_string(),
        command_id: claim.command_id.clone(),
        command_title: title,
        literal_sequence: binding.literal_sequence.clone(),
        bridge_outcome_class: outcome,
        resolver_layer: "user_profile_binding".to_string(),
        source_provenance_ref: Some(binding.source_provenance_ref.clone()),
        resolver_packet_ref: packet.map(|packet| packet.resolution_id),
        conflict_review_ref,
        behavior_change_axes: behavior_change_axes(
            outcome,
            preset,
            claim,
            &binding.literal_sequence,
        ),
        note: bridge_note(outcome, preset, claim),
    }
}

fn materialize_winning_binding(
    registry: &CommandRegistry,
    resolver: &aureline_input::keybindings::KeybindingResolver,
    rows_by_command: &BTreeMap<String, Vec<PresetBindingRow>>,
    platform: PlatformClass,
    claim: &AlphaCommandClaimRecord,
) -> AlphaWinningBindingInspection {
    let title = registry
        .get(&claim.command_id)
        .map(|entry| entry.title.clone())
        .unwrap_or_else(|| "<unknown command>".to_string());
    let Some(binding) = rows_by_command
        .get(&claim.command_id)
        .and_then(|rows| rows.first())
    else {
        return AlphaWinningBindingInspection {
            command_id: claim.command_id.clone(),
            command_title: title,
            literal_sequence: "unassigned".to_string(),
            sequence_state: "unbound".to_string(),
            winning_layer: None,
            winning_source_ref: None,
            reason_code: "no_candidate_bound".to_string(),
            resolver_packet_ref: None,
            conflict_review_ref: None,
            authority_class: claim.authority_class.clone(),
            preview_class: claim.preview_class.clone(),
            approval_posture_class: claim.approval_posture_class.clone(),
            narrowing: None,
        };
    };

    let packet = KeySequence::parse_literal_sequence(&binding.literal_sequence)
        .ok()
        .map(|sequence| resolver.resolve(&sequence, &default_inspection_scope(platform)));
    let (
        sequence_state,
        winning_layer,
        winning_source_ref,
        reason_code,
        resolver_packet_ref,
        conflict_review_ref,
        narrowing,
    ) = match packet {
        Some(packet) => {
            let winner_candidate = packet.winning_resolution.command_candidate.as_ref();
            let narrowing = narrowing_from_packet(&packet);
            (
                sequence_state_token(packet.sequence_state).to_string(),
                packet
                    .winning_resolution
                    .resolver_layer
                    .map(|layer| resolver_layer_token(layer).to_string()),
                winner_candidate
                    .and_then(|candidate| candidate.source_provenance_ref.clone())
                    .or_else(|| Some(binding.source_provenance_ref.clone())),
                resolution_reason_token(packet.winning_resolution.reason_code).to_string(),
                Some(packet.resolution_id),
                packet.conflict_review_ref,
                narrowing,
            )
        }
        None => (
            "unbound".to_string(),
            None,
            Some(binding.source_provenance_ref.clone()),
            "invalid_binding_sequence".to_string(),
            None,
            None,
            None,
        ),
    };

    AlphaWinningBindingInspection {
        command_id: claim.command_id.clone(),
        command_title: title,
        literal_sequence: binding.literal_sequence.clone(),
        sequence_state,
        winning_layer,
        winning_source_ref,
        reason_code,
        resolver_packet_ref,
        conflict_review_ref,
        authority_class: claim.authority_class.clone(),
        preview_class: claim.preview_class.clone(),
        approval_posture_class: claim.approval_posture_class.clone(),
        narrowing,
    }
}

fn settings_row_from_winner(
    winner: &AlphaWinningBindingInspection,
    conflicts: &[AlphaKeybindingConflictInspection],
) -> KeybindingSettingInspectionRecord {
    let source_layer = winner
        .winning_layer
        .as_deref()
        .map(settings_layer_from_token)
        .unwrap_or(KeybindingSettingSourceLayer::NotBound);
    let mut row = KeybindingSettingInspectionRecord::new(
        format!("keybinding-setting:{}", winner.command_id.replace(':', "_")),
        winner.command_id.clone(),
        winner.command_title.clone(),
        winner.literal_sequence.clone(),
        vec![KeybindingSettingSourceRecord {
            source_layer,
            source_ref: winner
                .winning_source_ref
                .clone()
                .unwrap_or_else(|| "source:not_bound".to_string()),
            source_label: winner
                .winning_source_ref
                .clone()
                .unwrap_or_else(|| "No active binding".to_string()),
            winner: winner.literal_sequence != "unassigned",
            outcome_reason_code: winner.reason_code.clone(),
        }],
        winner.preview_class.clone(),
        winner.approval_posture_class.clone(),
        winner.authority_class.clone(),
    );
    row.resolver_packet_ref = winner.resolver_packet_ref.clone();
    row.narrowing = winner.narrowing.clone();
    row.retained_report_ref = Some(RETAINED_TRANSLATION_REPORT_REF.to_string());
    row.conflict = conflicts
        .iter()
        .find(|conflict| {
            conflict.winning_command_id.as_deref() == Some(winner.command_id.as_str())
                || conflict
                    .losing_command_ids
                    .iter()
                    .any(|command_id| command_id == &winner.command_id)
        })
        .map(|conflict| KeybindingSettingsConflictRecord {
            conflict_review_ref: conflict.conflict_review_id.clone(),
            literal_sequence: conflict.literal_sequence.clone(),
            winning_command_id: conflict.winning_command_id.clone(),
            losing_command_ids: conflict.losing_command_ids.clone(),
            migration_report_ref: Some(conflict.linked_migration_report_ref.clone()),
        });
    row
}

fn materialize_conflict_inspections(
    platform: PlatformClass,
) -> Vec<AlphaKeybindingConflictInspection> {
    let mut conflicts = Vec::new();
    for preset in KeymapPresetId::all() {
        if let Ok(packets) = preset_conflicts(preset, platform) {
            for packet in packets {
                conflicts.push(conflict_inspection_from_packet(preset, packet));
            }
        }
    }
    conflicts.sort_by(|a, b| {
        a.preset_ref
            .cmp(&b.preset_ref)
            .then(a.literal_sequence.cmp(&b.literal_sequence))
    });
    conflicts
}

fn conflict_inspection_from_packet(
    preset: KeymapPresetId,
    packet: KeybindingConflictReviewPacketRecord,
) -> AlphaKeybindingConflictInspection {
    AlphaKeybindingConflictInspection {
        preset_ref: preset.preset_ref().to_string(),
        conflict_review_id: packet.conflict_review_id,
        literal_sequence: packet.inspected_sequence.literal_sequence,
        winning_command_id: packet
            .winning_resolution
            .command_candidate
            .map(|candidate| candidate.command.command_id),
        losing_command_ids: packet
            .losing_candidates
            .into_iter()
            .map(|losing| losing.candidate.command.command_id)
            .collect(),
        linked_migration_report_ref: RETAINED_TRANSLATION_REPORT_REF.to_string(),
        conflict_inspector_ref: CONFLICT_INSPECTOR_REF.to_string(),
    }
}

fn materialize_parity_row(claim: &AlphaCommandClaimRecord) -> AlphaKeybindingParityRow {
    let surface_command_id = |surface_family: &str| {
        claim
            .surface_parity
            .iter()
            .find(|row| row.surface_family == surface_family)
            .map(|row| row.projected_command_id.clone())
    };
    let palette_projected_command_id =
        surface_command_id("command_palette").or_else(|| Some(claim.command_id.clone()));
    let menu_projected_command_id =
        surface_command_id("menu_or_button").or_else(|| Some(claim.command_id.clone()));
    let keybinding_projected_command_id =
        surface_command_id("keybinding_help").or_else(|| Some(claim.command_id.clone()));
    let observed = [
        palette_projected_command_id.as_deref(),
        menu_projected_command_id.as_deref(),
        keybinding_projected_command_id.as_deref(),
    ];
    let stable_command_id_preserved = observed
        .iter()
        .flatten()
        .all(|projected| *projected == claim.command_id);
    let parity_status = if stable_command_id_preserved {
        "pass"
    } else {
        "blocking_drift"
    };

    AlphaKeybindingParityRow {
        command_id: claim.command_id.clone(),
        palette_projected_command_id,
        menu_projected_command_id,
        keybinding_projected_command_id,
        stable_command_id_preserved,
        authority_class: claim.authority_class.clone(),
        preview_class: claim.preview_class.clone(),
        approval_posture_class: claim.approval_posture_class.clone(),
        result_contract_class: claim.result_contract_class.clone(),
        parity_status: parity_status.to_string(),
    }
}

fn binding_rows_by_command(
    rows: Result<Vec<PresetBindingRow>, aureline_input::presets::PresetSeedError>,
) -> BTreeMap<String, Vec<PresetBindingRow>> {
    let mut by_command: BTreeMap<String, Vec<PresetBindingRow>> = BTreeMap::new();
    if let Ok(rows) = rows {
        for row in rows {
            by_command
                .entry(row.command_id.clone())
                .or_default()
                .push(row);
        }
    }
    for rows in by_command.values_mut() {
        rows.sort_by(|a, b| a.literal_sequence.cmp(&b.literal_sequence));
    }
    by_command
}

fn classify_bridge_outcome(
    preset: KeymapPresetId,
    claim: &AlphaCommandClaimRecord,
    literal_sequence: &str,
) -> KeybindingBridgeOutcomeClass {
    match (preset, claim.command_id.as_str()) {
        (KeymapPresetId::VsCode, "cmd:command_palette.open") => KeybindingBridgeOutcomeClass::Exact,
        (KeymapPresetId::VsCode, "cmd:workspace.open_folder")
            if literal_sequence == "Cmd+O" || literal_sequence == "Ctrl+K Ctrl+O" =>
        {
            KeybindingBridgeOutcomeClass::Exact
        }
        (KeymapPresetId::VsCode, "cmd:workspace.open_folder") => {
            KeybindingBridgeOutcomeClass::Translated
        }
        (KeymapPresetId::IntelliJ, "cmd:command_palette.open") => {
            KeybindingBridgeOutcomeClass::Translated
        }
        (KeymapPresetId::Vim, "cmd:command_palette.open") => KeybindingBridgeOutcomeClass::Partial,
        (KeymapPresetId::Emacs, "cmd:command_palette.open") => {
            KeybindingBridgeOutcomeClass::Translated
        }
        (_, "cmd:docs.open_in_browser") => KeybindingBridgeOutcomeClass::Shimmed,
        (_, "cmd:workspace.clone_repository")
        | (_, "cmd:workspace.import_profile")
        | (_, "cmd:workspace.restore_from_checkpoint") => KeybindingBridgeOutcomeClass::Partial,
        _ => KeybindingBridgeOutcomeClass::Translated,
    }
}

fn behavior_change_axes(
    outcome: KeybindingBridgeOutcomeClass,
    preset: KeymapPresetId,
    claim: &AlphaCommandClaimRecord,
    literal_sequence: &str,
) -> Vec<String> {
    match outcome {
        KeybindingBridgeOutcomeClass::Exact => Vec::new(),
        KeybindingBridgeOutcomeClass::Translated => vec!["gesture_changed".to_string()],
        KeybindingBridgeOutcomeClass::Partial => {
            let mut axes = vec!["surface_scope_narrowed".to_string()];
            if preset == KeymapPresetId::Vim {
                axes.push("mode_changed".to_string());
            }
            axes
        }
        KeybindingBridgeOutcomeClass::Shimmed => {
            vec![
                "bridge_dependency_required".to_string(),
                "surface_scope_narrowed".to_string(),
            ]
        }
        KeybindingBridgeOutcomeClass::Unsupported => vec![
            "command_identity_changed".to_string(),
            format!("unsupported_target:{}", claim.command_id),
            format!("unassigned_sequence:{literal_sequence}"),
        ],
    }
}

fn bridge_note(
    outcome: KeybindingBridgeOutcomeClass,
    preset: KeymapPresetId,
    claim: &AlphaCommandClaimRecord,
) -> String {
    match outcome {
        KeybindingBridgeOutcomeClass::Exact => {
            "Preset preserves command identity and shortcut posture.".to_string()
        }
        KeybindingBridgeOutcomeClass::Translated => {
            "Preset preserves command identity, but the familiar gesture differs.".to_string()
        }
        KeybindingBridgeOutcomeClass::Partial => format!(
            "{} exposes {} for the bounded alpha workflow; full incumbent behavior is not claimed.",
            preset.display_name(),
            claim.command_id
        ),
        KeybindingBridgeOutcomeClass::Shimmed => {
            "Command is reachable through Aureline help/browser handoff semantics, not native incumbent runtime parity.".to_string()
        }
        KeybindingBridgeOutcomeClass::Unsupported => {
            "No bounded alpha translation is available.".to_string()
        }
    }
}

fn default_inspection_scope(platform: PlatformClass) -> InspectionScope {
    InspectionScope {
        platform_class: platform,
        surface_ref: "surface:shell".to_string(),
        focus_context_ref: "focus:shell".to_string(),
        active_mode_ref: None,
        workspace_scope_ref: "workspace:alpha".to_string(),
        surface_support_class: SurfaceSupportClass::FullySupported,
    }
}

fn narrowing_from_packet(
    packet: &aureline_input::keybindings::KeybindingResolutionPacketRecord,
) -> Option<KeybindingNarrowingRecord> {
    match packet.sequence_state {
        SequenceResolutionState::BlockedByHost => Some(KeybindingNarrowingRecord {
            narrowing_class: "platform_reserved".to_string(),
            owner_ref: "platform:host".to_string(),
            explanation: "Host platform captured the sequence before dispatch.".to_string(),
        }),
        SequenceResolutionState::BlockedByPolicy => Some(KeybindingNarrowingRecord {
            narrowing_class: "admin_policy_lock".to_string(),
            owner_ref: packet.policy_context.policy_epoch.clone(),
            explanation: "Admin policy denied or pinned this binding.".to_string(),
        }),
        SequenceResolutionState::BlockedBySecurity => Some(KeybindingNarrowingRecord {
            narrowing_class: "emergency_security_hard_block".to_string(),
            owner_ref: packet.policy_context.policy_epoch.clone(),
            explanation: "Emergency or security hard block denied dispatch.".to_string(),
        }),
        _ => None,
    }
}

fn settings_layer_from_token(token: &str) -> KeybindingSettingSourceLayer {
    match token {
        "platform_reserved" => KeybindingSettingSourceLayer::PlatformReserved,
        "emergency_security_hard_block" => KeybindingSettingSourceLayer::EmergencySecurityHardBlock,
        "admin_policy_lock" => KeybindingSettingSourceLayer::AdminPolicyLock,
        "temporary_mode_overlay" => KeybindingSettingSourceLayer::TemporaryModeOverlay,
        "user_profile_binding" => KeybindingSettingSourceLayer::UserProfileBinding,
        "workspace_recommendation" => KeybindingSettingSourceLayer::WorkspaceRecommendation,
        "extension_binding" => KeybindingSettingSourceLayer::ExtensionBinding,
        "core_default" => KeybindingSettingSourceLayer::CoreDefault,
        _ => KeybindingSettingSourceLayer::NotBound,
    }
}

fn platform_token(platform: PlatformClass) -> &'static str {
    match platform {
        PlatformClass::Macos => "macos",
        PlatformClass::Windows => "windows",
        PlatformClass::Linux => "linux",
        PlatformClass::Web => "web",
        PlatformClass::CrossPlatform => "cross_platform",
    }
}

fn resolver_layer_token(layer: ResolverLayerClass) -> &'static str {
    match layer {
        ResolverLayerClass::PlatformReserved => "platform_reserved",
        ResolverLayerClass::EmergencySecurityHardBlock => "emergency_security_hard_block",
        ResolverLayerClass::AdminPolicyLock => "admin_policy_lock",
        ResolverLayerClass::TemporaryModeOverlay => "temporary_mode_overlay",
        ResolverLayerClass::UserProfileBinding => "user_profile_binding",
        ResolverLayerClass::WorkspaceRecommendation => "workspace_recommendation",
        ResolverLayerClass::ExtensionBinding => "extension_binding",
        ResolverLayerClass::CoreDefault => "core_default",
    }
}

fn sequence_state_token(state: SequenceResolutionState) -> &'static str {
    match state {
        SequenceResolutionState::Resolved => "resolved",
        SequenceResolutionState::WaitingForNextStroke => "waiting_for_next_stroke",
        SequenceResolutionState::TimedOut => "timed_out",
        SequenceResolutionState::Unbound => "unbound",
        SequenceResolutionState::BlockedByHost => "blocked_by_host",
        SequenceResolutionState::BlockedBySecurity => "blocked_by_security",
        SequenceResolutionState::BlockedByPolicy => "blocked_by_policy",
        SequenceResolutionState::DisabledCommand => "disabled_command",
        SequenceResolutionState::UnsupportedOnSurface => "unsupported_on_surface",
    }
}

fn resolution_reason_token(reason: ResolutionReasonCode) -> &'static str {
    match reason {
        ResolutionReasonCode::PlatformReservedBeforeDispatch => "platform_reserved_before_dispatch",
        ResolutionReasonCode::EmergencySecurityBlockedDispatch => {
            "emergency_security_blocked_dispatch"
        }
        ResolutionReasonCode::AdminPolicyLockedBinding => "admin_policy_locked_binding",
        ResolutionReasonCode::TemporaryModeOverlayPreempted => "temporary_mode_overlay_preempted",
        ResolutionReasonCode::HigherPrecedenceBindingWon => "higher_precedence_binding_won",
        ResolutionReasonCode::MoreSpecificBindingWon => "more_specific_binding_won",
        ResolutionReasonCode::PartialSequenceWaitingForNextStroke => {
            "partial_sequence_waiting_for_next_stroke"
        }
        ResolutionReasonCode::SequenceTimeoutExpired => "sequence_timeout_expired",
        ResolutionReasonCode::DisabledReasonInheritedFromDescriptor => {
            "disabled_reason_inherited_from_descriptor"
        }
        ResolutionReasonCode::HostCapturePreventsDispatch => "host_capture_prevents_dispatch",
        ResolutionReasonCode::SurfaceCannotHonorSequence => "surface_cannot_honor_sequence",
        ResolutionReasonCode::NoCandidateBound => "no_candidate_bound",
        ResolutionReasonCode::SameLayerCollisionRequiresReview => {
            "same_layer_collision_requires_review"
        }
    }
}

fn _winning_kind_token(kind: WinningResolutionKind) -> &'static str {
    match kind {
        WinningResolutionKind::CommandCandidate => "command_candidate",
        WinningResolutionKind::PlatformReserved => "platform_reserved",
        WinningResolutionKind::EmergencySecurityHardBlock => "emergency_security_hard_block",
        WinningResolutionKind::AdminPolicyLock => "admin_policy_lock",
        WinningResolutionKind::WaitingState => "waiting_state",
        WinningResolutionKind::Unbound => "unbound",
    }
}

/// Returns the command ids claimed by the alpha keybinding projection.
pub fn alpha_keybinding_command_ids() -> BTreeSet<String> {
    alpha_command_registry()
        .claimed_commands
        .iter()
        .map(|claim| claim.command_id.clone())
        .collect()
}

/// Materializes the default shell-facing alpha keybinding report.
pub fn default_alpha_keybinding_truth() -> AlphaKeybindingTruthReport {
    materialize_alpha_keybinding_truth(
        seeded_registry(),
        KeymapPresetId::VsCode,
        PlatformClass::Macos,
    )
}
