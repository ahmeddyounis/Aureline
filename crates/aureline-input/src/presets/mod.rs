//! Built-in keymap preset definitions.
//!
//! Presets seed the `user_profile_binding` layer with a named shortcut set so
//! runtime, help, and migration surfaces can project from one canonical source.

use std::collections::HashMap;
use std::fmt;

use aureline_commands::registry::seeded_registry;
use aureline_commands::CommandId;

use crate::keybindings::{
    seeded_keybinding_resolver, CandidateContext, CommandSemanticsSnapshot, InspectionScope,
    KeySequence, KeybindingConflictReviewPacketRecord, KeybindingResolver, ParseKeySequenceError,
    ResolverLayerClass,
};
use crate::keybindings::{PlatformClass, SequenceResolutionState, WinningResolutionKind};

/// Identifies a built-in keymap preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeymapPresetId {
    VsCode,
    IntelliJ,
    Vim,
    Emacs,
}

impl KeymapPresetId {
    /// Returns all built-in presets in a stable order suitable for UI cycling.
    pub const fn all() -> [Self; 4] {
        [Self::VsCode, Self::IntelliJ, Self::Vim, Self::Emacs]
    }

    /// Returns a stable opaque ref for the preset, suitable for provenance fields.
    pub const fn preset_ref(self) -> &'static str {
        match self {
            Self::VsCode => "preset:keymap:vs_code",
            Self::IntelliJ => "preset:keymap:intellij",
            Self::Vim => "preset:keymap:vim",
            Self::Emacs => "preset:keymap:emacs",
        }
    }

    /// Returns a short UI-friendly label.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::VsCode => "VS Code",
            Self::IntelliJ => "IntelliJ",
            Self::Vim => "Vim",
            Self::Emacs => "Emacs",
        }
    }
}

/// Error returned when applying a preset fails validation.
#[derive(Debug)]
pub enum PresetSeedError {
    UnknownCommandId(String),
    InvalidSequence {
        sequence: String,
        detail: ParseKeySequenceError,
    },
}

impl fmt::Display for PresetSeedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCommandId(command_id) => write!(f, "unknown command_id: {command_id}"),
            Self::InvalidSequence { sequence, detail } => {
                write!(f, "invalid key sequence {sequence}: {detail}")
            }
        }
    }
}

impl std::error::Error for PresetSeedError {}

/// One preset binding row suitable for help/palette projections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetBindingRow {
    pub command_id: CommandId,
    pub literal_sequence: String,
    pub candidate_ref: String,
    pub source_provenance_ref: String,
}

#[derive(Debug, Clone, Copy)]
struct PresetBindingSpec {
    command_id: &'static str,
    macos_sequence: &'static str,
    other_sequence: &'static str,
}

fn specs_for(preset: KeymapPresetId) -> &'static [PresetBindingSpec] {
    match preset {
        KeymapPresetId::VsCode => &VS_CODE_SPECS,
        KeymapPresetId::IntelliJ => &INTELLIJ_SPECS,
        KeymapPresetId::Vim => &VIM_SPECS,
        KeymapPresetId::Emacs => &EMACS_SPECS,
    }
}

fn sequence_for_platform(spec: PresetBindingSpec, platform: PlatformClass) -> &'static str {
    match platform {
        PlatformClass::Macos => spec.macos_sequence,
        _ => spec.other_sequence,
    }
}

fn sanitize_sequence_for_candidate_ref(sequence: &str) -> String {
    sequence.replace([' ', '+'], "_").to_lowercase()
}

fn candidate_ref_for(preset: KeymapPresetId, command_id: &str, sequence: &str) -> String {
    format!(
        "candidate:{}:{}:{}",
        preset.preset_ref(),
        command_id,
        sanitize_sequence_for_candidate_ref(sequence)
    )
}

