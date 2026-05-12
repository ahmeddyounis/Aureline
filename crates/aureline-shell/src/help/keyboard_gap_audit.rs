//! Launch-critical keyboard path audit projection for help surfaces.
//!
//! The audit consumes the canonical command registry plus the keybinding
//! resolver's preset rows and conflict packets so the help surface can explain
//! which alpha paths are keyboard covered, which are explicitly outside the
//! current claim, and which still need command exposure work.

use std::collections::{BTreeMap, BTreeSet};

use aureline_commands::CommandRegistry;
use aureline_input::keybindings::{
    InspectionScope, KeySequence, PlatformClass, ResolverLayerClass, SequenceResolutionState,
    SurfaceSupportClass, WinningResolutionKind,
};
use aureline_input::presets::{
    preset_binding_rows, preset_conflicts, resolver_with_preset, KeymapPresetId,
};
use serde::{Deserialize, Serialize};

/// Boundary schema revision for the launch-critical keyboard audit projection.
pub const KEYBOARD_GAP_AUDIT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy)]
struct AlphaKeyboardSurfaceSpec {
    surface_id: &'static str,
    surface_label: &'static str,
    path_class: &'static str,
    command_ids: &'static [&'static str],
    keyboard_route: &'static str,
    focus_return_state: &'static str,
    focus_return_target_ref: &'static str,
    actionable_gap: Option<&'static str>,
    explicit_non_goal: Option<&'static str>,
}

