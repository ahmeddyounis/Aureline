use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Instant;

use aureline_build_info as build_info;
use aureline_commands::invocation::{
    mint_approval_ticket_ref, mint_basis_snapshot_ref, mint_invocation_session_id,
    mint_preview_record_ref, AliasUsedBlock, ApprovalPostureBlock, ArgumentProvenanceEntry,
    ArtifactRefEntry, CommandInvocationSession, CommandResultPacketRecord, ContextRefsBlock,
    EnablementDecisionBlock, EvidenceRefEntry, ExportPostureBlock, InvocationContextSnapshot,
    InvocationCreatedArtifactRefEntry, InvocationOutcomeBlock, InvocationSessionPacketRecord,
    NoBypassGuards, ResultBodyBlock, RollbackHandleRefBlock,
};
use aureline_commands::registry::seeded_registry;
use aureline_commands::{
    CommandEnablementContext, CommandRegistry, CommandRegistryEntryRecord, DisabledReasonCode,
    EnablementDecisionClass, PreflightDecisionClass,
};
use aureline_input::keybindings::{
    seeded_keybinding_resolver, InspectionScope, KeySequence, KeyStroke, KeybindingResolver,
    Modifiers, PlatformClass, SequenceResolutionState, SurfaceSupportClass, WinningResolutionKind,
};
use aureline_input::presets::{preset_binding_rows, resolver_with_preset, KeymapPresetId};
use aureline_shell::app_frame::desktop_frame::{
    DesktopFrame, NewEditorGroupOutcome, SplitViolation,
};
use aureline_shell::commands::diagnostics_sheet::{
    diagnostics_sheet_lines, materialize_command_diagnostics_sheet_record,
    write_diagnostics_sheet_log, CommandDiagnosticsSheetRecord,
};
use aureline_shell::commands::invocation_preview::{
    invocation_preview_sheet_lines, materialize_command_invocation_preview_sheet_record,
    write_invocation_preview_sheet_log, CommandInvocationPreviewSheetRecord,
};
use aureline_shell::commands::CommandReviewRuntimeInputs;
use aureline_shell::help::keybinding_inspector::build_inspector_lines;
use aureline_shell::layout::split_tree::PaneId;
use aureline_shell::layout::zone_registry::{Rect, ShellZoneId};
use aureline_shell::palette::preview::{
    argument_provenance_map_for, copy_payload_for, materialize_palette_preview_record,
    write_preview_log, PaletteCopyIntent, PalettePreviewRuntimeInputs, PalettePreviewSelection,
};
use aureline_shell::palette::results_view::palette_view_rows;
use aureline_shell::palette::{CommandPaletteCommit, CommandPaletteState};
use aureline_shell::start_center::{
    build_action_rows as start_center_action_rows, StartCenterRuntimeInputs, StartCenterState,
    START_CENTER_PRESENTATION_LABEL, START_CENTER_PRESENTATION_SUBTITLE,
};
use aureline_ui::tokens::{
    seeded_token_registry, ColorRgba, ThemeClass, TokenRegistry, TokenRegistryError,
};

use arboard::Clipboard;
use font8x8::{UnicodeFonts as _, BASIC_FONTS};
use softbuffer::{Context, Surface};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

type SoftbufferSurface = Surface<Arc<winit::window::Window>, Arc<winit::window::Window>>;

#[derive(Debug, Clone)]
struct ShellRenderTokens {
    bg_canvas: ColorRgba,
    bg_subtle: ColorRgba,
    bg_surface: ColorRgba,
    bg_raised: ColorRgba,
    bg_hover: ColorRgba,
    bg_active: ColorRgba,
    bg_overlay: ColorRgba,
    text_primary: ColorRgba,
    text_secondary: ColorRgba,
    text_muted: ColorRgba,
    text_inverse: ColorRgba,
    border_default: ColorRgba,
    border_strong: ColorRgba,
    icon_default: ColorRgba,
    icon_muted: ColorRgba,
    focus_ring: ColorRgba,
    accent_brand: ColorRgba,
    accent_interactive: ColorRgba,
}

impl ShellRenderTokens {
    fn load(registry: &TokenRegistry) -> Result<Self, TokenRegistryError> {
        Ok(Self {
            bg_canvas: registry.require_color("al.color.bg.canvas")?,
            bg_subtle: registry.require_color("al.color.bg.subtle")?,
            bg_surface: registry.require_color("al.color.bg.surface")?,
            bg_raised: registry.require_color("al.color.bg.raised")?,
            bg_hover: registry.require_color("al.color.bg.hover")?,
            bg_active: registry.require_color("al.color.bg.active")?,
            bg_overlay: registry.require_color("al.color.bg.overlay")?,
            text_primary: registry.require_color("al.color.text.primary")?,
            text_secondary: registry.require_color("al.color.text.secondary")?,
            text_muted: registry.require_color("al.color.text.muted")?,
            text_inverse: registry.require_color("al.color.text.inverse")?,
            border_default: registry.require_color("al.color.border.default")?,
            border_strong: registry.require_color("al.color.border.strong")?,
            focus_ring: registry.require_color("al.color.focus.ring")?,
            icon_default: registry.require_color("al.color.icon.default")?,
            icon_muted: registry.require_color("al.color.icon.muted")?,
            accent_brand: registry.require_color("al.color.accent.brand")?,
            accent_interactive: registry.require_color("al.color.accent.interactive")?,
        })
    }

    const fn zone_background(&self, zone: ShellZoneId) -> ColorRgba {
        match zone {
            ShellZoneId::TitleContextBar => self.bg_raised,
            ShellZoneId::ActivityRail => self.bg_subtle,
            ShellZoneId::LeftSidebar => self.bg_surface,
            ShellZoneId::MainWorkspace => self.bg_canvas,
            ShellZoneId::RightInspector => self.bg_surface,
            ShellZoneId::BottomPanel => self.bg_surface,
            ShellZoneId::StatusBar => self.bg_raised,
            ShellZoneId::TransientOverlay => self.bg_overlay,
        }
    }
}

#[derive(Debug, Clone)]
struct ShellRenderStyle {
    tokens: ShellRenderTokens,
    stroke_default: u32,
    stroke_focus: u32,
    space_2: u32,
    space_3: u32,
    space_4: u32,
    space_6: u32,
    status_warning: ColorRgba,
    status_warning_border: ColorRgba,
    status_warning_fill: ColorRgba,
    status_success: ColorRgba,
    status_success_border: ColorRgba,
    status_success_fill: ColorRgba,
}