fn command_snapshot(command_id: &str) -> Result<CommandSemanticsSnapshot, PresetSeedError> {
    let registry = seeded_registry();
    let Some(entry) = registry.get(command_id) else {
        return Err(PresetSeedError::UnknownCommandId(command_id.to_string()));
    };

    Ok(CommandSemanticsSnapshot {
        command_id: entry.descriptor.command_id.clone(),
        command_revision_ref: entry.descriptor.command_revision_ref.clone(),
        preview_class: entry.descriptor.preview_class.clone(),
        approval_posture_class: entry.descriptor.approval_posture_class.clone(),
        capability_scope_class: entry.descriptor.capability_scope_class.clone(),
        required_evidence_ref_classes: entry
            .descriptor
            .result_contract
            .evidence_ref_class_required
            .clone(),
    })
}

fn default_candidate_context() -> CandidateContext {
    CandidateContext {
        surface_ref: None,
        focus_context_ref: None,
        active_mode_ref: None,
        workspace_scope_ref: None,
        scope_specificity_rank: 1,
    }
}

/// Returns the preset bindings for use in help or palette surfaces.
pub fn preset_binding_rows(
    preset: KeymapPresetId,
    platform: PlatformClass,
) -> Result<Vec<PresetBindingRow>, PresetSeedError> {
    let mut rows = Vec::new();
    for spec in specs_for(preset) {
        let literal_sequence = sequence_for_platform(*spec, platform);
        KeySequence::parse_literal_sequence(literal_sequence).map_err(|detail| {
            PresetSeedError::InvalidSequence {
                sequence: literal_sequence.to_string(),
                detail,
            }
        })?;
        rows.push(PresetBindingRow {
            command_id: spec.command_id.to_string(),
            literal_sequence: literal_sequence.to_string(),
            candidate_ref: candidate_ref_for(preset, spec.command_id, literal_sequence),
            source_provenance_ref: preset.preset_ref().to_string(),
        });
    }
    Ok(rows)
}

/// Applies the preset bindings to the resolver as `user_profile_binding`.
pub fn apply_preset_to_resolver(
    resolver: &mut KeybindingResolver,
    preset: KeymapPresetId,
    platform: PlatformClass,
) -> Result<(), PresetSeedError> {
    for row in preset_binding_rows(preset, platform)? {
        let sequence =
            KeySequence::parse_literal_sequence(&row.literal_sequence).map_err(|detail| {
                PresetSeedError::InvalidSequence {
                    sequence: row.literal_sequence.clone(),
                    detail,
                }
            })?;
        let command = command_snapshot(&row.command_id)?;
        resolver.push_candidate(
            ResolverLayerClass::UserProfileBinding,
            row.candidate_ref,
            command,
            sequence,
            Some(row.source_provenance_ref),
            Some(format!("Preset binding ({})", preset.display_name())),
            default_candidate_context(),
        );
    }
    Ok(())
}

/// Builds a resolver suitable for live shell usage with a selected preset layered in.
pub fn resolver_with_preset(
    preset: KeymapPresetId,
    platform: PlatformClass,
) -> Result<KeybindingResolver, PresetSeedError> {
    let mut resolver = seeded_keybinding_resolver().clone();
    apply_preset_to_resolver(&mut resolver, preset, platform)?;
    Ok(resolver)
}

