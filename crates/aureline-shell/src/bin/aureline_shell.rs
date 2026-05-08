use std::num::NonZeroU32;
use std::sync::Arc;

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
use aureline_commands::{CommandRegistry, CommandRegistryEntryRecord};
use aureline_shell::app_frame::desktop_frame::{
    DesktopFrame, NewEditorGroupOutcome, SplitViolation,
};
use aureline_shell::layout::split_tree::PaneId;
use aureline_shell::layout::zone_registry::{Rect, ShellZoneId};
use aureline_input::keybindings::{
    seeded_keybinding_resolver, InspectionScope, KeySequence, KeyStroke, Modifiers, PlatformClass,
    SequenceResolutionState, SurfaceSupportClass, WinningResolutionKind,
};

use font8x8::{UnicodeFonts as _, BASIC_FONTS};
use softbuffer::{Context, Surface};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

type SoftbufferSurface = Surface<Arc<winit::window::Window>, Arc<winit::window::Window>>;

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
    let mut overlay: Option<ShellOverlayState> = None;
    let mut command_runtime = CommandRuntimeState::default();
    let mut keybinding_runtime = KeybindingRuntimeState::default();

    window.request_redraw();

    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => elwt.exit(),
            WindowEvent::Resized(_) => {
                relayout_and_redraw(&window, &mut surface, &mut frame);
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                relayout_and_redraw(&window, &mut surface, &mut frame);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                held_modifiers.update_from_key_event(&event);
                if handle_key_event(
                    &window,
                    registry,
                    &mut frame,
                    &mut palette,
                    &mut overlay,
                    &mut command_runtime,
                    &mut keybinding_runtime,
                    &held_modifiers,
                    event,
                ) {
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
                    overlay.as_ref(),
                    &command_runtime,
                    &keybinding_runtime,
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
    CommandPalette,
    KeybindingChord,
}

impl DispatchOrigin {
    const fn issuing_surface(self) -> &'static str {
        match self {
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

fn alias_used_for(entry: &CommandRegistryEntryRecord, origin: DispatchOrigin) -> AliasUsedBlock {
    match origin {
        DispatchOrigin::CommandPalette => AliasUsedBlock {
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

fn argument_provenance_map_for(entry: &CommandRegistryEntryRecord) -> Vec<ArgumentProvenanceEntry> {
    match entry.descriptor.command_id.as_str() {
        "cmd:workspace.open_folder" => vec![
            ArgumentProvenanceEntry {
                argument_name: "workspace_scope_ref".to_string(),
                provenance: "user_selected_from_palette_suggestion".to_string(),
                resolved_value_ref: Some("workspace-scope:folder:recent:01".to_string()),
            },
            ArgumentProvenanceEntry {
                argument_name: "add_to_workspace".to_string(),
                provenance: "default_from_descriptor".to_string(),
                resolved_value_ref: Some("value:bool:false".to_string()),
            },
        ],
        "cmd:workspace.import_profile" => vec![
            ArgumentProvenanceEntry {
                argument_name: "import_source_ref".to_string(),
                provenance: "user_selected_from_palette_suggestion".to_string(),
                resolved_value_ref: Some("import-source:placeholder:01".to_string()),
            },
            ArgumentProvenanceEntry {
                argument_name: "apply_scope".to_string(),
                provenance: "default_from_descriptor".to_string(),
                resolved_value_ref: Some("enum:workspace.import_profile:profile_only".to_string()),
            },
            ArgumentProvenanceEntry {
                argument_name: "create_restore_checkpoint".to_string(),
                provenance: "default_from_descriptor".to_string(),
                resolved_value_ref: Some("value:bool:true".to_string()),
            },
        ],
        _ => entry
            .descriptor
            .typed_arguments
            .iter()
            .map(|slot| ArgumentProvenanceEntry {
                argument_name: slot.argument_name.clone(),
                provenance: slot
                    .default_provenance_when_omitted
                    .clone()
                    .unwrap_or_else(|| "user_typed".to_string()),
                resolved_value_ref: None,
            })
            .collect(),
    }
}

fn make_session(
    frame: &DesktopFrame,
    entry: &CommandRegistryEntryRecord,
    origin: DispatchOrigin,
    execution_intent: &str,
    preview_shown: bool,
    preview_record_ref: Option<String>,
    approval_state: &str,
    approval_ticket_ref: Option<String>,
) -> CommandInvocationSession {
    let canonical_verb = entry.descriptor.canonical_verb.clone();
    let basis_snapshot_ref = mint_basis_snapshot_ref(&canonical_verb);
    let focused = Some(format!("shell-zone:{}", frame.focused_zone().name()));

    let enablement = EnablementDecisionBlock {
        decision_class: entry.seed_enablement_snapshot.decision_class.clone(),
        disabled_reason_code: entry.seed_enablement_snapshot.disabled_reason_code.clone(),
        repair_hook_ref: entry.seed_enablement_snapshot.repair_hook_ref.clone(),
    };

    CommandInvocationSession {
        invocation_session_id: mint_invocation_session_id(&canonical_verb),
        canonical_command_id: entry.descriptor.command_id.clone(),
        command_revision_ref: entry.descriptor.command_revision_ref.clone(),
        canonical_verb,
        issuing_surface: origin.issuing_surface().to_string(),
        authority_class: "user_initiated_local".to_string(),
        alias_used: alias_used_for(entry, origin),
        argument_provenance_map: argument_provenance_map_for(entry),
        context_snapshot: InvocationContextSnapshot {
            focused_entity_ref: focused.clone(),
            selection_ref: None,
            workspace_trust_state: "trusted".to_string(),
            execution_context_id: entry.descriptor.policy_context.execution_context_id.clone(),
            scope_filter_class_ref: None,
            basis_snapshot_ref: basis_snapshot_ref.clone(),
        },
        context_refs: ContextRefsBlock {
            focused_entity_ref: focused,
            selection_ref: None,
            workspace_ref: None,
            workspace_trust_state: "trusted".to_string(),
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
    )
}

fn dispatch_registry_entry(
    command_runtime: &mut CommandRuntimeState,
    _registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    palette: &mut CommandPaletteState,
    overlay: &mut Option<ShellOverlayState>,
    entry: &CommandRegistryEntryRecord,
    origin: DispatchOrigin,
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

    let mut session = make_session(
        frame,
        entry,
        origin,
        execution_intent,
        preview_shown,
        preview_record_ref.clone(),
        &approval_state,
        approval_ticket_ref.clone(),
    );

    let unresolved_required = entry
        .descriptor
        .typed_arguments
        .iter()
        .filter(|slot| slot.is_required)
        .any(|slot| {
            session
                .argument_provenance_map
                .iter()
                .find(|row| row.argument_name == slot.argument_name)
                .and_then(|row| row.resolved_value_ref.as_ref())
                .is_none()
        });

    if unresolved_required {
        session.enablement_decision = EnablementDecisionBlock {
            decision_class: "disabled_with_reason".to_string(),
            disabled_reason_code: Some("required_argument_unresolved".to_string()),
            repair_hook_ref: None,
        };
        let invocation = invocation_and_result_denied(&session, "required_argument_unresolved");
        command_runtime.record(invocation);
        return true;
    }

    if session.enablement_decision.decision_class != "enabled" {
        let denied_code = session
            .enablement_decision
            .disabled_reason_code
            .clone()
            .unwrap_or_else(|| "policy_blocked_in_context".to_string());
        let invocation = invocation_and_result_denied(&session, &denied_code);
        command_runtime.record(invocation);
        return true;
    }

    match entry.descriptor.command_id.as_str() {
        "cmd:command_palette.open" => {
            palette.open();
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
            true
        }
        "cmd:workspace.import_profile" => {
            session.preview_posture.preview_shown = true;
            session.preview_posture.preview_record_ref =
                Some(mint_preview_record_ref(&entry.descriptor.canonical_verb));
            *overlay = Some(ShellOverlayState::command_preview(
                frame.focused_zone(),
                frame.focused_editor_group(),
                entry.clone(),
                session,
            ));
            frame.focus_zone(ShellZoneId::TransientOverlay);
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
    disabled_reason_code: &str,
) -> RecordedCommandInvocation {
    let outcome = InvocationOutcomeBlock {
        outcome_class: "denied_by_enablement".to_string(),
        disabled_reason_code: Some(disabled_reason_code.to_string()),
        warnings_summary_refs: Vec::new(),
        partially_applied_artifact_refs: Vec::new(),
        unapplied_artifact_refs: Vec::new(),
    };
    let session_packet = session.invocation_session_packet(outcome, Vec::new(), Vec::new());

    let result = ResultBodyBlock {
        outcome_code: "denied_by_enablement".to_string(),
        warning_codes: Vec::new(),
        error_codes: vec![disabled_reason_code.to_string()],
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
    overlay: &mut Option<ShellOverlayState>,
    command_runtime: &mut CommandRuntimeState,
    keybinding_runtime: &mut KeybindingRuntimeState,
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
        if code == KeyCode::Enter {
            let selected = palette.selected_entry(registry).cloned();
            palette.close();
            if let Some(entry) = selected {
                let changed = dispatch_registry_entry(
                    command_runtime,
                    registry,
                    frame,
                    palette,
                    overlay,
                    &entry,
                    DispatchOrigin::CommandPalette,
                );
                window.set_title(&window_title(Some(frame.focused_zone()), None));
                return changed;
            }
            window.set_title(&window_title(Some(frame.focused_zone()), None));
            return true;
        }
        if palette.handle_key(code) {
            window.set_title(&window_title(
                Some(frame.focused_zone()),
                palette
                    .is_open()
                    .then(|| palette.selected_entry(registry))
                    .flatten(),
            ));
            return true;
        }
        return false;
    }

    if let Some(state) = overlay.as_mut() {
        let outcome = state.handle_key(code, frame);
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
        let packet = seeded_keybinding_resolver().resolve(&sequence, &inspection_scope);
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
        _ => false,
    }
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
    overlay: Option<&ShellOverlayState>,
    command_runtime: &CommandRuntimeState,
    keybinding_runtime: &KeybindingRuntimeState,
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

    // Background.
    fill(&mut buffer, 0x0012171c);

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
        let color = zone_color(zone);
        fill_rect(&mut buffer, physical.width, physical.height, rect, color);

        match zone {
            ShellZoneId::MainWorkspace => {
                for group in frame.editor_group_layouts() {
                    let group_rect = to_physical_rect(group.rect, scale);
                    let group_color = editor_group_color(group.group_id);
                    fill_rect(
                        &mut buffer,
                        physical.width,
                        physical.height,
                        group_rect,
                        group_color,
                    );
                    if group.group_id == frame.focused_editor_group()
                        && frame.focused_zone() == ShellZoneId::MainWorkspace
                    {
                        stroke_rect(
                            &mut buffer,
                            physical.width,
                            physical.height,
                            group_rect,
                            2,
                            0x00ffffff,
                        );
                    }

                    let label = format!(
                        "group:{}  tabs:{}{}",
                        group.group_id.value(),
                        group.tab_count,
                        if group.tabbed_compare_active {
                            "  [tabbed compare]"
                        } else {
                            ""
                        }
                    );
                    draw_text(
                        &mut buffer,
                        physical.width,
                        physical.height,
                        group_rect.x.saturating_add(6),
                        group_rect.y.saturating_add(6),
                        1,
                        &label,
                        0x00e6edf3,
                    );
                }
            }
            _ => {
                for (slot_id, slot_rect) in frame.slot_rects_within_zone(zone, logical_rect) {
                    let slot_rect = to_physical_rect(slot_rect, scale);
                    let slot_color = slot_color(slot_id);
                    fill_rect(
                        &mut buffer,
                        physical.width,
                        physical.height,
                        slot_rect,
                        slot_color,
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
                2,
                0x00ffffff,
            );
        }

        let zone_label = format!("zone: {}", zone.name());
        draw_text(
            &mut buffer,
            physical.width,
            physical.height,
            rect.x.saturating_add(6),
            rect.y.saturating_add(2),
            1,
            &zone_label,
            0x00aab7c4,
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
        );
    }

    if let Some(overlay) = overlay {
        draw_shell_overlay(
            &mut buffer,
            physical.width,
            physical.height,
            window.scale_factor(),
            frame,
            overlay,
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
        let text = format!("fallback_modes: [{}]   last_cmd: {}   last_keybinding: {}   keys: Cmd/Ctrl+Shift+P palette (resolver), Enter run, Ctrl+\\ split, Ctrl+G next group, Ctrl+O add tab, Ctrl+W close group, Ctrl+I inspector (sheet)   packets: .logs/command_packets", modes, last, last_keybinding);
        draw_text(
            &mut buffer,
            physical.width,
            physical.height,
            status.x.saturating_add(6),
            status.y.saturating_add(6),
            1,
            &text,
            0x00c9d3de,
        );
    }

    buffer.present()?;
    Ok(())
}

#[derive(Debug, Clone)]
struct CommandPreviewOverlay {
    entry: CommandRegistryEntryRecord,
    session: CommandInvocationSession,
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
    CommandPreview(CommandPreviewOverlay),
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

    fn command_preview(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        entry: CommandRegistryEntryRecord,
        session: CommandInvocationSession,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::CommandPreview(CommandPreviewOverlay { entry, session }),
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

    fn handle_key(&mut self, code: KeyCode, frame: &mut DesktopFrame) -> OverlayKeyOutcome {
        let mut command_decision = None;
        let handled = match (&mut self.kind, code) {
            (_, KeyCode::Escape) => {
                if let ShellOverlayKind::CommandPreview(preview) = &mut self.kind {
                    preview.session.approval_posture.approval_state = "approval_denied".to_string();
                    command_decision = Some(CommandOverlayDecision::PreviewCancelled {
                        entry: preview.entry.clone(),
                        session: preview.session.clone(),
                    });
                }
                self.close(frame);
                true
            }
            (ShellOverlayKind::CommandPreview(preview), KeyCode::Enter) => {
                preview.session.approval_posture.approval_state = "approval_granted".to_string();
                if preview
                    .session
                    .approval_posture
                    .approval_ticket_ref
                    .is_none()
                    && preview
                        .session
                        .approval_posture
                        .approval_posture_class_declared
                        != "no_approval_required"
                {
                    preview.session.approval_posture.approval_ticket_ref =
                        Some(mint_approval_ticket_ref(&preview.session.canonical_verb));
                }
                command_decision = Some(CommandOverlayDecision::PreviewApproved {
                    entry: preview.entry.clone(),
                    session: preview.session.clone(),
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

fn editor_group_color(group_id: PaneId) -> u32 {
    let hash = group_id.value().wrapping_mul(2654435761) as u32;
    let r = (hash & 0xff) as u32;
    let g = ((hash >> 8) & 0xff) as u32;
    let b = ((hash >> 16) & 0xff) as u32;
    0x00000000 | (r << 16) | (g << 8) | b
}

fn draw_shell_overlay(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    scale_factor: f64,
    frame: &DesktopFrame,
    overlay: &ShellOverlayState,
) {
    let overlay_rect = to_physical_rect(frame.layout().transient_overlay, scale_factor);
    let sheet_w = (overlay_rect.width / 2).max(260);
    let sheet_rect = Rect::new(
        overlay_rect.right().saturating_sub(sheet_w),
        overlay_rect.y.saturating_add(60),
        sheet_w,
        overlay_rect.height.saturating_sub(120),
    );

    fill_rect(buffer, width, height, overlay_rect, 0x88000000);
    fill_rect(buffer, width, height, sheet_rect, 0x00202a35);
    stroke_rect(buffer, width, height, sheet_rect, 2, 0x00ffffff);

    match &overlay.kind {
        ShellOverlayKind::InspectorSheet => {
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(12),
                1,
                "Inspector (sheet) — Esc closes",
                0x00ffffff,
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(28),
                1,
                "Truth: inspector is sheeted on narrow widths; focus returns to the invoking pane.",
                0x00c9d3de,
            );
        }
        ShellOverlayKind::CommandPreview(preview) => {
            let header = format!(
                "Preview — {}  (Esc cancel, Enter apply)",
                preview.entry.title
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(12),
                1,
                &header,
                0x00ffffff,
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(28),
                1,
                &format!("command_id: {}", preview.entry.command_id()),
                0x00c9d3de,
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(44),
                1,
                &format!(
                    "preview_class: {}",
                    preview.session.preview_posture.preview_class_declared
                ),
                0x00c9d3de,
            );
            if let Some(preview_ref) = preview.session.preview_posture.preview_record_ref.as_ref() {
                draw_text(
                    buffer,
                    width,
                    height,
                    sheet_rect.x.saturating_add(12),
                    sheet_rect.y.saturating_add(60),
                    1,
                    &format!("preview_ref: {}", preview_ref),
                    0x00c9d3de,
                );
            }
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(78),
                1,
                "Decision mints an invocation session + result packet.",
                0x00c9d3de,
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(94),
                1,
                "Packets: .logs/command_packets",
                0x00c9d3de,
            );
        }
        ShellOverlayKind::CommandTrace(trace) => {
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(12),
                1,
                "Command Trace — Esc closes",
                0x00ffffff,
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(28),
                1,
                "Packets: .logs/command_packets",
                0x00c9d3de,
            );

            let mut y = sheet_rect.y.saturating_add(48);
            for line in &trace.lines {
                if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(12) {
                    break;
                }
                draw_text(
                    buffer,
                    width,
                    height,
                    sheet_rect.x.saturating_add(12),
                    y,
                    1,
                    line,
                    0x00c9d3de,
                );
                y = y.saturating_add(14);
            }
            if trace.lines.is_empty() {
                draw_text(
                    buffer,
                    width,
                    height,
                    sheet_rect.x.saturating_add(12),
                    y,
                    1,
                    "No invocations recorded yet.",
                    0x00c9d3de,
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
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(12),
                1,
                &header,
                0x00ffffff,
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(28),
                1,
                "Choose fallback: Up/Down, Enter confirm, Esc cancel",
                0x00aab7c4,
            );

            let options = ["Tabbed compare (recommended)", "Staged peek", "Cancel"];
            for (idx, label) in options.iter().enumerate() {
                let y = sheet_rect.y.saturating_add(52 + (idx as u32) * 18);
                if idx == *selection {
                    let highlight = Rect::new(
                        sheet_rect.x.saturating_add(8),
                        y.saturating_sub(2),
                        sheet_rect.width.saturating_sub(16),
                        16,
                    );
                    fill_rect(buffer, width, height, highlight, 0x002d3b4a);
                }
                draw_text(
                    buffer,
                    width,
                    height,
                    sheet_rect.x.saturating_add(14),
                    y,
                    1,
                    label,
                    if idx == *selection {
                        0x00ffffff
                    } else {
                        0x00c9d3de
                    },
                );
            }
        }
        ShellOverlayKind::StagedPeek => {
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(12),
                1,
                "Staged peek (sheet) — Esc closes",
                0x00ffffff,
            );
            draw_text(
                buffer,
                width,
                height,
                sheet_rect.x.saturating_add(12),
                sheet_rect.y.saturating_add(28),
                1,
                "This placeholder represents a temporary narrow-width compare peek with focus return.",
                0x00c9d3de,
            );
        }
    }
}

#[derive(Debug, Clone)]
struct CommandPaletteState {
    open: bool,
    selection: usize,
    visible_entry_indices: Vec<usize>,
}

impl CommandPaletteState {
    fn new(registry: &CommandRegistry) -> Self {
        let mut state = Self {
            open: false,
            selection: 0,
            visible_entry_indices: Vec::new(),
        };
        state.rebuild_visible_entries(registry);
        state
    }

    fn rebuild_visible_entries(&mut self, registry: &CommandRegistry) {
        self.visible_entry_indices = registry
            .entries()
            .iter()
            .enumerate()
            .filter_map(|(idx, entry)| {
                let desktop_ok = entry
                    .descriptor
                    .client_scopes
                    .iter()
                    .any(|scope| scope == "desktop_product");
                let visible_in_palette =
                    entry.descriptor.palette_visibility != "hidden_palette_callable_only";
                (desktop_ok && visible_in_palette).then_some(idx)
            })
            .collect();
        self.selection = self
            .selection
            .min(self.visible_entry_indices.len().saturating_sub(1));
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn open(&mut self) {
        self.open = true;
        self.selection = self
            .selection
            .min(self.visible_entry_indices.len().saturating_sub(1));
    }

    fn close(&mut self) {
        self.open = false;
    }

    fn selected_entry<'a>(
        &self,
        registry: &'a CommandRegistry,
    ) -> Option<&'a CommandRegistryEntryRecord> {
        let idx = *self.visible_entry_indices.get(self.selection)?;
        registry.entries().get(idx)
    }

    fn handle_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Escape => {
                self.close();
                true
            }
            KeyCode::ArrowDown => {
                if !self.visible_entry_indices.is_empty() {
                    self.selection = (self.selection + 1) % self.visible_entry_indices.len();
                }
                true
            }
            KeyCode::ArrowUp => {
                if !self.visible_entry_indices.is_empty() {
                    self.selection = (self.selection + self.visible_entry_indices.len() - 1)
                        % self.visible_entry_indices.len();
                }
                true
            }
            _ => false,
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

fn zone_color(zone: ShellZoneId) -> u32 {
    match zone {
        ShellZoneId::TitleContextBar => 0x0023303b,
        ShellZoneId::ActivityRail => 0x001c2a36,
        ShellZoneId::LeftSidebar => 0x001d3230,
        ShellZoneId::MainWorkspace => 0x001f2730,
        ShellZoneId::RightInspector => 0x002d2634,
        ShellZoneId::BottomPanel => 0x00221f2a,
        ShellZoneId::StatusBar => 0x001a2b1f,
        ShellZoneId::TransientOverlay => 0x00000000,
    }
}

fn slot_color(slot_id: &str) -> u32 {
    // Deterministic hash-to-color so placeholder slots remain visually distinct
    // without needing text rendering yet.
    let mut hash: u32 = 2166136261;
    for b in slot_id.as_bytes() {
        hash ^= u32::from(*b);
        hash = hash.wrapping_mul(16777619);
    }
    let r = (hash & 0xff) as u32;
    let g = ((hash >> 8) & 0xff) as u32;
    let b = ((hash >> 16) & 0xff) as u32;
    0x00000000 | (r << 16) | (g << 8) | b
}

fn fill(buffer: &mut [u32], color: u32) {
    for px in buffer {
        *px = color;
    }
}

fn fill_rect(buffer: &mut [u32], width: u32, height: u32, rect: Rect, color: u32) {
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
                *px = color;
            }
        }
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
    fill_rect(buffer, width, height, overlay_physical, 0x00101010);

    // Panel inside the slot.
    let panel_padding = 16u32;
    let panel = Rect::new(
        slot_physical.x.saturating_add(panel_padding),
        slot_physical.y.saturating_add(panel_padding),
        slot_physical.width.saturating_sub(panel_padding * 2),
        slot_physical.height.saturating_sub(panel_padding * 2),
    );
    if panel.is_empty() {
        return;
    }

    fill_rect(buffer, width, height, panel, 0x00161b22);
    stroke_rect(buffer, width, height, panel, 2, 0x0041556b);

    let text_scale = 2u32;
    let line_h = 8 * text_scale + 6;
    let mut cursor_y = panel.y.saturating_add(12);
    let cursor_x = panel.x.saturating_add(12);

    draw_text(
        buffer,
        width,
        height,
        cursor_x,
        cursor_y,
        text_scale,
        "Command Palette (Cmd/Ctrl+Shift+P)",
        0x00e6edf3,
    );
    cursor_y = cursor_y.saturating_add(line_h);

    draw_text(
        buffer,
        width,
        height,
        cursor_x,
        cursor_y,
        text_scale,
        "Up/Down: select   Enter: run   Esc: close",
        0x00aab7c4,
    );
    cursor_y = cursor_y.saturating_add(line_h + 6);

    for (row, entry) in palette_rows(registry).iter().enumerate() {
        if cursor_y.saturating_add(line_h) > panel.bottom().saturating_sub(12) {
            break;
        }
        let selected = row == palette.selection;
        if selected {
            let highlight = Rect::new(
                panel.x.saturating_add(6),
                cursor_y.saturating_sub(2),
                panel.width.saturating_sub(12),
                line_h,
            );
            fill_rect(buffer, width, height, highlight, 0x00202a35);
        }

        let mut line = format!("{}  —  {}", entry.title, entry.command_id());
        if entry.seed_enablement_snapshot.decision_class != "enabled" {
            if let Some(code) = &entry.seed_enablement_snapshot.disabled_reason_code {
                line.push_str("  [");
                line.push_str(code);
                line.push(']');
            }
        }

        draw_text(
            buffer,
            width,
            height,
            cursor_x,
            cursor_y,
            text_scale,
            &line,
            if selected { 0x00ffffff } else { 0x00c9d3de },
        );
        cursor_y = cursor_y.saturating_add(line_h);
    }
}

fn palette_rows<'a>(registry: &'a CommandRegistry) -> Vec<&'a CommandRegistryEntryRecord> {
    registry
        .entries()
        .iter()
        .filter(|entry| {
            entry
                .descriptor
                .client_scopes
                .iter()
                .any(|scope| scope == "desktop_product")
                && entry.descriptor.palette_visibility != "hidden_palette_callable_only"
        })
        .collect()
}

fn draw_text(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    mut x: u32,
    y: u32,
    scale: u32,
    text: &str,
    color: u32,
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
    color: u32,
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

#[derive(Debug, Default, Clone, Copy)]
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

#[derive(Debug, Default)]
struct KeybindingRuntimeState {
    last_summary: Option<String>,
}

impl KeybindingRuntimeState {
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
    color: u32,
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
                *px = color;
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
    color: u32,
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