const ALPHA_KEYBOARD_SURFACE_SPECS: &[AlphaKeyboardSurfaceSpec] = &[
    AlphaKeyboardSurfaceSpec {
        surface_id: "start_center.entry_actions",
        surface_label: "Start Center entry actions",
        path_class: "entry",
        command_ids: &[
            "cmd:workspace.open_folder",
            "cmd:workspace.clone_repository",
            "cmd:workspace.import_profile",
            "cmd:workspace.restore_from_checkpoint",
        ],
        keyboard_route: "Tab or arrow to an entry action, then Enter; command palette search reaches the same command ids.",
        focus_return_state: "returned_exact",
        focus_return_target_ref: "focus:start_center.selected_entry_action",
        actionable_gap: None,
        explicit_non_goal: None,
    },
    AlphaKeyboardSurfaceSpec {
        surface_id: "onboarding.first_run_no_account",
        surface_label: "First-run no-account path",
        path_class: "onboarding",
        command_ids: &["cmd:workspace.open_folder", "cmd:workspace.import_profile"],
        keyboard_route: "Start Center selection or command palette search for Open Folder / Import Profile.",
        focus_return_state: "returned_exact",
        focus_return_target_ref: "focus:start_center.first_run_action",
        actionable_gap: None,
        explicit_non_goal: None,
    },
    AlphaKeyboardSurfaceSpec {
        surface_id: "migration.import_review",
        surface_label: "Migration import review",
        path_class: "migration_review",
        command_ids: &["cmd:workspace.import_profile"],
        keyboard_route: "Open Import Profile from Start Center or palette, review the preview sheet, then Enter to confirm or Esc to cancel.",
        focus_return_state: "returned_current_batch_or_detail_owner",
        focus_return_target_ref: "focus:import_review.source_row",
        actionable_gap: None,
        explicit_non_goal: None,
    },
    AlphaKeyboardSurfaceSpec {
        surface_id: "palette.command_diagnostics",
        surface_label: "Palette diagnostics",
        path_class: "palette_diagnostics",
        command_ids: &["cmd:command_palette.open", "cmd:labs.open_command_trace"],
        keyboard_route: "Open the palette with the active preset shortcut; disabled rows expose diagnostics from the selected row.",
        focus_return_state: "returned_exact",
        focus_return_target_ref: "focus:command_palette.query_input",
        actionable_gap: None,
        explicit_non_goal: None,
    },
    AlphaKeyboardSurfaceSpec {
        surface_id: "command_preview.import_profile",
        surface_label: "Preview-required command flow",
        path_class: "preview_required_command",
        command_ids: &["cmd:workspace.import_profile"],
        keyboard_route: "Invoke Import Profile by keybinding or palette, then use the invocation preview sheet before apply.",
        focus_return_state: "returned_current_batch_or_detail_owner",
        focus_return_target_ref: "focus:invocation_preview.primary_review_action",
        actionable_gap: None,
        explicit_non_goal: None,
    },
    AlphaKeyboardSurfaceSpec {
        surface_id: "restore.session_restore",
        surface_label: "Restore and recovery handoff",
        path_class: "restore",
        command_ids: &["cmd:workspace.restore_from_checkpoint"],
        keyboard_route: "Use the restore action from Start Center or palette; Esc returns to the restore card and preserves the selected checkpoint.",
        focus_return_state: "returned_placeholder_announced",
        focus_return_target_ref: "focus:restore_prompt.selected_checkpoint",
        actionable_gap: None,
        explicit_non_goal: None,
    },
    AlphaKeyboardSurfaceSpec {
        surface_id: "git.clone_review",
        surface_label: "Git source acquisition review",
        path_class: "git",
        command_ids: &["cmd:workspace.clone_repository"],
        keyboard_route: "Invoke Clone Repository from Start Center or palette, then review the clone sheet before network activity.",
        focus_return_state: "returned_current_batch_or_detail_owner",
        focus_return_target_ref: "focus:clone_review.remote_url_field",
        actionable_gap: None,
        explicit_non_goal: None,
    },
    AlphaKeyboardSurfaceSpec {
        surface_id: "git.status_commit_baseline",
        surface_label: "Git status, stage, and commit baseline",
        path_class: "git",
        command_ids: &[],
        keyboard_route: "No command-backed alpha route is registered yet.",
        focus_return_state: "focus_loss_denied",
        focus_return_target_ref: "focus:source_control.pending_registry_row",
        actionable_gap: Some(
            "Seed source-control command descriptors for status, stage, unstage, commit, and open-diff, then attach resolver-backed shortcuts and focus-return fixtures.",
        ),
        explicit_non_goal: None,
    },
    AlphaKeyboardSurfaceSpec {
        surface_id: "trust.restricted_mode_review",
        surface_label: "Trust and restricted-mode review",
        path_class: "trust",
        command_ids: &[],
        keyboard_route: "The native shell demo exposes a trust-state keyboard toggle, but no trust-review command descriptor is registered yet.",
        focus_return_state: "focus_loss_denied",
        focus_return_target_ref: "focus:trust_banner.pending_registry_row",
        actionable_gap: Some(
            "Promote trust review, trust elevation, continue restricted, and open trust details into command descriptors before claiming full trust-surface parity.",
        ),
        explicit_non_goal: None,
    },
    AlphaKeyboardSurfaceSpec {
        surface_id: "enterprise.rollout_depth",
        surface_label: "Enterprise rollout and fleet-depth keyboard proof",
        path_class: "non_goal",
        command_ids: &[],
        keyboard_route: "Not claimed for the current external alpha keyboard audit.",
        focus_return_state: "focus_not_applicable_non_interactive",
        focus_return_target_ref: "focus:not_applicable",
        actionable_gap: None,
        explicit_non_goal: Some(
            "Fleet rollout and enterprise admin depth are outside this alpha keyboard audit; the trust row above covers the local restricted-mode baseline.",
        ),
    },
];