/// Computes a fast conflict summary for the preset bindings on the given platform.
pub fn preset_conflicts(
    preset: KeymapPresetId,
    platform: PlatformClass,
) -> Result<Vec<KeybindingConflictReviewPacketRecord>, PresetSeedError> {
    let resolver = resolver_with_preset(preset, platform)?;
    let rows = preset_binding_rows(preset, platform)?;

    let mut sequences: HashMap<String, Vec<String>> = HashMap::new();
    for row in rows {
        sequences
            .entry(row.literal_sequence)
            .or_default()
            .push(row.command_id);
    }

    let inspection_scope = InspectionScope {
        platform_class: platform,
        surface_ref: "surface:shell".to_string(),
        focus_context_ref: "focus:shell".to_string(),
        active_mode_ref: None,
        workspace_scope_ref: "workspace:unknown".to_string(),
        surface_support_class: crate::keybindings::SurfaceSupportClass::FullySupported,
    };

    let mut packets = Vec::new();
    for (literal_sequence, command_ids) in sequences {
        if command_ids.len() < 2 {
            continue;
        }
        let inspected =
            KeySequence::parse_literal_sequence(&literal_sequence).map_err(|detail| {
                PresetSeedError::InvalidSequence {
                    sequence: literal_sequence.clone(),
                    detail,
                }
            })?;

        let packet = resolver.resolve(&inspected, &inspection_scope);
        if packet.sequence_state == SequenceResolutionState::Unbound
            && packet.winning_resolution.winner_kind == WinningResolutionKind::Unbound
            && packet.winning_resolution.reason_code
                == crate::keybindings::ResolutionReasonCode::SameLayerCollisionRequiresReview
        {
            if let Some(review) = resolver.conflict_review(&inspected, &inspection_scope) {
                packets.push(review);
            }
        }
    }

    Ok(packets)
}

const VS_CODE_SPECS: [PresetBindingSpec; 10] = [
    PresetBindingSpec {
        command_id: "cmd:command_palette.open",
        macos_sequence: "Cmd+Shift+P",
        other_sequence: "Ctrl+Shift+P",
    },
    PresetBindingSpec {
        command_id: "cmd:terminal.toggle",
        macos_sequence: "Cmd+`",
        other_sequence: "Ctrl+`",
    },
    PresetBindingSpec {
        command_id: "cmd:editor.find",
        macos_sequence: "Cmd+F",
        other_sequence: "Ctrl+F",
    },
    PresetBindingSpec {
        command_id: "cmd:editor.replace",
        macos_sequence: "Cmd+Alt+F",
        other_sequence: "Ctrl+H",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.open_folder",
        macos_sequence: "Cmd+Shift+O",
        other_sequence: "Ctrl+Shift+O",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.clone_repository",
        macos_sequence: "Cmd+Shift+C",
        other_sequence: "Ctrl+Shift+C",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.import_profile",
        macos_sequence: "Cmd+Shift+I",
        other_sequence: "Ctrl+Shift+I",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.restore_from_checkpoint",
        macos_sequence: "Cmd+Shift+R",
        other_sequence: "Ctrl+Shift+R",
    },
    PresetBindingSpec {
        command_id: "cmd:docs.open_in_browser",
        macos_sequence: "Cmd+Shift+H",
        other_sequence: "Ctrl+Shift+H",
    },
    PresetBindingSpec {
        command_id: "cmd:labs.open_command_trace",
        macos_sequence: "Cmd+Shift+Y",
        other_sequence: "Ctrl+Shift+Y",
    },
];

const INTELLIJ_SPECS: [PresetBindingSpec; 10] = [
    PresetBindingSpec {
        command_id: "cmd:command_palette.open",
        macos_sequence: "Cmd+Shift+A",
        other_sequence: "Ctrl+Shift+A",
    },
    PresetBindingSpec {
        command_id: "cmd:terminal.toggle",
        macos_sequence: "Cmd+`",
        other_sequence: "Ctrl+`",
    },
    PresetBindingSpec {
        command_id: "cmd:editor.find",
        macos_sequence: "Cmd+F",
        other_sequence: "Ctrl+F",
    },
    PresetBindingSpec {
        command_id: "cmd:editor.replace",
        macos_sequence: "Cmd+R",
        other_sequence: "Ctrl+R",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.open_folder",
        macos_sequence: "Cmd+Shift+O",
        other_sequence: "Ctrl+Shift+O",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.clone_repository",
        macos_sequence: "Cmd+Shift+K",
        other_sequence: "Ctrl+Shift+K",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.import_profile",
        macos_sequence: "Cmd+Shift+J",
        other_sequence: "Ctrl+Shift+J",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.restore_from_checkpoint",
        macos_sequence: "Cmd+Shift+R",
        other_sequence: "Ctrl+Shift+R",
    },
    PresetBindingSpec {
        command_id: "cmd:docs.open_in_browser",
        macos_sequence: "Cmd+Shift+H",
        other_sequence: "Ctrl+Shift+H",
    },
    PresetBindingSpec {
        command_id: "cmd:labs.open_command_trace",
        macos_sequence: "Cmd+Shift+Y",
        other_sequence: "Ctrl+Shift+Y",
    },
];