impl ShellRenderStyle {
    fn load(registry: &TokenRegistry) -> Result<Self, TokenRegistryError> {
        Ok(Self {
            tokens: ShellRenderTokens::load(registry)?,
            stroke_default: registry.require_stroke_px("stroke.border.default")?,
            stroke_focus: registry.require_stroke_px("stroke.focus.ring")?,
            space_2: registry.require_space_px("space.2")?,
            space_3: registry.require_space_px("space.3")?,
            space_4: registry.require_space_px("space.4")?,
            space_6: registry.require_space_px("space.6")?,
            status_warning: registry.require_color("status.warning")?,
            status_warning_border: registry.require_color("status.warning.border")?,
            status_warning_fill: registry.require_color("status.warning.fill")?,
            status_success: registry.require_color("status.success")?,
            status_success_border: registry.require_color("status.success.border")?,
            status_success_fill: registry.require_color("status.success.fill")?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct ShellAppearanceState {
    theme: ThemeClass,
}

impl Default for ShellAppearanceState {
    fn default() -> Self {
        Self {
            theme: ThemeClass::DarkReference,
        }
    }
}

impl ShellAppearanceState {
    const fn theme(self) -> ThemeClass {
        self.theme
    }

    fn toggle_light_dark(&mut self) {
        self.theme = match self.theme {
            ThemeClass::DarkReference => ThemeClass::LightParity,
            ThemeClass::LightParity => ThemeClass::DarkReference,
            ThemeClass::HighContrastDark => ThemeClass::HighContrastLight,
            ThemeClass::HighContrastLight => ThemeClass::HighContrastDark,
        };
    }

    fn toggle_high_contrast(&mut self) {
        self.theme = match self.theme {
            ThemeClass::DarkReference => ThemeClass::HighContrastDark,
            ThemeClass::LightParity => ThemeClass::HighContrastLight,
            ThemeClass::HighContrastDark => ThemeClass::DarkReference,
            ThemeClass::HighContrastLight => ThemeClass::LightParity,
        };
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let registry = seeded_registry();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title(window_title(None, None))
            .with_inner_size(LogicalSize::new(1920.0, 1080.0))
            .build(&event_loop)?,
    );

    let context = Context::new(window.clone())?;
    let mut surface = Surface::new(&context, window.clone())?;

    let mut frame = {
        let logical = window.inner_size().to_logical::<u32>(window.scale_factor());
        DesktopFrame::new(logical.width, logical.height)
    };
    let mut held_modifiers = HeldModifiers::default();
    let mut palette = CommandPaletteState::new(registry);
    let mut start_center = StartCenterState::new();
    let mut overlay: Option<ShellOverlayState> = None;
    let mut command_runtime = CommandRuntimeState::default();
    let mut keybinding_runtime = KeybindingRuntimeState::new(platform_class_for_shell());
    let mut enablement_runtime = CommandEnablementRuntimeState::default();
    let mut clipboard = ClipboardState::new();
    let mut appearance = ShellAppearanceState::default();

    window.request_redraw();

    event_loop.run(move |event, elwt| match event {
        Event::AboutToWait => {
            let now = Instant::now();
            if palette.tick(registry, &keybinding_runtime.shortcuts_by_command_id, now) {
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette.selected_entry(registry),
                ));
                window.request_redraw();
            }
            if let Some(deadline) = palette.next_wake_deadline(now) {
                elwt.set_control_flow(ControlFlow::WaitUntil(deadline));
            } else {
                elwt.set_control_flow(ControlFlow::Wait);
            }
        }
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::Resized(_) => {
                relayout_and_redraw(&window, &mut surface, &mut frame);
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                relayout_and_redraw(&window, &mut surface, &mut frame);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let before_modifiers = held_modifiers;
                held_modifiers.update_from_key_event(&event);
                let modifiers_changed = before_modifiers != held_modifiers;
                if handle_key_event(
                    &window,
                    registry,
                    &mut frame,
                    &mut palette,
                    &mut start_center,
                    &mut overlay,
                    &mut command_runtime,
                    &mut keybinding_runtime,
                    &mut enablement_runtime,
                    &mut clipboard,
                    &mut appearance,
                    &held_modifiers,
                    event,
                ) || (palette.is_open() && modifiers_changed)
                {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = draw(
                    &window,
                    &mut surface,
                    registry,
                    &frame,
                    &palette,
                    &start_center,
                    overlay.as_ref(),
                    &command_runtime,
                    &keybinding_runtime,
                    &enablement_runtime,
                    &appearance,
                    &held_modifiers,
                ) {
                    eprintln!("aureline_shell: draw failed: {err}");
                    elwt.exit();
                }
            }
            _ => {}
        },
        _ => {}
    })?;
    Ok(())
}

fn window_title(
    focused: Option<ShellZoneId>,
    palette_selected: Option<&CommandRegistryEntryRecord>,
) -> String {
    let identity = build_info::build_identity();
    let focus_suffix = focused
        .map(|z| format!(" — focus: {}", z.name()))
        .unwrap_or_default();
    let palette_suffix = palette_selected
        .map(|entry| format!(" — cmd: {}", entry.command_id()))
        .unwrap_or_default();
    format!(
        "Aureline Shell{}{}{}",
        focus_suffix,
        palette_suffix,
        format!(" ({})", identity.commit_short)
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DispatchOrigin {
    StartCenter,
    CommandPalette,
    KeybindingChord,
}

impl DispatchOrigin {
    const fn issuing_surface(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::CommandPalette => "command_palette",
            Self::KeybindingChord => "keybinding_chord",
        }
    }
}

#[derive(Debug, Clone)]
struct RecordedCommandInvocation {
    session_packet: InvocationSessionPacketRecord,
    result_packet: CommandResultPacketRecord,
}

#[derive(Debug, Default)]
struct CommandRuntimeState {
    records: Vec<RecordedCommandInvocation>,
    last_command_label: Option<String>,
}

impl CommandRuntimeState {
    fn record(&mut self, invocation: RecordedCommandInvocation) {
        self.last_command_label = Some(format!(
            "{} — {}",
            invocation.result_packet.result.outcome_code,
            invocation.result_packet.invocation.canonical_command_id
        ));
        self.write_packets(&invocation);
        self.records.push(invocation);
        if self.records.len() > 64 {
            self.records.drain(0..(self.records.len() - 64));
        }
    }

    fn note_non_command_action(&mut self, label: impl Into<String>) {
        self.last_command_label = Some(label.into());
    }

    fn recent_lines(&self, limit: usize) -> Vec<String> {
        self.records
            .iter()
            .rev()
            .take(limit)
            .map(|row| {
                format!(
                    "{}  {}  ({})",
                    row.result_packet.result.outcome_code,
                    row.result_packet.invocation.canonical_command_id,
                    row.result_packet.invocation.issuing_surface
                )
            })
            .collect()
    }

    fn packet_root_dir() -> std::path::PathBuf {
        std::path::PathBuf::from(".logs").join("command_packets")
    }

    fn sanitize_filename(value: &str) -> String {
        value
            .chars()
            .map(|ch| match ch {
                ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
                other => other,
            })
            .collect()
    }

    fn write_packets(&self, invocation: &RecordedCommandInvocation) {
        let root = Self::packet_root_dir();
        if std::fs::create_dir_all(&root).is_err() {
            return;
        }

        let session_name = format!(
            "{}.invocation.json",
            Self::sanitize_filename(&invocation.session_packet.invocation_session_id)
        );
        if let Ok(json) = invocation.session_packet.to_pretty_json() {
            let _ = std::fs::write(root.join(session_name), json);
        }

        let result_name = format!(
            "{}.result.json",
            Self::sanitize_filename(&invocation.result_packet.result_packet_id)
        );
        if let Ok(json) = invocation.result_packet.to_pretty_json() {
            let _ = std::fs::write(root.join(result_name), json);
        }
    }
}

#[derive(Default)]
struct ClipboardState {
    clipboard: Option<Clipboard>,
}

impl ClipboardState {
    fn new() -> Self {
        Self {
            clipboard: Clipboard::new().ok(),
        }
    }

    fn set_text(&mut self, text: &str) -> Result<(), String> {
        if self.clipboard.is_none() {
            self.clipboard = Clipboard::new().ok();
        }
        let Some(clipboard) = self.clipboard.as_mut() else {
            return Err("clipboard unavailable".to_string());
        };
        clipboard
            .set_text(text.to_string())
            .map_err(|err| err.to_string())
    }
}

#[derive(Debug, Clone)]
struct CommandEnablementRuntimeState {
    workspace_trust_state: String,
    execution_context_available: bool,
    provider_linked: Option<bool>,
    credential_available: Option<bool>,
    policy_disabled: bool,
    policy_blocked_in_context: bool,
}

impl Default for CommandEnablementRuntimeState {
    fn default() -> Self {
        Self {
            workspace_trust_state: "trusted".to_string(),
            execution_context_available: true,
            provider_linked: None,
            credential_available: None,
            policy_disabled: false,
            policy_blocked_in_context: false,
        }
    }
}

impl CommandEnablementRuntimeState {
    fn toggle_trust_state(&mut self) {
        self.workspace_trust_state = if self.workspace_trust_state == "trusted" {
            "restricted".to_string()
        } else {
            "trusted".to_string()
        };
    }

    fn toggle_execution_context(&mut self) {
        self.execution_context_available = !self.execution_context_available;
    }

    fn toggle_policy_blocked(&mut self) {
        self.policy_blocked_in_context = !self.policy_blocked_in_context;
    }
}

fn alias_used_for(entry: &CommandRegistryEntryRecord, origin: DispatchOrigin) -> AliasUsedBlock {
    match origin {
        DispatchOrigin::StartCenter | DispatchOrigin::CommandPalette => AliasUsedBlock {
            alias_kind: "canonical".to_string(),
            alias_id: None,
            alias_state: "not_applicable".to_string(),
            resolves_to_canonical_command_id: entry.descriptor.command_id.clone(),
            migration_trace_ref: None,
            support_window_ref: None,
        },
        DispatchOrigin::KeybindingChord => {
            let key_alias = entry
                .descriptor
                .aliases
                .iter()
                .find(|alias| alias.alias_kind == "keybinding_target")
                .map(|alias| alias.alias_id.clone());
            match key_alias {
                Some(alias_id) => AliasUsedBlock {
                    alias_kind: "keybinding_target".to_string(),
                    alias_id: Some(alias_id),
                    alias_state: "active".to_string(),
                    resolves_to_canonical_command_id: entry.descriptor.command_id.clone(),
                    migration_trace_ref: None,
                    support_window_ref: None,
                },
                None => AliasUsedBlock {
                    alias_kind: "canonical".to_string(),
                    alias_id: None,
                    alias_state: "not_applicable".to_string(),
                    resolves_to_canonical_command_id: entry.descriptor.command_id.clone(),
                    migration_trace_ref: None,
                    support_window_ref: None,
                },
            }
        }
    }
}

fn make_session(
    frame: &DesktopFrame,
    entry: &CommandRegistryEntryRecord,
    origin: DispatchOrigin,
    execution_intent: &str,
    workspace_trust_state: &str,
    argument_provenance_map: Vec<ArgumentProvenanceEntry>,
    preview_shown: bool,
    preview_record_ref: Option<String>,
    approval_state: &str,
    approval_ticket_ref: Option<String>,
) -> CommandInvocationSession {
    let canonical_verb = entry.descriptor.canonical_verb.clone();
    let basis_snapshot_ref = mint_basis_snapshot_ref(&canonical_verb);
    let focused = Some(format!("shell-zone:{}", frame.focused_zone().name()));

    let enablement = EnablementDecisionBlock {
        decision_class: EnablementDecisionClass::Enabled,
        disabled_reason_code: None,
        repair_hook_ref: None,
    };

    CommandInvocationSession {
        invocation_session_id: mint_invocation_session_id(&canonical_verb),
        canonical_command_id: entry.descriptor.command_id.clone(),
        command_revision_ref: entry.descriptor.command_revision_ref.clone(),
        canonical_verb,
        issuing_surface: origin.issuing_surface().to_string(),
        authority_class: "user_initiated_local".to_string(),
        alias_used: alias_used_for(entry, origin),
        argument_provenance_map,
        context_snapshot: InvocationContextSnapshot {
            focused_entity_ref: focused.clone(),
            selection_ref: None,
            workspace_trust_state: workspace_trust_state.to_string(),
            execution_context_id: entry.descriptor.policy_context.execution_context_id.clone(),
            scope_filter_class_ref: None,
            basis_snapshot_ref: basis_snapshot_ref.clone(),
        },
        context_refs: ContextRefsBlock {
            focused_entity_ref: focused,
            selection_ref: None,
            workspace_ref: None,
            workspace_trust_state: workspace_trust_state.to_string(),
            execution_context_id: entry.descriptor.policy_context.execution_context_id.clone(),
            scope_filter_class_ref: None,
            basis_snapshot_ref,
            context_object_refs: vec![format!(
                "policy-epoch:{}",
                entry.descriptor.policy_context.policy_epoch
            )],
        },
        enablement_decision: enablement,
        preview_posture: aureline_commands::invocation::PreviewPostureBlock {
            preview_class_declared: entry.descriptor.preview_class.clone(),
            preview_shown,
            preview_record_ref,
        },
        approval_posture: ApprovalPostureBlock {
            approval_posture_class_declared: entry.descriptor.approval_posture_class.clone(),
            approval_state: approval_state.to_string(),
            approval_ticket_ref: approval_ticket_ref.map(|v| v.to_string()),
        },
        execution_intent: execution_intent.to_string(),
        policy_context: entry.descriptor.policy_context.clone(),
        redaction_class: entry.descriptor.redaction_class.clone(),
    }
}

fn dispatch_command_id(
    command_runtime: &mut CommandRuntimeState,
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    palette: &mut CommandPaletteState,
    overlay: &mut Option<ShellOverlayState>,
    command_id: &str,
    origin: DispatchOrigin,
    enablement_runtime: &CommandEnablementRuntimeState,
) -> bool {
    dispatch_command_id_with_arguments(
        command_runtime,
        registry,
        frame,
        palette,
        overlay,
        command_id,
        origin,
        enablement_runtime,
        None,
    )
}

fn dispatch_command_id_with_arguments(
    command_runtime: &mut CommandRuntimeState,
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    palette: &mut CommandPaletteState,
    overlay: &mut Option<ShellOverlayState>,
    command_id: &str,
    origin: DispatchOrigin,
    enablement_runtime: &CommandEnablementRuntimeState,
    argument_provenance_map_override: Option<Vec<ArgumentProvenanceEntry>>,
) -> bool {
    let Some(entry) = registry.get(command_id).cloned() else {
        return false;
    };
    dispatch_registry_entry(
        command_runtime,
        registry,
        frame,
        palette,
        overlay,
        &entry,
        origin,
        enablement_runtime,
        argument_provenance_map_override,
    )
}

fn dispatch_registry_entry(
    command_runtime: &mut CommandRuntimeState,
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    palette: &mut CommandPaletteState,
    overlay: &mut Option<ShellOverlayState>,
    entry: &CommandRegistryEntryRecord,
    origin: DispatchOrigin,
    enablement_runtime: &CommandEnablementRuntimeState,
    argument_provenance_map_override: Option<Vec<ArgumentProvenanceEntry>>,
) -> bool {
    let preview_record_ref: Option<String> = None;
    let preview_shown = false;
    let mut approval_state = "not_required".to_string();
    let mut approval_ticket_ref: Option<String> = None;

    if entry.descriptor.approval_posture_class != "no_approval_required" {
        approval_state = "approval_pending".to_string();
        approval_ticket_ref = Some(mint_approval_ticket_ref(&entry.descriptor.canonical_verb));
    }

    let execution_intent = match entry.descriptor.command_id.as_str() {
        "cmd:workspace.open_folder" => "apply_direct_trusted_path",
        "cmd:workspace.import_profile" => "apply_after_preview",
        _ => "query_only_no_mutation",
    };

    let argument_provenance_map =
        argument_provenance_map_override.unwrap_or_else(|| argument_provenance_map_for(entry));

    let mut session = make_session(
        frame,
        entry,
        origin,
        execution_intent,
        enablement_runtime.workspace_trust_state.as_str(),
        argument_provenance_map,
        preview_shown,
        preview_record_ref.clone(),
        &approval_state,
        approval_ticket_ref.clone(),
    );

    let enablement_context = CommandEnablementContext {
        client_scope: "desktop_product".to_string(),
        workspace_trust_state: enablement_runtime.workspace_trust_state.clone(),
        execution_context_available: enablement_runtime.execution_context_available,
        provider_linked: enablement_runtime.provider_linked,
        credential_available: enablement_runtime.credential_available,
        policy_disabled: enablement_runtime.policy_disabled,
        policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
        argument_provenance_map: session.argument_provenance_map.clone(),
    };
    let preflight = entry.preflight(&enablement_context);
    let enablement_snapshot = preflight.enablement_snapshot.clone();
    session.enablement_decision = EnablementDecisionBlock {
        decision_class: enablement_snapshot.decision_class,
        disabled_reason_code: enablement_snapshot.disabled_reason_code,
        repair_hook_ref: enablement_snapshot.repair_hook_ref,
    };

    let review_runtime = CommandReviewRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
        execution_context_available: enablement_runtime.execution_context_available,
        provider_linked: enablement_runtime.provider_linked,
        credential_available: enablement_runtime.credential_available,
        policy_disabled: enablement_runtime.policy_disabled,
        policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
    };

    if matches!(
        preflight.decision_class,
        PreflightDecisionClass::BlockedByPolicy | PreflightDecisionClass::DisabledWithReason
    ) {
        let denied_code = session
            .enablement_decision
            .disabled_reason_code
            .unwrap_or(DisabledReasonCode::PolicyBlockedInContext);
        let invocation = invocation_and_result_denied(&session, denied_code);
        command_runtime.record(invocation);

        let record = materialize_command_diagnostics_sheet_record(entry, review_runtime);
        write_diagnostics_sheet_log(&record);
        *overlay = Some(ShellOverlayState::command_diagnostics(
            frame.focused_zone(),
            frame.focused_editor_group(),
            record,
        ));
        frame.focus_zone(ShellZoneId::TransientOverlay);
        return true;
    }

    palette.note_command_invoked(entry.command_id());

    if matches!(
        preflight.decision_class,
        PreflightDecisionClass::PreviewRequired | PreflightDecisionClass::ApprovalRequired
    ) {
        session.preview_posture.preview_shown = true;
        if session.preview_posture.preview_record_ref.is_none() {
            session.preview_posture.preview_record_ref =
                Some(mint_preview_record_ref(&entry.descriptor.canonical_verb));
        }
        let record =
            materialize_command_invocation_preview_sheet_record(entry, &session, review_runtime);
        write_invocation_preview_sheet_log(&record);
        *overlay = Some(ShellOverlayState::invocation_preview(
            frame.focused_zone(),
            frame.focused_editor_group(),
            entry.clone(),
            record,
        ));
        frame.focus_zone(ShellZoneId::TransientOverlay);
        return true;
    }

    match entry.descriptor.command_id.as_str() {
        "cmd:command_palette.open" => {
            let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            palette.open(registry, cwd);
            let invocation = invocation_and_result_simple_success(&session, "succeeded");
            command_runtime.record(invocation);
            true
        }
        "cmd:labs.open_command_trace" => {
            let lines = command_runtime.recent_lines(18);
            *overlay = Some(ShellOverlayState::command_trace(
                frame.focused_zone(),
                frame.focused_editor_group(),
                lines,
            ));
            frame.focus_zone(ShellZoneId::TransientOverlay);
            let invocation = invocation_and_result_simple_success(&session, "succeeded");
            command_runtime.record(invocation);
            true
        }
        "cmd:workspace.open_folder" => {
            let invocation = invocation_and_result_open_folder_succeeded(&session);
            command_runtime.record(invocation);
            frame.open_placeholder_tab();
            true
        }
        "cmd:workspace.import_profile" => {
            let invocation = invocation_and_result_unimplemented(&session);
            command_runtime.record(invocation);
            true
        }
        _ => {
            let invocation = invocation_and_result_unimplemented(&session);
            command_runtime.record(invocation);
            true
        }
    }
}

fn invocation_and_result_denied(
    session: &CommandInvocationSession,
    disabled_reason_code: DisabledReasonCode,
) -> RecordedCommandInvocation {
    let outcome = InvocationOutcomeBlock {
        outcome_class: "denied_by_enablement".to_string(),
        disabled_reason_code: Some(disabled_reason_code),
        warnings_summary_refs: Vec::new(),
        partially_applied_artifact_refs: Vec::new(),
        unapplied_artifact_refs: Vec::new(),
    };
    let session_packet = session.invocation_session_packet(outcome, Vec::new(), Vec::new());

    let result = ResultBodyBlock {
        outcome_code: "denied_by_enablement".to_string(),
        warning_codes: Vec::new(),
        error_codes: vec![disabled_reason_code.as_str().to_string()],
        created_artifact_refs: Vec::new(),
        notification_refs: Vec::new(),
        activity_refs: Vec::new(),
        rollback_handle_ref: RollbackHandleRefBlock {
            rollback_handle_posture: "not_applicable_no_mutation".to_string(),
            rollback_handle_id: None,
        },
        checkpoint_refs: Vec::new(),
        evidence_refs: Vec::new(),
        export_posture: ExportPostureBlock {
            export_posture_class: "exportable_with_redaction".to_string(),
            redaction_class: session.redaction_class.clone(),
            export_review_ref: None,
            portable_profile_allowed: true,
            support_bundle_allowed: true,
        },
    };

    let result_packet = session.command_result_packet(
        session.mint_attempt_id(),
        session.mint_result_packet_id(),
        result,
        format!(
            "parity-expectation:{}:result-contract:01",
            session.canonical_verb
        ),
        NoBypassGuards::strict(),
    );

    RecordedCommandInvocation {
        session_packet,
        result_packet,
    }
}

fn invocation_and_result_simple_success(
    session: &CommandInvocationSession,
    outcome_code: &str,
) -> RecordedCommandInvocation {
    let outcome = InvocationOutcomeBlock {
        outcome_class: outcome_code.to_string(),
        disabled_reason_code: None,
        warnings_summary_refs: Vec::new(),
        partially_applied_artifact_refs: Vec::new(),
        unapplied_artifact_refs: Vec::new(),
    };
    let session_packet = session.invocation_session_packet(outcome, Vec::new(), Vec::new());

    let result = ResultBodyBlock {
        outcome_code: outcome_code.to_string(),
        warning_codes: Vec::new(),
        error_codes: Vec::new(),
        created_artifact_refs: Vec::new(),
        notification_refs: Vec::new(),
        activity_refs: Vec::new(),
        rollback_handle_ref: RollbackHandleRefBlock {
            rollback_handle_posture: "not_applicable_no_mutation".to_string(),
            rollback_handle_id: None,
        },
        checkpoint_refs: Vec::new(),
        evidence_refs: Vec::new(),
        export_posture: ExportPostureBlock {
            export_posture_class: "exportable_metadata_default".to_string(),
            redaction_class: session.redaction_class.clone(),
            export_review_ref: None,
            portable_profile_allowed: true,
            support_bundle_allowed: true,
        },
    };

    let result_packet = session.command_result_packet(
        session.mint_attempt_id(),
        session.mint_result_packet_id(),
        result,
        format!(
            "parity-expectation:{}:result-contract:01",
            session.canonical_verb
        ),
        NoBypassGuards::strict(),
    );

    RecordedCommandInvocation {
        session_packet,
        result_packet,
    }
}

fn invocation_and_result_open_folder_succeeded(
    session: &CommandInvocationSession,
) -> RecordedCommandInvocation {
    let journal_entry_ref = session
        .invocation_session_id
        .replacen("inv:", "journal-entry:", 1);
    let audit_event_ref = session.invocation_session_id.replacen("inv:", "audit:", 1);

    let outcome = InvocationOutcomeBlock {
        outcome_class: "succeeded".to_string(),
        disabled_reason_code: None,
        warnings_summary_refs: Vec::new(),
        partially_applied_artifact_refs: Vec::new(),
        unapplied_artifact_refs: Vec::new(),
    };
    let session_packet = session.invocation_session_packet(
        outcome,
        vec![InvocationCreatedArtifactRefEntry {
            result_contract_class: "journal_entry_appended_ref".to_string(),
            artifact_ref: journal_entry_ref.clone(),
        }],
        vec![
            EvidenceRefEntry {
                evidence_ref_class: "mutation_journal_entry_ref".to_string(),
                evidence_id: journal_entry_ref.clone(),
            },
            EvidenceRefEntry {
                evidence_ref_class: "audit_event_ref".to_string(),
                evidence_id: audit_event_ref.clone(),
            },
        ],
    );

    let result = ResultBodyBlock {
        outcome_code: "succeeded".to_string(),
        warning_codes: Vec::new(),
        error_codes: Vec::new(),
        created_artifact_refs: vec![ArtifactRefEntry {
            result_contract_class: "journal_entry_appended_ref".to_string(),
            artifact_ref: journal_entry_ref.clone(),
            artifact_role: "side_effect_record".to_string(),
        }],
        notification_refs: Vec::new(),
        activity_refs: Vec::new(),
        rollback_handle_ref: RollbackHandleRefBlock {
            rollback_handle_posture: "not_reversible_by_contract".to_string(),
            rollback_handle_id: None,
        },
        checkpoint_refs: Vec::new(),
        evidence_refs: vec![EvidenceRefEntry {
            evidence_ref_class: "mutation_journal_entry_ref".to_string(),
            evidence_id: journal_entry_ref,
        }],
        export_posture: ExportPostureBlock {
            export_posture_class: "exportable_with_redaction".to_string(),
            redaction_class: session.redaction_class.clone(),
            export_review_ref: None,
            portable_profile_allowed: true,
            support_bundle_allowed: true,
        },
    };

    let result_packet = session.command_result_packet(
        session.mint_attempt_id(),
        session.mint_result_packet_id(),
        result,
        format!(
            "parity-expectation:{}:result-contract:01",
            session.canonical_verb
        ),
        NoBypassGuards::strict(),
    );

    RecordedCommandInvocation {
        session_packet,
        result_packet,
    }
}

fn invocation_and_result_unimplemented(
    session: &CommandInvocationSession,
) -> RecordedCommandInvocation {
    let outcome = InvocationOutcomeBlock {
        outcome_class: "failed_with_typed_error".to_string(),
        disabled_reason_code: None,
        warnings_summary_refs: Vec::new(),
        partially_applied_artifact_refs: Vec::new(),
        unapplied_artifact_refs: Vec::new(),
    };
    let session_packet = session.invocation_session_packet(outcome, Vec::new(), Vec::new());

    let result = ResultBodyBlock {
        outcome_code: "failed_with_typed_error".to_string(),
        warning_codes: Vec::new(),
        error_codes: vec!["typed_runtime_failure".to_string()],
        created_artifact_refs: Vec::new(),
        notification_refs: Vec::new(),
        activity_refs: Vec::new(),
        rollback_handle_ref: RollbackHandleRefBlock {
            rollback_handle_posture: "not_reversible_by_contract".to_string(),
            rollback_handle_id: None,
        },
        checkpoint_refs: Vec::new(),
        evidence_refs: Vec::new(),
        export_posture: ExportPostureBlock {
            export_posture_class: "exportable_with_redaction".to_string(),
            redaction_class: session.redaction_class.clone(),
            export_review_ref: None,
            portable_profile_allowed: true,
            support_bundle_allowed: true,
        },
    };

    let result_packet = session.command_result_packet(
        session.mint_attempt_id(),
        session.mint_result_packet_id(),
        result,
        format!(
            "parity-expectation:{}:result-contract:01",
            session.canonical_verb
        ),
        NoBypassGuards::strict(),
    );

    RecordedCommandInvocation {
        session_packet,
        result_packet,
    }
}

fn finalize_command_overlay_decision(
    command_runtime: &mut CommandRuntimeState,
    _registry: &CommandRegistry,
    decision: CommandOverlayDecision,
) {
    match decision {
        CommandOverlayDecision::PreviewApproved { entry, session } => {
            let invocation = match entry.descriptor.command_id.as_str() {
                "cmd:workspace.import_profile" => {
                    invocation_and_result_import_profile_succeeded(&session)
                }
                _ => invocation_and_result_unimplemented(&session),
            };
            command_runtime.record(invocation);
        }
        CommandOverlayDecision::PreviewCancelled { entry, session } => {
            let invocation = match entry.descriptor.command_id.as_str() {
                "cmd:workspace.import_profile" => {
                    invocation_and_result_import_profile_cancelled(&session)
                }
                _ => invocation_and_result_unimplemented(&session),
            };
            command_runtime.record(invocation);
        }
    }
}

fn invocation_and_result_import_profile_cancelled(
    session: &CommandInvocationSession,
) -> RecordedCommandInvocation {
    let preview_ref = session
        .preview_posture
        .preview_record_ref
        .clone()
        .unwrap_or_else(|| mint_preview_record_ref(&session.canonical_verb));

    let outcome = InvocationOutcomeBlock {
        outcome_class: "cancelled_by_user".to_string(),
        disabled_reason_code: None,
        warnings_summary_refs: Vec::new(),
        partially_applied_artifact_refs: Vec::new(),
        unapplied_artifact_refs: Vec::new(),
    };

    let session_packet = session.invocation_session_packet(
        outcome,
        vec![InvocationCreatedArtifactRefEntry {
            result_contract_class: "preview_record_emitted_ref".to_string(),
            artifact_ref: preview_ref.clone(),
        }],
        vec![EvidenceRefEntry {
            evidence_ref_class: "preview_record_ref".to_string(),
            evidence_id: preview_ref.clone(),
        }],
    );

    let result = ResultBodyBlock {
        outcome_code: "cancelled_by_user".to_string(),
        warning_codes: Vec::new(),
        error_codes: Vec::new(),
        created_artifact_refs: vec![ArtifactRefEntry {
            result_contract_class: "preview_record_emitted_ref".to_string(),
            artifact_ref: preview_ref.clone(),
            artifact_role: "preview_record".to_string(),
        }],
        notification_refs: Vec::new(),
        activity_refs: Vec::new(),
        rollback_handle_ref: RollbackHandleRefBlock {
            rollback_handle_posture: "not_applicable_no_mutation".to_string(),
            rollback_handle_id: None,
        },
        checkpoint_refs: Vec::new(),
        evidence_refs: vec![EvidenceRefEntry {
            evidence_ref_class: "preview_record_ref".to_string(),
            evidence_id: preview_ref,
        }],
        export_posture: ExportPostureBlock {
            export_posture_class: "exportable_with_redaction".to_string(),
            redaction_class: session.redaction_class.clone(),
            export_review_ref: None,
            portable_profile_allowed: true,
            support_bundle_allowed: true,
        },
    };

    let result_packet = session.command_result_packet(
        session.mint_attempt_id(),
        session.mint_result_packet_id(),
        result,
        "parity-expectation:workspace.import_profile:result-contract:01".to_string(),
        NoBypassGuards::strict(),
    );

    RecordedCommandInvocation {
        session_packet,
        result_packet,
    }
}

fn invocation_and_result_import_profile_succeeded(
    session: &CommandInvocationSession,
) -> RecordedCommandInvocation {
    let preview_ref = session
        .preview_posture
        .preview_record_ref
        .clone()
        .unwrap_or_else(|| mint_preview_record_ref(&session.canonical_verb));
    let journal_entry_ref = session
        .invocation_session_id
        .replacen("inv:", "journal-entry:", 1);
    let rollback_handle_id = session
        .invocation_session_id
        .replacen("inv:", "rollback-handle:", 1);
    let audit_event_ref = session.invocation_session_id.replacen("inv:", "audit:", 1);

    let outcome = InvocationOutcomeBlock {
        outcome_class: "succeeded".to_string(),
        disabled_reason_code: None,
        warnings_summary_refs: Vec::new(),
        partially_applied_artifact_refs: Vec::new(),
        unapplied_artifact_refs: Vec::new(),
    };

    let session_packet = session.invocation_session_packet(
        outcome,
        vec![
            InvocationCreatedArtifactRefEntry {
                result_contract_class: "preview_record_emitted_ref".to_string(),
                artifact_ref: preview_ref.clone(),
            },
            InvocationCreatedArtifactRefEntry {
                result_contract_class: "journal_entry_appended_ref".to_string(),
                artifact_ref: journal_entry_ref.clone(),
            },
            InvocationCreatedArtifactRefEntry {
                result_contract_class: "rollback_ticket_emitted_ref".to_string(),
                artifact_ref: rollback_handle_id.clone(),
            },
        ],
        vec![
            EvidenceRefEntry {
                evidence_ref_class: "preview_record_ref".to_string(),
                evidence_id: preview_ref.clone(),
            },
            EvidenceRefEntry {
                evidence_ref_class: "mutation_journal_entry_ref".to_string(),
                evidence_id: journal_entry_ref.clone(),
            },
            EvidenceRefEntry {
                evidence_ref_class: "rollback_ticket_ref".to_string(),
                evidence_id: rollback_handle_id.clone(),
            },
            EvidenceRefEntry {
                evidence_ref_class: "audit_event_ref".to_string(),
                evidence_id: audit_event_ref.clone(),
            },
        ],
    );

    let result = ResultBodyBlock {
        outcome_code: "succeeded".to_string(),
        warning_codes: Vec::new(),
        error_codes: Vec::new(),
        created_artifact_refs: vec![
            ArtifactRefEntry {
                result_contract_class: "preview_record_emitted_ref".to_string(),
                artifact_ref: preview_ref.clone(),
                artifact_role: "preview_record".to_string(),
            },
            ArtifactRefEntry {
                result_contract_class: "journal_entry_appended_ref".to_string(),
                artifact_ref: journal_entry_ref.clone(),
                artifact_role: "side_effect_record".to_string(),
            },
            ArtifactRefEntry {
                result_contract_class: "rollback_ticket_emitted_ref".to_string(),
                artifact_ref: rollback_handle_id.clone(),
                artifact_role: "rollback_ticket".to_string(),
            },
        ],
        notification_refs: Vec::new(),
        activity_refs: Vec::new(),
        rollback_handle_ref: RollbackHandleRefBlock {
            rollback_handle_posture: "handle_available".to_string(),
            rollback_handle_id: Some(rollback_handle_id.clone()),
        },
        checkpoint_refs: Vec::new(),
        evidence_refs: vec![
            EvidenceRefEntry {
                evidence_ref_class: "preview_record_ref".to_string(),
                evidence_id: preview_ref,
            },
            EvidenceRefEntry {
                evidence_ref_class: "mutation_journal_entry_ref".to_string(),
                evidence_id: journal_entry_ref,
            },
            EvidenceRefEntry {
                evidence_ref_class: "rollback_ticket_ref".to_string(),
                evidence_id: rollback_handle_id,
            },
        ],
        export_posture: ExportPostureBlock {
            export_posture_class: "exportable_with_redaction".to_string(),
            redaction_class: session.redaction_class.clone(),
            export_review_ref: None,
            portable_profile_allowed: true,
            support_bundle_allowed: true,
        },
    };

    let result_packet = session.command_result_packet(
        session.mint_attempt_id(),
        session.mint_result_packet_id(),
        result,
        "parity-expectation:workspace.import_profile:result-contract:01".to_string(),
        NoBypassGuards::strict(),
    );

    RecordedCommandInvocation {
        session_packet,
        result_packet,
    }
}

fn handle_key_event(
    window: &winit::window::Window,
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    palette: &mut CommandPaletteState,
    start_center: &mut StartCenterState,
    overlay: &mut Option<ShellOverlayState>,
    command_runtime: &mut CommandRuntimeState,
    keybinding_runtime: &mut KeybindingRuntimeState,
    enablement_runtime: &mut CommandEnablementRuntimeState,
    clipboard: &mut ClipboardState,
    appearance: &mut ShellAppearanceState,
    modifiers: &HeldModifiers,
    event: KeyEvent,
) -> bool {
    if event.state != ElementState::Pressed || event.repeat {
        return false;
    }

    let PhysicalKey::Code(code) = event.physical_key else {
        return false;
    };

    if palette.is_open() {
        return match code {
            KeyCode::KeyC if modifiers.ctrl_or_logo() => {
                let runtime = PalettePreviewRuntimeInputs {
                    client_scope: "desktop_product",
                    workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
                    execution_context_available: enablement_runtime.execution_context_available,
                    provider_linked: enablement_runtime.provider_linked,
                    credential_available: enablement_runtime.credential_available,
                    policy_disabled: enablement_runtime.policy_disabled,
                    policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
                };
                let preview = materialize_palette_preview_record(
                    palette.selected_key(),
                    registry,
                    &keybinding_runtime.shortcuts_by_command_id,
                    runtime,
                );
                let PalettePreviewSelection::Command(command) = &preview.selection else {
                    command_runtime.note_non_command_action("copy: no command selected");
                    return true;
                };

                let preferred_intent = if modifiers.shift {
                    PaletteCopyIntent::CliSkeleton
                } else {
                    PaletteCopyIntent::CommandId
                };
                let payload = copy_payload_for(command, preferred_intent)
                    .or_else(|| copy_payload_for(command, PaletteCopyIntent::CommandId));
                let Some(payload) = payload else {
                    command_runtime.note_non_command_action(format!(
                        "copy: unavailable — {}",
                        command.command_id
                    ));
                    return true;
                };

                match clipboard.set_text(payload) {
                    Ok(()) => {
                        write_preview_log(&preview);
                        let label = match preferred_intent {
                            PaletteCopyIntent::CliSkeleton
                                if command.copy.cli_skeleton.is_some() =>
                            {
                                "copied cli skeleton"
                            }
                            _ => "copied command id",
                        };
                        command_runtime
                            .note_non_command_action(format!("{label} — {}", command.command_id));
                        true
                    }
                    Err(err) => {
                        command_runtime.note_non_command_action(format!(
                            "copy failed — {} ({})",
                            command.command_id, err
                        ));
                        true
                    }
                }
            }
            KeyCode::KeyD if modifiers.ctrl_or_logo() => {
                let Some(entry) = palette.selected_entry(registry) else {
                    command_runtime.note_non_command_action("diagnostics: no command selected");
                    return true;
                };

                let runtime = CommandReviewRuntimeInputs {
                    client_scope: "desktop_product",
                    workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
                    execution_context_available: enablement_runtime.execution_context_available,
                    provider_linked: enablement_runtime.provider_linked,
                    credential_available: enablement_runtime.credential_available,
                    policy_disabled: enablement_runtime.policy_disabled,
                    policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
                };
                let record = materialize_command_diagnostics_sheet_record(entry, runtime);
                write_diagnostics_sheet_log(&record);

                palette.write_snapshot_log(registry, &keybinding_runtime.shortcuts_by_command_id);
                palette.close();

                *overlay = Some(ShellOverlayState::command_diagnostics(
                    frame.focused_zone(),
                    frame.focused_editor_group(),
                    record,
                ));
                frame.focus_zone(ShellZoneId::TransientOverlay);
                command_runtime
                    .note_non_command_action(format!("diagnostics — {}", entry.command_id()));
                window.set_title(&window_title(Some(frame.focused_zone()), None));
                true
            }
            KeyCode::Enter => {
                palette.write_snapshot_log(registry, &keybinding_runtime.shortcuts_by_command_id);
                let commit = palette.commit(registry);
                match commit {
                    Some(CommandPaletteCommit::CommandId(command_id)) => {
                        let changed = dispatch_command_id(
                            command_runtime,
                            registry,
                            frame,
                            palette,
                            overlay,
                            &command_id,
                            DispatchOrigin::CommandPalette,
                            enablement_runtime,
                        );
                        window.set_title(&window_title(Some(frame.focused_zone()), None));
                        changed
                    }
                    Some(CommandPaletteCommit::FilePath(relative_path)) => {
                        frame.open_placeholder_tab();
                        command_runtime
                            .note_non_command_action(format!("opened file — {}", relative_path));
                        window.set_title(&window_title(Some(frame.focused_zone()), None));
                        true
                    }
                    None => {
                        window.set_title(&window_title(Some(frame.focused_zone()), None));
                        true
                    }
                }
            }
            KeyCode::Escape => {
                palette.write_snapshot_log(registry, &keybinding_runtime.shortcuts_by_command_id);
                palette.close();
                window.set_title(&window_title(Some(frame.focused_zone()), None));
                true
            }
            KeyCode::ArrowDown => {
                let handled = palette.handle_arrow_down();
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette.selected_entry(registry),
                ));
                handled
            }
            KeyCode::ArrowUp => {
                let handled = palette.handle_arrow_up();
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette.selected_entry(registry),
                ));
                handled
            }
            KeyCode::Backspace => {
                let handled =
                    palette.handle_backspace(registry, &keybinding_runtime.shortcuts_by_command_id);
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette.selected_entry(registry),
                ));
                handled
            }
            _ => {
                if !modifiers.ctrl_or_logo() {
                    if let Some(text) = event.text.as_deref() {
                        let mut changed = false;
                        for ch in text.chars() {
                            changed |= palette.handle_text_input(
                                ch,
                                registry,
                                &keybinding_runtime.shortcuts_by_command_id,
                            );
                        }
                        if changed {
                            window.set_title(&window_title(
                                Some(frame.focused_zone()),
                                palette.selected_entry(registry),
                            ));
                            return true;
                        }
                    }
                }
                false
            }
        };
    }

    if let Some(state) = overlay.as_mut() {
        let outcome = state.handle_key(code, frame, keybinding_runtime);
        if let Some(decision) = outcome.command_decision {
            finalize_command_overlay_decision(command_runtime, registry, decision);
        }
        if outcome.handled {
            if state.closed {
                *overlay = None;
            }
            window.set_title(&window_title(Some(frame.focused_zone()), None));
            return true;
        }
        return false;
    }

    if let Some((sequence, inspection_scope)) =
        keybinding_sequence_and_scope_from_shell(code, modifiers, frame)
    {
        let packet = keybinding_runtime
            .resolver
            .resolve(&sequence, &inspection_scope);
        keybinding_runtime.record(packet.clone());

        if packet.sequence_state == SequenceResolutionState::Resolved
            && packet.winning_resolution.winner_kind == WinningResolutionKind::CommandCandidate
        {
            if let Some(candidate) = packet.winning_resolution.command_candidate.as_ref() {
                let changed = dispatch_command_id(
                    command_runtime,
                    registry,
                    frame,
                    palette,
                    overlay,
                    candidate.command.command_id.as_str(),
                    DispatchOrigin::KeybindingChord,
                    enablement_runtime,
                );
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette
                        .is_open()
                        .then(|| palette.selected_entry(registry))
                        .flatten(),
                ));
                return changed;
            }
        }
    }

    if frame.focused_zone() == ShellZoneId::MainWorkspace && focused_editor_group_is_empty(frame) {
        let runtime = StartCenterRuntimeInputs {
            client_scope: "desktop_product",
            workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
            execution_context_available: enablement_runtime.execution_context_available,
            provider_linked: enablement_runtime.provider_linked,
            credential_available: enablement_runtime.credential_available,
            policy_disabled: enablement_runtime.policy_disabled,
            policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
        };
        let rows = start_center_action_rows(registry, runtime);
        let row_count = rows.len();

        match code {
            KeyCode::ArrowDown => {
                start_center.select_next(row_count);
                window.set_title(&window_title(Some(frame.focused_zone()), None));
                return true;
            }
            KeyCode::ArrowUp => {
                start_center.select_prev(row_count);
                window.set_title(&window_title(Some(frame.focused_zone()), None));
                return true;
            }
            KeyCode::Enter => {
                let idx = start_center.selection().min(row_count.saturating_sub(1));
                let Some(row) = rows.get(idx) else {
                    return true;
                };
                let changed = dispatch_command_id_with_arguments(
                    command_runtime,
                    registry,
                    frame,
                    palette,
                    overlay,
                    row.command_id,
                    DispatchOrigin::StartCenter,
                    enablement_runtime,
                    Some(row.argument_provenance_map.clone()),
                );
                window.set_title(&window_title(Some(frame.focused_zone()), None));
                return changed;
            }
            _ => {}
        }
    }

    match code {
        KeyCode::Tab => {
            frame.focus_next();
            window.set_title(&window_title(Some(frame.focused_zone()), None));
            true
        }
        KeyCode::KeyO => {
            if modifiers.ctrl_or_logo() {
                frame.open_placeholder_tab();
                true
            } else {
                false
            }
        }
        KeyCode::Backslash => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                match frame.request_split_focused_editor_group() {
                    NewEditorGroupOutcome::Created { .. } => true,
                    NewEditorGroupOutcome::WouldViolateMinimum(violation) => {
                        *overlay = Some(ShellOverlayState::split_choice(
                            frame.focused_zone(),
                            frame.focused_editor_group(),
                            violation,
                        ));
                        frame.focus_zone(ShellZoneId::TransientOverlay);
                        true
                    }
                }
            } else if modifiers.ctrl_or_logo() {
                match frame.request_split_focused_editor_group() {
                    NewEditorGroupOutcome::Created { .. } => true,
                    NewEditorGroupOutcome::WouldViolateMinimum(violation) => {
                        *overlay = Some(ShellOverlayState::split_choice(
                            frame.focused_zone(),
                            frame.focused_editor_group(),
                            violation,
                        ));
                        frame.focus_zone(ShellZoneId::TransientOverlay);
                        true
                    }
                }
            } else {
                false
            }
        }
        KeyCode::KeyG => {
            if modifiers.ctrl_or_logo() {
                frame.focus_next_editor_group();
                window.set_title(&window_title(Some(frame.focused_zone()), None));
                true
            } else {
                false
            }
        }
        KeyCode::KeyW => {
            if modifiers.ctrl_or_logo() {
                frame.close_focused_editor_group()
            } else {
                false
            }
        }
        KeyCode::KeyI => {
            if modifiers.ctrl_or_logo() && frame.layout().right_inspector.is_none() {
                *overlay = Some(ShellOverlayState::inspector_sheet(
                    frame.focused_zone(),
                    frame.focused_editor_group(),
                ));
                frame.focus_zone(ShellZoneId::TransientOverlay);
                true
            } else {
                false
            }
        }
        KeyCode::KeyT => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                enablement_runtime.toggle_trust_state();
                true
            } else {
                false
            }
        }
        KeyCode::KeyE => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                enablement_runtime.toggle_execution_context();
                true
            } else {
                false
            }
        }
        KeyCode::KeyB => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                enablement_runtime.toggle_policy_blocked();
                true
            } else {
                false
            }
        }
        KeyCode::KeyL => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                appearance.toggle_light_dark();
                true
            } else {
                false
            }
        }
        KeyCode::KeyH => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                appearance.toggle_high_contrast();
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn focused_editor_group_is_empty(frame: &DesktopFrame) -> bool {
    let focused = frame.focused_editor_group();
    frame
        .editor_group_layouts()
        .into_iter()
        .find(|group| group.group_id == focused)
        .map(|group| group.tab_count == 0)
        .unwrap_or(false)
}