/// Serializable audit packet rendered by help and validation surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaKeyboardGapAudit {
    /// Boundary record kind.
    pub record_kind: String,
    /// Schema version for this projection.
    pub schema_version: u32,
    /// Active preset ref used for per-command winning-binding attribution.
    pub active_preset_ref: String,
    /// Platform class used for platform-specific shortcut projection.
    pub platform_class: String,
    /// Source artifacts consumed by this projection.
    pub resolver_output_refs: Vec<String>,
    /// Launch-critical keyboard path rows.
    pub rows: Vec<AlphaKeyboardPathRow>,
    /// Preset profile coverage across the claimed command-backed rows.
    pub preset_coverage: Vec<PresetProfileCoverage>,
    /// Resolver conflict packets surfaced by preset/profile inspection.
    pub conflict_reports: Vec<KeybindingConflictSummary>,
    /// Actionable gap rows that remain before broader alpha claims.
    pub remaining_gaps: Vec<ActionableKeyboardGap>,
}

/// One launch-critical surface row in the keyboard audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaKeyboardPathRow {
    /// Stable surface id for fixtures and support exports.
    pub surface_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Functional path class, such as entry, restore, palette, Git, or trust.
    pub path_class: String,
    /// Coverage state for the row.
    pub coverage_state: String,
    /// Keyboard route state for the row.
    pub keyboard_route_state: String,
    /// User-facing keyboard route or the explicit gap/non-goal note.
    pub keyboard_route: String,
    /// Focus-return proof state from the focus contract vocabulary.
    pub focus_return_state: String,
    /// Stable focus target expected after dismissal, cancel, or completion.
    pub focus_return_target_ref: String,
    /// Command exposure rows derived from the canonical command registry.
    pub command_exposures: Vec<CommandKeyboardExposure>,
    /// Explicit non-goal note when the row is outside the current claim.
    pub explicit_non_goal: Option<String>,
    /// Required follow-up when the row remains a launch-critical gap.
    pub actionable_gap: Option<String>,
}

/// Command-level keyboard attribution derived from resolver output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandKeyboardExposure {
    /// Stable command id under inspection.
    pub command_id: String,
    /// Command title from the canonical registry.
    pub command_title: String,
    /// Current exposure state.
    pub exposure_state: String,
    /// Current active keybinding or `unassigned`.
    pub active_keybinding: String,
    /// Resolver source provenance for the winning binding.
    pub winning_source_ref: Option<String>,
    /// Winning resolver layer token.
    pub winning_layer: Option<String>,
    /// Sequence state returned by the resolver.
    pub sequence_state: String,
    /// Winning resolution kind returned by the resolver.
    pub winning_resolution_kind: String,
}

/// Coverage summary for one preset profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresetProfileCoverage {
    /// Stable preset ref.
    pub preset_ref: String,
    /// Display name for the preset.
    pub display_name: String,
    /// Number of required command-backed rows with a binding in this preset.
    pub command_backed_bindings: usize,
    /// Number of command-backed rows expected by this audit.
    pub expected_command_backed_bindings: usize,
    /// Number of conflicts surfaced by the resolver for this preset.
    pub conflict_count: usize,
    /// Coverage state for this preset.
    pub coverage_state: String,
}

/// Compact conflict summary projected from resolver conflict packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeybindingConflictSummary {
    /// Stable preset ref that produced the conflict.
    pub preset_ref: String,
    /// Literal sequence under conflict.
    pub literal_sequence: String,
    /// Resolver conflict-review id.
    pub conflict_review_id: String,
    /// Winning command id when one exists.
    pub winning_command_id: Option<String>,
    /// Losing command ids that require review.
    pub losing_command_ids: Vec<String>,
}

/// One actionable keyboard gap emitted by the audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionableKeyboardGap {
    /// Surface id carrying the gap.
    pub surface_id: String,
    /// Functional path class that owns the gap.
    pub path_class: String,
    /// Follow-up needed before widening the claim.
    pub action_required: String,
}