const VIM_SPECS: [PresetBindingSpec; 11] = [
    PresetBindingSpec {
        command_id: "cmd:command_palette.open",
        macos_sequence: "Ctrl+P",
        other_sequence: "Ctrl+P",
    },
    PresetBindingSpec {
        command_id: "cmd:terminal.toggle",
        macos_sequence: "Cmd+`",
        other_sequence: "Ctrl+`",
    },
    PresetBindingSpec {
        command_id: "cmd:editor.find",
        macos_sequence: "Ctrl+F",
        other_sequence: "Ctrl+F",
    },
    PresetBindingSpec {
        command_id: "cmd:editor.replace",
        macos_sequence: "Ctrl+H",
        other_sequence: "Ctrl+H",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.open_folder",
        macos_sequence: "Ctrl+Shift+O",
        other_sequence: "Ctrl+Shift+O",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.clone_repository",
        macos_sequence: "Ctrl+Shift+K",
        other_sequence: "Ctrl+Shift+K",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.import_profile",
        macos_sequence: "Ctrl+Shift+J",
        other_sequence: "Ctrl+Shift+J",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.restore_from_checkpoint",
        macos_sequence: "Ctrl+Shift+R",
        other_sequence: "Ctrl+Shift+R",
    },
    PresetBindingSpec {
        command_id: "cmd:docs.open_in_browser",
        macos_sequence: "Ctrl+Shift+Y",
        other_sequence: "Ctrl+Shift+Y",
    },
    // Intentional early-stage collision: two candidates share the same sequence so
    // conflict-help surfaces have a live example to inspect.
    PresetBindingSpec {
        command_id: "cmd:labs.open_command_trace",
        macos_sequence: "Ctrl+Shift+Y",
        other_sequence: "Ctrl+Shift+Y",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.open_folder",
        macos_sequence: "Ctrl+Shift+F",
        other_sequence: "Ctrl+Shift+F",
    },
];

const EMACS_SPECS: [PresetBindingSpec; 10] = [
    PresetBindingSpec {
        command_id: "cmd:command_palette.open",
        macos_sequence: "Alt+X",
        other_sequence: "Alt+X",
    },
    PresetBindingSpec {
        command_id: "cmd:terminal.toggle",
        macos_sequence: "Cmd+`",
        other_sequence: "Ctrl+`",
    },
    PresetBindingSpec {
        command_id: "cmd:editor.find",
        macos_sequence: "Ctrl+F",
        other_sequence: "Ctrl+F",
    },
    PresetBindingSpec {
        command_id: "cmd:editor.replace",
        macos_sequence: "Ctrl+H",
        other_sequence: "Ctrl+H",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.open_folder",
        macos_sequence: "Ctrl+Shift+O",
        other_sequence: "Ctrl+Shift+O",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.clone_repository",
        macos_sequence: "Ctrl+Shift+K",
        other_sequence: "Ctrl+Shift+K",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.import_profile",
        macos_sequence: "Ctrl+Shift+J",
        other_sequence: "Ctrl+Shift+J",
    },
    PresetBindingSpec {
        command_id: "cmd:workspace.restore_from_checkpoint",
        macos_sequence: "Ctrl+Shift+R",
        other_sequence: "Ctrl+Shift+R",
    },
    PresetBindingSpec {
        command_id: "cmd:docs.open_in_browser",
        macos_sequence: "Ctrl+Shift+H",
        other_sequence: "Ctrl+Shift+H",
    },
    PresetBindingSpec {
        command_id: "cmd:labs.open_command_trace",
        macos_sequence: "Ctrl+Shift+Y",
        other_sequence: "Ctrl+Shift+Y",
    },
];