fn relayout_and_redraw(
    window: &winit::window::Window,
    surface: &mut SoftbufferSurface,
    frame: &mut DesktopFrame,
) {
    let physical = window.inner_size();
    if physical.width == 0 || physical.height == 0 {
        return;
    }
    let logical = physical.to_logical::<u32>(window.scale_factor());
    frame.relayout(logical.width, logical.height);

    if let (Some(w), Some(h)) = (
        NonZeroU32::new(physical.width),
        NonZeroU32::new(physical.height),
    ) {
        let _ = surface.resize(w, h);
    }
    window.request_redraw();
}

fn draw(
    window: &winit::window::Window,
    surface: &mut SoftbufferSurface,
    registry: &CommandRegistry,
    frame: &DesktopFrame,
    palette: &CommandPaletteState,
    start_center: &StartCenterState,
    overlay: Option<&ShellOverlayState>,
    command_runtime: &CommandRuntimeState,
    keybinding_runtime: &KeybindingRuntimeState,
    enablement_runtime: &CommandEnablementRuntimeState,
    appearance: &ShellAppearanceState,
    held_modifiers: &HeldModifiers,
) -> Result<(), Box<dyn std::error::Error>> {
    let physical = window.inner_size();
    if physical.width == 0 || physical.height == 0 {
        return Ok(());
    }
    surface.resize(
        NonZeroU32::new(physical.width).ok_or("window width is zero")?,
        NonZeroU32::new(physical.height).ok_or("window height is zero")?,
    )?;

    let mut buffer = surface.buffer_mut()?;
    let width = physical.width as usize;
    let height = physical.height as usize;
    if buffer.len() != width.saturating_mul(height) {
        return Ok(());
    }

    let token_registry = seeded_token_registry(appearance.theme())?;
    let style = ShellRenderStyle::load(token_registry)?;

    // Background.
    fill(&mut buffer, style.tokens.bg_canvas);

    let scale = window.scale_factor();
    for zone in ShellZoneId::ALL {
        let zone = *zone;
        if zone == ShellZoneId::TransientOverlay {
            continue;
        }
        let Some(logical_rect) = frame.layout().zone(zone) else {
            continue;
        };
        let rect = to_physical_rect(logical_rect, scale);
        let color = style.tokens.zone_background(zone);
        fill_rect(&mut buffer, physical.width, physical.height, rect, color);

        match zone {
            ShellZoneId::MainWorkspace => {
                for group in frame.editor_group_layouts() {
                    let group_rect = to_physical_rect(group.rect, scale);
                    fill_rect(
                        &mut buffer,
                        physical.width,
                        physical.height,
                        group_rect,
                        style.tokens.bg_surface,
                    );
                    stroke_rect(
                        &mut buffer,
                        physical.width,
                        physical.height,
                        group_rect,
                        style.stroke_default,
                        style.tokens.border_default,
                    );
                    if group.group_id == frame.focused_editor_group()
                        && frame.focused_zone() == ShellZoneId::MainWorkspace
                    {
                        stroke_rect(
                            &mut buffer,
                            physical.width,
                            physical.height,
                            group_rect,
                            style.stroke_focus,
                            style.tokens.focus_ring,
                        );
                    }

                    if group.tab_count == 0 {
                        let focused = group.group_id == frame.focused_editor_group()
                            && frame.focused_zone() == ShellZoneId::MainWorkspace;
                        draw_start_center_surface(
                            &mut buffer,
                            physical.width,
                            physical.height,
                            registry,
                            start_center,
                            enablement_runtime,
                            group_rect,
                            &style,
                            focused,
                        );
                    } else {
                        let label = format!(
                            "editor group {}   tabs: {}{}",
                            group.group_id.value(),
                            group.tab_count,
                            if group.tabbed_compare_active {
                                "   compare: tabbed"
                            } else {
                                ""
                            }
                        );
                        draw_text(
                            &mut buffer,
                            physical.width,
                            physical.height,
                            group_rect.x.saturating_add(style.space_2),
                            group_rect.y.saturating_add(style.space_2),
                            1,
                            &label,
                            style.tokens.text_secondary,
                        );
                    }
                }
            }
            _ => {
                for (slot_id, slot_rect) in frame.slot_rects_within_zone(zone, logical_rect) {
                    let slot_rect = to_physical_rect(slot_rect, scale);
                    fill_rect(
                        &mut buffer,
                        physical.width,
                        physical.height,
                        slot_rect,
                        style.tokens.bg_surface,
                    );
                    stroke_rect(
                        &mut buffer,
                        physical.width,
                        physical.height,
                        slot_rect,
                        style.stroke_default,
                        style.tokens.border_default,
                    );
                    draw_text(
                        &mut buffer,
                        physical.width,
                        physical.height,
                        slot_rect.x.saturating_add(style.space_2),
                        slot_rect.y.saturating_add(style.space_2),
                        1,
                        slot_id,
                        style.tokens.text_muted,
                    );
                }
            }
        }

        if zone == frame.focused_zone() {
            stroke_rect(
                &mut buffer,
                physical.width,
                physical.height,
                rect,
                style.stroke_focus,
                style.tokens.focus_ring,
            );
        }

        let zone_label = format!("zone: {}", zone.name());
        draw_text(
            &mut buffer,
            physical.width,
            physical.height,
            rect.x.saturating_add(style.space_2),
            rect.y.saturating_add(style.space_2 / 2),
            1,
            &zone_label,
            style.tokens.text_muted,
        );
    }

    if palette.is_open() {
        draw_command_palette_overlay(
            &mut buffer,
            physical.width,
            physical.height,
            scale,
            registry,
            frame,
            palette,
            keybinding_runtime,
            enablement_runtime,
            &style,
            held_modifiers,
        );
    }

    if let Some(overlay) = overlay {
        draw_shell_overlay(
            &mut buffer,
            physical.width,
            physical.height,
            window.scale_factor(),
            registry,
            frame,
            overlay,
            keybinding_runtime,
            &style,
        );
    }

    let modes = frame
        .responsive_fallback_modes()
        .into_iter()
        .map(|m| m.name())
        .collect::<Vec<_>>()
        .join(", ");
    let status = to_physical_rect(frame.layout().status_bar, scale);
    if !status.is_empty() {
        let last = command_runtime
            .last_command_label
            .as_deref()
            .unwrap_or("no recent command");
        let last_keybinding = keybinding_runtime
            .last_summary
            .as_deref()
            .unwrap_or("no recent keybinding resolution");
        let exec_ctx = if enablement_runtime.execution_context_available {
            "available"
        } else {
            "unavailable"
        };
        let policy = if enablement_runtime.policy_blocked_in_context {
            "blocked"
        } else {
            "allow"
        };
        let palette_keys = keybinding_runtime.shortcuts_label("cmd:command_palette.open");
        let trust_state = enablement_runtime.workspace_trust_state.as_str();
        let theme_label = appearance.theme().token();
        let text = format!(
            "theme: {}   fallback_modes: [{}]   last_cmd: {}   last_keybinding: {}   enablement: trust={} exec_ctx={} policy={}   keymap: {} ({})   keys: {} palette (resolver), Enter run, Ctrl+\\\\ split, Ctrl+G next group, Ctrl+O add tab, Ctrl+W close group, Ctrl+I keybinding inspector   toggles: Cmd/Ctrl+Shift+T trust, Cmd/Ctrl+Shift+E exec_ctx, Cmd/Ctrl+Shift+B policy, Cmd/Ctrl+Shift+L theme, Cmd/Ctrl+Shift+H high contrast   packets: .logs/command_packets",
            theme_label,
            modes,
            last,
            last_keybinding,
            trust_state,
            exec_ctx,
            policy,
            keybinding_runtime.active_preset.display_name(),
            keybinding_runtime.active_preset.preset_ref(),
            palette_keys
        );

        let badge_text = match trust_state {
            "restricted" => "Restricted",
            _ => "Trusted",
        };
        let (badge_fg, badge_border, badge_fill) = match trust_state {
            "restricted" => (
                style.status_warning,
                style.status_warning_border,
                style.status_warning_fill,
            ),
            _ => (
                style.status_success,
                style.status_success_border,
                style.status_success_fill,
            ),
        };
        let badge_scale = 1u32;
        let badge_char_w = 8u32.saturating_mul(badge_scale);
        let badge_padding = style.space_2 / 2;
        let badge_w = badge_char_w
            .saturating_mul(badge_text.len() as u32)
            .saturating_add(badge_padding.saturating_mul(2));
        let badge_h = 8u32
            .saturating_mul(badge_scale)
            .saturating_add(badge_padding.saturating_mul(2));
        let badge_rect = Rect::new(
            status.x.saturating_add(style.space_2),
            status.y.saturating_add(style.space_2 / 2),
            badge_w,
            badge_h.min(status.height.saturating_sub(1)),
        );
        fill_rect(
            &mut buffer,
            physical.width,
            physical.height,
            badge_rect,
            badge_fill,
        );
        stroke_rect(
            &mut buffer,
            physical.width,
            physical.height,
            badge_rect,
            style.stroke_default,
            badge_border,
        );
        draw_text(
            &mut buffer,
            physical.width,
            physical.height,
            badge_rect.x.saturating_add(badge_padding),
            badge_rect.y.saturating_add(badge_padding),
            badge_scale,
            badge_text,
            badge_fg,
        );

        draw_text(
            &mut buffer,
            physical.width,
            physical.height,
            status
                .x
                .saturating_add(style.space_2)
                .saturating_add(badge_rect.width)
                .saturating_add(style.space_2),
            status.y.saturating_add(style.space_2 / 2),
            1,
            &text,
            style.tokens.text_muted,
        );
    }

    buffer.present()?;
    Ok(())
}