/// Materializes the launch-critical keyboard gap audit.
pub fn materialize_alpha_keyboard_gap_audit(
    registry: &CommandRegistry,
    active_preset: KeymapPresetId,
    platform: PlatformClass,
) -> AlphaKeyboardGapAudit {
    let resolver = resolver_with_preset(active_preset, platform)
        .unwrap_or_else(|_| aureline_input::keybindings::seeded_keybinding_resolver().clone());
    let preset_rows = preset_binding_rows(active_preset, platform).unwrap_or_default();
    let binding_rows_by_command = binding_rows_by_command_id(preset_rows);

    let rows: Vec<AlphaKeyboardPathRow> = ALPHA_KEYBOARD_SURFACE_SPECS
        .iter()
        .map(|spec| {
            materialize_path_row(
                registry,
                &resolver,
                &binding_rows_by_command,
                platform,
                *spec,
            )
        })
        .collect();
    let remaining_gaps = rows
        .iter()
        .filter_map(|row| {
            row.actionable_gap
                .as_ref()
                .map(|action_required| ActionableKeyboardGap {
                    surface_id: row.surface_id.clone(),
                    path_class: row.path_class.clone(),
                    action_required: action_required.clone(),
                })
        })
        .collect();

    AlphaKeyboardGapAudit {
        record_kind: "alpha_keyboard_gap_audit_record".to_string(),
        schema_version: KEYBOARD_GAP_AUDIT_SCHEMA_VERSION,
        active_preset_ref: active_preset.preset_ref().to_string(),
        platform_class: platform_token(platform).to_string(),
        resolver_output_refs: vec![
            "crates/aureline-input/src/keybindings/mod.rs".to_string(),
            "crates/aureline-input/src/presets/mod.rs".to_string(),
            "crates/aureline-shell/src/help/keybinding_inspector.rs".to_string(),
        ],
        rows,
        preset_coverage: materialize_preset_coverage(platform),
        conflict_reports: materialize_conflict_reports(platform),
        remaining_gaps,
    }
}

/// Builds compact text lines for the reachable keybinding help surface.
pub fn build_audit_summary_lines(
    registry: &CommandRegistry,
    active_preset: KeymapPresetId,
    platform: PlatformClass,
) -> Vec<String> {
    let audit = materialize_alpha_keyboard_gap_audit(registry, active_preset, platform);
    let mut lines = vec![
        "".to_string(),
        format!("Alpha keyboard audit — preset: {}", audit.active_preset_ref),
    ];

    for row in &audit.rows {
        let command_summary = if row.command_exposures.is_empty() {
            "no command descriptor".to_string()
        } else {
            row.command_exposures
                .iter()
                .map(|exposure| format!("{} [{}]", exposure.command_id, exposure.active_keybinding))
                .collect::<Vec<_>>()
                .join(", ")
        };
        lines.push(format!(
            "- {}  —  {}  —  focus={}  —  {}",
            row.surface_label, row.coverage_state, row.focus_return_state, command_summary
        ));
    }

    if audit.remaining_gaps.is_empty() {
        lines.push("- remaining gaps: none".to_string());
    } else {
        lines.push("Remaining gaps".to_string());
        for gap in audit.remaining_gaps {
            lines.push(format!("- {}  —  {}", gap.surface_id, gap.action_required));
        }
    }

    lines
}