#[derive(Debug, Clone)]
struct CommandDiagnosticsOverlay {
    record: CommandDiagnosticsSheetRecord,
}

#[derive(Debug, Clone)]
struct CommandInvocationPreviewOverlay {
    entry: CommandRegistryEntryRecord,
    record: CommandInvocationPreviewSheetRecord,
}

#[derive(Debug, Clone)]
struct CommandTraceOverlay {
    lines: Vec<String>,
}

#[derive(Debug, Clone)]
enum CommandOverlayDecision {
    PreviewApproved {
        entry: CommandRegistryEntryRecord,
        session: CommandInvocationSession,
    },
    PreviewCancelled {
        entry: CommandRegistryEntryRecord,
        session: CommandInvocationSession,
    },
}

#[derive(Debug)]
struct OverlayKeyOutcome {
    handled: bool,
    command_decision: Option<CommandOverlayDecision>,
}

#[derive(Debug, Clone)]
enum ShellOverlayKind {
    InspectorSheet,
    SplitChoice {
        violation: SplitViolation,
        selection: usize,
    },
    StagedPeek,
    CommandDiagnostics(CommandDiagnosticsOverlay),
    InvocationPreview(CommandInvocationPreviewOverlay),
    CommandTrace(CommandTraceOverlay),
}

#[derive(Debug, Clone)]
struct ShellOverlayState {
    kind: ShellOverlayKind,
    focus_return_zone: ShellZoneId,
    focus_return_group: PaneId,
    closed: bool,
}

impl ShellOverlayState {
    fn inspector_sheet(focus_return_zone: ShellZoneId, focus_return_group: PaneId) -> Self {
        Self {
            kind: ShellOverlayKind::InspectorSheet,
            focus_return_zone,
            focus_return_group,
            closed: false,
        }
    }

    fn split_choice(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        violation: SplitViolation,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::SplitChoice {
                violation,
                selection: 0,
            },
            focus_return_zone,
            focus_return_group,
            closed: false,
        }
    }

    fn command_diagnostics(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        record: CommandDiagnosticsSheetRecord,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::CommandDiagnostics(CommandDiagnosticsOverlay { record }),
            focus_return_zone,
            focus_return_group,
            closed: false,
        }
    }

    fn invocation_preview(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        entry: CommandRegistryEntryRecord,
        record: CommandInvocationPreviewSheetRecord,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::InvocationPreview(CommandInvocationPreviewOverlay {
                entry,
                record,
            }),
            focus_return_zone,
            focus_return_group,
            closed: false,
        }
    }

    fn command_trace(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        lines: Vec<String>,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::CommandTrace(CommandTraceOverlay { lines }),
            focus_return_zone,
            focus_return_group,
            closed: false,
        }
    }

    fn close(&mut self, frame: &mut DesktopFrame) {
        self.closed = true;
        frame.focus_zone(self.focus_return_zone);
        if self.focus_return_zone == ShellZoneId::MainWorkspace {
            frame.focus_editor_group(self.focus_return_group);
        }
    }

    fn handle_key(
        &mut self,
        code: KeyCode,
        frame: &mut DesktopFrame,
        keybinding_runtime: &mut KeybindingRuntimeState,
    ) -> OverlayKeyOutcome {
        let mut command_decision = None;
        let handled = match (&mut self.kind, code) {
            (_, KeyCode::Escape) => {
                if let ShellOverlayKind::InvocationPreview(preview) = &mut self.kind {
                    preview
                        .record
                        .invocation_session
                        .approval_posture
                        .approval_state = "approval_denied".to_string();
                    command_decision = Some(CommandOverlayDecision::PreviewCancelled {
                        entry: preview.entry.clone(),
                        session: preview.record.invocation_session.clone(),
                    });
                }
                self.close(frame);
                true
            }
            (ShellOverlayKind::InspectorSheet, KeyCode::ArrowLeft) => {
                keybinding_runtime.cycle_preset(-1);
                true
            }
            (ShellOverlayKind::InspectorSheet, KeyCode::ArrowRight) => {
                keybinding_runtime.cycle_preset(1);
                true
            }
            (ShellOverlayKind::InvocationPreview(preview), KeyCode::Enter) => {
                preview
                    .record
                    .invocation_session
                    .approval_posture
                    .approval_state = "approval_granted".to_string();
                if preview
                    .record
                    .invocation_session
                    .approval_posture
                    .approval_ticket_ref
                    .is_none()
                    && preview
                        .record
                        .invocation_session
                        .approval_posture
                        .approval_posture_class_declared
                        != "no_approval_required"
                {
                    preview
                        .record
                        .invocation_session
                        .approval_posture
                        .approval_ticket_ref = Some(mint_approval_ticket_ref(
                        &preview.record.invocation_session.canonical_verb,
                    ));
                }
                command_decision = Some(CommandOverlayDecision::PreviewApproved {
                    entry: preview.entry.clone(),
                    session: preview.record.invocation_session.clone(),
                });
                self.close(frame);
                true
            }
            (ShellOverlayKind::SplitChoice { selection, .. }, KeyCode::ArrowDown) => {
                *selection = (*selection + 1) % 3;
                true
            }
            (ShellOverlayKind::SplitChoice { selection, .. }, KeyCode::ArrowUp) => {
                *selection = (*selection + 3 - 1) % 3;
                true
            }
            (ShellOverlayKind::SplitChoice { selection, .. }, KeyCode::Enter) => {
                match *selection {
                    0 => {
                        frame.engage_tabbed_compare_fallback();
                        self.close(frame);
                    }
                    1 => {
                        self.kind = ShellOverlayKind::StagedPeek;
                    }
                    _ => {
                        self.close(frame);
                    }
                }
                true
            }
            _ => false,
        };

        OverlayKeyOutcome {
            handled,
            command_decision,
        }
    }
}