fn materialize_path_row(
    registry: &CommandRegistry,
    resolver: &aureline_input::keybindings::KeybindingResolver,
    binding_rows_by_command: &BTreeMap<String, Vec<aureline_input::presets::PresetBindingRow>>,
    platform: PlatformClass,
    spec: AlphaKeyboardSurfaceSpec,
) -> AlphaKeyboardPathRow {
    let command_exposures = spec
        .command_ids
        .iter()
        .map(|command_id| {
            materialize_command_exposure(
                registry,
                resolver,
                binding_rows_by_command,
                platform,
                spec,
                command_id,
            )
        })
        .collect();
    let coverage_state = if spec.explicit_non_goal.is_some() {
        "explicit_non_goal"
    } else if spec.actionable_gap.is_some() {
        "gap_action_required"
    } else {
        "covered"
    };
    let keyboard_route_state = if spec.explicit_non_goal.is_some() {
        "explicit_non_goal"
    } else if spec.actionable_gap.is_some() {
        "gap_action_required"
    } else {
        "documented_keyboard_route"
    };

    AlphaKeyboardPathRow {
        surface_id: spec.surface_id.to_string(),
        surface_label: spec.surface_label.to_string(),
        path_class: spec.path_class.to_string(),
        coverage_state: coverage_state.to_string(),
        keyboard_route_state: keyboard_route_state.to_string(),
        keyboard_route: spec.keyboard_route.to_string(),
        focus_return_state: spec.focus_return_state.to_string(),
        focus_return_target_ref: spec.focus_return_target_ref.to_string(),
        command_exposures,
        explicit_non_goal: spec.explicit_non_goal.map(str::to_string),
        actionable_gap: spec.actionable_gap.map(str::to_string),
    }
}

fn materialize_command_exposure(
    registry: &CommandRegistry,
    resolver: &aureline_input::keybindings::KeybindingResolver,
    binding_rows_by_command: &BTreeMap<String, Vec<aureline_input::presets::PresetBindingRow>>,
    platform: PlatformClass,
    spec: AlphaKeyboardSurfaceSpec,
    command_id: &str,
) -> CommandKeyboardExposure {
    let Some(entry) = registry.get(command_id) else {
        return CommandKeyboardExposure {
            command_id: command_id.to_string(),
            command_title: "<missing command>".to_string(),
            exposure_state: "missing_from_registry".to_string(),
            active_keybinding: "unassigned".to_string(),
            winning_source_ref: None,
            winning_layer: None,
            sequence_state: "unbound".to_string(),
            winning_resolution_kind: "unbound".to_string(),
        };
    };

    let Some(binding_row) = binding_rows_by_command
        .get(command_id)
        .and_then(|rows| rows.first())
    else {
        return CommandKeyboardExposure {
            command_id: command_id.to_string(),
            command_title: entry.title.clone(),
            exposure_state: "registered_unassigned".to_string(),
            active_keybinding: "unassigned".to_string(),
            winning_source_ref: None,
            winning_layer: None,
            sequence_state: "unbound".to_string(),
            winning_resolution_kind: "unbound".to_string(),
        };
    };

    let Ok(sequence) = KeySequence::parse_literal_sequence(&binding_row.literal_sequence) else {
        return CommandKeyboardExposure {
            command_id: command_id.to_string(),
            command_title: entry.title.clone(),
            exposure_state: "invalid_binding_sequence".to_string(),
            active_keybinding: binding_row.literal_sequence.clone(),
            winning_source_ref: Some(binding_row.source_provenance_ref.clone()),
            winning_layer: None,
            sequence_state: "unbound".to_string(),
            winning_resolution_kind: "unbound".to_string(),
        };
    };

    let packet = resolver.resolve(&sequence, &inspection_scope_for(platform, spec));
    let (winning_source_ref, winning_layer, winning_kind) =
        if let Some(candidate) = packet.winning_resolution.command_candidate.as_ref() {
            (
                candidate.source_provenance_ref.clone(),
                Some(resolver_layer_token(candidate.resolver_layer).to_string()),
                winning_resolution_kind_token(packet.winning_resolution.winner_kind).to_string(),
            )
        } else {
            (
                Some(binding_row.source_provenance_ref.clone()),
                packet
                    .winning_resolution
                    .resolver_layer
                    .map(|layer| resolver_layer_token(layer).to_string()),
                winning_resolution_kind_token(packet.winning_resolution.winner_kind).to_string(),
            )
        };

    CommandKeyboardExposure {
        command_id: command_id.to_string(),
        command_title: entry.title.clone(),
        exposure_state: "registered_bound".to_string(),
        active_keybinding: binding_row.literal_sequence.clone(),
        winning_source_ref,
        winning_layer,
        sequence_state: sequence_state_token(packet.sequence_state).to_string(),
        winning_resolution_kind: winning_kind,
    }
}