fn draw_shell_overlay(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    scale_factor: f64,
    registry: &CommandRegistry,
    frame: &DesktopFrame,
    overlay: &ShellOverlayState,
    keybinding_runtime: &KeybindingRuntimeState,
    style: &ShellRenderStyle,
) {
    let overlay_rect = to_physical_rect(frame.layout().transient_overlay, scale_factor);
    let sheet_w = (overlay_rect.width / 2).max(260);
    let sheet_margin_y = style
        .space_6
        .saturating_mul(2)
        .saturating_add(style.space_3);
    let sheet_rect = Rect::new(
        overlay_rect.right().saturating_sub(sheet_w),
        overlay_rect.y.saturating_add(sheet_margin_y),
        sheet_w,
        overlay_rect
            .height
            .saturating_sub(sheet_margin_y.saturating_mul(2)),
    );

    fill_rect(buffer, width, height, overlay_rect, style.tokens.bg_overlay);
    fill_rect(buffer, width, height, sheet_rect, style.tokens.bg_raised);
    stroke_rect(
        buffer,
        width,
        height,
        sheet_rect,
        style.stroke_default,
        style.tokens.border_strong,
    );

    match &overlay.kind {
        ShellOverlayKind::InspectorSheet => {
            let lines = build_inspector_lines(
                registry,
                keybinding_runtime.active_preset,
                keybinding_runtime.platform_class,
            );
            let mut cursor_y = sheet_rect.y.saturating_add(style.space_3);
            let cursor_x = sheet_rect.x.saturating_add(style.space_3);
            let line_h = 8u32.saturating_add(style.space_2.saturating_mul(3) / 4);
            for line in lines {
                if cursor_y.saturating_add(line_h)
                    > sheet_rect.bottom().saturating_sub(style.space_3)
                {
                    break;
                }
                draw_text(
                    buffer,
                    width,
                    height,
                    cursor_x,
                    cursor_y,
                    1,
                    &line,
                    style.tokens.text_muted,
                );
                cursor_y = cursor_y.saturating_add(line_h);
            }
        }
        ShellOverlayKind::CommandDiagnostics(sheet) => {
            let lines = diagnostics_sheet_lines(&sheet.record);
            let mut cursor_y = sheet_rect.y.saturating_add(style.space_3);
            let cursor_x = sheet_rect.x.saturating_add(style.space_3);
            let line_h = 8u32.saturating_add(style.space_2.saturating_mul(3) / 4);
            for (idx, line) in lines.into_iter().enumerate() {
                if cursor_y.saturating_add(line_h)
                    > sheet_rect.bottom().saturating_sub(style.space_3)
                {
                    break;
                }
                let color = match idx {
                    0 => style.tokens.text_primary,
                    1 => style.tokens.text_secondary,
                    _ => style.tokens.text_muted,
                };
                draw_text(buffer, width, height, cursor_x, cursor_y, 1, &line, color);
                cursor_y = cursor_y.saturating_add(line_h);
            }
        }
        ShellOverlayKind::InvocationPreview(preview) => {
            let lines = invocation_preview_sheet_lines(&preview.record);
            let mut cursor_y = sheet_rect.y.saturating_add(style.space_3);
            let cursor_x = sheet_rect.x.saturating_add(style.space_3);
            let line_h = 8u32.saturating_add(style.space_2.saturating_mul(3) / 4);
            for (idx, line) in lines.into_iter().enumerate() {
                if cursor_y.saturating_add(line_h)
                    > sheet_rect.bottom().saturating_sub(style.space_3)
                {
                    break;
                }
                let color = match idx {
                    0 => style.tokens.text_primary,
                    1 => style.tokens.text_secondary,
                    _ => style.tokens.text_muted,
                };
                draw_text(buffer, width, height, cursor_x, cursor_y, 1, &line, color);
                cursor_y = cursor_y.saturating_add(line_h);
            }
        }
        ShellOverlayKind::CommandTrace(trace) => {
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(style.space_3),
                sheet_rect.y.saturating_add(style.space_3),
                1,
                "Command Trace — Esc closes",
                style.tokens.text_primary,
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(style.space_3),
                sheet_rect
                    .y
                    .saturating_add(style.space_3)
                    .saturating_add(16),
                1,
                "Packets: .logs/command_packets",
                style.tokens.text_muted,
            );

            let mut y = sheet_rect
                .y
                .saturating_add(style.space_3)
                .saturating_add(32);
            for line in &trace.lines {
                if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(style.space_3) {
                    break;
                }
                draw_text(
                    buffer,
                    width,
                    height,
                    sheet_rect.x.saturating_add(style.space_3),
                    y,
                    1,
                    line,
                    style.tokens.text_muted,
                );
                y = y.saturating_add(14);
            }
            if trace.lines.is_empty() {
                draw_text(
                    buffer,
                    width,
                    height,
                    sheet_rect.x.saturating_add(style.space_3),
                    y,
                    1,
                    "No invocations recorded yet.",
                    style.tokens.text_muted,
                );
            }
        }
        ShellOverlayKind::SplitChoice {
            violation,
            selection,
        } => {
            let header = format!(
                "Split would violate min group width (min {}px, attempted {}px).",
                violation.main_workspace_minimum_width, violation.attempted_per_group_width
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(style.space_3),
                sheet_rect.y.saturating_add(style.space_3),
                1,
                &header,
                style.tokens.text_primary,
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(style.space_3),
                sheet_rect
                    .y
                    .saturating_add(style.space_3)
                    .saturating_add(16),
                1,
                "Choose fallback: Up/Down, Enter confirm, Esc cancel",
                style.tokens.text_secondary,
            );

            let options = ["Tabbed compare (recommended)", "Staged peek", "Cancel"];
            for (idx, label) in options.iter().enumerate() {
                let y = sheet_rect
                    .y
                    .saturating_add(style.space_6.saturating_mul(2))
                    .saturating_add((idx as u32) * 18);
                if idx == *selection {
                    let highlight = Rect::new(
                        sheet_rect.x.saturating_add(style.space_2),
                        y.saturating_sub(2),
                        sheet_rect.width.saturating_sub(style.space_4),
                        16,
                    );
                    fill_rect(buffer, width, height, highlight, style.tokens.bg_hover);
                }
                draw_text(
                    buffer,
                    width,
                    height,
                    sheet_rect.x.saturating_add(style.space_3),
                    y,
                    1,
                    label,
                    if idx == *selection {
                        style.tokens.text_primary
                    } else {
                        style.tokens.text_muted
                    },
                );
            }
        }
        ShellOverlayKind::StagedPeek => {
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(style.space_3),
                sheet_rect.y.saturating_add(style.space_3),
                1,
                "Staged peek (sheet) — Esc closes",
                style.tokens.text_primary,
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(style.space_3),
                sheet_rect
                    .y
                    .saturating_add(style.space_3)
                    .saturating_add(16),
                1,
                "This placeholder represents a temporary narrow-width compare peek with focus return.",
                style.tokens.text_muted,
            );
        }
    }
}

fn to_physical_rect(rect: Rect, scale_factor: f64) -> Rect {
    let scale = |v: u32| -> u32 { ((v as f64) * scale_factor).round().max(0.0) as u32 };
    Rect::new(
        scale(rect.x),
        scale(rect.y),
        scale(rect.width),
        scale(rect.height),
    )
}

fn fill(buffer: &mut [u32], color: ColorRgba) {
    let rgb = color.to_u32_rgb();
    buffer.fill(rgb);
}

fn fill_rect(buffer: &mut [u32], width: u32, height: u32, rect: Rect, color: ColorRgba) {
    if rect.is_empty() || width == 0 || height == 0 {
        return;
    }
    let max_x = width.saturating_sub(1);
    let max_y = height.saturating_sub(1);
    let x0 = rect.x.min(max_x);
    let y0 = rect.y.min(max_y);
    let x1 = rect.right().min(width);
    let y1 = rect.bottom().min(height);

    for y in y0..y1 {
        let row = (y as usize).saturating_mul(width as usize);
        for x in x0..x1 {
            let idx = row.saturating_add(x as usize);
            if let Some(px) = buffer.get_mut(idx) {
                if color.a == 255 {
                    *px = color.to_u32_rgb();
                } else {
                    *px = color.blend_over_u32(*px);
                }
            }
        }
    }
}

fn draw_start_center_surface(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    registry: &CommandRegistry,
    start_center: &StartCenterState,
    enablement_runtime: &CommandEnablementRuntimeState,
    rect: Rect,
    style: &ShellRenderStyle,
    focused: bool,
) {
    let padding = style.space_4;
    let card = Rect::new(
        rect.x.saturating_add(padding),
        rect.y.saturating_add(padding),
        rect.width.saturating_sub(padding.saturating_mul(2)),
        rect.height.saturating_sub(padding.saturating_mul(2)),
    );
    if card.is_empty() {
        return;
    }

    fill_rect(buffer, width, height, card, style.tokens.bg_raised);
    stroke_rect(
        buffer,
        width,
        height,
        card,
        style.stroke_default,
        style.tokens.border_default,
    );

    let header_x = card.x.saturating_add(style.space_3);
    let mut y = card.y.saturating_add(style.space_3);

    draw_text(
        buffer,
        width,
        height,
        header_x,
        y,
        2,
        START_CENTER_PRESENTATION_LABEL,
        style.tokens.text_primary,
    );
    y = y
        .saturating_add(8u32.saturating_mul(2))
        .saturating_add(style.space_2);

    draw_text(
        buffer,
        width,
        height,
        header_x,
        y,
        1,
        START_CENTER_PRESENTATION_SUBTITLE,
        style.tokens.text_secondary,
    );
    y = y.saturating_add(18);

    let runtime = StartCenterRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
        execution_context_available: enablement_runtime.execution_context_available,
        provider_linked: enablement_runtime.provider_linked,
        credential_available: enablement_runtime.credential_available,
        policy_disabled: enablement_runtime.policy_disabled,
        policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
    };
    let rows = start_center_action_rows(registry, runtime);
    let selected = start_center.selection().min(rows.len().saturating_sub(1));

    let row_height = 44;
    let row_gap = style.space_2;
    let row_width = card.width.saturating_sub(style.space_3.saturating_mul(2));
    for (idx, row) in rows.iter().enumerate() {
        let row_rect = Rect::new(header_x, y, row_width, row_height);
        if row_rect.bottom().saturating_add(style.space_3) > card.bottom() {
            break;
        }

        let is_selected = focused && idx == selected;
        if is_selected {
            fill_rect(buffer, width, height, row_rect, style.tokens.bg_hover);
            stroke_rect(
                buffer,
                width,
                height,
                row_rect,
                style.stroke_focus,
                style.tokens.accent_interactive,
            );
        } else {
            fill_rect(buffer, width, height, row_rect, style.tokens.bg_surface);
            stroke_rect(
                buffer,
                width,
                height,
                row_rect,
                style.stroke_default,
                style.tokens.border_default,
            );
        }

        let status = row
            .preflight
            .as_ref()
            .map(|p| preflight_decision_class_label(p.decision_class))
            .unwrap_or("missing");
        let label = format!("{}   [{}]", row.title, status);
        let label_color = if is_selected {
            style.tokens.text_primary
        } else {
            style.tokens.text_secondary
        };
        draw_text(
            buffer,
            width,
            height,
            row_rect.x.saturating_add(style.space_2),
            row_rect.y.saturating_add(style.space_2),
            1,
            &label,
            label_color,
        );

        let mut detail = row.summary.to_string();
        if let Some(preflight) = row.preflight.as_ref() {
            if let Some(reason) = preflight.enablement_snapshot.disabled_reason_code {
                detail.push_str("  — ");
                detail.push_str(reason.as_str());
            }
        }
        draw_text(
            buffer,
            width,
            height,
            row_rect.x.saturating_add(style.space_2),
            row_rect.y.saturating_add(style.space_2).saturating_add(14),
            1,
            &detail,
            style.tokens.text_muted,
        );

        y = y.saturating_add(row_height).saturating_add(row_gap);
    }

    if y.saturating_add(22) < card.bottom() {
        draw_text(
            buffer,
            width,
            height,
            header_x,
            card.bottom()
                .saturating_sub(style.space_3)
                .saturating_sub(12),
            1,
            "↑/↓ select • Enter run • Cmd/Ctrl+Shift+P palette",
            style.tokens.text_muted,
        );
    }

    if card.height > style.space_6.saturating_mul(4) {
        let accent = Rect::new(card.x, card.y, style.stroke_focus.max(2), card.height);
        fill_rect(buffer, width, height, accent, style.tokens.accent_brand);
    }
}

fn preflight_decision_class_label(decision: PreflightDecisionClass) -> &'static str {
    match decision {
        PreflightDecisionClass::Allowed => "ready",
        PreflightDecisionClass::BlockedByPolicy => "blocked",
        PreflightDecisionClass::DisabledWithReason => "disabled",
        PreflightDecisionClass::PreviewRequired => "preview",
        PreflightDecisionClass::ApprovalRequired => "approval",
    }
}

fn draw_command_palette_overlay(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    scale_factor: f64,
    registry: &CommandRegistry,
    frame: &DesktopFrame,
    palette: &CommandPaletteState,
    keybinding_runtime: &KeybindingRuntimeState,
    enablement_runtime: &CommandEnablementRuntimeState,
    style: &ShellRenderStyle,
    held_modifiers: &HeldModifiers,
) {
    let Some(overlay_logical) = frame.layout().zone(ShellZoneId::TransientOverlay) else {
        return;
    };
    let overlay_physical = to_physical_rect(overlay_logical, scale_factor);
    let slots = frame.slot_rects_within_zone(ShellZoneId::TransientOverlay, overlay_logical);
    let slot = slots
        .iter()
        .find(|(id, _)| *id == "slot.overlay.command_palette")
        .map(|(_, rect)| *rect)
        .unwrap_or(overlay_logical);
    let slot_physical = to_physical_rect(slot, scale_factor);

    // Dim the entire window.
    fill_rect(
        buffer,
        width,
        height,
        overlay_physical,
        style.tokens.bg_overlay,
    );

    // Panel inside the slot.
    let panel_padding = style.space_4;
    let panel = Rect::new(
        slot_physical.x.saturating_add(panel_padding),
        slot_physical.y.saturating_add(panel_padding),
        slot_physical.width.saturating_sub(panel_padding * 2),
        slot_physical.height.saturating_sub(panel_padding * 2),
    );
    if panel.is_empty() {
        return;
    }

    fill_rect(buffer, width, height, panel, style.tokens.bg_raised);
    stroke_rect(
        buffer,
        width,
        height,
        panel,
        style.stroke_default,
        style.tokens.border_strong,
    );

    let text_scale = 2u32;
    let line_h = (8u32.saturating_mul(text_scale)).saturating_add(style.space_2);
    let mut cursor_y = panel.y.saturating_add(style.space_3);
    let cursor_x = panel.x.saturating_add(style.space_3);

    draw_text(
        buffer,
        width,
        height,
        cursor_x,
        cursor_y,
        text_scale,
        "Command Palette (Cmd/Ctrl+Shift+P)",
        style.tokens.text_primary,
    );
    cursor_y = cursor_y.saturating_add(line_h);

    draw_text(
        buffer,
        width,
        height,
        cursor_x,
        cursor_y,
        text_scale,
        "Type to search. Up/Down: select   Enter: run   Esc: close",
        style.tokens.text_secondary,
    );
    cursor_y = cursor_y
        .saturating_add(line_h)
        .saturating_add(style.space_2 / 2);

    let preview_runtime = PalettePreviewRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
        execution_context_available: enablement_runtime.execution_context_available,
        provider_linked: enablement_runtime.provider_linked,
        credential_available: enablement_runtime.credential_available,
        policy_disabled: enablement_runtime.policy_disabled,
        policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
    };
    let preview = materialize_palette_preview_record(
        palette.selected_key(),
        registry,
        &keybinding_runtime.shortcuts_by_command_id,
        preview_runtime,
    );

    let selected_key = palette.selected_key().cloned();
    let view_rows = palette_view_rows(
        palette,
        registry,
        &keybinding_runtime.shortcuts_by_command_id,
        |entry| {
            let enablement_context = CommandEnablementContext {
                client_scope: "desktop_product".to_string(),
                workspace_trust_state: enablement_runtime.workspace_trust_state.clone(),
                execution_context_available: enablement_runtime.execution_context_available,
                provider_linked: enablement_runtime.provider_linked,
                credential_available: enablement_runtime.credential_available,
                policy_disabled: enablement_runtime.policy_disabled,
                policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
                argument_provenance_map: argument_provenance_map_for(entry),
            };
            let snapshot = entry.evaluate_enablement(&enablement_context);
            (snapshot.decision_class, snapshot.disabled_reason_code)
        },
    );

    let inner_padding = style.space_3;
    let footer_lines = 2u32;
    let footer_height = footer_lines
        .saturating_mul(line_h)
        .saturating_add(style.space_3);
    let footer = Rect::new(
        panel.x.saturating_add(inner_padding),
        panel
            .bottom()
            .saturating_sub(inner_padding)
            .saturating_sub(footer_height),
        panel.width.saturating_sub(inner_padding.saturating_mul(2)),
        footer_height,
    );
    if footer.is_empty() {
        return;
    }

    let content_height = footer
        .y
        .saturating_sub(cursor_y)
        .saturating_sub(style.space_2);
    let content = Rect::new(
        panel.x.saturating_add(inner_padding),
        cursor_y,
        panel.width.saturating_sub(inner_padding.saturating_mul(2)),
        content_height,
    );
    if content.is_empty() {
        return;
    }

    let gap = style.space_3;
    let char_w = 8u32.saturating_mul(text_scale);
    let min_list_w = char_w.saturating_mul(48);
    let min_preview_w = char_w.saturating_mul(36);
    let (list_rect, preview_rect) = if content.width > min_list_w + gap + min_preview_w {
        let max_list_w = content
            .width
            .saturating_sub(gap)
            .saturating_sub(min_preview_w);
        let list_w = (content.width.saturating_mul(3) / 5)
            .max(min_list_w)
            .min(max_list_w);
        let preview_w = content.width.saturating_sub(list_w).saturating_sub(gap);
        (
            Rect::new(content.x, content.y, list_w, content.height),
            Rect::new(
                content.x.saturating_add(list_w).saturating_add(gap),
                content.y,
                preview_w,
                content.height,
            ),
        )
    } else {
        (content, Rect::new(0, 0, 0, 0))
    };

    let max_list_cols = (list_rect.width / char_w).saturating_sub(1) as usize;
    let mut list_y = list_rect.y;
    let list_x = list_rect.x;
    for row in view_rows.iter() {
        if list_y.saturating_add(line_h) > list_rect.bottom() {
            break;
        }
        let selected = row
            .key
            .as_ref()
            .and_then(|key| selected_key.as_ref().map(|s| (key, s)))
            .map(|(k, s)| k == s)
            .unwrap_or(false);
        if selected && !row.is_group_header {
            let highlight = Rect::new(
                list_rect.x,
                list_y.saturating_sub(style.space_2 / 4),
                list_rect.width,
                line_h,
            );
            fill_rect(buffer, width, height, highlight, style.tokens.bg_hover);
        }

        let line = if max_list_cols == 0 {
            String::new()
        } else {
            row.text.chars().take(max_list_cols).collect::<String>()
        };
        draw_text(
            buffer,
            width,
            height,
            list_x,
            list_y,
            text_scale,
            &line,
            if selected && !row.is_group_header {
                style.tokens.text_primary
            } else if row.is_group_header {
                style.tokens.text_secondary
            } else {
                style.tokens.text_muted
            },
        );
        list_y = list_y.saturating_add(line_h);
    }

    if !preview_rect.is_empty() {
        fill_rect(buffer, width, height, preview_rect, style.tokens.bg_surface);
        stroke_rect(
            buffer,
            width,
            height,
            preview_rect,
            style.stroke_default,
            style.tokens.border_default,
        );

        let max_preview_cols = (preview_rect.width / char_w).saturating_sub(1) as usize;
        let mut preview_y = preview_rect.y.saturating_add(style.space_2);
        let preview_x = preview_rect.x.saturating_add(style.space_2);

        let mut preview_lines: Vec<String> = Vec::new();
        match &preview.selection {
            PalettePreviewSelection::None => {
                preview_lines.push("No selection".to_string());
            }
            PalettePreviewSelection::File(file) => {
                preview_lines.push("File".to_string());
                preview_lines.push(file.relative_path.clone());
            }
            PalettePreviewSelection::Command(command) => {
                preview_lines.push("Command".to_string());
                preview_lines.push(command.title.clone());
                preview_lines.push(command.command_id.clone());
                if !command.shortcuts.is_empty() {
                    preview_lines.push(format!("Keys: {}", command.shortcuts.join(", ")));
                }
                preview_lines.push(format!("Verb: {}", command.canonical_verb));
                preview_lines.push(format!("Preflight: {}", command.preflight.decision_class));
                if command.preflight.enablement_snapshot.decision_class
                    != EnablementDecisionClass::Enabled
                {
                    let code = command
                        .preflight
                        .enablement_snapshot
                        .disabled_reason_code
                        .map(|c| c.as_str())
                        .unwrap_or("unknown");
                    preview_lines.push(format!("Disabled: {}", code));
                }
                if !command.typed_arguments.is_empty() {
                    preview_lines.push("Args:".to_string());
                    for arg in &command.typed_arguments {
                        let required = if arg.is_required {
                            "required"
                        } else {
                            "optional"
                        };
                        preview_lines.push(format!(
                            "- {} ({}, {})",
                            arg.argument_name, arg.argument_kind, required
                        ));
                    }
                }
            }
        }

        for line in preview_lines {
            if preview_y.saturating_add(line_h)
                > preview_rect.bottom().saturating_sub(style.space_2)
            {
                break;
            }
            let clipped = if max_preview_cols == 0 {
                String::new()
            } else {
                line.chars().take(max_preview_cols).collect::<String>()
            };
            draw_text(
                buffer,
                width,
                height,
                preview_x,
                preview_y,
                text_scale,
                &clipped,
                style.tokens.text_secondary,
            );
            preview_y = preview_y.saturating_add(line_h);
        }
    }

    fill_rect(buffer, width, height, footer, style.tokens.bg_subtle);
    stroke_rect(
        buffer,
        width,
        height,
        footer,
        style.stroke_default,
        style.tokens.border_default,
    );

    let footer_x = footer.x.saturating_add(style.space_2);
    let mut footer_y = footer.y.saturating_add(style.space_2);
    let footer_cols = (footer.width / char_w).saturating_sub(1) as usize;

    let (footer_line_1, footer_line_2) = match &preview.selection {
        PalettePreviewSelection::Command(command) => {
            let cli_hint = if command.copy.cli_skeleton.is_some() {
                "Shift: cli skeleton"
            } else {
                "Shift: (no cli)"
            };
            let copy_hint = if held_modifiers.ctrl_or_logo() {
                if held_modifiers.shift && command.copy.cli_skeleton.is_some() {
                    "C: copy cli skeleton"
                } else {
                    "C: copy id"
                }
            } else {
                "Cmd/Ctrl+C: copy id"
            };
            let diagnostics_hint = if held_modifiers.ctrl_or_logo() {
                "D: diagnostics"
            } else {
                "Cmd/Ctrl+D: diagnostics"
            };
            (
                format!(
                    "Enter: invoke   {}   {}   ({})",
                    copy_hint, diagnostics_hint, cli_hint
                ),
                format!(
                    "Preview: {}   Approval: {}   Side-effects: {}",
                    command.preview_class,
                    command.approval_posture_class,
                    command.dominant_side_effect_class
                ),
            )
        }
        PalettePreviewSelection::File(_) => (
            "Enter: open   Esc: close".to_string(),
            "Up/Down: select".to_string(),
        ),
        PalettePreviewSelection::None => ("Type to search. Esc: close".to_string(), String::new()),
    };

    for line in [footer_line_1, footer_line_2] {
        if footer_y.saturating_add(line_h) > footer.bottom().saturating_sub(style.space_2) {
            break;
        }
        let clipped = if footer_cols == 0 {
            String::new()
        } else {
            line.chars().take(footer_cols).collect::<String>()
        };
        draw_text(
            buffer,
            width,
            height,
            footer_x,
            footer_y,
            text_scale,
            &clipped,
            style.tokens.text_secondary,
        );
        footer_y = footer_y.saturating_add(line_h);
    }
}