fn binding_rows_by_command_id(
    rows: Vec<aureline_input::presets::PresetBindingRow>,
) -> BTreeMap<String, Vec<aureline_input::presets::PresetBindingRow>> {
    let mut by_command: BTreeMap<String, Vec<aureline_input::presets::PresetBindingRow>> =
        BTreeMap::new();
    for row in rows {
        by_command
            .entry(row.command_id.clone())
            .or_default()
            .push(row);
    }
    by_command
}

fn inspection_scope_for(
    platform: PlatformClass,
    spec: AlphaKeyboardSurfaceSpec,
) -> InspectionScope {
    InspectionScope {
        platform_class: platform,
        surface_ref: format!("surface:{}", spec.surface_id),
        focus_context_ref: format!("focus:{}", spec.surface_id),
        active_mode_ref: None,
        workspace_scope_ref: "workspace:external_alpha".to_string(),
        surface_support_class: SurfaceSupportClass::FullySupported,
    }
}

fn required_command_ids() -> BTreeSet<&'static str> {
    ALPHA_KEYBOARD_SURFACE_SPECS
        .iter()
        .flat_map(|spec| spec.command_ids.iter().copied())
        .collect()
}

fn materialize_preset_coverage(platform: PlatformClass) -> Vec<PresetProfileCoverage> {
    let required = required_command_ids();
    let expected_count = required.len();
    KeymapPresetId::all()
        .iter()
        .map(|preset| {
            let rows = preset_binding_rows(*preset, platform).unwrap_or_default();
            let bound_count = rows
                .iter()
                .filter(|row| required.contains(row.command_id.as_str()))
                .map(|row| row.command_id.as_str())
                .collect::<BTreeSet<_>>()
                .len();
            let conflict_count = preset_conflicts(*preset, platform)
                .map(|conflicts| conflicts.len())
                .unwrap_or(0);
            PresetProfileCoverage {
                preset_ref: preset.preset_ref().to_string(),
                display_name: preset.display_name().to_string(),
                command_backed_bindings: bound_count,
                expected_command_backed_bindings: expected_count,
                conflict_count,
                coverage_state: if bound_count == expected_count {
                    "covers_claimed_command_set".to_string()
                } else {
                    "missing_claimed_command_binding".to_string()
                },
            }
        })
        .collect()
}

fn materialize_conflict_reports(platform: PlatformClass) -> Vec<KeybindingConflictSummary> {
    let mut reports = Vec::new();
    for preset in KeymapPresetId::all() {
        if let Ok(conflicts) = preset_conflicts(preset, platform) {
            for conflict in conflicts {
                reports.push(KeybindingConflictSummary {
                    preset_ref: preset.preset_ref().to_string(),
                    literal_sequence: conflict.inspected_sequence.literal_sequence,
                    conflict_review_id: conflict.conflict_review_id,
                    winning_command_id: conflict
                        .winning_resolution
                        .command_candidate
                        .map(|candidate| candidate.command.command_id),
                    losing_command_ids: conflict
                        .losing_candidates
                        .into_iter()
                        .map(|losing| losing.candidate.command.command_id)
                        .collect(),
                });
            }
        }
    }
    reports
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

fn winning_resolution_kind_token(kind: WinningResolutionKind) -> &'static str {
    match kind {
        WinningResolutionKind::CommandCandidate => "command_candidate",
        WinningResolutionKind::PlatformReserved => "platform_reserved",
        WinningResolutionKind::EmergencySecurityHardBlock => "emergency_security_hard_block",
        WinningResolutionKind::AdminPolicyLock => "admin_policy_lock",
        WinningResolutionKind::WaitingState => "waiting_state",
        WinningResolutionKind::Unbound => "unbound",
    }
}