fn draw_text(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    mut x: u32,
    y: u32,
    scale: u32,
    text: &str,
    color: ColorRgba,
) {
    for ch in text.chars() {
        draw_glyph(buffer, width, height, x, y, scale, ch, color);
        x = x.saturating_add(8 * scale);
    }
}

fn draw_glyph(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    scale: u32,
    ch: char,
    color: ColorRgba,
) {
    let glyph = BASIC_FONTS.get(ch).or_else(|| BASIC_FONTS.get('?'));
    let Some(rows) = glyph else {
        return;
    };
    for (row, bits) in rows.iter().enumerate() {
        let row_bits = *bits;
        for bit in 0..8usize {
            if row_bits & (1u8 << bit) == 0 {
                continue;
            }
            let px = x.saturating_add((bit as u32).saturating_mul(scale));
            let py = y.saturating_add((row as u32).saturating_mul(scale));
            draw_scaled_pixel(buffer, width, height, px, py, scale, color);
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct HeldModifiers {
    ctrl: bool,
    alt: bool,
    shift: bool,
    logo: bool,
}

impl HeldModifiers {
    fn ctrl_or_logo(self) -> bool {
        self.ctrl || self.logo
    }

    fn update_from_key_event(&mut self, event: &KeyEvent) {
        let PhysicalKey::Code(code) = event.physical_key else {
            return;
        };
        let pressed = event.state == ElementState::Pressed;
        match code {
            KeyCode::ControlLeft | KeyCode::ControlRight => self.ctrl = pressed,
            KeyCode::AltLeft | KeyCode::AltRight => self.alt = pressed,
            KeyCode::ShiftLeft | KeyCode::ShiftRight => self.shift = pressed,
            KeyCode::SuperLeft | KeyCode::SuperRight => self.logo = pressed,
            _ => {}
        }
    }
}

#[derive(Debug)]
struct KeybindingRuntimeState {
    platform_class: PlatformClass,
    active_preset: KeymapPresetId,
    resolver: KeybindingResolver,
    shortcuts_by_command_id: HashMap<String, Vec<String>>,
    last_summary: Option<String>,
}

impl KeybindingRuntimeState {
    fn new(platform_class: PlatformClass) -> Self {
        let mut state = Self {
            platform_class,
            active_preset: KeymapPresetId::VsCode,
            resolver: seeded_keybinding_resolver().clone(),
            shortcuts_by_command_id: HashMap::new(),
            last_summary: None,
        };
        state.rebuild();
        state
    }

    fn rebuild(&mut self) {
        self.resolver = resolver_with_preset(self.active_preset, self.platform_class)
            .unwrap_or_else(|_| seeded_keybinding_resolver().clone());

        self.shortcuts_by_command_id.clear();
        if let Ok(rows) = preset_binding_rows(self.active_preset, self.platform_class) {
            for row in rows {
                self.shortcuts_by_command_id
                    .entry(row.command_id)
                    .or_default()
                    .push(row.literal_sequence);
            }
        }

        for sequences in self.shortcuts_by_command_id.values_mut() {
            sequences.sort();
            sequences.dedup();
        }
    }

    fn cycle_preset(&mut self, direction: i32) {
        let presets = KeymapPresetId::all();
        let Some(idx) = presets
            .iter()
            .position(|preset| *preset == self.active_preset)
        else {
            self.active_preset = presets[0];
            self.rebuild();
            return;
        };
        let len = presets.len() as i32;
        let next = (idx as i32 + direction).rem_euclid(len) as usize;
        self.active_preset = presets[next];
        self.rebuild();
    }

    fn shortcuts_label(&self, command_id: &str) -> String {
        self.shortcuts_by_command_id
            .get(command_id)
            .map(|seqs| seqs.join(", "))
            .unwrap_or_else(|| "unbound".to_string())
    }

    fn record(&mut self, packet: aureline_input::keybindings::KeybindingResolutionPacketRecord) {
        let winner = match packet.winning_resolution.winner_kind {
            WinningResolutionKind::CommandCandidate => packet
                .winning_resolution
                .command_candidate
                .as_ref()
                .map(|c| c.command.command_id.as_str())
                .unwrap_or("<missing-command>"),
            WinningResolutionKind::PlatformReserved => "platform_reserved",
            WinningResolutionKind::EmergencySecurityHardBlock => "security_blocked",
            WinningResolutionKind::AdminPolicyLock => "policy_locked",
            WinningResolutionKind::WaitingState => "waiting_for_next_stroke",
            WinningResolutionKind::Unbound => "unbound",
        };
        let layer = packet
            .winning_resolution
            .resolver_layer
            .map(|l| format!("{l:?}"))
            .unwrap_or_else(|| "none".to_string());
        self.last_summary = Some(format!(
            "{} => {} (layer: {}, state: {:?})",
            packet.inspected_sequence.literal_sequence, winner, layer, packet.sequence_state
        ));
    }
}

fn platform_class_for_shell() -> PlatformClass {
    #[cfg(target_os = "macos")]
    {
        PlatformClass::Macos
    }
    #[cfg(target_os = "windows")]
    {
        PlatformClass::Windows
    }
    #[cfg(target_os = "linux")]
    {
        PlatformClass::Linux
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        PlatformClass::CrossPlatform
    }
}

fn keybinding_sequence_and_scope_from_shell(
    code: KeyCode,
    modifiers: &HeldModifiers,
    frame: &DesktopFrame,
) -> Option<(KeySequence, InspectionScope)> {
    let key = key_string_for_keycode(code)?;
    let stroke = KeyStroke {
        modifiers: Modifiers {
            ctrl: modifiers.ctrl,
            alt: modifiers.alt,
            shift: modifiers.shift,
            cmd: modifiers.logo,
        },
        key,
    };
    let sequence = KeySequence::new(vec![stroke]);
    let inspection_scope = InspectionScope {
        platform_class: platform_class_for_shell(),
        surface_ref: "surface:shell".to_string(),
        focus_context_ref: format!("focus:{}", frame.focused_zone().name()),
        active_mode_ref: None,
        workspace_scope_ref: "workspace:unknown".to_string(),
        surface_support_class: SurfaceSupportClass::FullySupported,
    };
    Some((sequence, inspection_scope))
}

fn key_string_for_keycode(code: KeyCode) -> Option<String> {
    match code {
        KeyCode::KeyA => Some("A".to_string()),
        KeyCode::KeyB => Some("B".to_string()),
        KeyCode::KeyC => Some("C".to_string()),
        KeyCode::KeyD => Some("D".to_string()),
        KeyCode::KeyE => Some("E".to_string()),
        KeyCode::KeyF => Some("F".to_string()),
        KeyCode::KeyG => Some("G".to_string()),
        KeyCode::KeyH => Some("H".to_string()),
        KeyCode::KeyI => Some("I".to_string()),
        KeyCode::KeyJ => Some("J".to_string()),
        KeyCode::KeyK => Some("K".to_string()),
        KeyCode::KeyL => Some("L".to_string()),
        KeyCode::KeyM => Some("M".to_string()),
        KeyCode::KeyN => Some("N".to_string()),
        KeyCode::KeyO => Some("O".to_string()),
        KeyCode::KeyP => Some("P".to_string()),
        KeyCode::KeyQ => Some("Q".to_string()),
        KeyCode::KeyR => Some("R".to_string()),
        KeyCode::KeyS => Some("S".to_string()),
        KeyCode::KeyT => Some("T".to_string()),
        KeyCode::KeyU => Some("U".to_string()),
        KeyCode::KeyV => Some("V".to_string()),
        KeyCode::KeyW => Some("W".to_string()),
        KeyCode::KeyX => Some("X".to_string()),
        KeyCode::KeyY => Some("Y".to_string()),
        KeyCode::KeyZ => Some("Z".to_string()),
        KeyCode::Space => Some("Space".to_string()),
        _ => None,
    }
}

fn draw_scaled_pixel(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    scale: u32,
    color: ColorRgba,
) {
    if scale == 0 || x >= width || y >= height {
        return;
    }
    let max_x = width.saturating_sub(1);
    let max_y = height.saturating_sub(1);
    let x1 = x.saturating_add(scale).min(max_x.saturating_add(1));
    let y1 = y.saturating_add(scale).min(max_y.saturating_add(1));
    for yy in y..y1 {
        let row = (yy as usize).saturating_mul(width as usize);
        for xx in x..x1 {
            let idx = row.saturating_add(xx as usize);
            if let Some(px) = buffer.get_mut(idx) {
                if color.a == 255 {
                    *px = color.to_u32_rgb();
                } else {
                    *px = color.blend_over_u32(*px);
                }
            }
        }
    }
}

fn stroke_rect(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rect: Rect,
    thickness: u32,
    color: ColorRgba,
) {
    if rect.is_empty() || thickness == 0 {
        return;
    }
    let t = thickness;
    // top
    fill_rect(
        buffer,
        width,
        height,
        Rect::new(rect.x, rect.y, rect.width, t.min(rect.height)),
        color,
    );
    // bottom
    fill_rect(
        buffer,
        width,
        height,
        Rect::new(
            rect.x,
            rect.bottom().saturating_sub(t),
            rect.width,
            t.min(rect.height),
        ),
        color,
    );
    // left
    fill_rect(
        buffer,
        width,
        height,
        Rect::new(rect.x, rect.y, t.min(rect.width), rect.height),
        color,
    );
    // right
    fill_rect(
        buffer,
        width,
        height,
        Rect::new(
            rect.right().saturating_sub(t),
            rect.y,
            t.min(rect.width),
            rect.height,
        ),
        color,
    );
}
